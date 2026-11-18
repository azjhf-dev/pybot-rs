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
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.food_left_out: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED() -> Py { ensure_init(); G_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); G_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FOOD() -> Py { ensure_init(); G_NOT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FOOD(v: Py) { ensure_init(); G_NOT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT() -> Py { ensure_init(); G_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT(v: Py) { ensure_init(); G_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A plate left out: the clock, for the line that names no food at all.\n\n\"How long can food sit out\", \"left out of the fridge all night is it ok\", \"I left my lunch out on\nthe counter\" - the keeping rows in cooking.py answer these for a food that is named (\"the chicken\nwas left out all night is it safe\" gets the chicken card, with its own clock) and cannot answer\nthem when no food is named, because their key is the food. Until this row existed those lines\ncame back as small talk: the reading heard \"left out\" as being left out of the group and the\ncomposer said \"Loneliness is rough.\" to somebody asking about their dinner.\n\nSo the key here is the state - left out, sat out, out of the fridge, out on the counter - and the\nreply is the general rule that holds whatever the food is. A named food is held out in AVOID, so\nthe food's own card keeps every line it can answer and this row answers only what is left: the\nline that says \"food\", \"lunch\", \"dinner\", \"leftovers\", or nothing but the question.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.food_left_out.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("left out"), py_str("left it out"), py_str("left them out"), py_str("sat out"), py_str("out of the fridge"), py_str("out on the counter"), py_str("out on the side"), py_str("out on the worktop"), py_str("on the worktop"), py_str("left on the counter"), py_str("left on the side"), py_str("left on the table"), py_str("left on the worktop"), py_str("out overnight"), py_str("left overnight"), py_str("in my bag"), py_str("in my lunch bag"), py_str("in my rucksack")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.food_left_out.OUT"); }
    s_OUT(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("is it ok"), py_str("is it okay"), py_str("is it safe"), py_str("still ok"), py_str("still fine"), py_str("still safe"), py_str("overnight"), py_str("all night"), py_str("since this morning"), py_str("since last night"), py_str("can i eat"), py_str("can i still eat"), py_str("ok to eat"), py_str("safe to eat"), py_str("should i throw"), py_str("throw it away"), py_str("bin it"), py_str("gone off"), py_str("how long"), py_str("keep it"), py_str("reheat"), py_str("food"), py_str("lunch"), py_str("dinner"), py_str("supper"), py_str("tea"), py_str("meal"), py_str("meals"), py_str("leftovers"), py_str("plate"), py_str("takeaway"), py_str("my food"), py_str("the food"), py_str("something")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.food_left_out.ASKS"); }
    s_ASKS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("fish"), py_str("salmon"), py_str("prawns"), py_str("prawn"), py_str("ham"), py_str("mince"), py_str("sausages"), py_str("sausage"), py_str("bacon"), py_str("rice"), py_str("pasta"), py_str("milk"), py_str("cream"), py_str("cheese"), py_str("butter"), py_str("yoghurt"), py_str("yogurt"), py_str("egg"), py_str("eggs"), py_str("soup"), py_str("stew"), py_str("curry"), py_str("gravy"), py_str("cake"), py_str("bread"), py_str("sandwich"), py_str("salad"), py_str("pizza"), py_str("chips"), py_str("mayonnaise"), py_str("mayo"), py_str("custard"), py_str("quiche"), py_str("sushi"), py_str("beans"), py_str("peas"), py_str("potatoes"), py_str("lasagne"), py_str("lasagna"), py_str("casserole"), py_str("roast"), py_str("stuffing"), py_str("pate"), py_str("prawn cocktail"), py_str("formula"), py_str("baby bottle"), py_str("baby"), py_str("bottle"), py_str("breast milk"), py_str("breastmilk"), py_str("milk powder"), py_str("powdered milk")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.food_left_out.NAMED"); }
    s_NAMED(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("team"), py_str("group"), py_str("friends"), py_str("party"), py_str("invited"), py_str("feel"), py_str("felt"), py_str("feeling"), py_str("at work"), py_str("school"), py_str("playground"), py_str("chat"), py_str("wedding"), py_str("the lads"), py_str("the girls"), py_str("family"), py_str("excluded"), py_str("ignored"), py_str("laughed at"), py_str("mentioned"), py_str("conversation"), py_str("phone"), py_str("mobile"), py_str("keys"), py_str("wallet"), py_str("purse"), py_str("laptop"), py_str("charger"), py_str("book"), py_str("washing"), py_str("clothes"), py_str("laundry"), py_str("dog"), py_str("cat"), py_str("pet"), py_str("bin"), py_str("recycling"), py_str("paint"), py_str("tools"), py_str("bike"), py_str("car keys"), py_str("parcel"), py_str("post"), py_str("rubbish"), py_str("bins"), py_str("battery"), py_str("medicines"), py_str("tablets"), py_str("razor"), py_str("bleach"), py_str("washing up"), py_str("takeaway"), py_str("take away"), py_str("cooked"), py_str("cooking"), py_str("roast"), py_str("restaurant"), py_str("chinese"), py_str("kebab"), py_str("biryani"), py_str("doggy bag"), py_str("carry out"), py_str("ordered in"), py_str("delivery")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.food_left_out.NOT_FOOD"); }
    s_NOT_FOOD(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.food_left_out.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("left-out-clock")); __args.push(py_tuple(vec![g_OUT(), g_ASKS()])); __args.push(py_str("The clock decides, not the look: two hours at room temperature is the limit, and one hour if the room is hot. Under that, box it and put it in the fridge and use it in the next day or two. Over it, bin anything with meat, fish, rice, dairy, eggs or a sauce, because the bugs that make you ill leave the taste and the smell alone and reheating does not undo it. Bread, cake, biscuits, jam and fruit are the exceptions and keep on the counter for days.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NAMED(), &g_NOT_FOOD())), ("bias".to_string(), Py::Int(0))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.food_left_out".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKS" => Some(g_ASKS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "NAMED" => Some(g_NAMED()),
            "NOT_FOOD" => Some(g_NOT_FOOD()),
            "OUT" => Some(g_OUT()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASKS" => s_ASKS(v),
            "ENTRIES" => s_ENTRIES(v),
            "NAMED" => s_NAMED(v),
            "NOT_FOOD" => s_NOT_FOOD(v),
            "OUT" => s_OUT(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.food_left_out", module_obj);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "NAMED", g_NAMED, s_NAMED);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "NOT_FOOD", g_NOT_FOOD, s_NOT_FOOD);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "OUT", g_OUT, s_OUT);
    pyrt::xrefs::reg_access("knowledge.food_left_out", "entry", g_entry, s_entry);
}
