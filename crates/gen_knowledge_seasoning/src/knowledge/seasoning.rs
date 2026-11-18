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
thread_local! { static G__MULLED_WINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PUMPKIN_LATTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPICE_AMOUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPICE_NOT_A_DOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__MULLED_WINE() -> Py { ensure_init(); G__MULLED_WINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MULLED_WINE(v: Py) { ensure_init(); G__MULLED_WINE.with(|c| *c.borrow_mut() = v); }
pub fn g__PUMPKIN_LATTE() -> Py { ensure_init(); G__PUMPKIN_LATTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PUMPKIN_LATTE(v: Py) { ensure_init(); G__PUMPKIN_LATTE.with(|c| *c.borrow_mut() = v); }
pub fn g__SPICE_AMOUNTS() -> Py { ensure_init(); G__SPICE_AMOUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPICE_AMOUNTS(v: Py) { ensure_init(); G__SPICE_AMOUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPICE_NOT_A_DOSE() -> Py { ensure_init(); G__SPICE_NOT_A_DOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPICE_NOT_A_DOSE(v: Py) { ensure_init(); G__SPICE_NOT_A_DOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Seasoning: how much of a spice goes in the dish, as a figure rather than a rule.\n\n\"How much cinnamon in a cake\", \"how many cloves of garlic in a curry\", \"how many cloves for\nmulled wine\" and \"how much chilli powder for four\" are one question about one ingredient, and\nnone of them had an owner here: the cake's tin table read out the tin sizes, the spice row\nanswered \"spices are not bad for you\", the units row took the mulled wine and the planting row\ntook the garlic. The amount is a fact about the spice and is the same whatever dish is named\nbeside it, so it is a table read by the spice - one line per ingredient - rather than a rule\nper dish. Garlic sits first so that \"cloves of garlic\" is the garlic figure and not the clove\none.\n\nA line that is not a dose stays out: growing a spice, whether it is good for you, a pet that\nate it, a jar that went off, the price of the jar and a count of buns for a crowd all have\ntheir own answers, and each of them is a word in _SPICE_NOT_A_DOSE.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("garlic"), py_str("cloves of garlic"), py_str("clove of garlic"), py_str("garlic cloves"), py_str("garlic clove")]), py_str("Two or three cloves for a curry, stew or sauce that serves four: a fat clove is about a teaspoon once crushed, so a teaspoon of the jarred paste stands in for one.")]), py_tuple(vec![py_tuple(vec![py_str("garlic powder"), py_str("garlic granules")]), py_str("Half a level teaspoon of garlic powder for a curry for four, which stands in for a clove or two; granules are weaker, so use a whole teaspoon, and add either one with the liquid rather than at the end, because they need a minute to soften.")]), py_tuple(vec![py_tuple(vec![py_str("black pepper"), py_str("ground pepper"), py_str("peppercorns"), py_str("pepper")]), py_str("Half a teaspoon of ground black pepper for a dish that serves four, stirred in at the end with the salt: pepper is aroma rather than heat, and it goes flat when it cooks for a long time, so a long stew wants it twice - once at the start and a little at the end.")]), py_tuple(vec![py_tuple(vec![py_str("vanilla"), py_str("vanilla extract"), py_str("vanilla essence")]), py_str("A teaspoon of vanilla extract in a 20cm cake; essence is weaker, so use two, and vanilla paste about the same as the extract.")]), py_tuple(vec![py_tuple(vec![py_str("cinnamon")]), py_str("A teaspoon or two in a 20cm sponge, and half a teaspoon on a bowl of porridge: start with one and taste, because an old jar is weaker than a new one.")]), py_tuple(vec![py_tuple(vec![py_str("baking powder")]), py_str("Two level teaspoons per 200g of plain flour, which is what lifts a sponge. Self-raising flour already has it in, so add none there.")]), py_tuple(vec![py_tuple(vec![py_str("bicarbonate of soda"), py_str("bicarbonate"), py_str("bicarb"), py_str("baking soda")]), py_str("Half a teaspoon per 200g of plain flour, with something acid - buttermilk, yogurt, lemon juice - to set it off; on its own it leaves a soapy taste.")]), py_tuple(vec![py_tuple(vec![py_str("allspice"), py_str("ground allspice"), py_str("pimento"), py_str("pimento berries")]), py_str("A quarter teaspoon of ground allspice for a 20cm cake, or six berries in a jar of pickle: it tastes of clove, cinnamon and nutmeg at once, so a little stands in for all three.")]), py_tuple(vec![py_tuple(vec![py_str("mixed spice"), py_str("pumpkin spice")]), py_str("Two teaspoons in a 20cm cake, or a teaspoon across a dozen buns: it is a blend, so it is gentler than any single spice in it.")]), py_tuple(vec![py_tuple(vec![py_str("nutmeg")]), py_str("A quarter of a whole nut, grated, for a 20cm cake or a pan of rice pudding. Whole nutmeg keeps its flavour for years and ground loses it in months.")]), py_tuple(vec![py_tuple(vec![py_str("ground ginger"), py_str("ginger")]), py_str("A teaspoon of ground ginger in a 20cm cake or a stir-fry for four; fresh ginger is about three times the weight for the same heat.")]), py_tuple(vec![py_tuple(vec![py_str("garam masala")]), py_str("A teaspoon or two for a curry for four, stirred in at the very end with the heat off: it is a finishing blend, and cooked from the start it loses the scent that is the point of it.")]), py_tuple(vec![py_tuple(vec![py_str("curry powder"), py_str("masala")]), py_str("A tablespoon of curry powder for a curry for four. Fry it in the oil for half a minute before the liquid goes in and it tastes twice as strong.")]), py_tuple(vec![py_tuple(vec![py_str("chilli powder"), py_str("chili powder"), py_str("cayenne"), py_str("chilli flakes"), py_str("chili flakes")]), py_str("Half to one teaspoon for four, and taste before the second half: heat builds as it cooks, so it is easy to overshoot and hard to take back.")]), py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chili"), py_str("chillies"), py_str("chilis"), py_str("chile")]), py_str("One chilli for four is enough heat for most people: add a second at the end if you want more, and taking out the seeds and white pith halves the heat without losing the taste.")]), py_tuple(vec![py_tuple(vec![py_str("cumin"), py_str("ground cumin"), py_str("cumin seeds")]), py_str("A teaspoon of ground cumin in a chilli or a curry for four, or a tablespoon of the seeds, which want thirty seconds in a dry hot pan first to wake them up.")]), py_tuple(vec![py_tuple(vec![py_str("fresh coriander"), py_str("coriander leaves"), py_str("coriander to garnish"), py_str("coriander garnish")]), py_str("A small handful of fresh coriander for four, torn in at the end: the stems hold most of the flavour, so chop them into the sauce and keep the leaves for the top.")]), py_tuple(vec![py_tuple(vec![py_str("coriander"), py_str("ground coriander"), py_str("coriander seeds")]), py_str("A teaspoon of ground coriander for a curry for four; the seeds toast in a dry pan in half a minute and grind to something fresher and more citrus than the jar.")]), py_tuple(vec![py_tuple(vec![py_str("turmeric")]), py_str("Half a teaspoon is colour and a teaspoon is flavour for a dish that serves four; more than that turns bitter, which is why it is never heaped in.")]), py_tuple(vec![py_tuple(vec![py_str("paprika"), py_str("smoked paprika")]), py_str("A teaspoon for four is enough for the colour and the smoke; it burns in a hot pan, so add it with the liquid rather than at the start.")]), py_tuple(vec![py_tuple(vec![py_str("oregano"), py_str("dried oregano")]), py_str("A teaspoon of dried oregano for four, or a tablespoon of fresh leaves. Dried oregano wants ten minutes in the sauce to soften; fresh is stirred in at the end.")]), py_tuple(vec![py_tuple(vec![py_str("thyme"), py_str("fresh thyme"), py_str("dried thyme"), py_str("thyme leaves")]), py_str("Three or four sprigs of fresh thyme for four, or a teaspoon of the dried leaves: the woody stems come out before serving, so tie them in a bundle if you want the job easy.")]), py_tuple(vec![py_tuple(vec![py_str("rosemary"), py_str("fresh rosemary"), py_str("dried rosemary")]), py_str("One sprig for a roast for four, or a teaspoon of the needles chopped fine. It is strong and resinous, so it goes in at the start with the meat rather than at the end.")]), py_tuple(vec![py_tuple(vec![py_str("sage"), py_str("fresh sage"), py_str("dried sage")]), py_str("Four or five leaves for four, or a teaspoon dried and crumbled: it is the strongest of the soft herbs and turns medicinal if it is doubled, so it is the one to under-do.")]), py_tuple(vec![py_tuple(vec![py_str("bay leaf"), py_str("bay leaves"), py_str("bay")]), py_str("One leaf for a stew or a pan of rice for four, and take it out at the end: two is plenty in a big pot, and a broken leaf left in is what makes a stew taste of the cupboard.")]), py_tuple(vec![py_tuple(vec![py_str("basil"), py_str("fresh basil"), py_str("dried basil")]), py_str("A handful of fresh leaves for four, torn in off the heat at the end, or a teaspoon of dried in the sauce while it cooks. Dried basil loses the scent that is the reason to use it.")]), py_tuple(vec![py_tuple(vec![py_str("parsley"), py_str("fresh parsley"), py_str("dried parsley")]), py_str("Two tablespoons of chopped fresh parsley for four, stirred in at the end. Dried parsley is mostly for colour, so use twice as much and expect little taste from it.")]), py_tuple(vec![py_tuple(vec![py_str("dill"), py_str("fresh dill")]), py_str("A tablespoon of chopped dill for four, in at the end: it is a soft herb and the scent goes with heat, so it is stirred through just before the pot comes off.")]), py_tuple(vec![py_tuple(vec![py_str("mint"), py_str("fresh mint"), py_str("dried mint")]), py_str("A handful of leaves for four, torn rather than chopped, or a teaspoon of dried mint. In a yogurt sauce or a salad it wants to be the last thing that goes in.")]), py_tuple(vec![py_tuple(vec![py_str("mixed herbs"), py_str("italian herbs"), py_str("herbes de provence"), py_str("dried herbs")]), py_str("A teaspoon of mixed dried herbs for four; a tablespoon if the pot is a big one and cooking for an hour or more, because the dried leaves give up their taste slowly.")]), py_tuple(vec![py_tuple(vec![py_str("spice"), py_str("spices"), py_str("spice mix"), py_str("seasoning"), py_str("mixed herbs and spices")]), py_str("A tablespoon of a ground spice blend for a dish that serves four, and a tablespoon over a kilo of chicken: taste after the first spoonful, because blends vary more in heat than their labels let on.")]), py_tuple(vec![py_tuple(vec![py_str("saffron")]), py_str("Three or four threads for a paella or a risotto for four, steeped in a spoonful of the hot stock first: any more than that and it starts to taste of the chemist's shelf.")]), py_tuple(vec![py_tuple(vec![py_str("star anise"), py_str("star aniseed"), py_str("stars anise")]), py_str("One whole star anise for a bottle of mulled wine or a slow-cooked pork for four: it is liquorice-strong, and two would take over the whole pot.")]), py_tuple(vec![py_tuple(vec![py_str("clove"), py_str("cloves"), py_str("ground clove"), py_str("ground cloves")]), py_str("A quarter teaspoon of ground cloves for a 20cm cake, or four to six whole ones in a bottle of mulled wine: it is strong, and more than that tastes medicinal.")])]) }
    s__SPICE_AMOUNTS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("plant"), py_str("planting"), py_str("planted"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("sowing"), py_str("good for"), py_str("bad for"), py_str("healthy"), py_str("safe"), py_str("allergy"), py_str("allergic"), py_str("ate"), py_str("eaten"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("baby"), py_str("toddler"), py_str("pregnan"), py_str("out of date"), py_str("gone off"), py_str("mouldy"), py_str("price"), py_str("prices"), py_str("water"), py_str("sun"), py_str("sunlight"), py_str("soil"), py_str("compost"), py_str("fertiliser"), py_str("fertilizer"), py_str("plant food"), py_str("in a pot"), py_str("in the garden"), py_str("cost"), py_str("buy"), py_str("cheapest"), py_str("substitute"), py_str("instead of"), py_str("swap"), py_str("each"), py_str("servings"), py_str("portions"), py_str("buns"), py_str("rolls"), py_str("batches"), py_str("a crowd"), py_str("how much is"), py_str("how much are"), py_str("how much for"), py_str("jar"), py_str("jars"), py_str("in a bulb"), py_str("in one bulb"), py_str("a bulb of"), py_str("bulb of"), py_str("in my coffee"), py_str("in coffee"), py_str("in my tea"), py_str("in tea"), py_str("too much"), py_str("how much is too much"), py_str("limit"), py_str("a day"), py_str("per day"), py_str("con carne"), py_str("bell pepper"), py_str("bell peppers"), py_str("red pepper"), py_str("green pepper"), py_str("yellow pepper"), py_str("orange pepper"), py_str("sweet pepper"), py_str("sweet peppers"), py_str("jalapeno"), py_str("jalapenos"), py_str("scotch bonnet"), py_str("romano"), py_str("padron")]) }
    s__SPICE_NOT_A_DOSE(__data2());
    s__MULLED_WINE(py_str("Four to six whole cloves, a cinnamon stick snapped in two and a star anise in a bottle of red: warm it gently for twenty minutes without letting it boil, and taste before you add any sugar."));
    s__PUMPKIN_LATTE(py_str("A quarter to half a teaspoon for a mug, whisked into the coffee with the milk and a little sugar: it is mostly cinnamon, so it needs the sugar to taste of anything."));
    fn __data3() -> Py { py_tuple(vec![py_str("clove"), py_str("cloves"), py_str("garlic"), py_str("pepper"), py_str("black pepper"), py_str("ground pepper"), py_str("peppercorns"), py_str("cinnamon"), py_str("vanilla"), py_str("baking powder"), py_str("bicarbonate"), py_str("bicarb"), py_str("baking soda"), py_str("mixed spice"), py_str("allspice"), py_str("pumpkin spice"), py_str("nutmeg"), py_str("ginger"), py_str("curry powder"), py_str("garam masala"), py_str("masala"), py_str("chilli powder"), py_str("chili powder"), py_str("cayenne"), py_str("chilli flakes"), py_str("chilli"), py_str("chili"), py_str("chillies"), py_str("turmeric"), py_str("paprika"), py_str("cumin"), py_str("coriander"), py_str("saffron"), py_str("oregano"), py_str("thyme"), py_str("rosemary"), py_str("sage"), py_str("bay leaf"), py_str("bay leaves"), py_str("bay"), py_str("basil"), py_str("parsley"), py_str("dill"), py_str("mint"), py_str("mixed herbs"), py_str("italian herbs"), py_str("dried herbs"), py_str("star anise"), py_str("cardamom"), py_str("fennel seeds"), py_str("mustard seeds"), py_str("spice"), py_str("spices"), py_str("spice mix"), py_str("seasoning")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("amount"), py_str("spoon"), py_str("spoons"), py_str("teaspoon"), py_str("teaspoons"), py_str("tablespoon"), py_str("tablespoons"), py_str("in a curry"), py_str("in the curry"), py_str("in a stew"), py_str("in the stew"), py_str("in a sauce"), py_str("in the sauce"), py_str("in a soup"), py_str("in the soup"), py_str("in a chilli"), py_str("in the chilli")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("amount"), py_str("spoon"), py_str("spoons"), py_str("teaspoon"), py_str("teaspoons"), py_str("tablespoon"), py_str("tablespoons"), py_str("grams"), py_str("handful")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spice-amount")); __args.push(py_tuple(vec![__data3(), __data4()])); __args.push(py_func("_spice_amount", |args: &[Py]| _spice_amount(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), __data5()), ("avoid".to_string(), op_add(&g__SPICE_NOT_A_DOSE(), &py_tuple(vec![py_str("calories"), py_str("carbs"), py_str("protein"), py_str("how many calories"), py_str("cholesterol")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _spice_amount(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut drink: Py = Py::None;
    let mut first_at: Py = Py::None;
    let mut first_len: Py = Py::None;
    let mut first_reply: Py = Py::None;
    let mut found: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut variant: Py = Py::None;
    let mut variants: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The figure for one spice, when the line asks how much of it goes in a dish.");
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SPICE_NOT_A_DOSE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("mulled wine")))) {
    return g__MULLED_WINE();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("pumpkin spice"))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("latte"), py_str("coffee"), py_str("cappuccino"), py_str("mug"), py_str("cup")])) { let mut drink: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &drink.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return g__PUMPKIN_LATTE();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0), py_str("")]), 3, None);
    first_at = __unpk1[0].clone();
    first_len = __unpk1[1].clone();
    first_reply = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&g__SPICE_AMOUNTS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    variants = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&variants.clone());
    for __item in __iter4 {
    variant = __item.clone();
    found = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &variant.clone()), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    continue;
    }
    at = mcall(&found.clone(), "start", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&first_at.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = op_lt(&at.clone(), &first_at.clone()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&at.clone(), &first_at.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[variant.clone()]).unwrap_or_else(|e| exc::raise(e)), &first_len.clone()) } } } } } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![at.clone(), call_builtin("len", &[variant.clone()]).unwrap_or_else(|e| exc::raise(e)), reply.clone()]), 3, None);
    first_at = __unpk5[0].clone();
    first_len = __unpk5[1].clone();
    first_reply = __unpk5[2].clone();
    }
    }
    }
    return first_reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.seasoning".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_MULLED_WINE" => Some(g__MULLED_WINE()),
            "_PUMPKIN_LATTE" => Some(g__PUMPKIN_LATTE()),
            "_SPICE_AMOUNTS" => Some(g__SPICE_AMOUNTS()),
            "_SPICE_NOT_A_DOSE" => Some(g__SPICE_NOT_A_DOSE()),
            "_spice_amount" => Some(py_func("_spice_amount", |args: &[Py]| _spice_amount(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_MULLED_WINE" => s__MULLED_WINE(v),
            "_PUMPKIN_LATTE" => s__PUMPKIN_LATTE(v),
            "_SPICE_AMOUNTS" => s__SPICE_AMOUNTS(v),
            "_SPICE_NOT_A_DOSE" => s__SPICE_NOT_A_DOSE(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.seasoning", module_obj);
    pyrt::xrefs::reg_fn("knowledge.seasoning", "_spice_amount", py_func("_spice_amount", |args: &[Py]| _spice_amount(args)));
    pyrt::xrefs::reg_access("knowledge.seasoning", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.seasoning", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.seasoning", "_MULLED_WINE", g__MULLED_WINE, s__MULLED_WINE);
    pyrt::xrefs::reg_access("knowledge.seasoning", "_PUMPKIN_LATTE", g__PUMPKIN_LATTE, s__PUMPKIN_LATTE);
    pyrt::xrefs::reg_access("knowledge.seasoning", "_SPICE_AMOUNTS", g__SPICE_AMOUNTS, s__SPICE_AMOUNTS);
    pyrt::xrefs::reg_access("knowledge.seasoning", "_SPICE_NOT_A_DOSE", g__SPICE_NOT_A_DOSE, s__SPICE_NOT_A_DOSE);
    pyrt::xrefs::reg_access("knowledge.seasoning", "entry", g_entry, s_entry);
}
