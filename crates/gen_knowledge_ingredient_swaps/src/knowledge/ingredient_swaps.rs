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
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HONEY_COMES_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SWAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_HONEY_COMES_IN() -> Py { ensure_init(); G_HONEY_COMES_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HONEY_COMES_IN(v: Py) { ensure_init(); G_HONEY_COMES_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DIET() -> Py { ensure_init(); G_NOT_A_DIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DIET(v: Py) { ensure_init(); G_NOT_A_DIET.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SWAP() -> Py { ensure_init(); G_NOT_A_SWAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SWAP(v: Py) { ensure_init(); G_NOT_A_SWAP.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_ASK() -> Py { ensure_init(); G_OUT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_ASK(v: Py) { ensure_init(); G_OUT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SWAP_ASK() -> Py { ensure_init(); G_SWAP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWAP_ASK(v: Py) { ensure_init(); G_SWAP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What stands in for an ingredient: the sugar a recipe runs out of, and the flour.\n\nThe rest of the swap family (butter, egg, milk, yeast, self-raising) lives in\nbot/knowledge/cooking.py. These two are here because they were the shapes that had no answer\nat all: \"i ran out of sugar\" was read as a feeling and answered \"Nice one - how did that feel?\",\n\"out of sugar\" got \"I'm listening\", and \"what can i use instead of sugar\" reached the\ncut-down-on-sugar card in nutrition, which is a diet question and the wrong job for someone\nmid-recipe with a cake in the bowl.\n\nA shortage and a swap are the same question - \"im out of sugar\", \"no sugar, what can i use\",\n\"can i use granulated instead of caster sugar\" - so both rows are keyed on the ingredient plus a\nshortage-or-swap phrase. The diet lines keep their own answer in nutrition, and a flourless\nrecipe keeps its own answer, so the sugar row stands down on \"cut down\", \"give up\", \"sweets\" and\non comparison questions (\"difference\", \"vs\", \"same as\"), and the flour row stands down on\n\"without flour\" and \"flourless\".\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_SWAP_ASK(py_tuple(vec![py_str("instead"), py_str("substitute"), py_str("swap"), py_str("replace"), py_str("alternative"), py_str("what can i use"), py_str("what do i use"), py_str("what should i use"), py_str("what else can i use"), py_str("use in place of"), py_str("stand-in"), py_str("stand in")]));
    fn __data1() -> Py { py_tuple(vec![py_str("out of"), py_str("ran out"), py_str("run out"), py_str("none left"), py_str("nothing left"), py_str("havent got"), py_str("haven't got"), py_str("dont have"), py_str("don't have"), py_str("no sugar"), py_str("no flour"), py_str("no honey"), py_str("no icing sugar"), py_str("no caster sugar"), py_str("no plain flour"), py_str("no sugar left"), py_str("no flour left"), py_str("no honey left")]) }
    s_OUT_ASK(__data1());
    s_ASK(op_add(&g_SWAP_ASK(), &g_OUT_ASK()));
    fn __data2() -> Py { py_tuple(vec![py_str("cut down"), py_str("cut out"), py_str("give up"), py_str("less sugar"), py_str("reduce"), py_str("diet"), py_str("dieting"), py_str("calories"), py_str("sweets"), py_str("sweets and"), py_str("biscuits"), py_str("chocolate"), py_str("cravings"), py_str("crave"), py_str("stop eating"), py_str("healthy"), py_str("healthier"), py_str("lose weight"), py_str("cut sugar"), py_str("cut the sugar"), py_str("cutting sugar"), py_str("add sugar"), py_str("no idea"), py_str("in my coffee"), py_str("in my tea"), py_str("take no sugar"), py_str("takes no sugar")]) }
    s_NOT_A_DIET(__data2());
    s_HONEY_COMES_IN(py_tuple(vec![py_str("use honey"), py_str("using honey"), py_str("honey instead"), py_str("honey rather")]));
    fn __data3() -> Py { py_tuple(vec![py_str("difference"), py_str("different"), py_str("same as"), py_str("vs"), py_str("versus"), py_str("better for you"), py_str("gone hard"), py_str("set hard"), py_str("hard as"), py_str("lumpy"), py_str("caked"), py_str("clumped"), py_str("goes hard"), py_str("go hard"), py_str("how do i store"), py_str("keep it soft"), py_str("storing"), py_str("burnt"), py_str("burned"), py_str("burnt on"), py_str("stuck on"), py_str("stuck to"), py_str("scorched"), py_str("charred"), py_str("cleanout of date"), py_str("use by"), py_str("use-by"), py_str("past its date"), py_str("past the date"), py_str("best before"), py_str("gone past")]) }
    s_NOT_A_SWAP(__data3());
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.ingredient_swaps".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK" => Some(g_ASK()),
            "HONEY_COMES_IN" => Some(g_HONEY_COMES_IN()),
            "NOT_A_DIET" => Some(g_NOT_A_DIET()),
            "NOT_A_SWAP" => Some(g_NOT_A_SWAP()),
            "OUT_ASK" => Some(g_OUT_ASK()),
            "SWAP_ASK" => Some(g_SWAP_ASK()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASK" => s_ASK(v),
            "HONEY_COMES_IN" => s_HONEY_COMES_IN(v),
            "NOT_A_DIET" => s_NOT_A_DIET(v),
            "NOT_A_SWAP" => s_NOT_A_SWAP(v),
            "OUT_ASK" => s_OUT_ASK(v),
            "SWAP_ASK" => s_SWAP_ASK(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.ingredient_swaps", module_obj);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "HONEY_COMES_IN", g_HONEY_COMES_IN, s_HONEY_COMES_IN);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "NOT_A_DIET", g_NOT_A_DIET, s_NOT_A_DIET);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "NOT_A_SWAP", g_NOT_A_SWAP, s_NOT_A_SWAP);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "OUT_ASK", g_OUT_ASK, s_OUT_ASK);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "SWAP_ASK", g_SWAP_ASK, s_SWAP_ASK);
    pyrt::xrefs::reg_access("knowledge.ingredient_swaps", "entry", g_entry, s_entry);
}
