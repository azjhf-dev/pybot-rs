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
thread_local! { static G_ROAST_MINUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROAST_NOT_FOR_US: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROAST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULE_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_ASK_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_BRASSICAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_COOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_COOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_EXTRA_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_GENERIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_HOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_HOW_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_HOW_EXTRA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_LEAVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_NAMED_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_NOT_FOR_US: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_PODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_PREVENTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_RAW_FAULTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_ROASTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_SOFT_FAULTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_SOFT_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_WET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RULE_OUT_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ROAST_MINUTES() -> Py { ensure_init(); G_ROAST_MINUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROAST_MINUTES(v: Py) { ensure_init(); G_ROAST_MINUTES.with(|c| *c.borrow_mut() = v); }
pub fn g_ROAST_NOT_FOR_US() -> Py { ensure_init(); G_ROAST_NOT_FOR_US.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROAST_NOT_FOR_US(v: Py) { ensure_init(); G_ROAST_NOT_FOR_US.with(|c| *c.borrow_mut() = v); }
pub fn g_ROAST_WORDS() -> Py { ensure_init(); G_ROAST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROAST_WORDS(v: Py) { ensure_init(); G_ROAST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RULE_OUT() -> Py { ensure_init(); G_RULE_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULE_OUT(v: Py) { ensure_init(); G_RULE_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_ASKS() -> Py { ensure_init(); G_VEG_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_ASKS(v: Py) { ensure_init(); G_VEG_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_ASK_BETWEEN() -> Py { ensure_init(); G_VEG_ASK_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_ASK_BETWEEN(v: Py) { ensure_init(); G_VEG_ASK_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_BRASSICAS() -> Py { ensure_init(); G_VEG_BRASSICAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_BRASSICAS(v: Py) { ensure_init(); G_VEG_BRASSICAS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_COOK() -> Py { ensure_init(); G_VEG_COOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_COOK(v: Py) { ensure_init(); G_VEG_COOK.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_COOKED() -> Py { ensure_init(); G_VEG_COOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_COOKED(v: Py) { ensure_init(); G_VEG_COOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_EXTRA_AVOID() -> Py { ensure_init(); G_VEG_EXTRA_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_EXTRA_AVOID(v: Py) { ensure_init(); G_VEG_EXTRA_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_FILLER() -> Py { ensure_init(); G_VEG_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_FILLER(v: Py) { ensure_init(); G_VEG_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_GENERIC() -> Py { ensure_init(); G_VEG_GENERIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_GENERIC(v: Py) { ensure_init(); G_VEG_GENERIC.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_HOW() -> Py { ensure_init(); G_VEG_HOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_HOW(v: Py) { ensure_init(); G_VEG_HOW.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_HOW_ASKS() -> Py { ensure_init(); G_VEG_HOW_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_HOW_ASKS(v: Py) { ensure_init(); G_VEG_HOW_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_HOW_EXTRA() -> Py { ensure_init(); G_VEG_HOW_EXTRA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_HOW_EXTRA(v: Py) { ensure_init(); G_VEG_HOW_EXTRA.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_LEAVES() -> Py { ensure_init(); G_VEG_LEAVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_LEAVES(v: Py) { ensure_init(); G_VEG_LEAVES.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_NAMED_METHOD() -> Py { ensure_init(); G_VEG_NAMED_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_NAMED_METHOD(v: Py) { ensure_init(); G_VEG_NAMED_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_NOT_FOR_US() -> Py { ensure_init(); G_VEG_NOT_FOR_US.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_NOT_FOR_US(v: Py) { ensure_init(); G_VEG_NOT_FOR_US.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_OTHER() -> Py { ensure_init(); G_VEG_OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_OTHER(v: Py) { ensure_init(); G_VEG_OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_PODS() -> Py { ensure_init(); G_VEG_PODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_PODS(v: Py) { ensure_init(); G_VEG_PODS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_PREVENTION() -> Py { ensure_init(); G_VEG_PREVENTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_PREVENTION(v: Py) { ensure_init(); G_VEG_PREVENTION.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_RAW_FAULTS() -> Py { ensure_init(); G_VEG_RAW_FAULTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_RAW_FAULTS(v: Py) { ensure_init(); G_VEG_RAW_FAULTS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_ROASTED() -> Py { ensure_init(); G_VEG_ROASTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_ROASTED(v: Py) { ensure_init(); G_VEG_ROASTED.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_SOFT_FAULTS() -> Py { ensure_init(); G_VEG_SOFT_FAULTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_SOFT_FAULTS(v: Py) { ensure_init(); G_VEG_SOFT_FAULTS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_SOFT_NAMES() -> Py { ensure_init(); G_VEG_SOFT_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_SOFT_NAMES(v: Py) { ensure_init(); G_VEG_SOFT_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_TIME() -> Py { ensure_init(); G_VEG_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_TIME(v: Py) { ensure_init(); G_VEG_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_TIME_WORDS() -> Py { ensure_init(); G_VEG_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_TIME_WORDS(v: Py) { ensure_init(); G_VEG_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_WET() -> Py { ensure_init(); G_VEG_WET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_WET(v: Py) { ensure_init(); G_VEG_WET.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_FAMILIES() -> Py { ensure_init(); G_WATER_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_FAMILIES(v: Py) { ensure_init(); G_WATER_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_WORDS() -> Py { ensure_init(); G_WATER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_WORDS(v: Py) { ensure_init(); G_WATER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__RULE_OUT_REACH() -> Py { ensure_init(); G__RULE_OUT_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RULE_OUT_REACH(v: Py) { ensure_init(); G__RULE_OUT_REACH.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How long a vegetable takes, and what \"done\" looks like when it gets there.\n\nOne domain of its own so its many vegetable names cannot change how rare a word is inside\nanother domain's keys (lookup.entries points that out: a new domain may not take a point away\nfrom an entry somewhere else).\n\nThe boiling and steaming times of the vegetables `cooking.py` does not already answer:\ncauliflower, carrots, cabbage, swede, leek, courgette, asparagus, sweetcorn, spinach, kale,\nbutternut squash, celeriac, mange tout, beetroot, turnip, fennel, pak choi and spring greens.\n\"how long does cauliflower take to boil\" was a fallback before this table.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_VEG_TIME(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("cauliflower")]), py_str("6 to 8 minutes for florets in already boiling salted water, 12 to 15 for a whole small one, and it is done when a knife goes into the stalk with no resistance. Steamed, give it 2 or 3 minutes more.")]), py_tuple(vec![py_tuple(vec![py_str("carrot")]), py_str("10 to 15 minutes for thick coins, 8 for thin ones, 20 for whole carrots, in already boiling salted water. They are done when a knife slides in easily - a little bite is better than soft, and they go on cooking once drained.")]), py_tuple(vec![py_tuple(vec![py_str("cabbage")]), py_str("5 to 8 minutes for shredded cabbage in already boiling salted water, and less for the loose outer leaves. It is done when it has wilted and lost its raw edge but still has some bite; overcook it and it turns sweet and grey.")]), py_tuple(vec![py_tuple(vec![py_str("swede")]), py_str("25 to 30 minutes in 3cm chunks in boiling salted water, or 40 to 50 minutes whole. The knife test is the one that counts: it should go in with no resistance at all, because slightly underdone swede is hard and watery.")]), py_tuple(vec![py_tuple(vec![py_str("leek")]), py_str("6 to 8 minutes in thick rings, 3 to 4 if you slice them thin, in boiling salted water. Rinse the grit out from between the layers first - leeks hold soil better than any other vegetable - and stop while they still hold their shape.")]), py_tuple(vec![py_tuple(vec![py_str("courgette")]), py_str("3 to 4 minutes in boiling salted water, and no more: it is mostly water, so it goes from just right to mush fast. Better still, fry thick slices in a little oil for the same time - boiled courgette tastes of very little.")]), py_tuple(vec![py_tuple(vec![py_str("asparagus")]), py_str("3 to 5 minutes depending on thickness, standing up in a pan with the tips out of the water if you can, or in a wide pan. It is done when a knife goes into the stalk - bend one and it should still have some spring.")]), py_tuple(vec![py_tuple(vec![py_str("globe artichoke"), py_str("globe artichokes"), py_str("artichoke"), py_str("artichokes")]), py_str("25 to 35 minutes whole in already boiling salted water, standing up in a deep pan with the tips out of the water if they fit. It is done when a leaf near the middle pulls away easily; drain it upside down so the water runs out of the leaves.")]), py_tuple(vec![py_tuple(vec![py_str("jerusalem artichoke"), py_str("jerusalem artichokes"), py_str("sunchoke"), py_str("sunchokes")]), py_str("10 to 12 minutes in already boiling salted water, scrubbed and cut into even pieces, and it is done when a knife goes in with no resistance at all. Roasting suits it better than boiling - 25 to 30 minutes at 200C (180C fan) - because it goes sweet and nutty at the edges.")]), py_tuple(vec![py_tuple(vec![py_str("sweetcorn"), py_str("corn on the cob"), py_str("corn cob")]), py_str("5 to 8 minutes for a cob in boiling unsalted water (salt makes the kernels tough), 4 to 5 from frozen. It is done when the kernels are bright yellow and a sharp knife cuts one easily.")]), py_tuple(vec![py_tuple(vec![py_str("spinach")]), py_str("1 to 2 minutes, and that is generous: put it in boiling water, push it under, and take it out as soon as it has wilted. Spinach is nearly all water, so longer only means losing it - squeeze it out and season it after.")]), py_tuple(vec![py_tuple(vec![py_str("kale")]), py_str("3 to 5 minutes in boiling salted water, stalky pieces first, and 6 to 8 if you want it soft enough for a soup. Strip the tough stems out before it goes in, and drain it well - kale holds a lot of water.")]), py_tuple(vec![py_tuple(vec![py_str("butternut squash"), py_str("squash")]), py_str("15 to 20 minutes in 3cm chunks in boiling salted water, or 25 to 30 in bigger pieces. It is done when a knife slides in with no resistance; roast it instead if you want it sweet and caramelised rather than watery.")]), py_tuple(vec![py_tuple(vec![py_str("celeriac")]), py_str("15 to 20 minutes in 2cm chunks in boiling salted water with a squeeze of lemon, which keeps it pale. It is done when a knife slides in easily, and it makes a good mash with butter and plenty of pepper.")]), py_tuple(vec![py_tuple(vec![py_str("mange tout"), py_str("mangetout"), py_str("sugar snap")]), py_str("2 to 3 minutes in boiling salted water, then straight into cold water so they stay bright. They are done when they are hot through and still squeak when you bite one - the pod is the point.")]), py_tuple(vec![py_tuple(vec![py_str("french beans"), py_str("green beans"), py_str("runner beans"), py_str("string beans")]), py_str("4 to 6 minutes for french beans, 5 to 7 for runner beans, in boiling salted water. They are done when a bean bends without snapping; top and tail them and string the runners first.")]), py_tuple(vec![py_tuple(vec![py_str("broad beans"), py_str("broad bean"), py_str("fava beans")]), py_str("2 to 3 minutes for small ones and 4 to 5 for big ones, in boiling salted water, then slip the grey skins off if they are big. They are done when a bean is tender and tastes sweet, not floury.")]), py_tuple(vec![py_tuple(vec![py_str("kohlrabi")]), py_str("12 to 15 minutes in wedges or 20 to 25 whole, peeled, in boiling salted water - it is done when a knife slides into the middle with no resistance. Steaming takes the same time and keeps more of the sweetness.")]), py_tuple(vec![py_tuple(vec![py_str("beetroot")]), py_str("45 to 60 minutes whole and unpeeled in boiling water, 25 to 30 if you cut it into wedges. A knife should slide in with no resistance, and the skin rubs off under cold water once it is cool enough to handle.")]), py_tuple(vec![py_tuple(vec![py_str("turnip")]), py_str("15 to 20 minutes in 3cm chunks in boiling salted water, and 25 to 30 whole. It is done when a knife goes in easily; drain it well, because turnip holds water and a wet mash is a sad mash.")]), py_tuple(vec![py_tuple(vec![py_str("fennel")]), py_str("8 to 10 minutes in wedges in boiling salted water, cut through the root so they hold together. It is done when a knife goes into the thickest part; boiled fennel goes sweet and soft, and a squeeze of lemon at the end keeps it fresh.")]), py_tuple(vec![py_tuple(vec![py_str("pak choi"), py_str("pak choy"), py_str("bok choy")]), py_str("2 to 3 minutes, halved lengthways, in boiling salted water - the leaves wilt almost at once. Better in a hot pan with a little oil, garlic and soy for the same 3 minutes.")]), py_tuple(vec![py_tuple(vec![py_str("spring greens"), py_str("collard"), py_str("greens")]), py_str("4 to 6 minutes shredded in boiling salted water, stalky bits first. They are done when they are tender but still green; drain them well and dress with butter or a little oil.")])]));
    s_VEG_HOW(py_dict(vec![(py_str("carrot"), py_str("peel or just scrub them, cut into batons, and boil or steam for 6 to 8 minutes until a knife goes in easily. Roast batons at 200C (180C fan) for 25 to 30 minutes with oil and a little honey. Whole small ones take 10 to 12 minutes boiled, and older carrots are sweeter and take longer.")), (py_str("cabbage"), py_str("shred it and cook it briefly, or it goes grey and soft: boil or steam 4 to 5 minutes, or fry it in butter for 5 to 6 with a splash of water and the lid on for the first half. Salt at the end. Red cabbage is the exception and wants a slow braise of 45 minutes or more, with vinegar and something sweet.")), (py_str("spinach"), py_str("it needs seconds, not minutes: wash it, put it in a hot pan with a knob of butter and stir for 1 to 2 minutes until it has just collapsed, then drain off the water it gives. Boiling it makes it slimy; squeeze it dry and dress it with butter, nutmeg or a little garlic.")), (py_str("asparagus"), py_str("snap off the woody end of each spear (it breaks where it should), then boil or steam 3 to 4 minutes for thin spears or 5 to 6 for fat ones, or roast at 200C (180C fan) for 10 to 12 minutes with oil, salt and lemon. It is done when a knife goes in with no crunch left.")), (py_str("leek"), py_str("slice it first, then wash it: grit hides between the layers, so swill the slices in a bowl of water and lift them out. Sweat them in butter with the lid on for 8 to 10 minutes, stirring now and then, until soft but not browned, and a splash of water stops them catching. They go sweet, and they are the base of potato and leek soup.")), (py_str("courgette"), py_str("slice them about 1cm thick, salt them for 10 minutes and pat them dry if you want them firm, then fry in a hot pan for 3 to 4 minutes a side until golden. To roast, toss in oil and bake at 200C (180C fan) for 20 minutes. Don't crowd the pan: they steam rather than brown.")), (py_str("kale"), py_str("strip the leaves off the stalks, then either steam 4 to 5 minutes until tender, or toss with oil and salt and roast at 180C (160C fan) for 12 to 15 minutes until the edges crisp. Raw kale is fine in a salad if you massage it with oil and lemon for a minute first, which softens it.")), (py_str("swede"), py_str("peel and cube it, then boil for 20 to 25 minutes until it mashes easily, and mash it with butter, salt and pepper - a little cream or a knob of butter takes away the sharpness. It roasts well too, in 2cm cubes at 200C (180C fan) for 30 to 35 minutes.")), (py_str("globe artichoke"), py_str("cut the top inch off, snip the points off the outer leaves, and boil or steam 25 to 35 minutes until a leaf near the middle pulls away easily. Eat the leaves dipped in butter or vinaigrette, then scrape out the fuzzy choke and eat the heart. To roast, halve them, oil the cut sides and pack them cut down in a dish at 200C (180C fan) for 25 to 30 minutes with a splash of water and a lid for the first half.")), (py_str("jerusalem artichoke"), py_str("scrub them instead of peeling, since the skin is thin, and roast them 25 to 30 minutes at 200C (180C fan) in oil, or boil 10 to 12 minutes and mash them with butter like potato: they go sweet and nutty fast, so test early. A squeeze of lemon or a splash of vinegar keeps them from discolouring, and start with a small portion, because they earn their other name."))]));
    s_VEG_HOW_EXTRA(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("green beans"), py_str("runner beans"), py_str("french beans"), py_str("string beans"), py_str("fine beans")]), py_str("top and tail them, and slice runner beans thinly, then boil or steam 4 to 5 minutes for green beans or 5 to 7 for runner beans: they should still squeak. Drain them and toss with butter or a little garlic. Old runner beans have strings and hard seeds, so slice them fine and cook them a few minutes longer.")]), py_tuple(vec![py_tuple(vec![py_str("mushroom"), py_str("mushrooms"), py_str("button mushrooms"), py_str("chestnut mushrooms")]), py_str("get the pan hot and add them dry first for a minute so they give up their water, then add butter or oil and cook 4 to 5 more minutes until golden. Wipe or rinse them quickly and dry them - either is fine - and don't crowd the pan or they boil in their own juice. Season at the end, not the start.")])]));
    s_VEG_HOW_ASKS(py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("prepare"), py_str("prep"), py_str("prepare"), py_str("boil"), py_str("boiled"), py_str("boiling"), py_str("steam"), py_str("steamed"), py_str("steaming"), py_str("best way"), py_str("method"), py_str("properly")]));
    s_VEG_NAMED_METHOD(py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("oven"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("grill"), py_str("grilled"), py_str("air fryer"), py_str("airfryer"), py_str("microwave"), py_str("barbecue"), py_str("bbq"), py_str("slow cooker"), py_str("pressure cooker")]));
    s_VEG_OTHER(py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("oven"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("grill"), py_str("grilled"), py_str("air fryer"), py_str("microwave"), py_str("barbecue"), py_str("bbq"), py_str("soup"), py_str("puree"), py_str("mash"), py_str("fridge"), py_str("freezer"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("leftovers"), py_str("leftover"), py_str("calories"), py_str("calorie"), py_str("healthy"), py_str("health"), py_str("protein"), py_str("carb"), py_str("carbs"), py_str("vitamin"), py_str("vitamins"), py_str("iron"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("rabbit"), py_str("plant"), py_str("plants"), py_str("planting"), py_str("grow"), py_str("growing"), py_str("garden"), py_str("allotment"), py_str("seed"), py_str("seeds"), py_str("buy"), py_str("buying"), py_str("cost"), py_str("price"), py_str("cheap"), py_str("shop"), py_str("supermarket"), py_str("market"), py_str("recipe"), py_str("allergy"), py_str("allergic"), py_str("baby"), py_str("weaning"), py_str("blw"), py_str("diet"), py_str("diabetes"), py_str("sugar"), py_str("barely"), py_str("raw"), py_str("purple"), py_str("yellow"), py_str("brown"), py_str("mouldy"), py_str("gone off"), py_str("gone soft"), py_str("went soft"), py_str("gone limp"), py_str("limp"), py_str("bendy"), py_str("rubbery"), py_str("sprouting"), py_str("sprouted"), py_str("wilted"), py_str("wilt"), py_str("slimy"), py_str("spaghetti squash"), py_str("acorn squash"), py_str("pattypan")]));
    s_VEG_TIME_WORDS(py_tuple(vec![py_str("how long"), py_str("long"), py_str("minutes"), py_str("mins"), py_str("minute"), py_str("time"), py_str("times"), py_str("hour"), py_str("hours"), py_str("tender"), py_str("done"), py_str("ready"), py_str("take"), py_str("takes"), py_str("need"), py_str("needs")]));
    s_VEG_NOT_FOR_US(py_tuple(vec![py_str("fridge"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("leftovers"), py_str("leftover"), py_str("calories"), py_str("calorie"), py_str("healthy"), py_str("health"), py_str("protein"), py_str("carb"), py_str("carbs"), py_str("vitamin"), py_str("vitamins"), py_str("iron"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("rabbit"), py_str("plant"), py_str("plants"), py_str("planting"), py_str("grow"), py_str("growing"), py_str("garden"), py_str("allotment"), py_str("seed"), py_str("seeds"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("cost"), py_str("price"), py_str("cheap"), py_str("shop"), py_str("supermarket"), py_str("market"), py_str("allergy"), py_str("allergic"), py_str("baby"), py_str("weaning"), py_str("diet"), py_str("diabetes"), py_str("sugar"), py_str("mouldy"), py_str("gone off"), py_str("gone soft"), py_str("went soft"), py_str("gone limp"), py_str("limp"), py_str("bendy"), py_str("rubbery"), py_str("wilted"), py_str("wilting"), py_str("slimy"), py_str("are sprouting"), py_str("is sprouting"), py_str("have sprouted"), py_str("has sprouted"), py_str("gone to seed"), py_str("harvest"), py_str("harvesting"), py_str("pick"), py_str("picking"), py_str("picked"), py_str("ripe"), py_str("ripen"), py_str("ripened"), py_str("when are"), py_str("are my"), py_str("are they ready"), py_str("ready to pick"), py_str("soak"), py_str("soaks"), py_str("soaking"), py_str("soaked"), py_str("overnight"), py_str("dried"), py_str("dry"), py_str("dont like"), py_str("don't like"), py_str("do not like"), py_str("doesnt like"), py_str("doesn't like"), py_str("do not like them"), py_str("do not like it"), py_str("does not like them"), py_str("hate"), py_str("hates"), py_str("hated"), py_str("cant stand"), py_str("can't stand"), py_str("not a fan"), py_str("fussy"), py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("wont touch"), py_str("won't touch")]));
    s_VEG_ASK_BETWEEN(py_tuple(vec![py_str("how"), py_str("long"), py_str("time"), py_str("times"), py_str("minutes"), py_str("mins"), py_str("take"), py_str("takes")]));
    s_VEG_FILLER(py_tuple(vec![py_str("take"), py_str("takes"), py_str("took"), py_str("to"), py_str("for"), py_str("in"), py_str("on"), py_str("the"), py_str("a"), py_str("an"), py_str("oven"), py_str("when"), py_str("at"), py_str("should"), py_str("i"), py_str("you"), py_str("do"), py_str("need"), py_str("long"), py_str("how")]));
    s_VEG_COOK(py_tuple(vec![py_str("boil"), py_str("boiled"), py_str("boiling"), py_str("steam"), py_str("steamed"), py_str("steaming"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("grill"), py_str("grilled"), py_str("simmer"), py_str("simmered")]));
    s_VEG_ASKS(py_tuple(vec![py_str("boil"), py_str("boiled"), py_str("boiling"), py_str("steam"), py_str("steamed"), py_str("steaming"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("how long"), py_str("long"), py_str("time"), py_str("times"), py_str("minutes"), py_str("mins"), py_str("tender"), py_str("done"), py_str("ready"), py_str("take"), py_str("prepare"), py_str("prep"), py_str("best way"), py_str("method"), py_str("properly")]));
    s_RULE_OUT(py_tuple(vec![py_str("no"), py_str("not"), py_str("without"), py_str("dont"), py_str("don't"), py_str("doesnt"), py_str("doesn't"), py_str("didnt"), py_str("didn't"), py_str("hate"), py_str("hates"), py_str("hated"), py_str("dislike"), py_str("dislikes"), py_str("allergic"), py_str("intolerant"), py_str("avoid"), py_str("avoids"), py_str("avoiding"), py_str("free"), py_str("off"), py_str("skip"), py_str("skipping"), py_str("except")]));
    s__RULE_OUT_REACH(Py::Int(4));
    s_ROAST_MINUTES(py_dict(vec![(py_str("carrot"), py_tuple(vec![py_str("25 to 30"), py_str("halved lengthways or cut into batons")])), (py_str("parsnip"), py_tuple(vec![py_str("30 to 35"), py_str("cut into wedges, and they want a little honey or sugar to brown")])), (py_str("leek"), py_tuple(vec![py_str("25 to 30"), py_str("halved lengthways and laid cut side down")])), (py_str("cabbage"), py_tuple(vec![py_str("25 to 30"), py_str("cut into wedges through the core so they hold together")])), (py_str("sprouts"), py_tuple(vec![py_str("25 to 30"), py_str("halved and laid cut side down")])), (py_str("brussels sprouts"), py_tuple(vec![py_str("25 to 30"), py_str("halved and laid cut side down")])), (py_str("green beans"), py_tuple(vec![py_str("12 to 15"), py_str("kept whole")])), (py_str("french beans"), py_tuple(vec![py_str("12 to 15"), py_str("kept whole")])), (py_str("runner beans"), py_tuple(vec![py_str("12 to 15"), py_str("kept whole")])), (py_str("broccoli"), py_tuple(vec![py_str("15 to 18"), py_str("cut into florets with the stalk split")])), (py_str("courgette"), py_tuple(vec![py_str("18 to 20"), py_str("halved lengthways")])), (py_str("onion"), py_tuple(vec![py_str("30 to 35"), py_str("cut into wedges through the root")])), (py_str("beetroot"), py_tuple(vec![py_str("50 to 60"), py_str("left whole and unpeeled in foil")])), (py_str("swede"), py_tuple(vec![py_str("35 to 40"), py_str("cut into 3cm chunks")])), (py_str("squash"), py_tuple(vec![py_str("30 to 35"), py_str("cut into 3cm chunks")])), (py_str("butternut squash"), py_tuple(vec![py_str("30 to 35"), py_str("cut into 3cm chunks")])), (py_str("pepper"), py_tuple(vec![py_str("20 to 25"), py_str("cut into thick strips")])), (py_str("peppers"), py_tuple(vec![py_str("20 to 25"), py_str("cut into thick strips")])), (py_str("fennel"), py_tuple(vec![py_str("25 to 30"), py_str("cut into wedges through the root")])), (py_str("kohlrabi"), py_tuple(vec![py_str("30 to 35"), py_str("cut into wedges")])), (py_str("turnip"), py_tuple(vec![py_str("30 to 35"), py_str("cut into 3cm chunks")])), (py_str("celeriac"), py_tuple(vec![py_str("30 to 35"), py_str("cut into 3cm chunks")])), (py_str("artichoke"), py_tuple(vec![py_str("25 to 30"), py_str("halved, oiled on the cut side and packed cut down in a dish")])), (py_str("globe artichoke"), py_tuple(vec![py_str("25 to 30"), py_str("halved, oiled on the cut side and packed cut down in a dish")])), (py_str("jerusalem artichoke"), py_tuple(vec![py_str("25 to 30"), py_str("scrubbed, halved if large, in a single layer")])), (py_str("jerusalem artichokes"), py_tuple(vec![py_str("25 to 30"), py_str("scrubbed, halved if large, in a single layer")])), (py_str("kale"), py_tuple(vec![py_str("10 to 12"), py_str("torn off the stalk and tossed in oil, for the crispness rather than a side")])), (py_str("aubergine"), py_tuple(vec![py_str("25 to 30"), py_str("cut into 3cm chunks")])), (py_str("tomato"), py_tuple(vec![py_str("15 to 20"), py_str("halved, cut side up")])), (py_str("red onion"), py_tuple(vec![py_str("30 to 35"), py_str("cut into wedges through the root")])), (py_str("mushroom"), py_tuple(vec![py_str("15 to 20"), py_str("left whole if small and halved if large, with room between them because crowded mushrooms steam instead of browning")])), (py_str("mushrooms"), py_tuple(vec![py_str("15 to 20"), py_str("left whole if small and halved if large, with room between them because crowded mushrooms steam instead of browning")])), (py_str("asparagus"), py_tuple(vec![py_str("10 to 12"), py_str("kept whole; they brown rather than bend")])), (py_str("sweetcorn"), py_tuple(vec![py_str("25 to 30"), py_str("cobs turned once half way; loose kernels take 12 to 15")])), (py_str("corn on the cob"), py_tuple(vec![py_str("25 to 30"), py_str("cobs turned once half way; loose kernels take 12 to 15")])), (py_str("shallots"), py_tuple(vec![py_str("25 to 30"), py_str("left whole and unpeeled, or halved through the root")])), (py_str("garlic"), py_tuple(vec![py_str("25 to 30"), py_str("a whole head with the top sliced off and wrapped in foil, then squeezed out of its skins")])), (py_str("radish"), py_tuple(vec![py_str("15 to 20"), py_str("halved - the heat goes and they come out sweet and mild")])), (py_str("romanesco"), py_tuple(vec![py_str("20 to 25"), py_str("cut into florets like a cauliflower, and quicker because the florets are smaller")])), (py_str("spring onion"), py_tuple(vec![py_str("8 to 10"), py_str("kept whole; they scorch at the tips before the middle softens")])), (py_str("spring onions"), py_tuple(vec![py_str("8 to 10"), py_str("kept whole; they scorch at the tips before the middle softens")])), (py_str("pak choi"), py_tuple(vec![py_str("8 to 10"), py_str("halved lengthways and laid cut side down")])), (py_str("tenderstem broccoli"), py_tuple(vec![py_str("12 to 15"), py_str("kept whole, stalks and all")])), (py_str("chicory"), py_tuple(vec![py_str("20 to 25"), py_str("halved through the root so the leaves hold together")])), (py_str("okra"), py_tuple(vec![py_str("12 to 15"), py_str("left whole with the tops trimmed, and dry so it does not go slimy")])), (py_str("mangetout"), py_tuple(vec![py_str("8 to 10"), py_str("kept whole and turned once; they are done when the pods blister")]))]));
    s_ROAST_WORDS(py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("roasts"), py_str("oven"), py_str("baked"), py_str("baking"), py_str("bake"), py_str("air fryer"), py_str("in the oven"), py_str("in an oven"), py_str("in my oven"), py_str("tray bake")]));
    s_ROAST_NOT_FOR_US(py_tuple(vec![py_str("boil"), py_str("boiled"), py_str("boiling"), py_str("steam"), py_str("steamed"), py_str("steaming"), py_str("parboil"), py_str("parboiled"), py_str("blanch"), py_str("blanched"), py_str("microwave"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("grill"), py_str("grilled"), py_str("soup"), py_str("mash"), py_str("puree"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("calories"), py_str("plant"), py_str("plants"), py_str("grow"), py_str("growing"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("buy"), py_str("cost"), py_str("price"), py_str("cheap"), py_str("shop"), py_str("gone soft"), py_str("gone limp"), py_str("mouldy"), py_str("slimy"), py_str("wilted"), py_str("sprouting")]));
    s_VEG_EXTRA_AVOID(py_dict(vec![(py_str("globe artichoke"), py_tuple(vec![py_str("jerusalem"), py_str("jerusalem artichoke"), py_str("jerusalem artichokes"), py_str("sunchoke"), py_str("sunchokes")]))]));
    s_ENTRIES(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_TIME()) { let __unpk229 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk229[0].clone(); let mut said: Py = __unpk229[1].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&names.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _slug(args))(&__args) }); __args.push(py_tuple(vec![names.clone(), g_VEG_ASKS()])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&names.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_VEG_HOW(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reply_for(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&g_VEG_NOT_FOR_US(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&names.clone(), &Py::Int(0))); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_VEG_EXTRA_AVOID(), "get", args))(&__args) })), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_HOW_EXTRA()) { let __unpk7156 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk7156[0].clone(); let mut how: Py = __unpk7156[1].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&names.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _slug(args))(&__args) }); __args.push(py_tuple(vec![names.clone(), g_VEG_ASKS()])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __args.push(Py::None); __args.push(how.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reply_for(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), g_VEG_NOT_FOR_US()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_ENTRIES(op_add(&g_ENTRIES(), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("veg-roast")); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ROAST_MINUTES()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_tuple(vec![py_str("in the oven"), py_str("oven"), py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("air fryer"), py_str("how long"), py_str("long"), py_str("minutes"), py_str("time"), py_str("roasting time"), py_str("roast time"), py_str("time in the oven"), py_str("oven time"), py_str("how long in the oven")])])); __args.push(g__roast_reply()); __kws.extend(vec![("avoid".to_string(), g_VEG_NOT_FOR_US()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }])));
    s_ENTRIES(op_add(&g_ENTRIES(), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("veg-chips-crisp")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chips"), py_str("chip"), py_str("fries"), py_str("french fries"), py_str("oven chips"), py_str("homemade chips"), py_str("chips and fries"), py_str("wedges"), py_str("potato wedges"), py_str("wedge")]), py_tuple(vec![py_str("soggy"), py_str("soft"), py_str("limp"), py_str("floppy"), py_str("crispy"), py_str("crisp"), py_str("crispier"), py_str("crisp up"), py_str("go crispy"), py_str("not crispy"), py_str("not crisp"), py_str("wont crisp"), py_str("won't crisp")])])); __args.push(py_str("Soggy chips are a steam problem, so the fix is dry and hot: pat fresh-cut chips dry (rinse the starch off first if you cut them yourself) and put frozen ones in straight from the freezer, then get the oven and the tray properly hot - 200C (180C fan) - and spread them in one layer with space around each chip, because a crowded tray steams instead of crisping. Salt them as they come out and not before, since salt pulls water to the surface. If they still come out limp, cook them twice: 10 minutes at 160C to soften the middle, then 8 minutes at 190C to crisp the outside, with a few minutes resting in between.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("temperature"), py_str("how much"), py_str("portion"), py_str("portions"), py_str("price"), py_str("calorie"), py_str("calories"), py_str("healthy"), py_str("unhealthy"), py_str("freeze"), py_str("refreeze"), py_str("defrost"), py_str("reheat"), py_str("leftover"), py_str("left over"), py_str("goes with"), py_str("with what"), py_str("chip shop"), py_str("takeaway"), py_str("roast potato"), py_str("roast potatoes"), py_str("roasties"), py_str("chicken"), py_str("fish"), py_str("prawn"), py_str("prawns"), py_str("halloumi"), py_str("tofu"), py_str("nugget"), py_str("nuggets"), py_str("goujon"), py_str("goujons"), py_str("schnitzel"), py_str("katsu"), py_str("scotch egg"), py_str("onion ring"), py_str("onion rings"), py_str("spring roll"), py_str("spring rolls"), py_str("batter"), py_str("battered"), py_str("breaded"), py_str("breadcrumb"), py_str("breadcrumbs"), py_str("panko"), py_str("tempura")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fried-crisp")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("fish"), py_str("prawn"), py_str("prawns"), py_str("halloumi"), py_str("nugget"), py_str("nuggets"), py_str("goujons"), py_str("schnitzel"), py_str("katsu"), py_str("scotch egg"), py_str("onion rings"), py_str("spring rolls"), py_str("batter"), py_str("battered"), py_str("coated"), py_str("panko"), py_str("tempura"), py_str("fried")]), py_tuple(vec![py_str("crispy"), py_str("crisp"), py_str("crunchy"), py_str("crunch"), py_str("soggy"), py_str("soft"), py_str("limp"), py_str("goes soft"), py_str("gone soft"), py_str("not crispy"), py_str("wont crisp"), py_str("won't crisp"), py_str("stay crisp"), py_str("keep crisp")])])); __args.push(py_str("Three things keep a fried coating crisp: a dry surface, oil that is hot enough, and air under it once it is out. Pat the food dry and flour it before the egg and crumbs, because a wet surface steams from the inside; fry at about 175C to 180C, which is a cube of bread browning in 60 seconds, since cooler oil soaks in and hotter oil browns the outside before the middle is done. Drain it on a rack over a tray rather than on kitchen paper, which traps the steam against the underside, and never put a lid or foil over the plate: serve it straight away, or hold it uncovered in a 100C oven on a rack, which keeps the coating dry for about 20 minutes. If you are cooking in batches, do not pile the finished pieces on top of each other.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("temperature"), py_str("how much"), py_str("portion"), py_str("portions"), py_str("calorie"), py_str("calories"), py_str("healthy"), py_str("unhealthy"), py_str("freeze"), py_str("refreeze"), py_str("defrost"), py_str("reheat"), py_str("leftover"), py_str("left over"), py_str("goes with"), py_str("chip"), py_str("chips"), py_str("fries"), py_str("roast potato"), py_str("roast potatoes"), py_str("pizza"), py_str("recipe"), py_str("marinade"), py_str("marinate"), py_str("bake"), py_str("roast"), py_str("air fryer"), py_str("sous vide"), py_str("raw"), py_str("pink"), py_str("safe"), py_str("done"), py_str("tofu"), py_str("pressed tofu"), py_str("freezer"), py_str("frozen"), py_str("freeze"), py_str("thaw"), py_str("thawed"), py_str("thawing"), py_str("defrost"), py_str("defrosted"), py_str("defrosting"), py_str("refreeze"), py_str("refrozen"), py_str("last night"), py_str("still ok"), py_str("still okay"), py_str("safe to eat"), py_str("gone off"), py_str("went off")])), ("bias".to_string(), Py::Int(5))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }])));
    s_WATER_WORDS(py_tuple(vec![py_str("soggy"), py_str("going soggy"), py_str("goes soggy"), py_str("gone soggy"), py_str("went soggy"), py_str("go soggy"), py_str("soggy when"), py_str("watery"), py_str("watery when"), py_str("leaking water"), py_str("leaks water"), py_str("gives up water"), py_str("let out water"), py_str("mushy"), py_str("wet")]));
    s_WATER_FAMILIES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("courgette"), py_str("courgettes"), py_str("zucchini")]), py_str("Courgette goes soggy because it is mostly water, so the water has to come out before it hits the pan: slice it about 1cm thick, salt it for 10 to 15 minutes, pat it dry, then cook it in a wide hot pan in one layer with no lid, or roast it at 220C spread out. Crowding the pan steams it, and a lid guarantees it - and if it is going in a salad, salt it and drain it first rather than dressing it early.")]), py_tuple(vec![py_tuple(vec![py_str("cucumber"), py_str("cucumbers")]), py_str("Cucumber goes soggy because salt and dressing pull the water out of it: slice it, salt it for 10 minutes and drain it on kitchen paper, or scoop the watery middle out with a teaspoon, and dress it at the table rather than an hour before. For a sandwich it wants to be patted dry and kept away from the bread until you eat.")]), py_tuple(vec![py_tuple(vec![py_str("aubergine"), py_str("aubergines"), py_str("eggplant"), py_str("eggplants")]), py_str("Aubergine goes soggy when it soaks up oil: salt the slices for 20 minutes, pat them dry, then fry in hot oil in one layer, or brush them with oil and roast at 200C until the edges colour. It is done when it collapses and looks glossy, not when it is merely soft.")]), py_tuple(vec![py_tuple(vec![py_str("mushroom"), py_str("mushrooms")]), py_str("Mushrooms go soggy when they are crowded or covered: they are mostly water, so give them a wide hot pan with a little oil, one layer, no lid, and let the water cook off before you turn them. Salt only at the end, or they give up their water early and stew.")]), py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("cherry tomatoes")]), py_str("Tomato goes watery when it is cooked with the lid on or with too much salt early: halve them, salt them in a colander for 10 minutes and let the juice run off, then roast them cut side up at 200C, or cook them uncovered so the water reduces instead of pooling.")]), py_tuple(vec![py_tuple(vec![py_str("pepper"), py_str("peppers"), py_str("bell pepper"), py_str("bell peppers")]), py_str("Peppers go soft and watery when they are cooked fast in a crowded pan: cut them into strips of the same size and give them a hot wide pan in one layer, or roast them at 200C until the skins blister - then they are sweet rather than wet.")]), py_tuple(vec![py_tuple(vec![py_str("butternut squash"), py_str("squash"), py_str("pumpkin")]), py_str("Squash goes soggy when it is steamed rather than roasted: cut it into 3cm chunks, toss in oil, spread it in one layer and roast at 200C for 30 to 35 minutes, turning once. Boiled or covered, it holds on to the water and turns to mush.")])]));
    s_ENTRIES(op_add(&g_ENTRIES(), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("veg-watery")); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WATER_FAMILIES()) { let __unpk6343 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk6343[0].clone(); let mut _fix: Py = __unpk6343[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, g_WATER_WORDS()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _watery_veg(args))(&__args) } })); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("thawed"), py_str("thaw"), py_str("thawing"), py_str("defrosted"), py_str("defrost"), py_str("how long"), py_str("last"), py_str("lasts"), py_str("leftovers"), py_str("leftover"), py_str("reheat"), py_str("calories"), py_str("calorie"), py_str("healthy"), py_str("protein"), py_str("vitamin"), py_str("plant"), py_str("plants"), py_str("planting"), py_str("grow"), py_str("growing"), py_str("garden"), py_str("allotment"), py_str("seed"), py_str("seeds"), py_str("harvest"), py_str("pick"), py_str("picking"), py_str("ripe"), py_str("buy"), py_str("buying"), py_str("cost"), py_str("price"), py_str("cheap"), py_str("supermarket"), py_str("allergy"), py_str("allergic"), py_str("baby"), py_str("weaning"), py_str("diabetes"), py_str("mouldy"), py_str("gone off"), py_str("wilted"), py_str("wilting"), py_str("slimy"), py_str("limp")])), ("bias".to_string(), Py::Int(5))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }])));
    s_VEG_SOFT_FAULTS(py_tuple(vec![py_str("soggy"), py_str("limp"), py_str("floppy"), py_str("mushy"), py_str("slimy"), py_str("watery"), py_str("overcooked"), py_str("over cooked"), py_str("too soft"), py_str("went soft"), py_str("gone soft"), py_str("gone soggy"), py_str("going soggy"), py_str("gone limp"), py_str("went limp"), py_str("not crisp"), py_str("not crispy"), py_str("wont crisp"), py_str("won't crisp"), py_str("soft and wet")]));
    s_VEG_SOFT_NAMES(op_add(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_TIME()) { let __unpk6631 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk6631[0].clone(); let mut _said: Py = __unpk6631[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_HOW_EXTRA()) { let __unpk6295 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk6295[0].clone(); let mut _how: Py = __unpk6295[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_VEG_HOW()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("veggies"), py_str("mixed veg"), py_str("stir fry veg"), py_str("roast veg"), py_str("leafy greens"), py_str("broccoli"), py_str("peas"), py_str("sprouts"), py_str("sprout"), py_str("parsnip"), py_str("parsnips"), py_str("romanesco"), py_str("broccolini"), py_str("spring onions"), py_str("shallot"), py_str("shallots"), py_str("butternut"), py_str("pumpkin"), py_str("aubergine"), py_str("aubergines")])));
    s_VEG_ROASTED(py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("oven"), py_str("baked"), py_str("baking"), py_str("air fryer"), py_str("airfryer"), py_str("tray"), py_str("traybake"), py_str("grill"), py_str("grilled")]));
    s_VEG_WET(py_tuple(vec![py_str("courgette"), py_str("courgettes"), py_str("zucchini"), py_str("aubergine"), py_str("aubergines"), py_str("marrow"), py_str("tomato"), py_str("tomatoes"), py_str("squash"), py_str("butternut squash"), py_str("butternut"), py_str("pumpkin")]));
    s_VEG_LEAVES(py_tuple(vec![py_str("spinach"), py_str("kale"), py_str("chard"), py_str("cavolo nero"), py_str("spring greens"), py_str("greens"), py_str("leafy greens"), py_str("collard")]));
    s_VEG_PODS(py_tuple(vec![py_str("peas"), py_str("mange tout"), py_str("mangetout"), py_str("sugar snap"), py_str("green beans"), py_str("runner beans"), py_str("french beans"), py_str("string beans"), py_str("broad beans"), py_str("broad bean"), py_str("fava beans")]));
    s_VEG_RAW_FAULTS(py_tuple(vec![py_str("gone soft"), py_str("went soft"), py_str("gone limp"), py_str("went limp"), py_str("gone floppy"), py_str("gone bendy"), py_str("gone rubbery"), py_str("gone wrinkly"), py_str("gone leathery"), py_str("lost its crunch"), py_str("lost their crunch"), py_str("gone dry"), py_str("dried out")]));
    s_VEG_PREVENTION(py_tuple(vec![py_str("how do i stop"), py_str("how can i stop"), py_str("how to stop"), py_str("how do i keep"), py_str("how can i keep"), py_str("how do i prevent"), py_str("how do i make sure"), py_str("how long"), py_str("how do i cook"), py_str("how do i boil"), py_str("how do i steam"), py_str("how do i roast"), py_str("how do i fry"), py_str("stop it going"), py_str("stop them going"), py_str("stop it from going"), py_str("stop them from going"), py_str("keep it from"), py_str("keep them from"), py_str("so it doesnt go"), py_str("so they dont go"), py_str("so it doesn't go"), py_str("so they don't go")]));
    s_VEG_GENERIC(py_tuple(vec![py_str("veg"), py_str("vegetables"), py_str("veggie"), py_str("veggies"), py_str("mixed veg"), py_str("stir fry veg"), py_str("root veg"), py_str("veg mix"), py_str("veg of"), py_str("some veg")]));
    s_VEG_COOKED(py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("boil"), py_str("boiled"), py_str("boiling"), py_str("steam"), py_str("steamed"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("oven"), py_str("pan"), py_str("microwave"), py_str("air fryer"), py_str("stir fry"), py_str("stir fried"), py_str("grill"), py_str("grilled"), py_str("bake"), py_str("baked")]));
    s_VEG_BRASSICAS(py_tuple(vec![py_str("broccoli"), py_str("cauliflower"), py_str("cabbage"), py_str("sprouts"), py_str("sprout"), py_str("romanesco"), py_str("broccolini"), py_str("pak choi"), py_str("pak choy"), py_str("bok choy"), py_str("kalettes"), py_str("savoy"), py_str("kohlrabi"), py_str("turnip")]));
    s_ENTRIES(op_add(&g_ENTRIES(), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("veg-gone-soft")); __args.push(py_tuple(vec![g_VEG_SOFT_NAMES(), g_VEG_SOFT_FAULTS()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _typed_line(args))(&__args) }); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _soft_veg(args))(&__args) } })); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("chips"), py_str("chip"), py_str("fries"), py_str("wedges"), py_str("roasties"), py_str("salad"), py_str("lettuce"), py_str("apple"), py_str("fruit"), py_str("pasta"), py_str("rice"), py_str("noodles"), py_str("bread"), py_str("cake"), py_str("pastry"), py_str("meat"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("fish"), py_str("egg"), py_str("soup"), py_str("sauce"), py_str("gravy"), py_str("curry"), py_str("stew"), py_str("pie"), py_str("pizza"), py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("fridge"), py_str("keep"), py_str("keeps"), py_str("calories"), py_str("healthy"), py_str("recipe"), py_str("use up"), py_str("use it up"), py_str("what can i do"), py_str("can i do with"), py_str("left over"), py_str("leftover"), py_str("leftovers"), py_str("meal idea"), py_str("meal ideas")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }])));
}
pub fn _phrase_in(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let _ = py_str("True when the phrase is in the line, on word boundaries (\"soft\" is not \"soften\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?![a-z])"))); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _takes_to(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut spot: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spot") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spot"))) };
    let mut between: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for \"carrots take to roast\" and \"carrots take in the oven\", and nothing else.");
    between = slice_or_raise(&words.clone(), &op_add(&position.clone(), &Py::Int(1)), &spot.clone(), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&between.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_VEG_FILLER(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_VEG_COOK(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("take"), py_str("takes"), py_str("took")]), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _near(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut index: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the phrase sits among the words of the line, or None.\n\n    The plural counts as the same word: \"how long do i roast carrots\" has the method word next\n    to the vegetable, and reading \"carrot\" alone would have missed it.\n    ");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &wanted.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&word.clone(), &op_add(&wanted.clone(), &py_str("s")))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&op_add(&word.clone(), &py_str("s")), &wanted.clone())) } } } }) {
    return index.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_word(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut have: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("True when the whole word (or its plain plural) is in the line.\n\n    `_same_word` is deliberately loose, which suits a dish name typed with a slip but not a\n    vegetable: it would read \"cake\" as \"kale\". A vegetable timing is a claim about one food,\n    so it is matched exactly.\n    ");
    have = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::tokens(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&have.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&have.clone());
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&one.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&one.clone(), &op_add(&word.clone(), &py_str("s")))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&op_add(&one.clone(), &py_str("s")), &word.clone())) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &word.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _name_in(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut have: Py = Py::None;
    let mut i: Py = Py::None;
    let mut last: Py = Py::None;
    let mut run: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("True when `name` is in the line as whole words, singular or plural.\n\n    \"mange tout\" and \"pak choi\" are two words, and the plural of \"runner bean\" is \"runner\n    beans\": the last word of the name takes the plural, the rest has to match exactly.\n    ");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::tokens(args))(&__args) };
    have = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::tokens(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&have.clone())) } }) {
    return Py::Bool(false);
    }
    last = op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1));
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(have.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    i = __item.clone();
    run = slice_or_raise(&have.clone(), &i.clone(), &op_add(&i.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&run.clone(), &Py::None, &last.clone(), &Py::None), &slice_or_raise(&wanted.clone(), &Py::None, &last.clone(), &Py::None))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&run.clone(), &last.clone(), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); __args.push(geti(&wanted.clone(), &last.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _ruled_out(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut before: Py = Py::None;
    let mut name: Py = Py::None;
    let mut run: Py = Py::None;
    let mut start: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names one of these foods only to rule it out.\n\n    The rule-out word has to be close in front of the food it rules out, so \"i dont like\n    mushrooms, how long do i cook carrots\" keeps the carrot row, and \"carrots with no butter\"\n    keeps it too: the words after the vegetable are another ingredient's.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    start = __item.clone();
    run = slice_or_raise(&words.clone(), &start.clone(), &op_add(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&slice_or_raise(&run.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &slice_or_raise(&wanted.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&run.clone(), &wanted.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&run.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); __args.push(geti(&wanted.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }))) {
    continue;
    }
    before = slice_or_raise(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&start.clone(), &g__RULE_OUT_REACH())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &start.clone(), &Py::None);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_RULE_OUT(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _typed_line(args: &[Py]) -> Py {
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
    let mut now: Py = Py::None;
    let _ = py_str("The line as the person typed it, or \"\".\n\n    The `understood` an entry is handed has the context words of the conversation appended to\n    it (\"and how long does that take egg spinach\"), and a follow-up is about the dish, not about\n    its spinach: that line, after a frittata with spinach, was answered \"Spinach: 1 to 2\n    minutes\" - a true fact and the wrong answer. `memory[\"understood\"]` holds the line typed.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(now.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&now.clone(), "get", args))(&__args) } } }) {
    return geti(&now.clone(), &py_str("text"));
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    }
    return py_str("");
    Py::None
}
pub fn _reply_for(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut how: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { Py::None };
    let mut answer: Py = Py::None;
    let _ = py_str("The entry for one vegetable: its own time, or how to cook it, or None.\n\n    One entry per vegetable rather than one entry for the table, because a follow-up is\n    answered from the answer just given: `bot/modules/answers.py` narrows the last reply to the\n    figure asked for, and it stands down only when the thing the line names reached an entry of\n    its own. With a single `veg-time` entry every vegetable was the same entry, so \"how long to\n    boil a swede\" straight after a kohlrabi answer came back \"12 to 15 minutes.\" - the kohlrabi\n    time with its name trimmed off. An entry of its own makes each vegetable a thing of its own.\n    ");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut how: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut names: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut ask: Py = Py::None;
    let mut between: Py = Py::None;
    let mut head: Py = Py::None;
    let mut how_asked: Py = Py::None;
    let mut name: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut position: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut text: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _typed_line(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_ASKS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(names.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ruled_out(args))(&__args) }) {
    return Py::None;
    }
    how_asked = { let __b = Py::Bool(!py_is(&how.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_HOW_ASKS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_TIME_WORDS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_NAMED_METHOD()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&how_asked.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_TIME_WORDS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("freezer"), py_str("freeze"), py_str("freezing")])) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("boil"), py_str("boiling"), py_str("steam"), py_str("steaming"), py_str("cook"), py_str("cooking"), py_str("blanch")])) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    where_ = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }) {
    where_ = name.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&where_.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("in the oven"), py_str("in an oven"), py_str("in my oven"), py_str("in the air fryer"), py_str("in a hot oven")])) { let mut phrase: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    position = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&where_.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _near(args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) };
    let __iter2 = py_iter_or_raise(&g_VEG_OTHER());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "split", args))(&__args) }, &where_.clone()))) {
    return Py::None;
    }
    spot = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _near(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&position.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&spot.clone(), &Py::None)) } }) {
    continue;
    }
    if py_truthy(&op_lt(&spot.clone(), &position.clone())) {
    between = slice_or_raise(&words.clone(), &op_add(&spot.clone(), &Py::Int(1)), &position.clone(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_ASK_BETWEEN()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&between.clone(), &ask.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    } else {
    if py_truthy(&{ let __b = op_gt(&spot.clone(), &position.clone()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(position.clone()); __args.push(spot.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _takes_to(args))(&__args) } } }) {
    return Py::None;
    } else {
    if py_truthy(&op_le(&op_sub(&spot.clone(), &position.clone()), &Py::Int(2))) {
    return Py::None;
    }
    }
    }
    }
    head = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&where_.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&where_.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&how_asked.clone()) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_str("frozen")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }) {
    return op_add(&op_add(&head.clone(), &py_str(", from frozen: straight into already boiling water and take 2 minutes off - ")), &how.clone());
    }
    return op_add(&op_add(&head.clone(), &py_str(": ")), &how.clone());
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return op_add(&op_add(&head.clone(), &py_str(": ")), &how.clone());
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_str("frozen")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }) {
    return op_add(&op_add(&head.clone(), &py_str(", from frozen: go straight into already boiling water and take 2 minutes off - ")), &said.clone());
    }
    return op_add(&op_add(&head.clone(), &py_str(": ")), &said.clone());
    Py::None
}
    answer = {{ let answer = answer.clone(); let how = how.clone(); let names = names.clone(); let said = said.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), how.clone(), names.clone(), said.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _roast_reply(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut how: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut name: Py = Py::None;
    let mut text: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How long one vegetable takes in the oven, or None when the line asks something else.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _typed_line(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ROAST_WORDS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    lowered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ROAST_NOT_FOR_US()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ROAST_MINUTES()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ruled_out(args))(&__args) }) {
    return Py::None;
    }
    where_ = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ROAST_MINUTES()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }) {
    where_ = name.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&where_.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(where_.clone()); __args.push(py_tuple(vec![py_str("25 to 30"), py_str("cut into even pieces")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ROAST_MINUTES(), "get", args))(&__args) }, 2, None);
    minutes = __unpk2[0].clone();
    how = __unpk2[1].clone();
    head = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&where_.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&where_.clone(), &Py::Int(1), &Py::None, &Py::None));
    return op_add(&op_add(&op_add(&op_add(&op_add(&head.clone(), &py_str(": ")), &minutes.clone()), &py_str(" minutes in the oven at 200C (180C fan), ")), &how.clone()), &py_str(", in a single layer with a little oil so it browns rather than steams. It is done when the edges are browned and a knife goes into the thickest piece easily."));
    Py::None
}
pub fn _slug(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let _ = py_str("An entry id for a vegetable: \"butternut squash\" is `veg-butternut-squash`.");
    return op_add(&py_str("veg-"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9]+")); __args.push(py_str("-")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) });
    Py::None
}
pub fn _watery_veg(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut fix: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The fix for a vegetable that has gone wet, per family; None when the line names none.");
    words = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) } } else { Py::None };
    text = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) };
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_WATER_FAMILIES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    fix = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &geti(&best.clone(), &Py::Int(0))) } } } }) {
    best = py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, fix.clone()]);
    }
    }
    }
    return if py_truthy(&best.clone()) { geti(&best.clone(), &Py::Int(1)) } else { Py::None };
    Py::None
}
pub fn _soft_veg(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The fix for a vegetable that came out soft, wet or limp, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_SOFT_NAMES()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&said.clone(), &text.clone())) } }) {
    text = said.clone();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("slimy"), py_str("smelly"), py_str("sour"), py_str("gone off")])) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("A raw vegetable that is slimy and smells is going off, so bin the affected leaves or the whole bag: that slime is bacteria, not water, and cooking will not make it safe to eat. Slime on a cooked vegetable is the harmless kind - it means too long in the water - so drain it, press the water out and dress it with oil and lemon.");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_RAW_FAULTS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_COOKED()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_str("A raw vegetable that has gone limp has lost water, not flavour: it is still safe to eat, and half an hour in a bowl of iced water firms it up again for a salad. After that, cook it - roast it at 200C, or put it in a soup or a stir-fry - because a limp one will not crisp back on its own. For the next lot, a bag or a lidded tub in the salad drawer keeps it firm for weeks, and cutting the green tops off carrots stops them pulling the water out of the root.");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_PREVENTION()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_GENERIC()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_PODS()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Peas and beans go from squeaky to mushy in about a minute: 2 minutes for frozen peas straight into boiling water, 4 to 5 for green beans, and drain them the moment they are bright. If they are already soft, toss them in butter with a squeeze of lemon and serve them as they are - a soft pea will not firm up - or let them cool into a salad or a soup.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_LEAVES()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Spinach, kale and other leaves go slimy in a pan of water, so cook them in almost none: a hot pan with a knob of butter and a spoon of water, 1 to 2 minutes until they have just collapsed, then press the water out. Raw leaves that have only gone limp come back in 15 minutes in iced water; cooked ones that are slimy were in too much water, so squeeze them dry and dress them with butter or a little vinegar, or fold them into a sauce.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_BRASSICAS()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Broccoli, cauliflower and cabbage go soft and grey from too long at a rolling boil: 4 to 5 minutes for broccoli florets, 6 to 8 for sprouts or shredded cabbage, 12 to 15 for a whole cauliflower, then straight into a colander with the lid off, because it goes on cooking in its own steam. If it has already gone grey and sulphury, give it a minute in a hot pan with butter and garlic, or put it in a gratin or a soup.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_WET()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _name_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Courgette, aubergine and tomato are mostly water, so that water has to come out before the heat goes on: slice them, salt them for 10 minutes and pat them dry, then fry or roast in one layer in a hot pan and do not crowd it, because a crowded pan steams them instead of browning. Already limp: turn the heat up to drive the water off, or put them in a sauce or a soup.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_ROASTED()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_in(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("A vegetable that comes out of the oven soft was steamed rather than roasted: it went in wet, or the tray was crowded. Dry it, toss it in oil, spread it in one layer with air between the pieces, and give it 200C (180C fan) with the door shut. Already limp: turn the heat up for the last 10 minutes to colour what you can, and serve it with something sharp - it will not crisp back.");
    }
    return py_str("A vegetable goes limp and wet from too long in the water, from a lid left on so the steam cannot escape, or from a pan too small for the amount: cook it in a shallow layer with a splash of water rather than a deep panful, taste it a minute before the time in the recipe, and drain it the second a knife slides in, lid off. If it is already soft, drive the water off in a hot dry pan with a little butter, or put it in a soup or a sauce, because it will not crisp again. A raw vegetable gone limp in the fridge is the same rescue: it has lost water rather than been overcooked, so roast it at 200C or put it in a soup within the day.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.vegetables".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "ROAST_MINUTES" => Some(g_ROAST_MINUTES()),
            "ROAST_NOT_FOR_US" => Some(g_ROAST_NOT_FOR_US()),
            "ROAST_WORDS" => Some(g_ROAST_WORDS()),
            "RULE_OUT" => Some(g_RULE_OUT()),
            "VEG_ASKS" => Some(g_VEG_ASKS()),
            "VEG_ASK_BETWEEN" => Some(g_VEG_ASK_BETWEEN()),
            "VEG_BRASSICAS" => Some(g_VEG_BRASSICAS()),
            "VEG_COOK" => Some(g_VEG_COOK()),
            "VEG_COOKED" => Some(g_VEG_COOKED()),
            "VEG_EXTRA_AVOID" => Some(g_VEG_EXTRA_AVOID()),
            "VEG_FILLER" => Some(g_VEG_FILLER()),
            "VEG_GENERIC" => Some(g_VEG_GENERIC()),
            "VEG_HOW" => Some(g_VEG_HOW()),
            "VEG_HOW_ASKS" => Some(g_VEG_HOW_ASKS()),
            "VEG_HOW_EXTRA" => Some(g_VEG_HOW_EXTRA()),
            "VEG_LEAVES" => Some(g_VEG_LEAVES()),
            "VEG_NAMED_METHOD" => Some(g_VEG_NAMED_METHOD()),
            "VEG_NOT_FOR_US" => Some(g_VEG_NOT_FOR_US()),
            "VEG_OTHER" => Some(g_VEG_OTHER()),
            "VEG_PODS" => Some(g_VEG_PODS()),
            "VEG_PREVENTION" => Some(g_VEG_PREVENTION()),
            "VEG_RAW_FAULTS" => Some(g_VEG_RAW_FAULTS()),
            "VEG_ROASTED" => Some(g_VEG_ROASTED()),
            "VEG_SOFT_FAULTS" => Some(g_VEG_SOFT_FAULTS()),
            "VEG_SOFT_NAMES" => Some(g_VEG_SOFT_NAMES()),
            "VEG_TIME" => Some(g_VEG_TIME()),
            "VEG_TIME_WORDS" => Some(g_VEG_TIME_WORDS()),
            "VEG_WET" => Some(g_VEG_WET()),
            "WATER_FAMILIES" => Some(g_WATER_FAMILIES()),
            "WATER_WORDS" => Some(g_WATER_WORDS()),
            "_RULE_OUT_REACH" => Some(g__RULE_OUT_REACH()),
            "_a_word" => Some(py_func("_a_word", |args: &[Py]| _a_word(args))),
            "_name_in" => Some(py_func("_name_in", |args: &[Py]| _name_in(args))),
            "_near" => Some(py_func("_near", |args: &[Py]| _near(args))),
            "_phrase_in" => Some(py_func("_phrase_in", |args: &[Py]| _phrase_in(args))),
            "_reply_for" => Some(py_func("_reply_for", |args: &[Py]| _reply_for(args))),
            "_roast_reply" => Some(py_func("_roast_reply", |args: &[Py]| _roast_reply(args))),
            "_ruled_out" => Some(py_func("_ruled_out", |args: &[Py]| _ruled_out(args))),
            "_slug" => Some(py_func("_slug", |args: &[Py]| _slug(args))),
            "_soft_veg" => Some(py_func("_soft_veg", |args: &[Py]| _soft_veg(args))),
            "_takes_to" => Some(py_func("_takes_to", |args: &[Py]| _takes_to(args))),
            "_typed_line" => Some(py_func("_typed_line", |args: &[Py]| _typed_line(args))),
            "_watery_veg" => Some(py_func("_watery_veg", |args: &[Py]| _watery_veg(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "ROAST_MINUTES" => s_ROAST_MINUTES(v),
            "ROAST_NOT_FOR_US" => s_ROAST_NOT_FOR_US(v),
            "ROAST_WORDS" => s_ROAST_WORDS(v),
            "RULE_OUT" => s_RULE_OUT(v),
            "VEG_ASKS" => s_VEG_ASKS(v),
            "VEG_ASK_BETWEEN" => s_VEG_ASK_BETWEEN(v),
            "VEG_BRASSICAS" => s_VEG_BRASSICAS(v),
            "VEG_COOK" => s_VEG_COOK(v),
            "VEG_COOKED" => s_VEG_COOKED(v),
            "VEG_EXTRA_AVOID" => s_VEG_EXTRA_AVOID(v),
            "VEG_FILLER" => s_VEG_FILLER(v),
            "VEG_GENERIC" => s_VEG_GENERIC(v),
            "VEG_HOW" => s_VEG_HOW(v),
            "VEG_HOW_ASKS" => s_VEG_HOW_ASKS(v),
            "VEG_HOW_EXTRA" => s_VEG_HOW_EXTRA(v),
            "VEG_LEAVES" => s_VEG_LEAVES(v),
            "VEG_NAMED_METHOD" => s_VEG_NAMED_METHOD(v),
            "VEG_NOT_FOR_US" => s_VEG_NOT_FOR_US(v),
            "VEG_OTHER" => s_VEG_OTHER(v),
            "VEG_PODS" => s_VEG_PODS(v),
            "VEG_PREVENTION" => s_VEG_PREVENTION(v),
            "VEG_RAW_FAULTS" => s_VEG_RAW_FAULTS(v),
            "VEG_ROASTED" => s_VEG_ROASTED(v),
            "VEG_SOFT_FAULTS" => s_VEG_SOFT_FAULTS(v),
            "VEG_SOFT_NAMES" => s_VEG_SOFT_NAMES(v),
            "VEG_TIME" => s_VEG_TIME(v),
            "VEG_TIME_WORDS" => s_VEG_TIME_WORDS(v),
            "VEG_WET" => s_VEG_WET(v),
            "WATER_FAMILIES" => s_WATER_FAMILIES(v),
            "WATER_WORDS" => s_WATER_WORDS(v),
            "_RULE_OUT_REACH" => s__RULE_OUT_REACH(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.vegetables", module_obj);
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_phrase_in", py_func("_phrase_in", |args: &[Py]| _phrase_in(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_takes_to", py_func("_takes_to", |args: &[Py]| _takes_to(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_near", py_func("_near", |args: &[Py]| _near(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_a_word", py_func("_a_word", |args: &[Py]| _a_word(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_name_in", py_func("_name_in", |args: &[Py]| _name_in(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_ruled_out", py_func("_ruled_out", |args: &[Py]| _ruled_out(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_typed_line", py_func("_typed_line", |args: &[Py]| _typed_line(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_reply_for", py_func("_reply_for", |args: &[Py]| _reply_for(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_roast_reply", py_func("_roast_reply", |args: &[Py]| _roast_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_slug", py_func("_slug", |args: &[Py]| _slug(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_watery_veg", py_func("_watery_veg", |args: &[Py]| _watery_veg(args)));
    pyrt::xrefs::reg_fn("knowledge.vegetables", "_soft_veg", py_func("_soft_veg", |args: &[Py]| _soft_veg(args)));
    pyrt::xrefs::reg_access("knowledge.vegetables", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.vegetables", "ROAST_MINUTES", g_ROAST_MINUTES, s_ROAST_MINUTES);
    pyrt::xrefs::reg_access("knowledge.vegetables", "ROAST_NOT_FOR_US", g_ROAST_NOT_FOR_US, s_ROAST_NOT_FOR_US);
    pyrt::xrefs::reg_access("knowledge.vegetables", "ROAST_WORDS", g_ROAST_WORDS, s_ROAST_WORDS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "RULE_OUT", g_RULE_OUT, s_RULE_OUT);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_ASKS", g_VEG_ASKS, s_VEG_ASKS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_ASK_BETWEEN", g_VEG_ASK_BETWEEN, s_VEG_ASK_BETWEEN);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_BRASSICAS", g_VEG_BRASSICAS, s_VEG_BRASSICAS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_COOK", g_VEG_COOK, s_VEG_COOK);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_COOKED", g_VEG_COOKED, s_VEG_COOKED);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_EXTRA_AVOID", g_VEG_EXTRA_AVOID, s_VEG_EXTRA_AVOID);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_FILLER", g_VEG_FILLER, s_VEG_FILLER);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_GENERIC", g_VEG_GENERIC, s_VEG_GENERIC);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_HOW", g_VEG_HOW, s_VEG_HOW);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_HOW_ASKS", g_VEG_HOW_ASKS, s_VEG_HOW_ASKS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_HOW_EXTRA", g_VEG_HOW_EXTRA, s_VEG_HOW_EXTRA);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_LEAVES", g_VEG_LEAVES, s_VEG_LEAVES);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_NAMED_METHOD", g_VEG_NAMED_METHOD, s_VEG_NAMED_METHOD);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_NOT_FOR_US", g_VEG_NOT_FOR_US, s_VEG_NOT_FOR_US);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_OTHER", g_VEG_OTHER, s_VEG_OTHER);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_PODS", g_VEG_PODS, s_VEG_PODS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_PREVENTION", g_VEG_PREVENTION, s_VEG_PREVENTION);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_RAW_FAULTS", g_VEG_RAW_FAULTS, s_VEG_RAW_FAULTS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_ROASTED", g_VEG_ROASTED, s_VEG_ROASTED);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_SOFT_FAULTS", g_VEG_SOFT_FAULTS, s_VEG_SOFT_FAULTS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_SOFT_NAMES", g_VEG_SOFT_NAMES, s_VEG_SOFT_NAMES);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_TIME", g_VEG_TIME, s_VEG_TIME);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_TIME_WORDS", g_VEG_TIME_WORDS, s_VEG_TIME_WORDS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "VEG_WET", g_VEG_WET, s_VEG_WET);
    pyrt::xrefs::reg_access("knowledge.vegetables", "WATER_FAMILIES", g_WATER_FAMILIES, s_WATER_FAMILIES);
    pyrt::xrefs::reg_access("knowledge.vegetables", "WATER_WORDS", g_WATER_WORDS, s_WATER_WORDS);
    pyrt::xrefs::reg_access("knowledge.vegetables", "_RULE_OUT_REACH", g__RULE_OUT_REACH, s__RULE_OUT_REACH);
}
