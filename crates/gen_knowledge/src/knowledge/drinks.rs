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
thread_local! { static G__DIFFERENCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRINKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRINK_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GOES_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_STRENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MY_OWN_CUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STRENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__DIFFERENCES() -> Py { ensure_init(); G__DIFFERENCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DIFFERENCES(v: Py) { ensure_init(); G__DIFFERENCES.with(|c| *c.borrow_mut() = v); }
pub fn g__DRINKS() -> Py { ensure_init(); G__DRINKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRINKS(v: Py) { ensure_init(); G__DRINKS.with(|c| *c.borrow_mut() = v); }
pub fn g__DRINK_NAMES() -> Py { ensure_init(); G__DRINK_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRINK_NAMES(v: Py) { ensure_init(); G__DRINK_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__GOES_WITH() -> Py { ensure_init(); G__GOES_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GOES_WITH(v: Py) { ensure_init(); G__GOES_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g__ITSELF() -> Py { ensure_init(); G__ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITSELF(v: Py) { ensure_init(); G__ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_STRENGTH() -> Py { ensure_init(); G__ITS_STRENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_STRENGTH(v: Py) { ensure_init(); G__ITS_STRENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_TASTE() -> Py { ensure_init(); G__ITS_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_TASTE(v: Py) { ensure_init(); G__ITS_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g__MEALS() -> Py { ensure_init(); G__MEALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEALS(v: Py) { ensure_init(); G__MEALS.with(|c| *c.borrow_mut() = v); }
pub fn g__MY_OWN_CUP() -> Py { ensure_init(); G__MY_OWN_CUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MY_OWN_CUP(v: Py) { ensure_init(); G__MY_OWN_CUP.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TASTE() -> Py { ensure_init(); G__NOT_A_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TASTE(v: Py) { ensure_init(); G__NOT_A_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g__PAIRS() -> Py { ensure_init(); G__PAIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAIRS(v: Py) { ensure_init(); G__PAIRS.with(|c| *c.borrow_mut() = v); }
pub fn g__STRENGTH() -> Py { ensure_init(); G__STRENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STRENGTH(v: Py) { ensure_init(); G__STRENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__WEIGHT() -> Py { ensure_init(); G__WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEIGHT(v: Py) { ensure_init(); G__WEIGHT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Drink: which bottle goes with the food that is named.\n\nThe answers lane owns this domain. A pairing is a fact about a food, not a sentence about a\nperson, so the key is the word for wine crossed with the words that ask the pairing question,\nand the reply is chosen from the food the line names. The cooking domain already holds a\n`wine-with-food` row that answers every pairing question with one paragraph on weight (\"a light\nwhite with fish or chicken, a fuller red with red meat\"); that rule is right when the line names\nno food and useless when it names one, because \"what wine goes with pizza\" wants a bottle. This\ndomain therefore answers only the lines that name a food or a meal, and returns None otherwise,\nso the cooking row keeps the rest.\n\nNothing here is ageing or priced: the wine is named by grape or region, which does not go out of\ndate the way a bottle and a price on a shelf does.\n\nThe second half is the drink itself. \"What does prosecco taste like\", \"what is a flat white\", \"is\ngin strong\" and \"what does prosecco go with\" are four questions about one thing, so they are one\ntable keyed by the drink and four rows that read their answer off it. A drink named in a line is\nanswered about that drink however the line is worded; a line about the machine, the price, the\nshelf, the brewing or the shop keeps its own answer.\n");
    s_ANSWERS(py_tuple(vec![py_str("food-and-drink")]));
    s__PAIRS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("salmon"), py_str("trout"), py_str("prawn"), py_str("prawns"), py_str("seafood"), py_str("cod"), py_str("haddock"), py_str("sushi"), py_str("shellfish"), py_str("mussels"), py_str("scallops"), py_str("whitebait"), py_str("sea bass")]), py_str("A dry white is the safe pair: Sauvignon Blanc, Picpoul, Albarino or a dry Riesling. Salmon and trout take a light red such as Pinot Noir just as well.")]), py_tuple(vec![py_tuple(vec![py_str("curry"), py_str("curries"), py_str("tikka"), py_str("masala"), py_str("balti"), py_str("thai"), py_str("korma"), py_str("jalfrezi"), py_str("madras"), py_str("chilli con carne"), py_str("tacos"), py_str("burrito"), py_str("mexican"), py_str("salsa"), py_str("spicy"), py_str("hot wings"), py_str("indian"), py_str("biryani"), py_str("kebab"), py_str("falafel")]), py_str("An off-dry Riesling or a Gewurztraminer: a little sweetness cools the heat. Lager or a pale ale work just as well, and a big oaky red is the one to avoid.")]), py_tuple(vec![py_tuple(vec![py_str("chinese"), py_str("stir fry"), py_str("stir-fry"), py_str("noodles"), py_str("szechuan"), py_str("sichuan"), py_str("dumplings"), py_str("spring rolls"), py_str("sushi rice")]), py_str("Riesling or a dry rose, or a pale ale with the fried things: sweetness suits the soy and the spice, and a heavy red fights both.")]), py_tuple(vec![py_tuple(vec![py_str("steak"), py_str("beef"), py_str("lamb"), py_str("venison"), py_str("game"), py_str("burger"), py_str("burgers"), py_str("casserole"), py_str("stew"), py_str("roast beef"), py_str("brisket"), py_str("ribs")]), py_str("A red with some grip: Malbec, Cabernet Sauvignon, Syrah or a Rioja. Lamb likes claret, and a slow-cooked stew likes the same reds.")]), py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("pork"), py_str("veal"), py_str("roast chicken"), py_str("sausage"), py_str("sausages"), py_str("ham")]), py_str("Chicken and pork take either: a Chardonnay or white Burgundy, or a light red such as Pinot Noir or Grenache. Roast chicken with a red is the usual choice in a pub.")]), py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("pizza"), py_str("lasagne"), py_str("lasagna"), py_str("bolognese"), py_str("ragu"), py_str("tomato"), py_str("tomatoes"), py_str("meatballs"), py_str("parmigiana")]), py_str("Sangiovese or Chianti: the acidity in the wine meets the tomato, and a heavy oaky red tastes flat next to it.")]), py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("cheeses"), py_str("cheddar"), py_str("brie"), py_str("blue cheese"), py_str("stilton")]), py_str("Match the weight: a crisp white with goat's cheese and a soft rind, a red with cheddar, and port or a sweet wine with blue.")]), py_tuple(vec![py_tuple(vec![py_str("chocolate"), py_str("dessert"), py_str("pudding"), py_str("cake"), py_str("trifle"), py_str("ice cream"), py_str("fruit tart")]), py_str("A wine as sweet as the pudding: port or Banyuls with chocolate, and a sweet Riesling or Muscat with fruit puddings. A dry red turns bitter next to sugar.")]), py_tuple(vec![py_tuple(vec![py_str("salad"), py_str("vegetarian"), py_str("veggie"), py_str("quiche"), py_str("omelette"), py_str("soup"), py_str("risotto"), py_str("pasta salad")]), py_str("Sauvignon Blanc, a dry rose or a light red served cool: light food wants a light wine.")])]));
    s__MEALS(py_tuple(vec![py_str("dinner"), py_str("dinner party"), py_str("meal"), py_str("lunch"), py_str("supper"), py_str("christmas"), py_str("barbecue"), py_str("bbq"), py_str("st patricks"), py_str("takeaway"), py_str("takeout")]));
    s__WEIGHT(py_str("Match the weight of the food: a dry white with fish or a light starter, a red with red meat and anything tomatoey, and a touch of sweetness with spicy food. If a bottle is all you have, a dry rose or a medium red sits happily with most things on the table."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wine-with-the-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wine"), py_str("wines"), py_str("red or white"), py_str("white or red"), py_str("drink with"), py_str("drinking with")]), py_tuple(vec![py_str("goes with"), py_str("go with"), py_str("goes down"), py_str("pair"), py_str("pairs"), py_str("pairing"), py_str("match"), py_str("matches"), py_str("with"), py_str("for"), py_str("best"), py_str("which"), py_str("serve"), py_str("serving"), py_str("have"), py_str("have with"), py_str("should i have"), py_str("bring"), py_str("take"), py_str("bottle"), py_str("red or white"), py_str("recommend")])])); __args.push(g__wine_pair()); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("avoid".to_string(), py_tuple(vec![py_str("buy"), py_str("buys"), py_str("buying"), py_str("bought"), py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("money"), py_str("tenner"), py_str("tenners"), py_str("value"), py_str("spend"), py_str("budget"), py_str("afford"), py_str("supermarket"), py_str("shop"), py_str("shops"), py_str("shopping"), py_str("offer"), py_str("offers"), py_str("deal"), py_str("deals"), py_str("price"), py_str("gift"), py_str("present"), py_str("birthday"), py_str("wedding"), py_str("bring to"), py_str("open"), py_str("opened"), py_str("unopened"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("how long"), py_str("left over"), py_str("leftover"), py_str("vinegary"), py_str("gone off"), py_str("corkscrew"), py_str("opener"), py_str("fridge"), py_str("chill"), py_str("chilled"), py_str("cold"), py_str("temperature"), py_str("decant"), py_str("units"), py_str("unit"), py_str("limit"), py_str("safe"), py_str("too much"), py_str("a day"), py_str("every day"), py_str("week"), py_str("paracetamol"), py_str("ibuprofen"), py_str("painkiller"), py_str("painkillers"), py_str("antibiotic"), py_str("antibiotics"), py_str("penicillin"), py_str("amoxicillin"), py_str("metronidazole"), py_str("tablets"), py_str("medicine"), py_str("medication"), py_str("pills"), py_str("prescription"), py_str("antihistamine"), py_str("antidepressant"), py_str("statins"), py_str("codeine"), py_str("pregnant"), py_str("alcohol free"), py_str("alcohol-free"), py_str("non alcoholic"), py_str("sober"), py_str("hangover"), py_str("calories"), py_str("good for"), py_str("bad for"), py_str("health"), py_str("healthy"), py_str("stain"), py_str("stains"), py_str("spill"), py_str("spilled"), py_str("spilt"), py_str("sofa"), py_str("carpet"), py_str("shirt"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("cooked"), py_str("recipe"), py_str("recipes"), py_str("sauce"), py_str("stew"), py_str("casserole"), py_str("marinate"), py_str("marinade"), py_str("deglaze"), py_str("substitute"), py_str("instead of"), py_str("glaze"), py_str("gravy"), py_str("oven"), py_str("bake")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lemonade-and-iced-tea")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lemonade"), py_str("homemade lemonade"), py_str("cordial"), py_str("orange squash"), py_str("lemon squash"), py_str("blackcurrant squash"), py_str("lime cordial")]), py_tuple(vec![py_str("make"), py_str("making"), py_str("made"), py_str("how do i"), py_str("how to"), py_str("recipe"), py_str("homemade"), py_str("from scratch"), py_str("what goes in")])])); __args.push(py_str("Squeeze three lemons into 100g of sugar dissolved in 100ml of just-boiled water, top it up with a litre of cold water and taste it - a pinch of salt makes it taste more of lemon, not less. A squash is the same drink from a bottle of cordial, and a sprig of mint or a sliced lemon in the jug is what a pub does. It keeps two days in the fridge.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("buy"), py_str("buys"), py_str("shop"), py_str("supermarket"), py_str("price"), py_str("cost"), py_str("offer"), py_str("brand"), py_str("calories"), py_str("how much sugar"), py_str("healthy"), py_str("good for"), py_str("bad for"), py_str("stain"), py_str("stains"), py_str("spill"), py_str("carpet"), py_str("fizzy"), py_str("coke"), py_str("cola"), py_str("wine"), py_str("beer"), py_str("gin"), py_str("vodka"), py_str("cocktail"), py_str("pint"), py_str("alcohol")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mulled-wine")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mulled wine"), py_str("gluhwein"), py_str("mulled cider"), py_str("wassail"), py_str("mulled")]), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("homemade"), py_str("how do i"), py_str("how to"), py_str("what goes in")])])); __args.push(py_str("Warm a bottle of red with two tablespoons of sugar, a sliced orange, a cinnamon stick, two star anise and four cloves: keep it at a bare shiver for 15 minutes and never let it boil, which takes the alcohol and the fruit out of it. A splash of brandy at the end makes it stronger, and mulled cider is the same drink made with cider and no sugar.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("buy"), py_str("shop"), py_str("price"), py_str("cost"), py_str("gift"), py_str("present"), py_str("units"), py_str("unit"), py_str("limit"), py_str("pregnant"), py_str("alcohol free"), py_str("alcohol-free"), py_str("non alcoholic"), py_str("sober"), py_str("hangover"), py_str("how long does it keep")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hot-chocolate")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hot chocolate"), py_str("hot cocoa"), py_str("drinking chocolate"), py_str("cocoa")]), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("how do i"), py_str("how to"), py_str("proper"), py_str("best"), py_str("what goes in")])])); __args.push(py_str("Warm 200ml of milk and melt 20g of dark chocolate into it, chopped small, stirring until it is smooth - a pinch of salt makes it taste more of chocolate, not less. Cocoa powder works too: a teaspoon each of cocoa and sugar made into a paste with a little cold milk first, so it does not go lumpy. Keep it under a simmer, because milk that boils tastes cooked.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("calories"), py_str("calorie"), py_str("diet"), py_str("lose weight"), py_str("caffeine"), py_str("sleep"), py_str("before bed"), py_str("stain"), py_str("stains"), py_str("spill"), py_str("buy"), py_str("brand"), py_str("mug"), py_str("healthy"), py_str("good for"), py_str("without milk"), py_str("no milk"), py_str("dairy free"), py_str("dairy-free"), py_str("oat milk"), py_str("plant milk"), py_str("almond milk"), py_str("soy milk"), py_str("vegan"), py_str("lactose")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hard-water")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hard water"), py_str("water is hard"), py_str("water hard"), py_str("water is so hard"), py_str("water so hard"), py_str("water hardness"), py_str("hardness"), py_str("water is very hard"), py_str("hard and soft water"), py_str("soft water"), py_str("chalky water"), py_str("minerals in the water"), py_str("minerals in my water")]), py_tuple(vec![py_str("bad for you"), py_str("bad for"), py_str("safe"), py_str("safely"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("drink"), py_str("drinking"), py_str("drank"), py_str("harmful"), py_str("dangerous"), py_str("healthy"), py_str("health"), py_str("kidney"), py_str("kidneys"), py_str("stones"), py_str("skin"), py_str("eczema"), py_str("dry"), py_str("why"), py_str("cause"), py_str("causes"), py_str("softener"), py_str("soften"), py_str("filter"), py_str("bottled"), py_str("baby"), py_str("formula"), py_str("milk"), py_str("boil"), py_str("calcium"), py_str("safe to drink")])])); __args.push(py_str("Not bad for you: the calcium and magnesium that make water hard are minerals you get from food anyway, mains water is tested to the same standard whatever its hardness, and the studies find no link between hard water and kidney stones - some find slightly fewer stones in hard-water areas. It is hard because it has come through chalk or limestone, which is why the same country has soft water in the west and hard water in the south east. What it does do is leave a chalky film on taps, kettles and shower glass, lather soap and shampoo less so you use more of them, and feel drier on the skin - a filter jug or a scale reducer is enough for most homes. The one to know about is a baby's bottle: a water softener swaps the calcium for sodium, so make formula with unsoftened mains water, boiled once and cooled.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("kettle"), py_str("kettles"), py_str("descale"), py_str("descaling"), py_str("descales"), py_str("scale"), py_str("scaled"), py_str("limescale"), py_str("shower"), py_str("showerhead"), py_str("shower head"), py_str("taps"), py_str("marks"), py_str("stains"), py_str("iron"), py_str("steam iron"), py_str("washing machine"), py_str("dishwasher"), py_str("bill"), py_str("bills"), py_str("meter"), py_str("supplier"), py_str("account"), py_str("contract"), py_str("cancel"), py_str("standing charge"), py_str("swimming"), py_str("pool"), py_str("fish"), py_str("aquarium"), py_str("pond"), py_str("plant"), py_str("plants"), py_str("garden")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("milk-in-a-coffee")); __args.push(py_tuple(vec![py_tuple(vec![py_str("latte"), py_str("cappuccino"), py_str("flat white"), py_str("mocha"), py_str("macchiato"), py_str("cortado"), py_str("piccolo"), py_str("latte macchiato"), py_str("late")]), py_tuple(vec![py_str("ml"), py_str("millilitres"), py_str("milliliters"), py_str("how much milk"), py_str("milk goes in"), py_str("milk in it"), py_str("milk in a"), py_str("milk in the"), py_str("milk in my"), py_str("ml of milk"), py_str("milk to"), py_str("ratio"), py_str("how much of it is milk"), py_str("mostly milk"), py_str("milk does it have"), py_str("ounces of milk"), py_str("coffee goes in"), py_str("coffee in it"), py_str("coffee to milk"), py_str("how much of it")])])); __args.push(py_str("Most of the cup is milk: a latte is about five parts steamed milk to one of espresso, so roughly 200ml of milk and a 60ml double shot in a medium, and 300 to 350ml of milk in a large. A flat white is the same coffee with about half of that, 100 to 120ml, and a cappuccino sits between the two at about 150ml - a third of it foam, so ask for it wet if you want more milk and dry if you want more air. A macchiato or a cortado is a splash, 20 to 40ml, and a mocha is a latte with chocolate in it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("calorie"), py_str("calories"), py_str("kcal"), py_str("sugar"), py_str("caffeine"), py_str("decaf"), py_str("decaffeinated"), py_str("diet"), py_str("lose weight"), py_str("healthy"), py_str("good for"), py_str("bad for"), py_str("how many calories"), py_str("how many kcal"), py_str("how many spoons"), py_str("how many sugars"), py_str("price"), py_str("cost"), py_str("buy"), py_str("cheap"), py_str("stain"), py_str("stains"), py_str("spill"), py_str("spilt"), py_str("machine"), py_str("froth"), py_str("frothing"), py_str("foam"), py_str("latte art"), py_str("recipe"), py_str("make a"), py_str("make one"), py_str("make my own"), py_str("how do i make"), py_str("how to make"), py_str("barista"), py_str("instead of"), py_str("instead"), py_str("in place of"), py_str("substitute"), py_str("substitutes"), py_str("swap"), py_str("replace"), py_str("rather than"), py_str("out of milk"), py_str("ran out")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("froth-milk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("froth"), py_str("frothed"), py_str("frothing"), py_str("frother"), py_str("frothy milk"), py_str("milk foam"), py_str("steamed milk"), py_str("microfoam"), py_str("foam on my coffee")]), py_tuple(vec![py_str("milk"), py_str("coffee"), py_str("how do i"), py_str("how to"), py_str("without"), py_str("no machine"), py_str("jar"), py_str("cafetiere"), py_str("french press"), py_str("latte"), py_str("cappuccino")])])); __args.push(py_str("Cold milk in a jar with a lid, shaken hard for 30 seconds and then 30 seconds in the microwave, gives a real foam with no machine, and a French press pumped up and down 15 times does the same. Whole milk froths best because the fat holds the bubbles, and stop heating at about 60 to 65C - past that the foam collapses and the milk tastes scalded.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("coffee machine"), py_str("my machine"), py_str("machine is"), py_str("machine not"), py_str("machines"), py_str("buy"), py_str("descale"), py_str("limescale"), py_str("clean"), py_str("cleaning"), py_str("broken"), py_str("not working"), py_str("coffee shop"), py_str("latte art"), py_str("stain"), py_str("bean"), py_str("beans"), py_str("grinder")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tea-types")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tea"), py_str("teas"), py_str("green tea"), py_str("black tea"), py_str("white tea"), py_str("oolong"), py_str("pu erh"), py_str("pu-erh"), py_str("herbal tea"), py_str("herbal teas"), py_str("rooibos"), py_str("camomile"), py_str("chamomile"), py_str("earl grey"), py_str("english breakfast"), py_str("assam"), py_str("darjeeling"), py_str("jasmine tea"), py_str("peppermint tea"), py_str("fruit tea"), py_str("fruit teas"), py_str("infusion"), py_str("infusions")]), py_tuple(vec![py_str("kinds"), py_str("kind"), py_str("types"), py_str("type"), py_str("sorts"), py_str("are there"), py_str("difference"), py_str("differences"), py_str("different"), py_str("which one"), py_str("which is"), py_str("which should"), py_str("which do"), py_str("what are"), py_str("what's the difference"), py_str("whats the difference"), py_str("choose"), py_str("pick"), py_str("best"), py_str("caffeine free"), py_str("no caffeine"), py_str("without caffeine"), py_str("should i"), py_str("should we"), py_str("what should i"), py_str("recommend"), py_str("recommendation"), py_str("what tea")])])); __args.push(py_str("Nearly all of it is the same leaf at different levels of oxidation: green tea is dried before it can darken and tastes grassy, black tea is fully oxidised and malty, and oolong sits between them, roasted and floral. White tea is the young leaf barely processed, light and sweet, and pu-erh is aged and earthy, an acquired taste. Peppermint, chamomile and rooibos are infusions of other plants rather than tea, so they have no caffeine and suit the evening. Everyday picks: English breakfast or Assam if you take milk, Earl Grey for the citrus, and a plain green or jasmine drunk without milk.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("brew"), py_str("brewing"), py_str("brewed"), py_str("steep"), py_str("steeped"), py_str("how long"), py_str("bitter"), py_str("stewed"), py_str("cloudy"), py_str("teabag"), py_str("teabags"), py_str("tea bag"), py_str("tea bags"), py_str("kettle"), py_str("cuppa"), py_str("how much"), py_str("caffeine"), py_str("pregnant"), py_str("pregnancy"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("child"), py_str("sleep"), py_str("bedtime"), py_str("before bed"), py_str("stain"), py_str("stains"), py_str("spill"), py_str("making"), py_str("make"), py_str("recipe"), py_str("mug"), py_str("cup of"), py_str("buy"), py_str("price"), py_str("brand"), py_str("supers"), py_str("bubble tea"), py_str("boba"), py_str("chai"), py_str("matcha"), py_str("coffee"), py_str("espresso"), py_str("cake"), py_str("biscuit"), py_str("biscuits"), py_str("scone"), py_str("sugar"), py_str("every day"), py_str("a day"), py_str("a week"), py_str("how many cups"), py_str("bad for"), py_str("good for"), py_str("healthy"), py_str("health")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    s__DRINKS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("flat white"), py_str("flat whites")]), py_str("A flat white is a double shot of espresso under a thin layer of steamed milk, about 160ml in the cup."), py_str("Strong and milky at once: the coffee is at the front and the milk softens it without hiding it, and the foam is a thin glossy layer rather than a cap you eat with a spoon."), py_str("It is a morning coffee on its own. With food, a plain croissant or a slice of toast beats anything sweet, because the milk and the sugar fight.")]), py_tuple(vec![py_tuple(vec![py_str("latte"), py_str("lattes"), py_str("caffe latte")]), py_str("A latte is one or two shots of espresso topped up with a lot of steamed milk, in a tall glass or a big cup."), py_str("Milky and mild: the milk is most of the drink and the coffee is a soft bitterness behind it, with a thin foam on top. Ask for it with an extra shot if you want to taste the coffee."), py_str("It is milky enough to drink with breakfast or a sweet thing, and it goes flat with a sharp lemon cake or anything acidic.")]), py_tuple(vec![py_tuple(vec![py_str("cappuccino"), py_str("cappuccinos"), py_str("capuccino")]), py_str("A cappuccino is an espresso with equal parts steamed milk and thick foam, usually in a smaller cup than a latte."), py_str("It is a third coffee, a third milk and a third foam, so the first sip is dry foam and the coffee is stronger and shorter than in a latte. In Italy it is a morning drink, not one for after a meal."), py_str("Best with something plain at breakfast - a pastry or a biscuit - and it is the wrong coffee to sit beside a meal.")]), py_tuple(vec![py_tuple(vec![py_str("americano"), py_str("americanos"), py_str("caffe americano"), py_str("long black")]), py_str("An americano is one or two shots of espresso with hot water added, so it is black coffee made in a cup."), py_str("Black and clean, with the coffee's bitterness and no milk to soften it: longer and less intense than an espresso, because the water thins it. A long black is the same thing with the water in the cup first, which keeps more of the crema."), py_str("It is the coffee for a meal or for a long sit, and the one to order if milk upsets you.")]), py_tuple(vec![py_tuple(vec![py_str("espresso"), py_str("espressos"), py_str("expresso"), py_str("double espresso"), py_str("short black")]), py_str("An espresso is a small, strong shot of coffee forced through finely ground beans by a machine, about 25 to 30ml."), py_str("Concentrated and bittersweet with a thick crema on top: coffee, caramel and a dry finish, drunk in two or three sips. It has less caffeine than a mug of filter coffee, not more."), py_str("It is the finish to a meal rather than a drink with one, and a small sweet thing beside it is the Italian way.")]), py_tuple(vec![py_tuple(vec![py_str("macchiato"), py_str("macchiatos"), py_str("latte macchiato")]), py_str("A macchiato is an espresso with a spoonful of milk foam on top. A latte macchiato is the other way round: a glass of hot milk with a shot poured into it."), py_str("An espresso macchiato tastes of the coffee with a soft, slightly sweet top; a latte macchiato is milky with a coffee layer sitting above it, so the first sip is coffee and the rest is warm milk."), py_str("A macchiato is a two-minute coffee on its own; the latte macchiato is a breakfast drink.")]), py_tuple(vec![py_tuple(vec![py_str("mocha"), py_str("mochas"), py_str("caffe mocha"), py_str("mocha latte")]), py_str("A mocha is a latte with chocolate syrup or powder stirred into the espresso before the milk goes in."), py_str("Sweet and chocolatey, coffee second: it is closer to a hot chocolate with a kick than to a coffee, and the chocolate hides a weak shot. Ask for half the syrup if you want to taste the coffee at all."), py_str("It is a pudding as much as a drink, so it wants no cake beside it.")]), py_tuple(vec![py_tuple(vec![py_str("filter coffee"), py_str("filter"), py_str("cafetiere"), py_str("french press"), py_str("drip coffee"), py_str("black coffee")]), py_str("Filter coffee is ground coffee steeped in hot water and then filtered out, through paper in a dripper or a metal mesh in a cafetiere."), py_str("Cleaner and lighter than an espresso, with the fruit and the acidity of the bean still in it: it tastes of tea and fruit as much as of coffee, and it goes bitter if it is left to stew on a hot plate."), py_str("It is the drink for a second cup, and it takes milk and food better than an espresso does.")]), py_tuple(vec![py_tuple(vec![py_str("cold brew"), py_str("cold brew coffee")]), py_str("Cold brew is ground coffee steeped in cold water for 12 to 18 hours and then strained, and served over ice."), py_str("Round and low in acid, sweeter than hot-brewed coffee with almost none of the sharpness, and it is stronger in caffeine than the same volume of filter coffee because the ratio is concentrated."), py_str("It is a summer drink: neat over ice, or cut with milk like a latte.")]), py_tuple(vec![py_tuple(vec![py_str("green tea")]), py_str("Green tea is unfermented tea leaf, brewed at about 80C rather than boiling."), py_str("Grassy and light, with a sweetness at the back and a slight bitterness or astringency if it is left to steep - the leaves keep giving, so a second brew is softer than the first."), py_str("It goes with rice, fish and anything light, and it is the drink to have with sushi.")]), py_tuple(vec![py_tuple(vec![py_str("earl grey"), py_str("earl gray"), py_str("earl grey tea")]), py_str("Earl grey is black tea scented with bergamot, the citrus oil from a bitter orange."), py_str("Black tea first, then the bergamot on top: floral and perfumed, and it turns soapy if the milk is heavy or the bag is left in too long."), py_str("It wants a slice of lemon rather than milk, and a plain biscuit or a scone.")]), py_tuple(vec![py_tuple(vec![py_str("matcha"), py_str("matcha tea"), py_str("matcha latte")]), py_str("Matcha is green tea leaf ground to a powder and whisked into hot water, so the whole leaf is drunk rather than steeped and thrown away."), py_str("Vegetal and savoury, like spinach and fresh grass, with a bitterness that stays in the mouth. In a latte the milk turns it sweet and soft, and the colour is the point as much as the taste."), py_str("It is good with anything sweet and milky, and it fights with citrus.")]), py_tuple(vec![py_tuple(vec![py_str("chai"), py_str("chai tea"), py_str("chai latte"), py_str("masala chai")]), py_str("Chai is black tea boiled with milk, sugar and whole spices - cardamom, ginger, cinnamon, cloves - so it is a spiced milky tea rather than tea with a flavour."), py_str("Warm and sweet, with the spices arriving after the tea: ginger heat, cardamom and clove behind the milk. A chai latte in a coffee shop is the same idea with more sugar and foam."), py_str("It is a winter drink and it takes a biscuit or something fried and savoury.")]), py_tuple(vec![py_tuple(vec![py_str("prosecco")]), py_str("Prosecco is a sparkling white wine from northern Italy, made in a tank rather than in the bottle, which keeps it cheap and fruity."), py_str("Light and fizzy with green apple, pear and a little white flower: it is off-dry rather than sweet, and the bubbles make it feel fresher and less alcoholic than it is. Drink it young - it does not improve in the bottle."), py_str("It goes with salty and fried things - crisps, olives, fried fish - and with fruit desserts; it is the wrong glass for a steak.")]), py_tuple(vec![py_tuple(vec![py_str("champagne"), py_str("sparkling wine"), py_str("cava"), py_str("cremant"), py_str("cremant de loire")]), py_str("Champagne is sparkling wine from the Champagne region of France, fermented a second time in the bottle, which is what the price pays for. Cava and cremant are made the same way elsewhere."), py_str("Tighter and more yeasty than prosecco: brioche and toast from the time on the lees, with a sharper acidity and a finer bubble, and the best of it is dry rather than sweet."), py_str("It goes with oysters, fried food and salty nuts, and with a roast chicken as well as with a celebration.")]), py_tuple(vec![py_tuple(vec![py_str("red wine")]), py_str("Red wine is wine fermented with the skins of the grapes, which is where the colour and the tannin come from."), py_str("From light and juicy to heavy and drying depending on the grape: the tannin is the grip that dries your mouth, and it is what makes a young red taste harsh with nothing to eat."), py_str("It goes with red meat, stews and hard cheese, and badly with anything vinegary or very spicy.")]), py_tuple(vec![py_tuple(vec![py_str("white wine")]), py_str("White wine is wine fermented without the skins, so it is pale and lower in tannin."), py_str("From bone dry and citrusy to sweet and honeyed: the cool-climate ones taste of lemon and green apple and the warm ones of peach and melon, and oak adds vanilla and butter."), py_str("It goes with fish, chicken, salads and soft cheese, and a dry one is the safer glass with food than a red.")]), py_tuple(vec![py_tuple(vec![py_str("rose"), py_str("rosé"), py_str("rose wine"), py_str("pink wine")]), py_str("Rosé is wine that spent a few hours with the red grape skins, enough for the colour and not for the tannin."), py_str("Fresh and dry, usually with strawberry, watermelon and a little citrus, and served cold. The pale Provencal kind is the driest; the darker, sweeter ones are a different drink."), py_str("It goes with salads, grilled vegetables, chicken and anything you would eat outdoors.")]), py_tuple(vec![py_tuple(vec![py_str("lager"), py_str("lagers")]), py_str("Lager is beer brewed with a bottom-fermenting yeast at a cold temperature and then stored, which is where the clean taste comes from."), py_str("Crisp and cold, lightly bitter and dry at the end, with very little fruit: the point is how clean it is, which is why it goes with almost everything."), py_str("It goes with a curry, fish and chips, pizza and a hot day. The colder it is the less you taste it, so a good one is served cool rather than iced.")]), py_tuple(vec![py_tuple(vec![py_str("ipa"), py_str("ipas"), py_str("india pale ale"), py_str("pale ale")]), py_str("An IPA is a heavily hopped pale ale; the hops were originally there to keep the beer on the voyage to India."), py_str("Bitter and citrusy or piney, with the hop flavour at the front and a long bitter finish. A hazy New England IPA is softer and fruitier, and a pale ale is the same idea turned down."), py_str("It goes with a burger, fried chicken or a strong cheese, and it flattens a delicate fish.")]), py_tuple(vec![py_tuple(vec![py_str("stout"), py_str("stouts"), py_str("guinness"), py_str("porter")]), py_str("Stout is a dark beer brewed with roasted barley, which is where the coffee and chocolate come from."), py_str("Roasted and smooth: coffee, dark chocolate and a dry, bitter finish, and the creamy sort is served on nitrogen so the head is thick and the bubbles are tiny."), py_str("It goes with oysters, stews and chocolate, and it is a meal in itself compared with a lager.")]), py_tuple(vec![py_tuple(vec![py_str("cider"), py_str("ciders")]), py_str("Cider is fermented apple juice. In the UK it is alcoholic and still or sparkling; in the US the word usually means the cloudy juice."), py_str("From bone dry and tannic to sweet and appley: the dry ones taste of the fruit and the skin and the sweet ones of juice, and the strength varies more than with beer."), py_str("It goes with pork, cheese and anything with apple in it, and it is the drink for a pub garden.")]), py_tuple(vec![py_tuple(vec![py_str("gin"), py_str("gin and tonic")]), py_str("Gin is a clear spirit distilled with juniper and then other botanicals, and a gin and tonic is that with tonic water and ice."), py_str("Juniper first, then whatever the maker put in - citrus peel, coriander, angelica, sometimes cucumber or violet. The tonic adds bitterness and sugar, so a G&T tastes of the tonic as much as the gin."), py_str("It goes with olives, salted nuts and fried fish; it is a before-dinner drink.")]), py_tuple(vec![py_tuple(vec![py_str("whisky"), py_str("whiskey"), py_str("scotch"), py_str("bourbon"), py_str("single malt")]), py_str("Whisky is a spirit distilled from grain and aged in wood, which is where the colour and most of the flavour come from. Scotch and bourbon are the two most people meet."), py_str("From light and honeyed to smoky and medicinal: bourbon is sweeter and tastes of vanilla and caramel from the new oak, and a peated Scotch of smoke and tar. A drop of water opens it up rather than diluting it."), py_str("It goes with dark chocolate, blue cheese and a fire, and it is not a drink to have with a meal.")]), py_tuple(vec![py_tuple(vec![py_str("vodka")]), py_str("Vodka is a spirit distilled to as close to pure alcohol and water as the law allows, then cut to strength."), py_str("Almost nothing, which is the point: clean and slightly sweet at the front and hot at the back, with the flavour coming from whatever it is mixed with. A cheap one tastes of the distillery and a good one of nothing."), py_str("It goes with tonic, tomato juice or a martini glass, and with salty things.")]), py_tuple(vec![py_tuple(vec![py_str("rum")]), py_str("Rum is distilled from sugar cane or its molasses. The clear kind is unaged; the dark kind is aged in wood."), py_str("Sweet and round, with vanilla, caramel and sometimes a burnt sugar edge; a white rum is lighter and drier, and an aged one is closer to a whisky in weight."), py_str("It goes with lime and ginger beer, and with anything you would put brown sugar on.")]), py_tuple(vec![py_tuple(vec![py_str("beer"), py_str("beers"), py_str("a pint of beer"), py_str("real ale")]), py_str("Beer is brewed from grain, usually barley, with hops for bitterness and yeast for the alcohol."), py_str("From light and crisp to dark and roasted, depending on the malt: the hops give the bitterness and the fruit, the malt gives the sweetness and the colour. Most beer is between 4 and 5.5 per cent, and served colder than wine."), py_str("It goes with fried food, pizza, curry and a hot afternoon, and a hoppy one with a burger.")]), py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("coffees"), py_str("a coffee")]), py_str("Coffee is the roasted and ground seed of a fruit, brewed with hot water."), py_str("Bitter and acidic at once, with the roast on top: darker roasts taste of chocolate and toast, lighter ones of fruit and citrus, and the bitterness is the part that survives milk and sugar."), py_str("It goes with anything sweet in the morning and with nothing much after a meal, which is why a small espresso is the usual finish.")]), py_tuple(vec![py_tuple(vec![py_str("tea"), py_str("teas"), py_str("a cup of tea"), py_str("builder's tea")]), py_str("Tea is the leaf of one plant, Camellia sinensis, brewed in hot water; how it is processed decides whether it is black, green or white."), py_str("Tannin first: dry and slightly astringent, with the flavour of the leaf behind it, and it goes bitter if it is left in the water too long. Milk softens the tannin, which is why a strong tea takes milk and a delicate one does not."), py_str("It goes with a biscuit, a slice of cake or a fried breakfast, and it is the drink for bad news.")]), py_tuple(vec![py_tuple(vec![py_str("oat milk"), py_str("oat drink"), py_str("oatly"), py_str("oat milk latte")]), py_str("Oat milk is oats soaked, blended and strained, with an enzyme step that turns some of the starch to sugar, which is why it is the plant milk that foams."), py_str("Oaty and faintly sweet like the milk left at the bottom of a bowl of porridge, with a creaminess that comes from the oats rather than fat. In coffee the ordinary sort splits and tastes of cardboard; a barista version has oil added and holds."), py_str("It goes with coffee, cereal and anything oaty or banana-flavoured, and it curdles in a strong acid the way no dairy milk does.")]), py_tuple(vec![py_tuple(vec![py_str("baileys"), py_str("irish cream"), py_str("cream liqueur")]), py_str("Irish cream is whiskey, cream and sugar, homogenised so the cream does not separate - Baileys is the one everybody means."), py_str("Sweet and thick, of whiskey and cream with a chocolate edge: it is a pudding in a glass rather than a drink, and the alcohol is under the sugar."), py_str("It goes over ice, in coffee and over vanilla ice cream. Do not put it in a citrus drink - the acid splits the cream - and keep the bottle in the fridge once it is open.")]), py_tuple(vec![py_tuple(vec![py_str("negroni"), py_str("negronis")]), py_str("A negroni is equal parts gin, Campari and sweet vermouth, stirred over ice and poured over a big cube with a strip of orange peel."), py_str("Bitter first, then sweet and herby: the Campari is the grapefruit-and-rhubarb bitterness, the vermouth is the sweetness behind it, and the gin is the spine rather than the flavour."), py_str("It is an aperitivo before dinner rather than a drink with food, and it is strong: about 24 per cent once mixed, so one is the usual.")]), py_tuple(vec![py_tuple(vec![py_str("aperol spritz"), py_str("spritz"), py_str("spritzes"), py_str("aperol")]), py_str("A spritz is three parts prosecco, two parts Aperol and a splash of soda, over ice in a wine glass with a slice of orange."), py_str("Light, fizzy and bittersweet - orange and rhubarb from the Aperol, the wine behind it - and much weaker than it looks, because most of the glass is ice and prosecco."), py_str("It is a drink for a hot afternoon or before dinner. It goes with salty things - olives, crisps, a bowl of nuts - and nothing sweet.")]), py_tuple(vec![py_tuple(vec![py_str("kombucha")]), py_str("Kombucha is sweet tea fermented with a culture of yeast and bacteria, which turns the sugar into acid and a little alcohol."), py_str("Sour and slightly fizzy, like a vinegar you can drink, with the fruit or herb it was brewed with on top: it is the sourness, not the sweetness, that most people notice first."), py_str("It goes with sushi and anything rich, and it is a good stand-in for a beer at lunch.")])]));
    s__STRENGTH(py_dict(vec![(py_str("vodka"), py_str("About 37.5 to 40 per cent alcohol. A 25ml measure is roughly one unit.")), (py_str("gin"), py_str("About 37.5 to 40 per cent alcohol, so a 25ml measure is roughly one unit; a double with tonic is about two, which is more than a pint of ordinary lager.")), (py_str("rum"), py_str("About 37.5 to 40 per cent alcohol, so a 25ml measure is roughly one unit.")), (py_str("whisky"), py_str("About 40 per cent alcohol, so a 25ml measure is roughly one unit; cask strength bottlings go up to 60 per cent and need water.")), (py_str("lager"), py_str("About 4 to 4.5 per cent, so a pint is roughly two units. A premium lager or a strong continental one runs to 5 per cent and more.")), (py_str("ipa"), py_str("Usually 5 to 6.5 per cent, so a pint is about three units; a double IPA can be 8 per cent or more.")), (py_str("ale"), py_str("About 4 to 5 per cent, so a pint is roughly two to two and a half units.")), (py_str("stout"), py_str("About 4.2 per cent for the usual draught stout, so a pint is about two and a half units; an imperial stout is a different drink at 9 per cent and up.")), (py_str("beer"), py_str("Ordinary draught beer is about 4 per cent and a strong one 5 to 6, so a pint is roughly two to three units.")), (py_str("cider"), py_str("About 4.5 per cent for most, but cider runs from 1.5 per cent to 8 and more, so check the bottle: a pint of 7.5 per cent cider is nearly four units.")), (py_str("prosecco"), py_str("About 11 per cent, so a 125ml glass is roughly 1.4 units - a little less than the same glass of wine.")), (py_str("champagne"), py_str("About 12 per cent, so a 125ml glass is roughly 1.5 units.")), (py_str("white wine"), py_str("About 11 to 13 per cent, so a 175ml glass is roughly two units and a large 250ml glass about three.")), (py_str("rose"), py_str("About 11 to 12.5 per cent, so a 175ml glass is roughly two units.")), (py_str("red wine"), py_str("About 13 to 14 per cent, so a 175ml glass is roughly 2.3 units - a large glass is over three. White is usually a little weaker at 11 to 13 per cent, so a red is stronger, but not by much.")), (py_str("baileys"), py_str("About 17 per cent, so a 50ml measure is roughly 0.9 units - and it comes in measures, not a tumbler, for that reason.")), (py_str("negroni"), py_str("About 24 per cent once mixed, since it is a third each of gin, Campari and vermouth, so a negroni is about 2.2 units.")), (py_str("spritz"), py_str("About 8 to 11 per cent, because the prosecco is stretched with soda and ice, so a large glass is roughly 1.5 units - weaker than a glass of wine.")), (py_str("oat milk"), Py::None), (py_str("coffee"), Py::None), (py_str("tea"), Py::None), (py_str("green tea"), Py::None), (py_str("earl grey"), Py::None), (py_str("matcha"), Py::None), (py_str("chai"), Py::None), (py_str("kombucha"), Py::None), (py_str("flat white"), Py::None), (py_str("latte"), Py::None), (py_str("cappuccino"), Py::None), (py_str("americano"), Py::None), (py_str("espresso"), Py::None), (py_str("macchiato"), Py::None), (py_str("mocha"), Py::None), (py_str("filter coffee"), Py::None), (py_str("cold brew"), Py::None)]));
    s__ITS_STRENGTH(py_tuple(vec![py_str("strong"), py_str("stronger"), py_str("strongest"), py_str("weaker"), py_str("strength"), py_str("how strong"), py_str("alcohol"), py_str("alcohol content"), py_str("more alcohol"), py_str("less alcohol"), py_str("abv"), py_str("units"), py_str("per cent"), py_str("percent"), py_str("proof"), py_str("get drunk"), py_str("drunk")]));
    s__ITS_TASTE(py_tuple(vec![py_str("taste like"), py_str("tastes like"), py_str("taste of"), py_str("tastes of"), py_str("taste"), py_str("tastes"), py_str("how does it taste"), py_str("what is it like"), py_str("like to drink"), py_str("whats it like"), py_str("what's it like"), py_str("sweet"), py_str("bitter"), py_str("fizzy"), py_str("flavour"), py_str("flavor"), py_str("taste good"), py_str("tastes good")]));
    s__ITSELF(py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("tell me about"), py_str("explain"), py_str("define"), py_str("definition"), py_str("difference"), py_str("compare"), py_str("how is it made"), py_str("made of")]));
    s__GOES_WITH(py_tuple(vec![py_str("go with"), py_str("goes with"), py_str("goes in"), py_str("pair with"), py_str("pairs with"), py_str("eat with"), py_str("have with"), py_str("go best"), py_str("go well"), py_str("match"), py_str("drink with"), py_str("food with")]));
    s__NOT_A_TASTE(py_tuple(vec![py_str("machine"), py_str("machines"), py_str("descale"), py_str("descaling"), py_str("grinder"), py_str("grind"), py_str("beans"), py_str("bean"), py_str("ground"), py_str("recipe"), py_str("how do i make"), py_str("how to make"), py_str("how do you make"), py_str("at home"), py_str("barista"), py_str("cup"), py_str("cups"), py_str("mug"), py_str("shop"), py_str("buy"), py_str("sell"), py_str("price"), py_str("prices"), py_str("cost"), py_str("how much"), py_str("cheap"), py_str("expensive"), py_str("priced"), py_str("stock"), py_str("shelf"), py_str("supermarket"), py_str("calories"), py_str("caffeine"), py_str("how many units"), py_str("alcohol"), py_str("safe"), py_str("pregnant"), py_str("breastfeeding"), py_str("bed"), py_str("sleep"), py_str("before a run"), py_str("water"), py_str("hydration"), py_str("stain"), py_str("stains"), py_str("sugar"), py_str("sugars"), py_str("addict"), py_str("allergy"), py_str("allergic"), py_str("reaction"), py_str("heartburn"), py_str("acid reflux"), py_str("thin"), py_str("thinning"), py_str("wine stain"), py_str("corked"), py_str("hangover"), py_str("bottle"), py_str("vintage"), py_str("cellar"), py_str("keep"), py_str("storing"), py_str("store"), py_str("freeze"), py_str("open a bottle"), py_str("opens"), py_str("shelf life"), py_str("use by"), py_str("date"), py_str("tastes wrong"), py_str("taste wrong"), py_str("tastes off"), py_str("tastes funny"), py_str("tastes bad"), py_str("taste bad"), py_str("tastes stale"), py_str("tastes burnt"), py_str("tastes sour"), py_str("tastes flat"), py_str("gone off"), py_str("off milk"), py_str("for a beginner"), py_str("for beginners"), py_str("good one"), py_str("good beer"), py_str("good wine"), py_str("best beer"), py_str("best wine"), py_str("recommend"), py_str("suggest"), py_str("which one")]));
    s__DRINK_NAMES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DRINKS()) { let mut row: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(0))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__MY_OWN_CUP(py_tuple(vec![py_str("my coffee"), py_str("my tea"), py_str("my drink"), py_str("my latte"), py_str("my beer"), py_str("my wine"), py_str("this coffee"), py_str("this tea"), py_str("this beer"), py_str("this wine"), py_str("the coffee i"), py_str("the tea i"), py_str("instant")]));
    s__DIFFERENCES(py_dict(vec![(py_tuple(vec![py_str("cappuccino"), py_str("latte")]), py_str("A latte is the milkier of the two: a lot of steamed milk with a thin foam on top, in a bigger cup. A cappuccino is a third coffee, a third milk and a third foam, in a smaller cup, so it is drier to drink and tastes more of the coffee.")), (py_tuple(vec![py_str("flat white"), py_str("latte")]), py_str("A flat white is smaller and stronger: a double shot under a thin, glossy layer of milk. A latte is mostly milk. Order the flat white for the coffee and the latte for something mild and long.")), (py_tuple(vec![py_str("cappuccino"), py_str("flat white")]), py_str("Both are small and strong. The flat white has thinner, glossier milk and no foam cap; the cappuccino is a third foam, so the first sip is dry and there is less milk under it.")), (py_tuple(vec![py_str("americano"), py_str("espresso")]), py_str("An americano is an espresso with hot water added: the same coffee, a longer drink, and much less intense. An espresso is the 25 to 30ml shot on its own, with a thick crema.")), (py_tuple(vec![py_str("champagne"), py_str("prosecco")]), py_str("Champagne is fermented a second time in the bottle, so it tastes of brioche and toast with a sharper acidity; prosecco is made in a tank, so it is fruitier, lighter and cheaper. Both are dry, and prosecco is the easier one to drink.")), (py_tuple(vec![py_str("ale"), py_str("lager")]), py_str("Lager is fermented cold with a bottom yeast, so it is clean, crisp and lightly bitter. An ale is fermented warm, so it tastes fruitier and more bitter from the hops.")), (py_tuple(vec![py_str("ipa"), py_str("lager")]), py_str("An IPA is far hoppier - bitter, citrusy or piney, with a long finish - and usually stronger. A lager is clean and light, and the point of it is that it tastes of very little.")), (py_tuple(vec![py_str("cider"), py_str("lager")]), py_str("Cider is fermented apple juice, so it tastes of fruit and is often sweeter; beer is brewed from grain, so it tastes of hops and malt. Dry cider is the one closest to a lager in weight.")), (py_tuple(vec![py_str("red wine"), py_str("white wine")]), py_str("Red is fermented with the grape skins, which gives it the colour and the tannin that dries your mouth; white has no skins, so it is lighter in body and tastes of citrus, apple and stone fruit. Reds suit red meat, whites suit fish and chicken.")), (py_tuple(vec![py_str("green tea"), py_str("black tea"), py_str("earl grey")]), py_str("Green tea is unfermented leaf brewed at about 80C, so it is grassy and light. Black tea is fully oxidised and brewed boiling, so it is darker and maltier and takes milk; earl grey is black tea with bergamot oil on it.")), (py_tuple(vec![py_str("gin"), py_str("vodka")]), py_str("Gin is vodka with juniper and other botanicals distilled in, so it tastes of something. Vodka is distilled to taste of almost nothing, which is why it is the one you mix.")), (py_tuple(vec![py_str("whisky"), py_str("vodka")]), py_str("Whisky is grain spirit aged in wood, so it is coloured and tastes of vanilla, caramel or smoke; vodka is unaged and filtered to be neutral. Whisky is sipped and vodka is usually mixed.")), (py_tuple(vec![py_str("matcha"), py_str("green tea")]), py_str("Matcha is green tea leaf ground to a powder and whisked into the water, so you drink the whole leaf: it is stronger, creamier and more bitter than green tea, which is the leaf steeped and taken out.")), (py_tuple(vec![py_str("cappuccino"), py_str("flat white"), py_str("mocha")]), py_str("A mocha is a latte with chocolate in it: sweet, and coffee second. A flat white is the strongest of the milky coffees and a cappuccino is the foamiest."))]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drink-taste")); __args.push(py_tuple(vec![g__DRINK_NAMES(), g__ITS_TASTE()])); __args.push(g__the_drink_taste()); __kws.extend(vec![("avoid".to_string(), g__NOT_A_TASTE()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drink-strength")); __args.push(py_tuple(vec![g__DRINK_NAMES(), g__ITS_STRENGTH()])); __args.push(g__how_strong()); __kws.extend(vec![("avoid".to_string(), g__NOT_A_TASTE()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drink-what-it-is")); __args.push(py_tuple(vec![g__DRINK_NAMES(), g__ITSELF()])); __args.push(g__the_drink_itself()); __kws.extend(vec![("avoid".to_string(), g__NOT_A_TASTE()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drink-goes-with")); __args.push(py_tuple(vec![g__DRINK_NAMES(), g__GOES_WITH()])); __args.push(g__what_it_goes_with()); __kws.extend(vec![("avoid".to_string(), g__NOT_A_TASTE()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drink-difference")); __args.push(py_tuple(vec![g__DRINK_NAMES(), py_tuple(vec![py_str("difference between"), py_str("difference"), py_str("vs"), py_str("versus"), py_str("compared")])])); __args.push(g__the_difference()); __kws.extend(vec![("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
}
pub fn _named(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line names one of these foods, as whole words.");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    pattern = op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:s|es)?(?![a-z0-9])"));
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _wine_pair(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut foods: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The bottle that goes with the food named, or the weight rule for a meal with no food.\n\n    Returns None when the line names neither, so a bottle to buy, a gift, a unit count and a\n    wine stain all stay with the rows that answer them.\n    ");
    text = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    text = mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__PAIRS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    foods = __unpk2[0].clone();
    answer = __unpk2[1].clone();
    if py_truthy(&_named(&[text.clone(), foods.clone()])) {
    return answer.clone();
    }
    }
    if py_truthy(&_named(&[text.clone(), g__MEALS()])) {
    return g__WEIGHT();
    }
    return Py::None;
    Py::None
}
pub fn _how_strong(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("How strong the drink named in the line is, when the line asks that rather than the taste.");
    row = _a_drink(&[reading.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&geti(&row.clone(), &Py::Int(0)));
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &name.clone()))) {
    return { let __b = mcall(&g__STRENGTH(), "get", &[name.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__STRENGTH(), "get", &[geti(&geti(&row.clone(), &Py::Int(0)), &Py::Int(0))]) } };
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_drink(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The drink named in the line, as its row, or None.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    said = stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_TASTE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MY_OWN_CUP()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g__DRINKS());
    for __item in __iter1 {
    row = __item.clone();
    names = geti(&row.clone(), &Py::Int(0));
    let __iter2 = py_iter_or_raise(&names.clone());
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), row.clone()]);
    }
    }
    }
    }
    return if py_truthy(&best.clone()) { geti(&best.clone(), &Py::Int(1)) } else { Py::None };
    Py::None
}
pub fn _the_drink_taste(args: &[Py]) -> Py {
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
    let mut row: Py = Py::None;
    let _ = py_str("What the drink named in the line tastes like.");
    row = _a_drink(&[reading.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    return geti(&row.clone(), &Py::Int(2));
    Py::None
}
pub fn _the_drink_itself(args: &[Py]) -> Py {
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
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("What the drink is, for the line that asks what it is rather than what it tastes like.");
    row = _a_drink(&[reading.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str(" like "))); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str(" like")]) } }) {
    return geti(&row.clone(), &Py::Int(2));
    }
    return geti(&row.clone(), &Py::Int(1));
    Py::None
}
pub fn _what_it_goes_with(args: &[Py]) -> Py {
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
    let mut row: Py = Py::None;
    let _ = py_str("What to eat with it.");
    row = _a_drink(&[reading.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    return geti(&row.clone(), &Py::Int(3));
    Py::None
}
pub fn _the_difference(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut row: Py = Py::None;
    let mut rows: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("Two drinks named in one line: what the difference is.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    said = stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g__DRINKS());
    for __item in __iter1 {
    row = __item.clone();
    let __iter2 = py_iter_or_raise(&geti(&row.clone(), &Py::Int(0)));
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![mcall(&said.clone(), "index", &[op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))]), geti(&geti(&row.clone(), &Py::Int(0)), &Py::Int(0))])]);
    break;
    }
    }
    }
    let _ = mcall(&found.clone(), "sort", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    pair = py_set(vec![geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(1)), geti(&geti(&found.clone(), &Py::Int(1)), &Py::Int(1))]);
    let __iter3 = py_iter_or_raise(&mcall(&g__DIFFERENCES(), "items", &[]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    names = __unpk4[0].clone();
    reply = __unpk4[1].clone();
    if py_truthy(&op_le(&pair.clone(), &call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return reply.clone();
    }
    }
    rows = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g__DRINKS()) { let mut row: Py = __item.clone(); __out.push((geti(&geti(&row.clone(), &Py::Int(0)), &Py::Int(0)), row.clone())); } py_dict(__out) })() };
    let __unpk5 = py_unpack(&py_tuple(vec![mcall(&rows.clone(), "get", &[geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(1))]), mcall(&rows.clone(), "get", &[geti(&geti(&found.clone(), &Py::Int(1)), &Py::Int(1))])]), 2, None);
    first = __unpk5[0].clone();
    second = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&first.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&second.clone(), &Py::None)) } }) {
    return Py::None;
    }
    return op_add(&op_add(&geti(&first.clone(), &Py::Int(1)), &py_str(" ")), &geti(&second.clone(), &Py::Int(1)));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.drinks".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_DIFFERENCES" => Some(g__DIFFERENCES()),
            "_DRINKS" => Some(g__DRINKS()),
            "_DRINK_NAMES" => Some(g__DRINK_NAMES()),
            "_GOES_WITH" => Some(g__GOES_WITH()),
            "_ITSELF" => Some(g__ITSELF()),
            "_ITS_STRENGTH" => Some(g__ITS_STRENGTH()),
            "_ITS_TASTE" => Some(g__ITS_TASTE()),
            "_MEALS" => Some(g__MEALS()),
            "_MY_OWN_CUP" => Some(g__MY_OWN_CUP()),
            "_NOT_A_TASTE" => Some(g__NOT_A_TASTE()),
            "_PAIRS" => Some(g__PAIRS()),
            "_STRENGTH" => Some(g__STRENGTH()),
            "_WEIGHT" => Some(g__WEIGHT()),
            "_a_drink" => Some(py_func("_a_drink", |args: &[Py]| _a_drink(args))),
            "_how_strong" => Some(py_func("_how_strong", |args: &[Py]| _how_strong(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_the_difference" => Some(py_func("_the_difference", |args: &[Py]| _the_difference(args))),
            "_the_drink_itself" => Some(py_func("_the_drink_itself", |args: &[Py]| _the_drink_itself(args))),
            "_the_drink_taste" => Some(py_func("_the_drink_taste", |args: &[Py]| _the_drink_taste(args))),
            "_what_it_goes_with" => Some(py_func("_what_it_goes_with", |args: &[Py]| _what_it_goes_with(args))),
            "_wine_pair" => Some(py_func("_wine_pair", |args: &[Py]| _wine_pair(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_DIFFERENCES" => s__DIFFERENCES(v),
            "_DRINKS" => s__DRINKS(v),
            "_DRINK_NAMES" => s__DRINK_NAMES(v),
            "_GOES_WITH" => s__GOES_WITH(v),
            "_ITSELF" => s__ITSELF(v),
            "_ITS_STRENGTH" => s__ITS_STRENGTH(v),
            "_ITS_TASTE" => s__ITS_TASTE(v),
            "_MEALS" => s__MEALS(v),
            "_MY_OWN_CUP" => s__MY_OWN_CUP(v),
            "_NOT_A_TASTE" => s__NOT_A_TASTE(v),
            "_PAIRS" => s__PAIRS(v),
            "_STRENGTH" => s__STRENGTH(v),
            "_WEIGHT" => s__WEIGHT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.drinks", module_obj);
    pyrt::xrefs::reg_fn("knowledge.drinks", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_wine_pair", py_func("_wine_pair", |args: &[Py]| _wine_pair(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_how_strong", py_func("_how_strong", |args: &[Py]| _how_strong(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_a_drink", py_func("_a_drink", |args: &[Py]| _a_drink(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_the_drink_taste", py_func("_the_drink_taste", |args: &[Py]| _the_drink_taste(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_the_drink_itself", py_func("_the_drink_itself", |args: &[Py]| _the_drink_itself(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_what_it_goes_with", py_func("_what_it_goes_with", |args: &[Py]| _what_it_goes_with(args)));
    pyrt::xrefs::reg_fn("knowledge.drinks", "_the_difference", py_func("_the_difference", |args: &[Py]| _the_difference(args)));
    pyrt::xrefs::reg_access("knowledge.drinks", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.drinks", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.drinks", "_DIFFERENCES", g__DIFFERENCES, s__DIFFERENCES);
    pyrt::xrefs::reg_access("knowledge.drinks", "_DRINKS", g__DRINKS, s__DRINKS);
    pyrt::xrefs::reg_access("knowledge.drinks", "_DRINK_NAMES", g__DRINK_NAMES, s__DRINK_NAMES);
    pyrt::xrefs::reg_access("knowledge.drinks", "_GOES_WITH", g__GOES_WITH, s__GOES_WITH);
    pyrt::xrefs::reg_access("knowledge.drinks", "_ITSELF", g__ITSELF, s__ITSELF);
    pyrt::xrefs::reg_access("knowledge.drinks", "_ITS_STRENGTH", g__ITS_STRENGTH, s__ITS_STRENGTH);
    pyrt::xrefs::reg_access("knowledge.drinks", "_ITS_TASTE", g__ITS_TASTE, s__ITS_TASTE);
    pyrt::xrefs::reg_access("knowledge.drinks", "_MEALS", g__MEALS, s__MEALS);
    pyrt::xrefs::reg_access("knowledge.drinks", "_MY_OWN_CUP", g__MY_OWN_CUP, s__MY_OWN_CUP);
    pyrt::xrefs::reg_access("knowledge.drinks", "_NOT_A_TASTE", g__NOT_A_TASTE, s__NOT_A_TASTE);
    pyrt::xrefs::reg_access("knowledge.drinks", "_PAIRS", g__PAIRS, s__PAIRS);
    pyrt::xrefs::reg_access("knowledge.drinks", "_STRENGTH", g__STRENGTH, s__STRENGTH);
    pyrt::xrefs::reg_access("knowledge.drinks", "_WEIGHT", g__WEIGHT, s__WEIGHT);
}
