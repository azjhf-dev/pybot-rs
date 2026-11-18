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
thread_local! { static G__A_TRIP_OF_SOME_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEEDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.kit_lists: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TRIP_OF_SOME_LENGTH() -> Py { ensure_init(); G__A_TRIP_OF_SOME_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TRIP_OF_SOME_LENGTH(v: Py) { ensure_init(); G__A_TRIP_OF_SOME_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__NEEDS() -> Py { ensure_init(); G__NEEDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEEDS(v: Py) { ensure_init(); G__NEEDS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The kit for a small plan: a picnic, a gym bag, a spa day, an interview, a barbecue, a day at\nthe beach, an exam, a cold, a race, a hospital bag, a driving test and a swim.\n\n\"What do i need for a picnic\", \"what should i take to a barbecue\" and the like are the commonest\nof the everyday kit questions, and 42 of the 103 deck turns that ask what somebody needs for a\nthing were answered with a shrug (answers, #answers 16). Each row is keyed by the plan and asks\nfor a need word, so \"how do i cook on a barbecue\" keeps its own answer.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kit_lists.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("food-and-drink"), py_str("fitness"), py_str("work")]));
    fn __data1() -> Py { py_tuple(vec![py_str("need"), py_str("needs"), py_str("needed"), py_str("bring"), py_str("brings"), py_str("take"), py_str("takes"), py_str("pack"), py_str("packing"), py_str("kit"), py_str("list"), py_str("prepare"), py_str("prep"), py_str("essentials"), py_str("supplies")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kit_lists._NEEDS"); }
    s__NEEDS(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kit_lists._A_TRIP_OF_SOME_LENGTH"); }
    s__A_TRIP_OF_SOME_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:holiday|holidays|vacation|vacations|fortnight|fortnights|week away|weeks away|week at|weeks at|week in|weeks in|long weekend|next week|next month|week after next|\\d+\\s*(?:days?|nights?|weeks?)|a couple of (?:days|nights|weeks)|couple of (?:days|nights|weeks)|few (?:days|nights|weeks))\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("table"), py_str("bench"), py_str("site"), py_str("recipe"), py_str("cake"), py_str("bread"), py_str("sandwich"), py_str("blanket"), py_str("basket"), py_str("hamper"), py_str("good"), py_str("best"), py_str("nice"), py_str("ideas"), py_str("food"), py_str("eat"), py_str("something"), py_str("lunch"), py_str("snack")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("how long"), py_str("how much"), py_str("parking"), py_str("car park"), py_str("tide"), py_str("tides"), py_str("swim"), py_str("swimming"), py_str("dog"), py_str("dogs"), py_str("wear"), py_str("nearest"), py_str("seaside"), py_str("coast"), py_str("hotel"), py_str("hotels"), py_str("stay"), py_str("staying"), py_str("place to stay"), py_str("accommodation"), py_str("somewhere to stay"), py_str("guest house"), py_str("b&b"), py_str("airbnb"), py_str("food"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("lunch"), py_str("dinner"), py_str("breakfast"), py_str("snack"), py_str("snacks"), py_str("picnic"), py_str("restaurant"), py_str("cafe"), py_str("get there"), py_str("travel"), py_str("train"), py_str("coach"), py_str("flight"), py_str("flights")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("revise"), py_str("revision"), py_str("revising"), py_str("how to"), py_str("how long"), py_str("result"), py_str("results"), py_str("grade"), py_str("grades"), py_str("resit"), py_str("coursework"), py_str("panic"), py_str("nervous"), py_str("anxiety"), py_str("remark"), py_str("re-mark")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("sore"), py_str("sore throat"), py_str("chest"), py_str("cough"), py_str("coughs"), py_str("flu"), py_str("how long"), py_str("how to"), py_str("get rid"), py_str("shift"), py_str("cure"), py_str("medicine"), py_str("tablets"), py_str("antibiotics"), py_str("doctor"), py_str("gp"), py_str("child"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("toddler"), py_str("pregnant"), py_str("asthma")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("training"), py_str("train for"), py_str("plan"), py_str("how long"), py_str("how to"), py_str("pace"), py_str("pacing"), py_str("beginner"), py_str("schedule"), py_str("lose weight"), py_str("diet"), py_str("injury"), py_str("injured"), py_str("knee"), py_str("shin"), py_str("blister"), py_str("blisters")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("appointment"), py_str("appointments"), py_str("scan"), py_str("scans"), py_str("surgery"), py_str("operation"), py_str("admission"), py_str("discharge"), py_str("visit"), py_str("visiting"), py_str("letter"), py_str("parking"), py_str("nursery"), py_str("car seat"), py_str("cot")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("theory"), py_str("provisional"), py_str("licence"), py_str("license"), py_str("book"), py_str("booking"), py_str("cancel"), py_str("cost"), py_str("price"), py_str("fail"), py_str("failed"), py_str("passed"), py_str("nervous"), py_str("lessons"), py_str("instructor"), py_str("insurance"), py_str("mot"), py_str("age"), py_str("how long"), py_str("waiting"), py_str("what happens"), py_str("tips"), py_str("advice")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("learn"), py_str("cant"), py_str("can't"), py_str("scared"), py_str("fear"), py_str("afraid"), py_str("breaststroke"), py_str("front crawl"), py_str("technique"), py_str("how long"), py_str("how often"), py_str("how much"), py_str("price"), py_str("cost"), py_str("membership"), py_str("sea"), py_str("open water"), py_str("lose weight"), py_str("child"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("baby"), py_str("lane"), py_str("club"), py_str("classes"), py_str("timetable"), py_str("lesson"), py_str("lessons"), py_str("class"), py_str("course")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.kit_lists.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-picnic-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("picnic"), py_str("picnics")]), g__NEEDS()])); __args.push(py_str("A rug, a cool bag with a freezer block, plates and cups you do not mind losing, a bottle opener, napkins, a bin bag and water. Skip anything heavy with mayonnaise unless it is eaten within the hour: sausage rolls, cheese, fruit and a flask keep for hours.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data2())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-gym-bag-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gym"), py_str("gyms"), py_str("workout"), py_str("workouts")]), g__NEEDS()])); __args.push(py_str("For the gym: trainers you only wear indoors, two towels (one for the floor and machines), a water bottle, a padlock for the locker, deodorant and a bag for wet kit. A resistance band and a notebook beat anything else a shop will sell you.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("join"), py_str("membership"), py_str("contract"), py_str("cancel"), py_str("price"), py_str("cost"), py_str("cheap"), py_str("app")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-spa-day-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spa"), py_str("spas")]), g__NEEDS()])); __args.push(py_str("Swimwear, flip-flops, a towel and robe if the place does not lend them, a hair tie, a bottle of water, and your own shampoo if you are fussy. Leave the phone in the locker: the pools and steam rooms dehydrate you faster than people expect.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), py_tuple(vec![py_str("price"), py_str("cost"), py_str("cheap"), py_str("voucher"), py_str("book"), py_str("booking"), py_str("cancel")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("an-interview-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("interview"), py_str("interviews")]), g__NEEDS()])); __args.push(py_str("Take the letter or email with the address and time, a printed copy of your CV, a pen and a notepad, and the name of the person you are meeting. Photo ID for the building, and ask at the desk for them by name.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), py_tuple(vec![py_str("questions"), py_str("answers"), py_str("nervous"), py_str("nerves"), py_str("salary"), py_str("job offer"), py_str("thank you letter")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-barbecue-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("barbecue"), py_str("bbq"), py_str("barbeque"), py_str("cookout")]), g__NEEDS()])); __args.push(py_str("Charcoal (about 1kg for ten people), a chimney starter and firelighters, long tongs, a wire brush, a tray for the cooked meat, and a meat thermometer. Put a bucket of water or sand beside it; a gas one needs a spare bottle instead.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), py_tuple(vec![py_str("sauce"), py_str("marinade"), py_str("rub"), py_str("recipe"), py_str("how long"), py_str("temperature"), py_str("weather")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-beach-day-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("beach day"), py_str("beach"), py_str("beach trip"), py_str("day at the beach")]), g__NEEDS()])); __args.push(py_func("_the_day_at_the_beach", |args: &[Py]| _the_day_at_the_beach(args))); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data3())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("an-exam-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("exam"), py_str("exams"), py_str("mock exam"), py_str("gcse exam"), py_str("a level exam")]), g__NEEDS()])); __args.push(py_str("Two pens, a pencil, a spare, an eraser, and your photo ID plus the letter with the room and the time on it. A clear bottle of water if the rules allow one, and nothing else in your pockets - a phone switched off still counts as being in the room, so leave it at home.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data4())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-cold-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("a cold"), py_str("my cold"), py_str("this cold"), py_str("a bad cold"), py_str("a chesty cold")]), g__NEEDS()])); __args.push(py_str("Paracetamol or ibuprofen, a decongestant spray, tissues, honey and lemon, and a thermometer. Rest and plenty to drink do most of the work, and a cold clears in about a week without antibiotics - see a GP if you are short of breath, if a fever lasts more than five days, or if you are getting worse after day five rather than better.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data5())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-race-day-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("marathon"), py_str("half marathon"), py_str("a 10k"), py_str("race day")]), g__NEEDS()])); __args.push(py_str("Nothing new on the day: the shoes, socks and kit you did your long runs in, plus your race number, safety pins, a bottle, and the gels or sweets you have practised with. Eat the breakfast you have tested, get there an hour early, and set off slower than feels right.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data6())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-hospital-bag")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hospital bag"), py_str("baby bag"), py_str("birth bag"), py_str("bag for hospital"), py_str("bag for the hospital")]), g__NEEDS()])); __args.push(py_str("Nappies and wipes, three or four babygrows and vests, maternity pads, your own toiletries and a phone charger with a long cable, plus snacks and a water bottle. Pack two bags - a small one for the labour room and a bigger one for the ward - and keep your notes and ID on the top of one, because someone asks for them at the door.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data7())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-driving-test-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("driving test"), py_str("practical test"), py_str("practical driving test"), py_str("driving test day")]), g__NEEDS()])); __args.push(py_str("Your provisional licence and the theory pass certificate if you have the paper one, and the car you are using, with L plates front and back, insurance that covers a learner, a valid MOT and enough fuel. Take glasses or contact lenses if you need them to drive, and be there ten minutes early - the examiner reads the documents before you move off.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data8())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-swim-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("swimming"), py_str("a swim"), py_str("swim session"), py_str("swimming session")]), g__NEEDS()])); __args.push(py_str("Goggles, a swimming costume or trunks, a towel and a padlock for the locker, plus flip-flops for the poolside and a bottle of water. A swim hat if the pool asks for one, and a pound coin for an old locker.")); __kws.extend(vec![("ask".to_string(), g__NEEDS()), ("guard".to_string(), py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))), ("avoid".to_string(), __data9())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _a_trip_of_some_length(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is about a stay of some days rather than one occasion.");
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    return call_builtin("bool", &[mcall(&g__A_TRIP_OF_SOME_LENGTH(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_for_the_kit(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is asking what to have with them, for one occasion and not a trip.");
    words = call_builtin("set", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NEEDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_trip_of_some_length(&[understood.clone()]))) } };
    Py::None
}
pub fn _a_trip_said_before(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when a turn or two back named a stay of some days rather than the day itself.\n\n    A guard cannot read the conversation, so the trip is read here: \"my holiday is next week\"\n    then \"what should i pack for a beach\" is the week's packing list, which another row answers,\n    and the day's sun cream is the wrong answer to it.\n    ");
    turns = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    earlier = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    return call_builtin("bool", &[mcall(&g__A_TRIP_OF_SOME_LENGTH(), "search", &[mcall(&py_str(" "), "join", &[earlier.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_day_at_the_beach(args: &[Py]) -> Py {
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
    let _ = py_str("The kit for a day at the beach, or nothing when the conversation is a trip away.");
    if py_truthy(&_a_trip_said_before(&[memory.clone()])) {
    return Py::None;
    }
    return py_str("Factor 30 or higher sun cream, a bottle of water each, a hat, and a windbreak or umbrella for shade, plus a towel each and one spare for the car. Take cash for parking and a bag for your rubbish, because bins on the sand are thin on the ground.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.kit_lists".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_TRIP_OF_SOME_LENGTH" => Some(g__A_TRIP_OF_SOME_LENGTH()),
            "_NEEDS" => Some(g__NEEDS()),
            "_a_trip_of_some_length" => Some(py_func("_a_trip_of_some_length", |args: &[Py]| _a_trip_of_some_length(args))),
            "_a_trip_said_before" => Some(py_func("_a_trip_said_before", |args: &[Py]| _a_trip_said_before(args))),
            "_asks_for_the_kit" => Some(py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args))),
            "_the_day_at_the_beach" => Some(py_func("_the_day_at_the_beach", |args: &[Py]| _the_day_at_the_beach(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_A_TRIP_OF_SOME_LENGTH" => s__A_TRIP_OF_SOME_LENGTH(v),
            "_NEEDS" => s__NEEDS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.kit_lists", module_obj);
    pyrt::xrefs::reg_fn("knowledge.kit_lists", "_a_trip_of_some_length", py_func("_a_trip_of_some_length", |args: &[Py]| _a_trip_of_some_length(args)));
    pyrt::xrefs::reg_fn("knowledge.kit_lists", "_asks_for_the_kit", py_func("_asks_for_the_kit", |args: &[Py]| _asks_for_the_kit(args)));
    pyrt::xrefs::reg_fn("knowledge.kit_lists", "_a_trip_said_before", py_func("_a_trip_said_before", |args: &[Py]| _a_trip_said_before(args)));
    pyrt::xrefs::reg_fn("knowledge.kit_lists", "_the_day_at_the_beach", py_func("_the_day_at_the_beach", |args: &[Py]| _the_day_at_the_beach(args)));
    pyrt::xrefs::reg_access("knowledge.kit_lists", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.kit_lists", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.kit_lists", "_A_TRIP_OF_SOME_LENGTH", g__A_TRIP_OF_SOME_LENGTH, s__A_TRIP_OF_SOME_LENGTH);
    pyrt::xrefs::reg_access("knowledge.kit_lists", "_NEEDS", g__NEEDS, s__NEEDS);
    pyrt::xrefs::reg_access("knowledge.kit_lists", "entry", g_entry, s_entry);
}
