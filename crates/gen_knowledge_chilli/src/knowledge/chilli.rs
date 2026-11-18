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
thread_local! { static G_A_COOK_CLOCK_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KEEPING_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POT_OF_CHILLI_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRY_HEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRY_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRY_ORDER_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FAULT_IN_THE_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEAN_STEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILLI_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHORTAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_said_in_this_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.chilli: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COOK_CLOCK_VERB() -> Py { ensure_init(); G_A_COOK_CLOCK_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COOK_CLOCK_VERB(v: Py) { ensure_init(); G_A_COOK_CLOCK_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KEEPING_CUE() -> Py { ensure_init(); G_A_KEEPING_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KEEPING_CUE(v: Py) { ensure_init(); G_A_KEEPING_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POT_OF_CHILLI_KEEPS() -> Py { ensure_init(); G_A_POT_OF_CHILLI_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POT_OF_CHILLI_KEEPS(v: Py) { ensure_init(); G_A_POT_OF_CHILLI_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED() -> Py { ensure_init(); G_COOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED(v: Py) { ensure_init(); G_COOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRY_HEAT() -> Py { ensure_init(); G_CURRY_HEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRY_HEAT(v: Py) { ensure_init(); G_CURRY_HEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRY_NAMES() -> Py { ensure_init(); G_CURRY_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRY_NAMES(v: Py) { ensure_init(); G_CURRY_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRY_ORDER_LINE() -> Py { ensure_init(); G_CURRY_ORDER_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRY_ORDER_LINE(v: Py) { ensure_init(); G_CURRY_ORDER_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH() -> Py { ensure_init(); G_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH(v: Py) { ensure_init(); G_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAT_WORDS() -> Py { ensure_init(); G_HEAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAT_WORDS(v: Py) { ensure_init(); G_HEAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FAULT_IN_THE_DISH() -> Py { ensure_init(); G__A_FAULT_IN_THE_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FAULT_IN_THE_DISH(v: Py) { ensure_init(); G__A_FAULT_IN_THE_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__BEAN_STEMS() -> Py { ensure_init(); G__BEAN_STEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEAN_STEMS(v: Py) { ensure_init(); G__BEAN_STEMS.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILLI_IN_THE_LINE() -> Py { ensure_init(); G__CHILLI_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILLI_IN_THE_LINE(v: Py) { ensure_init(); G__CHILLI_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAT_WORDS() -> Py { ensure_init(); G__HEAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAT_WORDS(v: Py) { ensure_init(); G__HEAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHORTAGE() -> Py { ensure_init(); G__SHORTAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHORTAGE(v: Py) { ensure_init(); G__SHORTAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_said_in_this_line() -> Py { ensure_init(); G_said_in_this_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_said_in_this_line(v: Py) { ensure_init(); G_said_in_this_line.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Chillies as a vegetable: what to do with a glut, how hot the thing is, and what stands in.\n\nThe answers lane owns this domain. A chilli is a dish as well as a vegetable, and the dish\nanswers every way of asking for the recipe, so these are the questions the recipe is the wrong\nanswer to: keeping a box of them, freezing them, drying them, chopping them, how hot one is and\nwhat to use when there are none. Each row is keyed by the chilli names plus the words that name\nthe job or the property, so \"how hot is a jalapeno\" and \"how spicy is a chilli\" reach the same\nanswer without either of them naming a chilli variety.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_said_in_this_line(py_func("said_in_this_line", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "said_in_this_line", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_DISH(py_tuple(vec![py_str("con carne"), py_str("con carni"), py_str("bolognese"), py_str("chilli beef"), py_str("stew"), py_str("recipe"), py_str("dish"), py_str("saucepan"), py_str("in a curry")]));
    s_COOKED(py_tuple(vec![py_str(" pot "), py_str("pot of"), py_str("a pot"), py_str("batch"), py_str("cooked"), py_str("already made"), py_str("made it")]));
    s__CHILLI_IN_THE_LINE(py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chile"), py_str("chiles"), py_str("jalapeno"), py_str("jalapenos"), py_str("habanero"), py_str("scotch bonnet"), py_str("birds eye")]));
    fn __data1() -> Py { py_tuple(vec![py_str("simmer"), py_str("simmering"), py_str("simmered"), py_str("cook"), py_str("cooking"), py_str("boil"), py_str("boiling"), py_str("fry"), py_str("frying"), py_str("fried"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("oven"), py_str("hob"), py_str("pan"), py_str("minutes"), py_str("stir fry"), py_str("grill"), py_str("steam")]) }
    s_HEAT_WORDS(__data1());
    s_CURRY_HEAT(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("korma"), py_str("pasanda"), py_str("butter chicken"), py_str("makhani")]), py_str("mildest on the menu, creamy and with no chilli heat at all")]), py_tuple(vec![py_tuple(vec![py_str("tikka masala"), py_str("tikka"), py_str("masala")]), py_str("mild, a tomato and cream sauce with a little warmth")]), py_tuple(vec![py_tuple(vec![py_str("rogan josh"), py_str("dhansak"), py_str("balti")]), py_str("medium, warming rather than burning")]), py_tuple(vec![py_tuple(vec![py_str("madras")]), py_str("the first properly hot one on most menus, hot without being punishing")]), py_tuple(vec![py_tuple(vec![py_str("jalfrezi")]), py_str("hot, with green chillies and peppers left in")]), py_tuple(vec![py_tuple(vec![py_str("vindaloo")]), py_str("very hot, and hotter in some kitchens than others")]), py_tuple(vec![py_tuple(vec![py_str("phal"), py_str("naga"), py_str("tindaloo")]), py_str("the hottest thing on the menu, and past the point where most people taste anything else")])]));
    s_CURRY_NAMES({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g_CURRY_HEAT()) { let __unpk5573 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk5573[0].clone(); let mut _u: Py = __unpk5573[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push((name.clone(), name.clone())); } } py_dict(__out) })() });
    s_CURRY_ORDER_LINE(py_str("korma, pasanda, butter chicken, tikka masala, rogan josh, balti, dhansak, madras, jalfrezi, vindaloo, phaal"));
    fn __data2() -> Py { py_tuple(vec![py_str("spicy"), py_str("spice"), py_str("spicier"), py_str("hot"), py_str("hotter"), py_str("hottest"), py_str("mild"), py_str("milder"), py_str("mildest"), py_str("heat"), py_str("strength"), py_str("how strong"), py_str("scoville"), py_str("burn"), py_str("burning"), py_str("too spicy"), py_str("too hot"), py_str("which is mildest"), py_str("least spicy")]) }
    s__HEAT_WORDS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("too spicy"), py_str("too hot to eat"), py_str("too hot for me"), py_str("too hot for the kids"), py_str("way too hot"), py_str("way too spicy"), py_str("far too spicy"), py_str("far too hot"), py_str("cool it down"), py_str("cool the heat"), py_str("cool this down"), py_str("make it milder"), py_str("make it less spicy"), py_str("less spicy"), py_str("not so spicy"), py_str("too much chilli"), py_str("too much chili"), py_str("blow your head off"), py_str("burning my mouth")]) }
    s__A_FAULT_IN_THE_DISH(__data3());
    s__SHORTAGE(stdlib_call("re", "compile", &[py_str("\\b(?:but no|and no|no|without|short of|missing|havent got|haven't got|not got|ran out of|run out of)\\s+((?:[a-z']+\\s*){0,3})")]).unwrap_or_else(|e| exc::raise(e)));
    s__BEAN_STEMS(py_tuple(vec![py_str("bean"), py_str("kidney"), py_str("pinto"), py_str("cannelli"), py_str("haricot"), py_str("baked"), py_str("black")]));
    s_A_COOK_CLOCK_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:take|takes|taking|simmer|simmering|boil|boiling|fry|frying|bake|baking|roast|roasting|cook|cooks|cooking|poach|steam|grill)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_KEEPING_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|kept|last|lasts|lasting|store|stored|fridge|freezer|freeze|frozen|good for|leftover|left over|reheat|reheats|out of date|use by|still good|safe to eat|night|tomorrow)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POT_OF_CHILLI_KEEPS(py_str("A pot of chilli keeps 3 days in the fridge, in a sealed box, once it has cooled within two hours of cooking; it freezes for 3 months and it is better the next day anyway. Reheat it once, until it is piping hot right through, and don't leave it out on the hob overnight."));
    fn __data4() -> Py { py_tuple(vec![py_str("too spicy"), py_str("too hot"), py_str("less spicy"), py_str("too dry"), py_str("is dry"), py_str("tastes dry"), py_str("came out dry"), py_str("too wet"), py_str("plant"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("seedling"), py_str("powder"), py_str("flakes"), py_str("paste"), py_str("freeze"), py_str("frozen"), py_str("drying"), py_str("dry them"), py_str("instead of"), py_str("substitute"), py_str("swap"), py_str("replace"), py_str("oil"), py_str("take"), py_str("takes"), py_str("taking")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("good for"), py_str("last"), py_str("lasts"), py_str("how long"), py_str("store"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("leftover"), py_str("left over")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("plant"), py_str("grow"), py_str("growing"), py_str("seedling"), py_str("sow"), py_str("powder"), py_str("flakes"), py_str("paste"), py_str("how hot"), py_str("too spicy"), py_str("too dry"), py_str("instead of"), py_str("con carne"), py_str("con carni"), py_str("take"), py_str("takes"), py_str("taking"), py_str("simmer"), py_str("simmering"), py_str("boil"), py_str("boiling"), py_str("fry"), py_str("frying"), py_str("oven"), py_str("hob"), py_str("how long to cook"), py_str("cooking time")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("korma"), py_str("pasanda"), py_str("butter chicken"), py_str("tikka masala"), py_str("tikka"), py_str("rogan josh"), py_str("dhansak"), py_str("balti"), py_str("madras"), py_str("jalfrezi"), py_str("vindaloo"), py_str("phal"), py_str("naga"), py_str("tindaloo"), py_str("curry"), py_str("curries")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("recipe"), py_str("how do i make"), py_str("how to make"), py_str("calories"), py_str("sugar"), py_str("healthy"), py_str("wine"), py_str("beer"), py_str("drink"), py_str("side"), py_str("sides"), py_str("goes with"), py_str("naan"), py_str("rice"), py_str("hangover"), py_str("heartburn"), py_str("reflux"), py_str("indigestion"), py_str("stomach"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("kids"), py_str("plant"), py_str("grow"), py_str("seedling"), py_str("powder"), py_str("flakes"), py_str("paste")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("split"), py_str("splits"), py_str("splitting"), py_str("curdled"), py_str("curdle"), py_str("curdling"), py_str("grainy"), py_str("seized"), py_str("too thick"), py_str("too thin"), py_str("watery"), py_str("runny"), py_str("calories"), py_str("healthy"), py_str("how many"), py_str("protein"), py_str("freeze"), py_str("frozen"), py_str("mouldy"), py_str("out of date"), py_str("gone off")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("too spicy"), py_str("too hot"), py_str("too dry"), py_str("is dry"), py_str("plant"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("not working"), py_str("broken"), py_str("instead of"), py_str("oil"), py_str("leftover"), py_str("left over"), py_str("cooked"), py_str("already made"), py_str("batch")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("too spicy"), py_str("too hot"), py_str("too dry"), py_str("is dry"), py_str("tastes dry"), py_str("came out dry"), py_str("too wet"), py_str("plant"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("tumble dryer"), py_str("washing"), py_str("instead of"), py_str("oil"), py_str("boiled dry"), py_str("boils dry"), py_str("boil dry"), py_str("boiling dry"), py_str("boiled away"), py_str("boils away"), py_str("caught on the bottom"), py_str("stuck to the bottom"), py_str("saucepan"), py_str("in the pan"), py_str("in the pot")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("deseed"), py_str("de-seed"), py_str("seed"), py_str("seeds"), py_str("pith"), py_str("too spicy"), py_str("too hot"), py_str("less spicy"), py_str("mild"), py_str("milder"), py_str("cool down"), py_str("cool it"), py_str("flakes"), py_str("powder"), py_str("paste"), py_str("plant"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("seedling")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("in my eye"), py_str("in the eye"), py_str("in my eyes"), py_str("my eye"), py_str("my eyes"), py_str("eyeball"), py_str("eye is"), py_str("eyes are"), py_str("eye hurts"), py_str("eyes hurt"), py_str("stinging"), py_str("stings"), py_str("rubbed my eye"), py_str("rubbing my eye"), py_str("in her eye"), py_str("in his eye"), py_str("in their eye"), py_str("his eye"), py_str("her eye"), py_str("their eye")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("deseed"), py_str("de-seed"), py_str("pith"), py_str("how hot"), py_str("how spicy"), py_str("too spicy"), py_str("too hot"), py_str("plant"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("seedling"), py_str("store"), py_str("freeze"), py_str("keep"), py_str("dry them"), py_str("drying"), py_str("make chilli oil")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("chop"), py_str("deseed"), py_str("seed"), py_str("pith"), py_str("how hot"), py_str("how spicy"), py_str("too spicy"), py_str("less spicy"), py_str("store chillies"), py_str("freeze"), py_str("instead of"), py_str("substitute"), py_str("plant"), py_str("grow"), py_str("sow")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("how do i make"), py_str("how do you make"), py_str("recipe"), py_str("green beans"), py_str("runner beans"), py_str("broad beans"), py_str("soak"), py_str("soaking"), py_str("dried beans"), py_str("grow"), py_str("plant"), py_str("sow"), py_str("beans on toast"), py_str("side"), py_str("salad")]) }
    s_ENTRIES(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-keep")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chilli pepper"), py_str("jalapeno"), py_str("jalapenos"), py_str("habanero"), py_str("scotch bonnet")]), py_tuple(vec![py_str("use up"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("storage"), py_str("how long"), py_str("last"), py_str("left over"), py_str("leftover"), py_str("glut"), py_str("plenty of")])])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); _the_vegetables_or_the_pot(&[memory.clone(), understood.clone(), py_str("Loose in the fridge drawer they keep a week or two, and longer in a paper bag than in plastic: the bag holds the moisture that turns them soft. A glut is better frozen or dried than left to go wrinkly - both keep the heat - and a handful also becomes chilli oil, which is the fastest way to use up twenty at once."), py_func("_the_plans_keeping_answer", |args: &[Py]| _the_plans_keeping_answer(args))]) })); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&g_DISH(), &g_HEAT_WORDS()), &__data4()), &g_COOKED())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-pot-keep")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chilli con carne"), py_str("chilli beef")]), __data5()])); __args.push(py_func("_the_pot_keeps", |args: &[Py]| _the_pot_keeps(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str(" pot "), py_str("pot of"), py_str("a pot"), py_str("batch"), py_str("cooked"), py_str("made"), py_str("leftover"), py_str("left over"), py_str("con carne"), py_str(" pan "), py_str("pan of"), py_str("a pan")])), ("avoid".to_string(), __data6()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("curry-heat")); __args.push(py_tuple(vec![__data7(), g__HEAT_WORDS()])); __args.push(py_func("_curry_heat", |args: &[Py]| _curry_heat(args))); __kws.extend(vec![("avoid".to_string(), __data8()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("curry-yoghurt")); __args.push(py_tuple(vec![py_tuple(vec![py_str("curry"), py_str("curries"), py_str("dal"), py_str("daal"), py_str("dhal"), py_str("tikka"), py_str("masala"), py_str("korma"), py_str("rogan josh"), py_str("balti")]), py_tuple(vec![py_str("yoghurt"), py_str("yogurt"), py_str("dahi"), py_str("cream"), py_str("coconut milk"), py_str("raita"), py_str("creme fraiche"), py_str("crème fraîche"), py_str("soured cream"), py_str("sour cream")])])); __args.push(py_str("Yes - a spoonful of yoghurt stirred in at the end is the usual finish: it cools the heat and rounds off a sharp or bitter edge. Take the pan off the heat first and stir the yoghurt into a spoonful of the sauce in a cup before it goes back in, so it does not split; full-fat or Greek yoghurt splits less than low-fat, and a splash of cream or coconut milk does the same job.")); __kws.extend(vec![("avoid".to_string(), __data9()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-freeze")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chilli pepper"), py_str("jalapeno"), py_str("jalapenos"), py_str("habanero"), py_str("scotch bonnet")]), py_tuple(vec![py_str("freeze"), py_str("frozen"), py_str("freezing"), py_str("bag up"), py_str("ice cube")])])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); _the_vegetables_or_the_pot(&[memory.clone(), understood.clone(), py_str("Freeze them whole in a bag and slice them still frozen: they go soft, so they are for cooking rather than for a salad, but the heat survives a few months. Chopped into an ice-cube tray with a little oil is the other way, one cube per curry, and there is no need to blanch them first."), py_func("_the_cooked_dish_freezing_answer", |args: &[Py]| _the_cooked_dish_freezing_answer(args))]) })); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_DISH(), &__data10()), &g_COOKED())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-dry")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chilli pepper"), py_str("jalapeno"), py_str("jalapenos"), py_str("habanero"), py_str("scotch bonnet")]), py_tuple(vec![py_str("dry"), py_str("dried"), py_str("drying"), py_str("dehydrate"), py_str("dehydrator"), py_str("dry them out")])])); __args.push(py_str("Thread them on cotton and hang the string by a sunny window until they are leathery and rattle when you shake one, which takes two to three weeks in a warm room, or split them and dry them in a low oven at 60C with the door ajar for six to eight hours. Dry them whole and they keep a year in a jar; grind them for flakes once they are crisp.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_DISH(), &__data11())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-vegetable")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chilli pepper"), py_str("jalapeno"), py_str("jalapenos"), py_str("habanero"), py_str("scotch bonnet"), py_str("birds eye"), py_str("bird's eye")]), py_tuple(vec![py_str("chop"), py_str("chopped"), py_str("slice"), py_str("prepare"), py_str("prep"), py_str("how hot"), py_str("hot"), py_str("heat"), py_str("spicy"), py_str("strength"), py_str("how strong")])])); __args.push(py_str("Heat is in the variety before it is in the size: a jalapeno is mild enough to eat raw, a bird's eye or a scotch bonnet will heat a whole curry, and the same kind varies from one to the next, so taste a sliver before you commit to two. Chop them small and add them early for a rounded heat, or late and sliced for a sharp one; the white pith and the seeds carry most of it, so scraping them out takes the edge off without making it mild. Wash your hands straight after and keep them off your eyes, or wear gloves for the very hot ones: the oil stays on skin for hours and soap barely shifts it.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_DISH(), &__data12()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-eye")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chili"), py_str("chillis"), py_str("jalapeno"), py_str("jalapenos"), py_str("habanero"), py_str("scotch bonnet"), py_str("chilli oil"), py_str("chilli juice")]), __data13()])); __args.push(py_str("Don't rub it: rinse the eye with milk if you have some, then clean, cool water or saline for ten to fifteen minutes with the lid held open, and blink it through. The burn fades over an hour or so and the redness can last a day. Ring 111 or go to an eye casualty the same day if the pain is getting worse rather than better, if your sight is blurred, or if you wear a contact lens - take it out first. Wash your hands and the knife before you touch your face again, because the oil sits on skin for hours and soap barely shifts it.")); __kws.extend(vec![("avoid".to_string(), g_DISH())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-substitute")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chilli flakes"), py_str("chilli powder"), py_str("chilli sauce"), py_str("chili"), py_str("chilli paste"), py_str("cayenne"), py_str("paprika"), py_str("gochujang"), py_str("sriracha")]), py_tuple(vec![py_str("instead of"), py_str("substitute"), py_str("substitution"), py_str("swap"), py_str("replace"), py_str("alternative"), py_str("no chillies"), py_str("run out"), py_str("ran out"), py_str("out of"), py_str("stand in"), py_str("same as")])])); __args.push(py_str("Paprika plus a pinch of cayenne is the closest thing to chilli powder: about four parts paprika to one of cayenne, and smoked paprika if you want the depth back. For flakes, a teaspoon is roughly one fresh chilli, so half a teaspoon of cayenne or a chopped red pepper does the job, and a spoon of sriracha, gochujang or hot sauce adds heat and a little sweetness. Without any of those, black pepper and a little ginger give the bite even though the flavour is different.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_DISH(), &__data14()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-oil")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli oil"), py_str("chili oil")]), py_tuple(vec![py_str("make"), py_str("recipe"), py_str("how do i"), py_str("store"), py_str("keep"), py_str("last"), py_str("shelf life"), py_str("infuse"), py_str("chillies in"), py_str("with dried")])])); __args.push(py_str("Warm 200ml of a plain oil with four sliced chillies and a crushed garlic clove over the lowest heat for five minutes - it should barely bubble, because burnt chilli turns bitter - then let it cool completely before you bottle it. Keep it in the fridge and use it within a fortnight, since fresh garlic and fresh chilli in oil will grow things at room temperature; dried flakes in oil keep far longer, up to three months cold.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_DISH(), &__data15()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chilli-beans")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chili"), py_str("chilli con carne"), py_str("chili con carne"), py_str("chillies")]), py_tuple(vec![py_str("beans"), py_str("bean"), py_str("kidney beans"), py_str("kidney bean"), py_str("black beans"), py_str("pinto beans"), py_str("cannellini"), py_str("baked beans"), py_str("haricot"), py_str("tin of beans"), py_str("tinned beans")])])); __args.push(py_str("Beans are optional, and it is a regional thing rather than a rule: Tex-Mex chilli usually has none, and most home versions have kidney beans. If you want them, a 400g tin of kidney or black beans, drained and rinsed, goes in with the tomatoes so it simmers the last 30 minutes - about one tin to 500g of mince, and no longer, because they go floury. Without them, keep the mince at 500g and add an extra onion or a handful of red lentils with the stock so it does not go dry and sharp. Baked beans are the sweet one: they work, but they want less sugar and a splash of vinegar in the pot.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("chilli"), py_str("chili")])), ("avoid".to_string(), __data16()), ("guard".to_string(), py_func("_short_of_something_else", |args: &[Py]| _short_of_something_else(args))), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _the_vegetables_or_the_pot(args: &[Py]) -> Py {
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
    let mut vegetable_answer: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "vegetable_answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: vegetable_answer"))) };
    let mut pot_answer: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pot_answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pot_answer"))) };
    let mut pot: Py = Py::None;
    let _ = py_str("The vegetable's answer, or the cooked dish's when this line is only pointing at one.\n\n    \"im making a chilli tonight\", \"how long does it need\", and then \"can i freeze it\" was answered\n    \"Freeze them whole in a bag and slice them still frozen\", which is a bag of peppers, not a pot\n    of stew. The last line names nothing, so the thing being asked about is the dish the talk is\n    on, and the answer for a cooked dish is one this base already keeps: the dish's own plan\n    sentence, or the freezing table in `bot/knowledge/cooking_entries.py`. Both are imported\n    rather than said again here, and a chat with no dish in it still gets the vegetable's answer.\n    ");
    if py_truthy(&_a_chilli_in_this_line(&[memory.clone(), understood.clone()])) {
    return vegetable_answer.clone();
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _a_dish_in_the_chat(&[memory.clone(), understood.clone()]) } }) {
    pot = py_call(&pot_answer, &[memory.clone(), understood.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pot.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pot.clone(), "strip", &[]) } }) {
    return pot.clone();
    }
    }
    return vegetable_answer.clone();
    Py::None
}
pub fn _a_dish_in_the_chat(args: &[Py]) -> Py {
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
    let mut dishes: Py = Py::None;
    let _ = py_str("True when the conversation is on a dish, so \"it\" is a pot and not a bag of peppers.");
    dishes = pyrt::xrefs::xmodule("knowledge.dishes");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&dishes.clone(), "dish_for", &[memory.clone(), understood.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_plans_keeping_answer(args: &[Py]) -> Py {
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
    let mut dishes: Py = Py::None;
    let _ = py_str("The dish's own keeping sentence, for a pot the conversation is already on.");
    dishes = pyrt::xrefs::xmodule("knowledge.dishes");
    return mcall(&dishes.clone(), "follow_up", &[memory.clone(), understood.clone()]);
    Py::None
}
pub fn _the_cooked_dish_freezing_answer(args: &[Py]) -> Py {
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
    let mut freeze_cooked_answer: Py = Py::None;
    let _ = py_str("The freezing answer for a cooked dish, from the table that keeps them all.");
    freeze_cooked_answer = py_func("freeze_cooked_answer", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking_entries", "freeze_cooked_answer", args));
    return py_call(&freeze_cooked_answer, &[memory.clone(), understood.clone()]);
    Py::None
}
pub fn _a_chilli_in_this_line(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this line names the vegetable rather than leaning on a dish said earlier.");
    text = mcall(&pyrt::xrefs::xcall("knowledge.lookup", "said_in_this_line", &[understood.clone(), memory.clone()]), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CHILLI_IN_THE_LINE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _curry_heat(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The dish's place on the menu's scale, or the whole order when the line names no dish.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    said = mcall(&py_str(" "), "join", &[words.clone()]);
    flat = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    flat = stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), flat.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_FAULT_IN_THE_DISH()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&flat.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[g_CURRY_HEAT()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    names = __unpk3[0].clone();
    place = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &name.clone()))) {
    return op_modulo(&py_str("A %s is %s. Mildest to hottest the usual menu runs %s, so ask for it mild and the kitchen leaves the chilli out - and if you want heat with flavour, step one up the list rather than two."), &py_tuple(vec![name.clone(), place.clone(), g_CURRY_ORDER_LINE()]));
    }
    }
    }
    return op_modulo(&py_str("Mildest to hottest the usual takeaway menu runs %s. A korma or pasanda has no chilli heat at all, a tikka masala is mild, and a jalfrezi and up are the hot end; ask for any of them mild and the kitchen makes it without the chilli."), &g_CURRY_ORDER_LINE());
    Py::None
}
pub fn _short_of_something_else(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut near: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when the line says what it has run out of and that thing is not a bean.");
    let __iter1 = py_iter_or_raise(&mcall(&g__SHORTAGE(), "finditer", &[pyrt::xrefs::xcall("knowledge.lookup", "said_text", &[understood.clone()])]));
    for __item in __iter1 {
    match_ = __item.clone();
    near = slice_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[]), &Py::None, &Py::Int(3), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "startswith", &[g__BEAN_STEMS()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_pot_keeps(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The pot's keeping clock, or nothing when the line asks how long a pan takes to cook.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g_A_COOK_CLOCK_VERB(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_KEEPING_CUE(), "search", &[said.clone()]))) } }) {
    return py_str("");
    }
    return g_A_POT_OF_CHILLI_KEEPS();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.chilli".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "A_COOK_CLOCK_VERB" => Some(g_A_COOK_CLOCK_VERB()),
            "A_KEEPING_CUE" => Some(g_A_KEEPING_CUE()),
            "A_POT_OF_CHILLI_KEEPS" => Some(g_A_POT_OF_CHILLI_KEEPS()),
            "COOKED" => Some(g_COOKED()),
            "CURRY_HEAT" => Some(g_CURRY_HEAT()),
            "CURRY_NAMES" => Some(g_CURRY_NAMES()),
            "CURRY_ORDER_LINE" => Some(g_CURRY_ORDER_LINE()),
            "DISH" => Some(g_DISH()),
            "ENTRIES" => Some(g_ENTRIES()),
            "HEAT_WORDS" => Some(g_HEAT_WORDS()),
            "_A_FAULT_IN_THE_DISH" => Some(g__A_FAULT_IN_THE_DISH()),
            "_BEAN_STEMS" => Some(g__BEAN_STEMS()),
            "_CHILLI_IN_THE_LINE" => Some(g__CHILLI_IN_THE_LINE()),
            "_HEAT_WORDS" => Some(g__HEAT_WORDS()),
            "_SHORTAGE" => Some(g__SHORTAGE()),
            "_a_chilli_in_this_line" => Some(py_func("_a_chilli_in_this_line", |args: &[Py]| _a_chilli_in_this_line(args))),
            "_a_dish_in_the_chat" => Some(py_func("_a_dish_in_the_chat", |args: &[Py]| _a_dish_in_the_chat(args))),
            "_curry_heat" => Some(py_func("_curry_heat", |args: &[Py]| _curry_heat(args))),
            "_short_of_something_else" => Some(py_func("_short_of_something_else", |args: &[Py]| _short_of_something_else(args))),
            "_the_cooked_dish_freezing_answer" => Some(py_func("_the_cooked_dish_freezing_answer", |args: &[Py]| _the_cooked_dish_freezing_answer(args))),
            "_the_plans_keeping_answer" => Some(py_func("_the_plans_keeping_answer", |args: &[Py]| _the_plans_keeping_answer(args))),
            "_the_pot_keeps" => Some(py_func("_the_pot_keeps", |args: &[Py]| _the_pot_keeps(args))),
            "_the_vegetables_or_the_pot" => Some(py_func("_the_vegetables_or_the_pot", |args: &[Py]| _the_vegetables_or_the_pot(args))),
            "entry" => Some(g_entry()),
            "said_in_this_line" => Some(g_said_in_this_line()),
            "lookup" => Some(pyrt::xrefs::xmodule("knowledge.lookup")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "A_COOK_CLOCK_VERB" => s_A_COOK_CLOCK_VERB(v),
            "A_KEEPING_CUE" => s_A_KEEPING_CUE(v),
            "A_POT_OF_CHILLI_KEEPS" => s_A_POT_OF_CHILLI_KEEPS(v),
            "COOKED" => s_COOKED(v),
            "CURRY_HEAT" => s_CURRY_HEAT(v),
            "CURRY_NAMES" => s_CURRY_NAMES(v),
            "CURRY_ORDER_LINE" => s_CURRY_ORDER_LINE(v),
            "DISH" => s_DISH(v),
            "ENTRIES" => s_ENTRIES(v),
            "HEAT_WORDS" => s_HEAT_WORDS(v),
            "_A_FAULT_IN_THE_DISH" => s__A_FAULT_IN_THE_DISH(v),
            "_BEAN_STEMS" => s__BEAN_STEMS(v),
            "_CHILLI_IN_THE_LINE" => s__CHILLI_IN_THE_LINE(v),
            "_HEAT_WORDS" => s__HEAT_WORDS(v),
            "_SHORTAGE" => s__SHORTAGE(v),
            "entry" => s_entry(v),
            "said_in_this_line" => s_said_in_this_line(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.chilli", module_obj);
    pyrt::xrefs::reg_fn("knowledge.chilli", "_the_vegetables_or_the_pot", py_func("_the_vegetables_or_the_pot", |args: &[Py]| _the_vegetables_or_the_pot(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_a_dish_in_the_chat", py_func("_a_dish_in_the_chat", |args: &[Py]| _a_dish_in_the_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_the_plans_keeping_answer", py_func("_the_plans_keeping_answer", |args: &[Py]| _the_plans_keeping_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_the_cooked_dish_freezing_answer", py_func("_the_cooked_dish_freezing_answer", |args: &[Py]| _the_cooked_dish_freezing_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_a_chilli_in_this_line", py_func("_a_chilli_in_this_line", |args: &[Py]| _a_chilli_in_this_line(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_curry_heat", py_func("_curry_heat", |args: &[Py]| _curry_heat(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_short_of_something_else", py_func("_short_of_something_else", |args: &[Py]| _short_of_something_else(args)));
    pyrt::xrefs::reg_fn("knowledge.chilli", "_the_pot_keeps", py_func("_the_pot_keeps", |args: &[Py]| _the_pot_keeps(args)));
    pyrt::xrefs::reg_access("knowledge.chilli", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.chilli", "A_COOK_CLOCK_VERB", g_A_COOK_CLOCK_VERB, s_A_COOK_CLOCK_VERB);
    pyrt::xrefs::reg_access("knowledge.chilli", "A_KEEPING_CUE", g_A_KEEPING_CUE, s_A_KEEPING_CUE);
    pyrt::xrefs::reg_access("knowledge.chilli", "A_POT_OF_CHILLI_KEEPS", g_A_POT_OF_CHILLI_KEEPS, s_A_POT_OF_CHILLI_KEEPS);
    pyrt::xrefs::reg_access("knowledge.chilli", "COOKED", g_COOKED, s_COOKED);
    pyrt::xrefs::reg_access("knowledge.chilli", "CURRY_HEAT", g_CURRY_HEAT, s_CURRY_HEAT);
    pyrt::xrefs::reg_access("knowledge.chilli", "CURRY_NAMES", g_CURRY_NAMES, s_CURRY_NAMES);
    pyrt::xrefs::reg_access("knowledge.chilli", "CURRY_ORDER_LINE", g_CURRY_ORDER_LINE, s_CURRY_ORDER_LINE);
    pyrt::xrefs::reg_access("knowledge.chilli", "DISH", g_DISH, s_DISH);
    pyrt::xrefs::reg_access("knowledge.chilli", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.chilli", "HEAT_WORDS", g_HEAT_WORDS, s_HEAT_WORDS);
    pyrt::xrefs::reg_access("knowledge.chilli", "_A_FAULT_IN_THE_DISH", g__A_FAULT_IN_THE_DISH, s__A_FAULT_IN_THE_DISH);
    pyrt::xrefs::reg_access("knowledge.chilli", "_BEAN_STEMS", g__BEAN_STEMS, s__BEAN_STEMS);
    pyrt::xrefs::reg_access("knowledge.chilli", "_CHILLI_IN_THE_LINE", g__CHILLI_IN_THE_LINE, s__CHILLI_IN_THE_LINE);
    pyrt::xrefs::reg_access("knowledge.chilli", "_HEAT_WORDS", g__HEAT_WORDS, s__HEAT_WORDS);
    pyrt::xrefs::reg_access("knowledge.chilli", "_SHORTAGE", g__SHORTAGE, s__SHORTAGE);
    pyrt::xrefs::reg_access("knowledge.chilli", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.chilli", "said_in_this_line", g_said_in_this_line, s_said_in_this_line);
}
