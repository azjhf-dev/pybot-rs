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
thread_local! { static G__ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITEM_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JUDGING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_ITEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.prices: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED() -> Py { ensure_init(); G__ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED(v: Py) { ensure_init(); G__ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JUDGEMENT() -> Py { ensure_init(); G__A_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JUDGEMENT(v: Py) { ensure_init(); G__A_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__ITEMS() -> Py { ensure_init(); G__ITEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITEMS(v: Py) { ensure_init(); G__ITEMS.with(|c| *c.borrow_mut() = v); }
pub fn g__ITEM_NAMES() -> Py { ensure_init(); G__ITEM_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITEM_NAMES(v: Py) { ensure_init(); G__ITEM_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__JUDGING() -> Py { ensure_init(); G__JUDGING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JUDGING(v: Py) { ensure_init(); G__JUDGING.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_WORD() -> Py { ensure_init(); G__MONEY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_WORD(v: Py) { ensure_init(); G__MONEY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_ITEM() -> Py { ensure_init(); G__NOT_AN_ITEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_ITEM(v: Py) { ensure_init(); G__NOT_AN_ITEM.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PRICE() -> Py { ensure_init(); G__NOT_A_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PRICE(v: Py) { ensure_init(); G__NOT_A_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g__PRICE() -> Py { ensure_init(); G__PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRICE(v: Py) { ensure_init(); G__PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What an everyday thing costs: the price of a pint, a loaf, a bottle of wine.\n\nThe answers lane owns this domain. A price on a shelf moves, so nothing here is a live figure -\nbut \"how much is a pint of beer\" and \"how much does a loaf cost\" had no card at all, and the rows\nthat took them answered a different question: the 14-units-a-week limit for the pint, a wine\npairing for the bottle, and \"Prices move too fast for me to know. A quick look at the seller's own\nsite will beat any guess\" for the milk, which is a shop rather than a price.\n\nWhat is held here is the ordinary range, the year it was written, and what makes the top of the\nrange, because that is the part that stays true: a supermarket pint is a third of a pub one and\nLondon is dearer than a small town. A line that asks how much it *weighs*, how much goes *in* a\ncake, or how much is *too much* is another domain's question and this one stands down.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("money")]));
    fn __data2() -> Py { py_tuple(vec![py_str("pint of beer"), py_str("pint of lager"), py_str("pint of bitter"), py_str("pint of ale"), py_str("pint of cider"), py_str("pint of guinness"), py_str("pint of stout"), py_str("a pint"), py_str("beer"), py_str("lager"), py_str("bitter"), py_str("cider"), py_str("ale"), py_str("stout")]) }
    fn __data1() -> Py { py_tuple(vec![py_tuple(vec![__data2(), py_str("A pint is about £4 to £5.50 in most pubs and £6 to £7.50 in London and the big cities, so £5.50 is a fair average to budget. A 500ml bottle or can from a supermarket is £1.50 to £2.50, and a four-pack £5 to £7.")]), py_tuple(vec![py_tuple(vec![py_str("bottle of wine"), py_str("wine"), py_str("red wine"), py_str("white wine"), py_str("rose"), py_str("prosecco"), py_str("champagne")]), py_str("A drinkable supermarket bottle is £6 to £8 and a step up is £10 to £15; a pub or restaurant charges £18 to £26 for the same sort of bottle. A small 187ml bottle is £2.50 to £3.50.")]), py_tuple(vec![py_tuple(vec![py_str("pint of milk"), py_str("milk"), py_str("bottle of milk")]), py_str("About 70p a pint, so a two-pint bottle is £1.25 to £1.45 and a four-pint one £1.70 to £2. The filtered and the organic sorts cost half as much again, and a corner shop is dearer than a supermarket.")]), py_tuple(vec![py_tuple(vec![py_str("loaf"), py_str("loaf of bread"), py_str("bread"), py_str("sliced bread")]), py_str("£1.20 to £1.60 for a supermarket own-brand white or wholemeal loaf, £2.50 to £4 for a sourdough or a bakery loaf, and about £1 for the value line.")]), py_tuple(vec![py_tuple(vec![py_str("eggs"), py_str("half a dozen eggs"), py_str("dozen eggs"), py_str("box of eggs")]), py_str("£1.50 to £2.20 for six free-range eggs and £2.50 to £3.50 for twelve, with large and organic at the top of that.")]), py_tuple(vec![py_tuple(vec![py_str("lentils"), py_str("lentil"), py_str("red lentils"), py_str("bag of lentils"), py_str("dried lentils"), py_str("split peas"), py_str("dried beans"), py_str("chickpeas"), py_str("pulses")]), py_str("Dry red lentils are £1 to £1.50 for 500g and £2 to £2.50 a kilo, and chickpeas and dried beans are much the same. That is why a lentil curry is one of the cheapest dinners there is: a kilo is about ten portions.")]), py_tuple(vec![py_tuple(vec![py_str("potatoes"), py_str("potato"), py_str("kilo of potatoes"), py_str("bag of potatoes"), py_str("new potatoes"), py_str("spuds")]), py_str("About 60p to £1 a kilo for white potatoes, so a 2.5kg bag is £1.50 to £2.50. New potatoes and the small salad ones are £1.50 to £2.50 a kilo, and a baking potato about 25p to 40p each.")]), py_tuple(vec![py_tuple(vec![py_str("whole chicken"), py_str("whole chickens"), py_str("chicken"), py_str("a roasting chicken")]), py_str("£4 to £6 for a whole chicken of about 1.5kg to 2kg, and £3.50 to £5 for a smaller one. That is four to six portions of meat plus a pot of stock, which is why it costs less per meal than the same weight of breasts.")]), py_tuple(vec![py_tuple(vec![py_str("stamp"), py_str("stamps"), py_str("first class stamp"), py_str("second class stamp"), py_str("book of stamps"), py_str("book of first class stamps")]), py_str("About £1.70 for a first class stamp and 87p for a second class one in 2025, and a book of eight first class is about £13.50. A large letter costs more - about £2.70 first class.")]), py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("flat white"), py_str("latte"), py_str("cappuccino"), py_str("cup of coffee"), py_str("takeaway coffee")]), py_str("£3 to £3.80 for a flat white or a latte in a chain and £3.50 to £4.50 in an independent cafe, with oat or soya milk often 30 to 50p extra.")]), py_tuple(vec![py_tuple(vec![py_str("tea"), py_str("cup of tea"), py_str("pot of tea")]), py_str("£2.50 to £3.50 for a pot of tea in a cafe, and about 3p a cup made at home from a £2.50 box of 80 bags.")]), py_tuple(vec![py_tuple(vec![py_str("cigarettes"), py_str("fags"), py_str("packet of cigarettes"), py_str("pack of cigarettes"), py_str("twenty cigarettes")]), py_str("About £14 to £16 for 20 cigarettes in 2025, and £5 to £6 for a 30g pouch of rolling tobacco.")]), py_tuple(vec![py_tuple(vec![py_str("punnet"), py_str("punnet of strawberries"), py_str("strawberries"), py_str("raspberries"), py_str("blueberries"), py_str("berries"), py_str("punnet of berries")]), py_str("A 227g punnet of strawberries is £2 to £2.50 in summer and £3 to £4 out of season, and a punnet of raspberries or blueberries £2 to £3. British berries in June and July are about half the price of the imported ones in January.")]), py_tuple(vec![py_tuple(vec![py_str("fish and chips"), py_str("fish supper"), py_str("chippy"), py_str("chip shop"), py_str("cod and chips"), py_str("haddock and chips"), py_str("sausage supper")]), py_str("£9 to £12 for fish and chips from a chip shop and £12 to £16 in London and the big cities, with a sit-down restaurant a few pounds more than that. A small or pensioner's portion is £6 to £8, and mushy peas about £1.50.")]), py_tuple(vec![py_tuple(vec![py_str("takeaway"), py_str("takeaway curry"), py_str("chinese takeaway"), py_str("indian takeaway"), py_str("pizza delivery"), py_str("delivered pizza"), py_str("a pizza"), py_str("kebab")]), py_str("£9 to £13 for one takeaway main - a curry, a Chinese dish, a large pizza - and £15 to £25 for a delivery for two once the sides and the fee are on it. Collecting it yourself saves the £3 to £5 delivery fee.")]), py_tuple(vec![py_tuple(vec![py_str("trainers"), py_str("a pair of trainers"), py_str("running shoes"), py_str("sneakers"), py_str("school shoes")]), py_str("£45 to £85 for a decent pair of everyday trainers, and last season's colour of the same shoe is often half price. Proper running shoes start about £70, and school shoes come in cheaper.")]), py_tuple(vec![py_tuple(vec![py_str("gym membership"), py_str("gym"), py_str("leisure centre"), py_str("swimming pool"), py_str("pool membership")]), py_str("A gym is £25 to £45 a month, with a joining fee of about £20 on top. Pay as you go is about £8 a visit, which is worth working out against a month of a membership you do not use.")]), py_tuple(vec![py_tuple(vec![py_str("cinema ticket"), py_str("cinema"), py_str("the pictures"), py_str("film ticket"), py_str("imax ticket")]), py_str("£8 to £15 for an adult cinema ticket, and up to £20 in London or for IMAX and a reclining seat. Most chains have a cheaper showing on a weekday before five.")]), py_tuple(vec![py_tuple(vec![py_str("netflix"), py_str("streaming service"), py_str("netflix subscription"), py_str("disney plus"), py_str("prime video"), py_str("spotify")]), py_str("£5 to £16 a month for a streaming service, depending on the tier: the ad-supported ones are at the bottom of that and the ad-free 4K ones at the top. Paying for a year upfront, where it is offered, works out about two months free.")]), py_tuple(vec![py_tuple(vec![py_str("bread and milk"), py_str("weekly shop"), py_str("food shop"), py_str("groceries")]), py_str("The average household spends about £70 to £80 a week on food shopping, from £40 for one person cooking from scratch to £150 for a family that buys ready meals.")])]) }
    s__ITEMS(__data1());
    s__ITEM_NAMES(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITEMS()) { let __unpk4212 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk4212[0].clone(); let mut _u: Py = __unpk4212[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("how much is"), py_str("how much are"), py_str("how much does"), py_str("how much do"), py_str("how much would"), py_str("how much for"), py_str("what does"), py_str("what do"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("cheaper"), py_str("cheapest"), py_str("going rate"), py_str("these days"), py_str("nowadays"), py_str("average"), py_str("too much"), py_str("too expensive"), py_str("too dear"), py_str("overpriced"), py_str("over the odds"), py_str("about right"), py_str("fair price"), py_str("reasonable"), py_str("expensive"), py_str("rip off"), py_str("ripped off"), py_str("a lot of money")]) }
    s__ASKED(__data3());
    s__A_FIGURE(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)\\s?\\d[\\d,]*(?:\\.\\d+)?|\\b\\d[\\d,]*(?:\\.\\d+)?\\s*(?:k\\b|grand\\b|quid|pounds?|bucks)|\\b\\d[\\d,]*(?:\\.\\d+)?\\s+(?:for\\b|a\\b|an\\b|per\\b|each\\b|every\\b|too\\b|to\\s+spend\\b|in\\s+total\\b|all\\s+in\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:too much|too expensive|too dear|overpriced|over the odds|about right|fair price|a fair price|reasonable price|rip off|ripped off|a lot of money|worth it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MONEY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:price|prices|priced|cost|costs|charge|charges|fee|fees|pay|pays|paid|quid)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__JUDGING(stdlib_call("re", "compile", &[py_str("(?:\\u00a3\\s*\\d|\\b\\d+(?:\\.\\d+)?\\s*(?:pounds?|quid|pence|gbp|p\\b))[^.?!]*?\\b(?:a lot|lot for|too much|expensive|cheap|cheaper|cheapest|dear|dearer|overpriced|rip.?off|worth it|worth the|worth buying|reasonable|a fair price|fair price|fair|good price|bad price|good buy|bad buy|good value|poor value|a bargain|a steal|ok|okay|alright|all right|acceptable|about right|sensible|good deal|bad deal|worth the money)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:how much (?:is|are|does|do|would)|price|prices|cost|costs|cheaper|cheapest|expensive|going rate|these days|nowadays|average)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:weigh|weighs|grams?|g\\b|ml\\b|millilitres?|litres?|cups?|tablespoons?|teaspoons?|recipe|recipes|in a cake|for a cake|per person|serves?|servings?|calories|units?|safe|limit|how many|should i drink|drink a week|a day|make|making|grow|keep|keeps|last|lasts|freeze|store|storing|goes in|do i need|is in|rise|rises|rising|dough|yeast|knead|prove|proofing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_AN_ITEM(stdlib_call("re", "compile", &[py_str("\\b(?:bill|bills|meter|tariff|standing charge|contract|insurance|premium|tax|rent|mortgage|salary|wage|wages|pension|fine|postage|delivery|shipping|vets?|prescription|train ticket|bus ticket|plane ticket|flight ticket|concert ticket|rail fare|flight|hotel|rental|deposit|lesson|course|subscription|mobile|broadband|energy|gas|electric|electricity|water bill|council|send|sending|sent|parcel|parcels|letter|letters|post office|recorded|tracked|sandwich|sandwiches|soup|curry|pie|nuggets|wings|thighs|breasts|breast|fillet|drumsticks|kebab|kiev|kievs|goujons?|burgers?|strips|wrapping?|for dinner|for tea|ireland|irish|dublin|america|australia|canada|spain|france|germany)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("units"), py_str("how many units"), py_str("safe"), py_str("limit"), py_str("cut down"), py_str("reduce"), py_str("calories"), py_str("goes with"), py_str("pair"), py_str("pairing"), py_str("match"), py_str("which wine"), py_str("with pizza"), py_str("with curry"), py_str("with steak"), py_str("offer"), py_str("offers"), py_str("deal"), py_str("deals"), py_str("supermarket offer"), py_str("loyalty"), py_str("coupon"), py_str("voucher"), py_str("weigh"), py_str("grams"), py_str("ml"), py_str("recipe"), py_str("in a cake"), py_str("for a cake"), py_str("per person"), py_str("serves"), py_str("how many"), py_str("make"), py_str("bake"), py_str("grow"), py_str("keep"), py_str("freeze"), py_str("bill"), py_str("bills"), py_str("meter"), py_str("tariff"), py_str("rent"), py_str("insurance"), py_str("tax")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("everyday-prices")); __args.push(py_tuple(vec![g__ASKED(), g__ITEM_NAMES()])); __args.push(py_func("_the_item_reply", |args: &[Py]| _the_item_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("_a_price_was_asked", |args: &[Py]| _a_price_was_asked(args))), ("avoid".to_string(), __data4())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn a_price_is_judged(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line brings a figure in money and asks whether it is a lot.\n\n    The cards that answer the subject itself stand down on this, because the question is about\n    the money: the verdict is put against the range the same thing sells at, which is what the\n    person's figure is being compared with.\n\n        a_price_is_judged(\"is 3 pounds a lot for a chicken\")   -> True\n        a_price_is_judged(\"is 3 pounds a lot for a chicken\") -> True\n        a_price_is_judged(\"how long do i roast a 4 pound chicken\")  -> False\n        a_price_is_judged(\"how much is a chicken\")                  -> False\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[mcall(&g__JUDGING(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_text(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line as it was said, from the reading every entry is handed.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    text = { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&reading.clone(), "get", &[py_str("text")]) } };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    words = mcall(&reading.clone(), "get", &[py_str("words")]);
    text = if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[words.clone()]) } else { py_str("") };
    }
    return mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    Py::None
}
pub fn _a_price_was_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut figured: Py = Py::None;
    let mut judged: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line asks what an everyday thing costs, and nothing else about it.");
    text = _the_text(&[reading.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__NOT_AN_ITEM(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__NOT_A_PRICE(), "search", &[text.clone()]) } } } }) {
    return Py::Bool(false);
    }
    judged = call_builtin("bool", &[mcall(&g__A_JUDGEMENT(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    figured = call_builtin("bool", &[mcall(&g__A_FIGURE(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = judged.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&figured.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__MONEY_WORD(), "search", &[text.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__PRICE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = judged.clone(); if !py_truthy(&__b) { __b } else { figured.clone() } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_item_reply(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut hit: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut names: Py = Py::None;
    let mut other: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The price of the one item the line names, or the general line when it names several.");
    text = _the_text(&[reading.clone()]);
    said = py_list(vec![]);
    named = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g__ITEMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    hit = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&hit.clone()) {
    let _ = mcall(&named.clone(), "append", &[hit.clone()]);
    let _ = mcall(&said.clone(), "append", &[reply.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    longest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut hit: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(hit.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&longest.clone()) { let mut name: Py = __item.clone(); __out.push(op_sub(&call_builtin("set", &[mcall(&name.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("a"), py_str("an"), py_str("the")]))); } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[said.clone(), words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8815 = py_unpack(&__item.clone(), 2, None); let mut reply: Py = __unpk8815[0].clone(); let mut mine: Py = __unpk8815[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut other: Py = __item.clone(); __out.push(op_lt(&mine.clone(), &other.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(reply.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return op_add(&py_str("As a rough guide, in 2025: "), &mcall(&py_str(" "), "join", &[slice_or_raise(&said.clone(), &Py::None, &Py::Int(3), &Py::None)]));
    }
    return op_add(&geti(&said.clone(), &Py::Int(0)), &py_str(" Those are 2025 prices, and they are the sort of thing that moves."));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.prices".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASKED" => Some(g__ASKED()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_JUDGEMENT" => Some(g__A_JUDGEMENT()),
            "_ITEMS" => Some(g__ITEMS()),
            "_ITEM_NAMES" => Some(g__ITEM_NAMES()),
            "_JUDGING" => Some(g__JUDGING()),
            "_MONEY_WORD" => Some(g__MONEY_WORD()),
            "_NOT_AN_ITEM" => Some(g__NOT_AN_ITEM()),
            "_NOT_A_PRICE" => Some(g__NOT_A_PRICE()),
            "_PRICE" => Some(g__PRICE()),
            "_a_price_was_asked" => Some(py_func("_a_price_was_asked", |args: &[Py]| _a_price_was_asked(args))),
            "_the_item_reply" => Some(py_func("_the_item_reply", |args: &[Py]| _the_item_reply(args))),
            "_the_text" => Some(py_func("_the_text", |args: &[Py]| _the_text(args))),
            "a_price_is_judged" => Some(py_func("a_price_is_judged", |args: &[Py]| a_price_is_judged(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASKED" => s__ASKED(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            "_A_JUDGEMENT" => s__A_JUDGEMENT(v),
            "_ITEMS" => s__ITEMS(v),
            "_ITEM_NAMES" => s__ITEM_NAMES(v),
            "_JUDGING" => s__JUDGING(v),
            "_MONEY_WORD" => s__MONEY_WORD(v),
            "_NOT_AN_ITEM" => s__NOT_AN_ITEM(v),
            "_NOT_A_PRICE" => s__NOT_A_PRICE(v),
            "_PRICE" => s__PRICE(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.prices", module_obj);
    pyrt::xrefs::reg_fn("knowledge.prices", "a_price_is_judged", py_func("a_price_is_judged", |args: &[Py]| a_price_is_judged(args)));
    pyrt::xrefs::reg_fn("knowledge.prices", "_the_text", py_func("_the_text", |args: &[Py]| _the_text(args)));
    pyrt::xrefs::reg_fn("knowledge.prices", "_a_price_was_asked", py_func("_a_price_was_asked", |args: &[Py]| _a_price_was_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.prices", "_the_item_reply", py_func("_the_item_reply", |args: &[Py]| _the_item_reply(args)));
    pyrt::xrefs::reg_access("knowledge.prices", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.prices", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.prices", "_ASKED", g__ASKED, s__ASKED);
    pyrt::xrefs::reg_access("knowledge.prices", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
    pyrt::xrefs::reg_access("knowledge.prices", "_A_JUDGEMENT", g__A_JUDGEMENT, s__A_JUDGEMENT);
    pyrt::xrefs::reg_access("knowledge.prices", "_ITEMS", g__ITEMS, s__ITEMS);
    pyrt::xrefs::reg_access("knowledge.prices", "_ITEM_NAMES", g__ITEM_NAMES, s__ITEM_NAMES);
    pyrt::xrefs::reg_access("knowledge.prices", "_JUDGING", g__JUDGING, s__JUDGING);
    pyrt::xrefs::reg_access("knowledge.prices", "_MONEY_WORD", g__MONEY_WORD, s__MONEY_WORD);
    pyrt::xrefs::reg_access("knowledge.prices", "_NOT_AN_ITEM", g__NOT_AN_ITEM, s__NOT_AN_ITEM);
    pyrt::xrefs::reg_access("knowledge.prices", "_NOT_A_PRICE", g__NOT_A_PRICE, s__NOT_A_PRICE);
    pyrt::xrefs::reg_access("knowledge.prices", "_PRICE", g__PRICE, s__PRICE);
    pyrt::xrefs::reg_access("knowledge.prices", "entry", g_entry, s_entry);
}
