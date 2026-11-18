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
pub static A_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLUB_SEARCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ILL_OR_HURT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MY_OWN_CHILDHOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RETURN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RUNS_BUT_IS_NOT_YOU: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TEAM_RETURN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TEAM_SPORTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_RETURN_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FIRST_PERSON_WITHOUT_THE_PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_CHILD() -> Py { ensure_init(); A_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = A_CHILD.write() { *w = v; } }
pub fn g_CLUB_SEARCH() -> Py { ensure_init(); CLUB_SEARCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLUB_SEARCH(v: Py) { ensure_init(); if let Ok(mut w) = CLUB_SEARCH.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g_ILL_OR_HURT() -> Py { ensure_init(); ILL_OR_HURT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ILL_OR_HURT(v: Py) { ensure_init(); if let Ok(mut w) = ILL_OR_HURT.write() { *w = v; } }
pub fn g_MY_OWN_CHILDHOOD() -> Py { ensure_init(); MY_OWN_CHILDHOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MY_OWN_CHILDHOOD(v: Py) { ensure_init(); if let Ok(mut w) = MY_OWN_CHILDHOOD.write() { *w = v; } }
pub fn g_RETURN_WORDS() -> Py { ensure_init(); RETURN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RETURN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = RETURN_WORDS.write() { *w = v; } }
pub fn g_RUNS_BUT_IS_NOT_YOU() -> Py { ensure_init(); RUNS_BUT_IS_NOT_YOU.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RUNS_BUT_IS_NOT_YOU(v: Py) { ensure_init(); if let Ok(mut w) = RUNS_BUT_IS_NOT_YOU.write() { *w = v; } }
pub fn g_TEAM_RETURN_WORDS() -> Py { ensure_init(); TEAM_RETURN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TEAM_RETURN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TEAM_RETURN_WORDS.write() { *w = v; } }
pub fn g_TEAM_SPORTS() -> Py { ensure_init(); TEAM_SPORTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TEAM_SPORTS(v: Py) { ensure_init(); if let Ok(mut w) = TEAM_SPORTS.write() { *w = v; } }
pub fn g__A_RETURN_WORD() -> Py { ensure_init(); _A_RETURN_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_RETURN_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_RETURN_WORD.write() { *w = v; } }
pub fn g__FIRST_PERSON_WITHOUT_THE_PRONOUN() -> Py { ensure_init(); _FIRST_PERSON_WITHOUT_THE_PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FIRST_PERSON_WITHOUT_THE_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = _FIRST_PERSON_WITHOUT_THE_PRONOUN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Two \"start again\" questions: running and cycling, the two people come back to most.\n\nBoth had an answer, and both were the wrong one. A line about wanting to run again reached the\ncoming-back-after-illness rule, so a person asking how to restart was told what to do if they had\nflu. A line about wanting to cycle again reached the menstrual-cycle entry, and \"start again\" on\nits own reached a laptop that keeps asking to restart.\n\n  how do i start running again                    the illness rule (\"above the neck and no\n                                                  temperature\")\n  i used to run and want to start again           \"An update is waiting and will keep asking\"\n  how do i start cycling again                    nothing at all\n  i stopped cycling and want to start again       \"An update is waiting and will keep asking\"\n  i want to get back on my bike                   \"What's stopped it happening so far?\"\n\nEvery one of them is a return, not a first go and not an illness, so the return words in the key\nare what the row asks for: without \"again\", \"back\" or \"used to\", a plain \"how do i start running\"\nis the beginner plan that already answers it well.\n\nThe answer for both is the same shape: less than you think, every other day, add a little each\nweek, and no speed for six weeks. That is what makes a comeback stick, and it is what neither the\nillness rule nor a laptop update says.\n");
    s_ANSWERS(py_tuple(vec![py_str("fitness"), py_str("sport")]));
    s_RETURN_WORDS(py_tuple(vec![py_str("again"), py_str("back on the bike"), py_str("back on my bike"), py_str("back on a bike"), py_str("back in the saddle"), py_str("restart"), py_str("restarting"), py_str("used to"), py_str("out of the habit"), py_str("lapsed"), py_str("years off"), py_str("havent"), py_str("haven't"), py_str("not run"), py_str("not ridden"), py_str("not cycled"), py_str("stopped running"), py_str("stopped cycling"), py_str("stopped jogging")]));
    s_ILL_OR_HURT(py_tuple(vec![py_str("ill"), py_str("illness"), py_str("sick"), py_str("sickness"), py_str("flu"), py_str("fever"), py_str("cold"), py_str("cough"), py_str("virus"), py_str("bug"), py_str("infection"), py_str("temperature"), py_str("chest"), py_str("hospital"), py_str("operation"), py_str("surgery"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("injury"), py_str("injured"), py_str("injuries"), py_str("sprain"), py_str("sprained"), py_str("tweak"), py_str("tweaked"), py_str("pulled"), py_str("knee"), py_str("knees"), py_str("shin"), py_str("ankle"), py_str("calf"), py_str("achilles"), py_str("back pain"), py_str("physio"), py_str("gp"), py_str("doctor"), py_str("pregnant"), py_str("pregnancy"), py_str("after the baby"), py_str("postnatal"), py_str("fracture"), py_str("fractured"), py_str("broken bone"), py_str("broken foot"), py_str("broken leg"), py_str("broken ankle")]));
    s_RUNS_BUT_IS_NOT_YOU(py_tuple(vec![py_str("toilet"), py_str("loo"), py_str("loos"), py_str("lav"), py_str("lavatory"), py_str("wc"), py_str("bog"), py_str("cistern"), py_str("flush"), py_str("ballcock"), py_str("faucet"), py_str("sink"), py_str("drain"), py_str("plughole"), py_str("shower"), py_str("hose"), py_str("sprinkler"), py_str("irrigation"), py_str("boiler"), py_str("radiator"), py_str("heater"), py_str("aircon"), py_str("air conditioning"), py_str("fridge"), py_str("freezer"), py_str("dishwasher"), py_str("washing machine"), py_str("tumble dryer"), py_str("hoover"), py_str("vacuum"), py_str("dyson"), py_str("generator"), py_str("compressor"), py_str("app"), py_str("apps"), py_str("software"), py_str("update"), py_str("updates"), py_str("computer"), py_str("laptop"), py_str("phone"), py_str("tablet"), py_str("printer"), py_str("clock"), py_str("meter"), py_str("nose"), py_str("mascara"), py_str("makeup"), py_str("paint"), py_str("ink"), py_str("glue"), py_str("candle"), py_str("campaign"), py_str("school run"), py_str("school runs"), py_str("nursery run"), py_str("errand run"), py_str("errand runs"), py_str("supermarket run"), py_str("shop run"), py_str("banks run")]));
    s_TEAM_SPORTS(py_tuple(vec![py_str("football"), py_str("rugby"), py_str("netball"), py_str("hockey"), py_str("basketball"), py_str("soccer"), py_str("gaelic"), py_str("lacrosse"), py_str("cricket"), py_str("squash"), py_str("badminton"), py_str("five a side"), py_str("5 a side"), py_str("tennis"), py_str("table tennis"), py_str("ping pong"), py_str("padel"), py_str("volleyball"), py_str("rounders"), py_str("softball"), py_str("baseball"), py_str("handball"), py_str("racketball"), py_str("racquetball")]));
    s_TEAM_RETURN_WORDS(add(&g_RETURN_WORDS(), &py_tuple(vec![py_str("play again"), py_str("playing again"), py_str("playing football again"), py_str("play football again"), py_str("back playing"), py_str("back to training"), py_str("training again"), py_str("back on the pitch"), py_str("back on the field"), py_str("take up"), py_str("take it up"), py_str("took up"), py_str("took it up"), py_str("pick it up"), py_str("pick up a racket"), py_str("havent played"), py_str("haven't played"), py_str("not played"), py_str("stopped playing"), py_str("since school"), py_str("since i was"), py_str("when i was"), py_str("back to playing"), py_str("start playing again")])));
    s_CLUB_SEARCH(py_tuple(vec![py_str("club"), py_str("clubs"), py_str("join"), py_str("joining"), py_str("near me"), py_str("taster"), py_str("trial"), py_str("trials"), py_str("where can i"), py_str("find a team"), py_str("find a club"), py_str("local team"), py_str("local club"), py_str("watch"), py_str("watching"), py_str("ticket"), py_str("tickets"), py_str("on telly"), py_str("sky sports"), py_str("fixture"), py_str("fixtures"), py_str("league table"), py_str("how much"), py_str("price"), py_str("cost")]));
    s__FIRST_PERSON_WITHOUT_THE_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanting|used to|thinking (?:of|about)|hoping to|planning to|trying to|decided to|going to start|back in|back on|took (?:it )?up|picked (?:it )?up|started playing|havent|haven't|not played|since school|since college|since uni|years off|out of the habit|lapsed)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD(stdlib_call("re", "compile", &[py_str("\\b(?:kids?|child|children|toddler|son|daughter|under-?\\d+|junior|schoolboy|schoolgirl)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MY_OWN_CHILDHOOD(stdlib_call("re", "compile", &[py_str("\\b(?:since|when|as) i was (?:a )?(?:kid|young|little|small|a boy|a girl|\\d+)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_RETURN_WORD(py_tuple(vec![py_str("again"), py_str("back"), py_str("used to"), py_str("since"), py_str("havent"), py_str("haven't"), py_str("not played"), py_str("stopped"), py_str("years off"), py_str("out of the habit"), py_str("lapsed"), py_str("restart"), py_str("any more"), py_str("no longer")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("running-again")); __args.push(py_tuple(vec![py_tuple(vec![py_str("running"), py_str("run"), py_str("jogging"), py_str("jog"), py_str("5k"), py_str("10k"), py_str("parkrun"), py_str("treadmill")]), py_tuple(vec![py_str("again"), py_str("back"), py_str("back into"), py_str("start"), py_str("starting"), py_str("restart"), py_str("restarting"), py_str("used to"), py_str("stopped running"), py_str("stop running"), py_str("havent run"), py_str("haven't run"), py_str("not run"), py_str("years"), py_str("years off"), py_str("out of the habit"), py_str("lapsed"), py_str("return"), py_str("take it up"), py_str("got back"), py_str("running again"), py_str("run again"), py_str("jog again"), py_str("shoes"), py_str("kit")])])); __args.push(py_str("Come back at about half of what you used to do: run-walk to start, one minute of easy jogging to two of walking, twenty minutes, three times a week, and add a minute of running a week for six weeks before you think about pace. Keep every run slow enough to talk, take a day off between them, and expect the first fortnight to feel harder than the numbers say. Sort the shoes first if yours have been in a cupboard for years - worn-out cushioning is what makes a comeback hurt.")); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("ask".to_string(), g_RETURN_WORDS()), ("avoid".to_string(), add(&add(&g_ILL_OR_HURT(), &g_RUNS_BUT_IS_NOT_YOU()), &py_tuple(vec![py_str("bath"), py_str("tap"), py_str("water"), py_str("engine"), py_str("car"), py_str("bus"), py_str("train"), py_str("late"), py_str("business"), py_str("shop"), py_str("errand"), py_str("machine"), py_str("ballot"), py_str("class"), py_str("course"), py_str("club"), py_str("parkrun time"), py_str("marathon"), py_str("half"), py_str("plan"), py_str("training plan"), py_str("couch to 5k"), py_str("dog"), py_str("child"), py_str("kids")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cycling-again")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cycling"), py_str("cycle"), py_str("bike"), py_str("bicycle"), py_str("pushbike"), py_str("riding")]), py_tuple(vec![py_str("again"), py_str("back"), py_str("back into"), py_str("start"), py_str("starting"), py_str("restart"), py_str("restarting"), py_str("used to"), py_str("stopped cycling"), py_str("stop cycling"), py_str("havent ridden"), py_str("haven't ridden"), py_str("not ridden"), py_str("not cycled"), py_str("years"), py_str("years off"), py_str("out of the habit"), py_str("lapsed"), py_str("return"), py_str("take it up"), py_str("got back"), py_str("cycling again"), py_str("ride again"), py_str("get back on")])])); __args.push(py_str("Ride less than you think: twenty to thirty minutes on a flat, quiet route, every other day, and add about ten minutes a week rather than going for the ride you used to do. Give the bike ten minutes first - tyres pumped hard, brakes that stop it, a chain with a drop of oil - and set the saddle so your knee is almost straight at the bottom of the pedal, because a saddle too low is what makes a first ride hurt. Stay in an easy gear and spin rather than push.")); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("ask".to_string(), g_RETURN_WORDS()), ("avoid".to_string(), add(&g_ILL_OR_HURT(), &py_tuple(vec![py_str("period"), py_str("periods"), py_str("menstrual"), py_str("monthly"), py_str("cycle day"), py_str("ovulation"), py_str("wash cycle"), py_str("washing machine"), py_str("dishwasher"), py_str("spin class"), py_str("motorbike"), py_str("motorcycle"), py_str("scooter"), py_str("moped"), py_str("engine"), py_str("puncture"), py_str("chain came off"), py_str("brakes squeal"), py_str("gears slip"), py_str("fix my bike"), py_str("how much is a bike"), py_str("which bike"), py_str("buy a bike"), py_str("child"), py_str("kids"), py_str("toddler"), py_str("balance bike"), py_str("processor"), py_str("cpu"), py_str("gpu"), py_str("chip"), py_str("chips"), py_str("ryzen"), py_str("intel"), py_str("graphics"), py_str("motherboard"), py_str("ram"), py_str("memory"), py_str("ssd"), py_str("hard drive"), py_str("charge cycle"), py_str("duty cycle"), py_str("life cycle"), py_str("product cycle"), py_str("menstrual cycle")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("team-sport-again")); __args.push(py_tuple(vec![g_TEAM_SPORTS(), g_TEAM_RETURN_WORDS()])); __args.push(py_str("Fitness and touch come back at different speeds, and it is the second one that gets people hurt. Train twice a week for a fortnight before you play any match at all, then a half, then a full one - about six weeks from the first session to playing as you did. If it was an injury that stopped you, the date is the physio's and not the calendar's: the usual guide is a week of easy training for every week you were out, and nothing at match pace until you can sprint and turn without thinking about it.")); __kws.extend(vec![("ask".to_string(), g_TEAM_RETURN_WORDS()), ("bias".to_string(), Py::Int(5)), ("guard".to_string(), g__a_return_to_team_sport()), ("avoid".to_string(), add(&add(&g_ILL_OR_HURT(), &g_CLUB_SEARCH()), &py_tuple(vec![py_str("playstation"), py_str("xbox"), py_str("video game"), py_str("games console"), py_str("fantasy"), py_str("betting"), py_str("bet"), py_str("school team"), py_str("school match"), py_str("school sports day"), py_str("butternut"), py_str("roast"), py_str("roasted"), py_str("soup"), py_str("eat"), py_str("eaten"), py_str("eating"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("recipe"), py_str("bake"), py_str("baked"), py_str("grow"), py_str("grown"), py_str("growing"), py_str("seeds"), py_str("chips")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _a_return_to_team_sport(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is the person coming back to it, not asking where to play.\n\n    The row and sport.py's taster-session card both fit \"when can i play football again\" - a sport\n    and \"again\" - and that card is the wrong answer to a when. This keeps the row to the return\n    itself: the person has to be the one playing, and nothing in the line can be about finding,\n    joining or watching a team.\n    ");
    reading = { let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CLUB_SEARCH()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_CHILD(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MY_OWN_CHILDHOOD(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_RETURN_WORD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:i|im|i'm|ive|i've|we|my)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__FIRST_PERSON_WITHOUT_THE_PRONOUN(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.comeback".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "A_CHILD" => Some(g_A_CHILD()),
            "CLUB_SEARCH" => Some(g_CLUB_SEARCH()),
            "ENTRIES" => Some(g_ENTRIES()),
            "ILL_OR_HURT" => Some(g_ILL_OR_HURT()),
            "MY_OWN_CHILDHOOD" => Some(g_MY_OWN_CHILDHOOD()),
            "RETURN_WORDS" => Some(g_RETURN_WORDS()),
            "RUNS_BUT_IS_NOT_YOU" => Some(g_RUNS_BUT_IS_NOT_YOU()),
            "TEAM_RETURN_WORDS" => Some(g_TEAM_RETURN_WORDS()),
            "TEAM_SPORTS" => Some(g_TEAM_SPORTS()),
            "_A_RETURN_WORD" => Some(g__A_RETURN_WORD()),
            "_FIRST_PERSON_WITHOUT_THE_PRONOUN" => Some(g__FIRST_PERSON_WITHOUT_THE_PRONOUN()),
            "_a_return_to_team_sport" => Some(py_func("_a_return_to_team_sport", |args: &[Py]| _a_return_to_team_sport(args))),
            _ => None,
        }),
        set: None,
    }))
}
