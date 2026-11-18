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
thread_local! { static G_FOOD_THAT_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOODS_THAT_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_THAT_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SMOKING_IS_NOT_A_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_nail_is_a_persons: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_food_is_being_kept: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_screens_are_somebody_elses: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.routines: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_THAT_KEEPS() -> Py { ensure_init(); G_FOOD_THAT_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_THAT_KEEPS(v: Py) { ensure_init(); G_FOOD_THAT_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_GOODS_THAT_KEEP() -> Py { ensure_init(); G_GOODS_THAT_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOODS_THAT_KEEP(v: Py) { ensure_init(); G_GOODS_THAT_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_ASK() -> Py { ensure_init(); G_HABIT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_ASK(v: Py) { ensure_init(); G_HABIT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS_SUBJECTS() -> Py { ensure_init(); G_KEEPS_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS_SUBJECTS(v: Py) { ensure_init(); G_KEEPS_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS_VERB() -> Py { ensure_init(); G_KEEPS_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS_VERB(v: Py) { ensure_init(); G_KEEPS_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_THAT_KEEPS() -> Py { ensure_init(); G_MACHINE_THAT_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_THAT_KEEPS(v: Py) { ensure_init(); G_MACHINE_THAT_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_WORDS() -> Py { ensure_init(); G__MONEY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_WORDS(v: Py) { ensure_init(); G__MONEY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SMOKING_IS_NOT_A_HABIT() -> Py { ensure_init(); G__SMOKING_IS_NOT_A_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SMOKING_IS_NOT_A_HABIT(v: Py) { ensure_init(); G__SMOKING_IS_NOT_A_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_WORDS() -> Py { ensure_init(); G__STATE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_WORDS(v: Py) { ensure_init(); G__STATE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__the_nail_is_a_persons() -> Py { ensure_init(); G__the_nail_is_a_persons.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_nail_is_a_persons(v: Py) { ensure_init(); G__the_nail_is_a_persons.with(|c| *c.borrow_mut() = v); }
pub fn g_a_food_is_being_kept() -> Py { ensure_init(); G_a_food_is_being_kept.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_food_is_being_kept(v: Py) { ensure_init(); G_a_food_is_being_kept.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_screens_are_somebody_elses() -> Py { ensure_init(); G_screens_are_somebody_elses.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_screens_are_somebody_elses(v: Py) { ensure_init(); G_screens_are_somebody_elses.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Routines: starting one, keeping it going after a slip, and stopping one you do not want.\n\nThe answers lane owns this domain. The habits.py domain next door answers the habits people\nname (vaping, drinking); this one answers the behaviour around any habit at all, so a line that\ndoes name smoking or drinking is left to those entries by the avoid lists below. People ask\nabout habits in their own words and about anything: walking every morning, practising an\ninstrument, saving, flossing, cutting down on phone time. The useful answer is the same every\ntime — make it small, tie it to something that already happens, do not rely on feeling like it\n— so the entries here are keyed by the shape of the question (build one, keep one going after a\nslip, break one, get off the phone) rather than by the subject of the habit.\n\nNothing here repeats a domain's own facts: the training plan is sport.py's, the sleep routine is\nhealth.py's, quitting smoking and cutting down on alcohol are health's and safety's, and study\ntechnique is study.py's. This domain only holds behaviour change, which is what nobody else has.\n");
    s__the_nail_is_a_persons(py_func("_the_nail_is_a_persons", |args: &[Py]| pyrt::xrefs::xcall("knowledge.body", "_the_nail_is_a_persons", args)));
    s_screens_are_somebody_elses(py_func("screens_are_somebody_elses", |args: &[Py]| pyrt::xrefs::xcall("knowledge.wording", "screens_are_somebody_elses", args)));
    s_a_food_is_being_kept(py_func("a_food_is_being_kept", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "a_food_is_being_kept", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("health"), py_str("fitness"), py_str("school-and-study")]));
    fn __data1() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("tips"), py_str("help"), py_str("should"), py_str("can i"), py_str("best way"), py_str("keep"), py_str("keep on"), py_str("stick"), py_str("start"), py_str("stop"), py_str("break"), py_str("build"), py_str("get into"), py_str("get back"), py_str("get rid"), py_str("kick"), py_str("ditch"), py_str("want to"), py_str("trying to"), py_str("struggling"), py_str("cut down"), py_str("always"), py_str("never"), py_str("end up"), py_str("stay"), py_str("broke"), py_str("broken"), py_str("breaking"), py_str("slipped"), py_str("slip"), py_str("failed"), py_str("fell off"), py_str("blew it"), py_str("missed"), py_str("miss a day"), py_str("missed a day"), py_str("skipped"), py_str("skip a")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.HABIT_ASK"); }
    s_HABIT_ASK(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.MACHINE_THAT_KEEPS"); }
    s_MACHINE_THAT_KEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:washing machine|tumble dryer|dishwasher|extractor fan|garage door|door bell|doorbell|boiler|radiator|heating|hot water|shower|tap|toilet|cistern|fridge|freezer|oven|hob|microwave|kettle|toaster|vacuum|hoover|printer|router|wifi|broadband|laptop|phone|computer|screen|monitor|tablet|tv|television|camera|watch|headphones|earbuds|battery|charger|socket|plug|bulb|lights|fan|car|van|bike|engine|alarm|lock|window)s?\\s+(?:keeps|keep|kept)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.FOOD_THAT_KEEPS"); }
    s_FOOD_THAT_KEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:rice|pasta|noodles?|bread|dough|batter|cake|cakes|pastry|sauce|gravy|soup|stew|curry|porridge|oats|custard|jam|chocolate|caramel|fudge|icing|meringue|potato|potatoes|chips|meat|chicken|fish|egg|eggs|yoghurt|yogurt|milk|cream|tofu|beans|lentils|couscous|quinoa|pancakes?|scones?|biscuits?|crust|pizza|mayo|mayonnaise|hollandaise|risotto|stir fry|mash|omelette|scrambled eggs|steak|bacon|sausages?)s?\\s+(?:keeps|keep|kept)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines._STATE_WORDS"); }
    s__STATE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:cold|lukewarm|warm|hot|stale|staler|mouldy|moldy|off|flat|soggy|dry|curdled|lumpy|burnt|burned)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.GOODS_THAT_KEEP"); }
    s_GOODS_THAT_KEEP(stdlib_call("re", "compile", &[py_str("\\b(?:tea|coffee|soup|broth|stew|dinner|food|milk|bread|loaf|toast|cake|cuppa|brew|hot chocolate|porridge|rice|pasta|gravy|sauce|drink|pint|beer|wine|juice|cup|mug|pot|bath|shower water|washing|laundry)\\b\\s+(?:keeps|keep|kept)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("they"), py_str("he"), py_str("she"), py_str("people"), py_str("everyone"), py_str("habit"), py_str("habits"), py_str("routine"), py_str("routines"), py_str("streak"), py_str("streaks"), py_str("resolution"), py_str("resolutions"), py_str("goal"), py_str("goals"), py_str("plan"), py_str("plans"), py_str("planning"), py_str("effort"), py_str("efforts"), py_str("attempt"), py_str("attempts"), py_str("progress"), py_str("motivation"), py_str("discipline"), py_str("willpower"), py_str("schedule"), py_str("diary"), py_str("calendar"), py_str("app"), py_str("tracker"), py_str("journal"), py_str("challenge"), py_str("programme"), py_str("program"), py_str("training"), py_str("practice"), py_str("savings"), py_str("budget"), py_str("diet"), py_str("weight"), py_str("sleep"), py_str("mood"), py_str("mind"), py_str("thoughts"), py_str("anxiety"), py_str("work"), py_str("study"), py_str("revision"), py_str("job"), py_str("career"), py_str("friendship"), py_str("friendships"), py_str("marriage"), py_str("relationship"), py_str("sobriety"), py_str("quit"), py_str("quitting"), py_str("smoking"), py_str("drinking"), py_str("gambling"), py_str("walking"), py_str("running"), py_str("swimming"), py_str("cycling"), py_str("running plan"), py_str("writing"), py_str("reading"), py_str("meditation"), py_str("exercise"), py_str("exercising"), py_str("yoga"), py_str("gym"), py_str("cooking"), py_str("tidying")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.KEEPS_SUBJECTS"); }
    s_KEEPS_SUBJECTS(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.KEEPS_VERB"); }
    s_KEEPS_VERB(stdlib_call("re", "compile", &[py_str("\\b([a-z]+)\\s+(?:keeps|keep|kept)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("smoke"), py_str("smoking"), py_str("smoker"), py_str("smokers"), py_str("smoked"), py_str("cigarette"), py_str("cigarettes"), py_str("fag"), py_str("fags"), py_str("vape"), py_str("vapes"), py_str("vaping"), py_str("nicotine"), py_str("tobacco"), py_str("e cigarette"), py_str("e-cigarette")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines._SMOKING_IS_NOT_A_HABIT"); }
    s__SMOKING_IS_NOT_A_HABIT(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("pay"), py_str("pays"), py_str("paying"), py_str("pay for"), py_str("paid"), py_str("pays for"), py_str("payment"), py_str("bill"), py_str("billed"), py_str("billing"), py_str("subscription"), py_str("subscriptions"), py_str("subscribed"), py_str("cost"), py_str("costs"), py_str("costing"), py_str("price"), py_str("prices"), py_str("priced"), py_str("charge"), py_str("charged"), py_str("charges"), py_str("refund"), py_str("refunded"), py_str("cancel"), py_str("cancelled"), py_str("canceled"), py_str("cancel my"), py_str("free trial"), py_str("upgrade"), py_str("premium"), py_str("monthly"), py_str("direct debit"), py_str("invoice")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines._MONEY_WORDS"); }
    s__MONEY_WORDS(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("drink"), py_str("alcohol"), py_str("drug"), py_str("gambling"), py_str("gamble"), py_str("sugar"), py_str("diet"), py_str("lose weight"), py_str("quit"), py_str("nail"), py_str("biting"), py_str("thumb"), py_str("bedtime"), py_str("nap"), py_str("baby"), py_str("toddler"), py_str("kids"), py_str("child"), py_str("puppy"), py_str("toilet"), py_str("cleaning"), py_str("clean the"), py_str("tidy"), py_str("chores"), py_str("food"), py_str("eat"), py_str("eating"), py_str("sugar"), py_str("workout"), py_str("workouts"), py_str("gym"), py_str("exercise"), py_str("exercises"), py_str("training"), py_str("study"), py_str("studying"), py_str("revision"), py_str("revise"), py_str("revising"), py_str("sleep"), py_str("morning"), py_str("mornings"), py_str("night"), py_str("nights"), py_str("evening"), py_str("evenings"), py_str("biscuit"), py_str("biscuits"), py_str("chocolate"), py_str("snack"), py_str("snacks"), py_str("salt"), py_str("caffeine"), py_str("carb"), py_str("carbs"), py_str("dairy"), py_str("meat"), py_str("takeaway")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("habit"), py_str("habits"), py_str("streak"), py_str("streaks"), py_str("routine"), py_str("resolution"), py_str("resolutions"), py_str("goal"), py_str("goals"), py_str("keep starting"), py_str("make it stick"), py_str("make it stick this time"), py_str("stick with it"), py_str("stick at it"), py_str("keep it up"), py_str("keep going"), py_str("keep at it"), py_str("dont give up"), py_str("not give up"), py_str("stay on it"), py_str("starting and stopping"), py_str("started and stopped"), py_str("give up"), py_str("gave up"), py_str("keep failing"), py_str("keep messing up"), py_str("my streak"), py_str("the streak"), py_str("my habit"), py_str("the habit"), py_str("my routine"), py_str("my resolution"), py_str("new years resolution"), py_str("new year resolution")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("keep starting"), py_str("starting and stopping"), py_str("started and stopped"), py_str("give up"), py_str("gave up"), py_str("lose it"), py_str("lost it"), py_str("slip"), py_str("slipped"), py_str("missed"), py_str("miss a"), py_str("keep failing"), py_str("again"), py_str("failed"), py_str("fail"), py_str("restart"), py_str("back on track"), py_str("cant stick"), py_str("can't stick"), py_str("keep"), py_str("stick"), py_str("make it stick"), py_str("carry on"), py_str("keep on"), py_str("keep going"), py_str("lose"), py_str("lost"), py_str("going"), py_str("broken"), py_str("broke"), py_str("break"), py_str("breaking"), py_str("reset"), py_str("broke my streak"), py_str("break my streak"), py_str("breaking my streak"), py_str("broke the streak"), py_str("breaks my streak"), py_str("my streak broke"), py_str("lost my streak"), py_str("ruined my streak"), py_str("streak broke"), py_str("missed my streak"), py_str("missed the streak"), py_str("missed a day"), py_str("after a week"), py_str("after a month"), py_str("after a few weeks"), py_str("week"), py_str("weeks"), py_str("month"), py_str("months"), py_str("soon"), py_str("every time"), py_str("stay"), py_str("stay on track"), py_str("on track"), py_str("stick with"), py_str("with my")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("my serve"), py_str("serve keeps"), py_str("serves keep"), py_str("serving"), py_str("my swing"), py_str("swing keeps"), py_str("golf swing"), py_str("putt"), py_str("putts"), py_str("forehand"), py_str("backhand"), py_str("volley"), py_str("batting"), py_str("darts"), py_str("golf"), py_str("tennis"), py_str("badminton"), py_str("squash"), py_str("cricket"), py_str("netball")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("alcohol"), py_str("drink"), py_str("drug"), py_str("gambling"), py_str("diet"), py_str("lose weight"), py_str("quit"), py_str("nail"), py_str("biting"), py_str("bedtime"), py_str("sleep"), py_str("baby"), py_str("toddler"), py_str("kids"), py_str("child"), py_str("food"), py_str("eat"), py_str("eating"), py_str("sugar"), py_str("biscuit"), py_str("biscuits"), py_str("chocolate"), py_str("snack"), py_str("snacks"), py_str("salt"), py_str("caffeine"), py_str("carb"), py_str("carbs"), py_str("dairy"), py_str("meat"), py_str("takeaway"), py_str("cancer"), py_str("diagnosis"), py_str("diagnosed"), py_str("chemo"), py_str("hospice"), py_str("terminally"), py_str("bereaved"), py_str("funeral"), py_str("died"), py_str("dying"), py_str("going over"), py_str("over it"), py_str("replaying"), py_str("in my head"), py_str("going round"), py_str("maths"), py_str("math"), py_str("english"), py_str("science"), py_str("biology"), py_str("chemistry"), py_str("physics"), py_str("history"), py_str("geography"), py_str("french"), py_str("spanish"), py_str("german"), py_str("revision"), py_str("revise"), py_str("revising"), py_str("exam"), py_str("exams"), py_str("past paper"), py_str("past papers"), py_str("mock"), py_str("mocks"), py_str("my grades"), py_str("my marks"), py_str("gcse"), py_str("a level"), py_str("a levels"), py_str("driving test"), py_str("driving lesson"), py_str("driving lessons"), py_str("driving instructor"), py_str("car"), py_str("cars"), py_str("van"), py_str("mot"), py_str("garage"), py_str("engine"), py_str("boiler"), py_str("radiator"), py_str("washing machine"), py_str("dishwasher"), py_str("tumble dryer"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("worth fixing"), py_str("worth repairing"), py_str("keeps going black"), py_str("going black"), py_str("goes black"), py_str("went black"), py_str("rent"), py_str("rents"), py_str("council tax"), py_str("my tax"), py_str("taxes"), py_str("the price of"), py_str("prices"), py_str("price"), py_str("bills"), py_str("my bills"), py_str("energy bill"), py_str("cost of living"), py_str("interest rate"), py_str("interest rates"), py_str("fares"), py_str("mortgage"), py_str("inflation"), py_str("keeps going flat"), py_str("going flat"), py_str("keeps going dark"), py_str("keeps going dim"), py_str("black screen"), py_str("blank screen"), py_str("keeps going blank"), py_str("going blank"), py_str("keeps flickering"), py_str("flickering"), py_str("flickers"), py_str("keeps turning itself off"), py_str("turns itself off"), py_str("keeps restarting"), py_str("restarting itself"), py_str("keeps freezing"), py_str("keeps crashing"), py_str("wont turn on"), py_str("wont switch on"), py_str("wont come on"), py_str("keeps dropping"), py_str("keeps disconnecting"), py_str("keeps cutting out"), py_str("no signal"), py_str("screen keeps going"), py_str("screen going black"), py_str("screen going blank"), py_str("running"), py_str("jogging"), py_str("swimming"), py_str("cycling"), py_str("walking"), py_str("learning"), py_str("revision"), py_str("revising"), py_str("study"), py_str("studying"), py_str("keeps going off"), py_str("keeps going out"), py_str("keeps coming back"), py_str("keeps dropping"), py_str("keeps going blank"), py_str("gone blank"), py_str("goes blank"), py_str("keeps going fuzzy"), py_str("keeps losing the picture"), py_str("keeps breaking up"), py_str("keeps going dark"), py_str("keeps leaking"), py_str("keeps tripping"), py_str("keeps cutting out"), py_str("keeps beeping"), py_str("keeps flashing"), py_str("keeps stopping"), py_str("keeps jamming"), py_str("will that get better"), py_str("will it get better"), py_str("will this get better"), py_str("will they get better"), py_str("does it get better"), py_str("does that get better"), py_str("get better on its own"), py_str("clear up on its own"), py_str("go away on its own"), py_str("get better if i"), py_str("if i keep at it"), py_str("telly"), py_str("going off"), py_str("switches off"), py_str("switching off"), py_str("turns off"), py_str("turning off"), py_str("power"), py_str("electricity"), py_str("lights"), py_str("socket")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("break"), py_str("stop"), py_str("quit"), py_str("cut down"), py_str("cut back"), py_str("give up"), py_str("get rid"), py_str("kick"), py_str("ditch"), py_str("less"), py_str("reduce"), py_str("bad"), py_str("worse"), py_str("control")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("alcohol"), py_str("drinking"), py_str("drunk"), py_str("drug"), py_str("drugs"), py_str("weed"), py_str("gambling"), py_str("gamble"), py_str("betting"), py_str("porn"), py_str("sugar"), py_str("sugary"), py_str("caffeine"), py_str("coffee"), py_str("nail"), py_str("biting"), py_str("thumb"), py_str("skin"), py_str("picking"), py_str("hair"), py_str("food"), py_str("eat"), py_str("eating"), py_str("sugar"), py_str("biscuit"), py_str("biscuits"), py_str("chocolate"), py_str("snack"), py_str("snacks"), py_str("salt"), py_str("caffeine"), py_str("carbs"), py_str("dairy"), py_str("meat"), py_str("takeaway"), py_str("phone"), py_str("screen")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("tips"), py_str("help"), py_str("stop"), py_str("can i"), py_str("keep"), py_str("always"), py_str("cant stop"), py_str("can't stop"), py_str("want to"), py_str("i bite"), py_str("i chew"), py_str("i bit"), py_str("i keep biting"), py_str("i keep chewing"), py_str("i always bite"), py_str("i always chew"), py_str("i nibble"), py_str("i keep nibbling"), py_str("bite my nails"), py_str("biting my nails"), py_str("chew my nails"), py_str("chewing my nails"), py_str("nail biting"), py_str("nails biting"), py_str("bitten"), py_str("keep biting my nails"), py_str("want to stop biting"), py_str("cant stop biting"), py_str("can't stop biting"), py_str("biting them")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("yellow"), py_str("thick"), py_str("thickened"), py_str("thickening"), py_str("crumbly"), py_str("crumbling"), py_str("crumbles"), py_str("discoloured"), py_str("discolored"), py_str("brown"), py_str("white spots"), py_str("ridging"), py_str("ridges"), py_str("fungus"), py_str("fungal"), py_str("finger"), py_str("thumb is"), py_str("toe"), py_str("toenail"), py_str("fungal"), py_str("infection"), py_str("painting"), py_str("polish colour"), py_str("gel"), py_str("acrylic"), py_str("broken nail"), py_str("split"), py_str("bruised"), py_str("bruise"), py_str("black"), py_str("blue"), py_str("dropped"), py_str("slammed"), py_str("hammer"), py_str("caught it"), py_str("fell off"), py_str("lifting"), py_str("lifted"), py_str("sore"), py_str("throbbing"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("inflamed"), py_str("red and"), py_str("abscess"), py_str("bleeding"), py_str("doctor"), py_str("gp"), py_str("pharmacist"), py_str("my child"), py_str("my children"), py_str("my son"), py_str("my daughter"), py_str("my boy"), py_str("my girl"), py_str("my toddler"), py_str("my teenager"), py_str("their nails"), py_str("they bite"), py_str("he bites"), py_str("she bites"), py_str("his nails"), py_str("her nails")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("phone"), py_str("scroll"), py_str("scrolling"), py_str("doomscroll"), py_str("doomscrolling"), py_str("notifications"), py_str("instagram"), py_str("facebook"), py_str("snapchat"), py_str("tiktok"), py_str("twitter"), py_str("reddit"), py_str("youtube"), py_str("whatsapp"), py_str("linkedin"), py_str("social media"), py_str("socials")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("distract"), py_str("distracting"), py_str("distracted"), py_str("distraction"), py_str("end up"), py_str("cant stop"), py_str("can't stop"), py_str("put it down"), py_str("all day"), py_str("waste"), py_str("wasting"), py_str("instead of"), py_str("too much time"), py_str("stop scrolling"), py_str("scroll"), py_str("scrolling"), py_str("at night"), py_str("before bed"), py_str("way too much"), py_str("doomscroll"), py_str("doomscrolling"), py_str("distracting me"), py_str("distracts me"), py_str("at work"), py_str("while i work"), py_str("screen time"), py_str("first thing"), py_str("in the morning"), py_str("use it less"), py_str("phone less"), py_str("reach for it"), py_str("stop"), py_str("stops"), py_str("stopping"), py_str("stayed off"), py_str("time on it"), py_str("cut down"), py_str("cutting down"), py_str("cut back"), py_str("cut down on it"), py_str("addicted to my phone"), py_str("addicted to it"), py_str("phone addiction"), py_str("always on my phone"), py_str("always on it"), py_str("on it all the time"), py_str("on my phone all"), py_str("cant put it down"), py_str("can't put it down"), py_str("my phone habit"), py_str("phone habits"), py_str("out of control"), py_str("too attached"), py_str("glued to my phone"), py_str("glued to it")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("tips"), py_str("help"), py_str("should"), py_str("can i"), py_str("keep"), py_str("stop"), py_str("cant"), py_str("can't"), py_str("ways"), py_str("distracting me"), py_str("distracts me"), py_str("keeps"), py_str("too much"), py_str("less"), py_str("says"), py_str("told me"), py_str("my phone habit")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("battery"), py_str("charger"), py_str("charging"), py_str("dying"), py_str("dies"), py_str("cracked"), py_str("screen is"), py_str("wifi"), py_str("subscription"), py_str("subscriptions"), py_str("unsubscribe"), py_str("phone bill"), py_str("direct debit"), py_str("free trial"), py_str("touchscreen"), py_str("touch screen"), py_str("stopped working"), py_str("not working"), py_str("doesnt work"), py_str("does not work"), py_str("not responding"), py_str("wont respond"), py_str("unresponsive"), py_str("frozen"), py_str("freezes"), py_str("freezing"), py_str("restart"), py_str("photo"), py_str("photos"), py_str("picture"), py_str("pictures"), py_str("camera"), py_str("pics"), py_str("video"), py_str("videos"), py_str("selfie"), py_str("selfies"), py_str("shoot"), py_str("photograph"), py_str("photographs"), py_str("zoom"), py_str("lens"), py_str("wi-fi"), py_str("sim"), py_str("contract"), py_str("storage"), py_str("storage full"), py_str("update"), py_str("water"), py_str("broken"), py_str("stolen"), py_str("new phone"), py_str("which phone"), py_str("upgrade"), py_str("kids"), py_str("kid"), py_str("child"), py_str("children"), py_str("son"), py_str("daughter"), py_str("toddler"), py_str("baby"), py_str("teenager"), py_str("waking"), py_str("wake me"), py_str("woke me"), py_str("asleep"), py_str("alarm"), py_str("ping"), py_str("pinging"), py_str("pings"), py_str("sound"), py_str("sounds"), py_str("vibrate"), py_str("vibrating"), py_str("ring"), py_str("ringing"), py_str("do not disturb"), py_str("silent"), py_str("buzz"), py_str("buzzes"), py_str("buzzing"), py_str("beep"), py_str("beeping"), py_str("beeps"), py_str("block"), py_str("blocked"), py_str("blocking"), py_str("timing out"), py_str("times out"), py_str("timed out"), py_str("timeout"), py_str("screen goes off"), py_str("goes off"), py_str("stays on"), py_str("auto lock"), py_str("autolock"), py_str("lock screen"), py_str("screen locks"), py_str("notification"), py_str("notifications"), py_str("using data"), py_str("use data"), py_str("uses data"), py_str("so much data"), py_str("lots of data"), py_str("mobile data"), py_str("data usage"), py_str("data plan"), py_str("data allowance"), py_str("data cap"), py_str("data limit"), py_str("gb of data"), py_str("eating my data"), py_str("eat my data"), py_str("eats my data"), py_str("saving data"), py_str("save data"), py_str("hotspot"), py_str("tethering"), py_str("block this"), py_str("scam"), py_str("spam"), py_str("nuisance"), py_str("taxi"), py_str("lost it"), py_str("left it"), py_str("left my phone"), py_str("stolen"), py_str("find my"), py_str("mouse"), py_str("mice"), py_str("keyboard"), py_str("trackpad"), py_str("touchpad"), py_str("photo"), py_str("photos"), py_str("picture"), py_str("pictures"), py_str("photography"), py_str("camera"), py_str("cameras"), py_str("blurry"), py_str("blurred"), py_str("in focus"), py_str("zoom")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.routines.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("habit-build-routine")); __args.push(py_tuple(vec![py_tuple(vec![py_str("habit"), py_str("habits"), py_str("routine"), py_str("ritual")]), py_tuple(vec![py_str("build"), py_str("start"), py_str("make"), py_str("form"), py_str("form a"), py_str("get into"), py_str("new"), py_str("better"), py_str("good"), py_str("create"), py_str("stick")])])); __args.push(py_str("Make it too small to fail: two minutes, one press-up, one page. Tie it to something you already do at the same time each day — after you brush your teeth, before you sit down with a coffee — so the day decides instead of your mood. Do one habit at a time for a month, and mark it off on a calendar you see, because seeing the chain is what keeps it going when the first enthusiasm runs out.")); __kws.extend(vec![("ask".to_string(), g_HABIT_ASK()), ("avoid".to_string(), op_add(&g__SMOKING_IS_NOT_A_HABIT(), &__data5()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("habit-restart")); __args.push(py_tuple(vec![__data6(), __data7()])); __args.push(py_str("The rule that keeps a habit alive is never miss twice: one missed day is a rest, two is how a habit ends, so the day after a slip you do the smallest version — one minute, one page — and the chain starts again. Plan that now, before the bad week, and cut the habit to a size that survives it rather than resetting to the version you could only do while life was calm. A streak you broke is not a score you lost: keep the total visible, mark today, and carry on from there instead of waiting for a fresh Monday.")); __kws.extend(vec![("ask".to_string(), g_HABIT_ASK()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_a_thing_behaving_badly(&[understood.clone()]))) })), ("avoid".to_string(), op_add(&op_add(&__data8(), &g__SMOKING_IS_NOT_A_HABIT()), &__data9()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("habit-break")); __args.push(py_tuple(vec![py_tuple(vec![py_str("habit"), py_str("habits"), py_str("routine"), py_str("cue"), py_str("trigger"), py_str("willpower")]), __data10()])); __args.push(py_str("Work on the trigger rather than the willpower: note when the habit happens for three days, then change that moment — the biscuit tin out of the cupboard, the phone charging in the hall, a different route past the shop. Have a replacement ready for the same slot, because an empty slot fills itself. When the urge arrives, wait it out for ten minutes; urges peak and fall instead of building, so the ten minutes is the whole trick.")); __kws.extend(vec![("ask".to_string(), g_HABIT_ASK()), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g__SMOKING_IS_NOT_A_HABIT(), &__data11()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nail-biting")); __args.push(py_tuple(vec![py_tuple(vec![py_str("nail"), py_str("nails"), py_str("nail biting"), py_str("biting my nails"), py_str("nibbling my nails"), py_str("nibble my nails"), py_str("nibbled nails")]), py_tuple(vec![py_str("bite"), py_str("biting"), py_str("bit"), py_str("bitten"), py_str("stop"), py_str("chew"), py_str("chewing"), py_str("habit"), py_str("help")])])); __args.push(py_str("Keep them short and filed, because a ragged edge is what a tooth finds. Notice the moment you start — driving, on the phone, reading — and give that hand something else to do: a ring to twist, a pen, a stress ball. A bitter-tasting polish from the pharmacy works because it makes you aware of the bite, not because it is unpleasant; the point is the pause, and the habit fades once you notice it often enough.")); __kws.extend(vec![("ask".to_string(), __data12()), ("guard".to_string(), py_func("_the_nail_is_a_persons", |args: &[Py]| pyrt::xrefs::xcall("knowledge.body", "_the_nail_is_a_persons", args))), ("avoid".to_string(), __data13())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("phone-distraction")); __args.push(py_tuple(vec![__data14(), __data15()])); __args.push(py_str("Take the choice out of the moment rather than counting on willpower: put the phone in another room while you get on with something, turn off everything except calls, and charge it outside the bedroom so the first and last hour of the day are phone-free. If you want a handbrake, set a screen-time limit and let someone else hold the passcode, since a limit you can dismiss is not a limit.")); __kws.extend(vec![("ask".to_string(), __data16()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); { let __b = _not_about_money(&[reading.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.wording", "screens_are_somebody_elses", &[reading.clone()]))) } } })), ("avoid".to_string(), __data17())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    fn __data18() -> Py { py_tuple(vec![py_str("rush"), py_str("rushed"), py_str("rushing"), py_str("late"), py_str("always late"), py_str("running late"), py_str("chaos"), py_str("hectic"), py_str("stress"), py_str("stressful"), py_str("smoother"), py_str("calmer"), py_str("easier"), py_str("slow"), py_str("routine"), py_str("tips"), py_str("plan"), py_str("organised"), py_str("organized"), py_str("time"), py_str("quicker"), py_str("faster"), py_str("start")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("refuses"), py_str("refusing"), py_str("wont go"), py_str("won't go"), py_str("school refusal"), py_str("anxiety"), py_str("wont get up"), py_str("won't get up"), py_str("cant wake"), py_str("can't wake"), py_str("oversleep"), py_str("sleep through"), py_str("insomnia"), py_str("wake up early"), py_str("waking up"), py_str("night shift"), py_str("shift work"), py_str("commute"), py_str("traffic"), py_str("train times"), py_str("bus times"), py_str("missed the bus"), py_str("alarm clock"), py_str("snooze"), py_str("complain"), py_str("complaining"), py_str("complaint"), py_str("complaints"), py_str("refund"), py_str("delay repay"), py_str("cancelled"), py_str("canceled"), py_str("cancellation"), py_str("delayed"), py_str("replacement bus"), py_str("depressed"), py_str("depression"), py_str("exhausted"), py_str("burnout"), py_str("baby"), py_str("newborn"), py_str("toddler"), py_str("night feeds")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("morning-rush")); __args.push(py_tuple(vec![py_tuple(vec![py_str("morning"), py_str("mornings"), py_str("school run"), py_str("get out the door"), py_str("out the door"), py_str("before work"), py_str("before school"), py_str("getting ready")]), __data18()])); __args.push(py_str("The night before does most of it: clothes out, bag and keys by the door, breakfast and lunch decided, and the alarm set for the time you actually need rather than the time you hope for. In the morning keep only three things that must happen - washed and dressed, something eaten, out of the door - and let the rest wait, because a fixed order you do without deciding is what stops the rush. Give yourself fifteen minutes more than you think you need for the first week, and leave the phone face down until you are out, because one scroll is where the last ten minutes go.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("tips"), py_str("help"), py_str("should"), py_str("ways"), py_str("make"), py_str("stop"), py_str("less"), py_str("cant"), py_str("can't")])), ("avoid".to_string(), __data19()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _a_machine_behaving_badly(args: &[Py]) -> Py {
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
    let mut field: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Whether the line is a thing that keeps doing something, not a habit of the person's.");
    if py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "a_food_is_being_kept", &[understood.clone()])) {
    return Py::Bool(true);
    }
    said = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    field = __item.clone();
    value = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[field.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { value.clone() } }) {
    said = value.clone();
    break;
    }
    }
    if py_truthy(&mcall(&g_MACHINE_THAT_KEEPS(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_GOODS_THAT_KEEP(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__STATE_WORDS(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_thing_behaving_badly(args: &[Py]) -> Py {
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
    let mut field: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subjects: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a machine or a pot of food keeps doing something, rather than a habit.\n\n    Both are the same reading of \"keeps\": the subject in front of the verb is a thing going\n    wrong, so the fault card owns the line and the habit card stands down. A person is still a\n    person - \"i keep burning the rice\" names the rice after the verb, not before it.\n    ");
    if py_truthy(&_a_machine_behaving_badly(&[understood.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    field = __item.clone();
    value = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[field.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { value.clone() } }) {
    said = mcall(&value.clone(), "lower", &[]);
    if py_truthy(&mcall(&g_FOOD_THAT_KEEPS(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    subjects = mcall(&g_KEEPS_VERB(), "findall", &[said.clone()]);
    if py_truthy(&{ let __b = subjects.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&subjects.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_KEEPS_SUBJECTS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _not_about_money(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when the line asks about paying, so the habit rows stand down for it.");
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("text"), py_str("raw")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MONEY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.routines".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FOOD_THAT_KEEPS" => Some(g_FOOD_THAT_KEEPS()),
            "GOODS_THAT_KEEP" => Some(g_GOODS_THAT_KEEP()),
            "HABIT_ASK" => Some(g_HABIT_ASK()),
            "KEEPS_SUBJECTS" => Some(g_KEEPS_SUBJECTS()),
            "KEEPS_VERB" => Some(g_KEEPS_VERB()),
            "MACHINE_THAT_KEEPS" => Some(g_MACHINE_THAT_KEEPS()),
            "_MONEY_WORDS" => Some(g__MONEY_WORDS()),
            "_SMOKING_IS_NOT_A_HABIT" => Some(g__SMOKING_IS_NOT_A_HABIT()),
            "_STATE_WORDS" => Some(g__STATE_WORDS()),
            "_a_machine_behaving_badly" => Some(py_func("_a_machine_behaving_badly", |args: &[Py]| _a_machine_behaving_badly(args))),
            "_a_thing_behaving_badly" => Some(py_func("_a_thing_behaving_badly", |args: &[Py]| _a_thing_behaving_badly(args))),
            "_not_about_money" => Some(py_func("_not_about_money", |args: &[Py]| _not_about_money(args))),
            "_the_nail_is_a_persons" => Some(g__the_nail_is_a_persons()),
            "a_food_is_being_kept" => Some(g_a_food_is_being_kept()),
            "entry" => Some(g_entry()),
            "screens_are_somebody_elses" => Some(g_screens_are_somebody_elses()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "FOOD_THAT_KEEPS" => s_FOOD_THAT_KEEPS(v),
            "GOODS_THAT_KEEP" => s_GOODS_THAT_KEEP(v),
            "HABIT_ASK" => s_HABIT_ASK(v),
            "KEEPS_SUBJECTS" => s_KEEPS_SUBJECTS(v),
            "KEEPS_VERB" => s_KEEPS_VERB(v),
            "MACHINE_THAT_KEEPS" => s_MACHINE_THAT_KEEPS(v),
            "_MONEY_WORDS" => s__MONEY_WORDS(v),
            "_SMOKING_IS_NOT_A_HABIT" => s__SMOKING_IS_NOT_A_HABIT(v),
            "_STATE_WORDS" => s__STATE_WORDS(v),
            "_the_nail_is_a_persons" => s__the_nail_is_a_persons(v),
            "a_food_is_being_kept" => s_a_food_is_being_kept(v),
            "entry" => s_entry(v),
            "screens_are_somebody_elses" => s_screens_are_somebody_elses(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.routines", module_obj);
    pyrt::xrefs::reg_fn("knowledge.routines", "_a_machine_behaving_badly", py_func("_a_machine_behaving_badly", |args: &[Py]| _a_machine_behaving_badly(args)));
    pyrt::xrefs::reg_fn("knowledge.routines", "_a_thing_behaving_badly", py_func("_a_thing_behaving_badly", |args: &[Py]| _a_thing_behaving_badly(args)));
    pyrt::xrefs::reg_fn("knowledge.routines", "_not_about_money", py_func("_not_about_money", |args: &[Py]| _not_about_money(args)));
    pyrt::xrefs::reg_access("knowledge.routines", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.routines", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.routines", "FOOD_THAT_KEEPS", g_FOOD_THAT_KEEPS, s_FOOD_THAT_KEEPS);
    pyrt::xrefs::reg_access("knowledge.routines", "GOODS_THAT_KEEP", g_GOODS_THAT_KEEP, s_GOODS_THAT_KEEP);
    pyrt::xrefs::reg_access("knowledge.routines", "HABIT_ASK", g_HABIT_ASK, s_HABIT_ASK);
    pyrt::xrefs::reg_access("knowledge.routines", "KEEPS_SUBJECTS", g_KEEPS_SUBJECTS, s_KEEPS_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.routines", "KEEPS_VERB", g_KEEPS_VERB, s_KEEPS_VERB);
    pyrt::xrefs::reg_access("knowledge.routines", "MACHINE_THAT_KEEPS", g_MACHINE_THAT_KEEPS, s_MACHINE_THAT_KEEPS);
    pyrt::xrefs::reg_access("knowledge.routines", "_MONEY_WORDS", g__MONEY_WORDS, s__MONEY_WORDS);
    pyrt::xrefs::reg_access("knowledge.routines", "_SMOKING_IS_NOT_A_HABIT", g__SMOKING_IS_NOT_A_HABIT, s__SMOKING_IS_NOT_A_HABIT);
    pyrt::xrefs::reg_access("knowledge.routines", "_STATE_WORDS", g__STATE_WORDS, s__STATE_WORDS);
    pyrt::xrefs::reg_access("knowledge.routines", "_the_nail_is_a_persons", g__the_nail_is_a_persons, s__the_nail_is_a_persons);
    pyrt::xrefs::reg_access("knowledge.routines", "a_food_is_being_kept", g_a_food_is_being_kept, s_a_food_is_being_kept);
    pyrt::xrefs::reg_access("knowledge.routines", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.routines", "screens_are_somebody_elses", g_screens_are_somebody_elses, s_screens_are_somebody_elses);
}
