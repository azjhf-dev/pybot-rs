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
thread_local! { static G_COUNTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EU_MEMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIGHEST_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEAKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_THE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BRITAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COMPARES_NATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINENT_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINENT_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINENT_NOTES_BY_COUNTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTRY_ALIASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTRY_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTRY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CROSSING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DESTINATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DIRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EVERYDAY_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIST_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NATION_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NATION_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_COUNTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_IN_EU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ORDINAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ORDINAL_POS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POS_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECORD_BIGGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECORD_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECORD_POPULOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECORD_SMALLEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REGIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REGION_COUNTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REGION_EXTREME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REGION_EXTREME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RIVER_LENGTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RIVER_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RIVER_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RIVER_NAME_NEEDS_RIVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_FOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_COUNTRIES() -> Py { ensure_init(); G_COUNTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTRIES(v: Py) { ensure_init(); G_COUNTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EU_MEMBERS() -> Py { ensure_init(); G_EU_MEMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EU_MEMBERS(v: Py) { ensure_init(); G_EU_MEMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_HIGHEST_IN() -> Py { ensure_init(); G_HIGHEST_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIGHEST_IN(v: Py) { ensure_init(); G_HIGHEST_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_PEAKS() -> Py { ensure_init(); G_PEAKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEAKS(v: Py) { ensure_init(); G_PEAKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_THE() -> Py { ensure_init(); G_TAKES_THE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_THE(v: Py) { ensure_init(); G_TAKES_THE.with(|c| *c.borrow_mut() = v); }
pub fn g__ARTICLE() -> Py { ensure_init(); G__ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARTICLE(v: Py) { ensure_init(); G__ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g__BRITAIN() -> Py { ensure_init(); G__BRITAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BRITAIN(v: Py) { ensure_init(); G__BRITAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__COMPARES_NATIONS() -> Py { ensure_init(); G__COMPARES_NATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COMPARES_NATIONS(v: Py) { ensure_init(); G__COMPARES_NATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINENTS() -> Py { ensure_init(); G__CONTINENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINENTS(v: Py) { ensure_init(); G__CONTINENTS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINENT_COUNTS() -> Py { ensure_init(); G__CONTINENT_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINENT_COUNTS(v: Py) { ensure_init(); G__CONTINENT_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINENT_NOTES() -> Py { ensure_init(); G__CONTINENT_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINENT_NOTES(v: Py) { ensure_init(); G__CONTINENT_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINENT_NOTES_BY_COUNTRY() -> Py { ensure_init(); G__CONTINENT_NOTES_BY_COUNTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINENT_NOTES_BY_COUNTRY(v: Py) { ensure_init(); G__CONTINENT_NOTES_BY_COUNTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINENT_WORDS() -> Py { ensure_init(); G__CONTINENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINENT_WORDS(v: Py) { ensure_init(); G__CONTINENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTRY_ALIASES() -> Py { ensure_init(); G__COUNTRY_ALIASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTRY_ALIASES(v: Py) { ensure_init(); G__COUNTRY_ALIASES.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTRY_PHRASES() -> Py { ensure_init(); G__COUNTRY_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTRY_PHRASES(v: Py) { ensure_init(); G__COUNTRY_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTRY_WORDS() -> Py { ensure_init(); G__COUNTRY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTRY_WORDS(v: Py) { ensure_init(); G__COUNTRY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CROSSING_WORD() -> Py { ensure_init(); G__CROSSING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CROSSING_WORD(v: Py) { ensure_init(); G__CROSSING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__DESTINATION() -> Py { ensure_init(); G__DESTINATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DESTINATION(v: Py) { ensure_init(); G__DESTINATION.with(|c| *c.borrow_mut() = v); }
pub fn g__DIRECTION() -> Py { ensure_init(); G__DIRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DIRECTION(v: Py) { ensure_init(); G__DIRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g__EVERYDAY_PLACE() -> Py { ensure_init(); G__EVERYDAY_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EVERYDAY_PLACE(v: Py) { ensure_init(); G__EVERYDAY_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAST() -> Py { ensure_init(); G__LEAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAST(v: Py) { ensure_init(); G__LEAST.with(|c| *c.borrow_mut() = v); }
pub fn g__LIST_COUNT() -> Py { ensure_init(); G__LIST_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIST_COUNT(v: Py) { ensure_init(); G__LIST_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__NATION_NAMED() -> Py { ensure_init(); G__NATION_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NATION_NAMED(v: Py) { ensure_init(); G__NATION_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__NATION_SAID() -> Py { ensure_init(); G__NATION_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NATION_SAID(v: Py) { ensure_init(); G__NATION_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_COUNTRY() -> Py { ensure_init(); G__NOT_A_COUNTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_COUNTRY(v: Py) { ensure_init(); G__NOT_A_COUNTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PLACE() -> Py { ensure_init(); G__NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PLACE(v: Py) { ensure_init(); G__NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_IN_EU() -> Py { ensure_init(); G__NOT_IN_EU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_IN_EU(v: Py) { ensure_init(); G__NOT_IN_EU.with(|c| *c.borrow_mut() = v); }
pub fn g__ORDINAL() -> Py { ensure_init(); G__ORDINAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ORDINAL(v: Py) { ensure_init(); G__ORDINAL.with(|c| *c.borrow_mut() = v); }
pub fn g__ORDINAL_POS() -> Py { ensure_init(); G__ORDINAL_POS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ORDINAL_POS(v: Py) { ensure_init(); G__ORDINAL_POS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACES() -> Py { ensure_init(); G__PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACES(v: Py) { ensure_init(); G__PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g__POS_WORD() -> Py { ensure_init(); G__POS_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POS_WORD(v: Py) { ensure_init(); G__POS_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__RECORD_BIGGEST() -> Py { ensure_init(); G__RECORD_BIGGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECORD_BIGGEST(v: Py) { ensure_init(); G__RECORD_BIGGEST.with(|c| *c.borrow_mut() = v); }
pub fn g__RECORD_LIST() -> Py { ensure_init(); G__RECORD_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECORD_LIST(v: Py) { ensure_init(); G__RECORD_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g__RECORD_POPULOUS() -> Py { ensure_init(); G__RECORD_POPULOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECORD_POPULOUS(v: Py) { ensure_init(); G__RECORD_POPULOUS.with(|c| *c.borrow_mut() = v); }
pub fn g__RECORD_SMALLEST() -> Py { ensure_init(); G__RECORD_SMALLEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECORD_SMALLEST(v: Py) { ensure_init(); G__RECORD_SMALLEST.with(|c| *c.borrow_mut() = v); }
pub fn g__REGIONS() -> Py { ensure_init(); G__REGIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REGIONS(v: Py) { ensure_init(); G__REGIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__REGION_COUNTRY() -> Py { ensure_init(); G__REGION_COUNTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REGION_COUNTRY(v: Py) { ensure_init(); G__REGION_COUNTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__REGION_EXTREME() -> Py { ensure_init(); G__REGION_EXTREME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REGION_EXTREME(v: Py) { ensure_init(); G__REGION_EXTREME.with(|c| *c.borrow_mut() = v); }
pub fn g__REGION_EXTREME_WORDS() -> Py { ensure_init(); G__REGION_EXTREME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REGION_EXTREME_WORDS(v: Py) { ensure_init(); G__REGION_EXTREME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__RIVER_LENGTHS() -> Py { ensure_init(); G__RIVER_LENGTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RIVER_LENGTHS(v: Py) { ensure_init(); G__RIVER_LENGTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__RIVER_LENGTH_ASK() -> Py { ensure_init(); G__RIVER_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RIVER_LENGTH_ASK(v: Py) { ensure_init(); G__RIVER_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__RIVER_NAMES() -> Py { ensure_init(); G__RIVER_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RIVER_NAMES(v: Py) { ensure_init(); G__RIVER_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__RIVER_NAME_NEEDS_RIVER() -> Py { ensure_init(); G__RIVER_NAME_NEEDS_RIVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RIVER_NAME_NEEDS_RIVER(v: Py) { ensure_init(); G__RIVER_NAME_NEEDS_RIVER.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_FOUR() -> Py { ensure_init(); G__THE_FOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_FOUR(v: Py) { ensure_init(); G__THE_FOUR.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Geography: the everyday facts people check, in quizzes, homework and arguments.\n\nThe answers lane owns this domain. Capitals and city guides live in bot/knowledge/travel.py;\nthis file is the other side of geography: how many, how big, how long, with the one number\nor caveat that makes an answer worth remembering.\n");
    s__REGIONS(py_tuple(vec![py_str("africa"), py_str("europe"), py_str("asia"), py_str("north america"), py_str("south america"), py_str("central america"), py_str("caribbean"), py_str("oceania"), py_str("australia"), py_str("uk"), py_str("britain"), py_str("england"), py_str("scotland"), py_str("wales"), py_str("ireland")]));
    s_ANSWERS(py_tuple(vec![py_str("geography-and-travel")]));
    s_PEAKS(py_dict(vec![(py_str("ben nevis"), py_tuple(vec![py_str("Ben Nevis"), Py::Int(1345), py_str("the highest mountain in the UK, in Scotland above Fort William. The walk up from Glen Nevis is about 8 hours there and back.")])), (py_str("snowdon"), py_tuple(vec![py_str("Snowdon (Yr Wyddfa)"), Py::Int(1085), py_str("the highest mountain in Wales, and the busiest in the UK. The Snowdon Ranger path is the gentlest way up without the train.")])), (py_str("yr wyddfa"), py_tuple(vec![py_str("Snowdon (Yr Wyddfa)"), Py::Int(1085), py_str("the highest mountain in Wales, and the busiest in the UK.")])), (py_str("scafell pike"), py_tuple(vec![py_str("Scafell Pike"), Py::Int(978), py_str("the highest mountain in England, in the Lake District, and the hardest of the three national peaks to walk because of the loose stone.")])), (py_str("slieve donard"), py_tuple(vec![py_str("Slieve Donard"), Py::Int(850), py_str("the highest mountain in Northern Ireland, right above Newcastle in County Down.")])), (py_str("carrauntoohil"), py_tuple(vec![py_str("Carrauntoohil"), Py::Int(1038), py_str("the highest mountain in Ireland, in the MacGillycuddy's Reeks, and the usual way up is through the Devil's Ladder.")])), (py_str("pen y fan"), py_tuple(vec![py_str("Pen y Fan"), Py::Int(886), py_str("the highest point in the Brecon Beacons and the most popular walk in south Wales, about 4 miles there and back.")])), (py_str("helvellyn"), py_tuple(vec![py_str("Helvellyn"), Py::Int(950), py_str("the third highest in England, and Striding Edge is the ridge that makes it famous - a scramble, not a walk.")])), (py_str("kilimanjaro"), py_tuple(vec![py_str("Kilimanjaro"), Py::Int(5895), py_str("the highest mountain in Africa and the tallest free-standing mountain in the world, rising straight out of the plain with no range around it.")])), (py_str("mont blanc"), py_tuple(vec![py_str("Mont Blanc"), Py::Int(4808), py_str("the highest peak in the Alps and in western Europe, on the French-Italian border.")])), (py_str("matterhorn"), py_tuple(vec![py_str("The Matterhorn"), Py::Int(4478), py_str("the pyramid on the Swiss-Italian border, and the last of the great Alpine peaks to be climbed, in 1865.")])), (py_str("k2"), py_tuple(vec![py_str("K2"), Py::Int(8611), py_str("the second highest mountain in the world, on the Pakistan-China border, and a much harder and deadlier climb than Everest.")])), (py_str("denali"), py_tuple(vec![py_str("Denali"), Py::Int(6190), py_str("the highest mountain in North America, in Alaska, and the biggest rise of any mountain measured base to summit.")])), (py_str("aconcagua"), py_tuple(vec![py_str("Aconcagua"), Py::Int(6961), py_str("the highest mountain in South America and the highest anywhere outside Asia.")])), (py_str("elbrus"), py_tuple(vec![py_str("Mount Elbrus"), Py::Int(5642), py_str("the highest mountain in Europe if the Caucasus counts as Europe; otherwise the European record is Mont Blanc.")])), (py_str("fuji"), py_tuple(vec![py_str("Mount Fuji"), Py::Int(3776), py_str("the highest mountain in Japan, a volcano that last erupted in 1707, and a day's climb in summer.")])), (py_str("table mountain"), py_tuple(vec![py_str("Table Mountain"), Py::Int(1085), py_str("the flat-topped mountain over Cape Town, and the cable car or the Platteklip Gorge path gets you up.")])), (py_str("kosciuszko"), py_tuple(vec![py_str("Mount Kosciuszko"), Py::Int(2228), py_str("the highest mountain in mainland Australia, a long walk rather than a climb.")])), (py_str("aoraki"), py_tuple(vec![py_str("Aoraki / Mount Cook"), Py::Int(3724), py_str("the highest mountain in New Zealand, in the Southern Alps, and a serious climb rather than a walk.")]))]));
    s_HIGHEST_IN(py_dict(vec![(py_str("wales"), py_str("Snowdon (Yr Wyddfa) is the highest, at 1,085 metres (3,560 ft)")), (py_str("england"), py_str("Scafell Pike is the highest, at 978 metres (3,209 ft)")), (py_str("scotland"), py_str("Ben Nevis is the highest, at 1,345 metres (4,413 ft)")), (py_str("northern ireland"), py_str("Slieve Donard is the highest, at 850 metres (2,789 ft)")), (py_str("ireland"), py_str("Carrauntoohil is the highest, at 1,038 metres (3,406 ft)")), (py_str("uk"), py_str("Ben Nevis is the highest, at 1,345 metres (4,413 ft)")), (py_str("britain"), py_str("Ben Nevis is the highest, at 1,345 metres (4,413 ft)")), (py_str("great britain"), py_str("Ben Nevis is the highest, at 1,345 metres (4,413 ft)")), (py_str("europe"), py_str("Mount Elbrus is the highest, at 5,642 metres (18,510 ft), if the Caucasus counts as Europe. If it does not, the record is Mont Blanc at 4,808 metres (15,774 ft)")), (py_str("africa"), py_str("Kilimanjaro is the highest, at 5,895 metres (19,341 ft)")), (py_str("asia"), py_str("Everest is the highest, at 8,849 metres (29,032 ft)")), (py_str("north america"), py_str("Denali is the highest, at 6,190 metres (20,310 ft)")), (py_str("south america"), py_str("Aconcagua is the highest, at 6,961 metres (22,838 ft)")), (py_str("australia"), py_str("Mount Kosciuszko is the highest on the mainland, at 2,228 metres (7,310 ft). The highest in Australian territory is Mawson Peak on Heard Island")), (py_str("oceania"), py_str("Puncak Jaya in New Guinea is the highest, at 4,884 metres (16,024 ft)")), (py_str("antarctica"), py_str("Mount Vinson is the highest, at 4,892 metres (16,050 ft)")), (py_str("alps"), py_str("Mont Blanc is the highest, at 4,808 metres (15,774 ft)")), (py_str("usa"), py_str("Denali is the highest, at 6,190 metres (20,310 ft)")), (py_str("united states"), py_str("Denali is the highest, at 6,190 metres (20,310 ft)")), (py_str("canada"), py_str("Mount Logan is the highest, at 5,959 metres (19,551 ft)")), (py_str("spain"), py_str("Mount Teide in Tenerife is the highest, at 3,715 metres (12,188 ft)")), (py_str("france"), py_str("Mont Blanc is the highest, at 4,808 metres (15,774 ft)")), (py_str("italy"), py_str("Mont Blanc (Monte Bianco) is the highest, at 4,808 metres (15,774 ft), on the French border")), (py_str("japan"), py_str("Mount Fuji is the highest, at 3,776 metres (12,388 ft)")), (py_str("india"), py_str("Kangchenjunga is the highest, at 8,586 metres (28,169 ft)")), (py_str("new zealand"), py_str("Aoraki / Mount Cook is the highest, at 3,724 metres (12,218 ft)"))]));
    s__CONTINENT_COUNTS(py_tuple(vec![py_tuple(vec![py_str("north america"), py_str("North America has 23 countries, from Canada down to Panama; the Caribbean islands are counted separately as part of the region.")]), py_tuple(vec![py_str("south america"), py_str("South America has 12 countries, from Colombia and Venezuela round to Argentina and Chile.")]), py_tuple(vec![py_str("africa"), py_str("Africa has 54 countries, or 55 if you count Western Sahara, which is disputed. Nigeria is the most populous and Algeria the largest by area.")]), py_tuple(vec![py_str("europe"), py_str("Europe has 44 countries, or about 50 if you count the transcontinental ones such as Russia and Turkey. The exact number depends on where you draw the eastern edge.")]), py_tuple(vec![py_str("asia"), py_str("Asia has 48 countries, or 49 if you count Taiwan. Russia and Turkey are usually counted in Europe as well, which is why some lists differ.")]), py_tuple(vec![py_str("oceania"), py_str("Oceania has 14 countries, spread across Australia, New Zealand, Papua New Guinea and the Pacific island states.")]), py_tuple(vec![py_str("antarctica"), py_str("Antarctica has no countries. It is governed by the Antarctic Treaty, and the only people there are scientists at research stations.")])]));
    s__PLACES(py_tuple(vec![py_str("north america"), py_str("south america"), py_str("central america"), py_str("caribbean"), py_str("africa"), py_str("europe"), py_str("asia"), py_str("oceania"), py_str("antarctica")]));
    s__REGION_COUNTRY(py_dict(vec![(py_str("africa"), py_dict(vec![(py_str("area"), py_str("Algeria is the largest country in Africa by area, at about 2.4 million square km - most of it Sahara. The Democratic Republic of the Congo is second.")), (py_str("small"), py_str("The Seychelles is the smallest African country by area, an island nation of about 115 islands. If you mean the smallest on the mainland, it is The Gambia.")), (py_str("people"), py_tuple(vec![py_str("Nigeria"), py_str("Ethiopia"), py_str("Egypt"), py_str("DR Congo"), py_str("South Africa")])), (py_str("populous"), py_str("Nigeria has the most people in Africa, about 230 million. Algeria is the biggest by area and the Seychelles is the smallest country.")), (py_str("least"), py_str("The Seychelles has the fewest people in Africa, about 100,000, and it is the smallest country there by area too.")), (py_str("top"), py_str("The three biggest countries in Africa by area are Algeria (2.4 million square km), DR Congo (2.3 million) and Sudan (1.9 million). By people it is Nigeria first, then Ethiopia and Egypt.")), (py_str("bottom"), py_str("The three smallest countries in Africa by area are the Seychelles (455 square km), São Tomé and Príncipe (about 1,000) and Comoros (about 1,900)."))])), (py_str("europe"), py_dict(vec![(py_str("area"), py_str("Ukraine is the largest country wholly inside Europe, at about 600,000 square km; European Russia is bigger but is usually counted separately.")), (py_str("small"), py_str("Vatican City is the smallest country in Europe, and in the world, at about 0.49 square km. Monaco is second, at about 2 square km.")), (py_str("people"), py_tuple(vec![py_str("Russia"), py_str("Germany"), py_str("United Kingdom"), py_str("France"), py_str("Italy")])), (py_str("populous"), py_str("Russia has the most people in Europe, about 110 million in its European part, and Germany the most of any country entirely in Europe, about 84 million. Vatican City is the smallest.")), (py_str("least"), py_str("Vatican City has the fewest people in Europe, a few hundred, and it is the smallest by area too.")), (py_str("top"), py_str("The three biggest countries in Europe by area are Russia's European part, Ukraine (604,000 square km) and France (551,000, without its overseas parts). By people it is Russia first, then Germany and the United Kingdom.")), (py_str("bottom"), py_str("The three smallest countries in Europe by area are Vatican City (half a square km), Monaco (about 2) and San Marino (61)."))])), (py_str("asia"), py_dict(vec![(py_str("area"), py_str("China is the largest country in Asia, at about 9.6 million square km. India is second; Russia is bigger but is counted as transcontinental.")), (py_str("small"), py_str("The Maldives is the smallest country in Asia, at about 300 square km spread over some 1,200 islands.")), (py_str("people"), py_tuple(vec![py_str("India"), py_str("China"), py_str("Indonesia"), py_str("Pakistan"), py_str("Bangladesh")])), (py_str("populous"), py_str("India has the most people in Asia, about 1.45 billion. China is the biggest country entirely in Asia and the Maldives is the smallest.")), (py_str("least"), py_str("Brunei has the fewest people in Asia, about 455,000, though the Maldives is smaller by area.")), (py_str("top"), py_str("The three biggest countries in Asia by area are China (9.6 million square km), India (3.3 million) and Kazakhstan (2.7 million). By people it is India first, then China and Indonesia.")), (py_str("bottom"), py_str("The three smallest countries in Asia by area are the Maldives (about 300 square km), Singapore (about 730) and Bahrain (about 780)."))])), (py_str("north america"), py_dict(vec![(py_str("area"), py_str("Canada is the largest country in North America, at about 10 million square km; the United States is a close second.")), (py_str("small"), py_str("Saint Kitts and Nevis is the smallest country in North America, at about 260 square km. Grenada and Saint Vincent are close behind.")), (py_str("people"), py_tuple(vec![py_str("United States"), py_str("Mexico"), py_str("Canada"), py_str("Guatemala"), py_str("Haiti")])), (py_str("populous"), py_str("The United States has the most people in North America, about 340 million. Canada is the biggest by area and Saint Kitts and Nevis is the smallest.")), (py_str("least"), py_str("Saint Kitts and Nevis has the fewest people in North America, about 47,000, and it is the smallest by area too.")), (py_str("top"), py_str("The three biggest countries in North America by area are Canada (10 million square km), the United States (9.8 million) and Mexico (1.96 million). By people it is the United States first, then Mexico and Canada.")), (py_str("bottom"), py_str("The three smallest countries in North America by area are Saint Kitts and Nevis (about 260 square km), Grenada (about 340) and Saint Vincent and the Grenadines (about 390)."))])), (py_str("central america"), py_dict(vec![(py_str("area"), py_str("Nicaragua is the largest country in Central America, at about 130,000 square km; Honduras and Guatemala come next.")), (py_str("small"), py_str("El Salvador is the smallest country in Central America, at about 21,000 square km, and the most densely populated on the mainland."))])), (py_str("south america"), py_dict(vec![(py_str("area"), py_str("Brazil is the largest country in South America, at about 8.5 million square km - nearly half the continent. Argentina is second.")), (py_str("small"), py_str("Suriname is the smallest country in South America, at about 164,000 square km. Uruguay and Guyana are next.")), (py_str("people"), py_tuple(vec![py_str("Brazil"), py_str("Colombia"), py_str("Argentina"), py_str("Peru"), py_str("Venezuela")])), (py_str("populous"), py_str("Brazil has the most people in South America, about 215 million. It is the biggest by area too, and Suriname is the smallest.")), (py_str("least"), py_str("Suriname has the fewest people in South America, about 620,000, and it is the smallest by area too.")), (py_str("top"), py_str("The three biggest countries in South America by area are Brazil (8.5 million square km), Argentina (2.8 million) and Peru (1.3 million). By people it is Brazil first, then Colombia and Argentina.")), (py_str("bottom"), py_str("The three smallest countries in South America by area are Suriname (164,000 square km), Uruguay (176,000) and Guyana (215,000)."))])), (py_str("oceania"), py_dict(vec![(py_str("area"), py_str("Australia is by far the largest country in Oceania, at about 7.7 million square km; Papua New Guinea is a distant second.")), (py_str("small"), py_str("Nauru is the smallest country in Oceania, at about 21 square km - the smallest island country in the world. Tuvalu is next.")), (py_str("people"), py_tuple(vec![py_str("Australia"), py_str("Papua New Guinea"), py_str("New Zealand"), py_str("Fiji"), py_str("Solomon Islands")])), (py_str("populous"), py_str("Australia has the most people in Oceania, about 26 million. It is the biggest by area too, and Nauru is the smallest.")), (py_str("least"), py_str("Nauru has the fewest people in Oceania, about 12,000, and it is the smallest by area too.")), (py_str("top"), py_str("The three biggest countries in Oceania by area are Australia (7.7 million square km), Papua New Guinea (463,000) and New Zealand (268,000). Australia has by far the most people too, then Papua New Guinea and New Zealand.")), (py_str("bottom"), py_str("The three smallest countries in Oceania by area are Nauru (21 square km), Tuvalu (26) and the Marshall Islands (181)."))])), (py_str("caribbean"), py_dict(vec![(py_str("area"), py_str("Cuba is the largest country in the Caribbean, at about 110,000 square km; the Dominican Republic is second.")), (py_str("small"), py_str("Saint Kitts and Nevis is the smallest country in the Caribbean, at about 260 square km, and the smallest in the Americas."))])), (py_str("antarctica"), py_dict(vec![(py_str("area"), py_str("Antarctica has no countries, so there is no biggest one: the whole continent is governed by the Antarctic Treaty and the only people there are scientists at research stations.")), (py_str("small"), py_str("Antarctica has no countries, so there is no smallest one: it is governed by the Antarctic Treaty and the only people there are scientists at research stations.")), (py_str("populous"), py_str("Antarctica has no countries and no permanent residents, only scientists at research stations under the Antarctic Treaty.")), (py_str("least"), py_str("Antarctica has no countries and no permanent residents, only scientists at research stations under the Antarctic Treaty.")), (py_str("top"), py_str("Antarctica has no countries, so there is no ranking: it is governed by the Antarctic Treaty and the only people there are scientists at research stations.")), (py_str("bottom"), py_str("Antarctica has no countries, so there is no ranking: it is governed by the Antarctic Treaty and the only people there are scientists at research stations."))]))]));
    s__BRITAIN(py_tuple(vec![py_str("uk"), py_str("u.k."), py_str("united kingdom"), py_str("britain"), py_str("great britain"), py_str("england"), py_str("scotland"), py_str("wales"), py_str("northern ireland"), py_str("country"), py_str("the country"), py_str("this country"), py_str("our country"), py_str("home"), py_str("back home")]));
    s__CROSSING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:abroad|overseas|customs|import|imports|importing|imported|declare|declaring|duty|back home|(?:bring|brings|bringing|brought|take|takes|taking|took)\\s+(?:(?!the\\s+back\\b)\\w+\\s+){0,3}?back(?!\\s*up\\b)|from abroad|(?:bring|brings|bringing|brought|take|takes|taking|took)\\s+(?:\\w+\\s+){0,3}?home|suitcase|suitcases|luggage|hold bag|hold luggage|checked bag|checked luggage|carry-on|carry on bag|rucksack|backpack)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__DIRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:from|into|to)\\s+(((?!(?:from|into|to|in|at|for|with|and|but|if|when|because|so|on|about|before|after)\\b)[a-z][a-z'\\u2019.-]*\\s*){1,3})")]).unwrap_or_else(|e| exc::raise(e)));
    s__EVERYDAY_PLACE(py_tuple(vec![py_str("work"), py_str("the office"), py_str("office"), py_str("school"), py_str("nursery"), py_str("college"), py_str("university"), py_str("class"), py_str("the library"), py_str("library"), py_str("canteen"), py_str("staff room"), py_str("cafeteria"), py_str("the hospital"), py_str("hospital"), py_str("ward"), py_str("clinic"), py_str("surgery"), py_str("care home"), py_str("the church"), py_str("church"), py_str("mosque"), py_str("temple"), py_str("synagogue"), py_str("the restaurant"), py_str("restaurant"), py_str("cafe"), py_str("the pub"), py_str("pub"), py_str("bar"), py_str("cinema"), py_str("theatre"), py_str("theater"), py_str("gig"), py_str("concert"), py_str("festival"), py_str("the match"), py_str("match"), py_str("gym"), py_str("club"), py_str("meeting"), py_str("interview"), py_str("shift"), py_str("the station"), py_str("station"), py_str("the airport"), py_str("airport"), py_str("ferry"), py_str("the ferry"), py_str("train"), py_str("the train"), py_str("bus"), py_str("the bus"), py_str("coach"), py_str("plane"), py_str("flight"), py_str("the car"), py_str("campervan"), py_str("caravan"), py_str("tent"), py_str("campsite"), py_str("the hotel"), py_str("hotel"), py_str("hotel room"), py_str("hostel"), py_str("b&b"), py_str("airbnb"), py_str("house"), py_str("flat"), py_str("the park"), py_str("park"), py_str("the beach"), py_str("beach"), py_str("party"), py_str("birthday"), py_str("dinner"), py_str("dinner party"), py_str("lunch"), py_str("tea"), py_str("supper"), py_str("brunch"), py_str("breakfast"), py_str("barbecue"), py_str("bbq"), py_str("picnic"), py_str("buffet"), py_str("potluck"), py_str("bake sale"), py_str("wake"), py_str("funeral"), py_str("christening"), py_str("wedding"), py_str("housewarming"), py_str("baby shower"), py_str("engagement"), py_str("anniversary"), py_str("reunion"), py_str("do"), py_str("get-together"), py_str("gathering"), py_str("playdate"), py_str("sleepover"), py_str("fair"), py_str("fete"), py_str("market"), py_str("sale"), py_str("christmas"), py_str("easter"), py_str("thanksgiving"), py_str("hanukkah"), py_str("diwali"), py_str("eid"), py_str("mothers day"), py_str("fathers day"), py_str("a meet"), py_str("the class")]));
    s__NOT_A_PLACE(py_tuple(vec![py_str("eat"), py_str("make"), py_str("cook"), py_str("bake"), py_str("bring"), py_str("take"), py_str("do"), py_str("share"), py_str("serve"), py_str("drink"), py_str("feed"), py_str("contribute"), py_str("prepare"), py_str("buy"), py_str("get"), py_str("have"), py_str("go"), py_str("be"), py_str("see"), py_str("help"), py_str("keep"), py_str("put"), py_str("leave"), py_str("give"), py_str("add"), py_str("use"), py_str("find"), py_str("stay"), py_str("come"), py_str("know"), py_str("say"), py_str("ask"), py_str("try"), py_str("start"), py_str("fill"), py_str("cover"), py_str("heat"), py_str("freeze"), py_str("store"), py_str("carry"), py_str("hold"), py_str("wrap"), py_str("cut"), py_str("mix"), py_str("pick"), py_str("choose"), py_str("order"), py_str("book")]));
    s__ARTICLE(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|my|our|his|her|their|your|some|any|this|that|these|those)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s__DESTINATION(stdlib_call("re", "compile", &[py_str("\\b(?:into|to)\\s+(((?!(?:from|into|to|in|at|for|with|and|but|if|when|because|so|on|about|before|after)\\b)[a-z][a-z'\\u2019.-]*\\s*){1,3})")]).unwrap_or_else(|e| exc::raise(e)));
    s__ORDINAL(stdlib_call("re", "compile", &[py_str("\\b(?:second|third|fourth|fifth|2nd|3rd|4th|5th)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ORDINAL_POS(py_dict(vec![(py_str("second"), Py::Int(2)), (py_str("2nd"), Py::Int(2)), (py_str("third"), Py::Int(3)), (py_str("3rd"), Py::Int(3)), (py_str("fourth"), Py::Int(4)), (py_str("4th"), Py::Int(4)), (py_str("fifth"), Py::Int(5)), (py_str("5th"), Py::Int(5))]));
    s__POS_WORD(py_dict(vec![(Py::Int(2), py_str("second")), (Py::Int(3), py_str("third")), (Py::Int(4), py_str("fourth")), (Py::Int(5), py_str("fifth"))]));
    s__CONTINENT_WORDS(py_tuple(vec![py_tuple(vec![py_str("north american"), py_str("north america")]), py_tuple(vec![py_str("south american"), py_str("south america")]), py_tuple(vec![py_str("north america"), py_str("north america")]), py_tuple(vec![py_str("south america"), py_str("south america")]), py_tuple(vec![py_str("african"), py_str("africa")]), py_tuple(vec![py_str("africa"), py_str("africa")]), py_tuple(vec![py_str("european"), py_str("europe")]), py_tuple(vec![py_str("europe"), py_str("europe")]), py_tuple(vec![py_str("asian"), py_str("asia")]), py_tuple(vec![py_str("asia"), py_str("asia")]), py_tuple(vec![py_str("oceania"), py_str("oceania")]), py_tuple(vec![py_str("antarctica"), py_str("antarctica")])]));
    s__RECORD_BIGGEST(py_tuple(vec![py_str("biggest"), py_str("largest"), py_str("biggest by area"), py_str("largest by area")]));
    s__RECORD_SMALLEST(py_tuple(vec![py_str("smallest"), py_str("tiniest"), py_str("smallest by area")]));
    s__RECORD_POPULOUS(py_tuple(vec![py_str("most populous"), py_str("most people"), py_str("biggest population"), py_str("largest population"), py_str("most populated"), py_str("by population"), py_str("in population"), py_str("population wise"), py_str("by people"), py_str("in terms of population"), py_str("in terms of people")]));
    s__LEAST(py_tuple(vec![py_str("fewest people"), py_str("least populous"), py_str("smallest population"), py_str("fewest population"), py_str("smallest by population"), py_str("least people"), py_str("least in population"), py_str("least populated"), py_str("fewest in population")]));
    s__LIST_COUNT(py_str("\\b(?:two|three|four|five|six|seven|eight|nine|ten|\\d+)\\b"));
    s__RECORD_LIST(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:list|top|rank|ranking)\\b|"), &g__LIST_COUNT()), &py_str("\\s+(?:of\\s+the\\s+)?(?:biggest|largest|smallest|tiniest|populous)|\\b(?:biggest|largest|smallest|tiniest|populous)\\s+countries\\s+(?:in|of)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__REGION_EXTREME(py_dict(vec![(py_str("africa"), py_dict(vec![(py_str("river"), py_str("The Nile, at about 6,650 km - the longest river in Africa and usually counted the longest in the world. The Congo carries far more water.")), (py_str("mountain"), py_str("Kilimanjaro, at 5,895 m, in Tanzania - the highest mountain in Africa, and a walk-up rather than a climb, though the altitude is the hard part.")), (py_str("desert"), py_str("The Sahara, at about 9 million square km - the largest hot desert in the world, roughly the size of the United States."))])), (py_str("europe"), py_dict(vec![(py_str("river"), py_str("The Volga, at about 3,530 km, running through Russia to the Caspian Sea. The Danube is second, at about 2,850 km, and passes through ten countries.")), (py_str("mountain"), py_str("Mount Elbrus, at 5,642 m, in the Russian Caucasus - the highest in Europe and one of the seven summits."))])), (py_str("asia"), py_dict(vec![(py_str("river"), py_str("The Yangtze, at about 6,300 km - the longest river wholly in Asia. Russia's Ob and Yenisei are longer but are only partly in Asia.")), (py_str("mountain"), py_str("Mount Everest, at 8,849 m - the highest in Asia and in the world.")), (py_str("desert"), py_str("The Arabian Desert, at about 2.3 million square km. The Gobi is the biggest cold desert, and the Sahara is the biggest hot one - it is in Africa."))])), (py_str("north america"), py_dict(vec![(py_str("river"), py_str("The Missouri-Mississippi, at about 6,275 km from source to the Gulf of Mexico. The Mississippi alone is about 3,770 km.")), (py_str("mountain"), py_str("Denali, at 6,190 m, in Alaska. Mount Logan in Canada is second, at 5,959 m.")), (py_str("desert"), py_str("The Great Basin Desert, at about 490,000 square km, in the western United States. The Chihuahuan, shared with Mexico, is the biggest hot one."))])), (py_str("south america"), py_dict(vec![(py_str("river"), py_str("The Amazon, at about 6,400 km, and far and away the biggest by water - about a fifth of all the river water on Earth.")), (py_str("mountain"), py_str("Aconcagua, at 6,961 m, in Argentina - the highest mountain outside Asia.")), (py_str("desert"), py_str("The Patagonian Desert, at about 670,000 square km. The Atacama is the driest desert, not the biggest."))])), (py_str("oceania"), py_dict(vec![(py_str("river"), py_str("The Murray, at about 2,508 km, with the Darling - the longest river system in Oceania, running through Australia.")), (py_str("mountain"), py_str("Puncak Jaya, at 4,884 m, in Indonesian Papua. Australia's own highest is Mount Kosciuszko, at 2,228 m.")), (py_str("desert"), py_str("The Great Victoria Desert, at about 348,000 square km, in Australia."))])), (py_str("uk"), py_dict(vec![(py_str("river"), py_str("The Severn, at about 354 km (220 miles), is the longest river in the UK, rising in Wales and reaching the sea past Bristol. The Thames is second at about 346 km.")), (py_str("lake"), py_str("Lough Neagh in Northern Ireland is the largest lake in the UK by area, at about 383 square km. Loch Ness holds the most water, about 7.5 cubic km.")), (py_str("deep lake"), py_str("Loch Morar is the deepest lake in Britain, at 310 metres, deeper than the North Sea is in most places. Loch Ness is 230 metres but holds far more water.")), (py_str("desert"), py_str("There are no deserts in the UK: nowhere here is dry enough to count as one. The closest thing is a dune system such as the Sefton coast or Braunton Burrows."))])), (py_str("britain"), py_dict(vec![(py_str("river"), py_str("The Severn, at about 354 km, is the longest river in Britain, with the Thames second at about 346 km and the Trent third.")), (py_str("lake"), py_str("Loch Lomond, at about 71 square km, is the largest lake in Britain by area. Windermere is the largest in England at about 14.7 square km.")), (py_str("deep lake"), py_str("Loch Morar, at 310 metres, is the deepest lake in Britain. Loch Ness is second at 230 metres and holds more water than all the lakes of England and Wales together."))])), (py_str("england"), py_dict(vec![(py_str("river"), py_str("The Severn, at about 354 km (220 miles), is the longest river in England, though it rises in Wales. The Thames is second at about 346 km, and the longest river wholly inside England.")), (py_str("lake"), py_str("Windermere is the largest lake in England, about 14.7 square km and 18 km long. Wastwater is the deepest, at 74 metres, and was voted Britain's favourite view.")), (py_str("deep lake"), py_str("Wastwater in the Lake District is the deepest lake in England, at 74 metres, and the coldest: it is a long narrow trench under the Screes."))])), (py_str("scotland"), py_dict(vec![(py_str("river"), py_str("The Tay, at about 188 km (117 miles), is the longest river in Scotland and the biggest by water, carrying more than the Thames and the Severn together.")), (py_str("lake"), py_str("Loch Lomond is the largest lake in Scotland by area, about 71 square km. Loch Ness is bigger by volume and holds more fresh water than all of England and Wales put together.")), (py_str("deep lake"), py_str("Loch Morar in the west Highlands is the deepest lake in Scotland and in Britain, at 310 metres."))])), (py_str("wales"), py_dict(vec![(py_str("river"), py_str("The Severn rises in the Welsh hills and is the longest river to run through Wales, at about 354 km. The longest river wholly inside Wales is the Tywi, at about 120 km.")), (py_str("lake"), py_str("Llyn Tegid, also called Bala Lake, is the largest natural lake in Wales at about 4.8 square km, and it has its own species of whitefish, the gwyniad."))])), (py_str("northern ireland"), py_dict(vec![(py_str("river"), py_str("The Bann, at about 129 km, is the longest river in Northern Ireland: the Upper Bann feeds Lough Neagh and the Lower Bann carries it to the sea at Coleraine.")), (py_str("lake"), py_str("Lough Neagh is the largest lake in Northern Ireland and in the British Isles, at about 383 square km, and it supplies much of Belfast's drinking water."))])), (py_str("ireland"), py_dict(vec![(py_str("river"), py_str("The Shannon, at about 360 km, is the longest river in Ireland, running from the Cavan hills south to Limerick and the Atlantic.")), (py_str("lake"), py_str("Lough Neagh, in Northern Ireland, is the largest lake on the island of Ireland at about 383 square km. The largest in the Republic is Lough Corrib, about 176 square km, in County Galway."))])), (py_str("antarctica"), py_dict(vec![(py_str("river"), py_str("There are no real rivers: the Onyx is a seasonal meltwater stream about 30 km long, and it is the longest in Antarctica.")), (py_str("mountain"), py_str("Vinson Massif, at 4,892 m - the highest in Antarctica, and the coldest of the seven summits.")), (py_str("desert"), py_str("Antarctica is itself the largest desert on Earth, at about 14 million square km: a desert is about how little it rains, not about sand."))]))]));
    s__REGION_EXTREME_WORDS(call_builtin("tuple", &[g__REGION_EXTREME()]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTRIES(py_dict(vec![(py_str("albania"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("montenegro"), py_str("kosovo"), py_str("north macedonia"), py_str("greece")]), py_str("")])), (py_str("andorra"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("france"), py_str("spain")]), py_str("")])), (py_str("austria"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("germany"), py_str("czech republic"), py_str("slovakia"), py_str("hungary"), py_str("slovenia"), py_str("italy"), py_str("switzerland"), py_str("liechtenstein")]), py_str("")])), (py_str("belarus"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("poland"), py_str("lithuania"), py_str("latvia"), py_str("russia"), py_str("ukraine")]), py_str("")])), (py_str("belgium"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("france"), py_str("luxembourg"), py_str("germany"), py_str("netherlands")]), py_str("")])), (py_str("bosnia and herzegovina"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("croatia"), py_str("serbia"), py_str("montenegro")]), py_str("")])), (py_str("bulgaria"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("romania"), py_str("serbia"), py_str("north macedonia"), py_str("greece"), py_str("turkey")]), py_str("")])), (py_str("croatia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("slovenia"), py_str("hungary"), py_str("serbia"), py_str("bosnia and herzegovina"), py_str("montenegro")]), py_str("")])), (py_str("cyprus"), py_tuple(vec![py_str("Europe"), py_tuple(vec![]), py_str("It is an island in the eastern Mediterranean, closer to Turkey and Syria than to Greece, but it is in the EU.")])), (py_str("czech republic"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("germany"), py_str("poland"), py_str("slovakia"), py_str("austria")]), py_str("")])), (py_str("denmark"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("germany")]), py_str("")])), (py_str("estonia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("latvia"), py_str("russia")]), py_str("")])), (py_str("finland"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("sweden"), py_str("norway"), py_str("russia")]), py_str("")])), (py_str("france"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("belgium"), py_str("luxembourg"), py_str("germany"), py_str("switzerland"), py_str("italy"), py_str("monaco"), py_str("spain"), py_str("andorra")]), py_str("It also has a border in South America, through French Guiana, with Brazil and Suriname.")])), (py_str("germany"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("denmark"), py_str("poland"), py_str("czech republic"), py_str("austria"), py_str("switzerland"), py_str("france"), py_str("luxembourg"), py_str("belgium"), py_str("netherlands")]), py_str("")])), (py_str("greece"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("albania"), py_str("north macedonia"), py_str("bulgaria"), py_str("turkey")]), py_str("")])), (py_str("hungary"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("austria"), py_str("slovakia"), py_str("ukraine"), py_str("romania"), py_str("serbia"), py_str("croatia"), py_str("slovenia")]), py_str("")])), (py_str("iceland"), py_tuple(vec![py_str("Europe"), py_tuple(vec![]), py_str("It is an island in the North Atlantic, about 290 km from Greenland and further from anywhere else.")])), (py_str("ireland"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("united kingdom")]), py_str("The border is the one between the Republic of Ireland and Northern Ireland.")])), (py_str("italy"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("france"), py_str("switzerland"), py_str("austria"), py_str("slovenia"), py_str("san marino"), py_str("vatican city")]), py_str("")])), (py_str("kosovo"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("serbia"), py_str("montenegro"), py_str("albania"), py_str("north macedonia")]), py_str("")])), (py_str("latvia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("estonia"), py_str("russia"), py_str("belarus"), py_str("lithuania")]), py_str("")])), (py_str("liechtenstein"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("switzerland"), py_str("austria")]), py_str("")])), (py_str("lithuania"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("latvia"), py_str("belarus"), py_str("poland"), py_str("russia")]), py_str("The Russian border is with Kaliningrad, which is cut off from the rest of Russia.")])), (py_str("luxembourg"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("belgium"), py_str("germany"), py_str("france")]), py_str("")])), (py_str("malta"), py_tuple(vec![py_str("Europe"), py_tuple(vec![]), py_str("It is an island country south of Sicily.")])), (py_str("moldova"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("romania"), py_str("ukraine")]), py_str("")])), (py_str("monaco"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("france")]), py_str("")])), (py_str("montenegro"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("croatia"), py_str("bosnia and herzegovina"), py_str("serbia"), py_str("kosovo"), py_str("albania")]), py_str("")])), (py_str("netherlands"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("germany"), py_str("belgium")]), py_str("")])), (py_str("north macedonia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("serbia"), py_str("kosovo"), py_str("bulgaria"), py_str("greece"), py_str("albania")]), py_str("")])), (py_str("norway"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("sweden"), py_str("finland"), py_str("russia")]), py_str("")])), (py_str("poland"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("germany"), py_str("czech republic"), py_str("slovakia"), py_str("ukraine"), py_str("belarus"), py_str("lithuania"), py_str("russia")]), py_str("")])), (py_str("portugal"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("spain")]), py_str("")])), (py_str("romania"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("ukraine"), py_str("moldova"), py_str("bulgaria"), py_str("serbia"), py_str("hungary")]), py_str("")])), (py_str("russia"), py_tuple(vec![py_str("Europe and Asia"), py_tuple(vec![py_str("norway"), py_str("finland"), py_str("estonia"), py_str("latvia"), py_str("lithuania"), py_str("poland"), py_str("belarus"), py_str("ukraine"), py_str("georgia"), py_str("azerbaijan"), py_str("kazakhstan"), py_str("china"), py_str("mongolia"), py_str("north korea")]), py_str("It is the biggest country in the world, and most of its land is in Asia.")])), (py_str("san marino"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("italy")]), py_str("It is a tiny country inside Italy.")])), (py_str("serbia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("hungary"), py_str("romania"), py_str("bulgaria"), py_str("north macedonia"), py_str("kosovo"), py_str("montenegro"), py_str("bosnia and herzegovina"), py_str("croatia")]), py_str("")])), (py_str("slovakia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("czech republic"), py_str("poland"), py_str("ukraine"), py_str("hungary"), py_str("austria")]), py_str("")])), (py_str("slovenia"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("italy"), py_str("austria"), py_str("hungary"), py_str("croatia")]), py_str("")])), (py_str("spain"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("portugal"), py_str("france"), py_str("andorra")]), py_str("It also has two small borders in Africa, with Morocco at Ceuta and Melilla, and the border with Gibraltar is with the UK.")])), (py_str("sweden"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("norway"), py_str("finland")]), py_str("")])), (py_str("switzerland"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("france"), py_str("germany"), py_str("austria"), py_str("liechtenstein"), py_str("italy")]), py_str("")])), (py_str("ukraine"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("belarus"), py_str("russia"), py_str("poland"), py_str("slovakia"), py_str("hungary"), py_str("romania"), py_str("moldova")]), py_str("")])), (py_str("united kingdom"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("ireland")]), py_str("That is the border between Northern Ireland and the Republic of Ireland: England, Scotland and Wales are on islands, so they have no land border.")])), (py_str("vatican city"), py_tuple(vec![py_str("Europe"), py_tuple(vec![py_str("italy")]), py_str("It is the smallest country in the world, inside Rome.")])), (py_str("afghanistan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("iran"), py_str("pakistan"), py_str("china"), py_str("tajikistan"), py_str("uzbekistan"), py_str("turkmenistan")]), py_str("")])), (py_str("armenia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("georgia"), py_str("azerbaijan"), py_str("iran"), py_str("turkey")]), py_str("")])), (py_str("azerbaijan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("russia"), py_str("georgia"), py_str("armenia"), py_str("iran"), py_str("turkey")]), py_str("")])), (py_str("bahrain"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is an island country in the Persian Gulf, joined to Saudi Arabia by a causeway rather than a border.")])), (py_str("bangladesh"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("india"), py_str("myanmar")]), py_str("")])), (py_str("bhutan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("china"), py_str("india")]), py_str("")])), (py_str("brunei"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("malaysia")]), py_str("It is on the island of Borneo, surrounded by Malaysia.")])), (py_str("cambodia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("thailand"), py_str("laos"), py_str("vietnam")]), py_str("")])), (py_str("china"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("north korea"), py_str("russia"), py_str("mongolia"), py_str("kazakhstan"), py_str("kyrgyzstan"), py_str("tajikistan"), py_str("afghanistan"), py_str("pakistan"), py_str("india"), py_str("nepal"), py_str("bhutan"), py_str("myanmar"), py_str("laos"), py_str("vietnam")]), py_str("")])), (py_str("georgia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("russia"), py_str("azerbaijan"), py_str("armenia"), py_str("turkey")]), py_str("The US state of Georgia is in North America.")])), (py_str("india"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("pakistan"), py_str("afghanistan"), py_str("china"), py_str("nepal"), py_str("bhutan"), py_str("bangladesh"), py_str("myanmar")]), py_str("")])), (py_str("indonesia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("malaysia"), py_str("papua new guinea"), py_str("timor-leste")]), py_str("It is the biggest group of islands in the world, with more than 17,000 of them.")])), (py_str("iran"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("iraq"), py_str("turkey"), py_str("armenia"), py_str("azerbaijan"), py_str("turkmenistan"), py_str("afghanistan"), py_str("pakistan")]), py_str("")])), (py_str("iraq"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("turkey"), py_str("iran"), py_str("kuwait"), py_str("saudi arabia"), py_str("jordan"), py_str("syria")]), py_str("")])), (py_str("israel"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("lebanon"), py_str("syria"), py_str("jordan"), py_str("egypt")]), py_str("It also borders the West Bank and the Gaza Strip.")])), (py_str("japan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is an island country: four main islands and thousands of small ones, with no land border.")])), (py_str("jordan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("israel"), py_str("syria"), py_str("iraq"), py_str("saudi arabia")]), py_str("It also borders the West Bank.")])), (py_str("kazakhstan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("russia"), py_str("china"), py_str("kyrgyzstan"), py_str("uzbekistan"), py_str("turkmenistan")]), py_str("It is the biggest landlocked country in the world.")])), (py_str("kuwait"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("iraq"), py_str("saudi arabia")]), py_str("")])), (py_str("kyrgyzstan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("kazakhstan"), py_str("china"), py_str("tajikistan"), py_str("uzbekistan")]), py_str("")])), (py_str("laos"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("china"), py_str("myanmar"), py_str("thailand"), py_str("cambodia"), py_str("vietnam")]), py_str("It is the only country in South East Asia with no coast.")])), (py_str("lebanon"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("syria"), py_str("israel")]), py_str("")])), (py_str("malaysia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("thailand"), py_str("indonesia"), py_str("brunei")]), py_str("The border with Indonesia is on the island of Borneo.")])), (py_str("maldives"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is a chain of about 1,200 islands in the Indian Ocean.")])), (py_str("mongolia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("russia"), py_str("china")]), py_str("")])), (py_str("myanmar"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("bangladesh"), py_str("india"), py_str("china"), py_str("laos"), py_str("thailand")]), py_str("")])), (py_str("nepal"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("china"), py_str("india")]), py_str("")])), (py_str("north korea"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("china"), py_str("russia"), py_str("south korea")]), py_str("")])), (py_str("oman"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("united arab emirates"), py_str("saudi arabia"), py_str("yemen")]), py_str("")])), (py_str("pakistan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("iran"), py_str("afghanistan"), py_str("china"), py_str("india")]), py_str("")])), (py_str("philippines"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is a group of more than 7,000 islands, with no land border.")])), (py_str("qatar"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("saudi arabia")]), py_str("Saudi Arabia is its only land border.")])), (py_str("saudi arabia"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("jordan"), py_str("iraq"), py_str("kuwait"), py_str("qatar"), py_str("united arab emirates"), py_str("oman"), py_str("yemen")]), py_str("")])), (py_str("singapore"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is an island city-state joined to Malaysia by a causeway.")])), (py_str("south korea"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("north korea")]), py_str("North Korea is its only land border.")])), (py_str("sri lanka"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is an island south of India.")])), (py_str("syria"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("turkey"), py_str("iraq"), py_str("jordan"), py_str("israel"), py_str("lebanon")]), py_str("")])), (py_str("taiwan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![]), py_str("It is an island about 130 km from the coast of China.")])), (py_str("tajikistan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("kyrgyzstan"), py_str("china"), py_str("afghanistan"), py_str("uzbekistan")]), py_str("")])), (py_str("thailand"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("myanmar"), py_str("laos"), py_str("cambodia"), py_str("malaysia")]), py_str("")])), (py_str("timor-leste"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("indonesia")]), py_str("It is on the eastern half of the island of Timor.")])), (py_str("turkey"), py_tuple(vec![py_str("Europe and Asia"), py_tuple(vec![py_str("greece"), py_str("bulgaria"), py_str("georgia"), py_str("armenia"), py_str("azerbaijan"), py_str("iran"), py_str("iraq"), py_str("syria")]), py_str("Most of it is in Asia, with a small part in Europe across the Bosphorus.")])), (py_str("turkmenistan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("kazakhstan"), py_str("uzbekistan"), py_str("afghanistan"), py_str("iran")]), py_str("")])), (py_str("united arab emirates"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("saudi arabia"), py_str("oman")]), py_str("")])), (py_str("uzbekistan"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("kazakhstan"), py_str("kyrgyzstan"), py_str("tajikistan"), py_str("afghanistan"), py_str("turkmenistan")]), py_str("")])), (py_str("vietnam"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("china"), py_str("laos"), py_str("cambodia")]), py_str("")])), (py_str("yemen"), py_tuple(vec![py_str("Asia"), py_tuple(vec![py_str("saudi arabia"), py_str("oman")]), py_str("")])), (py_str("algeria"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("tunisia"), py_str("libya"), py_str("niger"), py_str("mali"), py_str("mauritania"), py_str("morocco"), py_str("western sahara")]), py_str("It is the biggest country in Africa by area.")])), (py_str("angola"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("namibia"), py_str("zambia"), py_str("democratic republic of the congo"), py_str("republic of the congo")]), py_str("")])), (py_str("benin"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("togo"), py_str("burkina faso"), py_str("niger"), py_str("nigeria")]), py_str("")])), (py_str("botswana"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("namibia"), py_str("zambia"), py_str("zimbabwe"), py_str("south africa")]), py_str("")])), (py_str("burkina faso"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("mali"), py_str("niger"), py_str("benin"), py_str("togo"), py_str("ghana"), py_str("ivory coast")]), py_str("")])), (py_str("burundi"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("rwanda"), py_str("tanzania"), py_str("democratic republic of the congo")]), py_str("")])), (py_str("cameroon"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("nigeria"), py_str("chad"), py_str("central african republic"), py_str("gabon"), py_str("equatorial guinea"), py_str("republic of the congo")]), py_str("")])), (py_str("cape verde"), py_tuple(vec![py_str("Africa"), py_tuple(vec![]), py_str("It is a group of islands in the Atlantic, about 570 km off the coast of West Africa.")])), (py_str("central african republic"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("chad"), py_str("sudan"), py_str("south sudan"), py_str("cameroon"), py_str("republic of the congo"), py_str("democratic republic of the congo")]), py_str("")])), (py_str("chad"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("libya"), py_str("sudan"), py_str("central african republic"), py_str("cameroon"), py_str("nigeria"), py_str("niger")]), py_str("")])), (py_str("djibouti"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("eritrea"), py_str("ethiopia"), py_str("somalia")]), py_str("")])), (py_str("egypt"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("libya"), py_str("sudan"), py_str("israel")]), py_str("The Sinai peninsula, across the Suez Canal, is in Asia.")])), (py_str("equatorial guinea"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("cameroon"), py_str("gabon")]), py_str("")])), (py_str("eritrea"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("sudan"), py_str("ethiopia"), py_str("djibouti")]), py_str("")])), (py_str("eswatini"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("south africa"), py_str("mozambique")]), py_str("It used to be called Swaziland.")])), (py_str("ethiopia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("eritrea"), py_str("djibouti"), py_str("somalia"), py_str("kenya"), py_str("south sudan"), py_str("sudan")]), py_str("")])), (py_str("gabon"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("cameroon"), py_str("equatorial guinea"), py_str("republic of the congo")]), py_str("")])), (py_str("gambia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("senegal")]), py_str("Senegal surrounds it on every side but the sea.")])), (py_str("ghana"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("ivory coast"), py_str("burkina faso"), py_str("togo")]), py_str("")])), (py_str("guinea"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("guinea-bissau"), py_str("senegal"), py_str("mali"), py_str("ivory coast"), py_str("liberia"), py_str("sierra leone")]), py_str("")])), (py_str("guinea-bissau"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("senegal"), py_str("guinea")]), py_str("")])), (py_str("ivory coast"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("liberia"), py_str("guinea"), py_str("mali"), py_str("burkina faso"), py_str("ghana")]), py_str("")])), (py_str("kenya"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("ethiopia"), py_str("somalia"), py_str("south sudan"), py_str("uganda"), py_str("tanzania")]), py_str("")])), (py_str("lesotho"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("south africa")]), py_str("South Africa surrounds it completely.")])), (py_str("liberia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("sierra leone"), py_str("guinea"), py_str("ivory coast")]), py_str("")])), (py_str("libya"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("tunisia"), py_str("algeria"), py_str("niger"), py_str("chad"), py_str("sudan"), py_str("egypt")]), py_str("")])), (py_str("madagascar"), py_tuple(vec![py_str("Africa"), py_tuple(vec![]), py_str("It is the fourth largest island in the world, off the east coast of Africa.")])), (py_str("malawi"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("zambia"), py_str("tanzania"), py_str("mozambique")]), py_str("")])), (py_str("mali"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("algeria"), py_str("niger"), py_str("burkina faso"), py_str("ivory coast"), py_str("guinea"), py_str("senegal"), py_str("mauritania")]), py_str("")])), (py_str("mauritania"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("western sahara"), py_str("algeria"), py_str("mali"), py_str("senegal")]), py_str("")])), (py_str("mauritius"), py_tuple(vec![py_str("Africa"), py_tuple(vec![]), py_str("It is an island country in the Indian Ocean.")])), (py_str("morocco"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("algeria"), py_str("western sahara")]), py_str("It also borders Spain at Ceuta and Melilla, two small Spanish territories on its coast.")])), (py_str("mozambique"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("tanzania"), py_str("malawi"), py_str("zambia"), py_str("zimbabwe"), py_str("south africa"), py_str("eswatini")]), py_str("")])), (py_str("namibia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("angola"), py_str("zambia"), py_str("botswana"), py_str("south africa")]), py_str("")])), (py_str("niger"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("algeria"), py_str("libya"), py_str("chad"), py_str("nigeria"), py_str("benin"), py_str("burkina faso"), py_str("mali")]), py_str("")])), (py_str("nigeria"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("benin"), py_str("niger"), py_str("chad"), py_str("cameroon")]), py_str("")])), (py_str("rwanda"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("uganda"), py_str("tanzania"), py_str("burundi"), py_str("democratic republic of the congo")]), py_str("")])), (py_str("senegal"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("mauritania"), py_str("mali"), py_str("guinea"), py_str("guinea-bissau"), py_str("gambia")]), py_str("")])), (py_str("sierra leone"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("guinea"), py_str("liberia")]), py_str("")])), (py_str("somalia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("djibouti"), py_str("ethiopia"), py_str("kenya")]), py_str("")])), (py_str("south africa"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("namibia"), py_str("botswana"), py_str("zimbabwe"), py_str("mozambique"), py_str("eswatini"), py_str("lesotho")]), py_str("It completely surrounds Lesotho, which is the third country in the world to sit inside another.")])), (py_str("south sudan"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("sudan"), py_str("ethiopia"), py_str("kenya"), py_str("uganda"), py_str("democratic republic of the congo"), py_str("central african republic")]), py_str("")])), (py_str("sudan"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("egypt"), py_str("libya"), py_str("chad"), py_str("central african republic"), py_str("south sudan"), py_str("ethiopia"), py_str("eritrea")]), py_str("")])), (py_str("tanzania"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("kenya"), py_str("uganda"), py_str("rwanda"), py_str("burundi"), py_str("zambia"), py_str("malawi"), py_str("mozambique"), py_str("democratic republic of the congo")]), py_str("")])), (py_str("togo"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("ghana"), py_str("burkina faso"), py_str("benin")]), py_str("")])), (py_str("tunisia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("algeria"), py_str("libya")]), py_str("")])), (py_str("uganda"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("south sudan"), py_str("kenya"), py_str("tanzania"), py_str("rwanda"), py_str("democratic republic of the congo")]), py_str("")])), (py_str("western sahara"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("morocco"), py_str("algeria"), py_str("mauritania")]), py_str("It is a disputed territory, not a country most lists count.")])), (py_str("zambia"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("angola"), py_str("democratic republic of the congo"), py_str("tanzania"), py_str("malawi"), py_str("mozambique"), py_str("zimbabwe"), py_str("botswana"), py_str("namibia")]), py_str("")])), (py_str("zimbabwe"), py_tuple(vec![py_str("Africa"), py_tuple(vec![py_str("zambia"), py_str("mozambique"), py_str("south africa"), py_str("botswana")]), py_str("")])), (py_str("antigua and barbuda"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is a group of islands in the Caribbean.")])), (py_str("bahamas"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is a group of about 700 islands off Florida.")])), (py_str("barbados"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is an island in the eastern Caribbean.")])), (py_str("belize"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("mexico"), py_str("guatemala")]), py_str("")])), (py_str("canada"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("united states")]), py_str("The border with the United States is the longest land border in the world, about 8,890 km.")])), (py_str("costa rica"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("nicaragua"), py_str("panama")]), py_str("")])), (py_str("cuba"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is the largest island in the Caribbean.")])), (py_str("dominica"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is an island in the eastern Caribbean.")])), (py_str("dominican republic"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("haiti")]), py_str("It shares the island of Hispaniola with Haiti.")])), (py_str("el salvador"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("guatemala"), py_str("honduras")]), py_str("")])), (py_str("grenada"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is an island in the eastern Caribbean.")])), (py_str("guatemala"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("mexico"), py_str("belize"), py_str("honduras"), py_str("el salvador")]), py_str("")])), (py_str("haiti"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("dominican republic")]), py_str("It shares the island of Hispaniola with the Dominican Republic.")])), (py_str("honduras"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("guatemala"), py_str("el salvador"), py_str("nicaragua")]), py_str("")])), (py_str("jamaica"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is an island south of Cuba.")])), (py_str("mexico"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("united states"), py_str("guatemala"), py_str("belize")]), py_str("")])), (py_str("nicaragua"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("honduras"), py_str("costa rica")]), py_str("")])), (py_str("panama"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("costa rica"), py_str("colombia")]), py_str("The border with Colombia is the one where Central America meets South America.")])), (py_str("saint kitts and nevis"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is a two-island country in the Caribbean.")])), (py_str("saint lucia"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is an island in the eastern Caribbean.")])), (py_str("saint vincent and the grenadines"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is a group of Caribbean islands.")])), (py_str("trinidad and tobago"), py_tuple(vec![py_str("North America"), py_tuple(vec![]), py_str("It is a two-island country off Venezuela.")])), (py_str("united states"), py_tuple(vec![py_str("North America"), py_tuple(vec![py_str("canada"), py_str("mexico")]), py_str("It has 50 states, and Alaska is the part that touches Canada's west.")])), (py_str("argentina"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("chile"), py_str("bolivia"), py_str("paraguay"), py_str("brazil"), py_str("uruguay")]), py_str("")])), (py_str("bolivia"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("peru"), py_str("brazil"), py_str("paraguay"), py_str("argentina"), py_str("chile")]), py_str("")])), (py_str("brazil"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("french guiana"), py_str("suriname"), py_str("guyana"), py_str("venezuela"), py_str("colombia"), py_str("peru"), py_str("bolivia"), py_str("paraguay"), py_str("argentina"), py_str("uruguay")]), py_str("It borders every country in South America except Chile and Ecuador, and French Guiana is part of France.")])), (py_str("chile"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("peru"), py_str("bolivia"), py_str("argentina")]), py_str("")])), (py_str("colombia"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("panama"), py_str("venezuela"), py_str("brazil"), py_str("peru"), py_str("ecuador")]), py_str("")])), (py_str("ecuador"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("colombia"), py_str("peru")]), py_str("")])), (py_str("guyana"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("venezuela"), py_str("brazil"), py_str("suriname")]), py_str("")])), (py_str("paraguay"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("bolivia"), py_str("brazil"), py_str("argentina")]), py_str("")])), (py_str("peru"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("ecuador"), py_str("colombia"), py_str("brazil"), py_str("bolivia"), py_str("chile")]), py_str("")])), (py_str("suriname"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("guyana"), py_str("brazil"), py_str("french guiana")]), py_str("")])), (py_str("uruguay"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("brazil"), py_str("argentina")]), py_str("")])), (py_str("venezuela"), py_tuple(vec![py_str("South America"), py_tuple(vec![py_str("colombia"), py_str("brazil"), py_str("guyana")]), py_str("")])), (py_str("australia"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is the only country that is a whole continent, so it has no land border.")])), (py_str("fiji"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is a group of more than 300 islands in the Pacific.")])), (py_str("new zealand"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is two main islands in the south Pacific, with no land border.")])), (py_str("papua new guinea"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![py_str("indonesia")]), py_str("It shares the island of New Guinea with Indonesia.")])), (py_str("samoa"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is an island country in the Pacific.")])), (py_str("solomon islands"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is a group of islands in the Pacific.")])), (py_str("tonga"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is a group of about 170 islands in the Pacific.")])), (py_str("vanuatu"), py_tuple(vec![py_str("Oceania"), py_tuple(vec![]), py_str("It is a group of islands in the Pacific.")]))]));
    s__NOT_A_COUNTRY(py_dict(vec![(py_str("french guiana"), py_str("French Guiana (France)"))]));
    s__COUNTRY_ALIASES(py_dict(vec![(py_str("usa"), py_str("united states")), (py_str("u s a"), py_str("united states")), (py_str("the united states"), py_str("united states")), (py_str("united states of america"), py_str("united states")), (py_str("the states"), py_str("united states")), (py_str("uk"), py_str("united kingdom")), (py_str("the uk"), py_str("united kingdom")), (py_str("britain"), py_str("united kingdom")), (py_str("great britain"), py_str("united kingdom")), (py_str("the united kingdom"), py_str("united kingdom")), (py_str("holland"), py_str("netherlands")), (py_str("burma"), py_str("myanmar")), (py_str("czechia"), py_str("czech republic")), (py_str("swaziland"), py_str("eswatini")), (py_str("cabo verde"), py_str("cape verde")), (py_str("macedonia"), py_str("north macedonia")), (py_str("east timor"), py_str("timor-leste")), (py_str("ivory coast"), py_str("ivory coast")), (py_str("cote divoire"), py_str("ivory coast")), (py_str("uae"), py_str("united arab emirates")), (py_str("the uae"), py_str("united arab emirates")), (py_str("saudi"), py_str("saudi arabia")), (py_str("the philippines"), py_str("philippines")), (py_str("the netherlands"), py_str("netherlands")), (py_str("the bahamas"), py_str("bahamas")), (py_str("the gambia"), py_str("gambia")), (py_str("the congo"), py_str("republic of the congo")), (py_str("drc"), py_str("democratic republic of the congo")), (py_str("vatican"), py_str("vatican city")), (py_str("the vatican"), py_str("vatican city")), (py_str("holy see"), py_str("vatican city"))]));
    s__COUNTRY_PHRASES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&call_builtin("set", &[g_COUNTRIES()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g__COUNTRY_ALIASES()]).unwrap_or_else(|e| exc::raise(e)))); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COUNTRY_WORDS(op_add(&call_builtin("tuple", &[g_COUNTRIES()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[g__COUNTRY_ALIASES()]).unwrap_or_else(|e| exc::raise(e))));
    s_TAKES_THE(py_tuple(vec![py_str("united states"), py_str("united kingdom"), py_str("netherlands"), py_str("united arab emirates"), py_str("philippines"), py_str("bahamas"), py_str("gambia"), py_str("maldives"), py_str("central african republic"), py_str("democratic republic of the congo"), py_str("republic of the congo")]));
    s__CONTINENTS(py_dict(vec![(py_str("europe"), py_str("Europe")), (py_str("european"), py_str("Europe")), (py_str("asia"), py_str("Asia")), (py_str("asian"), py_str("Asia")), (py_str("africa"), py_str("Africa")), (py_str("african"), py_str("Africa")), (py_str("north america"), py_str("North America")), (py_str("south america"), py_str("South America")), (py_str("oceania"), py_str("Oceania")), (py_str("australasia"), py_str("Oceania"))]));
    s__CONTINENT_NOTES(py_dict(vec![(py_str("Europe"), py_str("Kosovo is counted by some lists and not others, and Russia and Turkey straddle Europe and Asia, so the total depends on where you draw the edge.")), (py_str("Asia"), py_str("Russia and Turkey are usually counted in Europe as well, and Taiwan is counted by some lists and not others.")), (py_str("Africa"), py_str("Western Sahara is counted by some lists and not others.")), (py_str("North America"), py_str("Central America and the Caribbean are part of North America, which is why the count is higher than the three big countries people think of.")), (py_str("South America"), py_str("It is 12 countries plus French Guiana, which is part of France.")), (py_str("Oceania"), py_str("The usual count is 14, with the Pacific island states making up most of them."))]));
    s__CONTINENT_NOTES_BY_COUNTRY(py_dict(vec![(py_str("russia"), py_str("It is the biggest country in the world, and most of its land is in Asia, but most of its people live in the European part.")), (py_str("turkey"), py_str("Most of it is in Asia, with a small part in Europe across the Bosphorus, and Istanbul sits on both sides.")), (py_str("egypt"), py_str("The Sinai peninsula, across the Suez Canal, is in Asia, which is why Egypt is sometimes called transcontinental.")), (py_str("georgia"), py_str("The US state of Georgia is in North America, not Europe.")), (py_str("cyprus"), py_str("It is an island in the eastern Mediterranean, closer to Turkey and Syria than to Greece, but it is in the EU.")), (py_str("australia"), py_str("It is the only country that is a whole continent, which is why the answer is both.")), (py_str("kazakhstan"), py_str("A small part of it west of the Ural river is in Europe.")), (py_str("spain"), py_str("It is in Europe, and it also has two small pieces of land in Africa, Ceuta and Melilla."))]));
    s_EU_MEMBERS(py_tuple(vec![py_str("austria"), py_str("belgium"), py_str("bulgaria"), py_str("croatia"), py_str("cyprus"), py_str("czech republic"), py_str("denmark"), py_str("estonia"), py_str("finland"), py_str("france"), py_str("germany"), py_str("greece"), py_str("hungary"), py_str("ireland"), py_str("italy"), py_str("latvia"), py_str("lithuania"), py_str("luxembourg"), py_str("malta"), py_str("netherlands"), py_str("poland"), py_str("portugal"), py_str("romania"), py_str("slovakia"), py_str("slovenia"), py_str("spain"), py_str("sweden")]));
    s__NOT_IN_EU(py_dict(vec![(py_str("united kingdom"), py_str("the UK left the EU on 31 January 2020, after the Brexit vote in 2016.")), (py_str("norway"), py_str("it is in the single market through the EEA, so it follows most EU rules without a vote on them.")), (py_str("switzerland"), py_str("it has a set of agreements with the EU instead, and its own currency.")), (py_str("iceland"), py_str("it is in the single market through the EEA, and not a member.")), (py_str("turkey"), py_str("it has been a candidate for years, and talks have been frozen since 2018.")), (py_str("ukraine"), py_str("it applied in 2022 and was made a candidate, but joining takes years.")), (py_str("serbia"), py_str("it is a candidate, in talks since 2014.")), (py_str("montenegro"), py_str("it is a candidate, and the furthest along of the Balkan countries.")), (py_str("bosnia and herzegovina"), py_str("it is a candidate, since 2022.")), (py_str("georgia"), py_str("it is a candidate, since 2023.")), (py_str("moldova"), py_str("it is a candidate, since 2022.")), (py_str("albania"), py_str("it is a candidate, in talks since 2024.")), (py_str("north macedonia"), py_str("it is a candidate, in talks since 2022.")), (py_str("kosovo"), py_str("it applied in 2022 but is not a candidate, partly because five EU countries do not recognise it.")), (py_str("russia"), py_str("it is not a member and never has been."))]));
    s__COMPARES_NATIONS(py_tuple(vec![py_str("difference between"), py_str("different from"), py_str("same as"), py_str("the same"), py_str("difference"), py_str("versus"), py_str(" vs "), py_str("or britain"), py_str("or the uk"), py_str("or england"), py_str("british isles"), py_str("great britain"), py_str("the uk and britain"), py_str("britain and the uk")]));
    s__NATION_NAMED(stdlib_call("re", "compile", &[py_str("\\b(northern ireland|great britain|united kingdom|england|scotland|wales|britain|uk|ireland|europe)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NATION_SAID(py_dict(vec![(py_str("uk"), py_str("the UK")), (py_str("united kingdom"), py_str("the UK")), (py_str("britain"), py_str("Britain")), (py_str("great britain"), py_str("Great Britain")), (py_str("england"), py_str("England")), (py_str("scotland"), py_str("Scotland")), (py_str("wales"), py_str("Wales")), (py_str("northern ireland"), py_str("Northern Ireland")), (py_str("ireland"), py_str("Ireland"))]));
    s__THE_FOUR(py_tuple(vec![py_str("england"), py_str("scotland"), py_str("wales"), py_str("northern ireland")]));
    s__RIVER_NAMES(py_tuple(vec![py_str("nile"), py_str("amazon"), py_str("mississippi"), py_str("missouri"), py_str("danube"), py_str("volga"), py_str("yangtze"), py_str("congo"), py_str("rhine"), py_str("seine"), py_str("thames"), py_str("loire"), py_str("po"), py_str("tagus"), py_str("elbe"), py_str("oder"), py_str("ganges"), py_str("indus"), py_str("mekong"), py_str("yellow river"), py_str("ob"), py_str("yenisei"), py_str("murray"), py_str("darling"), py_str("colorado"), py_str("rio grande"), py_str("parana"), py_str("orinoco"), py_str("niger"), py_str("zambezi"), py_str("yangtze kiang"), py_str("amur"), py_str("lena"), py_str("irrawaddy"), py_str("euphrates"), py_str("tigris")]));
    s__RIVER_LENGTHS(py_tuple(vec![py_tuple(vec![py_str("nile"), py_str("The Nile runs about 6,650 km from its source to the Mediterranean, which makes it the longest river in Africa and usually the longest in the world - with the Amazon a close second and carrying far more water.")]), py_tuple(vec![py_str("amazon"), py_str("The Amazon runs about 6,400 km from its source in the Peruvian Andes to the Atlantic, and it is the biggest river in the world by water - about a fifth of all the river water on Earth.")]), py_tuple(vec![py_str("mississippi"), py_str("The Mississippi is about 3,770 km from Minnesota to the Gulf of Mexico, or about 6,275 km if you count the Missouri from its source, which is how it is usually measured.")]), py_tuple(vec![py_str("missouri"), py_str("The Missouri is about 3,767 km, from Montana to where it joins the Mississippi near St Louis - the longest river in North America on its own.")]), py_tuple(vec![py_str("danube"), py_str("The Danube runs about 2,850 km from Germany's Black Forest to the Black Sea, through ten countries - more than any other river in the world.")]), py_tuple(vec![py_str("volga"), py_str("The Volga runs about 3,530 km through Russia to the Caspian Sea, the longest river in Europe.")]), py_tuple(vec![py_str("yangtze"), py_str("The Yangtze runs about 6,300 km across China to the sea at Shanghai, the longest river in Asia.")]), py_tuple(vec![py_str("congo"), py_str("The Congo runs about 4,700 km, the deepest river in the world and second only to the Amazon by water.")]), py_tuple(vec![py_str("rhine"), py_str("The Rhine runs about 1,230 km from the Swiss Alps to the North Sea at Rotterdam, and it is the busiest freight river in Europe.")]), py_tuple(vec![py_str("seine"), py_str("The Seine runs about 775 km, from Burgundy through Paris to the sea at Le Havre.")]), py_tuple(vec![py_str("thames"), py_str("The Thames runs about 346 km, from the Cotswolds through London to the Thames Estuary.")]), py_tuple(vec![py_str("loire"), py_str("The Loire runs about 1,006 km, the longest river in France, from the Massif Central to the Atlantic at Saint-Nazaire.")]), py_tuple(vec![py_str("po"), py_str("The Po runs about 652 km across northern Italy to the Adriatic, the longest river in Italy.")]), py_tuple(vec![py_str("tagus"), py_str("The Tagus runs about 1,007 km from Spain through Portugal to Lisbon and the Atlantic, the longest river on the Iberian peninsula.")]), py_tuple(vec![py_str("elbe"), py_str("The Elbe runs about 1,094 km from the Czech Republic through Germany to the North Sea at Hamburg.")]), py_tuple(vec![py_str("oder"), py_str("The Oder runs about 840 km from the Czech Republic to the Baltic, most of it along the border between Germany and Poland.")]), py_tuple(vec![py_str("ganges"), py_str("The Ganges runs about 2,525 km from the Himalayas to the Bay of Bengal, and with its delta it is one of the most populous river basins in the world.")]), py_tuple(vec![py_str("indus"), py_str("The Indus runs about 3,180 km from Tibet through Pakistan to the Arabian Sea, and the civilisation along it was one of the first cities anywhere.")]), py_tuple(vec![py_str("mekong"), py_str("The Mekong runs about 4,350 km from the Tibetan plateau through China, Laos, Thailand, Cambodia and Vietnam to the South China Sea.")]), py_tuple(vec![py_str("yellow river"), py_str("The Yellow River runs about 5,464 km across northern China, the second longest in the country after the Yangtze.")]), py_tuple(vec![py_str("ob"), py_str("The Ob runs about 3,650 km across Siberia to the Arctic Ocean, or about 5,410 km with its main tributary, the Irtysh.")]), py_tuple(vec![py_str("yenisei"), py_str("The Yenisei runs about 3,487 km across Siberia to the Arctic, and with its headwaters it is one of the longest river systems in the world.")]), py_tuple(vec![py_str("murray"), py_str("The Murray runs about 2,508 km through Australia to the sea in South Australia, and with the Darling it is the longest river system in the country.")]), py_tuple(vec![py_str("darling"), py_str("The Darling runs about 1,472 km through New South Wales and Queensland, the longest single river in Australia.")]), py_tuple(vec![py_str("colorado"), py_str("The Colorado runs about 2,330 km from the Rocky Mountains to the Gulf of California, through the Grand Canyon on the way.")]), py_tuple(vec![py_str("rio grande"), py_str("The Rio Grande runs about 3,051 km from Colorado to the Gulf of Mexico, and it forms the border between Texas and Mexico.")]), py_tuple(vec![py_str("parana"), py_str("The Parana runs about 4,880 km through Brazil, Paraguay and Argentina to the Rio de la Plata, second in South America only to the Amazon.")]), py_tuple(vec![py_str("orinoco"), py_str("The Orinoco runs about 2,140 km through Venezuela to the Atlantic, and it links to the Amazon through a natural channel, the Casiquiare.")]), py_tuple(vec![py_str("niger"), py_str("The Niger runs about 4,200 km in a great curve through West Africa to the Gulf of Guinea, the longest river in the region.")]), py_tuple(vec![py_str("zambezi"), py_str("The Zambezi runs about 2,574 km through southern Africa to the Indian Ocean, and Victoria Falls is on it.")]), py_tuple(vec![py_str("amur"), py_str("The Amur runs about 2,824 km along the border between Russia and China to the Pacific, or about 4,444 km with its headwaters.")]), py_tuple(vec![py_str("lena"), py_str("The Lena runs about 4,400 km across Siberia to the Arctic Ocean, and it freezes solid for months each winter.")]), py_tuple(vec![py_str("irrawaddy"), py_str("The Irrawaddy runs about 2,170 km through Myanmar to the sea, the country's main river for transport and rice.")]), py_tuple(vec![py_str("euphrates"), py_str("The Euphrates runs about 2,800 km from Turkey through Syria and Iraq to the Shatt al-Arab, where it joins the Tigris.")]), py_tuple(vec![py_str("tigris"), py_str("The Tigris runs about 1,850 km from Turkey through Iraq to the Shatt al-Arab, and the land between it and the Euphrates is ancient Mesopotamia.")])]));
    s__RIVER_NAME_NEEDS_RIVER(call_builtin("frozenset", &[py_tuple(vec![py_str("amazon"), py_str("po"), py_str("ob"), py_str("lena"), py_str("darling"), py_str("congo"), py_str("niger"), py_str("elbe"), py_str("oder"), py_str("amur"), py_str("parana"), py_str("colorado"), py_str("murray")])]).unwrap_or_else(|e| exc::raise(e)));
    s__RIVER_LENGTH_ASK(stdlib_call("re", "compile", &[py_str("\\b(how long|how far|long is|length|km|kilometres|kilometers|miles|distance)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("continent-record")); __args.push(py_tuple(vec![op_add(&op_add(&op_add(&g__RECORD_BIGGEST(), &g__RECORD_SMALLEST()), &g__RECORD_POPULOUS()), &g__LEAST()), py_tuple(vec![py_str("africa"), py_str("african"), py_str("europe"), py_str("european"), py_str("asia"), py_str("asian"), py_str("north america"), py_str("north american"), py_str("south america"), py_str("south american"), py_str("oceania"), py_str("antarctica")])])); __args.push(g__continent_record()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("how many"), py_str("number of"), py_str("count"), py_str("world"), py_str("highest"), py_str("longest"), py_str("deepest"), py_str("mountain"), py_str("river"), py_str("lake"), py_str("desert"), py_str("island"), py_str("islands"), py_str("ocean"), py_str("sea"), py_str("city"), py_str("cities"), py_str("capital"), py_str("continent"), py_str("continents"), py_str("economy"), py_str("economies"), py_str("gdp"), py_str("richest"), py_str("poorest"), py_str("wealthiest"), py_str("richer"), py_str("poorer"), py_str("second"), py_str("third"), py_str("fourth"), py_str("fifth"), py_str("next biggest"), py_str("south east asia"), py_str("southeast asia"), py_str("east asia"), py_str("central asia"), py_str("south asia"), py_str("west africa"), py_str("east africa"), py_str("north africa"), py_str("southern africa"), py_str("sub saharan"), py_str("western europe"), py_str("eastern europe"), py_str("northern europe"), py_str("southern europe"), py_str("middle east"), py_str("european union"), py_str("scandinavia"), py_str("balkans"), py_str("latin america")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("countries-count")); __args.push(py_tuple(vec![py_str("countries"), py_tuple(vec![py_str("how many"), py_str("world"), py_str("in the world")])])); __args.push(_world(&[py_str("The usual answer is 195: 193 members of the United Nations plus the Vatican and Palestine, which are observers. Some lists count Taiwan or Kosovo, which is why you sometimes see 196 or 197.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("smallest"), py_str("tiniest"), py_str("populous"), py_str("have i"), py_str("i've"), py_str("ive"), py_str("been to"), py_str("visited"), py_str("africa"), py_str("europe"), py_str("asia"), py_str("north america"), py_str("south america"), py_str("oceania"), py_str("antarctica"), py_str("uk"), py_str("united kingdom"), py_str("britain"), py_str("great britain"), py_str("states"), py_str("have"), py_str("has"), py_str("with"), py_str("without"), py_str("allow"), py_str("allows"), py_str("legal"), py_str("ban"), py_str("bans"), py_str("speak"), py_str("speaks"), py_str("drive"), py_str("use"), py_str("recognise"), py_str("recognize"), py_str("border"), py_str("borders"), py_str("next to"), py_str("neighbour"), py_str("neighbours"), py_str("adjacent")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("countries-healthcare")); __args.push(py_tuple(vec![py_str("countries"), py_tuple(vec![py_str("free healthcare"), py_str("free health care"), py_str("universal healthcare"), py_str("universal health care"), py_str("healthcare"), py_str("health care")])])); __args.push(py_str("There's no single number: it depends whether you count tax-funded systems, insurance schemes and small fees. The WHO's universal health coverage index and Our World in Data's country list are the usual places to check the current figure.")); __kws.extend(vec![("avoid".to_string(), op_add(&g__REGIONS(), &py_tuple(vec![py_str("in the world")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("continent-count")); __args.push(py_tuple(vec![py_str("countries"), py_tuple(vec![py_str("africa"), py_str("europe"), py_str("asia"), py_str("north america"), py_str("south america"), py_str("oceania"), py_str("antarctica")])])); __args.push(g__continent_count()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("smallest"), py_str("tiniest"), py_str("populous"), py_str("richest"), py_str("poorest"), py_str("most populous"), py_str("population"), py_str("longest"), py_str("shortest"), py_str("tallest"), py_str("highest"), py_str("lowest"), py_str("deepest"), py_str("cheapest"), py_str("have i"), py_str("i've"), py_str("ive"), py_str("been to"), py_str("visited"), py_str("which"), py_str("list"), py_str("name"), py_str("speak"), py_str("language"), py_str("languages"), py_str("currency"), py_str("euro"), py_str("capital"), py_str("monarchy"), py_str("monarchies"), py_str("free"), py_str("healthcare"), py_str("health care"), py_str("death penalty"), py_str("capital punishment"), py_str("abortion"), py_str("weed"), py_str("marijuana"), py_str("same sex"), py_str("gay marriage"), py_str("drive"), py_str("recognise"), py_str("recognize")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("uk-countries")); __args.push(py_tuple(vec![py_str("countries"), py_tuple(vec![py_str("uk"), py_str("united kingdom"), py_str("britain"), py_str("great britain")])])); __args.push(py_str("The UK is four countries: England, Scotland, Wales and Northern Ireland. Great Britain is the island with the first three on it, which is why people mix the two up.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("city"), py_str("cities"), py_str("capital"), py_str("have i"), py_str("visited"), py_str("have"), py_str("has"), py_str("with"), py_str("without")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("us-states")); __args.push(py_tuple(vec![py_str("states"), py_tuple(vec![py_str("us"), py_str("usa"), py_str("united states"), py_str("america")])])); __args.push(py_str("The United States has 50 states, plus Washington DC, which is a federal district rather than a state. Alaska and Hawaii are the two outside the mainland.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("australia"), py_str("india"), py_str("mexico"), py_str("brazil"), py_str("south america"), py_str("north america"), py_str("central america"), py_str("have i"), py_str("been to"), py_str("visited"), py_str("which"), py_str("list"), py_str("name"), py_str("capital"), py_str("capital city"), py_str("have"), py_str("has"), py_str("with"), py_str("without"), py_str("legal"), py_str("allow"), py_str("allows"), py_str("banned"), py_str("ban"), py_str("weed"), py_str("marijuana"), py_str("abortion"), py_str("vote")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, crate::knowledge::lookup::entry(&[py_str("named-peak"), py_tuple(vec![call_builtin("tuple", &[g_PEAKS()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("high"), py_str("higher"), py_str("height"), py_str("tall"), py_str("taller"), py_str("metres"), py_str("meters"), py_str("feet"), py_str("how high"), py_str("how tall"), py_str("summit"), py_str("big")])]), g__peak()]), crate::knowledge::lookup::entry(&[py_str("highest-in-place"), py_tuple(vec![py_tuple(vec![py_str("highest"), py_str("tallest"), py_str("high"), py_str("biggest"), py_str("highest mountain")]), py_tuple(vec![py_str("mountain"), py_str("mount"), py_str("peak"), py_str("point"), py_str("summit")])]), g__highest_in_place()]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("highest-mountain")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mountain"), py_str("mount"), py_str("everest")]), py_tuple(vec![py_str("highest"), py_str("tallest"), py_str("everest"), py_str("world"), py_str("tall"), py_str("high"), py_str("how"), py_str("metres"), py_str("feet"), py_str("big")])])); __args.push(region_extreme(&[py_str("mountain"), py_str("Mount Everest is the highest, at 8,849 metres (29,032 feet) above sea level. Measured from the base to the summit, Mauna Kea in Hawaii is taller, because most of it is under the sea.")])); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("everest"), py_str("highest"), py_str("tallest"), py_str("world")])), ("avoid".to_string(), op_add(&op_add(&py_tuple(vec![py_str("how many"), py_str("number of"), py_str("count"), py_str("climb"), py_str("climbing"), py_str("deaths"), py_str("died")]), &call_builtin("tuple", &[g_PEAKS()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("tuple", &[g_HIGHEST_IN()]).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("oceans-count")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ocean"), py_str("oceans"), py_str("sea")]), py_tuple(vec![py_str("how many"), py_str("five"), py_str("world"), py_str("largest"), py_str("biggest"), py_str("deepest"), py_str("deep")])])); __args.push(_world(&[py_str("There are five oceans: Pacific, Atlantic, Indian, Southern and Arctic. The Pacific is the biggest and deepest; its Mariana Trench goes down about 11 km, deeper than Everest is tall.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how deep"), py_str("how far down"), py_str("depth"), py_str("deep is")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, crate::knowledge::lookup::entry(&[py_str("ocean-depth"), py_tuple(vec![py_tuple(vec![py_str("ocean"), py_str("oceans"), py_str("sea")]), py_tuple(vec![py_str("how deep"), py_str("how far down"), py_str("depth"), py_str("mariana"), py_str("trench")])]), _world(&[py_str("The ocean averages about 3.7 km deep. The deepest point is the Mariana Trench in the Pacific, at about 10.9 km down - if Everest stood there, its summit would still be more than 2 km under the surface.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("continents-count")); __args.push(py_tuple(vec![py_tuple(vec![py_str("continent"), py_str("continents")]), py_tuple(vec![py_str("how many"), py_str("seven"), py_str("world")])])); __args.push(_world(&[py_str("Seven is the usual count: Africa, Antarctica, Asia, Europe, North America, Oceania and South America. Some countries teach six, joining Europe and Asia into Eurasia.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("smallest")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("world-population")); __args.push(py_tuple(vec![py_tuple(vec![py_str("population"), py_str("people")]), py_tuple(vec![py_str("world"), py_str("earth")])])); __args.push(_world(&[py_str("About 8.2 billion people, and it is still rising, though more slowly. India passed China as the most populous country in 2023, and both have around 1.4 billion.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("hungry"), py_str("starving"), py_str("poor"), py_str("without"), py_str("have"), py_str("has"), py_str("live in"), py_str("living in"), py_str("speak"), py_str("speaking"), py_str("died"), py_str("die"), py_str("born"), py_str("own"), py_str("use"), py_str("need")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("largest-desert")); __args.push(py_tuple(vec![py_str("desert"), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("hot"), py_str("world")])])); __args.push(region_extreme(&[py_str("desert"), py_str("Antarctica is technically the largest desert, because a desert is defined by how little it rains. The Sahara is the largest hot desert, at about 9 million square kilometres, roughly the size of the United States.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how many deserts"), py_str("number of deserts"), py_str("count deserts")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("largest-island")); __args.push(py_tuple(vec![py_str("island"), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("world")])])); __args.push(_world(&[py_str("Greenland is the largest island, at about 2.2 million square kilometres. Australia is bigger but is counted as a continent rather than an island, which is the usual quiz gotcha.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how many islands"), py_str("number of islands"), py_str("count islands")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smallest-country")); __args.push(py_tuple(vec![py_str("country"), py_tuple(vec![py_str("smallest"), py_str("world")])])); __args.push(country_extreme(&[py_str("small"), py_str("Vatican City is the smallest country, at about 0.49 square kilometres and roughly 800 people. Monaco is the second smallest, and both are smaller than most city parks.")])); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("how many countries"), py_str("number of countries"), py_str("count countries"), py_str("largest"), py_str("biggest"), py_str("largest by area"), py_str("biggest by area"), py_str("population"), py_str("populous"), py_str("people live"), py_str("inhabitants"), py_str("most people"), py_str("have"), py_str("has"), py_str("with"), py_str("without"), py_str("allow"), py_str("allows"), py_str("legal"), py_str("ban"), py_str("bans")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("largest-continent")); __args.push(py_tuple(vec![py_str("continent"), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("world")])])); __args.push(_world(&[py_str("Asia is the largest continent, at about 44.6 million square kilometres - roughly 30% of the world's land. Africa is second and Antarctica third.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how many"), py_str("number of"), py_str("count"), py_str("smallest")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smallest-continent")); __args.push(py_tuple(vec![py_str("continent"), py_tuple(vec![py_str("smallest"), py_str("world")])])); __args.push(_world(&[py_str("Australia is the smallest continent, at about 8.6 million square kilometres. Some lists call the region Oceania, which also includes New Zealand and the Pacific islands, but Australia is the continent itself.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how many"), py_str("number of"), py_str("count"), py_str("largest"), py_str("biggest")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, crate::knowledge::lookup::entry(&[py_str("highest-waterfall"), py_tuple(vec![py_tuple(vec![py_str("waterfall"), py_str("falls")]), py_tuple(vec![py_str("highest"), py_str("tallest"), py_str("world")])]), _world(&[py_str("Angel Falls in Venezuela is the highest waterfall, at 979 metres. Its water drops in one unbroken fall of about 807 metres, so it is still falling long after Niagara would have hit the bottom.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("largest-lake")); __args.push(py_tuple(vec![py_str("lake"), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("world")])])); __args.push(region_extreme(&[py_str("lake"), py_str("The Caspian Sea is the largest lake, at about 371,000 square kilometres, and it is salty. Lake Superior is the largest freshwater lake by area; Lake Baikal holds the most fresh water.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how many"), py_str("number of"), py_str("count"), py_str("deep"), py_str("deepest"), py_str("depth"), py_str("near me"), py_str("nearby"), py_str("local"), py_str("around here")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("deepest-lake")); __args.push(py_tuple(vec![py_str("lake"), py_tuple(vec![py_str("deepest"), py_str("depth"), py_str("deep")])])); __args.push(region_extreme(&[py_str("deep lake"), py_str("Lake Baikal in Siberia is the deepest lake, at 1,642 metres. It holds about a fifth of the world's unfrozen fresh water, more than all the Great Lakes together.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("largest"), py_str("biggest"), py_str("near me"), py_str("nearby"), py_str("local"), py_str("around here")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, crate::knowledge::lookup::entry(&[py_str("longest-mountain-range"), py_tuple(vec![py_tuple(vec![py_str("mountain range"), py_str("range of mountains")]), py_tuple(vec![py_str("longest"), py_str("biggest"), py_str("world")])]), _world(&[py_str("The Andes is the longest mountain range above sea level, running about 7,000 km down the west of South America. The Mid-Atlantic Ridge is longer, but almost all of it is under the sea.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hottest-place")); __args.push(py_tuple(vec![py_str("hottest"), py_tuple(vec![py_str("place"), py_str("earth"), py_str("world"), py_str("temperature"), py_str("ever"), py_str("recorded")])])); __args.push(_world(&[py_str("Death Valley in California holds the highest air temperature reliably recorded: 56.7C in 1913. The Danakil Depression in Ethiopia and Dasht-e Lut in Iran are often called the hottest places on average, because they are hot for longer.")])); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("place"), py_str("earth"), py_str("world"), py_str("planet"), py_str("on earth"), py_str("in the world"), py_str("temperature")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coldest-place")); __args.push(py_tuple(vec![py_str("coldest"), py_tuple(vec![py_str("place"), py_str("earth"), py_str("world"), py_str("temperature"), py_str("ever"), py_str("recorded")])])); __args.push(_world(&[py_str("Antarctica is the coldest place on Earth: Vostok Station recorded -89.2C in 1983. The coldest place with people living in it is Oymyakon in Siberia, where winter temperatures reach about -50C.")])); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("place"), py_str("earth"), py_str("world"), py_str("planet"), py_str("on earth"), py_str("in the world"), py_str("temperature")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("country-borders")); __args.push(py_tuple(vec![g__COUNTRY_WORDS(), py_tuple(vec![py_str("border"), py_str("borders"), py_str("bordering"), py_str("next to"), py_str("neighbour"), py_str("neighbours"), py_str("neighbouring"), py_str("neighboring"), py_str("neighbor"), py_str("neighbors"), py_str("adjacent"), py_str("touches"), py_str("touch"), py_str("share a border"), py_str("shares a border"), py_str("land border"), py_str("adjoin"), py_str("adjoins")])])); __args.push(g__borders()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("border"), py_str("borders"), py_str("bordering"), py_str("next to"), py_str("neighbour"), py_str("neighbours"), py_str("neighbouring"), py_str("neighboring"), py_str("neighbor"), py_str("neighbors"), py_str("adjacent"), py_str("touches"), py_str("share a border"), py_str("shares a border"), py_str("land border"), py_str("adjoin"), py_str("adjoins")])), ("avoid".to_string(), py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("trip"), py_str("visit"), py_str("visiting"), py_str("move"), py_str("moving"), py_str("moved"), py_str("emigrate"), py_str("immigrate"), py_str("visa"), py_str("flight"), py_str("flights"), py_str("fly"), py_str("flying"), py_str("going"), py_str("went"), py_str("been"), py_str("live"), py_str("lives"), py_str("living"), py_str("relocate"), py_str("hotel"), py_str("stay"), py_str("staying"), py_str("drive"), py_str("driving"), py_str("work there"), py_str("job there")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("country-continent")); __args.push(py_tuple(vec![g__COUNTRY_WORDS(), py_tuple(vec![py_str("continent"), py_str("continents"), py_str("which continent"), py_str("what continent"), py_str("part of the world"), py_str("europe"), py_str("european"), py_str("asia"), py_str("asian"), py_str("africa"), py_str("african"), py_str("north america"), py_str("south america"), py_str("oceania")])])); __args.push(g__continent()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("continent"), py_str("continents"), py_str("part of the world"), py_str("in europe"), py_str("in asia"), py_str("in africa"), py_str("in north america"), py_str("in south america"), py_str("in oceania")])), ("avoid".to_string(), py_tuple(vec![py_str("holiday"), py_str("trip"), py_str("visit"), py_str("move"), py_str("moving"), py_str("emigrate"), py_str("visa"), py_str("flight"), py_str("go to"), py_str("going to"), py_str("honeymoon"), py_str("live in"), py_str("living in"), py_str("eu"), py_str("euros 20"), py_str("european union"), py_str("world cup"), py_str("olympics"), py_str("qualify")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eu-membership")); __args.push(py_tuple(vec![op_add(&g__COUNTRY_WORDS(), &py_tuple(vec![py_str("eu"), py_str("european union")])), py_tuple(vec![py_str("eu"), py_str("european union"), py_str("in the eu"), py_str("member"), py_str("members"), py_str("part of"), py_str("join"), py_str("joined"), py_str("belong")])])); __args.push(g__in_eu()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("eu"), py_str("european union")])), ("avoid".to_string(), py_tuple(vec![py_str("holiday"), py_str("trip"), py_str("visa"), py_str("flight"), py_str("travel"), py_str("moving"), py_str("live"), py_str("living"), py_str("work there"), py_str("study there"), py_str("euro note"), py_str("euro coin"), py_str("euros"), py_str("exchange")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("continent-countries")); __args.push(py_tuple(vec![py_tuple(vec![py_str("europe"), py_str("european"), py_str("asia"), py_str("asian"), py_str("africa"), py_str("african"), py_str("north america"), py_str("south america"), py_str("oceania")]), py_tuple(vec![py_str("which countries"), py_str("what countries"), py_str("countries"), py_str("names of the countries")])])); __args.push(g__continent_countries()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("which countries"), py_str("what countries"), py_str("countries")])), ("avoid".to_string(), py_tuple(vec![py_str("how many"), py_str("number of"), py_str("visit"), py_str("holiday"), py_str("trip"), py_str("best"), py_str("should i"), py_str("worth"), py_str("cheap"), py_str("expensive"), py_str("go to"), py_str("moving"), py_str("live"), py_str("fly"), py_str("flight"), py_str("city"), py_str("cities"), py_str("capital")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("most-people-country")); __args.push(py_tuple(vec![py_tuple(vec![py_str("most people"), py_str("most populous"), py_str("biggest population"), py_str("largest population"), py_str("most population"), py_str("highest population"), py_str("most populated"), py_str("population"), py_str("populous")]), py_tuple(vec![py_str("country"), py_str("countries"), py_str("nation"), py_str("world")])])); __args.push(_world(&[py_str("India has the most people, at about 1.45 billion, just ahead of China at about 1.41 billion since 2023. The United States is a distant third, at about 340 million.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("city"), py_str("cities"), py_str("town"), py_str("state"), py_str("states"), py_str("continent"), py_str("county"), py_str("counties"), py_str("in my"), py_str("school"), py_str("class"), py_str("office"), py_str("team"), py_str("street")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("river-length")); __args.push(py_tuple(vec![op_add(&g__RIVER_NAMES(), &py_tuple(vec![py_str("river"), py_str("river long"), py_str("length of the")])), py_tuple(vec![py_str("how long"), py_str("how far"), py_str("long is"), py_str("length"), py_str("km"), py_str("kilometres"), py_str("kilometers"), py_str("miles"), py_str("distance"), py_str("longest")])])); __args.push(g__river_length()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how long"), py_str("how far"), py_str("long is"), py_str("length"), py_str("km"), py_str("kilometres"), py_str("kilometers"), py_str("miles"), py_str("distance")])), ("avoid".to_string(), py_tuple(vec![py_str("cruise"), py_str("cruise ship"), py_str("boat"), py_str("kayak"), py_str("canoe"), py_str("paddle"), py_str("swim"), py_str("dive"), py_str("fish"), py_str("fishing"), py_str("holiday"), py_str("trip"), py_str("visit"), py_str("hotel"), py_str("walk"), py_str("hike"), py_str("bank"), py_str("banks"), py_str("flood"), py_str("flooding"), py_str("pollution"), py_str("sewage"), py_str("clean up")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("australia-both")); __args.push(py_tuple(vec![py_str("australia"), py_tuple(vec![py_str("continent"), py_str("country"), py_str("both"), py_str("continent or a country")])])); __args.push(py_str("Both: Australia is the smallest continent and the only country that is a whole continent. It is also the sixth largest country by area, and the only one that is both a country and a continent.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("continent"), py_str("country"), py_str("both")])), ("avoid".to_string(), py_tuple(vec![py_str("how many"), py_str("number of"), py_str("cities"), py_str("city"), py_str("capital"), py_str("visa"), py_str("holiday"), py_str("trip"), py_str("flight"), py_str("weather"), py_str("move"), py_str("living"), py_str("live"), py_str("visited"), py_str("been")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("euro-countries")); __args.push(py_tuple(vec![py_tuple(vec![py_str("euro"), py_str("euros"), py_str("eurozone"), py_str("single currency")]), py_tuple(vec![py_str("which countries"), py_str("what countries"), py_str("countries"), py_str("members"), py_str("use"), py_str("using"), py_str("adopt"), py_str("adopted"), py_str("currency")])])); __args.push(py_str("Twenty countries use the euro: Austria, Belgium, Croatia, Cyprus, Estonia, Finland, France, Germany, Greece, Ireland, Italy, Latvia, Lithuania, Luxembourg, Malta, the Netherlands, Portugal, Slovakia, Slovenia and Spain. Kosovo and Montenegro use it without being in the eurozone, and Denmark has a fixed rate but keeps the krone.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("euro"), py_str("euros"), py_str("eurozone"), py_str("single currency")])), ("avoid".to_string(), py_tuple(vec![py_str("take"), py_str("bring"), py_str("change"), py_str("exchange"), py_str("worth"), py_str("rate"), py_str("cash"), py_str("cashpoint"), py_str("atm"), py_str("card"), py_str("holiday"), py_str("spend"), py_str("spending"), py_str("budget"), py_str("cheaper")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("uk-parts")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ireland"), py_str("northern ireland"), py_str("england"), py_str("scotland"), py_str("wales"), py_str("british isles"), py_str("great britain"), py_str("britain")]), py_tuple(vec![py_str("part of europe"), py_str("in europe"), py_str("part of the uk"), py_str("part of uk"), py_str("in the uk"), py_str("in uk"), py_str("british"), py_str("uk"), py_str("britain"), py_str("independent"), py_str("separate country"), py_str("same country"), py_str("a country"), py_str("country"), py_str("sovereign"), py_str("difference"), py_str("different"), py_str("same"), py_str("versus"), py_str(" vs "), py_str("isles"), py_str("mean"), py_str("england"), py_str("scotland"), py_str("wales"), py_str("ireland"), py_str("northern ireland"), py_str("britain"), py_str("uk"), py_str("united kingdom"), py_str("great britain"), py_str("british isles")])])); __args.push(g__uk_part()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("part of"), py_str("in the uk"), py_str("in uk"), py_str("british"), py_str("independent"), py_str("separate"), py_str("in britain"), py_str("part of britain"), py_str("a country"), py_str("country"), py_str("sovereign"), py_str("difference"), py_str("different"), py_str("same"), py_str("versus"), py_str("isles"), py_str("british isles"), py_str("the same as"), py_str("mean")])), ("avoid".to_string(), py_tuple(vec![py_str("holiday"), py_str("trip"), py_str("visit"), py_str("moving"), py_str("move"), py_str("live"), py_str("living"), py_str("drive"), py_str("flight"), py_str("going"), py_str("hotel"), py_str("weather"), py_str("visa"), py_str("job"), py_str("food"), py_str("best"), py_str("what is there"), py_str("worth"), py_str("plug"), py_str("plugs"), py_str("socket"), py_str("sockets"), py_str("adapter"), py_str("adaptor"), py_str("voltage"), py_str("charger"), py_str("currency"), py_str("cash"), py_str("euro"), py_str("euros"), py_str("pound"), py_str("pounds"), py_str("weather"), py_str("climate"), py_str("nhs"), py_str("health"), py_str("hospital"), py_str("tax"), py_str("vat"), py_str("council tax"), py_str("wage"), py_str("wages"), py_str("salary"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("cheap"), py_str("expensive"), py_str("school"), py_str("schools"), py_str("education"), py_str("university"), py_str("exam"), py_str("exams"), py_str("age"), py_str("aged"), py_str("drinking"), py_str("alcohol"), py_str("law"), py_str("laws"), py_str("legal"), py_str("police"), py_str("crime"), py_str("passport"), py_str("licence"), py_str("license"), py_str("train"), py_str("trains"), py_str("bus"), py_str("buses"), py_str("rent"), py_str("mortgage"), py_str("house prices"), py_str("benefits"), py_str("pension"), py_str("pensions"), py_str("pint"), py_str("beer"), py_str("dog"), py_str("dogs"), py_str("pet"), py_str("pets")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _world(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut answer: Py = Py::None;
    let mut place: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A world record, unless the line asks about a named region: Africa's biggest country is\n    Algeria, not Russia, its highest mountain is Kilimanjaro, and the UK is four countries, not\n    195. Passing is better than the world answer to a region question.");
    place = stdlib_call("re", "compile", &[py_str("\\b(?:in|of|make up|makes up|making up)\\s+(?:the\\s+)?([a-z']+)")]).unwrap_or_else(|e| exc::raise(e));
    whole = py_tuple(vec![py_str("world"), py_str("earth"), py_str("planet"), py_str("all"), py_str("them"), py_str("these"), py_str("those")]);
    unit = py_tuple(vec![py_str("feet"), py_str("foot"), py_str("metres"), py_str("meters"), py_str("metre"), py_str("meter"), py_str("kilometres"), py_str("kilometers"), py_str("km"), py_str("miles"), py_str("mile"), py_str("centimetres"), py_str("centimeters"), py_str("cm"), py_str("inches"), py_str("inch"), py_str("total")]);
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut place: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut unit: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut whole: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut clean: Py = Py::None;
    let mut named: Py = Py::None;
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    named = mcall(&place.clone(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&whole.clone(), &mcall(&named.clone(), "group", &[Py::Int(1)]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&unit.clone(), &mcall(&named.clone(), "group", &[Py::Int(1)]))) } } } }) {
    return Py::None;
    }
    return reply.clone();
    Py::None
}
    answer = {{ let answer = answer.clone(); let place = place.clone(); let reply = reply.clone(); let unit = unit.clone(); let whole = whole.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), place.clone(), reply.clone(), unit.clone(), whole.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _feet(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut metres: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "metres") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: metres"))) };
    let _ = py_str("The same height in feet, so an answer works in either unit.");
    return call_builtin("int", &[call_builtin("round", &[op_truediv(&metres.clone(), &Py::Float(0.3048))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    return op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _peak_named(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The peak row the line names, the longest name first so \"table mountain\" wins over a word.");
    let __unpk1 = py_unpack(&py_tuple(vec![_said(&[understood.clone()]), Py::None, py_str("")]), 3, None);
    text = __unpk1[0].clone();
    found = __unpk1[1].clone();
    best = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_PEAKS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    name = __unpk3[0].clone();
    row = __unpk3[1].clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![row.clone(), name.clone()]), 2, None);
    found = __unpk4[0].clone();
    best = __unpk4[1].clone();
    }
    }
    return found.clone();
    Py::None
}
pub fn _peak(args: &[Py]) -> Py {
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
    let mut metres: Py = Py::None;
    let mut name: Py = Py::None;
    let mut note: Py = Py::None;
    let mut row: Py = Py::None;
    row = _peak_named(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&row.clone(), 3, None);
    name = __unpk1[0].clone();
    metres = __unpk1[1].clone();
    note = __unpk1[2].clone();
    return op_modulo(&py_str("%s is %s metres (%s feet) high: %s"), &py_tuple(vec![name.clone(), call_builtin("format", &[metres.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[_feet(&[metres.clone()]), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), op_add(&mcall(&note.clone(), "rstrip", &[py_str(".")]), &py_str("."))]));
    Py::None
}
pub fn _highest_in_place(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut text: Py = Py::None;
    text = _said(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_HIGHEST_IN()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    place = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[place.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&geti(&g_HIGHEST_IN(), &place.clone()), &py_str("."));
    }
    }
    return Py::None;
    Py::None
}
pub fn _continent_count(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut name: Py = Py::None;
    let mut text: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let _ = py_str("The continent's own total, unless the line asks about a subset of its countries.\n\n    Read the raw line, not the widened reading: after \"how many countries in africa\" the\n    next line \"how many countries have free healthcare\" carries africa in its context and\n    would otherwise get the Africa answer again. A trailing \"have\" is the natural\n    \"how many countries does africa have\"; a have/has/with before a noun is a subset.\n    ");
    text = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    trimmed = stdlib_call("re", "sub", &[py_str("\\s+(?:have|has)\\s*[?.!]*$"), py_str(""), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:have|has|with|without|allow|allows|legal|banned?|bans|speak|speaks|use|uses|need|needs)\\b"), trimmed.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__CONTINENT_COUNTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    count = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return count.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _bare_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut shorter: Py = Py::None;
    let _ = py_str("The place words after a direction, without the article in front of them.");
    phrase = mcall(&mcall(&{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "strip", &[py_str(".,!?;:")]);
    while py_truthy(&Py::Bool(true)) {
    shorter = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(phrase.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ARTICLE(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&shorter.clone(), &phrase.clone()))) {
    return phrase.clone();
    }
    phrase = shorter.clone();
    }
    Py::None
}
pub fn directed_place(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut place: Py = Py::None;
    let _ = py_str("The place a line carries something towards, in its own words, or None.\n\n    \"can i bring food into brazil\" -> \"brazil\"; \"what should i bring to a dinner party\" ->\n    \"dinner party\". The last word of a longer phrase is what decides, so \"into the north of\n    spain\" is read through \"north of spain\".\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g__DIRECTION(), "finditer", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    match_ = __item.clone();
    place = _bare_place(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = place.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_PLACE(), &place.clone())) } }) {
    return place.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _is_everyday(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut place: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut name: Py = Py::None;
    let _ = py_str("True when the place words name a room, a building or a do rather than a country.");
    place = _bare_place(&[place.clone()]);
    let __iter1 = py_iter_or_raise(&g__EVERYDAY_PLACE());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&place.clone(), &name.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&place.clone(), "startswith", &[op_add(&name.clone(), &py_str(" "))]); if py_truthy(&__b) { __b } else { mcall(&place.clone(), "endswith", &[op_add(&py_str(" "), &name.clone())]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _is_a_place(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let _ = py_str("The place the line travels towards, if it is one and not an everyday room or do.");
    place = directed_place(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _is_everyday(&[place.clone()]) } }) {
    return Py::None;
    }
    return place.clone();
    Py::None
}
pub fn crosses_a_border(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line carries something across a border rather than to a do.\n\n    `customs-food` speaks only for these lines and `bring-dish` stands down for them, so the two\n    rows can never read one line two ways.\n    ");
    line = mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&mcall(&g__CROSSING_WORD(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&_is_a_place(&[line.clone()]), &Py::None));
    Py::None
}
pub fn destination_place(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut place: Py = Py::None;
    let _ = py_str("The place a line travels into or to, in its own words, or None.\n\n    Only \"into\" and \"to\": \"back from france\" names where the food came from, which is the\n    question the Great Britain rule answers, so it is not a destination.\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g__DESTINATION(), "finditer", &[mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]));
    for __item in __iter1 {
    match_ = __item.clone();
    place = _bare_place(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = place.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_PLACE(), &place.clone())) } }) {
    return place.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn carries_it_out_of_britain(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let _ = py_str("True when the place travelled to is named and is not Britain.\n\n    The food rule in this knowledge base is the one for coming into Great Britain, so a line\n    about carrying food into Spain or China is not that row's question: it stands down and the\n    bot hedges rather than give one country's rule for another. A line with no place named\n    (\"can i bring cheese back from france\", \"can i bring ham home\") is coming here, and speaks.\n    ");
    place = destination_place(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _is_everyday(&[place.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&g__BRITAIN(), &place.clone()));
    Py::None
}
pub fn _ordinal_position(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut position: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Which place an ordinal names: 2 for \"second\" and \"2nd\", or None.");
    let __iter1 = py_iter_or_raise(&mcall(&g__ORDINAL_POS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    position = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return position.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _has(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut w: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _region_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut name: Py = Py::None;
    let mut region: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The region a line names, adjectives included, or None.\n\n    \"which african country is the largest\" names Africa with the adjective, so the plain names in\n    _PLACES are not enough on their own.\n    ");
    let __iter1 = py_iter_or_raise(&g__CONTINENT_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    region = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return region.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&g__PLACES());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _region_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut table: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { Py::None };
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    table = if py_truthy(&Py::Bool(!py_is(&table.clone(), &Py::None))) { table.clone() } else { g__REGION_COUNTRY() };
    name = _region_name(&[clean.clone()]);
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&table.clone(), &name.clone())) } }) {
    return geti(&table.clone(), &name.clone());
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&table.clone()) { let mut key: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__PLACES(), &key.clone()))) {  __out.push(key.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[key.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&table.clone(), &key.clone());
    }
    }
    return Py::None;
    Py::None
}
pub fn country_extreme(args: &[Py]) -> Py {
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
    let mut world_reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "world_reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: world_reply"))) };
    let mut answer: Py = Py::None;
    let _ = py_str("The region's own largest or smallest country, or the world record on an unqualified line.\n\n    kind is \"area\" for the biggest and \"small\" for the smallest; world_reply is the world answer\n    the entry already had, so nothing a person got for \"the biggest country\" changes.\n    ");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut kind: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut world_reply: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut clean: Py = Py::None;
    let mut look: Py = Py::None;
    let mut people: Py = Py::None;
    let mut position: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut row: Py = Py::None;
    let mut where_: Py = Py::None;
    clean = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    clean = mcall(&clean.clone(), "lower", &[]);
    row = _region_row(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&row.clone()))) {
    return world_reply.clone();
    }
    look = if py_truthy(&_has(&[clean.clone(), g__RECORD_SMALLEST()])) { py_str("small") } else { kind.clone() };
    ranked = mcall(&row.clone(), "get", &[if py_truthy(&Py::Bool(py_eq(&look.clone(), &py_str("small")))) { py_str("bottom") } else { py_str("top") }]);
    position = _ordinal_position(&[clean.clone()]);
    people = { let __b = mcall(&row.clone(), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&{ let __b = position.clone(); if !py_truthy(&__b) { __b } else { { let __b = _has(&[clean.clone(), g__RECORD_POPULOUS()]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[people.clone()]).unwrap_or_else(|e| exc::raise(e)), &position.clone()) } } } }) {
    where_ = mcall(&{ let __b = _region_name(&[clean.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }, "title", &[]);
    return op_modulo(&py_str("%s has the %s most people in %s: %s is first, then %s and %s."), &py_tuple(vec![geti(&people.clone(), &op_sub(&position.clone(), &Py::Int(1))), geti(&g__POS_WORD(), &position.clone()), where_.clone(), geti(&people.clone(), &Py::Int(0)), geti(&people.clone(), &Py::Int(1)), geti(&people.clone(), &Py::Int(2))]));
    }
    if py_truthy(&mcall(&g__ORDINAL(), "search", &[clean.clone()])) {
    return { let __b = ranked.clone(); if py_truthy(&__b) { __b } else { world_reply.clone() } };
    }
    if py_truthy(&mcall(&g__RECORD_LIST(), "search", &[clean.clone()])) {
    return { let __b = ranked.clone(); if py_truthy(&__b) { __b } else { world_reply.clone() } };
    }
    return { let __b = mcall(&row.clone(), "get", &[look.clone()]); if py_truthy(&__b) { __b } else { world_reply.clone() } };
    Py::None
}
    answer = {{ let answer = answer.clone(); let kind = kind.clone(); let world_reply = world_reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), kind.clone(), world_reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _continent_record(args: &[Py]) -> Py {
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
    let mut has: Py = Py::None;
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Africa's biggest country, Europe's smallest, Asia's most people: the record inside the\n    continent the line names, with the record that was asked for first.\n\n    Only a continent this file knows is answered, and Antarctica is answered with the fact that\n    it has no countries rather than passed, because that is the answer to the question.\n    ");
    text = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    row = _region_row(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&row.clone()))) {
    return Py::None;
    }
pub fn __nested_has_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut row: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "words").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut w: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    has = {{ let has = has.clone(); let memory = memory.clone(); let row = row.clone(); let text = text.clone(); let understood = understood.clone(); py_func("has", move |args: &[Py]| { let mut full = vec![has.clone(), memory.clone(), row.clone(), text.clone(), understood.clone()]; full.extend_from_slice(args); __nested_has_0(&full) }) }};
    if py_truthy(&{ let __b = py_call(&has, &[g__LEAST()]); if py_truthy(&__b) { __b } else { { let __b = py_call(&has, &[g__RECORD_SMALLEST()]); if !py_truthy(&__b) { __b } else { py_call(&has, &[g__RECORD_POPULOUS()]) } } } }) {
    return mcall(&row.clone(), "get", &[py_str("least")]);
    }
    if py_truthy(&py_call(&has, &[g__RECORD_POPULOUS()])) {
    return mcall(&row.clone(), "get", &[py_str("populous")]);
    }
    return Py::None;
    Py::None
}
pub fn region_extreme(args: &[Py]) -> Py {
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
    let mut world_reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "world_reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: world_reply"))) };
    let mut answer: Py = Py::None;
    let mut world: Py = Py::None;
    let _ = py_str("The region's own river, mountain or desert; the world answer only on an unqualified line.\n\n    The fallback is _world, so a named place this file has no fact for (a country, a subregion)\n    still passes rather than gets the world's answer.\n    ");
    world = _world(&[world_reply.clone()]);
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut kind: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut world: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut world_reply: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut clean: Py = Py::None;
    let mut row: Py = Py::None;
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    row = _region_row(&[clean.clone(), g__REGION_EXTREME()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return py_call(&world, &[memory.clone(), understood.clone()]);
    }
    return mcall(&row.clone(), "get", &[kind.clone()]);
    Py::None
}
    answer = {{ let answer = answer.clone(); let kind = kind.clone(); let world = world.clone(); let world_reply = world_reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), kind.clone(), world.clone(), world_reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _display_country(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A country key as it is written in a sentence: Czech Republic, the US, Ivory Coast.");
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("united states")))) {
    return py_str("the US");
    }
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("united kingdom")))) {
    return py_str("the UK");
    }
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("united arab emirates")))) {
    return py_str("the UAE");
    }
    if py_truthy(&Py::Bool(py_contains(&g__NOT_A_COUNTRY(), &name.clone()))) {
    return geti(&g__NOT_A_COUNTRY(), &name.clone());
    }
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&name.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("of"), py_str("the")]), &word.clone())); if !py_truthy(&__b) { __b } else { words.clone() } }) {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str("-")))) {
    let _ = mcall(&words.clone(), "append", &[mcall(&py_str("-"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&word.clone(), "split", &[py_str("-")])) { let mut part: Py = __item.clone(); __out.push(mcall(&part.clone(), "capitalize", &[])); } py_list(__out) })() }])]);
    } else {
    let _ = mcall(&words.clone(), "append", &[mcall(&word.clone(), "capitalize", &[])]);
    }
    }
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _the_country(args: &[Py]) -> Py {
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
    let mut shown: Py = Py::None;
    let _ = py_str("The name as it reads in a sentence: \"France\", \"the Netherlands\", \"the UK\".");
    shown = _display_country(&[name.clone()]);
    if py_truthy(&{ let __b = mcall(&shown.clone(), "startswith", &[py_str("the ")]); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_TAKES_THE(), &name.clone())) } }) {
    return shown.clone();
    }
    return op_add(&py_str("the "), &shown.clone());
    Py::None
}
pub fn _asked(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is a question at all, which is what these facts answer.\n\n    A country is named in a great many statements - \"my neighbour is from france\", \"the people\n    next door are from poland\", \"our neighbours in spain are friendly\" - and a border is not what\n    any of them are about. The understanding engine already reads a line as a question or a\n    statement, so these rows only answer the questions and pass on the rest.\n    ");
    return Py::Bool(py_eq(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str("question")));
    Py::None
}
pub fn _sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let _ = py_str("The reply with its first letter a capital, for the names that start with \"the\".");
    return if py_truthy(&answer.clone()) { op_add(&mcall(&slice_or_raise(&answer.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&answer.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { answer.clone() };
    Py::None
}
pub fn _countries_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut _u: Py = Py::None;
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut taken: Py = Py::None;
    let _ = py_str("The countries a line names, in the order it says them, as keys into COUNTRIES.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    found = __unpk1[0].clone();
    taken = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g__COUNTRY_PHRASES());
    for __item in __iter2 {
    phrase = __item.clone();
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    match_ = __item.clone();
    let __unpk4 = py_unpack(&mcall(&match_.clone(), "span", &[]), 2, None);
    start = __unpk4[0].clone();
    end = __unpk4[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&taken.clone()) { let __unpk6631 = py_unpack(&__item.clone(), 2, None); let mut before: Py = __unpk6631[0].clone(); let mut after: Py = __unpk6631[1].clone(); __out.push({ let __b = op_lt(&before.clone(), &end.clone()); if !py_truthy(&__b) { __b } else { op_lt(&start.clone(), &after.clone()) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    let _ = mcall(&taken.clone(), "append", &[py_tuple(vec![start.clone(), end.clone()])]);
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![start.clone(), mcall(&g__COUNTRY_ALIASES(), "get", &[phrase.clone(), phrase.clone()])])]);
    }
    }
    let _ = mcall(&found.clone(), "sort", &[]);
    said = py_list(vec![]);
    let __iter5 = py_iter_or_raise(&found.clone());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    _u = __unpk6[0].clone();
    name = __unpk6[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &name.clone()))) {
    let _ = mcall(&said.clone(), "append", &[name.clone()]);
    }
    }
    return said.clone();
    Py::None
}
pub fn _join_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut n: Py = Py::None;
    let _ = py_str("A list said the way a person says it: one, two and three.");
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut n: Py = __item.clone(); __out.push(_the_country(&[n.clone()])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&names.clone(), &Py::Int(0));
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&names.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&names.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _continent_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut best: Py = Py::None;
    let mut name: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("The continent a line names, as the name this file uses, or None.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&g__CONTINENTS(), "items", &[])); __kws.extend(vec![("key".to_string(), { let best = best.clone(); let clean = clean.clone(); let name = name.clone(); let phrase = phrase.clone(); py_func("lambda", move |args: &[Py]| { let best = best.clone(); let clean = clean.clone(); let name = name.clone(); let phrase = phrase.clone(); let row: Py = __kwp(args, 0, "row").unwrap_or(Py::None); call_builtin("len", &[geti(&row.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)) }) }), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk2[0].clone();
    name = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    best = name.clone();
    break;
    }
    }
    return best.clone();
    Py::None
}
pub fn _borders(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut first: Py = Py::None;
    let mut next_to: Py = Py::None;
    let mut note: Py = Py::None;
    let mut other: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Which countries touch the one the line names, and whether two of them touch each other.");
    if py_truthy(&Py::Bool(!py_truthy(&_asked(&[understood.clone()])))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    said = _countries_said(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    first = geti(&said.clone(), &Py::Int(0));
    row = mcall(&g_COUNTRIES(), "get", &[first.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&row.clone(), 3, None);
    _u = __unpk1[0].clone();
    next_to = __unpk1[1].clone();
    note = __unpk1[2].clone();
    if py_truthy(&op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    other = geti(&said.clone(), &Py::Int(1));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&next_to.clone(), &other.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&geti(&{ let __b = mcall(&g_COUNTRIES(), "get", &[other.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![py_str(""), py_tuple(vec![])]) } }, &Py::Int(1)), &first.clone())) } }) {
    return _sentence(&[op_modulo(&py_str("Yes - %s and %s share a border."), &py_tuple(vec![_the_country(&[first.clone()]), _display_country(&[other.clone()])]))]);
    }
    return _sentence(&[op_modulo(&py_str("No - %s and %s do not share a land border."), &py_tuple(vec![_the_country(&[first.clone()]), _display_country(&[other.clone()])]))]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&next_to.clone()))) {
    return _sentence(&[op_modulo(&py_str("%s has no land neighbours.%s"), &py_tuple(vec![_the_country(&[first.clone()]), if py_truthy(&note.clone()) { op_add(&py_str(" "), &note.clone()) } else { py_str("") }]))]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow many\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return _sentence(&[op_modulo(&py_str("%s has %d land neighbours: %s."), &py_tuple(vec![_the_country(&[first.clone()]), call_builtin("len", &[next_to.clone()]).unwrap_or_else(|e| exc::raise(e)), _join_names(&[next_to.clone()])]))]);
    }
    answer = op_modulo(&py_str("%s borders %s."), &py_tuple(vec![_the_country(&[first.clone()]), _join_names(&[next_to.clone()])]));
    return _sentence(&[op_add(&answer.clone(), &if py_truthy(&note.clone()) { op_add(&py_str(" "), &note.clone()) } else { py_str("") })]);
    Py::None
}
pub fn _continent(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut note: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The continent a named country is in, and whether it is in the one the line names.\n\n    \"what continent is egypt in\" and \"is egypt in africa\" are the same fact said two ways, so the\n    same row answers both: the second one starts with yes or no and says where it is instead.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_asked(&[understood.clone()])))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    said = _countries_said(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    row = mcall(&g_COUNTRIES(), "get", &[geti(&said.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&row.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&row.clone(), &Py::Int(0)))) } }) {
    return Py::None;
    }
    where_ = geti(&row.clone(), &Py::Int(0));
    answer = op_modulo(&py_str("%s is in %s."), &py_tuple(vec![_the_country(&[geti(&said.clone(), &Py::Int(0))]), where_.clone()]));
    asked = _continent_said(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&asked.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&mcall(&where_.clone(), "lower", &[]), &mcall(&asked.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&asked.clone(), &where_.clone())) } } } }) {
    answer = op_add(&py_str("Yes - "), &answer.clone());
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&asked.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&asked.clone(), &where_.clone())) } }) {
    answer = op_add(&py_str("No - "), &answer.clone());
    }
    }
    note = mcall(&g__CONTINENT_NOTES_BY_COUNTRY(), "get", &[geti(&said.clone(), &Py::Int(0)), py_str("")]);
    return _sentence(&[op_add(&answer.clone(), &if py_truthy(&note.clone()) { op_add(&py_str(" "), &note.clone()) } else { py_str("") })]);
    Py::None
}
pub fn _continent_countries(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut members: Py = Py::None;
    let mut name: Py = Py::None;
    let mut region: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("The countries usually counted in a continent, from the same table the borders come from.");
    if py_truthy(&Py::Bool(!py_truthy(&_asked(&[understood.clone()])))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    region = _continent_said(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&region.clone(), &Py::None))) {
    return Py::None;
    }
    members = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_COUNTRIES(), "items", &[])) { let __unpk7040 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk7040[0].clone(); let mut row: Py = __unpk7040[1].clone(); if py_truthy(&Py::Bool(py_eq(&geti(&row.clone(), &Py::Int(0)), &region.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&members.clone()))) {
    return Py::None;
    }
    return op_modulo(&py_str("The countries usually counted in %s: %s. %s"), &py_tuple(vec![region.clone(), _join_names(&[members.clone()]), mcall(&g__CONTINENT_NOTES(), "get", &[region.clone(), py_str("")])]));
    Py::None
}
pub fn _in_eu(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut country: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a named country is in the EU, or which countries are.");
    if py_truthy(&Py::Bool(!py_truthy(&_asked(&[understood.clone()])))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    said = _countries_said(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("Twenty-seven countries are in the EU: Austria, Belgium, Bulgaria, Croatia, Cyprus, Czech Republic, Denmark, Estonia, Finland, France, Germany, Greece, Hungary, Ireland, Italy, Latvia, Lithuania, Luxembourg, Malta, the Netherlands, Poland, Portugal, Romania, Slovakia, Slovenia, Spain and Sweden.");
    }
    country = geti(&said.clone(), &Py::Int(0));
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(leave|leaves|leaving|left|quit|quitting|brexit)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&Py::Bool(py_eq(&country.clone(), &py_str("united kingdom")))) {
    return py_str("Yes - the UK left the EU on 31 January 2020, after the Brexit vote in 2016. It has been outside the single market and customs union since then.");
    }
    if py_truthy(&Py::Bool(py_contains(&g_EU_MEMBERS(), &country.clone()))) {
    return _sentence(&[op_modulo(&py_str("No - %s is still in the EU; it is one of the 27 members."), &_the_country(&[country.clone()]))]);
    }
    return _sentence(&[op_modulo(&py_str("No - %s was never in the EU, so there was nothing to leave."), &_the_country(&[country.clone()]))]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_EU_MEMBERS(), &country.clone()))) {
    return _sentence(&[op_modulo(&py_str("Yes - %s is one of the 27 countries in the EU."), &_the_country(&[country.clone()]))]);
    }
    reason = mcall(&g__NOT_IN_EU(), "get", &[country.clone()]);
    if py_truthy(&reason.clone()) {
    return _sentence(&[op_modulo(&py_str("No - %s is not in the EU: %s"), &py_tuple(vec![_the_country(&[country.clone()]), reason.clone()]))]);
    }
    return _sentence(&[op_modulo(&py_str("No - %s is not in the EU."), &_the_country(&[country.clone()]))]);
    Py::None
}
pub fn _the_nations_explained(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let _ = py_str("The one explanation every comparison of these names comes to.");
    return py_str("Great Britain is the island: England, Scotland and Wales. The UK is the country, which is that island plus Northern Ireland. So England is in Britain and in the UK, Northern Ireland is in the UK but not in Britain, and the British Isles is only geography - both islands, including the Republic of Ireland, which is a separate country.");
    Py::None
}
pub fn _the_pair_a_part_of_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut one: Py = Py::None;
    let mut split: Py = Py::None;
    let mut two: Py = Py::None;
    let _ = py_str("The answer for \"is X part of Y\" when both are named, or None when the UK is the answer.");
    split = stdlib_call("re", "search", &[py_str("\\bpart of\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&split.clone()))) {
    return Py::None;
    }
    before = mcall(&g__NATION_NAMED(), "findall", &[slice_or_raise(&clean.clone(), &Py::None, &mcall(&split.clone(), "start", &[]), &Py::None)]);
    after = mcall(&g__NATION_NAMED(), "findall", &[slice_or_raise(&clean.clone(), &mcall(&split.clone(), "end", &[]), &Py::None, &Py::None)]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&after.clone())) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&before.clone(), &op_neg(&Py::Int(1))), geti(&after.clone(), &Py::Int(0))]), 2, None);
    one = __unpk1[0].clone();
    two = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &two.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("uk"), py_str("united kingdom")]), &two.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&two.clone(), &py_str("europe")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__THE_FOUR(), &one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("britain"), py_str("great britain"), py_str("uk"), py_str("united kingdom")]), &one.clone())) } }) {
    return op_modulo(&py_str("Yes - %s is in Europe. Every part of the UK is: the UK left the European Union in 2020, but it never left the continent."), &geti(&g__NATION_SAID(), &one.clone()));
    }
    return py_str("Yes - Ireland is in Europe, and in the European Union. It is a separate country from the UK: only Northern Ireland is part of the UK.");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__THE_FOUR(), &one.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__THE_FOUR(), &two.clone())) } }) {
    return op_modulo(&py_str("No - %s is not part of %s. They are both part of the UK, which is England, Scotland, Wales and Northern Ireland."), &py_tuple(vec![geti(&g__NATION_SAID(), &one.clone()), geti(&g__NATION_SAID(), &two.clone())]));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("britain"), py_str("great britain")]), &two.clone()))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("northern ireland"), py_str("ireland")]), &one.clone()))) {
    return op_modulo(&py_str("No - %s is not in Great Britain: Great Britain is England, Scotland and Wales.%s"), &py_tuple(vec![geti(&g__NATION_SAID(), &one.clone()), if py_truthy(&Py::Bool(py_eq(&one.clone(), &py_str("northern ireland")))) { py_str(" Northern Ireland is in the UK all the same, on the island of Ireland.") } else { py_str(" The Republic of Ireland is a separate country.") }]));
    }
    return op_modulo(&py_str("Yes - %s is part of Great Britain, the island that also has %s on it. Great Britain and Northern Ireland together are the UK."), &py_tuple(vec![geti(&g__NATION_SAID(), &one.clone()), if py_truthy(&Py::Bool(py_eq(&one.clone(), &py_str("scotland")))) { py_str("England and Wales") } else { if py_truthy(&Py::Bool(py_eq(&one.clone(), &py_str("england")))) { py_str("Scotland and Wales") } else { py_str("England and Scotland") } }]));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("ireland")]), &two.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__THE_FOUR(), &one.clone())) } }) {
    return op_modulo(&py_str("No - %s is not part of Ireland. Ireland is a separate country; %s"), &py_tuple(vec![geti(&g__NATION_SAID(), &one.clone()), if py_truthy(&Py::Bool(py_eq(&one.clone(), &py_str("northern ireland")))) { py_str("Northern Ireland is the part of the UK on the same island.") } else { py_str("it shares its island with Northern Ireland, which is part of the UK.") }]));
    }
    return Py::None;
    Py::None
}
pub fn _uk_part(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut comparison: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut part: Py = Py::None;
    let mut says: Py = Py::None;
    let _ = py_str("Whether a part of the UK is in the UK, and whether Ireland is one of them.");
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
pub fn __nested_says_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut comparison: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut pair: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut phrase: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "phrase").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return Py::Bool(!py_is(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    Py::None
}
    says = {{ let clean = clean.clone(); let comparison = comparison.clone(); let memory = memory.clone(); let name = name.clone(); let pair = pair.clone(); let part = part.clone(); let says = says.clone(); let understood = understood.clone(); py_func("says", move |args: &[Py]| { let mut full = vec![clean.clone(), comparison.clone(), memory.clone(), name.clone(), pair.clone(), part.clone(), says.clone(), understood.clone()]; full.extend_from_slice(args); __nested_says_0(&full) }) }};
    comparison = { let __b = py_call(&says, &[py_str("british isles")]); if py_truthy(&__b) { __b } else { { let __b = py_call(&says, &[py_str("difference")]); if py_truthy(&__b) { __b } else { { let __b = py_call(&says, &[py_str("same")]); if py_truthy(&__b) { __b } else { { let __b = py_call(&says, &[py_str("versus")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str(" vs ")]) } } } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_asked(&[understood.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&comparison.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&comparison.clone()) {
    return _the_nations_explained(&[]);
    }
    pair = _the_pair_a_part_of_names(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&pair.clone(), &Py::None))) {
    return pair.clone();
    }
    if py_truthy(&py_call(&says, &[py_str("northern ireland")])) {
    part = py_str("Northern Ireland");
    } else {
    if py_truthy(&py_call(&says, &[py_str("ireland")])) {
    return py_str("No - Ireland is not part of the UK. The Republic of Ireland is a separate country; Northern Ireland is the part of the UK on the same island. Great Britain is England, Scotland and Wales.");
    } else {
    part = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("england"), py_str("scotland"), py_str("wales")])) { let mut name: Py = __item.clone(); if py_truthy(&py_call(&says, &[name.clone()])) {  __out.push(mcall(&name.clone(), "title", &[])); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    if py_truthy(&Py::Bool(py_is(&part.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = py_call(&says, &[py_str("a country")]); if py_truthy(&__b) { __b } else { { let __b = py_call(&says, &[py_str("independent country")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("sovereign")]) } } } }) {
    if py_truthy(&Py::Bool(py_eq(&part.clone(), &py_str("Northern Ireland")))) {
    return py_str("Northern Ireland is called a province or a region rather than a country, but it is part of the UK all the same: the UK is England, Scotland, Wales and Northern Ireland.");
    }
    return op_modulo(&py_str("Yes - %s is a country, one of the four in the UK. It is not a sovereign state on its own: the UK is the country that sits at the UN and signs treaties."), &part.clone());
    }
    if py_truthy(&{ let __b = py_call(&says, &[py_str("independent")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("separate country")]) } }) {
    return op_modulo(&py_str("No - %s is not a separate country: it is one of the four parts of the UK, along with England, Scotland, Wales and Northern Ireland. The UK is the country."), &part.clone());
    }
    return op_modulo(&py_str("Yes - %s is part of the UK. The UK is England, Scotland, Wales and Northern Ireland; Great Britain is the island with the first three on it."), &part.clone());
    Py::None
}
pub fn _river_length(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut name: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The length of the river the line names, or None when it names none of them.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__RIVER_LENGTH_ASK(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__RIVER_LENGTHS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    text = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__RIVER_NAME_NEEDS_RIVER(), &name.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\briver\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    return text.clone();
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.geography".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "COUNTRIES" => Some(g_COUNTRIES()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EU_MEMBERS" => Some(g_EU_MEMBERS()),
            "HIGHEST_IN" => Some(g_HIGHEST_IN()),
            "PEAKS" => Some(g_PEAKS()),
            "TAKES_THE" => Some(g_TAKES_THE()),
            "_ARTICLE" => Some(g__ARTICLE()),
            "_BRITAIN" => Some(g__BRITAIN()),
            "_COMPARES_NATIONS" => Some(g__COMPARES_NATIONS()),
            "_CONTINENTS" => Some(g__CONTINENTS()),
            "_CONTINENT_COUNTS" => Some(g__CONTINENT_COUNTS()),
            "_CONTINENT_NOTES" => Some(g__CONTINENT_NOTES()),
            "_CONTINENT_NOTES_BY_COUNTRY" => Some(g__CONTINENT_NOTES_BY_COUNTRY()),
            "_CONTINENT_WORDS" => Some(g__CONTINENT_WORDS()),
            "_COUNTRY_ALIASES" => Some(g__COUNTRY_ALIASES()),
            "_COUNTRY_PHRASES" => Some(g__COUNTRY_PHRASES()),
            "_COUNTRY_WORDS" => Some(g__COUNTRY_WORDS()),
            "_CROSSING_WORD" => Some(g__CROSSING_WORD()),
            "_DESTINATION" => Some(g__DESTINATION()),
            "_DIRECTION" => Some(g__DIRECTION()),
            "_EVERYDAY_PLACE" => Some(g__EVERYDAY_PLACE()),
            "_LEAST" => Some(g__LEAST()),
            "_LIST_COUNT" => Some(g__LIST_COUNT()),
            "_NATION_NAMED" => Some(g__NATION_NAMED()),
            "_NATION_SAID" => Some(g__NATION_SAID()),
            "_NOT_A_COUNTRY" => Some(g__NOT_A_COUNTRY()),
            "_NOT_A_PLACE" => Some(g__NOT_A_PLACE()),
            "_NOT_IN_EU" => Some(g__NOT_IN_EU()),
            "_ORDINAL" => Some(g__ORDINAL()),
            "_ORDINAL_POS" => Some(g__ORDINAL_POS()),
            "_PLACES" => Some(g__PLACES()),
            "_POS_WORD" => Some(g__POS_WORD()),
            "_RECORD_BIGGEST" => Some(g__RECORD_BIGGEST()),
            "_RECORD_LIST" => Some(g__RECORD_LIST()),
            "_RECORD_POPULOUS" => Some(g__RECORD_POPULOUS()),
            "_RECORD_SMALLEST" => Some(g__RECORD_SMALLEST()),
            "_REGIONS" => Some(g__REGIONS()),
            "_REGION_COUNTRY" => Some(g__REGION_COUNTRY()),
            "_REGION_EXTREME" => Some(g__REGION_EXTREME()),
            "_REGION_EXTREME_WORDS" => Some(g__REGION_EXTREME_WORDS()),
            "_RIVER_LENGTHS" => Some(g__RIVER_LENGTHS()),
            "_RIVER_LENGTH_ASK" => Some(g__RIVER_LENGTH_ASK()),
            "_RIVER_NAMES" => Some(g__RIVER_NAMES()),
            "_RIVER_NAME_NEEDS_RIVER" => Some(g__RIVER_NAME_NEEDS_RIVER()),
            "_THE_FOUR" => Some(g__THE_FOUR()),
            "_asked" => Some(py_func("_asked", |args: &[Py]| _asked(args))),
            "_bare_place" => Some(py_func("_bare_place", |args: &[Py]| _bare_place(args))),
            "_borders" => Some(py_func("_borders", |args: &[Py]| _borders(args))),
            "_continent" => Some(py_func("_continent", |args: &[Py]| _continent(args))),
            "_continent_count" => Some(py_func("_continent_count", |args: &[Py]| _continent_count(args))),
            "_continent_countries" => Some(py_func("_continent_countries", |args: &[Py]| _continent_countries(args))),
            "_continent_record" => Some(py_func("_continent_record", |args: &[Py]| _continent_record(args))),
            "_continent_said" => Some(py_func("_continent_said", |args: &[Py]| _continent_said(args))),
            "_countries_said" => Some(py_func("_countries_said", |args: &[Py]| _countries_said(args))),
            "_display_country" => Some(py_func("_display_country", |args: &[Py]| _display_country(args))),
            "_feet" => Some(py_func("_feet", |args: &[Py]| _feet(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "_highest_in_place" => Some(py_func("_highest_in_place", |args: &[Py]| _highest_in_place(args))),
            "_in_eu" => Some(py_func("_in_eu", |args: &[Py]| _in_eu(args))),
            "_is_a_place" => Some(py_func("_is_a_place", |args: &[Py]| _is_a_place(args))),
            "_is_everyday" => Some(py_func("_is_everyday", |args: &[Py]| _is_everyday(args))),
            "_join_names" => Some(py_func("_join_names", |args: &[Py]| _join_names(args))),
            "_ordinal_position" => Some(py_func("_ordinal_position", |args: &[Py]| _ordinal_position(args))),
            "_peak" => Some(py_func("_peak", |args: &[Py]| _peak(args))),
            "_peak_named" => Some(py_func("_peak_named", |args: &[Py]| _peak_named(args))),
            "_region_name" => Some(py_func("_region_name", |args: &[Py]| _region_name(args))),
            "_region_row" => Some(py_func("_region_row", |args: &[Py]| _region_row(args))),
            "_river_length" => Some(py_func("_river_length", |args: &[Py]| _river_length(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_sentence" => Some(py_func("_sentence", |args: &[Py]| _sentence(args))),
            "_the_country" => Some(py_func("_the_country", |args: &[Py]| _the_country(args))),
            "_the_nations_explained" => Some(py_func("_the_nations_explained", |args: &[Py]| _the_nations_explained(args))),
            "_the_pair_a_part_of_names" => Some(py_func("_the_pair_a_part_of_names", |args: &[Py]| _the_pair_a_part_of_names(args))),
            "_uk_part" => Some(py_func("_uk_part", |args: &[Py]| _uk_part(args))),
            "_world" => Some(py_func("_world", |args: &[Py]| _world(args))),
            "carries_it_out_of_britain" => Some(py_func("carries_it_out_of_britain", |args: &[Py]| carries_it_out_of_britain(args))),
            "country_extreme" => Some(py_func("country_extreme", |args: &[Py]| country_extreme(args))),
            "crosses_a_border" => Some(py_func("crosses_a_border", |args: &[Py]| crosses_a_border(args))),
            "destination_place" => Some(py_func("destination_place", |args: &[Py]| destination_place(args))),
            "directed_place" => Some(py_func("directed_place", |args: &[Py]| directed_place(args))),
            "region_extreme" => Some(py_func("region_extreme", |args: &[Py]| region_extreme(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "COUNTRIES" => s_COUNTRIES(v),
            "ENTRIES" => s_ENTRIES(v),
            "EU_MEMBERS" => s_EU_MEMBERS(v),
            "HIGHEST_IN" => s_HIGHEST_IN(v),
            "PEAKS" => s_PEAKS(v),
            "TAKES_THE" => s_TAKES_THE(v),
            "_ARTICLE" => s__ARTICLE(v),
            "_BRITAIN" => s__BRITAIN(v),
            "_COMPARES_NATIONS" => s__COMPARES_NATIONS(v),
            "_CONTINENTS" => s__CONTINENTS(v),
            "_CONTINENT_COUNTS" => s__CONTINENT_COUNTS(v),
            "_CONTINENT_NOTES" => s__CONTINENT_NOTES(v),
            "_CONTINENT_NOTES_BY_COUNTRY" => s__CONTINENT_NOTES_BY_COUNTRY(v),
            "_CONTINENT_WORDS" => s__CONTINENT_WORDS(v),
            "_COUNTRY_ALIASES" => s__COUNTRY_ALIASES(v),
            "_COUNTRY_PHRASES" => s__COUNTRY_PHRASES(v),
            "_COUNTRY_WORDS" => s__COUNTRY_WORDS(v),
            "_CROSSING_WORD" => s__CROSSING_WORD(v),
            "_DESTINATION" => s__DESTINATION(v),
            "_DIRECTION" => s__DIRECTION(v),
            "_EVERYDAY_PLACE" => s__EVERYDAY_PLACE(v),
            "_LEAST" => s__LEAST(v),
            "_LIST_COUNT" => s__LIST_COUNT(v),
            "_NATION_NAMED" => s__NATION_NAMED(v),
            "_NATION_SAID" => s__NATION_SAID(v),
            "_NOT_A_COUNTRY" => s__NOT_A_COUNTRY(v),
            "_NOT_A_PLACE" => s__NOT_A_PLACE(v),
            "_NOT_IN_EU" => s__NOT_IN_EU(v),
            "_ORDINAL" => s__ORDINAL(v),
            "_ORDINAL_POS" => s__ORDINAL_POS(v),
            "_PLACES" => s__PLACES(v),
            "_POS_WORD" => s__POS_WORD(v),
            "_RECORD_BIGGEST" => s__RECORD_BIGGEST(v),
            "_RECORD_LIST" => s__RECORD_LIST(v),
            "_RECORD_POPULOUS" => s__RECORD_POPULOUS(v),
            "_RECORD_SMALLEST" => s__RECORD_SMALLEST(v),
            "_REGIONS" => s__REGIONS(v),
            "_REGION_COUNTRY" => s__REGION_COUNTRY(v),
            "_REGION_EXTREME" => s__REGION_EXTREME(v),
            "_REGION_EXTREME_WORDS" => s__REGION_EXTREME_WORDS(v),
            "_RIVER_LENGTHS" => s__RIVER_LENGTHS(v),
            "_RIVER_LENGTH_ASK" => s__RIVER_LENGTH_ASK(v),
            "_RIVER_NAMES" => s__RIVER_NAMES(v),
            "_RIVER_NAME_NEEDS_RIVER" => s__RIVER_NAME_NEEDS_RIVER(v),
            "_THE_FOUR" => s__THE_FOUR(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.geography", module_obj);
    pyrt::xrefs::reg_fn("knowledge.geography", "_world", py_func("_world", |args: &[Py]| _world(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_feet", py_func("_feet", |args: &[Py]| _feet(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_peak_named", py_func("_peak_named", |args: &[Py]| _peak_named(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_peak", py_func("_peak", |args: &[Py]| _peak(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_highest_in_place", py_func("_highest_in_place", |args: &[Py]| _highest_in_place(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_continent_count", py_func("_continent_count", |args: &[Py]| _continent_count(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_bare_place", py_func("_bare_place", |args: &[Py]| _bare_place(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "directed_place", py_func("directed_place", |args: &[Py]| directed_place(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_is_everyday", py_func("_is_everyday", |args: &[Py]| _is_everyday(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_is_a_place", py_func("_is_a_place", |args: &[Py]| _is_a_place(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "crosses_a_border", py_func("crosses_a_border", |args: &[Py]| crosses_a_border(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "destination_place", py_func("destination_place", |args: &[Py]| destination_place(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "carries_it_out_of_britain", py_func("carries_it_out_of_britain", |args: &[Py]| carries_it_out_of_britain(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_ordinal_position", py_func("_ordinal_position", |args: &[Py]| _ordinal_position(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_region_name", py_func("_region_name", |args: &[Py]| _region_name(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_region_row", py_func("_region_row", |args: &[Py]| _region_row(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "country_extreme", py_func("country_extreme", |args: &[Py]| country_extreme(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_continent_record", py_func("_continent_record", |args: &[Py]| _continent_record(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "region_extreme", py_func("region_extreme", |args: &[Py]| region_extreme(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_display_country", py_func("_display_country", |args: &[Py]| _display_country(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_the_country", py_func("_the_country", |args: &[Py]| _the_country(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_asked", py_func("_asked", |args: &[Py]| _asked(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_sentence", py_func("_sentence", |args: &[Py]| _sentence(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_countries_said", py_func("_countries_said", |args: &[Py]| _countries_said(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_join_names", py_func("_join_names", |args: &[Py]| _join_names(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_continent_said", py_func("_continent_said", |args: &[Py]| _continent_said(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_borders", py_func("_borders", |args: &[Py]| _borders(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_continent", py_func("_continent", |args: &[Py]| _continent(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_continent_countries", py_func("_continent_countries", |args: &[Py]| _continent_countries(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_in_eu", py_func("_in_eu", |args: &[Py]| _in_eu(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_the_nations_explained", py_func("_the_nations_explained", |args: &[Py]| _the_nations_explained(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_the_pair_a_part_of_names", py_func("_the_pair_a_part_of_names", |args: &[Py]| _the_pair_a_part_of_names(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_uk_part", py_func("_uk_part", |args: &[Py]| _uk_part(args)));
    pyrt::xrefs::reg_fn("knowledge.geography", "_river_length", py_func("_river_length", |args: &[Py]| _river_length(args)));
    pyrt::xrefs::reg_access("knowledge.geography", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.geography", "COUNTRIES", g_COUNTRIES, s_COUNTRIES);
    pyrt::xrefs::reg_access("knowledge.geography", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.geography", "EU_MEMBERS", g_EU_MEMBERS, s_EU_MEMBERS);
    pyrt::xrefs::reg_access("knowledge.geography", "HIGHEST_IN", g_HIGHEST_IN, s_HIGHEST_IN);
    pyrt::xrefs::reg_access("knowledge.geography", "PEAKS", g_PEAKS, s_PEAKS);
    pyrt::xrefs::reg_access("knowledge.geography", "TAKES_THE", g_TAKES_THE, s_TAKES_THE);
    pyrt::xrefs::reg_access("knowledge.geography", "_ARTICLE", g__ARTICLE, s__ARTICLE);
    pyrt::xrefs::reg_access("knowledge.geography", "_BRITAIN", g__BRITAIN, s__BRITAIN);
    pyrt::xrefs::reg_access("knowledge.geography", "_COMPARES_NATIONS", g__COMPARES_NATIONS, s__COMPARES_NATIONS);
    pyrt::xrefs::reg_access("knowledge.geography", "_CONTINENTS", g__CONTINENTS, s__CONTINENTS);
    pyrt::xrefs::reg_access("knowledge.geography", "_CONTINENT_COUNTS", g__CONTINENT_COUNTS, s__CONTINENT_COUNTS);
    pyrt::xrefs::reg_access("knowledge.geography", "_CONTINENT_NOTES", g__CONTINENT_NOTES, s__CONTINENT_NOTES);
    pyrt::xrefs::reg_access("knowledge.geography", "_CONTINENT_NOTES_BY_COUNTRY", g__CONTINENT_NOTES_BY_COUNTRY, s__CONTINENT_NOTES_BY_COUNTRY);
    pyrt::xrefs::reg_access("knowledge.geography", "_CONTINENT_WORDS", g__CONTINENT_WORDS, s__CONTINENT_WORDS);
    pyrt::xrefs::reg_access("knowledge.geography", "_COUNTRY_ALIASES", g__COUNTRY_ALIASES, s__COUNTRY_ALIASES);
    pyrt::xrefs::reg_access("knowledge.geography", "_COUNTRY_PHRASES", g__COUNTRY_PHRASES, s__COUNTRY_PHRASES);
    pyrt::xrefs::reg_access("knowledge.geography", "_COUNTRY_WORDS", g__COUNTRY_WORDS, s__COUNTRY_WORDS);
    pyrt::xrefs::reg_access("knowledge.geography", "_CROSSING_WORD", g__CROSSING_WORD, s__CROSSING_WORD);
    pyrt::xrefs::reg_access("knowledge.geography", "_DESTINATION", g__DESTINATION, s__DESTINATION);
    pyrt::xrefs::reg_access("knowledge.geography", "_DIRECTION", g__DIRECTION, s__DIRECTION);
    pyrt::xrefs::reg_access("knowledge.geography", "_EVERYDAY_PLACE", g__EVERYDAY_PLACE, s__EVERYDAY_PLACE);
    pyrt::xrefs::reg_access("knowledge.geography", "_LEAST", g__LEAST, s__LEAST);
    pyrt::xrefs::reg_access("knowledge.geography", "_LIST_COUNT", g__LIST_COUNT, s__LIST_COUNT);
    pyrt::xrefs::reg_access("knowledge.geography", "_NATION_NAMED", g__NATION_NAMED, s__NATION_NAMED);
    pyrt::xrefs::reg_access("knowledge.geography", "_NATION_SAID", g__NATION_SAID, s__NATION_SAID);
    pyrt::xrefs::reg_access("knowledge.geography", "_NOT_A_COUNTRY", g__NOT_A_COUNTRY, s__NOT_A_COUNTRY);
    pyrt::xrefs::reg_access("knowledge.geography", "_NOT_A_PLACE", g__NOT_A_PLACE, s__NOT_A_PLACE);
    pyrt::xrefs::reg_access("knowledge.geography", "_NOT_IN_EU", g__NOT_IN_EU, s__NOT_IN_EU);
    pyrt::xrefs::reg_access("knowledge.geography", "_ORDINAL", g__ORDINAL, s__ORDINAL);
    pyrt::xrefs::reg_access("knowledge.geography", "_ORDINAL_POS", g__ORDINAL_POS, s__ORDINAL_POS);
    pyrt::xrefs::reg_access("knowledge.geography", "_PLACES", g__PLACES, s__PLACES);
    pyrt::xrefs::reg_access("knowledge.geography", "_POS_WORD", g__POS_WORD, s__POS_WORD);
    pyrt::xrefs::reg_access("knowledge.geography", "_RECORD_BIGGEST", g__RECORD_BIGGEST, s__RECORD_BIGGEST);
    pyrt::xrefs::reg_access("knowledge.geography", "_RECORD_LIST", g__RECORD_LIST, s__RECORD_LIST);
    pyrt::xrefs::reg_access("knowledge.geography", "_RECORD_POPULOUS", g__RECORD_POPULOUS, s__RECORD_POPULOUS);
    pyrt::xrefs::reg_access("knowledge.geography", "_RECORD_SMALLEST", g__RECORD_SMALLEST, s__RECORD_SMALLEST);
    pyrt::xrefs::reg_access("knowledge.geography", "_REGIONS", g__REGIONS, s__REGIONS);
    pyrt::xrefs::reg_access("knowledge.geography", "_REGION_COUNTRY", g__REGION_COUNTRY, s__REGION_COUNTRY);
    pyrt::xrefs::reg_access("knowledge.geography", "_REGION_EXTREME", g__REGION_EXTREME, s__REGION_EXTREME);
    pyrt::xrefs::reg_access("knowledge.geography", "_REGION_EXTREME_WORDS", g__REGION_EXTREME_WORDS, s__REGION_EXTREME_WORDS);
    pyrt::xrefs::reg_access("knowledge.geography", "_RIVER_LENGTHS", g__RIVER_LENGTHS, s__RIVER_LENGTHS);
    pyrt::xrefs::reg_access("knowledge.geography", "_RIVER_LENGTH_ASK", g__RIVER_LENGTH_ASK, s__RIVER_LENGTH_ASK);
    pyrt::xrefs::reg_access("knowledge.geography", "_RIVER_NAMES", g__RIVER_NAMES, s__RIVER_NAMES);
    pyrt::xrefs::reg_access("knowledge.geography", "_RIVER_NAME_NEEDS_RIVER", g__RIVER_NAME_NEEDS_RIVER, s__RIVER_NAME_NEEDS_RIVER);
    pyrt::xrefs::reg_access("knowledge.geography", "_THE_FOUR", g__THE_FOUR, s__THE_FOUR);
}
