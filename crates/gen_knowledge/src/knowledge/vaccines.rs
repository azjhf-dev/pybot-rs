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
thread_local! { static G_JAB_SAFETY_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASLES_JAB_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PNEUMONIA_JAB_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANCY_JAB_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHINGLES_JAB_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TETANUS_JAB_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VACCINE_BOOSTER_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_APPOINTMENT_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING_IF_IT_IS_SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JAB_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JAB_BY_DISEASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_JAB_SAFETY_ANSWER() -> Py { ensure_init(); G_JAB_SAFETY_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JAB_SAFETY_ANSWER(v: Py) { ensure_init(); G_JAB_SAFETY_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASLES_JAB_ANSWER() -> Py { ensure_init(); G_MEASLES_JAB_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASLES_JAB_ANSWER(v: Py) { ensure_init(); G_MEASLES_JAB_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_PETS() -> Py { ensure_init(); G_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETS(v: Py) { ensure_init(); G_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_PNEUMONIA_JAB_ANSWER() -> Py { ensure_init(); G_PNEUMONIA_JAB_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PNEUMONIA_JAB_ANSWER(v: Py) { ensure_init(); G_PNEUMONIA_JAB_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANCY_JAB_ANSWER() -> Py { ensure_init(); G_PREGNANCY_JAB_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_JAB_ANSWER(v: Py) { ensure_init(); G_PREGNANCY_JAB_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_SHINGLES_JAB_ANSWER() -> Py { ensure_init(); G_SHINGLES_JAB_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHINGLES_JAB_ANSWER(v: Py) { ensure_init(); G_SHINGLES_JAB_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_TETANUS_JAB_ANSWER() -> Py { ensure_init(); G_TETANUS_JAB_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TETANUS_JAB_ANSWER(v: Py) { ensure_init(); G_TETANUS_JAB_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_VACCINE_BOOSTER_ANSWER() -> Py { ensure_init(); G_VACCINE_BOOSTER_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VACCINE_BOOSTER_ANSWER(v: Py) { ensure_init(); G_VACCINE_BOOSTER_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_APPOINTMENT_DAY() -> Py { ensure_init(); G__AN_APPOINTMENT_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_APPOINTMENT_DAY(v: Py) { ensure_init(); G__AN_APPOINTMENT_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING_IF_IT_IS_SAFE() -> Py { ensure_init(); G__ASKING_IF_IT_IS_SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING_IF_IT_IS_SAFE(v: Py) { ensure_init(); G__ASKING_IF_IT_IS_SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JAB_WORD() -> Py { ensure_init(); G__A_JAB_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JAB_WORD(v: Py) { ensure_init(); G__A_JAB_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__JAB_BY_DISEASE() -> Py { ensure_init(); G__JAB_BY_DISEASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JAB_BY_DISEASE(v: Py) { ensure_init(); G__JAB_BY_DISEASE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Vaccinations: which jab a disease names, when a booster is due, and whether it is safe.\n\nThe answers lane owns this domain. It is a domain of its own rather than another run of rows at\nthe end of bot/knowledge/health.py for two reasons. The word \"booster\" is three things - a jab, a\nwifi repeater and the child car seat - and the rows that own the other two live in\nbot/knowledge/technology.py and bot/knowledge/cars.py, so the jab's rows are the third side of\nthat cross-product and read best together with the comment that says so. And health.py is the\nlargest file in the knowledge base, close enough to the uploader's size ceiling that every\nappend there is a conflict with whoever appended last; a new domain file is one file the other\nlanes never touch.\n\nImported back from health.py, not copied: `PETS` (a dog's booster is the vet's) and `_the_line`,\nthe wording reader every guard in that file uses.\n");
    s_PETS(crate::knowledge::health::g_PETS());
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    s_VACCINE_BOOSTER_ANSWER(py_str("The covid and flu boosters are offered each autumn to anyone 65 or over, pregnant, in a care home, or living with a condition like asthma, diabetes or a weak immune system, plus carers and health staff; the invitation comes to you, and failing that a pharmacy or the NHS app will book it. They are yearly because protection fades, and outside those groups a pharmacy gives the flu jab for about £15 to £20."));
    s_TETANUS_JAB_ANSWER(py_str("You need a tetanus booster if it is more than 5 years since your last one and the wound was dirty - soil, a bite, a deep puncture or a burn - while a clean small cut within 10 years is usually fine. A pharmacist, a minor injuries unit or A&E can give it the same day, so ask today rather than waiting. Get a deep or heavily bleeding wound looked at straight away, not just for the jab."));
    s_MEASLES_JAB_ANSWER(py_str("Measles is covered by the MMR, which is two doses: the first at about 1 year old and the second at 3 years and 4 months, and two doses is what counts as protected. An adult born after 1970 who never had two doses, or cannot remember, can have the MMR free at a GP surgery - and anyone born before 1970 is usually immune from having had measles itself. One dose gives about 90 per cent protection and two gives about 99."));
    s_SHINGLES_JAB_ANSWER(py_str("The shingles vaccine is offered on the NHS when you turn 65 and again at 70, and it is two doses given two months apart. If you have already had shingles you can still have it, because it lowers the chance of it coming back, and younger adults with a weakened immune system are offered it too - ask the GP if you are not sure whether you have had it."));
    s_PNEUMONIA_JAB_ANSWER(py_str("The pneumonia jab (pneumococcal) is a one-off rather than a yearly one, offered at 65 and to anyone with asthma, diabetes, heart, kidney or liver disease, or no spleen. If you have had it you do not need it again, and the GP record will say - the yearly autumn jab you are thinking of is the flu one, which is a different vaccine."));
    s_JAB_SAFETY_ANSWER(py_str("A sore arm, tiredness and a mild fever for a day or two are the ordinary after-effects, and serious reactions are rare - a severe allergic reaction is about one in a million doses, which is why you wait 15 minutes afterwards. The flu jab cannot give you flu because it is not a live virus, while the MMR and the children's nasal flu spray are live but weakened and are not given in pregnancy or to someone whose immune system is badly weakened. Mention any reaction you have had before when you book, and the pharmacist or GP will say whether it is a reason to skip one."));
    s_PREGNANCY_JAB_ANSWER(py_str("The flu and covid jabs are offered in pregnancy at any stage, because both illnesses are harder on you and the baby then, and the whooping cough jab is offered from about 16 weeks and again in each pregnancy. Say you are pregnant when you book, at the GP, the pharmacy or the maternity team, and they will give you the one for your stage."));
    s__AN_APPOINTMENT_DAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|tonight|next week|next month|this week|this afternoon|this morning|booked|booking|appointment|appointments|rescheduled|rebooked|cancelled|canceled|yesterday|already had|already been|just had)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_JAB_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(covid|flu|jab|jabs|jabbed|jabbing|vaccine|vaccines|vaccinate|vaccinated|vaccination|vaccinations|immunisation|immunization|immunised|immunized|inoculation|tetanus|lockjaw|booster shot|booster jab|pregnan\\w*|expecting|whooping cough|shingles|pneumonia)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__JAB_BY_DISEASE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("measles"), py_str("mumps"), py_str("rubella"), py_str("mmr")]), g_MEASLES_JAB_ANSWER()]), py_tuple(vec![py_tuple(vec![py_str("shingles")]), g_SHINGLES_JAB_ANSWER()]), py_tuple(vec![py_tuple(vec![py_str("pneumonia"), py_str("pneumococcal")]), g_PNEUMONIA_JAB_ANSWER()])]));
    s__ASKING_IF_IT_IS_SAFE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\w+\\s+){0,4}safe\\b|\\bany (?:risk|risks|danger)\\b|\\bside effects?\\b|\\bdangerous\\b|\\bharmful\\b|\\bbad for (?:me|you)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ENTRIES(py_list(vec![]));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("vaccine-booster")); __args.push(py_tuple(vec![py_tuple(vec![py_str("booster"), py_str("booster jab"), py_str("booster shot"), py_str("booster dose"), py_str("booster vaccine"), py_str("top up jab"), py_str("covid booster"), py_str("covid jab"), py_str("covid vaccine"), py_str("covid vaccination"), py_str("covid shot"), py_str("jab"), py_str("jabs"), py_str("jabbed"), py_str("vaccine"), py_str("vaccines"), py_str("vaccination"), py_str("vaccinations"), py_str("immunisation"), py_str("immunization"), py_str("tetanus"), py_str("tetanus jab"), py_str("tetanus shot"), py_str("flu booster"), py_str("measles"), py_str("mumps"), py_str("rubella"), py_str("mmr"), py_str("shingles"), py_str("shingles jab"), py_str("pneumonia"), py_str("pneumococcal")]), py_tuple(vec![py_str("need"), py_str("needed"), py_str("should"), py_str("when"), py_str("due"), py_str("overdue"), py_str("get"), py_str("got"), py_str("have"), py_str("had"), py_str("worth"), py_str("how often"), py_str("again"), py_str("another"), py_str("next"), py_str("am i"), py_str("do i"), py_str("can i"), py_str("is it"), py_str("time"), py_str("book"), py_str("where"), py_str("free"), py_str("pay"), py_str("top up"), py_str("safe"), py_str("ok"), py_str("fine")])])); __args.push(g__vaccine_booster_answer()); __kws.extend(vec![("avoid".to_string(), op_add(&py_tuple(vec![py_str("seat"), py_str("car seat"), py_str("booster seat"), py_str("child seat"), py_str("isofix"), py_str("cushion"), py_str("trolley"), py_str("backless"), py_str("high backed"), py_str("high-backed"), py_str("rear facing"), py_str("rear-facing"), py_str("135"), py_str("height"), py_str("booster cushion"), py_str("pushchair"), py_str("high chair"), py_str("car"), py_str("cars"), py_str("van"), py_str("driving"), py_str("travel"), py_str("flight"), py_str("plane"), py_str("holiday"), py_str("wifi"), py_str("wi fi"), py_str("wireless"), py_str("router"), py_str("internet"), py_str("broadband"), py_str("signal"), py_str("reception"), py_str("mobile signal"), py_str("tv"), py_str("telly"), py_str("aerial"), py_str("speaker"), py_str("amp"), py_str("hifi"), py_str("punch"), py_str("punching"), py_str("boxing"), py_str("sparring"), py_str("after the"), py_str("after my"), py_str("paracetamol"), py_str("ibuprofen"), py_str("alcohol"), py_str("drink"), py_str("yellow fever"), py_str("typhoid"), py_str("hepatitis"), py_str("rabies"), py_str("dengue"), py_str("malaria"), py_str("travel vaccine"), py_str("travel jab"), py_str("travel clinic"), py_str("sore"), py_str("ache"), py_str("hurts"), py_str("hurting"), py_str("swollen"), py_str("side effect"), py_str("side effects"), py_str("arm"), py_str("shoulder"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("newborn"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("my son"), py_str("my daughter"), py_str("my boy"), py_str("my girl"), py_str("school"), py_str("nursery"), py_str("puppy"), py_str("kitten"), py_str("hamster"), py_str("rabbit"), py_str("parrot")]), &crate::knowledge::health::g_PETS())), ("guard".to_string(), g__the_vaccine_row_is_the_answer())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
}
pub fn _it_is_not_an_appointment_already(args: &[Py]) -> Py {
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
    let _ = py_str("A jab booked for a day is news to react to, not the schedule question.");
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_str("question")))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::health::_the_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_APPOINTMENT_DAY(), "search", args))(&__args) }));
    Py::None
}
pub fn _the_vaccine_row_is_the_answer(args: &[Py]) -> Py {
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
    let _ = py_str("Every gate: not a booked jab, not the flu on its own, and not a booster of another kind.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _it_is_not_an_appointment_already(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_the_flu_on_its_own(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_booster_is_the_jab(args))(&__args) } } } } };
    Py::None
}
pub fn _the_booster_is_the_jab(args: &[Py]) -> Py {
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
    let mut lean: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A booster alone is the jab only when the chat it follows is about a jab as well.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::health::_the_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_JAB_WORD(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    lean = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leans_on")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }))) {
    return Py::Bool(true);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_JAB_WORD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _not_the_flu_on_its_own(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A plain flu question is the flu-jab row's, which is the more specific answer.\n\n    The flu on its own (\"when do I get my flu vaccine\") names the same jab this row does, so it\n    scored here too and took the flu card's line. It stays with the flu card unless the line adds\n    what makes it this row's question - a pregnancy, or the word booster.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::health::_the_line(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("flu"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("pregnan"), py_str("expecting"), py_str("booster"), py_str("jabbed"), py_str("booster jab")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _vaccine_booster_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The disease names the jab; the timings are the pregnancy, the wound and the autumn.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::health::_the_line(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("pregnan"), py_str("expecting"), py_str("due in"), py_str("weeks gone")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_PREGNANCY_JAB_ANSWER();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKING_IF_IT_IS_SAFE(), "search", args))(&__args) }) {
    return g_JAB_SAFETY_ANSWER();
    }
    let __iter1 = py_iter_or_raise(&g__JAB_BY_DISEASE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    answer = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return answer.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("tetanus"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("lockjaw"))) } }) {
    return g_TETANUS_JAB_ANSWER();
    }
    return g_VACCINE_BOOSTER_ANSWER();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.vaccines".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "JAB_SAFETY_ANSWER" => Some(g_JAB_SAFETY_ANSWER()),
            "MEASLES_JAB_ANSWER" => Some(g_MEASLES_JAB_ANSWER()),
            "PETS" => Some(g_PETS()),
            "PNEUMONIA_JAB_ANSWER" => Some(g_PNEUMONIA_JAB_ANSWER()),
            "PREGNANCY_JAB_ANSWER" => Some(g_PREGNANCY_JAB_ANSWER()),
            "SHINGLES_JAB_ANSWER" => Some(g_SHINGLES_JAB_ANSWER()),
            "TETANUS_JAB_ANSWER" => Some(g_TETANUS_JAB_ANSWER()),
            "VACCINE_BOOSTER_ANSWER" => Some(g_VACCINE_BOOSTER_ANSWER()),
            "_AN_APPOINTMENT_DAY" => Some(g__AN_APPOINTMENT_DAY()),
            "_ASKING_IF_IT_IS_SAFE" => Some(g__ASKING_IF_IT_IS_SAFE()),
            "_A_JAB_WORD" => Some(g__A_JAB_WORD()),
            "_JAB_BY_DISEASE" => Some(g__JAB_BY_DISEASE()),
            "_it_is_not_an_appointment_already" => Some(py_func("_it_is_not_an_appointment_already", |args: &[Py]| _it_is_not_an_appointment_already(args))),
            "_not_the_flu_on_its_own" => Some(py_func("_not_the_flu_on_its_own", |args: &[Py]| _not_the_flu_on_its_own(args))),
            "_the_booster_is_the_jab" => Some(py_func("_the_booster_is_the_jab", |args: &[Py]| _the_booster_is_the_jab(args))),
            "_the_vaccine_row_is_the_answer" => Some(py_func("_the_vaccine_row_is_the_answer", |args: &[Py]| _the_vaccine_row_is_the_answer(args))),
            "_vaccine_booster_answer" => Some(py_func("_vaccine_booster_answer", |args: &[Py]| _vaccine_booster_answer(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "JAB_SAFETY_ANSWER" => s_JAB_SAFETY_ANSWER(v),
            "MEASLES_JAB_ANSWER" => s_MEASLES_JAB_ANSWER(v),
            "PETS" => s_PETS(v),
            "PNEUMONIA_JAB_ANSWER" => s_PNEUMONIA_JAB_ANSWER(v),
            "PREGNANCY_JAB_ANSWER" => s_PREGNANCY_JAB_ANSWER(v),
            "SHINGLES_JAB_ANSWER" => s_SHINGLES_JAB_ANSWER(v),
            "TETANUS_JAB_ANSWER" => s_TETANUS_JAB_ANSWER(v),
            "VACCINE_BOOSTER_ANSWER" => s_VACCINE_BOOSTER_ANSWER(v),
            "_AN_APPOINTMENT_DAY" => s__AN_APPOINTMENT_DAY(v),
            "_ASKING_IF_IT_IS_SAFE" => s__ASKING_IF_IT_IS_SAFE(v),
            "_A_JAB_WORD" => s__A_JAB_WORD(v),
            "_JAB_BY_DISEASE" => s__JAB_BY_DISEASE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.vaccines", module_obj);
    pyrt::xrefs::reg_fn("knowledge.vaccines", "_it_is_not_an_appointment_already", py_func("_it_is_not_an_appointment_already", |args: &[Py]| _it_is_not_an_appointment_already(args)));
    pyrt::xrefs::reg_fn("knowledge.vaccines", "_the_vaccine_row_is_the_answer", py_func("_the_vaccine_row_is_the_answer", |args: &[Py]| _the_vaccine_row_is_the_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.vaccines", "_the_booster_is_the_jab", py_func("_the_booster_is_the_jab", |args: &[Py]| _the_booster_is_the_jab(args)));
    pyrt::xrefs::reg_fn("knowledge.vaccines", "_not_the_flu_on_its_own", py_func("_not_the_flu_on_its_own", |args: &[Py]| _not_the_flu_on_its_own(args)));
    pyrt::xrefs::reg_fn("knowledge.vaccines", "_vaccine_booster_answer", py_func("_vaccine_booster_answer", |args: &[Py]| _vaccine_booster_answer(args)));
    pyrt::xrefs::reg_access("knowledge.vaccines", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.vaccines", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.vaccines", "JAB_SAFETY_ANSWER", g_JAB_SAFETY_ANSWER, s_JAB_SAFETY_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "MEASLES_JAB_ANSWER", g_MEASLES_JAB_ANSWER, s_MEASLES_JAB_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "PETS", g_PETS, s_PETS);
    pyrt::xrefs::reg_access("knowledge.vaccines", "PNEUMONIA_JAB_ANSWER", g_PNEUMONIA_JAB_ANSWER, s_PNEUMONIA_JAB_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "PREGNANCY_JAB_ANSWER", g_PREGNANCY_JAB_ANSWER, s_PREGNANCY_JAB_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "SHINGLES_JAB_ANSWER", g_SHINGLES_JAB_ANSWER, s_SHINGLES_JAB_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "TETANUS_JAB_ANSWER", g_TETANUS_JAB_ANSWER, s_TETANUS_JAB_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "VACCINE_BOOSTER_ANSWER", g_VACCINE_BOOSTER_ANSWER, s_VACCINE_BOOSTER_ANSWER);
    pyrt::xrefs::reg_access("knowledge.vaccines", "_AN_APPOINTMENT_DAY", g__AN_APPOINTMENT_DAY, s__AN_APPOINTMENT_DAY);
    pyrt::xrefs::reg_access("knowledge.vaccines", "_ASKING_IF_IT_IS_SAFE", g__ASKING_IF_IT_IS_SAFE, s__ASKING_IF_IT_IS_SAFE);
    pyrt::xrefs::reg_access("knowledge.vaccines", "_A_JAB_WORD", g__A_JAB_WORD, s__A_JAB_WORD);
    pyrt::xrefs::reg_access("knowledge.vaccines", "_JAB_BY_DISEASE", g__JAB_BY_DISEASE, s__JAB_BY_DISEASE);
}
