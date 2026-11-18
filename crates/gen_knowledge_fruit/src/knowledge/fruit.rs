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
thread_local! { static G_CITRUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CITRUS_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRUIT_METHODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROWN_FROM_A_PIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUICE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUICING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KERNEL_FRUIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEED_TO_PEEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PIPS_AND_STONES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLANT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEED_SOURCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOP_SEED_VEG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SKIN_IN_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STONE_FRUIT_PIPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TENDER_FRUIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.fruit: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CITRUS() -> Py { ensure_init(); G_CITRUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CITRUS(v: Py) { ensure_init(); G_CITRUS.with(|c| *c.borrow_mut() = v); }
pub fn g_CITRUS_NAMES() -> Py { ensure_init(); G_CITRUS_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CITRUS_NAMES(v: Py) { ensure_init(); G_CITRUS_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_WORDS() -> Py { ensure_init(); G_EAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_WORDS(v: Py) { ensure_init(); G_EAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FRUIT_METHODS() -> Py { ensure_init(); G_FRUIT_METHODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRUIT_METHODS(v: Py) { ensure_init(); G_FRUIT_METHODS.with(|c| *c.borrow_mut() = v); }
pub fn g_GROWN_FROM_A_PIP() -> Py { ensure_init(); G_GROWN_FROM_A_PIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROWN_FROM_A_PIP(v: Py) { ensure_init(); G_GROWN_FROM_A_PIP.with(|c| *c.borrow_mut() = v); }
pub fn g_JUICE_ASKS() -> Py { ensure_init(); G_JUICE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUICE_ASKS(v: Py) { ensure_init(); G_JUICE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUICE_WORDS() -> Py { ensure_init(); G_JUICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUICE_WORDS(v: Py) { ensure_init(); G_JUICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUICING() -> Py { ensure_init(); G_JUICING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUICING(v: Py) { ensure_init(); G_JUICING.with(|c| *c.borrow_mut() = v); }
pub fn g_KERNEL_FRUIT() -> Py { ensure_init(); G_KERNEL_FRUIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KERNEL_FRUIT(v: Py) { ensure_init(); G_KERNEL_FRUIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NEED_TO_PEEL() -> Py { ensure_init(); G_NEED_TO_PEEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEED_TO_PEEL(v: Py) { ensure_init(); G_NEED_TO_PEEL.with(|c| *c.borrow_mut() = v); }
pub fn g_PIPS_AND_STONES() -> Py { ensure_init(); G_PIPS_AND_STONES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PIPS_AND_STONES(v: Py) { ensure_init(); G_PIPS_AND_STONES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLANT_WORDS() -> Py { ensure_init(); G_PLANT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLANT_WORDS(v: Py) { ensure_init(); G_PLANT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PREP_WORDS() -> Py { ensure_init(); G_PREP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREP_WORDS(v: Py) { ensure_init(); G_PREP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SEED_SOURCE() -> Py { ensure_init(); G_SEED_SOURCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEED_SOURCE(v: Py) { ensure_init(); G_SEED_SOURCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOP_SEED_VEG() -> Py { ensure_init(); G_SHOP_SEED_VEG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOP_SEED_VEG(v: Py) { ensure_init(); G_SHOP_SEED_VEG.with(|c| *c.borrow_mut() = v); }
pub fn g_SKIN_IN_A_DISH() -> Py { ensure_init(); G_SKIN_IN_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SKIN_IN_A_DISH(v: Py) { ensure_init(); G_SKIN_IN_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_STONE_FRUIT_PIPS() -> Py { ensure_init(); G_STONE_FRUIT_PIPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STONE_FRUIT_PIPS(v: Py) { ensure_init(); G_STONE_FRUIT_PIPS.with(|c| *c.borrow_mut() = v); }
pub fn g_TENDER_FRUIT() -> Py { ensure_init(); G_TENDER_FRUIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TENDER_FRUIT(v: Py) { ensure_init(); G_TENDER_FRUIT.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Fruit: getting one out of its skin, one method per fruit.\n\nThe answers lane owns this domain. Getting the flesh out of a kiwi, the seeds out of a\npomegranate and the stone out of a peach are three different jobs, so the entries are keyed by\nthe fruit the line names rather than by the verb: \"how do i deseed a pomegranate\", \"how do i cut a\npomegranate\" and \"how do i get the seeds out of a pomegranate\" are one question.\n\nWhy its own domain: these are neither dishes nor cooking methods, and on main they had no home at\nall. Nine of them were shrugs, and the two that found an entry found the wrong one - \"how do i cut\na pineapple\" got the pineapple-on-pizza opinion and \"how do i cut an avocado\" got the\nkeeping-a-cut-avocado card. cuisine's table is already at the size ceiling, so this is a file of\nits own rather than another append there.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_FRUIT_METHODS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("pomegranate"), py_str("pomegranates"), py_str("pomegranate seeds")]), py_str("Cut the crown off and score the skin down the ridges, then pull it apart over a big bowl of water: the seeds sink, the white pith floats, and nothing sprays your shirt. For a whole one in a hurry, hold a half cut-side down over a bowl and whack the back with a wooden spoon and the seeds drop out in a minute.")]), py_tuple(vec![py_tuple(vec![py_str("mango"), py_str("mangoes"), py_str("mangos")]), py_str("Slice a cheek off each side of the flat stone, score the flesh in a grid without cutting through the skin, then push the skin inside out and slice the cubes off. The stone and the flesh round it are the cook's, and a peeler takes the skin off the two cheeks if you would rather have slices.")]), py_tuple(vec![py_tuple(vec![py_str("pineapple"), py_str("pineapples")]), py_str("Cut the top and the base off flat so it stands up, then slice the skin off in strips from top to bottom, following the curve, and take the eyes out with the tip of the knife. Quarter it lengthways and cut the woody core out of each quarter.")]), py_tuple(vec![py_tuple(vec![py_str("kiwi"), py_str("kiwis"), py_str("kiwifruit")]), py_str("Top and tail it, then either run a vegetable peeler round it or cut it in half and scoop the flesh out with a teaspoon, which goes round the skin in one movement. A kiwi that is soft enough to eat peels with a spoon and no knife at all.")]), py_tuple(vec![py_tuple(vec![py_str("peach"), py_str("peaches"), py_str("nectarine"), py_str("nectarines"), py_str("plum"), py_str("plums"), py_str("apricot"), py_str("apricots"), py_str("cherry"), py_str("cherries")]), py_str("Run the knife round the seam, all the way round rather than through the stone, then twist the halves apart. A ripe peach or plum gives its stone up with a thumb; a clingstone that will not, cut it into quarters round the stone and flick it out. A cherry wants a bottle neck or a paperclip pushed into the dimple.")]), py_tuple(vec![py_tuple(vec![py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes")]), py_str("Roll it hard on the counter with your palm first to break the cells inside, then cut it across the middle rather than end to end and squeeze with the cut face out, so the pips stay in your hand. A fork twisted in the half gets more juice out than a squeeze does, and half a minute in a microwave gets more again.")]), py_tuple(vec![py_tuple(vec![py_str("coconut"), py_str("coconuts")]), py_str("Three soft eyes sit at one end: push a screwdriver or a clean nail through the softest, drain the water into a glass, then hit round the equator with the back of a heavy knife or a hammer, turning it as you go, until it cracks in two. The flesh comes out with a knife run between it and the shell.")]), py_tuple(vec![py_tuple(vec![py_str("avocado"), py_str("avocados")]), py_str("Run the knife round the stone lengthways, twist the two halves apart, then whack the stone with the blade and twist it out, or push a spoon in behind it. Slip a spoon between the skin and the flesh to peel a half, and cut it only when you are about to eat it.")]), py_tuple(vec![py_tuple(vec![py_str("apple"), py_str("apples")]), py_str("Quarter it round the core with a knife and cut the core out of each quarter at an angle, or push an apple corer straight down over the stalk. A peeled or cut apple browns in minutes, so toss it in a little lemon juice if it has to wait.")]), py_tuple(vec![py_tuple(vec![py_str("melon"), py_str("melons"), py_str("watermelon"), py_str("watermelons"), py_str("cantaloupe"), py_str("honeydew")]), py_str("Cut it in half through the middle rather than end to end, scoop the seeds out with a spoon, then stand each half on its flat face and slice the skin off downwards in strips. A melon baller on a halved one is quicker than cubing, and watermelon gives up its seeds the same way, with a fork to flick out the stragglers.")]), py_tuple(vec![py_tuple(vec![py_str("orange"), py_str("oranges"), py_str("grapefruit"), py_str("grapefruits"), py_str("tangerine"), py_str("tangerines"), py_str("satsuma"), py_str("satsumas"), py_str("clementine"), py_str("clementines")]), py_str("Cut the top and the bottom off so it stands, then slice the peel and pith off in strips down the sides, following the curve, until no white is left. Cut the segments out between the membranes over a bowl to catch the juice, which beats peeling a grapefruit with your fingers and still finding pith.")])]));
    s_JUICE_WORDS(py_tuple(vec![py_str("juice"), py_str("juiced"), py_str("juicing"), py_str("squeeze"), py_str("squeezing"), py_str("squeeze it")]));
    fn __data1() -> Py { py_tuple(vec![py_str("how do i juice"), py_str("how do you juice"), py_str("how to juice"), py_str("how can i juice"), py_str("how would i juice"), py_str("how should i juice"), py_str("how do i squeeze"), py_str("how do you squeeze"), py_str("how to squeeze"), py_str("how can i squeeze"), py_str("how do i get the juice"), py_str("how do you get the juice"), py_str("how to get the juice"), py_str("get the juice out"), py_str("get more juice"), py_str("get more juice out"), py_str("get the most juice"), py_str("get all the juice"), py_str("get juice out of"), py_str("best way to juice"), py_str("easiest way to juice"), py_str("way to juice"), py_str("way to squeeze"), py_str("juice a"), py_str("juice an"), py_str("juice the"), py_str("juice some"), py_str("juice it"), py_str("juice them"), py_str("juicing a"), py_str("juicing the"), py_str("squeeze a"), py_str("squeeze an"), py_str("squeeze the"), py_str("squeeze it"), py_str("squeeze them"), py_str("squeezing a"), py_str("squeezing an"), py_str("squeezing the"), py_str("no juicer"), py_str("without a juicer"), py_str("by hand")]) }
    s_JUICE_ASKS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes"), py_str("orange"), py_str("oranges"), py_str("grapefruit"), py_str("grapefruits"), py_str("tangerine"), py_str("tangerines"), py_str("satsuma"), py_str("satsumas"), py_str("clementine"), py_str("clementines")]) }
    s_CITRUS(__data2());
    s_JUICING(py_str("Roll it hard on the counter with your palm first to break the cells inside, then cut it across the middle rather than end to end and squeeze with the cut face out, so the pips stay in your hand. A fork twisted in the half gets more out than a squeeze does, and half a minute in a microwave gets more again. Sieve it if the pips and pith would spoil what it goes in."));
    fn __data3() -> Py { py_tuple(vec![py_str("cut"), py_str("cutting"), py_str("peel"), py_str("peeling"), py_str("core"), py_str("coring"), py_str("segment"), py_str("segments"), py_str("supreme"), py_str("stone"), py_str("stoned"), py_str("destone"), py_str("de-stone"), py_str("deseed"), py_str("de-seed"), py_str("deskin"), py_str("open"), py_str("opening"), py_str("prep"), py_str("prepping"), py_str("prepare"), py_str("preparing"), py_str("slice"), py_str("slicing"), py_str("chop"), py_str("chunks"), py_str("wedges"), py_str("top and tail"), py_str("get the seeds out"), py_str("take the stone out"), py_str("get the stone out")]) }
    s_PREP_WORDS(__data3());
    s_NEED_TO_PEEL(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|should|must|need|needs|have|has|ought|would|will)\\s+(?:i|we|you|they|it)\\s+(?:really\\s+|actually\\s+|even\\s+)?(?:need\\s+to\\s+|have\\s+to\\s+|has\\s+to\\s+)?(?:be\\s+)?peel(?:ed)?\\b|\\bpeel\\s+(?:the\\s+|my\\s+|these\\s+|those\\s+)?(?:apples?|pears?|plums?|peaches|kiwis?|grapes?)\\s+(?:before|first|or not)\\b|\\bis it necessary to peel\\b|\\bnecessary to peel\\b|\\bworth peeling\\b|\\bmust be peeled\\b|\\bneeds peeling\\b|\\bneed peeling\\b|\\b(?:is|is it|isn't it|would it be|is it not)\\s+(?:better|best|worth|fine)\\s+to\\s+peel\\b|\\bbetter to peel\\b|\\bbest to peel\\b|\\bstill worth peeling\\b|\\b(?:do|does|would|should)\\s+(?:you|people|most people|anyone|everyone)\\s+(?:usually\\s+|always\\s+|normally\\s+|even\\s+|bother to\\s+)?peel\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SKIN_IN_A_DISH(stdlib_call("re", "compile", &[py_str("\\b(?:crumble|cobblers?|pies?|pie filling|tarts?|cake|cakes|muffins?|loaf|loaves|bread|brownies?|porridge|baked|baking|roast|roasted|stewed|stew|soup)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("pip"), py_str("pips"), py_str("pippin"), py_str("pippins"), py_str("seed"), py_str("seeds"), py_str("stone"), py_str("stones"), py_str("kernel"), py_str("kernels"), py_str("nut"), py_str("pit"), py_str("pits")]) }
    s_PIPS_AND_STONES(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("plum"), py_str("plums"), py_str("peach"), py_str("peaches"), py_str("nectarine"), py_str("nectarines"), py_str("cherry"), py_str("cherries"), py_str("apricot"), py_str("apricots"), py_str("damson"), py_str("damsons"), py_str("avocado"), py_str("avocados"), py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes"), py_str("orange"), py_str("oranges"), py_str("clementine"), py_str("clementines"), py_str("satsuma"), py_str("satsumas"), py_str("tangerine"), py_str("tangerines"), py_str("grapefruit"), py_str("grapefruits"), py_str("fig"), py_str("figs"), py_str("olive"), py_str("olives"), py_str("pomegranate"), py_str("pomegranates"), py_str("mango"), py_str("mangoes"), py_str("apple tree"), py_str("fruit tree"), py_str("cherry tree"), py_str("peach tree"), py_str("lemon tree"), py_str("orange tree"), py_str("plum tree"), py_str("pear tree")]) }
    s_GROWN_FROM_A_PIP(__data5());
    s_SHOP_SEED_VEG(py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("cherry tomato"), py_str("cherry tomatoes"), py_str("beefsteak tomato")]));
    fn __data6() -> Py { py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("cherry"), py_str("cherries"), py_str("plum"), py_str("plums"), py_str("peach"), py_str("peaches"), py_str("apricot"), py_str("apricots"), py_str("nectarine"), py_str("nectarines"), py_str("damson"), py_str("damsons"), py_str("bitter almond"), py_str("almond"), py_str("almonds"), py_str("fruit pip"), py_str("fruit pips"), py_str("apple pip"), py_str("apple pips"), py_str("apple seed"), py_str("apple seeds")]) }
    s_STONE_FRUIT_PIPS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("shop"), py_str("shops"), py_str("supermarket"), py_str("supermarkets"), py_str("store"), py_str("stores"), py_str("store bought"), py_str("store-bought"), py_str("from a tomato"), py_str("from the tomato"), py_str("from tomatoes"), py_str("in a tomato"), py_str("in the tomato"), py_str("in tomatoes"), py_str("seeds in"), py_str("seeds from"), py_str("seed from"), py_str("seed in"), py_str("saved seed"), py_str("save seed"), py_str("save the seed"), py_str("saving seed"), py_str("save"), py_str("saves"), py_str("saving"), py_str("saved"), py_str("harvest"), py_str("harvesting"), py_str("harvested"), py_str("keep the seed"), py_str("keeping seed"), py_str("kept seed"), py_str("collected"), py_str("collect the seed"), py_str("left over"), py_str("leftover"), py_str("last year"), py_str("next year"), py_str("from my own"), py_str("from our own")]) }
    s_SEED_SOURCE(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("plant"), py_str("plants"), py_str("planted"), py_str("planting"), py_str("grow"), py_str("grows"), py_str("growing"), py_str("grew"), py_str("sow"), py_str("sows"), py_str("sowing"), py_str("sprout"), py_str("sprouts"), py_str("sprouting"), py_str("germinate"), py_str("germinating"), py_str("propagate"), py_str("seedling"), py_str("seedlings"), py_str("sapling"), py_str("saplings"), py_str("from seed"), py_str("from a seed"), py_str("from a pip"), py_str("from pips"), py_str("from a stone"), py_str("from stones"), py_str("from a kernel"), py_str("will it grow"), py_str("can it grow"), py_str("save"), py_str("saves"), py_str("saving"), py_str("saved"), py_str("keep the seed"), py_str("keeping seed"), py_str("harvest"), py_str("harvesting"), py_str("collect the seed"), py_str("collecting seed"), py_str("store seed"), py_str("store the seed"), py_str("next year")]) }
    s_PLANT_WORDS(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("eat"), py_str("eats"), py_str("eating"), py_str("eaten"), py_str("swallow"), py_str("swallowed"), py_str("swallowing"), py_str("chew"), py_str("chewed"), py_str("chewing"), py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("dangerous"), py_str("harmful"), py_str("hurt me"), py_str("bad for me"), py_str("deadly"), py_str("cyanide"), py_str("safe to"), py_str("safe"), py_str("should i eat"), py_str("can i eat"), py_str("is it ok to eat"), py_str("is it okay to eat"), py_str("kill"), py_str("kills"), py_str("killed"), py_str("die"), py_str("died"), py_str("dying"), py_str("fatal"), py_str("how many would"), py_str("would kill"), py_str("would die"), py_str("make me ill"), py_str("make me sick"), py_str("how many are safe"), py_str("how many kernels"), py_str("how many pips"), py_str("how many seeds")]) }
    s_EAT_WORDS(__data9());
    s_TENDER_FRUIT(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("fig"), py_str("figs")]), py_str("A fig seed does come up in two or three weeks in warmth, but almost nobody grows figs from seed: the seedling is a lottery, it takes four to six years to fruit, and it may never fruit at all in a cool climate. Take a cutting instead - a 15cm piece of last year's wood in late winter, pushed into free-draining compost, roots in about six weeks. Figs also fruit best with their roots fenced in, which is why they do so well in a pot against a sunny wall.")]), py_tuple(vec![py_tuple(vec![py_str("pomegranate"), py_str("pomegranates")]), py_str("A pomegranate seed does grow, and it comes fairly true to the fruit it came from, but it is a Mediterranean tree: five years or more to the first fruit, a hot sheltered wall, and in a cool summer the fruit can stay hard and sour. Sow it fresh in spring at about 21C. Cuttings are quicker and are how the named varieties are sold, and a bought plant fruits in two or three years.")]), py_tuple(vec![py_tuple(vec![py_str("mango"), py_str("mangoes"), py_str("mangos")]), py_str("A mango stone will sprout on a windowsill - take the stone out of its hairy husk, stand it on its edge in damp compost with the top third showing, and it comes up in two or three weeks - but it is a tropical tree. It wants 25C and more, it grows into something far too large for a room, and it will not fruit in a temperate house. It is a good plant to grow for the leaves, not for fruit.")])]));
    fn __data10() -> Py { py_tuple(vec![py_str("grapefruit"), py_str("grapefruits"), py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes"), py_str("orange"), py_str("oranges"), py_str("clementine"), py_str("clementines"), py_str("satsuma"), py_str("satsumas"), py_str("tangerine"), py_str("tangerines"), py_str("citrus")]) }
    s_CITRUS_NAMES(__data10());
    s_KERNEL_FRUIT(py_tuple(vec![py_str("apricot kernel"), py_str("apricot kernels"), py_str("bitter kernel"), py_str("bitter kernels"), py_str("bitter apricot"), py_str("bitter almonds"), py_str("bitter almond"), py_str("apricot seeds"), py_str("apricot seed"), py_str("kernels"), py_str("kernel")]));
    fn __data11() -> Py { py_tuple(vec![py_str("pizza"), py_str("pizzas"), py_str("hawaiian"), py_str("curry"), py_str("chutney"), py_str("crumble"), py_str("tart"), py_str("cake"), py_str("shell went in"), py_str("shell in"), py_str("grow"), py_str("growing"), py_str("tree"), py_str("trees"), py_str("plant"), py_str("plants"), py_str("buy"), py_str("buying"), py_str("price"), py_str("cost"), py_str("shop"), py_str("supermarket"), py_str("ripe"), py_str("ripen"), py_str("ripening"), py_str("smoothie"), py_str("juice bar"), py_str("how many calories"), py_str("healthy"), py_str("frozen"), py_str("tomato"), py_str("tomatoes"), py_str("how do i store"), py_str("how do i keep"), py_str("how do you store"), py_str("how do you keep"), py_str("store a cut"), py_str("keep a cut"), py_str("keeping it"), py_str("store it"), py_str("keep it"), py_str("goes brown"), py_str("going brown"), py_str("stays green"), py_str("stay green"), py_str("hard as a stone"), py_str("hard as stone"), py_str("hard as stones"), py_str("hard as a rock"), py_str("hard as rocks"), py_str("rock hard"), py_str("rock-hard"), py_str("still hard"), py_str("so hard"), py_str("too hard"), py_str("hard as"), py_str("unripe"), py_str("not ripe"), py_str("nowhere near ripe"), py_str("sour and hard"), py_str("firm as a stone"), py_str("like a stone")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("kiwi"), py_str("kiwis"), py_str("kiwifruit"), py_str("peach"), py_str("peaches"), py_str("plum"), py_str("plums"), py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("grape"), py_str("grapes")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("eat"), py_str("eating"), py_str("edible"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("have to"), py_str("need to"), py_str("should i"), py_str("can i"), py_str("do i"), py_str("with the skin"), py_str("leave the skin"), py_str("unpeeled")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("apple sauce"), py_str("applesauce"), py_str("cranberry sauce"), py_str("gooseberry sauce"), py_str("how do i peel"), py_str("how to peel"), py_str("how do i cut"), py_str("how to cut"), py_str("how do i prepare"), py_str("freeze"), py_str("freezer"), py_str("freezing"), py_str("frozen"), py_str("defrost"), py_str("defrosting"), py_str("cut a"), py_str("cut an"), py_str("cut the"), py_str("slice"), py_str("slicing"), py_str("deseed"), py_str("core"), py_str("juicing"), py_str("juice"), py_str("calories"), py_str("how many"), py_str("ripe"), py_str("ripen"), py_str("buy"), py_str("hard"), py_str("hard as"), py_str("still hard"), py_str("rock hard"), py_str("unripe"), py_str("not ripe"), py_str("sour"), py_str("tasteless"), py_str("no flavour"), py_str("no flavor"), py_str("soften"), py_str("softer"), py_str("price"), py_str("grow"), py_str("growing"), py_str("tree"), py_str("allergy"), py_str("allergic"), py_str("reaction"), py_str("itch"), py_str("itches"), py_str("itching"), py_str("itchy"), py_str("tingle"), py_str("tingles"), py_str("tingling"), py_str("rash"), py_str("rashes"), py_str("hives"), py_str("raw"), py_str("sore mouth"), py_str("mouth is sore"), py_str("oven"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("do i bake"), py_str("to bake"), py_str("bake them"), py_str("bake it"), py_str("bake a"), py_str("bake an"), py_str("bake the"), py_str("bake for"), py_str("how long to bake"), py_str("bake at"), py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("grill"), py_str("grilled"), py_str("poach"), py_str("poached"), py_str("stew"), py_str("stewed"), py_str("microwave"), py_str("how long"), py_str("minutes"), py_str("temperature"), py_str("gas mark"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("store"), py_str("stores"), py_str("storing"), py_str("storage"), py_str("fresh"), py_str("fresher"), py_str("freshest"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("fridge"), py_str("counter"), py_str("going off"), py_str("gone off"), py_str("bruise"), py_str("bruised"), py_str("bruises"), py_str("shrivel"), py_str("shrivelled"), py_str("wrinkled"), py_str("mouldy"), py_str("mould"), py_str("moldy"), py_str("mold"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("puppy"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("toddler"), py_str("baby"), py_str("weaning"), py_str("going brown"), py_str("goes brown"), py_str("go brown"), py_str("gone brown"), py_str("turn brown"), py_str("turns brown"), py_str("turning brown"), py_str("brown quickly"), py_str("stop it going"), py_str("stop them going"), py_str("from browning"), py_str("from going brown"), py_str("not go brown"), py_str("rabbit"), py_str("rabbits"), py_str("bunny"), py_str("bunnies"), py_str("hamster"), py_str("hamsters"), py_str("guinea pig"), py_str("guinea pigs"), py_str("guineapig"), py_str("guineapigs"), py_str("budgie"), py_str("budgies"), py_str("parrot"), py_str("parrots"), py_str("tortoise"), py_str("tortoises"), py_str("horse"), py_str("horses"), py_str("pony"), py_str("ponies"), py_str("chinchilla"), py_str("chinchillas"), py_str("gerbil"), py_str("gerbils"), py_str("ferret"), py_str("ferrets"), py_str("apple id"), py_str("apple account"), py_str("apple watch"), py_str("apple pay"), py_str("apple music"), py_str("apple tv"), py_str("apple store"), py_str("applecare"), py_str("apple care"), py_str("apple support"), py_str("apple pencil"), py_str("apple inc"), py_str("icloud"), py_str("iphone"), py_str("ipad"), py_str("macbook"), py_str("airpods"), py_str("app store"), py_str("apple id password"), py_str("apple id locked"), py_str("pick a"), py_str("pick the"), py_str("how do i pick"), py_str("how to pick"), py_str("which ones"), py_str("which one"), py_str("which apple"), py_str("which apples"), py_str("choose"), py_str("choosing"), py_str("best apple"), py_str("best apples"), py_str("good apple"), py_str("good apples"), py_str("ripe one"), py_str("ripest"), py_str("how long"), py_str("how many minutes"), py_str("how long in"), py_str("how hot"), py_str("gas mark"), py_str("what temperature"), py_str("temperature"), py_str("degrees"), py_str("180c"), py_str("200c"), py_str("mins")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("banana"), py_str("bananas"), py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("peach"), py_str("peaches"), py_str("plum"), py_str("plums"), py_str("avocado"), py_str("avocados"), py_str("mango"), py_str("mangoes"), py_str("fruit"), py_str("nectarine"), py_str("nectarines"), py_str("tomato"), py_str("tomatoes")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("bruised"), py_str("bruise"), py_str("bruises"), py_str("brown spot"), py_str("brown spots"), py_str("black spot"), py_str("black spots"), py_str("brown patch"), py_str("brown patches"), py_str("brown inside"), py_str("gone brown"), py_str("went brown"), py_str("soft spot"), py_str("soft spots"), py_str("knocked"), py_str("dropped"), py_str("dented")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("my leg"), py_str("my arm"), py_str("my knee"), py_str("my shin"), py_str("my thigh"), py_str("my back"), py_str("my ribs"), py_str("my hand"), py_str("my foot"), py_str("my ankle"), py_str("my wrist"), py_str("my shoulder"), py_str("my head"), py_str("i fell"), py_str("i bumped"), py_str("fell over"), py_str("bumped into"), py_str("after a fall"), py_str("hit my"), py_str("icing"), py_str("ice it"), py_str("swollen"), py_str("swelling"), py_str("child"), py_str("toddler"), py_str("baby"), py_str("bleeding"), py_str("nosebleed"), py_str("growing"), py_str("grow"), py_str("tree"), py_str("plant"), py_str("tree is"), py_str("dropping"), py_str("dropping fruit"), py_str("falling off"), py_str("falls off"), py_str("off the tree"), py_str("on the tree"), py_str("june drop"), py_str("why do i bruise"), py_str("bruise easily"), py_str("bruises easily"), py_str("recipe"), py_str("bake"), py_str("bread"), py_str("muffin"), py_str("loaf")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("how do i stone"), py_str("how to stone"), py_str("destone"), py_str("de-stone"), py_str("get the stone out"), py_str("take the stone out"), py_str("remove the stone"), py_str("without the pips"), py_str("pips out"), py_str("seeds out"), py_str("deseed"), py_str("de-seed"), py_str("recipe"), py_str("recipes"), py_str("cake"), py_str("cakes"), py_str("chutney"), py_str("jam"), py_str("pie"), py_str("crumble"), py_str("smoothie"), py_str("juice"), py_str("juicing"), py_str("eat"), py_str("eating"), py_str("eaten"), py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("cyanide"), py_str("kill"), py_str("die"), py_str("fatal"), py_str("how many would"), py_str("harm"), py_str("harmful"), py_str("safe"), py_str("dangerous"), py_str("ill"), py_str("sick"), py_str("hospital"), py_str("hard"), py_str("hard as"), py_str("as hard as"), py_str("rock hard"), py_str("still hard"), py_str("unripe"), py_str("not ripe"), py_str("ripe"), py_str("ripen"), py_str("ripening"), py_str("soft"), py_str("soften"), py_str("softer"), py_str("squishy"), py_str("tasteless"), py_str("no taste"), py_str("sour"), py_str("bland"), py_str("watery"), py_str("healthy"), py_str("calories"), py_str("price"), py_str("cost"), py_str("how much does"), py_str("buy"), py_str("buying"), py_str("sainsbury"), py_str("tesco"), py_str("asda"), py_str("aldi"), py_str("lidl"), py_str("waitrose"), py_str("support"), py_str("supporting"), py_str("stake"), py_str("stakes"), py_str("prune"), py_str("pruning"), py_str("blight"), py_str("yellow leaves"), py_str("water them"), py_str("watering"), py_str("feed"), py_str("feeding"), py_str("fertiliser"), py_str("personal independence payment"), py_str("benefit"), py_str("benefits"), py_str("claim"), py_str("dla"), py_str("esa"), py_str("universal credit")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("plant"), py_str("plants"), py_str("planted"), py_str("planting"), py_str("grow"), py_str("grows"), py_str("growing"), py_str("grew"), py_str("sow"), py_str("sowing"), py_str("sprout"), py_str("sprouting"), py_str("seedling"), py_str("sapling"), py_str("from seed"), py_str("from a pip"), py_str("from a stone"), py_str("tree"), py_str("trees"), py_str("garden"), py_str("allotment"), py_str("compost"), py_str("pot them"), py_str("pot it"), py_str("germinate"), py_str("how do i stone"), py_str("how to stone"), py_str("destone"), py_str("de-stone"), py_str("get the stone out"), py_str("take the stone out"), py_str("remove the stone"), py_str("deseed"), py_str("de-seed"), py_str("recipe"), py_str("cake"), py_str("chutney"), py_str("jam"), py_str("pie"), py_str("crumble"), py_str("smoothie"), py_str("personal independence payment"), py_str("benefit"), py_str("benefits"), py_str("claim"), py_str("dla"), py_str("esa"), py_str("universal credit"), py_str("itch"), py_str("itches"), py_str("itching"), py_str("itchy"), py_str("tingle"), py_str("tingles"), py_str("tingling"), py_str("rash"), py_str("rashes"), py_str("hives"), py_str("sore mouth")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("juice-yield")); __args.push(py_tuple(vec![py_tuple(vec![py_str("much juice"), py_str("juice in"), py_str("juice from"), py_str("how many lemons"), py_str("how many limes"), py_str("how many oranges"), py_str("how many grapefruit"), py_str("cup of juice"), py_str("tablespoons of juice"), py_str("ml of juice")]), op_add(&g_CITRUS(), &py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("cup"), py_str("cups"), py_str("tablespoon"), py_str("tablespoons"), py_str("ml")]))])); __args.push(py_str("A lemon gives about 3 tablespoons of juice, 45ml, and a lime about 2, 30ml. A big orange gives about half a cup (120ml) and a small one a third of a cup; a grapefruit gives about two thirds of a cup. So a cup of lemon juice is four or five lemons, and rolling them hard on the counter before squeezing gets a little more out of each.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("get the juice out"), py_str("get the juice"), py_str("get more juice"), py_str("get the most juice"), py_str("squeeze"), py_str("squeezing"), py_str("juice a"), py_str("juice an"), py_str("juice the"), py_str("how do i juice")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("prepare-fruit")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FRUIT_METHODS()) { let __unpk786 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk786[0].clone(); let mut _method: Py = __unpk786[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), op_add(&g_PREP_WORDS(), &g_JUICE_WORDS())])); __args.push(py_func("_fruit_prep", |args: &[Py]| _fruit_prep(args))); __kws.extend(vec![("avoid".to_string(), __data11()), ("ask".to_string(), op_add(&g_PREP_WORDS(), &g_JUICE_WORDS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fruit-skin-edible")); __args.push(py_tuple(vec![__data12(), py_tuple(vec![py_str("skin"), py_str("skins"), py_str("peel")]), __data13()])); __args.push(py_func("_skin_reply", |args: &[Py]| _skin_reply(args))); __kws.extend(vec![("avoid".to_string(), __data14()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bruised-fruit")); __args.push(py_tuple(vec![__data15(), __data16()])); __args.push(py_func("_bruise_reply", |args: &[Py]| _bruise_reply(args))); __kws.extend(vec![("avoid".to_string(), __data17()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("grow-from-a-pip")); __args.push(py_tuple(vec![op_add(&g_GROWN_FROM_A_PIP(), &g_SHOP_SEED_VEG()), g_PIPS_AND_STONES(), g_PLANT_WORDS()])); __args.push(py_func("_growing_from_a_pip", |args: &[Py]| _growing_from_a_pip(args))); __kws.extend(vec![("avoid".to_string(), __data18()), ("ask".to_string(), g_PIPS_AND_STONES())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pips-to-eat")); __args.push(py_tuple(vec![g_STONE_FRUIT_PIPS(), g_PIPS_AND_STONES(), g_EAT_WORDS()])); __args.push(py_func("_a_pip_to_eat", |args: &[Py]| _a_pip_to_eat(args))); __kws.extend(vec![("avoid".to_string(), __data19()), ("ask".to_string(), op_add(&g_PIPS_AND_STONES(), &g_EAT_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _said(args: &[Py]) -> Py {
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
    let _ = py_str("The line, with the punctuation taken out, so a word can be looked for on its own.\n\n    A substring is not a word: \"plum\" sits inside \"plumber\", \"lime\" inside \"limestone\" and \"cherry\"\n    inside \"cherry red\", and a plumber asking how to cut a pipe should not get a peach.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    Py::None
}
pub fn _holds(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))));
    Py::None
}
pub fn _fruit_prep(args: &[Py]) -> Py {
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
    let mut method: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The way to cut, peel, juice or stone the fruit the line names, one method per fruit.\n\n    A juicing line is not a knife line: the citrus rows have a way to get the juice out, and the\n    rest stand down, because \"quarter it round the core\" is not how a pineapple is juiced.\n    ");
    line = _said(&[{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_JUICE_WORDS()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_JUICE_ASKS()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CITRUS()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { g_JUICING() } else { Py::None };
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PREP_WORDS()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_FRUIT_METHODS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    method = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return method.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _skin_reply(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut in_a_dish: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The skin is food, and the answer names the fruit the line names.\n\n    A kiwi is the one people ask about and the one that needs the scrubbing, because of the fuzz\n    and because the skin holds more fibre and vitamin C than the flesh does. Peach, plum, apple,\n    pear and grape need only washing.\n    ");
    said = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    lead = if py_truthy(&mcall(&g_NEED_TO_PEEL(), "search", &[said.clone()])) { py_str("No need to peel - ") } else { py_str("Yes - ") };
    in_a_dish = call_builtin("bool", &[mcall(&g_SKIN_IN_A_DISH(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    dish = if py_truthy(&in_a_dish.clone()) { py_str(" For a crumble, a pie or a cake, leave it on: the skin softens as it bakes and holds the fruit together, and peel only if you want the filling perfectly smooth.") } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("kiwi"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("kiwifruit"))) } }) {
    body = py_str("kiwi skin is edible, and it holds more fibre and vitamin C than the green flesh, so scrub it under the tap and eat it if the fuzz does not put you off: the skin is thin, and rubbing it under water takes most of the hairiness off. A ripe kiwi eaten skin and all tastes the same, only tarter near the skin.");
    if py_truthy(&mcall(&lead.clone(), "startswith", &[py_str("No need")])) {
    return op_add(&lead.clone(), &body.clone());
    }
    return op_add(&py_str("Yes - "), &body.clone());
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("grape"), py_str("grapes")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&lead.clone(), &py_str("grape skins are food - that is where the colour, the tannin and much of the fibre sit - so wash them and eat them, and only peel a grape for a small child or someone who cannot manage the skin. A white bloom on the skin is natural yeast, not spoilage, so rinse it rather than peel it."));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("peach"), py_str("peaches"), py_str("plum"), py_str("plums")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&op_add(&lead.clone(), &py_str("a peach or a plum is eaten with the skin on, and washing it is enough: the skin holds the colour and much of the fibre, and peeling only matters for a smooth chutney or for somebody who finds the fuzz unpleasant. Pour boiling water over a peach for 30 seconds and the skin slips off if you do want it off.")), &dish.clone());
    }
    return op_add(&op_add(&op_add(&lead.clone(), &py_str("apple and pear skin is the best part of it: the fibre, the colour and the quercetin that gives the skin its job are all in there, so wash the fruit and eat it as it is. Peeling loses about a third of the fibre and most of the vitamin C, and the only ones worth peeling are a waxed one if the wax bothers you, or fruit for a small child who cannot chew the skin.")), &dish.clone()), &py_str(" The one case where the answer is cook it: if raw apple or pear makes your mouth itch or tingle, heat settles that - stewed, baked or in a crumble the protein that does it is broken down, and peeled is usually fine too."));
    Py::None
}
pub fn _bruise_reply(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("A bruise on fruit is a knock, not a germ, and the fruit is still food.\n\n    The banana is the one people ask about and the one where the bruise makes it sweeter, so it\n    gets its own sentence; the rest share the oxidation explanation and the mould line.\n    ");
    said = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("banana")))) {
    return py_str("A bruised banana is fine to eat: the bruise is the flesh knocked and oxidising, not rot, and a brown-spotted banana is sweeter because the starch has turned to sugar, which is why it is the one for banana bread. Cut out a soft black patch if you do not like the look of it, and bin the fruit only if the skin has split and it smells fermented or has mould on it.");
    }
    return py_str("A bruise on fruit is a knock, not spoilage: the cell walls are broken and the flesh browns as it oxidises, which is why an apple or a peach browns where it was dropped. Cut the patch out if the texture bothers you, or eat it - the rest of the fruit is unchanged. Bin it only when the bruise is wet and sour-smelling or there is mould, because a soft bruise is the way in for rot.");
    Py::None
}
pub fn _the_fruit(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The first fruit word in the line, as one bare noun: \"a grapefruit\", \"a fig\".");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&_holds(&[line.clone(), word.clone()])) {
    return if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))) } }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() };
    }
    }
    return py_str("citrus");
    Py::None
}
pub fn _growing_from_a_pip(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a pip, stone or seed will grow, which depends on which fruit it came from.\n\n    The honest answer for an apple or a peach is that it will grow but not come true: the pip is\n    a cross, so the fruit is a surprise and the wait is years. A lemon or an orange from a\n    supermarket seed is much better odds, and an avocado stone grows easily and never fruits on a\n    windowsill. The one sentence all of them share is the way to a named variety: buy a grafted\n    tree.\n    ");
    line = _said(&[{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("avocado"), py_str("avocados")])) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("An avocado stone sprouts easily and almost never gives you avocados: push three cocktail sticks into the fat end and sit it over a glass so the base is just in the water, and roots show in four to six weeks. Pot it in free-draining compost when the shoot is 15cm, keep it in bright light and pinch the top out to make it bushy. It is a handsome houseplant that wants a bigger pot each year; fruit needs a mature tree, two of them for cross-pollination, and a warm climate.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SHOP_SEED_VEG()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SEED_SOURCE()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    lead = if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("can i"), py_str("could i"), py_str("is it possible"), py_str("will it"), py_str("would it"), py_str("does it")])) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { py_str("Yes, and it is worth doing: ") } else { py_str("Save the seed first: ") };
    return op_add(&lead.clone(), &py_str("scoop the seeds and their jelly into a glass, add a little water and leave them three or four days until a film of mould sits on top - that ferment kills the germination inhibitor in the jelly - then rinse them, dry them on paper and sow them indoors from late February at 18-21C, about 6mm deep. A supermarket tomato is often a hybrid, so the fruit may come smaller or uneven, but it grows and it fruits, and dry seed in a paper envelope still comes up three or four years later."));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CITRUS()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&_the_fruit(&[line.clone(), g_CITRUS_NAMES()]), -1, "")); __s.push_str(" seed from the fruit bowl does grow, and comes closer to the parent than an apple pip does, but it is slow: expect five to fifteen years before fruit, and a plant that wants a sunny windowsill in winter and a spot outside in summer. Sow it fresh, before the seed dries out, in moist seed compost at about 21C, and pot it on as it grows. A named variety comes as a grafted tree, which fruits in two or three years and is what most people want."); py_string(__s) };
    }
    let __iter1 = py_iter_or_raise(&g_TENDER_FRUIT());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return said.clone();
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("peach"), py_str("peaches"), py_str("nectarine"), py_str("nectarines"), py_str("plum"), py_str("plums"), py_str("cherry"), py_str("cherries"), py_str("apricot"), py_str("apricots"), py_str("damson"), py_str("damsons"), py_str("olive"), py_str("olives")])) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A stone will grow, but it needs a winter first: clean the flesh off, mix the stone with damp compost in a sealed bag and keep it in the fridge for six to ten weeks, then pot it and wait - plums and peaches come up in a couple of months after that. Expect four to eight years before fruit, and the fruit will not be the variety you ate, because a stone fruit is cross-pollinated. For fruit you can count on, buy a bare-root tree from November to March and plant it where it gets sun.");
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("apple tree"), py_str("pear tree"), py_str("fruit tree"), py_str("fruit pip"), py_str("fruit pips")])) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return py_str("An apple or pear pip does grow, but it will not give you the apple you ate: pips are crossed with whatever tree pollinated the flower, so the fruit is a lottery, often hard and sour, and it takes five to ten years to find out. If you want to try, keep the pips damp in the fridge for six to eight weeks - they need cold to break dormancy - then sow them in compost on a windowsill, where they come up in three to four weeks, and pot them on. For eating apples, buy a named variety: a bare-root tree planted between November and March fruits in three or four years and tastes of something.");
    Py::None
}
pub fn _a_pip_to_eat(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("Whether the pips in the fruit are dangerous, which is a dose question, not a yes or no.");
    line = _said(&[{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KERNEL_FRUIT()) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Careful with these ones: a bitter apricot kernel holds about forty times the amygdalin of an apple pip, and amygdalin releases cyanide when it is chewed. One or two do most adults no harm; around ten to twenty bitter kernels have caused poisoning and deaths, and a child needs far fewer, so a child should have none at all. Bitter almonds are the same thing under another name, and the 'vitamin B17' sold as a cancer cure is laetrile, which is the same cyanide story and does not treat cancer. If someone has eaten a handful, ring 999 - headache, dizziness, vomiting, a flushed face or confusion after eating them is an emergency - and ring 111 if they feel well but you are unsure. Sweet kernels from a labelled packet are much lower, but an unlabelled bag is not worth the risk.");
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bhow many\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("pip"), py_str("pips"), py_str("seed"), py_str("seeds"), py_str("stone"), py_str("stones"), py_str("kernel"), py_str("kernels")])) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:in|inside|does|has|have|are there)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    if py_truthy(&{ let __b = _holds(&[line.clone(), py_str("apple")]); if py_truthy(&__b) { __b } else { _holds(&[line.clone(), py_str("apples")]) } }) {
    return py_str("Usually five to ten, in two rows in the core: an apple has five seed chambers and each holds one or two pips. A big apple has more pips than a small one, and a pippin is simply an apple grown from a pip. None of them will grow into the apple you ate, because the tree they came from was pollinated by another one nearby.");
    }
    if py_truthy(&{ let __b = _holds(&[line.clone(), py_str("lemon")]); if py_truthy(&__b) { __b } else { { let __b = _holds(&[line.clone(), py_str("lemons")]); if py_truthy(&__b) { __b } else { { let __b = _holds(&[line.clone(), py_str("lime")]); if py_truthy(&__b) { __b } else { { let __b = _holds(&[line.clone(), py_str("limes")]); if py_truthy(&__b) { __b } else { { let __b = _holds(&[line.clone(), py_str("orange")]); if py_truthy(&__b) { __b } else { _holds(&[line.clone(), py_str("oranges")]) } } } } } } } } } }) {
    return py_str("A lemon or a lime has about eight to ten seeds, an orange about the same, and they sit in the segments rather than in a core. A citrus seed is more likely to grow than an apple pip, and it still takes a few years and a sunny windowsill to become a plant worth keeping.");
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("bird"), py_str("birds"), py_str("parrot")])) { let mut word: Py = __item.clone(); __out.push(_holds(&[line.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("The pips and stones of apples, pears, cherries, plums, peaches and apricots are worse for a dog or a cat than for you: they hold amygdalin, which releases cyanide when one is crushed and chewed, and a small animal needs much less than a person does. A couple swallowed whole usually pass straight through, but call the vet if a pet has chewed a handful, and keep the cores and stones out of the bowl you put down.");
    }
    return py_str("Fine, in the amounts anyone eats by accident: the pips and kernels of apples, pears, cherries, plums, peaches and apricots hold amygdalin, which releases a little cyanide when the pip is crushed and chewed - but you would have to chew a cupful at once to be in trouble. A pip swallowed whole comes out the other end intact, so a child who ate two or three is not an emergency; watch for a sore tummy and ring 111 if you are worried. The flesh is the part to eat, and the pips are best left in the core.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.fruit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "CITRUS" => Some(g_CITRUS()),
            "CITRUS_NAMES" => Some(g_CITRUS_NAMES()),
            "EAT_WORDS" => Some(g_EAT_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FRUIT_METHODS" => Some(g_FRUIT_METHODS()),
            "GROWN_FROM_A_PIP" => Some(g_GROWN_FROM_A_PIP()),
            "JUICE_ASKS" => Some(g_JUICE_ASKS()),
            "JUICE_WORDS" => Some(g_JUICE_WORDS()),
            "JUICING" => Some(g_JUICING()),
            "KERNEL_FRUIT" => Some(g_KERNEL_FRUIT()),
            "NEED_TO_PEEL" => Some(g_NEED_TO_PEEL()),
            "PIPS_AND_STONES" => Some(g_PIPS_AND_STONES()),
            "PLANT_WORDS" => Some(g_PLANT_WORDS()),
            "PREP_WORDS" => Some(g_PREP_WORDS()),
            "SEED_SOURCE" => Some(g_SEED_SOURCE()),
            "SHOP_SEED_VEG" => Some(g_SHOP_SEED_VEG()),
            "SKIN_IN_A_DISH" => Some(g_SKIN_IN_A_DISH()),
            "STONE_FRUIT_PIPS" => Some(g_STONE_FRUIT_PIPS()),
            "TENDER_FRUIT" => Some(g_TENDER_FRUIT()),
            "_a_pip_to_eat" => Some(py_func("_a_pip_to_eat", |args: &[Py]| _a_pip_to_eat(args))),
            "_bruise_reply" => Some(py_func("_bruise_reply", |args: &[Py]| _bruise_reply(args))),
            "_fruit_prep" => Some(py_func("_fruit_prep", |args: &[Py]| _fruit_prep(args))),
            "_growing_from_a_pip" => Some(py_func("_growing_from_a_pip", |args: &[Py]| _growing_from_a_pip(args))),
            "_holds" => Some(py_func("_holds", |args: &[Py]| _holds(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_skin_reply" => Some(py_func("_skin_reply", |args: &[Py]| _skin_reply(args))),
            "_the_fruit" => Some(py_func("_the_fruit", |args: &[Py]| _the_fruit(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "CITRUS" => s_CITRUS(v),
            "CITRUS_NAMES" => s_CITRUS_NAMES(v),
            "EAT_WORDS" => s_EAT_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "FRUIT_METHODS" => s_FRUIT_METHODS(v),
            "GROWN_FROM_A_PIP" => s_GROWN_FROM_A_PIP(v),
            "JUICE_ASKS" => s_JUICE_ASKS(v),
            "JUICE_WORDS" => s_JUICE_WORDS(v),
            "JUICING" => s_JUICING(v),
            "KERNEL_FRUIT" => s_KERNEL_FRUIT(v),
            "NEED_TO_PEEL" => s_NEED_TO_PEEL(v),
            "PIPS_AND_STONES" => s_PIPS_AND_STONES(v),
            "PLANT_WORDS" => s_PLANT_WORDS(v),
            "PREP_WORDS" => s_PREP_WORDS(v),
            "SEED_SOURCE" => s_SEED_SOURCE(v),
            "SHOP_SEED_VEG" => s_SHOP_SEED_VEG(v),
            "SKIN_IN_A_DISH" => s_SKIN_IN_A_DISH(v),
            "STONE_FRUIT_PIPS" => s_STONE_FRUIT_PIPS(v),
            "TENDER_FRUIT" => s_TENDER_FRUIT(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.fruit", module_obj);
    pyrt::xrefs::reg_fn("knowledge.fruit", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_holds", py_func("_holds", |args: &[Py]| _holds(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_fruit_prep", py_func("_fruit_prep", |args: &[Py]| _fruit_prep(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_skin_reply", py_func("_skin_reply", |args: &[Py]| _skin_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_bruise_reply", py_func("_bruise_reply", |args: &[Py]| _bruise_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_the_fruit", py_func("_the_fruit", |args: &[Py]| _the_fruit(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_growing_from_a_pip", py_func("_growing_from_a_pip", |args: &[Py]| _growing_from_a_pip(args)));
    pyrt::xrefs::reg_fn("knowledge.fruit", "_a_pip_to_eat", py_func("_a_pip_to_eat", |args: &[Py]| _a_pip_to_eat(args)));
    pyrt::xrefs::reg_access("knowledge.fruit", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.fruit", "CITRUS", g_CITRUS, s_CITRUS);
    pyrt::xrefs::reg_access("knowledge.fruit", "CITRUS_NAMES", g_CITRUS_NAMES, s_CITRUS_NAMES);
    pyrt::xrefs::reg_access("knowledge.fruit", "EAT_WORDS", g_EAT_WORDS, s_EAT_WORDS);
    pyrt::xrefs::reg_access("knowledge.fruit", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.fruit", "FRUIT_METHODS", g_FRUIT_METHODS, s_FRUIT_METHODS);
    pyrt::xrefs::reg_access("knowledge.fruit", "GROWN_FROM_A_PIP", g_GROWN_FROM_A_PIP, s_GROWN_FROM_A_PIP);
    pyrt::xrefs::reg_access("knowledge.fruit", "JUICE_ASKS", g_JUICE_ASKS, s_JUICE_ASKS);
    pyrt::xrefs::reg_access("knowledge.fruit", "JUICE_WORDS", g_JUICE_WORDS, s_JUICE_WORDS);
    pyrt::xrefs::reg_access("knowledge.fruit", "JUICING", g_JUICING, s_JUICING);
    pyrt::xrefs::reg_access("knowledge.fruit", "KERNEL_FRUIT", g_KERNEL_FRUIT, s_KERNEL_FRUIT);
    pyrt::xrefs::reg_access("knowledge.fruit", "NEED_TO_PEEL", g_NEED_TO_PEEL, s_NEED_TO_PEEL);
    pyrt::xrefs::reg_access("knowledge.fruit", "PIPS_AND_STONES", g_PIPS_AND_STONES, s_PIPS_AND_STONES);
    pyrt::xrefs::reg_access("knowledge.fruit", "PLANT_WORDS", g_PLANT_WORDS, s_PLANT_WORDS);
    pyrt::xrefs::reg_access("knowledge.fruit", "PREP_WORDS", g_PREP_WORDS, s_PREP_WORDS);
    pyrt::xrefs::reg_access("knowledge.fruit", "SEED_SOURCE", g_SEED_SOURCE, s_SEED_SOURCE);
    pyrt::xrefs::reg_access("knowledge.fruit", "SHOP_SEED_VEG", g_SHOP_SEED_VEG, s_SHOP_SEED_VEG);
    pyrt::xrefs::reg_access("knowledge.fruit", "SKIN_IN_A_DISH", g_SKIN_IN_A_DISH, s_SKIN_IN_A_DISH);
    pyrt::xrefs::reg_access("knowledge.fruit", "STONE_FRUIT_PIPS", g_STONE_FRUIT_PIPS, s_STONE_FRUIT_PIPS);
    pyrt::xrefs::reg_access("knowledge.fruit", "TENDER_FRUIT", g_TENDER_FRUIT, s_TENDER_FRUIT);
    pyrt::xrefs::reg_access("knowledge.fruit", "entry", g_entry, s_entry);
}
