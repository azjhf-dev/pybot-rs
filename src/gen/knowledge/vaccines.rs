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
pub static JAB_SAFETY_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEASLES_JAB_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PNEUMONIA_JAB_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREGNANCY_JAB_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHINGLES_JAB_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TETANUS_JAB_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VACCINE_BOOSTER_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_APPOINTMENT_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKING_IF_IT_IS_SAFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_JAB_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _JAB_BY_DISEASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_JAB_SAFETY_ANSWER() -> Py { ensure_init(); JAB_SAFETY_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JAB_SAFETY_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = JAB_SAFETY_ANSWER.write() { *w = v; } }
pub fn g_MEASLES_JAB_ANSWER() -> Py { ensure_init(); MEASLES_JAB_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEASLES_JAB_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = MEASLES_JAB_ANSWER.write() { *w = v; } }
pub fn g_PNEUMONIA_JAB_ANSWER() -> Py { ensure_init(); PNEUMONIA_JAB_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PNEUMONIA_JAB_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = PNEUMONIA_JAB_ANSWER.write() { *w = v; } }
pub fn g_PREGNANCY_JAB_ANSWER() -> Py { ensure_init(); PREGNANCY_JAB_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_JAB_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = PREGNANCY_JAB_ANSWER.write() { *w = v; } }
pub fn g_SHINGLES_JAB_ANSWER() -> Py { ensure_init(); SHINGLES_JAB_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHINGLES_JAB_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = SHINGLES_JAB_ANSWER.write() { *w = v; } }
pub fn g_TETANUS_JAB_ANSWER() -> Py { ensure_init(); TETANUS_JAB_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TETANUS_JAB_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = TETANUS_JAB_ANSWER.write() { *w = v; } }
pub fn g_VACCINE_BOOSTER_ANSWER() -> Py { ensure_init(); VACCINE_BOOSTER_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VACCINE_BOOSTER_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = VACCINE_BOOSTER_ANSWER.write() { *w = v; } }
pub fn g__AN_APPOINTMENT_DAY() -> Py { ensure_init(); _AN_APPOINTMENT_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_APPOINTMENT_DAY(v: Py) { ensure_init(); if let Ok(mut w) = _AN_APPOINTMENT_DAY.write() { *w = v; } }
pub fn g__ASKING_IF_IT_IS_SAFE() -> Py { ensure_init(); _ASKING_IF_IT_IS_SAFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKING_IF_IT_IS_SAFE(v: Py) { ensure_init(); if let Ok(mut w) = _ASKING_IF_IT_IS_SAFE.write() { *w = v; } }
pub fn g__A_JAB_WORD() -> Py { ensure_init(); _A_JAB_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_JAB_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_JAB_WORD.write() { *w = v; } }
pub fn g__JAB_BY_DISEASE() -> Py { ensure_init(); _JAB_BY_DISEASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__JAB_BY_DISEASE(v: Py) { ensure_init(); if let Ok(mut w) = _JAB_BY_DISEASE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Vaccinations: which jab a disease names, when a booster is due, and whether it is safe.\n\nThe answers lane owns this domain. It is a domain of its own rather than another run of rows at\nthe end of bot/knowledge/health.py for two reasons. The word \"booster\" is three things - a jab, a\nwifi repeater and the child car seat - and the rows that own the other two live in\nbot/knowledge/technology.py and bot/knowledge/cars.py, so the jab's rows are the third side of\nthat cross-product and read best together with the comment that says so. And health.py is the\nlargest file in the knowledge base, close enough to the uploader's size ceiling that every\nappend there is a conflict with whoever appended last; a new domain file is one file the other\nlanes never touch.\n\nImported back from health.py, not copied: `PETS` (a dog's booster is the vet's) and `_the_line`,\nthe wording reader every guard in that file uses.\n");
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    s_VACCINE_BOOSTER_ANSWER(py_str("The covid and flu boosters are offered each autumn to anyone 65 or over, pregnant, in a care home, or living with a condition like asthma, diabetes or a weak immune system, plus carers and health staff; the invitation comes to you, and failing that a pharmacy or the NHS app will book it. They are yearly because protection fades, and outside those groups a pharmacy gives the flu jab for about £15 to £20."));
    s_TETANUS_JAB_ANSWER(py_str("You need a tetanus booster if it is more than 5 years since your last one and the wound was dirty - soil, a bite, a deep puncture or a burn - while a clean small cut within 10 years is usually fine. A pharmacist, a minor injuries unit or A&E can give it the same day, so ask today rather than waiting. Get a deep or heavily bleeding wound looked at straight away, not just for the jab."));
    s_MEASLES_JAB_ANSWER(py_str("Measles is covered by the MMR, which is two doses: the first at about 1 year old and the second at 3 years and 4 months, and two doses is what counts as protected. An adult born after 1970 who never had two doses, or cannot remember, can have the MMR free at a GP surgery - and anyone born before 1970 is usually immune from having had measles itself. One dose gives about 90 per cent protection and two gives about 99."));
    s_SHINGLES_JAB_ANSWER(py_str("The shingles vaccine is offered on the NHS when you turn 65 and again at 70, and it is two doses given two months apart. If you have already had shingles you can still have it, because it lowers the chance of it coming back, and younger adults with a weakened immune system are offered it too - ask the GP if you are not sure whether you have had it."));
    s_PNEUMONIA_JAB_ANSWER(py_str("The pneumonia jab (pneumococcal) is a one-off rather than a yearly one, offered at 65 and to anyone with asthma, diabetes, heart, kidney or liver disease, or no spleen. If you have had it you do not need it again, and the GP record will say - the yearly autumn jab you are thinking of is the flu one, which is a different vaccine."));
    s_JAB_SAFETY_ANSWER(py_str("A sore arm, tiredness and a mild fever for a day or two are the ordinary after-effects, and serious reactions are rare - a severe allergic reaction is about one in a million doses, which is why you wait 15 minutes afterwards. The flu jab cannot give you flu because it is not a live virus, while the MMR and the children's nasal flu spray are live but weakened and are not given in pregnancy or to someone whose immune system is badly weakened. Mention any reaction you have had before when you book, and the pharmacist or GP will say whether it is a reason to skip one."));
    s_PREGNANCY_JAB_ANSWER(py_str("The flu and covid jabs are offered in pregnancy at any stage, because both illnesses are harder on you and the baby then, and the whooping cough jab is offered from about 16 weeks and again in each pregnancy. Say you are pregnant when you book, at the GP, the pharmacy or the maternity team, and they will give you the one for your stage."));
    s__AN_APPOINTMENT_DAY(stdlib_call("re", "compile", &[py_str("\\b(monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|tonight|next week|next month|this week|this afternoon|this morning|booked|booking|appointment|appointments|rescheduled|rebooked|cancelled|canceled|yesterday|already had|already been|just had)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_JAB_WORD(stdlib_call("re", "compile", &[py_str("\\b(covid|flu|jab|jabs|jabbed|jabbing|vaccine|vaccines|vaccinate|vaccinated|vaccination|vaccinations|immunisation|immunization|immunised|immunized|inoculation|tetanus|lockjaw|booster shot|booster jab|pregnan\\w*|expecting|whooping cough|shingles|pneumonia)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__JAB_BY_DISEASE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("measles"), py_str("mumps"), py_str("rubella"), py_str("mmr")]), g_MEASLES_JAB_ANSWER()]), py_tuple(vec![py_tuple(vec![py_str("shingles")]), g_SHINGLES_JAB_ANSWER()]), py_tuple(vec![py_tuple(vec![py_str("pneumonia"), py_str("pneumococcal")]), g_PNEUMONIA_JAB_ANSWER()])]));
    s__ASKING_IF_IT_IS_SAFE(stdlib_call("re", "compile", &[py_str("\\b(?:\\w+\\s+){0,4}safe\\b|\\bany (?:risk|risks|danger)\\b|\\bside effects?\\b|\\bdangerous\\b|\\bharmful\\b|\\bbad for (?:me|you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("vaccine-booster")); __args.push(py_tuple(vec![py_tuple(vec![py_str("booster"), py_str("booster jab"), py_str("booster shot"), py_str("booster dose"), py_str("booster vaccine"), py_str("top up jab"), py_str("covid booster"), py_str("covid jab"), py_str("covid vaccine"), py_str("covid vaccination"), py_str("covid shot"), py_str("jab"), py_str("jabs"), py_str("jabbed"), py_str("vaccine"), py_str("vaccines"), py_str("vaccination"), py_str("vaccinations"), py_str("immunisation"), py_str("immunization"), py_str("tetanus"), py_str("tetanus jab"), py_str("tetanus shot"), py_str("flu booster"), py_str("measles"), py_str("mumps"), py_str("rubella"), py_str("mmr"), py_str("shingles"), py_str("shingles jab"), py_str("pneumonia"), py_str("pneumococcal")]), py_tuple(vec![py_str("need"), py_str("needed"), py_str("should"), py_str("when"), py_str("due"), py_str("overdue"), py_str("get"), py_str("got"), py_str("have"), py_str("had"), py_str("worth"), py_str("how often"), py_str("again"), py_str("another"), py_str("next"), py_str("am i"), py_str("do i"), py_str("can i"), py_str("is it"), py_str("time"), py_str("book"), py_str("where"), py_str("free"), py_str("pay"), py_str("top up"), py_str("safe"), py_str("ok"), py_str("fine")])])); __args.push(g__vaccine_booster_answer()); __kws.extend(vec![("avoid".to_string(), add(&py_tuple(vec![py_str("seat"), py_str("car seat"), py_str("booster seat"), py_str("child seat"), py_str("isofix"), py_str("cushion"), py_str("trolley"), py_str("backless"), py_str("high backed"), py_str("high-backed"), py_str("rear facing"), py_str("rear-facing"), py_str("135"), py_str("height"), py_str("booster cushion"), py_str("pushchair"), py_str("high chair"), py_str("car"), py_str("cars"), py_str("van"), py_str("driving"), py_str("travel"), py_str("flight"), py_str("plane"), py_str("holiday"), py_str("wifi"), py_str("wi fi"), py_str("wireless"), py_str("router"), py_str("internet"), py_str("broadband"), py_str("signal"), py_str("reception"), py_str("mobile signal"), py_str("tv"), py_str("telly"), py_str("aerial"), py_str("speaker"), py_str("amp"), py_str("hifi"), py_str("punch"), py_str("punching"), py_str("boxing"), py_str("sparring"), py_str("after the"), py_str("after my"), py_str("paracetamol"), py_str("ibuprofen"), py_str("alcohol"), py_str("drink"), py_str("yellow fever"), py_str("typhoid"), py_str("hepatitis"), py_str("rabies"), py_str("dengue"), py_str("malaria"), py_str("travel vaccine"), py_str("travel jab"), py_str("travel clinic"), py_str("sore"), py_str("ache"), py_str("hurts"), py_str("hurting"), py_str("swollen"), py_str("side effect"), py_str("side effects"), py_str("arm"), py_str("shoulder"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("newborn"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("my son"), py_str("my daughter"), py_str("my boy"), py_str("my girl"), py_str("school"), py_str("nursery"), py_str("puppy"), py_str("kitten"), py_str("hamster"), py_str("rabbit"), py_str("parrot")]), &crate::gen::knowledge::health::g_PETS())), ("guard".to_string(), g__the_vaccine_row_is_the_answer())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
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
    if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("question")))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g__AN_APPOINTMENT_DAY(), "search", &[crate::gen::knowledge::health::_the_line(&[understood.clone()])])));
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
    return { let __b = _it_is_not_an_appointment_already(&[understood.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _not_the_flu_on_its_own(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _the_booster_is_the_jab(&[understood.clone()]) } } } };
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
    if py_truthy(&mcall(&g__A_JAB_WORD(), "search", &[crate::gen::knowledge::health::_the_line(&[understood.clone()])])) {
    return Py::Bool(true);
    }
    lean = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("leans_on")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[{ let __b = mcall(&lean.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])))) {
    return Py::Bool(true);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[py_list(vec![call_builtin("str", &[{ let __b = mcall(&lean.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("str", &[{ let __b = mcall(&lean.clone(), "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&lean.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])]), "lower", &[]);
    return call_builtin("bool", &[mcall(&g__A_JAB_WORD(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
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
    line = crate::gen::knowledge::health::_the_line(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("flu"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("pregnan"), py_str("expecting"), py_str("booster"), py_str("jabbed"), py_str("booster jab")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
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
    line = crate::gen::knowledge::health::_the_line(&[understood.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("pregnan"), py_str("expecting"), py_str("due in"), py_str("weeks gone")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_PREGNANCY_JAB_ANSWER();
    }
    if py_truthy(&mcall(&g__ASKING_IF_IT_IS_SAFE(), "search", &[line.clone()])) {
    return g_JAB_SAFETY_ANSWER();
    }
    let __iter1 = py_iter_or_raise(&g__JAB_BY_DISEASE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    answer = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
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
        set: None,
    }))
}
