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
pub static _ABRASIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ACID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ANY_MACHINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BEDDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BLEACH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COATING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DELICATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HAND_LIQUID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HARD_STONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOW_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IRON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KNIFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MACHINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MATTRESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_WASHABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SCREEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SOFT_METAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STONEWARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TRAPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g__ABRASIVE() -> Py { ensure_init(); _ABRASIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ABRASIVE(v: Py) { ensure_init(); if let Ok(mut w) = _ABRASIVE.write() { *w = v; } }
pub fn g__ACID() -> Py { ensure_init(); _ACID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ACID(v: Py) { ensure_init(); if let Ok(mut w) = _ACID.write() { *w = v; } }
pub fn g__ANY_MACHINE() -> Py { ensure_init(); _ANY_MACHINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ANY_MACHINE(v: Py) { ensure_init(); if let Ok(mut w) = _ANY_MACHINE.write() { *w = v; } }
pub fn g__BEDDING() -> Py { ensure_init(); _BEDDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BEDDING(v: Py) { ensure_init(); if let Ok(mut w) = _BEDDING.write() { *w = v; } }
pub fn g__BLEACH() -> Py { ensure_init(); _BLEACH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BLEACH(v: Py) { ensure_init(); if let Ok(mut w) = _BLEACH.write() { *w = v; } }
pub fn g__COATED() -> Py { ensure_init(); _COATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COATED(v: Py) { ensure_init(); if let Ok(mut w) = _COATED.write() { *w = v; } }
pub fn g__COATING() -> Py { ensure_init(); _COATING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COATING(v: Py) { ensure_init(); if let Ok(mut w) = _COATING.write() { *w = v; } }
pub fn g__DELICATE() -> Py { ensure_init(); _DELICATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DELICATE(v: Py) { ensure_init(); if let Ok(mut w) = _DELICATE.write() { *w = v; } }
pub fn g__HAND_LIQUID() -> Py { ensure_init(); _HAND_LIQUID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HAND_LIQUID(v: Py) { ensure_init(); if let Ok(mut w) = _HAND_LIQUID.write() { *w = v; } }
pub fn g__HARD_STONE() -> Py { ensure_init(); _HARD_STONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HARD_STONE(v: Py) { ensure_init(); if let Ok(mut w) = _HARD_STONE.write() { *w = v; } }
pub fn g__HOW_ASK() -> Py { ensure_init(); _HOW_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOW_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _HOW_ASK.write() { *w = v; } }
pub fn g__IRON() -> Py { ensure_init(); _IRON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IRON(v: Py) { ensure_init(); if let Ok(mut w) = _IRON.write() { *w = v; } }
pub fn g__KNIFE() -> Py { ensure_init(); _KNIFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KNIFE(v: Py) { ensure_init(); if let Ok(mut w) = _KNIFE.write() { *w = v; } }
pub fn g__MACHINE() -> Py { ensure_init(); _MACHINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MACHINE(v: Py) { ensure_init(); if let Ok(mut w) = _MACHINE.write() { *w = v; } }
pub fn g__MATTRESS() -> Py { ensure_init(); _MATTRESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MATTRESS(v: Py) { ensure_init(); if let Ok(mut w) = _MATTRESS.write() { *w = v; } }
pub fn g__NOT_WASHABLE() -> Py { ensure_init(); _NOT_WASHABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_WASHABLE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_WASHABLE.write() { *w = v; } }
pub fn g__SCREEN() -> Py { ensure_init(); _SCREEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SCREEN(v: Py) { ensure_init(); if let Ok(mut w) = _SCREEN.write() { *w = v; } }
pub fn g__SOFT_METAL() -> Py { ensure_init(); _SOFT_METAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SOFT_METAL(v: Py) { ensure_init(); if let Ok(mut w) = _SOFT_METAL.write() { *w = v; } }
pub fn g__STONE() -> Py { ensure_init(); _STONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STONE(v: Py) { ensure_init(); if let Ok(mut w) = _STONE.write() { *w = v; } }
pub fn g__STONEWARE() -> Py { ensure_init(); _STONEWARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STONEWARE(v: Py) { ensure_init(); if let Ok(mut w) = _STONEWARE.write() { *w = v; } }
pub fn g__TRAPPED() -> Py { ensure_init(); _TRAPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TRAPPED(v: Py) { ensure_init(); if let Ok(mut w) = _TRAPPED.write() { *w = v; } }
pub fn g__WET() -> Py { ensure_init(); _WET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WET(v: Py) { ensure_init(); if let Ok(mut w) = _WET.write() { *w = v; } }
pub fn g__WOOD() -> Py { ensure_init(); _WOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WOOD(v: Py) { ensure_init(); if let Ok(mut w) = _WOOD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A thing put on another thing: bleach on a mattress, vinegar on granite.\n\nThe everyday question of this shape is \"can i use <product> on <surface>\", and the bot had no\nrow of that shape at all: every cleaning row asks what the *job* is (\"how do i clean a mattress\",\n\"how do i get a stain out of the sofa\"), so a line that names a product and a surface skipped all\nof them and landed on whatever row held one of the two words:\n\n    can i use bleach on a mattress          -> how to bleach white cotton\n    can i use vinegar on a phone screen     -> how much screen time for children\n    can i use a metal scourer on a non stick pan -> why food tastes of metal\n    can i use white vinegar on a granite worktop -> a search hedge\n\nThe rows here are keyed by the *pair*: the product in the first group, the surface in the second,\nbecause \"will this damage that\" is the question and either word alone is something else. Each\nanswer says what happens and what to use instead, which is what the person wanted to know.\n");
    s__MATTRESS(py_tuple(vec![py_str("mattress"), py_str("mattresses"), py_str("memory foam"), py_str("foam"), py_str("mattress topper"), py_str("mattress protector"), py_str("mattress cover"), py_str("headboard"), py_str("bed base"), py_str("divan")]));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    s__ACID(py_tuple(vec![py_str("vinegar"), py_str("white vinegar"), py_str("malt vinegar"), py_str("distilled vinegar"), py_str("lemon juice"), py_str("lemon"), py_str("citric acid"), py_str("descaler"), py_str("descaling"), py_str("limescale remover"), py_str("barkeeper"), py_str("bar keepers friend"), py_str("acid")]));
    s__STONE(py_tuple(vec![py_str("granite"), py_str("granite worktop"), py_str("granite worktops"), py_str("granite counter"), py_str("granite top"), py_str("marble"), py_str("marble worktop"), py_str("marble worktops"), py_str("marble floor"), py_str("marble tiles"), py_str("limestone"), py_str("travertine"), py_str("onyx"), py_str("alabaster"), py_str("quartz"), py_str("quartz worktop"), py_str("quartz worktops"), py_str("stone worktop"), py_str("stone worktops"), py_str("stone floor"), py_str("stone floors"), py_str("slate worktop"), py_str("slate floor"), py_str("slate hearth"), py_str("stone counter"), py_str("stone counters")]));
    s__BLEACH(py_tuple(vec![py_str("bleach"), py_str("bleaching"), py_str("milton"), py_str("bleach spray"), py_str("domestos"), py_str("thin bleach"), py_str("household bleach"), py_str("mould spray"), py_str("mould remover")]));
    s__BEDDING(py_tuple(vec![py_str("mattress"), py_str("mattresses"), py_str("memory foam"), py_str("foam mattress"), py_str("mattress topper"), py_str("mattress protector"), py_str("mattress cover"), py_str("pillow"), py_str("pillows"), py_str("duvet"), py_str("duvets"), py_str("bedding"), py_str("headboard")]));
    s__WET(py_tuple(vec![py_str("water"), py_str("spray"), py_str("sprays"), py_str("spraying"), py_str("wipes"), py_str("wipe"), py_str("wipes"), py_str("damp cloth"), py_str("glass cleaner"), py_str("windex"), py_str("window cleaner"), py_str("alcohol"), py_str("isopropyl"), py_str("vodka"), py_str("hand sanitiser"), py_str("hand sanitizer"), py_str("soap"), py_str("soapy water"), py_str("vinegar"), py_str("bleach")]));
    s__SCREEN(py_tuple(vec![py_str("phone screen"), py_str("phone"), py_str("laptop screen"), py_str("laptop"), py_str("tv screen"), py_str("tv"), py_str("television"), py_str("monitor"), py_str("computer screen"), py_str("tablet"), py_str("tablet screen"), py_str("ipad"), py_str("iphone"), py_str("screen"), py_str("screens"), py_str("kindle"), py_str("e reader"), py_str("watch screen"), py_str("smartwatch"), py_str("touchscreen"), py_str("touch screen")]));
    s__ABRASIVE(py_tuple(vec![py_str("scourer"), py_str("scourers"), py_str("metal scourer"), py_str("wire wool"), py_str("steel wool"), py_str("brillo"), py_str("abrasive"), py_str("abrasives"), py_str("sandpaper"), py_str("scrub"), py_str("scrubbing"), py_str("scrubber"), py_str("wire brush"), py_str("metal brush"), py_str("bleach"), py_str("oven cleaner"), py_str("sharp knife"), py_str("knife")]));
    s__COATED(py_tuple(vec![py_str("non stick"), py_str("non-stick"), py_str("nonstick"), py_str("teflon"), py_str("tefal"), py_str("coated pan"), py_str("coated pans"), py_str("frying pan"), py_str("frying pans"), py_str("wok"), py_str("air fryer"), py_str("air fryer basket"), py_str("rice cooker"), py_str("rice cooker bowl"), py_str("baking tray"), py_str("baking trays"), py_str("griddle"), py_str("crepe pan"), py_str("pancake pan"), py_str("bread tin"), py_str("cake tin"), py_str("muffin tin")]));
    s__HAND_LIQUID(py_tuple(vec![py_str("washing up liquid"), py_str("washing-up liquid"), py_str("fairy liquid"), py_str("dish soap"), py_str("dishwashing liquid"), py_str("hand soap"), py_str("shower gel"), py_str("bubble bath"), py_str("shampoo"), py_str("body wash"), py_str("swarfega"), py_str("laundry liquid"), py_str("detergent"), py_str("soap")]));
    s__MACHINE(py_tuple(vec![py_str("dishwasher"), py_str("dish washers"), py_str("dishwasher tablets"), py_str("dishwasher tablet")]));
    s__ANY_MACHINE(add(&g__MACHINE(), &py_tuple(vec![py_str("washing machine"), py_str("washing machines"), py_str("washer"), py_str("tumble dryer"), py_str("dryer")])));
    s__NOT_WASHABLE(py_tuple(vec![py_str("wooden spoon"), py_str("wooden spoons"), py_str("wooden spatula"), py_str("wooden spatulas"), py_str("wooden board"), py_str("wooden chopping board"), py_str("chopping board"), py_str("wooden handle"), py_str("wooden handles"), py_str("knife"), py_str("knives"), py_str("chefs knife"), py_str("chef's knife"), py_str("kitchen knife"), py_str("carving knife"), py_str("bread knife"), py_str("aluminium"), py_str("aluminum"), py_str("cast iron"), py_str("carbon steel"), py_str("copper"), py_str("crystal"), py_str("crystal glass"), py_str("gold rim"), py_str("gold rimmed"), py_str("gold-rimmed"), py_str("fine china"), py_str("bone china"), py_str("thermos"), py_str("vacuum flask"), py_str("garlic press"), py_str("cheese grater"), py_str("non stick"), py_str("non-stick"), py_str("nonstick"), py_str("silver cutlery"), py_str("silver"), py_str("pewter")]));
    s__HOW_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|may|is it|is|are|will|would|safe|ok|okay|alright)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WOOD(py_tuple(vec![py_str("wood"), py_str("wooden"), py_str("bamboo"), py_str("teak"), py_str("board"), py_str("boards"), py_str("chopping board"), py_str("spoon"), py_str("spoons"), py_str("spatula"), py_str("spatulas"), py_str("utensil"), py_str("utensils"), py_str("handle"), py_str("handles"), py_str("mug tree"), py_str("bowl"), py_str("bowls"), py_str("rolling pin")]));
    s__KNIFE(py_tuple(vec![py_str("knife"), py_str("knives"), py_str("blade"), py_str("blades"), py_str("chef's knife"), py_str("chefs knife"), py_str("carving knife"), py_str("bread knife"), py_str("kitchen knife"), py_str("steak knife"), py_str("steak knives")]));
    s__IRON(py_tuple(vec![py_str("cast iron"), py_str("carbon steel"), py_str("skillet"), py_str("griddle pan"), py_str("seasoned pan")]));
    s__SOFT_METAL(py_tuple(vec![py_str("aluminium"), py_str("aluminum"), py_str("copper"), py_str("silver"), py_str("silver cutlery"), py_str("pewter"), py_str("brass"), py_str("tin"), py_str("tinned"), py_str("cake tin"), py_str("muffin tin"), py_str("baking tray"), py_str("roasting tin")]));
    s__DELICATE(py_tuple(vec![py_str("crystal"), py_str("crystal glass"), py_str("wine glass"), py_str("wine glasses"), py_str("decanter"), py_str("gold rim"), py_str("gold rimmed"), py_str("gold-rimmed"), py_str("china"), py_str("bone china"), py_str("fine china"), py_str("antique"), py_str("hand painted"), py_str("hand-painted"), py_str("printed glass")]));
    s__TRAPPED(py_tuple(vec![py_str("thermos"), py_str("vacuum flask"), py_str("flask"), py_str("garlic press"), py_str("cheese grater"), py_str("grater"), py_str("lemon squeezer"), py_str("squeezer"), py_str("pepper mill"), py_str("coffee grinder"), py_str("whisk"), py_str("colander"), py_str("potato ricer")]));
    s__STONEWARE(py_tuple(vec![py_str("marble"), py_str("mortar"), py_str("pestle"), py_str("mortar and pestle"), py_str("stoneware"), py_str("slate"), py_str("granite"), py_str("soapstone"), py_str("terracotta"), py_str("pizza stone"), py_str("baking stone"), py_str("pizza steel")]));
    s__NOT_WASHABLE(add(&add(&g__NOT_WASHABLE(), &g__TRAPPED()), &g__STONEWARE()));
    s__COATING(py_tuple(vec![py_str("non stick"), py_str("non-stick"), py_str("nonstick"), py_str("teflon"), py_str("coated pan"), py_str("coated"), py_str("air fryer"), py_str("air fryer basket"), py_str("rice cooker"), py_str("bread maker"), py_str("bread machine")]));
    s__HARD_STONE(py_tuple(vec![py_str("quartz"), py_str("quartz worktop"), py_str("quartz worktops"), py_str("granite"), py_str("granite worktop"), py_str("granite worktops"), py_str("granite counter"), py_str("granite top"), py_str("slate"), py_str("slate worktop"), py_str("slate floor"), py_str("slate hearth")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("acid-on-stone")); __args.push(py_tuple(vec![g__ACID(), g__STONE()])); __args.push(g__acid_on_stone()); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("make"), py_str("makes"), py_str("recipe"), py_str("mix"), py_str("homemade"), py_str("kettle"), py_str("shower"), py_str("glass"), py_str("mirror"), py_str("drain"), py_str("plughole"), py_str("grout"), py_str("toilet"), py_str("weeds"), py_str("weed"), py_str("kill"), py_str("ant"), py_str("ants"), py_str("wasp"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("remove"), py_str("removing")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bleach-on-a-mattress")); __args.push(py_tuple(vec![g__BLEACH(), g__BEDDING()])); __args.push(g__bleach_on_bedding()); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("avoid".to_string(), py_tuple(vec![py_str("cotton"), py_str("towels"), py_str("whites"), py_str("white shirt"), py_str("shirt"), py_str("school uniform"), py_str("nappy"), py_str("nappies"), py_str("nursery"), py_str("bottle"), py_str("bathroom"), py_str("toilet"), py_str("grout"), py_str("mould on the wall"), py_str("washing machine")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bleach-on-wood")); __args.push(py_tuple(vec![g__BLEACH(), py_tuple(vec![py_str("wood"), py_str("wooden"), py_str("chopping board"), py_str("chopping boards"), py_str("cutting board"), py_str("cutting boards"), py_str("table"), py_str("tables"), py_str("oak"), py_str("pine"), py_str("floorboard"), py_str("floorboards"), py_str("floor boards"), py_str("decking"), py_str("deck"), py_str("work bench"), py_str("workbench"), py_str("shed"), py_str("fence"), py_str("door")])])); __args.push(py_str("No: household bleach takes the finish off and lifts the grain into a pale, furry patch, and a board keeps the smell of it. For a real mark, sand the patch and use an oxalic-acid wood bleach from a DIY shop, following the tin. For everyday care, hot water, a scrub of coarse salt, and standing it up to dry does more than bleach ever will.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("stain"), py_str("stains"), py_str("mould"), py_str("mold"), py_str("white marks"), py_str("water marks"), py_str("ring"), py_str("polish"), py_str("varnish"), py_str("oil it"), py_str("wax")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bleach-on-skin")); __args.push(py_tuple(vec![g__BLEACH(), py_tuple(vec![py_str("cut"), py_str("cuts"), py_str("wound"), py_str("wounds"), py_str("graze"), py_str("grazed"), py_str("broken skin"), py_str("sore"), py_str("open cut"), py_str("open wound"), py_str("cut on my finger"), py_str("cut on my hand"), py_str("burn"), py_str("blister")])])); __args.push(py_str("No, never on a cut or on broken skin: bleach burns the tissue, kills the new cells and leaves the cut slower to close. Rinse it under cool running water for five minutes, press a clean cloth on it for ten, then cover it. Wash splashes off skin with soap and water. A deep, dirty or gaping cut, or one that will not stop bleeding, wants 111 or a walk-in centre today.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("white"), py_str("towels"), py_str("shirt"), py_str("cotton"), py_str("laundry"), py_str("wash"), py_str("toilet"), py_str("grout"), py_str("wood"), py_str("floor"), py_str("mattress"), py_str("nappy"), py_str("nappies"), py_str("bottle"), py_str("bathroom")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bleach-on-grout")); __args.push(py_tuple(vec![g__BLEACH(), py_tuple(vec![py_str("grout"), py_str("grouting"), py_str("tile grout"), py_str("tiles"), py_str("tile"), py_str("shower grout"), py_str("bathroom grout"), py_str("kitchen grout")])])); __args.push(py_str("Yes on white cement grout: a cup of thin bleach in four litres of water, brushed on, left fifteen minutes and rinsed, kills the mould at the root. Never on coloured or epoxy grout, which it bleaches patchy, and never mixed with a limescale remover. A stiff brush and an old toothbrush do most of the work; a bicarbonate paste lifts the rest.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("stain"), py_str("stains"), py_str("white marks"), py_str("haze"), py_str("smell"), py_str("smells"), py_str("mould on the wall"), py_str("shift"), py_str("replace"), py_str("cost"), py_str("how much does"), py_str("cleaner"), py_str("clean the tiles"), py_str("how do i clean")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("liquid-on-a-screen")); __args.push(py_tuple(vec![g__WET(), g__SCREEN()])); __args.push(py_str("A cloth, not a bottle: a microfibre cloth just damp with water, wrung out, wiped over with the screen off, then a dry corner for the streaks. No sprays straight onto a screen, because the liquid runs into the edge; no vinegar, alcohol, glass cleaner or hand sanitiser, because they take the coating off and leave the smeary patch you are trying to clean.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("how much"), py_str("how long"), py_str("time"), py_str("children"), py_str("child"), py_str("kids"), py_str("kid"), py_str("toddler"), py_str("baby"), py_str("bedtime"), py_str("sleep"), py_str("addict"), py_str("addicted"), py_str("addiction"), py_str("cracked"), py_str("smashed"), py_str("broken"), py_str("replace"), py_str("replacement"), py_str("repair"), py_str("crack"), py_str("not working"), py_str("wont turn on"), py_str("won't turn on"), py_str("black"), py_str("dead"), py_str("battery"), py_str("charge"), py_str("buy"), py_str("buying"), py_str("which one"), py_str("best one"), py_str("protector")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("abrasive-on-a-coated-pan")); __args.push(py_tuple(vec![g__ABRASIVE(), g__COATED()])); __args.push(py_str("Not with a scourer or wire wool: it takes the coating off in flakes, the pan stops being non-stick, and those flakes are what people find in the food. Use the soft side of a sponge with hot water and washing-up liquid while the pan is still warm, and a wooden or silicone spatula in it. Soak a burnt patch with water and a little bicarbonate rather than scrubbing it.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("metal taste"), py_str("taste"), py_str("tastes"), py_str("tasting"), py_str("metallic"), py_str("rust"), py_str("rusty"), py_str("warp"), py_str("warped"), py_str("handle"), py_str("loose handle"), py_str("season"), py_str("seasoning"), py_str("re-season"), py_str("reseason"), py_str("cast iron"), py_str("stainless"), py_str("buy"), py_str("buying"), py_str("which pan"), py_str("best pan"), py_str("replace"), py_str("replacing"), py_str("worth")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hand-liquid-in-a-machine")); __args.push(py_tuple(vec![g__HAND_LIQUID(), g__ANY_MACHINE()])); __args.push(py_str("No: washing-up liquid and hand soap lather, and a dishwasher or a washing machine turns a teaspoon of it into foam that comes out of the door and can reach the pump. Dishwasher tablets or powder in the dishwasher, washing powder or liquid in the washing machine, and a splash of hand liquid only for hand washing. If it has already foamed, run the machine empty with a rinse.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("hand wash"), py_str("by hand"), py_str("sink"), py_str("bowl"), py_str("washing up by hand"), py_str("cheaper"), py_str("cheapest"), py_str("save"), py_str("saving"), py_str("cost"), py_str("much"), py_str("how much"), py_str("which one"), py_str("best"), py_str("eco"), py_str("short"), py_str("tablet"), py_str("tablets"), py_str("salt"), py_str("rinse aid")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fine-in-the-dishwasher")); __args.push(py_tuple(vec![py_tuple(vec![py_str("plate"), py_str("plates"), py_str("bowl"), py_str("bowls"), py_str("mug"), py_str("mugs"), py_str("cup"), py_str("cups"), py_str("beaker"), py_str("beakers"), py_str("tumbler"), py_str("tumblers"), py_str("cutlery"), py_str("fork"), py_str("forks"), py_str("spoon"), py_str("spoons"), py_str("teaspoon"), py_str("teaspoons"), py_str("dish"), py_str("dishes"), py_str("saucepan"), py_str("saucepans"), py_str("pan"), py_str("pans"), py_str("tray"), py_str("trays"), py_str("lunchbox"), py_str("lunch box"), py_str("tupperware"), py_str("plastic box"), py_str("plastic container"), py_str("pyrex"), py_str("jug"), py_str("casserole"), py_str("mixing bowl"), py_str("plate rack")]), g__MACHINE()])); __args.push(py_str("Yes, if there is no hand-wash label on it: everyday china, plain glass, stainless cutlery and pans, and plastic that says dishwasher safe. Top rack for lids and thin plastic, and anything that holds water goes face down. Scrape rather than rinse, and leave the wooden spoons, the sharp knives and the cast iron for the sink.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("broken"), py_str("cracked"), py_str("chipped"), py_str("chips"), py_str("stained"), py_str("stains"), py_str("not cleaning"), py_str("smells"), py_str("leaking"), py_str("buy"), py_str("buying"), py_str("which"), py_str("best"), py_str("cheaper"), py_str("cheapest"), py_str("cost"), py_str("much"), py_str("worth"), py_str("how do i load"), py_str("load"), py_str("stack"), py_str("fix"), py_str("repair")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-a-dishwasher-ruins")); __args.push(py_tuple(vec![g__NOT_WASHABLE(), g__MACHINE()])); __args.push(g__what_the_machine_ruins()); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("not draining"), py_str("wont drain"), py_str("won't drain"), py_str("leaking"), py_str("leak"), py_str("smell"), py_str("smells"), py_str("not cleaning"), py_str("filter"), py_str("salt"), py_str("rinse aid"), py_str("tablet"), py_str("tablets"), py_str("powder"), py_str("how do i load"), py_str("load"), py_str("stack"), py_str("buy"), py_str("buying"), py_str("which"), py_str("cheaper"), py_str("cheapest"), py_str("cost"), py_str("much"), py_str("worth"), py_str("fix"), py_str("repair"), py_str("error"), py_str("broken"), py_str("not working"), py_str("noise"), py_str("noisy")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _bleach_on_bedding(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    text = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&_names(&[text.clone(), g__MATTRESS()])) {
    return py_str("No: the cover yellows and the foam keeps the smell of it, and bleach does not reach the sweat and the mites you are after. Vacuum both sides, dust it with bicarbonate of soda for an hour and vacuum again, and dab a stain with cool water and a little washing-up liquid. Air it with the bed stripped, and a washable protector saves the next one.");
    }
    return py_str("Not neat bleach: the cotton weakens and goes yellow, and a synthetic duvet or pillow clogs and holds the smell of it. A white cotton cover takes a laundry bleach in a hot wash at the label's temperature; anything else wants detergent at 60 and a long dry. Pillows and duvets go in a big drum with a couple of towels to balance it.");
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[modulo(&py_str("\\b%s\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), text.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _acid_on_stone(args: &[Py]) -> Py {
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
    let mut acid: Py = Py::None;
    let mut text: Py = Py::None;
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&_names(&[text.clone(), py_tuple(vec![py_str("descaler"), py_str("descaling"), py_str("limescale remover")])])) {
    acid = py_str("descaler");
    } else {
    if py_truthy(&_names(&[text.clone(), py_tuple(vec![py_str("lemon juice"), py_str("lemon")])])) {
    acid = py_str("lemon juice");
    } else {
    acid = py_str("vinegar");
    }
    }
    if py_truthy(&_names(&[text.clone(), g__HARD_STONE()])) {
    return add(&add(&py_str("That one stands it: quartz and a sealed granite or slate surface take an acid splash — "), &acid.clone()), &py_str(" — so wipe it off rather than leave it standing. Marble, limestone, travertine and onyx are the ones that etch, and the dull patch they leave never comes back. Warm water and a drop of washing-up liquid is all any of them needs."));
    }
    return add(&add(&py_str("That one etches: acid dulls a marble, limestone, travertine or onyx surface and the patch never comes back, so a wipe with "), &acid.clone()), &py_str(" is the wrong cleaner. Granite and quartz stand one now and then but do not need it. Warm water with a drop of washing-up liquid, dried off, and a pH-neutral stone cleaner for the rest."));
    Py::None
}
pub fn _what_the_machine_ruins(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The reason the thing in the person's hand cannot go in the machine.\n\n    The row answers every \"can i put X in the dishwasher\" line, and a reply that lists wood, steel\n    aluminium and crystal answers none of them well. The line is read for the material it names\n    and the reply is about that one, with the by-hand rule after it.\n    ");
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    after = py_str("By hand in hot water, and dried straight away. Stainless steel, glass, plastic boards and everyday china are fine in the machine.");
    if py_truthy(&_names(&[text.clone(), g__IRON()])) {
    return py_str("Not a seasoned pan: the wash strips the seasoning and the heat dries it to rust, so it comes out grey and sticks. Scrub it with hot water and a stiff brush while it is warm, dry it on the hob, and wipe a little oil over it. Stainless steel and glass are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__WOOD()])) {
    return py_str("Not a wooden one: it swells, splits along the grain and the glue in a board lets go, and a wooden handle soon feels rough. Wash it by hand in hot soapy water, stand it up to dry rather than flat, and rub food-safe oil into a board or a spoon now and then. Plastic boards are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__KNIFE()])) {
    return py_str("Not a knife: the edge knocks against the basket and the other cutlery, the handle can swell or lift, and the point comes out blunt. Wash it by hand in hot soapy water, dry it at once, and put it back in a block or on a magnetic strip. Stainless steel and everyday china are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__SOFT_METAL()])) {
    return py_str("Not bare aluminium or copper: the salt and the drying cycle leave it grey, pitted and chalky, and a tinned baking tray comes up patchy. By hand in hot soapy water is all it needs, and dry it at once. Stainless steel, glass and china are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__TRAPPED()])) {
    return py_str("Not that one: the water gets into a space it cannot drain out of, and it comes out rusty, chalky or sour, or with the soap still in it. Rinse it by hand in hot water and leave it to drain upside down. Plain crockery, stainless steel, glass and plastic boards are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__STONEWARE()])) {
    return py_str("Not stone: the detergent and the drying cycle take the surface off a marble mortar or a slate board and leave it dull and chalky, and a pizza stone soaks up water and can split in the oven next time. Rinse it in warm water and a stiff brush, no soap if you can, and dry it slowly. Plain crockery, stainless steel and glass are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__DELICATE()])) {
    return py_str("Not that: crystal clouds and can chip, a gold rim or a printed pattern comes off in the hot water, and anything with a trapped space holds water that rusts or smells. Wash it by hand in warm water with a drop of washing-up liquid and dry it with a cloth. Plain glass, stainless steel and everyday china are fine in the machine.");
    }
    if py_truthy(&_names(&[text.clone(), g__COATING()])) {
    return py_str("Not a coated one: the hot water and the detergent dull the coating and it starts to flake, and the same goes for an air fryer basket or a rice cooker bowl. Wash it with the soft side of a sponge and dry it before it goes back. Stainless steel, glass and china are fine in the machine.");
    }
    return py_str("Look for the dishwasher-safe mark on the base or the handle first, and wash by hand anything without one: wood swells, thin metal and a sharp edge suffer, and china with a gold rim loses it. Stainless steel, glass and everyday china are fine in the machine.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.what_damages_what".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ABRASIVE" => Some(g__ABRASIVE()),
            "_ACID" => Some(g__ACID()),
            "_ANY_MACHINE" => Some(g__ANY_MACHINE()),
            "_BEDDING" => Some(g__BEDDING()),
            "_BLEACH" => Some(g__BLEACH()),
            "_COATED" => Some(g__COATED()),
            "_COATING" => Some(g__COATING()),
            "_DELICATE" => Some(g__DELICATE()),
            "_HAND_LIQUID" => Some(g__HAND_LIQUID()),
            "_HARD_STONE" => Some(g__HARD_STONE()),
            "_HOW_ASK" => Some(g__HOW_ASK()),
            "_IRON" => Some(g__IRON()),
            "_KNIFE" => Some(g__KNIFE()),
            "_MACHINE" => Some(g__MACHINE()),
            "_MATTRESS" => Some(g__MATTRESS()),
            "_NOT_WASHABLE" => Some(g__NOT_WASHABLE()),
            "_SCREEN" => Some(g__SCREEN()),
            "_SOFT_METAL" => Some(g__SOFT_METAL()),
            "_STONE" => Some(g__STONE()),
            "_STONEWARE" => Some(g__STONEWARE()),
            "_TRAPPED" => Some(g__TRAPPED()),
            "_WET" => Some(g__WET()),
            "_WOOD" => Some(g__WOOD()),
            "_acid_on_stone" => Some(py_func("_acid_on_stone", |args: &[Py]| _acid_on_stone(args))),
            "_bleach_on_bedding" => Some(py_func("_bleach_on_bedding", |args: &[Py]| _bleach_on_bedding(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_what_the_machine_ruins" => Some(py_func("_what_the_machine_ruins", |args: &[Py]| _what_the_machine_ruins(args))),
            _ => None,
        }),
        set: None,
    }))
}
