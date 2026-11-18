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
thread_local! { static G_EXTRA_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEEL_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOUTH_TIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SMALL_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_HABITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_HABIT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_HABIT_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_HABIT_EXTRA_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP_SMALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORRY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANOTHER_ADULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANOTHER_PERSONS_MEMORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FORGOTTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MY_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PUTTING_A_CHARGE_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEBODY_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE_FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMETHING_FORGOTTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_thing_has_stopped: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_screens_are_somebody_elses: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_EXTRA_ENTRIES() -> Py { ensure_init(); G_EXTRA_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA_ENTRIES(v: Py) { ensure_init(); G_EXTRA_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FEEL_BAD() -> Py { ensure_init(); G_FEEL_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEEL_BAD(v: Py) { ensure_init(); G_FEEL_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_TIME_WORDS() -> Py { ensure_init(); G_HABIT_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_TIME_WORDS(v: Py) { ensure_init(); G_HABIT_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_WORDS() -> Py { ensure_init(); G_HABIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_WORDS(v: Py) { ensure_init(); G_HABIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOUTH_TIP() -> Py { ensure_init(); G_MOUTH_TIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOUTH_TIP(v: Py) { ensure_init(); G_MOUTH_TIP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SMALL_HABIT() -> Py { ensure_init(); G_NOT_A_SMALL_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SMALL_HABIT(v: Py) { ensure_init(); G_NOT_A_SMALL_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_SUBJECT() -> Py { ensure_init(); G_PET_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_SUBJECT(v: Py) { ensure_init(); G_PET_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_HABITS() -> Py { ensure_init(); G_SMALL_HABITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_HABITS(v: Py) { ensure_init(); G_SMALL_HABITS.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_HABIT_ASK() -> Py { ensure_init(); G_SMALL_HABIT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_HABIT_ASK(v: Py) { ensure_init(); G_SMALL_HABIT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_HABIT_ENTRIES() -> Py { ensure_init(); G_SMALL_HABIT_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_HABIT_ENTRIES(v: Py) { ensure_init(); G_SMALL_HABIT_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_HABIT_EXTRA_AVOID() -> Py { ensure_init(); G_SMALL_HABIT_EXTRA_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_HABIT_EXTRA_AVOID(v: Py) { ensure_init(); G_SMALL_HABIT_EXTRA_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP() -> Py { ensure_init(); G_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); G_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP_SMALL() -> Py { ensure_init(); G_STOP_SMALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP_SMALL(v: Py) { ensure_init(); G_STOP_SMALL.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PARTY() -> Py { ensure_init(); G_THIRD_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PARTY(v: Py) { ensure_init(); G_THIRD_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g_WORRY_ASK() -> Py { ensure_init(); G_WORRY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORRY_ASK(v: Py) { ensure_init(); G_WORRY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ANOTHER_ADULT() -> Py { ensure_init(); G__ANOTHER_ADULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANOTHER_ADULT(v: Py) { ensure_init(); G__ANOTHER_ADULT.with(|c| *c.borrow_mut() = v); }
pub fn g__ANOTHER_PERSONS_MEMORY() -> Py { ensure_init(); G__ANOTHER_PERSONS_MEMORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANOTHER_PERSONS_MEMORY(v: Py) { ensure_init(); G__ANOTHER_PERSONS_MEMORY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RELATIVE() -> Py { ensure_init(); G__A_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RELATIVE(v: Py) { ensure_init(); G__A_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__FORGETTING() -> Py { ensure_init(); G__FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FORGETTING(v: Py) { ensure_init(); G__FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g__FORGOTTEN() -> Py { ensure_init(); G__FORGOTTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FORGOTTEN(v: Py) { ensure_init(); G__FORGOTTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__MY_AGE() -> Py { ensure_init(); G__MY_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MY_AGE(v: Py) { ensure_init(); G__MY_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_ANY() -> Py { ensure_init(); G__PET_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_ANY(v: Py) { ensure_init(); G__PET_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g__PUTTING_A_CHARGE_UP() -> Py { ensure_init(); G__PUTTING_A_CHARGE_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PUTTING_A_CHARGE_UP(v: Py) { ensure_init(); G__PUTTING_A_CHARGE_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEBODY_ELSE() -> Py { ensure_init(); G__SOMEBODY_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEBODY_ELSE(v: Py) { ensure_init(); G__SOMEBODY_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE_FORGETTING() -> Py { ensure_init(); G__SOMEONE_ELSE_FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE_FORGETTING(v: Py) { ensure_init(); G__SOMEONE_ELSE_FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMETHING_FORGOTTEN() -> Py { ensure_init(); G__SOMETHING_FORGOTTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMETHING_FORGOTTEN(v: Py) { ensure_init(); G__SOMETHING_FORGOTTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_a_thing_has_stopped() -> Py { ensure_init(); G_a_thing_has_stopped.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_thing_has_stopped(v: Py) { ensure_init(); G_a_thing_has_stopped.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_screens_are_somebody_elses() -> Py { ensure_init(); G_screens_are_somebody_elses.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_screens_are_somebody_elses(v: Py) { ensure_init(); G_screens_are_somebody_elses.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Stopping something: vaping, drinking. The ask is always \"how do I stop\".\n\nThe smoking half lives in `science.py` (#1413, entry `quit-smoking`), which is approved and\nanswers `how do i quit smoking` with the same method; this table answers the two habits that\nwere still a shrug and must not take the smoking line back.\n\nThese are the everyday habits people decide to give up on a Monday, and the answer is not\na lecture: it is the one method that is known to work, how long the bad part lasts, and\nwhat to do when it does. Each entry needs the habit and a word for stopping, so\n\"does smoking cause cancer\" and \"how many units in a pint\" stay with the entries that\nanswer them, and \"i spilled beer on my shirt\" stays with the stain answer.\n\nThe avoid lists hold whole phrases (\"smoked salmon\", \"smoke alarm\") rather than the bare\nword, because an avoid matches the inflections of what it lists: avoiding \"smoked\" also\nthrows away \"smoking\", which is the line every entry here is for.\n");
    s_a_thing_has_stopped(py_func("a_thing_has_stopped", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "a_thing_has_stopped", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_screens_are_somebody_elses(py_func("screens_are_somebody_elses", |args: &[Py]| pyrt::xrefs::xcall("knowledge.wording", "screens_are_somebody_elses", args)));
    fn __data1() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("pup"), py_str("pups"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("rabbit"), py_str("rabbits"), py_str("bunny"), py_str("bunnies"), py_str("hamster"), py_str("hamsters"), py_str("guinea pig"), py_str("guinea pigs"), py_str("gerbil"), py_str("gerbils"), py_str("ferret"), py_str("ferrets"), py_str("parrot"), py_str("parrots"), py_str("bird"), py_str("birds"), py_str("horse"), py_str("horses"), py_str("pony"), py_str("ponies"), py_str("tortoise"), py_str("tortoises"), py_str("pet"), py_str("pets"), py_str("litter tray")]) }
    s_PET_SUBJECT(__data1());
    s__PET_ANY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PET_SUBJECT()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOUTH_TIP(py_str("Give the mouth or the fingers a stand-in: gum, a straw to chew, a pen you cannot bite to pieces, a bead on a chain. It happens at a desk or in front of a screen, so put the substitute where you sit and move the pen or the mirror out of reach."));
    s__ANOTHER_ADULT(py_str("(?:mum|mom|mam|mother|dad|father|parent|parents|husband|wife|partner|spouse|other half|nan|nana|gran|granny|grandma|grandad|grandpa|grandmother|grandfather|son|daughter|child|kid|brother|sister|aunt|auntie|uncle|cousin|in law|mother in law|father in law|friend|neighbour|neighbor|boss|colleague|coworker|flatmate|housemate|landlord|carer|pal|chum|mate|nephew|niece|grandchild|grandson|granddaughter|fiance|fiancee|stepmum|stepdad|sister in law|brother in law|missus|other half|girlfriend|boyfriend)"));
    s__FORGOTTEN(py_str("(?:forget|forgets|forgetting|forgot|forgotten|forgetful|forgetfulness|memory|remember|remembers|repeating|repeats|confused|confusion|getting confused|scatty|absent[- ]minded|away with it)"));
    s__SOMEONE_ELSE_FORGETTING(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:my|his|her|our|their|the)\\s+(?:\\w+\\s+){0,2}"), &g__ANOTHER_ADULT()), &py_str("\\b[^.!?]{0,30}?\\b")), &g__FORGOTTEN()), &py_str("\\b|\\b(?:he|she|they)\\s+(?:keeps?|is|are|has|have|always|never|used to)\\b[^.!?]{0,25}?\\b")), &g__FORGOTTEN()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__MY_AGE(stdlib_call("re", "compile", &[py_str("\\b(?:im|i am|i'm)\\s+(?:nearly |almost |just |only )?(?:\\d{2}|forty|fifty|sixty|seventy|eighty|ninety)\\b|\\bat my age\\b|\\bmy age\\b|\\bgetting older\\b|\\bas i get older\\b|\\bsince i turned\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FORGETTING(stdlib_call("re", "compile", &[py_str("\\b(?:forget|forgets|forgetting|forgot|forgotten|forgetful|memory|remember)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_RELATIVE(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our)\\s+(?:mum|mom|mother|dad|father|parents|gran|granny|nan|nana|grandma|grandmother|grandad|grandpa|grandfather|grandparents?|wife|husband|partner|spouse|other half|son|daughter|brother|sister|aunt|auntie|uncle|cousin|boyfriend|girlfriend|friend|mate|neighbour|neighbor|mother in law|father in law)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMEBODY_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they)\\s+(?:keeps?\\s+forgetting|forgets|is\\s+forgetful|cant\\s+remember|can't\\s+remember|cant\\s+recall|can't\\s+recall|repeats|is\\s+confused)")]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMETHING_FORGOTTEN(stdlib_call("re", "compile", &[py_str("\\b(?:forgett?ing|forgets|forgetful|remember|remembers|recall|repeat(?:s|ing)?|confused|gets lost|losing|loses)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FEEL_BAD(py_tuple(vec![py_str("feel awful"), py_str("feel terrible"), py_str("feel horrible"), py_str("feel like giving up"), py_str("i feel rubbish"), py_str("feel so low"), py_str("feel depressed")]));
    fn __data2() -> Py { py_tuple(vec![py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my boyfriend"), py_str("my girlfriend"), py_str("my mum"), py_str("my dad"), py_str("my parents"), py_str("my son"), py_str("my daughter"), py_str("my brother"), py_str("my sister"), py_str("my friend"), py_str("my friends"), py_str("my nan"), py_str("my gran"), py_str("my granny"), py_str("my grandma"), py_str("my grandad"), py_str("my grandpa"), py_str("my grandparent"), py_str("my uncle"), py_str("my aunt"), py_str("my auntie"), py_str("my cousin"), py_str("my colleague"), py_str("my mate"), py_str("my mother in law"), py_str("my father in law"), py_str("my flatmate"), py_str("help my"), py_str("he wants to"), py_str("she wants to"), py_str("he keeps"), py_str("she keeps")]) }
    s_THIRD_PARTY(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("quit"), py_str("quitting"), py_str("quit for good"), py_str("stop"), py_str("stopping"), py_str("give up"), py_str("giving up"), py_str("cut down"), py_str("cutting down"), py_str("cut back"), py_str("cutting back"), py_str("cut back on"), py_str("cutting back on"), py_str("ease off"), py_str("easing off"), py_str("pack in"), py_str("packed in"), py_str("cant stop"), py_str("can't stop"), py_str("quit smoking"), py_str("stop smoking"), py_str("give up smoking"), py_str("quit cigarettes"), py_str("stop drinking"), py_str("quit drinking"), py_str("cut down on drinking"), py_str("cut down on alcohol"), py_str("quit vaping"), py_str("stop vaping")]) }
    s_STOP(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("skin picking"), py_str("picking my skin"), py_str("pick my skin"), py_str("picking at my skin"), py_str("picking spots"), py_str("picking at spots"), py_str("picking my spots"), py_str("picking at my face"), py_str("picking my face"), py_str("picking at scabs"), py_str("picking scabs"), py_str("picking a scab"), py_str("picking"), py_str("picking at")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("anger"), py_str("angry"), py_str("temper"), py_str("irritable"), py_str("irritated"), py_str("irritability"), py_str("rage"), py_str("raging"), py_str("losing my temper"), py_str("lose my temper"), py_str("lost my temper"), py_str("short fuse"), py_str("snappy"), py_str("snapping at"), py_str("snapped at"), py_str("lashing out"), py_str("blowing up"), py_str("blow up at")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("worry"), py_str("worrying"), py_str("worried"), py_str("anxiety"), py_str("anxious"), py_str("panicky"), py_str("dread"), py_str("overwhelmed"), py_str("nerves"), py_str("my nerves"), py_str("on edge"), py_str("wound up"), py_str("keyed up"), py_str("butterflies"), py_str("racing heart"), py_str("nervous"), py_str("calm down"), py_str("calm myself down"), py_str("worked up"), py_str("shaky"), py_str("jittery"), py_str("stop worrying"), py_str("worrying about"), py_str("worry about"), py_str("worries about"), py_str("worries me"), py_str("worrying me"), py_str("keep worrying"), py_str("always worrying"), py_str("worrying all the time"), py_str("how do i relax"), py_str("relax"), py_str("relaxing"), py_str("cant relax"), py_str("can't relax")]) }
    s_SMALL_HABITS(py_tuple(vec![py_tuple(vec![py_str("skin"), __data4(), py_str("Picking is a hand habit, so give the hand something else: moisturise so the spot feels different, keep a fidget toy or a ring where your fingers go, and cover it with a plaster for the first week so your fingers stop there. Mirrors and screens are where it starts, so keep them out of reach.")]), py_tuple(vec![py_str("knuckles"), py_tuple(vec![py_str("knuckles"), py_str("cracking my knuckles"), py_str("crack my knuckles"), py_str("cracking knuckles"), py_str("fidgeting"), py_str("fidget"), py_str("jiggling my leg"), py_str("shaking my leg"), py_str("bouncing my leg")]), py_str("Cracking and fidgeting are how the body burns off a wriggle, so the fix is a quieter outlet rather than sitting still: a stress ball in your pocket, a foot pressed to the floor, or a short walk every hour. It happens most in meetings and while waiting.")]), py_tuple(vec![py_str("temper"), __data5(), py_str("Anger is mostly a physical build-up, so catch it early rather than arguing with it: notice the first sign - jaw, shoulders, a louder voice - and leave the room for five minutes, breathing out for longer than you breathe in. Say \"I need a minute\" rather than the thing you are about to say. Then look at the fuel: too little sleep, no lunch, four coffees and a hangover will do more to your temper than any argument. If the fuse has been short for weeks rather than days, see a GP: anxiety, depression and a thyroid problem all arrive as irritability, and talking therapy works on anger.")]), py_tuple(vec![py_str("worry"), __data6(), py_str("Give the worry a time and a page: fifteen minutes in the evening when you write down everything on your mind, then ask of each one what you would actually do if it happened, and what the next step is. When it turns up at three in the afternoon, write one line to bring to the slot and go back to what you were doing - postponing is the part that trains it. Ten slow breaths out, longer than the in, settle the body enough to do that. If it is most days for six months or more, that is worth the GP and talking therapy, which works well on this.")]), py_tuple(vec![py_str("swearing"), py_tuple(vec![py_str("swearing"), py_str("swear"), py_str("bad language"), py_str("foul language"), py_str("cursing")]), py_str("Pick one substitute word and use it out loud for a week - 'sugar', 'flip', whatever you can say at work - because the habit is in the sound, not the meaning. It spikes with pain and frustration, so slow your breath for a second before you answer.")]), py_tuple(vec![py_str("snooze"), py_tuple(vec![py_str("snooze"), py_str("snoozing"), py_str("hitting snooze"), py_str("my alarm"), py_str("alarm")]), py_str("Put the alarm across the room and take the first one for a week, even when you are tired: the nine extra minutes cost you the deep sleep at the end of the night and leave you groggier. If mornings are brutal, go to bed half an hour earlier that week.")]), py_tuple(vec![py_str("hair"), py_tuple(vec![py_str("pulling my hair"), py_str("pull my hair"), py_str("pulling my hair out"), py_str("twirling my hair"), py_str("twirl my hair"), py_str("twirling hair"), py_str("chewing my hair"), py_str("biting my hair"), py_str("playing with my hair"), py_str("hair twirling"), py_str("trichotillomania")]), py_str("Keep it up and back - a bobble, a clip, a hat - so there is nothing loose for your fingers to find, and give them a ring to reach for instead. It happens most when you are bored or thinking hard. If you are pulling hair out in patches, see a GP: it is a known condition, not something you do on purpose.")]), py_tuple(vec![py_str("mouth"), py_tuple(vec![py_str("chewing my pen"), py_str("chew my pen"), py_str("biting my pen"), py_str("chewing gum"), py_str("chewing"), py_str("chewing my fingers"), py_str("chew my fingers"), py_str("biting my fingers"), py_str("biting my cheeks"), py_str("biting my lips"), py_str("chewing my lips"), py_str("licking my lips")]), py_func("_mouth_tip", |args: &[Py]| _mouth_tip(args))])]));
    fn __data7() -> Py { py_tuple(vec![py_str("stop"), py_str("stopping"), py_str("quit"), py_str("quitting"), py_str("kick"), py_str("kicking"), py_str("give up"), py_str("giving up"), py_str("cut down"), py_str("cutting down"), py_str("cut back"), py_str("get out of"), py_str("cant stop"), py_str("can't stop"), py_str("how do i stop"), py_str("i keep"), py_str("keep"), py_str("i always"), py_str("how do i control"), py_str("how can i control"), py_str("how do i manage"), py_str("how do i handle"), py_str("how do i deal with"), py_str("how do i get rid of"), py_str("how can i stop"), py_str("how can i quit"), py_str("help me stop"), py_str("what can i do about")]) }
    s_STOP_SMALL(__data7());
    s_HABIT_WORDS(py_tuple(vec![py_str("habit"), py_str("habits"), py_str("routine"), py_str("routines"), py_str("streak"), py_str("willpower"), py_str("resolution")]));
    fn __data8() -> Py { py_tuple(vec![py_str("how long"), py_str("how many days"), py_str("how many weeks"), py_str("take"), py_str("takes"), py_str("took"), py_str("days"), py_str("weeks"), py_str("months"), py_str("21 days"), py_str("three weeks"), py_str("66 days"), py_str("really"), py_str("how long does it take"), py_str("how long to break"), py_str("how long to stop"), py_str("how many days"), py_str("how many weeks")]) }
    s_HABIT_TIME_WORDS(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("tyre"), py_str("tire"), py_str("puncture"), py_str("scratch"), py_str("screw"), py_str("hammer"), py_str("polish"), py_str("hoover"), py_str("vacuum"), py_str("dyson"), py_str("mower"), py_str("lawnmower"), py_str("crumbs"), py_str("crumb"), py_str("varnish"), py_str("manicure"), py_str("salon"), py_str("split"), py_str("splitting"), py_str("peel"), py_str("peeling"), py_str("chipping"), py_str("brittle"), py_str("strengthen"), py_str("grow"), py_str("growing"), py_str("fake"), py_str("acrylic"), py_str("extensions"), py_str("dentist"), py_str("tooth"), py_str("teeth"), py_str("guitar"), py_str("piano"), py_str("data"), py_str("battery"), py_str("storage"), py_str("signal"), py_str("contract"), py_str("sim"), py_str("bill"), py_str("network"), py_str("wifi"), py_str("hacked"), py_str("stolen"), py_str("lost my phone"), py_str("screen"), py_str("smoking"), py_str("smoke"), py_str("cigarette"), py_str("cigarettes"), py_str("vape"), py_str("vaping"), py_str("alcohol"), py_str("drinking"), py_str("drink"), py_str("beer"), py_str("wine"), py_str("hangover"), py_str("bet"), py_str("betting"), py_str("gambling"), py_str("porn"), py_str("drugs"), py_str("breaking out"), py_str("breakout"), py_str("break outs"), py_str("acne"), py_str("brittle"), py_str("snapping"), py_str("splitting"), py_str("split end"), py_str("split ends"), py_str("frizzy"), py_str("frizz"), py_str("dry"), py_str("dryness"), py_str("greasy"), py_str("itchy"), py_str("itching"), py_str("rash"), py_str("eczema"), py_str("dermatitis"), py_str("thinning"), py_str("hair loss"), py_str("bald"), py_str("dandruff"), py_str("colour"), py_str("dye"), py_str("dyeing"), py_str("heartburn"), py_str("reflux"), py_str("indigestion"), py_str("acid"), py_str("bloating"), py_str("hiccups"), py_str("cough"), py_str("snoring"), py_str("snore"), py_str("picky eater"), py_str("picky eaters"), py_str("fussy eater"), py_str("fussy eaters"), py_str("faddy eater"), py_str("picky husband")]) }
    s_NOT_A_SMALL_HABIT(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("son"), py_str("daughter"), py_str("toddler"), py_str("baby"), py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("my brother"), py_str("my sister"), py_str("my boss"), py_str("my manager"), py_str("neighbour"), py_str("neighbor"), py_str("dog"), py_str("puppy"), py_str("cat"), py_str("traffic"), py_str("driver"), py_str("road"), py_str("computer"), py_str("laptop"), py_str("phone"), py_str("game"), py_str("football"), py_str("referee"), py_str("shouting at my kids"), py_str("at my kids"), py_str("at the kids"), py_str("at my children"), py_str("at my partner"), py_str("at my husband"), py_str("at my wife"), py_str("snapping at my partner"), py_str("his temper"), py_str("her temper"), py_str("their temper"), py_str("your temper"), py_str("loses his temper"), py_str("loses her temper"), py_str("lost his temper"), py_str("lost her temper"), py_str("losing his temper"), py_str("losing her temper"), py_str("his rag"), py_str("her rag"), py_str("temper with me"), py_str("loses it with me"), py_str("lost it with me")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("money"), py_str("rent"), py_str("bill"), py_str("bills"), py_str("job"), py_str("work"), py_str("office"), py_str("boss"), py_str("deadline"), py_str("shift"), py_str("interview"), py_str("exam"), py_str("exams"), py_str("test"), py_str("results"), py_str("health"), py_str("cancer"), py_str("lump"), py_str("pregnant"), py_str("pregnancy"), py_str("mum"), py_str("mom"), py_str("dad"), py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("flight"), py_str("flying"), py_str("wedding"), py_str("party"), py_str("speech"), py_str("presentation"), py_str("the future"), py_str("the news"), py_str("puppies"), py_str("pup"), py_str("cats"), py_str("kittens"), py_str("rabbit"), py_str("bird"), py_str("parrot"), py_str("horse"), py_str("toddler"), py_str("toddlers"), py_str("palpitations"), py_str("racing heart when"), py_str("chest pain"), py_str("short of breath"), py_str("nervous system"), py_str("nerve pain"), py_str("trapped nerve"), py_str("sciatica"), py_str("pinched nerve"), py_str("root canal"), py_str("tooth nerve"), py_str("nerves in my"), py_str("afraid of the dark"), py_str("customer"), py_str("customers"), py_str("patient"), py_str("patients"), py_str("passenger"), py_str("client"), py_str("clients"), py_str("colleague"), py_str("colleagues"), py_str("staff"), py_str("crew"), py_str("someone else"), py_str("the other one")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("pet"), py_str("pets"), py_str("rabbit"), py_str("hamster"), py_str("parrot"), py_str("bird"), py_str("horse"), py_str("teething"), py_str("sleeve"), py_str("collar"), py_str("nail"), py_str("nails"), py_str("nail biting"), py_str("cuticles"), py_str("clothes"), py_str("clothing"), py_str("shirt"), py_str("shirts"), py_str("trousers"), py_str("jeans"), py_str("dress"), py_str("dresses"), py_str("skirt"), py_str("jumper"), py_str("jumpers"), py_str("cardigan"), py_str("coat"), py_str("jacket"), py_str("hoodie"), py_str("leggings"), py_str("shorts"), py_str("towel"), py_str("bedding"), py_str("sheet"), py_str("sheets"), py_str("duvet"), py_str("fabric"), py_str("wool"), py_str("carpet"), py_str("carpets"), py_str("rug"), py_str("rugs"), py_str("sofa"), py_str("settee"), py_str("couch"), py_str("mattress"), py_str("upholstery"), py_str("floor"), py_str("floors"), py_str("tiles"), py_str("pavement"), py_str("path"), py_str("wall"), py_str("walls"), py_str("shoe"), py_str("shoes"), py_str("trainer"), py_str("trainers"), py_str("boot"), py_str("boots"), py_str("table"), py_str("chair"), py_str("washing machine"), py_str("dryer"), py_str("tumble dryer"), py_str("car seat"), py_str("car"), py_str("seats")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("sugar"), py_str("sugary"), py_str("sugar in"), py_str("sugars"), py_str("night"), py_str("at night"), py_str("in the evening"), py_str("evening"), py_str("after dinner"), py_str("late"), py_str("before bed"), py_str("night time"), py_str("nighttime")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("burglar"), py_str("burglars"), py_str("intruder"), py_str("intruders"), py_str("house alarm"), py_str("security alarm"), py_str("security system"), py_str("alarm system"), py_str("alarm panel"), py_str("control panel"), py_str("the panel"), py_str("sensor"), py_str("sensors"), py_str("cctv"), py_str("smoke"), py_str("smoke alarm"), py_str("smoke detector"), py_str("carbon monoxide"), py_str("car alarm"), py_str("alarm on my car"), py_str("van"), py_str("vehicle"), py_str("fob"), py_str("key fob"), py_str("immobiliser"), py_str("siren"), py_str("car park"), py_str("mechanic"), py_str("pendant"), py_str("pendant alarm"), py_str("falls alarm"), py_str("fall alarm"), py_str("personal alarm"), py_str("telecare"), py_str("careline"), py_str("lifeline"), py_str("care alarm"), py_str("panic alarm"), py_str("panic button"), py_str("medical alarm")]) }
    s_SMALL_HABIT_EXTRA_AVOID(py_dict(vec![(py_str("temper"), __data10()), (py_str("worry"), __data11()), (py_str("mouth"), __data12()), (py_str("snacks"), __data13()), (py_str("snooze"), __data14())]));
    fn __data15() -> Py { py_tuple(vec![py_str("calm"), py_str("calming"), py_str("calm down"), py_str("calm myself down"), py_str("how do i calm"), py_str("how can i calm"), py_str("settle"), py_str("settle myself"), py_str("how do i settle"), py_str("chill out"), py_str("wind down"), py_str("feel better"), py_str("get through")]) }
    s_WORRY_ASK(op_add(&g_STOP_SMALL(), &__data15()));
    s_WORRY_ASK(op_add(&g_WORRY_ASK(), &py_tuple(vec![py_str("how do i"), py_str("how can i"), py_str("what helps"), py_str("what helps with"), py_str("what can i do"), py_str("helps"), py_str("help"), py_str("helping"), py_str("what should i do"), py_str("any tips"), py_str("tips")])));
    fn __data16() -> Py { py_tuple(vec![py_str("hair"), py_str("hairs"), py_str("my hair"), py_str("relaxer"), py_str("relaxers"), py_str("relax my hair"), py_str("relaxing my hair"), py_str("beard"), py_str("braids"), py_str("doctor"), py_str("gp"), py_str("gp appointment"), py_str("appointment"), py_str("surgery"), py_str("nurse"), py_str("practice nurse"), py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("symptom"), py_str("symptoms"), py_str("pain"), py_str("ache"), py_str("aches"), py_str("spot"), py_str("spots"), py_str("mole"), py_str("lump"), py_str("bleeding"), py_str("fever"), py_str("temperature"), py_str("test results"), py_str("scan"), py_str("x-ray"), py_str("results")]) }
    let _ = py_setitem(&g_SMALL_HABIT_EXTRA_AVOID(), &py_str("worry"), op_add(&mcall(&g_SMALL_HABIT_EXTRA_AVOID(), "get", &[py_str("worry"), py_tuple(vec![])]), &__data16()));
    s_SMALL_HABIT_ASK(py_dict(vec![(py_str("worry"), g_WORRY_ASK())]));
    s__ANOTHER_PERSONS_MEMORY(stdlib_call("re", "compile", &[py_str("\\b(?:mum|mom|mother|dad|father|parent|parents|grandma|grandmother|grandad|grandfather|gran|nan|nana|granny|wife|husband|partner|spouse|other half|mother in law|father in law|aunt|auntie|uncle|brother|sister)\\b[^.?!]{0,40}\\b(?:forgets?|forgetting|forgetful|forgot|keeps forgetting|repeats|repeating|confused|confusion)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SMALL_HABIT_ENTRIES({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SMALL_HABITS()) { let __unpk7452 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk7452[0].clone(); let mut keys: Py = __unpk7452[1].clone(); let mut tip: Py = __unpk7452[2].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("stop-"), &name.clone())); __args.push(py_tuple(vec![keys.clone(), mcall(&g_SMALL_HABIT_ASK(), "get", &[name.clone(), g_STOP_SMALL()])])); __args.push(tip.clone()); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_SMALL_HABIT(), &g_THIRD_PARTY()), &g_FEEL_BAD()), &mcall(&g_SMALL_HABIT_EXTRA_AVOID(), "get", &[name.clone(), py_tuple(vec![])]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
    fn __data17() -> Py { py_tuple(vec![py_str("work break"), py_str("shift"), py_str("rest between"), py_str("holiday"), py_str("annual leave"), py_str("routine blood test"), py_str("morning routine for school"), py_str("gym"), py_str("gyms"), py_str("workout"), py_str("workouts"), py_str("exercise"), py_str("training"), py_str("run"), py_str("running"), py_str("walk"), py_str("commute"), py_str("morning routine"), py_str("bedtime routine"), py_str("evening routine"), py_str("school run"), py_str("breaking my streak"), py_str("broke my streak"), py_str("breaks my streak"), py_str("keep breaking my streak"), py_str("streak keeps breaking"), py_str("losing my streak"), py_str("lost my streak"), py_str("ruined my streak"), py_str("missed my streak"), py_str("keep breaking the"), py_str("my streak"), py_str("the streak"), py_str("missed a day"), py_str("miss a day")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("build"), py_str("start"), py_str("make it stick"), py_str("make a habit"), py_str("make it a habit"), py_str("form"), py_str("stick"), py_str("sticking"), py_str("keep up"), py_str("new"), py_str("begin"), py_str("get into"), py_str("get back into"), py_str("every day"), py_str("daily")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("retire"), py_str("retires"), py_str("retiring"), py_str("retired"), py_str("retirement"), py_str("gave up work"), py_str("given up work"), py_str("stopped working"), py_str("no longer working"), py_str("finished work"), py_str("left work"), py_str("packed in work"), py_str("last day at work"), py_str("not working any more"), py_str("not working"), py_str("no work to go to"), py_str("stopped work"), py_str("given up work")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("lost"), py_str("at a loss"), py_str("feel lost"), py_str("normal"), py_str("strange"), py_str("odd"), py_str("weird"), py_str("adjust"), py_str("adjusting"), py_str("get used to"), py_str("getting used to"), py_str("cope"), py_str("coping"), py_str("struggling"), py_str("what do i do"), py_str("what do people do"), py_str("what to do with myself"), py_str("know what to do"), py_str("what now"), py_str("bored"), py_str("boredom"), py_str("lonely"), py_str("loneliness"), py_str("miss"), py_str("missing"), py_str("structure"), py_str("routine"), py_str("purpose"), py_str("with myself"), py_str("nothing to do"), py_str("keep busy"), py_str("keep occupied"), py_str("how do i"), py_str("tips"), py_str("advice")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("pension"), py_str("pensions"), py_str("state pension"), py_str("pot"), py_str("savings"), py_str("money"), py_str("afford"), py_str("tax"), py_str("annuity"), py_str("drawdown"), py_str("lump sum"), py_str("ni"), py_str("national insurance"), py_str("how much"), py_str("how many years"), py_str("when can i"), py_str("at 55"), py_str("at 60"), py_str("at 65"), py_str("at 66"), py_str("at 67"), py_str("early retirement"), py_str("retire early"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my mum"), py_str("my dad"), py_str("my colleague"), py_str("my boss"), py_str("he "), py_str("she "), py_str("party"), py_str("parties"), py_str("retirement party"), py_str("leaving do"), py_str("leaving party"), py_str("celebrate"), py_str("celebration"), py_str("send off"), py_str("guest list"), py_str("invitations"), py_str("invite people"), py_str("venue"), py_str("catering"), py_str("speech"), py_str("toast"), py_str("present"), py_str("gift")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("always late"), py_str("im always late"), py_str("i am always late"), py_str("i'm always late"), py_str("stop being late"), py_str("keep being late"), py_str("keeps being late"), py_str("never on time"), py_str("not on time"), py_str("punctual"), py_str("timekeeping"), py_str("time keeping"), py_str("late again"), py_str("late for everything"), py_str("late every day"), py_str("always rushing"), py_str("running out of time in the morning")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("how"), py_str("why"), py_str("stop"), py_str("help"), py_str("advice"), py_str("tips"), py_str("time"), py_str("morning"), py_str("better"), py_str("fix"), py_str("always"), py_str("keep"), py_str("what"), py_str("should")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("parcel"), py_str("parcels"), py_str("delivery"), py_str("deliveries"), py_str("train"), py_str("trains"), py_str("bus"), py_str("buses"), py_str("flight"), py_str("flights"), py_str("payment"), py_str("rent"), py_str("bill"), py_str("bills"), py_str("fee"), py_str("fees"), py_str("my pay"), py_str("our pay"), py_str("the pay"), py_str("wages"), py_str("wage"), py_str("salary"), py_str("payslip"), py_str("payslips"), py_str("payday"), py_str("payroll"), py_str("pay packet"), py_str("paid me"), py_str("refund"), py_str("invoice"), py_str("period"), py_str("periods"), py_str("pregnant"), py_str("baby"), py_str("midwife"), py_str("deadline"), py_str("tax return"), py_str("boss"), py_str("manager"), py_str("interview"), py_str("school run"), py_str("night shift"), py_str("late for work?"), py_str("sorry im late"), py_str("tell my boss"), py_str("what do i say")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("meditation"), py_str("meditate"), py_str("meditating"), py_str("meditates"), py_str("meditated"), py_str("mindfulness"), py_str("mindful"), py_str("breathing exercise"), py_str("breathing exercises"), py_str("headspace"), py_str("mbsr"), py_str("mbct"), py_str("guided meditation")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("start"), py_str("begin"), py_str("learn"), py_str("how"), py_str("long"), py_str("often"), py_str("work"), py_str("works"), py_str("worth"), py_str("help"), py_str("helps"), py_str("better"), py_str("app"), py_str("apps"), py_str("beginners"), py_str("beginner"), py_str("every day"), py_str("daily"), py_str("routine"), py_str("habit"), py_str("should"), py_str("try"), py_str("anxiety"), py_str("anxious"), py_str("stress"), py_str("stressed"), py_str("calm"), py_str("panic"), py_str("relax")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("pray"), py_str("prayer"), py_str("praying"), py_str("bible"), py_str("church"), py_str("yoga"), py_str("pilates"), py_str("cough"), py_str("asthma"), py_str("inhaler"), py_str("peak flow"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("children"), py_str("kids"), py_str("kid"), py_str("school"), py_str("sleep"), py_str("insomnia")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("what"), py_str("whats"), py_str("is"), py_str("are"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("definition"), py_str("define"), py_str("explain"), py_str("explained"), py_str("difference"), py_str("different"), py_str("actually")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("pray"), py_str("prayer"), py_str("praying"), py_str("bible"), py_str("church"), py_str("yoga"), py_str("pilates"), py_str("cough"), py_str("asthma"), py_str("inhaler"), py_str("peak flow"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("children"), py_str("kids"), py_str("kid"), py_str("school"), py_str("sleep"), py_str("insomnia")]) }
    s_EXTRA_ENTRIES(op_add(&py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("habit-time")); __args.push(py_tuple(vec![g_HABIT_WORDS(), g_HABIT_TIME_WORDS()])); __args.push(py_str("About two months, not the three weeks people quote: in the study that measured it the median was 66 days, and the easy ones (a glass of water with breakfast) settled in a few weeks while the hard ones took most of a year. Missing one day made no difference to how long it took, so a slip is not a restart - the rule is never miss twice.")); __kws.extend(vec![("avoid".to_string(), __data17()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("habit-build")); __args.push(py_tuple(vec![op_add(&g_HABIT_WORDS(), &py_tuple(vec![py_str("something new"), py_str("something every day")])), __data18()])); __args.push(py_str("Attach it to something you already do every day - after the kettle boils, after you brush your teeth - because a new habit needs an old one to hang on. Make the first version almost too small to fail (one page, one glass, two minutes), keep it in the same place and time, and put a tick on a wall calendar: the streak is what carries it past the first fortnight, when the novelty has gone.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("work break"), py_str("shift"), py_str("holiday"), py_str("salary"), py_str("job"), py_str("interview"), py_str("pan"), py_str("oven"), py_str("recipe"), py_str("sauce"), py_str("bake")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("retired-and-at-a-loss")); __args.push(py_tuple(vec![__data19(), __data20()])); __args.push(py_str("That lost feeling is the usual thing and it has a shape: the first few months are the holiday, the flat part comes next, and by a year most people have a week they like again. What went with the job was not only the money - it was the diary, the reason to be up, and the people you saw without arranging it. So rebuild those three on purpose and small: one fixed thing a week that somebody expects you at (a class, a shift at a charity shop, a badminton game, a regular coffee), one reason to be dressed and out by nine on a weekday, and one afternoon a week that is properly yours. Say yes to the first invitation even when staying in is easier, because the invitations thin out when you stop going. If the mornings are still heavy all day after six months, mention it to the GP - low mood after a big change is common and treatable - but drift at the start is not a warning sign, it is the gap between one life and the next.")); __kws.extend(vec![("avoid".to_string(), __data21()), ("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "a_thing_has_stopped", &[understood.clone()]))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("always-late")); __args.push(py_tuple(vec![__data22(), __data23()])); __args.push(py_str("Lateness is nearly always the plan rather than the person: time one real journey at the hour you actually travel and add ten minutes, then set an alarm for leaving rather than only for waking. Lay out clothes, bag and keys the night before, and book the one earlier train instead of telling yourself you will hurry. If it is the last five minutes that keep going, drop the one job you do on the way out: being five minutes early costs nothing.")); __kws.extend(vec![("avoid".to_string(), __data24()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meditate-start")); __args.push(py_tuple(vec![__data25(), __data26(), py_tuple(vec![py_str("wander"), py_str("wanders"), py_str("wandering"), py_str("wandered"), py_str("wandering mind"), py_str("mind wanders"), py_str("distracted"), py_str("distraction"), py_str("drift"), py_str("drifts"), py_str("drifting"), py_str("monkey mind")])])); __args.push(py_str("Five minutes sitting still with your attention on your breathing is the whole of it: the mind wanders, you notice and come back, and the coming back is the exercise, not a failure. Same time each day, for two weeks before you judge it. The eight-week courses, MBSR and MBCT, are the ones that have been tested, for low mood and for anxiety, and an app like Headspace teaches the same thing.")); __kws.extend(vec![("avoid".to_string(), __data27()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meditation-what-it-is")); __args.push(py_tuple(vec![py_tuple(vec![py_str("meditation"), py_str("meditating"), py_str("meditate"), py_str("mindfulness"), py_str("mindful"), py_str("mbsr"), py_str("mbct")]), __data28()])); __args.push(py_str("Meditation is paying attention to one thing on purpose - usually the breath - and coming back to it each time you notice you have drifted; an empty mind is not the aim, and the noticing is the skill. The tested versions are the eight-week courses, MBSR and MBCT, mainly for low mood and anxiety.")); __kws.extend(vec![("avoid".to_string(), __data29()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]), &g_SMALL_HABIT_ENTRIES()));
    fn __data30() -> Py { py_tuple(vec![py_str("putting the rent up"), py_str("putting my rent up"), py_str("putting our rent up"), py_str("putting up the rent"), py_str("putting up my rent"), py_str("puts the rent up"), py_str("puts my rent up"), py_str("put the rent up"), py_str("put my rent up"), py_str("rent going up"), py_str("rent went up"), py_str("rent keeps going up"), py_str("putting the price up"), py_str("putting prices up"), py_str("putting up the price"), py_str("putting up prices"), py_str("puts the price up"), py_str("puts prices up"), py_str("put the price up"), py_str("put prices up"), py_str("putting the price of"), py_str("putting up the price of"), py_str("putting the tax up"), py_str("putting taxes up"), py_str("putting up taxes"), py_str("putting the council tax up"), py_str("putting the fares up"), py_str("putting fares up"), py_str("putting up fares"), py_str("putting the bills up"), py_str("putting up the bills"), py_str("putting the charges up"), py_str("putting the fees up"), py_str("putting the rates up"), py_str("putting the cost up"), py_str("putting costs up")]) }
    s__PUTTING_A_CHARGE_UP(__data30());
    fn __data31() -> Py { py_tuple(vec![py_str("procrastinat"), py_str("procrastinating"), py_str("putting off"), py_str("put off"), py_str("put things off"), py_str("put it off"), py_str("cant start"), py_str("can't start"), py_str("cannot start"), py_str("keep putting"), py_str("keep procrastinating"), py_str("keep putting off"), py_str("keep putting things off"), py_str("always putting things off"), py_str("procrastinate"), py_str("lazy at work"), py_str("lazy at my job"), py_str("lazy with work"), py_str("lazy at home"), py_str("lazy at the moment"), py_str("lazy lately"), py_str("so lazy"), py_str("am lazy"), py_str("im lazy"), py_str("feeling lazy"), py_str("no motivation at work"), py_str("no motivation at my job"), py_str("no motivation for work"), py_str("no motivation to work"), py_str("no motivation at home"), py_str("cant be bothered to start"), py_str("do nothing all day"), py_str("doing nothing all day"), py_str("get nothing done"), py_str("gets nothing done"), py_str("nothing gets done")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("stop"), py_str("always"), py_str("constantly"), py_str("keep"), py_str("keeps"), py_str("everything"), py_str("anything"), py_str("stuck"), py_str("help"), py_str("how"), py_str("at work"), py_str("work"), py_str("my job"), py_str("job"), py_str("office"), py_str("shift"), py_str("desk"), py_str("at home"), py_str("chores"), py_str("housework"), py_str("uni"), py_str("school"), py_str("revising"), py_str("the gym"), py_str("putting it off"), py_str("kept putting it off"), py_str("keeps putting it off"), py_str("putting off this"), py_str("putting off that"), py_str("putting off going"), py_str("putting off making"), py_str("putting off ringing"), py_str("putting off calling"), py_str("putting off booking")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("study"), py_str("studying"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("homework"), py_str("essay"), py_str("exam"), py_str("exams"), py_str("gcse"), py_str("a-level"), py_str("mocks"), py_str("uni"), py_str("university"), py_str("college"), py_str("school"), py_str("coursework"), py_str("dissertation"), py_str("thesis"), py_str("assignment"), py_str("deadline"), py_str("project due"), py_str("has a project"), py_str("project about"), py_str("project on"), py_str("school project"), py_str("poster about"), py_str("put me down"), py_str("puts me down"), py_str("putting me down"), py_str("put down"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("teenager"), py_str("teen"), py_str("flatmate"), py_str("housemate"), py_str("his share"), py_str("her share"), py_str("doesnt help"), py_str("doesn't help"), py_str("work from home"), py_str("working from home"), py_str("wfh"), py_str("from home")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("coil"), py_str("coils"), py_str("battery"), py_str("charger"), py_str("shop"), py_str("cheapest"), py_str("price"), py_str("flavour"), py_str("flavor"), py_str("illegal"), py_str("law"), py_str("underage"), py_str("ban")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("less"), py_str("reduce"), py_str("reducing"), py_str("too much"), py_str("every day"), py_str("every night"), py_str("how much is too much"), py_str("a night"), py_str("every evening"), py_str("a bottle a night"), py_str("a bottle every night"), py_str("a pint a day"), py_str("each night")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("stain"), py_str("stains"), py_str("spill"), py_str("spilled"), py_str("spilt"), py_str("shirt"), py_str("sofa"), py_str("carpet"), py_str("coffee"), py_str("tea"), py_str("water"), py_str("juice"), py_str("milk"), py_str("fizzy"), py_str("thirsty"), py_str("kebab"), py_str("miss a"), py_str("miss the"), py_str("miss my"), py_str("missing a"), py_str("craving"), py_str("crave"), py_str("craves"), py_str("alcohol free"), py_str("alcohol-free"), py_str("alcoholfree"), py_str("non alcoholic"), py_str("non-alcoholic"), py_str("nonalcoholic"), py_str("zero alcohol"), py_str("booze free"), py_str("no alcohol"), py_str("energy"), py_str("energy drink"), py_str("energy drinks"), py_str("red bull"), py_str("monster"), py_str("relentless"), py_str("lucozade"), py_str("sports drink"), py_str("sports drinks"), py_str("protein shake"), py_str("caffeine"), py_str("espresso"), py_str("cola"), py_str("coke"), py_str("chocolate"), py_str("sauce"), py_str("recipe"), py_str("braised"), py_str("marinade"), py_str("good for you"), py_str("good for me"), py_str("healthy"), py_str("healthier"), py_str("benefit"), py_str("benefits"), py_str("hangover"), py_str("hungover"), py_str("drank too much"), py_str("last night"), py_str("so hard"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("pups"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("rabbit"), py_str("rabbits"), py_str("bunny"), py_str("bunnies"), py_str("hamster"), py_str("hamsters"), py_str("guinea pig"), py_str("guinea pigs"), py_str("gerbil"), py_str("gerbils"), py_str("rat"), py_str("rats"), py_str("hamster's"), py_str("chinchilla"), py_str("ferret"), py_str("horse"), py_str("horses"), py_str("pony"), py_str("ponies"), py_str("kitten"), py_str("kittens"), py_str("litter tray"), py_str("water bottle")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("phone"), py_str("phones"), py_str("his phone"), py_str("her phone"), py_str("their phone"), py_str("screens"), py_str("screen"), py_str("scrolling"), py_str("scroll"), py_str("instagram"), py_str("tiktok"), py_str("facebook"), py_str("social media"), py_str("tablet"), py_str("ipad"), py_str("youtube")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("always on"), py_str("on his phone"), py_str("on her phone"), py_str("on their phone"), py_str("addicted"), py_str("glued to"), py_str("too much"), py_str("never puts"), py_str("wont put"), py_str("won't put"), py_str("cant put"), py_str("can't put"), py_str("all day"), py_str("all the time"), py_str("ignores me"), py_str("stuck on")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("help"), py_str("should"), py_str("can i"), py_str("says"), py_str("told me"), py_str("wont"), py_str("won't"), py_str("always"), py_str("spends"), py_str("is on"), py_str("on his"), py_str("on her"), py_str("on their"), py_str("glued"), py_str("addicted"), py_str("never puts"), py_str("ignores me")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("toddler"), py_str("son"), py_str("daughter"), py_str("teen"), py_str("teens"), py_str("teenager"), py_str("year old"), py_str("homework"), py_str("school"), py_str("screen time"), py_str("contract"), py_str("bill"), py_str("tariff"), py_str("price"), py_str("cost"), py_str("upgrade"), py_str("battery"), py_str("charge"), py_str("cracked"), py_str("repair"), py_str("replace"), py_str("lost"), py_str("stolen"), py_str("storage")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("keep forgetting"), py_str("keeps forgetting"), py_str("keep forgetting to"), py_str("keeps forgetting to"), py_str("always forgetting"), py_str("i forget to"), py_str("forget to"), py_str("forgetting to"), py_str("forgot to"), py_str("keep missing"), py_str("stop forgetting"), py_str("keep forgetting about"), py_str("remember to do"), py_str("remember to")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("to"), py_str("ring"), py_str("call"), py_str("phone"), py_str("book"), py_str("pay"), py_str("renew"), py_str("post"), py_str("send"), py_str("email"), py_str("collect"), py_str("pick up"), py_str("buy"), py_str("order"), py_str("do"), py_str("make"), py_str("take"), py_str("go"), py_str("appointment"), py_str("appointments"), py_str("dentist"), py_str("doctor"), py_str("gp"), py_str("optician"), py_str("bill"), py_str("bills"), py_str("errand"), py_str("errands"), py_str("job"), py_str("jobs"), py_str("chore"), py_str("chores"), py_str("renewal"), py_str("form"), py_str("parcel"), py_str("parcels"), py_str("things"), py_str("stuff"), py_str("shopping"), py_str("return"), py_str("returns")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("how"), py_str("stop"), py_str("keep"), py_str("keeps"), py_str("always"), py_str("help"), py_str("better"), py_str("tips"), py_str("tip"), py_str("remind"), py_str("reminder"), py_str("reminders"), py_str("system")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("tablet"), py_str("tablets"), py_str("medication"), py_str("medicine"), py_str("pill"), py_str("pills"), py_str("prescription"), py_str("water"), py_str("drink"), py_str("drinking"), py_str("where i put"), py_str("where i left"), py_str("where ive put"), py_str("keys"), py_str("glasses"), py_str("wallet"), py_str("phone number"), py_str("password"), py_str("pin"), py_str("name of"), py_str("names"), py_str("word"), py_str("spelling"), py_str("what i read"), py_str("book i"), py_str("reading"), py_str("revise"), py_str("revision"), py_str("study"), py_str("exam"), py_str("exams"), py_str("coursework"), py_str("someone"), py_str("somebody"), py_str("my ex"), py_str("grief"), py_str("died"), py_str("passed away"), py_str("birthday card"), py_str("present"), py_str("gift"), py_str("birthday"), py_str("birthdays"), py_str("anniversary"), py_str("anniversaries"), py_str("why"), py_str("what causes"), py_str("what makes"), py_str("is it normal")]) }
    s_ENTRIES(op_add(&g_EXTRA_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("putting-things-off")); __args.push(py_tuple(vec![__data31(), __data32()])); __args.push(py_str("Shrink the job until it is almost silly: name the very next physical action — open the letter, put the shoes by the door — and do only that one, for five minutes, with permission to do it badly. Putting things off is usually about the task feeling too big to begin, and it stops the moment you are in it. Decide tonight what tomorrow's first step is, so it is an instruction rather than a choice.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data33(), &g__PUTTING_A_CHARGE_UP())), ("guard".to_string(), py_func("_the_writer_is_the_one_forgetting", |args: &[Py]| _the_writer_is_the_one_forgetting(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quit-vaping")); __args.push(py_tuple(vec![py_tuple(vec![py_str("vaping"), py_str("vape"), py_str("vapes"), py_str("vaper"), py_str("e cigarette"), py_str("e cigarettes"), py_str("e cig"), py_str("elf bar"), py_str("nic salt")]), g_STOP()])); __args.push(py_str("Staying on the vape beats going back to cigarettes, so if it is the only thing keeping you off tobacco, the win is not to stop. To come off it: drop the nicotine strength a step every two weeks, keep it out of sight rather than in your hand, and set a date instead of drifting down. The hand-to-mouth habit outlasts the nicotine, so keep a straw or gum where the vape used to live. Feeling low or snappy for a week is withdrawal, and it passes.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&__data34(), &g_THIRD_PARTY()), &g_FEEL_BAD()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cut-down-drinking")); __args.push(py_tuple(vec![py_tuple(vec![py_str("drinking"), py_str("drink"), py_str("alcohol"), py_str("beer"), py_str("wine"), py_str("booze"), py_str("units"), py_str("pint"), py_str("pints"), py_str("vodka"), py_str("whisky"), py_str("gin")]), op_add(&g_STOP(), &__data35())])); __args.push(py_str("Write down what you really drink for a fortnight - most people are surprised - then take three drink-free days a week as the first change, since days off are easier than smaller measures. Alternate each drink with a glass of water, keep nothing in the house on the days you are off it, and tell one person you are doing it. If you drink every day and shake or sweat when you stop, see a GP first: coming off alcohol suddenly can be dangerous.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&__data36(), &g_THIRD_PARTY()), &g_FEEL_BAD()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("their-phone-habit")); __args.push(py_tuple(vec![__data37(), __data38()])); __args.push(py_str("Say the thing you miss rather than a number of minutes, because a limit argued about in the moment is a row about the phone and a request is something they can answer: \"I miss talking at dinner\" works where \"you are on that thing again\" does not. Ask for one phone-free slot a day, a meal is the easy one, and put your own phone face down for it, because a rule that covers both of you holds and one imposed on them does not. If the scrolling is new, or they seem flat as well as glued to it, the phone is the symptom and a chat about how they are is the better move.")); __kws.extend(vec![("ask".to_string(), __data39()), ("avoid".to_string(), __data40()), ("guard".to_string(), py_func("screens_are_somebody_elses", |args: &[Py]| pyrt::xrefs::xcall("knowledge.wording", "screens_are_somebody_elses", args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("forgotten-errand")); __args.push(py_tuple(vec![__data41(), __data42()])); __args.push(py_str("Set the reminder the moment you think of it, while the phone is in your hand: \"remind me to ring the dentist at nine tomorrow\" into Siri or Google, or an alarm labelled with the job. Anything with a date - dentist, MOT, renewal, a birthday - goes in the calendar now, with an alert the day before and a yearly repeat for birthdays. A job that takes under two minutes is quicker done than remembered, so do it there and then; for the rest, leave the note where you will trip over it - the door keys, the kettle, the front door.")); __kws.extend(vec![("ask".to_string(), __data43()), ("avoid".to_string(), __data44()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); { let __b = _the_errand_is_the_speakers_own(&[reading.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_someone_else_is_forgetting(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_my_own_memory_and_my_age(&[reading.clone()]))) } } } } }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])));
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    fn __data45() -> Py { py_tuple(vec![py_str("worrying about me"), py_str("worries about me"), py_str("worry about me"), py_str("worried about me"), py_str("anxious about me"), py_str("fussing about me"), py_str("fusses about me"), py_str("fussing over me"), py_str("stop worrying about me"), py_str("stop fussing about me"), py_str("my parents worry"), py_str("parents worrying"), py_str("my mum worries"), py_str("my mom worries"), py_str("my dad worries"), py_str("mum worrying"), py_str("mom worrying"), py_str("dad worrying"), py_str("they worry"), py_str("she worries"), py_str("he worries"), py_str("overprotective"), py_str("over protective"), py_str("smothering"), py_str("keeps checking on me"), py_str("keep checking on me"), py_str("always checking on me"), py_str("worries too much"), py_str("worry too much"), py_str("worrying too much"), py_str("worries about me all the time"), py_str("worrying about me all the time"), py_str("always worrying about me"), py_str("parents are overprotective"), py_str("overprotective parents"), py_str("controlling parents"), py_str("get my mum to stop worrying"), py_str("get my mom to stop worrying"), py_str("get my dad to stop worrying"), py_str("get my parents to stop worrying"), py_str("get her to stop worrying"), py_str("get him to stop worrying"), py_str("mum to stop worrying"), py_str("dad to stop worrying"), py_str("parents to stop worrying"), py_str("to stop worrying about me")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("how do i"), py_str("how can i"), py_str("what can i do"), py_str("what do i do"), py_str("stop"), py_str("help"), py_str("deal with"), py_str("get them to"), py_str("make them stop"), py_str("reassure"), py_str("calm"), py_str("tell them"), py_str("explain to them"), py_str("get her to"), py_str("get him to"), py_str("handle"), py_str("why does"), py_str("why do"), py_str("are"), py_str("always"), py_str("all the time"), py_str("every day"), py_str("constantly"), py_str("too much"), py_str("so much"), py_str("keeps"), py_str("keep")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("can"), py_str("should"), py_str("do"), py_str("i"), py_str("my"), py_str("help"), py_str("stop"), py_str("deal"), py_str("get"), py_str("make"), py_str("reassure"), py_str("calm"), py_str("tell"), py_str("explain"), py_str("to"), py_str("why"), py_str("they"), py_str("she"), py_str("he")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("horse"), py_str("rabbit"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("children"), py_str("son"), py_str("daughter"), py_str("money"), py_str("rent"), py_str("bills"), py_str("debt"), py_str("doctor"), py_str("gp"), py_str("appointment"), py_str("headache"), py_str("symptom"), py_str("results"), py_str("exam"), py_str("exams"), py_str("revision"), py_str("interview"), py_str("wedding"), py_str("flight"), py_str("flying")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-worry-about-me")); __args.push(py_tuple(vec![__data45(), __data46()])); __args.push(py_str("Give them a fact rather than a reassurance: \"I'll text you when I'm on the train\" works where \"don't worry\" does not, because a worry needs something to hold. Ask what exactly they are afraid of, answer that one thing once, and then stop explaining - saying it a second and third time is what teaches them the worry is worth asking about.")); __kws.extend(vec![("ask".to_string(), __data47()), ("avoid".to_string(), __data48()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _about_a_pet(args: &[Py]) -> Py {
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
    let mut leans: Py = Py::None;
    let mut remembered: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Whether the subject this line points back to is an animal.\n\n    Only the carried subject counts - the \"it\"/\"them\" of the line, or the subject of the line\n    it leans on - never the words the person typed, because a line that types an animal already\n    reaches the pet entry on its own words.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = py_list(vec![]);
    remembered = mcall(&understood.clone(), "get", &[py_str("ref")]);
    if py_truthy(&call_builtin("isinstance", &[remembered.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&remembered.clone(), "values", &[])) { let mut value: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(value.clone()); } } py_list(__out) })() }]);
    }
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    subject = mcall(&leans.clone(), "get", &[py_str("subject")]);
    if py_truthy(&call_builtin("isinstance", &[subject.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[subject.clone()]);
    }
    text = mcall(&leans.clone(), "get", &[py_str("text")]);
    if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[text.clone()]);
    }
    }
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__PET_ANY(), "search", &[mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _pet_row_answers_before_it(args: &[Py]) -> Py {
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
    let mut _forms: Py = Py::None;
    let mut all_entries: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut item: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut score: Py = Py::None;
    let mut together: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a row about the animal itself can answer this line instead of the mouth one.\n\n    Standing aside is only worth it when an answer is waiting: the composer's next reading of\n    a follow-up is the line with the earlier subject put back in, and when that reading reaches\n    nothing the turn can end on a shape answer about something else. So the reading is made\n    here, and this row steps aside only for a match in the animals' own domain.\n    ");
    all_entries = py_func("entries", |args: &[Py]| pyrt::xrefs::xcall("knowledge", "entries", args));
    score = py_func("score", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "score", args));
    _forms = py_func("_forms", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "_forms", args));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    carried = py_list(vec![]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&leans.clone(), "get", &[py_str("words")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    carried = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&leans.clone(), &py_str("words"))) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    together = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&carried.clone(), &words.clone())) { let mut word: Py = __item.clone(); __out.push(py_call(&_forms, &[word.clone()])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&together.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let __iter6 = py_iter_or_raise(&py_call(&all_entries, &[]));
    for __item in __iter6 {
    item = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)), &py_str("pets"))); if !py_truthy(&__b) { __b } else { py_call(&score, &[item.clone(), together.clone(), op_add(&carried.clone(), &words.clone())]) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return Py::Bool(false);
    Py::None
}
pub fn _mouth_tip(args: &[Py]) -> Py {
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
    let _ = py_str("The mouth tip, unless the thing being chewed is an animal's habit and not a person's.");
    if py_truthy(&{ let __b = _about_a_pet(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _pet_row_answers_before_it(&[memory.clone(), understood.clone()]) } }) {
    return Py::None;
    }
    return g_MOUTH_TIP();
    Py::None
}
pub fn _my_own_memory_and_my_age(args: &[Py]) -> Py {
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
    let mut line_of: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the speaker is asking about their own memory coming with age. A `guard=` test.");
    line_of = py_func("line_of", |args: &[Py]| pyrt::xrefs::xcall("compose.repair", "line_of", args));
    said = py_call(&line_of, &[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = mcall(&g__MY_AGE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__FORGETTING(), "search", &[text.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut line_of: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line and whatever the reading carried with it, as one lower-case string.\n\n    A follow-up is answered on the words of the line before it as well (\"thats scary\" after \"my\n    mum keeps forgetting things\" is still her memory), so a guard about *whose* forgetting it is\n    has to read the carried words too, not only the line as typed.\n    ");
    line_of = py_func("line_of", |args: &[Py]| pyrt::xrefs::xcall("compose.repair", "line_of", args));
    said = py_list(vec![py_call(&line_of, &[understood.clone()])]);
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("leans_on"), py_str("ref"), py_str("topic")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&understood.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&value.clone(), "values", &[])) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[item.clone(), py_tuple(vec![py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e))), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    words = mcall(&value.clone(), "get", &[py_str("words")]);
    if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    }
    } else {
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    }
    }
    }
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn _someone_else_is_forgetting(args: &[Py]) -> Py {
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
    let _ = py_str("Whether another adult, not the speaker, is the one forgetting. A `guard=` test.");
    text = _words_of(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__SOMEONE_ELSE_FORGETTING(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__MY_AGE(), "search", &[text.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_writer_is_the_one_forgetting(args: &[Py]) -> Py {
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
    let mut relative: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the writer is the one with the errand, so the reminder card may answer.\n\n    A guard says whether its entry may answer, so this is the positive form: the card stands\n    down when the line names somebody else as the forgetter.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__SOMEBODY_ELSE(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    relative = mcall(&g__A_RELATIVE(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = relative.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__SOMETHING_FORGOTTEN(), "search", &[slice_or_raise(&said.clone(), &mcall(&relative.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_errand_is_the_speakers_own(args: &[Py]) -> Py {
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
    let _ = py_str("False when the forgetting is somebody else's, so `parent-memory` answers it instead.\n\n    The job shape stays here: \"my mum keeps forgetting to pay her bills\" and \"my brother keeps\n    forgetting to ring me\" name a task with \"to\", and that is what this card tells you how to\n    remember, whoever ends up doing it.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__ANOTHER_PERSONS_MEMORY(), "search", &[said.clone()])))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bto\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.habits".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EXTRA_ENTRIES" => Some(g_EXTRA_ENTRIES()),
            "FEEL_BAD" => Some(g_FEEL_BAD()),
            "HABIT_TIME_WORDS" => Some(g_HABIT_TIME_WORDS()),
            "HABIT_WORDS" => Some(g_HABIT_WORDS()),
            "MOUTH_TIP" => Some(g_MOUTH_TIP()),
            "NOT_A_SMALL_HABIT" => Some(g_NOT_A_SMALL_HABIT()),
            "PET_SUBJECT" => Some(g_PET_SUBJECT()),
            "SMALL_HABITS" => Some(g_SMALL_HABITS()),
            "SMALL_HABIT_ASK" => Some(g_SMALL_HABIT_ASK()),
            "SMALL_HABIT_ENTRIES" => Some(g_SMALL_HABIT_ENTRIES()),
            "SMALL_HABIT_EXTRA_AVOID" => Some(g_SMALL_HABIT_EXTRA_AVOID()),
            "STOP" => Some(g_STOP()),
            "STOP_SMALL" => Some(g_STOP_SMALL()),
            "THIRD_PARTY" => Some(g_THIRD_PARTY()),
            "WORRY_ASK" => Some(g_WORRY_ASK()),
            "_ANOTHER_ADULT" => Some(g__ANOTHER_ADULT()),
            "_ANOTHER_PERSONS_MEMORY" => Some(g__ANOTHER_PERSONS_MEMORY()),
            "_A_RELATIVE" => Some(g__A_RELATIVE()),
            "_FORGETTING" => Some(g__FORGETTING()),
            "_FORGOTTEN" => Some(g__FORGOTTEN()),
            "_MY_AGE" => Some(g__MY_AGE()),
            "_PET_ANY" => Some(g__PET_ANY()),
            "_PUTTING_A_CHARGE_UP" => Some(g__PUTTING_A_CHARGE_UP()),
            "_SOMEBODY_ELSE" => Some(g__SOMEBODY_ELSE()),
            "_SOMEONE_ELSE_FORGETTING" => Some(g__SOMEONE_ELSE_FORGETTING()),
            "_SOMETHING_FORGOTTEN" => Some(g__SOMETHING_FORGOTTEN()),
            "_about_a_pet" => Some(py_func("_about_a_pet", |args: &[Py]| _about_a_pet(args))),
            "_mouth_tip" => Some(py_func("_mouth_tip", |args: &[Py]| _mouth_tip(args))),
            "_my_own_memory_and_my_age" => Some(py_func("_my_own_memory_and_my_age", |args: &[Py]| _my_own_memory_and_my_age(args))),
            "_pet_row_answers_before_it" => Some(py_func("_pet_row_answers_before_it", |args: &[Py]| _pet_row_answers_before_it(args))),
            "_someone_else_is_forgetting" => Some(py_func("_someone_else_is_forgetting", |args: &[Py]| _someone_else_is_forgetting(args))),
            "_the_errand_is_the_speakers_own" => Some(py_func("_the_errand_is_the_speakers_own", |args: &[Py]| _the_errand_is_the_speakers_own(args))),
            "_the_writer_is_the_one_forgetting" => Some(py_func("_the_writer_is_the_one_forgetting", |args: &[Py]| _the_writer_is_the_one_forgetting(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            "a_thing_has_stopped" => Some(g_a_thing_has_stopped()),
            "entry" => Some(g_entry()),
            "screens_are_somebody_elses" => Some(g_screens_are_somebody_elses()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "EXTRA_ENTRIES" => s_EXTRA_ENTRIES(v),
            "FEEL_BAD" => s_FEEL_BAD(v),
            "HABIT_TIME_WORDS" => s_HABIT_TIME_WORDS(v),
            "HABIT_WORDS" => s_HABIT_WORDS(v),
            "MOUTH_TIP" => s_MOUTH_TIP(v),
            "NOT_A_SMALL_HABIT" => s_NOT_A_SMALL_HABIT(v),
            "PET_SUBJECT" => s_PET_SUBJECT(v),
            "SMALL_HABITS" => s_SMALL_HABITS(v),
            "SMALL_HABIT_ASK" => s_SMALL_HABIT_ASK(v),
            "SMALL_HABIT_ENTRIES" => s_SMALL_HABIT_ENTRIES(v),
            "SMALL_HABIT_EXTRA_AVOID" => s_SMALL_HABIT_EXTRA_AVOID(v),
            "STOP" => s_STOP(v),
            "STOP_SMALL" => s_STOP_SMALL(v),
            "THIRD_PARTY" => s_THIRD_PARTY(v),
            "WORRY_ASK" => s_WORRY_ASK(v),
            "_ANOTHER_ADULT" => s__ANOTHER_ADULT(v),
            "_ANOTHER_PERSONS_MEMORY" => s__ANOTHER_PERSONS_MEMORY(v),
            "_A_RELATIVE" => s__A_RELATIVE(v),
            "_FORGETTING" => s__FORGETTING(v),
            "_FORGOTTEN" => s__FORGOTTEN(v),
            "_MY_AGE" => s__MY_AGE(v),
            "_PET_ANY" => s__PET_ANY(v),
            "_PUTTING_A_CHARGE_UP" => s__PUTTING_A_CHARGE_UP(v),
            "_SOMEBODY_ELSE" => s__SOMEBODY_ELSE(v),
            "_SOMEONE_ELSE_FORGETTING" => s__SOMEONE_ELSE_FORGETTING(v),
            "_SOMETHING_FORGOTTEN" => s__SOMETHING_FORGOTTEN(v),
            "a_thing_has_stopped" => s_a_thing_has_stopped(v),
            "entry" => s_entry(v),
            "screens_are_somebody_elses" => s_screens_are_somebody_elses(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.habits", module_obj);
    pyrt::xrefs::reg_fn("knowledge.habits", "_about_a_pet", py_func("_about_a_pet", |args: &[Py]| _about_a_pet(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_pet_row_answers_before_it", py_func("_pet_row_answers_before_it", |args: &[Py]| _pet_row_answers_before_it(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_mouth_tip", py_func("_mouth_tip", |args: &[Py]| _mouth_tip(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_my_own_memory_and_my_age", py_func("_my_own_memory_and_my_age", |args: &[Py]| _my_own_memory_and_my_age(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_words_of", py_func("_words_of", |args: &[Py]| _words_of(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_someone_else_is_forgetting", py_func("_someone_else_is_forgetting", |args: &[Py]| _someone_else_is_forgetting(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_the_writer_is_the_one_forgetting", py_func("_the_writer_is_the_one_forgetting", |args: &[Py]| _the_writer_is_the_one_forgetting(args)));
    pyrt::xrefs::reg_fn("knowledge.habits", "_the_errand_is_the_speakers_own", py_func("_the_errand_is_the_speakers_own", |args: &[Py]| _the_errand_is_the_speakers_own(args)));
    pyrt::xrefs::reg_access("knowledge.habits", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.habits", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.habits", "EXTRA_ENTRIES", g_EXTRA_ENTRIES, s_EXTRA_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.habits", "FEEL_BAD", g_FEEL_BAD, s_FEEL_BAD);
    pyrt::xrefs::reg_access("knowledge.habits", "HABIT_TIME_WORDS", g_HABIT_TIME_WORDS, s_HABIT_TIME_WORDS);
    pyrt::xrefs::reg_access("knowledge.habits", "HABIT_WORDS", g_HABIT_WORDS, s_HABIT_WORDS);
    pyrt::xrefs::reg_access("knowledge.habits", "MOUTH_TIP", g_MOUTH_TIP, s_MOUTH_TIP);
    pyrt::xrefs::reg_access("knowledge.habits", "NOT_A_SMALL_HABIT", g_NOT_A_SMALL_HABIT, s_NOT_A_SMALL_HABIT);
    pyrt::xrefs::reg_access("knowledge.habits", "PET_SUBJECT", g_PET_SUBJECT, s_PET_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.habits", "SMALL_HABITS", g_SMALL_HABITS, s_SMALL_HABITS);
    pyrt::xrefs::reg_access("knowledge.habits", "SMALL_HABIT_ASK", g_SMALL_HABIT_ASK, s_SMALL_HABIT_ASK);
    pyrt::xrefs::reg_access("knowledge.habits", "SMALL_HABIT_ENTRIES", g_SMALL_HABIT_ENTRIES, s_SMALL_HABIT_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.habits", "SMALL_HABIT_EXTRA_AVOID", g_SMALL_HABIT_EXTRA_AVOID, s_SMALL_HABIT_EXTRA_AVOID);
    pyrt::xrefs::reg_access("knowledge.habits", "STOP", g_STOP, s_STOP);
    pyrt::xrefs::reg_access("knowledge.habits", "STOP_SMALL", g_STOP_SMALL, s_STOP_SMALL);
    pyrt::xrefs::reg_access("knowledge.habits", "THIRD_PARTY", g_THIRD_PARTY, s_THIRD_PARTY);
    pyrt::xrefs::reg_access("knowledge.habits", "WORRY_ASK", g_WORRY_ASK, s_WORRY_ASK);
    pyrt::xrefs::reg_access("knowledge.habits", "_ANOTHER_ADULT", g__ANOTHER_ADULT, s__ANOTHER_ADULT);
    pyrt::xrefs::reg_access("knowledge.habits", "_ANOTHER_PERSONS_MEMORY", g__ANOTHER_PERSONS_MEMORY, s__ANOTHER_PERSONS_MEMORY);
    pyrt::xrefs::reg_access("knowledge.habits", "_A_RELATIVE", g__A_RELATIVE, s__A_RELATIVE);
    pyrt::xrefs::reg_access("knowledge.habits", "_FORGETTING", g__FORGETTING, s__FORGETTING);
    pyrt::xrefs::reg_access("knowledge.habits", "_FORGOTTEN", g__FORGOTTEN, s__FORGOTTEN);
    pyrt::xrefs::reg_access("knowledge.habits", "_MY_AGE", g__MY_AGE, s__MY_AGE);
    pyrt::xrefs::reg_access("knowledge.habits", "_PET_ANY", g__PET_ANY, s__PET_ANY);
    pyrt::xrefs::reg_access("knowledge.habits", "_PUTTING_A_CHARGE_UP", g__PUTTING_A_CHARGE_UP, s__PUTTING_A_CHARGE_UP);
    pyrt::xrefs::reg_access("knowledge.habits", "_SOMEBODY_ELSE", g__SOMEBODY_ELSE, s__SOMEBODY_ELSE);
    pyrt::xrefs::reg_access("knowledge.habits", "_SOMEONE_ELSE_FORGETTING", g__SOMEONE_ELSE_FORGETTING, s__SOMEONE_ELSE_FORGETTING);
    pyrt::xrefs::reg_access("knowledge.habits", "_SOMETHING_FORGOTTEN", g__SOMETHING_FORGOTTEN, s__SOMETHING_FORGOTTEN);
    pyrt::xrefs::reg_access("knowledge.habits", "a_thing_has_stopped", g_a_thing_has_stopped, s_a_thing_has_stopped);
    pyrt::xrefs::reg_access("knowledge.habits", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.habits", "screens_are_somebody_elses", g_screens_are_somebody_elses, s_screens_are_somebody_elses);
}
