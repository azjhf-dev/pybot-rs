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


thread_local! { static G_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUES_AFTER_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT understanding.senses: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_CUES() -> Py { ensure_init(); G_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUES(v: Py) { ensure_init(); G_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_CUES_AFTER_A_FIGURE() -> Py { ensure_init(); G_CUES_AFTER_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUES_AFTER_A_FIGURE(v: Py) { ensure_init(); G_CUES_AFTER_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A word that names two everyday things, read from the whole line rather than the word.\n\nSome words are the name of one thing in a line and another thing in the next. \"room\" is the space\nin a kitchen and a bed for the night; \"bat\" is a wooden thing you hit a ball with even though the\nline says nothing about a game. The knowledge base matches on words, so a row that owns one sense\nanswers the other sense's line: \"i have no room in my kitchen\" was sent the hotel-overbooking card,\nbecause \"room\" and \"no room\" are that row's key, and the same line typed \"no room in my kitchen\"\nreached the storage answer. The rows had fenced each other with phrases (\"have no room\" was on the\nstorage row's `avoid` list to stop a hotel line being told to put a shelf up), so the same words\nmeant two things and only the phrase said which.\n\nWhich sense a line means is not in the word: it is in the rest of the sentence. \"in my kitchen\",\n\"cupboard\", \"put away\", \"too much stuff\" say the space; \"hotel\", \"booking\", \"two nights\",\n\"reception\" say the bed. So this module reads the whole line - every word of it, wherever the word\nstands - and reports the sense the line supports.\n\nThe trick is the word nobody says: a line that says nothing either way gets **no reading**, and the\nrows that own a sense stand aside instead of guessing. \"my bat is broken\" names a bat and a break,\nand nothing in it says the animal or the game, so `senses()` says nothing about it and a row keyed\non the game does not answer it. Guessing is what put the cricket coaching on a broken bat.\n\n`understand()` reports the lot under one key, `sense`: a dict of word to the sense the line\nsupports, holding only the words the line actually settles. A row guards on it with a plain test,\nso a word added here is a row's guard away from fixing every line about it.\n");
    s_CUES(py_dict(vec![(py_str("room"), py_dict(vec![(py_str("space"), call_builtin("frozenset", &[mcall(&py_str("\n            kitchen kitchens bedroom bedrooms bathroom bathrooms cupboard cupboards wardrobe\n            wardrobes freezer fridge car boot shed garage loft attic cellar hall hallway landing\n            landing flat house home drawer drawers shelf shelves cupboard worktop bench desk\n            storage store storing put fit fits fitting tidy tidier tidy-up clutter stuff boxes\n            box cupboard space cramped clutter pots pans utensils microwave toaster kettle\n            small tiny little cramped narrow cramped squeeze squashed poky pokey tight\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("booking"), call_builtin("frozenset", &[mcall(&py_str("\n            hotel hotels motel hostel guest house booking bookings booked book reservation\n            reservations reception receptionist desk check checkin check-in concierge night\n            nights twin double suite ensuite en-suite overbooked upgrade holiday stay staying\n            need needs needed want wants wanted prefer rather looking searching\n            traveller traveler luggage suitcase suitcases case cases bags abroad airport\n            flight trip refunded\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))])), (py_str("bat"), py_dict(vec![(py_str("sport"), call_builtin("frozenset", &[mcall(&py_str("\n            cricket bowling bowler batsman batsmen innings wicket wickets pitch stumps willow\n            grip oiling oil knocked-in handle tennis ping pong table shot cover drive yorker\n            spinner seamer nets net match club league batting batsman's bails crease\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("animal"), call_builtin("frozenset", &[mcall(&py_str("\n            flew fly flying loft attic roof rafters eaves belfry wings wing bitten rabies\n            droppings squeaking squeak colony pup hibernating hibernate dusk rescued\n            hedgehog tree trees garden shed hole\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))])), (py_str("coach"), py_dict(vec![(py_str("vehicle"), call_builtin("frozenset", &[mcall(&py_str("\n            train trains rail railway station stations platform platform ticket tickets journey\n            journeys seat seats carriage carriages bus buses megabus national-express travel\n            travelling traveling trip legroom route timetable delay delays luggage left-luggage\n            book booked booking reservations departures\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("teacher"), call_builtin("frozenset", &[mcall(&py_str("\n            football rugby netball cricket hockey tennis kids child children junior juniors\n            under u7 u8 u9 u10 u11 u12 u13 u14 u15 u16 session sessions drill drills players\n            squad tactics coaching goalkeeper striker defenders midfield training skills\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("furniture"), call_builtin("frozenset", &[mcall(&py_str("\n            sofa settee couch lounge upholstery reupholster reupholstered fabric cushions\n            cushion seater armchair furniture corner three-piece living rooms room front-room\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))])), (py_str("degrees"), py_dict(vec![(py_str("temperature"), call_builtin("frozenset", &[mcall(&py_str("\n            hot cold warm warmer warmest freezing boiling heat humid humidity celsius\n            fahrenheit weather forecast summer winter autumn spring sunny sun shade outside\n            today tonight tomorrow thermometers thermometer fever thermostat oven roasting\n            heatwave heatwaves sweaty sunscreen brollies\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("university"), call_builtin("frozenset", &[mcall(&py_str("\n            uni university universities college ucas graduate graduates graduated graduation\n            student students masters bachelors bachelor bsc ba ma phd honours honours-degree\n            dissertation campus term semester freshers results classified first-class 2:1\n            studying studied study course courses academic tutor lecturer professor\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("angle"), call_builtin("frozenset", &[mcall(&py_str("\n            angle angles geometry triangle protractor rotate rotation rotated rotation\n            turnaround corner corners bend bends slope sloped incline gradient\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))])), (py_str("late"), py_dict(vec![(py_str("time"), call_builtin("frozenset", &[mcall(&py_str("\n            work job boss manager office meeting interview deadline shift roster rota overtime\n            colleague bus buses train tram tube flight plane ferry fired sacked redundant\n            school class lesson nursery childcare pickup collection homework exam detention\n            court dentist hairdresser barber cinema theatre match kickoff traffic jam queue\n            road motorway accident closed cancelled overslept alarm form\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("cycle"), call_builtin("frozenset", &[mcall(&py_str("\n            period missed pregnant pregnancy test bleed bleeding spotting cramp pill\n            contraceptive ovulate ovulating ovulation conceive conceiving conception cycle\n            womb trimester midwife\n            "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))]))]));
    s_CUES_AFTER_A_FIGURE(py_dict(vec![(py_str("degrees"), py_str("temperature"))]));
    s__A_FIGURE(call_builtin("frozenset", &[mcall(&py_str("\n    one two three four five six seven eight nine ten eleven twelve twenty thirty forty fifty\n    sixty seventy eighty ninety hundred thousand zero\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_figure_before(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut before: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut i: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("Whether a figure stands right in front of `word` in the line as typed.");
    forms = _forms(&[word.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[{ let __b = mcall(&line.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    token = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&forms.clone(), &mcall(&call_builtin("str", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&i.clone(), &Py::Int(0))) } }) {
    continue;
    }
    before = mcall(&mcall(&call_builtin("str", &[geti(&geti(&line.clone(), &py_str("norm")), &op_sub(&i.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str(".,!?;:")]);
    if py_truthy(&{ let __b = mcall(&before.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_FIGURE(), &before.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _forms(args: &[Py]) -> Py {
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
    let mut forms: Py = Py::None;
    let _ = py_str("The word and the plain form the readers compare it by (\"nights\" is \"night\").");
    forms = py_set(vec![word.clone()]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))) } } } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("es")]) } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)]);
    }
    return forms.clone();
    Py::None
}
pub fn _the_words(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut form: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every word of the line, from the reading, folded to the forms a cue is written in.");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&line.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    let _ = mcall(&words.clone(), "append", &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    }
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&line.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&clean.clone()) {
    let _ = mcall(&words.clone(), "extend", &[mcall(&clean.clone(), "split", &[])]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&_forms(&[word.clone()])) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _how_many_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cues: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cues") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cues"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut cue: Py = Py::None;
    let mut form: Py = Py::None;
    let _ = py_str("How many of the line's words one sense accounts for, each one counted once.\n\n    A cue list is written in the forms a person types, so it often holds a word and its plural\n    beside each other (\"hotel hotels\", \"kitchen kitchens\"). Counting the list rather than the\n    words then lets one word said once outvote a line that says two other things: \"my hotel room\n    is too small\" scored booking 2 and space 1 on the word `hotel` alone.\n    ");
    return call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&cues.clone()) { let mut cue: Py = __item.clone(); for __item in py_iter_or_raise(&op_bitand(&_forms(&[cue.clone()]), &words.clone())) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn senses(args: &[Py]) -> Py {
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
    let mut by_sense: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut found: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut sense: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`{word: sense}` for each two-sense word whose sense the whole line supports.\n\n    The line is the `normalise()` reading, so `norm` and `clean` are both read. A sense needs one\n    more cue than the other sense of the same word: a tie is a line that says both (or neither),\n    and there the honest answer is no reading at all.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_dict(vec![]);
    }
    words = _the_words(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_dict(vec![]);
    }
    found = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_CUES(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    by_sense = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&_forms(&[word.clone()]), &words.clone())))) {
    continue;
    }
    scored = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&by_sense.clone(), "items", &[])) { let __unpk2484 = py_unpack(&__item.clone(), 2, None); let mut sense: Py = __unpk2484[0].clone(); let mut cues: Py = __unpk2484[1].clone(); __out.push(py_tuple(vec![_how_many_words(&[cues.clone(), words.clone()]), sense.clone()])); } py_list(__out) })() }); __kws.extend(vec![("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&call_builtin("len", &[scored.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&geti(&scored.clone(), &Py::Int(0)), &Py::Int(0)), &Py::Int(0)))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CUES_AFTER_A_FIGURE(), &word.clone())); if !py_truthy(&__b) { __b } else { _a_figure_before(&[line.clone(), word.clone()]) } }) {
    let _ = py_setitem(&found.clone(), &word.clone(), geti(&g_CUES_AFTER_A_FIGURE(), &word.clone()));
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&geti(&scored.clone(), &Py::Int(0)), &Py::Int(0)), &geti(&geti(&scored.clone(), &Py::Int(1)), &Py::Int(0))))) {
    continue;
    }
    let _ = py_setitem(&found.clone(), &word.clone(), geti(&geti(&scored.clone(), &Py::Int(0)), &Py::Int(1)));
    }
    return found.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.senses".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CUES" => Some(g_CUES()),
            "CUES_AFTER_A_FIGURE" => Some(g_CUES_AFTER_A_FIGURE()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_a_figure_before" => Some(py_func("_a_figure_before", |args: &[Py]| _a_figure_before(args))),
            "_forms" => Some(py_func("_forms", |args: &[Py]| _forms(args))),
            "_how_many_words" => Some(py_func("_how_many_words", |args: &[Py]| _how_many_words(args))),
            "_the_words" => Some(py_func("_the_words", |args: &[Py]| _the_words(args))),
            "senses" => Some(py_func("senses", |args: &[Py]| senses(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CUES" => s_CUES(v),
            "CUES_AFTER_A_FIGURE" => s_CUES_AFTER_A_FIGURE(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.senses", module_obj);
    pyrt::xrefs::reg_fn("understanding.senses", "_a_figure_before", py_func("_a_figure_before", |args: &[Py]| _a_figure_before(args)));
    pyrt::xrefs::reg_fn("understanding.senses", "_forms", py_func("_forms", |args: &[Py]| _forms(args)));
    pyrt::xrefs::reg_fn("understanding.senses", "_the_words", py_func("_the_words", |args: &[Py]| _the_words(args)));
    pyrt::xrefs::reg_fn("understanding.senses", "_how_many_words", py_func("_how_many_words", |args: &[Py]| _how_many_words(args)));
    pyrt::xrefs::reg_fn("understanding.senses", "senses", py_func("senses", |args: &[Py]| senses(args)));
    pyrt::xrefs::reg_access("understanding.senses", "CUES", g_CUES, s_CUES);
    pyrt::xrefs::reg_access("understanding.senses", "CUES_AFTER_A_FIGURE", g_CUES_AFTER_A_FIGURE, s_CUES_AFTER_A_FIGURE);
    pyrt::xrefs::reg_access("understanding.senses", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
}
