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
thread_local! { static G_ASK_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_EMERGENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_REQUEST_TO_EXPLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHAT_HAPPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASK_ABOUT() -> Py { ensure_init(); G_ASK_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_ABOUT(v: Py) { ensure_init(); G_ASK_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_EMERGENCY() -> Py { ensure_init(); G__NOT_AN_EMERGENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_EMERGENCY(v: Py) { ensure_init(); G__NOT_AN_EMERGENCY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PERSON() -> Py { ensure_init(); G__NOT_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PERSON(v: Py) { ensure_init(); G__NOT_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_REQUEST_TO_EXPLAIN() -> Py { ensure_init(); G__NOT_A_REQUEST_TO_EXPLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_REQUEST_TO_EXPLAIN(v: Py) { ensure_init(); G__NOT_A_REQUEST_TO_EXPLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__WHAT_HAPPENS() -> Py { ensure_init(); G__WHAT_HAPPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHAT_HAPPENS(v: Py) { ensure_init(); G__WHAT_HAPPENS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The procedures people are sent for, asked about by name.\n\nThe knowledge base holds conditions (conditions.py), symptoms (body.py), the scans (health.py)\nand what to do before an appointment (appointments.py), but nothing held the procedure itself:\n\"what is a colonoscopy\", \"what happens during a vasectomy\", \"what is a root canal\" were all\nanswered \"That's beyond what I know well, sorry\" or \"I'm not sure about that one\". A person who\nhas been handed a date for one of these wants four things, and the card is built in that order:\nwhat it is, what happens on the day, how long it takes, and what the recovery is.\n\nSo the subject here is the name of the procedure and the second half is how the line asks about\nit: the explain shapes (\"what is\", \"what happens during\", \"how does it work\"), the length\nshapes (\"how long does it take\", \"how long in hospital\"), the pain shapes (\"does it hurt\", \"will\ni be awake\"), and the recovery shapes (\"how long off work\", \"how long to recover\"). `explain=True`\nlets the bare name reach the card, and `report=True` lets a person say they are having one (\"i\nhave a colonoscopy on friday\") and hear the answer rather than \"Got it\".\n\nWhat these rows deliberately do not take: the preparation question, which is appointments.py\n(\"do i need to fast before\", \"what should i bring\"), a scan, which is health.py (\"what is an mri\nscan\"), and a named condition, which is conditions.py (\"what is endometriosis\"). A line about\npain, bleeding or a sudden turn for the worse is nobody's card here.\n");
    s_ASK_ABOUT(crate::knowledge::lookup::g_ASK_ABOUT());
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    s__WHAT_HAPPENS(op_add(&crate::knowledge::lookup::g_ASK_ABOUT(), &py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what are"), py_str("what happens"), py_str("what goes on"), py_str("happens during"), py_str("happens in"), py_str("happens on the day"), py_str("goes on during"), py_str("during"), py_str("involve"), py_str("involves"), py_str("explain"), py_str("tell me about"), py_str("how does it work"), py_str("how does that work"), py_str("how it works"), py_str("what do they do"), py_str("what will they do"), py_str("what are they doing"), py_str("what is it like"), py_str("does it hurt"), py_str("hurt"), py_str("painful"), py_str("pain"), py_str("awake"), py_str("sedated"), py_str("sedation"), py_str("asleep"), py_str("general anaesthetic"), py_str("anaesthetic"), py_str("anesthetic"), py_str("recover"), py_str("recovery"), py_str("recovering"), py_str("afterwards"), py_str("after it"), py_str("after the"), py_str("how long"), py_str("take"), py_str("takes"), py_str("in hospital"), py_str("off work"), py_str("back to work"), py_str("drive"), py_str("driving"), py_str("sore"), py_str("how many"), py_str("check up")])));
    s__NOT_A_REQUEST_TO_EXPLAIN(py_tuple(vec![py_str("should i"), py_str("should we"), py_str("shall i"), py_str("do i need"), py_str("do i have to"), py_str("have to"), py_str("worth it"), py_str("worth having"), py_str("instead"), py_str("or not"), py_str("better than"), py_str("compare"), py_str("rather than"), py_str("how much"), py_str("cost"), py_str("costs"), py_str("price"), py_str("private"), py_str("nhs"), py_str("waiting list"), py_str("how long is the wait"), py_str("cancel"), py_str("cancelled"), py_str("reschedule"), py_str("missed"), py_str("letter"), py_str("results"), py_str("my results")]));
    s__NOT_AN_EMERGENCY(py_tuple(vec![py_str("chest pain"), py_str("cant breathe"), py_str("can't breathe"), py_str("cannot breathe"), py_str("unconscious"), py_str("not breathing"), py_str("collapsed"), py_str("collapsed at"), py_str("a&e"), py_str("999"), py_str("ambulance"), py_str("bleeding badly"), py_str("heavy bleeding"), py_str("suicidal"), py_str("overdose"), py_str("took an overdose")]));
    s__NOT_A_PERSON(py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("hamster"), py_str("rabbit"), py_str("horse"), py_str("guinea pig"), py_str("plant"), py_str("plants"), py_str("car"), py_str("cars"), py_str("boiler"), py_str("washing machine"), py_str("phone")]));
    s_ENTRIES(py_list(vec![]));
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("colonoscopy"), py_tuple(vec![py_str("colonoscopy"), py_str("colonoscopies"), py_str("camera up my back passage"), py_str("bowel camera")]), py_str("A colonoscopy is a camera on a flexible tube put up through the back passage to look all the way round the bowel, and it takes about 20 to 30 minutes. You are given sedation and pain relief through a cannula, so most people are drowsy and remember little of it, and you go home the same day once you have eaten and been to the toilet - but not driving, so bring someone. Wind and a bloated ache for a day are the usual aftermath, and the bowel prep the day before matters more than anything else: the cleaner the bowel, the more they can see.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("endoscopy"), py_tuple(vec![py_str("endoscopy"), py_str("endoscopies"), py_str("gastroscopy"), py_str("camera down my throat"), py_str("camera in my stomach")]), py_str("An endoscopy is a thin camera on a tube passed down your throat to look at the food pipe, the stomach and the top of the bowel, and it takes 10 to 20 minutes. Your throat is numbed with a spray and you are usually offered sedation, so the gagging is the worst of it rather than pain; you go home the same day, but not driving if you took the sedation. A sore throat and a bit of wind for a day are normal, and you will be asked not to eat for about six hours before.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("sigmoidoscopy"), py_tuple(vec![py_str("sigmoidoscopy"), py_str("flexible sigmoidoscopy"), py_str("sigmoidoscopies")]), py_str("A flexible sigmoidoscopy is a camera on a tube into the lower bowel only, about the last half metre, and it takes 10 to 15 minutes - less of a journey than a full colonoscopy. Most people have it without sedation, lying on their side, with some cramping and wind as the tube goes round the bends; you go home straight afterwards and can drive. The bowel prep is usually an enema at the hospital rather than a day of drinking the stuff.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("cystoscopy"), py_tuple(vec![py_str("cystoscopy"), py_str("cystoscopies"), py_str("camera in my bladder"), py_str("camera up my urethra")]), py_str("A cystoscopy is a thin camera passed up the water pipe into the bladder, and it takes about 5 to 10 minutes. It is done with a numbing gel, so it is uncomfortable rather than painful - a sharp sting as it goes in and a strong urge to pass water while it is there - and you can drive yourself home. Expect stinging and pinkish urine for a day or two and drink plenty of water to flush it through; ring the ward if it is bright red or you get a fever.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("angiogram"), py_tuple(vec![py_str("angiogram"), py_str("angiograms"), py_str("cardiac catheter"), py_str("catheter test")]), py_str("An angiogram is a dye test that shows the arteries: a fine tube goes into the wrist or the groin under local anaesthetic, dye is injected, and X-rays follow it round. It takes 20 to 40 minutes and does not hurt beyond the sting of the numbing injection. You rest with pressure on the puncture for a few hours, go home the same day in most cases, and should keep the arm or leg easy for 24 hours - no lifting, no driving that day.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("angioplasty"), py_tuple(vec![py_str("angioplasty"), py_str("stent"), py_str("stents"), py_str("a stent put in"), py_str("balloon"), py_str("stent fitted")]), py_str("An angioplasty is the same tube as an angiogram, with a tiny balloon on the end that is blown up to squash the narrowed artery open, and usually a mesh stent left in to hold it there. It takes 30 to 60 minutes, you are awake with numbing at the wrist or groin, and you are usually in for the day or one night. The stent is permanent and grows a layer of the artery's own lining over itself, so it is not something that needs changing.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("lumbar-puncture"), py_tuple(vec![py_str("lumbar puncture"), py_str("spinal tap"), py_str("lumbar punctures")]), py_str("A lumbar puncture is a needle into the lower back to draw off a sample of the fluid around the spinal cord, and it takes about 15 minutes. The skin is numbed first, so the worst is a push and a strange tingling down a leg; you lie flat for an hour or so afterwards and go home the same day. Drink plenty and keep still for the rest of the day - the headache that follows is the commonest problem, and lying flat is what settles it.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("blood-transfusion"), py_tuple(vec![py_str("blood transfusion"), py_str("transfusion"), py_str("transfusions"), py_str("units of blood"), py_str("pint of blood")]), py_str("A blood transfusion is donated blood run into a vein through a cannula, one unit at a time, each taking about two to four hours. It is not an operation: you sit or lie while it drips, and the nurse checks your pulse, blood pressure and temperature before each unit and again at the start of it. Most people have it as a day patient, feel warmer and less breathless within a day, and the main thing to report is a rash, shivering or a sudden fever.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("dialysis"), py_tuple(vec![py_str("dialysis"), py_str("haemodialysis"), py_str("hemodialysis"), py_str("kidney dialysis")]), py_str("Dialysis does the filtering job the kidneys cannot: your blood is taken out, run past a filter and put back, either in sessions at a unit - usually three a week, four hours each - or at home overnight. It is not painful, and people read, sleep or watch something through it; the needle going in at the start of each session is the only sharp bit, and a fistula in the arm is looked after carefully for that reason. Between sessions the work is fluids, salt and potassium, and the team sets the limits.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("radiotherapy"), py_tuple(vec![py_str("radiotherapy"), py_str("radiation therapy"), py_str("radiotherapy treatment"), py_str("radiotherapy sessions")]), py_str("Radiotherapy is a beam of X-rays aimed at one small area to damage the cells growing out of control, and each session is short - about 10 to 20 minutes, with the treatment itself only a few minutes. It does not hurt and does not make you radioactive; you lie still on a couch under the machine, usually every weekday for a few weeks, and the planning session before the first one takes longer than the treatment. Tiredness and sore skin over the area build up over the course, and your team gives you the cream and the rules.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("chemotherapy"), py_tuple(vec![py_str("chemotherapy"), py_str("chemo"), py_str("chemotherapy treatment"), py_str("chemo sessions")]), py_str("Chemotherapy is cancer drugs that travel through the whole body, given as a drip, as tablets or both, in cycles of a treatment day followed by a few weeks off so the body recovers. The drip itself does not hurt and takes an hour or a few depending on the drugs; a cannula or a line goes in first. The side effects that matter are the ones your team gives you drugs for in advance - sickness, a sore mouth, and above all an infection risk in the week after, so a temperature over 37.5C or feeling shivery is a phone call to the unit the same day, not a wait.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("hysterectomy"), py_tuple(vec![py_str("hysterectomy"), py_str("hysterectomies"), py_str("womb removed"), py_str("womb taken out")]), py_str("A hysterectomy is the womb taken out, either through the tummy, through the vagina or by keyhole, and the operation takes about an hour to an hour and a half. You are asleep for it and in hospital for one to four nights depending on the route. Recovery is the long part: two weeks of taking it very easy, six weeks before lifting anything heavier than a kettle or driving on the road, and no sex or swimming for about six weeks. If the ovaries come out too, that is the menopause starting, and it can be managed.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("keyhole-surgery"), py_tuple(vec![py_str("keyhole surgery"), py_str("laparoscopy"), py_str("laparoscopic surgery"), py_str("keyhole operation"), py_str("keyhole")]), py_str("Keyhole surgery is an operation done through two or three small cuts rather than one long one: a slim camera goes in through the belly button, the surgeon works off the screen, and the cuts are each a centimetre or so, closed with a stitch or glue. The operation takes about as long as the open version but the recovery is much shorter - often home the same day or the next, back to light normal in a week. The gas they use to make room is what leaves you with a bloated ache in the shoulders for a day or two.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("vasectomy"), py_tuple(vec![py_str("vasectomy"), py_str("vasectomies"), py_str("the snip"), py_str("snipped")]), py_str("A vasectomy is the two tubes that carry sperm from the testicles cut and sealed, so the semen has no sperm in it; it takes about 15 to 30 minutes under local anaesthetic. You are awake, the worst is the numbing injection, and you go home the same day in loose trousers with a bag of frozen peas. Two days of taking it easy, a week before anything strenuous, and it is not instant: you need a sample checked at about 12 weeks and another clear one before you rely on it, so use something else until then.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("root-canal"), py_tuple(vec![py_str("root canal"), py_str("root canal treatment"), py_str("root canals"), py_str("root canal work")]), py_str("A root canal is a tooth saved rather than taken out: the dentist drills in from the top, cleans the infected nerve out of the canals inside the root, and seals them. It takes one or two visits of 45 minutes to an hour and a half, and with modern numbing it feels much like a filling - the pain comes from the infection before the visit, not from the drilling. The tooth is brittle afterwards, so a crown on a back tooth is the usual last step, and some soreness for two or three days is normal.")])]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("knee-replacement")); __args.push(py_tuple(vec![py_str("knee replacement"), py_str("knee replacements"), py_str("new knee"), py_str("knee op")])); __args.push(py_str("A knee replacement takes the worn surfaces off the end of the thigh bone and the top of the shin bone and puts a metal and plastic joint in their place; the operation takes about an hour to 90 minutes under a spinal or general anaesthetic. Most people are up on it the day after with a frame, in for two to four nights, and the physio starts straight away because the bending is what needs the work. Expect swelling and a stiff, warm knee for weeks, walking sticks for about six weeks, and up to a year before it feels like your own.")); __kws.extend(vec![("extra_avoid".to_string(), py_tuple(vec![py_str("recover"), py_str("recovery"), py_str("recovering"), py_str("off work"), py_str("back to work"), py_str("drive"), py_str("driving"), py_str("how long am i off"), py_str("sick note")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_row(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[_a_row(&[py_str("mammogram"), py_tuple(vec![py_str("mammogram"), py_str("mammograms"), py_str("breast screening"), py_str("breast x-ray"), py_str("breast xray")]), py_str("A mammogram is an X-ray of each breast, taken by pressing it between two plates for a few seconds at a time, four pictures in all, and the whole appointment is about 20 minutes. It is uncomfortable rather than painful - a firm squeeze that stops as soon as the picture is taken - and you can drive yourself home and go back to work. The screening programme calls women every three years from 50 to 70; a recall for another look is common and mostly not cancer, because screening is meant to find the things too small to feel.")])]);
}
pub fn _a_row(args: &[Py]) -> Py {
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
    let mut variants: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "variants") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: variants"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut extra_avoid: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "extra_avoid") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); __args.push(py_tuple(vec![variants.clone(), g__WHAT_HAPPENS()])); __args.push(reply.clone()); __kws.extend(vec![("ask".to_string(), variants.clone()), ("avoid".to_string(), op_add(&op_add(&op_add(&g__NOT_A_REQUEST_TO_EXPLAIN(), &g__NOT_AN_EMERGENCY()), &g__NOT_A_PERSON()), &extra_avoid.clone())), ("explain".to_string(), Py::Bool(true)), ("report".to_string(), Py::Bool(true)), ("bias".to_string(), bias.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.procedures".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK_ABOUT" => Some(g_ASK_ABOUT()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_NOT_AN_EMERGENCY" => Some(g__NOT_AN_EMERGENCY()),
            "_NOT_A_PERSON" => Some(g__NOT_A_PERSON()),
            "_NOT_A_REQUEST_TO_EXPLAIN" => Some(g__NOT_A_REQUEST_TO_EXPLAIN()),
            "_WHAT_HAPPENS" => Some(g__WHAT_HAPPENS()),
            "_a_row" => Some(py_func("_a_row", |args: &[Py]| _a_row(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASK_ABOUT" => s_ASK_ABOUT(v),
            "ENTRIES" => s_ENTRIES(v),
            "_NOT_AN_EMERGENCY" => s__NOT_AN_EMERGENCY(v),
            "_NOT_A_PERSON" => s__NOT_A_PERSON(v),
            "_NOT_A_REQUEST_TO_EXPLAIN" => s__NOT_A_REQUEST_TO_EXPLAIN(v),
            "_WHAT_HAPPENS" => s__WHAT_HAPPENS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.procedures", module_obj);
    pyrt::xrefs::reg_fn("knowledge.procedures", "_a_row", py_func("_a_row", |args: &[Py]| _a_row(args)));
    pyrt::xrefs::reg_access("knowledge.procedures", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.procedures", "ASK_ABOUT", g_ASK_ABOUT, s_ASK_ABOUT);
    pyrt::xrefs::reg_access("knowledge.procedures", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.procedures", "_NOT_AN_EMERGENCY", g__NOT_AN_EMERGENCY, s__NOT_AN_EMERGENCY);
    pyrt::xrefs::reg_access("knowledge.procedures", "_NOT_A_PERSON", g__NOT_A_PERSON, s__NOT_A_PERSON);
    pyrt::xrefs::reg_access("knowledge.procedures", "_NOT_A_REQUEST_TO_EXPLAIN", g__NOT_A_REQUEST_TO_EXPLAIN, s__NOT_A_REQUEST_TO_EXPLAIN);
    pyrt::xrefs::reg_access("knowledge.procedures", "_WHAT_HAPPENS", g__WHAT_HAPPENS, s__WHAT_HAPPENS);
}
