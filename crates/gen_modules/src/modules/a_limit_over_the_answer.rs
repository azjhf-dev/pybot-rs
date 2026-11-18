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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DANGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INSTRUCTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD_INS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVE_CUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS() -> Py { ensure_init(); G__ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); G__ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__DANGLING() -> Py { ensure_init(); G__DANGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DANGLING(v: Py) { ensure_init(); G__DANGLING.with(|c| *c.borrow_mut() = v); }
pub fn g__INSTRUCTION() -> Py { ensure_init(); G__INSTRUCTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INSTRUCTION(v: Py) { ensure_init(); G__INSTRUCTION.with(|c| *c.borrow_mut() = v); }
pub fn g__JOINER() -> Py { ensure_init(); G__JOINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOINER(v: Py) { ensure_init(); G__JOINER.with(|c| *c.borrow_mut() = v); }
pub fn g__KINDS() -> Py { ensure_init(); G__KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KINDS(v: Py) { ensure_init(); G__KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD_INS() -> Py { ensure_init(); G__LEAD_INS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD_INS(v: Py) { ensure_init(); G__LEAD_INS.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVE_CUT() -> Py { ensure_init(); G__MOVE_CUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVE_CUT(v: Py) { ensure_init(); G__MOVE_CUT.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVE_WORDS() -> Py { ensure_init(); G__MOVE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVE_WORDS(v: Py) { ensure_init(); G__MOVE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TAIL() -> Py { ensure_init(); G__TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAIL(v: Py) { ensure_init(); G__TAIL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A limit of theirs, said over an answer already given, changes that answer - it is not a receipt.\n\nPeople give the answer a constraint once they have heard it, in the shortest way they can:\n\n    the kitchen tap is dripping / we rent the flat     -> \"Right, so you rent. What are you\n                                                          trying to sort out?\"\n    i want to visit my sister / we have no car          -> \"You have no car - noted.\"\n    the wedding is in italy / its in a field            -> \"I'm listening - say more if it helps.\"\n\nEvery one of those is the bot taking the fact and handing the conversation back. Nobody is trying\nto sort out what they just asked about: they are telling the bot what the answer has to work\naround. A friend does the one thing the limit changes and gets on with the plan.\n\nSo this module answers **only** the turn that states the limit, and only when there is an answer\nabove it to change: the answer's own first move is repeated with the limit already in it.\n\nThe answer in hand is `memory[\"last_answer\"]`, which only `bot/modules/answers.py` writes, so this\nmodule can only ever fire after a card from that module: a limit said after a reply from anywhere\nelse (\"whats the weather like tomorrow\" / \"we have no car\") is left to the greeting it gets today.\nThat is a real narrowing and it is the honest one - the limit needs an answer to change.\n\nIt also has to be about the same kind of thing as the limit. That is read from the answer's own\n`keys`, not from its prose: \"water\" is in a balcony's word list and in \"350ml warm water\" of every\nbread recipe, so matching the prose told a bread answer a balcony was enough for it. One of the\nkind's words among the answer's keys is enough; a word in the prose needs a second one beside it,\nbecause one shared word in a sentence is a coincidence.\n\nEverything else is left alone: a limit opening the conversation belongs to\n`bot/compose/circumstance.py`, a limit that is a question (\"can we do it by train\") is the answers\nlane's, and a line with anything else in it keeps its own reading.\n\nTwo limits are deliberately not here. \"money is tight\" and \"how long have you got\" are answered by\ncards of their own, and repeating the answer's first move in front of them is the answer given\nagain with a lead-in - the arbiter is right to give the turn to the card. The five kinds below are\nthe ones where the limit changes *whose job it is* or *where the thing goes*, which the answer in\nhand cannot say for itself.\n");
    s_PRIORITY(Py::Int(500));
    s__LEAD(stdlib_call("re", "compile", &[py_str("^(?:well|ok|okay|so|oh|ah|and|but|honestly|actually|yeah|sorry|anyway|course|of\\s+course)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TAIL(stdlib_call("re", "compile", &[py_str("(?:\\s+(?:tbh|tho|though|anyway|actually|really|literally|honestly|i\\s+suppose|i\\s+guess|still|also|too|by\\s+the\\s+way|btw|atm|at\\s+the\\s+moment|lol|ha|haha|so\\s+yeah|though|mate|dude|sadly|unfortunately))+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS(stdlib_call("re", "compile", &[py_str("\\?|^\\s*(?:what|how|when|where|why|who|can|could|should|would|do|does|did|is|are|was|were|will|shall|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__KINDS(py_tuple(vec![py_tuple(vec![py_str("rent"), py_tuple(vec![stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:just\\s+)?rent(?:ing)?(?:\\s+(?:a|an|the|our|this)\\s+(?:flat|house|place|home|bungalow|room|council\\s+flat))?$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:are|re)\\s+(?:renting|in\\s+a\\s+rented|renters?)$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:its|it\\s+is)\\s+(?:a\\s+)?(?:rented|rental|council|housing\\s+association)(?:\\s+(?:flat|house|place|home))?$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s+(?:are|re)?\\s*(?:just\\s+)?tenants?$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s+(?:dont|do\\s+not)\\s+own\\s+(?:it|the\\s+flat|the\\s+house|our\\s+home)$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("repair"), py_str("fix"), py_str("damp"), py_str("boiler"), py_str("radiator"), py_str("tap"), py_str("leak"), py_str("leaking"), py_str("dripping"), py_str("drip"), py_str("wall"), py_str("ceiling"), py_str("paint"), py_str("heating"), py_str("plumb"), py_str("roof"), py_str("floor"), py_str("fuse"), py_str("washer"), py_str("mould"), py_str("mold"), py_str("window"), py_str("door"), py_str("drains"), py_str("toilet"), py_str("shower"), py_str("bath"), py_str("landlord"), py_str("carpet"), py_str("electric"), py_str("socket"), py_str("gutter"), py_str("chimney")]), py_tuple(vec![py_str("Then it's your landlord's to fix, not yours - but do the free bit first: {move}"), py_str("Renting puts that on them: report it in writing, and meanwhile {move_lower}"), py_str("Right, so it's their job. What you can do now: {move_lower}")])]), py_tuple(vec![py_str("no_car"), py_tuple(vec![stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:have|ve)\\s+(?:got\\s+)?no\\s+car$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:there\\s+is\\s+)?no\\s+car(?:\\s+here)?$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:dont|do\\s+not|can\\s*not|can't|cant)\\s*drive$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:dont|do\\s+not)\\s+have\\s+(?:a\\s+)?car$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:its|it\\s+is)\\s+(?:a\\s+)?no\\s+car\\s+(?:household|family)$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("drive"), py_str("driving"), py_str("car"), py_str("travel"), py_str("journey"), py_str("get there"), py_str("getting there"), py_str("route"), py_str("station"), py_str("bus"), py_str("train"), py_str("coach"), py_str("parking"), py_str("park"), py_str("trip"), py_str("visit"), py_str("ferry"), py_str("flight"), py_str("fly"), py_str("airport"), py_str("motorway")]), py_tuple(vec![py_str("Then go by train or bus instead of driving - and the rest of it still holds: {move_lower}"), py_str("No car changes the how, not the plan: {move_lower}"), py_str("Then book the train rather than drive. Everything else stands: {move_lower}")])]), py_tuple(vec![py_str("only_a_balcony"), py_tuple(vec![stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:only|just)\\s+(?:have|ve\\s+got|got)\\s+(?:a\\s+|one\\s+)?(?:small\\s+)?(?:balcony|windowsill|window\\s+box|yard|patio|terrace|yard)$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:its|it\\s+is)\\s+(?:just\\s+)?(?:a\\s+)?(?:balcony|windowsill|window\\s+box)$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("plant"), py_str("sow"), py_str("sowing"), py_str("grow"), py_str("growing"), py_str("pot"), py_str("pots"), py_str("garden"), py_str("soil"), py_str("seed"), py_str("seeds"), py_str("compost"), py_str("bed"), py_str("patch"), py_str("allotment"), py_str("water"), py_str("sun"), py_str("shade"), py_str("herbs"), py_str("tomato"), py_str("tomatoes"), py_str("flower"), py_str("flowers"), py_str("bulb")]), py_tuple(vec![py_str("Then keep it all in pots - that's the part that matters: {move_lower}"), py_str("A balcony is enough for that: {move_lower}"), py_str("Then it's pots on the balcony rather than a bed: {move_lower}")])]), py_tuple(vec![py_str("no_garden"), py_tuple(vec![stdlib_call("re", "compile", &[py_str("^(?:we|i)\\s*(?:dont|do\\s+not)\\s+have\\s+(?:a\\s+|any\\s+)?(?:garden|outside\\s+space|yard|plot|allotment)$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:no|without\\s+a)\\s+(?:garden|outside\\s+space|allotment)$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("plant"), py_str("sow"), py_str("sowing"), py_str("grow"), py_str("growing"), py_str("pot"), py_str("pots"), py_str("garden"), py_str("soil"), py_str("seed"), py_str("seeds"), py_str("compost"), py_str("bed"), py_str("patch"), py_str("allotment"), py_str("water"), py_str("sun"), py_str("shade"), py_str("herbs"), py_str("tomato"), py_str("tomatoes"), py_str("flower"), py_str("flowers"), py_str("bulb")]), py_tuple(vec![py_str("Then keep it all in pots or a window box - that's the part that matters: {move_lower}"), py_str("No garden is fine for that: {move_lower}"), py_str("Then it's containers rather than a bed: {move_lower}")])]), py_tuple(vec![py_str("a_nervous_animal"), py_tuple(vec![stdlib_call("re", "compile", &[py_str("^(?:hes|he\\s+is|shes|she\\s+is|its|it\\s+is)\\s+(?:a\\s+)?(?:rescue|rescued)(?:\\s+(?:and\\s+)?(?:quite\\s+|very\\s+|really\\s+)?(?:nervous|anxious|shy|scared|terrified|timid))?$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:hes|he\\s+is|shes|she\\s+is|its|it\\s+is)\\s+(?:quite\\s+|very\\s+|really\\s+)?(?:nervous|anxious|scared|terrified|timid|shy)$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:hes|shes|its)\\s+(?:a\\s+)?rescue$")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("lead"), py_str("leash"), py_str("walk"), py_str("walks"), py_str("train"), py_str("training"), py_str("scratch"), py_str("scratching"), py_str("bark"), py_str("barking"), py_str("sofa"), py_str("litter"), py_str("collar"), py_str("harness"), py_str("garden"), py_str("vet"), py_str("animal"), py_str("rescue"), py_str("puppy")]), py_tuple(vec![py_str("Then go slower than that and keep every go short: {move_lower}"), py_str("A nervous rescue needs that done in small doses: {move_lower}"), py_str("Then take it at their pace - the first step is the same: {move_lower}")])])]));
    s__INSTRUCTION(stdlib_call("re", "compile", &[py_str("^(?:sow|plant|prune|feed|mulch|pot|repot|thin|stake|tie|harvest|practise|practice|learn|watch|read|listen|repeat|spend|save|budget|compare|download|install|print|wear|cut|freeze|store|label|turn|put|take|use|keep|start|add|set|get|open|close|ask|book|check|try|make|give|water|clean|wash|report|write|stand|hold|cut|mix|place|leave|move|cover|wrap|soak|strip|sand|fill|fit|call|text|email|ring|go|walk|run|sit|lie|rest|eat|drink|buy|pick|choose|measure|weigh|boil|fry|bake|simmer|stir|season|remove|replace|tighten|loosen|unplug|switch|plug|drain|bleed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DANGLING(call_builtin("frozenset", &[py_tuple(vec![py_str("and"), py_str("or"), py_str("with"), py_str("a"), py_str("an"), py_str("the"), py_str("of"), py_str("to"), py_str("for"), py_str("in"), py_str("on"), py_str("at"), py_str("plus"), py_str("then"), py_str("but"), py_str("so"), py_str("your"), py_str("my"), py_str("its")])]).unwrap_or_else(|e| exc::raise(e)));
    s__JOINER(stdlib_call("re", "compile", &[py_str("^(?:so|and|then|but|just|really|also|first|afterwards?)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAD_INS(stdlib_call("re", "compile", &[py_str("^(?:same\\s+as\\s+before|the\\s+first\\s+thing\\s+to\\s+do|first\\s+thing|so|right|okay|ok|then)\\s*:\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MOVE_WORDS(Py::Int(45));
    s__MOVE_CUT(Py::Int(34));
}
pub fn _clean(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut before: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("The line with the tone and the lead-in taken off, lower case, no end punctuation.");
    low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" .!,")]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    before = low.clone();
    low = mcall(&g__LEAD(), "sub", &[py_str(""), low.clone()]);
    low = mcall(&g__TAIL(), "sub", &[py_str(""), low.clone()]);
    if py_truthy(&Py::Bool(py_eq(&low.clone(), &before.clone()))) {
    break;
    }
    }
    return mcall(&mcall(&low.clone(), "lower", &[]), "strip", &[py_str(" .,!")]);
    Py::None
}
pub fn _a_limit(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut low: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut shapes: Py = Py::None;
    let _ = py_str("`(kind, words, leads)` for a line that is only a limit, or None.");
    low = _clean(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { mcall(&g__ASKS(), "search", &[low.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    kind = __unpk2[0].clone();
    shapes = __unpk2[1].clone();
    about = __unpk2[2].clone();
    leads = __unpk2[3].clone();
    let __iter3 = py_iter_or_raise(&shapes.clone());
    for __item in __iter3 {
    shape = __item.clone();
    if py_truthy(&mcall(&shape.clone(), "match", &[low.clone()])) {
    return py_tuple(vec![kind.clone(), about.clone(), leads.clone()]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_answer(args: &[Py]) -> Py {
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
    let mut record: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer in hand, while the chat is still about it.\n\n    It is `memory[\"last_answer\"]`, which only `bot/modules/answers.py` writes, so this module can\n    only ever fire after a card from that module: a limit said after a reply from anywhere else\n    (\"whats the weather like tomorrow\" / \"we have no car\") is left to the greeting it gets today.\n    That is a real narrowing, and the record's own `keys` are what say whether the answer is about\n    the same kind of thing as the limit.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    record = mcall(&memory.clone(), "get", &[py_str("last_answer")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[mcall(&record.clone(), "get", &[py_str("reply")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    turn = mcall(&record.clone(), "get", &[py_str("turn")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = op_lt(&turn.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_gt(&op_sub(&op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &turn.clone()), &Py::Int(2)) } } } }) {
    return Py::None;
    }
    return record.clone();
    Py::None
}
pub fn _what_the_answer_is_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let _ = py_str("The words the answer is about, from the record's own keys.\n\n    Not the prose: \"water\" is in a balcony's word list and in \"350ml warm water\" of every bread\n    recipe, so a bread answer was being told a balcony is enough for it. The keys are the subject\n    the card answered.\n    ");
    keys = if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&record.clone(), "get", &[py_str("keys")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[keys.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    keys = mcall(&keys.clone(), "split", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[keys.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    Py::None
}
pub fn _the_answer_is_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut about: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: about"))) };
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the answer is about the same kind of thing as the limit.\n\n    One of the kind's words in the answer's own keys is enough, because the keys are the subject\n    the card answered. The prose needs two: \"water\" is in every bread recipe and in a balcony's\n    word list, so one shared word in a sentence is a coincidence, while two of them (\"sow\",\n    \"pot\", \"sun\") make the answer the one the limit is about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&about.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&about.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_what_the_answer_is_about(&[record.clone()]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    low = mcall(&geti(&record.clone(), &py_str("reply")), "lower", &[]);
    return op_ge(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&about.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&low.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _the_move(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The answer's first instruction, said back as a whole sentence, or \"\".\n\n    A clause is not a sentence: \"turn the water off at the isolating valve under the sink, take the\n    handle off\" stops before the part that finishes the job (\"... and take it to a plumbing\n    merchant to match it\"), and a person reads an instruction that breaks off. So the move runs on\n    to the end of the sentence it starts in. Only a sentence too long to say back is cut, and then\n    at a whole clause with the full stop put back on.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&answer.clone(), "replace", &[py_str("—"), py_str("- ")]), "split", &[])]);
    text = mcall(&g__LEAD_INS(), "sub", &[py_str(""), text.clone()]);
    first = mcall(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s"), text.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" .;:,-")]);
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&first.clone(), &py_str(":")))) {
    let __unpk1 = py_unpack(&mcall(&first.clone(), "split", &[py_str(":"), Py::Int(1)]), 2, None);
    head = __unpk1[0].clone();
    tail = __unpk1[1].clone();
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&tail.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    first = mcall(&tail.clone(), "strip", &[]);
    }
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[,;]\\s"), first.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&{ let __b = parts.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__INSTRUCTION(), "match", &[mcall(&g__JOINER(), "sub", &[py_str(""), geti(&parts.clone(), &Py::Int(0))])]))) } }) {
    found = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    part = __unpk3[1].clone();
    if py_truthy(&mcall(&g__INSTRUCTION(), "match", &[mcall(&g__JOINER(), "sub", &[py_str(""), part.clone()])])) {
    parts = slice_or_raise(&parts.clone(), &index.clone(), &Py::None, &Py::None);
    found = Py::Bool(true);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(mcall(&g__JOINER(), "sub", &[py_str(""), part.clone()])); } py_list(__out) })() };
    whole = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(part.clone()); } py_list(__out) })() };
    out = py_list(vec![]);
    let __iter4 = py_iter_or_raise(&whole.clone());
    for __item in __iter4 {
    part = __item.clone();
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&mcall(&py_str(" "), "join", &[op_add(&out.clone(), &py_list(vec![part.clone()]))]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__MOVE_WORDS()) } }) {
    break;
    }
    let _ = mcall(&out.clone(), "append", &[part.clone()]);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&mcall(&py_str(" "), "join", &[out.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__MOVE_CUT()) } }) {
    kept = py_list(vec![]);
    let __iter5 = py_iter_or_raise(&out.clone());
    for __item in __iter5 {
    part = __item.clone();
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&mcall(&py_str(" "), "join", &[op_add(&kept.clone(), &py_list(vec![part.clone()]))]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__MOVE_CUT()) } }) {
    break;
    }
    let _ = mcall(&kept.clone(), "append", &[part.clone()]);
    }
    out = { let __b = kept.clone(); if py_truthy(&__b) { __b } else { slice_or_raise(&out.clone(), &Py::None, &Py::Int(1), &Py::None) } };
    }
    cut = if py_truthy(&out.clone()) { mcall(&py_str(", "), "join", &[out.clone()]) } else { first.clone() };
    cut = mcall(&cut.clone(), "strip", &[py_str(" ,;:-.")]);
    while py_truthy(&{ let __b = cut.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DANGLING(), &mcall(&mcall(&geti(&mcall(&cut.clone(), "split", &[]), &op_neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(" ,.;:-")]))) } }) {
    cut = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&cut.clone(), "split", &[]), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    cut = mcall(&cut.clone(), "strip", &[py_str(" ,;:-.")]);
    if py_truthy(&op_gt(&mcall(&cut.clone(), "count", &[py_str("(")]), &mcall(&cut.clone(), "count", &[py_str(")")]))) {
    cut = mcall(&slice_or_raise(&cut.clone(), &Py::None, &mcall(&cut.clone(), "index", &[py_str("(")]), &Py::None), "strip", &[py_str(" .;:-, ")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&cut.clone()))) {
    return py_str("");
    }
    return op_add(&op_add(&mcall(&geti(&cut.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&cut.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("This line is the limit itself, and there is an answer above it for the limit to change.\n\n    Declared rather than won on PRIORITY: the answers lane holds a card for a limit said on its\n    own (a limit said with no answer above it is a card in `bot/compose/circumstance.py`), and that\n    card answers a line nobody asked as a question. The limit is not a new topic - it is a change\n    to the answer already given.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_is(&_a_limit(&[text.clone()]), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[_the_answer(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("Nothing to keep: the limit is answered against the answer above it, every time.");
    return Py::None;
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut about: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut low: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut pick_from: Py = Py::None;
    let mut read: Py = Py::None;
    let mut record: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer in hand with the limit worked into it, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    read = _a_limit(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&read.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&read.clone(), 3, None);
    kind = __unpk1[0].clone();
    about = __unpk1[1].clone();
    leads = __unpk1[2].clone();
    record = _the_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&record.clone(), &Py::None))) {
    return Py::None;
    }
    answer = geti(&record.clone(), &py_str("reply"));
    low = mcall(&answer.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&_the_answer_is_about(&[record.clone(), about.clone()])))) {
    return Py::None;
    }
    move_ = _the_move(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&move_.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("rent"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("landlord"))) } }) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    pick_from = if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    lead = geti(&leads.clone(), &op_modulo(&pick_from.clone(), &call_builtin("len", &[leads.clone()]).unwrap_or_else(|e| exc::raise(e))));
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("move".to_string(), move_.clone()), ("move_lower".to_string(), op_add(&mcall(&slice_or_raise(&move_.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&move_.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) };
    return mcall(&py_str(" "), "join", &[mcall(&reply.clone(), "split", &[])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_limit_over_the_answer".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_ASKS" => Some(g__ASKS()),
            "_DANGLING" => Some(g__DANGLING()),
            "_INSTRUCTION" => Some(g__INSTRUCTION()),
            "_JOINER" => Some(g__JOINER()),
            "_KINDS" => Some(g__KINDS()),
            "_LEAD" => Some(g__LEAD()),
            "_LEAD_INS" => Some(g__LEAD_INS()),
            "_MOVE_CUT" => Some(g__MOVE_CUT()),
            "_MOVE_WORDS" => Some(g__MOVE_WORDS()),
            "_TAIL" => Some(g__TAIL()),
            "_a_limit" => Some(py_func("_a_limit", |args: &[Py]| _a_limit(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_the_answer" => Some(py_func("_the_answer", |args: &[Py]| _the_answer(args))),
            "_the_answer_is_about" => Some(py_func("_the_answer_is_about", |args: &[Py]| _the_answer_is_about(args))),
            "_the_move" => Some(py_func("_the_move", |args: &[Py]| _the_move(args))),
            "_what_the_answer_is_about" => Some(py_func("_what_the_answer_is_about", |args: &[Py]| _what_the_answer_is_about(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "_ASKS" => s__ASKS(v),
            "_DANGLING" => s__DANGLING(v),
            "_INSTRUCTION" => s__INSTRUCTION(v),
            "_JOINER" => s__JOINER(v),
            "_KINDS" => s__KINDS(v),
            "_LEAD" => s__LEAD(v),
            "_LEAD_INS" => s__LEAD_INS(v),
            "_MOVE_CUT" => s__MOVE_CUT(v),
            "_MOVE_WORDS" => s__MOVE_WORDS(v),
            "_TAIL" => s__TAIL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_limit_over_the_answer", module_obj);
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "_a_limit", py_func("_a_limit", |args: &[Py]| _a_limit(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "_the_answer", py_func("_the_answer", |args: &[Py]| _the_answer(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "_what_the_answer_is_about", py_func("_what_the_answer_is_about", |args: &[Py]| _what_the_answer_is_about(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "_the_answer_is_about", py_func("_the_answer_is_about", |args: &[Py]| _the_answer_is_about(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "_the_move", py_func("_the_move", |args: &[Py]| _the_move(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.a_limit_over_the_answer", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_ASKS", g__ASKS, s__ASKS);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_DANGLING", g__DANGLING, s__DANGLING);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_INSTRUCTION", g__INSTRUCTION, s__INSTRUCTION);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_JOINER", g__JOINER, s__JOINER);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_KINDS", g__KINDS, s__KINDS);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_LEAD_INS", g__LEAD_INS, s__LEAD_INS);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_MOVE_CUT", g__MOVE_CUT, s__MOVE_CUT);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_MOVE_WORDS", g__MOVE_WORDS, s__MOVE_WORDS);
    pyrt::xrefs::reg_access("modules.a_limit_over_the_answer", "_TAIL", g__TAIL, s__TAIL);
}
