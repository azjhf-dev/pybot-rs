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
thread_local! { static G_OUT_TAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOST_OR_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_ABOUT_TODAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUT_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_OUT_TAKE() -> Py { ensure_init(); G_OUT_TAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_TAKE(v: Py) { ensure_init(); G_OUT_TAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_WHEN() -> Py { ensure_init(); G_OUT_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_WHEN(v: Py) { ensure_init(); G_OUT_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOCK() -> Py { ensure_init(); G__A_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOCK(v: Py) { ensure_init(); G__A_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g__LOST_OR_TAKEN() -> Py { ensure_init(); G__LOST_OR_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOST_OR_TAKEN(v: Py) { ensure_init(); G__LOST_OR_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_ABOUT_TODAY() -> Py { ensure_init(); G__NOT_ABOUT_TODAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_ABOUT_TODAY(v: Py) { ensure_init(); G__NOT_ABOUT_TODAY.with(|c| *c.borrow_mut() = v); }
pub fn g__OUT_WHEN() -> Py { ensure_init(); G__OUT_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUT_WHEN(v: Py) { ensure_init(); G__OUT_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Whether today needs a coat: the one clothes question with no garment to buy, wash or mend.\n\n\"Do I need a coat today\" is a yes or no about the next few hours, and two rows in this package\nsay in their own comments that they leave it to somebody else: `clothing.py`'s `child-coat`\nkeeps \"today\" and \"weather\" out of its key because \"do the kids need a coat today\" is a forecast\nquestion, and `shopping.py`'s `coat-worth` keeps \"tonight\", \"tomorrow\" and \"umbrella\" out for the\nsame reason. Nothing answered it, so those lines got a shrug (\"That's your call, not mine\"), a\nweather note with no answer in it (\"A weather app has the next few days in one tap\"), or a card\nabout something else entirely (\"is it cold enough for a coat\" was answered with the washing\nmachine's 30C setting).\n\nThe bot cannot see a forecast. What a friend says is the rule: take it, because being cold or\nwet for an hour costs more than carrying a coat you never put on, and read the forecast for the\nhours you will actually be out rather than the day's high.\n\nThe row lives in its own domain rather than in `clothing.py` because the garment words are shared\nwith the buying card, the washing card and a coat of paint, and the guard below is what keeps\nthem apart - a rule about the next few hours, not a fact about a coat.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("shopping"), py_str("nature-and-weather")]));
    fn __data1() -> Py { py_tuple(vec![py_str("today"), py_str("tonight"), py_str("tomorrow"), py_str("this morning"), py_str("this afternoon"), py_str("this evening"), py_str("in the morning"), py_str("in the afternoon"), py_str("in the evening"), py_str("right now"), py_str("later"), py_str("later on"), py_str("out"), py_str("outside"), py_str("weather"), py_str("forecast"), py_str("cold out"), py_str("chilly out"), py_str("warm out"), py_str("raining"), py_str("rain today"), py_str("enough"), py_str("the weekend"), py_str("this weekend"), py_str("saturday"), py_str("sunday"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("after work"), py_str("school run"), py_str("when i go out"), py_str("to work"), py_str("at work"), py_str("for work"), py_str("to school"), py_str("at school"), py_str("for school"), py_str("to the office"), py_str("at the office"), py_str("in the office"), py_str("this pm"), py_str("this am"), py_str("in the morning"), py_str("before work")]) }
    s_OUT_WHEN(__data1());
    s_OUT_TAKE(py_str("Take it: being cold or wet for an hour costs you far more than carrying something you never put on, and it is the bright morning that catches people out. Read the forecast for the hours you will actually be out, because the day's high is at two in the afternoon and the cold and the showers come later; under it a thin layer you can take off is worth more than a thicker coat, and if you will be standing about after dark, that is the case for the warm one."));
    fn __data2() -> Py { py_tuple(vec![py_str("what coat"), py_str("what jacket"), py_str("which coat"), py_str("which jacket"), py_str("should i get"), py_str("should i buy"), py_str("worth it"), py_str("is it worth"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("price"), py_str("cheap"), py_str("expensive"), py_str("best"), py_str("recommend"), py_str("shop"), py_str("sale"), py_str("size"), py_str("fits"), py_str("fit me"), py_str("too small"), py_str("too big"), py_str("outgrew"), py_str("needs a new"), py_str("need a new"), py_str("a new coat"), py_str("a new jacket"), py_str("new coat"), py_str("new jacket"), py_str("brand new"), py_str("zip"), py_str("zipper"), py_str("torn"), py_str("ripped"), py_str("stain"), py_str("smell"), py_str("store"), py_str("storage"), py_str("put away"), py_str("moth"), py_str("wash"), py_str("clean"), py_str("repair"), py_str("waterproof it"), py_str("reproof"), py_str("leak"), py_str("paint"), py_str("second coat"), py_str("coats of"), py_str("jacket potato"), py_str("potato"), py_str("antivirus"), py_str("laptop"), py_str("computer"), py_str("macbook"), py_str("city break"), py_str("weekend away"), py_str("holiday"), py_str("packing"), py_str("for the trip"), py_str("winter coat"), py_str("warm coat"), py_str("coat thats"), py_str("coat that is"), py_str("waterproof coat"), py_str("puffer"), py_str("ordered"), py_str("order"), py_str("medium"), py_str("small"), py_str("large"), py_str("arrived"), py_str("delivery"), py_str("wore"), py_str("worn"), py_str("inside out"), py_str("yesterday"), py_str("all day"), py_str("big enough"), py_str("long enough"), py_str("too long"), py_str("too short"), py_str("sleeves"), py_str("sleeve"), py_str("shoulders"), py_str("alter"), py_str("altering"), py_str("tailor")]) }
    s__NOT_ABOUT_TODAY(__data2());
    s__OUT_WHEN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OUT_WHEN()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__LOST_OR_TAKEN(stdlib_call("re", "compile", &[py_str("\\b(?:swiped|stolen|stole|nicked|robbed|burgled|pickpocket\\w*|went missing|gone missing|is missing|are missing|left (?:it|them|mine|his|hers|my|the) (?:on|in|at)|someone took|took my|taken my|los(?:t|ing|es) (?:his|her|their|my|our|the|it|them))\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CLOCK(stdlib_call("re", "compile", &[py_str("\\b\\d{1,2}\\s?(?:am|pm)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("coat"), py_str("coats"), py_str("jacket"), py_str("jackets"), py_str("raincoat"), py_str("brolly"), py_str("umbrella"), py_str("umbrellas"), py_str("warm layer"), py_str("jumper"), py_str("fleece"), py_str("cardigan"), py_str("hat"), py_str("gloves"), py_str("scarf")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("pram"), py_str("pushchair"), py_str("car seat"), py_str("travel system"), py_str("hiking"), py_str("hike"), py_str("hill"), py_str("hills"), py_str("walking"), py_str("walk"), py_str("cycling"), py_str("running"), py_str("football"), py_str("rugby"), py_str("match"), py_str("camping"), py_str("festival"), py_str("wedding"), py_str("funeral"), py_str("interview"), py_str("uniform"), py_str("suit")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coat-out-today")); __args.push(py_tuple(vec![__data3(), py_tuple(vec![py_str("need"), py_str("take"), py_str("bring"), py_str("pack"), py_str("wear"), py_str("wearing"), py_str("put on"), py_str("should i"), py_str("do i"), py_str("will i"), py_str("shall i"), py_str("bother")]), g_OUT_WHEN()])); __args.push(g_OUT_TAKE()); __kws.extend(vec![("guard".to_string(), py_func("_a_garment_to_take", |args: &[Py]| _a_garment_to_take(args))), ("avoid".to_string(), __data4()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _a_garment_to_take(args: &[Py]) -> Py {
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
    let mut filler: Py = Py::None;
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is a yes or no about taking a garment out today.\n\n    The garment words are shared by a buying question, a washing question and a coat of paint, so\n    the reading is done here rather than in the key: what this card answers is whether the thing\n    goes on today, and every other question about a coat wants a price, a fabric, a wash or a\n    fault instead.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("words"), py_str("clean"), py_str("text")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&py_str("[],'"));
    for __item in __iter1 {
    filler = __item.clone();
    said = mcall(&said.clone(), "replace", &[filler.clone(), py_str(" ")]);
    }
    let __iter2 = py_iter_or_raise(&op_add(&py_tuple(vec![py_str("how do i"), py_str("how do you"), py_str("how can i"), py_str("how to")]), &g__NOT_ABOUT_TODAY()));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &word.clone()))) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&mcall(&g__LOST_OR_TAKEN(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__OUT_WHEN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_CLOCK(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.coats".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "OUT_TAKE" => Some(g_OUT_TAKE()),
            "OUT_WHEN" => Some(g_OUT_WHEN()),
            "_A_CLOCK" => Some(g__A_CLOCK()),
            "_LOST_OR_TAKEN" => Some(g__LOST_OR_TAKEN()),
            "_NOT_ABOUT_TODAY" => Some(g__NOT_ABOUT_TODAY()),
            "_OUT_WHEN" => Some(g__OUT_WHEN()),
            "_a_garment_to_take" => Some(py_func("_a_garment_to_take", |args: &[Py]| _a_garment_to_take(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "OUT_TAKE" => s_OUT_TAKE(v),
            "OUT_WHEN" => s_OUT_WHEN(v),
            "_A_CLOCK" => s__A_CLOCK(v),
            "_LOST_OR_TAKEN" => s__LOST_OR_TAKEN(v),
            "_NOT_ABOUT_TODAY" => s__NOT_ABOUT_TODAY(v),
            "_OUT_WHEN" => s__OUT_WHEN(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.coats", module_obj);
    pyrt::xrefs::reg_fn("knowledge.coats", "_a_garment_to_take", py_func("_a_garment_to_take", |args: &[Py]| _a_garment_to_take(args)));
    pyrt::xrefs::reg_access("knowledge.coats", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.coats", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.coats", "OUT_TAKE", g_OUT_TAKE, s_OUT_TAKE);
    pyrt::xrefs::reg_access("knowledge.coats", "OUT_WHEN", g_OUT_WHEN, s_OUT_WHEN);
    pyrt::xrefs::reg_access("knowledge.coats", "_A_CLOCK", g__A_CLOCK, s__A_CLOCK);
    pyrt::xrefs::reg_access("knowledge.coats", "_LOST_OR_TAKEN", g__LOST_OR_TAKEN, s__LOST_OR_TAKEN);
    pyrt::xrefs::reg_access("knowledge.coats", "_NOT_ABOUT_TODAY", g__NOT_ABOUT_TODAY, s__NOT_ABOUT_TODAY);
    pyrt::xrefs::reg_access("knowledge.coats", "_OUT_WHEN", g__OUT_WHEN, s__OUT_WHEN);
    pyrt::xrefs::reg_access("knowledge.coats", "entry", g_entry, s_entry);
}
