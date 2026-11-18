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
thread_local! { static G__ASKING_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COLD_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONSTIPATION_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUGH_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CYSTITIS_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DIARRHOEA_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EARACHE_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAY_FEVER_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEADACHE_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEARTBURN_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IS_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IS_PREGNANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_AN_EARACHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_COUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_HEADACHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_SLEEP_AID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_A_SORE_THROAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_CONSTIPATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_CYSTITIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_DIARRHOEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_HAY_FEVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_HEARTBURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_PERIOD_PAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_THRUSH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_VERTIGO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDICINE_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PRESCRIPTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_SHELF_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_BRAND_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERIOD_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_WORDS_FOR_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREGNANT_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRESCRIBED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRESCRIBED_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SLEEP_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THROAT_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THRUSH_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERTIGO_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__ASKING_FOR_IT() -> Py { ensure_init(); G__ASKING_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING_FOR_IT(v: Py) { ensure_init(); G__ASKING_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_CARD() -> Py { ensure_init(); G__BACK_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_CARD(v: Py) { ensure_init(); G__BACK_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD_CARD() -> Py { ensure_init(); G__CHILD_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD_CARD(v: Py) { ensure_init(); G__CHILD_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__COLD_CARD() -> Py { ensure_init(); G__COLD_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COLD_CARD(v: Py) { ensure_init(); G__COLD_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__CONSTIPATION_CARD() -> Py { ensure_init(); G__CONSTIPATION_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONSTIPATION_CARD(v: Py) { ensure_init(); G__CONSTIPATION_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__COUGH_CARD() -> Py { ensure_init(); G__COUGH_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUGH_CARD(v: Py) { ensure_init(); G__COUGH_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTER() -> Py { ensure_init(); G__COUNTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTER(v: Py) { ensure_init(); G__COUNTER.with(|c| *c.borrow_mut() = v); }
pub fn g__CYSTITIS_CARD() -> Py { ensure_init(); G__CYSTITIS_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CYSTITIS_CARD(v: Py) { ensure_init(); G__CYSTITIS_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__DIARRHOEA_CARD() -> Py { ensure_init(); G__DIARRHOEA_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DIARRHOEA_CARD(v: Py) { ensure_init(); G__DIARRHOEA_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__EARACHE_CARD() -> Py { ensure_init(); G__EARACHE_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EARACHE_CARD(v: Py) { ensure_init(); G__EARACHE_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__HAY_FEVER_CARD() -> Py { ensure_init(); G__HAY_FEVER_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAY_FEVER_CARD(v: Py) { ensure_init(); G__HAY_FEVER_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__HEADACHE_CARD() -> Py { ensure_init(); G__HEADACHE_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEADACHE_CARD(v: Py) { ensure_init(); G__HEADACHE_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__HEARTBURN_CARD() -> Py { ensure_init(); G__HEARTBURN_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEARTBURN_CARD(v: Py) { ensure_init(); G__HEARTBURN_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__IS_A_CHILD() -> Py { ensure_init(); G__IS_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IS_A_CHILD(v: Py) { ensure_init(); G__IS_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__IS_PREGNANT() -> Py { ensure_init(); G__IS_PREGNANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IS_PREGNANT(v: Py) { ensure_init(); G__IS_PREGNANT.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_AN_EARACHE() -> Py { ensure_init(); G__ITS_AN_EARACHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_AN_EARACHE(v: Py) { ensure_init(); G__ITS_AN_EARACHE.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_BACK() -> Py { ensure_init(); G__ITS_A_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_BACK(v: Py) { ensure_init(); G__ITS_A_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_COLD() -> Py { ensure_init(); G__ITS_A_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_COLD(v: Py) { ensure_init(); G__ITS_A_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_COUGH() -> Py { ensure_init(); G__ITS_A_COUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_COUGH(v: Py) { ensure_init(); G__ITS_A_COUGH.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_HEADACHE() -> Py { ensure_init(); G__ITS_A_HEADACHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_HEADACHE(v: Py) { ensure_init(); G__ITS_A_HEADACHE.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_SLEEP_AID() -> Py { ensure_init(); G__ITS_A_SLEEP_AID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_SLEEP_AID(v: Py) { ensure_init(); G__ITS_A_SLEEP_AID.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_A_SORE_THROAT() -> Py { ensure_init(); G__ITS_A_SORE_THROAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_A_SORE_THROAT(v: Py) { ensure_init(); G__ITS_A_SORE_THROAT.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_CONSTIPATION() -> Py { ensure_init(); G__ITS_CONSTIPATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_CONSTIPATION(v: Py) { ensure_init(); G__ITS_CONSTIPATION.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_CYSTITIS() -> Py { ensure_init(); G__ITS_CYSTITIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_CYSTITIS(v: Py) { ensure_init(); G__ITS_CYSTITIS.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_DIARRHOEA() -> Py { ensure_init(); G__ITS_DIARRHOEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_DIARRHOEA(v: Py) { ensure_init(); G__ITS_DIARRHOEA.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_HAY_FEVER() -> Py { ensure_init(); G__ITS_HAY_FEVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_HAY_FEVER(v: Py) { ensure_init(); G__ITS_HAY_FEVER.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_HEARTBURN() -> Py { ensure_init(); G__ITS_HEARTBURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_HEARTBURN(v: Py) { ensure_init(); G__ITS_HEARTBURN.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_PERIOD_PAIN() -> Py { ensure_init(); G__ITS_PERIOD_PAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_PERIOD_PAIN(v: Py) { ensure_init(); G__ITS_PERIOD_PAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_THRUSH() -> Py { ensure_init(); G__ITS_THRUSH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_THRUSH(v: Py) { ensure_init(); G__ITS_THRUSH.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_VERTIGO() -> Py { ensure_init(); G__ITS_VERTIGO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_VERTIGO(v: Py) { ensure_init(); G__ITS_VERTIGO.with(|c| *c.borrow_mut() = v); }
pub fn g__KINDS() -> Py { ensure_init(); G__KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KINDS(v: Py) { ensure_init(); G__KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDICINE_THINGS() -> Py { ensure_init(); G__MEDICINE_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDICINE_THINGS(v: Py) { ensure_init(); G__MEDICINE_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PRESCRIPTION() -> Py { ensure_init(); G__NOT_A_PRESCRIPTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PRESCRIPTION(v: Py) { ensure_init(); G__NOT_A_PRESCRIPTION.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_SHELF_PRICE() -> Py { ensure_init(); G__NOT_A_SHELF_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SHELF_PRICE(v: Py) { ensure_init(); G__NOT_A_SHELF_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_SYMPTOM() -> Py { ensure_init(); G__NOT_A_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SYMPTOM(v: Py) { ensure_init(); G__NOT_A_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_BRAND_CARD() -> Py { ensure_init(); G__OWN_BRAND_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_BRAND_CARD(v: Py) { ensure_init(); G__OWN_BRAND_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__PERIOD_CARD() -> Py { ensure_init(); G__PERIOD_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERIOD_CARD(v: Py) { ensure_init(); G__PERIOD_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__PETS() -> Py { ensure_init(); G__PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PETS(v: Py) { ensure_init(); G__PETS.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_CARD() -> Py { ensure_init(); G__PET_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_CARD(v: Py) { ensure_init(); G__PET_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_WORDS_FOR_AVOID() -> Py { ensure_init(); G__PET_WORDS_FOR_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_WORDS_FOR_AVOID(v: Py) { ensure_init(); G__PET_WORDS_FOR_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__PREGNANT_CARD() -> Py { ensure_init(); G__PREGNANT_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREGNANT_CARD(v: Py) { ensure_init(); G__PREGNANT_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__PRESCRIBED() -> Py { ensure_init(); G__PRESCRIBED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRESCRIBED(v: Py) { ensure_init(); G__PRESCRIBED.with(|c| *c.borrow_mut() = v); }
pub fn g__PRESCRIBED_CARD() -> Py { ensure_init(); G__PRESCRIBED_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRESCRIBED_CARD(v: Py) { ensure_init(); G__PRESCRIBED_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__PRICE_WORDS() -> Py { ensure_init(); G__PRICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRICE_WORDS(v: Py) { ensure_init(); G__PRICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SLEEP_CARD() -> Py { ensure_init(); G__SLEEP_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SLEEP_CARD(v: Py) { ensure_init(); G__SLEEP_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__THROAT_CARD() -> Py { ensure_init(); G__THROAT_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THROAT_CARD(v: Py) { ensure_init(); G__THROAT_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__THRUSH_CARD() -> Py { ensure_init(); G__THRUSH_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THRUSH_CARD(v: Py) { ensure_init(); G__THRUSH_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__VERTIGO_CARD() -> Py { ensure_init(); G__VERTIGO_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERTIGO_CARD(v: Py) { ensure_init(); G__VERTIGO_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_last_user() -> Py { ensure_init(); G_last_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_user(v: Py) { ensure_init(); G_last_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What the pharmacy can give you, asked about a symptom rather than about a product.\n\nA domain of its own because the question is the counter rather than the body: \"can i get\nanything for it over the counter\", \"what can i get from the chemist for a cough\" and \"is there\na tablet for vertigo\" are all asking what is on the shelf and whether they need a prescription,\nand the answer is different for a headache, a back and a bladder infection. Every one of them\nreached nothing, a hand-off to a pharmacist, or the shelf-price shopping card.\n\nThe kind of symptom is read out of the line, or out of the line this one leans on (\"my back has\nbeen hurting\" ... \"can i get anything for it over the counter\"), which is the shape people\nactually type in a chat: the second turn names the symptom as \"it\".\n");
    s_last_user(py_func("last_user", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_user", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    fn __data1() -> Py { py_tuple(vec![py_str("over the counter"), py_str("over-the-counter"), py_str("otc"), py_str("from the chemist"), py_str("at the chemist"), py_str("chemist"), py_str("from a pharmacy"), py_str("at the pharmacy"), py_str("from the pharmacy"), py_str("pharmacy"), py_str("pharmacist"), py_str("off the shelf"), py_str("without a prescription"), py_str("no prescription"), py_str("need a prescription"), py_str("prescription for"), py_str("on prescription"), py_str("ask the chemist"), py_str("ask a pharmacist"), py_str("ask the pharmacist"), py_str("from boots"), py_str("from superdrug"), py_str("shelf"), py_str("tablet for"), py_str("tablets for"), py_str("pill for"), py_str("pills for"), py_str("something for it"), py_str("anything for it"), py_str("medicine for"), py_str("medication for"), py_str("anything that will"), py_str("something that will")]) }
    s__COUNTER(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("can i"), py_str("could i"), py_str("what can i"), py_str("what should i"), py_str("should i"), py_str("is there"), py_str("are there"), py_str("is it ok"), py_str("is it safe"), py_str("anything"), py_str("something"), py_str("what is the best"), py_str("whats the best"), py_str("which"), py_str("do i need"), py_str("will i need"), py_str("do you need"), py_str("buy"), py_str("get"), py_str("take"), py_str("try"), py_str("help"), py_str("work"), py_str("works"), py_str("recommend"), py_str("how much"), py_str("price"), py_str("cost"), py_str("costs"), py_str("more"), py_str("another"), py_str("best"), py_str("should"), py_str("will")]) }
    s__ASKING_FOR_IT(__data2());
    s__ITS_A_HEADACHE(py_tuple(vec![py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("migraines"), py_str("head ache"), py_str("head hurts"), py_str("sore head")]));
    s__ITS_A_BACK(py_tuple(vec![py_str("back pain"), py_str("my back"), py_str("lower back"), py_str("sciatica"), py_str("slipped disc"), py_str("backache"), py_str("back ache"), py_str("back is"), py_str("back has been")]));
    s__ITS_PERIOD_PAIN(py_tuple(vec![py_str("period pain"), py_str("period pains"), py_str("cramps"), py_str("period cramps"), py_str("menstrual"), py_str("period is"), py_str("periods are"), py_str("heavy period"), py_str("pmt")]));
    s__ITS_A_SORE_THROAT(py_tuple(vec![py_str("sore throat"), py_str("throat is sore"), py_str("throat hurts"), py_str("throat"), py_str("tonsils"), py_str("tonsillitis"), py_str("scratchy throat"), py_str("swallowing hurts")]));
    s__ITS_A_COUGH(py_tuple(vec![py_str("cough"), py_str("coughs"), py_str("coughing"), py_str("chesty"), py_str("dry cough"), py_str("barking")]));
    s__ITS_A_COLD(py_tuple(vec![py_str("cold"), py_str("colds"), py_str("blocked nose"), py_str("stuffy nose"), py_str("runny nose"), py_str("sniffles"), py_str("congested"), py_str("congestion"), py_str("flu"), py_str("sinusitis"), py_str("sinus")]));
    s__ITS_HAY_FEVER(py_tuple(vec![py_str("hay fever"), py_str("hayfever"), py_str("allergy"), py_str("allergies"), py_str("allergic"), py_str("pollen"), py_str("sneezing"), py_str("itchy eyes"), py_str("watery eyes")]));
    s__ITS_HEARTBURN(py_tuple(vec![py_str("heartburn"), py_str("indigestion"), py_str("acid reflux"), py_str("reflux"), py_str("acid"), py_str("gerd")]));
    s__ITS_DIARRHOEA(py_tuple(vec![py_str("diarrhoea"), py_str("diarrhea"), py_str("the runs"), py_str("loose stools"), py_str("dioralyte"), py_str("stomach bug"), py_str("sick bug"), py_str("food poisoning")]));
    s__ITS_CONSTIPATION(py_tuple(vec![py_str("constipation"), py_str("constipated"), py_str("cannot poo"), py_str("cant poo"), py_str("hard stools"), py_str("laxative"), py_str("laxatives")]));
    s__ITS_THRUSH(py_tuple(vec![py_str("thrush"), py_str("yeast infection"), py_str("candida"), py_str("itchy down there"), py_str("white discharge")]));
    s__ITS_VERTIGO(py_tuple(vec![py_str("vertigo"), py_str("dizzy"), py_str("dizziness"), py_str("spinning"), py_str("light headed"), py_str("lightheaded"), py_str("bppv")]));
    s__ITS_A_SLEEP_AID(py_tuple(vec![py_str("sleep aid"), py_str("sleeping tablet"), py_str("sleeping tablets"), py_str("cant sleep"), py_str("cannot sleep"), py_str("insomnia"), py_str("sleeping pill"), py_str("sleeping pills"), py_str("help me sleep")]));
    s__ITS_CYSTITIS(py_tuple(vec![py_str("cystitis"), py_str("uti"), py_str("urine infection"), py_str("urinary infection"), py_str("when i pee"), py_str("when i wee"), py_str("stings"), py_str("stinging"), py_str("burning when i"), py_str("water infection")]));
    s__ITS_AN_EARACHE(py_tuple(vec![py_str("earache"), py_str("ear ache"), py_str("ear infection"), py_str("ear is"), py_str("ear hurts"), py_str("sore ear")]));
    s__HEADACHE_CARD(py_str("Paracetamol or ibuprofen is what the counter has for a headache: take the dose on the pack with water and something to eat, and one painkiller at a time to start. Taken more than a couple of days a week for months, either can cause headaches of its own. A headache that came on in seconds, is the worst you have ever had, or comes with a fever and a stiff neck, weakness, confusion or a change in vision is a 999 call, not a tablet."));
    s__BACK_CARD(py_str("Ibuprofen is what a pharmacist will suggest for a strained back, with paracetamol alongside if you need it, and a heat pad; keep moving, because rest makes it stiffer. Say yes to their questions about your stomach, kidneys, heart and blood thinners, as ibuprofen is not for everyone. Numbness between your legs, a weak leg, or trouble passing water is a same-day GP appointment."));
    s__PERIOD_CARD(py_str("Ibuprofen works better than paracetamol for period pain, and starting it the day before helps; the pharmacist can add codeine (co-codamol) or suggest mefenamic acid if that is not enough. A heat pad and moving about ease it too. Pain that stops you living normally, or bleeding that soaks a pad an hour, is worth a GP appointment."));
    s__THROAT_CARD(py_str("Paracetamol or ibuprofen, plus a medicated lozenge, is what the counter has for a sore throat, and most settle within a week without anything else. Ask about a spray with a local anaesthetic if swallowing is the worst part. See a GP if you cannot swallow fluids, cannot open your mouth wide, or it lasts more than two weeks."));
    s__COUGH_CARD(py_str("For a cough the pharmacist has honey-based syrups and simple linctus; the evidence for cough medicines is weak, and honey in warm water does about as well, with paracetamol or ibuprofen for the aches. Ask about a cough medicine for a night cough only - a chesty one is better left to clear. A cough with breathlessness, blood, chest pain, a fever that will not go, or more than three weeks needs a doctor."));
    s__COLD_CARD(py_str("Nothing cures a cold, so the counter is for the symptoms: paracetamol or ibuprofen for aches and fever, a decongestant spray or tablet for a blocked nose, and honey for the cough. Decongestant tablets are not for you if you have high blood pressure, heart trouble or take an MAOI, so say so at the counter. Most clear in a week to ten days; a cold getting worse after a week, or a fever with breathlessness, needs a GP."));
    s__HAY_FEVER_CARD(py_str("A once-a-day antihistamine - loratadine, cetirizine or the pharmacy's own brand, which is the same tablet for less - is the first thing to try for hay fever, and a steroid nose spray or eye drops can be added if the nose and eyes are where it is. The spray works best started a week or two before your season. Wheezing or a tight chest with it is a GP question."));
    s__HEARTBURN_CARD(py_str("An antacid or an alginate (Gaviscon and the like) settles heartburn in minutes, and a 14-day course of a tablet such as omeprazole is the next step if it keeps coming back - the pharmacist can suggest both. Losing a little weight, smaller meals and nothing to eat in the three hours before bed do the rest. Trouble swallowing, weight loss or pain on swallowing is a GP appointment rather than a repeat bottle."));
    s__DIARRHOEA_CARD(py_str("The counter has oral rehydration sachets (Dioralyte) and, if you have to travel, loperamide; the fluid is the part that matters, and most of it settles in two to three days. Keep eating plain food and stay off dairy and alcohol until it does. Blood in it, a high fever, or not keeping any water down needs a GP - and 111 if you are frail, pregnant or very young."));
    s__CONSTIPATION_CARD(py_str("Start with a bulk-forming laxative such as ispaghula (Fybogel) or a stool softener, with more water and fibre, and give it a few days - the pharmacist can add a short course of a stimulant laxative if nothing has moved. Ask before using them regularly, as the bowels get used to it. Constipation that comes with bleeding, weight loss, or a change that lasts weeks is worth a GP look."));
    s__THRUSH_CARD(py_str("Thrush is an itch and a thick white discharge, and a pharmacist can treat it without a prescription: a clotrimazole pessary or cream, or a single fluconazole tablet if you are not pregnant and not on certain medicines - say what else you take. The itch eases within a few days, and the whole course still needs finishing. See a GP if it is the first time you have had it, it keeps returning, or there is bleeding, pain or a smelly discharge. Cotton rather than lace, nothing perfumed down there, and no douching, is what stops it coming back."));
    s__VERTIGO_CARD(py_str("For the common spinning kind, the fix is the Epley manoeuvre rather than a tablet - a GP or physio can do it in one visit and you can learn it - while the counter can give prochlorperazine (Stemetil) for a few days to soften the spinning. It makes you drowsy, so no driving on it. Vertigo with double vision, slurred speech, weakness down one side, or a sudden bad headache is a 999 call."));
    s__SLEEP_CARD(py_str("The counter has a short course of a sedating antihistamine (diphenhydramine, promethazine) for a few nights, and melatonin on prescription or from a pharmacist in some places; all of them leave you groggy and none is a long-term answer. The routine does more: the same wake-up time, no screens or caffeine late, and a cool dark room. Ask before mixing any of it with alcohol, antidepressants or other sedatives; weeks of broken sleep is a GP conversation."));
    s__CYSTITIS_CARD(py_str("A pharmacist can treat a simple bladder infection in women aged 16 to 64 with a short course of an antibiotic (trimethoprim or nitrofurantoin) - ask at the counter, as many pharmacies run the scheme. Drink plenty and take paracetamol for the burn. Blood in the urine, a fever, pain in your back or side, being pregnant, or being a man with these symptoms means a GP the same day."));
    s__EARACHE_CARD(py_str("Paracetamol or ibuprofen is the counter's answer for earache, and most in adults settle in a few days as the cold behind it clears; ask about drops only if the ear itself is itchy or weeping, as drops do not help a middle-ear infection. Nothing down the ear canal, no cotton buds. Discharge, a high fever, or a child under two with earache is a GP appointment."));
    fn __data3() -> Py { py_tuple(vec![py_tuple(vec![g__ITS_A_HEADACHE(), g__HEADACHE_CARD()]), py_tuple(vec![g__ITS_A_BACK(), g__BACK_CARD()]), py_tuple(vec![g__ITS_PERIOD_PAIN(), g__PERIOD_CARD()]), py_tuple(vec![g__ITS_A_SORE_THROAT(), g__THROAT_CARD()]), py_tuple(vec![g__ITS_A_COUGH(), g__COUGH_CARD()]), py_tuple(vec![g__ITS_A_COLD(), g__COLD_CARD()]), py_tuple(vec![g__ITS_HAY_FEVER(), g__HAY_FEVER_CARD()]), py_tuple(vec![g__ITS_HEARTBURN(), g__HEARTBURN_CARD()]), py_tuple(vec![g__ITS_DIARRHOEA(), g__DIARRHOEA_CARD()]), py_tuple(vec![g__ITS_CONSTIPATION(), g__CONSTIPATION_CARD()]), py_tuple(vec![g__ITS_THRUSH(), g__THRUSH_CARD()]), py_tuple(vec![g__ITS_VERTIGO(), g__VERTIGO_CARD()]), py_tuple(vec![g__ITS_A_SLEEP_AID(), g__SLEEP_CARD()]), py_tuple(vec![g__ITS_CYSTITIS(), g__CYSTITIS_CARD()]), py_tuple(vec![g__ITS_AN_EARACHE(), g__EARACHE_CARD()])]) }
    s__KINDS(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my kitten"), py_str("my rabbit"), py_str("my horse"), py_str("my pet"), py_str("my hamster"), py_str("the dog"), py_str("the cat"), py_str("cold tap"), py_str("cold water"), py_str("cold weather"), py_str("its cold"), py_str("it is cold"), py_str("freezing"), py_str("the fridge"), py_str("the freezer"), py_str("the heating"), py_str("the boiler"), py_str("cold sore"), py_str("cold call"), py_str("cold caller"), py_str("cold email"), py_str("in cold blood")]) }
    s__NOT_A_SYMPTOM(__data4());
    s__OWN_BRAND_CARD(py_str("The own-brand box is the same medicine for less: match the active ingredient and the dose on the back, and a pharmacy's own hay fever or painkiller tablet is usually a fraction of the branded price. Ask the pharmacist to put the two boxes side by side if the names confuse - the licence number and the ingredient are the parts that must match, not the box. The one place it matters is a medicine you take for something serious, where the brand can change how much you absorb."));
    s__PRESCRIBED_CARD(py_str("It cannot be bought over the counter - a prescription-only medicine is not sold off the shelf whatever the box looks like - but ordering more is a different question, and most pharmacies will send the repeat request to the surgery for you and have it ready in two or three days. Ask at the counter, and order about a week before you run out. If the course did not work, say so when you order: the same thing again is often not what the doctor would choose."));
    s__PET_CARD(py_str("Nothing from the chemist - a pharmacy's shelves are dosed for people, and a dog or cat can be poisoned by what is ordinary for us (paracetamol and ibuprofen especially, and some decongestants). Ring the vet for anything that lasts more than a day or two, and keep every medicine you buy out of reach, child locks and all. The one thing a pharmacy can do is tell you what is poisonous, so keep the box to read out."));
    fn __data5() -> Py { py_tuple(vec![py_str("how much"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("cheaper"), py_str("cheapest"), py_str("own brand"), py_str("own-brand"), py_str("brand name"), py_str("branded"), py_str("much does"), py_str("much are"), py_str("much is")]) }
    s__PRICE_WORDS(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("tablet"), py_str("tablets"), py_str("pill"), py_str("pills"), py_str("medicine"), py_str("medicines"), py_str("painkiller"), py_str("painkillers"), py_str("antihistamine"), py_str("antihistamines"), py_str("syrup"), py_str("cream"), py_str("drops"), py_str("sachets"), py_str("spray"), py_str("inhaler"), py_str("plasters")]) }
    s__MEDICINE_THINGS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my kitten"), py_str("my rabbit"), py_str("my hamster"), py_str("my horse"), py_str("my budgie"), py_str("my parrot"), py_str("my guinea pig"), py_str("my pet"), py_str("the dog"), py_str("the cat"), py_str("our dog"), py_str("our cat"), py_str("the puppy"), py_str("the kitten")]) }
    s__PETS(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("antibiotic"), py_str("antibiotics"), py_str("prescription"), py_str("prescribed"), py_str("on repeat"), py_str("repeat prescription"), py_str("the tablets i was given"), py_str("what the doctor gave me"), py_str("inhaler"), py_str("insulin"), py_str("epipen"), py_str("epipens"), py_str("statins"), py_str("blood pressure tablets")]) }
    s__PRESCRIBED(__data8());
    s__IS_PREGNANT(py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting a baby"), py_str("20 weeks"), py_str("12 weeks"), py_str("8 weeks"), py_str("six months pregnant"), py_str("breastfeeding"), py_str("nursing"), py_str("im feeding")]));
    fn __data9() -> Py { py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my baby"), py_str("my toddler"), py_str("my little one"), py_str("the kids"), py_str("my kids"), py_str("year old"), py_str("year-old"), py_str("months old"), py_str("my boy"), py_str("my girl"), py_str("my grandson"), py_str("my granddaughter"), py_str("my niece"), py_str("my nephew")]) }
    s__IS_A_CHILD(__data9());
    s__PREGNANT_CARD(py_str("Paracetamol is the painkiller to take in pregnancy - the lowest dose that works, for the shortest time - and ibuprofen and aspirin are the ones to leave alone unless a doctor prescribes them, especially after 20 weeks. Ask the pharmacist to check any other box against the pregnancy, including cold and flu remedies, because several carry a second ingredient that is not advised; a headache that will not shift, or one that comes with swelling, blurred vision or pain under the ribs, is a same-day call to the midwife."));
    s__CHILD_CARD(py_str("For a child the pharmacist will weigh the dose by age, so ask for children's paracetamol or ibuprofen and give it by the spoon or syringe in the box, never a kitchen spoon. Honey helps a cough over the age of one, and cough and cold medicines are not licensed for children under six, so skip those. A baby under three months with a fever, or a child who is limp, breathing fast or not drinking, is a same-day doctor rather than the counter."));
    fn __data10() -> Py { py_tuple(vec![py_str("morning after"), py_str("morning-after"), py_str("emergency contraception"), py_str("plan b"), py_str("levonelle"), py_str("ellaone"), py_str("ella one"), py_str("ulipristal"), py_str("contraception"), py_str("contraceptive"), py_str("the coil"), py_str("copper coil"), py_str("iud"), py_str("implant"), py_str("condoms")]) }
    s__NOT_A_SHELF_PRICE(__data10());
    s__NOT_A_PRESCRIPTION(py_tuple(vec![py_str("without a prescription"), py_str("without prescription"), py_str("no prescription"), py_str("not on prescription"), py_str("dont need a prescription"), py_str("don't need a prescription"), py_str("not prescribed")]));
    s__PET_WORDS_FOR_AVOID(g__PETS());
    fn __data11() -> Py { py_tuple(vec![py_str("more"), py_str("another"), py_str("again"), py_str("repeat"), py_str("top up"), py_str("buy one"), py_str("buy some"), py_str("buy it"), py_str("get one"), py_str("get some"), py_str("one"), py_str("some"), py_str("buy"), py_str("get"), py_str("take"), py_str("order")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("at-the-counter")); __args.push(py_tuple(vec![g__COUNTER(), g__ASKING_FOR_IT()])); __args.push(py_func("_at_the_counter", |args: &[Py]| _at_the_counter(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g__NOT_A_SYMPTOM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-pet-at-the-counter")); __args.push(py_tuple(vec![g__COUNTER(), g__ASKING_FOR_IT()])); __args.push(py_func("_a_pet_at_the_counter", |args: &[Py]| _a_pet_at_the_counter(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the-own-brand")); __args.push(py_tuple(vec![g__PRICE_WORDS(), g__MEDICINE_THINGS()])); __args.push(py_func("_the_own_brand", |args: &[Py]| _the_own_brand(args))); __kws.extend(vec![("guard".to_string(), py_func("_a_price_was_asked", |args: &[Py]| _a_price_was_asked(args))), ("bias".to_string(), Py::Int(7))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-prescribed-course")); __args.push(py_tuple(vec![g__COUNTER(), __data11()])); __args.push(py_func("_the_prescribed_course", |args: &[Py]| _the_prescribed_course(args))); __kws.extend(vec![("bias".to_string(), Py::Int(5))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _words_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut leans: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line and the line it leans on, as one lower-case string.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean")])) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[part.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    leans = mcall(&reading.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("subject"), py_str("topic")])) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[part.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "split", &[])]), "lower", &[]);
    Py::None
}
pub fn _the_earlier(args: &[Py]) -> Py {
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
    let _ = py_str("The last thing the person said, for the facts a chat carries from turn to turn.");
    earlier = if py_truthy(&Py::Bool(!py_is(&memory.clone(), &Py::None))) { pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]) } else { py_str("") };
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = earlier.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    Py::None
}
pub fn _the_kind_in(args: &[Py]) -> Py {
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
    let mut card: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The card for the symptom named in this text, or None when it names none of them.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_SYMPTOM()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    card = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return card.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The symptom in the line, or - when the line names none - in the turn before it.\n\n    \"i might buy one from the chemist\" after a chat about a cough is about the cough, and the\n    reading would not lean on it: the counter words are a shopping topic to the understanding\n    engine. The line's own words win, so a new symptom is never answered with the old one.\n    ");
    return { let __b = _the_kind_in(&[_words_of(&[reading.clone()])]); if py_truthy(&__b) { __b } else { _the_kind_in(&[_the_earlier(&[memory.clone()])]) } };
    Py::None
}
pub fn _a_person_the_shelf_treats_differently(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The card for a pregnancy or a child, or None when neither is named.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__IS_PREGNANT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__PREGNANT_CARD();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__IS_A_CHILD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__CHILD_CARD();
    }
    return Py::None;
    Py::None
}
pub fn _at_the_counter(args: &[Py]) -> Py {
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
    let mut different: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("What the pharmacy has for this symptom, in the card for that symptom.\n\n    A pregnancy or a child is read first, because the adult card is not the safe answer for\n    either: it opens on ibuprofen, which is not a pregnancy painkiller, and on a cough medicine\n    that is not licensed for a six-year-old. The symptom comes from the line, or from the turn\n    before it when the line names none, and a line about neither declines so the next entry can\n    have it.\n    ");
    said = _words_of(&[reading.clone()]);
    earlier = _the_earlier(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { _names_any(&[op_add(&op_add(&said.clone(), &py_str(" ")), &earlier.clone()), g__NOT_A_SYMPTOM()]) } }) {
    return Py::None;
    }
    different = _a_person_the_shelf_treats_differently(&[op_add(&op_add(&said.clone(), &py_str(" ")), &earlier.clone())]);
    if py_truthy(&different.clone()) {
    return different.clone();
    }
    return { let __b = _the_kind_in(&[said.clone()]); if py_truthy(&__b) { __b } else { _the_kind_in(&[earlier.clone()]) } };
    Py::None
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
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("The line and the one it leans on, lower case, as one string.");
    return _words_of(&[reading.clone()]);
    Py::None
}
pub fn _names_any(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_own_brand(args: &[Py]) -> Py {
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
    return g__OWN_BRAND_CARD();
    Py::None
}
pub fn _a_price_was_asked(args: &[Py]) -> Py {
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
    let _ = py_str("A price for a medicine on the shelf, not for a course that was prescribed.");
    said = _said(&[understood.clone()]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = _names_any(&[said.clone(), g__MEDICINE_THINGS()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_names_any(&[said.clone(), g__PRESCRIBED()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_names_any(&[said.clone(), g__NOT_A_SHELF_PRICE()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_any(&[said.clone(), g__PET_WORDS_FOR_AVOID()]))) } } } } } } } };
    Py::None
}
pub fn _the_prescribed_course(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The repeat card, for a line that is about something already on a prescription.");
    said = op_add(&op_add(&_words_of(&[reading.clone()]), &py_str(" ")), &_the_earlier(&[memory.clone()]));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_any(&[said.clone(), g__PRESCRIBED()]))) } }) {
    return Py::None;
    }
    if py_truthy(&_names_any(&[said.clone(), g__NOT_A_PRESCRIPTION()])) {
    return Py::None;
    }
    return g__PRESCRIBED_CARD();
    Py::None
}
pub fn _a_pet_at_the_counter(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The pet card when an animal is in the line or in the turn before it, else None.\n\n    The animal is read here rather than in a guard because a guard is handed only the reading:\n    \"my cat has a runny eye\" then \"is there anything from the chemist\" is a question about the\n    cat, and the understanding engine does not always say the second line leans on the first -\n    when it does not, a guard sees a line with no animal in it and the row never fires.\n    ");
    said = op_add(&op_add(&_words_of(&[reading.clone()]), &py_str(" ")), &_the_earlier(&[memory.clone()]));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_any(&[said.clone(), g__PETS()]))) } }) {
    return Py::None;
    }
    return g__PET_CARD();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.pharmacy".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASKING_FOR_IT" => Some(g__ASKING_FOR_IT()),
            "_BACK_CARD" => Some(g__BACK_CARD()),
            "_CHILD_CARD" => Some(g__CHILD_CARD()),
            "_COLD_CARD" => Some(g__COLD_CARD()),
            "_CONSTIPATION_CARD" => Some(g__CONSTIPATION_CARD()),
            "_COUGH_CARD" => Some(g__COUGH_CARD()),
            "_COUNTER" => Some(g__COUNTER()),
            "_CYSTITIS_CARD" => Some(g__CYSTITIS_CARD()),
            "_DIARRHOEA_CARD" => Some(g__DIARRHOEA_CARD()),
            "_EARACHE_CARD" => Some(g__EARACHE_CARD()),
            "_HAY_FEVER_CARD" => Some(g__HAY_FEVER_CARD()),
            "_HEADACHE_CARD" => Some(g__HEADACHE_CARD()),
            "_HEARTBURN_CARD" => Some(g__HEARTBURN_CARD()),
            "_IS_A_CHILD" => Some(g__IS_A_CHILD()),
            "_IS_PREGNANT" => Some(g__IS_PREGNANT()),
            "_ITS_AN_EARACHE" => Some(g__ITS_AN_EARACHE()),
            "_ITS_A_BACK" => Some(g__ITS_A_BACK()),
            "_ITS_A_COLD" => Some(g__ITS_A_COLD()),
            "_ITS_A_COUGH" => Some(g__ITS_A_COUGH()),
            "_ITS_A_HEADACHE" => Some(g__ITS_A_HEADACHE()),
            "_ITS_A_SLEEP_AID" => Some(g__ITS_A_SLEEP_AID()),
            "_ITS_A_SORE_THROAT" => Some(g__ITS_A_SORE_THROAT()),
            "_ITS_CONSTIPATION" => Some(g__ITS_CONSTIPATION()),
            "_ITS_CYSTITIS" => Some(g__ITS_CYSTITIS()),
            "_ITS_DIARRHOEA" => Some(g__ITS_DIARRHOEA()),
            "_ITS_HAY_FEVER" => Some(g__ITS_HAY_FEVER()),
            "_ITS_HEARTBURN" => Some(g__ITS_HEARTBURN()),
            "_ITS_PERIOD_PAIN" => Some(g__ITS_PERIOD_PAIN()),
            "_ITS_THRUSH" => Some(g__ITS_THRUSH()),
            "_ITS_VERTIGO" => Some(g__ITS_VERTIGO()),
            "_KINDS" => Some(g__KINDS()),
            "_MEDICINE_THINGS" => Some(g__MEDICINE_THINGS()),
            "_NOT_A_PRESCRIPTION" => Some(g__NOT_A_PRESCRIPTION()),
            "_NOT_A_SHELF_PRICE" => Some(g__NOT_A_SHELF_PRICE()),
            "_NOT_A_SYMPTOM" => Some(g__NOT_A_SYMPTOM()),
            "_OWN_BRAND_CARD" => Some(g__OWN_BRAND_CARD()),
            "_PERIOD_CARD" => Some(g__PERIOD_CARD()),
            "_PETS" => Some(g__PETS()),
            "_PET_CARD" => Some(g__PET_CARD()),
            "_PET_WORDS_FOR_AVOID" => Some(g__PET_WORDS_FOR_AVOID()),
            "_PREGNANT_CARD" => Some(g__PREGNANT_CARD()),
            "_PRESCRIBED" => Some(g__PRESCRIBED()),
            "_PRESCRIBED_CARD" => Some(g__PRESCRIBED_CARD()),
            "_PRICE_WORDS" => Some(g__PRICE_WORDS()),
            "_SLEEP_CARD" => Some(g__SLEEP_CARD()),
            "_THROAT_CARD" => Some(g__THROAT_CARD()),
            "_THRUSH_CARD" => Some(g__THRUSH_CARD()),
            "_VERTIGO_CARD" => Some(g__VERTIGO_CARD()),
            "_a_person_the_shelf_treats_differently" => Some(py_func("_a_person_the_shelf_treats_differently", |args: &[Py]| _a_person_the_shelf_treats_differently(args))),
            "_a_pet_at_the_counter" => Some(py_func("_a_pet_at_the_counter", |args: &[Py]| _a_pet_at_the_counter(args))),
            "_a_price_was_asked" => Some(py_func("_a_price_was_asked", |args: &[Py]| _a_price_was_asked(args))),
            "_at_the_counter" => Some(py_func("_at_the_counter", |args: &[Py]| _at_the_counter(args))),
            "_names_any" => Some(py_func("_names_any", |args: &[Py]| _names_any(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_the_earlier" => Some(py_func("_the_earlier", |args: &[Py]| _the_earlier(args))),
            "_the_kind" => Some(py_func("_the_kind", |args: &[Py]| _the_kind(args))),
            "_the_kind_in" => Some(py_func("_the_kind_in", |args: &[Py]| _the_kind_in(args))),
            "_the_own_brand" => Some(py_func("_the_own_brand", |args: &[Py]| _the_own_brand(args))),
            "_the_prescribed_course" => Some(py_func("_the_prescribed_course", |args: &[Py]| _the_prescribed_course(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            "entry" => Some(g_entry()),
            "last_user" => Some(g_last_user()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASKING_FOR_IT" => s__ASKING_FOR_IT(v),
            "_BACK_CARD" => s__BACK_CARD(v),
            "_CHILD_CARD" => s__CHILD_CARD(v),
            "_COLD_CARD" => s__COLD_CARD(v),
            "_CONSTIPATION_CARD" => s__CONSTIPATION_CARD(v),
            "_COUGH_CARD" => s__COUGH_CARD(v),
            "_COUNTER" => s__COUNTER(v),
            "_CYSTITIS_CARD" => s__CYSTITIS_CARD(v),
            "_DIARRHOEA_CARD" => s__DIARRHOEA_CARD(v),
            "_EARACHE_CARD" => s__EARACHE_CARD(v),
            "_HAY_FEVER_CARD" => s__HAY_FEVER_CARD(v),
            "_HEADACHE_CARD" => s__HEADACHE_CARD(v),
            "_HEARTBURN_CARD" => s__HEARTBURN_CARD(v),
            "_IS_A_CHILD" => s__IS_A_CHILD(v),
            "_IS_PREGNANT" => s__IS_PREGNANT(v),
            "_ITS_AN_EARACHE" => s__ITS_AN_EARACHE(v),
            "_ITS_A_BACK" => s__ITS_A_BACK(v),
            "_ITS_A_COLD" => s__ITS_A_COLD(v),
            "_ITS_A_COUGH" => s__ITS_A_COUGH(v),
            "_ITS_A_HEADACHE" => s__ITS_A_HEADACHE(v),
            "_ITS_A_SLEEP_AID" => s__ITS_A_SLEEP_AID(v),
            "_ITS_A_SORE_THROAT" => s__ITS_A_SORE_THROAT(v),
            "_ITS_CONSTIPATION" => s__ITS_CONSTIPATION(v),
            "_ITS_CYSTITIS" => s__ITS_CYSTITIS(v),
            "_ITS_DIARRHOEA" => s__ITS_DIARRHOEA(v),
            "_ITS_HAY_FEVER" => s__ITS_HAY_FEVER(v),
            "_ITS_HEARTBURN" => s__ITS_HEARTBURN(v),
            "_ITS_PERIOD_PAIN" => s__ITS_PERIOD_PAIN(v),
            "_ITS_THRUSH" => s__ITS_THRUSH(v),
            "_ITS_VERTIGO" => s__ITS_VERTIGO(v),
            "_KINDS" => s__KINDS(v),
            "_MEDICINE_THINGS" => s__MEDICINE_THINGS(v),
            "_NOT_A_PRESCRIPTION" => s__NOT_A_PRESCRIPTION(v),
            "_NOT_A_SHELF_PRICE" => s__NOT_A_SHELF_PRICE(v),
            "_NOT_A_SYMPTOM" => s__NOT_A_SYMPTOM(v),
            "_OWN_BRAND_CARD" => s__OWN_BRAND_CARD(v),
            "_PERIOD_CARD" => s__PERIOD_CARD(v),
            "_PETS" => s__PETS(v),
            "_PET_CARD" => s__PET_CARD(v),
            "_PET_WORDS_FOR_AVOID" => s__PET_WORDS_FOR_AVOID(v),
            "_PREGNANT_CARD" => s__PREGNANT_CARD(v),
            "_PRESCRIBED" => s__PRESCRIBED(v),
            "_PRESCRIBED_CARD" => s__PRESCRIBED_CARD(v),
            "_PRICE_WORDS" => s__PRICE_WORDS(v),
            "_SLEEP_CARD" => s__SLEEP_CARD(v),
            "_THROAT_CARD" => s__THROAT_CARD(v),
            "_THRUSH_CARD" => s__THRUSH_CARD(v),
            "_VERTIGO_CARD" => s__VERTIGO_CARD(v),
            "entry" => s_entry(v),
            "last_user" => s_last_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.pharmacy", module_obj);
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_words_of", py_func("_words_of", |args: &[Py]| _words_of(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_the_earlier", py_func("_the_earlier", |args: &[Py]| _the_earlier(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_the_kind_in", py_func("_the_kind_in", |args: &[Py]| _the_kind_in(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_the_kind", py_func("_the_kind", |args: &[Py]| _the_kind(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_a_person_the_shelf_treats_differently", py_func("_a_person_the_shelf_treats_differently", |args: &[Py]| _a_person_the_shelf_treats_differently(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_at_the_counter", py_func("_at_the_counter", |args: &[Py]| _at_the_counter(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_names_any", py_func("_names_any", |args: &[Py]| _names_any(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_the_own_brand", py_func("_the_own_brand", |args: &[Py]| _the_own_brand(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_a_price_was_asked", py_func("_a_price_was_asked", |args: &[Py]| _a_price_was_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_the_prescribed_course", py_func("_the_prescribed_course", |args: &[Py]| _the_prescribed_course(args)));
    pyrt::xrefs::reg_fn("knowledge.pharmacy", "_a_pet_at_the_counter", py_func("_a_pet_at_the_counter", |args: &[Py]| _a_pet_at_the_counter(args)));
    pyrt::xrefs::reg_access("knowledge.pharmacy", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ASKING_FOR_IT", g__ASKING_FOR_IT, s__ASKING_FOR_IT);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_BACK_CARD", g__BACK_CARD, s__BACK_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_CHILD_CARD", g__CHILD_CARD, s__CHILD_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_COLD_CARD", g__COLD_CARD, s__COLD_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_CONSTIPATION_CARD", g__CONSTIPATION_CARD, s__CONSTIPATION_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_COUGH_CARD", g__COUGH_CARD, s__COUGH_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_COUNTER", g__COUNTER, s__COUNTER);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_CYSTITIS_CARD", g__CYSTITIS_CARD, s__CYSTITIS_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_DIARRHOEA_CARD", g__DIARRHOEA_CARD, s__DIARRHOEA_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_EARACHE_CARD", g__EARACHE_CARD, s__EARACHE_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_HAY_FEVER_CARD", g__HAY_FEVER_CARD, s__HAY_FEVER_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_HEADACHE_CARD", g__HEADACHE_CARD, s__HEADACHE_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_HEARTBURN_CARD", g__HEARTBURN_CARD, s__HEARTBURN_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_IS_A_CHILD", g__IS_A_CHILD, s__IS_A_CHILD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_IS_PREGNANT", g__IS_PREGNANT, s__IS_PREGNANT);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_AN_EARACHE", g__ITS_AN_EARACHE, s__ITS_AN_EARACHE);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_A_BACK", g__ITS_A_BACK, s__ITS_A_BACK);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_A_COLD", g__ITS_A_COLD, s__ITS_A_COLD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_A_COUGH", g__ITS_A_COUGH, s__ITS_A_COUGH);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_A_HEADACHE", g__ITS_A_HEADACHE, s__ITS_A_HEADACHE);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_A_SLEEP_AID", g__ITS_A_SLEEP_AID, s__ITS_A_SLEEP_AID);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_A_SORE_THROAT", g__ITS_A_SORE_THROAT, s__ITS_A_SORE_THROAT);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_CONSTIPATION", g__ITS_CONSTIPATION, s__ITS_CONSTIPATION);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_CYSTITIS", g__ITS_CYSTITIS, s__ITS_CYSTITIS);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_DIARRHOEA", g__ITS_DIARRHOEA, s__ITS_DIARRHOEA);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_HAY_FEVER", g__ITS_HAY_FEVER, s__ITS_HAY_FEVER);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_HEARTBURN", g__ITS_HEARTBURN, s__ITS_HEARTBURN);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_PERIOD_PAIN", g__ITS_PERIOD_PAIN, s__ITS_PERIOD_PAIN);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_THRUSH", g__ITS_THRUSH, s__ITS_THRUSH);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_ITS_VERTIGO", g__ITS_VERTIGO, s__ITS_VERTIGO);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_KINDS", g__KINDS, s__KINDS);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_MEDICINE_THINGS", g__MEDICINE_THINGS, s__MEDICINE_THINGS);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_NOT_A_PRESCRIPTION", g__NOT_A_PRESCRIPTION, s__NOT_A_PRESCRIPTION);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_NOT_A_SHELF_PRICE", g__NOT_A_SHELF_PRICE, s__NOT_A_SHELF_PRICE);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_NOT_A_SYMPTOM", g__NOT_A_SYMPTOM, s__NOT_A_SYMPTOM);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_OWN_BRAND_CARD", g__OWN_BRAND_CARD, s__OWN_BRAND_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PERIOD_CARD", g__PERIOD_CARD, s__PERIOD_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PETS", g__PETS, s__PETS);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PET_CARD", g__PET_CARD, s__PET_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PET_WORDS_FOR_AVOID", g__PET_WORDS_FOR_AVOID, s__PET_WORDS_FOR_AVOID);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PREGNANT_CARD", g__PREGNANT_CARD, s__PREGNANT_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PRESCRIBED", g__PRESCRIBED, s__PRESCRIBED);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PRESCRIBED_CARD", g__PRESCRIBED_CARD, s__PRESCRIBED_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_PRICE_WORDS", g__PRICE_WORDS, s__PRICE_WORDS);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_SLEEP_CARD", g__SLEEP_CARD, s__SLEEP_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_THROAT_CARD", g__THROAT_CARD, s__THROAT_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_THRUSH_CARD", g__THRUSH_CARD, s__THRUSH_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "_VERTIGO_CARD", g__VERTIGO_CARD, s__VERTIGO_CARD);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.pharmacy", "last_user", g_last_user, s_last_user);
}
