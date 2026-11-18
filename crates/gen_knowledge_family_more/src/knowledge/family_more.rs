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


thread_local! { static G_EASTER_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUMMER_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FIRST_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_QUESTIONED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_STOPPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WINTER_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ADULT_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ADULT_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_SPELLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NAPPY_BAG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NURSING_CAREER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAST_COULD_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAST_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAST_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAST_PAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAST_PUBLIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROWN_UP_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEFT_THE_COUNTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAPPY_AGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NATION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TERM_STARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_DATES_ARE_LOCAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRAVEL_HOLIDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_phone_rule_and_an_age: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_teenager_is_named: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_child_pronouns: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.family_more: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_EASTER_BREAK() -> Py { ensure_init(); G_EASTER_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASTER_BREAK(v: Py) { ensure_init(); G_EASTER_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SUMMER_BREAK() -> Py { ensure_init(); G_SUMMER_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUMMER_BREAK(v: Py) { ensure_init(); G_SUMMER_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FIRST_HOUR() -> Py { ensure_init(); G_THE_FIRST_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FIRST_HOUR(v: Py) { ensure_init(); G_THE_FIRST_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_QUESTIONED() -> Py { ensure_init(); G_THE_QUESTIONED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_QUESTIONED(v: Py) { ensure_init(); G_THE_QUESTIONED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_STOPPED() -> Py { ensure_init(); G_THE_STOPPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_STOPPED(v: Py) { ensure_init(); G_THE_STOPPED.with(|c| *c.borrow_mut() = v); }
pub fn g_WINTER_BREAK() -> Py { ensure_init(); G_WINTER_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WINTER_BREAK(v: Py) { ensure_init(); G_WINTER_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g__ADULT_AGE() -> Py { ensure_init(); G__ADULT_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ADULT_AGE(v: Py) { ensure_init(); G__ADULT_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__ADULT_RELATION() -> Py { ensure_init(); G__ADULT_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ADULT_RELATION(v: Py) { ensure_init(); G__ADULT_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_FIGURE() -> Py { ensure_init(); G__AGE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_FIGURE(v: Py) { ensure_init(); G__AGE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_SPELLED() -> Py { ensure_init(); G__AGE_SPELLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_SPELLED(v: Py) { ensure_init(); G__AGE_SPELLED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NAPPY_BAG() -> Py { ensure_init(); G__A_NAPPY_BAG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NAPPY_BAG(v: Py) { ensure_init(); G__A_NAPPY_BAG.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NURSING_CAREER() -> Py { ensure_init(); G__A_NURSING_CAREER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NURSING_CAREER(v: Py) { ensure_init(); G__A_NURSING_CAREER.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAST_COULD_NOT() -> Py { ensure_init(); G__BREAST_COULD_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAST_COULD_NOT(v: Py) { ensure_init(); G__BREAST_COULD_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAST_ILL() -> Py { ensure_init(); G__BREAST_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAST_ILL(v: Py) { ensure_init(); G__BREAST_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAST_LENGTH() -> Py { ensure_init(); G__BREAST_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAST_LENGTH(v: Py) { ensure_init(); G__BREAST_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAST_PAIN() -> Py { ensure_init(); G__BREAST_PAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAST_PAIN(v: Py) { ensure_init(); G__BREAST_PAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAST_PUBLIC() -> Py { ensure_init(); G__BREAST_PUBLIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAST_PUBLIC(v: Py) { ensure_init(); G__BREAST_PUBLIC.with(|c| *c.borrow_mut() = v); }
pub fn g__GROWN_UP_SAID() -> Py { ensure_init(); G__GROWN_UP_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROWN_UP_SAID(v: Py) { ensure_init(); G__GROWN_UP_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__LEFT_THE_COUNTRY() -> Py { ensure_init(); G__LEFT_THE_COUNTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEFT_THE_COUNTRY(v: Py) { ensure_init(); G__LEFT_THE_COUNTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__NAPPY_AGES() -> Py { ensure_init(); G__NAPPY_AGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAPPY_AGES(v: Py) { ensure_init(); G__NAPPY_AGES.with(|c| *c.borrow_mut() = v); }
pub fn g__NATION_WORDS() -> Py { ensure_init(); G__NATION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NATION_WORDS(v: Py) { ensure_init(); G__NATION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_UNIT() -> Py { ensure_init(); G__NOT_A_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_UNIT(v: Py) { ensure_init(); G__NOT_A_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_TIME_ASK() -> Py { ensure_init(); G__SCHOOL_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_TIME_ASK(v: Py) { ensure_init(); G__SCHOOL_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_WORDS() -> Py { ensure_init(); G__SCHOOL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_WORDS(v: Py) { ensure_init(); G__SCHOOL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_YEAR() -> Py { ensure_init(); G__SCHOOL_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_YEAR(v: Py) { ensure_init(); G__SCHOOL_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g__TERM_STARTS() -> Py { ensure_init(); G__TERM_STARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TERM_STARTS(v: Py) { ensure_init(); G__TERM_STARTS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_DATES_ARE_LOCAL() -> Py { ensure_init(); G__THE_DATES_ARE_LOCAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_DATES_ARE_LOCAL(v: Py) { ensure_init(); G__THE_DATES_ARE_LOCAL.with(|c| *c.borrow_mut() = v); }
pub fn g__TRAVEL_HOLIDAY() -> Py { ensure_init(); G__TRAVEL_HOLIDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRAVEL_HOLIDAY(v: Py) { ensure_init(); G__TRAVEL_HOLIDAY.with(|c| *c.borrow_mut() = v); }
pub fn g__a_phone_rule_and_an_age() -> Py { ensure_init(); G__a_phone_rule_and_an_age.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_phone_rule_and_an_age(v: Py) { ensure_init(); G__a_phone_rule_and_an_age.with(|c| *c.borrow_mut() = v); }
pub fn g__a_teenager_is_named() -> Py { ensure_init(); G__a_teenager_is_named.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_teenager_is_named(v: Py) { ensure_init(); G__a_teenager_is_named.with(|c| *c.borrow_mut() = v); }
pub fn g__the_child_pronouns() -> Py { ensure_init(); G__the_child_pronouns.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_child_pronouns(v: Py) { ensure_init(); G__the_child_pronouns.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The family rows from \"the questions a parent asks in the first years\" to the end.\n\n`family.py` went over the publisher's 1 MiB file limit (1,051,549 bytes) and the world then\nrefused every PR whose clone carried it, with `{\"error\": \"file too large: bot/knowledge/family.py\nis over 1048576 bytes\"}`, so no lane could change the family cards at all. The rows are data\nrather than code and they are the end of the list, so they move as one block, the way\n`home_craft.py`, `cooking_more.py` and `cooking_entries.py` hold the rows for their own files.\n\nNothing about the rows changed. `family.py` imports this file at the end, where these rows used\nto stand, and extends its own list with this one, so `family.ENTRIES` holds the same entries in\nthe same order as before. The helpers this block defines (`_school_date`, `_half_term`,\n`_said_date`, ...) travel with it; the three names it borrows from `family.py` are imported by\nname, which works because `family.py` only imports this module after every helper it needs is\nalready defined, and this module has no ENTRIES of its own for the registry to read as a domain.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s__ADULT_RELATION(pyrt::xrefs::xget("knowledge.family", "_ADULT_RELATION"));
    s__a_phone_rule_and_an_age(py_func("_a_phone_rule_and_an_age", |args: &[Py]| pyrt::xrefs::xcall("knowledge.family", "_a_phone_rule_and_an_age", args)));
    s__a_teenager_is_named(py_func("_a_teenager_is_named", |args: &[Py]| pyrt::xrefs::xcall("knowledge.family", "_a_teenager_is_named", args)));
    s__the_child_pronouns(py_func("_the_child_pronouns", |args: &[Py]| pyrt::xrefs::xcall("knowledge.family", "_the_child_pronouns", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.ENTRIES"); }
    s_ENTRIES(py_list(vec![]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._LEFT_THE_COUNTRY"); }
    s__LEFT_THE_COUNTRY(stdlib_call("re", "compile", &[py_str("\\b(?:emigrat\\w*|abroad|overseas|moving country|leaving the country|left the country)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("leaving for"), py_str("leaving to"), py_str("leaves for"), py_str("is leaving"), py_str("left for"), py_str("is off to"), py_str("off to live"), py_str("moving to live"), py_str("moving to"), py_str("is moving to"), py_str("going to live in"), py_str("going to live there"), py_str("going to live abroad"), py_str("emigrating"), py_str("is emigrating"), py_str("emigrated"), py_str("has emigrated"), py_str("moving abroad"), py_str("is moving abroad"), py_str("moving overseas"), py_str("is moving overseas"), py_str("going abroad"), py_str("is going abroad"), py_str("going overseas"), py_str("going travelling"), py_str("going traveling"), py_str("is going travelling"), py_str("leaving the country"), py_str("moving away to"), py_str("has left the country"), py_str("left the country")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("daughter"), py_str("son"), py_str("child"), py_str("kid"), py_str("kids"), py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("brother"), py_str("sister"), py_str("nan"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("niece"), py_str("nephew"), py_str("grandson"), py_str("granddaughter"), py_str("family"), py_str("best friend"), py_str("mate"), py_str("friend"), py_str("girlfriend"), py_str("boyfriend"), py_str("partner"), py_str("fiance"), py_str("fiancee"), py_str("husband"), py_str("wife")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("for a week"), py_str("for a fortnight"), py_str("for two weeks"), py_str("for ten days"), py_str("for a month"), py_str("on holiday"), py_str("coming back"), py_str("comes back"), py_str("visiting"), py_str("care home"), py_str("nursing home"), py_str("sheltered"), py_str("a flat"), py_str("a house"), py_str("a bungalow"), py_str("new place"), py_str("next door"), py_str("same town"), py_str("the city"), py_str("own flat"), py_str("own place"), py_str("own house"), py_str("his own"), py_str("her own"), py_str("their own"), py_str("down the road"), py_str("moving in with"), py_str("in with his"), py_str("in with her"), py_str("in with us")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-person-going-to-live-abroad")); __args.push(py_tuple(vec![__data1(), __data2()])); __args.push(py_str("Say the hard thing while they are still here rather than at the airport: one evening just the two of you, and tell them what they have been to you. Then put the contact in the diary before they go - a real time each week rather than 'we'll see'. The first month after is the quiet one, so have one thing in it that is only yours.")); __kws.extend(vec![("guard".to_string(), py_func("_another_country_is_in_the_line", |args: &[Py]| _another_country_is_in_the_line(args))), ("avoid".to_string(), __data3()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data4() -> Py { py_tuple(vec![py_str("take"), py_str("takes"), py_str("taking"), py_str("took"), py_str("refuse"), py_str("refuses"), py_str("refusing"), py_str("refused"), py_str("wont"), py_str("won't"), py_str("will not"), py_str("wont take"), py_str("won't take"), py_str("get my baby"), py_str("get him"), py_str("get her"), py_str("start"), py_str("starting"), py_str("switch"), py_str("switching"), py_str("introduce"), py_str("introducing"), py_str("combine"), py_str("combination"), py_str("back to work"), py_str("returning to work"), py_str("return to work"), py_str("how do i")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("newborns"), py_str("infant"), py_str("little one"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("weeks old"), py_str("week old"), py_str("month old"), py_str("months old"), py_str("breastfed"), py_str("breastfeeding"), py_str("nursery"), py_str("childminder"), py_str("him"), py_str("her"), py_str("introduce a bottle"), py_str("introducing a bottle"), py_str("introducing the bottle"), py_str("introduce the bottle"), py_str("introduce bottles"), py_str("introducing bottles"), py_str("start a bottle"), py_str("start bottles"), py_str("start formula"), py_str("introduce formula"), py_str("introducing formula"), py_str("introduce a formula"), py_str("introducing a formula"), py_str("introduce the formula"), py_str("a formula"), py_str("the formula"), py_str("switch to formula"), py_str("switching to formula"), py_str("combination feed"), py_str("combination feeding"), py_str("combi feed"), py_str("combi feeding"), py_str("mixed feeding"), py_str("top up with formula")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("not feeding"), py_str("isnt feeding"), py_str("isn't feeding"), py_str("refusing feeds"), py_str("reduced feeds"), py_str("not interested in feeds"), py_str("too sleepy to feed"), py_str("not taking feeds"), py_str("stopped feeding"), py_str("losing weight"), py_str("not gaining"), py_str("weaker"), py_str("floppy"), py_str("limp"), py_str("fever"), py_str("temperature"), py_str("stop my baby taking"), py_str("stop him taking"), py_str("stop her taking"), py_str("stop my baby having"), py_str("stop him having"), py_str("stop her having"), py_str("cup"), py_str("cups"), py_str("sippy cup"), py_str("free flow cup"), py_str("to a cup"), py_str("night feed"), py_str("night feeds"), py_str("off the bottle"), py_str("off bottles"), py_str("toddler"), py_str("stop giving"), py_str("stop the bottle"), py_str("stop using"), py_str("give up the bottle"), py_str("drop the bottle"), py_str("dropping the bottle"), py_str("coming off the bottle"), py_str("come off the bottle"), py_str("off the bottle"), py_str("at night"), py_str("night time"), py_str("night-time"), py_str("nighttime"), py_str("overnight"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("puppy"), py_str("puppies"), py_str("pet"), py_str("pets"), py_str("water bottle"), py_str("gym"), py_str("flight"), py_str("flying"), py_str("liquids"), py_str("100ml"), py_str("100 ml"), py_str("sterilise"), py_str("sterilizer"), py_str("sterilise"), py_str("steriliser"), py_str("washing"), py_str("wash"), py_str("clean"), py_str("pump"), py_str("breast pump"), py_str("which bottle"), py_str("best bottle"), py_str("brand"), py_str("cost"), py_str("buy"), py_str("how many bottles"), py_str("how many"), py_str("maths"), py_str("math"), py_str("mathematics"), py_str("algebra"), py_str("equation"), py_str("quadratic"), py_str("trigonometry"), py_str("geometry"), py_str("percentages"), py_str("gcse"), py_str("a level"), py_str("physics"), py_str("chemistry"), py_str("revision"), py_str("revising"), py_str("revise"), py_str("coursework"), py_str("exam"), py_str("exam paper"), py_str("mock exam"), py_str("spreadsheet"), py_str("excel"), py_str("autosum"), py_str("rearrange"), py_str("formula sheet"), py_str("learn the formula"), py_str("remember the formula")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("baby-refuses-the-bottle")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bottle"), py_str("bottles"), py_str("teat"), py_str("teats"), py_str("expressed milk"), py_str("formula"), py_str("bottle feeding"), py_str("bottle fed"), py_str("take a bottle"), py_str("take the bottle")]), __data4()])); __args.push(py_str("Give it when the baby is calm rather than hungry, and have somebody else offer it - a baby who can smell milk will hold out for the breast. Warm the teat and use a slow-flow one, hold them fairly upright and keep the bottle level so the milk comes at their pace rather than pouring in. Offer one bottle a day for a week without forcing it, and change the teat, the room and the person holding them before you decide they will not take one. If they are under six weeks old, are losing weight or the nappies are dry, ring the health visitor rather than pushing on.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), __data5()), ("avoid".to_string(), op_add(&__data6(), &pyrt::xrefs::xget("knowledge.family", "_ADULT_RELATION")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._GROWN_UP_SAID"); }
    s__GROWN_UP_SAID(py_tuple(vec![py_str("out all night"), py_str("boyfriend"), py_str("girlfriend")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._AGE_FIGURE"); }
    s__AGE_FIGURE(py_str("(?:1[89]|[2-9][0-9])"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._AGE_SPELLED"); }
    s__AGE_SPELLED(py_str("(?:eighteen|nineteen|twenty|thirty|forty|fifty|sixty(?:[- ](?:one|two|three|four|five|six|seven|eight|nine))?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._NOT_A_UNIT"); }
    s__NOT_A_UNIT(py_str("(?!\\s*(?:min|mins|minute|minutes|hour|hours|hr|hrs|day|days|week|weeks|month|months)\\b)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._ADULT_AGE"); }
    s__ADULT_AGE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:is|are|was|were|turns?|turned|now)\\s+(?:"), &g__AGE_FIGURE()), &py_str("|")), &g__AGE_SPELLED()), &py_str(")\\b")), &g__NOT_A_UNIT()), &py_str("|\\b(?:")), &g__AGE_FIGURE()), &py_str("|")), &g__AGE_SPELLED()), &py_str(")\\s*(?:years?|yrs?)(?:\\s*old)?\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_str("anxious"), py_str("anxiety"), py_str("worried"), py_str("worry"), py_str("worries"), py_str("worrying"), py_str("nervous"), py_str("scared"), py_str("frightened"), py_str("panic"), py_str("panics"), py_str("panicking"), py_str("overthinking"), py_str("on edge"), py_str("stressed"), py_str("reassurance"), py_str("reassure"), py_str("reassuring")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl"), py_str("toddler"), py_str("teenager"), py_str("teen"), py_str("year old"), py_str("years old"), py_str("little one"), py_str("my little"), py_str("school age")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("school"), py_str("class"), py_str("teacher"), py_str("lesson"), py_str("lessons"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("sats"), py_str("gcse"), py_str("a level"), py_str("revision"), py_str("i am anxious"), py_str("im anxious"), py_str("i'm anxious"), py_str("i feel anxious"), py_str("my anxiety"), py_str("i am worried"), py_str("im worried"), py_str("i'm worried"), py_str("i feel worried"), py_str("i get anxious"), py_str("i have anxiety"), py_str("about myself"), py_str("panic attack"), py_str("panic attacks")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("child-worry")); __args.push(py_tuple(vec![__data7(), __data8()])); __args.push(py_str("Name it and keep the day predictable: \"you've been worrying about that a lot\" does more than \"don't worry\", because the worry is real to them and being told it is nothing teaches them to hide it. Then take one small step towards whatever they are avoiding and praise the step rather than the result, and keep bedtime and meals at the same time so their body has something steady to settle on. If worry is stopping them going to school, eating or sleeping for more than a couple of weeks, ask the GP about children's mental health services.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&__data9(), &pyrt::xrefs::xget("knowledge.family", "_ADULT_RELATION"))), ("guard".to_string(), py_func("_the_child_is_not_grown", |args: &[Py]| _the_child_is_not_grown(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data10() -> Py { py_tuple(vec![py_str("puberty"), py_str("period"), py_str("periods"), py_str("bodies"), py_str("body changes"), py_str("body changing"), py_str("growing up"), py_str("sex"), py_str("sex education"), py_str("birds and bees"), py_str("voice breaking"), py_str("breasts"), py_str("wet dream"), py_str("wet dreams"), py_str("underarm hair"), py_str("pubic hair"), py_str("hormones"), py_str("body changes"), py_str("the changes"), py_str("what changes")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("talk"), py_str("talking"), py_str("tell"), py_str("explain"), py_str("discuss"), py_str("bring up"), py_str("what do i say"), py_str("what should i say"), py_str("how do i"), py_str("how to"), py_str("start"), py_str("started"), py_str("starting"), py_str("when"), py_str("what age"), py_str("old enough"), py_str("should i")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("puberty"), py_str("period"), py_str("periods"), py_str("bodies"), py_str("body"), py_str("growing up"), py_str("grow up"), py_str("sex"), py_str("sex education"), py_str("birds and bees"), py_str("voice"), py_str("breasts"), py_str("wet dream"), py_str("hair"), py_str("hormones"), py_str("changes"), py_str("girl"), py_str("girls"), py_str("boy"), py_str("boys"), py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("teenager"), py_str("teen")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("late"), py_str("missed"), py_str("pain"), py_str("painful"), py_str("cramps"), py_str("cramping"), py_str("heavy"), py_str("bleeding"), py_str("spotting"), py_str("irregular"), py_str("contraception"), py_str("pregnant"), py_str("pregnancy"), py_str("test"), py_str("my period"), py_str("periods stopped"), py_str("no period"), py_str("first period"), py_str("period after"), py_str("pain during sex"), py_str("bleed after sex"), py_str("stds"), py_str("sti"), py_str("condom"), py_str("condoms"), py_str("chicken"), py_str("turkey"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("cooked"), py_str("roast"), py_str("roasted"), py_str("bake"), py_str("baked"), py_str("juicy"), py_str("moist"), py_str("tender"), py_str("oven"), py_str("grill"), py_str("grilled"), py_str("fry"), py_str("fried"), py_str("pan"), py_str("boil"), py_str("poach"), py_str("recipe"), py_str("dinner"), py_str("sausages"), py_str("breast of"), py_str("breast milk"), py_str("breastmilk"), py_str("expressed milk"), py_str("formula milk"), py_str("pumped milk"), py_str("fat"), py_str("fats"), py_str("calories"), py_str("diet"), py_str("dieting"), py_str("overweight"), py_str("obese"), py_str("bmi"), py_str("body fat"), py_str("belly"), py_str("lose weight"), py_str("losing weight"), py_str("weight loss"), py_str("lose a stone"), py_str("put on weight"), py_str("burn"), py_str("burning")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("talk-about-puberty")); __args.push(py_tuple(vec![__data10(), __data11()])); __args.push(py_str("Start earlier and in smaller pieces than feels comfortable: the changes usually begin around nine or ten, and a first period comes about two years after the breasts start to develop, so the explaining has to be done before it happens. Use the proper words, keep it to a few minutes at a time in the car or at bedtime, and cover both sexes so a boy hears about periods and a girl about wet dreams. A good book left where they will find it does half the work, and \"ask me anything, including the embarrassing bits\" keeps the door open.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), __data12()), ("avoid".to_string(), __data13())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data14() -> Py { py_tuple(vec![py_str("eating"), py_str("eats"), py_str("ate"), py_str("eaten"), py_str("taking"), py_str("takes"), py_str("took"), py_str("stealing"), py_str("steals"), py_str("stolen"), py_str("food"), py_str("fridge"), py_str("cupboard"), py_str("leftovers"), py_str("labelled"), py_str("label"), py_str("share"), py_str("sharing")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("rent"), py_str("deposit"), py_str("bills"), py_str("bill"), py_str("pays"), py_str("paid"), py_str("loud"), py_str("noise"), py_str("music"), py_str("party"), py_str("clean"), py_str("cleaning"), py_str("washing up"), py_str("mess"), py_str("dirty"), py_str("smells"), py_str("smell"), py_str("depressed"), py_str("lonely"), py_str("moving out"), py_str("moves out"), py_str("mould"), py_str("damp"), py_str("landlord")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("flatmate-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("flatmate"), py_str("flatmates"), py_str("housemate"), py_str("housemates"), py_str("roommate"), py_str("roommates"), py_str("house share"), py_str("house mate"), py_str("houseshare")]), __data14()])); __args.push(py_str("Label a shelf and one tub of your own, then say it once, face to face: \"I'm happy to share milk and bread, but I'd rather the rest was mine\" is something a person can agree to, and a note on the fridge is not. Agree a shared shelf for the things you both use, and if it carries on, ask to split the shopping bill rather than policing the fridge.")); __kws.extend(vec![("avoid".to_string(), __data15())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data16() -> Py { py_tuple(vec![py_str("teenager"), py_str("teen"), py_str("teenagers"), py_str("son"), py_str("daughter"), py_str("14 year old"), py_str("15 year old"), py_str("16 year old"), py_str("13 year old"), py_str("17 year old"), py_str("year 10"), py_str("year 11"), py_str("gcse"), py_str("gcses"), py_str("secondary school"), py_str("high school")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("wont get up"), py_str("won't get up"), py_str("wont get out of bed"), py_str("won't get out of bed"), py_str("wont wake"), py_str("won't wake"), py_str("wont wake up"), py_str("won't wake up"), py_str("late for school"), py_str("cant wake him"), py_str("can't wake him"), py_str("cant wake her"), py_str("can't wake her"), py_str("sleeps in"), py_str("sleeping in"), py_str("wont go to school"), py_str("won't go to school"), py_str("refusing school"), py_str("refuses school"), py_str("school refusal"), py_str("misses the bus"), py_str("always late"), py_str("wont get ready"), py_str("won't get ready"), py_str("stays up"), py_str("staying up"), py_str("wont go to sleep"), py_str("won't go to sleep"), py_str("cant get him up"), py_str("can't get him up"), py_str("cant get her up"), py_str("can't get her up")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("phone"), py_str("phones"), py_str("charger"), py_str("screens"), py_str("screen time"), py_str("tablet"), py_str("ipad"), py_str("xbox"), py_str("playstation"), py_str("console"), py_str("gaming"), py_str("tiktok"), py_str("instagram"), py_str("snapchat"), py_str("youtube"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("newborn"), py_str("months old"), py_str("i cant get out of bed"), py_str("i can't get out of bed"), py_str("i wont get up"), py_str("my alarm"), py_str("shift work"), py_str("night shift"), py_str("insomnia"), py_str("i cant sleep"), py_str("i can't sleep"), py_str("i feel"), py_str("i am"), py_str("im "), py_str("depressed"), py_str("depression"), py_str("anxious"), py_str("low mood"), py_str("self harm"), py_str("exam stress"), py_str("revision"), py_str("cant be bothered"), py_str("lazy")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("teen-wont-get-up")); __args.push(py_tuple(vec![__data16(), __data17()])); __args.push(py_str("Aim at the wake time, not the bedtime: get them up at the same time every day, weekends included, and open the curtains and put breakfast out straight away, because the body clock follows the morning light and the evening sorts itself out a week or two later. Take the phone and the charger out of the room, and let them stay up later rather than fight a 10pm battle a teenager's body cannot lose. If they are missing school most days for more than a couple of weeks, or sleeping through alarms and falling asleep in class, ring the school and ask the GP about their sleep - that is not laziness.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_a_teenager_is_named", |args: &[Py]| pyrt::xrefs::xcall("knowledge.family", "_a_teenager_is_named", args))), ("avoid".to_string(), __data18())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data19() -> Py { py_tuple(vec![py_str("phone"), py_str("phones"), py_str("mobile"), py_str("ipad"), py_str("tablet"), py_str("screen time"), py_str("screens"), py_str("gaming"), py_str("games console"), py_str("playstation"), py_str("xbox"), py_str("nintendo"), py_str("device"), py_str("devices"), py_str("social media"), py_str("tiktok"), py_str("snapchat"), py_str("instagram"), py_str("youtube")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("teen"), py_str("teenager"), py_str("year old"), py_str("years old"), py_str("take it off"), py_str("take it away"), py_str("take his"), py_str("take her"), py_str("take the"), py_str("confiscate"), py_str("limit"), py_str("limits"), py_str("too much"), py_str("whole life"), py_str("addicted"), py_str("a phone"), py_str("have a phone"), py_str("have tiktok"), py_str("have snapchat"), py_str("have instagram"), py_str("too young")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("cracked"), py_str("crack"), py_str("smashed"), py_str("broken"), py_str("repair"), py_str("battery"), py_str("charge not"), py_str("wont charge"), py_str("clean"), py_str("cleaning"), py_str("cleaned"), py_str("wipe"), py_str("wiping"), py_str("smudge"), py_str("smudges"), py_str("smear"), py_str("smears"), py_str("fingerprint"), py_str("fingerprints"), py_str("greasy"), py_str("mark on the screen"), py_str("scratch"), py_str("scratches"), py_str("scratched"), py_str("won't charge"), py_str("water damage"), py_str("warranty"), py_str("contract"), py_str("sim"), py_str("tariff"), py_str("which phone"), py_str("what phone"), py_str("best phone"), py_str("new phone"), py_str("buy a phone"), py_str("how much does"), py_str("cost"), py_str("at night"), py_str("bedtime"), py_str("staying up"), py_str("stays up"), py_str("till"), py_str("2am"), py_str("go to sleep"), py_str("get to sleep"), py_str("in bed"), py_str("wont sleep"), py_str("won't sleep"), py_str("holder"), py_str("car holder"), py_str("mount"), py_str("phone mount"), py_str("cradle"), py_str("windscreen"), py_str("dashboard"), py_str("vent clip"), py_str("out of the car"), py_str("out of my car"), py_str("saw something"), py_str("seen something"), py_str("saw a video"), py_str("watched something"), py_str("what they saw"), py_str("inappropriate"), py_str("upsetting video"), py_str("scary video"), py_str("lost"), py_str("lose"), py_str("loses"), py_str("lost it"), py_str("lost her"), py_str("lost his"), py_str("lost their"), py_str("mislaid"), py_str("stolen"), py_str("stole"), py_str("robbed"), py_str("find my"), py_str("tracker"), py_str("left it"), py_str("left her phone"), py_str("left his phone"), py_str("cant find"), py_str("can't find"), py_str("cannot find"), py_str("charge"), py_str("charging"), py_str("charger"), py_str("battery"), py_str("port"), py_str("interview"), py_str("cv"), py_str("application"), py_str("tablets"), py_str("pills"), py_str("medicine"), py_str("meds"), py_str("prescription"), py_str("antibiotics"), py_str("pharmacy"), py_str("dose"), py_str("doctor said"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("rabbit"), py_str("hamster"), py_str("guinea pig"), py_str("pet"), py_str("pets"), py_str("the vet"), py_str("vet")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-child-and-a-phone")); __args.push(py_tuple(vec![__data19(), __data20()])); __args.push(py_func("_a_phone_rule_and_an_age", |args: &[Py]| pyrt::xrefs::xcall("knowledge.family", "_a_phone_rule_and_an_age", args))); __kws.extend(vec![("avoid".to_string(), __data21())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data22() -> Py { py_tuple(vec![py_str("3 month"), py_str("3 months"), py_str("three month"), py_str("three months"), py_str("4 month"), py_str("4 months"), py_str("four month"), py_str("four months"), py_str("6 week"), py_str("6 weeks"), py_str("six week"), py_str("six weeks"), py_str("2 month"), py_str("2 months"), py_str("two month"), py_str("two months")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("6 month"), py_str("6 months"), py_str("six month"), py_str("six months"), py_str("7 month"), py_str("7 months"), py_str("seven month"), py_str("seven months"), py_str("8 month"), py_str("8 months"), py_str("eight months"), py_str("9 month"), py_str("9 months"), py_str("nine months")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("1 year"), py_str("1 years"), py_str("one year"), py_str("12 month"), py_str("12 months"), py_str("twelve months"), py_str("18 month"), py_str("18 months"), py_str("eighteen months"), py_str("toddler"), py_str("2 year"), py_str("2 years"), py_str("two year"), py_str("two years")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._NAPPY_AGES"); }
    s__NAPPY_AGES(py_tuple(vec![py_tuple(vec![__data22(), py_str("By three months it is about six to eight a day - a pack of 30 every four or five days - and most babies have moved up two sizes by then.")]), py_tuple(vec![__data23(), py_str("By six months it is about five to seven a day, so a pack of 30 lasts around five days, and most babies are in size 3 or 4 by then.")]), py_tuple(vec![__data24(), py_str("A one year old gets through about four to six a day - two packs of 30 a month - and a toddler nearer four, falling off as potty training starts.")])]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nappies-how-many")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nappies"), py_str("nappy"), py_str("diapers"), py_str("diaper")]), py_tuple(vec![py_str("how many"), py_str("a day"), py_str("per day"), py_str("do i need"), py_str("will i need"), py_str("a week"), py_str("how often"), py_str("newborn"), py_str("a month"), py_str("stock up"), py_str("buy")])])); __args.push(py_func("_nappies_a_day", |args: &[Py]| _nappies_a_day(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("rash"), py_str("sore"), py_str("red bottom"), py_str("cream"), py_str("how do i change"), py_str("how to change"), py_str("nappy bag"), py_str("changing bag"), py_str("pack"), py_str("leaking"), py_str("leak")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._A_NAPPY_BAG"); }
    s__A_NAPPY_BAG(py_str("Pack for two changes plus one for luck: four or five nappies, wipes or two damp flannels in a bag, a fold-out changing mat, nappy sacks, a full spare vest and sleepsuit, a muslin and a spare top for you. Add a feed - a bottle with the powder in a pot, or a cover if you are breastfeeding - and a snack and water for yourself. Restock it the moment you get home, because the one day it is empty is the day you are out."));
    fn __data25() -> Py { py_tuple(vec![py_str("rash"), py_str("cream"), py_str("how do i change"), py_str("how to change"), py_str("hospital bag"), py_str("hospital bag for birth"), py_str("weekend"), py_str("weekend away"), py_str("week away"), py_str("for a weekend"), py_str("holiday"), py_str("holidays"), py_str("suitcase"), py_str("for a trip"), py_str("overnight"), py_str("labour"), py_str("birth")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nappy-bag")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nappy bag"), py_str("changing bag"), py_str("baby bag"), py_str("packing bag")]), py_tuple(vec![py_str("nappy"), py_str("nappies"), py_str("baby"), py_str("newborn"), py_str("out"), py_str("going out"), py_str("day out"), py_str("hospital bag"), py_str("pack"), py_str("take"), py_str("need")])])); __args.push(g__A_NAPPY_BAG()); __kws.extend(vec![("avoid".to_string(), __data25())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nappy-bag-packing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("what should i pack"), py_str("what do i pack"), py_str("what shall i pack"), py_str("what to pack"), py_str("what should we pack"), py_str("what should i take")]), py_tuple(vec![py_str("nappy"), py_str("nappies"), py_str("baby"), py_str("babys"), py_str("newborn"), py_str("formula"), py_str("bottle"), py_str("changing"), py_str("feed"), py_str("moses basket")])])); __args.push(g__A_NAPPY_BAG()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("rash"), py_str("cream"), py_str("how do i change"), py_str("how to change"), py_str("hospital bag for birth"), py_str("labour"), py_str("birth"), py_str("weekend away"), py_str("week away"), py_str("for a weekend")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nappy-cream")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nappy cream"), py_str("barrier cream"), py_str("sudocrem"), py_str("metanium"), py_str("bepanthen"), py_str("nappy ointment")]), py_tuple(vec![py_str("which"), py_str("best"), py_str("what"), py_str("should i use"), py_str("do i need"), py_str("buy"), py_str("use"), py_str("recommend"), py_str("how much"), py_str("safe"), py_str("cream")])])); __args.push(py_str("A barrier cream with zinc oxide in it is the one to have - Sudocrem, Metanium or Bepanthen all do the same job - and a thin layer at every change is enough once the skin looks pink. Thick white paste is for a sore bottom, not for prevention. If it is still red after three days, or there are spots at the edges, that is usually thrush and needs an antifungal cream from the health visitor rather than more barrier cream.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("rash that"), py_str("thrush"), py_str("nappy rash is not"), py_str("how do i change"), py_str("ingest"), py_str("eat"), py_str("swallowed")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data26() -> Py { py_tuple(vec![py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("painful"), py_str("pain"), py_str("sore"), py_str("soreness"), py_str("cracked"), py_str("bleeding"), py_str("latch"), py_str("latching"), py_str("engorged"), py_str("mastitis"), py_str("blocked duct")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._BREAST_PAIN"); }
    s__BREAST_PAIN(__data26());
    fn __data27() -> Py { py_tuple(vec![py_str("how long"), py_str("how many months"), py_str("what age"), py_str("until"), py_str("till"), py_str("how old"), py_str("months"), py_str("years"), py_str("how much longer"), py_str("toddler"), py_str("older baby"), py_str("one year"), py_str("two years"), py_str("how long for"), py_str("how many years")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._BREAST_LENGTH"); }
    s__BREAST_LENGTH(__data27());
    fn __data28() -> Py { py_tuple(vec![py_str(" im ill"), py_str(" if im ill"), py_str("im ill"), py_str("i am ill"), py_str("feel ill"), py_str("feeling ill"), py_str("poorly"), py_str("unwell"), py_str("i am sick"), py_str("im sick"), py_str("being sick"), py_str("a cold"), py_str("with a cold"), py_str("got a cold"), py_str("flu"), py_str("fever"), py_str("temperature"), py_str("chest infection"), py_str("covid"), py_str("food poisoning"), py_str("vomiting"), py_str("diarrhoea"), py_str("diarrhea"), py_str("migraine"), py_str("tonsillitis"), py_str("infection"), py_str("on antibiotics")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._BREAST_ILL"); }
    s__BREAST_ILL(__data28());
    fn __data29() -> Py { py_tuple(vec![py_str("cant"), py_str("can't"), py_str("cannot"), py_str("couldnt"), py_str("couldn't"), py_str("could not"), py_str("unable"), py_str("not able"), py_str("didnt manage"), py_str("didn't manage"), py_str("never managed"), py_str("gave up"), py_str("give up"), py_str("stopped"), py_str("stopping"), py_str("had to stop"), py_str("wouldnt latch"), py_str("wouldn't latch"), py_str("wouldnt feed"), py_str("wouldn't feed"), py_str("feel bad"), py_str("feeling bad"), py_str("feel guilty"), py_str("feeling guilty"), py_str("guilty"), py_str("feel awful"), py_str("failure"), py_str("a failure"), py_str("not good enough"), py_str("let my baby down"), py_str("let her down"), py_str("let him down"), py_str("cried"), py_str("beat myself up")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._BREAST_COULD_NOT"); }
    s__BREAST_COULD_NOT(__data29());
    fn __data30() -> Py { py_tuple(vec![py_str("retrain"), py_str("retraining"), py_str("retrained"), py_str("career"), py_str("careers"), py_str("become a nurse"), py_str("becoming a nurse"), py_str("be a nurse"), py_str("train as a nurse"), py_str("training as a nurse"), py_str("nursing degree"), py_str("nursing course"), py_str("nursing job"), py_str("nursing home"), py_str("staff nurse"), py_str("ward"), py_str("go back to nursing"), py_str("return to nursing"), py_str("back to nursing"), py_str("qualify as a nurse"), py_str("nurse training"), py_str("midwife"), py_str("midwifery"), py_str("nursery nurse"), py_str("veterinary nurse"), py_str("dental nurse"), py_str("salary"), py_str("shifts"), py_str("hospital"), py_str("agency"), py_str("apply for nursing"), py_str("nursing at"), py_str("job"), py_str("jobs"), py_str("worth it"), py_str("pay"), py_str("paid"), py_str("pension"), py_str("part time"), py_str("full time"), py_str("bank staff"), py_str("nhs")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._A_NURSING_CAREER"); }
    s__A_NURSING_CAREER(__data30());
    fn __data31() -> Py { py_tuple(vec![py_str("in public"), py_str("public"), py_str("in a cafe"), py_str("in a restaurant"), py_str("on a train"), py_str("on the bus"), py_str("in front of"), py_str("people stare"), py_str("stares"), py_str("ask me to stop"), py_str("ask me to leave"), py_str("cover up"), py_str("cover myself"), py_str("let me"), py_str("legal"), py_str("where can i feed"), py_str("feeding in public")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._BREAST_PUBLIC"); }
    s__BREAST_PUBLIC(__data31());
    fn __data32() -> Py { py_tuple(vec![py_str("breastfeeding"), py_str("breast feeding"), py_str("breastfeed"), py_str("breast fed"), py_str("breastfed"), py_str("nursing"), py_str("chestfeeding"), py_str("chest feeding"), py_str("exclusively breastfeeding"), py_str("bottle of formula"), py_str("give formula"), py_str("giving formula"), py_str("switch to formula"), py_str("mixed feeding"), py_str("breast and formula"), py_str("formula and breast")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("ok"), py_str("okay"), py_str("normal"), py_str("alright"), py_str("fine"), py_str("is it"), py_str("is that"), py_str("should i"), py_str("shouldnt i"), py_str("cant"), py_str("can't"), py_str("couldnt"), py_str("couldn't"), py_str("unable"), py_str("guilty"), py_str("feel bad"), py_str("feel awful"), py_str("gave up"), py_str("stopped"), py_str("in public"), py_str("public"), py_str("can i"), py_str("good"), py_str("healthy"), py_str("hurt"), py_str("hurts"), py_str("painful"), py_str("sore"), py_str("how long"), py_str("how often"), py_str("how many"), py_str("months"), py_str("years"), py_str("worth it"), py_str("what age"), py_str("until"), py_str("till"), py_str("stop"), py_str("carry on"), py_str("keep going"), py_str("safe"), py_str("worried"), py_str("worry"), py_str("formula"), py_str("bottle"), py_str("bottles"), py_str("mixed")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("coffee"), py_str("caffeine"), py_str("tea"), py_str("alcohol"), py_str("wine"), py_str("beer"), py_str("gin"), py_str("vodka"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("medicine"), py_str("medication"), py_str("paracetamol"), py_str("ibuprofen"), py_str("antibiotic"), py_str("antibiotics"), py_str("tablet"), py_str("tablets"), py_str("pill"), py_str("eat"), py_str("eating"), py_str("food"), py_str("foods"), py_str("diet"), py_str("what can i take"), py_str("can i take"), py_str("what to take"), py_str("take something"), py_str("painkiller"), py_str("painkillers"), py_str("remedy"), py_str("anything for the pain"), py_str("cheese"), py_str("sushi"), py_str("peanuts"), py_str("liver"), py_str("pump"), py_str("pumping"), py_str("express"), py_str("expressing"), py_str("wean"), py_str("weaning"), py_str("solids"), py_str("sleep"), py_str("co-sleep"), py_str("teeth"), py_str("cracked nipple"), py_str("a nurse"), py_str("a nurse in"), py_str("nursing assistant"), py_str("nurse work"), py_str("nurse job"), py_str("nurse shift"), py_str("nurse shifts"), py_str("nurse salary"), py_str("does a nurse"), py_str("do nurses"), py_str("nurses work")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breastfeeding-basics")); __args.push(py_tuple(vec![__data32(), __data33()])); __args.push(py_func("_breastfeeding_basics", |args: &[Py]| _breastfeeding_basics(args))); __kws.extend(vec![("guard".to_string(), py_func("_not_a_nursing_career", |args: &[Py]| _not_a_nursing_career(args))), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data34())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data35() -> Py { py_tuple(vec![py_str("school"), py_str("schools"), py_str("kids"), py_str("children"), py_str("child"), py_str("term"), py_str("terms"), py_str("break"), py_str("breaks"), py_str("term dates"), py_str("break up"), py_str("breaks up"), py_str("broke up"), py_str("nursery"), py_str("academy"), py_str("primary"), py_str("secondary"), py_str("infant"), py_str("junior"), py_str("sixth form"), py_str("pupil"), py_str("pupils"), py_str("holidays"), py_str("half term")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._SCHOOL_WORDS"); }
    s__SCHOOL_WORDS(__data35());
    fn __data36() -> Py { py_tuple(vec![py_str("spain"), py_str("portugal"), py_str("greece"), py_str("turkey"), py_str("france"), py_str("italy"), py_str("florida"), py_str("abroad"), py_str("flights"), py_str("flight"), py_str("hotel"), py_str("hotels"), py_str("where"), py_str("book"), py_str("booking"), py_str("booked"), py_str("cheap"), py_str("cheapest"), py_str("all inclusive"), py_str("package"), py_str("resort"), py_str("villa"), py_str("airbnb"), py_str("my holiday"), py_str("our holiday"), py_str("shall we go"), py_str("to go"), py_str("should we go")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._TRAVEL_HOLIDAY"); }
    s__TRAVEL_HOLIDAY(__data36());
    fn __data37() -> Py { py_tuple(vec![py_str("when"), py_str("date"), py_str("dates"), py_str("start"), py_str("starts"), py_str("begin"), py_str("begins"), py_str("end"), py_str("ends"), py_str("finish"), py_str("finishes"), py_str("back"), py_str("go back"), py_str("how long"), py_str("how many"), py_str("days"), py_str("weeks"), py_str("till"), py_str("until"), py_str("until"), py_str("countdown"), py_str("soon"), py_str("time")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._SCHOOL_TIME_ASK"); }
    s__SCHOOL_TIME_ASK(__data37());
    fn __data38() -> Py { py_tuple(vec![py_str("scotland"), py_str("scottish"), py_str("glasgow"), py_str("edinburgh"), py_str("aberdeen"), py_str("dundee"), py_str("inverness"), py_str("stirling"), py_str("fife"), py_str("paisley"), py_str("perthshire"), py_str("highlands"), py_str("shetland"), py_str("orkney")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("england"), py_str("english"), py_str("london"), py_str("manchester"), py_str("birmingham"), py_str("leeds"), py_str("bristol"), py_str("newcastle"), py_str("liverpool"), py_str("sheffield"), py_str("nottingham"), py_str("yorkshire"), py_str("kent"), py_str("essex"), py_str("cornwall"), py_str("surrey"), py_str("devon")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._NATION_WORDS"); }
    s__NATION_WORDS(py_tuple(vec![py_tuple(vec![py_str("scotland"), __data38()]), py_tuple(vec![py_str("wales"), py_tuple(vec![py_str("wales"), py_str("welsh"), py_str("cardiff"), py_str("swansea"), py_str("newport"), py_str("wrexham"), py_str("bangor"), py_str("caernarfon"), py_str("bridgend"), py_str("carmarthen")])]), py_tuple(vec![py_str("northern ireland"), py_tuple(vec![py_str("northern ireland"), py_str("northern irish"), py_str("belfast"), py_str("derry"), py_str("londonderry"), py_str("armagh"), py_str("lisburn"), py_str("newry"), py_str("antrim"), py_str("ulster")])]), py_tuple(vec![py_str("ireland"), py_tuple(vec![py_str("ireland"), py_str("irish"), py_str("dublin"), py_str("cork"), py_str("galway"), py_str("limerick"), py_str("republic of ireland")])]), py_tuple(vec![py_str("england"), __data39()])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._SCHOOL_YEAR"); }
    s__SCHOOL_YEAR(py_dict(vec![(py_str("scotland"), py_str("Scottish schools go back in the middle of August, usually between the 15th and the 20th, and break up at the end of June. The holidays are a week in mid-October, about two weeks at Christmas, a week or so at Easter and the long summer break from the last week of June.")), (py_str("wales"), py_str("Welsh schools usually go back in the first days of September and break up in the third week of July, with half term a week in late October and about two weeks at Christmas. Each council sets its own dates and they differ by a few days.")), (py_str("northern ireland"), py_str("Schools in Northern Ireland go back in the first days of September and finish at the end of June, with half terms in late October and mid-February, two weeks at Christmas and a week or two at Easter.")), (py_str("ireland"), py_str("Irish schools go back at the end of August or in the first days of September and finish at the end of June, with a week off in late October, two weeks at Christmas, a week in February and Easter week.")), (py_str("england"), Py::None)]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._THE_DATES_ARE_LOCAL"); }
    s__THE_DATES_ARE_LOCAL(py_str(" Every council, education authority and academy sets its own exact dates, so check the council's school term dates page or the school's own calendar for the day."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.SUMMER_BREAK"); }
    s_SUMMER_BREAK(py_str("English state schools break up in the third week of July and go back in the first days of September"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.WINTER_BREAK"); }
    s_WINTER_BREAK(py_str("The Christmas break runs from just before Christmas to the first days of January"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.EASTER_BREAK"); }
    s_EASTER_BREAK(py_str("The Easter break is the two weeks around Easter weekend: schools usually break up on the Friday a week before Good Friday and go back the Monday after Easter Monday"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more._TERM_STARTS"); }
    s__TERM_STARTS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("summer term"), py_str("summer terms")]), py_str("The summer term starts in the second half of April, a week after the Easter break, and runs to the third week of July")]), py_tuple(vec![py_tuple(vec![py_str("autumn term"), py_str("autumn terms"), py_str("winter term")]), py_str("The autumn term starts in the first days of September and runs to about 19 December")]), py_tuple(vec![py_tuple(vec![py_str("spring term"), py_str("spring terms")]), py_str("The spring term starts on the first Tuesday or Wednesday in January and runs to the Easter break")])]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-break-summer")); __args.push(py_tuple(vec![py_tuple(vec![py_str("summer holidays"), py_str("summer holiday"), py_str("summer break"), py_str("summer term"), py_str("summer hols"), py_str("six weeks holiday"), py_str("six weeks holidays"), py_str("school summer holidays"), py_str("break up for summer"), py_str("break up for the summer"), py_str("breaks up for summer")]), g__SCHOOL_TIME_ASK()])); __args.push(py_func("_summer_break", |args: &[Py]| _summer_break(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), g__SCHOOL_WORDS()), ("avoid".to_string(), op_add(&g__TRAVEL_HOLIDAY(), &py_tuple(vec![py_str("activities"), py_str("things to do"), py_str("what to do"), py_str("days out"), py_str("keep them busy"), py_str("ideas"), py_str("bored"), py_str("rainy"), py_str("suncream"), py_str("packing")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data40() -> Py { py_tuple(vec![py_str("santa"), py_str("presents"), py_str("present"), py_str("dinner"), py_str("turkey"), py_str("tree"), py_str("decorations"), py_str("gift"), py_str("gifts"), py_str("advent"), py_str("carols"), py_str("carol"), py_str("nativity"), py_str("activities"), py_str("things to do"), py_str("what to do")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-break-christmas")); __args.push(py_tuple(vec![py_tuple(vec![py_str("christmas holidays"), py_str("christmas break"), py_str("christmas term"), py_str("break up for christmas"), py_str("schools break up for christmas"), py_str("christmas school holidays"), py_str("end of term")]), g__SCHOOL_TIME_ASK()])); __args.push(py_func("_winter_break", |args: &[Py]| _winter_break(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), g__SCHOOL_WORDS()), ("avoid".to_string(), __data40())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-break-easter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("easter holidays"), py_str("easter break"), py_str("break up for easter"), py_str("easter term")]), g__SCHOOL_TIME_ASK()])); __args.push(py_func("_easter_break", |args: &[Py]| _easter_break(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), g__SCHOOL_WORDS()), ("avoid".to_string(), py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("chocolate"), py_str("church"), py_str("service"), py_str("lamb"), py_str("activities"), py_str("things to do"), py_str("what to do")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data41() -> Py { py_tuple(vec![py_str("activities"), py_str("things to do"), py_str("what to do"), py_str("days out"), py_str("ideas"), py_str("bored"), py_str("rainy"), py_str("keep them busy"), py_str("kids club"), py_str("camps"), py_str("childcare"), py_str("time off work"), py_str("cover"), py_str("work"), py_str("what can we do"), py_str("what can i do"), py_str("what shall we do"), py_str("what should we do"), py_str("do with the kids"), py_str("do with my"), py_str("entertain"), py_str("rainy day"), py_str("stuck indoors"), py_str("how do i keep")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-half-term")); __args.push(py_tuple(vec![py_tuple(vec![py_str("half term"), py_str("halfterm"), py_str("half-term"), py_str("half terms")]), op_add(&g__SCHOOL_TIME_ASK(), &py_tuple(vec![py_str("off")]))])); __args.push(py_func("_half_term", |args: &[Py]| _half_term(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data41())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data42() -> Py { py_tuple(vec![py_str("school go back"), py_str("schools go back"), py_str("go back to school"), py_str("school goes back"), py_str("back to school date"), py_str("term starts"), py_str("new term"), py_str("school return"), py_str("school starts again"), py_str("kids go back"), py_str("children go back"), py_str("go back after christmas"), py_str("back after christmas"), py_str("go back after half term"), py_str("go back after easter")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("uniform"), py_str("shoes"), py_str("lunchbox"), py_str("bag"), py_str("nervous"), py_str("anxious"), py_str("worried"), py_str("first day"), py_str("friends"), py_str("shopping"), py_str("supplies"), py_str("clothes"), py_str("bullying"), py_str("teacher"), py_str("won't go"), py_str("wont go"), py_str("refusing"), py_str("my child"), py_str("my son"), py_str("my daughter"), py_str("my baby"), py_str("my boy"), py_str("my girl"), py_str("he is ill"), py_str("she is ill")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-back-to-school")); __args.push(py_tuple(vec![__data42(), py_tuple(vec![py_str("when"), py_str("date"), py_str("dates"), py_str("term dates"), py_str("term"), py_str("starts"), py_str("start again"), py_str("back"), py_str("first day"), py_str("return")])])); __args.push(py_func("_back_to_school", |args: &[Py]| _back_to_school(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), py_tuple(vec![py_str("school"), py_str("schools"), py_str("kids"), py_str("children"), py_str("child"), py_str("term"), py_str("term dates"), py_str("nursery")])), ("avoid".to_string(), __data43())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data44() -> Py { py_tuple(vec![py_str("school holidays"), py_str("school holiday"), py_str("term dates"), py_str("school term"), py_str("school terms"), py_str("school breaks up"), py_str("schools break up"), py_str("kids off school"), py_str("children off school"), py_str("kids break up"), py_str("children break up"), py_str("kids finish school"), py_str("when do we break up")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-holidays-when")); __args.push(py_tuple(vec![__data44(), g__SCHOOL_TIME_ASK()])); __args.push(py_func("_school_holidays_general", |args: &[Py]| _school_holidays_general(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g__TRAVEL_HOLIDAY(), &py_tuple(vec![py_str("activities"), py_str("things to do"), py_str("what to do"), py_str("days out"), py_str("keep them busy"), py_str("ideas"), py_str("bored"), py_str("rainy"), py_str("childcare"), py_str("cover"), py_str("juggle"), py_str("work")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.THE_FIRST_HOUR"); }
    s_THE_FIRST_HOUR(py_str("Get the station and the custody record number first, because that reference is what everyone else quotes from then on. Ask for the free duty solicitor: saying you want one is not an admission and it is the step most people skip. Note the time of the arrest as well, since the clock starts there - usually 24 hours before they have to charge or release."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.THE_STOPPED"); }
    s_THE_STOPPED(py_str("A stop and search needs grounds: ask which power is being used and why, and ask for the record slip, which carries the officer's number, the station and the reason. Being stopped is not the same as being arrested, so write down the time and the names while it is fresh, and complain to the force if the slip is refused."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.THE_QUESTIONED"); }
    s_THE_QUESTIONED(py_str("If they are under 18 an appropriate adult - you, or another adult - has to be there before they are interviewed, so say that at the desk. Under caution 'no comment' is allowed and the solicitor is free; write down what was said and when, while you both remember it."));
    fn __data45() -> Py { py_tuple(vec![py_str("arrested"), py_str("arrest"), py_str("arrests"), py_str("police custody"), py_str("in custody"), py_str("held in custody"), py_str("taken into custody"), py_str("remanded"), py_str("charged with"), py_str("been charged"), py_str("on bail"), py_str("out on bail"), py_str("bailed"), py_str("police station"), py_str("in trouble with the police"), py_str("police trouble"), py_str("stopped by the police"), py_str("stopped by police"), py_str("police stopped"), py_str("stop and search"), py_str("searched by the police"), py_str("searched by police"), py_str("police search"), py_str("questioned by police"), py_str("questioned by the police"), py_str("police interview"), py_str("under caution"), py_str("taken to the station")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("teenager"), py_str("teen"), py_str("boy"), py_str("girl"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("brother"), py_str("sister"), py_str("nephew"), py_str("niece"), py_str("stepson"), py_str("stepdaughter"), py_str("mum"), py_str("dad"), py_str("mother"), py_str("father"), py_str("i"), py_str("me"), py_str("myself"), py_str("my family")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-police-matter-in-the-family")); __args.push(py_tuple(vec![__data45(), __data46()])); __args.push(py_func("_police_matter", |args: &[Py]| _police_matter(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("custody of"), py_str("in custody of")])), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data47() -> Py { py_tuple(vec![py_str("lisp"), py_str("lisps"), py_str("lisping"), py_str("speech therapy"), py_str("speech therapist"), py_str("speech and language"), py_str("says s as th"), py_str("says th for s"), py_str("says th instead"), py_str("th for s"), py_str("f for th"), py_str("cant say his s"), py_str("cant say her s"), py_str("cant say r"), py_str("cant say his r"), py_str("cant say her r"), py_str("hard to understand"), py_str("pronounces"), py_str("pronunciation"), py_str("stammer"), py_str("stutter"), py_str("stutters"), py_str("stammering"), py_str("sounds"), py_str("sound"), py_str("wrong words"), py_str("words wrong"), py_str("cant say"), py_str("unclear"), py_str("unclearly"), py_str("mispronounces")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("child"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("toddler"), py_str("year old"), py_str("years old"), py_str("still"), py_str("should i worry"), py_str("worried"), py_str("normal"), py_str("help"), py_str("referral"), py_str("referred"), py_str("nursery"), py_str("school"), py_str("health visitor"), py_str("gp"), py_str("cant say"), py_str("cannot say"), py_str("hard to understand"), py_str("struggles"), py_str("struggling")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("adult"), py_str("myself"), py_str("my own"), py_str("i have"), py_str("ive had"), py_str("i lisp"), py_str("i stammer"), py_str("i stutter"), py_str("my speech"), py_str("battery"), py_str("piano"), py_str("guitar"), py_str("audio"), py_str("music"), py_str("recording"), py_str("podcast"), py_str("accent"), py_str("language learning"), py_str("french"), py_str("spanish"), py_str("welsh"), py_str("dog"), py_str("puppy"), py_str("cat"), py_str("parrot"), py_str("budgie"), py_str("car"), py_str("engine")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("child-lisp")); __args.push(py_tuple(vec![__data47(), __data48()])); __args.push(py_str("Most children lisp while they are learning their sounds and grow out of it on their own - the /s/ and /z/ sounds often take until about four and a half, and /r/ and /th/ until six or seven. Don't correct them or ask them to say it again: repeat the word back correctly in your own sentence and let it go. Ask the health visitor or the GP for a speech and language referral if the lisp is still there at four and a half, if strangers cannot understand them at three, if they are losing sounds they used to say, or if they are being teased or going quiet about it at nursery.")); __kws.extend(vec![("avoid".to_string(), __data49())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data50() -> Py { py_tuple(vec![py_str("cries every bedtime"), py_str("cries at bedtime"), py_str("crying at bedtime"), py_str("cries every night"), py_str("cries when i leave"), py_str("cries when i put him down"), py_str("cries when i put her down"), py_str("screams at bedtime"), py_str("bedtime is a battle"), py_str("wont go to bed"), py_str("wont settle at night"), py_str("wont settle at bedtime"), py_str("fight at bedtime"), py_str("bedtime battle"), py_str("hates bedtime"), py_str("cries himself to sleep"), py_str("cries herself to sleep"), py_str("cries every nap")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("child"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("toddler"), py_str("baby"), py_str("year old"), py_str("years old"), py_str("months"), py_str("every night"), py_str("every bedtime"), py_str("bedtime"), py_str("nap"), py_str("naps"), py_str("sleep"), py_str("bed"), py_str("cot"), py_str("how do i"), py_str("what do i do"), py_str("help"), py_str("stop"), py_str("settle")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("adult"), py_str("myself"), py_str("my own"), py_str("i cry"), py_str("i cant sleep"), py_str("insomnia"), py_str("puppy"), py_str("dog"), py_str("puppies"), py_str("dog crying"), py_str("cat"), py_str("kitten"), py_str("crate"), py_str("neighbour"), py_str("neighbours"), py_str("baby monitor"), py_str("teething"), py_str("colic"), py_str("reflux"), py_str("newborn")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("child-bedtime-tears")); __args.push(py_tuple(vec![__data50(), __data51()])); __args.push(py_str("Tears at bedtime are usually overtiredness or a routine that changes shape from night to night: the same four or five steps in the same order at the same time (bath, milk, teeth, two books, lights low, into bed), starting the wind-down half an hour before, with the last nap not too late for a toddler. Do the same short goodnight every time and leave while they are awake but sleepy; if they cry, go back after a minute, then two, then three, with a quiet 'it's sleep time, I love you' and no light and no play. It is normal for the first three or four nights to be worse before they are better, and see the GP if they also snore, stop breathing, or are exhausted in the daytime.")); __kws.extend(vec![("avoid".to_string(), __data52())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data53() -> Py { py_tuple(vec![py_str("anger problems"), py_str("anger issues"), py_str("always angry"), py_str("angry all the time"), py_str("so angry"), py_str("gets angry"), py_str("gets so angry"), py_str("angry at everything"), py_str("loses his temper"), py_str("loses her temper"), py_str("bad temper"), py_str("has a temper"), py_str("a temper"), py_str("rages"), py_str("raging"), py_str("explosive"), py_str("lashes out"), py_str("screams at me"), py_str("shouts at me"), py_str("throwing things"), py_str("throws things"), py_str("smashing things"), py_str("breaks things")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("child"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("toddler"), py_str("teenager"), py_str("year old"), py_str("years old"), py_str("always"), py_str("every day"), py_str("all the time"), py_str("help"), py_str("what can i do"), py_str("how do i"), py_str("worried"), py_str("worries me"), py_str("school"), py_str("at school"), py_str("at home"), py_str("behaviour")]) }
    fn __data55() -> Py { py_tuple(vec![py_str("i get"), py_str("i lose"), py_str("i shout"), py_str("i yell"), py_str("i scream"), py_str("i hit"), py_str("i am"), py_str("im"), py_str("at work"), py_str("my job"), py_str("my boss"), py_str("my office"), py_str("colleagues"), py_str("colleague"), py_str("shift"), py_str("commute"), py_str("adult"), py_str("myself"), py_str("my own"), py_str("i am angry"), py_str("im angry"), py_str("my anger"), py_str("my partner"), py_str("husband"), py_str("wife"), py_str("boss"), py_str("colleague"), py_str("friend"), py_str("road rage"), py_str("hits"), py_str("hitting"), py_str("hit me"), py_str("hits me"), py_str("bangs his head"), py_str("bangs her head"), py_str("self harm"), py_str("himself"), py_str("herself"), py_str("hurt himself"), py_str("hurt herself"), py_str("puppy"), py_str("dog"), py_str("cat")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("child-anger")); __args.push(py_tuple(vec![__data53(), __data54()])); __args.push(py_str("Anger in a child is usually the top of something else - hunger, tiredness, screens off too fast, or a day with no say in it - so name the feeling for them, stay calm yourself, and give the same short limit every time; a child who is shouting needs you quieter, not louder. Pick the moment later, when everyone is calm, to talk about what set it off and what they could do instead, and give them something physical to do with it (run, kick a ball, punch a cushion). Ask the school or the GP for help if it is most days, if someone is getting hurt, or if it has got worse over a few weeks rather than better.")); __kws.extend(vec![("avoid".to_string(), __data55())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data56() -> Py { py_tuple(vec![py_str("daughter"), py_str("son"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("my girl"), py_str("my boy"), py_str("year old"), py_str("13 year old"), py_str("14 year old"), py_str("15 year old"), py_str("16 year old"), py_str("17 year old"), py_str("is 13"), py_str("is 14"), py_str("is 15"), py_str("is 16"), py_str("is 17"), py_str("what age"), py_str("how old"), py_str("old enough"), py_str("legal age"), py_str("age limit"), py_str("is it legal"), py_str("whats the age"), py_str("minimum age"), py_str("too young"), py_str("under 18"), py_str("under 16"), py_str("legal"), py_str("illegal"), py_str("allowed"), py_str("against the law"), py_str("is it legal")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("hair"), py_str("dye"), py_str("hair dye"), py_str("school"), py_str("tattoo removal"), py_str("remove a tattoo"), py_str("laser"), py_str("my tattoo"), py_str("infected"), py_str("infection"), py_str("swollen"), py_str("my ear"), py_str("piercing"), py_str("piercings"), py_str("pierced"), py_str("pierce")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("daughter"), py_str("son"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("my girl"), py_str("my boy"), py_str("year old"), py_str("13 year old"), py_str("14 year old"), py_str("15 year old"), py_str("16 year old"), py_str("17 year old"), py_str("is 13"), py_str("is 14"), py_str("is 15"), py_str("is 16"), py_str("is 17"), py_str("what age"), py_str("how old"), py_str("old enough"), py_str("legal age"), py_str("age limit"), py_str("is it legal"), py_str("whats the age"), py_str("minimum age"), py_str("too young"), py_str("under 18"), py_str("under 16"), py_str("legal"), py_str("illegal"), py_str("allowed"), py_str("against the law"), py_str("is it legal")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("piercing"), py_str("piercings"), py_str("pierce"), py_str("pierced"), py_str("ears pierced"), py_str("ear pierced"), py_str("earrings"), py_str("belly button"), py_str("nose ring"), py_str("navel"), py_str("helix"), py_str("tragus"), py_str("lobe"), py_str("lobes")]) }
    let __aug60 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-teenager-wants-a-tattoo")); __args.push(py_tuple(vec![__data56(), py_tuple(vec![py_str("tattoo"), py_str("tattoos"), py_str("tattooed"), py_str("tattooing")])])); __args.push(py_func("_a_teenagers_tattoo", |args: &[Py]| _a_teenagers_tattoo(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("my"), py_str("can"), py_str("should"), py_str("is"), py_str("what"), py_str("how")])), ("avoid".to_string(), __data57())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-teenager-wants-a-piercing")); __args.push(py_tuple(vec![__data58(), __data59()])); __args.push(py_func("_a_teenagers_piercing", |args: &[Py]| _a_teenagers_piercing(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("my"), py_str("can"), py_str("should"), py_str("is"), py_str("what"), py_str("how")])), ("avoid".to_string(), py_tuple(vec![py_str("hair"), py_str("dye"), py_str("hair dye"), py_str("school"), py_str("infected"), py_str("infection"), py_str("swollen"), py_str("nosebleed"), py_str("my ear hurts"), py_str("tattoo"), py_str("tattoos")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.family_more.ENTRIES"); }
    s_ENTRIES(__aug60);
}
pub fn _another_country_is_in_the_line(args: &[Py]) -> Py {
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
    let mut geography: Py = Py::None;
    let mut low: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("True when the line says the person is going to some other country, not just away.\n\n    A country's name is the plainest way to say it, so the names the knowledge base already\n    holds are the test rather than a second list to keep. \"moving to\" with a town in it is a\n    move down the road: the line has to name somewhere in another country, or say `abroad`,\n    `overseas` or `emigrating`.\n    ");
    low = op_add(&op_add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![mcall(&understood.clone(), "get", &[py_str("text")]), mcall(&understood.clone(), "get", &[py_str("clean")])])) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[])), &py_str(" "));
    if py_truthy(&mcall(&g__LEFT_THE_COUNTRY(), "search", &[low.clone()])) {
    return Py::Bool(true);
    }
    geography = pyrt::xrefs::xmodule("knowledge.geography");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&geography.clone(), "COUNTRIES")) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_child_is_not_grown(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when the child the line is about has a grown-up's age or life on them.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    text = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GROWN_UP_SAID()) { let mut said: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &said.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__ADULT_AGE(), "search", &[text.clone()])));
    Py::None
}
pub fn _nappies_a_day(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("How many nappies a day, at the age the line names.");
    said = op_add(&op_add(&py_str(" "), &mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g__NAPPY_AGES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    ages = __unpk2[0].clone();
    answer = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&ages.clone());
    for __item in __iter3 {
    age = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &age.clone())))) {
    return answer.clone();
    }
    }
    }
    return py_str("A newborn gets through eight to ten a day - about seventy a week, so two big packs a month or a subscription. It drops to around six to seven a day by six months and four or five by a year. Buy one pack of newborn size and one of the next size up: they grow out of newborn in a fortnight, and a size that leaks is usually too small.");
    Py::None
}
pub fn _not_a_nursing_career(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when the line is about the job rather than about feeding a baby.");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_NURSING_CAREER()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&text.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { mcall(&mcall(&text.clone(), "strip", &[]), "endswith", &[word.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _breastfeeding_basics(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    clean = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BREAST_COULD_NOT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("No - a baby who is fed is fed, and how they were fed is not something you owe anyone an apology for. If it did not work, that is usually a latch, a supply or a medical reason rather than effort, and formula is a complete food: the health visitor will help you swap over or mix the two without a fuss. If the guilt is sticking, say so to your midwife or GP - it is common, it has a name, and it is treatable.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BREAST_PUBLIC()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Yes - you can feed your baby wherever you happen to be, and in the UK the law protects it: nobody can ask you to stop, move on or leave because you are breastfeeding. A scarf or a layered top is plenty if you want cover, and if you would rather have a quiet corner, a cafe, library or shopping centre will nearly always find you one - ask the staff.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BREAST_PAIN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("It should not hurt beyond the first few seconds of a feed, so get help early rather than pushing through: a midwife, health visitor or a local breastfeeding clinic will watch a whole feed and fix the latch, which is the cause most of the time. Cracked or bleeding nipples, a red painful patch on the breast with a fever, or a baby who stops feeding are the reasons to be seen the same day.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BREAST_ILL()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Yes - keep feeding through it: your milk carries the antibodies you are making against whatever it is, so it is the best protection the baby can get, and stopping suddenly is what causes mastitis and a dip in supply. Drink more than you feel like and feed lying down if you are wiped out, and check any medicine with a pharmacist first - most are fine.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BREAST_LENGTH()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Only breast milk for about the first six months, then keep going alongside solid food for as long as you both want - the UK advice is to two years and beyond, and plenty of babies stop before that. There is no point where the milk stops being worth it, and no rule you have to keep to if it is not working for you.");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str(" formula"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str(" bottle "))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str(" bottles "))) } } } }) {
    return py_str("Formula is a safe, complete food and a baby fed formula is a baby fed well - the only thing it does not carry is the antibodies that come in your milk, which is why breast milk is the default advice rather than the only good choice. Mixed feeding works too, and a baby on both gets the antibodies from the feeds you do give. What matters is that the baby is growing and that feeding is not miserable for you.");
    }
    return py_str("Yes - it is the normal way to feed a baby and it is what the milk is for. Feed on demand rather than by the clock: a newborn feeds eight to twelve times in 24 hours, and six or more wet nappies a day with steady weight gain say it is going in. If it hurts, the baby is sleepy at the breast or not putting on weight, a midwife or health visitor will see you the same week.");
    Py::None
}
pub fn _said_date(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut datetime: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A date said the way a person says it: \"Tuesday 21 July\", with the year when it is not the\n    one we are in, because a break-up date said in November is next year's.");
    datetime = stdlib_module("datetime");
    said = op_modulo(&py_str("%s %d %s"), &py_tuple(vec![mcall(&found.clone(), "strftime", &[py_str("%A")]), gattr(&found.clone(), "day"), mcall(&found.clone(), "strftime", &[py_str("%B")])]));
    if py_truthy(&Py::Bool(!py_eq(&gattr(&found.clone(), "year"), &gattr(&mcall(&stdlib_attr("datetime", "date").unwrap_or_else(|e| exc::raise(e)), "today", &[]), "year")))) {
    let __aug1 = op_add(&said.clone(), &op_modulo(&py_str(" %d"), &gattr(&found.clone(), "year")));
    said = __aug1;
    }
    return said.clone();
    Py::None
}
pub fn _school_date(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut month: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut day: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut datetime: Py = Py::None;
    let mut found: Py = Py::None;
    let mut today: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The usual date of a school break, this year if it has not been and gone, else next year.");
    datetime = stdlib_module("datetime");
    today = mcall(&stdlib_attr("datetime", "date").unwrap_or_else(|e| exc::raise(e)), "today", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![gattr(&today.clone(), "year"), op_add(&gattr(&today.clone(), "year"), &Py::Int(1))]));
    for __item in __iter1 {
    year = __item.clone();
    found = mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), day.clone()]);
    if py_truthy(&op_ge(&found.clone(), &today.clone())) {
    return found.clone();
    }
    }
    return mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), month.clone(), day.clone()]);
    Py::None
}
pub fn _this_or_next(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut datetime: Py = Py::None;
    let _ = py_str("How the date is introduced: \"this year that is Wednesday 21 July\", or \"the next one is\".");
    datetime = stdlib_module("datetime");
    if py_truthy(&Py::Bool(py_eq(&gattr(&found.clone(), "year"), &gattr(&mcall(&stdlib_attr("datetime", "date").unwrap_or_else(|e| exc::raise(e)), "today", &[]), "year")))) {
    return op_modulo(&py_str("this year that is %s"), &_said_date(&[found.clone()]));
    }
    return op_modulo(&py_str("the next one is %s"), &_said_date(&[found.clone()]));
    Py::None
}
pub fn _about_days(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut days: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "days") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: days"))) };
    let mut weeks: Py = Py::None;
    let _ = py_str("\"36 days (five weeks)\" - the count in days, and in weeks when it is worth saying.");
    if py_truthy(&op_le(&days.clone(), &Py::Int(10))) {
    return op_modulo(&py_str("%d days"), &days.clone());
    }
    weeks = call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return op_modulo(&py_str("%d days (about %d weeks)"), &py_tuple(vec![days.clone(), weeks.clone()]));
    Py::None
}
pub fn _asks_the_length(args: &[Py]) -> Py {
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
    let _ = py_str("\"How long are the summer holidays\" wants six weeks, not the days until they start.");
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bhow long (?:is|are|do|does|will)\\b|\\bhow many weeks off\\b|\\bhow long (?:a|an|the)\\b|\\blast(?:s|ed)?\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _counts_the_wait(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line wants the days until a break rather than when it is or how long.");
    if py_truthy(&_asks_the_length(&[clean.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:how (?:long|many)|days|weeks|till|until|countdown|left)\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_nation_said(args: &[Py]) -> Py {
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
    let mut nation: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The nation whose school year the line is asking about, or \"\" when it names none.");
    let __iter1 = py_iter_or_raise(&g__NATION_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    nation = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return nation.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_nations_year(args: &[Py]) -> Py {
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
    let mut nation: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The school year of the nation the chat is in, or None for a chat that names none.\n\n    A line that names no nation is still in one if an earlier line did: a chat that said Glasgow\n    two turns ago is a Glasgow chat, and its next question about term dates is about Scotland.\n    ");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    nation = _the_nation_said(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&nation.clone())); if !py_truthy(&__b) { __b } else { memory.clone() } }) {
    recent = py_func("recent", |args: &[Py]| pyrt::xrefs::xcall("memory", "recent", args));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[py_call(&recent, &[memory.clone(), Py::Int(6)])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    nation = _the_nation_said(&[op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "))]);
    if py_truthy(&nation.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&nation.clone()))) {
    return Py::None;
    }
    said = mcall(&g__SCHOOL_YEAR(), "get", &[nation.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return op_add(&said.clone(), &g__THE_DATES_ARE_LOCAL());
    Py::None
}
pub fn _summer_break(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut there: Py = Py::None;
    let mut usual: Py = Py::None;
    let _ = py_str("When the summer holidays start, or how long it is until they do.");
    there = _the_nations_year(&[memory.clone(), understood.clone()]);
    if py_truthy(&there.clone()) {
    return there.clone();
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    usual = _school_date(&[Py::Int(7), Py::Int(21)]);
    if py_truthy(&_asks_the_length(&[clean.clone()])) {
    return op_modulo(&py_str("Six weeks off: %s. The exact two days are on your school's term dates, because a school can start back a day or two either side."), &g_SUMMER_BREAK());
    }
    if py_truthy(&_counts_the_wait(&[clean.clone()])) {
    datetime = stdlib_module("datetime");
    days = gattr(&op_sub(&usual.clone(), &mcall(&stdlib_attr("datetime", "date").unwrap_or_else(|e| exc::raise(e)), "today", &[])), "days");
    return op_modulo(&py_str("%s until the summer holidays: %s, so they break up on %s. The exact day is on your school's term dates, and it can sit a day or two either side."), &py_tuple(vec![_about_days(&[days.clone()]), g_SUMMER_BREAK(), _said_date(&[usual.clone()])]));
    }
    return op_modulo(&py_str("%s - %s. The exact day is on your school's term dates, and it can sit a day or two either side."), &py_tuple(vec![g_SUMMER_BREAK(), _this_or_next(&[usual.clone()])]));
    Py::None
}
pub fn _winter_break(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut there: Py = Py::None;
    let mut usual: Py = Py::None;
    let _ = py_str("The Christmas holidays, and the count to them when the line asks how long.");
    there = _the_nations_year(&[memory.clone(), understood.clone()]);
    if py_truthy(&there.clone()) {
    return there.clone();
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    usual = _school_date(&[Py::Int(12), Py::Int(19)]);
    if py_truthy(&_counts_the_wait(&[clean.clone()])) {
    datetime = stdlib_module("datetime");
    days = gattr(&op_sub(&usual.clone(), &mcall(&stdlib_attr("datetime", "date").unwrap_or_else(|e| exc::raise(e)), "today", &[])), "days");
    return op_modulo(&py_str("%s until the Christmas holidays: schools usually break up on about 19 December, and %s, then go back the first Tuesday or Wednesday in January."), &py_tuple(vec![_about_days(&[days.clone()]), _this_or_next(&[usual.clone()])]));
    }
    return op_modulo(&py_str("%s: the break-up is usually about 19 December, %s, and back on the first Tuesday or Wednesday in January. Your school's term dates give the exact two days."), &py_tuple(vec![g_WINTER_BREAK(), _this_or_next(&[usual.clone()])]));
    Py::None
}
pub fn _easter_break(args: &[Py]) -> Py {
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
    let mut there: Py = Py::None;
    let _ = py_str("The Easter holidays, which follow Easter weekend rather than a fixed date.");
    there = _the_nations_year(&[memory.clone(), understood.clone()]);
    if py_truthy(&there.clone()) {
    return there.clone();
    }
    return op_modulo(&py_str("%s. Easter moves each year, so the two weeks move with it - your school's term dates name the Friday it breaks up and the Monday it goes back."), &g_EASTER_BREAK());
    Py::None
}
pub fn _half_term(args: &[Py]) -> Py {
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
    let mut there: Py = Py::None;
    let _ = py_str("Half term: a week in October, February and May, on the school's own dates.");
    there = _the_nations_year(&[memory.clone(), understood.clone()]);
    if py_truthy(&there.clone()) {
    return there.clone();
    }
    return py_str("Half term is one week off: the last week of October, the middle of February and the last week of May. Which week it falls in is set by your school, and the three dates for the year are on its term dates page - the October one is usually the week the clocks go back.");
    Py::None
}
pub fn _back_to_school(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut there: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The day term starts: the term the line named, or all three returns when it names none.");
    there = _the_nations_year(&[memory.clone(), understood.clone()]);
    if py_truthy(&there.clone()) {
    return there.clone();
    }
    clean = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    let __iter1 = py_iter_or_raise(&g__TERM_STARTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_modulo(&py_str("%s. Your school's term dates give the exact day, and a school can add an inset day at the start of term."), &said.clone());
    }
    }
    return py_str("The summer return is the first days of September, after Christmas it is the first Tuesday or Wednesday in January, and after Easter it is the Monday a week after Easter Monday. Your school's term dates give the exact day, and a school can add an inset day at the start of term.");
    Py::None
}
pub fn _school_holidays_general(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut soon: Py = Py::None;
    let mut summer: Py = Py::None;
    let mut there: Py = Py::None;
    let mut today: Py = Py::None;
    let mut winter: Py = Py::None;
    let _ = py_str("The shape of the school year, for a line that names no particular break.\n\n    A line like \"when do the kids break up\" names no break at all, so the useful half of the\n    answer is the next one on the calendar, and the dates of the two that have a usual date.\n    ");
    there = _the_nations_year(&[memory.clone(), understood.clone()]);
    if py_truthy(&there.clone()) {
    return there.clone();
    }
    datetime = stdlib_module("datetime");
    today = mcall(&stdlib_attr("datetime", "date").unwrap_or_else(|e| exc::raise(e)), "today", &[]);
    summer = _school_date(&[Py::Int(7), Py::Int(21)]);
    winter = _school_date(&[Py::Int(12), Py::Int(19)]);
    if py_truthy(&op_lt(&winter.clone(), &summer.clone())) {
    soon = op_modulo(&py_str("the Christmas break, from about 19 December (%s) to the first days of January"), &py_tuple(vec![_said_date(&[winter.clone()])]));
    } else {
    soon = op_modulo(&py_str("the summer break, from the third week of July (%s) to the first days of September"), &py_tuple(vec![_said_date(&[summer.clone()])]));
    }
    if py_truthy(&_counts_the_wait(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    days = gattr(&op_sub(&if py_truthy(&op_lt(&winter.clone(), &summer.clone())) { winter.clone() } else { summer.clone() }, &today.clone()), "days");
    return op_modulo(&py_str("%s until the next break - %s. The exact dates come from your school or your council."), &py_tuple(vec![_about_days(&[days.clone()]), soon.clone()]));
    }
    return op_modulo(&py_str("The next one is %s. English state schools have six weeks off in summer, two at Christmas, two at Easter and a week's half term in October, February and May; the exact dates come from your school or your council."), &soon.clone());
    Py::None
}
pub fn _police_matter(args: &[Py]) -> Py {
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
    let _ = py_str("The first hour of an arrest, or what a stop, a search or a questioning allows.");
    said = mcall(&{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(questioned|questioning|interviewed|interview|under caution|asked questions|statement|appropriate adult)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return g_THE_QUESTIONED();
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(stopped|stop and search|searched|searches|search|handcuffed|detained|frisked)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return g_THE_STOPPED();
    }
    return g_THE_FIRST_HOUR();
    Py::None
}
pub fn _the_age_in_the_line(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("\"a 15 year old\" in the line, or None.\n\n    The two cards below are about a rule with an age in it, and a card that says \"14\" to a line\n    about a 16 year old is a different child and a different rule. So the age the parent gave is\n    the one the answer names, and the fallback is the rule's own wording (\"under 18\") rather than\n    a guess at the child.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str(" ") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    pattern = py_str("\\b(1[0-9]|[89])\\s*(?:year|yr)s?\\s*old\\b|\\bis\\s+(1[0-9]|[89])\\b|\\bat\\s+(1[0-9]|[89])\\b|\\baged\\s+(1[0-9]|[89])\\b");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    pair = __item.clone();
    let __iter2 = py_iter_or_raise(&pair.clone());
    for __item in __iter2 {
    one = __item.clone();
    if py_truthy(&one.clone()) {
    return one.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_teenagers_tattoo(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The tattoo rule first, because a tattoo is what the line asked about.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str(" ") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    age = _the_age_in_the_line(&[understood.clone()]);
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b18th\\b|\\b(?:is|turns|turning|she is|he is)\\s+18\\b|\\b18\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&age.clone())) } }) {
    return py_str("At 18 it is her own decision and no consent is needed, so the useful thing now is which studio rather than whether: licensed only, ask to see the licence and the sterile sealed needles opened in front of you, and look at healed work rather than the photographs they choose. A design drawn now and lived with for a few months is what saves the cover-up later. A piercing is a different rule: 16 without consent for lobes and most placements, and a parent with ID below that.");
    }
    if py_truthy(&age.clone()) {
    who = op_modulo(&py_str("In the UK a tattoo is 18 with no exceptions, so a %s year old cannot be tattooed at all - and a studio that would do one is breaking the law and is the studio to avoid"), &age.clone());
    } else {
    who = py_str("In the UK a tattoo is 18 with no exceptions, so nobody under 18 can be tattooed at all - and a studio that would do one is breaking the law and is the studio to avoid");
    }
    return op_add(&who.clone(), &py_str(". What you can say yes to now is the design: have them draw it and live with it on paper for a year, because a good studio will still be there at 18. Piercings are the smaller ask, and a different rule: 16 without consent for lobes and most other placements, and under that a parent is usually there with ID, so ring the shop first because the rules differ by council."));
    Py::None
}
pub fn _a_teenagers_piercing(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut does: Py = Py::None;
    let mut he: Py = Py::None;
    let mut him: Py = Py::None;
    let mut his: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The piercing rule first: the person asked about an ear, not about a tattoo.");
    age = _the_age_in_the_line(&[understood.clone()]);
    let __unpk1 = py_unpack(&pyrt::xrefs::xcall("knowledge.family", "_the_child_pronouns", &[memory.clone(), understood.clone()]), 4, None);
    he = __unpk1[0].clone();
    him = __unpk1[1].clone();
    his = __unpk1[2].clone();
    does = __unpk1[3].clone();
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&age.clone(), "isdigit", &[]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("int", &[age.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16)) } } } }) {
    lead = op_modulo(&py_str("At %s %s can have lobes and most other placements pierced without a parent's consent, so this one is up to %s"), &py_tuple(vec![age.clone(), he.clone(), him.clone()]));
    } else {
    if py_truthy(&age.clone()) {
    lead = op_modulo(&py_str("At %s a piercing needs you there: most studios want a parent present with ID, and many will only do lobes below 16"), &age.clone());
    } else {
    lead = py_str("Under 16 a piercing usually needs a parent there with ID, and 16 is when lobes and most other placements stop needing consent");
    }
    }
    return op_add(&lead.clone(), &py_str(". Ring the shop before you go, because the age rules and the placements allowed differ by council, and only a licensed studio is worth using: gloves, sterile sealed needles, a hollow needle rather than a gun for anything but lobes, and written aftercare. A nose ring or a navel heals for months and wants cleaning twice a day, so agree who does that before the appointment. A tattoo is a different rule: 18, no exceptions."));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.family_more".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "EASTER_BREAK" => Some(g_EASTER_BREAK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "SUMMER_BREAK" => Some(g_SUMMER_BREAK()),
            "THE_FIRST_HOUR" => Some(g_THE_FIRST_HOUR()),
            "THE_QUESTIONED" => Some(g_THE_QUESTIONED()),
            "THE_STOPPED" => Some(g_THE_STOPPED()),
            "WINTER_BREAK" => Some(g_WINTER_BREAK()),
            "_ADULT_AGE" => Some(g__ADULT_AGE()),
            "_ADULT_RELATION" => Some(g__ADULT_RELATION()),
            "_AGE_FIGURE" => Some(g__AGE_FIGURE()),
            "_AGE_SPELLED" => Some(g__AGE_SPELLED()),
            "_A_NAPPY_BAG" => Some(g__A_NAPPY_BAG()),
            "_A_NURSING_CAREER" => Some(g__A_NURSING_CAREER()),
            "_BREAST_COULD_NOT" => Some(g__BREAST_COULD_NOT()),
            "_BREAST_ILL" => Some(g__BREAST_ILL()),
            "_BREAST_LENGTH" => Some(g__BREAST_LENGTH()),
            "_BREAST_PAIN" => Some(g__BREAST_PAIN()),
            "_BREAST_PUBLIC" => Some(g__BREAST_PUBLIC()),
            "_GROWN_UP_SAID" => Some(g__GROWN_UP_SAID()),
            "_LEFT_THE_COUNTRY" => Some(g__LEFT_THE_COUNTRY()),
            "_NAPPY_AGES" => Some(g__NAPPY_AGES()),
            "_NATION_WORDS" => Some(g__NATION_WORDS()),
            "_NOT_A_UNIT" => Some(g__NOT_A_UNIT()),
            "_SCHOOL_TIME_ASK" => Some(g__SCHOOL_TIME_ASK()),
            "_SCHOOL_WORDS" => Some(g__SCHOOL_WORDS()),
            "_SCHOOL_YEAR" => Some(g__SCHOOL_YEAR()),
            "_TERM_STARTS" => Some(g__TERM_STARTS()),
            "_THE_DATES_ARE_LOCAL" => Some(g__THE_DATES_ARE_LOCAL()),
            "_TRAVEL_HOLIDAY" => Some(g__TRAVEL_HOLIDAY()),
            "_a_phone_rule_and_an_age" => Some(g__a_phone_rule_and_an_age()),
            "_a_teenager_is_named" => Some(g__a_teenager_is_named()),
            "_a_teenagers_piercing" => Some(py_func("_a_teenagers_piercing", |args: &[Py]| _a_teenagers_piercing(args))),
            "_a_teenagers_tattoo" => Some(py_func("_a_teenagers_tattoo", |args: &[Py]| _a_teenagers_tattoo(args))),
            "_about_days" => Some(py_func("_about_days", |args: &[Py]| _about_days(args))),
            "_another_country_is_in_the_line" => Some(py_func("_another_country_is_in_the_line", |args: &[Py]| _another_country_is_in_the_line(args))),
            "_asks_the_length" => Some(py_func("_asks_the_length", |args: &[Py]| _asks_the_length(args))),
            "_back_to_school" => Some(py_func("_back_to_school", |args: &[Py]| _back_to_school(args))),
            "_breastfeeding_basics" => Some(py_func("_breastfeeding_basics", |args: &[Py]| _breastfeeding_basics(args))),
            "_counts_the_wait" => Some(py_func("_counts_the_wait", |args: &[Py]| _counts_the_wait(args))),
            "_easter_break" => Some(py_func("_easter_break", |args: &[Py]| _easter_break(args))),
            "_half_term" => Some(py_func("_half_term", |args: &[Py]| _half_term(args))),
            "_nappies_a_day" => Some(py_func("_nappies_a_day", |args: &[Py]| _nappies_a_day(args))),
            "_not_a_nursing_career" => Some(py_func("_not_a_nursing_career", |args: &[Py]| _not_a_nursing_career(args))),
            "_police_matter" => Some(py_func("_police_matter", |args: &[Py]| _police_matter(args))),
            "_said_date" => Some(py_func("_said_date", |args: &[Py]| _said_date(args))),
            "_school_date" => Some(py_func("_school_date", |args: &[Py]| _school_date(args))),
            "_school_holidays_general" => Some(py_func("_school_holidays_general", |args: &[Py]| _school_holidays_general(args))),
            "_summer_break" => Some(py_func("_summer_break", |args: &[Py]| _summer_break(args))),
            "_the_age_in_the_line" => Some(py_func("_the_age_in_the_line", |args: &[Py]| _the_age_in_the_line(args))),
            "_the_child_is_not_grown" => Some(py_func("_the_child_is_not_grown", |args: &[Py]| _the_child_is_not_grown(args))),
            "_the_child_pronouns" => Some(g__the_child_pronouns()),
            "_the_nation_said" => Some(py_func("_the_nation_said", |args: &[Py]| _the_nation_said(args))),
            "_the_nations_year" => Some(py_func("_the_nations_year", |args: &[Py]| _the_nations_year(args))),
            "_this_or_next" => Some(py_func("_this_or_next", |args: &[Py]| _this_or_next(args))),
            "_winter_break" => Some(py_func("_winter_break", |args: &[Py]| _winter_break(args))),
            "entry" => Some(g_entry()),
            "datetime" => Some(stdlib_module("datetime")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "EASTER_BREAK" => s_EASTER_BREAK(v),
            "ENTRIES" => s_ENTRIES(v),
            "SUMMER_BREAK" => s_SUMMER_BREAK(v),
            "THE_FIRST_HOUR" => s_THE_FIRST_HOUR(v),
            "THE_QUESTIONED" => s_THE_QUESTIONED(v),
            "THE_STOPPED" => s_THE_STOPPED(v),
            "WINTER_BREAK" => s_WINTER_BREAK(v),
            "_ADULT_AGE" => s__ADULT_AGE(v),
            "_ADULT_RELATION" => s__ADULT_RELATION(v),
            "_AGE_FIGURE" => s__AGE_FIGURE(v),
            "_AGE_SPELLED" => s__AGE_SPELLED(v),
            "_A_NAPPY_BAG" => s__A_NAPPY_BAG(v),
            "_A_NURSING_CAREER" => s__A_NURSING_CAREER(v),
            "_BREAST_COULD_NOT" => s__BREAST_COULD_NOT(v),
            "_BREAST_ILL" => s__BREAST_ILL(v),
            "_BREAST_LENGTH" => s__BREAST_LENGTH(v),
            "_BREAST_PAIN" => s__BREAST_PAIN(v),
            "_BREAST_PUBLIC" => s__BREAST_PUBLIC(v),
            "_GROWN_UP_SAID" => s__GROWN_UP_SAID(v),
            "_LEFT_THE_COUNTRY" => s__LEFT_THE_COUNTRY(v),
            "_NAPPY_AGES" => s__NAPPY_AGES(v),
            "_NATION_WORDS" => s__NATION_WORDS(v),
            "_NOT_A_UNIT" => s__NOT_A_UNIT(v),
            "_SCHOOL_TIME_ASK" => s__SCHOOL_TIME_ASK(v),
            "_SCHOOL_WORDS" => s__SCHOOL_WORDS(v),
            "_SCHOOL_YEAR" => s__SCHOOL_YEAR(v),
            "_TERM_STARTS" => s__TERM_STARTS(v),
            "_THE_DATES_ARE_LOCAL" => s__THE_DATES_ARE_LOCAL(v),
            "_TRAVEL_HOLIDAY" => s__TRAVEL_HOLIDAY(v),
            "_a_phone_rule_and_an_age" => s__a_phone_rule_and_an_age(v),
            "_a_teenager_is_named" => s__a_teenager_is_named(v),
            "_the_child_pronouns" => s__the_child_pronouns(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.family_more", module_obj);
    pyrt::xrefs::reg_fn("knowledge.family_more", "_another_country_is_in_the_line", py_func("_another_country_is_in_the_line", |args: &[Py]| _another_country_is_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_the_child_is_not_grown", py_func("_the_child_is_not_grown", |args: &[Py]| _the_child_is_not_grown(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_nappies_a_day", py_func("_nappies_a_day", |args: &[Py]| _nappies_a_day(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_not_a_nursing_career", py_func("_not_a_nursing_career", |args: &[Py]| _not_a_nursing_career(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_breastfeeding_basics", py_func("_breastfeeding_basics", |args: &[Py]| _breastfeeding_basics(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_said_date", py_func("_said_date", |args: &[Py]| _said_date(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_school_date", py_func("_school_date", |args: &[Py]| _school_date(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_this_or_next", py_func("_this_or_next", |args: &[Py]| _this_or_next(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_about_days", py_func("_about_days", |args: &[Py]| _about_days(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_asks_the_length", py_func("_asks_the_length", |args: &[Py]| _asks_the_length(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_counts_the_wait", py_func("_counts_the_wait", |args: &[Py]| _counts_the_wait(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_the_nation_said", py_func("_the_nation_said", |args: &[Py]| _the_nation_said(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_the_nations_year", py_func("_the_nations_year", |args: &[Py]| _the_nations_year(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_summer_break", py_func("_summer_break", |args: &[Py]| _summer_break(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_winter_break", py_func("_winter_break", |args: &[Py]| _winter_break(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_easter_break", py_func("_easter_break", |args: &[Py]| _easter_break(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_half_term", py_func("_half_term", |args: &[Py]| _half_term(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_back_to_school", py_func("_back_to_school", |args: &[Py]| _back_to_school(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_school_holidays_general", py_func("_school_holidays_general", |args: &[Py]| _school_holidays_general(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_police_matter", py_func("_police_matter", |args: &[Py]| _police_matter(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_the_age_in_the_line", py_func("_the_age_in_the_line", |args: &[Py]| _the_age_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_a_teenagers_tattoo", py_func("_a_teenagers_tattoo", |args: &[Py]| _a_teenagers_tattoo(args)));
    pyrt::xrefs::reg_fn("knowledge.family_more", "_a_teenagers_piercing", py_func("_a_teenagers_piercing", |args: &[Py]| _a_teenagers_piercing(args)));
    pyrt::xrefs::reg_access("knowledge.family_more", "EASTER_BREAK", g_EASTER_BREAK, s_EASTER_BREAK);
    pyrt::xrefs::reg_access("knowledge.family_more", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.family_more", "SUMMER_BREAK", g_SUMMER_BREAK, s_SUMMER_BREAK);
    pyrt::xrefs::reg_access("knowledge.family_more", "THE_FIRST_HOUR", g_THE_FIRST_HOUR, s_THE_FIRST_HOUR);
    pyrt::xrefs::reg_access("knowledge.family_more", "THE_QUESTIONED", g_THE_QUESTIONED, s_THE_QUESTIONED);
    pyrt::xrefs::reg_access("knowledge.family_more", "THE_STOPPED", g_THE_STOPPED, s_THE_STOPPED);
    pyrt::xrefs::reg_access("knowledge.family_more", "WINTER_BREAK", g_WINTER_BREAK, s_WINTER_BREAK);
    pyrt::xrefs::reg_access("knowledge.family_more", "_ADULT_AGE", g__ADULT_AGE, s__ADULT_AGE);
    pyrt::xrefs::reg_access("knowledge.family_more", "_ADULT_RELATION", g__ADULT_RELATION, s__ADULT_RELATION);
    pyrt::xrefs::reg_access("knowledge.family_more", "_AGE_FIGURE", g__AGE_FIGURE, s__AGE_FIGURE);
    pyrt::xrefs::reg_access("knowledge.family_more", "_AGE_SPELLED", g__AGE_SPELLED, s__AGE_SPELLED);
    pyrt::xrefs::reg_access("knowledge.family_more", "_A_NAPPY_BAG", g__A_NAPPY_BAG, s__A_NAPPY_BAG);
    pyrt::xrefs::reg_access("knowledge.family_more", "_A_NURSING_CAREER", g__A_NURSING_CAREER, s__A_NURSING_CAREER);
    pyrt::xrefs::reg_access("knowledge.family_more", "_BREAST_COULD_NOT", g__BREAST_COULD_NOT, s__BREAST_COULD_NOT);
    pyrt::xrefs::reg_access("knowledge.family_more", "_BREAST_ILL", g__BREAST_ILL, s__BREAST_ILL);
    pyrt::xrefs::reg_access("knowledge.family_more", "_BREAST_LENGTH", g__BREAST_LENGTH, s__BREAST_LENGTH);
    pyrt::xrefs::reg_access("knowledge.family_more", "_BREAST_PAIN", g__BREAST_PAIN, s__BREAST_PAIN);
    pyrt::xrefs::reg_access("knowledge.family_more", "_BREAST_PUBLIC", g__BREAST_PUBLIC, s__BREAST_PUBLIC);
    pyrt::xrefs::reg_access("knowledge.family_more", "_GROWN_UP_SAID", g__GROWN_UP_SAID, s__GROWN_UP_SAID);
    pyrt::xrefs::reg_access("knowledge.family_more", "_LEFT_THE_COUNTRY", g__LEFT_THE_COUNTRY, s__LEFT_THE_COUNTRY);
    pyrt::xrefs::reg_access("knowledge.family_more", "_NAPPY_AGES", g__NAPPY_AGES, s__NAPPY_AGES);
    pyrt::xrefs::reg_access("knowledge.family_more", "_NATION_WORDS", g__NATION_WORDS, s__NATION_WORDS);
    pyrt::xrefs::reg_access("knowledge.family_more", "_NOT_A_UNIT", g__NOT_A_UNIT, s__NOT_A_UNIT);
    pyrt::xrefs::reg_access("knowledge.family_more", "_SCHOOL_TIME_ASK", g__SCHOOL_TIME_ASK, s__SCHOOL_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.family_more", "_SCHOOL_WORDS", g__SCHOOL_WORDS, s__SCHOOL_WORDS);
    pyrt::xrefs::reg_access("knowledge.family_more", "_SCHOOL_YEAR", g__SCHOOL_YEAR, s__SCHOOL_YEAR);
    pyrt::xrefs::reg_access("knowledge.family_more", "_TERM_STARTS", g__TERM_STARTS, s__TERM_STARTS);
    pyrt::xrefs::reg_access("knowledge.family_more", "_THE_DATES_ARE_LOCAL", g__THE_DATES_ARE_LOCAL, s__THE_DATES_ARE_LOCAL);
    pyrt::xrefs::reg_access("knowledge.family_more", "_TRAVEL_HOLIDAY", g__TRAVEL_HOLIDAY, s__TRAVEL_HOLIDAY);
    pyrt::xrefs::reg_access("knowledge.family_more", "_a_phone_rule_and_an_age", g__a_phone_rule_and_an_age, s__a_phone_rule_and_an_age);
    pyrt::xrefs::reg_access("knowledge.family_more", "_a_teenager_is_named", g__a_teenager_is_named, s__a_teenager_is_named);
    pyrt::xrefs::reg_access("knowledge.family_more", "_the_child_pronouns", g__the_child_pronouns, s__the_child_pronouns);
    pyrt::xrefs::reg_access("knowledge.family_more", "entry", g_entry, s_entry);
}
