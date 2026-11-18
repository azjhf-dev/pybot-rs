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
thread_local! { static G__A_FINISH_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COAT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COAT_FINISHES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__finish: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__said: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__words: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__ASK() -> Py { ensure_init(); G__ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK(v: Py) { ensure_init(); G__ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FINISH_NAMED() -> Py { ensure_init(); G__A_FINISH_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FINISH_NAMED(v: Py) { ensure_init(); G__A_FINISH_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__COAT_ASK() -> Py { ensure_init(); G__COAT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COAT_ASK(v: Py) { ensure_init(); G__COAT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__COAT_FINISHES() -> Py { ensure_init(); G__COAT_FINISHES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COAT_FINISHES(v: Py) { ensure_init(); G__COAT_FINISHES.with(|c| *c.borrow_mut() = v); }
pub fn g__entry() -> Py { ensure_init(); G__entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__entry(v: Py) { ensure_init(); G__entry.with(|c| *c.borrow_mut() = v); }
pub fn g__finish() -> Py { ensure_init(); G__finish.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__finish(v: Py) { ensure_init(); G__finish.with(|c| *c.borrow_mut() = v); }
pub fn g__said() -> Py { ensure_init(); G__said.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__said(v: Py) { ensure_init(); G__said.with(|c| *c.borrow_mut() = v); }
pub fn g__words() -> Py { ensure_init(); G__words.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__words(v: Py) { ensure_init(); G__words.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How long a job takes: the duration of the thing, asked as a length of time.\n\nA whole family of everyday lines asks for a length of time and gets the card that explains the\nthing instead - how to do it, what it is, why it happens:\n\n    how long does it take to defrost a chicken   -> what a part-thawed chicken means\n    how long does a kitchen renovation take      -> how to get quotes\n    how long does an mot take                    -> how often the test is due\n    how long after a filling can i eat           -> what a filling feels like\n\nNone of those cards says the length of time the line asked for, so the person's actual question\nwent unanswered. Where the card does hold the figure, it holds it in its third sentence, after\nthe method, and STYLE.md asks for the answer in the first sentence.\n\nThe rows here are that one shape: the thing, and a line that asks how long it takes. Every row\ncarries the asking in `ask=`, so a line that says the same thing without asking keeps the card\nthat answers it:\n\n    i passed my driving test last week   -> the congratulations, not the test's 40 minutes\n    how do i defrost a chicken           -> the method card\n    how much is an mot                   -> the MOT price row\n    how long does an mot last            -> the MOT is annual, not a length of time\n    how do i get my deposit back         -> the deposit card\n\nWhat is deliberately not here: the cooking timetables (cooking.py roasts and boils every food\nwith its own minutes), keeping in the fridge (keeping.py and freshness.py), the lifespan of an\nappliance (home.py's LIFE table) and the time a machine takes to run a cycle (home.py's\nMACHINE_TAKES). Those four already answer \"how long\" for their own subject, and a second table\nfor them would drift.\n\nThe answers lane owns this file: pass a line you do not answer rather than guessing.\n");
    s__entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s__ASK(py_tuple(vec![py_str("how long"), py_str("how soon"), py_str("how many minutes"), py_str("how many hours"), py_str("how many days"), py_str("how many weeks"), py_str("how many months"), py_str("how much longer"), py_str("how much time"), py_str("how long till"), py_str("how long until")]));
    s__COAT_FINISHES(py_tuple(vec![py_tuple(vec![py_str("varnish"), py_tuple(vec![py_str("varnish"), py_str("varnishing"), py_str("varnished"), py_str("polyurethane"), py_str("lacquer"), py_str("varnish coats"), py_str("coat of varnish"), py_str("coats of varnish")]), py_str("Two coats, four to six hours apart for a water-based varnish - and 8 to 24 hours for an oil-based polyurethane, which is what most furniture varnish is, so read the tin before the second coat. Touch it first: hard and not tacky means it is ready, and a second coat on a tacky surface drags the first about.")]), py_tuple(vec![py_str("wood-stain"), py_tuple(vec![py_str("wood stain"), py_str("woodstain"), py_str("woodstain coats"), py_str("stain"), py_str("staining"), py_str("stained"), py_str("stain coats"), py_str("coats of stain"), py_str("coat of stain")]), py_str("One coat of wood stain is usually enough on new wood and two on anything that has gone grey, and a second coat goes on after four to six hours - sooner than that and it lifts the first. Wipe off the excess before it dries, because stain that pools sets patchy.")]), py_tuple(vec![py_str("oil-finish"), py_tuple(vec![py_str("danish oil"), py_str("teak oil"), py_str("tung oil"), py_str("linseed oil"), py_str("boiled linseed oil"), py_str("wood oil"), py_str("hardwax oil"), py_str("oil finish"), py_str("oak oil")]), py_str("Two or three coats, 8 to 24 hours apart for a danish, teak, tung or linseed oil - thin coats, the excess wiped off after half an hour, and the tin's own figure wins. It is dry to the touch in a few hours but not ready for the next coat, and an oil finish soaks into bare wood rather than sitting on it as a film.")]), py_tuple(vec![py_str("gloss"), py_tuple(vec![py_str("gloss"), py_str("gloss paint"), py_str("eggshell"), py_str("satin"), py_str("satinwood"), py_str("undercoat"), py_str("oil based"), py_str("oil-based"), py_str("coat of gloss"), py_str("coats of gloss")]), py_str("Two coats of gloss over an undercoat, a night apart: gloss, eggshell, satin and anything oil-based want 16 hours or more, so a door or a skirting board is a two-day job. Rub it down lightly between coats so the second one grips.")])]));
    s__A_FINISH_NAMED(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COAT_FINISHES()) { let __unpk4733 = py_unpack(&__item.clone(), 3, None); let mut _finish: Py = __unpk4733[0].clone(); let mut words: Py = __unpk4733[1].clone(); let mut _said: Py = __unpk4733[2].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__COAT_ASK(op_add(&g__ASK(), &py_tuple(vec![py_str("how many coats"), py_str("how many coats of")])));
    s_ANSWERS(py_tuple(vec![py_str("cars"), py_str("home-and-diy"), py_str("cooking"), py_str("food-and-drink"), py_str("health"), py_str("fitness"), py_str("technology"), py_str("sport"), py_str("friends-and-relationships")]));
    fn __data2() -> Py { py_tuple(vec![py_str("book"), py_str("booking"), py_str("cancel"), py_str("cost"), py_str("price"), py_str("how much"), py_str("theory"), py_str("passed"), py_str("pass"), py_str("fail"), py_str("failed"), py_str("nervous"), py_str("next week"), py_str("tomorrow"), py_str("on the 14th"), py_str("how do i"), py_str("how can i")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("charge my phone"), py_str("charge a phone"), py_str("charge the phone"), py_str("charge my iphone"), py_str("charging my phone"), py_str("charging a phone"), py_str("charging the phone"), py_str("charge my mobile"), py_str("charge my tablet"), py_str("charge my laptop"), py_str("charge a laptop"), py_str("charge a tablet"), py_str("charging a laptop"), py_str("phone charge"), py_str("charging time"), py_str("charge it up"), py_str("charge up my phone")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("defrost a chicken"), py_str("defrost the chicken"), py_str("defrosting a chicken"), py_str("defrost chicken"), py_str("thaw a chicken"), py_str("thawing a chicken"), py_str("defrost a turkey"), py_str("defrost a bird"), py_str("frozen chicken"), py_str("defrost meat"), py_str("defrost mince"), py_str("defrost fish"), py_str("defrosting meat")]) }
    fn __data1() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mot-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_tuple(vec![py_str("mot"), py_str("the mot"), py_str("an mot"), py_str("mot test")]), py_tuple(vec![py_str("take"), py_str("takes"), py_str("taking")])])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("mot")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("The test itself takes 45 minutes to an hour, and a retest about 20 minutes. Stay at the garage if you can, because the tester can show you the fail on the ramp, and you can drive it home afterwards unless the fail sheet says dangerous.")), ("avoid".to_string(), py_tuple(vec![py_str("last"), py_str("lasts"), py_str("lasting"), py_str("valid"), py_str("validity"), py_str("yearly"), py_str("annually"), py_str("how often"), py_str("every year"), py_str("expire"), py_str("fail sheet"), py_str("retake")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("driving-test-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_tuple(vec![py_str("driving test"), py_str("practical driving test"), py_str("practical test")]), py_tuple(vec![py_str("how long"), py_str("long"), py_str("take"), py_str("takes"), py_str("taking")])])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("driving test"), py_str("practical test")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("The test itself is about 40 minutes of driving, and the whole appointment about an hour with the eyesight check and the show-me questions first. The examiner tells you the result before you leave, so you are not waiting for it.")), ("avoid".to_string(), __data2()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kitchen-renovation-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("kitchen renovation"), py_str("kitchen refit"), py_str("renovation"), py_str("refit")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("renovation"), py_str("refit")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("Six to ten weeks on site for a kitchen once the units arrive, and three to five for a bathroom. The units are the long wait - order them six to eight weeks before you want the work done - so work the order date back from the day you want it finished.")), ("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("price"), py_str("how much"), py_str("cheaper"), py_str("quote"), py_str("quotes"), py_str("budget"), py_str("save"), py_str("myself"), py_str("yourself"), py_str("diy")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("paint-between-coats")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("between coats"), py_str("coats of paint"), py_str("second coat"), py_str("next coat"), py_str("recoat"), py_str("painting take")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("coat"), py_str("recoat"), py_str("paint"), py_str("varnish"), py_str("stain"), py_str("staining")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("Two to four hours between coats of emulsion, and a full day before you push furniture back against it. Gloss and anything oil-based want overnight, wood stain and water-based varnish four to six hours, and an oil-based varnish 8 to 24. Touch it first: hard and not tacky means it is ready, and the tin wins where it disagrees.")), ("avoid".to_string(), g__A_FINISH_NAMED()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("charge-a-phone-takes")); __kws.extend(vec![("key".to_string(), __data3()), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("charge"), py_str("charging")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("An hour and a half to two hours takes a phone, tablet or laptop from nearly flat to full, and half an hour on a fast charger gets you about half way. Overnight is fine - a modern one stops charging at 100% - but heat is what ages a battery, so take it out of a thick case and off the radiator.")), ("avoid".to_string(), py_tuple(vec![py_str("not charging"), py_str("wont charge"), py_str("will not charge"), py_str("draining"), py_str("drains"), py_str("hot"), py_str("warm"), py_str("cable"), py_str("charger"), py_str("battery health"), py_str("overnight"), py_str("yesterday")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("defrost-a-chicken-takes")); __kws.extend(vec![("key".to_string(), __data4()), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("defrost"), py_str("thaw"), py_str("frozen")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("A whole chicken takes about 24 hours in the fridge, pieces take 8 to 12, and a turkey two to three days. Keep it on the bottom shelf in a bowl, and never defrost meat on the worktop or in the sink - the outside warms long before the middle.")), ("avoid".to_string(), py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("roast"), py_str("oven"), py_str("microwave"), py_str("how do i"), py_str("how does"), py_str("safe"), py_str("went warm"), py_str("left out")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("after-a-filling-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("after a filling"), py_str("after my filling"), py_str("after the filling"), py_str("after a tooth filling"), py_str("after a dental filling"), py_str("after an extraction"), py_str("after a tooth out"), py_str("numb after a filling"), py_str("numbness after a filling")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("filling"), py_str("extraction")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("An hour to three hours, until the numbness has gone - after that you can eat, though keep to soft, cool food for the rest of the day. Chewing a hot or hard meal while you are still numb is how people bite their cheek or burn their mouth.")), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-bruise-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("bruise heal"), py_str("a bruise take"), py_str("bruises take"), py_str("bruise last"), py_str("bruising take"), py_str("black eye take"), py_str("bruise fade")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("bruise"), py_str("black eye")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("One to two weeks for a bruise to fade, going purple, then green, then yellow as the blood is cleared. A cold pack for 20 minutes at a time over the first day keeps it smaller, and a bruise that turns up with no knock behind it, or that you keep getting, is worth a blood test.")), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("swimming-after-eating-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("swimming after eating"), py_str("swim after eating"), py_str("swim after lunch"), py_str("swim after a meal"), py_str("swimming after a meal"), py_str("after eating before swimming"), py_str("before swimming")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("swim"), py_str("swimming")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("About half an hour after a big meal, and no wait at all after a snack: the cramp rule comes from one small study and the evidence does not back it up. Keep out of deep water if you have been drinking, and don't swim on a stomach that feels too full.")), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-to-swim-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_tuple(vec![py_str("learn to swim"), py_str("learning to swim"), py_str("learn swimming"), py_str("swim lessons"), py_str("swimming lessons"), py_str("adult swimming lessons"), py_str("lessons to swim"), py_str("swim a length"), py_str("swim 25 metres")]), py_tuple(vec![py_str("how long"), py_str("long"), py_str("take"), py_str("takes"), py_str("taking"), py_str("till"), py_str("until")])])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("swim"), py_str("swimming")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("Most adults go from nervous to swimming a length in one course of weekly lessons, which is 10 to 12 weeks. Swimming 25 metres without stopping takes a term, and two terms if you are frightened of the water - lessons twice a week get there faster than once a week, because the water stops feeling strange.")), ("avoid".to_string(), py_tuple(vec![py_str("toddler"), py_str("baby"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("year old"), py_str("years old"), py_str("school"), py_str("nursery")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("swim-a-mile-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_tuple(vec![py_str("swim a mile"), py_str("swim one mile"), py_str("swim 1 mile"), py_str("swim a kilometre"), py_str("swim 1600m"), py_str("swim a km")]), py_tuple(vec![py_str("how long"), py_str("take"), py_str("takes"), py_str("taking")])])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("swim"), py_str("mile"), py_str("kilometre"), py_str("km")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("A steady front crawl does a mile - 64 lengths of a 25m pool - in about 30 to 40 minutes. Breaststroke, or a first season of swimming, is 45 minutes to an hour, and building up 200m at a time gets there better than trying the mile straight away.")), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("get-over-a-breakup-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("get over a breakup"), py_str("get over a break up"), py_str("over a breakup"), py_str("get over my ex"), py_str("get over him"), py_str("get over her"), py_str("broken heart"), py_str("breakup"), py_str("break up"), py_str("end of a relationship")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("breakup"), py_str("break up"), py_str("break-up"), py_str("ex"), py_str("relationship"), py_str("broken heart")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("The sharpest part is usually six to eight weeks, and most people feel like themselves again in three to six months. It does not go in a straight line - a bad day at three months is not a relapse - and seeing people on the days you least want to is what makes the weeks go faster.")), ("avoid".to_string(), py_tuple(vec![py_str("bereav"), py_str("died"), py_str("passed away"), py_str("funeral"), py_str("grief"), py_str("widow"), py_str("lost my")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-hair-mask-takes")); __kws.extend(vec![("key".to_string(), py_tuple(vec![py_str("leave a hair mask on"), py_str("leave a mask on"), py_str("leave the mask on"), py_str("leave a hair mask"), py_str("hair mask on"), py_str("hair masks on"), py_str("hair mask for"), py_str("hair mask in"), py_str("deep conditioning mask")])), ("ask".to_string(), g__ASK()), ("guard".to_string(), _typed(&[py_tuple(vec![py_str("mask")])])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), py_str("10 to 20 minutes is all a hair mask needs, and the label's own time is the one to follow. Longer adds nothing - hair can only take so much - and a warm towel wrapped round it does more than another ten minutes.")), ("avoid".to_string(), py_tuple(vec![py_str("last"), py_str("lasts"), py_str("expire"), py_str("shelf life"), py_str("go off"), py_str("gone off"), py_str("keep")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    s_ENTRIES(__data1());
    let __iter5 = py_iter_or_raise(&g__COAT_FINISHES());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 3, None);
    s__finish(__unpk6[0].clone());
    s__words(__unpk6[1].clone());
    s__said(__unpk6[2].clone());
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("between-coats-of-"), &g__finish())); __kws.extend(vec![("key".to_string(), py_tuple(vec![g__words(), py_tuple(vec![py_str("between coats"), py_str("coats"), py_str("coat"), py_str("second coat"), py_str("next coat"), py_str("recoat")])])), ("ask".to_string(), g__COAT_ASK()), ("guard".to_string(), _typed(&[g__words()])), ("shape".to_string(), py_tuple(vec![py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))])), ("reply".to_string(), g__said()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    }
}
pub fn _typed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut flat: Py = Py::None;
    let mut guard: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A guard: the row's subject has to be in the line the person typed.\n\n    The reading carries a subject over from an earlier turn, which is right for a follow-up\n    (\"how long does it take\" after \"i took a chicken out the freezer\") and wrong here: after\n    \"i'm saving for a deposit\" and \"what have i missed\", the line \"how long will that take\" is\n    rebuilt as \"how long will the deposit son take\", and the carried-over \"deposit\" handed it to\n    a deposit row. A guard reads the line itself, so a row of this file answers only a line that\n    names its subject.\n    ");
    flat = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&flat.clone(), "extend", &[word.clone()]);
    } else {
    let _ = mcall(&flat.clone(), "append", &[word.clone()]);
    }
    }
pub fn __nested_guard_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut flat: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut guard: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut said: Py = Py::None;
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&flat.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    guard = {{ let flat = flat.clone(); let guard = guard.clone(); let word = word.clone(); let words = words.clone(); py_func("guard", move |args: &[Py]| { let mut full = vec![flat.clone(), guard.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_guard_1(&full) }) }};
    return guard.clone();
    Py::None
}
pub fn _asks_a_length(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("Whether the line asks a length of time, as one signal for the score as well as the gate.\n\n    The ask is a hard gate on its own, but a row whose subject is one word (\"mot\") holds a\n    single position of the line and the matcher drops that before the gate is read. A shape is\n    a second signal in its own right, so the row is read, judged, and put against the card it\n    was losing to.\n    ");
    low = mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASK()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &part.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.takes".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASK" => Some(g__ASK()),
            "_A_FINISH_NAMED" => Some(g__A_FINISH_NAMED()),
            "_COAT_ASK" => Some(g__COAT_ASK()),
            "_COAT_FINISHES" => Some(g__COAT_FINISHES()),
            "_asks_a_length" => Some(py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args))),
            "_entry" => Some(g__entry()),
            "_finish" => Some(g__finish()),
            "_said" => Some(g__said()),
            "_typed" => Some(py_func("_typed", |args: &[Py]| _typed(args))),
            "_words" => Some(g__words()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASK" => s__ASK(v),
            "_A_FINISH_NAMED" => s__A_FINISH_NAMED(v),
            "_COAT_ASK" => s__COAT_ASK(v),
            "_COAT_FINISHES" => s__COAT_FINISHES(v),
            "_entry" => s__entry(v),
            "_finish" => s__finish(v),
            "_said" => s__said(v),
            "_words" => s__words(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.takes", module_obj);
    pyrt::xrefs::reg_fn("knowledge.takes", "_typed", py_func("_typed", |args: &[Py]| _typed(args)));
    pyrt::xrefs::reg_fn("knowledge.takes", "_asks_a_length", py_func("_asks_a_length", |args: &[Py]| _asks_a_length(args)));
    pyrt::xrefs::reg_access("knowledge.takes", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.takes", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.takes", "_ASK", g__ASK, s__ASK);
    pyrt::xrefs::reg_access("knowledge.takes", "_A_FINISH_NAMED", g__A_FINISH_NAMED, s__A_FINISH_NAMED);
    pyrt::xrefs::reg_access("knowledge.takes", "_COAT_ASK", g__COAT_ASK, s__COAT_ASK);
    pyrt::xrefs::reg_access("knowledge.takes", "_COAT_FINISHES", g__COAT_FINISHES, s__COAT_FINISHES);
    pyrt::xrefs::reg_access("knowledge.takes", "_entry", g__entry, s__entry);
    pyrt::xrefs::reg_access("knowledge.takes", "_finish", g__finish, s__finish);
    pyrt::xrefs::reg_access("knowledge.takes", "_said", g__said, s__said);
    pyrt::xrefs::reg_access("knowledge.takes", "_words", g__words, s__words);
}
