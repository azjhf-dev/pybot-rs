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
thread_local! { static G_CAKE_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_EGGS_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_EGGS_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_BASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_DAY_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_DECOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_DECOR_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_DOUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_DOUBLE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_SIZE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAKE_SIZE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUPCAKE_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUPCAKE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EGG_BATTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EGG_MERINGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EGG_PANCAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EGG_SKIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EGG_SPONGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_CAKE_AVOID() -> Py { ensure_init(); G_CAKE_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_AVOID(v: Py) { ensure_init(); G_CAKE_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_EGGS_AVOID() -> Py { ensure_init(); G_CAKE_EGGS_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_EGGS_AVOID(v: Py) { ensure_init(); G_CAKE_EGGS_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_EGGS_KEY() -> Py { ensure_init(); G_CAKE_EGGS_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_EGGS_KEY(v: Py) { ensure_init(); G_CAKE_EGGS_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_KEY() -> Py { ensure_init(); G_CAKE_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_KEY(v: Py) { ensure_init(); G_CAKE_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_BASE() -> Py { ensure_init(); G__CAKE_BASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_BASE(v: Py) { ensure_init(); G__CAKE_BASE.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_CLOCK() -> Py { ensure_init(); G__CAKE_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_CLOCK(v: Py) { ensure_init(); G__CAKE_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_COUNTS() -> Py { ensure_init(); G__CAKE_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_COUNTS(v: Py) { ensure_init(); G__CAKE_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_DAY_BEFORE() -> Py { ensure_init(); G__CAKE_DAY_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_DAY_BEFORE(v: Py) { ensure_init(); G__CAKE_DAY_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_DECOR() -> Py { ensure_init(); G__CAKE_DECOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_DECOR(v: Py) { ensure_init(); G__CAKE_DECOR.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_DECOR_WORDS() -> Py { ensure_init(); G__CAKE_DECOR_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_DECOR_WORDS(v: Py) { ensure_init(); G__CAKE_DECOR_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_DOUBLE() -> Py { ensure_init(); G__CAKE_DOUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_DOUBLE(v: Py) { ensure_init(); G__CAKE_DOUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_DOUBLE_WORDS() -> Py { ensure_init(); G__CAKE_DOUBLE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_DOUBLE_WORDS(v: Py) { ensure_init(); G__CAKE_DOUBLE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_NUMBERS() -> Py { ensure_init(); G__CAKE_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_NUMBERS(v: Py) { ensure_init(); G__CAKE_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_SHAPES() -> Py { ensure_init(); G__CAKE_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_SHAPES(v: Py) { ensure_init(); G__CAKE_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_SIZE() -> Py { ensure_init(); G__CAKE_SIZE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_SIZE(v: Py) { ensure_init(); G__CAKE_SIZE.with(|c| *c.borrow_mut() = v); }
pub fn g__CAKE_SIZE_WORDS() -> Py { ensure_init(); G__CAKE_SIZE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAKE_SIZE_WORDS(v: Py) { ensure_init(); G__CAKE_SIZE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CUPCAKE_KEEP() -> Py { ensure_init(); G__CUPCAKE_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUPCAKE_KEEP(v: Py) { ensure_init(); G__CUPCAKE_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g__CUPCAKE_WORDS() -> Py { ensure_init(); G__CUPCAKE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUPCAKE_WORDS(v: Py) { ensure_init(); G__CUPCAKE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__EGG_BATTER() -> Py { ensure_init(); G__EGG_BATTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EGG_BATTER(v: Py) { ensure_init(); G__EGG_BATTER.with(|c| *c.borrow_mut() = v); }
pub fn g__EGG_MERINGUE() -> Py { ensure_init(); G__EGG_MERINGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EGG_MERINGUE(v: Py) { ensure_init(); G__EGG_MERINGUE.with(|c| *c.borrow_mut() = v); }
pub fn g__EGG_PANCAKE() -> Py { ensure_init(); G__EGG_PANCAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EGG_PANCAKE(v: Py) { ensure_init(); G__EGG_PANCAKE.with(|c| *c.borrow_mut() = v); }
pub fn g__EGG_SKIP() -> Py { ensure_init(); G__EGG_SKIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EGG_SKIP(v: Py) { ensure_init(); G__EGG_SKIP.with(|c| *c.borrow_mut() = v); }
pub fn g__EGG_SPONGE() -> Py { ensure_init(); G__EGG_SPONGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EGG_SPONGE(v: Py) { ensure_init(); G__EGG_SPONGE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How many people a party cake feeds, and the tin that covers the number asked for.\n\nThis holds the sizing ladder for the `cake-size` row in dishes.py: the shapes a party cake comes\nin, how many each one feeds, and the arithmetic that picks the fewest tins for the number in the\nline. It lives here rather than in cooking.py because that file is at the repository's 1 MB\nlimit, and a number is the part of a cake question that will keep growing.\n\nIt also owns the row's key and avoid list, and the reply function that reads them.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s__CAKE_SHAPES(py_tuple(vec![py_tuple(vec![py_str("20cm tin"), py_str("20cm tins"), Py::Int(8), Py::Int(10)]), py_tuple(vec![py_str("23cm tin"), py_str("23cm tins"), Py::Int(12), Py::Int(14)]), py_tuple(vec![py_str("25cm tin"), py_str("25cm tins"), Py::Int(18), Py::Int(20)]), py_tuple(vec![py_str("30 x 20cm traybake"), py_str("30 x 20cm traybakes"), Py::Int(15), Py::Int(18)])]));
    fn __data1() -> Py { py_dict(vec![(py_str("a dozen"), Py::Int(12)), (py_str("dozen"), Py::Int(12)), (py_str("ten"), Py::Int(10)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("eighteen"), Py::Int(18)), (py_str("twenty"), Py::Int(20)), (py_str("twenty five"), Py::Int(25)), (py_str("thirty"), Py::Int(30)), (py_str("thirty five"), Py::Int(35)), (py_str("forty"), Py::Int(40)), (py_str("forty five"), Py::Int(45)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90)), (py_str("a hundred"), Py::Int(100)), (py_str("hundred"), Py::Int(100))]) }
    s__CAKE_COUNTS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("minute"), py_str("minutes"), py_str("min"), py_str("mins"), py_str("hour"), py_str("hours"), py_str("cm"), py_str("g"), py_str("kg"), py_str("degrees"), py_str("second"), py_str("seconds"), py_str("percent")]) }
    s__CAKE_CLOCK(__data2());
    s__CAKE_DAY_BEFORE(py_str("Bake it the day before, wrap it well and fill it with buttercream on the day."));
    s__CAKE_SIZE(py_str("Size it by the tin: 20cm serves 8 to 10, 23cm 12 to 14, 25cm 18 to 20, and a 30 x 20cm traybake cut into squares feeds 15 to 18."));
    s__CAKE_DOUBLE(py_str("To double a recipe double every weight and split it between two tins with the same heat and time."));
    s__CAKE_DECOR(py_str("Buttercream or icing between the layers and on top is all a party cake needs: spread it warm, chill the cake first, and keep it plain or add a handful of fruit or sweets."));
    s__CAKE_BASE(py_str("A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously."));
    fn __data3() -> Py { py_tuple(vec![py_str("how much cake"), py_str("size"), py_str("sized"), py_str("big"), py_str("bigger"), py_str("serve"), py_str("serving"), py_str("people"), py_str("guests"), py_str("crowd"), py_str("portions"), py_str("slices"), py_str("serves"), py_str("serve"), py_str("feeds"), py_str("feed"), py_str("heads")]) }
    s__CAKE_SIZE_WORDS(__data3());
    s__CUPCAKE_WORDS(py_tuple(vec![py_str("cupcake"), py_str("cupcakes"), py_str("cup cake"), py_str("cup cakes"), py_str("fairy cake"), py_str("fairy cakes")]));
    s__CUPCAKE_KEEP(py_str("Twelve to a tray and two a head, so one 12-hole tray covers six people: bake them at 180C (160C fan) for 12 to 18 minutes and ice them on the day."));
    s__CAKE_DOUBLE_WORDS(py_tuple(vec![py_str("double"), py_str("doubling"), py_str("halve"), py_str("halving"), py_str("half the"), py_str("scale up"), py_str("scaling up"), py_str("bigger batch"), py_str("twice as much"), py_str("two batches")]));
    fn __data4() -> Py { py_tuple(vec![py_str("ice"), py_str("icing"), py_str("iced"), py_str("decorate"), py_str("decorating"), py_str("decorated"), py_str("buttercream"), py_str("frost"), py_str("frosting"), py_str("fill it"), py_str("filling"), py_str("pipe"), py_str("piping"), py_str("writing on"), py_str("writing happy birthday")]) }
    s__CAKE_DECOR_WORDS(__data4());
    s__CAKE_NUMBERS(py_tuple(vec![py_str(""), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six")]));
    fn __data5() -> Py { py_tuple(vec![py_str("birthday"), py_str("party"), py_str("celebrate"), py_str("celebrating"), py_str("people"), py_str("guests"), py_str("crowd"), py_str("number"), py_str("scale"), py_str("scale up"), py_str("bigger"), py_str("double"), py_str("doubling"), py_str("halve"), py_str("how many"), py_str("serves"), py_str("serve"), py_str("feed"), py_str("serving"), py_str("slices"), py_str("portions")]) }
    s_CAKE_KEY(py_tuple(vec![py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("cupcake"), py_str("cupcakes"), py_str("fairy cakes")]), __data5()]));
    fn __data6() -> Py { py_tuple(vec![py_str("how long"), py_str("time"), py_str("temperature"), py_str("sink"), py_str("sank"), py_str("sunken"), py_str("stuck"), py_str("burn"), py_str("burnt"), py_str("nut"), py_str("nuts"), py_str("allergy"), py_str("gluten"), py_str("vegan"), py_str("egg"), py_str("eggs"), py_str("egg free"), py_str("eggless"), py_str("butter"), py_str("flour"), py_str("sugar"), py_str("cocoa"), py_str("vanilla"), py_str("what size tin"), py_str("which tin"), py_str("tin size"), py_str("how many tins"), py_str("clove"), py_str("cloves"), py_str("cinnamon"), py_str("nutmeg"), py_str("mixed spice"), py_str("allspice"), py_str("turmeric"), py_str("paprika"), py_str("cardamom"), py_str("saffron"), py_str("spice"), py_str("spices"), py_str("teaspoon"), py_str("tablespoon"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("freeze"), py_str("fridge"), py_str("calorie"), py_str("calories"), py_str("kcal"), py_str("cals")]) }
    s_CAKE_AVOID(__data6());
    s__EGG_BATTER(py_str("For batter, use four eggs to 200g plain flour and 300ml milk, with a pinch of salt, and rest it half an hour: the eggs are what make a Yorkshire rise, so don't cut them down. That fills a 12-hole tin."));
    s__EGG_PANCAKE(py_str("Two eggs to 150g plain flour and 300ml milk makes about eight thin pancakes, enough for four people. More eggs make them richer and more likely to tear; they don't make more pancakes."));
    s__EGG_MERINGUE(py_str("One egg white to 50g caster sugar, so a meringue nest for four takes two egg whites: whisk them to stiff peaks, then add the sugar a spoon at a time until it feels smooth between your fingers."));
    s__EGG_SPONGE(py_str("Weigh the eggs in their shells and use the same weight of soft butter, caster sugar and self-raising flour: three eggs fills a 20cm two-layer sandwich tin (about 175g of each), and four fills a 23cm one. The eggs are the measure, so there is no fixed count - a bigger bake starts by weighing more eggs."));
    s__EGG_SKIP(py_str("Not for the mix: the egg binds a cake and helps it rise, and in most sponges 60g of yoghurt, half a mashed banana or a tablespoon of ground flax in three of water does the same job. Egg-free sponges are a little more crumbly and stale sooner, so eat them within a day or two, and don't expect a swap to work in a meringue or a genoise, which are made of egg."));
    fn __data7() -> Py { py_tuple(vec![py_str("instead"), py_str("instead of"), py_str("substitute"), py_str("substitutes"), py_str("substituting"), py_str("swap"), py_str("swaps"), py_str("swapping"), py_str("replace"), py_str("replaces"), py_str("replacing"), py_str("replacement"), py_str("without"), py_str("egg free"), py_str("egg-free"), py_str("eggless"), py_str("vegan"), py_str("flax"), py_str("flaxseed"), py_str("allergy"), py_str("allergic")]) }
    s_CAKE_EGGS_AVOID(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("sponge cake"), py_str("victoria sponge"), py_str("madeira"), py_str("birthday cake"), py_str("sandwich cake"), py_str("yorkshire pudding"), py_str("batter"), py_str("pancakes"), py_str("pancake"), py_str("meringue"), py_str("meringues"), py_str("cupcakes"), py_str("cupcake")]) }
    s_CAKE_EGGS_KEY(py_tuple(vec![__data8(), py_tuple(vec![py_str("eggs"), py_str("egg"), py_str("how many eggs")])]));
}
pub fn _cake_count(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut value: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How many people the cake in this line is for, or 0 when the line names no number.");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("\\b(\\d{1,3})\\s+(?:people|guests|kids|children|adults|persons|portions|slices|of us|head)\\b"), py_str("\\bfor\\s+(\\d{1,3})\\b"), py_str("\\b(?:feeds|feed|serves|serve)\\s+(\\d{1,3})\\b"), py_str("\\bparty\\s+of\\s+(\\d{1,3})\\b")]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "finditer", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    match_ = __item.clone();
    after = mcall(&mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]), "split", &[py_str(" ")]);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__CAKE_CLOCK(), &mcall(&geti(&after.clone(), &Py::Int(0)), "strip", &[py_str(",.")]))) } }) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    let __iter3 = py_iter_or_raise(&mcall(&g__CAKE_COUNTS(), "items", &[]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    words = __unpk4[0].clone();
    value = __unpk4[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &words.clone()), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[value.clone()]);
    }
    }
    return if py_truthy(&found.clone()) { call_builtin("max", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    Py::None
}
pub fn _cake_asks(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line is one of these kinds of ask - by whole words, so \"iced\" is not \"ice\".");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _cake_tin_size(args: &[Py]) -> Py {
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
    let mut shape: Py = Py::None;
    let mut size: Py = Py::None;
    let _ = py_str("The tin a line names on its own - \"how many will a 25cm cake serve\" - or None.\n\n    Where a line names a tin and no headcount, the number that tin feeds is the answer, so it\n    leads the reply instead of the reader being told to read a table for four shapes.\n    ");
    text = op_add(&op_add(&py_str(" "), &mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str(" cm"), py_str("cm")])), &py_str(" "));
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b30\\s*x\\s*20\\s*cm\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__CAKE_SHAPES(), &Py::Int(3));
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&g__CAKE_SHAPES(), &Py::None, &Py::Int(3), &Py::None));
    for __item in __iter1 {
    shape = __item.clone();
    size = geti(&mcall(&geti(&shape.clone(), &Py::Int(0)), "split", &[py_str("cm")]), &Py::Int(0));
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &size.clone()), &py_str("\\s*cm\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return shape.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _cake_tins(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut count: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut best: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut high: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut low: Py = Py::None;
    let mut n: Py = Py::None;
    let mut n20: Py = Py::None;
    let mut n23: Py = Py::None;
    let mut n25: Py = Py::None;
    let mut ntr: Py = Py::None;
    let mut score: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut tins: Py = Py::None;
    let _ = py_str("The fewest tins that still feed this many people, and what they feed between them.\n\n    Fewest tins first, then fewest kinds of tin, because three of the same tin is a plan someone\n    can shop for and two tins and a traybake is not. A cake has to cover the number at the\n    smallest serve it gives (18cm serves 18 to 20), never at the top of the range.\n    ");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), Py::Int(9)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    n20 = __item.clone();
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), Py::Int(9)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    n23 = __item.clone();
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), Py::Int(9)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    n25 = __item.clone();
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), Py::Int(9)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    ntr = __item.clone();
    counts = py_tuple(vec![n20.clone(), n23.clone(), n25.clone(), ntr.clone()]);
    tins = call_builtin("sum", &[counts.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tins.clone())); if py_truthy(&__b) { __b } else { op_gt(&tins.clone(), &Py::Int(12)) } }) {
    continue;
    }
    low = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[counts.clone(), g__CAKE_SHAPES()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk9922 = py_unpack(&__item.clone(), 2, None); let mut n: Py = __unpk9922[0].clone(); let mut shape: Py = __unpk9922[1].clone(); __out.push(op_mul(&n.clone(), &geti(&shape.clone(), &Py::Int(2)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    high = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[counts.clone(), g__CAKE_SHAPES()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk355 = py_unpack(&__item.clone(), 2, None); let mut n: Py = __unpk355[0].clone(); let mut shape: Py = __unpk355[1].clone(); __out.push(op_mul(&n.clone(), &geti(&shape.clone(), &Py::Int(3)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&high.clone(), &count.clone())) {
    continue;
    }
    kinds = call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&counts.clone()) { let mut n: Py = __item.clone(); if py_truthy(&n.clone()) {  __out.push(n.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    score = py_tuple(vec![tins.clone(), kinds.clone(), op_sub(&high.clone(), &count.clone())]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&score.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![score.clone(), counts.clone(), low.clone(), high.clone()]);
    }
    }
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn cake_plan(args: &[Py]) -> Py {
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
    let mut _score: Py = Py::None;
    let mut count: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut fit: Py = Py::None;
    let mut high: Py = Py::None;
    let mut i: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut n: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut tins: Py = Py::None;
    let mut trays: Py = Py::None;
    let _ = py_str("The party cake answer: the tin for the number in the line, or the table when it names none.\n\n    Most of the lines that find this row are a birthday cake with no number in them, and those\n    keep the advice and the table. A line that does name a number gets the tin named back, which\n    is the answer to the question it asked.\n    ");
    line = op_add(&op_add(&py_str(" "), &call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    count = _cake_count(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    shape = _cake_tin_size(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_is(&shape.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(py_contains(&geti(&shape.clone(), &Py::Int(0)), &py_str("traybake")))) {
    said = { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&geti(&shape.clone(), &Py::Int(0)), -1, "")); __s.push_str(" cut into squares feeds "); __s.push_str(&fstr(&geti(&shape.clone(), &Py::Int(2)), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&geti(&shape.clone(), &Py::Int(3)), -1, "")); __s.push_str(". "); py_string(__s) };
    } else {
    said = { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&mcall(&geti(&shape.clone(), &Py::Int(0)), "replace", &[py_str(" tin"), py_str("")]), -1, "")); __s.push_str(" two-layer sponge feeds "); __s.push_str(&fstr(&geti(&shape.clone(), &Py::Int(2)), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&geti(&shape.clone(), &Py::Int(3)), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return mcall(&mcall(&py_str(" "), "join", &[py_tuple(vec![said.clone(), g__CAKE_DAY_BEFORE(), g__CAKE_DOUBLE()])]), "strip", &[]);
    }
    if py_truthy(&_cake_asks(&[line.clone(), g__CAKE_DOUBLE_WORDS()])) {
    return mcall(&g__CAKE_DOUBLE(), "strip", &[]);
    }
    if py_truthy(&_cake_asks(&[line.clone(), g__CAKE_DECOR_WORDS()])) {
    return mcall(&g__CAKE_DECOR(), "strip", &[]);
    }
    if py_truthy(&_cake_asks(&[line.clone(), g__CUPCAKE_WORDS()])) {
    return g__CUPCAKE_KEEP();
    }
    if py_truthy(&_cake_asks(&[line.clone(), g__CAKE_SIZE_WORDS()])) {
    return mcall(&mcall(&py_str(" "), "join", &[py_tuple(vec![g__CAKE_SIZE(), g__CAKE_DAY_BEFORE(), g__CAKE_DOUBLE()])]), "strip", &[]);
    }
    return g__CAKE_BASE();
    }
    if py_truthy(&_cake_asks(&[line.clone(), g__CUPCAKE_WORDS()])) {
    trays = op_neg(&op_floordiv(&op_mul(&op_neg(&count.clone()), &Py::Int(2)), &Py::Int(24)));
    said = { let mut __s = String::new(); __s.push_str(&fstr(&op_mul(&count.clone(), &Py::Int(2)), -1, "")); __s.push_str(" cupcakes for "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" people: two a head, baked at 180C for 12 to 18 minutes and iced on the day."); py_string(__s) };
    if py_truthy(&op_gt(&trays.clone(), &Py::Int(1))) {
    let __aug1 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" That is "); __s.push_str(&fstr(&trays.clone(), -1, "")); __s.push_str(" trays of 24, and a muffin tray takes 12 at a time."); py_string(__s) });
    said = __aug1;
    }
    return said.clone();
    }
    counts = Py::None;
    low = Py::None;
    high = Py::None;
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[if py_truthy(&op_le(&count.clone(), &Py::Int(20))) { slice_or_raise(&g__CAKE_SHAPES(), &Py::None, &Py::Int(3), &Py::None) } else { py_tuple(vec![]) }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    shape = __unpk3[1].clone();
    if py_truthy(&op_le(&count.clone(), &geti(&shape.clone(), &Py::Int(3)))) {
    counts = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(4)]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&i.clone(), &index.clone()))) { Py::Int(1) } else { Py::Int(0) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let __unpk4 = py_unpack(&py_tuple(vec![geti(&shape.clone(), &Py::Int(2)), geti(&shape.clone(), &Py::Int(3))]), 2, None);
    low = __unpk4[0].clone();
    high = __unpk4[1].clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&counts.clone(), &Py::None))) {
    fit = _cake_tins(&[count.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&fit.clone()))) {
    return Py::None;
    }
    let __unpk5 = py_unpack(&fit.clone(), 4, None);
    _score = __unpk5[0].clone();
    counts = __unpk5[1].clone();
    low = __unpk5[2].clone();
    high = __unpk5[3].clone();
    }
    tins = call_builtin("sum", &[counts.clone()]).unwrap_or_else(|e| exc::raise(e));
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[counts.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1334 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk1334[0].clone(); let mut n: Py = __unpk1334[1].clone(); if py_truthy(&n.clone()) {  __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&geti(&g__CAKE_NUMBERS(), &n.clone()), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&n.clone(), &Py::Int(1)))) { geti(&geti(&g__CAKE_SHAPES(), &i.clone()), &Py::Int(0)) } else { geti(&geti(&g__CAKE_SHAPES(), &i.clone()), &Py::Int(1)) }, -1, "")); py_string(__s) }); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&tins.clone(), &Py::Int(1)))) {
    shape = geti(&mcall(&geti(&parts.clone(), &Py::Int(0)), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(1));
    if py_truthy(&Py::Bool(py_contains(&shape.clone(), &py_str("traybake")))) {
    said = { let mut __s = String::new(); __s.push_str("One "); __s.push_str(&fstr(&shape.clone(), -1, "")); __s.push_str(" cut into squares feeds "); __s.push_str(&fstr(&low.clone(), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&high.clone(), -1, "")); __s.push_str(", which covers "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    } else {
    said = { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&mcall(&shape.clone(), "replace", &[py_str(" tin"), py_str("")]), -1, "")); __s.push_str(" two-layer sponge feeds "); __s.push_str(&fstr(&low.clone(), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&high.clone(), -1, "")); __s.push_str(", so one tin covers "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    } else {
    kinds = call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e));
    said = if py_truthy(&op_gt(&kinds.clone(), &Py::Int(1))) { { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&py_str(", "), "join", &[slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&geti(&parts.clone(), &op_neg(&Py::Int(1))), -1, "")); py_string(__s) } } else { geti(&parts.clone(), &Py::Int(0)) };
    said = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&if py_truthy(&op_gt(&tins.clone(), &Py::Int(1))) { py_str("feed") } else { py_str("feeds") }, -1, "")); __s.push_str(" "); __s.push_str(&fstr(&low.clone(), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&high.clone(), -1, "")); __s.push_str(" between them, which covers "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return mcall(&mcall(&py_str(" "), "join", &[py_tuple(vec![said.clone(), g__CAKE_DAY_BEFORE(), py_str("Double every weight and split it between the tins with the same heat and time.")])]), "strip", &[]);
    Py::None
}
pub fn cake_eggs(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("How many eggs go in a bake: the count is the weight of everything else.");
    line = op_add(&op_add(&py_str(" "), &call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&_cake_asks(&[line.clone(), py_tuple(vec![py_str("needed"), py_str("necessary"), py_str("essential"), py_str("required"), py_str("have to"), py_str("can i skip"), py_str("skip the egg"), py_str("skip eggs"), py_str("do without")])])) {
    return g__EGG_SKIP();
    }
    if py_truthy(&_cake_asks(&[line.clone(), py_tuple(vec![py_str("yorkshire"), py_str("batter"), py_str("toad in the hole"), py_str("popover")])])) {
    return g__EGG_BATTER();
    }
    if py_truthy(&_cake_asks(&[line.clone(), py_tuple(vec![py_str("pancake"), py_str("pancakes"), py_str("crepe"), py_str("crepes"), py_str("drop scone")])])) {
    return g__EGG_PANCAKE();
    }
    if py_truthy(&_cake_asks(&[line.clone(), py_tuple(vec![py_str("meringue"), py_str("meringues"), py_str("pavlova")])])) {
    return g__EGG_MERINGUE();
    }
    return g__EGG_SPONGE();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.cakes".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "CAKE_AVOID" => Some(g_CAKE_AVOID()),
            "CAKE_EGGS_AVOID" => Some(g_CAKE_EGGS_AVOID()),
            "CAKE_EGGS_KEY" => Some(g_CAKE_EGGS_KEY()),
            "CAKE_KEY" => Some(g_CAKE_KEY()),
            "_CAKE_BASE" => Some(g__CAKE_BASE()),
            "_CAKE_CLOCK" => Some(g__CAKE_CLOCK()),
            "_CAKE_COUNTS" => Some(g__CAKE_COUNTS()),
            "_CAKE_DAY_BEFORE" => Some(g__CAKE_DAY_BEFORE()),
            "_CAKE_DECOR" => Some(g__CAKE_DECOR()),
            "_CAKE_DECOR_WORDS" => Some(g__CAKE_DECOR_WORDS()),
            "_CAKE_DOUBLE" => Some(g__CAKE_DOUBLE()),
            "_CAKE_DOUBLE_WORDS" => Some(g__CAKE_DOUBLE_WORDS()),
            "_CAKE_NUMBERS" => Some(g__CAKE_NUMBERS()),
            "_CAKE_SHAPES" => Some(g__CAKE_SHAPES()),
            "_CAKE_SIZE" => Some(g__CAKE_SIZE()),
            "_CAKE_SIZE_WORDS" => Some(g__CAKE_SIZE_WORDS()),
            "_CUPCAKE_KEEP" => Some(g__CUPCAKE_KEEP()),
            "_CUPCAKE_WORDS" => Some(g__CUPCAKE_WORDS()),
            "_EGG_BATTER" => Some(g__EGG_BATTER()),
            "_EGG_MERINGUE" => Some(g__EGG_MERINGUE()),
            "_EGG_PANCAKE" => Some(g__EGG_PANCAKE()),
            "_EGG_SKIP" => Some(g__EGG_SKIP()),
            "_EGG_SPONGE" => Some(g__EGG_SPONGE()),
            "_cake_asks" => Some(py_func("_cake_asks", |args: &[Py]| _cake_asks(args))),
            "_cake_count" => Some(py_func("_cake_count", |args: &[Py]| _cake_count(args))),
            "_cake_tin_size" => Some(py_func("_cake_tin_size", |args: &[Py]| _cake_tin_size(args))),
            "_cake_tins" => Some(py_func("_cake_tins", |args: &[Py]| _cake_tins(args))),
            "cake_eggs" => Some(py_func("cake_eggs", |args: &[Py]| cake_eggs(args))),
            "cake_plan" => Some(py_func("cake_plan", |args: &[Py]| cake_plan(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "CAKE_AVOID" => s_CAKE_AVOID(v),
            "CAKE_EGGS_AVOID" => s_CAKE_EGGS_AVOID(v),
            "CAKE_EGGS_KEY" => s_CAKE_EGGS_KEY(v),
            "CAKE_KEY" => s_CAKE_KEY(v),
            "_CAKE_BASE" => s__CAKE_BASE(v),
            "_CAKE_CLOCK" => s__CAKE_CLOCK(v),
            "_CAKE_COUNTS" => s__CAKE_COUNTS(v),
            "_CAKE_DAY_BEFORE" => s__CAKE_DAY_BEFORE(v),
            "_CAKE_DECOR" => s__CAKE_DECOR(v),
            "_CAKE_DECOR_WORDS" => s__CAKE_DECOR_WORDS(v),
            "_CAKE_DOUBLE" => s__CAKE_DOUBLE(v),
            "_CAKE_DOUBLE_WORDS" => s__CAKE_DOUBLE_WORDS(v),
            "_CAKE_NUMBERS" => s__CAKE_NUMBERS(v),
            "_CAKE_SHAPES" => s__CAKE_SHAPES(v),
            "_CAKE_SIZE" => s__CAKE_SIZE(v),
            "_CAKE_SIZE_WORDS" => s__CAKE_SIZE_WORDS(v),
            "_CUPCAKE_KEEP" => s__CUPCAKE_KEEP(v),
            "_CUPCAKE_WORDS" => s__CUPCAKE_WORDS(v),
            "_EGG_BATTER" => s__EGG_BATTER(v),
            "_EGG_MERINGUE" => s__EGG_MERINGUE(v),
            "_EGG_PANCAKE" => s__EGG_PANCAKE(v),
            "_EGG_SKIP" => s__EGG_SKIP(v),
            "_EGG_SPONGE" => s__EGG_SPONGE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.cakes", module_obj);
    pyrt::xrefs::reg_fn("knowledge.cakes", "_cake_count", py_func("_cake_count", |args: &[Py]| _cake_count(args)));
    pyrt::xrefs::reg_fn("knowledge.cakes", "_cake_asks", py_func("_cake_asks", |args: &[Py]| _cake_asks(args)));
    pyrt::xrefs::reg_fn("knowledge.cakes", "_cake_tin_size", py_func("_cake_tin_size", |args: &[Py]| _cake_tin_size(args)));
    pyrt::xrefs::reg_fn("knowledge.cakes", "_cake_tins", py_func("_cake_tins", |args: &[Py]| _cake_tins(args)));
    pyrt::xrefs::reg_fn("knowledge.cakes", "cake_plan", py_func("cake_plan", |args: &[Py]| cake_plan(args)));
    pyrt::xrefs::reg_fn("knowledge.cakes", "cake_eggs", py_func("cake_eggs", |args: &[Py]| cake_eggs(args)));
    pyrt::xrefs::reg_access("knowledge.cakes", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.cakes", "CAKE_AVOID", g_CAKE_AVOID, s_CAKE_AVOID);
    pyrt::xrefs::reg_access("knowledge.cakes", "CAKE_EGGS_AVOID", g_CAKE_EGGS_AVOID, s_CAKE_EGGS_AVOID);
    pyrt::xrefs::reg_access("knowledge.cakes", "CAKE_EGGS_KEY", g_CAKE_EGGS_KEY, s_CAKE_EGGS_KEY);
    pyrt::xrefs::reg_access("knowledge.cakes", "CAKE_KEY", g_CAKE_KEY, s_CAKE_KEY);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_BASE", g__CAKE_BASE, s__CAKE_BASE);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_CLOCK", g__CAKE_CLOCK, s__CAKE_CLOCK);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_COUNTS", g__CAKE_COUNTS, s__CAKE_COUNTS);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_DAY_BEFORE", g__CAKE_DAY_BEFORE, s__CAKE_DAY_BEFORE);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_DECOR", g__CAKE_DECOR, s__CAKE_DECOR);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_DECOR_WORDS", g__CAKE_DECOR_WORDS, s__CAKE_DECOR_WORDS);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_DOUBLE", g__CAKE_DOUBLE, s__CAKE_DOUBLE);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_DOUBLE_WORDS", g__CAKE_DOUBLE_WORDS, s__CAKE_DOUBLE_WORDS);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_NUMBERS", g__CAKE_NUMBERS, s__CAKE_NUMBERS);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_SHAPES", g__CAKE_SHAPES, s__CAKE_SHAPES);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_SIZE", g__CAKE_SIZE, s__CAKE_SIZE);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CAKE_SIZE_WORDS", g__CAKE_SIZE_WORDS, s__CAKE_SIZE_WORDS);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CUPCAKE_KEEP", g__CUPCAKE_KEEP, s__CUPCAKE_KEEP);
    pyrt::xrefs::reg_access("knowledge.cakes", "_CUPCAKE_WORDS", g__CUPCAKE_WORDS, s__CUPCAKE_WORDS);
    pyrt::xrefs::reg_access("knowledge.cakes", "_EGG_BATTER", g__EGG_BATTER, s__EGG_BATTER);
    pyrt::xrefs::reg_access("knowledge.cakes", "_EGG_MERINGUE", g__EGG_MERINGUE, s__EGG_MERINGUE);
    pyrt::xrefs::reg_access("knowledge.cakes", "_EGG_PANCAKE", g__EGG_PANCAKE, s__EGG_PANCAKE);
    pyrt::xrefs::reg_access("knowledge.cakes", "_EGG_SKIP", g__EGG_SKIP, s__EGG_SKIP);
    pyrt::xrefs::reg_access("knowledge.cakes", "_EGG_SPONGE", g__EGG_SPONGE, s__EGG_SPONGE);
}
