#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_UNTIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_COOKING_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_KEEPING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_STEP_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _GARLIC_EMULSION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _GARLIC_YOGHURT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IS_A_BRANDY_BUTTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IS_A_SAUCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MADE_IN_THE_PAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_RECIPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAUCES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAUCE_OR_HOLLANDAISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAUCE_OR_TOUM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g__AN_UNTIL() -> Py { ensure_init(); _AN_UNTIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_UNTIL(v: Py) { ensure_init(); if let Ok(mut w) = _AN_UNTIL.write() { *w = v; } }
pub fn g__ASKING() -> Py { ensure_init(); _ASKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKING(v: Py) { ensure_init(); if let Ok(mut w) = _ASKING.write() { *w = v; } }
pub fn g__A_COOKING_TIME() -> Py { ensure_init(); _A_COOKING_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_COOKING_TIME(v: Py) { ensure_init(); if let Ok(mut w) = _A_COOKING_TIME.write() { *w = v; } }
pub fn g__A_KEEPING_WORD() -> Py { ensure_init(); _A_KEEPING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_KEEPING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_KEEPING_WORD.write() { *w = v; } }
pub fn g__A_STEP_LENGTH() -> Py { ensure_init(); _A_STEP_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_STEP_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = _A_STEP_LENGTH.write() { *w = v; } }
pub fn g__GARLIC_EMULSION() -> Py { ensure_init(); _GARLIC_EMULSION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__GARLIC_EMULSION(v: Py) { ensure_init(); if let Ok(mut w) = _GARLIC_EMULSION.write() { *w = v; } }
pub fn g__GARLIC_YOGHURT() -> Py { ensure_init(); _GARLIC_YOGHURT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__GARLIC_YOGHURT(v: Py) { ensure_init(); if let Ok(mut w) = _GARLIC_YOGHURT.write() { *w = v; } }
pub fn g__IS_A_BRANDY_BUTTER() -> Py { ensure_init(); _IS_A_BRANDY_BUTTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IS_A_BRANDY_BUTTER(v: Py) { ensure_init(); if let Ok(mut w) = _IS_A_BRANDY_BUTTER.write() { *w = v; } }
pub fn g__IS_A_SAUCE() -> Py { ensure_init(); _IS_A_SAUCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IS_A_SAUCE(v: Py) { ensure_init(); if let Ok(mut w) = _IS_A_SAUCE.write() { *w = v; } }
pub fn g__KEEPING() -> Py { ensure_init(); _KEEPING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING.write() { *w = v; } }
pub fn g__KEEPS() -> Py { ensure_init(); _KEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPS(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPS.write() { *w = v; } }
pub fn g__MADE_IN_THE_PAN() -> Py { ensure_init(); _MADE_IN_THE_PAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MADE_IN_THE_PAN(v: Py) { ensure_init(); if let Ok(mut w) = _MADE_IN_THE_PAN.write() { *w = v; } }
pub fn g__NOT_A_RECIPE() -> Py { ensure_init(); _NOT_A_RECIPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_RECIPE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_RECIPE.write() { *w = v; } }
pub fn g__SAUCES() -> Py { ensure_init(); _SAUCES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAUCES(v: Py) { ensure_init(); if let Ok(mut w) = _SAUCES.write() { *w = v; } }
pub fn g__SAUCE_OR_HOLLANDAISE() -> Py { ensure_init(); _SAUCE_OR_HOLLANDAISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAUCE_OR_HOLLANDAISE(v: Py) { ensure_init(); if let Ok(mut w) = _SAUCE_OR_HOLLANDAISE.write() { *w = v; } }
pub fn g__SAUCE_OR_TOUM() -> Py { ensure_init(); _SAUCE_OR_TOUM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAUCE_OR_TOUM(v: Py) { ensure_init(); if let Ok(mut w) = _SAUCE_OR_TOUM.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The sauces that go with a roast, one row each, keyed by the sauce rather than the wording.\n\nMain answered \"how do i make bread sauce\" with a white loaf - 500g of strong flour, 7g of yeast,\n350ml of water - because the word \"bread\" is the recipe's and the sauce was never a subject. The\nsame shape took the other three: \"how do i make cranberry sauce\" and \"how do i make mint sauce\"\nwere shrugs, and \"how do i make apple sauce\" was answered with the goodness of apple skin. A roast\ndinner is the meal people cook for a table of guests and the sauces are the part they ask about,\nso the four are here, with the horseradish and the brandy butter that go with the beef and the\npudding.\n\nWhy its own domain: each sauce is a short recipe and a keeping rule, and cooking_entries.py is\nwhere one-off rows go - this is a table of them. It answers the cooking topic and is gated on the\nword \"sauce\", so the herb itself, the fruit itself and the cranberry juice are somebody else's\nquestion.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s__IS_A_SAUCE(py_tuple(vec![py_str("sauce"), py_str("sauces"), py_str("sauce."), py_str("sauces?")]));
    s__ASKING(py_tuple(vec![py_str("sauce"), py_str("sauces"), py_str("make"), py_str("makes"), py_str("making"), py_str("made"), py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("cooking"), py_str("how"), py_str("what"), py_str("whats"), py_str("need"), py_str("want"), py_str("christmas"), py_str("turkey"), py_str("roast"), py_str("lamb"), py_str("beef"), py_str("pork"), py_str("goose"), py_str("ham"), py_str("dinner"), py_str("homemade"), py_str("from scratch"), py_str("jar"), py_str("bought"), py_str("ahead"), py_str("make it ahead"), py_str("ahead of time"), py_str("in advance"), py_str("the day before"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("last"), py_str("lasts"), py_str("freeze"), py_str("freezes"), py_str("freezer"), py_str("freezing"), py_str("frozen"), py_str("defrost"), py_str("fridge"), py_str("refrigerate"), py_str("how long"), py_str("how much"), py_str("sugar"), py_str("vinegar"), py_str("hot"), py_str("mild"), py_str("quick"), py_str("easy"), py_str("best"), py_str("good"), py_str("nice"), py_str("serve"), py_str("served"), py_str("with"), py_str("goes")]));
    s__NOT_A_RECIPE(py_tuple(vec![py_str("lumpy"), py_str("lumps"), py_str("lump"), py_str("thick"), py_str("thicker"), py_str("thicken"), py_str("thickened"), py_str("thickening"), py_str("runny"), py_str("thin"), py_str("watery"), py_str("sweet"), py_str("sweeter"), py_str("too sweet"), py_str("sharp"), py_str("sour"), py_str("bitter"), py_str("salty"), py_str("bland"), py_str("separated"), py_str("split"), py_str("curdled"), py_str("grainy"), py_str("gelatinous"), py_str("out of date"), py_str("use by"), py_str("gone off"), py_str("mouldy")]));
    s__KEEPING(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|kept|last|lasts|lasting|freeze|freezes|freezer|freezing|frozen|defrost|fridge|refrigerated|refrigerate|how long|how many days|till|until)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_KEEPING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|kept|last|lasts|lasting|freeze|freezes|freezer|freezing|frozen|defrost|fridge|refrigerated|refrigerate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_COOKING_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:how long|how many minutes)\\b[^.?!]{0,45}?\\b(?:cook|cooks|cooking|simmer|simmering|boil|boiling|bake|baking|roast|roasting|microwave|microwaving|reheat|reheating|stir|stirring|reduce|reducing|whisk|whisking|take|takes|leave|on the stove|on the hob|in the oven|stove|hob|oven)\\b|\\b(?:simmer|simmering|cook|cooking|boil|boiling|bake|baking|roast|roasting|stir|stirring|reduce|reducing|whisk|whisking)\\b[^.?!]{0,30}?\\buntil\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_UNTIL(stdlib_call("re", "compile", &[py_str("\\b(?:till|until)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_STEP_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:how long|how many minutes|how many hours)\\b[^.?!]{0,40}?\\b(?:sit|sits|sitting|rest|rests|resting|stand|stands|standing|set|sets|setting|soak|soaks|soaking|marinate|marinating|chill|chilling|before (?:you )?serv|before use|to cool)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__KEEPS(py_dict(vec![(py_str("bread sauce"), py_str("Bread sauce is best the day it is made. It keeps 2 days covered in the fridge, and it reheats gently with a splash of milk in a pan rather than in the microwave, which makes the crumbs grainy. It does not freeze well: the sauce splits and the breadcrumbs go gritty, so make it fresh and reheat what is left.")), (py_str("cranberry sauce"), py_str("In a clean jar with a lid the sauce keeps 2 weeks in the fridge, and it freezes for 3 months in small tubs. Thaw it overnight, because the set loosens a little and a stir brings it back; it keeps a week once a jar has been opened.")), (py_str("mint sauce"), py_str("Mint sauce keeps about a month in a sealed jar in the fridge, and it does not freeze: the mint goes black and limp and the vinegar takes over. Make it in small amounts and stir in fresh mint if it has been sitting a while.")), (py_str("apple sauce"), py_str("Apple sauce keeps 4 to 5 days in a covered dish in the fridge and freezes for 3 months in bags or tubs. Thaw it overnight and stir it, because it can look watery once it defrosts - a spoon of butter or a quick beat brings it back.")), (py_str("horseradish sauce"), py_str("Fresh horseradish sauce keeps 3 or 4 days in the fridge, and it does not freeze well: the cream splits and the heat fades. A jarred one lasts months in the fridge once opened.")), (py_str("brandy butter"), py_str("Brandy butter keeps 2 weeks covered in the fridge and freezes for 3 months - roll it in a log and slice it cold. Let it soften a few minutes at the table but do not leave it out of the fridge for long."))]));
    s__SAUCE_OR_TOUM(add(&g__IS_A_SAUCE(), &py_tuple(vec![py_str("toum")])));
    s__SAUCE_OR_HOLLANDAISE(add(&g__IS_A_SAUCE(), &py_tuple(vec![py_str("hollandaise"), py_str("bearnaise"), py_str("béarnaise")])));
    s__IS_A_BRANDY_BUTTER(py_tuple(vec![py_str("brandy butter"), py_str("hard sauce"), py_str("brandy cream"), py_str("brandy butter sauce")]));
    s__GARLIC_EMULSION(py_str("Crush 4 peeled garlic cloves with a good pinch of salt to a smooth paste, then add 200ml of neutral oil a few drops at a time, beating all the time, and a little lemon juice whenever it thickens - the garlic takes up the oil and it turns white and thick like mayonnaise. If it splits, put a fresh clove in a clean bowl and beat the split batch into it a spoonful at a time. For the quick yoghurt version, stir crushed garlic, lemon and salt into 200g of thick yoghurt."));
    s__GARLIC_YOGHURT(py_str("Crush one garlic clove to a paste with a good pinch of salt - crushing rather than chopping is what stops it biting - then stir it into 200g of thick Greek yoghurt with a squeeze of lemon and leave it 10 minutes. A tablespoon of mayonnaise keeps it from splitting when it meets hot food, and grated cucumber, mint or a little cumin turns it into a dip."));
    s__SAUCES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("bread sauce"), py_str("breadsauce")]), py_str("Simmer 300ml of whole milk with half an onion studded with 4 cloves and a bay leaf for 10 minutes, then fish the onion out, stir in 100g of fresh white breadcrumbs and simmer 3 to 4 minutes until it is thick and creamy. Season it with salt, pepper and grated nutmeg, and stir in a knob of butter at the end. Make it an hour ahead and reheat it gently with a splash of milk."), py_tuple(vec![py_str("stuffing"), py_str("stuffing a chicken"), py_str("sandwich"), py_str("toast"), py_str("breadcrumbs for"), py_str("stale bread")])]), py_tuple(vec![py_tuple(vec![py_str("cranberry sauce"), py_str("cranberry jam"), py_str("cranberries"), py_str("cranberry")]), py_str("Tip 250g of cranberries into a pan with 100g of sugar, the zest and juice of an orange and 2 tablespoons of water, and simmer it 8 to 10 minutes, until the berries pop and it is thick but still a little chunky. It firms up as it cools, so take it off while it is looser than you want it on the plate. A jar keeps a week in the fridge and it freezes well."), py_tuple(vec![py_str("juice"), py_str("uti"), py_str("water infection"), py_str("cystitis"), py_str("vitamin"), py_str("good for you"), py_str("healthy"), py_str("dried"), py_str("supplement"), py_str("tablets"), py_str("grow"), py_str("growing"), py_str("bush"), py_str("bog")])]), py_tuple(vec![py_tuple(vec![py_str("mint sauce"), py_str("mint jelly")]), py_str("Pour 4 tablespoons of boiling water over 4 tablespoons of sugar and stir until it has dissolved, add 4 tablespoons of white wine vinegar, then stir in a big handful of mint chopped as finely as you can and leave it 20 minutes. That is the sweet-sharp mint sauce for lamb; mint jelly is the bought set version, and a spoon of redcurrant jelly with fresh mint does the same job in a hurry."), py_tuple(vec![py_str("grow"), py_str("growing"), py_str("plant"), py_str("pot"), py_str("dying"), py_str("wilted"), py_str("yellow"), py_str("spread"), py_str("take over"), py_str("peppermint"), py_str("tea"), py_str("oil"), py_str("leaves in")])]), py_tuple(vec![py_tuple(vec![py_str("apple sauce"), py_str("applesauce"), py_str("apple compote")]), py_str("Peel, core and chop 3 cooking apples, put them in a pan with 2 tablespoons of water and 1 of sugar, and simmer 8 to 10 minutes with the lid on until they collapse. Beat them smooth or leave it chunky, and stir in a knob of butter for a richer one. Bramleys are the apples that go fluffy; eaters stay in lumps and taste sweeter."), py_tuple(vec![py_str("skin"), py_str("peel"), py_str("good for you"), py_str("healthy"), py_str("vitamin"), py_str("baby"), py_str("weaning"), py_str("tree"), py_str("grow"), py_str("orchard"), py_str("variety"), py_str("bake in the oven")])]), py_tuple(vec![py_tuple(vec![py_str("horseradish sauce"), py_str("horseradish cream"), py_str("horseradish")]), py_str("Grate fresh horseradish root finely - or use a jarred one - and fold about 2 tablespoons of it into 4 tablespoons of creme fraiche or whipped double cream with a squeeze of lemon, a pinch of salt and a little sugar. Grate it near an open window or in a food processor: the heat is in the fumes, not the flesh, and it hits the back of your nose."), py_tuple(vec![py_str("grow"), py_str("growing"), py_str("plant"), py_str("root in the garden"), py_str("is it a weed"), py_str("leaves")])]), py_tuple(vec![py_tuple(vec![py_str("brandy butter"), py_str("hard sauce"), py_str("brandy cream")]), py_str("Beat 100g of soft butter with 100g of icing sugar until it is pale, then add 3 to 4 tablespoons of brandy a spoonful at a time: it should be stiff enough to hold a spoon upright. Chill it in a dish and serve it cold with Christmas pudding or mince pies, where it melts into the pudding rather than sitting on top of it."), py_tuple(vec![py_str("how much brandy"), py_str("drunk"), py_str("alcohol free"), py_str("without alcohol"), py_str("for a child")]), g__IS_A_BRANDY_BUTTER()]), py_tuple(vec![py_tuple(vec![py_str("peppercorn sauce"), py_str("green peppercorn sauce"), py_str("pepper sauce"), py_str("peppercorn")]), py_str("Pour off most of the fat from the pan you cooked the steak in and set it back on a medium heat: add a splash of brandy or red wine and scrape the brown bits up, then 150ml of stock and a tablespoon of crushed green peppercorns, and let it reduce by half. Take it off the heat, stir in 100ml of double cream and a teaspoon of Dijon, and put it back for a minute until it coats a spoon. Season at the end and taste first: reduced stock is already salty, and a sauce made with cream takes a lot of pepper."), py_tuple(vec![py_str("grow"), py_str("growing"), py_str("plant"), py_str("peppercorns in"), py_str("grinder"), py_str("white pepper"), py_str("black pepper"), py_str("sneezing"), py_str("buy"), py_str("brand"), py_str("jar"), py_str("shop")]), g__IS_A_SAUCE()]), py_tuple(vec![py_tuple(vec![py_str("mushroom sauce"), py_str("mushroom cream sauce"), py_str("creamy mushroom sauce")]), py_str("Fry 200g of sliced mushrooms in butter over a high heat until the water they give off has gone and they squeak in the pan, then add a chopped shallot and a splash of wine or stock and scrape the brown bits up. Add 150ml of stock, reduce it by half, pour in 100ml of cream and simmer 3 minutes; finish off the heat with thyme or tarragon and a knob of butter. Do not crowd the pan or they steam grey instead of browning."), py_tuple(vec![py_str("grow"), py_str("growing"), py_str("foraging"), py_str("picking"), py_str("poisonous"), py_str("magic"), py_str("dried"), py_str("buy"), py_str("shop"), py_str("keep in the fridge"), py_str("gone slimy")]), g__IS_A_SAUCE()]), py_tuple(vec![py_tuple(vec![py_str("garlic sauce"), py_str("toum"), py_str("garlic cream"), py_str("lebanese garlic sauce"), py_str("garlic yoghurt sauce"), py_str("garlic yogurt sauce")]), g__garlic_sauce(), py_tuple(vec![py_str("buy"), py_str("jar"), py_str("shop"), py_str("brand"), py_str("grow"), py_str("planting"), py_str("bread"), py_str("chicken kiev"), py_str("calories"), py_str("healthy"), py_str("keep"), py_str("freeze")]), g__SAUCE_OR_TOUM()]), py_tuple(vec![py_tuple(vec![py_str("hollandaise"), py_str("hollandaise sauce"), py_str("bearnaise"), py_str("béarnaise")]), py_str("Whisk 2 egg yolks with a tablespoon of water and a teaspoon of lemon juice in a bowl over a pan of barely simmering water until it thickens enough to hold a ribbon, then take it off the heat and beat in 125g of melted butter a few drops at a time, with the bowl still warm but not hot. Season with salt and a little cayenne. If it gets too thick, a spoon of warm water loosens it; if it splits, a teaspoon of cold water beaten in hard usually brings it back. Keep it warm, never hot, or the yolks scramble."), py_tuple(vec![py_str("buy"), py_str("jar"), py_str("shop"), py_str("sachet"), py_str("packet"), py_str("leftover"), py_str("reheat"), py_str("freeze"), py_str("eggs benedict"), py_str("poached egg")]), g__SAUCE_OR_HOLLANDAISE()])]));
    s__MADE_IN_THE_PAN(py_tuple(vec![py_str("peppercorn sauce"), py_str("mushroom sauce")]));
    s_ENTRIES(_rows(&[]));
}
pub fn _garlic_sauce(args: &[Py]) -> Py {
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
    let _ = py_str("The oil emulsion, or the spoonful-in-yoghurt version when the line says yoghurt.");
    line = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("yoghurt"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("yogurt"))) } }) {
    return g__GARLIC_YOGHURT();
    }
    return g__GARLIC_EMULSION();
    Py::None
}
pub fn _keeps(args: &[Py]) -> Py {
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
    let mut keeping: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keeping") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: keeping"))) };
    let mut answer: Py = Py::None;
    let _ = py_str("The sauce's own keeping answer when the line asks how long it lasts, else the recipe.");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut keeping: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut reading: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reading").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut line: Py = Py::None;
    line = py_str("");
    if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    line = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&mcall(&g__A_KEEPING_WORD(), "search", &[line.clone()])) {
    return { let __b = keeping.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    }
    if py_truthy(&{ let __b = mcall(&g__AN_UNTIL(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_STEP_LENGTH(), "search", &[line.clone()]) } }) {
    return if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&reply, &[memory.clone(), reading.clone()]) } else { reply.clone() };
    }
    if py_truthy(&mcall(&g__A_COOKING_TIME(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = keeping.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__KEEPING(), "search", &[line.clone()]) } }) {
    return keeping.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&keeping.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__KEEPING(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    return if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&reply, &[memory.clone(), reading.clone()]) } else { reply.clone() };
    Py::None
}
    answer = py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), keeping.clone(), reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) });
    return answer.clone();
    Py::None
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
    let mut avoid: Py = Py::None;
    let mut entries: Py = Py::None;
    let mut gate: Py = Py::None;
    let mut names: Py = Py::None;
    let mut number: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut row: Py = Py::None;
    entries = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[g__SAUCES()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    number = __unpk2[0].clone();
    row = __unpk2[1].clone();
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&row.clone(), &Py::Int(0)), geti(&row.clone(), &Py::Int(1)), geti(&row.clone(), &Py::Int(2))]), 3, None);
    names = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    avoid = __unpk3[2].clone();
    gate = if py_truthy(&gt(&call_builtin("len", &[row.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) { geti(&row.clone(), &Py::Int(3)) } else { g__IS_A_SAUCE() };
    let _ = mcall(&entries.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(add(&py_str("sauce-"), &mcall(&geti(&names.clone(), &Py::Int(0)), "replace", &[py_str(" "), py_str("-")]))); __args.push(py_tuple(vec![names.clone(), g__ASKING()])); __args.push(_keeps(&[reply.clone(), mcall(&g__KEEPS(), "get", &[geti(&names.clone(), &Py::Int(0)), py_str("")])])); __kws.extend(vec![("ask".to_string(), gate.clone()), ("avoid".to_string(), add(&add(&avoid.clone(), &g__NOT_A_RECIPE()), &if py_truthy(&Py::Bool(py_contains(&g__MADE_IN_THE_PAN(), &geti(&names.clone(), &Py::Int(0))))) { py_tuple(vec![]) } else { py_tuple(vec![py_str("meat"), py_str("steak"), py_str("joint"), py_str("roasting tin"), py_str("gravy"), py_str("stock")]) })), ("bias".to_string(), add(&Py::Int(2), &if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(0)))) { Py::Int(1) } else { Py::Int(0) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    }
    return entries.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.sauces".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_AN_UNTIL" => Some(g__AN_UNTIL()),
            "_ASKING" => Some(g__ASKING()),
            "_A_COOKING_TIME" => Some(g__A_COOKING_TIME()),
            "_A_KEEPING_WORD" => Some(g__A_KEEPING_WORD()),
            "_A_STEP_LENGTH" => Some(g__A_STEP_LENGTH()),
            "_GARLIC_EMULSION" => Some(g__GARLIC_EMULSION()),
            "_GARLIC_YOGHURT" => Some(g__GARLIC_YOGHURT()),
            "_IS_A_BRANDY_BUTTER" => Some(g__IS_A_BRANDY_BUTTER()),
            "_IS_A_SAUCE" => Some(g__IS_A_SAUCE()),
            "_KEEPING" => Some(g__KEEPING()),
            "_KEEPS" => Some(g__KEEPS()),
            "_MADE_IN_THE_PAN" => Some(g__MADE_IN_THE_PAN()),
            "_NOT_A_RECIPE" => Some(g__NOT_A_RECIPE()),
            "_SAUCES" => Some(g__SAUCES()),
            "_SAUCE_OR_HOLLANDAISE" => Some(g__SAUCE_OR_HOLLANDAISE()),
            "_SAUCE_OR_TOUM" => Some(g__SAUCE_OR_TOUM()),
            "_garlic_sauce" => Some(py_func("_garlic_sauce", |args: &[Py]| _garlic_sauce(args))),
            "_keeps" => Some(py_func("_keeps", |args: &[Py]| _keeps(args))),
            "_rows" => Some(py_func("_rows", |args: &[Py]| _rows(args))),
            _ => None,
        }),
        set: None,
    }))
}
