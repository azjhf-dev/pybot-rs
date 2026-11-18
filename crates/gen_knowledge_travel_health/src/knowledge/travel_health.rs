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
thread_local! { static G__ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_NEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TRAVEL_CLINIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHECK_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JABS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JABS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MALARIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MALARIA_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MALARIA_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDICINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDICINES_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDICINES_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER_ONES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROUTINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROUTINE_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_REST_OF_THE_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN_TO_BOOK_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YELLOW_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YELLOW_FEVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YELLOW_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__ANIMAL() -> Py { ensure_init(); G__ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANIMAL(v: Py) { ensure_init(); G__ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g__AWAY() -> Py { ensure_init(); G__AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWAY(v: Py) { ensure_init(); G__AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_NEED() -> Py { ensure_init(); G__A_BARE_NEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_NEED(v: Py) { ensure_init(); G__A_BARE_NEED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TRAVEL_CLINIC() -> Py { ensure_init(); G__A_TRAVEL_CLINIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TRAVEL_CLINIC(v: Py) { ensure_init(); G__A_TRAVEL_CLINIC.with(|c| *c.borrow_mut() = v); }
pub fn g__CHECK_IT() -> Py { ensure_init(); G__CHECK_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHECK_IT(v: Py) { ensure_init(); G__CHECK_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__JABS() -> Py { ensure_init(); G__JABS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JABS(v: Py) { ensure_init(); G__JABS.with(|c| *c.borrow_mut() = v); }
pub fn g__JABS_ASK() -> Py { ensure_init(); G__JABS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JABS_ASK(v: Py) { ensure_init(); G__JABS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__MALARIA() -> Py { ensure_init(); G__MALARIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MALARIA(v: Py) { ensure_init(); G__MALARIA.with(|c| *c.borrow_mut() = v); }
pub fn g__MALARIA_ASKED() -> Py { ensure_init(); G__MALARIA_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MALARIA_ASKED(v: Py) { ensure_init(); G__MALARIA_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__MALARIA_SENTENCE() -> Py { ensure_init(); G__MALARIA_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MALARIA_SENTENCE(v: Py) { ensure_init(); G__MALARIA_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDICINES() -> Py { ensure_init(); G__MEDICINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDICINES(v: Py) { ensure_init(); G__MEDICINES.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDICINES_ASK() -> Py { ensure_init(); G__MEDICINES_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDICINES_ASK(v: Py) { ensure_init(); G__MEDICINES_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDICINES_REPLY() -> Py { ensure_init(); G__MEDICINES_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDICINES_REPLY(v: Py) { ensure_init(); G__MEDICINES_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TRIP() -> Py { ensure_init(); G__NOT_A_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TRIP(v: Py) { ensure_init(); G__NOT_A_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER_ONES() -> Py { ensure_init(); G__OTHER_ONES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER_ONES(v: Py) { ensure_init(); G__OTHER_ONES.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE() -> Py { ensure_init(); G__PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE(v: Py) { ensure_init(); G__PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACES() -> Py { ensure_init(); G__PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACES(v: Py) { ensure_init(); G__PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g__ROUTINE() -> Py { ensure_init(); G__ROUTINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROUTINE(v: Py) { ensure_init(); G__ROUTINE.with(|c| *c.borrow_mut() = v); }
pub fn g__ROUTINE_PLACE() -> Py { ensure_init(); G__ROUTINE_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROUTINE_PLACE(v: Py) { ensure_init(); G__ROUTINE_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_REST_OF_THE_LIST() -> Py { ensure_init(); G__THE_REST_OF_THE_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_REST_OF_THE_LIST(v: Py) { ensure_init(); G__THE_REST_OF_THE_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN_TO_BOOK_BARE() -> Py { ensure_init(); G__WHEN_TO_BOOK_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN_TO_BOOK_BARE(v: Py) { ensure_init(); G__WHEN_TO_BOOK_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g__YELLOW_ASKED() -> Py { ensure_init(); G__YELLOW_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YELLOW_ASKED(v: Py) { ensure_init(); G__YELLOW_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__YELLOW_FEVER() -> Py { ensure_init(); G__YELLOW_FEVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YELLOW_FEVER(v: Py) { ensure_init(); G__YELLOW_FEVER.with(|c| *c.borrow_mut() = v); }
pub fn g__YELLOW_SENTENCE() -> Py { ensure_init(); G__YELLOW_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YELLOW_SENTENCE(v: Py) { ensure_init(); G__YELLOW_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The jabs, the medicines and the health paperwork of a trip.\n\nA domain of its own because the question is about the trip rather than about the body: \"do i need\nany jabs for thailand\" wants when to book and which ones, and \"can i take my medication to italy\"\nwants the hand-luggage rule and the label on the box. Both were answered by the pharmacist hedge,\nwhich is honest and never says either, and \"what jabs do i need for kenya\" was the explainer of\nwhat a vaccine is - a good answer to a different question.\n\nThe country named is what makes it a trip: a line about a baby's jabs or about mixing two\nmedicines names no place, and it stays with the health rows. That is the whole of the guard, so\nthe rows are narrow on purpose.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("travel"), py_str("health")]));
    fn __data1() -> Py { py_tuple(vec![py_str("jab"), py_str("jabs"), py_str("vaccination"), py_str("vaccinations"), py_str("vaccinate"), py_str("vaccinated"), py_str("vaccine"), py_str("vaccines"), py_str("immunisation"), py_str("immunisations"), py_str("immunization"), py_str("immunizations"), py_str("travel jabs"), py_str("travel vaccines"), py_str("travel clinic"), py_str("yellow fever"), py_str("typhoid"), py_str("hepatitis a"), py_str("hepatitis"), py_str("rabies"), py_str("cholera"), py_str("japanese encephalitis"), py_str("tetanus"), py_str("malaria"), py_str("malaria tablets"), py_str("malaria pills"), py_str("antimalarial"), py_str("antimalarials")]) }
    s__JABS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("need"), py_str("needed"), py_str("needs"), py_str("required"), py_str("recommended"), py_str("do i need"), py_str("will i need"), py_str("which"), py_str("what"), py_str("before i go"), py_str("before we go"), py_str("before we travel"), py_str("get"), py_str("have"), py_str("should i get"), py_str("should i have"), py_str("worth getting"), py_str("certificate")]) }
    s__JABS_ASK(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("medication"), py_str("medications"), py_str("medicine"), py_str("medicines"), py_str("prescription"), py_str("prescriptions"), py_str("tablet"), py_str("tablets"), py_str("pills"), py_str("insulin"), py_str("epipen"), py_str("epi pen"), py_str("inhaler"), py_str("injections"), py_str("injection"), py_str("drugs"), py_str("medical supplies"), py_str("antibiotics"), py_str("antibiotic"), py_str("standby antibiotics"), py_str("antimalarials")]) }
    s__MEDICINES(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("take"), py_str("taking"), py_str("bring"), py_str("bringing"), py_str("carry"), py_str("carrying"), py_str("pack"), py_str("packing"), py_str("hand luggage"), py_str("carry on"), py_str("allowed"), py_str("legal"), py_str("abroad"), py_str("away"), py_str("travelling"), py_str("traveling"), py_str("travel"), py_str("flight"), py_str("flying"), py_str("fly"), py_str("airport"), py_str("customs")]) }
    s__MEDICINES_ASK(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("spain"), py_str("france"), py_str("italy"), py_str("greece"), py_str("portugal"), py_str("turkey"), py_str("cyprus"), py_str("malta"), py_str("croatia"), py_str("canaries"), py_str("canary islands"), py_str("mallorca"), py_str("majorca"), py_str("ibiza"), py_str("tenerife"), py_str("lanzarote"), py_str("sicily"), py_str("sardinia"), py_str("corsica"), py_str("germany"), py_str("austria"), py_str("switzerland"), py_str("holland"), py_str("netherlands"), py_str("belgium"), py_str("ireland"), py_str("poland"), py_str("czech republic"), py_str("prague"), py_str("hungary"), py_str("budapest"), py_str("slovenia"), py_str("slovakia"), py_str("romania"), py_str("bulgaria"), py_str("serbia"), py_str("albania"), py_str("montenegro"), py_str("bosnia"), py_str("estonia"), py_str("latvia"), py_str("lithuania"), py_str("finland"), py_str("sweden"), py_str("norway"), py_str("denmark"), py_str("iceland"), py_str("morocco"), py_str("tunisia"), py_str("egypt"), py_str("cape verde"), py_str("gambia"), py_str("senegal"), py_str("ghana"), py_str("nigeria"), py_str("kenya"), py_str("tanzania"), py_str("zanzibar"), py_str("ethiopia"), py_str("uganda"), py_str("rwanda"), py_str("south africa"), py_str("cape town"), py_str("johannesburg"), py_str("namibia"), py_str("botswana"), py_str("zimbabwe"), py_str("zambia"), py_str("madagascar"), py_str("mauritius"), py_str("seychelles"), py_str("dubai"), py_str("abu dhabi"), py_str("uae"), py_str("emirates"), py_str("qatar"), py_str("oman"), py_str("jordan"), py_str("israel"), py_str("lebanon"), py_str("saudi arabia"), py_str("india"), py_str("goa"), py_str("kerala"), py_str("rajasthan"), py_str("delhi"), py_str("mumbai"), py_str("pakistan"), py_str("bangladesh"), py_str("nepal"), py_str("bhutan"), py_str("sri lanka"), py_str("maldives"), py_str("thailand"), py_str("bangkok"), py_str("phuket"), py_str("vietnam"), py_str("cambodia"), py_str("laos"), py_str("myanmar"), py_str("burma"), py_str("malaysia"), py_str("singapore"), py_str("indonesia"), py_str("bali"), py_str("java"), py_str("philippines"), py_str("manila"), py_str("china"), py_str("beijing"), py_str("shanghai"), py_str("hong kong"), py_str("japan"), py_str("tokyo"), py_str("kyoto"), py_str("south korea"), py_str("seoul"), py_str("taiwan"), py_str("mongolia"), py_str("kazakhstan"), py_str("uzbekistan"), py_str("australia"), py_str("sydney"), py_str("melbourne"), py_str("new zealand"), py_str("auckland"), py_str("fiji"), py_str("samoa"), py_str("tahiti"), py_str("usa"), py_str("america"), py_str("the states"), py_str("new york"), py_str("florida"), py_str("california"), py_str("las vegas"), py_str("hawaii"), py_str("alaska"), py_str("canada"), py_str("toronto"), py_str("vancouver"), py_str("mexico"), py_str("cancun"), py_str("cuba"), py_str("havana"), py_str("jamaica"), py_str("barbados"), py_str("trinidad"), py_str("dominican republic"), py_str("puerto rico"), py_str("bahamas"), py_str("antigua"), py_str("st lucia"), py_str("grenada"), py_str("guatemala"), py_str("belize"), py_str("honduras"), py_str("costa rica"), py_str("panama"), py_str("nicaragua"), py_str("colombia"), py_str("ecuador"), py_str("galapagos"), py_str("peru"), py_str("machu picchu"), py_str("bolivia"), py_str("chile"), py_str("argentina"), py_str("buenos aires"), py_str("brazil"), py_str("rio"), py_str("uruguay"), py_str("paraguay"), py_str("venezuela"), py_str("guyana"), py_str("suriname")]) }
    s__PLACES(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("spain"), py_str("france"), py_str("italy"), py_str("greece"), py_str("portugal"), py_str("cyprus"), py_str("malta"), py_str("croatia"), py_str("canaries"), py_str("canary islands"), py_str("mallorca"), py_str("majorca"), py_str("menorca"), py_str("ibiza"), py_str("tenerife"), py_str("lanzarote"), py_str("fuerteventura"), py_str("gran canaria"), py_str("sicily"), py_str("sardinia"), py_str("corsica"), py_str("germany"), py_str("austria"), py_str("switzerland"), py_str("holland"), py_str("netherlands"), py_str("belgium"), py_str("ireland"), py_str("poland"), py_str("czech republic"), py_str("prague"), py_str("hungary"), py_str("budapest"), py_str("slovenia"), py_str("slovakia"), py_str("romania"), py_str("bulgaria"), py_str("estonia"), py_str("latvia"), py_str("lithuania"), py_str("finland"), py_str("sweden"), py_str("norway"), py_str("denmark"), py_str("iceland"), py_str("andorra"), py_str("monaco"), py_str("luxembourg"), py_str("liechtenstein"), py_str("gibraltar"), py_str("jersey"), py_str("guernsey"), py_str("isle of man")]) }
    s__ROUTINE(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("ghana"), py_str("nigeria"), py_str("senegal"), py_str("gambia"), py_str("kenya"), py_str("tanzania"), py_str("zanzibar"), py_str("ethiopia"), py_str("uganda"), py_str("rwanda"), py_str("zambia"), py_str("zimbabwe"), py_str("botswana"), py_str("namibia"), py_str("madagascar"), py_str("brazil"), py_str("peru"), py_str("colombia"), py_str("ecuador"), py_str("bolivia"), py_str("venezuela"), py_str("guyana"), py_str("suriname"), py_str("panama"), py_str("trinidad"), py_str("costa rica")]) }
    s__YELLOW_FEVER(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("ghana"), py_str("nigeria"), py_str("senegal"), py_str("gambia"), py_str("kenya"), py_str("tanzania"), py_str("zanzibar"), py_str("ethiopia"), py_str("uganda"), py_str("rwanda"), py_str("zambia"), py_str("zimbabwe"), py_str("botswana"), py_str("namibia"), py_str("madagascar"), py_str("malawi"), py_str("india"), py_str("goa"), py_str("kerala"), py_str("rajasthan"), py_str("delhi"), py_str("mumbai"), py_str("pakistan"), py_str("bangladesh"), py_str("nepal"), py_str("sri lanka"), py_str("thailand"), py_str("bangkok"), py_str("phuket"), py_str("vietnam"), py_str("cambodia"), py_str("laos"), py_str("myanmar"), py_str("burma"), py_str("malaysia"), py_str("indonesia"), py_str("bali"), py_str("java"), py_str("philippines"), py_str("manila"), py_str("brazil"), py_str("peru"), py_str("colombia"), py_str("ecuador"), py_str("bolivia"), py_str("venezuela"), py_str("guyana"), py_str("suriname"), py_str("panama"), py_str("honduras"), py_str("guatemala"), py_str("belize"), py_str("nicaragua")]) }
    s__MALARIA(__data8());
    s__ROUTINE_PLACE(stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![a-z])(%s)(?![a-z])"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__ROUTINE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut place: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[place.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AWAY(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:abroad|overseas|travel|travels|travelling|traveling|travelled|traveled|trip|trips|holiday|holidays|vacation|flying|fly|flight|flights|airport|goa|going away|going to|backpacking|honeymoon)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PLACE(stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![a-z])(%s)(?![a-z])"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__PLACES()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut place: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[place.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ANIMAL(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:dog|dogs|puppy|puppies|cat|cats|kitten|kittens|pet|pets|horse|horses|pony|rabbit|hamster|parrot|ferret|tortoise)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_TRIP(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:baby|babies|infant|infants|newborn|toddler|toddlers|child|children|kid|kids|son|daughter|mum|dad|mother|father|pregnant|pregnancy|expecting|school|nursery|nurse|tetanus|cut|cuts|wound|wounds|scratch|scratched|bite|bitten|flu|shingles|hpv|mmr|measles|mumps|rubella|whooping|childminder|side effects|side effect|reaction|sore arm|antibody|antibodies|immune)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BARE_NEED(stdlib_call("re", "compile", &[py_str("\\b(?:do|will|would|should|could) (?:i|we) (?:need|needs|have|get|be needing)\\b|\\bwhat (?:jabs|vaccinations|vaccines|injections|immunisations) (?:do|will|would|should) (?:i|we)\\b|\\bwhich (?:jabs|vaccinations|vaccines|injections)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TRAVEL_CLINIC(py_str("Book a travel clinic or a pharmacy that does travel jabs four to six weeks before you fly: some courses are two or three doses a month apart, and most of them take a fortnight to work."));
    s__THE_REST_OF_THE_LIST(py_str("Thailand and most of Asia, Africa and South America want hepatitis A and typhoid for almost any trip. Yellow fever is a certificate some countries in Africa and South America ask for on arrival, and malaria tablets are a separate decision about the risk in the area you are actually staying in. Check the country on the NHS Fit for Travel or the GOV.UK travel advice pages - that is the list that counts - and keep the record card with your passport."));
    s__CHECK_IT(py_str("Check %s on the NHS Fit for Travel page - that is the list that counts - and keep the record card with your passport."));
    s__WHEN_TO_BOOK_BARE(py_str("Book it four to six weeks before you fly, because some courses are two or three doses a month apart."));
    s__OTHER_ONES(py_str("The other ones for %s are hepatitis A and typhoid."));
    s__YELLOW_SENTENCE(py_str("Yellow fever is the one to ask about for %s: the vaccine is for the risk areas, and the certificate is what a border asks for when you arrive from one, so the clinic needs your route and not only the country."));
    s__MALARIA_SENTENCE(py_str("Malaria tablets for %s are a separate decision again - they depend on the area and the season you are going in."));
    s__MALARIA_ASKED(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:malaria|antimalarial|antimalarials)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__YELLOW_ASKED(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:yellow fever|certificate)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MEDICINES_REPLY(py_str("Carry it in your hand luggage rather than the hold, in the original box with the pharmacy label showing your name and the dose, and take a few more days' worth than the trip lasts. A repeat prescription printout or a short letter from the GP answers the customs question without an argument. Ordinary medicines are fine almost everywhere; anything controlled - codeine, strong painkillers, sleeping tablets, ADHD medicines - is worth checking with the embassy of the country before you fly, and a few want a certificate in advance. Insulin and anything that needs the cold goes in hand luggage too, in a cool bag and never against ice. Antibiotics are worth asking the GP about before you go rather than buying out there - some destinations get a standby course for a stomach bug, and it is a prescription, not a souvenir."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("travel-jabs")); __args.push(py_tuple(vec![g__JABS(), g__JABS_ASK()])); __args.push(py_func("_the_jabs_reply", |args: &[Py]| _the_jabs_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("_a_trip_is_named", |args: &[Py]| _a_trip_is_named(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    fn __data9() -> Py { py_tuple(vec![py_str("with my"), py_str("together"), py_str("mixing"), py_str("mix them"), py_str("side effect"), py_str("side effects"), py_str("alcohol"), py_str("pregnant"), py_str("breastfeeding"), py_str("overdose"), py_str("my baby"), py_str("my child"), py_str("my son"), py_str("my daughter")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("medication-abroad")); __args.push(py_tuple(vec![g__MEDICINES(), g__MEDICINES_ASK()])); __args.push(g__MEDICINES_REPLY()); __kws.extend(vec![("guard".to_string(), py_func("_a_trip_is_named", |args: &[Py]| _a_trip_is_named(args))), ("avoid".to_string(), __data9())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _a_trip_is_named(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line names somewhere to go, or says the words for going away.\n\n    The jabs and the medicines are the trip's question only when there is a trip: \"does my baby\n    need her jabs\" is the baby row's and \"can i take ibuprofen with my medication\" is the\n    pharmacist's, and neither names a place. An animal is the same sort of fence: \"do i need\n    jabs for my dog to travel abroad\" names both the animal and the trip, and it is the pet card\n    that answers it.\n    ");
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__ANIMAL(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__PLACE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__AWAY(), "search", &[text.clone()]) } }) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g__A_BARE_NEED(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NOT_A_TRIP(), "search", &[text.clone()]))) } };
    Py::None
}
pub fn _place_named(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("The first place in the line, as the person typed it and tidied for the middle of a\n    sentence, or None.\n\n    The routine list is searched first so that \"islands in spain\" answers for Spain rather than\n    for the Canaries, and the widest list last.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g__ROUTINE_PLACE(), g__PLACE()]));
    for __item in __iter1 {
    pattern = __item.clone();
    there = mcall(&pattern.clone(), "search", &[text.clone()]);
    if py_truthy(&there.clone()) {
    return mcall(&mcall(&mcall(&there.clone(), "group", &[Py::Int(1)]), "strip", &[]), "lower", &[]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_jabs_reply(args: &[Py]) -> Py {
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
    let mut asks_malaria: Py = Py::None;
    let mut asks_yellow: Py = Py::None;
    let mut malaria: Py = Py::None;
    let mut name: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut yellow: Py = Py::None;
    let _ = py_str("The answer for the place named, and the list only for a line with no place in it.\n\n    On the base the whole domain gave one five-sentence paragraph: the person who said Kenya was\n    told what \"Thailand and most of Asia, Africa and South America\" want and then told to look\n    Kenya up. The knowledge is a table keyed by place - the European holiday wants nothing, most\n    of the rest of the world wants hepatitis A and typhoid, and the yellow fever and malaria\n    places say so - and the place named picks the sentence. What the line asked for comes first:\n    \"do i need malaria tablets for kenya\" leads with malaria and \"do i need a yellow fever\n    certificate for brazil\" with the certificate.\n    ");
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    place = _place_named(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![g__A_TRAVEL_CLINIC(), g__THE_REST_OF_THE_LIST()]));
    }
    name = mcall(&place.clone(), "title", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__ROUTINE(), &place.clone()))) {
    return op_modulo(&py_str("Nothing beyond the routine jabs for %s - no travel clinic needed unless you are staying somewhere unusual or going on somewhere else. Make sure the routine ones are up to date (MMR, tetanus, polio), and check %s on the NHS Fit for Travel page if you want it in writing."), &py_tuple(vec![name.clone(), name.clone()]));
    }
    malaria = Py::Bool(py_contains(&g__MALARIA(), &place.clone()));
    yellow = Py::Bool(py_contains(&g__YELLOW_FEVER(), &place.clone()));
    asks_malaria = { let __b = malaria.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__MALARIA_ASKED(), "search", &[text.clone()]) } };
    asks_yellow = { let __b = yellow.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__YELLOW_ASKED(), "search", &[text.clone()]) } };
    said = py_list(vec![]);
    if py_truthy(&asks_malaria.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&py_str("Malaria tablets are the decision for %s, and it is the area and the season that decide them, so take the clinic's advice for exactly where you are staying."), &name.clone())]);
    let _ = mcall(&said.clone(), "append", &[g__WHEN_TO_BOOK_BARE()]);
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__OTHER_ONES(), &name.clone())]);
    if py_truthy(&yellow.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__YELLOW_SENTENCE(), &name.clone())]);
    }
    } else {
    if py_truthy(&asks_yellow.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&py_str("%s is in the yellow fever area: the vaccine is for the areas with risk, and the certificate is what a border asks for when you arrive from one, so the clinic needs your route and not only the country."), &name.clone())]);
    let _ = mcall(&said.clone(), "append", &[g__WHEN_TO_BOOK_BARE()]);
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__OTHER_ONES(), &name.clone())]);
    if py_truthy(&malaria.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__MALARIA_SENTENCE(), &name.clone())]);
    }
    } else {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&py_str("Plan on hepatitis A and typhoid for %s, and book a travel clinic or a pharmacy that does travel jabs four to six weeks before you fly: some courses are two or three doses a month apart, and most of them take a fortnight to work."), &name.clone())]);
    if py_truthy(&yellow.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__YELLOW_SENTENCE(), &name.clone())]);
    }
    if py_truthy(&malaria.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__MALARIA_SENTENCE(), &name.clone())]);
    }
    }
    }
    let _ = mcall(&said.clone(), "append", &[op_modulo(&g__CHECK_IT(), &name.clone())]);
    return mcall(&py_str(" "), "join", &[said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.travel_health".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ANIMAL" => Some(g__ANIMAL()),
            "_AWAY" => Some(g__AWAY()),
            "_A_BARE_NEED" => Some(g__A_BARE_NEED()),
            "_A_TRAVEL_CLINIC" => Some(g__A_TRAVEL_CLINIC()),
            "_CHECK_IT" => Some(g__CHECK_IT()),
            "_JABS" => Some(g__JABS()),
            "_JABS_ASK" => Some(g__JABS_ASK()),
            "_MALARIA" => Some(g__MALARIA()),
            "_MALARIA_ASKED" => Some(g__MALARIA_ASKED()),
            "_MALARIA_SENTENCE" => Some(g__MALARIA_SENTENCE()),
            "_MEDICINES" => Some(g__MEDICINES()),
            "_MEDICINES_ASK" => Some(g__MEDICINES_ASK()),
            "_MEDICINES_REPLY" => Some(g__MEDICINES_REPLY()),
            "_NOT_A_TRIP" => Some(g__NOT_A_TRIP()),
            "_OTHER_ONES" => Some(g__OTHER_ONES()),
            "_PLACE" => Some(g__PLACE()),
            "_PLACES" => Some(g__PLACES()),
            "_ROUTINE" => Some(g__ROUTINE()),
            "_ROUTINE_PLACE" => Some(g__ROUTINE_PLACE()),
            "_THE_REST_OF_THE_LIST" => Some(g__THE_REST_OF_THE_LIST()),
            "_WHEN_TO_BOOK_BARE" => Some(g__WHEN_TO_BOOK_BARE()),
            "_YELLOW_ASKED" => Some(g__YELLOW_ASKED()),
            "_YELLOW_FEVER" => Some(g__YELLOW_FEVER()),
            "_YELLOW_SENTENCE" => Some(g__YELLOW_SENTENCE()),
            "_a_trip_is_named" => Some(py_func("_a_trip_is_named", |args: &[Py]| _a_trip_is_named(args))),
            "_place_named" => Some(py_func("_place_named", |args: &[Py]| _place_named(args))),
            "_the_jabs_reply" => Some(py_func("_the_jabs_reply", |args: &[Py]| _the_jabs_reply(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ANIMAL" => s__ANIMAL(v),
            "_AWAY" => s__AWAY(v),
            "_A_BARE_NEED" => s__A_BARE_NEED(v),
            "_A_TRAVEL_CLINIC" => s__A_TRAVEL_CLINIC(v),
            "_CHECK_IT" => s__CHECK_IT(v),
            "_JABS" => s__JABS(v),
            "_JABS_ASK" => s__JABS_ASK(v),
            "_MALARIA" => s__MALARIA(v),
            "_MALARIA_ASKED" => s__MALARIA_ASKED(v),
            "_MALARIA_SENTENCE" => s__MALARIA_SENTENCE(v),
            "_MEDICINES" => s__MEDICINES(v),
            "_MEDICINES_ASK" => s__MEDICINES_ASK(v),
            "_MEDICINES_REPLY" => s__MEDICINES_REPLY(v),
            "_NOT_A_TRIP" => s__NOT_A_TRIP(v),
            "_OTHER_ONES" => s__OTHER_ONES(v),
            "_PLACE" => s__PLACE(v),
            "_PLACES" => s__PLACES(v),
            "_ROUTINE" => s__ROUTINE(v),
            "_ROUTINE_PLACE" => s__ROUTINE_PLACE(v),
            "_THE_REST_OF_THE_LIST" => s__THE_REST_OF_THE_LIST(v),
            "_WHEN_TO_BOOK_BARE" => s__WHEN_TO_BOOK_BARE(v),
            "_YELLOW_ASKED" => s__YELLOW_ASKED(v),
            "_YELLOW_FEVER" => s__YELLOW_FEVER(v),
            "_YELLOW_SENTENCE" => s__YELLOW_SENTENCE(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.travel_health", module_obj);
    pyrt::xrefs::reg_fn("knowledge.travel_health", "_a_trip_is_named", py_func("_a_trip_is_named", |args: &[Py]| _a_trip_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.travel_health", "_place_named", py_func("_place_named", |args: &[Py]| _place_named(args)));
    pyrt::xrefs::reg_fn("knowledge.travel_health", "_the_jabs_reply", py_func("_the_jabs_reply", |args: &[Py]| _the_jabs_reply(args)));
    pyrt::xrefs::reg_access("knowledge.travel_health", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.travel_health", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_ANIMAL", g__ANIMAL, s__ANIMAL);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_AWAY", g__AWAY, s__AWAY);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_A_BARE_NEED", g__A_BARE_NEED, s__A_BARE_NEED);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_A_TRAVEL_CLINIC", g__A_TRAVEL_CLINIC, s__A_TRAVEL_CLINIC);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_CHECK_IT", g__CHECK_IT, s__CHECK_IT);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_JABS", g__JABS, s__JABS);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_JABS_ASK", g__JABS_ASK, s__JABS_ASK);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_MALARIA", g__MALARIA, s__MALARIA);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_MALARIA_ASKED", g__MALARIA_ASKED, s__MALARIA_ASKED);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_MALARIA_SENTENCE", g__MALARIA_SENTENCE, s__MALARIA_SENTENCE);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_MEDICINES", g__MEDICINES, s__MEDICINES);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_MEDICINES_ASK", g__MEDICINES_ASK, s__MEDICINES_ASK);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_MEDICINES_REPLY", g__MEDICINES_REPLY, s__MEDICINES_REPLY);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_NOT_A_TRIP", g__NOT_A_TRIP, s__NOT_A_TRIP);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_OTHER_ONES", g__OTHER_ONES, s__OTHER_ONES);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_PLACE", g__PLACE, s__PLACE);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_PLACES", g__PLACES, s__PLACES);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_ROUTINE", g__ROUTINE, s__ROUTINE);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_ROUTINE_PLACE", g__ROUTINE_PLACE, s__ROUTINE_PLACE);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_THE_REST_OF_THE_LIST", g__THE_REST_OF_THE_LIST, s__THE_REST_OF_THE_LIST);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_WHEN_TO_BOOK_BARE", g__WHEN_TO_BOOK_BARE, s__WHEN_TO_BOOK_BARE);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_YELLOW_ASKED", g__YELLOW_ASKED, s__YELLOW_ASKED);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_YELLOW_FEVER", g__YELLOW_FEVER, s__YELLOW_FEVER);
    pyrt::xrefs::reg_access("knowledge.travel_health", "_YELLOW_SENTENCE", g__YELLOW_SENTENCE, s__YELLOW_SENTENCE);
    pyrt::xrefs::reg_access("knowledge.travel_health", "entry", g_entry, s_entry);
}
