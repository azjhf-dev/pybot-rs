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
thread_local! { static G__APPOINTMENT_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FAST_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIST_TO_CARRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TEST_THAT_TAKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEFORE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_AN_OPERATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_BLOOD_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_SCAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_THE_DENTIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_THE_OPTICIAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDICINE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_ABOUT_GETTING_READY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_MEDICAL_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_FASTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREPARING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREPARING_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STILL_MEDICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.appointments: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__APPOINTMENT_KINDS() -> Py { ensure_init(); G__APPOINTMENT_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__APPOINTMENT_KINDS(v: Py) { ensure_init(); G__APPOINTMENT_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FAST_LENGTH_ASK() -> Py { ensure_init(); G__A_FAST_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FAST_LENGTH_ASK(v: Py) { ensure_init(); G__A_FAST_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIST_TO_CARRY() -> Py { ensure_init(); G__A_LIST_TO_CARRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIST_TO_CARRY(v: Py) { ensure_init(); G__A_LIST_TO_CARRY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TEST_THAT_TAKES() -> Py { ensure_init(); G__A_TEST_THAT_TAKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TEST_THAT_TAKES(v: Py) { ensure_init(); G__A_TEST_THAT_TAKES.with(|c| *c.borrow_mut() = v); }
pub fn g__BEFORE_IT() -> Py { ensure_init(); G__BEFORE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEFORE_IT(v: Py) { ensure_init(); G__BEFORE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_AN_OPERATION() -> Py { ensure_init(); G__ITS_AN_OPERATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_AN_OPERATION(v: Py) { ensure_init(); G__ITS_AN_OPERATION.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_BLOOD_TEST() -> Py { ensure_init(); G__ITS_A_BLOOD_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_BLOOD_TEST(v: Py) { ensure_init(); G__ITS_A_BLOOD_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_SCAN() -> Py { ensure_init(); G__ITS_A_SCAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_SCAN(v: Py) { ensure_init(); G__ITS_A_SCAN.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_THE_DENTIST() -> Py { ensure_init(); G__ITS_THE_DENTIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_THE_DENTIST(v: Py) { ensure_init(); G__ITS_THE_DENTIST.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_THE_OPTICIAN() -> Py { ensure_init(); G__ITS_THE_OPTICIAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_THE_OPTICIAN(v: Py) { ensure_init(); G__ITS_THE_OPTICIAN.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDICINE_NAMED() -> Py { ensure_init(); G__MEDICINE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDICINE_NAMED(v: Py) { ensure_init(); G__MEDICINE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_ABOUT_GETTING_READY() -> Py { ensure_init(); G__NOT_ABOUT_GETTING_READY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_ABOUT_GETTING_READY(v: Py) { ensure_init(); G__NOT_ABOUT_GETTING_READY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_MEDICAL_VISIT() -> Py { ensure_init(); G__NOT_A_MEDICAL_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_MEDICAL_VISIT(v: Py) { ensure_init(); G__NOT_A_MEDICAL_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_FASTING() -> Py { ensure_init(); G__NOT_THE_FASTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_FASTING(v: Py) { ensure_init(); G__NOT_THE_FASTING.with(|c| *c.borrow_mut() = v); }
pub fn g__PREPARING_ASK() -> Py { ensure_init(); G__PREPARING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREPARING_ASK(v: Py) { ensure_init(); G__PREPARING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PREPARING_SUBJECT() -> Py { ensure_init(); G__PREPARING_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREPARING_SUBJECT(v: Py) { ensure_init(); G__PREPARING_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__STILL_MEDICAL() -> Py { ensure_init(); G__STILL_MEDICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STILL_MEDICAL(v: Py) { ensure_init(); G__STILL_MEDICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What to do before an appointment, asked either way round.\n\nA domain of its own because the question is about the visit rather than about the body: the same\n\"do i need to do anything before i go\" wants the dentist's answer after a dentist and the fasting\nrule after a blood test, and it reaches nothing at all if the kind of visit is not part of the\nkey. It answers health, so a follow-up in a health conversation can find it - a domain that does\nnot declare health is never searched with the line before it.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("health"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("appointment"), py_str("appointments"), py_str("appt"), py_str("appts"), py_str("dentist"), py_str("dental"), py_str("hygienist"), py_str("doctor"), py_str("gp"), py_str("surgery"), py_str("health centre"), py_str("health centre"), py_str("clinic"), py_str("hospital"), py_str("consultant"), py_str("specialist"), py_str("physio"), py_str("physiotherapy"), py_str("chiropodist"), py_str("optician"), py_str("eye test"), py_str("check up"), py_str("checkup"), py_str("scan"), py_str("scans"), py_str("x-ray"), py_str("xray"), py_str("x ray"), py_str("ultrasound"), py_str("mri"), py_str("ct scan"), py_str("mammogram"), py_str("colonoscopy"), py_str("endoscopy"), py_str("blood test"), py_str("blood tests"), py_str("referral"), py_str("outpatients")]) }
    s__APPOINTMENT_KINDS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("before"), py_str("beforehand"), py_str("in advance"), py_str("the day before"), py_str("night before"), py_str("before i go"), py_str("before it"), py_str("before i come in"), py_str("prepare"), py_str("preparing"), py_str("prep"), py_str("get ready"), py_str("be ready"), py_str("anything i should do"), py_str("anything i need"), py_str("anything i should"), py_str("is there anything"), py_str("do i need to do"), py_str("do i need to"), py_str("need to do anything"), py_str("what do i need"), py_str("what do i do"), py_str("what should i do"), py_str("what should i bring"), py_str("what do i bring"), py_str("bring"), py_str("bring with me"), py_str("take with me"), py_str("fast"), py_str("fasting"), py_str("should i eat"), py_str("should i drink"), py_str("eat before"), py_str("empty stomach"), py_str("how early"), py_str("what time should i")]) }
    s__BEFORE_IT(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("cancel"), py_str("cancelling"), py_str("canceling"), py_str("cancelled"), py_str("canceled"), py_str("reschedule"), py_str("rescheduling"), py_str("rebook"), py_str("rebooking"), py_str("postpone"), py_str("postponing"), py_str("call off"), py_str("didnt turn up"), py_str("did not turn up"), py_str("missed"), py_str("no show"), py_str("nervous"), py_str("scared"), py_str("afraid"), py_str("frightened"), py_str("terrified"), py_str("dread"), py_str("anxious"), py_str("anxiety"), py_str("phobia"), py_str("hate the dentist"), py_str("hate going"), py_str("toothache"), py_str("tooth ache"), py_str("pain"), py_str("hurts"), py_str("hurting"), py_str("abscess"), py_str("broken tooth"), py_str("chipped"), py_str("knocked out"), py_str("bleeding"), py_str("emergency"), py_str("a&e"), py_str("999"), py_str("urgent"), py_str("same day"), py_str("how much"), py_str("cost"), py_str("costs"), py_str("price"), py_str("free"), py_str("nhs"), py_str("register"), py_str("registering"), py_str("sign up"), py_str("find a dentist"), py_str("find a doctor"), py_str("take new patients"), py_str("waiting list"), py_str("how long will it take"), py_str("how long does it take"), py_str("how long will i be"), py_str("how long will it last"), py_str("after my appointment"), py_str("after the appointment"), py_str("afterwards"), py_str("what happens at"), py_str("what happens during"), py_str("results"), py_str("my results"), py_str("test results"), py_str("see a doctor"), py_str("see the doctor"), py_str("see a gp"), py_str("go to the doctor"), py_str("call a doctor"), py_str("should i go"), py_str("worth seeing"), py_str("visitors"), py_str("visiting"), py_str("visiting hours"), py_str("bring her"), py_str("bring him"), py_str("bring them"), py_str("bring a gift"), py_str("bring flowers"), py_str("bring something"), py_str("bring to my friend"), py_str("bring to my mum"), py_str("bring to my dad"), py_str("bring to see"), py_str("see a dentist"), py_str("see the dentist"), py_str("see an optician"), py_str("feel sick"), py_str("feeling sick"), py_str("nauseous"), py_str("implant"), py_str("extraction"), py_str("root canal"), py_str("crown"), py_str("dentures"), py_str("braces"), py_str("wisdom tooth")]) }
    s__NOT_ABOUT_GETTING_READY(__data3());
    s__ITS_A_BLOOD_TEST(py_tuple(vec![py_str("blood test"), py_str("blood tests"), py_str("blood sample"), py_str("bloods"), py_str("venepuncture"), py_str("cholesterol test"), py_str("thyroid test"), py_str("hba1c"), py_str("glucose test")]));
    fn __data4() -> Py { py_tuple(vec![py_str("scan"), py_str("scans"), py_str("x-ray"), py_str("xray"), py_str("x ray"), py_str("ultrasound"), py_str("mri"), py_str("ct scan"), py_str("mammogram"), py_str("colonoscopy"), py_str("endoscopy"), py_str("imaging"), py_str("radiographer")]) }
    s__ITS_A_SCAN(__data4());
    s__ITS_THE_DENTIST(py_tuple(vec![py_str("dentist"), py_str("dental"), py_str("hygienist"), py_str("check up"), py_str("checkup"), py_str("filling")]));
    s__ITS_THE_OPTICIAN(py_tuple(vec![py_str("optician"), py_str("opticians"), py_str("eye test"), py_str("sight test"), py_str("optometrist")]));
    s__ITS_AN_OPERATION(py_tuple(vec![py_str("surgery"), py_str("an operation"), py_str("operating"), py_str("anaesthetic"), py_str("anesthetic"), py_str("anaesthesia"), py_str("pre-op"), py_str("pre op"), py_str("preoperative"), py_str("theatre")]));
    fn __data5() -> Py { py_tuple(vec![py_str("shave"), py_str("shaving"), py_str("shaved"), py_str("hair removal"), py_str("wax"), py_str("waxing"), py_str("nail varnish"), py_str("nail polish"), py_str("make up"), py_str("makeup"), py_str("deodorant"), py_str("jewellery"), py_str("jewelry")]) }
    s__NOT_THE_FASTING(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("medication"), py_str("medications"), py_str("medicine"), py_str("medicines"), py_str("meds"), py_str("tablet"), py_str("tablets"), py_str("pills"), py_str("inhaler"), py_str("insulin"), py_str("blood thinner"), py_str("blood thinners"), py_str("warfarin"), py_str("statins"), py_str("antibiotic"), py_str("antibiotics"), py_str("painkiller"), py_str("painkillers")]) }
    s__MEDICINE_NAMED(__data6());
    s__A_LIST_TO_CARRY(py_tuple(vec![py_str("list"), py_str("lists"), py_str("boxes"), py_str("packet"), py_str("pack"), py_str("bring"), py_str("brings"), py_str("carry"), py_str("with me"), py_str("along"), py_str("prescription")]));
    s__A_FAST_LENGTH_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long\\b[^.?]*(?:fast|fasting|fasted|without food|no food|nothing to eat|empty stomach)|(?:fast|fasting|fasted)\\b[^.?]*\\bhow long\\b|\\bhow many hours\\b[^.?]*(?:fast|fasting|fasted|without food|no food|nothing to eat)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TEST_THAT_TAKES(stdlib_call("re", "compile", &[py_str("\\b(?:test|tests|appointment|appointments)\\s+(?:take|takes|took|last|lasts)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_str("eat"), py_str("eating"), py_str("drink"), py_str("drinking"), py_str("food"), py_str("water"), py_str("fast"), py_str("fasting"), py_str("empty stomach"), py_str("bring"), py_str("bring with me"), py_str("take with me"), py_str("get ready"), py_str("prepare"), py_str("preparing"), py_str("anything i should"), py_str("anything i need")]) }
    s__PREPARING_SUBJECT(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("before"), py_str("beforehand"), py_str("in advance"), py_str("day before"), py_str("night before"), py_str("before i go"), py_str("before i come in"), py_str("do i need to"), py_str("should i"), py_str("can i"), py_str("am i allowed"), py_str("what should i"), py_str("what do i"), py_str("what to"), py_str("what time"), py_str("how early"), py_str("is there anything")]) }
    s__PREPARING_ASK(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("bank"), py_str("building society"), py_str("job centre"), py_str("jobcentre"), py_str("benefits office"), py_str("council"), py_str("solicitor"), py_str("lawyer"), py_str("interview"), py_str("vet"), py_str("garage"), py_str("mot"), py_str("hairdresser"), py_str("barber"), py_str("estate agent"), py_str("letting agent"), py_str("landlord"), py_str("passport"), py_str("immigration"), py_str("school"), py_str("university"), py_str("mortgage"), py_str("tax office"), py_str("hmrc"), py_str("dwp")]) }
    s__NOT_A_MEDICAL_VISIT(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("dentist"), py_str("dental"), py_str("gp"), py_str("doctor"), py_str("hospital"), py_str("clinic"), py_str("nurse"), py_str("surgery"), py_str("health"), py_str("physio"), py_str("optician"), py_str("scan"), py_str("blood"), py_str("medical"), py_str("consultant"), py_str("midwife"), py_str("pharmacy"), py_str("a&e"), py_str("nhs")]) }
    s__STILL_MEDICAL(__data10());
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("before-an-appointment")); __args.push(py_tuple(vec![g__APPOINTMENT_KINDS(), g__BEFORE_IT()])); __args.push(py_func("_before_an_appointment", |args: &[Py]| _before_an_appointment(args))); __kws.extend(vec![("ask".to_string(), g__APPOINTMENT_KINDS()), ("avoid".to_string(), g__NOT_ABOUT_GETTING_READY())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("before-an-appointment-follow-up")); __args.push(py_tuple(vec![g__PREPARING_SUBJECT(), g__PREPARING_ASK()])); __args.push(py_func("_before_an_appointment", |args: &[Py]| _before_an_appointment(args))); __kws.extend(vec![("guard".to_string(), py_func("_follows_an_appointment", |args: &[Py]| _follows_an_appointment(args))), ("avoid".to_string(), g__NOT_ABOUT_GETTING_READY())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _a_dose_is_the_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut word: Py = Py::None;
    let _ = py_str("A tablet question rather than a fasting one, and not a list to carry in.\n\n    The verb is what divides them: \"should i take my medication before surgery\" is a dose, and\n    \"should i bring my medication list to the hospital\" is a thing to put in a bag, which is what\n    the preparation card answers.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MEDICINE_NAMED()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_LIST_TO_CARRY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut last_user: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The user's last line, or \"\" - read from whatever the runtime hands a card.\n\n    The understanding only hands the earlier line back as `leans_on` when the new line points at\n    nothing of its own, and \"should i eat before\" names food, so the lean is None exactly on the\n    follow-ups that need it most. The card reads the chat itself instead, and it reads it several\n    ways, because a card is passed a memory whose shape is the runtime's business: a helper that\n    moved, a turn that spells the user's line differently, or a copy of the memory with no turns\n    are all things a later change can do, and each one used to turn this card silently back into\n    the general one.\n    ");
    if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("context"), py_str("last_user"), py_str("earlier_line")]));
    for __item in __iter1 {
    key = __item.clone();
    said = mcall(&reading.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]), "lower", &[]);
    }
    }
    }
    let __try2 = exc::catch(|| {
    last_user = py_func("last_user", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_user", args));
    said = py_call(&last_user, &[memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    said = py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]), "lower", &[]);
    }
    turns = py_list(vec![]);
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&memory.clone(), "get", &[py_str("history")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } } } };
    }
    let __iter7 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter7 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter8 = py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("said"), py_str("text"), py_str("clean")]));
    for __item in __iter8 {
    key = __item.clone();
    said = mcall(&turn.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]), "lower", &[]);
    }
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_line_wants_a_length(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("Whether the line asks how long to go without food, which is the hours and not this advice.\n\n    `memory[\"understood\"][\"wants\"]` is the shared reading of the shape of answer a line asks for,\n    written once by bot/modules/intent.py, and \"time\" is a length. This card has no length in it,\n    so on a fasting question it stands aside and the row in health.py that opens with \"eight to\n    twelve hours without food\" answers. The card keeps every line it can answer itself: \"do i\n    need to fast\", \"what should i do before a blood test\", \"what should i bring\", and \"how long\n    does a fasting blood test take\", whose five minutes in the chair are here.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("wants")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &py_str("time")))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_TEST_THAT_TAKES(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__A_FAST_LENGTH_ASK(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _before_an_appointment(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The card for the kind of appointment the line is about, or the general one.");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    leans = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("leans_on")]);
    before = py_str("");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&leans.clone(), "get", &[py_str("text")]) } }) {
    before = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__APPOINTMENT_KINDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    earlier = _the_line_before(&[memory.clone(), reading.clone()]);
    if py_truthy(&{ let __b = earlier.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&line.clone(), &earlier.clone())) } }) {
    before = mcall(&op_add(&op_add(&before.clone(), &py_str(" ")), &earlier.clone()), "strip", &[]);
    }
    }
    if py_truthy(&before.clone()) {
    let __aug1 = op_add(&line.clone(), &op_add(&py_str(" "), &before.clone()));
    line = __aug1;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_MEDICAL_VISIT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STILL_MEDICAL()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITS_AN_OPERATION()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _a_dose_is_the_question(&[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITS_AN_OPERATION()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_THE_FASTING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("gp surgery"), py_str("the surgery"), py_str("at the surgery"), py_str("surgery hours"), py_str("surgery number")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return py_str("The hospital sets this one and you should not guess: usually nothing to eat for six hours before an anaesthetic and nothing clear to drink for two, and the letter or the pre-op call gives you the exact times. Ask about your regular tablets at the pre-op rather than deciding yourself, because most are continued but blood thinners are often stopped for a few days. Arrange for someone to bring you home, as you cannot drive or go home alone after a general anaesthetic.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITS_A_BLOOD_TEST()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&_the_line_wants_a_length(&[line.clone(), reading.clone()])) {
    return Py::None;
    }
    return py_str("Only fast if the letter or the person who booked it told you to: most blood tests need no fasting at all, and water is fine even when they do. Wear something with a short or loose sleeve so the arm is easy to get at, and keep your hands warm on the way in, which makes the vein easier to find. Keep taking your medicines as usual unless the letter says otherwise - thyroid and blood pressure tablets are meant to be taken, so ring the surgery rather than skipping one.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITS_A_SCAN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Follow the letter rather than a general rule, because what to do differs: a plain x-ray needs nothing, an ultrasound often wants a full bladder, some scans want an empty stomach for a few hours, and a colonoscopy is the bowel prep it gives you plus nothing to eat after the time it names. Leave metal at home for an MRI - jewellery, keys, an underwired bra - and tell them if you have a pacemaker, a metal implant or could be pregnant. Take the letter with you and ring the department if any of it is unclear or you cannot make the time.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITS_THE_DENTIST()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("For a check-up there is nothing to prepare: brush as usual and eat normally. Bring the names of the medicines you take, and say if you are on a blood thinner or have had a heart valve replaced, because they may want to check before treatment; tell them early if you are nervous, since they can plan the visit around it. Arrive a few minutes early, as the paperwork at a practice you have not been to before takes longer than the appointment.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ITS_THE_OPTICIAN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Take your current glasses, or the prescription if you have it, and any eye drops you use, and write down what you have noticed - blurring at a distance, trouble reading, glare at night - because the test is quicker when they know what to look for. Say if you drive: dilating drops blur your vision for a few hours, so you may want someone with you or the bus.");
    }
    return py_str("Read the letter or the text reminder first, because it says what to do before this one and most appointments need nothing. Worth doing every time: write down the two or three things you want to raise, most worrying first, and take the list of medicines you are on, or the boxes themselves. Arrive ten minutes early for a place you have not been to, and ring if you cannot make it - a missed slot usually means a long wait for the next one.");
    Py::None
}
pub fn _follows_an_appointment(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether this line is a question about an appointment the line before it named.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    before = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("subject"), py_str("topic")])) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[part.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    before = mcall(&mcall(&py_str(" "), "join", &[mcall(&before.clone(), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_MEDICAL_VISIT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&before.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STILL_MEDICAL()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&before.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__APPOINTMENT_KINDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&before.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.appointments".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_APPOINTMENT_KINDS" => Some(g__APPOINTMENT_KINDS()),
            "_A_FAST_LENGTH_ASK" => Some(g__A_FAST_LENGTH_ASK()),
            "_A_LIST_TO_CARRY" => Some(g__A_LIST_TO_CARRY()),
            "_A_TEST_THAT_TAKES" => Some(g__A_TEST_THAT_TAKES()),
            "_BEFORE_IT" => Some(g__BEFORE_IT()),
            "_ITS_AN_OPERATION" => Some(g__ITS_AN_OPERATION()),
            "_ITS_A_BLOOD_TEST" => Some(g__ITS_A_BLOOD_TEST()),
            "_ITS_A_SCAN" => Some(g__ITS_A_SCAN()),
            "_ITS_THE_DENTIST" => Some(g__ITS_THE_DENTIST()),
            "_ITS_THE_OPTICIAN" => Some(g__ITS_THE_OPTICIAN()),
            "_MEDICINE_NAMED" => Some(g__MEDICINE_NAMED()),
            "_NOT_ABOUT_GETTING_READY" => Some(g__NOT_ABOUT_GETTING_READY()),
            "_NOT_A_MEDICAL_VISIT" => Some(g__NOT_A_MEDICAL_VISIT()),
            "_NOT_THE_FASTING" => Some(g__NOT_THE_FASTING()),
            "_PREPARING_ASK" => Some(g__PREPARING_ASK()),
            "_PREPARING_SUBJECT" => Some(g__PREPARING_SUBJECT()),
            "_STILL_MEDICAL" => Some(g__STILL_MEDICAL()),
            "_a_dose_is_the_question" => Some(py_func("_a_dose_is_the_question", |args: &[Py]| _a_dose_is_the_question(args))),
            "_before_an_appointment" => Some(py_func("_before_an_appointment", |args: &[Py]| _before_an_appointment(args))),
            "_follows_an_appointment" => Some(py_func("_follows_an_appointment", |args: &[Py]| _follows_an_appointment(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_line_wants_a_length" => Some(py_func("_the_line_wants_a_length", |args: &[Py]| _the_line_wants_a_length(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_APPOINTMENT_KINDS" => s__APPOINTMENT_KINDS(v),
            "_A_FAST_LENGTH_ASK" => s__A_FAST_LENGTH_ASK(v),
            "_A_LIST_TO_CARRY" => s__A_LIST_TO_CARRY(v),
            "_A_TEST_THAT_TAKES" => s__A_TEST_THAT_TAKES(v),
            "_BEFORE_IT" => s__BEFORE_IT(v),
            "_ITS_AN_OPERATION" => s__ITS_AN_OPERATION(v),
            "_ITS_A_BLOOD_TEST" => s__ITS_A_BLOOD_TEST(v),
            "_ITS_A_SCAN" => s__ITS_A_SCAN(v),
            "_ITS_THE_DENTIST" => s__ITS_THE_DENTIST(v),
            "_ITS_THE_OPTICIAN" => s__ITS_THE_OPTICIAN(v),
            "_MEDICINE_NAMED" => s__MEDICINE_NAMED(v),
            "_NOT_ABOUT_GETTING_READY" => s__NOT_ABOUT_GETTING_READY(v),
            "_NOT_A_MEDICAL_VISIT" => s__NOT_A_MEDICAL_VISIT(v),
            "_NOT_THE_FASTING" => s__NOT_THE_FASTING(v),
            "_PREPARING_ASK" => s__PREPARING_ASK(v),
            "_PREPARING_SUBJECT" => s__PREPARING_SUBJECT(v),
            "_STILL_MEDICAL" => s__STILL_MEDICAL(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.appointments", module_obj);
    pyrt::xrefs::reg_fn("knowledge.appointments", "_a_dose_is_the_question", py_func("_a_dose_is_the_question", |args: &[Py]| _a_dose_is_the_question(args)));
    pyrt::xrefs::reg_fn("knowledge.appointments", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("knowledge.appointments", "_the_line_wants_a_length", py_func("_the_line_wants_a_length", |args: &[Py]| _the_line_wants_a_length(args)));
    pyrt::xrefs::reg_fn("knowledge.appointments", "_before_an_appointment", py_func("_before_an_appointment", |args: &[Py]| _before_an_appointment(args)));
    pyrt::xrefs::reg_fn("knowledge.appointments", "_follows_an_appointment", py_func("_follows_an_appointment", |args: &[Py]| _follows_an_appointment(args)));
    pyrt::xrefs::reg_access("knowledge.appointments", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.appointments", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.appointments", "_APPOINTMENT_KINDS", g__APPOINTMENT_KINDS, s__APPOINTMENT_KINDS);
    pyrt::xrefs::reg_access("knowledge.appointments", "_A_FAST_LENGTH_ASK", g__A_FAST_LENGTH_ASK, s__A_FAST_LENGTH_ASK);
    pyrt::xrefs::reg_access("knowledge.appointments", "_A_LIST_TO_CARRY", g__A_LIST_TO_CARRY, s__A_LIST_TO_CARRY);
    pyrt::xrefs::reg_access("knowledge.appointments", "_A_TEST_THAT_TAKES", g__A_TEST_THAT_TAKES, s__A_TEST_THAT_TAKES);
    pyrt::xrefs::reg_access("knowledge.appointments", "_BEFORE_IT", g__BEFORE_IT, s__BEFORE_IT);
    pyrt::xrefs::reg_access("knowledge.appointments", "_ITS_AN_OPERATION", g__ITS_AN_OPERATION, s__ITS_AN_OPERATION);
    pyrt::xrefs::reg_access("knowledge.appointments", "_ITS_A_BLOOD_TEST", g__ITS_A_BLOOD_TEST, s__ITS_A_BLOOD_TEST);
    pyrt::xrefs::reg_access("knowledge.appointments", "_ITS_A_SCAN", g__ITS_A_SCAN, s__ITS_A_SCAN);
    pyrt::xrefs::reg_access("knowledge.appointments", "_ITS_THE_DENTIST", g__ITS_THE_DENTIST, s__ITS_THE_DENTIST);
    pyrt::xrefs::reg_access("knowledge.appointments", "_ITS_THE_OPTICIAN", g__ITS_THE_OPTICIAN, s__ITS_THE_OPTICIAN);
    pyrt::xrefs::reg_access("knowledge.appointments", "_MEDICINE_NAMED", g__MEDICINE_NAMED, s__MEDICINE_NAMED);
    pyrt::xrefs::reg_access("knowledge.appointments", "_NOT_ABOUT_GETTING_READY", g__NOT_ABOUT_GETTING_READY, s__NOT_ABOUT_GETTING_READY);
    pyrt::xrefs::reg_access("knowledge.appointments", "_NOT_A_MEDICAL_VISIT", g__NOT_A_MEDICAL_VISIT, s__NOT_A_MEDICAL_VISIT);
    pyrt::xrefs::reg_access("knowledge.appointments", "_NOT_THE_FASTING", g__NOT_THE_FASTING, s__NOT_THE_FASTING);
    pyrt::xrefs::reg_access("knowledge.appointments", "_PREPARING_ASK", g__PREPARING_ASK, s__PREPARING_ASK);
    pyrt::xrefs::reg_access("knowledge.appointments", "_PREPARING_SUBJECT", g__PREPARING_SUBJECT, s__PREPARING_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.appointments", "_STILL_MEDICAL", g__STILL_MEDICAL, s__STILL_MEDICAL);
    pyrt::xrefs::reg_access("knowledge.appointments", "entry", g_entry, s_entry);
}
