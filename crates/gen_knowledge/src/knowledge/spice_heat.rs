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
thread_local! { static G_EXTRA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SPICE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_EXTRA() -> Py { ensure_init(); G_EXTRA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA(v: Py) { ensure_init(); G_EXTRA.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAT() -> Py { ensure_init(); G_HEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAT(v: Py) { ensure_init(); G_HEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT() -> Py { ensure_init(); G_HOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT(v: Py) { ensure_init(); G_HOT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SPICE_LINE() -> Py { ensure_init(); G_NOT_A_SPICE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SPICE_LINE(v: Py) { ensure_init(); G_NOT_A_SPICE_LINE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How hot a named spice is: one table, one row for each line of it.\n\n\"Is paprika spicy\", \"how hot is cumin\" and \"is cinnamon hot\" all reached a search hedge, and\n\"how hot is a paprika\" reached the weather card because the only \"hot\" the bot knew about was\nthe outdoors. Heat is a fact about a named spice, not a judgement, so it lives here as a table:\nthe names people write on a jar, the band the spice sits in, and the one thing worth knowing\nabout using it. Adding a spice is adding a line.\n\nEvery row is keyed on the names plus the words that ask the question, so \"is cayenne spicy\",\n\"how hot is cayenne\", \"how strong is cayenne\" and \"does cayenne make it hot\" reach the same\nverdict without the wording mattering. The ranges are the ordinary published Scoville bands and\nthey are said as ranges, because the same spice varies from jar to jar.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_HOT(py_tuple(vec![py_str("how hot"), py_str("how spicy"), py_str("is it hot"), py_str("is it spicy"), py_str("spicy"), py_str("hot"), py_str("heat"), py_str("hotter"), py_str("mild"), py_str("milder"), py_str("mildest"), py_str("hottest"), py_str("scoville"), py_str("strong"), py_str("strength"), py_str("fiery"), py_str("burn"), py_str("burns"), py_str("burning"), py_str("kick"), py_str("too much heat")]));
    s_NOT_A_SPICE_LINE(py_tuple(vec![py_str("engine"), py_str("coolant"), py_str("radiator"), py_str("brake"), py_str("porsche"), py_str("car"), py_str("van"), py_str("boiler"), py_str("heating"), py_str("oven temperature"), py_str("how hot should the oven"), py_str("weather"), py_str("forecast"), py_str("outside"), py_str("today"), py_str("tomorrow"), py_str("sun"), py_str("barbecue"), py_str("bbq"), py_str("instead of"), py_str("substitute"), py_str("swap"), py_str("replace"), py_str("where can i buy"), py_str("cheap"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("sow"), py_str("seedling"), py_str("recipe"), py_str("how do i make"), py_str("make my own"), py_str("how much"), py_str("how many")]));
    s_HEAT(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("paprika"), py_str("sweet paprika"), py_str("hungarian paprika")]), py_str("Paprika is mild, around 1,000 Scoville - closer to a red pepper than to a chilli. A teaspoon colours a pot and sweetens it more than it heats it, which is why a goulash can take two spoons and still be gentle.")]), py_tuple(vec![py_tuple(vec![py_str("smoked paprika"), py_str("smoky paprika"), py_str("pimenton")]), py_str("Smoked paprika is mild too, around 1,000 Scoville - the smoke comes from drying the peppers over oak, not from heat. Half a teaspoon is enough to taste it; more turns a dish bitter rather than hotter.")]), py_tuple(vec![py_tuple(vec![py_str("cayenne"), py_str("cayenne pepper")]), py_str("Cayenne is hot: about 30,000 to 50,000 Scoville, so a quarter teaspoon heats a whole pot. It has little taste of its own, which is why it is the thing to reach for when a dish needs heat and not another flavour.")]), py_tuple(vec![py_tuple(vec![py_str("chilli powder"), py_str("chili powder")]), py_str("Chilli powder is a blend, usually mild to medium with cumin and oregano in it, so the heat depends on the jar - some are mostly cayenne. Taste a pinch on your tongue before you decide how much goes in.")]), py_tuple(vec![py_tuple(vec![py_str("chilli flakes"), py_str("chili flakes"), py_str("red pepper flakes"), py_str("crushed chillies")]), py_str("Chilli flakes are medium-hot, around 30,000 to 50,000 Scoville, and they give their heat slowly: a pinch in oil at the start is warmer than the same pinch scattered at the end.")]), py_tuple(vec![py_tuple(vec![py_str("curry powder")]), py_str("Curry powder is mild - it is turmeric, cumin, coriander and fenugreek, and it is not where the heat in a curry comes from. Two spoons in a curry for four is about right, and the heat is added separately with chilli.")]), py_tuple(vec![py_tuple(vec![py_str("garam masala")]), py_str("Garam masala is warm, not hot: cinnamon, cardamom, cloves and pepper, ground together. A teaspoon goes in at the end of cooking, where the smell survives; boiled for an hour it tastes of dust.")]), py_tuple(vec![py_tuple(vec![py_str("cumin"), py_str("jeera")]), py_str("Cumin is not hot at all - it is warm, earthy and slightly bitter, and it is the smell of a curry rather than the fire in it. A teaspoon for four, fried in the oil for a few seconds first.")]), py_tuple(vec![py_tuple(vec![py_str("turmeric"), py_str("haldi")]), py_str("Turmeric is not hot: earthy and a little bitter, and too much of it tastes of soap. Half a teaspoon for four is plenty, and it stains a wooden spoon and a white shirt for good.")]), py_tuple(vec![py_tuple(vec![py_str("cinnamon"), py_str("cassia")]), py_str("Cinnamon is warm and sweet, not hot. The sticks sold cheaply are cassia, which is stronger and slightly bitter - fine in a curry, and worth knowing if a recipe came out tasting of cloves.")]), py_tuple(vec![py_tuple(vec![py_str("black pepper"), py_str("peppercorns"), py_str("ground pepper")]), py_str("Black pepper is mildly hot, at the back of the throat, and nothing like chilli: a whole teaspoon of it is gentler than a pinch of cayenne. Freshly ground is far sharper than the dust in a jar.")]), py_tuple(vec![py_tuple(vec![py_str("white pepper")]), py_str("White pepper is hotter than black, with a farmyard smell that suits soups, congee and Chinese cooking. Use half as much as you would black.")]), py_tuple(vec![py_tuple(vec![py_str("mustard powder"), py_str("english mustard"), py_str("mustard")]), py_str("Mustard is hot in a different place from chilli - it burns in the nose and fades in a few seconds. Mixed with water it is hottest; vinegar or hot liquid takes the edge off it.")]), py_tuple(vec![py_tuple(vec![py_str("ginger"), py_str("root ginger"), py_str("ginger powder")]), py_str("Ginger is warm and sharp rather than hot like chilli, and the burn fades in seconds. A thumb-sized piece grated into a curry for four is a good amount.")]), py_tuple(vec![py_tuple(vec![py_str("garlic powder"), py_str("garlic granules")]), py_str("Garlic powder is not hot at all - it is savoury and a little sweet, and it burns if it sits in hot oil, so add it with the liquid.")]), py_tuple(vec![py_tuple(vec![py_str("coriander"), py_str("coriander seed"), py_str("ground coriander")]), py_str("Coriander seed is not hot: it tastes of orange peel and hay, and it is the rounded body in a curry powder. Toast it before grinding and the smell doubles.")]), py_tuple(vec![py_tuple(vec![py_str("nutmeg")]), py_str("Nutmeg is warm and sweet, not hot - a quarter teaspoon in a cheese sauce or a pudding is the usual dose, and more tastes of medicine.")]), py_tuple(vec![py_tuple(vec![py_str("cloves"), py_str("ground cloves")]), py_str("Cloves are warm and very strong, not hot, and two or three in a pot is enough. Too many and the dish tastes of the dentist.")]), py_tuple(vec![py_tuple(vec![py_str("cardamom")]), py_str("Cardamom is aromatic, not hot: menthol and flowers. Crush the pods just before they go in, because the seeds lose their smell within a day of grinding.")]), py_tuple(vec![py_tuple(vec![py_str("five spice"), py_str("chinese five spice")]), py_str("Five spice is not hot - star anise, fennel, cinnamon, cloves and pepper - and it is strong, so half a teaspoon for four is the usual amount.")]), py_tuple(vec![py_tuple(vec![py_str("sumac")]), py_str("Sumac is not hot: it is sour and fruity, the taste of lemon without the wet. A teaspoon over a salad or grilled meat at the end.")]), py_tuple(vec![py_tuple(vec![py_str("horseradish"), py_str("wasabi")]), py_str("Horseradish is hot in the nose, like mustard, and the burn lasts seconds rather than minutes. A teaspoon of the creamed sort is plenty for a plate of beef.")]), py_tuple(vec![py_tuple(vec![py_str("sriracha")]), py_str("Sriracha is mild to medium, around 2,000 Scoville - ketchup-hot rather than chilli-hot, and sweet. A spoon per bowl adds heat slowly enough to taste as you go.")]), py_tuple(vec![py_tuple(vec![py_str("tabasco"), py_str("tabasco sauce")]), py_str("Tabasco is about 2,500 Scoville, sharper and much saltier than sriracha, so a few shakes per plate. It is vinegar-forward: added early it dulls, added at the table it does not.")]), py_tuple(vec![py_tuple(vec![py_str("gochujang")]), py_str("Gochujang is medium, around 1,000 to 2,000 Scoville, and sweet and fermented with it, so the heat arrives after the sweetness. A tablespoon for four.")]), py_tuple(vec![py_tuple(vec![py_str("harissa")]), py_str("Harissa is medium-hot, roughly 4,000 to 5,000 Scoville, with cumin and garlic in the paste. A spoon per portion stirred into the sauce at the end.")]), py_tuple(vec![py_tuple(vec![py_str("jalapeno"), py_str("jalapenos"), py_str("jalapeño")]), py_str("A jalapeño is mild to medium, 2,500 to 8,000 Scoville: hot enough to eat raw in slices, and much milder pickled or cooked. The seeds and white pith hold most of the heat.")]), py_tuple(vec![py_tuple(vec![py_str("serrano"), py_str("serranos")]), py_str("A serrano is 10,000 to 23,000 Scoville - two or three times a jalapeño, with a cleaner, greener taste. Half a one is enough for a curry for four.")]), py_tuple(vec![py_tuple(vec![py_str("poblano"), py_str("poblanos")]), py_str("A poblano is mild, 1,000 to 1,500 Scoville - roasted and stuffed it tastes of smoke and green pepper, not of heat.")]), py_tuple(vec![py_tuple(vec![py_str("ancho"), py_str("anchos")]), py_str("An ancho is a dried poblano: mild, 1,000 to 2,000 Scoville, raisin-sweet and a little smoky. Two soaked in a sauce for four give colour and depth without heat.")]), py_tuple(vec![py_tuple(vec![py_str("chipotle"), py_str("chipotles"), py_str("chipotle in adobo")]), py_str("A chipotle is a smoked, dried jalapeño, 2,500 to 8,000 Scoville. The adobo it comes in is milder than the pepper itself - one pepper and a spoon of the sauce for a pot for four.")]), py_tuple(vec![py_tuple(vec![py_str("habanero"), py_str("habaneros"), py_str("habañero")]), py_str("A habanero is very hot, 100,000 to 350,000 Scoville, and fruity with it. A quarter of one heats a pot for four, and it is worth wearing gloves for a batch: the oil stays on hands for hours and soap barely shifts it.")]), py_tuple(vec![py_tuple(vec![py_str("scotch bonnet"), py_str("scotch bonnets")]), py_str("A scotch bonnet is very hot, 100,000 to 350,000 Scoville, with a fruity, almost apricot taste that survives the heat. One, deseeded, is enough for a family curry.")]), py_tuple(vec![py_tuple(vec![py_str("allspice")]), py_str("Allspice is warm and peppery, not hot - it tastes of cinnamon, clove and nutmeg at once, which is where the name comes from. A few berries in a stew or a cake, and no heat at all.")]), py_tuple(vec![py_tuple(vec![py_str("star anise")]), py_str("Star anise is not hot: it is liquorice-sweet and very strong, so one star in a pot for four. Two and the dish tastes of cough sweets.")]), py_tuple(vec![py_tuple(vec![py_str("fennel seed"), py_str("fennel seeds"), py_str("fennel")]), py_str("Fennel seed is not hot - it is sweet and aniseedy, and it is half the taste of a sausage. A teaspoon crushed in a curry for four.")]), py_tuple(vec![py_tuple(vec![py_str("fenugreek"), py_str("methi")]), py_str("Fenugreek is not hot: it is bitter and maple-sweet, and a little goes a long way. The leaves are gentler than the seeds, which turn a dish bitter if they are fried too hard.")]), py_tuple(vec![py_tuple(vec![py_str("asafoetida"), py_str("hing")]), py_str("Asafoetida is not hot, but it is ferociously strong - a pinch in hot oil, no more, and it smells of onions and garlic once it has cooked. A quarter teaspoon for a pot for four.")]), py_tuple(vec![py_tuple(vec![py_str("curry leaves"), py_str("curry leaf")]), py_str("Curry leaves are not hot at all: they are citrusy and nutty, and they only give that up when fried in hot oil for a few seconds at the start. Dried ones taste of very little.")]), py_tuple(vec![py_tuple(vec![py_str("zaatar"), py_str("za'atar")]), py_str("Za'atar is not hot: it is thyme, sesame and sumac, so it is herby and sour. A spoon over bread or yoghurt, not a source of heat.")]), py_tuple(vec![py_tuple(vec![py_str("berbere")]), py_str("Berbere is medium to hot, roughly 3,000 to 5,000 Scoville - a blend of chilli, paprika, fenugreek and warm spices. A tablespoon for four gives an Ethiopian stew its heat.")]), py_tuple(vec![py_tuple(vec![py_str("jerk seasoning"), py_str("jerk spice"), py_str("jamaican jerk")]), py_str("Jerk seasoning is hot: it is scotch bonnet with allspice, thyme and sugar, so it can be 100,000 Scoville or more. A tablespoon per kilo of chicken, and less if the jar is old - chilli loses its heat over a year or two.")]), py_tuple(vec![py_tuple(vec![py_str("peri peri"), py_str("piri piri")]), py_str("Peri peri is hot, 50,000 to 175,000 Scoville, and sharp with lemon and vinegar. A spoon in a marinade for four is a strong heat once it is cooked down.")]), py_tuple(vec![py_tuple(vec![py_str("sambal"), py_str("sambal oelek")]), py_str("Sambal is medium to hot, 2,000 to 10,000 Scoville depending on the jar, and salty and sour with it. A teaspoon per portion stirred in at the end.")]), py_tuple(vec![py_tuple(vec![py_str("ras el hanout")]), py_str("Ras el hanout is a Moroccan blend and not usually hot - cumin, coriander, cinnamon, ginger and pepper - though some jars have chilli in. A tablespoon for a tagine or a tray of vegetables.")]), py_tuple(vec![py_tuple(vec![py_str("togarashi"), py_str("shichimi togarashi"), py_str("nanami togarashi")]), py_str("Togarashi is mild to medium, mostly sesame, orange peel and seaweed with a little chilli, so it is a finishing sprinkle rather than a heat source.")]), py_tuple(vec![py_tuple(vec![py_str("chilli oil"), py_str("chili oil"), py_str("chili crisp"), py_str("chilli crisp")]), py_str("Chilli oil is as hot as the chillies that went into it - usually medium, around 10,000 Scoville - and the crisp bits are hotter than the oil. A teaspoon over noodles.")]), py_tuple(vec![py_tuple(vec![py_str("hot sauce")]), py_str("Hot sauce covers everything from 500 to 300,000 Scoville, so the bottle is the only way to know: Tabasco is about 2,500, sriracha about 2,000, and a scotch bonnet sauce can be a hundred times hotter. Taste a drop before you commit.")]), py_tuple(vec![py_tuple(vec![py_str("birds eye"), py_str("bird's eye"), py_str("bird eye chilli"), py_str("thai chilli")]), py_str("A bird's eye chilli is hot, 50,000 to 100,000 Scoville - one or two will heat a curry for four, and they are hotter dried than fresh.")])]));
    s_EXTRA(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spice-hottest-chilli")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hottest chilli"), py_str("hottest chili"), py_str("hottest pepper"), py_str("hottest chillies"), py_str("worlds hottest"), py_str("strongest chilli")]), py_tuple(vec![py_str("which"), py_str("what"), py_str("is the"), py_str("hottest"), py_str("name")])])); __args.push(py_str("The hottest chilli on the ordinary record is Pepper X at around 2.7 million Scoville, after the Carolina Reaper at about 1.6 million - both too hot to eat raw, and used in sauces at a fraction of a percent. A habanero at 300,000 is already a hundred times a jalapeño.")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_SPICE_LINE()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spice-mildest-chilli")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mildest chilli"), py_str("mildest chili"), py_str("mildest pepper"), py_str("mild chilli"), py_str("mild chillies")]), py_tuple(vec![py_str("which"), py_str("what"), py_str("is the"), py_str("mildest"), py_str("name"), py_str("recommend")])])); __args.push(py_str("The mildest chillies are the ones grown to be eaten like vegetables: shishito, around 100 Scoville and mild nine times in ten, and poblano at 1,000 to 1,500. Deseeding a hotter one takes most of the heat out without losing the taste.")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_SPICE_LINE()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    s_ENTRIES(op_add(&_rows(&[]), &g_EXTRA()));
}
pub fn _rows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut built: Py = Py::None;
    let mut names: Py = Py::None;
    let mut number: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("One row per spice: the names it is written as, and the verdict, in the same shape.");
    built = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[g_HEAT()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    number = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    names = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    let _ = mcall(&built.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("spice-heat-"), &mcall(&mcall(&geti(&names.clone(), &Py::Int(0)), "replace", &[py_str(" "), py_str("-")]), "replace", &[py_str("'"), py_str("")]))); __args.push(py_tuple(vec![names.clone(), g_HOT()])); __args.push(reply.clone()); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_A_SPICE_LINE(), &py_tuple(vec![py_str("instead"), py_str("substitute"), py_str("ran out"), py_str("my curry"), py_str("too spicy"), py_str("too hot"), py_str("what does a dash"), py_str("what is a dash"), py_str("how much is a dash"), py_str("dash mean"), py_str("what does a pinch"), py_str("what is a pinch"), py_str("how much is a pinch"), py_str("pinch mean")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    }
    return call_builtin("tuple", &[built.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.spice_heat".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EXTRA" => Some(g_EXTRA()),
            "HEAT" => Some(g_HEAT()),
            "HOT" => Some(g_HOT()),
            "NOT_A_SPICE_LINE" => Some(g_NOT_A_SPICE_LINE()),
            "_rows" => Some(py_func("_rows", |args: &[Py]| _rows(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "EXTRA" => s_EXTRA(v),
            "HEAT" => s_HEAT(v),
            "HOT" => s_HOT(v),
            "NOT_A_SPICE_LINE" => s_NOT_A_SPICE_LINE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.spice_heat", module_obj);
    pyrt::xrefs::reg_fn("knowledge.spice_heat", "_rows", py_func("_rows", |args: &[Py]| _rows(args)));
    pyrt::xrefs::reg_access("knowledge.spice_heat", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.spice_heat", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.spice_heat", "EXTRA", g_EXTRA, s_EXTRA);
    pyrt::xrefs::reg_access("knowledge.spice_heat", "HEAT", g_HEAT, s_HEAT);
    pyrt::xrefs::reg_access("knowledge.spice_heat", "HOT", g_HOT, s_HOT);
    pyrt::xrefs::reg_access("knowledge.spice_heat", "NOT_A_SPICE_LINE", g_NOT_A_SPICE_LINE, s_NOT_A_SPICE_LINE);
}
