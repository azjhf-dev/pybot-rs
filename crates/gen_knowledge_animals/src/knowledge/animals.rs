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


thread_local! { static G_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURIOSITIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATS_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FARM_DIETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_ROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.animals: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANIMALS() -> Py { ensure_init(); G_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMALS(v: Py) { ensure_init(); G_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CURIOSITIES() -> Py { ensure_init(); G_CURIOSITIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURIOSITIES(v: Py) { ensure_init(); G_CURIOSITIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DIETS() -> Py { ensure_init(); G_DIETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIETS(v: Py) { ensure_init(); G_DIETS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_ASK() -> Py { ensure_init(); G_DIET_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_ASK(v: Py) { ensure_init(); G_DIET_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_AVOID() -> Py { ensure_init(); G_DIET_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_AVOID(v: Py) { ensure_init(); G_DIET_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_CUES() -> Py { ensure_init(); G_DIET_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_CUES(v: Py) { ensure_init(); G_DIET_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_EATS() -> Py { ensure_init(); G_EATS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATS(v: Py) { ensure_init(); G_EATS.with(|c| *c.borrow_mut() = v); }
pub fn g_EATS_ITSELF() -> Py { ensure_init(); G_EATS_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATS_ITSELF(v: Py) { ensure_init(); G_EATS_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_ASK() -> Py { ensure_init(); G_EAT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_ASK(v: Py) { ensure_init(); G_EAT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_AVOID() -> Py { ensure_init(); G_EAT_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_AVOID(v: Py) { ensure_init(); G_EAT_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_CUES() -> Py { ensure_init(); G_EAT_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_CUES(v: Py) { ensure_init(); G_EAT_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FARM_DIETS() -> Py { ensure_init(); G_FARM_DIETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FARM_DIETS(v: Py) { ensure_init(); G_FARM_DIETS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECORDS() -> Py { ensure_init(); G_RECORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECORDS(v: Py) { ensure_init(); G_RECORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_ROWS() -> Py { ensure_init(); G_WORD_ROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_ROWS(v: Py) { ensure_init(); G_WORD_ROWS.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_ASK() -> Py { ensure_init(); G__WORD_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_ASK(v: Py) { ensure_init(); G__WORD_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_AVOID() -> Py { ensure_init(); G__WORD_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_AVOID(v: Py) { ensure_init(); G__WORD_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_CUES() -> Py { ensure_init(); G__WORD_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_CUES(v: Py) { ensure_init(); G__WORD_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_SENTENCE() -> Py { ensure_init(); G__WORD_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_SENTENCE(v: Py) { ensure_init(); G__WORD_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Animals and wildlife: the questions people ask about the creatures around them.\n\nThe answers lane owns this domain. People ask these while watching something, reading to a\nchild, or answering homework: how long an animal lives, how big or fast it is, which one is the\nfastest, and why an animal does the strange thing it does. The facts are school level, the\nanswer names the number first, and a fact stays out of the reply when the animal has no row for\nthat question, so a line this table cannot answer passes to another domain instead of guessing.\n\nKept as data: one row per animal (the words people call it, and the facts it has), and one row\nper curiosity (the animal, the words that ask it, the answer). The entries are built from those\ntables, so adding a row is all it takes to answer another animal, and no entry holds a phrasing.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("nature-and-weather"), py_str("science")]));
    fn __data1() -> Py { py_tuple(vec![py_dict(vec![(py_str("names"), py_tuple(vec![py_str("elephant"), py_str("elephants"), py_str("african elephant"), py_str("african elephants")])), (py_str("life"), py_str("Elephants live 60 to 70 years in the wild, and a big bull can reach 70.")), (py_str("gestation"), py_str("An elephant carries her calf for about 22 months, the longest of any land animal - one calf, about 100 kg at birth, and the herd stands round her while she gives birth.")), (py_str("size"), py_str("An African elephant is the biggest land animal: a big bull is about 3 metres at the shoulder and 6 tonnes, and a newborn calf is around 100 kg.")), (py_str("speed"), py_str("An elephant can charge at about 40 km/h, which is faster than a person can run."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("lion"), py_str("lions")])), (py_str("life"), py_str("Lions live about 10 to 14 years in the wild; in a zoo they can reach 20.")), (py_str("size"), py_str("A male lion is about 190 kg and 1.2 metres at the shoulder, and the mane grows darker as he gets older.")), (py_str("speed"), py_str("A lion runs at up to 80 km/h, but only for a few seconds: it hunts by stalking close first."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("tiger"), py_str("tigers")])), (py_str("life"), py_str("Tigers live about 10 to 15 years in the wild.")), (py_str("size"), py_str("A male Siberian tiger is the biggest cat, at up to 300 kg and 3 metres nose to tail; a Bengal tiger is about 220 kg.")), (py_str("speed"), py_str("A tiger can sprint at about 60 km/h, but only over a short distance."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("cheetah"), py_str("cheetahs")])), (py_str("life"), py_str("Cheetahs live about 10 to 12 years in the wild.")), (py_str("speed"), py_str("The cheetah is the fastest land animal: about 110 km/h, and 0 to 100 km/h in about three seconds. It can only keep that up for 20 to 30 seconds, so it hunts by creeping close first."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("giraffe"), py_str("giraffes")])), (py_str("life"), py_str("Giraffes live about 25 years in the wild.")), (py_str("gestation"), py_str("A giraffe carries her calf for about 15 months, and the calf drops nearly two metres to the ground when it is born, then stands within the hour.")), (py_str("size"), py_str("The giraffe is the tallest animal: males reach about 5.5 metres, and more than a third of that is neck. The neck has seven bones, the same number as yours.")), (py_str("speed"), py_str("A giraffe can run at about 50 km/h, and its legs are long enough that a kick can kill a lion."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("zebra"), py_str("zebras")])), (py_str("life"), py_str("Zebras live about 20 to 25 years in the wild.")), (py_str("speed"), py_str("A zebra runs at up to 65 km/h and zigzags as it goes, which is harder for a lion to follow than a straight sprint."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rhino"), py_str("rhinos"), py_str("rhinoceros")])), (py_str("life"), py_str("Rhinos live 40 to 50 years.")), (py_str("gestation"), py_str("A rhino carries her calf for about 16 months, so well over a year, and a single calf every two to four years is normal.")), (py_str("size"), py_str("A white rhino is the second biggest land animal, at about 2,300 kg; a black rhino is smaller, around 1,000 kg, and has the longer horn.")), (py_str("speed"), py_str("A rhino can run at about 50 km/h, and it turns faster than you would expect for something that size."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("hippo"), py_str("hippos"), py_str("hippopotamus")])), (py_str("life"), py_str("Hippos live about 40 to 50 years.")), (py_str("size"), py_str("A hippo is about 1,500 kg and 1.5 metres at the shoulder, with teeth that can be half a metre long.")), (py_str("speed"), py_str("A hippo can run at about 30 km/h on land, and it is the animal that kills most people in Africa."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("gorilla"), py_str("gorillas")])), (py_str("life"), py_str("Gorillas live 35 to 40 years in the wild.")), (py_str("size"), py_str("An adult male gorilla is about 1.7 metres tall and 160 to 200 kg, and twice as strong in the arms as the average person."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("chimpanzee"), py_str("chimpanzees"), py_str("chimp"), py_str("chimps")])), (py_str("life"), py_str("Chimpanzees live 40 to 50 years in the wild.")), (py_str("size"), py_str("A chimpanzee is about 1.2 metres tall standing and 40 to 60 kg, and around four times stronger than a person of the same size."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("panda"), py_str("pandas"), py_str("giant panda"), py_str("giant pandas")])), (py_str("life"), py_str("Giant pandas live about 20 years in the wild, and nearer 30 in a zoo.")), (py_str("size"), py_str("A giant panda is about 100 to 115 kg, and it eats up to 12 kg of bamboo a day because bamboo gives it so little energy."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("koala"), py_str("koalas")])), (py_str("life"), py_str("Koalas live 10 to 15 years in the wild.")), (py_str("size"), py_str("A koala is about 60 to 85 cm and 4 to 15 kg, and it sleeps around 18 hours a day because eucalyptus leaves give it so little energy."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("kangaroo"), py_str("kangaroos")])), (py_str("life"), py_str("Red kangaroos live about 20 to 25 years.")), (py_str("size"), py_str("A male red kangaroo stands about 1.6 metres and weighs up to 85 kg, and it can cover 8 metres in one hop.")), (py_str("speed"), py_str("A red kangaroo can reach about 70 km/h in a burst, and it is faster over distance than a horse."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("whale"), py_str("whales")])), (py_str("life"), py_str("Whales are long-lived: a blue whale lives 80 to 90 years, a humpback about 45 to 50.")), (py_str("gestation"), py_str("A whale carries for 10 to 12 months, and the calf is born tail first and swims up for its first breath within seconds."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("blue whale"), py_str("blue whales")])), (py_str("life"), py_str("Blue whales live 80 to 90 years, and some live past 100.")), (py_str("gestation"), py_str("A blue whale carries her calf for 10 to 12 months, and it is about 7 metres long and 2.5 tonnes when it is born.")), (py_str("size"), py_str("The blue whale is the biggest animal alive, and the biggest that has ever lived: up to 30 metres and around 150 tonnes, with a heart the size of a small car.")), (py_str("speed"), py_str("A blue whale cruises at about 8 km/h and can burst to around 30 km/h when it is frightened."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("dolphin"), py_str("dolphins"), py_str("bottlenose dolphin"), py_str("bottlenose dolphins")])), (py_str("life"), py_str("Bottlenose dolphins live 40 to 50 years.")), (py_str("gestation"), py_str("A bottlenose dolphin carries her calf for about 12 months, and she feeds it milk for a year and a half to two years after that.")), (py_str("size"), py_str("A bottlenose dolphin is about 2.5 to 3 metres and 200 to 300 kg."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("shark"), py_str("sharks")])), (py_str("life"), py_str("Most sharks live 20 to 30 years, and the Greenland shark can live about 400, which makes it the longest-lived animal with a backbone.")), (py_str("size"), py_str("A great white is 4.5 to 6 metres and up to 1,100 kg; the whale shark, at 12 metres and 20 tonnes, is the biggest fish and eats only plankton."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("octopus"), py_str("octopuses"), py_str("octopi")])), (py_str("life"), py_str("Most octopuses live only one or two years, and die soon after they breed.")), (py_str("size"), py_str("The common octopus is around a metre across and 3 kg; the giant Pacific octopus can reach 5 metres arm tip to arm tip."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("penguin"), py_str("penguins"), py_str("emperor penguin"), py_str("emperor penguins")])), (py_str("life"), py_str("Penguins live 15 to 20 years in the wild.")), (py_str("size"), py_str("The emperor penguin is the biggest, at about 1.1 metres and 40 kg; the little blue penguin is around 40 cm and 1 kg."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("parrot"), py_str("parrots"), py_str("african grey"), py_str("macaw"), py_str("macaws")])), (py_str("life"), py_str("Parrots are long-lived: an African grey often reaches 50 to 60, and a macaw can pass 80."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("tortoise"), py_str("tortoises"), py_str("galapagos tortoise"), py_str("giant tortoise")])), (py_str("life"), py_str("Tortoises are the longest-lived land animals: a giant tortoise often reaches 100, and some have lived past 150."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("crocodile"), py_str("crocodiles"), py_str("alligator"), py_str("alligators")])), (py_str("life"), py_str("Crocodiles live 70 to 100 years, and they keep growing slowly all that time.")), (py_str("size"), py_str("A Nile crocodile reaches about 5 metres and 500 kg; the saltwater crocodile can go past 6 metres, the biggest of any reptile."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("snake"), py_str("snakes")])), (py_str("life"), py_str("Most snakes live 10 to 20 years in the wild, and a pet corn snake can pass 20."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("swan"), py_str("swans")])), (py_str("life"), py_str("Swans live 10 to 15 years in the wild.")), (py_str("size"), py_str("A mute swan is about 1.5 metres long with a wingspan of 2.2 metres, and weighs 10 to 12 kg."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("fox"), py_str("foxes")])), (py_str("life"), py_str("A fox usually lives only 3 to 5 years in the wild, though one in captivity can reach 12."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("hedgehog"), py_str("hedgehogs")])), (py_str("life"), py_str("Hedgehogs usually live only 3 to 5 years in the wild; the road is the main danger."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("squirrel"), py_str("squirrels"), py_str("grey squirrel"), py_str("grey squirrels")])), (py_str("life"), py_str("Grey squirrels live about 3 to 5 years in the wild."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("badger"), py_str("badgers")])), (py_str("life"), py_str("Badgers usually live 3 to 5 years in the wild, and can reach 10 in captivity."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("deer"), py_str("red deer"), py_str("roe deer")])), (py_str("life"), py_str("A red deer lives about 15 years and a roe deer about 7."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("eagle"), py_str("eagles"), py_str("golden eagle"), py_str("golden eagles"), py_str("bald eagle"), py_str("bald eagles")])), (py_str("life"), py_str("A golden eagle lives about 20 to 25 years in the wild.")), (py_str("size"), py_str("A golden eagle has a wingspan of about 2.2 metres; a bald eagle's is about 2.3 metres and it weighs 3 to 6 kg."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("owl"), py_str("owls"), py_str("barn owl"), py_str("barn owls")])), (py_str("life"), py_str("A barn owl usually lives only about four years in the wild, though in a barn or a sanctuary it can reach 15."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("bee"), py_str("bees"), py_str("honey bee"), py_str("honey bees"), py_str("honeybee"), py_str("honeybees")])), (py_str("life"), py_str("A worker bee lives about six weeks in summer, when it is flying all day; a queen lives three to five years."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("ant"), py_str("ants")])), (py_str("life"), py_str("A worker ant lives a year or two; a queen can live for decades.")), (py_str("speed"), py_str("An ant runs about 5 cm a second, which for something 5 mm long is the same as a person jogging."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("butterfly"), py_str("butterflies")])), (py_str("life"), py_str("An adult butterfly usually lives only a few weeks, though a butterfly that overwinters as an adult can live many months."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("mouse"), py_str("mice")])), (py_str("life"), py_str("A wild mouse usually lives under a year, and a pet one two to three years."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rat"), py_str("rats")])), (py_str("life"), py_str("A wild rat usually lives about a year, and a pet rat two to three years."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("cow"), py_str("cows"), py_str("cattle"), py_str("dairy cow"), py_str("dairy cows")])), (py_str("gestation"), py_str("A cow carries her calf for about 283 days, so nine months and a week - the vet can narrow it down, and a heifer with a first calf often goes a few days over. Signs she is close: a bagged-up udder, a slack tail end and restlessness the day before."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("sheep"), py_str("sheeps"), py_str("ewe"), py_str("ewes"), py_str("lamb"), py_str("lambs")])), (py_str("gestation"), py_str("A sheep carries her lambs for about 147 days, so five months, and a ewe with twins or triplets often goes a few days early. The last week she wants watching twice a day, because a ewe in trouble at lambing goes downhill fast."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("pig"), py_str("pigs"), py_str("piglet"), py_str("piglets"), py_str("sow"), py_str("sows"), py_str("boar"), py_str("boars")])), (py_str("gestation"), py_str("A pig carries her litter for 115 days - three months, three weeks and three days, which is the rule pig keepers use - and a litter is 8 to 12 piglets."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("chicken"), py_str("chickens"), py_str("hen"), py_str("hens"), py_str("cockerel"), py_str("cockerels")])), (py_str("gestation"), py_str("A hen is never pregnant: she lays an egg a day or so and the chick grows outside her. An egg takes 21 days to hatch, under a broody hen or in an incubator at 37.5C, turned an odd number of times a day."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("goat"), py_str("goats"), py_str("billy goat"), py_str("nanny goat"), py_str("doe goat")])), (py_str("gestation"), py_str("A goat carries her kids for about 150 days, so five months, and twins are the common litter. Her udder fills in the last two weeks and she will look for a corner to be alone in - that is the day to leave her quiet."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("pony"), py_str("ponies")])), (py_str("gestation"), py_str("A horse or pony carries her foal for about 340 days, so eleven months, and anything from 320 to 370 is normal - which is why the vet dates it from the covering rather than guessing. A mare foals fast, usually at night, and the foal should be up and feeding within two hours."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("horse"), py_str("horses"), py_str("mare"), py_str("mares"), py_str("foal"), py_str("foals")])), (py_str("gestation"), py_str("A horse carries her foal for about 340 days, so eleven months, and anything from 320 to 370 is normal - which is why the vet dates it from the covering rather than guessing. A mare foals fast, usually at night, and the foal should be up and feeding within two hours."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("bitch"), py_str("bitches"), py_str("puppy"), py_str("puppies"), py_str("labrador"), py_str("spaniel")])), (py_str("gestation"), py_str("A dog is pregnant for about 63 days, so nine weeks, and anything from 58 to 68 is normal - the date is a window, not a day. She is usually scanned or x-rayed around 55 days to count the pups, and she will want a quiet box a week before."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rabbit"), py_str("rabbits"), py_str("bunny"), py_str("bunnies")])), (py_str("gestation"), py_str("A rabbit kindles about 31 days after mating, so a month - which is why a rabbit you thought was alone can produce a litter before you have noticed she was pregnant. Leave her alone with a nest box and do not tidy it for a few days after the kits arrive."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("guinea pig"), py_str("guinea pigs"), py_str("cavy"), py_str("cavies")])), (py_str("gestation"), py_str("A guinea pig carries her pups for 63 to 70 days, so about nine to ten weeks, and the litter is usually two to four - born furred, eyes open and eating within hours."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("hamster"), py_str("hamsters")])), (py_str("gestation"), py_str("A hamster is pregnant for only 16 days, so barely over two weeks, and litters are five to nine. The father has to come out before she gives birth, because he will be chased and bitten once the pups are there."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("ferret"), py_str("ferrets")])), (py_str("gestation"), py_str("A ferret is pregnant for about 42 days, so six weeks, and a jill can be delayed: if she was mated late in the year the litter waits until the spring."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("orca"), py_str("orcas"), py_str("killer whale"), py_str("killer whales")])), (py_str("gestation"), py_str("An orca carries her calf for 15 to 18 months, the longest of the dolphins, and a calf stays with its mother for years."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("donkey"), py_str("donkeys")]))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("lizard"), py_str("lizards"), py_str("bearded dragon"), py_str("leopard gecko")]))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("polar bear"), py_str("polar bears")]))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("greyhound"), py_str("greyhounds")])), (py_str("speed"), py_str("A greyhound is the fastest dog, at about 70 km/h, and it reaches that in six strides from a standing start."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("horse"), py_str("horses")])), (py_str("speed"), py_str("A racehorse can reach about 70 km/h, and it can keep up a gallop for a couple of minutes rather than a few seconds."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("ostrich"), py_str("ostriches")])), (py_str("size"), py_str("The ostrich is the biggest bird: up to 2.7 metres tall and 150 kg. It cannot fly, but it runs at about 70 km/h, the fastest of any bird on land.")), (py_str("speed"), py_str("An ostrich runs at about 70 km/h over a short distance, and 40 to 50 km/h for longer, which is the fastest of any bird on land."))])]) }
    s_ANIMALS(__data1());
    fn __data2() -> Py { py_dict(vec![(py_tuple(vec![py_str("speed"), py_str("any")]), py_str("The peregrine falcon is the fastest animal: it reaches over 240 km/h in a hunting dive, though a cheetah is the fastest on land at about 110 km/h.")), (py_tuple(vec![py_str("speed"), py_str("land")]), py_str("The cheetah is the fastest land animal: about 110 km/h, from 0 to 100 in about three seconds, but only for 20 to 30 seconds at a time.")), (py_tuple(vec![py_str("speed"), py_str("sea")]), py_str("The sailfish is usually called the fastest fish, at around 100 km/h in short bursts, though that is hard to measure and often argued about.")), (py_tuple(vec![py_str("speed"), py_str("bird")]), py_str("The peregrine falcon, at over 240 km/h in a dive; in level flight a swift is the fastest, at around 110 km/h.")), (py_tuple(vec![py_str("big"), py_str("any")]), py_str("The blue whale is the biggest animal alive, and the biggest that has ever lived: up to 30 metres and around 150 tonnes.")), (py_tuple(vec![py_str("big"), py_str("sea")]), py_str("The blue whale: up to 30 metres and around 150 tonnes, with a heart the size of a small car and a tongue that weighs as much as an elephant.")), (py_tuple(vec![py_str("big"), py_str("land")]), py_str("The African elephant is the biggest land animal: a big bull is about 3 metres at the shoulder and 6 tonnes.")), (py_tuple(vec![py_str("big"), py_str("bird")]), py_str("The ostrich is the biggest bird, at up to 2.7 metres and 150 kg, and it cannot fly. The wandering albatross has the widest wingspan, about 3.5 metres.")), (py_tuple(vec![py_str("tall"), py_str("any")]), py_str("The giraffe: a male reaches about 5.5 metres, and more than a third of that is neck.")), (py_tuple(vec![py_str("tall"), py_str("bird")]), py_str("The ostrich, at up to 2.7 metres; the tallest flying bird is the crane, at about 1.8 metres.")), (py_tuple(vec![py_str("life"), py_str("any")]), py_str("The Greenland shark, at about 400 years, lives longest of any animal with a backbone. A giant tortoise can pass 150, and a blue whale 80 to 90.")), (py_tuple(vec![py_str("life"), py_str("land")]), py_str("The giant tortoise: often over 100 years, and the oldest known reached 190.")), (py_tuple(vec![py_str("strong"), py_str("any")]), py_str("An ant can carry 10 to 50 times its own weight, and a dung beetle can pull over 1,000 times its own, which is the strongest for its size.")), (py_tuple(vec![py_str("weak"), py_str("any")]), py_str("Weight for weight, an ant carries the most: 10 to 50 times its own body weight. A dung beetle can pull over 1,000 times its own.")), (py_tuple(vec![py_str("danger"), py_str("any")]), py_str("The mosquito, by a wide margin: it spreads malaria, dengue and yellow fever, which between them kill hundreds of thousands of people a year. It is not the bite that kills, so nets and repellent are the answer."))]) }
    s_RECORDS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("eat"), py_str("eats"), py_str("eating"), py_str("ate"), py_str("eaten"), py_str("diet"), py_str("feed"), py_str("feeds"), py_str("feeding"), py_str("fed"), py_str("food"), py_str("foods"), py_str("menu")]) }
    s_DIET_CUES(__data3());
    fn __data4() -> Py { py_dict(vec![(py_str("elephant"), py_str("Grass, leaves, bark and fruit: a big bull eats up to 150 kg of plants a day and drinks around 100 litres of water.")), (py_str("lion"), py_str("Meat, and only meat - zebra, wildebeest, antelope and buffalo, taken by the females in a group hunt. A lion eats around 5 to 7 kg a day and can go a week without.")), (py_str("tiger"), py_str("Meat: deer, wild pig and wild cattle, hunted alone and mostly at night. A tiger can eat up to 40 kg in one sitting and then go several days without.")), (py_str("cheetah"), py_str("Meat, and it has to be fresh: hares, young antelope and birds, caught in a sprint and eaten quickly before a lion or a hyena takes it. A cheetah eats around 3 kg a day.")), (py_str("giraffe"), py_str("Leaves, mostly acacia: the tongue and lips are tough enough to strip thorny twigs without harm. A giraffe eats around 34 kg of leaves a day and drinks only every few days.")), (py_str("zebra"), py_str("Grass, almost all of it: a zebra grazes up to 16 hours a day and needs water every day or two.")), (py_str("rhino"), py_str("Grass for the white rhino, which grazes with its head down; the black rhino browses bushes and leaves with its lip hooked round a twig. Both eat around 50 kg a day.")), (py_str("hippo"), py_str("Grass, and almost all of it at night on land: a hippo can eat 40 kg in a night and spends the day in the water.")), (py_str("gorilla"), py_str("Plants - leaves, shoots, stems and fruit - around 20 kg a day for a big male, with the odd insect. A mountain gorilla eats very little meat.")), (py_str("chimpanzee"), py_str("Mostly fruit, plus leaves, seeds and insects, and now and then meat from a monkey it has hunted, which is a group affair with a lot of noise.")), (py_str("panda"), py_str("Bamboo, almost nothing else: 12 to 38 kg a day, because bamboo gives so little energy that a panda eats for up to 14 hours of it.")), (py_str("koala"), py_str("Eucalyptus leaves only, and only a few species of them: a koala eats about half a kilo a day and sleeps 18 to 20 hours.")), (py_str("kangaroo"), py_str("Grass and leaves, grazed at dawn and dusk; a kangaroo needs little water and gets most of it from the plants.")), (py_str("whale"), py_str("Krill and small fish for a baleen whale, strained through the plates in its mouth; a toothed whale takes fish and squid.")), (py_str("blue whale"), py_str("Krill, and up to 4 tonnes of it a day in the feeding season, strained through the baleen plates; a blue whale eats little or nothing while it migrates.")), (py_str("dolphin"), py_str("Fish and squid, swallowed whole or shaken apart first: a dolphin eats around 10 to 20 kg of fish a day.")), (py_str("shark"), py_str("Fish, squid, seals and carrion, by species: a great white hunts seals, a basking shark strains plankton through its gills, and a nurse shark sucks shellfish off the seabed.")), (py_str("octopus"), py_str("Crabs, shellfish and small fish, cracked open with its beak and pulled apart; an octopus hunts at night and can taste with its arms.")), (py_str("penguin"), py_str("Fish, squid and krill, caught by diving: an emperor penguin can dive to 500 metres and stay under for 20 minutes.")), (py_str("tortoise"), py_str("Leaves, weeds and flowers - dandelion, plantain, clover, bindweed - for a Mediterranean tortoise, with no fruit and no dog food. A pet tortoise gets a pile about the size of its shell, once a day.")), (py_str("crocodile"), py_str("Meat: fish, birds, turtles and anything that comes to drink, taken in one lunge and swallowed in pieces. A crocodile can go months between meals.")), (py_str("eagle"), py_str("Meat: rabbits, fish, birds or carrion, by species, taken with the talons and torn up at a perch.")), (py_str("ostrich"), py_str("Mostly plants - grass, seeds, leaves and flowers - with the odd insect or lizard; an ostrich has no teeth and swallows small stones to grind its food.")), (py_str("horse"), py_str("Grass or hay, most of the day: a horse eats about 2% of its body weight a day, so 10 kg of hay for a 500 kg horse, in small amounts through the day, with water always available.")), (py_str("greyhound"), py_str("A complete dog food, and less than its size suggests: a retired racer of about 30 kg eats 400 to 500 g a day, split into two meals.")), (py_str("bee"), py_str("Nectar and pollen: the adults live on nectar, and the larvae on pollen and honey. A colony can bring in 20 to 30 kg of pollen in a year.")), (py_str("ant"), py_str("Almost anything - other insects, seeds, nectar and the honeydew off aphids. Some ants farm aphids for it, and some grow fungus on leaves instead.")), (py_str("cow"), py_str("Grass and hay, and a lot of it: a dairy cow eats around 20 kg of dry matter a day, which is most of a bale of hay, and drinks 60 to 100 litres of water.")), (py_str("sheep"), py_str("Grass, mostly, plus the clover and weeds in it: a sheep eats around 2 kg of dry matter a day and spends 8 to 10 hours of it grazing.")), (py_str("pig"), py_str("Almost anything, which is what an omnivore means: a pig eats 2 to 3 kg of feed a day and roots for grubs, roots and fallen fruit with a nose built to dig. It cannot sweat, so it wallows to cool down. Kitchen scraps and any meat are illegal to feed a pig in the UK, and that rule is what keeps swine fever out. A wild boar, the same animal, lives on roots, acorns, grubs and carrion over a much bigger range.")), (py_str("chicken"), py_str("Layers pellets or mash as the base, about 120 g a day per hen, plus grass, weeds, insects and vegetable scraps; grit and crushed oyster shell keep the eggshells hard. Bread and corn are treats rather than feed, and avocado, citrus peel, salt and anything mouldy are not for them at all.")), (py_str("goat"), py_str("Browse first, not grass: brambles, hedge, leaves, bark, weeds and thistles, then hay - about 2 kg of hay a day plus a mineral lick and water. A goat has a stomach for roughage, so rich grass or a lot of grain makes it ill, and it will strip a garden to the fence line given the chance.")), (py_str("pony"), py_str("Grass and hay, and less of it than people think: a 300 kg pony needs about 6 kg of hay a day in dry matter, plus water and a salt lick, and a native pony on good grass needs nothing else - which is why a fat pony is the one that gets laminitis in spring. Hard feed is for a pony in work, not for one standing in a field.")), (py_str("donkey"), py_str("Straw and hay, and less than a pony of the same size: a donkey has a slower digestive system and gets fat on good grass, so the work is keeping it thin with coarse hay and a bare paddock rather than feeding it up.")), (py_str("lizard"), py_str("Insects for nearly all of them - crickets, locusts and mealworms, dusted with a calcium powder - and the green ones take leaves and veg instead. It needs the right heat to digest at all, so the lamp matters as much as the food.")), (py_str("polar bear"), py_str("Seals, and mostly the ringed seal: a polar bear waits at a breathing hole and takes the seal as it surfaces, then eats the blubber first. It can go days between meals and swims long distances between patches of ice."))]) }
    s_EATS(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("ok to eat"), py_str("safe to eat"), py_str("still ok"), py_str("eat it"), py_str("to eat"), py_str("can i eat"), py_str("pink"), py_str("raw"), py_str("cooked"), py_str("undercooked"), py_str("dry"), py_str("left out"), py_str("out of date"), py_str("use by"), py_str("in the fridge"), py_str("in the freezer"), py_str("in the oven"), py_str("how long"), py_str("how many people"), py_str("whole"), py_str("roast"), py_str("recipe"), py_str("cook"), py_str("cooking"), py_str("thigh"), py_str("thighs"), py_str("breast"), py_str("breasts"), py_str("wing"), py_str("wings"), py_str("nugget"), py_str("nuggets"), py_str("stock"), py_str("soup"), py_str("bone"), py_str("bones"), py_str("gravy"), py_str("sandwich"), py_str("curry"), py_str("pie"), py_str("calories"), py_str("protein"), py_str("thaw"), py_str("dinner"), py_str("for tea"), py_str("for sunday"), py_str("feed my"), py_str("my dog"), py_str("my cat")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("ok to eat"), py_str("safe to eat"), py_str("still ok"), py_str("to eat"), py_str("pink"), py_str("raw"), py_str("cooked"), py_str("roast"), py_str("how many people"), py_str("whole"), py_str("in the oven"), py_str("how long"), py_str("how many eggs"), py_str("laying"), py_str("stop laying"), py_str("not laying"), py_str("broody"), py_str("sick"), py_str("ill"), py_str("unwell"), py_str("not eating")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("pork"), py_str("bacon"), py_str("ham"), py_str("gammon"), py_str("sausage"), py_str("sausages"), py_str("chop"), py_str("chops"), py_str("belly"), py_str("crackling"), py_str("roast"), py_str("in the oven"), py_str("how long"), py_str("ok to eat"), py_str("safe to eat"), py_str("pink")]) }
    s_EATS_ITSELF(py_dict(vec![(py_str("chicken"), __data5()), (py_str("hen"), __data6()), (py_str("pig"), __data7()), (py_str("goat"), py_tuple(vec![py_str("milk"), py_str("cheese"), py_str("curry"), py_str("in the oven"), py_str("how long"), py_str("ok to eat"), py_str("safe to eat"), py_str("get my goat"), py_str("got my goat"), py_str("scapegoat"), py_str("goatee")])), (py_str("pony"), py_tuple(vec![py_str("riding"), py_str("ride"), py_str("livery"), py_str("tack"), py_str("saddle"), py_str("how tall"), py_str("height"), py_str("shoes"), py_str("race"), py_str("racing"), py_str("bet"), py_str("betting")]))]));
    fn __data8() -> Py { py_tuple(vec![py_str("eat"), py_str("eats"), py_str("eaten"), py_str("eating"), py_str("food"), py_str("diet"), py_str("fed"), py_str("feed"), py_str("feeding"), py_str("hunt"), py_str("hunts"), py_str("hunting"), py_str("prey")]) }
    s_EAT_CUES(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("do"), py_str("does"), py_str("did"), py_str("should"), py_str("which"), py_str("how"), py_str("is"), py_str("are"), py_str("want to know"), py_str("tell me"), py_str("wondering")]) }
    s_EAT_ASK(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("should"), py_str("do"), py_str("does"), py_str("did"), py_str("which"), py_str("how"), py_str("any"), py_str("is"), py_str("are"), py_str("best"), py_str("good"), py_str("feed"), py_str("never"), py_str("not")]) }
    s_DIET_ASK(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("bin"), py_str("bins"), py_str("bin bag"), py_str("bin bags"), py_str("rubbish"), py_str("wheelie"), py_str("tip over"), py_str("knock over"), py_str("get rid"), py_str("keep away"), py_str("keep off"), py_str("keep out"), py_str("stop them"), py_str("repel"), py_str("traps"), py_str("trap"), py_str("poison"), py_str("shoot"), py_str("shooting"), py_str("licence"), py_str("protected"), py_str("lawn"), py_str("holes in the lawn"), py_str("holes in my lawn"), py_str("digging"), py_str("dig up"), py_str("dug up"), py_str("setts"), py_str("sett"), py_str("nest"), py_str("nests"), py_str("in my loft"), py_str("in the loft"), py_str("in the attic"), py_str("in my roof"), py_str("in the shed"), py_str("in my house"), py_str("in the kitchen"), py_str("droppings"), py_str("mice in"), py_str("rats in"), py_str("we have mice"), py_str("we have rats"), py_str("how do i"), py_str("how to get")]) }
    s_DIET_AVOID(__data11());
    fn __data12() -> Py { py_tuple(vec![py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my kitten"), py_str("my rabbit"), py_str("my guinea"), py_str("my hamster"), py_str("my horse"), py_str("my pony"), py_str("my tortoise"), py_str("my snake"), py_str("my beardie"), py_str("puppy"), py_str("kitten"), py_str("how much"), py_str("how often"), py_str("how many times"), py_str("portion"), py_str("portion size"), py_str("grams"), py_str("per day food"), py_str("safe to eat"), py_str("left out"), py_str("leftover"), py_str("leftovers"), py_str("reheat"), py_str("use by"), py_str("gone off"), py_str("out of date"), py_str("still ok")]) }
    s_EAT_AVOID(op_add(&g_DIET_AVOID(), &__data12()));
    fn __data13() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("fox"), py_str("foxes")]), g_DIET_CUES(), py_str("Almost anything, and mostly things we leave out: worms, beetles, slugs and grubs out of a lawn, small rodents, birds and their eggs, fruit and berries, and every kind of food waste - which is why a bin bag left out is the same meal, easier to reach. A cat is not on that list: an adult cat can look after itself and a fox backs off.")]), py_tuple(vec![py_tuple(vec![py_str("badger"), py_str("badgers")]), g_DIET_CUES(), py_str("Earthworms are most of it: a badger can eat well over a hundred in a night, which is why a lawn full of small conical holes is badger work rather than a pest. The rest is beetles and grubs, slugs, roots, windfall fruit, and now and then eggs or a chick, plus anything a bin bag offers.")]), py_tuple(vec![py_tuple(vec![py_str("squirrel"), py_str("squirrels"), py_str("grey squirrel"), py_str("grey squirrels")]), g_DIET_CUES(), py_str("Seeds and nuts first - acorns, hazelnuts, beech mast, sunflower seeds - then fungi, buds, shoots and fruit, and a bird feeder is the same food in an easier place. In autumn a grey squirrel buries thousands of nuts one at a time and remembers where most of them are.")]), py_tuple(vec![py_tuple(vec![py_str("owl"), py_str("owls"), py_str("tawny owl"), py_str("barn owl"), py_str("barn owls"), py_str("tawny owls")]), g_DIET_CUES(), py_str("Small mammals: a tawny owl lives on mice, voles and shrews, with the odd small bird, frog or beetle, and a barn owl is almost all voles. It swallows the prey whole and coughs up the fur and bones as a pellet, which is what you find under a roost.")]), py_tuple(vec![py_tuple(vec![py_str("deer"), py_str("deers"), py_str("roe deer"), py_str("muntjac"), py_str("fallow deer")]), g_DIET_CUES(), py_str("Grass, leaves, shoots, buds, ivy and brambles, and fruit in autumn - a deer browses at the height of a hedge rather than grazing it, which is why the roses are stripped to a line and the buds on your apple tree go first. A muntjac also eats bluebells and birds' eggs.")]), py_tuple(vec![py_tuple(vec![py_str("mouse"), py_str("mice"), py_str("house mouse"), py_str("house mice"), py_str("field mouse"), py_str("field mice")]), g_DIET_CUES(), py_str("Seeds and grain mostly, so a kitchen is a windfall: cereals, crumbs, nuts, chocolate, pet food, and the odd insect. A house mouse eats only about 3g a day and drinks very little, which is why a small spill keeps a family going.")]), py_tuple(vec![py_tuple(vec![py_str("rat"), py_str("rats"), py_str("brown rat"), py_str("brown rats")]), g_DIET_CUES(), py_str("Almost anything, about 25 to 30g a day, plus water: grain, fruit, scraps, pet food, eggs, even soap and candles. That range is why rat baiting works with something they already like, and why the bin and the bird feeder are the two places they go first.")]), py_tuple(vec![py_tuple(vec![py_str("frog"), py_str("frogs"), py_str("toad"), py_str("toads")]), g_DIET_CUES(), py_str("Moving prey it can fit in its mouth: slugs, snails, worms, beetles, flies, spiders and caterpillars, caught with a sticky tongue in about a tenth of a second. Tadpoles eat algae and plant matter instead, so a pond with a few leaves in it feeds them.")]), py_tuple(vec![py_tuple(vec![py_str("bat"), py_str("bats"), py_str("pipistrelle"), py_str("pipistrelles")]), g_DIET_CUES(), py_str("Insects, caught on the wing by echolocation: a single pipistrelle eats around 3,000 midges and mosquitoes in a night, which is why bats over a garden mean fewer bites. Fruit bats in the tropics eat fruit and nectar instead.")]), py_tuple(vec![py_tuple(vec![py_str("snake"), py_str("snakes"), py_str("grass snake"), py_str("grass snakes"), py_str("adder"), py_str("adders")]), g_DIET_CUES(), py_str("Whole prey, swallowed in one: a grass snake takes frogs, toads, newts, small fish, mice and the odd bird's egg, and an adder takes voles, mice, lizards and young birds. It can go days or weeks between meals, then sit still somewhere warm to digest.")]), py_tuple(vec![py_tuple(vec![py_str("seagull"), py_str("seagulls"), py_str("gull"), py_str("gulls"), py_str("herring gull"), py_str("herring gulls")]), g_DIET_CUES(), py_str("Anything it can win: fish and shellfish, worms and insects off a ploughed field, other birds' eggs and chicks, chips, and whatever the bin has. A herring gull will drop a shell on a car park to break it, which is the same trick, not a taste for cars.")]), py_tuple(vec![py_tuple(vec![py_str("pidgeon"), py_str("pidgeons"), py_str("pigeon"), py_str("pigeons"), py_str("wood pigeon"), py_str("wood pigeons")]), g_DIET_CUES(), py_str("Seeds and grain, then leaves, buds, berries and brassicas - a wood pigeon will strip a cabbage row - and the feral pigeons in town live on scraps and bread. Bread is not good for any of them: peas, oats or bird seed are the better thing to put down.")]), py_tuple(vec![py_tuple(vec![py_str("duck"), py_str("ducks"), py_str("mallard"), py_str("mallards")]), g_DIET_CUES(), py_str("Water plants and their seeds, grain, grass, insects, snails and the odd small fish. Bread is the thing to stop giving them: it fills a duck up without feeding it, and it fouls the water. Oats, peas, sweetcorn or bird seed do the job and float while they eat.")]), py_tuple(vec![py_tuple(vec![py_str("heron"), py_str("herons"), py_str("grey heron"), py_str("grey herons")]), g_DIET_CUES(), py_str("Fish and frogs first, then eels, water voles, insects, and in spring ducklings and moorhen chicks. A heron hunts by standing still in the shallows and striking, so a pond with a planted edge and deeper water gives the fish somewhere to hide.")]), py_tuple(vec![py_tuple(vec![py_str("swan"), py_str("swans"), py_str("mute swan"), py_str("mute swans")]), g_DIET_CUES(), py_str("Water plants pulled up from the bottom, plus grass, grain and the odd insect or snail. Bread is the wrong food: it is empty of much they need and it fouls the water, so grain, oats or shredded greens are the better offering.")]), py_tuple(vec![py_tuple(vec![py_str("butterfly"), py_str("butterflies"), py_str("caterpillar"), py_str("caterpillars")]), g_DIET_CUES(), py_str("Two different diets in one life: a caterpillar eats leaves, and each kind wants its own plant - stinging nettles for peacock and small tortoiseshell, holly for holly blue, buckthorn for brimstone - while the butterfly drinks nectar from flowers such as lavender, buddleia and sedum.")]), py_tuple(vec![py_tuple(vec![py_str("starling"), py_str("starlings")]), g_DIET_CUES(), py_str("Insects and worms from lawns and dung, then fruit, seeds and scraps; in autumn they switch to berries and orchards, which is when the murmurations gather before roosting. A starling will also take fat balls and kitchen scraps from a feeder.")]), py_tuple(vec![py_tuple(vec![py_str("robin"), py_str("robins")]), g_DIET_CUES(), py_str("Insects, worms, spiders and caterpillars for most of the year, then berries and fruit in winter when the ground is hard. That is the moment to put out mealworms, suet and a few sultanas, and a robin will take them from a hand within a couple of weeks.")]), py_tuple(vec![py_tuple(vec![py_str("magpie"), py_str("magpies")]), g_DIET_CUES(), py_str("Insects, carrion, seeds and fruit, plus eggs and nestlings in spring, which is why small birds mob one and why a garden with a magpie nesting goes quiet for a few weeks. It also takes scraps and pet food left outside.")])]) }
    s_DIETS(__data13());
    fn __data14() -> Py { py_tuple(vec![py_str("race"), py_str("racing"), py_str("racehorse"), py_str("bet"), py_str("betting"), py_str("each way"), py_str("riding"), py_str("livery"), py_str("tack"), py_str("shoe"), py_str("shoes"), py_str("height"), py_str("how tall")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("mad cow"), py_str("cow pat"), py_str("cow pats"), py_str("cow poo"), py_str("milk"), py_str("milking"), py_str("dairy"), py_str("cowbell"), py_str("beef"), py_str("steak"), py_str("steaks"), py_str("mince"), py_str("brisket"), py_str("roast"), py_str("in the oven"), py_str("how long"), py_str("ok to eat"), py_str("safe to eat"), py_str("pink"), py_str("protein"), py_str("calories")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("guinea"), py_str("guinea pig"), py_str("guinea pigs"), py_str("piggy bank"), py_str("pig out"), py_str("pig out on"), py_str("guinea-pig"), py_str("pork"), py_str("bacon"), py_str("ham"), py_str("gammon"), py_str("sausage"), py_str("sausages"), py_str("chop"), py_str("chops"), py_str("belly"), py_str("crackling"), py_str("roast"), py_str("in the oven"), py_str("how long"), py_str("ok to eat"), py_str("safe to eat"), py_str("pink")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("count"), py_str("counting"), py_str("counted"), py_str("sheepskin"), py_str("wool"), py_str("sheepdog"), py_str("sheep dog"), py_str("bollard"), py_str("black sheep"), py_str("lamb"), py_str("lamb chop"), py_str("lamb chops"), py_str("roast"), py_str("ok to eat"), py_str("safe to eat"), py_str("pink"), py_str("in the oven"), py_str("how long"), py_str("shoulder"), py_str("leg of")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("pox"), py_str("chicken pox"), py_str("chickenpox"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("raw"), py_str("recipe"), py_str("roast"), py_str("dinner"), py_str("thigh"), py_str("thighs"), py_str("breast"), py_str("breasts"), py_str("wing"), py_str("wings"), py_str("nugget"), py_str("nuggets"), py_str("stock"), py_str("soup"), py_str("bone"), py_str("bones"), py_str("gravy"), py_str("sandwich"), py_str("curry"), py_str("pie"), py_str("ok to eat"), py_str("safe to eat"), py_str("still ok"), py_str("eat it"), py_str("to eat"), py_str("can i eat"), py_str("pink"), py_str("dry"), py_str("left out"), py_str("out of date"), py_str("use by"), py_str("in the fridge"), py_str("in the freezer"), py_str("in the oven"), py_str("how long"), py_str("how many people"), py_str("whole"), py_str("for tea"), py_str("for dinner"), py_str("for sunday"), py_str("feed my"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("calories"), py_str("protein"), py_str("thaw")]) }
    s_FARM_DIETS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("horse"), py_str("horses"), py_str("pony"), py_str("ponies")]), g_DIET_CUES(), py_str("Grass and hay, roughly 2 per cent of its body weight a day in dry matter: a 500 kg horse gets through about 10 kg of hay plus water and a salt lick, and hard feed such as oats or barley is for work rather than the diet. Lush spring grass holds enough sugar to bring on laminitis, so a fat pony is the one to ration."), __data14()]), py_tuple(vec![py_tuple(vec![py_str("cow"), py_str("cows"), py_str("cattle"), py_str("heifer"), py_str("heifers"), py_str("bull"), py_str("bulls")]), g_DIET_CUES(), py_str("Grass and silage, in bulk: a dairy cow eats 50 to 60 kg of fresh grass or silage a day and drinks 60 to 100 litres of water. The rumen and its bugs are what break the grass down, which is why a little grain is a supplement and not a diet. Kitchen scraps are illegal to feed to farm animals in the UK, and have been since 2001."), __data15()]), py_tuple(vec![py_tuple(vec![py_str("pig"), py_str("pigs"), py_str("piglet"), py_str("piglets"), py_str("sow"), py_str("boar")]), g_DIET_CUES(), py_str("Almost anything, which is what an omnivore means: a pig eats 2 to 3 kg of feed a day, and roots for grubs, roots and fallen fruit with a nose built to dig. It cannot sweat, so it wallows to cool down. Kitchen scraps and any meat are illegal to feed to a pig in the UK, and that rule is what keeps swine fever out."), __data16()]), py_tuple(vec![py_tuple(vec![py_str("sheep"), py_str("ewe"), py_str("ewes"), py_str("lamb"), py_str("lambs")]), g_DIET_CUES(), py_str("Grass, and mostly grass: a ewe eats about 2 per cent of her body weight in dry matter a day, so 1.5 to 2 kg, plus a mineral lick and clean water. Late pregnancy wants more, and very rich spring grass or a sudden feed change is what brings on twin lamb disease and bloat, so any change is made over a week."), __data17()]), py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("chickens"), py_str("hen"), py_str("hens"), py_str("cockerel"), py_str("cockerels")]), g_DIET_CUES(), py_str("Layers pellets or mash as the base, about 120 g a day per hen, plus grass, weeds, insects and vegetable scraps; grit and crushed oyster shell keep the eggshells hard. Bread, pasta and corn are treats rather than feed, and avocado, citrus peel, salt and anything mouldy are not for them at all."), __data18()]), py_tuple(vec![py_tuple(vec![py_str("goat"), py_str("goats"), py_str("kid goat"), py_str("kid goats"), py_str("baby goat"), py_str("baby goats"), py_str("young goat"), py_str("young goats"), py_str("billy goat"), py_str("nanny goat")]), g_DIET_CUES(), py_str("Browse first, not grass: brambles, hedge, leaves, bark, weeds and thistles, then hay - about 2 kg of hay a day plus a mineral lick and water. A goat is a browser with a stomach for roughage, so rich grass or a lot of grain makes it ill, and it will strip a garden to the fence line given the chance."), py_tuple(vec![py_str("get my goat"), py_str("got my goat"), py_str("goatee"), py_str("scapegoat"), py_str("kids party"), py_str("goat curry"), py_str("goat cheese"), py_str("goats cheese"), py_str("milk"), py_str("in the oven"), py_str("how long"), py_str("ok to eat")])]), py_tuple(vec![py_tuple(vec![py_str("lion"), py_str("lions"), py_str("lioness"), py_str("lionesses")]), g_DIET_CUES(), py_str("Meat, and plenty of it: a lion averages 5 to 7 kg a day, taken from zebra, wildebeest, buffalo and smaller prey, and it can go several days between kills and then eat 30 kg at one sitting. The lionesses do most of the hunting, and the pride shares the carcass."), py_tuple(vec![py_str("lion bar"), py_str("lions club"), py_str("mane"), py_str("roar"), py_str("roaring"), py_str("tattoo")])]), py_tuple(vec![py_tuple(vec![py_str("elephant"), py_str("elephants")]), g_DIET_CUES(), py_str("Plants, in enormous quantity: an adult elephant eats 100 to 150 kg of grass, leaves, bark and fruit a day and drinks 100 to 200 litres of water, so it spends 16 hours a day feeding and travels to find it. Its dung is a seed bank, and a forest grows along the paths it makes."), py_tuple(vec![py_str("white elephant"), py_str("elephant in the room"), py_str("ivory"), py_str("tusk"), py_str("tusks"), py_str("pregnant"), py_str("pregnancy")])]), py_tuple(vec![py_tuple(vec![py_str("giraffe"), py_str("giraffes")]), g_DIET_CUES(), py_str("Acacia leaves, browsed four to five metres up: a giraffe eats up to 45 kg of leaves a day with a 45 cm tongue and lips tough enough for the thorns, and it drinks only every few days because the leaves hold so much water. It spreads its front legs to reach a waterhole."), py_tuple(vec![py_str("neck"), py_str("how tall"), py_str("height"), py_str("tall")])])]));
    fn __data19() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("bat"), py_str("bats")]), py_tuple(vec![py_str("blind"), py_str("blindness"), py_str("see"), py_str("seeing"), py_str("sight")]), py_str("Bats are not blind: they see about as well as a cat in dim light. What makes them special is echolocation, squeaks too high to hear that bounce back and tell them where things are in the dark.")]), py_tuple(vec![py_tuple(vec![py_str("zebra"), py_str("zebras")]), py_tuple(vec![py_str("stripe"), py_str("stripes"), py_str("striped")]), py_str("Nobody knows for certain, and the best evidence points at flies and heat rather than camouflage: the stripes seem to confuse biting flies and help the animal lose heat, and the pattern on each zebra is as individual as a fingerprint.")]), py_tuple(vec![py_tuple(vec![py_str("penguin"), py_str("penguins")]), py_tuple(vec![py_str("north pole"), py_str("arctic"), py_str("south pole"), py_str("antarctic"), py_str("where"), py_str("where do penguins live"), py_str("which pole")]), py_str("Penguins live in the southern half of the world: Antarctica, and the coasts of South America, Africa, Australia and New Zealand. There are none at the North Pole; the birds there are puffins, which are not penguins.")]), py_tuple(vec![py_tuple(vec![py_str("penguin"), py_str("penguins")]), py_tuple(vec![py_str("hold their breath"), py_str("hold its breath"), py_str("breath"), py_str("dive"), py_str("diving"), py_str("deep"), py_str("swim"), py_str("swimming")]), py_str("An emperor penguin can hold its breath for around 20 minutes and dive past 500 metres; most dives are one to five minutes and less than 100 metres.")]), py_tuple(vec![py_tuple(vec![py_str("penguin"), py_str("penguins")]), py_tuple(vec![py_str("warm"), py_str("cold"), py_str("freeze"), py_str("survive"), py_str("huddle"), py_str("huddles")]), py_str("Four things: a layer of blubber, feathers packed so tightly that water never reaches the skin, feet and flippers with a heat-exchange that keeps the blood warm, and huddling. In a huddle each bird takes a turn on the freezing outside.")]), py_tuple(vec![py_tuple(vec![py_str("ant"), py_str("ants")]), py_tuple(vec![py_str("winter"), py_str("cold"), py_str("survive"), py_str("survives"), py_str("hibernate"), py_str("hibernation")]), py_str("They slow down and go deep: a colony moves to the lowest, frost-free part of the nest, clusters together, and eats stored food, with the queen in the middle. They do not truly hibernate, they just keep still until it warms up.")]), py_tuple(vec![py_tuple(vec![py_str("ant"), py_str("ants")]), py_tuple(vec![py_str("strong"), py_str("stronger"), py_str("strength"), py_str("carry"), py_str("lift"), py_str("how much can")]), py_str("An ant carries 10 to 50 times its own body weight, and holds on with feet that grip smooth surfaces, which is why one can walk off with a whole crisp.")]), py_tuple(vec![py_tuple(vec![py_str("octopus"), py_str("octopuses"), py_str("octopi")]), py_tuple(vec![py_str("heart"), py_str("hearts"), py_str("how many")]), py_str("Three: two pump blood through the gills and one pumps it round the rest of the body. Their blood is blue, because it uses copper rather than iron to carry oxygen.")]), py_tuple(vec![py_tuple(vec![py_str("octopus"), py_str("octopuses"), py_str("octopi")]), py_tuple(vec![py_str("arm"), py_str("arms"), py_str("leg"), py_str("legs"), py_str("how many")]), py_str("Eight arms, and two rows of suckers on each, about 2,000 suckers in all. Each one can taste what it touches.")]), py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("fishes"), py_str("goldfish")]), py_tuple(vec![py_str("breathe"), py_str("breathing"), py_str("gills"), py_str("underwater"), py_str("under water"), py_str("oxygen"), py_str("need oxygen"), py_str("oxygen in the water"), py_str("air")]), py_str("They take oxygen out of the water instead of the air: water goes in at the mouth, passes over the gills, and the gills pull the oxygen out and pass the carbon dioxide back. A few fish, like bettas and gouramis, can also gulp air from the surface.")]), py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("fishes")]), py_tuple(vec![py_str("feel pain"), py_str("pain"), py_str("suffer"), py_str("suffering")]), py_str("The evidence says probably yes: fish have nerves and opioid receptors, and they learn to avoid a place where they were hurt. Scientists argued about it for years, and it is why handling rules for angling have tightened.")]), py_tuple(vec![py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies")]), py_tuple(vec![py_str("wag"), py_str("wagging"), py_str("wags"), py_str("tail"), py_str("tails")]), py_str("It is a signal, and the tail is not always saying the same thing: a loose, wide wag with a loose body means happy, a fast low wag can mean nerves, and a stiff high wag is usually a warning. Studies find a wag that swings more to the dog's right goes with a friendly meeting.")]), py_tuple(vec![py_tuple(vec![py_str("spider"), py_str("spiders")]), py_tuple(vec![py_str("how many legs"), py_str("legs"), py_str("eyes"), py_str("how many eyes")]), py_str("Spiders have eight legs and usually eight eyes, in rows across the front of the head. They have silk glands in the abdomen and spin the web out with their back legs.")]), py_tuple(vec![py_tuple(vec![py_str("bat"), py_str("bats")]), py_tuple(vec![py_str("sleep"), py_str("sleeps"), py_str("hibernate"), py_str("hibernation"), py_str("upside down")]), py_str("Bats sleep upside down, hanging by their feet, and many hibernate through the winter that way. Hanging upside down means a fall is a drop of a few centimetres, and taking off is just letting go.")]), py_tuple(vec![py_tuple(vec![py_str("beaver"), py_str("beavers")]), py_tuple(vec![py_str("dam"), py_str("dams"), py_str("why"), py_str("build")]), py_str("A beaver builds a dam to hold the water deep enough that the entrance to its lodge is under the surface, which keeps out foxes, wolves and bears. The pond also means it can swim to food in winter instead of walking over land.")]), py_tuple(vec![py_tuple(vec![py_str("swallow"), py_str("swallows"), py_str("swift"), py_str("swifts")]), py_tuple(vec![py_str("sleep"), py_str("sleeps"), py_str("fly"), py_str("flying"), py_str("land")]), py_str("Swifts sleep on the wing, climbing to where they can glide slowly for hours at a time. Some stay airborne for months, and they only land to nest.")]), py_tuple(vec![py_tuple(vec![py_str("chameleon"), py_str("chameleons")]), py_tuple(vec![py_str("change colour"), py_str("colour"), py_str("color"), py_str("why")]), py_str("They stretch or squeeze layers of pigment cells in the skin. It is mostly mood and temperature rather than camouflage: a calm chameleon is often green, and a cross one goes bright or dark.")]), py_tuple(vec![py_tuple(vec![py_str("elephant"), py_str("elephants")]), py_tuple(vec![py_str("trunk"), py_str("why"), py_str("how many bones")]), py_str("An elephant's trunk has no bones at all and around 40,000 muscles, which is why it can pick up a single peanut or lift a tree. Elephants greet by touching trunks.")]), py_tuple(vec![py_tuple(vec![py_str("hibernate"), py_str("hibernates"), py_str("hibernating"), py_str("hibernation")]), py_tuple(vec![py_str("why"), py_str("how"), py_str("animals"), py_str("bear"), py_str("bears"), py_str("hedgehog"), py_str("survive"), py_str("winter")]), py_str("Hibernation is a way of getting through a season with no food: the animal's temperature, heart rate and breathing drop, and it lives off the fat it laid down in autumn. A bear's heart can slow from 40 beats a minute to 8, and a hedgehog turns almost cold to the touch. A bat or a hedgehog woken in winter has spent a lot of energy for nothing, so leave one be.")])]) }
    s_CURIOSITIES(__data19());
    fn __data20() -> Py { py_tuple(vec![py_dict(vec![(py_str("names"), py_tuple(vec![py_str("goat"), py_str("goats")])), (py_str("young"), py_str("kid")), (py_str("female"), py_str("doe")), (py_str("male"), py_str("buck")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("cow"), py_str("cows"), py_str("cattle")])), (py_str("young"), py_str("calf")), (py_str("female"), py_str("cow")), (py_str("male"), py_str("bull")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("sheep"), py_str("ewe"), py_str("ewes")])), (py_str("young"), py_str("lamb")), (py_str("female"), py_str("ewe")), (py_str("male"), py_str("ram")), (py_str("group"), py_str("flock"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("pig"), py_str("pigs")])), (py_str("young"), py_str("piglet")), (py_str("female"), py_str("sow")), (py_str("male"), py_str("boar")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("horse"), py_str("horses")])), (py_str("young"), py_str("foal")), (py_str("female"), py_str("mare")), (py_str("male"), py_str("stallion")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("donkey"), py_str("donkeys")])), (py_str("young"), py_str("foal")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("dog"), py_str("dogs")])), (py_str("young"), py_str("puppy")), (py_str("female"), py_str("bitch")), (py_str("male"), py_str("stud")), (py_str("group"), py_str("pack"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("cat"), py_str("cats")])), (py_str("young"), py_str("kitten")), (py_str("female"), py_str("queen")), (py_str("male"), py_str("tom")), (py_str("group"), py_str("clowder"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("chicken"), py_str("chickens"), py_str("hen"), py_str("hens")])), (py_str("young"), py_str("chick")), (py_str("female"), py_str("hen")), (py_str("male"), py_str("cockerel")), (py_str("group"), py_str("flock"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("duck"), py_str("ducks")])), (py_str("young"), py_str("duckling")), (py_str("female"), py_str("duck")), (py_str("male"), py_str("drake")), (py_str("group"), py_str("raft"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("goose"), py_str("geese")])), (py_str("young"), py_str("gosling")), (py_str("group"), py_str("gaggle"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("swan"), py_str("swans")])), (py_str("young"), py_str("cygnet")), (py_str("female"), py_str("pen")), (py_str("male"), py_str("cob")), (py_str("group"), py_str("bevy"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("turkey"), py_str("turkeys")])), (py_str("young"), py_str("poult")), (py_str("group"), py_str("rafter"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("pigeon"), py_str("pigeons")])), (py_str("young"), py_str("squab")), (py_str("group"), py_str("flock"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("crow"), py_str("crows")])), (py_str("group"), py_str("murder"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("raven"), py_str("ravens")])), (py_str("group"), py_str("unkindness"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("owl"), py_str("owls")])), (py_str("young"), py_str("owlet")), (py_str("group"), py_str("parliament"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("eagle"), py_str("eagles")])), (py_str("young"), py_str("eaglet")), (py_str("group"), py_str("convocation"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("fox"), py_str("foxes")])), (py_str("young"), py_str("cub")), (py_str("female"), py_str("vixen")), (py_str("male"), py_str("dog")), (py_str("group"), py_str("skulk"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("wolf"), py_str("wolves")])), (py_str("young"), py_str("cub")), (py_str("group"), py_str("pack"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("deer")])), (py_str("young"), py_str("fawn")), (py_str("female"), py_str("doe")), (py_str("male"), py_str("stag")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rabbit"), py_str("rabbits")])), (py_str("young"), py_str("kit")), (py_str("group"), py_str("colony"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("hare"), py_str("hares")])), (py_str("young"), py_str("leveret")), (py_str("group"), py_str("husk"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("mouse"), py_str("mice")])), (py_str("young"), py_str("pup")), (py_str("group"), py_str("mischief"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rat"), py_str("rats")])), (py_str("young"), py_str("pup")), (py_str("group"), py_str("mischief"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("squirrel"), py_str("squirrels")])), (py_str("young"), py_str("kitten")), (py_str("group"), py_str("scurry"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("hedgehog"), py_str("hedgehogs")])), (py_str("young"), py_str("hoglet"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("lion"), py_str("lions")])), (py_str("young"), py_str("cub")), (py_str("group"), py_str("pride"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("tiger"), py_str("tigers")])), (py_str("young"), py_str("cub")), (py_str("group"), py_str("streak"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("bear"), py_str("bears")])), (py_str("young"), py_str("cub")), (py_str("group"), py_str("sleuth"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("elephant"), py_str("elephants")])), (py_str("young"), py_str("calf")), (py_str("group"), py_str("herd"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("kangaroo"), py_str("kangaroos")])), (py_str("young"), py_str("joey")), (py_str("group"), py_str("mob"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("camel"), py_str("camels")])), (py_str("young"), py_str("calf")), (py_str("group"), py_str("caravan"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("seal"), py_str("seals")])), (py_str("young"), py_str("pup")), (py_str("group"), py_str("colony"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("penguin"), py_str("penguins")])), (py_str("young"), py_str("chick")), (py_str("group"), py_str("waddle"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("ostrich"), py_str("ostriches")])), (py_str("young"), py_str("chick")), (py_str("group"), py_str("flock"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("bat"), py_str("bats")])), (py_str("young"), py_str("pup")), (py_str("group"), py_str("colony"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("frog"), py_str("frogs")])), (py_str("young"), py_str("tadpole")), (py_str("group"), py_str("army"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("snake"), py_str("snakes")])), (py_str("young"), py_str("hatchling")), (py_str("group"), py_str("nest"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("turtle"), py_str("turtles"), py_str("tortoise"), py_str("tortoises")])), (py_str("young"), py_str("hatchling")), (py_str("group"), py_str("bale"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("spider"), py_str("spiders")])), (py_str("young"), py_str("spiderling")), (py_str("group"), py_str("cluster"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("bee"), py_str("bees")])), (py_str("young"), py_str("larva")), (py_str("group"), py_str("colony"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("ant"), py_str("ants")])), (py_str("young"), py_str("larva")), (py_str("group"), py_str("colony"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("butterfly"), py_str("butterflies")])), (py_str("young"), py_str("caterpillar")), (py_str("group"), py_str("kaleidoscope"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("fish"), py_str("fishes")])), (py_str("young"), py_str("fry")), (py_str("group"), py_str("shoal"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("whale"), py_str("whales")])), (py_str("young"), py_str("calf")), (py_str("group"), py_str("pod"))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("dolphin"), py_str("dolphins")])), (py_str("young"), py_str("calf")), (py_str("group"), py_str("pod"))])]) }
    s_WORD_ROWS(__data20());
    s__WORD_CUES(py_dict(vec![(py_str("young"), py_tuple(vec![py_str("baby"), py_str("young"), py_str("newborn")])), (py_str("group"), py_tuple(vec![py_str("group"), py_str("collective noun"), py_str("bunch"), py_str("gathering")])), (py_str("female"), py_tuple(vec![py_str("female"), py_str("girl"), py_str("lady"), py_str("mother"), py_str("mum")])), (py_str("male"), py_tuple(vec![py_str("male"), py_str("boy"), py_str("man"), py_str("father"), py_str("dad")]))]));
    s__WORD_ASK(py_tuple(vec![py_str("called"), py_str("call"), py_str("word for"), py_str("name for"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("what do you"), py_str("which word"), py_str("term for"), py_str("collective noun")]));
    fn __data21() -> Py { py_tuple(vec![py_str("look after"), py_str("care for"), py_str("care of"), py_str("feed"), py_str("fed"), py_str("feeding"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("ate"), py_str("diet"), py_str("food"), py_str("what to feed"), py_str("how much"), py_str("cost"), py_str("price"), py_str("buy"), py_str("adopt"), py_str("keep"), py_str("keeping"), py_str("train"), py_str("vet"), py_str("pregnant"), py_str("sick"), py_str("ill"), py_str("how long"), py_str("how many babies"), py_str("live"), py_str("lives"), py_str("lifespan"), py_str("how do i"), py_str("should i")]) }
    s__WORD_AVOID(__data21());
    s__WORD_SENTENCE(py_dict(vec![(py_str("young"), py_str("A young {one} is {article} {word}.")), (py_str("group"), py_str("A group of {many} is {article} {word}.")), (py_str("female"), py_str("A female {one} is {article} {word}.")), (py_str("male"), py_str("A male {one} is {article} {word}."))]));
    fn __data22() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("gestation"), py_str("how long is a"), py_str("how long are"), py_str("how long is my"), py_str("how long do"), py_str("how many days"), py_str("how many weeks"), py_str("how many months"), py_str("due date"), py_str("when is she due"), py_str("when will she"), py_str("how far along"), py_str("give birth"), py_str("in pup"), py_str("is due"), py_str("due date"), py_str("due any day"), py_str("due next week"), py_str("due in"), py_str("due soon"), py_str("when is my"), py_str("when will my"), py_str("in lamb"), py_str("in foal"), py_str("in kid"), py_str("expecting"), py_str("kindle"), py_str("farrow"), py_str("whelp"), py_str("foal")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("bake"), py_str("baked"), py_str("boil"), py_str("boiled"), py_str("poach"), py_str("poached"), py_str("fry"), py_str("fried"), py_str("grill"), py_str("grilled"), py_str("oven"), py_str("air fryer"), py_str("slow cooker"), py_str("barbecue"), py_str("bbq"), py_str("thaw"), py_str("thawed"), py_str("defrost"), py_str("defrosted"), py_str("reheat"), py_str("reheated"), py_str("stuffing"), py_str("carve"), py_str("carving"), py_str("leftovers"), py_str("per kg"), py_str("per 500g"), py_str("per kilo"), py_str("joint"), py_str("recipe"), py_str("sausage"), py_str("mince"), py_str("minced"), py_str("fillet"), py_str("breast"), py_str("thigh"), py_str("brisket"), py_str("steak"), py_str("chop"), py_str("chops"), py_str("leg"), py_str("legs"), py_str("wing"), py_str("wings"), py_str("drumstick"), py_str("drumsticks"), py_str("simmer"), py_str("simmering"), py_str("stew"), py_str("stewed"), py_str("soup"), py_str("stock"), py_str("curry"), py_str("casserole"), py_str("pie"), py_str("dinner"), py_str("supper"), py_str("sandwich"), py_str("how long do i need for"), py_str("time for the"), py_str("i am pregnant"), py_str("im pregnant"), py_str("i'm pregnant"), py_str("my wife"), py_str("my girlfriend"), py_str("my partner"), py_str("my husband"), py_str("human"), py_str("trying for a baby"), py_str("ivf"), py_str("period"), py_str("my period"), py_str("sick"), py_str("vomiting"), py_str("not eating"), py_str("losing weight"), py_str("bleeding"), py_str("water broke"), py_str("waters broke"), py_str("contractions"), py_str("live"), py_str("lives"), py_str("lived"), py_str("lifespan"), py_str("life span"), py_str("life expectancy"), py_str("how old"), py_str("weigh"), py_str("weighs"), py_str("weight"), py_str("big"), py_str("large"), py_str("size"), py_str("tall"), py_str("go out"), py_str("walks"), py_str("walk"), py_str("jabs"), py_str("vaccine"), py_str("vaccinated"), py_str("vaccinations"), py_str("training"), py_str("lead"), py_str("recall"), py_str("season"), py_str("seasons"), py_str("in season"), py_str("into season"), py_str("come into"), py_str("came into"), py_str("on heat"), py_str("in heat"), py_str("heat"), py_str("split heat"), py_str("silent heat")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("big"), py_str("bigger"), py_str("biggest"), py_str("large"), py_str("largest"), py_str("weigh"), py_str("weighs"), py_str("weighing"), py_str("weight"), py_str("heavy"), py_str("heavier"), py_str("heaviest"), py_str("tall"), py_str("taller"), py_str("tallest"), py_str("long"), py_str("length"), py_str("size"), py_str("much does"), py_str("how big"), py_str("how large")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("live"), py_str("lives"), py_str("lifespan"), py_str("life span"), py_str("how old"), py_str("how many years"), py_str("trunk"), py_str("trunks"), py_str("tail"), py_str("tails"), py_str("tongue"), py_str("neck"), py_str("bone"), py_str("bones"), py_str("horn"), py_str("horns"), py_str("tooth"), py_str("teeth"), py_str("tusk"), py_str("tusks")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("fastest"), py_str("biggest"), py_str("largest"), py_str("tallest"), py_str("heaviest"), py_str("strongest"), py_str("weakest"), py_str("longest lived"), py_str("longest living"), py_str("longest-lived"), py_str("oldest"), py_str("most dangerous"), py_str("lives the longest"), py_str("live the longest"), py_str("lives longest"), py_str("longest life"), py_str("deadliest")]) }
    s_ENTRIES(op_add(&op_add(&op_add(&op_add(&py_list(vec![pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("animal-lifespan"), py_tuple(vec![_names_with(&[py_str("life")]), py_tuple(vec![py_str("live"), py_str("lives"), py_str("lived"), py_str("lifespan"), py_str("life span"), py_str("life expectancy"), py_str("how old do"), py_str("how old can"), py_str("how many years")])]), _fact(&[py_str("life")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal-gestation")); __args.push(py_tuple(vec![_names_with(&[py_str("gestation")]), __data22()])); __args.push(_fact(&[py_str("gestation")])); __kws.extend(vec![("avoid".to_string(), __data23()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal-size")); __args.push(py_tuple(vec![_names_with(&[py_str("size")]), __data24()])); __args.push(_fact(&[py_str("size")])); __kws.extend(vec![("avoid".to_string(), __data25())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("animal-speed"), py_tuple(vec![_names_with(&[py_str("speed")]), py_tuple(vec![py_str("fast"), py_str("faster"), py_str("fastest"), py_str("run"), py_str("runs"), py_str("running"), py_str("speed"), py_str("quick"), py_str("quickly"), py_str("how fast")])]), _fact(&[py_str("speed")])]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("animal-record"), py_tuple(vec![__data26(), py_tuple(vec![py_str("animal"), py_str("animals"), py_str("creature"), py_str("creatures")])]), py_func("_record", |args: &[Py]| _record(args))]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal-eats")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ANIMALS()) { let mut row: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&row.clone(), &py_str("names"))) { let mut name: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&row.clone(), &py_str("names"))) { let mut n: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_EATS(), &n.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(name.clone()); } } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g_EAT_CUES()])); __args.push(py_func("_eats", |args: &[Py]| _eats(args))); __kws.extend(vec![("avoid".to_string(), g_EAT_AVOID()), ("bias".to_string(), Py::Int(1)), ("ask".to_string(), g_EAT_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("animal-record-bird"), py_tuple(vec![py_tuple(vec![py_str("biggest"), py_str("largest"), py_str("heaviest"), py_str("tallest"), py_str("fastest")]), py_tuple(vec![py_str("bird"), py_str("birds")])]), py_func("_record", |args: &[Py]| _record(args))]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("animal-record-fish"), py_tuple(vec![py_tuple(vec![py_str("fastest"), py_str("biggest"), py_str("largest")]), py_tuple(vec![py_str("fish"), py_str("fishes"), py_str("shark"), py_str("sharks")])]), py_func("_record", |args: &[Py]| _record(args))])]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CURIOSITIES()) { let __unpk3597 = py_unpack(&__item.clone(), 3, None); let mut animal: Py = __unpk3597[0].clone(); let mut cue: Py = __unpk3597[1].clone(); let mut reply: Py = __unpk3597[2].clone(); __out.push(_curiosity(&[animal.clone(), cue.clone(), reply.clone()])); } py_list(__out) })() }), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DIETS()) { let __unpk4941 = py_unpack(&__item.clone(), 3, None); let mut animal: Py = __unpk4941[0].clone(); let mut cue: Py = __unpk4941[1].clone(); let mut reply: Py = __unpk4941[2].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(animal.clone()); __args.push(cue.clone()); __args.push(reply.clone()); __args.push(g_DIET_AVOID()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), g_DIET_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _curiosity(args))(&__args) }); } py_list(__out) })() }), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FARM_DIETS()) { let __unpk3966 = py_unpack(&__item.clone(), 4, None); let mut animal: Py = __unpk3966[0].clone(); let mut cue: Py = __unpk3966[1].clone(); let mut reply: Py = __unpk3966[2].clone(); let mut own: Py = __unpk3966[3].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(animal.clone()); __args.push(cue.clone()); __args.push(reply.clone()); __args.push(op_add(&g_DIET_AVOID(), &own.clone())); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), g_DIET_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _curiosity(args))(&__args) }); } py_list(__out) })() }), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("young"), py_str("group"), py_str("female"), py_str("male")])) { let mut kind: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("animal-word-"), &kind.clone())); __args.push(py_tuple(vec![_names_with(&[kind.clone(), g_WORD_ROWS()]), geti(&g__WORD_CUES(), &kind.clone())])); __args.push(_the_word(&[kind.clone()])); __kws.extend(vec![("avoid".to_string(), g__WORD_AVOID()), ("ask".to_string(), g__WORD_ASK()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() }));
}
pub fn _article(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("A or an, from the letter the word starts with: \"an unkindness\", \"a murder\".");
    return if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &mcall(&slice_or_raise(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(1), &Py::None), "lower", &[])))) { py_str("an") } else { py_str("a") };
    Py::None
}
pub fn _line(args: &[Py]) -> Py {
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
    let _ = py_str("The repaired line, lower case, with a space at each end, for a whole-word search.");
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    return op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _named(args: &[Py]) -> Py {
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
    let mut rows: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rows") { v.clone() }
    else { g_ANIMALS() };
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The animal row the line names, or None.\n\n    The longest matching name wins, so \"blue whale\" is read as the blue whale and not as the\n    whale row, and \"african elephant\" as the elephant.\n    ");
    text = _line(&[understood.clone()]);
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, py_str("")]), 2, None);
    found = __unpk1[0].clone();
    best = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&rows.clone());
    for __item in __iter2 {
    row = __item.clone();
    let __iter3 = py_iter_or_raise(&geti(&row.clone(), &py_str("names")));
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![row.clone(), name.clone()]), 2, None);
    found = __unpk4[0].clone();
    best = __unpk4[1].clone();
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _names_with(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fact: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fact") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fact"))) };
    let mut rows: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rows") { v.clone() }
    else { g_ANIMALS() };
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("Every animal name whose row has this fact, for an entry's key.");
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rows.clone()) { let mut row: Py = __item.clone(); if py_truthy(&mcall(&row.clone(), "get", &[fact.clone()])) {  for __item in py_iter_or_raise(&geti(&row.clone(), &py_str("names"))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("The word for this kind of name, from the row the line names, or None.\n\n    The reply is the row's value in one sentence, so no phrasing is written twice and a row that\n    lacks the fact (\"a hare has no word for a male here\") passes the line on rather than guessing.\n    ");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut kind: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut many: Py = Py::None;
    let mut names: Py = Py::None;
    let mut row: Py = Py::None;
    let mut word: Py = Py::None;
    row = _named(&[understood.clone(), g_WORD_ROWS()]);
    word = if py_truthy(&row.clone()) { mcall(&row.clone(), "get", &[kind.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::None;
    }
    names = geti(&row.clone(), &py_str("names"));
    many = if py_truthy(&op_gt(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&names.clone(), &Py::Int(1)) } else { op_add(&geti(&names.clone(), &Py::Int(0)), &py_str("s")) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("one".to_string(), geti(&names.clone(), &Py::Int(0))), ("many".to_string(), many.clone()), ("word".to_string(), word.clone()), ("article".to_string(), _article(&[word.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g__WORD_SENTENCE(), &kind.clone()), "format", args))(&__args) };
    Py::None
}
    reply = {{ let kind = kind.clone(); let reply = reply.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![kind.clone(), reply.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _fact(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fact: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fact") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fact"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("A reply from the animal row the line names, or None when the row does not have it.");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut fact: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut row: Py = Py::None;
    row = _named(&[understood.clone()]);
    return if py_truthy(&row.clone()) { mcall(&row.clone(), "get", &[fact.clone()]) } else { Py::None };
    Py::None
}
    reply = {{ let fact = fact.clone(); let reply = reply.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![fact.clone(), reply.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _eats(args: &[Py]) -> Py {
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
    let mut fence: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The food for the animal the line names, or None when its row has no line about food.");
    row = _named(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&row.clone()))) {
    return Py::None;
    }
    text = _line(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&geti(&row.clone(), &py_str("names")));
    for __item in __iter1 {
    name = __item.clone();
    fence = mcall(&g_EATS_ITSELF(), "get", &[name.clone()]);
    if py_truthy(&{ let __b = fence.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fence.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_EATS(), &name.clone()))) {
    return geti(&g_EATS(), &name.clone());
    }
    }
    return Py::None;
    Py::None
}
pub fn _record(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut what: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The record the line asks about: what it is the most of, and where.");
    text = _line(&[understood.clone()]);
    where_ = py_str("any");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bbird|birds\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    where_ = py_str("bird");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(fish|sea|ocean|water)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    where_ = py_str("sea");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(land|on land)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    where_ = py_str("land");
    }
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(longest lived|longest living|longest-lived|oldest|longest|lives the longest|live the longest)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    what = py_str("life");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(fastest|quickest|speed)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    what = py_str("speed");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\btall\\b|\\btallest\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    what = py_str("tall");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(strongest|weakest)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    what = if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("strong")))) { py_str("strong") } else { py_str("weak") };
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(dangerous|deadliest|kill)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    what = py_str("danger");
    } else {
    what = py_str("big");
    }
    }
    }
    }
    }
    return { let __b = mcall(&g_RECORDS(), "get", &[py_tuple(vec![what.clone(), where_.clone()])]); if py_truthy(&__b) { __b } else { mcall(&g_RECORDS(), "get", &[py_tuple(vec![what.clone(), py_str("any")])]) } };
    Py::None
}
pub fn _curiosity(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut animal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut avoid: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "avoid") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    let mut ask: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_tuple(vec![]) };
    let _ = py_str("One Entry per curiosity row, keyed by the animal and the words that ask it.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("animal-"), &stdlib_call("re", "sub", &[py_str("[^a-z0-9]+"), py_str("-"), op_add(&op_add(&geti(&animal.clone(), &Py::Int(0)), &py_str("-")), &geti(&cue.clone(), &Py::Int(0)))]).unwrap_or_else(|e| exc::raise(e)))); __args.push(py_tuple(vec![animal.clone(), cue.clone()])); __args.push(reply.clone()); __kws.extend(vec![("avoid".to_string(), avoid.clone()), ("bias".to_string(), bias.clone()), ("ask".to_string(), ask.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.animals".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANIMALS" => Some(g_ANIMALS()),
            "ANSWERS" => Some(g_ANSWERS()),
            "CURIOSITIES" => Some(g_CURIOSITIES()),
            "DIETS" => Some(g_DIETS()),
            "DIET_ASK" => Some(g_DIET_ASK()),
            "DIET_AVOID" => Some(g_DIET_AVOID()),
            "DIET_CUES" => Some(g_DIET_CUES()),
            "EATS" => Some(g_EATS()),
            "EATS_ITSELF" => Some(g_EATS_ITSELF()),
            "EAT_ASK" => Some(g_EAT_ASK()),
            "EAT_AVOID" => Some(g_EAT_AVOID()),
            "EAT_CUES" => Some(g_EAT_CUES()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FARM_DIETS" => Some(g_FARM_DIETS()),
            "RECORDS" => Some(g_RECORDS()),
            "WORD_ROWS" => Some(g_WORD_ROWS()),
            "_WORD_ASK" => Some(g__WORD_ASK()),
            "_WORD_AVOID" => Some(g__WORD_AVOID()),
            "_WORD_CUES" => Some(g__WORD_CUES()),
            "_WORD_SENTENCE" => Some(g__WORD_SENTENCE()),
            "_article" => Some(py_func("_article", |args: &[Py]| _article(args))),
            "_curiosity" => Some(py_func("_curiosity", |args: &[Py]| _curiosity(args))),
            "_eats" => Some(py_func("_eats", |args: &[Py]| _eats(args))),
            "_fact" => Some(py_func("_fact", |args: &[Py]| _fact(args))),
            "_line" => Some(py_func("_line", |args: &[Py]| _line(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_names_with" => Some(py_func("_names_with", |args: &[Py]| _names_with(args))),
            "_record" => Some(py_func("_record", |args: &[Py]| _record(args))),
            "_the_word" => Some(py_func("_the_word", |args: &[Py]| _the_word(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANIMALS" => s_ANIMALS(v),
            "ANSWERS" => s_ANSWERS(v),
            "CURIOSITIES" => s_CURIOSITIES(v),
            "DIETS" => s_DIETS(v),
            "DIET_ASK" => s_DIET_ASK(v),
            "DIET_AVOID" => s_DIET_AVOID(v),
            "DIET_CUES" => s_DIET_CUES(v),
            "EATS" => s_EATS(v),
            "EATS_ITSELF" => s_EATS_ITSELF(v),
            "EAT_ASK" => s_EAT_ASK(v),
            "EAT_AVOID" => s_EAT_AVOID(v),
            "EAT_CUES" => s_EAT_CUES(v),
            "ENTRIES" => s_ENTRIES(v),
            "FARM_DIETS" => s_FARM_DIETS(v),
            "RECORDS" => s_RECORDS(v),
            "WORD_ROWS" => s_WORD_ROWS(v),
            "_WORD_ASK" => s__WORD_ASK(v),
            "_WORD_AVOID" => s__WORD_AVOID(v),
            "_WORD_CUES" => s__WORD_CUES(v),
            "_WORD_SENTENCE" => s__WORD_SENTENCE(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.animals", module_obj);
    pyrt::xrefs::reg_fn("knowledge.animals", "_article", py_func("_article", |args: &[Py]| _article(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_line", py_func("_line", |args: &[Py]| _line(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_names_with", py_func("_names_with", |args: &[Py]| _names_with(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_the_word", py_func("_the_word", |args: &[Py]| _the_word(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_fact", py_func("_fact", |args: &[Py]| _fact(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_eats", py_func("_eats", |args: &[Py]| _eats(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_record", py_func("_record", |args: &[Py]| _record(args)));
    pyrt::xrefs::reg_fn("knowledge.animals", "_curiosity", py_func("_curiosity", |args: &[Py]| _curiosity(args)));
    pyrt::xrefs::reg_access("knowledge.animals", "ANIMALS", g_ANIMALS, s_ANIMALS);
    pyrt::xrefs::reg_access("knowledge.animals", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.animals", "CURIOSITIES", g_CURIOSITIES, s_CURIOSITIES);
    pyrt::xrefs::reg_access("knowledge.animals", "DIETS", g_DIETS, s_DIETS);
    pyrt::xrefs::reg_access("knowledge.animals", "DIET_ASK", g_DIET_ASK, s_DIET_ASK);
    pyrt::xrefs::reg_access("knowledge.animals", "DIET_AVOID", g_DIET_AVOID, s_DIET_AVOID);
    pyrt::xrefs::reg_access("knowledge.animals", "DIET_CUES", g_DIET_CUES, s_DIET_CUES);
    pyrt::xrefs::reg_access("knowledge.animals", "EATS", g_EATS, s_EATS);
    pyrt::xrefs::reg_access("knowledge.animals", "EATS_ITSELF", g_EATS_ITSELF, s_EATS_ITSELF);
    pyrt::xrefs::reg_access("knowledge.animals", "EAT_ASK", g_EAT_ASK, s_EAT_ASK);
    pyrt::xrefs::reg_access("knowledge.animals", "EAT_AVOID", g_EAT_AVOID, s_EAT_AVOID);
    pyrt::xrefs::reg_access("knowledge.animals", "EAT_CUES", g_EAT_CUES, s_EAT_CUES);
    pyrt::xrefs::reg_access("knowledge.animals", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.animals", "FARM_DIETS", g_FARM_DIETS, s_FARM_DIETS);
    pyrt::xrefs::reg_access("knowledge.animals", "RECORDS", g_RECORDS, s_RECORDS);
    pyrt::xrefs::reg_access("knowledge.animals", "WORD_ROWS", g_WORD_ROWS, s_WORD_ROWS);
    pyrt::xrefs::reg_access("knowledge.animals", "_WORD_ASK", g__WORD_ASK, s__WORD_ASK);
    pyrt::xrefs::reg_access("knowledge.animals", "_WORD_AVOID", g__WORD_AVOID, s__WORD_AVOID);
    pyrt::xrefs::reg_access("knowledge.animals", "_WORD_CUES", g__WORD_CUES, s__WORD_CUES);
    pyrt::xrefs::reg_access("knowledge.animals", "_WORD_SENTENCE", g__WORD_SENTENCE, s__WORD_SENTENCE);
    pyrt::xrefs::reg_access("knowledge.animals", "entry", g_entry, s_entry);
}
