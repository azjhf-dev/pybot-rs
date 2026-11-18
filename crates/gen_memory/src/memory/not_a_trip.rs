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


thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BELONGS_TO_THE_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING_AFTER_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_CARING_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDINARY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_BELONGS_TO_THE_PLACE() -> Py { ensure_init(); G_BELONGS_TO_THE_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BELONGS_TO_THE_PLACE(v: Py) { ensure_init(); G_BELONGS_TO_THE_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINER() -> Py { ensure_init(); G_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINER(v: Py) { ensure_init(); G_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING() -> Py { ensure_init(); G_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); G_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING_AFTER_TO() -> Py { ensure_init(); G_DOING_AFTER_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING_AFTER_TO(v: Py) { ensure_init(); G_DOING_AFTER_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER_BEFORE() -> Py { ensure_init(); G_FILLER_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER_BEFORE(v: Py) { ensure_init(); G_FILLER_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_BEFORE() -> Py { ensure_init(); G_GOING_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_BEFORE(v: Py) { ensure_init(); G_GOING_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_CARING_PLACE() -> Py { ensure_init(); G_IN_A_CARING_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_CARING_PLACE(v: Py) { ensure_init(); G_IN_A_CARING_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDINARY() -> Py { ensure_init(); G_ORDINARY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDINARY(v: Py) { ensure_init(); G_ORDINARY.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTICLE() -> Py { ensure_init(); G_PARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTICLE(v: Py) { ensure_init(); G_PARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON() -> Py { ensure_init(); G_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON(v: Py) { ensure_init(); G_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_TO() -> Py { ensure_init(); G_VISIT_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_TO(v: Py) { ensure_init(); G_VISIT_TO.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Whether a \"going to ...\" line is going anywhere at all.\n\nEvery \"going to\" line was read as a trip: the words after \"to\" became the place, and the person\nwas asked about it as if they were packing.\n\n    im thinking of going back to uni next year  ->  \"Uni, lovely. When is it?\"\n    im going to start a small business          ->  \"Start, lovely. When is it?\"\n\nNobody is going anywhere: one is a decision to weigh, and the other is a doing. The place is the\ncode that happens to follow \"to\", and both of these are what a noun phrase cannot be:\n\n- a determiner after the first word says that word is a verb - \"start a small business\" cannot be\n  a place called Start, because a place does not put \"a\" after its own name (\"the Lake District\"\n  keeps its \"the\" in front, which is where a determiner belongs);\n- a particle after it says the same thing. \"think about it\" and \"save up first\" are a verb and its\n  particle, and the words after \"to\" are read as a place word for word: \"Think About It, lovely.\n  When is it?\". The particles here are the ones no place name uses - \"upon\", \"on\" and \"of\" are\n  absent because Stratford upon Avon, Henley on Thames and the isle of wight are journeys, and\n  \"in\" is absent for \"Barton in the Clay\";\n- a verb of the body or the feelings after \"to\" is the doing itself, and nobody asks when a\n  person is going to cry: \"im going to cry\", \"im going to throw up\", \"im going to die of\n  embarrassment\" were answered \"Cry, lovely. When is it?\" and \"Die of Embarrassment - nice.\n  When are you going?\". A place is never a bare verb, so one word is enough to know;\n- a place of ordinary life is not a trip. School, work, the gym and the shops are already known\n  here, and uni, college and the nursery are the same errand: going back to one of those is not a\n  holiday and nobody wants their month asked about.\n");
    s_DETERMINER(call_builtin("frozenset", &[mcall(&py_str("\na an the my your our their his her its this that these those some any another each every no\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BELONGS_TO_THE_PLACE(call_builtin("frozenset", &[mcall(&py_str("the a an"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTICLE(call_builtin("frozenset", &[mcall(&py_str("\nabout up down out back off over through around round along into away\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DOING_AFTER_TO(call_builtin("frozenset", &[mcall(&py_str("\nwait waits waited waiting hold holds held holding hope hopes hoped hoping plan plans planned\nplanning think thinks thought thinking save saves saved saving spend spends spent spending\nfinish finishes finished finishing decide decides decided deciding apply applies applied applying\narrange arranges arranged arranging aim aims aimed aiming avoid avoids avoided avoiding\nworry worries worried worrying sort sorted sorting\nbe been being get gets got getting go goes going do does did doing see sees saw seeing\ntry tries tried trying have has had having keep keeps kept keeping quit quits quitting\nleave leaves left leaving start starts started starting stop stops stopped stopping\nmanage manages managed managing afford affords afforded affording book books booked booking\nlose loses lost losing win wins won winning sell sells sold selling\nask asks asked asking tell tells told telling call calls called calling look looks looked looking\nfind finds found finding put puts putting buy buys bought buying eat eats ate eating\ndrink drinks drank drinking work works worked working sleep sleeps slept sleeping\npass passes passed passing faint faints fainted fainting collapse collapses collapsed collapsing\nthrow throws threw thrown throwing die dies died dying move moves moved moving\nwean weans weaned weaning\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDINARY(call_builtin("frozenset", &[mcall(&py_str("\nuni university unis college colleges campus nursery nurseries creche childcare playschool\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DOING(call_builtin("frozenset", &[mcall(&py_str("\nfeel feels felt feeling\ncry cries cried crying sob sobs sobbing weep weeps weeping wail wails wailing\nlaugh laughs laughed laughing giggle giggles giggling chuckle chuckles chuckling\nscream screams screamed screaming shout shouts shouted shouting yell yells yelled yelling\npanic panics panicked panicking faint faints fainted fainting\nthrow throws threw throwing vomit vomits vomited vomiting puke pukes puked puking\nsneeze sneezes sneezed sneezing cough coughs coughed coughing hiccup hiccups hiccuping\nburp burps burped burping gag gags gagged gagging choke chokes choked choking\ndie dies died dying explode explodes exploded exploding burst bursts bursting\nmoan moans moaned moaning groan groans groaned groaning snore snores snored snoring\nsweat sweats sweated sweating blush blushes blushed blushing\nshiver shivers shivered shivering tremble trembles trembled trembling\nwet wee poo splutter splutters spluttering\n# The same word twice, once as the doing and once as the idiom it makes: \"im going to lose it\"\n# and \"im going to crack up\" are the temper, \"im going to pass out\" is the body, and \"im going\n# to snap\" is both. A place is never one of these on its own, so the second word does not matter.\nlose loses losing lost crack cracks cracked cracking snap snaps snapped snapping\npop pops popped popping flip flips flipped flipping pass passes passed passing\nbreak breaks breaking broke broken\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON(py_str("(?:sister|brother|mother|father|son|daughter|mum|mom|dad)[\\s-]in[\\s-]laws?|sisters?|brothers?|mum|mom|mam|mother|father|dad|parents?|gran|granny|nan|nana|grandma|granddad|grandad|grandpa|grandparents?|aunts?|aunty|auntie|uncles?|cousins?|in[\\s-]?laws?|sons?|daughters?|kids|children|family|friends?|mates?|pals?"));
    s_VISIT_TO(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:(?:to|at)\\s+|(?:to\\s+)?(?:see|seeing|visit|visiting)\\s+)(?P<own>my|our|his|her|their)\\s+(?:the\\s+)?(?:own\\s+)?(?P<person>"), &g_A_PERSON()), &py_str(")(?:\\u2019s|'s|s')?s?(?:\\s+(?P<thing>house|place|flat|home))?\\s*(?=$|[,.!?;]|\\b(?:for|on|at|this|next|tonight|tomorrow|today|later|after)\\b)")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_PERSON(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?P<own>my|our|his|her|their)\\s+(?:own\\s+)?(?P<person>"), &g_A_PERSON()), &py_str(")(?:\\u2019s|'s|s')?s?(?:\\s+(?P<thing>house|place|flat|home))?\\s*$")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:going|goes|went|off|away|over|down|up|round|around|out|back|heading|driving|flying|travelling|traveling|staying|stay)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLER_BEFORE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:so|and|but|oh|also|anyway|then|now|well|ok|okay|right|im|i'm|i am|we're|we are|were|i|we)\\s+){0,3}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_A_CARING_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:at|in|to)\\s+(?:the\\s+|a\\s+)?(?:hospital|hospice|care home|nursing home|residential home|clinic|infirmary|surgery|ward|a&e|ae|icu|intensive care|high dependency|coronary care|children'?s ward)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn a_doing_after_to(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut going_words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "going_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: going_words"))) };
    let mut first: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut second: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words after \"to\" are a verb and its object (\"start a small business\").");
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&going_words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("")]), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,.;:!?'’")])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BELONGS_TO_THE_PLACE(), &mcall(&mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(" ,.;:!?")]))) } }) {
    kept = slice_or_raise(&kept.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return Py::Bool(false);
    }
    first = mcall(&mcall(&geti(&kept.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(" ,.;:!?'’")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DOING_AFTER_TO(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DOING(), &first.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    second = mcall(&mcall(&geti(&kept.clone(), &Py::Int(1)), "lower", &[]), "strip", &[py_str(" ,.;:!?'’")]);
    return { let __b = Py::Bool(py_contains(&g_DETERMINER(), &second.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PARTICLE(), &second.clone())) } };
    Py::None
}
pub fn a_place_of_ordinary_life(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the place is where the person already goes, not somewhere they are going.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str(" ")])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(" ,.;:!?'’")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BELONGS_TO_THE_PLACE(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ORDINARY(), &geti(&words.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn not_a_destination(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut going_words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "going_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: going_words"))) };
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let _ = py_str("True when a \"going to ...\" line names a doing or an everyday place, not a trip.");
    return { let __b = a_doing_after_to(&[going_words.clone()]); if py_truthy(&__b) { __b } else { a_place_of_ordinary_life(&[place.clone()]) } };
    Py::None
}
pub fn _the_person_in(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Who a match names, said the way the trip reader stores a place: \"my brothers\", \"my in laws\",\n    \"my parents house\". The possessive in front is what makes it a person (\"my kids school\"), and\n    the answer says it back as theirs.");
    who = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("person")]), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&who.clone(), "startswith", &[py_str("in")])) {
    who = py_str("in-laws");
    }
    said = op_add(&op_add(&mcall(&mcall(&found.clone(), "group", &[py_str("own")]), "lower", &[]), &py_str(" ")), &who.clone());
    thing = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    return if py_truthy(&thing.clone()) { op_add(&op_add(&said.clone(), &py_str(" ")), &thing.clone()) } else { said.clone() };
    Py::None
}
pub fn a_person(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The person when a destination is only a person (\"my aunts\"), or \"\" when it is a place.");
    found = mcall(&g_THEIR_PERSON(), "match", &[call_builtin("str", &[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    return if py_truthy(&found.clone()) { _the_person_in(&[found.clone()]) } else { py_str("") };
    Py::None
}
pub fn the_visit(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The person a line is going to see, or \"\" when the words after \"to\" name a place.\n\n    \"im going to my sisters on saturday\" goes nowhere called My Sisters, and \"we are going to my in\n    laws\" is not a trip to Laws. Only a destination that ends at the person counts: \"im going to my\n    sisters in spain\" is a journey to Spain, and the trip reader keeps it. The line has to be one\n    about going, too, or the same words in another sentence are a different thing.\n    ");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_IN_A_CARING_PLACE(), "search", &[said.clone()])) {
    return py_str("");
    }
    found = mcall(&g_VISIT_TO(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    before = slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_GOING_BEFORE(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FILLER_BEFORE(), "match", &[before.clone()]) } }))) {
    return py_str("");
    }
    return _the_person_in(&[found.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.not_a_trip".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_PERSON" => Some(g_A_PERSON()),
            "BELONGS_TO_THE_PLACE" => Some(g_BELONGS_TO_THE_PLACE()),
            "DETERMINER" => Some(g_DETERMINER()),
            "DOING" => Some(g_DOING()),
            "DOING_AFTER_TO" => Some(g_DOING_AFTER_TO()),
            "FILLER_BEFORE" => Some(g_FILLER_BEFORE()),
            "GOING_BEFORE" => Some(g_GOING_BEFORE()),
            "IN_A_CARING_PLACE" => Some(g_IN_A_CARING_PLACE()),
            "ORDINARY" => Some(g_ORDINARY()),
            "PARTICLE" => Some(g_PARTICLE()),
            "THEIR_PERSON" => Some(g_THEIR_PERSON()),
            "VISIT_TO" => Some(g_VISIT_TO()),
            "_the_person_in" => Some(py_func("_the_person_in", |args: &[Py]| _the_person_in(args))),
            "a_doing_after_to" => Some(py_func("a_doing_after_to", |args: &[Py]| a_doing_after_to(args))),
            "a_person" => Some(py_func("a_person", |args: &[Py]| a_person(args))),
            "a_place_of_ordinary_life" => Some(py_func("a_place_of_ordinary_life", |args: &[Py]| a_place_of_ordinary_life(args))),
            "not_a_destination" => Some(py_func("not_a_destination", |args: &[Py]| not_a_destination(args))),
            "the_visit" => Some(py_func("the_visit", |args: &[Py]| the_visit(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_PERSON" => s_A_PERSON(v),
            "BELONGS_TO_THE_PLACE" => s_BELONGS_TO_THE_PLACE(v),
            "DETERMINER" => s_DETERMINER(v),
            "DOING" => s_DOING(v),
            "DOING_AFTER_TO" => s_DOING_AFTER_TO(v),
            "FILLER_BEFORE" => s_FILLER_BEFORE(v),
            "GOING_BEFORE" => s_GOING_BEFORE(v),
            "IN_A_CARING_PLACE" => s_IN_A_CARING_PLACE(v),
            "ORDINARY" => s_ORDINARY(v),
            "PARTICLE" => s_PARTICLE(v),
            "THEIR_PERSON" => s_THEIR_PERSON(v),
            "VISIT_TO" => s_VISIT_TO(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.not_a_trip", module_obj);
    pyrt::xrefs::reg_fn("memory.not_a_trip", "a_doing_after_to", py_func("a_doing_after_to", |args: &[Py]| a_doing_after_to(args)));
    pyrt::xrefs::reg_fn("memory.not_a_trip", "a_place_of_ordinary_life", py_func("a_place_of_ordinary_life", |args: &[Py]| a_place_of_ordinary_life(args)));
    pyrt::xrefs::reg_fn("memory.not_a_trip", "not_a_destination", py_func("not_a_destination", |args: &[Py]| not_a_destination(args)));
    pyrt::xrefs::reg_fn("memory.not_a_trip", "_the_person_in", py_func("_the_person_in", |args: &[Py]| _the_person_in(args)));
    pyrt::xrefs::reg_fn("memory.not_a_trip", "a_person", py_func("a_person", |args: &[Py]| a_person(args)));
    pyrt::xrefs::reg_fn("memory.not_a_trip", "the_visit", py_func("the_visit", |args: &[Py]| the_visit(args)));
    pyrt::xrefs::reg_access("memory.not_a_trip", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("memory.not_a_trip", "BELONGS_TO_THE_PLACE", g_BELONGS_TO_THE_PLACE, s_BELONGS_TO_THE_PLACE);
    pyrt::xrefs::reg_access("memory.not_a_trip", "DETERMINER", g_DETERMINER, s_DETERMINER);
    pyrt::xrefs::reg_access("memory.not_a_trip", "DOING", g_DOING, s_DOING);
    pyrt::xrefs::reg_access("memory.not_a_trip", "DOING_AFTER_TO", g_DOING_AFTER_TO, s_DOING_AFTER_TO);
    pyrt::xrefs::reg_access("memory.not_a_trip", "FILLER_BEFORE", g_FILLER_BEFORE, s_FILLER_BEFORE);
    pyrt::xrefs::reg_access("memory.not_a_trip", "GOING_BEFORE", g_GOING_BEFORE, s_GOING_BEFORE);
    pyrt::xrefs::reg_access("memory.not_a_trip", "IN_A_CARING_PLACE", g_IN_A_CARING_PLACE, s_IN_A_CARING_PLACE);
    pyrt::xrefs::reg_access("memory.not_a_trip", "ORDINARY", g_ORDINARY, s_ORDINARY);
    pyrt::xrefs::reg_access("memory.not_a_trip", "PARTICLE", g_PARTICLE, s_PARTICLE);
    pyrt::xrefs::reg_access("memory.not_a_trip", "THEIR_PERSON", g_THEIR_PERSON, s_THEIR_PERSON);
    pyrt::xrefs::reg_access("memory.not_a_trip", "VISIT_TO", g_VISIT_TO, s_VISIT_TO);
}
