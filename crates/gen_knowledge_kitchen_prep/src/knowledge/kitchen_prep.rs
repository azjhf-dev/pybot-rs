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
thread_local! { static G__ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BLANCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BLANCHABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FREEZING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEAT_ROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARBOIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RINSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCORE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEP_ROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEP_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.kitchen_prep: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK() -> Py { ensure_init(); G__ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK(v: Py) { ensure_init(); G__ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__BLANCH() -> Py { ensure_init(); G__BLANCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BLANCH(v: Py) { ensure_init(); G__BLANCH.with(|c| *c.borrow_mut() = v); }
pub fn g__BLANCHABLE() -> Py { ensure_init(); G__BLANCHABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BLANCHABLE(v: Py) { ensure_init(); G__BLANCHABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD() -> Py { ensure_init(); G__FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD(v: Py) { ensure_init(); G__FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__FREEZING() -> Py { ensure_init(); G__FREEZING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FREEZING(v: Py) { ensure_init(); G__FREEZING.with(|c| *c.borrow_mut() = v); }
pub fn g__MEAT_ROWS() -> Py { ensure_init(); G__MEAT_ROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEAT_ROWS(v: Py) { ensure_init(); G__MEAT_ROWS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_FOOD() -> Py { ensure_init(); G__NOT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_FOOD(v: Py) { ensure_init(); G__NOT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__PARBOIL() -> Py { ensure_init(); G__PARBOIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARBOIL(v: Py) { ensure_init(); G__PARBOIL.with(|c| *c.borrow_mut() = v); }
pub fn g__PAT() -> Py { ensure_init(); G__PAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAT(v: Py) { ensure_init(); G__PAT.with(|c| *c.borrow_mut() = v); }
pub fn g__PEEL() -> Py { ensure_init(); G__PEEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEEL(v: Py) { ensure_init(); G__PEEL.with(|c| *c.borrow_mut() = v); }
pub fn g__PRICK() -> Py { ensure_init(); G__PRICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRICK(v: Py) { ensure_init(); G__PRICK.with(|c| *c.borrow_mut() = v); }
pub fn g__RINSE() -> Py { ensure_init(); G__RINSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RINSE(v: Py) { ensure_init(); G__RINSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SCORE() -> Py { ensure_init(); G__SCORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCORE(v: Py) { ensure_init(); G__SCORE.with(|c| *c.borrow_mut() = v); }
pub fn g__SCORE_ASK() -> Py { ensure_init(); G__SCORE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCORE_ASK(v: Py) { ensure_init(); G__SCORE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SIFT() -> Py { ensure_init(); G__SIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIFT(v: Py) { ensure_init(); G__SIFT.with(|c| *c.borrow_mut() = v); }
pub fn g__STEPS() -> Py { ensure_init(); G__STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEPS(v: Py) { ensure_init(); G__STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g__STEP_ROWS() -> Py { ensure_init(); G__STEP_ROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEP_ROWS(v: Py) { ensure_init(); G__STEP_ROWS.with(|c| *c.borrow_mut() = v); }
pub fn g__STEP_VERBS() -> Py { ensure_init(); G__STEP_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEP_VERBS(v: Py) { ensure_init(); G__STEP_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIM() -> Py { ensure_init(); G__TRIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIM(v: Py) { ensure_init(); G__TRIM.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_last_user() -> Py { ensure_init(); G_last_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_user(v: Py) { ensure_init(); G_last_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Kitchen prep: the small things you do before the cooking, not the cooking.\n\nThe answers lane owns this domain. It holds two kinds of row, both about a step that happens\nbefore anything goes in the pan or the oven.\n\nThe room-temperature table: \"room temperature\" is two questions, how warm a bedroom should be and\nwhat to do with a steak before cooking it. Neither cooking's tables nor sleep's could hold the\nkitchen half without moving the rarity of a word a dozen other lines are weighed against -\n\"should I rest a steak\" went to a different answer the moment it did - so the row lives in a table\nof its own, keyed by the cut and the instruction.\n\nThe step table: \"do i need to sift flour\", \"do i have to peel the potatoes\", \"should i prick the\njacket potatoes\", \"do i need to score the pork skin\". The person already knows what they are\ncooking and is asking whether one step matters, and the answer is yes or no in the first words\nwith the one consequence that decides it. These lines were answered by a row that shared a word\nwith them and not the question: the vegetarian card for sifting flour, the coat card for a jacket\npotato, the side dishes table for peeling, the red-meat limit for patting a steak dry. The step is\nthe question; the food only picks which sentence inside the answer.\n\nOnly the steps with no card of their own are here. The cooking entries already answer resting\nmeat, soaking pulses, rinsing rice, washing mushrooms, pricking sausages, deseeding chillies,\ngreasing a tin, salting pasta water and marinating, and each row here stands down for exactly\nthose foods.\n");
    s_last_user(py_func("last_user", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_user", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("steak"), py_str("steaks"), py_str("chicken"), py_str("chicken breast"), py_str("turkey"), py_str("pork"), py_str("lamb"), py_str("beef"), py_str("fish"), py_str("meat"), py_str("the meat"), py_str("chops"), py_str("sausages")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("how long"), py_str("left out"), py_str("overnight"), py_str("gone off"), py_str("still good"), py_str("defrost"), py_str("thaw"), py_str("freeze"), py_str("frozen"), py_str("wash"), py_str("washing"), py_str("rinse"), py_str("safe"), py_str("food poisoning"), py_str("pink"), py_str("raw in the middle"), py_str("is it cooked"), py_str("cooked through"), py_str("cooked to"), py_str("done"), py_str("how do i know"), py_str("what temperature"), py_str("how hot"), py_str("oven"), py_str("rest a joint"), py_str("resting"), py_str("prick"), py_str("pricking"), py_str("pricked"), py_str("pierce"), py_str("pierced"), py_str("split"), py_str("splits"), py_str("burst"), py_str("bursts"), py_str("skin"), py_str("skins")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._MEAT_ROWS"); }
    s__MEAT_ROWS(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meat-room-temperature")); __args.push(py_tuple(vec![__data1(), py_tuple(vec![py_str("room temperature"), py_str("out of the fridge"), py_str("rest out"), py_str("twenty minutes"), py_str("an hour before"), py_str("come to room"), py_str("warm up before"), py_str("before cooking"), py_str("before it goes in")])])); __args.push(py_str("It barely matters: twenty minutes on the counter warms the meat by a couple of degrees inside, so the middle still cooks the same. Leaving raw meat out for hours is a food safety risk. What does make a difference is patting it dry and seasoning it just before it goes in the pan - a dry surface browns, a wet one steams.")); __kws.extend(vec![("avoid".to_string(), __data2()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._SIFT"); }
    s__SIFT(py_tuple(vec![py_str("sift"), py_str("sifts"), py_str("sifting"), py_str("sifted"), py_str("sieved"), py_str("sieve the")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._PEEL"); }
    s__PEEL(py_tuple(vec![py_str("peel"), py_str("peels"), py_str("peeling"), py_str("peeled")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._PRICK"); }
    s__PRICK(py_tuple(vec![py_str("prick"), py_str("pricks"), py_str("pricking"), py_str("pricked"), py_str("pierce"), py_str("piercing")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._SCORE"); }
    s__SCORE(py_tuple(vec![py_str("score"), py_str("scores"), py_str("scoring"), py_str("scored"), py_str("slash"), py_str("slashing"), py_str("slit"), py_str("slits"), py_str("slitting")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._BLANCH"); }
    s__BLANCH(py_tuple(vec![py_str("blanch"), py_str("blanches"), py_str("blanching"), py_str("blanched"), py_str("refresh"), py_str("refreshing")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._PARBOIL"); }
    s__PARBOIL(py_tuple(vec![py_str("parboil"), py_str("parboils"), py_str("parboiling"), py_str("parboiled"), py_str("part boil"), py_str("partly boil")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._RINSE"); }
    s__RINSE(py_tuple(vec![py_str("rinse"), py_str("rinses"), py_str("rinsing"), py_str("rinsed")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._PAT"); }
    s__PAT(py_tuple(vec![py_str("pat"), py_str("pats"), py_str("patting"), py_str("patted"), py_str("dab"), py_str("dabs"), py_str("dabbing")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._TRIM"); }
    s__TRIM(py_tuple(vec![py_str("trim"), py_str("trims"), py_str("trimming"), py_str("trimmed"), py_str("top and tail"), py_str("top and tailing")]));
    fn __data3() -> Py { py_tuple(vec![py_str("need"), py_str("needs"), py_str("needed"), py_str("have to"), py_str("has to"), py_str("should"), py_str("must"), py_str("necessary"), py_str("worth"), py_str("point"), py_str("bother"), py_str("bothering"), py_str("essential"), py_str("important")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._ASK"); }
    s__ASK(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._SCORE_ASK"); }
    s__SCORE_ASK(op_add(&g__ASK(), &py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("how should i"), py_str("how do you")])));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._FOOD"); }
    s__FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:potato|potatoes|spud|spuds|jacket|jackets|carrot|carrots|parsnip|parsnips|swede|turnip|tomato|tomatoes|apple|apples|pear|pears|peach|peaches|plum|plums|cucumber|cucumbers|courgette|courgettes|aubergine|aubergines|pepper|peppers|onion|onions|garlic|mushroom|mushrooms|bean|beans|chickpea|chickpeas|lentil|lentils|pea|peas|broccoli|cauliflower|cabbage|sprout|sprouts|spinach|kale|sweetcorn|corn|rice|risotto|pasta|spaghetti|noodles|flour|cocoa|sugar|oats|porridge|couscous|quinoa|bread|dough|loaf|sourdough|pastry|cake|sponge|batter|scone|scones|chicken|turkey|beef|lamb|pork|steak|mince|sausage|sausages|bacon|ham|gammon|duck|fish|salmon|cod|haddock|tuna|prawn|prawns|mussels|squid|chilli|chillies|chilies|ginger|cheese|butter|milk|cream|yoghurt|yogurt|egg|eggs|tofu|almonds|walnuts|peanuts|chips|chip|fries|vegetables|veg|salad|fruit|meat|joint|roast|skins?|fat|skin|asparagus|leek|leeks|celery|fennel|radish|radishes|spring onion|spring onions|scallion|scallions|beetroot|sweet potato|sweet potatoes|butternut|squash|pumpkin)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._NOT_FOOD"); }
    s__NOT_FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:hair|nails|beard|feet|foot|hands|paint|wall|wallpaper|wood|deck|fence|car|bike|engine|boiler|floor|tile|tiles|grout|bath|shower|toilet|drain|washing machine|dishwasher|kettle|loan|mortgage|insurance|email|letter|application|job|interview|contract|passport|visa|exam|hedge|lawn|tree|trees|bush)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._BLANCHABLE"); }
    s__BLANCHABLE(stdlib_call("re", "compile", &[py_str("\\b(?:vegetables|veg|greens|beans|green beans|runner beans|broad beans|carrots|peas|broccoli|cauliflower|sprouts|spinach|kale|sweetcorn|corn|cabbage|leeks|celery|asparagus|courgettes|mange tout|potatoes|potato|tomatoes|tomato|almonds|nuts)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._FREEZING"); }
    s__FREEZING(py_tuple(vec![py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("frozen"), py_str("freezer")]));
    fn __data4() -> Py { py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("peach"), py_str("peaches"), py_str("plum"), py_str("plums"), py_str("banana"), py_str("bananas"), py_str("orange"), py_str("oranges"), py_str("kiwi"), py_str("avocado"), py_str("mango"), py_str("ginger"), py_str("garlic"), py_str("onion"), py_str("onions"), py_str("shallot"), py_str("shallots"), py_str("prawn"), py_str("prawns"), py_str("mushroom"), py_str("mushrooms")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("rice"), py_str("risotto"), py_str("sushi"), py_str("pudding"), py_str("mushroom"), py_str("mushrooms"), py_str("tinned"), py_str("canned"), py_str("already cooked"), py_str("drain"), py_str("chicken"), py_str("turkey"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("steak"), py_str("mince"), py_str("meat"), py_str("fish"), py_str("salmon"), py_str("prawn"), py_str("prawns"), py_str("bacon"), py_str("gammon"), py_str("sausage"), py_str("sausages")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._STEPS"); }
    s__STEPS(py_tuple(vec![py_tuple(vec![py_str("sift"), g__SIFT(), py_func("_sift_reply", |args: &[Py]| _sift_reply(args)), py_tuple(vec![]), Py::None, Py::Int(1)]), py_tuple(vec![py_str("peel"), g__PEEL(), py_func("_peel_reply", |args: &[Py]| _peel_reply(args)), __data4(), Py::None, Py::Int(3)]), py_tuple(vec![py_str("prick"), g__PRICK(), py_func("_prick_reply", |args: &[Py]| _prick_reply(args)), py_tuple(vec![py_str("sausage"), py_str("sausages"), py_str("chipolata"), py_str("chipolatas"), py_str("black pudding")]), Py::None, Py::Int(1)]), py_tuple(vec![py_str("score"), g__SCORE(), py_func("_score_reply", |args: &[Py]| _score_reply(args)), py_tuple(vec![py_str("bread"), py_str("dough"), py_str("loaf"), py_str("loaves"), py_str("sourdough"), py_str("banneton"), py_str("boules")]), py_func("_how_or_whether", |args: &[Py]| _how_or_whether(args)), Py::Int(3), g__SCORE_ASK()]), py_tuple(vec![py_str("rinse"), g__RINSE(), py_func("_rinse_reply", |args: &[Py]| _rinse_reply(args)), __data5(), Py::None, Py::Int(1)]), py_tuple(vec![py_str("blanch"), op_add(&g__BLANCH(), &g__FREEZING()), py_func("_blanch_reply", |args: &[Py]| _blanch_reply(args)), py_tuple(vec![]), py_func("_blanching_or_freezing", |args: &[Py]| _blanching_or_freezing(args)), Py::Int(2)]), py_tuple(vec![py_str("parboil"), g__PARBOIL(), py_func("_parboil_reply", |args: &[Py]| _parboil_reply(args)), py_tuple(vec![]), Py::None, Py::Int(1)]), py_tuple(vec![py_str("pat-dry"), g__PAT(), py_func("_pat_dry_reply", |args: &[Py]| _pat_dry_reply(args)), py_tuple(vec![]), py_func("_patting_it_dry", |args: &[Py]| _patting_it_dry(args)), Py::Int(2)]), py_tuple(vec![py_str("trim"), g__TRIM(), py_func("_trim_reply", |args: &[Py]| _trim_reply(args)), py_tuple(vec![]), Py::None, Py::Int(1)])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._STEP_VERBS"); }
    s__STEP_VERBS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STEPS()) { let mut row: Py = __item.clone(); __out.push(geti(&row.clone(), &Py::Int(1))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep._STEP_ROWS"); }
    s__STEP_ROWS({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STEPS()) { let mut row: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[row.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)))) { op_add(&row.clone(), &py_tuple(vec![Py::None])) } else { row.clone() }); } py_list(__out) })() }) { let __unpk6061 = py_unpack(&__item.clone(), 7, None); let mut _id: Py = __unpk6061[0].clone(); let mut _forms: Py = __unpk6061[1].clone(); let mut _reply: Py = __unpk6061[2].clone(); let mut _avoid: Py = __unpk6061[3].clone(); let mut _guard: Py = __unpk6061[4].clone(); let mut _bias: Py = __unpk6061[5].clone(); let mut _ask: Py = __unpk6061[6].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("prep-"), &_id.clone())); __args.push(py_tuple(vec![_forms.clone(), { let __b = _ask.clone(); if py_truthy(&__b) { __b } else { g__ASK() } }])); __args.push(_reply.clone()); __kws.extend(vec![("avoid".to_string(), _avoid.clone()), ("bias".to_string(), _bias.clone()), ("guard".to_string(), if py_truthy(&Py::Bool(py_is(&_guard.clone(), &Py::None))) { py_func("_asks_whether_the_step_is_needed", |args: &[Py]| _asks_whether_the_step_is_needed(args)) } else { _guard.clone() })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kitchen_prep.ENTRIES"); }
    s_ENTRIES(op_add(&g__MEAT_ROWS(), &g__STEP_ROWS()));
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
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's own words, as the reading hands them over.");
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    text = { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&reading.clone(), "get", &[py_str("text")]) } };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    text = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    return op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _asks_whether_the_step_is_needed(args: &[Py]) -> Py {
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
    let _ = py_str("The row's guard: a food, an ask about the step, and nothing that is not food.");
    said = _said(&[understood.clone()]);
    if py_truthy(&mcall(&g__NOT_FOOD(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__FOOD(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("need"), py_str("have to"), py_str("has to"), py_str("should"), py_str("must"), py_str("necessary"), py_str("worth"), py_str("point"), py_str("bother"), py_str("essential"), py_str("important")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _how_or_whether(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Scoring has no other card for meat, and a bare \"how do i score pork skin\" is the same\n    question asked as a method: this row's own reply answers it too.");
    if py_truthy(&_asks_whether_the_step_is_needed(&[understood.clone()])) {
    return Py::Bool(true);
    }
    said = _said(&[understood.clone()]);
    if py_truthy(&{ let __b = mcall(&g__NOT_FOOD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__FOOD(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("how do i"), py_str("how to "), py_str("how should i"), py_str("how do you")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _patting_it_dry(args: &[Py]) -> Py {
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
    let _ = py_str("Pat dry is two words with the thing in the middle: \"should i pat the steak dry\".");
    return { let __b = _asks_whether_the_step_is_needed(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_said(&[understood.clone()]), &py_str(" dry"))) } };
    Py::None
}
pub fn _a_step_the_turn_before(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut forms: Py = Py::None;
    let _ = py_str("True when the turn before was a step about food, so a bare \"and if im freezing them\" has\n    both the food and the step to lean on. The subject of a follow-up is the turn before it.");
    earlier = if py_truthy(&Py::Bool(!py_is(&memory.clone(), &Py::None))) { pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]) } else { py_str("") };
    earlier = op_add(&op_add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = earlier.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&earlier.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { mcall(&g__NOT_FOOD(), "search", &[earlier.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__FOOD(), "search", &[earlier.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STEP_VERBS()) { let mut forms: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.extend(py_iter_or_raise(&forms.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says(args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_freezing_question(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A question about the freezer: the word for it, and it is not about something else.");
    said = _said(&[understood.clone()]);
    if py_truthy(&{ let __b = mcall(&g__NOT_FOOD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.extend(py_iter_or_raise(&g__FREEZING())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_says(&[said.clone(), py_str("potato"), py_str("potatoes"), py_str("chips"), py_str("chip"), py_str("fries")])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__BLANCHABLE(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(!py_truthy(&mcall(&g__FOOD(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" if "), py_str(" and "), py_str("what about"), py_str("then "), py_str("when ")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _blanching_or_freezing(args: &[Py]) -> Py {
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
    let _ = py_str("The blanch row's guard: whether to blanch, or what to do about the freezer, which is the\n    same step asked about keeping instead of eating.");
    return { let __b = _asks_whether_the_step_is_needed(&[understood.clone()]); if py_truthy(&__b) { __b } else { _a_freezing_question(&[understood.clone()]) } };
    Py::None
}
pub fn _says(args: &[Py]) -> Py {
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
    let mut words: Py = py_list(__pos.get(1..).unwrap_or(&[]).to_vec());
    let mut word: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _sift_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&_says(&[said.clone(), py_str("icing sugar"), py_str("powdered sugar"), py_str("icing")])) {
    return py_str("Yes: icing sugar clumps as soon as the bag is open, and a lump shows in icing or sits in a dusting like grit. A sieve is quicker than a whisk for a smooth glacé icing, and a fine one gives the even dusting you want on a cake.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("cocoa")])) {
    return py_str("Yes, always: cocoa is the one that clumps hardest, and lumps will not beat out of a batter or a sauce. Sift it with the flour or whisk it into hot water first.");
    }
    return py_str("Not from a fresh bag: modern plain and self-raising flour are already fine and loose. Sift it if the bag has been open for months, if there are lumps, or if you are making a light sponge or folding flour into beaten egg whites - the air sifting adds is what makes those rise. Cocoa powder always wants a sieve.");
    Py::None
}
pub fn _peel_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&_says(&[said.clone(), py_str("mash"), py_str("mashed")])) {
    return py_str("Not for mash: scrub them and leave the skin on if you like the flecks - most of the fibre sits there - and peel after boiling if you want the mash white and smooth. Peeling raw potatoes takes a third of the flesh off with the skin, so if you peel first, boil them in their own water.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("roast"), py_str("roasting"), py_str("roastie"), py_str("roasties"), py_str("oven")])) {
    return py_str("Yes for roast potatoes: peel them, cut them the same size, parboil 8 minutes, drain, shake in the pan to rough up the edges, then roast at 200C (180C fan) in hot fat for 40 to 45 minutes. Skins left on make a good, quicker wedge - just not a crisp roastie.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("boil"), py_str("boiling"), py_str("boiled"), py_str("salad"), py_str("salad")])) {
    return py_str("Not unless you want them white: scrub new potatoes and boil them with the skins on - that is where the fibre and a good part of the flavour sit - and peel only for mash, for a smooth soup, or if the skin has gone green or damaged.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("butternut"), py_str("squash"), py_str("pumpkin")])) {
    return py_str("Only if you want it in cubes: roast the halves cut side down in their skins - the flesh scoops out clean - but peel it for a soup or a tray of cubes, and mind your hands, because the skin is hard and slippy.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("beetroot")])) {
    return py_str("Roast them whole in their skins and the skin slips off afterwards: foil at 200C (180C fan) for an hour, then rub it away with kitchen paper. Peel raw beetroot only if you are grating it raw - but wear gloves, because it stains.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("sweet potato"), py_str("sweet potatoes")])) {
    return py_str("No need: the skin of a sweet potato is thin, it crisps in the oven, and it is where much of the fibre sits - scrub it and roast or bake it whole. Peel it only for a mash or a smooth soup.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("cucumber"), py_str("cucumbers")])) {
    return py_str("Only if the skin is thick, waxy or bitter at the stem end: a supermarket cucumber is fine unpeeled, and scoring it with a fork lengthways gives the ridged look without losing anything.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("carrot"), py_str("carrots"), py_str("parsnip"), py_str("parsnips"), py_str("swede"), py_str("turnip")])) {
    return py_str("Only if they are old: a young carrot or parsnip wants a scrub with a brush, while an old one tastes soapy and woody with the skin on. Peel a thin layer rather than down to the core, because the sweetest part is just under it.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("tomato"), py_str("tomatoes")])) {
    return py_str("Only for a smooth sauce or a clear soup: cut a small cross in the base, drop them in boiling water for 30 seconds and then into cold, and the skins slip off. For a chunky sauce or a tray of roasted tomatoes leave them on - they soften and disappear anyway.");
    }
    return py_str("Only if the skin is thick, muddy or bitter. Scrubbing does the same job on anything thin-skinned and a good part of the flavour sits right under it, so peel as thinly as you can when you do.");
    Py::None
}
pub fn _prick_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&_says(&[said.clone(), py_str("potato"), py_str("potatoes"), py_str("jacket"), py_str("jackets"), py_str("spud"), py_str("spuds")])) {
    return py_str("Yes, 4 or 5 deep pricks with a fork: without them the steam has nowhere to go and a jacket potato bursts in the oven. Rub the skin with oil and salt first and it crisps while it bakes.");
    }
    return py_str("Yes for anything with a skin going into a hot oven or a microwave - a tomato, a potato, an aubergine - so the steam has somewhere to go instead of splitting it open.");
    Py::None
}
pub fn _score_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&_says(&[said.clone(), py_str("pork"), py_str("crackling"), py_str("belly"), py_str("shoulder")])) {
    return py_str("Yes for crackling: score the skin about a centimetre deep in a diamond pattern, dry it well, rub in plenty of salt and start the joint hot. The scores are what let the fat render out and the skin bubble.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("bread"), py_str("dough"), py_str("loaf"), py_str("sourdough")])) {
    return py_str("Yes: one slash about a centimetre deep just before it goes in the oven lets the loaf open where you chose rather than where it wants to. A wet blade and a quick, confident cut are what make it clean.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("sausage"), py_str("sausages"), py_str("chipolata"), py_str("chipolatas"), py_str("black pudding")])) {
    return py_str("Not scored, and usually not even pricked: a good butcher's sausage pricked loses its juice, so cook it gently instead - a moderate pan or a 180C oven - and it will not split. A cheap, tight-skinned one splits whatever you do to it.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("fish"), py_str("salmon"), py_str("cod"), py_str("bass"), py_str("trout"), py_str("skin")])) {
    return py_str("Yes for a crisp pan skin: three shallow slashes through the skin stop the fillet curling and let the heat through. Score it before it goes in the pan, and dry the skin well.");
    }
    return py_str("Yes when the skin has to let something out: a joint of pork for crackling, a loaf that has to open where you want it, a fish fillet that would curl. Score about a centimetre deep and dry the surface first.");
    Py::None
}
pub fn _blanch_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.extend(py_iter_or_raise(&g__FREEZING())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says(args))(&__args) }) {
    if py_truthy(&_says(&[said.clone(), py_str("potato"), py_str("potatoes"), py_str("chips"), py_str("chip"), py_str("fries")])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__BLANCHABLE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { _a_step_the_turn_before(&[memory.clone()]) } }))) {
    return Py::None;
    }
    return py_str("Yes before freezing: two minutes in boiling water, then straight into iced water, dry them and bag them. That is what stops the enzymes that turn them grey and soft, so they keep about a year instead of going limp and grassy in a couple of months, and they go into the pan straight from frozen. Cooked leftover vegetables are fine to freeze without blanching.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("almond"), py_str("almonds"), py_str("nut"), py_str("nuts"), py_str("hazelnut"), py_str("hazelnuts"), py_str("walnut"), py_str("walnuts"), py_str("pistachio"), py_str("pistachios")])) {
    return py_str("Only if you want the skins off: a minute in boiling water and then straight into cold, and each one slips out of its skin between your fingers. For flaked or ground almonds there is nothing to do.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("potato"), py_str("potatoes"), py_str("chips"), py_str("chip"), py_str("fries")])) {
    return py_str("For chips, yes: three minutes in boiling water, then drain and dry them completely before frying. That softens the outside, and the roughened surface is what turns crisp.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("tomato"), py_str("tomatoes")])) {
    return py_str("That is the peeling step: 30 seconds in boiling water and straight into cold, and the skins slip off. Nothing else needs it.");
    }
    return py_str("Only if you are eating them later: two minutes in boiling salted water and then straight into iced water keeps green vegetables bright and just-cooked for a salad or a reheat. Into the freezer, it is the step that matters most - a minute or two blanched, then dried and bagged, and it keeps a year instead of going limp and grassy. Eaten straight away, steam them instead - one pan, and they keep more of the vitamins.");
    Py::None
}
pub fn _parboil_reply(args: &[Py]) -> Py {
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
    return py_str("Yes for roast potatoes and chips: about 8 minutes in boiling salted water until a knife goes in and the outside gives, then drain and shake them in the pan to roughen the edges, and roast at 200C (180C fan) in hot fat for 40 to 45 minutes. The roughened edges are what go crisp. Potatoes for mashing need the full boil, not this.");
    Py::None
}
pub fn _rinse_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&_says(&[said.clone(), py_str("quinoa")])) {
    return py_str("Yes, quinoa wants it: 30 seconds under cold water in a sieve takes off the saponin on the surface, which tastes soapy and bitter. It is the one grain that really needs it - and toast it dry in the pan afterwards and it smells nutty.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("barley"), py_str("pearl barley"), py_str("bulgur"), py_str("couscous"), py_str("lentil"), py_str("lentils")])) {
    return py_str("A quick rinse is enough, and it is mostly to check for grit and bits of husk: run cold water through it in a sieve until it runs clear. No soaking, no scrubbing - that is for dried beans, and it is a different question.");
    }
    return py_str("A quick rinse under cold water is enough for most grains and pulses - it clears out dust and loose husk, and takes no more than a few seconds in a sieve. Rice is the one where rinsing is a real choice: rinse for separate grains, leave it for a risotto or a rice pudding.");
    Py::None
}
pub fn _pat_dry_reply(args: &[Py]) -> Py {
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
    return py_str("Yes, and it is the step people skip: a wet surface steams instead of browning, so pat meat, fish and potatoes dry with kitchen paper and salt them just before they go in the pan. It matters most for a steak, a chicken breast with the skin on and anything going into a hot oven.");
    Py::None
}
pub fn _trim_reply(args: &[Py]) -> Py {
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
    said = _said(&[understood.clone()]);
    if py_truthy(&_says(&[said.clone(), py_str("steak"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("chicken"), py_str("meat"), py_str("joint"), py_str("gammon")])) {
    return py_str("Only if you want it leaner: a rim of fat bastes the meat and is what you are paying for, so leave it on and cut it away on the plate. Silverskin - the shiny, tough membrane - is the one that must come off, or the meat curls.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("asparagus")])) {
    return py_str("Yes, and the woody end knows where it is: bend each spear until it snaps, and throw the stub away - it is fibrous and never softens. Do not cut them all at the same length by eye, or you will keep the tough part on the thin spears.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("leek"), py_str("leeks")])) {
    return py_str("Yes: cut off the dark green top for stock, and slit the leek lengthways down the white so the grit between the layers can rinse out. A leek that looks clean is usually the one with sand in it.");
    }
    if py_truthy(&_says(&[said.clone(), py_str("celery")])) {
    return py_str("A rinse and a trim of the base is enough. If the strings bother you, peel them off the outside of the stalk with a vegetable peeler - the flavour is in the stalk, not the strings.");
    }
    return py_str("Only the tough bits: the stalk end of a bean, the woody end of an asparagus spear, the base of a sprout, the stringy root of a spring onion. Trimming more than that wastes the vegetable, and most of what people cut away cooks down to nothing anyway.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.kitchen_prep".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASK" => Some(g__ASK()),
            "_BLANCH" => Some(g__BLANCH()),
            "_BLANCHABLE" => Some(g__BLANCHABLE()),
            "_FOOD" => Some(g__FOOD()),
            "_FREEZING" => Some(g__FREEZING()),
            "_MEAT_ROWS" => Some(g__MEAT_ROWS()),
            "_NOT_FOOD" => Some(g__NOT_FOOD()),
            "_PARBOIL" => Some(g__PARBOIL()),
            "_PAT" => Some(g__PAT()),
            "_PEEL" => Some(g__PEEL()),
            "_PRICK" => Some(g__PRICK()),
            "_RINSE" => Some(g__RINSE()),
            "_SCORE" => Some(g__SCORE()),
            "_SCORE_ASK" => Some(g__SCORE_ASK()),
            "_SIFT" => Some(g__SIFT()),
            "_STEPS" => Some(g__STEPS()),
            "_STEP_ROWS" => Some(g__STEP_ROWS()),
            "_STEP_VERBS" => Some(g__STEP_VERBS()),
            "_TRIM" => Some(g__TRIM()),
            "_a_freezing_question" => Some(py_func("_a_freezing_question", |args: &[Py]| _a_freezing_question(args))),
            "_a_step_the_turn_before" => Some(py_func("_a_step_the_turn_before", |args: &[Py]| _a_step_the_turn_before(args))),
            "_asks_whether_the_step_is_needed" => Some(py_func("_asks_whether_the_step_is_needed", |args: &[Py]| _asks_whether_the_step_is_needed(args))),
            "_blanch_reply" => Some(py_func("_blanch_reply", |args: &[Py]| _blanch_reply(args))),
            "_blanching_or_freezing" => Some(py_func("_blanching_or_freezing", |args: &[Py]| _blanching_or_freezing(args))),
            "_how_or_whether" => Some(py_func("_how_or_whether", |args: &[Py]| _how_or_whether(args))),
            "_parboil_reply" => Some(py_func("_parboil_reply", |args: &[Py]| _parboil_reply(args))),
            "_pat_dry_reply" => Some(py_func("_pat_dry_reply", |args: &[Py]| _pat_dry_reply(args))),
            "_patting_it_dry" => Some(py_func("_patting_it_dry", |args: &[Py]| _patting_it_dry(args))),
            "_peel_reply" => Some(py_func("_peel_reply", |args: &[Py]| _peel_reply(args))),
            "_prick_reply" => Some(py_func("_prick_reply", |args: &[Py]| _prick_reply(args))),
            "_rinse_reply" => Some(py_func("_rinse_reply", |args: &[Py]| _rinse_reply(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_says" => Some(py_func("_says", |args: &[Py]| _says(args))),
            "_score_reply" => Some(py_func("_score_reply", |args: &[Py]| _score_reply(args))),
            "_sift_reply" => Some(py_func("_sift_reply", |args: &[Py]| _sift_reply(args))),
            "_trim_reply" => Some(py_func("_trim_reply", |args: &[Py]| _trim_reply(args))),
            "entry" => Some(g_entry()),
            "last_user" => Some(g_last_user()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASK" => s__ASK(v),
            "_BLANCH" => s__BLANCH(v),
            "_BLANCHABLE" => s__BLANCHABLE(v),
            "_FOOD" => s__FOOD(v),
            "_FREEZING" => s__FREEZING(v),
            "_MEAT_ROWS" => s__MEAT_ROWS(v),
            "_NOT_FOOD" => s__NOT_FOOD(v),
            "_PARBOIL" => s__PARBOIL(v),
            "_PAT" => s__PAT(v),
            "_PEEL" => s__PEEL(v),
            "_PRICK" => s__PRICK(v),
            "_RINSE" => s__RINSE(v),
            "_SCORE" => s__SCORE(v),
            "_SCORE_ASK" => s__SCORE_ASK(v),
            "_SIFT" => s__SIFT(v),
            "_STEPS" => s__STEPS(v),
            "_STEP_ROWS" => s__STEP_ROWS(v),
            "_STEP_VERBS" => s__STEP_VERBS(v),
            "_TRIM" => s__TRIM(v),
            "entry" => s_entry(v),
            "last_user" => s_last_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.kitchen_prep", module_obj);
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_asks_whether_the_step_is_needed", py_func("_asks_whether_the_step_is_needed", |args: &[Py]| _asks_whether_the_step_is_needed(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_how_or_whether", py_func("_how_or_whether", |args: &[Py]| _how_or_whether(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_patting_it_dry", py_func("_patting_it_dry", |args: &[Py]| _patting_it_dry(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_a_step_the_turn_before", py_func("_a_step_the_turn_before", |args: &[Py]| _a_step_the_turn_before(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_a_freezing_question", py_func("_a_freezing_question", |args: &[Py]| _a_freezing_question(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_blanching_or_freezing", py_func("_blanching_or_freezing", |args: &[Py]| _blanching_or_freezing(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_says", py_func("_says", |args: &[Py]| _says(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_sift_reply", py_func("_sift_reply", |args: &[Py]| _sift_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_peel_reply", py_func("_peel_reply", |args: &[Py]| _peel_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_prick_reply", py_func("_prick_reply", |args: &[Py]| _prick_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_score_reply", py_func("_score_reply", |args: &[Py]| _score_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_blanch_reply", py_func("_blanch_reply", |args: &[Py]| _blanch_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_parboil_reply", py_func("_parboil_reply", |args: &[Py]| _parboil_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_rinse_reply", py_func("_rinse_reply", |args: &[Py]| _rinse_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_pat_dry_reply", py_func("_pat_dry_reply", |args: &[Py]| _pat_dry_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.kitchen_prep", "_trim_reply", py_func("_trim_reply", |args: &[Py]| _trim_reply(args)));
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_ASK", g__ASK, s__ASK);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_BLANCH", g__BLANCH, s__BLANCH);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_BLANCHABLE", g__BLANCHABLE, s__BLANCHABLE);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_FOOD", g__FOOD, s__FOOD);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_FREEZING", g__FREEZING, s__FREEZING);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_MEAT_ROWS", g__MEAT_ROWS, s__MEAT_ROWS);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_NOT_FOOD", g__NOT_FOOD, s__NOT_FOOD);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_PARBOIL", g__PARBOIL, s__PARBOIL);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_PAT", g__PAT, s__PAT);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_PEEL", g__PEEL, s__PEEL);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_PRICK", g__PRICK, s__PRICK);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_RINSE", g__RINSE, s__RINSE);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_SCORE", g__SCORE, s__SCORE);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_SCORE_ASK", g__SCORE_ASK, s__SCORE_ASK);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_SIFT", g__SIFT, s__SIFT);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_STEPS", g__STEPS, s__STEPS);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_STEP_ROWS", g__STEP_ROWS, s__STEP_ROWS);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_STEP_VERBS", g__STEP_VERBS, s__STEP_VERBS);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "_TRIM", g__TRIM, s__TRIM);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.kitchen_prep", "last_user", g_last_user, s_last_user);
}
