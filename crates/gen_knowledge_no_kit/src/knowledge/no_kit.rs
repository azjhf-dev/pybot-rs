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
thread_local! { static G_NOT_A_SHORTAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_KIT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAND_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TOOL_IS_STILL_MISSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WITHOUT_A_MANNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_NOT_A_SHORTAGE() -> Py { ensure_init(); G_NOT_A_SHORTAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SHORTAGE(v: Py) { ensure_init(); G_NOT_A_SHORTAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_KIT_ASK() -> Py { ensure_init(); G_NO_KIT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_KIT_ASK(v: Py) { ensure_init(); G_NO_KIT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_STAND_IN() -> Py { ensure_init(); G_STAND_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAND_IN(v: Py) { ensure_init(); G_STAND_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_SWAP_ASK() -> Py { ensure_init(); G_SWAP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWAP_ASK(v: Py) { ensure_init(); G_SWAP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TOOL_IS_STILL_MISSING() -> Py { ensure_init(); G__A_TOOL_IS_STILL_MISSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TOOL_IS_STILL_MISSING(v: Py) { ensure_init(); G__A_TOOL_IS_STILL_MISSING.with(|c| *c.borrow_mut() = v); }
pub fn g__WITHOUT_A_MANNER() -> Py { ensure_init(); G__WITHOUT_A_MANNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WITHOUT_A_MANNER(v: Py) { ensure_init(); G__WITHOUT_A_MANNER.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The kit you haven't got: what to use instead of the thing that is not in the drawer.\n\nThe answers lane owns this domain. A cook or a householder who has started a job and found the\ntool missing asks one thing in a dozen shapes - \"i dont have a colander\", \"what if i only have\nplain flour\", \"no sieve what can i do\", \"what can i use instead of a tin opener\", \"can i make do\nwithout a whisk\" - so the shape of the ask is one shared tuple (NO_KIT_ASK below) and the thing\nthat is missing is the row's own key.\n\nWhy the rows are keyed on the thing and not on the job: the answer to \"no colander\" is the same\nanswer while the pasta, the potatoes or the berries are in the water, so the words in the reply\nspeak about the tool's job rather than about one dish. That also keeps the table small: one line\nper tool answers every dish it is used for.\n\nWhat must not reach the table, and is in NOT_A_SHORTAGE:\n\n* a tool that is broken rather than missing - \"my hoover has stopped working\" is a repair\n  question, and telling somebody to sweep instead is not the answer they asked for;\n* a tool that is being cleaned, stored, bought or chosen - \"which hoover should i buy\", \"how do\n  i clean a mop\";\n* a question about the thing itself - \"how long do you boil an egg in a colander\" is not a\n  thing, but \"what is a colander for\" is, and the explanation is a different answer.\n\nNone of the rows is a bodge that trades a small risk for a real one: the ladder row says to\nborrow a ladder rather than stand on a chair, because a fall is not an everyday inconvenience.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_SWAP_ASK(pyrt::xrefs::xget("knowledge.swaps", "SWAP_ASK"));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink"), py_str("home-and-diy")]));
    fn __data1() -> Py { py_tuple(vec![py_str("dont have"), py_str("don't have"), py_str("havent got"), py_str("haven't got"), py_str("havent any"), py_str("haven't any"), py_str("dont own"), py_str("don't own"), py_str("without"), py_str("no"), py_str("none"), py_str("only have"), py_str("only got"), py_str("just have"), py_str("just got"), py_str("all i have"), py_str("all ive got"), py_str("all i've got"), py_str("left with"), py_str("nothing but"), py_str("missing"), py_str("lost"), py_str("make do"), py_str("make-do"), py_str("improvise"), py_str("bodge"), py_str("borrow"), py_str("lend"), py_str("what can i use"), py_str("what do i use"), py_str("what should i use"), py_str("what else can i use"), py_str("anything else"), py_str("something else"), py_str("what else"), py_str("alternative"), py_str("replacement"), py_str("substitute"), py_str("instead"), py_str("swap"), py_str("replace"), py_str("ran out"), py_str("run out"), py_str("out of")]) }
    s_NO_KIT_ASK(op_add(&pyrt::xrefs::xget("knowledge.swaps", "SWAP_ASK"), &__data1()));
    fn __data2() -> Py { py_tuple(vec![py_str("broken"), py_str("broke"), py_str("not working"), py_str("stopped working"), py_str("stopping working"), py_str("wont work"), py_str("won't work"), py_str("doesnt work"), py_str("doesn't work"), py_str("repair"), py_str("fix"), py_str("fixing"), py_str("mend"), py_str("mending"), py_str("faulty"), py_str("spare part"), py_str("warranty"), py_str("guarantee"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("which one"), py_str("best"), py_str("cheapest"), py_str("recommend"), py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washing"), py_str("store"), py_str("storing"), py_str("storage"), py_str("limescale"), py_str("furred"), py_str("chalky"), py_str("full of scale"), py_str("scaled up"), py_str("build up of scale"), py_str("buildup of scale"), py_str("scale in the"), py_str("scale on the"), py_str("what is"), py_str("what are"), py_str("whats the"), py_str("what's the"), py_str("what is a"), py_str("whats a"), py_str("what's a"), py_str("how do i use"), py_str("how to use"), py_str("paint"), py_str("paints"), py_str("gloss"), py_str("emulsion"), py_str("varnish"), py_str("primer"), py_str("undercoat"), py_str("tip"), py_str("recycling"), py_str("dump"), py_str("skip"), py_str("waste"), py_str("sharpen"), py_str("reviews"), py_str("review"), py_str("price"), py_str("out of the"), py_str("oven"), py_str("ovens"), py_str("microwave"), py_str("microwaves"), py_str("slow cooker"), py_str("rice cooker"), py_str("pressure cooker"), py_str("air fryer"), py_str("cooker hood"), py_str("out of the fridge"), py_str("out of the freezer"), py_str("out of the oven"), py_str("out of the pan"), py_str("out of the window"), py_str("out of the cupboard"), py_str("temperature"), py_str("temperatures"), py_str("fever"), py_str("feverish"), py_str("feverishness")]) }
    s_NOT_A_SHORTAGE(__data2());
    fn __data3() -> Py { py_tuple(vec![py_tuple(vec![py_str("colander"), py_tuple(vec![py_str("colanders"), py_str("strainer"), py_str("strainers")]), py_str("A clean tea towel, or the pan's own lid held a little open over the sink: pour slowly and the water runs out between the two. A sieve over a bowl does it for rice and small pasta, and a tray of ice in a bowl of water stops it cooking on if you want the pasta cold.")]), py_tuple(vec![py_str("sieve"), py_tuple(vec![py_str("sieves"), py_str("sifter"), py_str("sifters")]), py_str("A fine colander or a tea strainer does it, and if you have neither, whisk the flour round a wide bowl for half a minute: the whisk breaks the lumps and lets the air in the same way sifting does.")]), py_tuple(vec![py_str("whisk"), py_tuple(vec![py_str("whisks"), py_str("balloon whisk"), py_str("hand whisk"), py_str("electric whisk")]), py_str("A fork beats eggs and cream, just slower: keep the bowl deep and the wrist loose, and for cream chill both bowl and cream first. A clean jam jar with the lid on will whip a small amount of cream in about a minute of shaking.")]), py_tuple(vec![py_str("rolling pin"), py_tuple(vec![py_str("rolling pins")]), py_str("Any smooth heavy cylinder does it: a wine bottle, a tall glass or a cold milk bottle, all of them dusted with flour. Chill the bottle first and the pastry stays cold, which keeps it short.")]), py_tuple(vec![py_str("grater"), py_tuple(vec![py_str("graters"), py_str("box grater"), py_str("cheese grater")]), py_str("A vegetable peeler makes shavings of hard cheese and chocolate, and a knife does for anything going into a sauce. For parmesan or zest, a fine Microplane is the tool worth buying; until then, chop it as small as you can and it melts the same.")]), py_tuple(vec![py_str("blender"), py_tuple(vec![py_str("blenders"), py_str("liquidiser"), py_str("liquidizer")]), py_str("A stick blender in a deep jug does most of it, and a food processor does the rest. A masher then a sieve gets you a smooth soup without either, and a jar with a tight lid shaken hard will dress a small amount of sauce.")]), py_tuple(vec![py_str("food processor"), py_tuple(vec![py_str("food processors"), py_str("processor"), py_str("magimix")]), py_str("A stick blender for anything wet, a box grater for anything hard, and a knife for anything rough. Chopping an onion small by hand takes two minutes; a blender turns it to water, so keep the blade out of it.")]), py_tuple(vec![py_str("measuring jug"), py_tuple(vec![py_str("measuring jugs"), py_str("jug"), py_str("measuring cup")]), py_str("A mug is about 250 to 300 ml, and a tablespoon is 15 ml and a teaspoon 5. Anything you would weigh, weigh in grams instead: for water and milk a millilitre is a gram, so 200 ml is 200g on the scales.")]), py_tuple(vec![py_str("tin opener"), py_tuple(vec![py_str("tin openers"), py_str("can opener"), py_str("can openers")]), py_str("For a ring-pull, slide a spoon handle under the ring and lever it up. For a plain tin, the back of a heavy spoon or a stiff knife pressed down on the rim and worked round the top will open it - slowly, and the edges are sharp, so hold it with a cloth.")]), py_tuple(vec![py_str("corkscrew"), py_tuple(vec![py_str("corkscrews"), py_str("bottle opener"), py_str("bottle openers")]), py_str("For a cork, push it in with the handle of a wooden spoon and then pour through a small sieve, or put the bottle in a shoe and tap the base against a wall (the cork walks out). For crown caps, a coin or the back of a lighter under the edge does it.")]), py_tuple(vec![py_str("potato masher"), py_tuple(vec![py_str("mashers"), py_str("potato mashers")]), py_str("A fork for anything soft, and the end of a wooden spoon for potatoes. A ricer or a sieve and the back of a spoon gives a smoother mash than a masher ever does, which is what you want for gnocchi.")]), py_tuple(vec![py_str("cake tin"), py_tuple(vec![py_str("cake tins"), py_str("round tin"), py_str("springform tin"), py_str("tins")]), py_str("A roasting tin lined with paper works: bake it as a traybake at the same temperature, checking five minutes earlier, and cut it into squares. A deep frying pan with a lid will bake a shallow cake on the hob if the lid goes on and the heat stays low.")]), py_tuple(vec![py_str("loaf tin"), py_tuple(vec![py_str("loaf tins"), py_str("bread tin")]), py_str("A roasting tin or a deep ovenproof saucepan lined with baking paper. Free-standing bread bakes perfectly well on a lined tray; the tin is there for the shape, and a round loaf tastes the same.")]), py_tuple(vec![py_str("baking parchment"), py_tuple(vec![py_str("parchment"), py_str("greaseproof paper"), py_str("baking paper"), py_str("silicon paper")]), py_str("Grease the tin well with butter and dust it with flour, or use a silicone mat, or foil brushed with oil. Parchment and greaseproof paper are not the same thing: greaseproof sticks to a cake, so grease it again on top if that is all you have.")]), py_tuple(vec![py_str("cling film"), py_tuple(vec![py_str("clingfilm"), py_str("plastic wrap"), py_str("saran wrap")]), py_str("A plate over the bowl, or a clean damp tea towel, keeps a surface from drying. For dough left to rise, an upturned bowl over the whole thing is better than film anyway.")]), py_tuple(vec![py_str("muslin"), py_tuple(vec![py_str("muslins"), py_str("cheesecloth"), py_str("jelly bag")]), py_str("A clean tea towel, a new J-cloth or a pair of clean tights works for jelly and for straining yoghurt. Scalding the cloth first with boiling water keeps it from tasting of soap, and hang it over a bowl with the corners tied to a cupboard handle.")]), py_tuple(vec![py_str("piping bag"), py_tuple(vec![py_str("piping bags"), py_str("icing bag")]), py_str("A freezer bag with a corner snipped off, or a sheet of baking paper rolled into a cone and taped. A good zip-lock bag will take a star nozzle if you cut the corner to its width.")]), py_tuple(vec![py_str("skewers"), py_tuple(vec![py_str("skewer"), py_str("kebab sticks"), py_str("bamboo skewers")]), py_str("Rosemary stems stripped of their leaves make fine skewers for lamb and chicken, and metal kebab sticks or a cake tester do the testing job. To test a cake, a knife blade or a strand of dry spaghetti works as well as a skewer.")]), py_tuple(vec![py_str("thermometer"), py_tuple(vec![py_str("thermometers"), py_str("meat thermometer"), py_str("sugar thermometer")]), py_str("For meat, press the thickest part with a finger: springy and firm is done, soft is not. A knife pushed in and pulled out shows the juice running clear for chicken, and for sugar the old test is a drop in cold water - a soft ball for fudge, a hard crack for toffee.")]), py_tuple(vec![py_str("funnel"), py_tuple(vec![py_str("funnels")]), py_str("A corner of baking paper, or the top half of a plastic bottle cut off and turned upside down. For jam into a jar, a jug with a spout and a steady hand beats both.")]), py_tuple(vec![py_str("mortar and pestle"), py_tuple(vec![py_str("pestle and mortar"), py_str("mortar"), py_str("pestle")]), py_str("A bowl and the end of a rolling pin crushes spices, or put them in a bag and roll over it with a tin. Whole spices toasted in a dry pan first crush far more easily, whatever you crush them with.")]), py_tuple(vec![py_str("slotted spoon"), py_tuple(vec![py_str("slotted spoons")]), py_str("A fork, or a normal spoon held back against the side of the pan as you lift, drains the same. A fish slice with holes does the job for anything bigger than a pea.")]), py_tuple(vec![py_str("tongs"), py_tuple(vec![py_str("tongs"), py_str("tongs")]), py_str("Two forks back to back, or a spatula and a spoon. Chopsticks are the best stand-in there is for turning small things in a hot pan.")]), py_tuple(vec![py_str("pastry brush"), py_tuple(vec![py_str("pastry brushes"), py_str("basting brush")]), py_str("Your fingertips for milk or egg wash, the back of a spoon for oil, and a scrunched piece of kitchen paper for glazing a pie. None of them shed bristles, which is the one thing the brush is for.")]), py_tuple(vec![py_str("fridge"), py_tuple(vec![py_str("fridges"), py_str("refrigerator"), py_str("refrigerators")]), py_str("Buy little and often and keep what has to stay cold in the coolest place you have: a lidded box in the garage, a bucket of cold water with a cloth over it for milk, and a windowsill on the shady side in winter. Cooked food is the one thing not to push - eat it the day you make it, and keep the shopping to what fits in one bag.")]), py_tuple(vec![py_str("freezer"), py_tuple(vec![py_str("freezers"), py_str("freezer compartment")]), py_str("Cook for the day rather than the week and keep nothing back: without freezing, leftovers are good for two days in the coldest spot you have and then they go. A cool box with ice packs buys you a day for meat, and a bag of ice in a bowl does for chilling a drink.")]), py_tuple(vec![py_str("kettle"), py_tuple(vec![py_str("kettles"), py_str("electric kettle")]), py_str("A saucepan of water on the hob boils in about the same time, and a microwave does a mug in ninety seconds. For tea, pour the pan water while it is still at a rolling boil; a thermos keeps a litre hot for hours if you want it ready.")]), py_tuple(vec![py_str("hob"), py_tuple(vec![py_str("hobs"), py_str("stove"), py_str("stovetop")]), py_str("A hob alone cooks more than people think: pasta, rice, stews, curries and anything in a covered pan, plus a great deal of fish and veg. What it cannot do is brown a crust, so finish under the grill if there is one - a tray bake is the other way to get an oven-ish result with no oven.")]), py_tuple(vec![py_str("plunger"), py_tuple(vec![py_str("plungers"), py_str("sink plunger"), py_str("toilet plunger")]), py_str("Fill the bowl with enough water to cover the cup and use the flat of your hand, or a flat-bottomed cup held upside down, in short hard pushes over the plughole. Seal the overflow with a wet cloth first or the push escapes up it; if the water is not draining at all, go straight to the trap underneath with a bucket.")]), py_tuple(vec![py_str("mop"), py_tuple(vec![py_str("mops"), py_str("floor mop")]), py_str("A cloth and a bucket: work backwards to the door, wringing as you go, and a dry towel under your feet dries the last of it. On a small floor a sponge and a bowl is quicker than a mop anyway.")]), py_tuple(vec![py_str("hoover"), py_tuple(vec![py_str("hoovers"), py_str("vacuum"), py_str("vacuum cleaner"), py_str("vacuums")]), py_str("A broom or a stiff brush for a hard floor, and a damp rubber glove or a damp cloth for hair on carpet - it gathers in one swipe. A lint roller lifts crumbs and pet hair off a sofa or a stair carpet.")]), py_tuple(vec![py_str("stepladder"), py_tuple(vec![py_str("stepladders"), py_str("step ladder"), py_str("ladder"), py_str("ladders")]), py_str("Borrow a proper ladder rather than stand on a chair, a bucket or a table: a fall from kitchen-counter height is how most DIY injuries happen, and the ceiling can wait. A neighbour, a friend or a hire shop for the afternoon is the cheap version.")]), py_tuple(vec![py_str("drill"), py_tuple(vec![py_str("drills"), py_str("power drill"), py_str("cordless drill")]), py_str("Self-adhesive hooks take anything light, and a bradawl or a strong nail and a hammer starts a hole in plasterboard and soft wood. For masonry, borrow a drill: a nail in brick either bends or takes a lump of the wall with it.")]), py_tuple(vec![py_str("hammer"), py_tuple(vec![py_str("hammers"), py_str("mallet"), py_str("mallets")]), py_str("A block of scrap wood or the heel of a heavy shoe taps in panel pins and small tacks without marking the surface. For anything bigger than that, wait, or borrow one: a saucepan is heavier than it looks and dents both the pan and the wall.")]), py_tuple(vec![py_str("spirit level"), py_tuple(vec![py_str("level"), py_str("levels"), py_str("bubble level")]), py_str("A phone has a level in it, and a clear bottle half full of water on its side reads the same way: the bubble sits between the two marks. For a shelf, measure down from the ceiling at both ends - a shelf that matches the ceiling looks level even when it isn't.")]), py_tuple(vec![py_str("stud finder"), py_tuple(vec![py_str("stud finders"), py_str("stud detector")]), py_str("Knock along the wall with a knuckle: a hollow tap is plasterboard with nothing behind it and a dull thud is a stud. A small magnet run across the wall finds the screws holding the board, and the stud is either side of them.")]), py_tuple(vec![py_str("garden strimmer"), py_tuple(vec![py_str("strimmer"), py_str("strimmers"), py_str("weed whacker"), py_str("weed trimmer")]), py_str("Shears, or a pair of long-handled edging shears, cut the same grass at the edges and cost nothing to run. Cut little and often rather than letting it get long - long grass wraps round any machine, including the borrowed one.")])]) }
    s_STAND_IN(__data3());
    s__WITHOUT_A_MANNER(stdlib_call("re", "compile", &[py_str("\\bwithout\\s+(?:\\w+ly\\s+)?([a-z]+ing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TOOL_IS_STILL_MISSING(call_builtin("frozenset", &[py_tuple(vec![py_str("using"), py_str("having"), py_str("needing"), py_str("buying"), py_str("owning"), py_str("spending"), py_str("paying"), py_str("borrowing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("no-kit")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STAND_IN()) { let mut row: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(1))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g_NO_KIT_ASK()])); __args.push(py_func("_no_kit", |args: &[Py]| _no_kit(args))); __kws.extend(vec![("avoid".to_string(), g_NOT_A_SHORTAGE()), ("guard".to_string(), py_func("_is_a_shortage", |args: &[Py]| _is_a_shortage(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line's words, in whatever shape the understanding engine handed them over.");
    words = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]);
    if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return mcall(&text.clone(), "split", &[]);
    Py::None
}
pub fn _the_word_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut row: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "row") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: row"))) };
    let _ = py_str("The name to say in the reply: the person's own word when they used a variant of it.");
    return geti(&row.clone(), &Py::Int(0));
    Py::None
}
pub fn _no_kit(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What stands in for the tool the line is missing, or \"\" when it names none.\n\n    The longest name wins, the way a cook would read it: \"potato masher\" is the masher and not\n    a potato, and a \"loaf tin\" is the tin and not the bread. The reply names the tool back to the\n    person only when it is a one-word tool, because \"a clean tea towel, or the pan's own lid\" is\n    the answer to \"no colander\" without repeating the word.\n    ");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    words = call_builtin("set", &[_words(&[reading.clone()])]).unwrap_or_else(|e| exc::raise(e));
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_STAND_IN());
    for __item in __iter1 {
    row = __item.clone();
    let __iter2 = py_iter_or_raise(&op_add(&py_tuple(vec![geti(&row.clone(), &Py::Int(0))]), &geti(&row.clone(), &Py::Int(1))));
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&name.clone(), &py_str(" ")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &name.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&best.clone(), &Py::Int(0))) } } } }) {
    best = py_tuple(vec![call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), row.clone()]);
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &name.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&best.clone(), &Py::Int(0))) } } } }) {
    best = py_tuple(vec![call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), row.clone()]);
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&best.clone(), &Py::None))) {
    return py_str("");
    }
    return geti(&geti(&best.clone(), &Py::Int(1)), &Py::Int(2));
    Py::None
}
pub fn _is_a_shortage(args: &[Py]) -> Py {
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
    let mut manners: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about a tool that is not there, so the table may answer it.");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    manners = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__WITHOUT_A_MANNER(), "finditer", &[text.clone()])) { let mut match_: Py = __item.clone(); __out.push(mcall(&match_.clone(), "group", &[Py::Int(1)])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&manners.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&manners.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g__A_TOOL_IS_STILL_MISSING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.no_kit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "NOT_A_SHORTAGE" => Some(g_NOT_A_SHORTAGE()),
            "NO_KIT_ASK" => Some(g_NO_KIT_ASK()),
            "STAND_IN" => Some(g_STAND_IN()),
            "SWAP_ASK" => Some(g_SWAP_ASK()),
            "_A_TOOL_IS_STILL_MISSING" => Some(g__A_TOOL_IS_STILL_MISSING()),
            "_WITHOUT_A_MANNER" => Some(g__WITHOUT_A_MANNER()),
            "_is_a_shortage" => Some(py_func("_is_a_shortage", |args: &[Py]| _is_a_shortage(args))),
            "_no_kit" => Some(py_func("_no_kit", |args: &[Py]| _no_kit(args))),
            "_the_word_for" => Some(py_func("_the_word_for", |args: &[Py]| _the_word_for(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "NOT_A_SHORTAGE" => s_NOT_A_SHORTAGE(v),
            "NO_KIT_ASK" => s_NO_KIT_ASK(v),
            "STAND_IN" => s_STAND_IN(v),
            "SWAP_ASK" => s_SWAP_ASK(v),
            "_A_TOOL_IS_STILL_MISSING" => s__A_TOOL_IS_STILL_MISSING(v),
            "_WITHOUT_A_MANNER" => s__WITHOUT_A_MANNER(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.no_kit", module_obj);
    pyrt::xrefs::reg_fn("knowledge.no_kit", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("knowledge.no_kit", "_the_word_for", py_func("_the_word_for", |args: &[Py]| _the_word_for(args)));
    pyrt::xrefs::reg_fn("knowledge.no_kit", "_no_kit", py_func("_no_kit", |args: &[Py]| _no_kit(args)));
    pyrt::xrefs::reg_fn("knowledge.no_kit", "_is_a_shortage", py_func("_is_a_shortage", |args: &[Py]| _is_a_shortage(args)));
    pyrt::xrefs::reg_access("knowledge.no_kit", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.no_kit", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.no_kit", "NOT_A_SHORTAGE", g_NOT_A_SHORTAGE, s_NOT_A_SHORTAGE);
    pyrt::xrefs::reg_access("knowledge.no_kit", "NO_KIT_ASK", g_NO_KIT_ASK, s_NO_KIT_ASK);
    pyrt::xrefs::reg_access("knowledge.no_kit", "STAND_IN", g_STAND_IN, s_STAND_IN);
    pyrt::xrefs::reg_access("knowledge.no_kit", "SWAP_ASK", g_SWAP_ASK, s_SWAP_ASK);
    pyrt::xrefs::reg_access("knowledge.no_kit", "_A_TOOL_IS_STILL_MISSING", g__A_TOOL_IS_STILL_MISSING, s__A_TOOL_IS_STILL_MISSING);
    pyrt::xrefs::reg_access("knowledge.no_kit", "_WITHOUT_A_MANNER", g__WITHOUT_A_MANNER, s__WITHOUT_A_MANNER);
    pyrt::xrefs::reg_access("knowledge.no_kit", "entry", g_entry, s_entry);
}
