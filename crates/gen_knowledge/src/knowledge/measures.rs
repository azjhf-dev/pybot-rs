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


thread_local! { static G_AMOUNT_IN_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAFFEINE_MG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_IN_PACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAKED_WITH_COFFEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COFFEE_CROWD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COFFEE_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COFFEE_RATIO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COFFEE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COOKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_NOT_COUNTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRESS_IS_A_GARMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_RECIPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONLY_MADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PORTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECIPE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AMOUNT_IN_A_DISH() -> Py { ensure_init(); G_AMOUNT_IN_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_IN_A_DISH(v: Py) { ensure_init(); G_AMOUNT_IN_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAFFEINE_MG() -> Py { ensure_init(); G_CAFFEINE_MG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAFFEINE_MG(v: Py) { ensure_init(); G_CAFFEINE_MG.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_IN_PACK() -> Py { ensure_init(); G_COUNT_IN_PACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_IN_PACK(v: Py) { ensure_init(); G_COUNT_IN_PACK.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_ASK() -> Py { ensure_init(); G_DISH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_ASK(v: Py) { ensure_init(); G_DISH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED_BEFORE() -> Py { ensure_init(); G__ASKED_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED_BEFORE(v: Py) { ensure_init(); G__ASKED_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__BAKED_WITH_COFFEE() -> Py { ensure_init(); G__BAKED_WITH_COFFEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKED_WITH_COFFEE(v: Py) { ensure_init(); G__BAKED_WITH_COFFEE.with(|c| *c.borrow_mut() = v); }
pub fn g__COFFEE_CROWD() -> Py { ensure_init(); G__COFFEE_CROWD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COFFEE_CROWD(v: Py) { ensure_init(); G__COFFEE_CROWD.with(|c| *c.borrow_mut() = v); }
pub fn g__COFFEE_METHOD() -> Py { ensure_init(); G__COFFEE_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COFFEE_METHOD(v: Py) { ensure_init(); G__COFFEE_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g__COFFEE_RATIO() -> Py { ensure_init(); G__COFFEE_RATIO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COFFEE_RATIO(v: Py) { ensure_init(); G__COFFEE_RATIO.with(|c| *c.borrow_mut() = v); }
pub fn g__COFFEE_TIME() -> Py { ensure_init(); G__COFFEE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COFFEE_TIME(v: Py) { ensure_init(); G__COFFEE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__COOKING_WORDS() -> Py { ensure_init(); G__COOKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COOKING_WORDS(v: Py) { ensure_init(); G__COOKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_ASK() -> Py { ensure_init(); G__COUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_ASK(v: Py) { ensure_init(); G__COUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_NOT_COUNTING() -> Py { ensure_init(); G__COUNT_NOT_COUNTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_NOT_COUNTING(v: Py) { ensure_init(); G__COUNT_NOT_COUNTING.with(|c| *c.borrow_mut() = v); }
pub fn g__DRESS_IS_A_GARMENT() -> Py { ensure_init(); G__DRESS_IS_A_GARMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRESS_IS_A_GARMENT(v: Py) { ensure_init(); G__DRESS_IS_A_GARMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_RECIPE() -> Py { ensure_init(); G__NOT_A_RECIPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_RECIPE(v: Py) { ensure_init(); G__NOT_A_RECIPE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_PRICE() -> Py { ensure_init(); G__NOT_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_PRICE(v: Py) { ensure_init(); G__NOT_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g__ONLY_MADE() -> Py { ensure_init(); G__ONLY_MADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONLY_MADE(v: Py) { ensure_init(); G__ONLY_MADE.with(|c| *c.borrow_mut() = v); }
pub fn g__PORTION() -> Py { ensure_init(); G__PORTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PORTION(v: Py) { ensure_init(); G__PORTION.with(|c| *c.borrow_mut() = v); }
pub fn g__RECIPE_ASK() -> Py { ensure_init(); G__RECIPE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECIPE_ASK(v: Py) { ensure_init(); G__RECIPE_ASK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Kitchen measures that are not a unit conversion: a pinch, and a strength of coffee.\n\nThe conversions themselves are `bot/knowledge/units.py`'s job, and it is one table:\n`how many grams in a cup of flour`, `how many grams in a teaspoon of salt` (6 g),\n`what gas mark is 160 fan` and `how many ml in a pint` are all answered from there, so this\nfile holds no second copy of any of them.\n\nWhat is left over is the everyday kitchen where the unit is approximate or the answer is not\na number at all: a pinch or a dash has no exact weight, and \"how many tablespoons of coffee\nper cup\" is a question about strength, not about tablespoons. Both are the answers lane's\nown data about cooking, keyed by meaning.\n");
    s_DISH_ASK(crate::knowledge::lookup::g_DISH_ASK());
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s__ASKED_BEFORE(py_tuple(vec![py_str("say"), py_str("said"), py_str("saying"), py_str("tell"), py_str("told"), py_str("remind"), py_str("remember"), py_str("mention"), py_str("mentioned"), py_str("earlier")]));
    s__NOT_PRICE(py_tuple(vec![py_str("price"), py_str("prices"), py_str("priced"), py_str("cost"), py_str("costs"), py_str("asking"), py_str("sell"), py_str("selling"), py_str("sold"), py_str("worth"), py_str("cheap"), py_str("expensive"), py_str("supermarket"), py_str("shop price")]));
    s_CAFFEINE_MG(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("instant"), py_str("nescafe"), py_str("granules")]), py_str("A mug of instant coffee is about 60 to 80mg")]), py_tuple(vec![py_tuple(vec![py_str("decaf"), py_str("decaffeinated")]), py_str("Decaf is 2 to 5mg a mug, so almost none")]), py_tuple(vec![py_tuple(vec![py_str("filter"), py_str("cafetiere"), py_str("french press"), py_str("percolator"), py_str("drip")]), py_str("A mug of filter coffee or cafetiere coffee is about 90 to 140mg")]), py_tuple(vec![py_tuple(vec![py_str("espresso"), py_str("americano"), py_str("flat white"), py_str("latte"), py_str("cappuccino"), py_str("single shot")]), py_str("A single espresso is about 65mg, and a latte or americano made with one shot is the same")]), py_tuple(vec![py_tuple(vec![py_str("green tea"), py_str("matcha")]), py_str("Green tea is about 25 to 40mg a mug")]), py_tuple(vec![py_tuple(vec![py_str("black tea"), py_str("tea"), py_str("tea bag"), py_str("tea bags"), py_str("builder")]), py_str("A mug of black tea is about 50mg")]), py_tuple(vec![py_tuple(vec![py_str("red bull"), py_str("energy drink"), py_str("energy drinks"), py_str("monster"), py_str("relentless")]), py_str("An energy drink is about 80mg a 250ml can, and the big 500ml cans are double that")]), py_tuple(vec![py_tuple(vec![py_str("cola"), py_str("coke"), py_str("pepsi"), py_str("fizzy")]), py_str("A 330ml can of cola is about 35mg")])]));
    s__BAKED_WITH_COFFEE(py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("bun"), py_str("buns"), py_str("bread"), py_str("loaf"), py_str("muffin"), py_str("muffins"), py_str("biscuit"), py_str("biscuits"), py_str("scone"), py_str("scones"), py_str("brownie"), py_str("brownies"), py_str("flapjack"), py_str("sponge"), py_str("sourdough"), py_str("dessert"), py_str("tart"), py_str("cheesecake"), py_str("pudding"), py_str("tiramisu"), py_str("walnut cake"), py_str("coffee cake")]));
    s__COFFEE_CROWD(py_tuple(vec![py_str("people"), py_str("crowd"), py_str("party"), py_str("batch"), py_str("litre"), py_str("litres"), py_str("jug"), py_str("flask"), py_str("urn"), py_str("thermos"), py_str("guests"), py_str("everyone"), py_str("mugs")]));
    s__COFFEE_TIME(py_tuple(vec![py_str("how long"), py_str("steep"), py_str("steeped"), py_str("press"), py_str("plunge"), py_str("minutes"), py_str("leave it"), py_str("how many minutes")]));
    s__COFFEE_RATIO(py_str("two level tablespoons of ground coffee per 200ml mug, which is about 12g, so 60g fills a litre jug and 240g makes 20 mugs"));
    s__COFFEE_METHOD(py_str("in a cafetiere, pour water a minute off the boil, leave it three to four minutes, then press and pour it all off the grounds, because coffee left standing on them goes on brewing and turns bitter"));
    s_COUNT_IN_PACK(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("spring onion"), py_str("spring onions"), py_str("scallion"), py_str("scallions")]), py_tuple(vec![py_str("bunch"), py_str("bunches")]), py_str("About 7 spring onions in a bunch"), py_str("a bunch is roughly 100g")]), py_tuple(vec![py_tuple(vec![py_str("banana"), py_str("bananas")]), py_tuple(vec![py_str("bunch"), py_str("bunches")]), py_str("About 5 bananas in a bunch"), py_str("they are sold by weight and a bunch is 4 to 6")]), py_tuple(vec![py_tuple(vec![py_str("clove"), py_str("cloves"), py_str("garlic")]), py_tuple(vec![py_str("bulb"), py_str("bulbs"), py_str("head"), py_str("heads"), py_str("garlic bulb")]), py_str("About 10 cloves in a bulb of garlic"), py_str("a bulb is 10 to 12, and a big one can be 15")]), py_tuple(vec![py_tuple(vec![py_str("walnut"), py_str("walnuts"), py_str("walnut halves")]), py_tuple(vec![py_str("handful"), py_str("handfuls")]), py_str("About 7 walnut halves in a handful"), py_str("a handful is 20 to 25g")]), py_tuple(vec![py_tuple(vec![py_str("almond"), py_str("almonds")]), py_tuple(vec![py_str("handful"), py_str("handfuls")]), py_str("About 20 to 25 almonds in a handful"), py_str("a handful is 30 to 35g")]), py_tuple(vec![py_tuple(vec![py_str("rasher"), py_str("rashers"), py_str("bacon")]), py_tuple(vec![py_str("packet"), py_str("packets"), py_str("pack"), py_str("packs")]), py_str("About 7 or 8 rashers in a 250g packet of bacon"), py_str("the pack says the weight, not the count")]), py_tuple(vec![py_tuple(vec![py_str("mushroom"), py_str("mushrooms")]), py_tuple(vec![py_str("punnet"), py_str("punnets"), py_str("pack"), py_str("packs"), py_str("box"), py_str("boxes")]), py_str("About 10 mushrooms in a 250g punnet"), py_str("a punnet is 250g, and a big one 400g")]), py_tuple(vec![py_tuple(vec![py_str("strawberr"), py_str("strawberries")]), py_tuple(vec![py_str("punnet"), py_str("punnets")]), py_str("About 20 to 25 strawberries in a 400g punnet"), py_str("the count depends on the size")]), py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes")]), py_tuple(vec![py_str("punnet"), py_str("punnets"), py_str("vine"), py_str("vines"), py_str("pack"), py_str("packs")]), py_str("About 6 tomatoes in a 400g punnet"), py_str("and a vine is usually 5 or 6 on the truss")])]));
    s__COUNT_ASK(py_tuple(vec![py_str("how many"), py_str("how many are"), py_str("many"), py_str("number of"), py_str("how much does"), py_str("how much do"), py_str("weigh"), py_str("weighs"), py_str("weight"), py_str("grams"), py_str("gram")]));
    s__COUNT_NOT_COUNTING(py_tuple(vec![py_str("plant"), py_str("planting"), py_str("planted"), py_str("grow"), py_str("growing"), py_str("grew"), py_str("sow"), py_str("sowing"), py_str("sow"), py_str("spacing"), py_str("apart"), py_str("deep"), py_str("when to"), py_str("best time"), py_str("should i eat"), py_str("can i eat"), py_str("a day"), py_str("per day"), py_str("every day"), py_str("calories"), py_str("carbs"), py_str("protein"), py_str("recipe"), py_str("cook"), py_str("cooked"), py_str("roast"), py_str("roasted"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("buy"), py_str("buying"), py_str("cheap"), py_str("freeze"), py_str("freezing"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("storing")]));
    s_AMOUNT_IN_A_DISH(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("victoria sponge"), py_str("sandwich cake"), py_str("cake mix"), py_str("baking"), py_str("bake"), py_str("baking a cake")]), py_dict(vec![(py_tuple(vec![py_str("eggs"), py_str("egg")]), py_tuple(vec![py_str("Three eggs for a 20cm sponge"), py_str("weigh them in their shells and use the same weight of soft butter, caster sugar and self-raising flour - about 175g of each for three medium eggs - so the cake always balances however big the eggs are")])), (py_tuple(vec![py_str("butter")]), py_tuple(vec![py_str("About 175g of soft butter for a 20cm sponge"), py_str("the exact weight is the weight of three eggs in their shells, with the same of caster sugar and self-raising flour")])), (py_tuple(vec![py_str("sugar"), py_str("caster sugar")]), py_tuple(vec![py_str("About 175g of caster sugar for a 20cm sponge"), py_str("weigh three eggs in their shells and use that weight of butter, sugar and self-raising flour, which is the whole of a victoria sponge"), py_str("A slice of a 20cm sponge has about 18g of sugar in it, and the whole cake has about 175g, which is nobody's portion - it is the slice that matters")])), (py_tuple(vec![py_str("flour")]), py_tuple(vec![py_str("About 175g of self-raising flour for a 20cm sponge"), py_str("weigh three eggs in their shells and use that weight of soft butter, caster sugar and flour; fold the flour in last and stop as soon as it is mixed")])), (py_tuple(vec![py_str("milk")]), py_tuple(vec![py_str("A tablespoon or two of milk at the end, and only if the batter is stiff"), py_str("a sponge batter should drop slowly off the spoon, so loosen it a spoonful at a time")]))])]), py_tuple(vec![py_tuple(vec![py_str("pancake"), py_str("pancakes"), py_str("batter"), py_str("crepe"), py_str("crepes")]), py_dict(vec![(py_tuple(vec![py_str("milk")]), py_tuple(vec![py_str("About 600ml of milk to 200g of plain flour and 3 eggs, which makes about 16 thin pancakes"), py_str("for four people; a thicker batter for American-style pancakes is 300ml of milk to 250g of self-raising flour and 2 eggs")])), (py_tuple(vec![py_str("flour")]), py_tuple(vec![py_str("200g of plain flour for four people"), py_str("with 3 eggs and 600ml of milk, which makes about 16 thin pancakes; a family with small children is happy on 150g, and they take a minute a side in a hot pan")])), (py_tuple(vec![py_str("sugar")]), py_tuple(vec![py_str("A teaspoon of sugar in the batter is plenty"), py_str("it helps them brown but does not make them sweet; the sweetness goes on at the table")]))])]), py_tuple(vec![py_tuple(vec![py_str("bolognese"), py_str("bolagnese"), py_str("ragu"), py_str("spag bol"), py_str("spaghetti bolognese"), py_str("mince")]), py_dict(vec![(py_tuple(vec![py_str("tomatoes"), py_str("tomato"), py_str("tins of tomatoes"), py_str("tin of tomatoes"), py_str("chopped tomatoes")]), py_tuple(vec![py_str("One 400g tin of chopped tomatoes to 500g of mince"), py_str("with a tablespoon of tomato puree and a good 40 minutes of simmering, which is what turns it from mince in tomato sauce into a ragu")])), (py_tuple(vec![py_str("onions"), py_str("onion")]), py_tuple(vec![py_str("One onion to 500g of mince"), py_str("chopped small and softened first, with a carrot and a stick of celery if you have them")])), (py_tuple(vec![py_str("garlic")]), py_tuple(vec![py_str("Two cloves of garlic to 500g of mince"), py_str("in with the onion at the end of its cooking so it does not burn")]))])]), py_tuple(vec![py_tuple(vec![py_str("curry"), py_str("curries"), py_str("korma"), py_str("tikka"), py_str("masala"), py_str("balti"), py_str("dahl"), py_str("daal")]), py_dict(vec![(py_tuple(vec![py_str("onions"), py_str("onion")]), py_tuple(vec![py_str("One large onion to 500g of meat or vegetables"), py_str("cooked slowly until it is properly golden before the spices go in, which is most of the flavour of a curry")])), (py_tuple(vec![py_str("cream"), py_str("double cream")]), py_tuple(vec![py_str("About 100ml of cream for a curry for four"), py_str("stirred in at the end off the heat so it does not split; yogurt goes in the same way, a spoonful at a time")])), (py_tuple(vec![py_str("tomatoes"), py_str("tomato"), py_str("tinned tomatoes"), py_str("chopped tomatoes")]), py_tuple(vec![py_str("One 400g tin of tomatoes for a curry for four"), py_str("or four fresh ones, and let it cook down until the oil separates before the meat or vegetables go back in")]))])]), py_tuple(vec![py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mash potato"), py_str("mashed potatoes")]), py_dict(vec![(py_tuple(vec![py_str("butter")]), py_tuple(vec![py_str("About 50g of butter per kilo of potatoes"), py_str("with 100ml of warm milk beat in at the end; warm milk and butter go in before the mashing, cold milk makes it gluey")])), (py_tuple(vec![py_str("milk")]), py_tuple(vec![py_str("About 100ml of warm milk per kilo of potatoes"), py_str("with 50g of butter, both warmed first so the mash does not go cold and gluey")])), (py_tuple(vec![py_str("salt")]), py_tuple(vec![py_str("A good pinch of salt in the water, and a little more once it is mashed"), py_str("salted water is the only chance the potato itself gets to be seasoned")]))])]), py_tuple(vec![py_tuple(vec![py_str("meringue"), py_str("meringues"), py_str("pavlova"), py_str("meringue nests")]), py_dict(vec![(py_tuple(vec![py_str("sugar"), py_str("caster sugar")]), py_tuple(vec![py_str("50g of caster sugar for each egg white"), py_str("added a spoonful at a time once the whites are at soft peaks, and the bowl and whisk completely free of grease or the whites will not hold")])), (py_tuple(vec![py_str("eggs"), py_str("egg"), py_str("egg whites")]), py_tuple(vec![py_str("One egg white to 50g of caster sugar, so three whites take 150g"), py_str("and a pavlova wants the same mix with a teaspoon of cornflour and vinegar folded in")]))])]), py_tuple(vec![py_tuple(vec![py_str("cup of tea"), py_str("a brew"), py_str("cuppa"), py_str("mug of tea")]), py_dict(vec![(py_tuple(vec![py_str("sugar")]), py_tuple(vec![py_str("None unless you add it: a level teaspoon of sugar is about 4g, so two sugars is 8g and roughly 30 calories in the mug"), py_str("the tea bag itself has no sugar in it at all")]))])]), py_tuple(vec![py_tuple(vec![py_str("porridge"), py_str("porage"), py_str("oats"), py_str("oatmeal")]), py_dict(vec![(py_tuple(vec![py_str("milk")]), py_tuple(vec![py_str("About 300ml of milk to 50g of oats for one bowl"), py_str("or half milk and half water, which is how most people like it; stir it as it comes to the boil or it catches on the pan")])), (py_tuple(vec![py_str("oats"), py_str("oat")]), py_tuple(vec![py_str("50g of oats and 300ml of milk for one bowl"), py_str("a pinch of salt in it makes a real difference, and it takes about five minutes on the hob or three in the microwave, stirred halfway")]))])]), py_tuple(vec![py_tuple(vec![py_str("gravy")]), py_dict(vec![(py_tuple(vec![py_str("stock"), py_str("water")]), py_tuple(vec![py_str("About 300ml of stock for gravy for four"), py_str("with a tablespoon of flour cooked into the meat juices first; add the stock a little at a time and stir, or it goes lumpy")]))])])]));
    s__RECIPE_ASK(py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("amount"), py_str("quantity")]));
    s__NOT_A_RECIPE(py_tuple(vec![py_str("is in"), py_str("are in"), py_str("is there in"), py_str("are there in"), py_str("contains"), py_str("contain"), py_str("how much is"), py_str("how much are"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("calories"), py_str("calorie"), py_str("carbs"), py_str("protein"), py_str("per slice"), py_str("per portion"), py_str("a day"), py_str("per day"), py_str("every day"), py_str("healthy"), py_str("healthier"), py_str("good for"), py_str("bad for"), py_str("is it ok"), py_str("is it okay")]));
    s__PORTION(py_tuple(vec![py_str("slice"), py_str("slices"), py_str("piece of"), py_str("portion"), py_str("serving"), py_str("per slice"), py_str("helping")]));
    s__ONLY_MADE(call_builtin("frozenset", &[py_tuple(vec![py_str("batter"), py_str("dough"), py_str("bolognese"), py_str("bolagnese"), py_str("ragu"), py_str("spag bol"), py_str("spaghetti bolognese"), py_str("mince"), py_str("curry"), py_str("curries"), py_str("korma"), py_str("tikka"), py_str("masala"), py_str("balti"), py_str("mash"), py_str("mashed potato"), py_str("mash potato"), py_str("mashed potatoes"), py_str("meringue"), py_str("meringues"), py_str("pavlova"), py_str("meringue nests"), py_str("gravy"), py_str("cup of tea"), py_str("cuppa"), py_str("mug of tea"), py_str("porridge"), py_str("porage"), py_str("yorkshire pudding"), py_str("yorkshires")])]).unwrap_or_else(|e| exc::raise(e)));
    s__COOKING_WORDS(py_tuple(vec![py_str("goes in"), py_str("go in"), py_str("put in"), py_str("add"), py_str("do i need"), py_str("should i use"), py_str("use"), py_str("recipe"), py_str("make"), py_str("making"), py_str("mix"), py_str("batter"), py_str("dough"), py_str("cook"), py_str("cooking"), py_str("for a"), py_str("for the"), py_str("for my"), py_str("to a"), py_str("into")]));
    s__DRESS_IS_A_GARMENT(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the|my|her|his|our|their|this|that|these|those|your|another|one|two|any|some)\\s+(?:\\w+\\s+){0,3}dress(?:es)?\\b|\\bsize\\s+\\d*\\s*dress(?:es)?\\b|\\b(?:wear|wears|wearing|wore|worn|buy|buys|bought|buying|make|makes|making|made|sew|sews|sewing|sewn|alter|alters|altering|altered|return|returns|returning|returned|try|tries|trying|tried|fit|fits|fitting|fitted|hem|hems|hemming|shorten|shortening|iron|ironing|hang|hanging|pack|packing|choose|choosing|pick|picking)\\s+(?:a|an|the|my|her|his|this|that|your)?\\s*(?:new\\s+|old\\s+|long\\s+|short\\s+|black\\s+|white\\s+|red\\s+|blue\\s+|green\\s+|pink\\s+|navy\\s+|floral\\s+|silk\\s+|linen\\s+|lace\\s+|summer\\s+|winter\\s+|evening\\s+|cocktail\\s+|prom\\s+|wedding\\s+|bridesmaid\\s+|christening\\s+|maxi\\s+|midi\\s+)?dress(?:es)?\\b|\\bdress\\s+(?:size|sizes|shop|shops|code|sense|hanger|hangers|pattern|patterns|fabric|material|making|alterations?)\\b|\\bdressmaking\\b|\\bwedding\\s+dress\\b|\\bbridesmaid\\s+dress(?:es)?\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("caffeine-per-drink")); __args.push(py_tuple(vec![py_tuple(vec![py_str("caffeine"), py_str("mg of caffeine")]), py_tuple(vec![py_str("how much"), py_str("much"), py_str("many"), py_str("amount"), py_str("mg"), py_str("milligrams"), py_str("have caffeine"), py_str("has caffeine"), py_str("caffeine in"), py_str("contain caffeine"), py_str("contains caffeine"), py_str("any caffeine")])])); __args.push(g__caffeine_per_drink()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("amount"), py_str("mg"), py_str("milligrams"), py_str("caffeine in"), py_str("have caffeine"), py_str("has caffeine"), py_str("contain caffeine"), py_str("contains caffeine"), py_str("any caffeine")])), ("avoid".to_string(), py_tuple(vec![py_str("safe"), py_str("limit"), py_str("too much"), py_str("pregnan"), py_str("day"), py_str("daily"), py_str("cut down"), py_str("cutting"), py_str("give up"), py_str("quitting"), py_str("sleep"), py_str("sleeping"), py_str("bed"), py_str("energy"), py_str("tired"), py_str("withdraw"), py_str("headache"), py_str("weak"), py_str("watery"), py_str("bitter"), py_str("sour"), py_str("taste"), py_str("tastes"), py_str("tasting"), py_str("brew"), py_str("brewing"), py_str("strong"), py_str("stronger"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("toddler"), py_str("baby"), py_str("year old"), py_str("years old"), py_str("teenager"), py_str("teenagers"), py_str("breastfeed"), py_str("breast feed"), py_str("breast feeding"), py_str("breastfed"), py_str("nursing"), py_str("nurse"), py_str("lactating"), py_str("chestfeed"), py_str("chest feed"), py_str("while feeding"), py_str("still feeding")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pinch")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pinch"), py_str("pinches"), py_str("dash")]), py_tuple(vec![py_str("how much"), py_str("what"), py_str("much"), py_str("salt"), py_str("spice"), py_str("recipe"), py_str("mean"), py_str("grams"), py_str("gram"), py_str("weigh"), py_str("weighs"), py_str("weight")])])); __args.push(py_str("A pinch is what you can hold between your thumb and finger: about 1/16 of a teaspoon, a little under half a gram of salt, and a good pinch nearer one. A dash of liquid is about 1/8 of a teaspoon, and a handful is 30 to 40g.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("big"), py_str("measure"), py_str("mean"), py_str("means"), py_str("size")])), ("avoid".to_string(), g__ASKED_BEFORE())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("splash")); __args.push(py_tuple(vec![py_tuple(vec![py_str("splash"), py_str("glug"), py_str("drizzle")]), py_tuple(vec![py_str("how much"), py_str("what"), py_str("much"), py_str("milk"), py_str("oil"), py_str("sauce"), py_str("cream"), py_str("recipe")])])); __args.push(py_str("A splash is a loose tablespoon or two, and a drizzle a thin stream over the top: both are meant to be tasted and topped up, so start small.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("mean"), py_str("means"), py_str("big")])), ("avoid".to_string(), g__ASKED_BEFORE())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dressing-quantity")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dressing"), py_str("vinaigrette"), py_str("salad dressing"), py_str("salad sauce")]), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("for 2"), py_str("for 3"), py_str("for 4"), py_str("for 5"), py_str("for 6"), py_str("for 7"), py_str("for 8"), py_str("for ten"), py_str("for twelve"), py_str("people"), py_str("per person"), py_str("a head"), py_str("salad"), py_str("make"), py_str("need")])])); __args.push(py_str("About a tablespoon (15ml) a head, made at 3 parts oil to 1 of vinegar or lemon: 45ml of oil and 15ml of vinegar dresses four, 70ml and 25ml dresses six, plus a teaspoon of mustard and a pinch of salt either way. Put the dressing in the bottom of the bowl and toss the leaves on top of it rather than pouring it over.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("per person"), py_str("per head"), py_str("a head"), py_str("each"), py_str("for 2"), py_str("for 3"), py_str("for 4"), py_str("for 5"), py_str("for 6"), py_str("for 7"), py_str("for 8"), py_str("for ten"), py_str("for twelve")])), ("guard".to_string(), g__a_dress_is_not_dressing()), ("avoid".to_string(), op_add(&op_add(&g__ASKED_BEFORE(), &g__NOT_PRICE()), &py_tuple(vec![py_str("wound"), py_str("wound dressing"), py_str("cut"), py_str("burn"), py_str("ulcer"), py_str("bandage"), py_str("surgical"), py_str("window dressing")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("knob")); __args.push(py_tuple(vec![py_tuple(vec![py_str("knob"), py_str("knobs"), py_str("dollop"), py_str("dollops"), py_str("squirt"), py_str("squirts"), py_str("gob"), py_str("a slug of"), py_str("slug of"), py_str("slugs of"), py_str("a squirt of"), py_str("squirt of"), py_str("a dollop of"), py_str("dollops of"), py_str("a knob of"), py_str("knobs of"), py_str("a gob of")]), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("what"), py_str("much"), py_str("butter"), py_str("cream"), py_str("size"), py_str("mean"), py_str("means"), py_str("big"), py_str("recipe")])])); __args.push(py_str("A knob is about a tablespoon, 15g; a dollop is nearer two tablespoons. Both are meant to be judged by eye, since a walnut-sized lump is a knob and you can always add a little more.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("mean"), py_str("means"), py_str("size"), py_str("big")])), ("avoid".to_string(), op_add(&g__ASKED_BEFORE(), &g__NOT_PRICE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spoonful")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spoonful"), py_str("spoonfuls"), py_str("spoon"), py_str("spoons"), py_str("cupful"), py_str("cupfuls")]), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("what"), py_str("much"), py_str("size"), py_str("mean"), py_str("means"), py_str("big"), py_str("recipe")])])); __args.push(py_str("A spoonful means a level tablespoon, 15ml unless the recipe says heaped; a cupful is 250ml, filled level rather than heaped. Go by the recipe where it gives a weight.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("mean"), py_str("means"), py_str("size"), py_str("big")])), ("avoid".to_string(), op_add(&op_add(&g__ASKED_BEFORE(), &g__NOT_PRICE()), &py_tuple(vec![py_str("cupful of rice"), py_str("cup of rice"), py_str("cupful of flour"), py_str("cup of flour"), py_str("cupful of sugar"), py_str("cup of sugar")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drop")); __args.push(py_tuple(vec![py_tuple(vec![py_str("a drop of"), py_str("drop of"), py_str("drops of"), py_str("a grating"), py_str("grating of"), py_str("a scattering"), py_str("scattering of"), py_str("a sprinkling"), py_str("sprinkling of")]), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("what"), py_str("much"), py_str("size"), py_str("mean"), py_str("means"), py_str("big")])])); __args.push(py_str("A drop is the smallest amount you can taste, a grating is one pass over the fine side of the grater, and a scattering is a light pinch over the top: all three are to taste, so start small and add more.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("mean"), py_str("means"), py_str("size"), py_str("big")])), ("avoid".to_string(), op_add(&g__ASKED_BEFORE(), &py_tuple(vec![py_str("eye"), py_str("ear"), py_str("nose"), py_str("diffuser"), py_str("price"), py_str("asking"), py_str("rain"), py_str("hint"), py_str("temperature"), py_str("charge")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bunch")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bunch"), py_str("bunches"), py_str("sprig"), py_str("sprigs"), py_str("punnet")]), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("what"), py_str("much"), py_str("size"), py_str("mean"), py_str("means"), py_str("big"), py_str("parsley"), py_str("thyme"), py_str("basil"), py_str("coriander"), py_str("mint"), py_str("dill"), py_str("rosemary")])])); __args.push(py_str("A bunch is whatever you can close one hand round at the stems; a sprig is a single small stem, and a punnet is the little basket soft fruit comes in. None of them is a weight, so go by eye, and add herbs a little at a time and taste as you go.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("mean"), py_str("means"), py_str("size"), py_str("big")])), ("avoid".to_string(), op_add(&op_add(&op_add(&g__ASKED_BEFORE(), &g__NOT_PRICE()), &crate::knowledge::lookup::g_DISH_ASK()), &py_tuple(vec![py_str("buy"), py_str("banana"), py_str("bananas"), py_str("grape"), py_str("grapes"), py_str("flower"), py_str("flowers"), py_str("wedding"), py_str("bouquet"), py_str("bouquets")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nip")); __args.push(py_tuple(vec![py_tuple(vec![py_str("a nip of"), py_str("nip of"), py_str("a swig of"), py_str("swig of"), py_str("a tot of"), py_str("tot of"), py_str("a nip"), py_str("a tot"), py_str("a swig"), py_str("nip measure"), py_str("single measure")]), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("what"), py_str("much"), py_str("size"), py_str("mean"), py_str("means"), py_str("big"), py_str("drink"), py_str("spirit"), py_str("spirits"), py_str("measure"), py_str("recipe"), py_str("pour")])])); __args.push(py_str("A nip or a tot is a 25ml measure, the small end of a pub single; a swig is a mouthful of whatever you are drinking and no measure at all. A shot in a recipe is usually 25 to 35ml, so measure it rather than free-pour.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("much"), py_str("many"), py_str("what"), py_str("mean"), py_str("means"), py_str("size"), py_str("big")])), ("avoid".to_string(), op_add(&g__ASKED_BEFORE(), &py_tuple(vec![py_str("bite"), py_str("nip at"), py_str("nip out"), py_str("nip to"), py_str("nip in")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coffee-strength")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("grounds"), py_str("ground coffee"), py_str("espresso"), py_str("cafetiere"), py_str("french press")]), py_tuple(vec![py_str("cup"), py_str("cups"), py_str("mug"), py_str("mugs"), py_str("per cup"), py_str("tablespoons"), py_str("strong"), py_str("cafetiere"), py_str("cafetieres"), py_str("french press"), py_str("jug"), py_str("strength"), py_str("scoop"), py_str("scoops"), py_str("weak"), py_str("weaker"), py_str("stronger"), py_str("people"), py_str("crowd"), py_str("party"), py_str("batch"), py_str("litre"), py_str("litres"), py_str("press"), py_str("plunge"), py_str("steep"), py_str("long"), py_str("use"), py_str("used")])])); __args.push(g__coffee_strength()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("per"), py_str("tablespoon"), py_str("tablespoons"), py_str("scoop"), py_str("scoops"), py_str("strong"), py_str("stronger"), py_str("strength"), py_str("weak"), py_str("weaker"), py_str("cafetiere"), py_str("french press"), py_str("mug"), py_str("make"), py_str("making"), py_str("brew"), py_str("ratio"), py_str("grounds"), py_str("ground coffee"), py_str("how long"), py_str("how many people"), py_str("people"), py_str("crowd"), py_str("party"), py_str("litre"), py_str("litres"), py_str("batch"), py_str("press"), py_str("plunge"), py_str("steep"), py_str("use")])), ("avoid".to_string(), py_tuple(vec![py_str("machine"), py_str("machines"), py_str("pod"), py_str("pods"), py_str("capsule"), py_str("capsules"), py_str("grinder"), py_str("clean"), py_str("cleaning"), py_str("descale"), py_str("descaler"), py_str("kettle"), py_str("buy"), py_str("best"), py_str("cheapest"), py_str("gift"), py_str("shop"), py_str("too much"), py_str("a day"), py_str("per day"), py_str("daily"), py_str("limit"), py_str("safe"), py_str("caffeine"), py_str("tea")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coffee-taste")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("ground coffee"), py_str("brew"), py_str("espresso"), py_str("americano")]), py_tuple(vec![py_str("bitter"), py_str("sour"), py_str("taste"), py_str("tastes"), py_str("tasting"), py_str("burnt"), py_str("weak"), py_str("watery"), py_str("strong"), py_str("stronger"), py_str("brew"), py_str("brewing"), py_str("grind"), py_str("better"), py_str("improve")])])); __args.push(py_str("Coffee that tastes wrong is nearly always the brew, not the bean: water just off the boil (about 90 to 95C rather than boiling), a slightly coarser grind and less time in the water, so 3 to 4 minutes in a cafetiere or 25 to 30 seconds for espresso. Bitter means too hot, too fine or too long; sour or thin means the other way, with a finer grind and a little longer. A tiny pinch of salt in the cup takes the edge off a bitter pot.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("why"), py_str("what"), py_str("make"), py_str("makes"), py_str("making"), py_str("fix"), py_str("improve"), py_str("better"), py_str("taste"), py_str("tastes"), py_str("tasting"), py_str("bitter"), py_str("sour"), py_str("burnt"), py_str("brew"), py_str("brewing"), py_str("grind"), py_str("weak"), py_str("watery"), py_str("strong"), py_str("stronger")])), ("avoid".to_string(), op_add(&g__ASKED_BEFORE(), &g__BAKED_WITH_COFFEE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count-in-a-pack")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bunch"), py_str("bunches"), py_str("punnet"), py_str("punnets"), py_str("packet"), py_str("packets"), py_str("handful"), py_str("handfuls"), py_str("bulb"), py_str("bulbs")]), py_tuple(vec![py_str("spring onion"), py_str("banana"), py_str("bananas"), py_str("clove"), py_str("cloves"), py_str("garlic"), py_str("walnut"), py_str("walnuts"), py_str("almond"), py_str("almonds"), py_str("rasher"), py_str("rashers"), py_str("bacon"), py_str("mushroom"), py_str("mushrooms"), py_str("strawberr"), py_str("strawberries"), py_str("tomato"), py_str("tomatoes")])])); __args.push(g__count_in_a_pack()); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), py_tuple(vec![py_str("how many"), py_str("how many are"), py_str("many"), py_str("number of"), py_str("how much does"), py_str("how much do"), py_str("weigh"), py_str("weighs"), py_str("weight"), py_str("grams"), py_str("gram")])), ("avoid".to_string(), op_add(&op_add(&g__ASKED_BEFORE(), &g__NOT_PRICE()), &py_tuple(vec![py_str("flower"), py_str("flowers"), py_str("wedding"), py_str("bouquet"), py_str("bouquets"), py_str("plant"), py_str("planting"), py_str("grow"), py_str("growing"), py_str("sow"), py_str("sowing"), py_str("apart"), py_str("spacing"), py_str("should i eat"), py_str("can i eat"), py_str("per day"), py_str("every day"), py_str("calories"), py_str("carbs"), py_str("protein")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("amount-in-a-dish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("eggs"), py_str("egg"), py_str("milk"), py_str("butter"), py_str("cream"), py_str("onions"), py_str("onion"), py_str("sugar"), py_str("flour"), py_str("tomatoes"), py_str("tomato"), py_str("garlic"), py_str("salt"), py_str("oats"), py_str("stock"), py_str("caster sugar")]), py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("pancake"), py_str("pancakes"), py_str("batter"), py_str("bolognese"), py_str("ragu"), py_str("curry"), py_str("curries"), py_str("korma"), py_str("tikka"), py_str("masala"), py_str("mash"), py_str("mashed potato"), py_str("meringue"), py_str("meringues"), py_str("pavlova"), py_str("porridge"), py_str("gravy"), py_str("cup of tea"), py_str("cuppa"), py_str("mug of tea"), py_str("yorkshire pudding")])])); __args.push(g__amount_in_a_dish()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much"), py_str("many"), py_str("amount"), py_str("quantity"), py_str("per person"), py_str("do i need"), py_str("goes in"), py_str("go in"), py_str("put in")])), ("avoid".to_string(), op_add(&op_add(&g__ASKED_BEFORE(), &g__NOT_PRICE()), &py_tuple(vec![py_str("how long"), py_str("how many minutes"), py_str("how many mins"), py_str("calories"), py_str("carbs"), py_str("protein"), py_str("how much is"), py_str("how much are"), py_str("weight"), py_str("how heavy")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _caffeine_per_drink(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9% ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    lead = py_str("");
    let __iter1 = py_iter_or_raise(&g_CAFFEINE_MG());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    phrase = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))) {
    lead = phrase.clone();
    break;
    }
    }
    if py_truthy(&lead.clone()) {
    break;
    }
    }
    rest = py_str("The usual daily limit for an adult is 400mg, which is about four instant coffees, and 200mg in pregnancy.");
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    return op_add(&py_str("It varies with the drink: a mug of instant coffee is 60 to 80mg, filter coffee 90 to 140mg, a single espresso about 65mg, black tea about 50mg, green tea 25 to 40mg and a 330ml cola about 35mg. "), &rest.clone());
    }
    return op_add(&op_add(&lead.clone(), &py_str(". ")), &rest.clone());
    Py::None
}
pub fn _coffee_strength(args: &[Py]) -> Py {
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
    let mut says: Py = Py::None;
    let _ = py_str("How much coffee to make, in the order the line asked for it.");
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9% ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
pub fn __nested_says_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut memory: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "words").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut word: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    says = {{ let memory = memory.clone(); let said = said.clone(); let says = says.clone(); let understood = understood.clone(); py_func("says", move |args: &[Py]| { let mut full = vec![memory.clone(), said.clone(), says.clone(), understood.clone()]; full.extend_from_slice(args); __nested_says_0(&full) }) }};
    if py_truthy(&py_call(&says, &[g__COFFEE_TIME()])) {
    return op_add(&op_add(&py_str("In a cafetiere: pour water a minute off the boil, leave it three to four minutes, then press and pour it all off the grounds or it goes on brewing and turns bitter. That is "), &g__COFFEE_RATIO()), &py_str("."));
    }
    if py_truthy(&py_call(&says, &[g__COFFEE_CROWD()])) {
    return op_add(&op_add(&py_str("240g of ground coffee makes 20 mugs, each mug holding about two level tablespoons, 12g, and 60g filling a litre jug. "), &mcall(&g__COFFEE_METHOD(), "capitalize", &[])), &py_str(". Go stronger for espresso, weaker with milk."));
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("Measure it by the mug: "), &g__COFFEE_RATIO()), &py_str(". ")), &mcall(&g__COFFEE_METHOD(), "capitalize", &[])), &py_str(". Stronger for espresso, weaker with milk."));
    Py::None
}
pub fn _count_in_a_pack(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut names: Py = Py::None;
    let mut note: Py = Py::None;
    let mut packs: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut things: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How many come in the bunch or the pack, from COUNT_IN_PACK, or nothing.");
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COUNT_NOT_COUNTING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COUNT_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&g_COUNT_IN_PACK());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    things = __unpk2[0].clone();
    packs = __unpk2[1].clone();
    count = __unpk2[2].clone();
    note = __unpk2[3].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&packs.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&py_str(" "), &word.clone()))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&names.clone()))) {
    continue;
    }
    return op_add(&op_add(&op_add(&count.clone(), &py_str(": ")), &note.clone()), &py_str(", so buy by weight and use the count as a guide."));
    }
    return py_str("");
    Py::None
}
pub fn _amount_in_a_dish(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut amounts: Py = Py::None;
    let mut as_eaten: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut dish_words: Py = Py::None;
    let mut ingredients: Py = Py::None;
    let mut named: Py = Py::None;
    let mut note: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How much of an ingredient goes into the dish the line names, or nothing.");
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RECIPE_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&g_AMOUNT_IN_A_DISH());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    dish_words = __unpk2[0].clone();
    amounts = __unpk2[1].clone();
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dish_words.clone()) { let mut dish: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &dish.clone()), &py_str(" "))))) {  __out.push(dish.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&mcall(&amounts.clone(), "items", &[]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    ingredients = __unpk4[0].clone();
    row = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ingredients.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str("s ")))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __unpk5 = py_unpack(&py_tuple(vec![geti(&row.clone(), &Py::Int(0)), geti(&row.clone(), &Py::Int(1))]), 2, None);
    amount = __unpk5[0].clone();
    note = __unpk5[1].clone();
    as_eaten = if py_truthy(&op_gt(&call_builtin("len", &[row.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { geti(&row.clone(), &Py::Int(2)) } else { py_str("") };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PORTION()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_RECIPE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut dish: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__ONLY_MADE(), &dish.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_cooking_words(&[said.clone()]))) } } } }) {
    if py_truthy(&Py::Bool(!py_truthy(&as_eaten.clone()))) {
    return py_str("");
    }
    return op_add(&as_eaten.clone(), &if py_truthy(&mcall(&as_eaten.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { py_str("") } else { py_str(".") });
    }
    return op_add(&op_add(&op_add(&op_add(&amount.clone(), &py_str(". ")), &mcall(&slice_or_raise(&note.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[])), &slice_or_raise(&note.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    }
    return py_str("");
    Py::None
}
pub fn _cooking_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The line asks for something to cook with, not for what is already in the food.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COOKING_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_dress_is_not_dressing(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line's \"dress\" is the garment, for the rows that key \"dressing\".");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__DRESS_IS_A_GARMENT(), "search", &[said.clone()])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.measures".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AMOUNT_IN_A_DISH" => Some(g_AMOUNT_IN_A_DISH()),
            "ANSWERS" => Some(g_ANSWERS()),
            "CAFFEINE_MG" => Some(g_CAFFEINE_MG()),
            "COUNT_IN_PACK" => Some(g_COUNT_IN_PACK()),
            "DISH_ASK" => Some(g_DISH_ASK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASKED_BEFORE" => Some(g__ASKED_BEFORE()),
            "_BAKED_WITH_COFFEE" => Some(g__BAKED_WITH_COFFEE()),
            "_COFFEE_CROWD" => Some(g__COFFEE_CROWD()),
            "_COFFEE_METHOD" => Some(g__COFFEE_METHOD()),
            "_COFFEE_RATIO" => Some(g__COFFEE_RATIO()),
            "_COFFEE_TIME" => Some(g__COFFEE_TIME()),
            "_COOKING_WORDS" => Some(g__COOKING_WORDS()),
            "_COUNT_ASK" => Some(g__COUNT_ASK()),
            "_COUNT_NOT_COUNTING" => Some(g__COUNT_NOT_COUNTING()),
            "_DRESS_IS_A_GARMENT" => Some(g__DRESS_IS_A_GARMENT()),
            "_NOT_A_RECIPE" => Some(g__NOT_A_RECIPE()),
            "_NOT_PRICE" => Some(g__NOT_PRICE()),
            "_ONLY_MADE" => Some(g__ONLY_MADE()),
            "_PORTION" => Some(g__PORTION()),
            "_RECIPE_ASK" => Some(g__RECIPE_ASK()),
            "_a_dress_is_not_dressing" => Some(py_func("_a_dress_is_not_dressing", |args: &[Py]| _a_dress_is_not_dressing(args))),
            "_amount_in_a_dish" => Some(py_func("_amount_in_a_dish", |args: &[Py]| _amount_in_a_dish(args))),
            "_caffeine_per_drink" => Some(py_func("_caffeine_per_drink", |args: &[Py]| _caffeine_per_drink(args))),
            "_coffee_strength" => Some(py_func("_coffee_strength", |args: &[Py]| _coffee_strength(args))),
            "_cooking_words" => Some(py_func("_cooking_words", |args: &[Py]| _cooking_words(args))),
            "_count_in_a_pack" => Some(py_func("_count_in_a_pack", |args: &[Py]| _count_in_a_pack(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AMOUNT_IN_A_DISH" => s_AMOUNT_IN_A_DISH(v),
            "ANSWERS" => s_ANSWERS(v),
            "CAFFEINE_MG" => s_CAFFEINE_MG(v),
            "COUNT_IN_PACK" => s_COUNT_IN_PACK(v),
            "DISH_ASK" => s_DISH_ASK(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASKED_BEFORE" => s__ASKED_BEFORE(v),
            "_BAKED_WITH_COFFEE" => s__BAKED_WITH_COFFEE(v),
            "_COFFEE_CROWD" => s__COFFEE_CROWD(v),
            "_COFFEE_METHOD" => s__COFFEE_METHOD(v),
            "_COFFEE_RATIO" => s__COFFEE_RATIO(v),
            "_COFFEE_TIME" => s__COFFEE_TIME(v),
            "_COOKING_WORDS" => s__COOKING_WORDS(v),
            "_COUNT_ASK" => s__COUNT_ASK(v),
            "_COUNT_NOT_COUNTING" => s__COUNT_NOT_COUNTING(v),
            "_DRESS_IS_A_GARMENT" => s__DRESS_IS_A_GARMENT(v),
            "_NOT_A_RECIPE" => s__NOT_A_RECIPE(v),
            "_NOT_PRICE" => s__NOT_PRICE(v),
            "_ONLY_MADE" => s__ONLY_MADE(v),
            "_PORTION" => s__PORTION(v),
            "_RECIPE_ASK" => s__RECIPE_ASK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.measures", module_obj);
    pyrt::xrefs::reg_fn("knowledge.measures", "_caffeine_per_drink", py_func("_caffeine_per_drink", |args: &[Py]| _caffeine_per_drink(args)));
    pyrt::xrefs::reg_fn("knowledge.measures", "_coffee_strength", py_func("_coffee_strength", |args: &[Py]| _coffee_strength(args)));
    pyrt::xrefs::reg_fn("knowledge.measures", "_count_in_a_pack", py_func("_count_in_a_pack", |args: &[Py]| _count_in_a_pack(args)));
    pyrt::xrefs::reg_fn("knowledge.measures", "_amount_in_a_dish", py_func("_amount_in_a_dish", |args: &[Py]| _amount_in_a_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.measures", "_cooking_words", py_func("_cooking_words", |args: &[Py]| _cooking_words(args)));
    pyrt::xrefs::reg_fn("knowledge.measures", "_a_dress_is_not_dressing", py_func("_a_dress_is_not_dressing", |args: &[Py]| _a_dress_is_not_dressing(args)));
    pyrt::xrefs::reg_access("knowledge.measures", "AMOUNT_IN_A_DISH", g_AMOUNT_IN_A_DISH, s_AMOUNT_IN_A_DISH);
    pyrt::xrefs::reg_access("knowledge.measures", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.measures", "CAFFEINE_MG", g_CAFFEINE_MG, s_CAFFEINE_MG);
    pyrt::xrefs::reg_access("knowledge.measures", "COUNT_IN_PACK", g_COUNT_IN_PACK, s_COUNT_IN_PACK);
    pyrt::xrefs::reg_access("knowledge.measures", "DISH_ASK", g_DISH_ASK, s_DISH_ASK);
    pyrt::xrefs::reg_access("knowledge.measures", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.measures", "_ASKED_BEFORE", g__ASKED_BEFORE, s__ASKED_BEFORE);
    pyrt::xrefs::reg_access("knowledge.measures", "_BAKED_WITH_COFFEE", g__BAKED_WITH_COFFEE, s__BAKED_WITH_COFFEE);
    pyrt::xrefs::reg_access("knowledge.measures", "_COFFEE_CROWD", g__COFFEE_CROWD, s__COFFEE_CROWD);
    pyrt::xrefs::reg_access("knowledge.measures", "_COFFEE_METHOD", g__COFFEE_METHOD, s__COFFEE_METHOD);
    pyrt::xrefs::reg_access("knowledge.measures", "_COFFEE_RATIO", g__COFFEE_RATIO, s__COFFEE_RATIO);
    pyrt::xrefs::reg_access("knowledge.measures", "_COFFEE_TIME", g__COFFEE_TIME, s__COFFEE_TIME);
    pyrt::xrefs::reg_access("knowledge.measures", "_COOKING_WORDS", g__COOKING_WORDS, s__COOKING_WORDS);
    pyrt::xrefs::reg_access("knowledge.measures", "_COUNT_ASK", g__COUNT_ASK, s__COUNT_ASK);
    pyrt::xrefs::reg_access("knowledge.measures", "_COUNT_NOT_COUNTING", g__COUNT_NOT_COUNTING, s__COUNT_NOT_COUNTING);
    pyrt::xrefs::reg_access("knowledge.measures", "_DRESS_IS_A_GARMENT", g__DRESS_IS_A_GARMENT, s__DRESS_IS_A_GARMENT);
    pyrt::xrefs::reg_access("knowledge.measures", "_NOT_A_RECIPE", g__NOT_A_RECIPE, s__NOT_A_RECIPE);
    pyrt::xrefs::reg_access("knowledge.measures", "_NOT_PRICE", g__NOT_PRICE, s__NOT_PRICE);
    pyrt::xrefs::reg_access("knowledge.measures", "_ONLY_MADE", g__ONLY_MADE, s__ONLY_MADE);
    pyrt::xrefs::reg_access("knowledge.measures", "_PORTION", g__PORTION, s__PORTION);
    pyrt::xrefs::reg_access("knowledge.measures", "_RECIPE_ASK", g__RECIPE_ASK, s__RECIPE_ASK);
}
