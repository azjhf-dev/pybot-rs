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
thread_local! { static G__APPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FLOURY_OR_WAXY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHEESE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHOCOLATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHOOSE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHOOSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FLOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAMB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSHROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POTATO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOMATO: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__APPLE() -> Py { ensure_init(); G__APPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__APPLE(v: Py) { ensure_init(); G__APPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FLOURY_OR_WAXY() -> Py { ensure_init(); G__A_FLOURY_OR_WAXY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FLOURY_OR_WAXY(v: Py) { ensure_init(); G__A_FLOURY_OR_WAXY.with(|c| *c.borrow_mut() = v); }
pub fn g__BEEF() -> Py { ensure_init(); G__BEEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEEF(v: Py) { ensure_init(); G__BEEF.with(|c| *c.borrow_mut() = v); }
pub fn g__CHEESE() -> Py { ensure_init(); G__CHEESE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHEESE(v: Py) { ensure_init(); G__CHEESE.with(|c| *c.borrow_mut() = v); }
pub fn g__CHOCOLATE() -> Py { ensure_init(); G__CHOCOLATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHOCOLATE(v: Py) { ensure_init(); G__CHOCOLATE.with(|c| *c.borrow_mut() = v); }
pub fn g__CHOOSE_WORDS() -> Py { ensure_init(); G__CHOOSE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHOOSE_WORDS(v: Py) { ensure_init(); G__CHOOSE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CHOOSING() -> Py { ensure_init(); G__CHOOSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHOOSING(v: Py) { ensure_init(); G__CHOOSING.with(|c| *c.borrow_mut() = v); }
pub fn g__FISH() -> Py { ensure_init(); G__FISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FISH(v: Py) { ensure_init(); G__FISH.with(|c| *c.borrow_mut() = v); }
pub fn g__FLOUR() -> Py { ensure_init(); G__FLOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FLOUR(v: Py) { ensure_init(); G__FLOUR.with(|c| *c.borrow_mut() = v); }
pub fn g__LAMB() -> Py { ensure_init(); G__LAMB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAMB(v: Py) { ensure_init(); G__LAMB.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSHROOM() -> Py { ensure_init(); G__MUSHROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSHROOM(v: Py) { ensure_init(); G__MUSHROOM.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_CHOICE() -> Py { ensure_init(); G__NOT_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_CHOICE(v: Py) { ensure_init(); G__NOT_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g__ONION() -> Py { ensure_init(); G__ONION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONION(v: Py) { ensure_init(); G__ONION.with(|c| *c.borrow_mut() = v); }
pub fn g__POTATO() -> Py { ensure_init(); G__POTATO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POTATO(v: Py) { ensure_init(); G__POTATO.with(|c| *c.borrow_mut() = v); }
pub fn g__RICE() -> Py { ensure_init(); G__RICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RICE(v: Py) { ensure_init(); G__RICE.with(|c| *c.borrow_mut() = v); }
pub fn g__TOMATO() -> Py { ensure_init(); G__TOMATO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOMATO(v: Py) { ensure_init(); G__TOMATO.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Which one to buy: the variety of an everyday ingredient for the job in hand.\n\n\"which potatoes are best for mash\", \"what rice should i use for a risotto\", \"which cheese for\npizza\" are not cooking-method questions and not recipe questions, and the bot had no answer for\nany of them: on main all twelve lines measured either got the method card for the ingredient\n(\"Boil them in salted water: 15 to 20 minutes ...\"), a recipe for a dish nobody asked about\n(\"How about mushrooms on toast ...\"), or the punt (\"I'm not sure about that one, sorry\"). A\nperson choosing in a shop wants one thing - the name of the variety - so that is the first\nsentence of every row here.\n\nOne domain of its own so that its ingredient names do not change how rare a word is inside\nanother domain's keys (lookup.entries points that out: a new domain may not take a point away\nfrom an entry somewhere else).\n\nThe shape of the family, rather than a list of lines: the key is the ingredient, and the line\nmust be *choosing* one - `which`, `what sort of`, `best for`, `should i get`. A method line\n(\"how do i boil potatoes\") and a portion line (\"how many potatoes per person\") fail the guard\nand keep the rows that answer them. The use decides the variety, so each row is one table of\n(kind of job -> variety and why) and a plain \"which potatoes are best\" gets the split.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s__CHOOSING(stdlib_call("re", "compile", &[py_str("\\b(which|what (sort|kind|type|variety)|best|better for|good for|should i (get|buy|use|pick)|shall i (get|buy|use)|right (one|type|kind)|the one for)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\b(how (long|many|much|do i|to)|minutes|temperature|calories|per person|recipe|reheat|freeze|freezer|leftover|left over|calorie|good for (you|me|us|kids|children|my|the)|bad for (you|me|us|kids|children|my|the)|healthy|unhealthy|fattening)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FLOURY_OR_WAXY(stdlib_call("re", "compile", &[py_str("\\b(?:floury|waxy|fluffy)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POTATO(py_tuple(vec![py_tuple(vec![py_str("\\broast(s|ed|ing)?\\b|\\broasties\\b|\\boven potato"), py_str("Maris Piper or King Edward for roast potatoes: a floury one crisps at the edges where a waxy one goes hard and pale. Parboil 8 minutes, shake them in the pan to rough the edges, then roast at 200C (180C fan).")]), py_tuple(vec![py_str("\\bmash(ed|ing)?\\b"), py_str("Maris Piper, King Edward or Rooster for mash: floury potatoes mash light, and waxy ones go gluey. Steam them dry for a minute after draining and mash with warm milk and butter.")]), py_tuple(vec![py_str("\\bwedges?\\b"), py_str("Maris Piper or King Edward for wedges: cut them into sixths, parboil 5 minutes, toss in oil and roast at 200C (180C fan) for 25 minutes. A floury potato crisps outside and stays fluffy in the middle.")]), py_tuple(vec![py_str("\\bchips?\\b|\\bfries\\b|\\bchippy\\b"), py_str("Maris Piper or Rooster for chips: floury potatoes take the double fry without going greasy, and the second fry at 190C is what makes them crisp.")]), py_tuple(vec![py_str("\\bsalad\\b|\\bboil(ed|ing)? whole\\b|\\bnew potato"), py_str("Charlotte, Jersey Royal or Nicola for a salad or boiled whole: waxy potatoes hold their shape, where a floury one breaks up in the bowl.")])]));
    s__APPLE(py_tuple(vec![py_tuple(vec![py_str("\\bcrumble\\b|\\bpie\\b|\\btart\\b|\\bcook(ing|ed)?\\b|\\bbak(e|ing)\\b|\\bstew(ed|ing)?\\b|\\bsauce\\b|\\bchutney\\b|\\bbramble\\b"), py_str("Bramley for a crumble or a pie: it is sharp and collapses where an eater stays in lumps. One eater (Cox or Braeburn) to two Bramleys if you like a bit of bite left.")]), py_tuple(vec![py_str("\\bsalad\\b|\\beat(ing)? raw\\b|\\braw\\b|\\bsnack\\b|\\blunchbox\\b|\\bcheese\\b"), py_str("Cox, Braeburn or Gala for eating raw: firm, sharp and they do not go floury in a bag. Pink Lady and Jazz are sweeter, so keep them for a snack on their own.")])]));
    s__ONION(py_tuple(vec![py_tuple(vec![py_str("\\braw\\b|\\bsalad\\b|\\bsalsa\\b|\\bsandwich\\b|\\bguacamole\\b|\\bburger\\b|\\bchutney\\b"), py_str("Red onion for anything raw - a salad, a salsa, a sandwich: it is milder and keeps its crunch. A shallot is gentler still and a good swap in a dressing.")]), py_tuple(vec![py_str("\\bcurry\\b|\\bstew\\b|\\bsoup\\b|\\bbolognese\\b|\\bsauce\\b|\\bslow\\b|\\bcasserol|\\bchilli\\b|\\bbase\\b|\\bfry(ing)?\\b|\\bbrown(ing)?\\b|\\bpasta\\b"), py_str("Brown onions for cooking: the most sugar, so they go sweet and soft in a curry or a stew. Shallots are milder and finer, which is why the French use them for a sauce.")])]));
    s__TOMATO(py_tuple(vec![py_tuple(vec![py_str("\\bsalad\\b|\\braw\\b|\\bsandwich\\b|\\bon toast\\b|\\bbruschetta\\b"), py_str("Cherry or piccolo tomatoes for a salad: smaller and sweeter, because they are bred to be eaten raw. On the vine are fresher than loose ones, and never keep them in the fridge.")]), py_tuple(vec![py_str("\\bsauce\\b|\\bpassata\\b|\\bpasta\\b|\\bstew\\b|\\bcook(ing|ed)?\\b|\\bbolognese\\b|\\bsoup\\b|\\bsauce\\b|\\bpizza\\b|\\bcurry\\b"), py_str("Plum tomatoes for a sauce - Roma or San Marzano: drier, fewer seeds, and they cook down sweet. Out of season a tin of plum tomatoes beats the fresh ones on the shelf.")])]));
    s__RICE(py_tuple(vec![py_tuple(vec![py_str("\\bcurry\\b|\\bbasmati\\b|\\bpilau\\b|\\bbiryani\\b|\\bindian\\b|\\bdaal\\b|\\bdhal\\b"), py_str("Basmati for a curry: long, separate grains that stay light under a wet sauce. Wash it until the water runs clear and cook it 1 to 1.5 with water.")]), py_tuple(vec![py_str("\\bthai\\b|\\bstir.?fr(y|ied)\\b|\\bjasmine\\b|\\bchinese\\b|\\bfried rice\\b"), py_str("Jasmine for Thai and stir-fried dishes: softer and a little sticky, so it eats well with a wet sauce. Sushi wants short-grain, which is a different rice again.")]), py_tuple(vec![py_str("\\bsushi\\b|\\bshort.?grain\\b|\\bkoshihikari\\b"), py_str("Short-grain for sushi - sold as sushi rice, and koshihikari if you want the best: it is sticky enough to hold the roll together where long-grain falls apart. Rinse it until the water runs clear and use a little less water than the packet says.")]), py_tuple(vec![py_str("\\brisotto\\b|\\barborio\\b|\\bcarnaroli\\b|\\bpaella\\b|\\bbomba\\b|\\bpudding\\b|\\bsoup\\b"), py_str("Arborio or carnaroli for a risotto (carnaroli holds its bite better), bomba for a paella, and short pudding rice for rice pudding. None of them take kindly to being rinsed.")])]));
    s__MUSHROOM(py_tuple(vec![py_tuple(vec![py_str("\\bstir.?fr(y|ied)\\b|\\bnoodle|\\basian\\b|\\bthai\\b|\\bchinese\\b"), py_str("Oyster or shiitake for a stir-fry: they hold their shape and taste of something. Add them to a hot pan dry first, then the oil, or they steam.")]), py_tuple(vec![py_str("\\bburger\\b|\\bgrill(ed|ing)?\\b|\\bstack\\b|\\bbreakfast\\b|\\btoast\\b"), py_str("Portobello for a burger or under the grill: big, flat and meaty enough to be the main thing. Brush them with oil and grill 4 minutes a side.")]), py_tuple(vec![py_str("\\bsalad\\b|\\braw\\b"), py_str("Button mushrooms for a raw salad: mild and firm. They are grown in compost, so wipe them rather than soak them, and dress them just before serving.")])]));
    s__CHEESE(py_tuple(vec![py_tuple(vec![py_str("\\bpizza\\b|\\bmelting\\b|\\bmelt\\b|\\btoastie\\b|\\bcalzone\\b"), py_str("Low-moisture mozzarella for a pizza: it melts and stretches without a puddle, which fresh mozzarella will give you. Tear a fresh one and drain it on kitchen paper first if that is what you have.")]), py_tuple(vec![py_str("\\bsandwich\\b|\\btoast\\b|\\bsauce\\b|\\bmacaroni\\b|\\bcauliflower\\b|\\bcrumble\\b|\\bpasta bake\\b"), py_str("A mature cheddar for a sandwich or a cheese sauce: it carries the flavour when it is diluted. Gruyere melts more smoothly, so a rarebit or a fondue wants that.")]), py_tuple(vec![py_str("\\bgrat(ed|ing)?\\b|\\bpasta\\b|\\brisotto\\b|\\bsoup\\b"), py_str("Parmesan (or pecorino) for grating over pasta, risotto or soup: salty and dry, so a little goes a long way. Buy it in a wedge and grate it; the ready-grated tub tastes of nothing.")]), py_tuple(vec![py_str("\\bsalad\\b|\\bfeta\\b|\\bhalloumi\\b|\\bgreek\\b"), py_str("Feta for a salad: salty and crumbly, and it does not melt into the leaves. Halloumi is the one for grilling or frying, because it holds its shape.")])]));
    s__FISH(py_tuple(vec![py_tuple(vec![py_str("\\bcurry\\b|\\bstew\\b|\\bpie\\b|\\bsoup\\b|\\bcasserole\\b|\\btraybake\\b|\\bbake\\b"), py_str("A firm white fish for a curry, a pie or a traybake - cod, haddock or hake: it holds together where a delicate one flakes away. Add it late and cook it gently.")]), py_tuple(vec![py_str("\\bfr(y|ied|ying)\\b|\\bpan\\b|\\bgrill(ed|ing)?\\b|\\broast(ed|ing)?\\b"), py_str("A firm white fish (cod, haddock, hake) or an oily one (salmon, mackerel) for the pan or the grill: both stand up to the heat. Mackerel is the cheapest fish worth eating twice a week.")]), py_tuple(vec![py_str("\\bsalad\\b|\\bsandwich\\b|\\braw\\b|\\bsushi\\b"), py_str("Salmon, tuna or a hot-smoked fish for a salad or a sandwich: meaty enough to be the main thing. Raw fish for sushi wants fish frozen first and sold as sushi grade.")])]));
    s__BEEF(py_tuple(vec![py_tuple(vec![py_str("\\bstew(s|ed|ing)?\\b|\\bslow\\b|\\bcasserole\\b|\\bbraise[ds]?\\b|\\bpot roast\\b|\\bpie\\b|\\bcurry\\b|\\bmince\\b|\\bbolognese\\b"), py_str("Chuck or blade for a stew or anything slow: both are full of connective tissue, which turns to gelatin and gives you a thick gravy. 2 to 3 hours at a bare simmer, and never a rolling boil.")]), py_tuple(vec![py_str("\\broast\\b|\\bsunday\\b|\\bcarvery\\b"), py_str("For roasting, topside or silverside for lean and easy to carve, or rib for flavour and fat. Beef for roasting wants 20 minutes at 220C then 15 minutes per 500g at 180C for rare.")]), py_tuple(vec![py_str("\\bsteak\\b|\\bfry(ing)?\\b|\\bgrill(ed|ing)?\\b|\\bbarbecue\\b|\\bbbq\\b|\\bsandwich\\b"), py_str("Ribeye for flavour, sirloin for a firmer bite, fillet for tenderness and the most money for the least taste. For a steak sandwich, a cheaper cut - flank or bavette - sliced thin across the grain is what restaurants use.")])]));
    s__FLOUR(py_tuple(vec![py_tuple(vec![py_str("\\bbread\\b|\\bpizza\\b|\\bbaguette\\b|\\bbap\\b|\\bfocaccia\\b|\\bpitta\\b|\\bnaan\\b"), py_str("Strong white bread flour for bread and pizza: the extra gluten is what makes the dough rise and stretch. Plain flour makes a flat, crumbly loaf, and wholemeal needs a little extra water and a longer rise.")]), py_tuple(vec![py_str("\\bcake\\b|\\bsponge\\b|\\bscone\\b|\\bself.?raising\\b|\\bvictoria\\b|\\bmuffin\\b|\\bflapjack\\b"), py_str("Self-raising flour for a sponge or a scone, or plain flour with 2 teaspoons of baking powder per 100g. Strong bread flour is the one to keep out of a cake, because the gluten makes it chewy.")]), py_tuple(vec![py_str("\\bpastry\\b|\\bpie\\b|\\btart\\b|\\bshortcrust\\b|\\byorkshire\\b|\\bbatter\\b|\\bsauce\\b|\\bthicken\\b|\\bcrumble\\b"), py_str("Plain flour for pastry, Yorkshire pudding, batter and sauces: no raising agent and less gluten than bread flour, so it stays short and crisp. Cornflour thickens a sauce without flouring the taste.")])]));
    s__CHOCOLATE(py_tuple(vec![py_tuple(vec![py_str("\\bbrownie|\\bcake|\\bcookie|\\bbiscuit|\\bmelt|\\bbaking|\\bbake|\\bmousse|\\btruffle|\\bganache|\\bsauce|\\bicing|\\bicing sugar|\\bcooking\\b"), py_str("A 70% dark chocolate for brownies and cakes: bitter enough to cut the sugar in the batter, and it melts cleanly. Milk chocolate makes a brownie too sweet and 85% makes it dry, so 70% is the one to buy.")]), py_tuple(vec![py_str("\\beat(ing)?\\b|\\bsnack\\b|\\bgift\\b|\\bbar\\b|\\braw\\b|\\bhot chocolate\\b|\\bdrink\\b"), py_str("For eating on its own, 70% is where chocolate starts to taste of something: a supermarket own-brand 70% is as good as the expensive bars for most people. Milk chocolate is a different thing, and better eaten than baked with.")])]));
    s__LAMB(py_tuple(vec![py_tuple(vec![py_str("\\bslow\\b|\\bstew(s|ed|ing)?\\b|\\bcasserole\\b|\\bbraise[ds]?\\b|\\bpulled\\b|\\bcurry\\b"), py_str("Shoulder for anything slow, or neck fillet cut into chunks for a stew: both give up their fat and gelatine over a couple of hours. Leg is the one to keep for roasting, because it is leaner and goes dry in a casserole.")]), py_tuple(vec![py_str("\\broast\\b|\\bsunday\\b|\\bcarvery\\b|\\bjoint\\b"), py_str("A leg for a roast - bone in for flavour, boned and rolled for easy carving. Shoulder is cheaper, sweeter and needs 4 hours at 160C instead of 20 minutes a pound at 200C.")]), py_tuple(vec![py_str("\\bchop|\\bgrill(ed|ing)?\\b|\\bbbq\\b|\\bbarbecue\\b|\\bfr(y|ied|ying)\\b|\\bpan\\b"), py_str("Chops for the grill or the pan - loin for lean, best end for the sweetest - and they want 4 minutes a side from room temperature. Cutlets and neck fillet are the cheaper ones that still take a quick cook.")])]));
    s__CHOOSE_WORDS(py_tuple(vec![py_str("which"), py_str("what"), py_str("what sort"), py_str("what kind"), py_str("what type"), py_str("best for"), py_str("better for"), py_str("good for"), py_str("should i get"), py_str("should i buy"), py_str("should i use"), py_str("shall i get"), py_str("do i use"), py_str("do i get"), py_str("do i buy"), py_str("should i pick"), py_str("right one"), py_str("the one for"), py_str("what kinds"), py_str("what types"), py_str("what sorts"), py_str("what varieties"), py_str("which kinds"), py_str("which types"), py_str("kinds of"), py_str("types of"), py_str("sorts of")]));
    s_ENTRIES(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-potato")); __args.push(py_tuple(vec![py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("spud"), py_str("spuds"), py_str("new potatoes")]), op_add(&g__CHOOSE_WORDS(), &py_tuple(vec![py_str("floury"), py_str("waxy"), py_str("fluffy")]))])); __args.push(_pick(&[g__POTATO(), py_str("Floury potatoes (Maris Piper, King Edward, Rooster) for roast, mash and chips, waxy ones (Charlotte, Jersey Royal, Nicola) for salads and boiling whole. The bag says which on the front, so buy by that rather than by the price.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("temperature"), py_str("calories"), py_str("per person"), py_str("recipe"), py_str("reheat"), py_str("freeze"), py_str("leftover"), py_str("left over"), py_str("grow"), py_str("growing"), py_str("chit"), py_str("sprout"), py_str("green"), py_str("peel")])), ("guard".to_string(), g__potato_kind_is_named()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-apple")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apple"), py_str("apples")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__APPLE(), py_str("Bramley for cooking, Cox or Braeburn for eating: a Bramley is sharp and falls apart, and an eater stays in pieces. A bag of 'cooking apples' at the market is a Bramley.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("storage"), py_str("brown"), py_str("went brown"), py_str("calories"), py_str("healthy"), py_str("tree"), py_str("grow"), py_str("growing"), py_str("recipe")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("apple"), py_str("apples")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-onion")); __args.push(py_tuple(vec![py_tuple(vec![py_str("onion"), py_str("onions"), py_str("shallot"), py_str("shallots")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__ONION(), py_str("Brown onions are the all-rounder: sweet and soft once cooked. Red for anything raw, shallots for a sauce you want gentler, and white for a Mexican dish.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("sprout"), py_str("sprouting"), py_str("make me cry"), py_str("cry"), py_str("eyes water"), py_str("chop"), py_str("chopping"), py_str("peel"), py_str("recipe"), py_str("calories")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("onion"), py_str("onions"), py_str("shallot"), py_str("shallots")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-tomato")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("cherry tomatoes"), py_str("plum tomatoes")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__TOMATO(), py_str("Plum tomatoes for cooking and cherry ones for eating raw: plum are drier and fewer-seeded, so they make a sauce; cherry are sweeter, so they make a salad.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("refrigerate"), py_str("ripen"), py_str("ripening"), py_str("green"), py_str("blight"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("recipe"), py_str("calories")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("cherry tomatoes"), py_str("plum tomatoes")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-rice")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("jasmine rice"), py_str("risotto rice"), py_str("pudding rice")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__RICE(), py_str("Basmati for curry, jasmine for Thai and stir-fried, arborio or carnaroli for risotto, pudding rice for rice pudding. Plain long-grain is the all-rounder and the one to buy in a big bag.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("cook"), py_str("cooking"), py_str("wash"), py_str("rinse"), py_str("leftover"), py_str("left over"), py_str("reheat"), py_str("freeze"), py_str("calories"), py_str("per person"), py_str("microwave"), py_str("recipe"), py_str("water"), py_str("phone"), py_str("phones"), py_str("iphone"), py_str("mobile"), py_str("laptop"), py_str("tablet"), py_str("camera"), py_str("device"), py_str("wet phone"), py_str("water damage"), py_str("dropped"), py_str("dropped it"), py_str("or pasta"), py_str("or potatoes"), py_str("or potato"), py_str("or bread"), py_str("or noodles"), py_str("or couscous"), py_str("better for you"), py_str("healthier"), py_str("healthiest")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("jasmine rice"), py_str("risotto rice"), py_str("pudding rice")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-mushroom")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mushroom"), py_str("mushrooms")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__MUSHROOM(), py_str("Chestnut mushrooms are the all-rounder for cooking, button for raw, portobello for a burger, and oyster or shiitake for a stir-fry. Wipe them rather than washing them.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("wash"), py_str("washing"), py_str("dirt"), py_str("grow"), py_str("growing"), py_str("pick"), py_str("picking"), py_str("forage"), py_str("foraging"), py_str("recipe"), py_str("calories"), py_str("poisonous"), py_str("safe")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("mushroom"), py_str("mushrooms")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-cheese")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("cheddar"), py_str("mozzarella"), py_str("parmesan"), py_str("feta")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__CHEESE(), py_str("Cheddar is the all-rounder for cooking and sandwiches, mozzarella for a pizza, parmesan for grating and feta for a salad. Gruyere is the one that melts smoothly for a rarebit.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("freeze"), py_str("mould"), py_str("mold"), py_str("calories"), py_str("healthy"), py_str("lactose"), py_str("milk"), py_str("vegan"), py_str("recipe")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("cheese"), py_str("cheddar"), py_str("mozzarella"), py_str("parmesan"), py_str("feta")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-fish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("cod"), py_str("haddock"), py_str("salmon"), py_str("mackerel"), py_str("hake")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__FISH(), py_str("Cod, haddock or hake for anything that cooks in a sauce, salmon or mackerel for the pan and the grill, and a firm white fish for a pie. Mackerel is the cheapest fish worth eating every week.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("freeze"), py_str("fresh"), py_str("smell"), py_str("calories"), py_str("healthy"), py_str("mercury"), py_str("pregnant"), py_str("bone"), py_str("bones"), py_str("recipe"), py_str("sustainable"), py_str("farmed"), py_str("wild")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("fish"), py_str("cod"), py_str("haddock"), py_str("salmon"), py_str("mackerel"), py_str("hake")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-flour")); __args.push(py_tuple(vec![py_tuple(vec![py_str("flour"), py_str("bread flour"), py_str("self raising flour"), py_str("plain flour")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__FLOUR(), py_str("Strong white bread flour for bread and pizza, plain for pastry and sauces, self-raising for a sponge. The bag names the job on the front.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("weevils"), py_str("moths"), py_str("calories"), py_str("healthy"), py_str("gluten"), py_str("freeze"), py_str("recipe"), py_str("substitute"), py_str("instead of")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("flour"), py_str("bread flour"), py_str("self raising flour"), py_str("plain flour")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-chocolate")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chocolate"), py_str("dark chocolate"), py_str("milk chocolate")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__CHOCOLATE(), py_str("70% dark for baking and for eating on its own: it is the one that tastes of chocolate without being bitter. Milk chocolate is for eating.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("bloom"), py_str("white bloom"), py_str("calories"), py_str("healthy"), py_str("caffeine"), py_str("dogs"), py_str("dog"), py_str("melt"), py_str("temper"), py_str("recipe")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("chocolate"), py_str("dark chocolate"), py_str("milk chocolate")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-lamb")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lamb"), py_str("mutton")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__LAMB(), py_str("Shoulder for anything slow, leg for a roast, chops for the grill. Shoulder is the cheaper half and the one with more flavour.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("freeze"), py_str("mince"), py_str("defrost"), py_str("calories"), py_str("healthy"), py_str("pink"), py_str("safe"), py_str("food poisoning"), py_str("recipe"), py_str("price")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("lamb"), py_str("mutton")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-beef")); __args.push(py_tuple(vec![py_tuple(vec![py_str("beef"), py_str("steak"), py_str("chuck"), py_str("brisket"), py_str("mince")]), g__CHOOSE_WORDS()])); __args.push(_pick(&[g__BEEF(), py_str("Chuck or blade for anything slow, topside or rib for a roast, ribeye or sirloin for a steak. The cheap cuts are the ones with the flavour; they just need time.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("freeze"), py_str("mince"), py_str("defrost"), py_str("calories"), py_str("healthy"), py_str("iron"), py_str("pink"), py_str("safe"), py_str("food poisoning"), py_str("recipe"), py_str("price"), py_str("cheap"), py_str("price per")])), ("guard".to_string(), _chooser(&[py_tuple(vec![py_str("beef"), py_str("steak"), py_str("chuck"), py_str("brisket"), py_str("mince")])])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
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
    let mut text: Py = Py::None;
    let _ = py_str("The repaired line, lower case, with a space at each end, for a whole-word search.");
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    return op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _names_pattern(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut n: Py = Py::None;
    let _ = py_str("The ingredient words as one alternation, longest first so \"plum tomatoes\" wins.");
    return mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut n: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[n.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    Py::None
}
pub fn _potato_kind_is_named(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks which sort of potato, by the chooser or by the sort's own word.");
    if py_truthy(&py_call(&_chooser(&[py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("spud"), py_str("spuds"), py_str("new potatoes")])]), &[understood.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    return { let __b = call_builtin("bool", &[mcall(&g__A_FLOURY_OR_WAXY(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bpotato"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _chooser(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut bare: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut guard: Py = Py::None;
    let mut ingredient: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut trail: Py = Py::None;
    let _ = py_str("A guard: the choosing word has to be about *this* ingredient.\n\n    \"which knife should i use for chopping an onion\" and \"which oil for roast potatoes\" hold a\n    choosing word, an ingredient and a use, and both are about the knife and the oil. So the\n    choice has to be attached to the ingredient - \"which potatoes\", \"what sort of potatoes\",\n    \"potatoes ... should i use\", \"potatoes are best for\" - and a word in between is a different\n    subject. A method, a portion or a health question is turned away whatever the word order.\n    ");
    ingredient = _names_pattern(&[names.clone()]);
    lead = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(which|what (sort|kind|type|variety)s? of|(sort|kind|type|variety)s? of|best|better|right)\\s+(the\\s+)?("), &ingredient.clone()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e));
    cut = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(which|what)\\s+((cut|piece|joint|part|side)s?\\s+of\\s+)(the\\s+)?("), &ingredient.clone()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e));
    bare = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bwhat\\s+(the\\s+)?("), &ingredient.clone()), &py_str(")\\b[^.!?]{0,15}?\\bfor\\b"))]).unwrap_or_else(|e| exc::raise(e));
    trail = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &ingredient.clone()), &py_str(")\\b[^.!?]{0,30}?\\b((should|do|shall|would) i (get|buy|use|pick)|is best|are best|best for|better for|good for)\\b"))]).unwrap_or_else(|e| exc::raise(e));
    kinds = stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:what|which)\\s+(?:are\\s+the\\s+|are\\s+)?(?:different\\s+|main\\s+|other\\s+|the\\s+)?(?:kinds|types|sorts|varieties)\\s+of\\s+(?:the\\s+)?("), &ingredient.clone()), &py_str(")\\b|\\b(?:what|which)\\s+(")), &ingredient.clone()), &py_str(")s?\\b[^.!?]{0,24}?\\b(?:are there|do you get|do you sell|come in)\\b"))]).unwrap_or_else(|e| exc::raise(e));
pub fn __nested_guard_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut bare: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut cut: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut guard: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut ingredient: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut kinds: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut lead: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut names: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut trail: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut text: Py = Py::None;
    text = _line(&[understood.clone()]);
    if py_truthy(&mcall(&g__NOT_A_CHOICE(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&lead.clone(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&cut.clone(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&bare.clone(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&trail.clone(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&kinds.clone(), "search", &[text.clone()]) } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    guard = {{ let bare = bare.clone(); let cut = cut.clone(); let guard = guard.clone(); let ingredient = ingredient.clone(); let kinds = kinds.clone(); let lead = lead.clone(); let names = names.clone(); let trail = trail.clone(); py_func("guard", move |args: &[Py]| { let mut full = vec![bare.clone(), cut.clone(), guard.clone(), ingredient.clone(), kinds.clone(), lead.clone(), names.clone(), trail.clone()]; full.extend_from_slice(args); __nested_guard_0(&full) }) }};
    return guard.clone();
    Py::None
}
pub fn choosing(args: &[Py]) -> Py {
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
    let mut e: Py = Py::None;
    let _ = py_str("True when the line choosing a variety of one of these ingredients.\n\n    `cooking.meal` asks this before it offers a dish: \"what flour should i use for bread\" holds\n    two pantry ingredients, so the what-can-I-make-with table answered it with a pancake recipe.\n    A line that is choosing a kind is not asking what to cook with it.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ENTRIES()) { let mut e: Py = __item.clone(); __out.push(mcall(&e.clone(), "guard", &[understood.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rows: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rows") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rows"))) };
    let mut default: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "default") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: default"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("A reply built from a (pattern -> sentence) table: the first use named wins.");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut default: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut rows: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    text = _line(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&rows.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return said.clone();
    }
    }
    return default.clone();
    Py::None
}
    reply = {{ let default = default.clone(); let reply = reply.clone(); let rows = rows.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![default.clone(), reply.clone(), rows.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.picking".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_APPLE" => Some(g__APPLE()),
            "_A_FLOURY_OR_WAXY" => Some(g__A_FLOURY_OR_WAXY()),
            "_BEEF" => Some(g__BEEF()),
            "_CHEESE" => Some(g__CHEESE()),
            "_CHOCOLATE" => Some(g__CHOCOLATE()),
            "_CHOOSE_WORDS" => Some(g__CHOOSE_WORDS()),
            "_CHOOSING" => Some(g__CHOOSING()),
            "_FISH" => Some(g__FISH()),
            "_FLOUR" => Some(g__FLOUR()),
            "_LAMB" => Some(g__LAMB()),
            "_MUSHROOM" => Some(g__MUSHROOM()),
            "_NOT_A_CHOICE" => Some(g__NOT_A_CHOICE()),
            "_ONION" => Some(g__ONION()),
            "_POTATO" => Some(g__POTATO()),
            "_RICE" => Some(g__RICE()),
            "_TOMATO" => Some(g__TOMATO()),
            "_chooser" => Some(py_func("_chooser", |args: &[Py]| _chooser(args))),
            "_line" => Some(py_func("_line", |args: &[Py]| _line(args))),
            "_names_pattern" => Some(py_func("_names_pattern", |args: &[Py]| _names_pattern(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_potato_kind_is_named" => Some(py_func("_potato_kind_is_named", |args: &[Py]| _potato_kind_is_named(args))),
            "choosing" => Some(py_func("choosing", |args: &[Py]| choosing(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_APPLE" => s__APPLE(v),
            "_A_FLOURY_OR_WAXY" => s__A_FLOURY_OR_WAXY(v),
            "_BEEF" => s__BEEF(v),
            "_CHEESE" => s__CHEESE(v),
            "_CHOCOLATE" => s__CHOCOLATE(v),
            "_CHOOSE_WORDS" => s__CHOOSE_WORDS(v),
            "_CHOOSING" => s__CHOOSING(v),
            "_FISH" => s__FISH(v),
            "_FLOUR" => s__FLOUR(v),
            "_LAMB" => s__LAMB(v),
            "_MUSHROOM" => s__MUSHROOM(v),
            "_NOT_A_CHOICE" => s__NOT_A_CHOICE(v),
            "_ONION" => s__ONION(v),
            "_POTATO" => s__POTATO(v),
            "_RICE" => s__RICE(v),
            "_TOMATO" => s__TOMATO(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.picking", module_obj);
    pyrt::xrefs::reg_fn("knowledge.picking", "_line", py_func("_line", |args: &[Py]| _line(args)));
    pyrt::xrefs::reg_fn("knowledge.picking", "_names_pattern", py_func("_names_pattern", |args: &[Py]| _names_pattern(args)));
    pyrt::xrefs::reg_fn("knowledge.picking", "_potato_kind_is_named", py_func("_potato_kind_is_named", |args: &[Py]| _potato_kind_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.picking", "_chooser", py_func("_chooser", |args: &[Py]| _chooser(args)));
    pyrt::xrefs::reg_fn("knowledge.picking", "choosing", py_func("choosing", |args: &[Py]| choosing(args)));
    pyrt::xrefs::reg_fn("knowledge.picking", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_access("knowledge.picking", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.picking", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.picking", "_APPLE", g__APPLE, s__APPLE);
    pyrt::xrefs::reg_access("knowledge.picking", "_A_FLOURY_OR_WAXY", g__A_FLOURY_OR_WAXY, s__A_FLOURY_OR_WAXY);
    pyrt::xrefs::reg_access("knowledge.picking", "_BEEF", g__BEEF, s__BEEF);
    pyrt::xrefs::reg_access("knowledge.picking", "_CHEESE", g__CHEESE, s__CHEESE);
    pyrt::xrefs::reg_access("knowledge.picking", "_CHOCOLATE", g__CHOCOLATE, s__CHOCOLATE);
    pyrt::xrefs::reg_access("knowledge.picking", "_CHOOSE_WORDS", g__CHOOSE_WORDS, s__CHOOSE_WORDS);
    pyrt::xrefs::reg_access("knowledge.picking", "_CHOOSING", g__CHOOSING, s__CHOOSING);
    pyrt::xrefs::reg_access("knowledge.picking", "_FISH", g__FISH, s__FISH);
    pyrt::xrefs::reg_access("knowledge.picking", "_FLOUR", g__FLOUR, s__FLOUR);
    pyrt::xrefs::reg_access("knowledge.picking", "_LAMB", g__LAMB, s__LAMB);
    pyrt::xrefs::reg_access("knowledge.picking", "_MUSHROOM", g__MUSHROOM, s__MUSHROOM);
    pyrt::xrefs::reg_access("knowledge.picking", "_NOT_A_CHOICE", g__NOT_A_CHOICE, s__NOT_A_CHOICE);
    pyrt::xrefs::reg_access("knowledge.picking", "_ONION", g__ONION, s__ONION);
    pyrt::xrefs::reg_access("knowledge.picking", "_POTATO", g__POTATO, s__POTATO);
    pyrt::xrefs::reg_access("knowledge.picking", "_RICE", g__RICE, s__RICE);
    pyrt::xrefs::reg_access("knowledge.picking", "_TOMATO", g__TOMATO, s__TOMATO);
}
