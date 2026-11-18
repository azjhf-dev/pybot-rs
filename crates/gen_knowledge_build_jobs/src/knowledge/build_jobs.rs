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
thread_local! { static G__A_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TOOLS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BUILD_JOBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BUILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.build_jobs: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_ASK() -> Py { ensure_init(); G__A_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_ASK(v: Py) { ensure_init(); G__A_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TOOLS_ASK() -> Py { ensure_init(); G__A_TOOLS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TOOLS_ASK(v: Py) { ensure_init(); G__A_TOOLS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__BUILD_JOBS() -> Py { ensure_init(); G__BUILD_JOBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BUILD_JOBS(v: Py) { ensure_init(); G__BUILD_JOBS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BUILD() -> Py { ensure_init(); G__NOT_A_BUILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BUILD(v: Py) { ensure_init(); G__NOT_A_BUILD.with(|c| *c.borrow_mut() = v); }
pub fn g__entry() -> Py { ensure_init(); G__entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__entry(v: Py) { ensure_init(); G__entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing being put up: how long it takes, and what it needs.\n\nThe answers lane owns this domain. Two questions come before anybody builds a shed, a\nplayhouse or a flat-pack wardrobe - how long will it take, and what will we need - and the\nbot answered neither: with a shed on order, \"how long does one take to put up\" and \"what\ntools will we need\" were both search hedges (report 137542), while a repair has its own\nclock (\"how long to fix a dishwasher\") and a shelf its own fixings.\n\nWhy its own domain: a job being put up is not a fault and not a coat of something. The\ntreating, repairing and lifespan rows all key the same nouns (shed, deck, fence panel) with\nthe same ask (\"how long\"), so a table of figures sitting among them is read against the\nwrong question. Here the job is the subject and the ask picks which half of the card is\nowed, so one table answers a shed, a bike store, a gazebo and a chest of drawers, and a new\njob is one line of data rather than a new row.\n");
    s__entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    fn __data1() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("bike shed"), py_str("bike store"), py_str("bike storage")]), py_str("An hour or two for two of you, and the base is the whole job."), py_str("A drill with masonry bits, a spanner, a spirit level and a tape measure; the fixings to the wall come with it and the ground anchor does not."), py_str("It has to be bolted down as well as built, or the first gale takes the lid and the frame with it.")]), py_tuple(vec![py_tuple(vec![py_str("summerhouse"), py_str("summer house"), py_str("log cabin")]), py_str("Two of you: two days for a cabin-style one, and the base and the roof are a day each."), py_str("A drill and driver, a spirit level, a stepladder, clamps, a saw and a mallet; the windows and the roofing felt come with it."), py_str("Level the base to within a few millimetres and let the timber settle for a week before you hang the doors, or they will bind by the second month.")]), py_tuple(vec![py_tuple(vec![py_str("gazebo"), py_str("gazebos"), py_str("pop up gazebo")]), py_str("Twenty minutes for two of you with a pop-up one; a wooden one is a day."), py_str("Nothing for a pop-up beyond its own frame and pegs; a wooden one wants a drill, a spirit level and a mallet."), py_str("Peg every leg and add the guy ropes, even on a still day, because a pop-up gazebo is a kite with a roof.")]), py_tuple(vec![py_tuple(vec![py_str("climbing frame"), py_str("climbing frames"), py_str("swing set"), py_str("swingset"), py_str("wooden swing")]), py_str("A day for two of you, and digging the leg holes is half of it."), py_str("A post hole digger, a spirit level, a drill and driver, a tape measure, a saw and a socket set."), py_str("Set the legs in concrete and check the level before it goes off, then tighten every bolt again after a fortnight of use.")]), py_tuple(vec![py_tuple(vec![py_str("raised bed"), py_str("raised beds")]), py_str("An afternoon for two of you, and filling it takes longer than building it."), py_str("A saw, a drill and driver, a spirit level, a tape measure and a staple gun for the liner."), py_str("Line the inside with a membrane if the timber is treated, and leave the boards with a gap at the bottom for water.")]), py_tuple(vec![py_tuple(vec![py_str("shed"), py_str("sheds")]), py_str("Two of you: a day for a shed up to 8x6, and the base is a day of its own."), py_str("A drill with wood and masonry bits, a tape measure, a spirit level, a stepladder, a mallet, a knife for the felt and a screwdriver; the screws come with it and the base does not."), py_str("Get the base flat and level first, on slabs or a timber frame, because every panel follows it - and paint or treat the bare timber before the walls close up.")]), py_tuple(vec![py_tuple(vec![py_str("playhouse"), py_str("play house"), py_str("wendy house"), py_str("cubby house")]), py_str("Two of you: two to three hours, and a plastic one that clicks together is less."), py_str("A screwdriver, a mallet, a spirit level and a tape measure; a wooden one comes with its screws but rarely with a base."), py_str("Stand it on something flat, a paved square or a timber frame, or the door will not line up and the walls will not meet at the corners.")]), py_tuple(vec![py_tuple(vec![py_str("greenhouse"), py_str("green house")]), py_str("Two of you: a weekend for an aluminium 6x8, and a day of that is getting the base right."), py_str("A drill with masonry bits, a spirit level, a stepladder, a tape measure, clamps and silicone; the glazing clips and the glass come with the frame."), py_str("The base has to be square and level to within a few millimetres, because the glass is cut to fit and will not go in otherwise.")]), py_tuple(vec![py_tuple(vec![py_str("fence panel"), py_str("fence panels"), py_str("feather edge"), py_str("closeboard")]), py_str("About an hour a panel for two of you once the posts are in, and the posts want a day to set first."), py_str("A post hole spade, a spirit level, a drill and driver, a saw and a string line; a panel is light but it is a sail, so this is a two-person job in any wind."), py_str("Fix the panels to the posts, not the other way round, and brace the end posts until the concrete has gone off.")]), py_tuple(vec![py_tuple(vec![py_str("decking"), py_str("deck boards"), py_str("deck board"), py_str("raised deck")]), py_str("A 3x3 metre deck is two or three weekends for one person, and the frame is most of it."), py_str("A saw, a drill and driver, a string line, a spirit level, a set square, clamps and stainless screws."), py_str("Level the frame and set the joists at 400mm centres before a board goes down, and leave a gap of about 5mm between boards so the water runs off.")]), py_tuple(vec![py_tuple(vec![py_str("wardrobe"), py_str("wardrobes"), py_str("flat pack"), py_str("flat-pack"), py_str("flatpack"), py_str("chest of drawers"), py_str("bed frame"), py_str("bookcase"), py_str("sideboard")]), py_str("Two of you: two to four hours, and the doors and drawers are half of it."), py_str("A Pozidriv screwdriver, a mallet, a tape measure and the Allen key that comes in the box; a cordless driver on its lowest clutch saves your wrists."), py_str("Build it on its side on a rug or a flattened box, and fix anything over a metre tall to the wall, because a chest of drawers on an open drawer will go over.")]), py_tuple(vec![py_tuple(vec![py_str("kitchen unit"), py_str("kitchen units"), py_str("base units"), py_str("wall units"), py_str("kitchen cabinets"), py_str("fit a kitchen"), py_str("fitting a kitchen"), py_str("kitchen fitting"), py_str("kitchen install"), py_str("installing a kitchen")]), py_str("A run of five units is a day if you have done it before and two or three the first time, and the worktop is a day of its own."), py_str("A spirit level, clamps, a drill and driver, a jigsaw, a tape measure and packers."), py_str("Set the first unit dead level and square to the wall and clamp the next to it: everything after that one follows it, so ten minutes spent there saves the whole run.")]), py_tuple(vec![py_tuple(vec![py_str("trampoline"), py_str("trampolines")]), py_str("Two of you: an hour or two, and the springs are most of it."), py_str("Two spanners, which are usually in the box, gloves, and the spring puller that comes with it."), py_str("Put it on flat grass with the headroom the manual gives, fit the net before the first bounce, and peg the legs down.")])]) }
    s__BUILD_JOBS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("how long"), py_str("how many hours"), py_str("how many days"), py_str("how much time"), py_str("time to"), py_str("how many people"), py_str("how many of us"), py_str("hours to"), py_str("days to"), py_str("a weekend"), py_str("an afternoon"), py_str("a day to"), py_str("long to put up"), py_str("long to build"), py_str("long to assemble")]) }
    s__A_TIME_ASK(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("what tools"), py_str("which tools"), py_str("tool"), py_str("tools"), py_str("tool kit"), py_str("toolkit"), py_str("equipment"), py_str("what do i need"), py_str("what will i need"), py_str("what do we need"), py_str("what will we need"), py_str("what else do i need"), py_str("what else will i need"), py_str("shopping list"), py_str("what to buy"), py_str("what bits")]) }
    s__A_TOOLS_ASK(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("last"), py_str("lasts"), py_str("lasting"), py_str("lifespan"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("cheap"), py_str("cheaper"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("which one"), py_str("best"), py_str("worth"), py_str("compare"), py_str("treat"), py_str("treating"), py_str("treats"), py_str("stain"), py_str("staining"), py_str("paint"), py_str("painting"), py_str("creosote"), py_str("preserver"), py_str("rot"), py_str("rotten"), py_str("repair"), py_str("repairs"), py_str("fix"), py_str("fixing"), py_str("broken"), py_str("blown"), py_str("leaking"), py_str("mould"), py_str("damp"), py_str("concrete"), py_str("ballast"), py_str("cement"), py_str("slabs"), py_str("tomato"), py_str("tomatoes"), py_str("cucumber"), py_str("chilli"), py_str("chillies"), py_str("pepper"), py_str("plants"), py_str("plant"), py_str("seeds"), py_str("sow"), py_str("sowing"), py_str("grow"), py_str("growing"), py_str("crop"), py_str("crops"), py_str("remove"), py_str("take down"), py_str("dispose"), py_str("skip")]) }
    s__NOT_A_BUILD(__data4());
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-job-being-put-up")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BUILD_JOBS()) { let __unpk136 = py_unpack(&__item.clone(), 4, None); let mut names: Py = __unpk136[0].clone(); let mut _time: Py = __unpk136[1].clone(); let mut _tools: Py = __unpk136[2].clone(); let mut _caveat: Py = __unpk136[3].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), op_add(&g__A_TIME_ASK(), &g__A_TOOLS_ASK())])); __args.push(py_func("_a_build_job", |args: &[Py]| _a_build_job(args))); __kws.extend(vec![("guard".to_string(), py_func("_a_job_being_put_up", |args: &[Py]| _a_job_being_put_up(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _a_build_job(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut caveat: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut job: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut time: Py = Py::None;
    let mut tools: Py = Py::None;
    let mut wants_time: Py = Py::None;
    let mut wants_tools: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How long the job takes, or what it needs, for the job the line names.");
    line = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    words = call_builtin("set", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    job = Py::None;
    let __iter1 = py_iter_or_raise(&g__BUILD_JOBS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    names = __unpk2[0].clone();
    time = __unpk2[1].clone();
    tools = __unpk2[2].clone();
    caveat = __unpk2[3].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&words.clone(), &name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &name.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    job = py_tuple(vec![time.clone(), tools.clone(), caveat.clone()]);
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&job.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&job.clone(), 3, None);
    time = __unpk3[0].clone();
    tools = __unpk3[1].clone();
    caveat = __unpk3[2].clone();
    wants_time = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_TIME_ASK()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    wants_tools = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_TOOLS_ASK()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let __unpk4 = py_unpack(&if py_truthy(&{ let __b = wants_tools.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wants_time.clone())) } }) { py_tuple(vec![tools.clone(), time.clone()]) } else { py_tuple(vec![time.clone(), tools.clone()]) }, 2, None);
    chosen = __unpk4[0].clone();
    spare = __unpk4[1].clone();
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("last_answer")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&last.clone(), &mcall(&_opens_with(&[chosen.clone()]), "lower", &[]))) } }) {
    if py_truthy(&Py::Bool(py_contains(&last.clone(), &mcall(&_opens_with(&[spare.clone()]), "lower", &[])))) {
    return Py::None;
    }
    chosen = spare.clone();
    }
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![chosen.clone(), caveat.clone()]));
    Py::None
}
pub fn _opens_with(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let _ = py_str("The first clause of a card sentence, which is what says the same thing twice.");
    return mcall(&geti(&mcall(&geti(&mcall(&sentence.clone(), "split", &[py_str(",")]), &Py::Int(0)), "split", &[py_str(" - ")]), &Py::Int(0)), "strip", &[]);
    Py::None
}
pub fn _a_job_being_put_up(args: &[Py]) -> Py {
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
    let mut _c: Py = Py::None;
    let mut _l: Py = Py::None;
    let mut _t: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut one: Py = Py::None;
    let mut stop: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True for a line about putting a thing up: it names a job and asks how long or what with.");
    text = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_BUILD()) { let mut stop: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &stop.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g__A_TIME_ASK(), &g__A_TOOLS_ASK())) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BUILD_JOBS()) { let __unpk7159 = py_unpack(&__item.clone(), 4, None); let mut names: Py = __unpk7159[0].clone(); let mut _t: Py = __unpk7159[1].clone(); let mut _l: Py = __unpk7159[2].clone(); let mut _c: Py = __unpk7159[3].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &name.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.build_jobs".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_TIME_ASK" => Some(g__A_TIME_ASK()),
            "_A_TOOLS_ASK" => Some(g__A_TOOLS_ASK()),
            "_BUILD_JOBS" => Some(g__BUILD_JOBS()),
            "_NOT_A_BUILD" => Some(g__NOT_A_BUILD()),
            "_a_build_job" => Some(py_func("_a_build_job", |args: &[Py]| _a_build_job(args))),
            "_a_job_being_put_up" => Some(py_func("_a_job_being_put_up", |args: &[Py]| _a_job_being_put_up(args))),
            "_entry" => Some(g__entry()),
            "_opens_with" => Some(py_func("_opens_with", |args: &[Py]| _opens_with(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_A_TIME_ASK" => s__A_TIME_ASK(v),
            "_A_TOOLS_ASK" => s__A_TOOLS_ASK(v),
            "_BUILD_JOBS" => s__BUILD_JOBS(v),
            "_NOT_A_BUILD" => s__NOT_A_BUILD(v),
            "_entry" => s__entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.build_jobs", module_obj);
    pyrt::xrefs::reg_fn("knowledge.build_jobs", "_a_build_job", py_func("_a_build_job", |args: &[Py]| _a_build_job(args)));
    pyrt::xrefs::reg_fn("knowledge.build_jobs", "_opens_with", py_func("_opens_with", |args: &[Py]| _opens_with(args)));
    pyrt::xrefs::reg_fn("knowledge.build_jobs", "_a_job_being_put_up", py_func("_a_job_being_put_up", |args: &[Py]| _a_job_being_put_up(args)));
    pyrt::xrefs::reg_access("knowledge.build_jobs", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.build_jobs", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.build_jobs", "_A_TIME_ASK", g__A_TIME_ASK, s__A_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.build_jobs", "_A_TOOLS_ASK", g__A_TOOLS_ASK, s__A_TOOLS_ASK);
    pyrt::xrefs::reg_access("knowledge.build_jobs", "_BUILD_JOBS", g__BUILD_JOBS, s__BUILD_JOBS);
    pyrt::xrefs::reg_access("knowledge.build_jobs", "_NOT_A_BUILD", g__NOT_A_BUILD, s__NOT_A_BUILD);
    pyrt::xrefs::reg_access("knowledge.build_jobs", "_entry", g__entry, s__entry);
}
