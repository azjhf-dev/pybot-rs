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
thread_local! { static G__ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DOG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIDNEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_OTHER_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.pets_health: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING() -> Py { ensure_init(); G__ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING(v: Py) { ensure_init(); G__ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g__CAT() -> Py { ensure_init(); G__CAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAT(v: Py) { ensure_init(); G__CAT.with(|c| *c.borrow_mut() = v); }
pub fn g__DOG() -> Py { ensure_init(); G__DOG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DOG(v: Py) { ensure_init(); G__DOG.with(|c| *c.borrow_mut() = v); }
pub fn g__KIDNEY() -> Py { ensure_init(); G__KIDNEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIDNEY(v: Py) { ensure_init(); G__KIDNEY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PET() -> Py { ensure_init(); G__NOT_A_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PET(v: Py) { ensure_init(); G__NOT_A_PET.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_OTHER_ANIMAL() -> Py { ensure_init(); G__NOT_THE_OTHER_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_OTHER_ANIMAL(v: Py) { ensure_init(); G__NOT_THE_OTHER_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A cat or a dog with kidney disease, which is the commonest long illness an old pet has.\n\n`tools/candidates.py` found this one in the deck: `my cat has kidney disease` was answered with\n\"You're worried about your cat... is it eating and drinking as usual\", and every question after\nit went somewhere else - \"how long has he got\" got the *lifespan* card (\"an indoor cat lives\nabout 12 to 15 years\"), \"what should i feed him\" got the ordinary portion card (\"3 or 4 pouches\nof 85g\"), \"how often should he see the vet\" got the yearly-booster card, and \"what is chronic\nkidney disease in cats\" got the search hedge. Chronic kidney disease is not a small thing to\nguess at: it is the diagnosis behind a large share of the last years of a cat's life, and the\nfew things that actually help (a renal diet, water, blood pressure, the bloods every few months,\nand taking a cat that stops eating to the vet the same day) are all cheap to say.\n\nThe rows live here rather than in `pets.py` because that file is at the size limit and waiting to\nbe split; nothing about them is specific to this file.\n\nWhat these rows do not take: a person's own kidneys (that is `body.py`), a dog or a rabbit\nasking about cats (each has its own row here, and the cat row stands down on the other animals),\nand a line about what a pet should eat in general, which the feeding rows answer.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("pets")]));
    s__KIDNEY(py_tuple(vec![py_str("chronic kidney disease"), py_str("kidney disease"), py_str("kidney failure"), py_str("renal failure"), py_str("renal disease"), py_str("kidney problem"), py_str("kidney trouble"), py_str("ckd"), py_str("kidneys are failing"), py_str("kidneys failing"), py_str("kidney damage"), py_str("renal")]));
    fn __data1() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("what does"), py_str("mean"), py_str("means"), py_str("how long"), py_str("how often"), py_str("feed"), py_str("food"), py_str("diet"), py_str("eat"), py_str("eating"), py_str("appetite"), py_str("water"), py_str("drink"), py_str("drinking"), py_str("thirsty"), py_str("stage"), py_str("stages"), py_str("treat"), py_str("treated"), py_str("treatment"), py_str("medicine"), py_str("symptoms"), py_str("signs"), py_str("vet"), py_str("manage"), py_str("managing"), py_str("slow"), py_str("slowing"), py_str("worse"), py_str("quality of life"), py_str("put to sleep"), py_str("painful"), py_str("in pain"), py_str("cope"), py_str("help"), py_str("do"), py_str("should i"), py_str("can i"), py_str("is it"), py_str("how do i"), py_str("what can i"), py_str("supplements"), py_str("phosphate"), py_str("blood pressure"), py_str("bloods"), py_str("test"), py_str("tests"), py_str("diagnosed"), py_str("what now"), py_str("next")]) }
    s__ASKING(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("kitty"), py_str("puss"), py_str("moggy"), py_str("tabby"), py_str("tomcat"), py_str("siamese"), py_str("bengal"), py_str("ragdoll"), py_str("persian")]) }
    s__CAT(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("pups"), py_str("bitch"), py_str("collie"), py_str("labrador"), py_str("terrier"), py_str("spaniel"), py_str("retriever"), py_str("jack russell"), py_str("poodle"), py_str("beagle"), py_str("greyhound"), py_str("staffie"), py_str("cocker"), py_str("cavapoo"), py_str("cockapoo")]) }
    s__DOG(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("my dad"), py_str("my mum"), py_str("my mother"), py_str("my father"), py_str("my husband"), py_str("my wife"), py_str("my nan"), py_str("my grandad"), py_str("my grandma"), py_str("my brother"), py_str("my sister"), py_str("my son"), py_str("my daughter"), py_str("i have"), py_str("i've got"), py_str("ive got"), py_str("myself"), py_str("my own kidneys"), py_str("human"), py_str("dialysis"), py_str("transplant"), py_str("nephrotic"), py_str("egfr"), py_str("creatinine"), py_str("my baby"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("child"), py_str("children"), py_str("my son"), py_str("my daughter"), py_str("neonate"), py_str("newborn")]) }
    s__NOT_A_PET(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("rabbit"), py_str("hamster"), py_str("guinea pig"), py_str("ferret"), py_str("parrot"), py_str("fish"), py_str("horse"), py_str("tortoise"), py_str("lizard"), py_str("snake"), py_str("rat"), py_str("mouse"), py_str("chinchilla")]) }
    s__NOT_THE_OTHER_ANIMAL(__data5());
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cat-kidney-disease")); __args.push(py_tuple(vec![g__KIDNEY(), op_add(&g__CAT(), &g__ASKING())])); __args.push(py_str("Chronic kidney disease is the kidneys slowly losing their filtering over months to years: it is the commonest long illness of an older cat, it is found on bloods rather than by looking, and most cats found early live for years with it. A prescription renal diet - wet food lower in phosphate and protein - is the one thing shown to slow it, and any wet food she will eat beats dry. Have the vet check her blood pressure and her urine protein as well as the bloods, since high blood pressure is treatable and causes the sudden blindness. Bloods and urine every three to six months, and her weight once a month at home. Ring the vet this week if she stops eating, loses weight, drinks and wees far more, vomits or starts hiding away, and the same day if she has not eaten for a whole day or her breath smells of ammonia - not eating is what makes them worse fastest.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g__NOT_A_PET(), &g__DOG()), &g__NOT_THE_OTHER_ANIMAL())), ("report".to_string(), Py::Bool(true)), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dog-kidney-disease")); __args.push(py_tuple(vec![g__KIDNEY(), op_add(&g__DOG(), &g__ASKING())])); __args.push(py_str("Chronic kidney disease in a dog is the kidneys losing their filtering slowly, usually found on a blood test in an older dog, and it is managed rather than cured - many dogs live comfortably for a year or two, some much longer. A prescription renal diet is the main thing: lower phosphate and moderate protein, and it works best started early, so ask the vet whether to switch now. Have the blood pressure and the urine protein checked too, because both are treatable and both make it worse; phosphate binders, a potassium supplement and something for sickness are added if needed. Bloods every three to six months, and the weight at home. Ring the vet this week if your dog stops eating, loses weight, is sick, drinks far more or seems dull, and the same day if it has not eaten for a whole day.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g__NOT_A_PET(), &g__CAT()), &g__NOT_THE_OTHER_ANIMAL())), ("report".to_string(), Py::Bool(true)), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.pets_health".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASKING" => Some(g__ASKING()),
            "_CAT" => Some(g__CAT()),
            "_DOG" => Some(g__DOG()),
            "_KIDNEY" => Some(g__KIDNEY()),
            "_NOT_A_PET" => Some(g__NOT_A_PET()),
            "_NOT_THE_OTHER_ANIMAL" => Some(g__NOT_THE_OTHER_ANIMAL()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASKING" => s__ASKING(v),
            "_CAT" => s__CAT(v),
            "_DOG" => s__DOG(v),
            "_KIDNEY" => s__KIDNEY(v),
            "_NOT_A_PET" => s__NOT_A_PET(v),
            "_NOT_THE_OTHER_ANIMAL" => s__NOT_THE_OTHER_ANIMAL(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.pets_health", module_obj);
    pyrt::xrefs::reg_access("knowledge.pets_health", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.pets_health", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.pets_health", "_ASKING", g__ASKING, s__ASKING);
    pyrt::xrefs::reg_access("knowledge.pets_health", "_CAT", g__CAT, s__CAT);
    pyrt::xrefs::reg_access("knowledge.pets_health", "_DOG", g__DOG, s__DOG);
    pyrt::xrefs::reg_access("knowledge.pets_health", "_KIDNEY", g__KIDNEY, s__KIDNEY);
    pyrt::xrefs::reg_access("knowledge.pets_health", "_NOT_A_PET", g__NOT_A_PET, s__NOT_A_PET);
    pyrt::xrefs::reg_access("knowledge.pets_health", "_NOT_THE_OTHER_ANIMAL", g__NOT_THE_OTHER_ANIMAL, s__NOT_THE_OTHER_ANIMAL);
    pyrt::xrefs::reg_access("knowledge.pets_health", "entry", g_entry, s_entry);
}
