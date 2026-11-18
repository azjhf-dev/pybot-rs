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
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GARMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_GARMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMELL_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GARMENT_THEN_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ODOUR_SOURCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_food_is_being_kept: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_only_a_container: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.odours: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_GARMENT() -> Py { ensure_init(); G_GARMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GARMENT(v: Py) { ensure_init(); G_GARMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_GARMENT() -> Py { ensure_init(); G_NOT_A_GARMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_GARMENT(v: Py) { ensure_init(); G_NOT_A_GARMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_SMELL() -> Py { ensure_init(); G_SMELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMELL(v: Py) { ensure_init(); G_SMELL.with(|c| *c.borrow_mut() = v); }
pub fn g_SMELL_SAID() -> Py { ensure_init(); G_SMELL_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMELL_SAID(v: Py) { ensure_init(); G_SMELL_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__GARMENT_THEN_FOOD() -> Py { ensure_init(); G__GARMENT_THEN_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GARMENT_THEN_FOOD(v: Py) { ensure_init(); G__GARMENT_THEN_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__ODOUR_SOURCE() -> Py { ensure_init(); G__ODOUR_SOURCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ODOUR_SOURCE(v: Py) { ensure_init(); G__ODOUR_SOURCE.with(|c| *c.borrow_mut() = v); }
pub fn g_a_food_is_being_kept() -> Py { ensure_init(); G_a_food_is_being_kept.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_food_is_being_kept(v: Py) { ensure_init(); G_a_food_is_being_kept.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_only_a_container() -> Py { ensure_init(); G_only_a_container.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_only_a_container(v: Py) { ensure_init(); G_only_a_container.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Odours in clothes: the smoke a coat holds, a smell a garment has picked up, and gym kit.\n\nThree everyday questions, and none of them had its own answer:\n\n  how do i get smoke smell out of a coat     the room: \"Air the room hard, wash the soft things\n                                             (curtains, cushions, throws)\"\n  how do i get the smell out of my coat      \"I'm not sure about that one, sorry\"\n  my jacket smells of bonfire                \"Thanks for telling me - I'm here if you want to dig\n                                             into any of that.\"\n  how do i get smoke out of a wool coat      nothing\n  my gym kit smells even after washing       \"Buy one adjustable dumbbell or one kettlebell ...\"\n  how do i get sweat smell out of gym clothes the biology of nervous sweat, and no fix\n\nThe smoke card is about a room - open the windows, wash the curtains - so a coat that came back\nfrom a bonfire gets told to air the house. The gym kit line reached a strength-training answer\nbecause \"kit\" is what a home gym is called in Britain.\n\nEach row names the garment and the smell that is in it, so the room, the washing machine, the shoes,\nthe towels and the fridge keep the cards they already answer well.\n");
    s_a_food_is_being_kept(py_func("a_food_is_being_kept", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "a_food_is_being_kept", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_only_a_container(py_func("only_a_container", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "only_a_container", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("shopping"), py_str("fitness")]));
    fn __data1() -> Py { py_tuple(vec![py_str("coat"), py_str("coats"), py_str("jacket"), py_str("jackets"), py_str("jumper"), py_str("jumpers"), py_str("cardigan"), py_str("fleece"), py_str("hoodie"), py_str("blazer"), py_str("suit"), py_str("suits"), py_str("dress"), py_str("dresses"), py_str("shirt"), py_str("shirts"), py_str("trousers"), py_str("skirt"), py_str("scarf"), py_str("clothes"), py_str("clothing"), py_str("wardrobe"), py_str("outfit"), py_str("uniform"), py_str("costume"), py_str("gloves"), py_str("hat"), py_str("beanie"), py_str("bag"), py_str("backpack"), py_str("rucksack"), py_str("handbag"), py_str("luggage"), py_str("suitcase"), py_str("sleeping bag"), py_str("tent"), py_str("jeans"), py_str("jean"), py_str("denim"), py_str("tshirt"), py_str("t shirt"), py_str("teeshirt"), py_str("tee shirt"), py_str("t-shirts"), py_str("t shirts"), py_str("joggers"), py_str("sweatpants"), py_str("trackies"), py_str("track suit"), py_str("tracksuit"), py_str("pyjamas"), py_str("pajamas"), py_str("pj s"), py_str("pjs"), py_str("nightie"), py_str("nightdress"), py_str("dressing gown"), py_str("bathrobe"), py_str("robe"), py_str("slippers"), py_str("socks"), py_str("sock"), py_str("tights"), py_str("stockings"), py_str("bra"), py_str("bras"), py_str("underwear"), py_str("boxers"), py_str("knickers"), py_str("pants"), py_str("apron"), py_str("overalls"), py_str("scrubs"), py_str("tabard")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.GARMENT"); }
    s_GARMENT(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelly"), py_str("smelling"), py_str("stink"), py_str("stinks"), py_str("stinking"), py_str("stunk"), py_str("odour"), py_str("odor"), py_str("reek"), py_str("reeking"), py_str("whiff"), py_str("musty"), py_str("must"), py_str("damp"), py_str("fusty"), py_str("sweat"), py_str("BO"), py_str("smoke"), py_str("smoky"), py_str("bonfire"), py_str("campfire"), py_str("fire"), py_str("cigarette"), py_str("cigarettes"), py_str("smoking"), py_str("tobacco"), py_str("nicotine"), py_str("vape"), py_str("curry"), py_str("cooking"), py_str("garlic"), py_str("perfume"), py_str("aftershave"), py_str("vinegar"), py_str("bleach"), py_str("petrol"), py_str("diesel"), py_str("paint"), py_str("chemical"), py_str("mildew"), py_str("mouldy")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.SMELL"); }
    s_SMELL(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("room"), py_str("rooms"), py_str("house"), py_str("home"), py_str("flat"), py_str("kitchen"), py_str("bedroom"), py_str("living room"), py_str("hall"), py_str("hallway"), py_str("car"), py_str("van"), py_str("sofa"), py_str("settee"), py_str("couch"), py_str("curtain"), py_str("curtains"), py_str("carpet"), py_str("rug"), py_str("rugs"), py_str("mattress"), py_str("bed"), py_str("bedding"), py_str("duvet"), py_str("pillow"), py_str("pillows"), py_str("sheet"), py_str("sheets"), py_str("towel"), py_str("towels"), py_str("washing machine"), py_str("washer"), py_str("dishwasher"), py_str("fridge"), py_str("freezer"), py_str("bin"), py_str("bins"), py_str("wheelie"), py_str("oven"), py_str("microwave"), py_str("kettle"), py_str("drain"), py_str("sink"), py_str("toilet"), py_str("shower"), py_str("walls"), py_str("wall"), py_str("ceiling"), py_str("paint"), py_str("furniture"), py_str("chair"), py_str("chairs"), py_str("table"), py_str("wardrobe"), py_str("wardrobe door"), py_str("drawer"), py_str("cupboard"), py_str("books"), py_str("book"), py_str("microwave oven"), py_str("car seats"), py_str("hands"), py_str("breath"), py_str("mouth"), py_str("feet"), py_str("shoes"), py_str("shoe"), py_str("trainers"), py_str("boots"), py_str("socks"), py_str("wellies"), py_str("helmet"), py_str("washing-up"), py_str("dishcloth"), py_str("cloth"), py_str("sponge"), py_str("bin bag")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.NOT_A_GARMENT"); }
    s_NOT_A_GARMENT(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("get rid of"), py_str("get out"), py_str("get the smell"), py_str("remove"), py_str("removing"), py_str("take away"), py_str("stop the smell"), py_str("smells of"), py_str("smells like"), py_str("smelling of"), py_str("stinks of"), py_str("stinks like"), py_str("how do i clean"), py_str("how do i wash"), py_str("how do i freshen"), py_str("freshen up"), py_str("air out"), py_str("air it"), py_str("how do i make it smell better"), py_str("how do i get it to stop smelling")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.ASK"); }
    s_ASK(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours._GARMENT_THEN_FOOD"); }
    s__GARMENT_THEN_FOOD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[g_GARMENT()])), &py_str(")\\s+(?:potato|potatoes|baked potato|jacket potato)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelly"), py_str("smelling"), py_str("smelled"), py_str("stink"), py_str("stinks"), py_str("stinking"), py_str("stunk"), py_str("odour"), py_str("odor"), py_str("reek"), py_str("reeks"), py_str("reeking"), py_str("reeked"), py_str("whiff"), py_str("whiffs"), py_str("musty"), py_str("must"), py_str("damp"), py_str("fusty"), py_str("sweat"), py_str("sweaty"), py_str("bo"), py_str("smoky"), py_str("mildew"), py_str("mouldy"), py_str("moldy"), py_str("stale"), py_str("fumes"), py_str("fumey"), py_str("scented")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.SMELL_SAID"); }
    s_SMELL_SAID(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("curry"), py_str("cooking"), py_str("garlic"), py_str("perfume"), py_str("aftershave"), py_str("vinegar"), py_str("bleach"), py_str("petrol"), py_str("diesel"), py_str("paint"), py_str("chemical"), py_str("fire"), py_str("smoke")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours._ODOUR_SOURCE"); }
    s__ODOUR_SOURCE(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("smoke"), py_str("smoky"), py_str("bonfire"), py_str("campfire"), py_str("woodsmoke"), py_str("wood smoke"), py_str("cigarette"), py_str("cigarettes"), py_str("cigarette smoke"), py_str("fag smoke"), py_str("smoking"), py_str("tobacco"), py_str("nicotine"), py_str("vape"), py_str("vaping"), py_str("fire"), py_str("fire pit"), py_str("chiminea"), py_str("bbq"), py_str("barbecue"), py_str("pub"), py_str("stale")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("stain"), py_str("stains"), py_str("stained"), py_str("staining"), py_str("marked"), py_str("yellowed"), py_str("fire alarm"), py_str("smoke alarm"), py_str("fireplace"), py_str("chimney"), py_str("wood burner"), py_str("smoking meat"), py_str("smoker"), py_str("how do i stop smoking"), py_str("quit smoking"), py_str("give up smoking"), py_str("smoke point"), py_str("burnt"), py_str("burning")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("gym kit"), py_str("gym clothes"), py_str("gym gear"), py_str("workout clothes"), py_str("workout gear"), py_str("sports kit"), py_str("running kit"), py_str("running clothes"), py_str("running gear"), py_str("training kit"), py_str("cycling kit"), py_str("gym top"), py_str("sports bra"), py_str("leggings"), py_str("base layer"), py_str("lycra"), py_str("sports clothes"), py_str("exercise clothes"), py_str("kit"), py_str("shorts"), py_str("gym shorts"), py_str("running shorts"), py_str("cycling shorts"), py_str("gym tops"), py_str("running tops"), py_str("gym top"), py_str("running top"), py_str("vest"), py_str("jersey"), py_str("sports socks"), py_str("running socks")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelly"), py_str("smelling"), py_str("stink"), py_str("stinks"), py_str("stinking"), py_str("sweat"), py_str("sweaty"), py_str("odour"), py_str("odor"), py_str("musty"), py_str("damp"), py_str("fusty"), py_str("washing"), py_str("washed"), py_str("wash"), py_str("clean"), py_str("bacteria"), py_str("after washing"), py_str("even after washing")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("dumbbell"), py_str("dumbbells"), py_str("kettlebell"), py_str("weights"), py_str("barbell"), py_str("routine"), py_str("programme"), py_str("program"), py_str("plan"), py_str("workout plan"), py_str("to buy"), py_str("buy"), py_str("which shoes"), py_str("trainers to buy"), py_str("body odour"), py_str("deodorant"), py_str("antiperspirant"), py_str("hyperhidrosis"), py_str("chafing"), py_str("blister"), py_str("runners knee"), py_str("injury"), py_str("chlorine"), py_str("pool"), py_str("swimming pool"), py_str("swim"), py_str("stairs"), py_str("stair"), py_str("landing"), py_str("hall"), py_str("hallway"), py_str("landings"), py_str("room"), py_str("rooms"), py_str("ceiling"), py_str("ceilings"), py_str("wall"), py_str("walls"), py_str("loft"), py_str("lofts"), py_str("attic"), py_str("garage"), py_str("shed"), py_str("sheds"), py_str("cupboard"), py_str("cupboards"), py_str("wardrobe"), py_str("wardrobes")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("sweat"), py_str("sweaty"), py_str("sweat smell"), py_str("smells of sweat"), py_str("smell of sweat"), py_str("mould"), py_str("mouldy"), py_str("mold"), py_str("moldy"), py_str("mildew"), py_str("mildewy"), py_str("mildewed"), py_str("spores"), py_str("black spots"), py_str("green spots"), py_str("swimsuit"), py_str("swimming costume"), py_str("bikini"), py_str("wetsuit"), py_str("wetsuits"), py_str("pool"), py_str("chlorine"), py_str("swimming"), py_str("swim"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("staining"), py_str("marks"), py_str("yellow marks"), py_str("armpit"), py_str("armpits"), py_str("fabric softener"), py_str("conditioner"), py_str("scent"), py_str("perfume to wear"), py_str("which perfume"), py_str("smell nice"), py_str("make it smell nice"), py_str("air freshener"), py_str("candle"), py_str("diffuser"), py_str("smoke"), py_str("smoky"), py_str("bonfire"), py_str("cigarette"), py_str("gym kit"), py_str("gym clothes"), py_str("running kit"), py_str("bag of"), py_str("bags of")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("sweat"), py_str("sweaty"), py_str("sweat smell"), py_str("smells of sweat"), py_str("smell of sweat"), py_str("damp"), py_str("damps"), py_str("musty"), py_str("mustiness"), py_str("damp smell"), py_str("smells damp"), py_str("smell damp"), py_str("smells of damp"), py_str("smell of damp"), py_str("dank"), py_str("mould"), py_str("mouldy"), py_str("mold"), py_str("moldy"), py_str("mildew"), py_str("mildewy"), py_str("mildewed"), py_str("spores"), py_str("black spots"), py_str("green spots"), py_str("swimsuit"), py_str("swimming costume"), py_str("bikini"), py_str("wetsuit"), py_str("wetsuits"), py_str("pool"), py_str("chlorine"), py_str("swimming"), py_str("swim"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("staining"), py_str("marks"), py_str("yellow marks"), py_str("armpit"), py_str("armpits"), py_str("fabric softener"), py_str("conditioner"), py_str("scent"), py_str("perfume to wear"), py_str("which perfume"), py_str("smell nice"), py_str("make it smell nice"), py_str("air freshener"), py_str("candle"), py_str("diffuser"), py_str("smoke"), py_str("smoky"), py_str("bonfire"), py_str("cigarette"), py_str("gym kit"), py_str("gym clothes"), py_str("running kit")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.odours.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smoke-in-clothes")); __args.push(py_tuple(vec![op_add(&g_GARMENT(), &py_tuple(vec![py_str("hood"), py_str("collar"), py_str("sleeves"), py_str("lining"), py_str("wool"), py_str("woollen"), py_str("dry clean only")])), __data7()])); __args.push(py_str("Air it before you wash it: hang it outside, or in a room with a window open, for a day, because most of the smell sits on the surface and blows off. Then wash it at whatever the label allows - 30C for a washable coat, 40C for a jumper - with half the usual detergent and no softener. If it cannot be washed, hang it in a steamy bathroom for twenty minutes and air it again, or put it in a bag with a saucer of bicarbonate of soda overnight. Don't spray perfume on it; that makes smoke plus scent, and it clings harder.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), g_SMELL()), ("avoid".to_string(), op_add(&g_NOT_A_GARMENT(), &__data8()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gym-kit-smell")); __args.push(py_tuple(vec![__data9(), __data10()])); __args.push(py_str("Wash kit cool - 30C is enough - with a scoop of bicarbonate of soda or a sports wash, no fabric softener at all, and air dry it. The smell is bacteria in the synthetic fibres, not in the sweat: sweat itself has no smell, and a warm wash with softener leaves a film the bacteria live in, which is why kit can smell the moment it gets damp again. Get it out of the gym bag the same day and hang it to dry before it goes in the basket.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), g_SMELL()), ("avoid".to_string(), op_add(&g_NOT_A_GARMENT(), &__data11()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smell-in-a-garment")); __args.push(py_tuple(vec![g_GARMENT(), op_add(&g_SMELL(), &py_tuple(vec![py_str("washing"), py_str("washed"), py_str("dry clean"), py_str("dry cleaning"), py_str("freshen"), py_str("air it"), py_str("aired")]))])); __args.push(py_str("Air it outside for a day first, then wash it if it can be washed - at the temperature on the label - with half the usual detergent and no softener, and dry it in the open air rather than on a radiator. Anything that cannot be washed - a wool coat, a blazer, a suit - goes in a steamy bathroom for twenty minutes, or in a bag with a saucer of bicarbonate of soda overnight, and then air. If it comes back after that, the smell is in what it is stored in, so wash the drawer or the wardrobe and let both dry out before the clothes go back.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_SMELL()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = _a_garment_is_not_a_food(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _a_source_is_not_a_smell(&[understood.clone()]) } } })), ("avoid".to_string(), op_add(&g_NOT_A_GARMENT(), &__data12()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smell-in-a-garment-why")); __args.push(py_tuple(vec![g_GARMENT(), op_add(&g_SMELL(), &py_tuple(vec![py_str("washing"), py_str("washed"), py_str("dry clean"), py_str("dry cleaning"), py_str("freshen"), py_str("air it"), py_str("aired")]))])); __args.push(py_str("It is bacteria eating what the wash leaves behind - sweat, body oil and skin cells - and they are not all removed by a cool quick wash, which is why a garment can smell the moment it warms up again. Synthetics hold it worst: polyester and elastane are oil-loving, so the film of body oil stays on the fibre, while cotton and linen let it go. Softener and too much detergent make it worse by leaving their own coating for the bacteria to sit in. So wash at the hottest the label allows with the amount of powder for a dirty load, skip the softener, and dry it fast and in the open air; once the smell has set in a wash at 60 with half a cup of bicarbonate of soda usually shifts it.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), py_tuple(vec![py_str("why"), py_str("how come"), py_str("what makes"), py_str("whats making"), py_str("what is making"), py_str("reason"), py_str("what causes"), py_str("what caused")])), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = _a_garment_is_not_a_food(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _a_source_is_not_a_smell(&[understood.clone()]) } } })), ("avoid".to_string(), op_add(&g_NOT_A_GARMENT(), &__data13()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _a_source_is_not_a_smell(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut source: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a line with a garment and a smell source in it is really about the smell.\n\n    A food, a bleach or a fire in the line is a smell question only when the line says the thing\n    smells: \"my coat smells of curry\" is, \"how do i get curry out of my white shirt\" is a stain. A\n    line with no source word at all is left alone, because \"how do i stop my coat smelling\" names\n    nothing but the smell.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    text = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ODOUR_SOURCE()) { let mut source: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &source.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SMELL_SAID()) { let mut named: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &named.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_garment_is_not_a_food(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a line with a garment word and a smell word in it is about a garment at all.\n\n    A garment word straight in front of a food word names the food: \"jacket potato\", \"jacket\n    potatoes\". Anything else - \"my jacket smells of cooking\", \"my coat smells of roast dinner\" -\n    is still the garment, however many kitchen words it carries.\n\n    A bag is where the shopping is, so a line about a food going mouldy or stale in one is the\n    food's question: \"why does bread go mouldy faster in a bag\" was told to air itself outside\n    for a day and wash at 30C. That is the only garment word in such a line, which is what\n    `only_a_container` checks; \"my gym bag smells musty\" has no food in it and keeps its answer.\n    ");
    words = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    text = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&mcall(&g__GARMENT_THEN_FOOD(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("knowledge.lookup", "a_food_is_being_kept", &[understood.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("knowledge.lookup", "only_a_container", &[understood.clone(), g_GARMENT()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.odours".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK" => Some(g_ASK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "GARMENT" => Some(g_GARMENT()),
            "NOT_A_GARMENT" => Some(g_NOT_A_GARMENT()),
            "SMELL" => Some(g_SMELL()),
            "SMELL_SAID" => Some(g_SMELL_SAID()),
            "_GARMENT_THEN_FOOD" => Some(g__GARMENT_THEN_FOOD()),
            "_ODOUR_SOURCE" => Some(g__ODOUR_SOURCE()),
            "_a_garment_is_not_a_food" => Some(py_func("_a_garment_is_not_a_food", |args: &[Py]| _a_garment_is_not_a_food(args))),
            "_a_source_is_not_a_smell" => Some(py_func("_a_source_is_not_a_smell", |args: &[Py]| _a_source_is_not_a_smell(args))),
            "a_food_is_being_kept" => Some(g_a_food_is_being_kept()),
            "entry" => Some(g_entry()),
            "only_a_container" => Some(g_only_a_container()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASK" => s_ASK(v),
            "ENTRIES" => s_ENTRIES(v),
            "GARMENT" => s_GARMENT(v),
            "NOT_A_GARMENT" => s_NOT_A_GARMENT(v),
            "SMELL" => s_SMELL(v),
            "SMELL_SAID" => s_SMELL_SAID(v),
            "_GARMENT_THEN_FOOD" => s__GARMENT_THEN_FOOD(v),
            "_ODOUR_SOURCE" => s__ODOUR_SOURCE(v),
            "a_food_is_being_kept" => s_a_food_is_being_kept(v),
            "entry" => s_entry(v),
            "only_a_container" => s_only_a_container(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.odours", module_obj);
    pyrt::xrefs::reg_fn("knowledge.odours", "_a_source_is_not_a_smell", py_func("_a_source_is_not_a_smell", |args: &[Py]| _a_source_is_not_a_smell(args)));
    pyrt::xrefs::reg_fn("knowledge.odours", "_a_garment_is_not_a_food", py_func("_a_garment_is_not_a_food", |args: &[Py]| _a_garment_is_not_a_food(args)));
    pyrt::xrefs::reg_access("knowledge.odours", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.odours", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("knowledge.odours", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.odours", "GARMENT", g_GARMENT, s_GARMENT);
    pyrt::xrefs::reg_access("knowledge.odours", "NOT_A_GARMENT", g_NOT_A_GARMENT, s_NOT_A_GARMENT);
    pyrt::xrefs::reg_access("knowledge.odours", "SMELL", g_SMELL, s_SMELL);
    pyrt::xrefs::reg_access("knowledge.odours", "SMELL_SAID", g_SMELL_SAID, s_SMELL_SAID);
    pyrt::xrefs::reg_access("knowledge.odours", "_GARMENT_THEN_FOOD", g__GARMENT_THEN_FOOD, s__GARMENT_THEN_FOOD);
    pyrt::xrefs::reg_access("knowledge.odours", "_ODOUR_SOURCE", g__ODOUR_SOURCE, s__ODOUR_SOURCE);
    pyrt::xrefs::reg_access("knowledge.odours", "a_food_is_being_kept", g_a_food_is_being_kept, s_a_food_is_being_kept);
    pyrt::xrefs::reg_access("knowledge.odours", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.odours", "only_a_container", g_only_a_container, s_only_a_container);
}
