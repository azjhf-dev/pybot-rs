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
thread_local! { static G_KIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ML_PER_MUG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_WHAT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BLANK_STUFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BURNT_EATING_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BURNT_EATING_SAFETY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BURNT_ON_A_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BURNT_PAN_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUPS_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FULL_KETTLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FULL_KETTLE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIT_DAMAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_BURNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_CLEAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POT_SPLASH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REMOVAL_FROM_A_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VESSEL_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__names: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__slug: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__what: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_name: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_what: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_KIT_WORDS() -> Py { ensure_init(); G_KIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIT_WORDS(v: Py) { ensure_init(); G_KIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ML_PER_MUG() -> Py { ensure_init(); G_ML_PER_MUG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ML_PER_MUG(v: Py) { ensure_init(); G_ML_PER_MUG.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_WHAT_IS() -> Py { ensure_init(); G_NOT_A_WHAT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_WHAT_IS(v: Py) { ensure_init(); G_NOT_A_WHAT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IS_ASK() -> Py { ensure_init(); G_WHAT_IS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IS_ASK(v: Py) { ensure_init(); G_WHAT_IS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PAN() -> Py { ensure_init(); G__A_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PAN(v: Py) { ensure_init(); G__A_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g__BLANK_STUFF() -> Py { ensure_init(); G__BLANK_STUFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BLANK_STUFF(v: Py) { ensure_init(); G__BLANK_STUFF.with(|c| *c.borrow_mut() = v); }
pub fn g__BURNT_EATING_REPLY() -> Py { ensure_init(); G__BURNT_EATING_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BURNT_EATING_REPLY(v: Py) { ensure_init(); G__BURNT_EATING_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__BURNT_EATING_SAFETY() -> Py { ensure_init(); G__BURNT_EATING_SAFETY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BURNT_EATING_SAFETY(v: Py) { ensure_init(); G__BURNT_EATING_SAFETY.with(|c| *c.borrow_mut() = v); }
pub fn g__BURNT_ON_A_PAN() -> Py { ensure_init(); G__BURNT_ON_A_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BURNT_ON_A_PAN(v: Py) { ensure_init(); G__BURNT_ON_A_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g__BURNT_PAN_REPLY() -> Py { ensure_init(); G__BURNT_PAN_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BURNT_PAN_REPLY(v: Py) { ensure_init(); G__BURNT_PAN_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORDS() -> Py { ensure_init(); G__COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORDS(v: Py) { ensure_init(); G__COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CUPS_SAID() -> Py { ensure_init(); G__CUPS_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUPS_SAID(v: Py) { ensure_init(); G__CUPS_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD() -> Py { ensure_init(); G__FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD(v: Py) { ensure_init(); G__FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__FULL_KETTLE() -> Py { ensure_init(); G__FULL_KETTLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FULL_KETTLE(v: Py) { ensure_init(); G__FULL_KETTLE.with(|c| *c.borrow_mut() = v); }
pub fn g__FULL_KETTLE_ASK() -> Py { ensure_init(); G__FULL_KETTLE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FULL_KETTLE_ASK(v: Py) { ensure_init(); G__FULL_KETTLE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__KIT_DAMAGE() -> Py { ensure_init(); G__KIT_DAMAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIT_DAMAGE(v: Py) { ensure_init(); G__KIT_DAMAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__KIT_THING() -> Py { ensure_init(); G__KIT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIT_THING(v: Py) { ensure_init(); G__KIT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PAN() -> Py { ensure_init(); G__NOT_A_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PAN(v: Py) { ensure_init(); G__NOT_A_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_BURNT() -> Py { ensure_init(); G__PAN_BURNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_BURNT(v: Py) { ensure_init(); G__PAN_BURNT.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_CLEAN() -> Py { ensure_init(); G__PAN_CLEAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_CLEAN(v: Py) { ensure_init(); G__PAN_CLEAN.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_DISH() -> Py { ensure_init(); G__PAN_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_DISH(v: Py) { ensure_init(); G__PAN_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_SOMETHING() -> Py { ensure_init(); G__PAN_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_SOMETHING(v: Py) { ensure_init(); G__PAN_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_STUCK() -> Py { ensure_init(); G__PAN_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_STUCK(v: Py) { ensure_init(); G__PAN_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g__POT_SPLASH() -> Py { ensure_init(); G__POT_SPLASH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POT_SPLASH(v: Py) { ensure_init(); G__POT_SPLASH.with(|c| *c.borrow_mut() = v); }
pub fn g__REMOVAL_FROM_A_PAN() -> Py { ensure_init(); G__REMOVAL_FROM_A_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REMOVAL_FROM_A_PAN(v: Py) { ensure_init(); G__REMOVAL_FROM_A_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g__VESSEL_PHRASES() -> Py { ensure_init(); G__VESSEL_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VESSEL_PHRASES(v: Py) { ensure_init(); G__VESSEL_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g__names() -> Py { ensure_init(); G__names.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__names(v: Py) { ensure_init(); G__names.with(|c| *c.borrow_mut() = v); }
pub fn g__slug() -> Py { ensure_init(); G__slug.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__slug(v: Py) { ensure_init(); G__slug.with(|c| *c.borrow_mut() = v); }
pub fn g__what() -> Py { ensure_init(); G__what.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__what(v: Py) { ensure_init(); G__what.with(|c| *c.borrow_mut() = v); }
pub fn g_name() -> Py { ensure_init(); G_name.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_name(v: Py) { ensure_init(); G_name.with(|c| *c.borrow_mut() = v); }
pub fn g_what() -> Py { ensure_init(); G_what.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_what(v: Py) { ensure_init(); G_what.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Kitchen kit: how much water in the kettle, and a pan or a tin with burnt food on it.\n\nTwo subjects that are about the thing rather than the food, and that share a fault: both were\nanswered badly by a neighbour. \"how much water in a kettle\" had no owner at all - the running-cost\ntable answers what a boil costs, not how full to fill it, and the brewing entry answers the leaves\n- so it was the honest shrug. A cake tin or a baking tray with burnt food on it was read as a burn\non a person (\"twenty minutes under cool running water\") or as a pan on the hob (\"simmer it for ten\nminutes\"), and neither is a job a tin can do.\n\nBoth answers are one fact each, used with what the line already says: a mug is about 300 ml, so\nthe count on the line sets the water, and a full 1.7-litre kettle is five or six mugs the other way\nround. Burnt-on food lifts in a soak, and the sentence is said for the thing that burnt on - a tin\nis told why not to scour it, a tray gets baking paper, a shelf gets the bath.\n\nThe same file holds what to do about a pan that is burnt on, or whose fat has smoked: the pan's\nsoak, the burnt dish's rescue and the reason a pan of oil smokes are all about the pan and what is\nin it, and the oil entry's key takes a bare \"burnt\" beside \"pan\", which is how a pan to clean was\nanswered with a lecture on smoke points. `_burnt_on_the_pan` is the line between the two.\n\nA domain of its own rather than a part of bot/knowledge/cooking.py, which is at the world's 1 MB\nfile cap: a domain is one file with ENTRIES (bot/knowledge/__init__.py), and the helpers below are\nthe ones `cooking` already uses (`_says`, `_words_and_text`, `_FOOD`, the pan and bake word lists\nand the vessel phrases), imported rather than copied.\n");
    s__BURNT_EATING_REPLY(crate::knowledge::cooking::g__BURNT_EATING_REPLY());
    s__BURNT_ON_A_PAN(crate::knowledge::cooking::g__BURNT_ON_A_PAN());
    s__FOOD(crate::knowledge::cooking::g__FOOD());
    s__PAN_DISH(crate::knowledge::cooking::g__PAN_DISH());
    s__REMOVAL_FROM_A_PAN(crate::knowledge::cooking::g__REMOVAL_FROM_A_PAN());
    s__VESSEL_PHRASES(crate::knowledge::cooking::g__VESSEL_PHRASES());
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink"), py_str("home-and-diy")]));
    s_ML_PER_MUG(Py::Int(300));
    s__POT_SPLASH(Py::Int(100));
    s__COUNT_WORDS(py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("a couple"), Py::Int(2)), (py_str("a couple of"), Py::Int(2)), (py_str("a few"), Py::Int(3)), (py_str("a few of"), Py::Int(3)), (py_str("couple"), Py::Int(2))]));
    s__CUPS_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(\\d{1,2}|a couple of|a few of|a couple|a few|twelve|eleven|ten|nine|eight|seven|six|five|four|three|two|one|an|a)\\s+(cups?|mugs?|cuppas?|people|persons?|of us)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__FULL_KETTLE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many\\s+(?:cups|mugs|cupfuls)\\b|\\bhow many\\b[^?.]{0,20}\\bkettle\\b|\\bkettle\\b[^?.]{0,20}\\bhow many\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__FULL_KETTLE(py_str("A full 1.7-litre kettle makes about five or six mugs of tea, since a mug is around 300 ml and you lose a little to steam. A 1-litre kettle does three. Boil only what you are going to drink and it is quicker and cheaper both."));
    s__BLANK_STUFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:food|foods|stuff|bits|bit|mess|residue|crud|gunk|grime|grease|leftovers|things|patches|marks|spots|remains|baking|baked on|burnt food|burnt bits)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BURNT_PAN_REPLY(py_str("Put an inch of water and two tablespoons of bicarbonate of soda in it and simmer for ten minutes, then lift the burnt layer with a wooden spoon: almost nothing needs scrubbing after that. Stainless steel and enamel can take a vinegar soak instead, but never scour a non-stick pan, since it takes the coating off for good."));
    s__PAN_CLEAN(op_add(&py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("cleaned"), py_str("scrub"), py_str("scour"), py_str("soak"), py_str("soaking"), py_str("wash"), py_str("washing"), py_str("comes off"), py_str("come off"), py_str("off the bottom"), py_str("off my pan"), py_str("off the pan"), py_str("off the inside"), py_str("off the base"), py_str("get it off"), py_str("gets it off"), py_str("still stuck"), py_str("stuck on"), py_str("wont come"), py_str("won't come"), py_str("off the tin"), py_str("off my tin"), py_str("off the tray"), py_str("off my tray"), py_str("off the sheet"), py_str("get the burnt off"), py_str("burnt off"), py_str("stained")]), &crate::knowledge::cooking::g__REMOVAL_FROM_A_PAN()));
    s__NOT_A_PAN(py_tuple(vec![py_str("tin"), py_str("tins"), py_str("cake tin"), py_str("loaf tin"), py_str("muffin tin"), py_str("baking tin"), py_str("roasting tin"), py_str("pie tin"), py_str("tart tin"), py_str("springform"), py_str("tray"), py_str("trays"), py_str("baking tray"), py_str("roasting tray"), py_str("baking sheet"), py_str("sheet"), py_str("sheets"), py_str("oven shelf"), py_str("shelf"), py_str("shelves"), py_str("rack"), py_str("mould"), py_str("molds"), py_str("mold")]));
    s__A_PAN(py_tuple(vec![py_str("pan"), py_str("pans"), py_str("pot"), py_str("pots"), py_str("saucepan"), py_str("saucepans"), py_str("wok"), py_str("woks"), py_str("skillet"), py_str("casserole"), py_str("griddle")]));
    s__PAN_STUCK(py_tuple(vec![py_str("stuck"), py_str("sticks"), py_str("sticking"), py_str("stick")]));
    s__PAN_SOMETHING(py_tuple(vec![py_str("something"), py_str("anything"), py_str("everything"), py_str("stuff"), py_str("bits"), py_str("whatever")]));
    s__PAN_BURNT(op_add(&py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("burn"), py_str("burning"), py_str("burns"), py_str("scorched"), py_str("scorching"), py_str("black"), py_str("blackened"), py_str("smoke"), py_str("smoking"), py_str("smoked"), py_str("layer"), py_str("stain"), py_str("stained"), py_str("charred")]), &g__PAN_CLEAN()));
    s__BURNT_EATING_SAFETY(py_tuple(vec![py_str("bad for"), py_str("bad for you"), py_str("bad for me"), py_str("harmful"), py_str("dangerous"), py_str("cancer"), py_str("carcinogen"), py_str("carcinogenic"), py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("safe to eat"), py_str("ok to eat"), py_str("okay to eat"), py_str("alright to eat"), py_str("can i eat"), py_str("can you eat"), py_str("should i eat"), py_str("still eat"), py_str("eat it"), py_str("is it safe"), py_str("is it ok"), py_str("is it okay"), py_str("is it bad"), py_str("healthy"), py_str("unhealthy"), py_str("does it matter")]));
    s__KIT_DAMAGE(py_tuple(vec![py_str("warped"), py_str("warping"), py_str("warp"), py_str("buckled"), py_str("buckling"), py_str("bent"), py_str("dented"), py_str("cracked"), py_str("crack"), py_str("cracks"), py_str("cracking"), py_str("split"), py_str("splits"), py_str("splitting"), py_str("snapped"), py_str("flaking"), py_str("peeling"), py_str("chipped"), py_str("chipping"), py_str("wobbly"), py_str("wobbles"), py_str("wobbling"), py_str("loose"), py_str("came off"), py_str("come off"), py_str("fell off"), py_str("fallen off"), py_str("rocks"), py_str("rocking"), py_str("spins"), py_str("spinning"), py_str("doesnt sit flat"), py_str("doesn't sit flat"), py_str("wont sit flat"), py_str("won't sit flat"), py_str("not flat"), py_str("bowed"), py_str("dished"), py_str("domed"), py_str("bulging"), py_str("bulged")]));
    s__KIT_THING(py_tuple(vec![py_str("pan"), py_str("pans"), py_str("frying pan"), py_str("saucepan"), py_str("saucepans"), py_str("wok"), py_str("skillet"), py_str("pot"), py_str("pots"), py_str("stock pot"), py_str("casserole"), py_str("casserole dish"), py_str("slow cooker"), py_str("slow cooker pot"), py_str("insert"), py_str("baking dish"), py_str("oven dish"), py_str("dish"), py_str("tin"), py_str("tins"), py_str("tray"), py_str("baking tray"), py_str("board"), py_str("chopping board"), py_str("cutting board"), py_str("wooden board"), py_str("kettle"), py_str("microwave plate"), py_str("turntable"), py_str("jug"), py_str("blender"), py_str("enamel"), py_str("coating"), py_str("non stick"), py_str("non-stick"), py_str("handle"), py_str("lid")]));
    s_WHAT_IS_ASK(py_tuple(vec![py_str("what is"), py_str("what are"), py_str("what's a"), py_str("what's an"), py_str("whats a"), py_str("whats an"), py_str("what does"), py_str("what do you use"), py_str("used for"), py_str("what for"), py_str("what is it for"), py_str("whats it for"), py_str("what's it for"), py_str("mean"), py_str("explain")]));
    s_NOT_A_WHAT_IS(py_tuple(vec![py_str("size"), py_str("what size"), py_str("how big"), py_str("how much"), py_str("how many"), py_str("how long"), py_str("price"), py_str("cost"), py_str("costs"), py_str("cheap"), py_str("cheapest"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("which"), py_str("best"), py_str("recommend"), py_str("worth"), py_str("broken"), py_str("broke"), py_str("not working"), py_str("repair"), py_str("fix"), py_str("mend"), py_str("rusty"), py_str("rust"), py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("instead"), py_str("substitute"), py_str("swap"), py_str("dont have"), py_str("don't have"), py_str("havent got"), py_str("haven't got"), py_str("without"), py_str("no"), py_str("i need a"), py_str("should i get"), py_str("shall i buy"), py_str("recipe"), py_str("recipes"), py_str("how do i use"), py_str("how to use"), py_str("how do i"), py_str("how to"), py_str("in the dishwasher"), py_str("dishwasher"), py_str("oven safe"), py_str("microwave"), py_str("think"), py_str("thought"), py_str("opinion"), py_str("good"), py_str("any good"), py_str("rate"), py_str("any cop"), py_str("what do i do"), py_str("what should i do"), py_str("help"), py_str("bite"), py_str("bites"), py_str("bitten"), py_str("sting"), py_str("stung"), py_str("plant"), py_str("plants"), py_str("crab"), py_str("tarantula"), py_str("black widow"), py_str("web"), py_str("webs"), py_str("mite"), py_str("mites"), py_str("daddy long legs"), py_str("huntsman"), py_str("money spider"), py_str("house spider"), py_str("garden spider"), py_str("houseplant"), py_str("houseplants")]));
    s_KIT_WORDS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("bundt tin"), py_str("bundt tins"), py_str("bundt pan"), py_str("bundt pans"), py_str("bundt")]), py_str("A bundt tin is the ring-shaped cake tin with fluted sides. The hole up the middle is why the cake bakes evenly - there is no cold, heavy middle to sit raw while the edge dries - and the flutes give more crust. The catch is the shape: butter and flour every groove, tapping the tin as you go, and let the cake sit ten minutes before turning it out, or a third of the crust stays behind. A standard recipe fills a 10-cup (2.5-litre) tin.")]), py_tuple(vec![py_tuple(vec![py_str("ricer"), py_str("ricers"), py_str("potato ricer"), py_str("potato ricers")]), py_str("A potato ricer is a press: you fill the hopper with cooked, dried potato and squeeze the handles, and the potato comes out through small holes in threads. That is what makes mash smooth without a machine - no lumps, and, unlike a blender or a food processor, it never overworks the starch, which is what turns mash gluey.")]), py_tuple(vec![py_tuple(vec![py_str("muslin"), py_str("muslins"), py_str("muslin cloth"), py_str("muslin cloths"), py_str("cheesecloth")]), py_str("Muslin is a loose woven cotton cloth, for straining and for holding things together: it lines a sieve to catch the fine bits in a stock or a jelly, wraps herbs and spices for a bouquet garni, and goes over a bowl of bread dough or a jug of milk as a cover that still lets air through. It is not the same as a tea towel - it is far looser, so liquid runs through it, and it can be boiled to clean it.")]), py_tuple(vec![py_tuple(vec![py_str("mezzaluna"), py_str("mezzalunas")]), py_str("A mezzaluna is a half-moon blade with a handle at each end, and you rock it back and forth across the board: for chopping herbs, nuts and garlic finely without a knife's stopping and starting. It is quick on a pile of parsley and hopeless on anything hard or wet.")]), py_tuple(vec![py_tuple(vec![py_str("zester"), py_str("zesters"), py_str("microplane"), py_str("microplanes"), py_str("fine grater")]), py_str("A zester takes the coloured outer peel off citrus without the white pith underneath, which is bitter. A microplane does the same job and also rasps hard cheese, nutmeg and garlic into fluff: one tool, a dozen uses. Take only the coloured layer - down to the pith is where the bitterness is.")]), py_tuple(vec![py_tuple(vec![py_str("proving basket"), py_str("proving baskets"), py_str("banneton"), py_str("bannetons"), py_str("proving basket liner")]), py_str("A proving basket - a banneton - is the bowl a shaped loaf sits in for its last rise. It holds the dough up as it spreads upwards instead of outwards, and its ridges leave the flour rings on the crust. Dust it heavily with flour (rice flour is best, it does not stick) or line it with a cloth; washed and wet, it goes mouldy.")]), py_tuple(vec![py_tuple(vec![py_str("trivet"), py_str("trivets"), py_str("pan stand"), py_str("pan stands")]), py_str("A trivet is the stand that keeps something hot off the table or the counter - a pan, a casserole, a slow cooker's pot. Wooden, cork, stone or metal, all it does is hold the heat away from the surface; a folded dry towel does the same job when the trivet is not there.")]), py_tuple(vec![py_tuple(vec![py_str("dutch oven"), py_str("dutch ovens"), py_str("cast iron pot"), py_str("cast iron casserole"), py_str("braiser"), py_str("braising pan")]), py_str("A dutch oven is a heavy lidded pot - cast iron, or cast aluminium with enamel on it - that goes from the hob to the oven, which is the whole point of it: brown the meat on the burner, put the lid on, and let the oven do the next two hours. The thick walls hold heat and spread it evenly, so it braises without scorching, and the tight lid keeps the steam in. The enamelled ones want no metal utensils; bare cast iron wants seasoning and drying on the hob before it goes away.")]), py_tuple(vec![py_tuple(vec![py_str("casserole dish"), py_str("casserole dishes"), py_str("oven dish"), py_str("ovenproof dish"), py_str("baking dish"), py_str("baking dishes")]), py_str("A casserole dish is the wide, deep, lidded dish for cooking in the oven - stews, braises, a gratin - and it is chosen by what it is made of: enamelled cast iron holds and spreads heat for a long braise, stoneware is cheaper and slower to heat, and glass lets you see the browning but cannot go under a grill that is close to it. It should say ovenproof and give a top temperature; anything without that line stays off the hob and out of a hot oven.")]), py_tuple(vec![py_tuple(vec![py_str("pestle and mortar"), py_str("mortar and pestle"), py_str("pestle"), py_str("pestles"), py_str("mortar"), py_str("mortars"), py_str("pestle and mortars")]), py_str("A pestle and mortar is a heavy bowl and a rounded club for crushing rather than cutting - spices, garlic, herbs, salt - which is why it gives a different, more fragrant result than a blade: the cell walls burst and the oils come out. Dry spices first, then anything wet, then wipe it out with salt and a cloth rather than washing it.")]), py_tuple(vec![py_tuple(vec![py_str("ramekin"), py_str("ramekins")]), py_str("A ramekin is a small straight-sided dish, for one portion of something baked or set - custard, a souffle, a fondant, potted shrimp, a dip on a plate. The straight sides matter: the contents slide out in one piece and a crust forms all the way up.")]), py_tuple(vec![py_tuple(vec![py_str("bench scraper"), py_str("bench scrapers"), py_str("dough scraper"), py_str("dough scrapers"), py_str("pastry scraper"), py_str("dough card")]), py_str("A bench scraper is a flat rectangle of metal with a handle - a pastry chef's third hand. It lifts a sticky dough off the counter, cuts it into portions, scrapes dried flour and dough off the bench in one pass, and carries chopped vegetables to the pan.")]), py_tuple(vec![py_tuple(vec![py_str("fish slice"), py_str("fish slices"), py_str("slotted turner"), py_str("turner")]), py_str("A fish slice is the wide, thin, usually slotted blade with a handle, for lifting things that fall apart - a whole fried fish, an egg, a slice of cake - and turning them without the corner of a spatula digging in. Thin and springy is what you want; a thick one is a burger press.")]), py_tuple(vec![py_tuple(vec![py_str("spider skimmer"), py_str("spider skimmers"), py_str("kitchen spider"), py_str("spider for chips"), py_str("skimmer"), py_str("skimmers"), py_str("spiders for chips")]), py_str("A spider is the wide, shallow wire-mesh basket on a long handle. It lifts food out of boiling water or hot oil - fried chips, blanched greens, a poached egg, a handful of pasta to test - and lets the water or oil run straight back into the pan, without dragging the pan off the heat.")]), py_tuple(vec![py_tuple(vec![py_str("skillet"), py_str("skillets")]), py_str("A skillet is a frying pan with straight, deeper sides and usually an ovenproof handle, built to take high heat: searing, frying, then going into the oven to finish. In Britain the word is used loosely for a heavy frying pan; the straight side is the part that matters, because it holds the fat and the food in.")]), py_tuple(vec![py_tuple(vec![py_str("slotted spoon"), py_str("slotted spoons")]), py_str("A slotted spoon has holes or slots in the bowl, for lifting the solid part out of a liquid and letting the liquid drain back: vegetables out of a pan, olives out of a jar, a poached egg out of the water, skimming the scum off a stock.")]), py_tuple(vec![py_tuple(vec![py_str("gratin dish"), py_str("gratin dishes")]), py_str("A gratin dish is a shallow, wide ovenproof dish - the opposite of a deep casserole. The point is the exposed surface: a shallow layer means more of the top browns and crisps, which is what a gratin is. A roasting tin will do at a push, if it is deep enough to hold the sauce.")]), py_tuple(vec![py_tuple(vec![py_str("springform tin"), py_str("springform tins"), py_str("spring form tin"), py_str("springform pan"), py_str("springform")]), py_str("A springform tin has a clip on the side: undo it and the ring lifts away from the cake, so a cheesecake or a torte goes out of the tin without being turned upside down. Line the base or the leaky ones drip, and run a knife round the inside before you release the clip, which stops a set top cracking.")]), py_tuple(vec![py_tuple(vec![py_str("piping bag"), py_str("piping bags"), py_str("icing bag")]), py_str("A piping bag is the cone you fill with icing, cream, choux or meringue mix and squeeze out through a nozzle, so it comes out in the shape you want - a rosette, a line, a swirl. A disposable bag plus a star nozzle covers most of it; a sandwich bag with the corner snipped off does for a swirl.")]), py_tuple(vec![py_tuple(vec![py_str("palette knife"), py_str("palette knives"), py_str("offset spatula")]), py_str("A palette knife is the long flexible blunt blade with no edge, for lifting, spreading and levelling rather than cutting: icing a cake flat, lifting a slice off the board, sliding under a fish to turn it. An offset one - the blade stepped down from the handle - is for spreading inside a tin.")]), py_tuple(vec![py_tuple(vec![py_str("sieve"), py_str("sieves"), py_str("fine sieve"), py_str("flour sieve")]), py_str("A sieve is the round mesh for flour, icing sugar, cocoa and custard: it breaks up lumps and puts air in, which is why a sponge recipe sifts. The mesh is fine, so it also strains small quantities of liquid and seeds; for a big pan of pasta you want a colander, whose holes are large enough for the water to leave in seconds.")]), py_tuple(vec![py_tuple(vec![py_str("stick blender"), py_str("stick blenders"), py_str("immersion blender"), py_str("hand blender"), py_str("hand blenders")]), py_str("A stick blender is the wand you put straight into the pan or jug: soup, sauce, a smoothie, all blended where they are, with nothing to pour and one thing to wash. Keep the blade under the surface before you switch on or it spatters, and do not use it on boiling soup in a deep pan without taking the pan off the heat first.")]), py_tuple(vec![py_tuple(vec![py_str("grater"), py_str("graters"), py_str("box grater"), py_str("cheese grater")]), py_str("A grater turns hard food into shreds: cheese, carrot, apple, potato, courgette, nutmeg. A box grater is the four-sided one - coarse for cheese and vegetables, fine for parmesan and zest, and a slicer on one side; a flat or rotary grater is for cheese at the table.")]), py_tuple(vec![py_tuple(vec![py_str("spatula"), py_str("spatulas")]), py_str("A spatula is the flat, flexible, blunt tool for scraping and folding - getting every bit of cake mix out of the bowl, folding egg whites in without knocking the air out, easing a fried egg off the pan. A silicone one will not scratch a non-stick pan; the metal sort that turns and lifts food is the fish slice.")]), py_tuple(vec![py_tuple(vec![py_str("ladle"), py_str("ladles")]), py_str("A ladle is the deep, long-handled spoon with a lip, for moving liquid in useful amounts: soup into bowls, stock into a jug, sauce over a plate. A 100ml ladle is about three tablespoons, which is a handy measure when a recipe says a ladleful.")]), py_tuple(vec![py_tuple(vec![py_str("saucepan"), py_str("saucepans")]), py_str("A saucepan is the deep, round pan with a long handle, for anything with liquid in it - sauces, rice, pasta, boiled eggs, milk. Size is the thing people get wrong: a small pan boils over and a big one scorches a small amount, so 16-18cm for one or two people and 20cm for a family.")]), py_tuple(vec![py_tuple(vec![py_str("roasting tin"), py_str("roasting tins"), py_str("roasting tray"), py_str("roasting dish")]), py_str("A roasting tin is the deep-sided metal tin that goes in the oven for joints, roast vegetables and traybakes. Metal, not glass, for high heat and for browning; leave a gap between the pieces so they roast rather than steam, and put the tin in the oven while it heats for crisper potatoes.")]), py_tuple(vec![py_tuple(vec![py_str("thermometer"), py_str("thermometers"), py_str("meat thermometer"), py_str("meat thermometers"), py_str("sugar thermometer"), py_str("jam thermometer"), py_str("food thermometer"), py_str("probe thermometer")]), py_str("A cooking thermometer takes the guesswork out of doneness: 75C in the middle of poultry, 71C for pork and minced meat, 63C for a beef joint eaten pink, and 104-105C for jam at setting point. Take the meat out of the oven to check it, or the reading climbs as the probe goes in, and leave it in for ten seconds rather than a moment.")]), py_tuple(vec![py_tuple(vec![py_str("kitchen scale"), py_str("kitchen scales"), py_str("weighing scales"), py_str("digital scales"), py_str("food scale")]), py_str("A kitchen scale weighs in grams, which is how most baking is written and the only way to be sure of flour, butter and sugar - a cup of flour varies by a third depending on how it is scooped. Put the bowl on, set it to zero, and add each thing in turn.")]), py_tuple(vec![py_tuple(vec![py_str("blender"), py_str("blenders"), py_str("jug blender")]), py_str("A jug blender is the tall jug with a blade in the base, for smoothies, soups, hummus and crushing ice: liquid in, smooth out. It needs enough liquid to keep the food moving, so a thick mix wants stirring or a little water; hot soup goes in a third at a time, with the lid held down, because steam lifts it off.")]), py_tuple(vec![py_tuple(vec![py_str("carving fork"), py_str("carving forks"), py_str("meat fork")]), py_str("A carving fork is the long two-pronged fork that holds the joint still while you cut it, so your fingers are not near the blade. The long prongs go in once, at the thick end, and the guard stops the knife sliding up towards your hand.")]), py_tuple(vec![py_tuple(vec![py_str("potato masher"), py_str("potato mashers"), py_str("masher"), py_str("mashers")]), py_str("A potato masher is the wavy wire or plate on a handle, for crushing cooked potato, swede, carrot or beans straight in the pan. It leaves a rougher mash than a ricer, which is fine for most things and wrong for a smooth one: add the butter and milk warm, and mash while the potato is hot.")]), py_tuple(vec![py_tuple(vec![py_str("cooling rack"), py_str("cooling racks"), py_str("wire rack"), py_str("wire racks"), py_str("cake rack")]), py_str("A cooling rack is the wire grid that lets air under a cake or a tray of biscuits as they cool, so the base does not go damp and soggy on the worktop. Leave the bake in its tin for ten minutes first, then move it to the rack; a rack upside down over the tin turns a cake out without breaking it.")]), py_tuple(vec![py_tuple(vec![py_str("stockpot"), py_str("stockpots"), py_str("stock pot"), py_str("stock pans")]), py_str("A stockpot is the tall, deep pan for a lot of liquid over a long time: stock, a big pan of pasta, a ham hock, a batch of soup. Tall and narrow is the point - less surface means less evaporation - which is the opposite of a wide saute pan.")]), py_tuple(vec![py_tuple(vec![py_str("saute pan"), py_str("saute pans"), py_str("sauté pan"), py_str("sauteuse")]), py_str("A saute pan has straight, fairly high sides and a wide flat base: enough floor to brown without crowding, enough wall to hold a sauce or a braise. It is the pan for a risotto, a curry or a chicken joint browned and then finished in the oven.")]), py_tuple(vec![py_tuple(vec![py_str("crepe pan"), py_str("crepe pans"), py_str("pancake pan")]), py_str("A crepe pan is a shallow pan with a very low lip and a flat base, so a ladle of batter can be swirled right to the edge instead of running up the side. A good non-stick frying pan of about 20cm does the same job; the thin lip is what makes it easier to get a palette knife under a crepe.")]), py_tuple(vec![py_tuple(vec![py_str("griddle"), py_str("griddles"), py_str("griddle pan"), py_str("flat griddle")]), py_str("A griddle is the thick flat plate - a pan, or a slab over the hob - for cooking with almost no fat and no side walls: pancakes, flatbreads, bacon, a steak. It gets very hot and stays hot, so things brown rather than steam; a ridged one leaves the dark lines and lets the fat run away.")]), py_tuple(vec![py_tuple(vec![py_str("pastry brush"), py_str("pastry brushes"), py_str("basting brush")]), py_str("A pastry brush is the small brush for glazing and greasing: beaten egg on a pie lid, milk on a bun, melted butter in a tin, a wash of syrup on a warm cake. Silicone takes the heat and washes clean; a bristle brush gives a finer finish but sheds, so keep it for cool jobs.")]), py_tuple(vec![py_tuple(vec![py_str("food mill"), py_str("food mills"), py_str("mouli"), py_str("moulinette")]), py_str("A food mill is the pan with a crank and a perforated disc underneath, for pureeing and sieving in one go: it pushes cooked apple, tomato or potato through the holes and leaves the skins and seeds behind. Swapping the disc sets how fine it comes out, which is why it makes smoother mash and seedless sauce without a blender.")]), py_tuple(vec![py_tuple(vec![py_str("colander"), py_str("colanders"), py_str("sieve colander")]), py_str("A colander is the bowl with holes in it, for draining the water off food - pasta, potatoes, rice, berries - which is quicker than lifting them out a spoonful at a time. Its holes are large, so a big pan of pasta water leaves in seconds and a custard runs straight through; the fine-mesh one for flour and icing sugar is a sieve. A metal colander takes boiling water straight from the pan, a plastic one is fine for cold rinsing but softens and stains under a full pan, and the ones with a single handle are for lifting it out of the sink.")]), py_tuple(vec![py_tuple(vec![py_str("mandoline"), py_str("mandolines"), py_str("mandolin slicer"), py_str("v slicer")]), py_str("A mandoline is a flat board with a blade set at an adjustable height, for slices of even thickness - 2mm potato for dauphinoise, cabbage for slaw - which a knife cannot match for speed. It is also the sharpest thing in most kitchens: use the hand guard, or a cut glove, every time, and stop while there is still a knuckle's width of vegetable left. Wash it straight after, since dried starch glues the blade in place and the next slice slips.")]), py_tuple(vec![py_tuple(vec![py_str("bain marie"), py_str("bain-marie"), py_str("water bath")]), py_str("A bain marie is a pan of simmering water with a bowl sitting in it, so what is in the bowl is heated by steam and never by flame: chocolate melts without seizing, custard and hollandaise thicken without curdling, and a sauce stays warm without catching. The bowl must not touch the water and the water must not boil hard - steam is gentle, a rolling boil splashes and scorches. A heatproof bowl over a saucepan does the same job, and a folded towel under the bowl stops it rattling.")]), py_tuple(vec![py_tuple(vec![py_str("balloon whisk"), py_str("balloon whisks"), py_str("whisk"), py_str("whisks"), py_str("wire whisk")]), py_str("A balloon whisk is the round, balloon-shaped wire one, for putting air in: egg whites to peaks, cream to soft folds, a batter smooth. Its wide loops move a lot of air, which a flat or a ball whisk does not, so it is the one for meringue; it is no good for a thick mixture - that is a wooden spoon's job. Use a big bowl and lift the whisk right out of the mixture each turn, and take the cream from the fridge cold.")]), py_tuple(vec![py_tuple(vec![py_str("pantry"), py_str("pantries"), py_str("larder"), py_str("larders")]), py_str("A pantry or larder is a cool, dark, dry cupboard for the food that keeps: tins, dried pasta and rice, flour, sugar, oil, spices. Cool and dry matter more than size - a shelf by a radiator or a sunny window turns flour and rice stale and brings weevils - so put it on an outside wall away from the cooker. Keep the oldest at the front of the shelf and the new shopping behind it, and write the date on anything you decant into a jar.")])]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tea-water")); __args.push(py_tuple(vec![py_tuple(vec![py_str("kettle"), py_str("kettles"), py_str("cups of tea"), py_str("mugs of tea"), py_str("mug of tea")]), py_tuple(vec![py_str("how much water"), py_str("much water"), py_str("how full"), py_str("how far to fill"), py_str("fill the kettle"), py_str("filling the kettle"), py_str("cups of water"), py_str("how much to fill"), py_str("full to fill"), py_str("how many cups"), py_str("how many mugs"), py_str("boil for"), py_str("how much in"), py_str("should i fill"), py_str("do i fill"), py_str("for two cups"), py_str("for three cups"), py_str("for four cups"), py_str("for five cups"), py_str("for six cups"), py_str("for one cup"), py_str("for a cup"), py_str("for two mugs"), py_str("for three mugs"), py_str("for four mugs"), py_str("for five mugs"), py_str("for six mugs"), py_str("for one mug"), py_str("for a mug"), py_str("for two people"), py_str("for three people"), py_str("for four people"), py_str("for six people"), py_str("for two of us"), py_str("for three of us"), py_str("for a couple"), py_str("for a few")])])); __args.push(g__tea_water()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how full"), py_str("how far"), py_str("how big"), py_str("should i"), py_str("do i need"), py_str("do i fill"), py_str("much water"), py_str("many cups"), py_str("many mugs"), py_str("fill"), py_str("full"), py_str("for one"), py_str("for a cup"), py_str("for a mug"), py_str("for two"), py_str("for three"), py_str("for four"), py_str("for five"), py_str("for six"), py_str("for a couple"), py_str("for a few"), py_str("for a crowd"), py_str("for the two of us"), py_str("for the three of us"), py_str("for the four of us")])), ("avoid".to_string(), py_tuple(vec![py_str("descale"), py_str("descaling"), py_str("descaled"), py_str("limescale"), py_str("scale"), py_str("scaled"), py_str("clean"), py_str("vinegar"), py_str("citric"), py_str("broken"), py_str("repair"), py_str("boil dry"), py_str("element"), py_str("filter"), py_str("hard water"), py_str("calorie"), py_str("calories"), py_str("caffeine"), py_str("sugar"), py_str("milk"), py_str("too many"), py_str("too much"), py_str("strong"), py_str("last"), py_str("lasts"), py_str("lifetime"), py_str("warranty"), py_str("keep warm"), py_str("switch off"), py_str("auto"), py_str("day"), py_str("days"), py_str("drink"), py_str("drinking"), py_str("drank"), py_str("had"), py_str("today"), py_str("protein"), py_str("carbs"), py_str("alcohol"), py_str("stain"), py_str("stains"), py_str("teeth"), py_str("how do i make"), py_str("steep"), py_str("brew"), py_str("how long"), py_str("how many minutes"), py_str("take to boil"), py_str("long to boil"), py_str("minutes"), py_str("cost"), py_str("costs"), py_str("price"), py_str("electricity"), py_str("energy"), py_str("buy"), py_str("buying"), py_str("best"), py_str("which kettle"), py_str("plastic"), py_str("taste"), py_str("tastes"), py_str("tasting"), py_str("slow"), py_str("leak"), py_str("leaks"), py_str("switch"), py_str("switching"), py_str("switches")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("burnt-tin")); __args.push(py_tuple(vec![py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("burnt on"), py_str("burnt bits"), py_str("black bits"), py_str("blackened"), py_str("scorched"), py_str("charred"), py_str("crusty"), py_str("black")]), py_tuple(vec![py_str("tin"), py_str("tins"), py_str("cake tin"), py_str("loaf tin"), py_str("muffin tin"), py_str("baking tin"), py_str("roasting tin"), py_str("pie tin"), py_str("tart tin"), py_str("springform"), py_str("tray"), py_str("trays"), py_str("baking tray"), py_str("roasting tray"), py_str("baking sheet"), py_str("oven shelf"), py_str("oven shelves"), py_str("shelf"), py_str("rack"), py_str("mould")])])); __args.push(g__burnt_tin()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("sticking"), py_str("sticks"), py_str("using"), py_str("use a"), py_str("use my"), py_str("what size"), py_str("what can i make"), py_str("recipe"), py_str("oven temperature"), py_str("what temperature"), py_str("how long"), py_str("why does"), py_str("where do i buy"), py_str("how much is"), py_str("worth"), py_str("springform leaks"), py_str("leaks"), py_str("rusty"), py_str("rust"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("mildew"), py_str("damp"), py_str("dehumidifier")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("burnt-pan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pan"), py_str("pans"), py_str("pot"), py_str("pots"), py_str("saucepan"), py_str("saucepans"), py_str("wok"), py_str("casserole"), py_str("skillet")]), py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("burn"), py_str("scorched"), py_str("scorching"), py_str("cleaning"), py_str("clean"), py_str("scour"), py_str("soak"), py_str("stuck"), py_str("bottom"), py_str("black"), py_str("outside"), py_str("outside of the pan"), py_str("outside bottom"), py_str("burnt black"), py_str("blackened"), py_str("stuck on it"), py_str("stuck on"), py_str("stuck to it"), py_str("burnt stuff"), py_str("stuff stuck"), py_str("burnt bits stuck"), py_str("rescue"), py_str("salvage"), py_str("ruined"), py_str("scorched pan"), py_str("burnt pan"), py_str("burned pan")])])); __args.push(g__burnt_pan()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("my hand"), py_str("my finger"), py_str("my arm"), py_str("my wrist"), py_str("my thumb"), py_str("myself"), py_str("oven"), py_str("worktop"), py_str("worktops"), py_str("counter"), py_str("countertop"), py_str("the table"), py_str("work surface"), py_str("worksurface"), py_str("benchtop"), py_str("the floor"), py_str("carpet"), py_str("roast"), py_str("toast"), py_str("carpet"), py_str("cake recipe"), py_str("the cake is burnt"), py_str("cake is burnt"), py_str("my cake burnt"), py_str("my cake is burnt"), py_str("my cakes burnt"), py_str("litter"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pet"), py_str("pets"), py_str("rabbit"), py_str("hamster"), py_str("nappy"), py_str("bin"), py_str("toilet"), py_str("nursery"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("mildew"), py_str("fungus"), py_str("plant pot"), py_str("flowerpot"), py_str("houseplant"), py_str("house plant")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kit-damaged")); __args.push(py_tuple(vec![g__KIT_DAMAGE(), g__KIT_THING()])); __args.push(g__kit_damaged()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("handle"), py_str("handles"), py_str("rust"), py_str("rusty"), py_str("rusted"), py_str("season"), py_str("seasoning"), py_str("non stick"), py_str("non-stick"), py_str("nonstick"), py_str("teflon"), py_str("scratched"), py_str("scratches"), py_str("phone"), py_str("mobile"), py_str("screen"), py_str("laptop"), py_str("tablet"), py_str("window"), py_str("windscreen"), py_str("wall"), py_str("ceiling"), py_str("tooth"), py_str("teeth"), py_str("bone"), py_str("skull"), py_str("rib"), py_str("ankle"), py_str("wrist"), py_str("mug"), py_str("cup"), py_str("tea cup"), py_str("teacup"), py_str("china"), py_str("crockery"), py_str("teapot"), py_str("recipe"), py_str("recipes"), py_str("what can i cook"), py_str("what can i make"), py_str("how do i cook"), py_str("how long to cook"), py_str("cook a"), py_str("roast"), py_str("bake"), py_str("baking a"), py_str("season"), py_str("seasoning"), py_str("buy a"), py_str("buying a"), py_str("which pan"), py_str("best pan"), py_str("what pan"), py_str("worth it"), py_str("warranty"), py_str("guarantee"), py_str("what is"), py_str("what are"), py_str("whats a"), py_str("what's a"), py_str("whats an"), py_str("what's an"), py_str("which"), py_str("best"), py_str("recommend")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KIT_WORDS()) { let __unpk5902 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk5902[0].clone(); let mut what: Py = __unpk5902[1].clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(py_str("-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&name.clone(), &Py::Int(0)), "replace", args))(&__args) }, name.clone(), what.clone()])); } py_list(__out) })() });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    s__slug(__unpk2[0].clone());
    s__names(__unpk2[1].clone());
    s__what(__unpk2[2].clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("kit-what-is-"), &g__slug())); __args.push(py_tuple(vec![g__names(), g_WHAT_IS_ASK()])); __args.push(g__what()); __kws.extend(vec![("avoid".to_string(), g_NOT_A_WHAT_IS()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    }
}
pub fn _ml(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut litres: Py = Py::None;
    let _ = py_str("A millilitre figure as a person would say it: 600 ml, then 1.2 litres.");
    if py_truthy(&op_ge(&value.clone(), &Py::Int(1000))) {
    litres = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("0")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&op_truediv(&value.clone(), &Py::Float(1000.0)), -1, &".1f")); py_string(__s) }, "rstrip", args))(&__args) }, "rstrip", args))(&__args) };
    return { let mut __s = String::new(); __s.push_str(&fstr(&litres.clone(), -1, "")); __s.push_str(" litres"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, "")); __s.push_str(" ml"); py_string(__s) };
    Py::None
}
pub fn _cups_said(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("(count, \"cups\" or \"people\") from the line itself, or (0, \"cups\").");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CUPS_SAID(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![Py::Int(0), py_str("cups")]);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    let __try1 = exc::catch(|| {
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ValueError"]) {
                __handled5 = true;
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COUNT_WORDS(), "get", args))(&__args) };
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    unit = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("people"), py_str("person"), py_str("of us")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "startswith", args))(&__args) }) { py_str("people") } else { py_str("cups") };
    return py_tuple(vec![if py_truthy(&op_gt(&count.clone(), &Py::Int(0))) { count.clone() } else { Py::Int(0) }, unit.clone()]);
    Py::None
}
pub fn _tea_water(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("How much water to boil, and how long it takes, from the line.");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Int(0), py_str("cups")]), 2, None);
    count = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numbers")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    number = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __try3 = exc::catch(|| {
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&op_lt(&value.clone(), &Py::Int(1))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("cup"), py_str("cups"), py_str("mug"), py_str("mugs"), py_str("cuppa"), py_str("cuppas")]), &said.clone()))) {
    let __unpk8 = py_unpack(&py_tuple(vec![value.clone(), py_str("cups")]), 2, None);
    count = __unpk8[0].clone();
    unit = __unpk8[1].clone();
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("people"), py_str("person"), py_str("of us"), py_str("guest"), py_str("guests")]), &said.clone()))) {
    let __unpk9 = py_unpack(&py_tuple(vec![value.clone(), py_str("people")]), 2, None);
    count = __unpk9[0].clone();
    unit = __unpk9[1].clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    let __unpk10 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cups_said(args))(&__args) }, 2, None);
    count = __unpk10[0].clone();
    unit = __unpk10[1].clone();
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FULL_KETTLE_ASK(), "search", args))(&__args) } } }) {
    return g__FULL_KETTLE();
    }
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return py_str("A mug of tea is about 300 ml, so count the mugs you are making and fill the kettle just past that: two cups is 600 ml, and a full 1.7-litre kettle is about five mugs. Boiling only what you will drink is the cheapest habit in the kitchen.");
    }
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&count.clone(), &g_ML_PER_MUG())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ml(args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("people")))) {
    what = { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" people, a mug each"); py_string(__s) };
    } else {
    if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) {
    what = py_str("one cup");
    } else {
    what = { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" cups"); py_string(__s) };
    }
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" for "); __s.push_str(&fstr(&what.clone(), -1, "")); __s.push_str(", since a mug holds roughly 300 ml and a splash more covers the element. Boiling only what you drink is the cheapest habit in the kitchen."); py_string(__s) };
    Py::None
}
pub fn _burnt_tin(args: &[Py]) -> Py {
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
    let mut baked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The soak, said for the thing that burnt on.\n\n    A tin, a mould and a springform are scoured at their peril, so the answer says why not. A\n    tray or a shelf is flat and takes a scourer, and it lies in the sink instead of being filled;\n    the shelf is the one that may not fit, and what stops it coming back is foil under the tin\n    rather than baking paper inside it.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_words_and_text(args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "add", args))(&__args) };
    }
    }
    baked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::knowledge::cooking::g__VESSEL_PHRASES(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BLANK_STUFF(), "sub", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(baked.clone()); __args.push(crate::knowledge::cooking::g__FOOD()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(op_add(&g__PAN_CLEAN(), &py_tuple(vec![py_str("off my tray"), py_str("off the tray"), py_str("off my tin"), py_str("off the tin"), py_str("get the burnt off"), py_str("off the sheet")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("coating"), py_str("coatings"), py_str("non"), py_str("nonstick"), py_str("teflon"), py_str("scratch"), py_str("scratched"), py_str("scratches"), py_str("scouring"), py_str("scour"), py_str("scourer"), py_str("wool"), py_str("scraper")]))) {
    return py_str("Bicarbonate and a wooden spoon will not lift a non-stick coating - it is wire wool and metal scourers that do. If the coating is already scratched through to the metal, that tin is done for anything that sticks, so line it with baking paper and keep it for bakes that turn out of the tin anyway.");
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("shelf"), py_str("shelves"), py_str("rack"), py_str("racks")]))) {
    return py_str("Soak it in hot water and two tablespoons of bicarbonate of soda for half an hour - the bath if the shelf will not fit in the sink - then lift the burnt patches off with a plastic scourer or a ball of foil. Anything really black takes a paste of bicarbonate and water left on overnight. Dry it before it goes back in, and put a sheet of foil on the shelf under the tin next time, which is what catches the drips.");
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("tray"), py_str("trays"), py_str("sheet"), py_str("sheets")]))) {
    return py_str("Soak it in the sink with hot water and two tablespoons of bicarbonate of soda for half an hour, then lift the burnt patches off with a wooden spoon or a plastic scourer. Anything really black takes a paste of bicarbonate and water left on overnight. Dry it properly before it goes away, and line it with baking paper next time, which is what stops it coming back.");
    }
    return py_str("Soak it in the sink with hot water and two tablespoons of bicarbonate of soda for half an hour, then lift the burnt patches off with a wooden spoon or a plastic scourer: tins are thin and scouring takes the non-stick off for good. Anything really black takes a paste of bicarbonate and water left on overnight. Dry it properly before it goes away, and line it with baking paper next time, which is what stops it coming back.");
    Py::None
}
pub fn _burnt_pan(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The pan's answer for a pan line, and nothing for a line whose dish is what burnt.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("norm")])) { let mut key: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__BURNT_EATING_SAFETY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) }) {
    return crate::knowledge::cooking::g__BURNT_EATING_REPLY();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__NOT_A_PAN()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__A_PAN()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__PAN_STUCK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__PAN_BURNT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__PAN_SOMETHING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) })) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__PAN_CLEAN()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) }) {
    return g__BURNT_PAN_REPLY();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(crate::knowledge::cooking::g__PAN_DISH()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_says(args))(&__args) }) {
    return py_str("");
    }
    return g__BURNT_PAN_REPLY();
    Py::None
}
pub fn _kit_damaged(args: &[Py]) -> Py {
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
    let mut has: Py = Py::None;
    let mut key: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A cracked, warped or worn-out piece of kitchen kit: whether it can be used, fixed or binned.\n\n    The answer follows the material, because that is what decides it: glass and ceramic cannot be\n    used with a crack in them, a wooden board can be sanded unless the split goes through, and a\n    warped pan is only a problem on a hob that needs a flat base.\n    ");
    understood = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("norm_text"), py_str("clean")])) { let mut key: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
pub fn __nested_has_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut key: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut phrases: Py = py_list(__pos.get(5..).unwrap_or(&[]).to_vec());
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s"), &phrase.clone())))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
    has = {{ let has = has.clone(); let key = key.clone(); let memory = memory.clone(); let text = text.clone(); let understood = understood.clone(); py_func("has", move |args: &[Py]| { let mut full = vec![has.clone(), key.clone(), memory.clone(), text.clone(), understood.clone()]; full.extend_from_slice(args); __nested_has_0(&full) }) }};
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("microwave plate")); __args.push(py_str("turntable")); __args.push(py_str("glass plate")); __args.push(py_str("plate in the microwave")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("Replace the part rather than the machine: a microwave plate or turntable ring is about £10 to £20 and sold by the model number on the sticker inside the door, or stamped on the plate itself. A cracked plate can shatter the next time it heats, so keep it out of the microwave until the new one comes.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("jug")); __args.push(py_str("blender")); __args.push(py_str("carafe")); __args.push(py_str("glass lid")); __args.push(py_str("lid")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("A cracked jug, blender jug or glass lid goes: it can let go all at once with something hot in it, and a crack you cannot scrub out of is not a food surface. Blenders and kettles take a spare jug or lid by model number, and a jam jar is a free substitute while you wait.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("board")); __args.push(py_str("chopping board")); __args.push(py_str("cutting board")); __args.push(py_str("wooden board")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("Sand it back and oil it if the split is shallow, because that closes the grain again; a crack that goes right through traps food where you cannot wash it out, and that board should go. Never soak a wooden board or put it in the dishwasher - wash it, stand it on edge to dry and oil it once a month.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("enamel")); __args.push(py_str("enamelled")); __args.push(py_str("chipped")); __args.push(py_str("chipping")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("A chip on the inside of an enamelled pan will rust and flake into the food, so that one is finished; a chip on the outside only matters for looks. Enamel repair paints are not food safe, so do not touch one in, and do not put an enamelled pan in the dishwasher, which is what chips the rim.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coating")); __args.push(py_str("non stick")); __args.push(py_str("non-stick")); __args.push(py_str("teflon")); __args.push(py_str("flaking")); __args.push(py_str("peeling")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("Once the coating is flaking the pan is done: the flakes come off into the food and the bare metal scorches what it touches. Re-coating is not a home job, so replace it - and washing it by hand, never heating it empty and never scouring it are what make the next one last three to five years.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("warped")); __args.push(py_str("warping")); __args.push(py_str("buckled")); __args.push(py_str("buckling")); __args.push(py_str("dented")); __args.push(py_str("bent")); __args.push(py_str("wobbles")); __args.push(py_str("wobbling")); __args.push(py_str("rocks")); __args.push(py_str("rocking")); __args.push(py_str("spins")); __args.push(py_str("doesnt sit flat")); __args.push(py_str("doesn't sit flat")); __args.push(py_str("wont sit flat")); __args.push(py_str("won't sit flat")); __args.push(py_str("not flat")); __args.push(py_str("bowed")); __args.push(py_str("dished")); __args.push(py_str("domed")); __args.push(py_str("bulging")); __args.push(py_str("bulged")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tin")); __args.push(py_str("tins")); __args.push(py_str("tray")); __args.push(py_str("trays")); __args.push(py_str("sheet")); __args.push(py_str("springform")); __args.push(py_str("mould")); __args.push(py_str("muffin")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("Stand it on the worktop and press it flat again with the heel of your hand: a baking tin that has bent in the wash still bakes the same, and the only thing you notice is the wobble. Set it on a flat baking sheet if the batter runs to one side. Replace it when the springform clip or the base no longer seats flat, because then it leaks.");
    }
    return py_str("A warped base only really matters on a flat hob: on gas the pan still heats, but on induction, ceramic or solid plate it will not sit flat and the heat goes patchy. Check it against a straight edge on the worktop. It cannot be flat again for certain, so a heavy-based pan is the replacement - thin ones go in one hot empty minute or a cold rinse in a hot pan.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kettle")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("A cracked kettle goes: water reaches the element and the connections, which is a shock risk rather than a leak to mop up. Unplug it and replace it - a new kettle is £15 to £30, and descaling the next one monthly is what keeps it from running hot.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("slow cooker")); __args.push(py_str("casserole")); __args.push(py_str("stoneware")); __args.push(py_str("ceramic")); __args.push(py_str("pot")); __args.push(py_str("pots")); __args.push(py_str("dish")); __args.push(py_str("dish")); __args.push(py_str("baking dish")); __args.push(py_str("oven dish")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&has, args))(&__args) }) {
    return py_str("Retire it if the crack is in the pot itself: stoneware fails right through when it heats, and a crack holds food you cannot wash out of it. Slow cooker inserts and casserole dishes are sold on their own by model number or size, £10 to £25, and a lidded oven dish that fits inside will do until the new one comes.");
    }
    return py_str("If the crack is right through, replace it: a split pot, dish or plate holds food and bacteria where you cannot wash, and it can fail completely the next time it heats. A chip or crack on an outside face that never touches food can usually be lived with.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.cook_kit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "KIT_WORDS" => Some(g_KIT_WORDS()),
            "ML_PER_MUG" => Some(g_ML_PER_MUG()),
            "NOT_A_WHAT_IS" => Some(g_NOT_A_WHAT_IS()),
            "WHAT_IS_ASK" => Some(g_WHAT_IS_ASK()),
            "_A_PAN" => Some(g__A_PAN()),
            "_BLANK_STUFF" => Some(g__BLANK_STUFF()),
            "_BURNT_EATING_REPLY" => Some(g__BURNT_EATING_REPLY()),
            "_BURNT_EATING_SAFETY" => Some(g__BURNT_EATING_SAFETY()),
            "_BURNT_ON_A_PAN" => Some(g__BURNT_ON_A_PAN()),
            "_BURNT_PAN_REPLY" => Some(g__BURNT_PAN_REPLY()),
            "_COUNT_WORDS" => Some(g__COUNT_WORDS()),
            "_CUPS_SAID" => Some(g__CUPS_SAID()),
            "_FOOD" => Some(g__FOOD()),
            "_FULL_KETTLE" => Some(g__FULL_KETTLE()),
            "_FULL_KETTLE_ASK" => Some(g__FULL_KETTLE_ASK()),
            "_KIT_DAMAGE" => Some(g__KIT_DAMAGE()),
            "_KIT_THING" => Some(g__KIT_THING()),
            "_NOT_A_PAN" => Some(g__NOT_A_PAN()),
            "_PAN_BURNT" => Some(g__PAN_BURNT()),
            "_PAN_CLEAN" => Some(g__PAN_CLEAN()),
            "_PAN_DISH" => Some(g__PAN_DISH()),
            "_PAN_SOMETHING" => Some(g__PAN_SOMETHING()),
            "_PAN_STUCK" => Some(g__PAN_STUCK()),
            "_POT_SPLASH" => Some(g__POT_SPLASH()),
            "_REMOVAL_FROM_A_PAN" => Some(g__REMOVAL_FROM_A_PAN()),
            "_VESSEL_PHRASES" => Some(g__VESSEL_PHRASES()),
            "_burnt_pan" => Some(py_func("_burnt_pan", |args: &[Py]| _burnt_pan(args))),
            "_burnt_tin" => Some(py_func("_burnt_tin", |args: &[Py]| _burnt_tin(args))),
            "_cups_said" => Some(py_func("_cups_said", |args: &[Py]| _cups_said(args))),
            "_kit_damaged" => Some(py_func("_kit_damaged", |args: &[Py]| _kit_damaged(args))),
            "_ml" => Some(py_func("_ml", |args: &[Py]| _ml(args))),
            "_names" => Some(g__names()),
            "_slug" => Some(g__slug()),
            "_tea_water" => Some(py_func("_tea_water", |args: &[Py]| _tea_water(args))),
            "_what" => Some(g__what()),
            "name" => Some(g_name()),
            "what" => Some(g_what()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "KIT_WORDS" => s_KIT_WORDS(v),
            "ML_PER_MUG" => s_ML_PER_MUG(v),
            "NOT_A_WHAT_IS" => s_NOT_A_WHAT_IS(v),
            "WHAT_IS_ASK" => s_WHAT_IS_ASK(v),
            "_A_PAN" => s__A_PAN(v),
            "_BLANK_STUFF" => s__BLANK_STUFF(v),
            "_BURNT_EATING_REPLY" => s__BURNT_EATING_REPLY(v),
            "_BURNT_EATING_SAFETY" => s__BURNT_EATING_SAFETY(v),
            "_BURNT_ON_A_PAN" => s__BURNT_ON_A_PAN(v),
            "_BURNT_PAN_REPLY" => s__BURNT_PAN_REPLY(v),
            "_COUNT_WORDS" => s__COUNT_WORDS(v),
            "_CUPS_SAID" => s__CUPS_SAID(v),
            "_FOOD" => s__FOOD(v),
            "_FULL_KETTLE" => s__FULL_KETTLE(v),
            "_FULL_KETTLE_ASK" => s__FULL_KETTLE_ASK(v),
            "_KIT_DAMAGE" => s__KIT_DAMAGE(v),
            "_KIT_THING" => s__KIT_THING(v),
            "_NOT_A_PAN" => s__NOT_A_PAN(v),
            "_PAN_BURNT" => s__PAN_BURNT(v),
            "_PAN_CLEAN" => s__PAN_CLEAN(v),
            "_PAN_DISH" => s__PAN_DISH(v),
            "_PAN_SOMETHING" => s__PAN_SOMETHING(v),
            "_PAN_STUCK" => s__PAN_STUCK(v),
            "_POT_SPLASH" => s__POT_SPLASH(v),
            "_REMOVAL_FROM_A_PAN" => s__REMOVAL_FROM_A_PAN(v),
            "_VESSEL_PHRASES" => s__VESSEL_PHRASES(v),
            "_names" => s__names(v),
            "_slug" => s__slug(v),
            "_what" => s__what(v),
            "name" => s_name(v),
            "what" => s_what(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.cook_kit", module_obj);
    pyrt::xrefs::reg_fn("knowledge.cook_kit", "_ml", py_func("_ml", |args: &[Py]| _ml(args)));
    pyrt::xrefs::reg_fn("knowledge.cook_kit", "_cups_said", py_func("_cups_said", |args: &[Py]| _cups_said(args)));
    pyrt::xrefs::reg_fn("knowledge.cook_kit", "_tea_water", py_func("_tea_water", |args: &[Py]| _tea_water(args)));
    pyrt::xrefs::reg_fn("knowledge.cook_kit", "_burnt_tin", py_func("_burnt_tin", |args: &[Py]| _burnt_tin(args)));
    pyrt::xrefs::reg_fn("knowledge.cook_kit", "_burnt_pan", py_func("_burnt_pan", |args: &[Py]| _burnt_pan(args)));
    pyrt::xrefs::reg_fn("knowledge.cook_kit", "_kit_damaged", py_func("_kit_damaged", |args: &[Py]| _kit_damaged(args)));
    pyrt::xrefs::reg_access("knowledge.cook_kit", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "KIT_WORDS", g_KIT_WORDS, s_KIT_WORDS);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "ML_PER_MUG", g_ML_PER_MUG, s_ML_PER_MUG);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "NOT_A_WHAT_IS", g_NOT_A_WHAT_IS, s_NOT_A_WHAT_IS);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "WHAT_IS_ASK", g_WHAT_IS_ASK, s_WHAT_IS_ASK);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_A_PAN", g__A_PAN, s__A_PAN);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_BLANK_STUFF", g__BLANK_STUFF, s__BLANK_STUFF);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_BURNT_EATING_REPLY", g__BURNT_EATING_REPLY, s__BURNT_EATING_REPLY);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_BURNT_EATING_SAFETY", g__BURNT_EATING_SAFETY, s__BURNT_EATING_SAFETY);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_BURNT_ON_A_PAN", g__BURNT_ON_A_PAN, s__BURNT_ON_A_PAN);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_BURNT_PAN_REPLY", g__BURNT_PAN_REPLY, s__BURNT_PAN_REPLY);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_COUNT_WORDS", g__COUNT_WORDS, s__COUNT_WORDS);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_CUPS_SAID", g__CUPS_SAID, s__CUPS_SAID);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_FOOD", g__FOOD, s__FOOD);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_FULL_KETTLE", g__FULL_KETTLE, s__FULL_KETTLE);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_FULL_KETTLE_ASK", g__FULL_KETTLE_ASK, s__FULL_KETTLE_ASK);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_KIT_DAMAGE", g__KIT_DAMAGE, s__KIT_DAMAGE);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_KIT_THING", g__KIT_THING, s__KIT_THING);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_NOT_A_PAN", g__NOT_A_PAN, s__NOT_A_PAN);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_PAN_BURNT", g__PAN_BURNT, s__PAN_BURNT);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_PAN_CLEAN", g__PAN_CLEAN, s__PAN_CLEAN);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_PAN_DISH", g__PAN_DISH, s__PAN_DISH);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_PAN_SOMETHING", g__PAN_SOMETHING, s__PAN_SOMETHING);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_PAN_STUCK", g__PAN_STUCK, s__PAN_STUCK);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_POT_SPLASH", g__POT_SPLASH, s__POT_SPLASH);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_REMOVAL_FROM_A_PAN", g__REMOVAL_FROM_A_PAN, s__REMOVAL_FROM_A_PAN);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_VESSEL_PHRASES", g__VESSEL_PHRASES, s__VESSEL_PHRASES);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_names", g__names, s__names);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_slug", g__slug, s__slug);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "_what", g__what, s__what);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "name", g_name, s_name);
    pyrt::xrefs::reg_access("knowledge.cook_kit", "what", g_what, s_what);
}
