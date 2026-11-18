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
thread_local! { static G_COMPARING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIOD_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIOD_PREPOSITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SALES_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ASK_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_TO_BUY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BUY_THE_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAS_FAILED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_asks_where_a_shop_answers: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.whenbuy: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPARING() -> Py { ensure_init(); G_COMPARING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPARING(v: Py) { ensure_init(); G_COMPARING.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_CALENDAR() -> Py { ensure_init(); G_NOT_THE_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_CALENDAR(v: Py) { ensure_init(); G_NOT_THE_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIOD_NOUNS() -> Py { ensure_init(); G_PERIOD_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIOD_NOUNS(v: Py) { ensure_init(); G_PERIOD_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIOD_PREPOSITION() -> Py { ensure_init(); G_PERIOD_PREPOSITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIOD_PREPOSITION(v: Py) { ensure_init(); G_PERIOD_PREPOSITION.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_ASK() -> Py { ensure_init(); G_PRICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK(v: Py) { ensure_init(); G_PRICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SALES_CALENDAR() -> Py { ensure_init(); G_SALES_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SALES_CALENDAR(v: Py) { ensure_init(); G_SALES_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ASK() -> Py { ensure_init(); G_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ASK(v: Py) { ensure_init(); G_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ASK_SHAPE() -> Py { ensure_init(); G_TIME_ASK_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ASK_SHAPE(v: Py) { ensure_init(); G_TIME_ASK_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASK() -> Py { ensure_init(); G_WHEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); G_WHEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASKING() -> Py { ensure_init(); G_WHEN_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASKING(v: Py) { ensure_init(); G_WHEN_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_TO_BUY() -> Py { ensure_init(); G_WHEN_TO_BUY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_TO_BUY(v: Py) { ensure_init(); G_WHEN_TO_BUY.with(|c| *c.borrow_mut() = v); }
pub fn g__BUY_THE_NEXT() -> Py { ensure_init(); G__BUY_THE_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BUY_THE_NEXT(v: Py) { ensure_init(); G__BUY_THE_NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__HAS_FAILED() -> Py { ensure_init(); G__HAS_FAILED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAS_FAILED(v: Py) { ensure_init(); G__HAS_FAILED.with(|c| *c.borrow_mut() = v); }
pub fn g_asks_where_a_shop_answers() -> Py { ensure_init(); G_asks_where_a_shop_answers.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_asks_where_a_shop_answers(v: Py) { ensure_init(); G_asks_where_a_shop_answers.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("When to buy things: the month a mattress, a sofa, a car or a coat is cheapest.\n\nA \"when\" question is not a \"which one\" question. \"When is the best time to buy a mattress\" was\nanswered with how to choose a mattress, \"when should i buy a new car\" with how to pick a car and\n\"when is the best time to buy a winter coat\" with what to look for in a coat: fluent, about the\nsame object, and no answer to the question that was asked. The choosing rows in\nbot/knowledge/shopping.py are about the object and this domain is about the calendar, so a line\nthat asks the calendar question reaches a row here.\n\nEach row is an item, the weeks it is actually cheap, and the one thing that catches people out.\nThe facts are the retail calendar everybody can check: the January sales, the Black Friday week\nat the end of November, the end of a season for the things that season sells, and the quiet\nmonths at a car dealer. Where a \"sale\" price is not a real discount - mattresses, most of all -\nthe row says so instead of pretending.\n");
    s_asks_where_a_shop_answers(py_func("asks_where_a_shop_answers", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "asks_where_a_shop_answers", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("shopping"), py_str("money")]));
    fn __data1() -> Py { py_tuple(vec![py_str("when"), py_str("what month"), py_str("which month"), py_str("what time of year"), py_str("time of year"), py_str("best time"), py_str("best month"), py_str("cheapest time"), py_str("cheapest month"), py_str("right time"), py_str("good time"), py_str("good month"), py_str("month to buy"), py_str("time to buy"), py_str("when to buy"), py_str("what time"), py_str("black friday"), py_str("january"), py_str("january sales"), py_str("sell off"), py_str("price drop"), py_str("prices drop"), py_str("go down"), py_str("come down"), py_str("going down"), py_str("get the best deal"), py_str("get a better deal"), py_str("get a good deal"), py_str("get the cheapest"), py_str("get a cheaper")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.TIME_ASK"); }
    s_TIME_ASK(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.TIME_ASK_SHAPE"); }
    s_TIME_ASK_SHAPE(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TIME_ASK()) { let mut phrase: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[op_add(&op_add(&py_str(" "), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s? "))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("when is"), py_str("when are"), py_str("when was"), py_str("when's"), py_str("when should"), py_str("when shall"), py_str("when can"), py_str("when could"), py_str("when do "), py_str("when does"), py_str("when did"), py_str("when will"), py_str("when would"), py_str("when to buy"), py_str("when to get"), py_str("when to sell"), py_str("when in the year"), py_str("when is the best"), py_str("when is a good"), py_str("when is the cheapest")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.WHEN_ASKING"); }
    s_WHEN_ASKING(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.PRICE_ASK"); }
    s_PRICE_ASK(py_tuple(vec![py_str("sale"), py_str("sales"), py_str("cheapest"), py_str("cheaper"), py_str("cheap"), py_str("discount"), py_str("discounts"), py_str("deal"), py_str("deals"), py_str("half price")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.WHEN_ASK"); }
    s_WHEN_ASK(op_add(&g_TIME_ASK(), &g_PRICE_ASK()));
    fn __data3() -> Py { py_tuple(vec![py_str("month"), py_str("months"), py_str("week"), py_str("weeks"), py_str("weekend"), py_str("weekends"), py_str("summer"), py_str("winter"), py_str("spring"), py_str("autumn"), py_str("christmas"), py_str("easter"), py_str("season"), py_str("seasons"), py_str("year"), py_str("years")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.PERIOD_NOUNS"); }
    s_PERIOD_NOUNS(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.PERIOD_PREPOSITION"); }
    s_PERIOD_PREPOSITION(py_tuple(vec![py_str("in"), py_str("at"), py_str("over"), py_str("during"), py_str("for")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.COMPARING"); }
    s_COMPARING(py_tuple(vec![py_str("cheaper"), py_str("cheapest"), py_str("best"), py_str("deal"), py_str("deals"), py_str("discount"), py_str("discounts"), py_str("sale"), py_str("sales"), py_str("half price"), py_str("price drop"), py_str("prices drop")]));
    fn __data4() -> Py { py_tuple(vec![py_str("replace"), py_str("replacing"), py_str("replacement"), py_str("change"), py_str("changes"), py_str("changing"), py_str("changed"), py_str("due for"), py_str("need a new"), py_str("need new"), py_str("insurance"), py_str("insure"), py_str("renew"), py_str("renews"), py_str("renewal"), py_str("renewals"), py_str("renewing"), py_str("policy"), py_str("policies"), py_str("premium"), py_str("premiums"), py_str("no claims"), py_str("auto renew"), py_str("auto-renew"), py_str("excess"), py_str("quote"), py_str("quotes"), py_str("cover"), py_str("covers"), py_str("mot"), py_str("tax"), py_str("service"), py_str("servicing"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("long does"), py_str("how often"), py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washing"), py_str("repair"), py_str("repairs"), py_str("warranty"), py_str("guarantee"), py_str("return"), py_str("refund"), py_str("delivery"), py_str("deliver"), py_str("install"), py_str("installation"), py_str("measure"), py_str("measuring"), py_str("size"), py_str("firm"), py_str("soft"), py_str("cheap one"), py_str("second hand"), py_str("secondhand"), py_str("used"), py_str("sell"), py_str("selling"), py_str("selling my"), py_str("start"), py_str("starts"), py_str("starting"), py_str("started"), py_str("wont start"), py_str("takes ages"), py_str("takes a while"), py_str("slow to"), py_str("stalls"), py_str("stalling"), py_str("cut out"), py_str("cuts out"), py_str("misfire"), py_str("flat"), py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("noise"), py_str("noisy"), py_str("smell"), py_str("smells"), py_str("warning light"), py_str("dashboard"), py_str("rid of"), py_str("dispose"), py_str("tip"), py_str("recycle"), py_str("book the van"), py_str("book a van"), py_str("book the removals"), py_str("hire a van"), py_str("hire the van"), py_str("hiring a van"), py_str("hired a van"), py_str("van hire"), py_str("removals"), py_str("man with a van"), py_str("moving house"), py_str("moving flat"), py_str("run"), py_str("runs"), py_str("running"), py_str("put"), py_str("turn"), py_str("switch"), py_str("use"), py_str("switch on"), py_str("switching on"), py_str("leave it on"), py_str("left on"), py_str("cover"), py_str("covers"), py_str("topper"), py_str("protector"), py_str("sheet"), py_str("sheets"), py_str("battery"), py_str("batteries"), py_str("charger"), py_str("case"), py_str("spare"), py_str("part"), py_str("parts"), py_str("filter"), py_str("filters"), py_str("bulb"), py_str("bulbs"), py_str("contract"), py_str("sim"), py_str("sim only"), py_str("upgrade"), py_str("pay monthly"), py_str("accessory"), py_str("accessories"), py_str("seat"), py_str("seats"), py_str("car seat"), py_str("cot"), py_str("cots"), py_str("pram"), py_str("buggy"), py_str("highchair"), py_str("pushchair"), py_str("take down"), py_str("taken down"), py_str("taking it down"), py_str("put up"), py_str("put it up"), py_str("putting up"), py_str("decorate"), py_str("decorating"), py_str("trim the tree"), py_str("worth it"), py_str("worth the"), py_str("worth buying"), py_str("good deal"), py_str("real deal"), py_str("good price"), py_str("fair price"), py_str("good buy"), py_str("rip off"), py_str("rip-off"), py_str("ripoff"), py_str("overpriced"), py_str("too expensive"), py_str("service"), py_str("serviced"), py_str("servicing"), py_str("mot"), py_str("mot test"), py_str("mot due"), py_str("service interval"), py_str("servicing interval"), py_str("short this month"), py_str("short this week"), py_str("short till"), py_str("short until"), py_str("short of money"), py_str("short on money"), py_str("short of cash"), py_str("short on cash"), py_str("short of funds"), py_str("a bit short"), py_str("bit short"), py_str("money is short"), py_str("moneys short"), py_str("money's short"), py_str("money is a bit short"), py_str("money is tight"), py_str("money is a bit tight"), py_str("tight month"), py_str("running short"), py_str("running a bit short"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("cooked"), py_str("roast"), py_str("roasting"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing"), py_str("preheat"), py_str("preheated"), py_str("simmer"), py_str("boil"), py_str("boiling"), py_str("season"), py_str("seasoning"), py_str("recipe"), py_str("chicken"), py_str("turkey"), py_str("joint of"), py_str("gas mark"), py_str("degrees"), py_str("temperature")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.NOT_THE_CALENDAR"); }
    s_NOT_THE_CALENDAR(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("broken"), py_str("broke"), py_str("breaks"), py_str("breaking"), py_str("faulty"), py_str("fault"), py_str("stopped"), py_str("not working"), py_str("isnt working"), py_str("isn't working"), py_str("went wrong"), py_str("gone wrong"), py_str("goes wrong"), py_str("packed in"), py_str("conked out"), py_str("died"), py_str("dead"), py_str("wont start"), py_str("won't start"), py_str("wont spin"), py_str("won't spin"), py_str("leaking"), py_str("months old"), py_str("years old")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy._HAS_FAILED"); }
    s__HAS_FAILED(__data5());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy._BUY_THE_NEXT"); }
    s__BUY_THE_NEXT(py_tuple(vec![py_str("when should i buy"), py_str("when to buy"), py_str("when should i get"), py_str("when to get"), py_str("best time to buy"), py_str("when is the best time to buy"), py_str("when should i replace"), py_str("time to buy a new"), py_str("when do i buy"), py_str("when can i buy"), py_str("when will i buy"), py_str("cheapest time to buy")]));
    fn __data6() -> Py { py_tuple(vec![py_str("coat"), py_str("coats"), py_str("winter coat"), py_str("winter coats"), py_str("jacket"), py_str("jackets"), py_str("winter boots"), py_str("wellingtons"), py_str("ski boots"), py_str("ski boot"), py_str("skiing"), py_str("ski"), py_str("snowboard"), py_str("snowboarding")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.WHEN_TO_BUY"); }
    s_WHEN_TO_BUY(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("mattress"), py_str("mattresses")]), py_str("The January sales and the Black Friday week are when mattresses are marked down, though most mattress prices are 'on sale' all year, so compare two or three on firmness rather than on the percentage off. Buy somewhere with a 60-night trial: ten minutes in a shop tells you little about how you will sleep.")]), py_tuple(vec![py_tuple(vec![py_str("sofa"), py_str("sofas"), py_str("settee"), py_str("settees"), py_str("couch"), py_str("couches"), py_str("armchair"), py_str("armchairs")]), py_str("Sofas are cheapest in the January sales and over bank holiday weekends, when the chains run 20 to 30 per cent off. A made-to-order sofa takes six to twelve weeks, so for Christmas you want to order by early November - and check the delivery date in writing rather than the one on the website.")]), py_tuple(vec![py_tuple(vec![py_str("tv"), py_str("tvs"), py_str("television"), py_str("televisions"), py_str("telly")]), py_str("Televisions are cheapest on Black Friday and in the January sales, and again in February to April, when the year's new models arrive and last year's drop a few hundred pounds for the same panel. If you do not need this year's model, spring is the quiet bargain.")]), py_tuple(vec![py_tuple(vec![py_str("washing machine"), py_str("washing machines"), py_str("tumble dryer"), py_str("tumble dryers"), py_str("fridge"), py_str("fridge freezer"), py_str("freezer"), py_str("dishwasher"), py_str("oven")]), py_str("White goods are cheapest on Black Friday and in the January sales, and the January price on last year's model usually beats the Black Friday one. Measure the gap and the doorway before you buy, and check the energy rating: over ten years the running cost matters more than the purchase price.")]), py_tuple(vec![py_tuple(vec![py_str("laptop"), py_str("laptops"), py_str("computer"), py_str("computers"), py_str("tablet"), py_str("tablets")]), py_str("Laptops and tablets are cheapest on Black Friday, in the January sales, and in the back-to-school weeks of late August when the makers push student models. A machine whose replacement has just launched drops the most, so check whether a newer model is out.")]), py_tuple(vec![py_tuple(vec![py_str("phone"), py_str("phones"), py_str("mobile"), py_str("mobiles"), py_str("iphone"), py_str("smartphone"), py_str("smartphones")]), py_str("The cheapest month for last year's phone is the one after the new model lands, which is usually late September to November, and Black Friday adds a further cut. Buying outright and keeping it four years is nearly always cheaper than the annual upgrade.")]), py_tuple(vec![py_tuple(vec![py_str("car"), py_str("cars"), py_str("van"), py_str("vans"), py_str("motorbike"), py_str("motorbikes")]), py_str("A car is cheapest at the end of a month or a quarter, when a dealer is chasing a sales target and will move on price, and in the quiet weeks of August and late December. New plates in March and September make the previous plate's stock cheaper, and a pre-registered car saves the most of all - just check you are the first registered keeper.")]), py_tuple(vec![__data6(), py_str("Winter coats and boots are cheapest in the January sales and at the end of the season in February, when the shops clear the rails for spring. Buy then for next winter, and buy the best you can afford: a waterproof shell at the higher price outlasts three cheap ones.")]), py_tuple(vec![py_tuple(vec![py_str("summer clothes"), py_str("holiday clothes"), py_str("swimwear"), py_str("swimsuit"), py_str("sandals"), py_str("shorts"), py_str("sunglasses")]), py_str("Summer clothes, swimwear and sandals are cheapest in August and early September, when the shops clear them out for knitwear, so late summer is the moment for next year's holiday. Sizes go first, so go early in the clearance rather than at the end of it.")]), py_tuple(vec![py_tuple(vec![py_str("garden furniture"), py_str("barbecue"), py_str("barbecues"), py_str("bbq"), py_str("patio"), py_str("parasol"), py_str("sun lounger")]), py_str("Garden furniture and barbecues are cheapest at the end of August and in September, when the season ends and the garden ranges go on clearance - but the choice is picked over by then. Buy late for price, or in March for choice.")]), py_tuple(vec![py_tuple(vec![py_str("bike"), py_str("bikes"), py_str("bicycle"), py_str("bicycles")]), py_str("Bikes are cheapest in September and October as the season ends, and again when next year's models arrive and the old ones are cleared. A last-year model with the same frame is the bargain worth having.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.SALES_CALENDAR"); }
    s_SALES_CALENDAR(py_str("The big retail sales are the January sales (from Boxing Day to the end of January), the Black Friday week at the end of November, the Easter and August bank holiday weekends, and the end-of-season clearances - summer things in August, winter things in January and February. January and Black Friday are the deepest cuts, and the item you want most is the one worth waiting for rather than buying on impulse at a weaker sale."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.whenbuy.ENTRIES"); }
    s_ENTRIES(op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHEN_TO_BUY()) { let __unpk136 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk136[0].clone(); let mut reply: Py = __unpk136[1].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("whenbuy-"), &mcall(&geti(&words.clone(), &Py::Int(0)), "replace", &[py_str(" "), py_str("-")]))); __args.push(py_tuple(vec![words.clone(), g_WHEN_ASK()])); __args.push(_whenbuy(&[reply.clone(), words.clone()])); __kws.extend(vec![("avoid".to_string(), _avoid_for(&[words.clone()])), ("guard".to_string(), { let reply = reply.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let reply = reply.clone(); let words = words.clone(); let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "asks_where_a_shop_answers", &[understood.clone()]))) }) }), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() }, &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("whenbuy-christmas")); __args.push(py_tuple(vec![py_tuple(vec![py_str("christmas tree"), py_str("christmas trees"), py_str("christmas decorations"), py_str("christmas lights"), py_str("decorations"), py_str("fairy lights"), py_str("tinsel"), py_str("baubles"), py_str("ornaments")]), g_WHEN_ASK()])); __args.push(py_func("_christmas", |args: &[Py]| _christmas(args))); __kws.extend(vec![("avoid".to_string(), _avoid_for(&[py_tuple(vec![py_str("christmas tree"), py_str("christmas decorations")])])), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "asks_where_a_shop_answers", &[understood.clone()]))) })), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("whenbuy-sales-calendar"), py_tuple(vec![py_tuple(vec![py_str("sale"), py_str("sales"), py_str("bargains"), py_str("discounts"), py_str("deals"), py_str("black friday"), py_str("january sales"), py_str("clearance"), py_str("reductions"), py_str("price cuts")]), py_tuple(vec![py_str("when"), py_str("what month"), py_str("which month"), py_str("when are"), py_str("time of year"), py_str("this year"), py_str("next month"), py_str("coming up")])]), py_func("_general_sales", |args: &[Py]| _general_sales(args))])])));
}
pub fn _a_period_beside_a_comparison(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a period named after a preposition is being compared: \"cheaper in the summer months\".");
    words = mcall(&line.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_COMPARING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("half price"), py_str("price drop"), py_str("prices drop")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_PERIOD_NOUNS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&at.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)) { let mut before: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PERIOD_PREPOSITION(), &before.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _line(args: &[Py]) -> Py {
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
    let mut reading_: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The line as words, from the reading, so a row reads the same text the matcher did.");
    reading_ = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    text = { let __b = mcall(&reading_.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading_.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if !py_truthy(&__b) { __b } else { mcall(&reading_.clone(), "get", &[py_str("words")]) } }) {
    text = mcall(&py_str(" "), "join", &[geti(&reading_.clone(), &py_str("words"))]);
    }
    return op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _says_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line has this word itself, not just a word the matcher's families cover.\n\n    The tables list both forms of everything that comes in two (\"tv\"/\"tvs\", \"sofa\"/\"sofas\"), so\n    asking for the word itself costs nothing. It is here because a family can be a coincidence:\n    the family of \"telly\" is \"tell\", so \"when do i tell work im pregnant\" and \"when should i tell\n    my mum\" were answered with the price of a new television. \"tell\" is not \"telly\".\n    ");
    forms = py_set(vec![name.clone(), op_add(&name.clone(), &py_str("s")), op_add(&name.clone(), &py_str("es"))]);
    if py_truthy(&mcall(&name.clone(), "endswith", &[py_str("s")])) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[form.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&name.clone(), &py_str(" "))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&line.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str(".,!?;:'\"()")]);
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { _one_edit_apart(&[word.clone(), name.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _one_edit_apart(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let mut i: Py = Py::None;
    let mut long_: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("True when `typed` is `name` with one letter added, dropped or swapped.");
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return op_le(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[typed.clone(), name.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5653 = py_unpack(&__item.clone(), 2, None); let mut a: Py = __unpk5653[0].clone(); let mut b: Py = __unpk5653[1].clone(); if py_truthy(&Py::Bool(!py_eq(&a.clone(), &b.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    }
    let __unpk1 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![typed.clone(), name.clone()]) } else { py_tuple(vec![name.clone(), typed.clone()]) }, 2, None);
    short = __unpk1[0].clone();
    long_ = __unpk1[1].clone();
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[long_.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push(Py::Bool(py_eq(&op_add(&slice_or_raise(&long_.clone(), &Py::None, &i.clone(), &Py::None), &slice_or_raise(&long_.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)), &short.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_the_calendar(args: &[Py]) -> Py {
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
    let mut own: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { py_tuple(vec![]) };
    let mut ask: Py = Py::None;
    let mut compared_period: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("True when the line asks when, and is not a how-long/repair/care question wearing a when.\n\n    The row's own item is taken out of the line first: \"a washing machine\" carries the word\n    \"washing\", which is how a care question is said, and reading it as one would stand the row\n    down for every washing-machine line it exists to answer.\n    ");
    line = _line(&[understood.clone()]);
    if py_truthy(&{ let __b = own.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&own.clone()) { let mut name: Py = __item.clone(); __out.push(_says_it(&[line.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&own.clone());
    for __item in __iter1 {
    name = __item.clone();
    line = mcall(&line.clone(), "replace", &[op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" ")), py_str(" ")]);
    line = mcall(&line.clone(), "replace", &[op_add(&op_add(&py_str(" "), &name.clone()), &py_str("s ")), py_str(" ")]);
    }
    ask = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TIME_ASK()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&line.clone(), &phrase.clone()))) {  __out.push(phrase.clone()); } } py_list(__out) })() };
    compared_period = _a_period_beside_a_comparison(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TIME_ASK_SHAPE()) { let mut shape: Py = __item.clone(); __out.push(mcall(&shape.clone(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { compared_period.clone() } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = ask.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&phrase.clone(), &py_str("when")))) {  __out.push(phrase.clone()); } } py_list(__out) })() })) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHEN_ASKING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HAS_FAILED()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BUY_THE_NEXT()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_THE_CALENDAR()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _avoid_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut own: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The care words, less the ones the row's own item is written with.\n\n    \"washing machine\" is spelt with the word that says a care question (\"wash\"), and an avoid\n    word matches the forms of a word, so a token comparison is not enough: the two are compared\n    the way the matcher compares them.\n    ");
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_THE_CALENDAR()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&own.clone()) { let mut name: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&name.clone(), "split", &[])) { let mut one: Py = __item.clone(); __out.push({ let __b = mcall(&one.clone(), "startswith", &[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[one.clone()]) } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _whenbuy(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut own: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own"))) };
    let mut answer: Py = Py::None;
    let _ = py_str("A calendar row's answer, and None for a line that is not asking the calendar question.");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut own: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&_asks_the_calendar(&[understood.clone(), own.clone()])))) {
    return Py::None;
    }
    return reply.clone();
    Py::None
}
    answer = {{ let answer = answer.clone(); let own = own.clone(); let reply = reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), own.clone(), reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _general_sales(args: &[Py]) -> Py {
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
    let _ = py_str("The whole-year calendar, for a line that asks when things are on sale and names no item.");
    if py_truthy(&Py::Bool(!py_truthy(&_asks_the_calendar(&[understood.clone()])))) {
    return Py::None;
    }
    return g_SALES_CALENDAR();
    Py::None
}
pub fn _christmas(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut lights: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut tree: Py = Py::None;
    let _ = py_str("A tree is bought in December; the decorations are bought in the January after.\n\n    Both are the same calendar question and the opposite answer, so the line leads: \"when should\n    i buy a christmas tree\" is mid-December, \"when are decorations cheapest\" is the first week of\n    January, and a line that names neither gets both halves in that order.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_asks_the_calendar(&[understood.clone()])))) {
    return Py::None;
    }
    line = _line(&[understood.clone()]);
    doing = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" take "), py_str(" taken "), py_str(" taking "), py_str(" put "), py_str(" putting "), py_str(" pack "), py_str(" packing "), py_str(" throw "), py_str(" go up "), py_str(" goes up "), py_str(" going up "), py_str(" gone up "), py_str(" come down "), py_str(" comes down "), py_str(" coming down ")])) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    gone = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" down "), py_str(" up "), py_str(" away "), py_str(" out ")])) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { gone.clone() } }) {
    return Py::None;
    }
    tree = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("tree"), py_str("trees"), py_str("real tree"), py_str("fir")])) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    lights = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("light"), py_str("lights"), py_str("fairy"), py_str("decorations"), py_str("baubles"), py_str("tinsel"), py_str("ornaments")])) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = tree.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&lights.clone())) } }) {
    return py_str("Buy a real tree in mid-December, a week or two before Christmas: a tree cut in November drops its needles by the big day, so ask the grower when it was cut and keep it in water. Artificial trees are cheapest in the January sales, so buy the one you want now and store it.");
    }
    if py_truthy(&{ let __b = lights.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&tree.clone())) } }) {
    return py_str("Christmas lights and decorations are cheapest in the first week of January, at 70 to 90 per cent off, so buy for next year on the 2nd or 3rd rather than in November, when the same set is full price.");
    }
    return py_str("Buy a real tree in mid-December, a week or two before Christmas, from a grower who will tell you when it was cut; lights and decorations are cheapest in the first week of January, at 70 to 90 per cent off, for next year.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.whenbuy".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "COMPARING" => Some(g_COMPARING()),
            "ENTRIES" => Some(g_ENTRIES()),
            "NOT_THE_CALENDAR" => Some(g_NOT_THE_CALENDAR()),
            "PERIOD_NOUNS" => Some(g_PERIOD_NOUNS()),
            "PERIOD_PREPOSITION" => Some(g_PERIOD_PREPOSITION()),
            "PRICE_ASK" => Some(g_PRICE_ASK()),
            "SALES_CALENDAR" => Some(g_SALES_CALENDAR()),
            "TIME_ASK" => Some(g_TIME_ASK()),
            "TIME_ASK_SHAPE" => Some(g_TIME_ASK_SHAPE()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "WHEN_ASKING" => Some(g_WHEN_ASKING()),
            "WHEN_TO_BUY" => Some(g_WHEN_TO_BUY()),
            "_BUY_THE_NEXT" => Some(g__BUY_THE_NEXT()),
            "_HAS_FAILED" => Some(g__HAS_FAILED()),
            "_a_period_beside_a_comparison" => Some(py_func("_a_period_beside_a_comparison", |args: &[Py]| _a_period_beside_a_comparison(args))),
            "_asks_the_calendar" => Some(py_func("_asks_the_calendar", |args: &[Py]| _asks_the_calendar(args))),
            "_avoid_for" => Some(py_func("_avoid_for", |args: &[Py]| _avoid_for(args))),
            "_christmas" => Some(py_func("_christmas", |args: &[Py]| _christmas(args))),
            "_general_sales" => Some(py_func("_general_sales", |args: &[Py]| _general_sales(args))),
            "_line" => Some(py_func("_line", |args: &[Py]| _line(args))),
            "_one_edit_apart" => Some(py_func("_one_edit_apart", |args: &[Py]| _one_edit_apart(args))),
            "_says_it" => Some(py_func("_says_it", |args: &[Py]| _says_it(args))),
            "_whenbuy" => Some(py_func("_whenbuy", |args: &[Py]| _whenbuy(args))),
            "asks_where_a_shop_answers" => Some(g_asks_where_a_shop_answers()),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "COMPARING" => s_COMPARING(v),
            "ENTRIES" => s_ENTRIES(v),
            "NOT_THE_CALENDAR" => s_NOT_THE_CALENDAR(v),
            "PERIOD_NOUNS" => s_PERIOD_NOUNS(v),
            "PERIOD_PREPOSITION" => s_PERIOD_PREPOSITION(v),
            "PRICE_ASK" => s_PRICE_ASK(v),
            "SALES_CALENDAR" => s_SALES_CALENDAR(v),
            "TIME_ASK" => s_TIME_ASK(v),
            "TIME_ASK_SHAPE" => s_TIME_ASK_SHAPE(v),
            "WHEN_ASK" => s_WHEN_ASK(v),
            "WHEN_ASKING" => s_WHEN_ASKING(v),
            "WHEN_TO_BUY" => s_WHEN_TO_BUY(v),
            "_BUY_THE_NEXT" => s__BUY_THE_NEXT(v),
            "_HAS_FAILED" => s__HAS_FAILED(v),
            "asks_where_a_shop_answers" => s_asks_where_a_shop_answers(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.whenbuy", module_obj);
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_a_period_beside_a_comparison", py_func("_a_period_beside_a_comparison", |args: &[Py]| _a_period_beside_a_comparison(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_line", py_func("_line", |args: &[Py]| _line(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_says_it", py_func("_says_it", |args: &[Py]| _says_it(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_one_edit_apart", py_func("_one_edit_apart", |args: &[Py]| _one_edit_apart(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_asks_the_calendar", py_func("_asks_the_calendar", |args: &[Py]| _asks_the_calendar(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_avoid_for", py_func("_avoid_for", |args: &[Py]| _avoid_for(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_whenbuy", py_func("_whenbuy", |args: &[Py]| _whenbuy(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_general_sales", py_func("_general_sales", |args: &[Py]| _general_sales(args)));
    pyrt::xrefs::reg_fn("knowledge.whenbuy", "_christmas", py_func("_christmas", |args: &[Py]| _christmas(args)));
    pyrt::xrefs::reg_access("knowledge.whenbuy", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "COMPARING", g_COMPARING, s_COMPARING);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "NOT_THE_CALENDAR", g_NOT_THE_CALENDAR, s_NOT_THE_CALENDAR);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "PERIOD_NOUNS", g_PERIOD_NOUNS, s_PERIOD_NOUNS);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "PERIOD_PREPOSITION", g_PERIOD_PREPOSITION, s_PERIOD_PREPOSITION);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "PRICE_ASK", g_PRICE_ASK, s_PRICE_ASK);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "SALES_CALENDAR", g_SALES_CALENDAR, s_SALES_CALENDAR);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "TIME_ASK", g_TIME_ASK, s_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "TIME_ASK_SHAPE", g_TIME_ASK_SHAPE, s_TIME_ASK_SHAPE);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "WHEN_ASK", g_WHEN_ASK, s_WHEN_ASK);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "WHEN_ASKING", g_WHEN_ASKING, s_WHEN_ASKING);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "WHEN_TO_BUY", g_WHEN_TO_BUY, s_WHEN_TO_BUY);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "_BUY_THE_NEXT", g__BUY_THE_NEXT, s__BUY_THE_NEXT);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "_HAS_FAILED", g__HAS_FAILED, s__HAS_FAILED);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "asks_where_a_shop_answers", g_asks_where_a_shop_answers, s_asks_where_a_shop_answers);
    pyrt::xrefs::reg_access("knowledge.whenbuy", "entry", g_entry, s_entry);
}
