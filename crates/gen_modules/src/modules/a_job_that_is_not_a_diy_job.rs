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


thread_local! { static G_A_HAZARD_NAMING_THE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRIED_AND_UNPLUGGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOB_TO_TRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOB_TO_TRY_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TRYING_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGISTER_HINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WIRING_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRY_IT_MYSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_HAZARD_NAMING_THE_JOB() -> Py { ensure_init(); G_A_HAZARD_NAMING_THE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAZARD_NAMING_THE_JOB(v: Py) { ensure_init(); G_A_HAZARD_NAMING_THE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRIED_AND_UNPLUGGED() -> Py { ensure_init(); G_CARRIED_AND_UNPLUGGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRIED_AND_UNPLUGGED(v: Py) { ensure_init(); G_CARRIED_AND_UNPLUGGED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOB_TO_TRY() -> Py { ensure_init(); G_NOT_A_JOB_TO_TRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB_TO_TRY(v: Py) { ensure_init(); G_NOT_A_JOB_TO_TRY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOB_TO_TRY_POOL() -> Py { ensure_init(); G_NOT_A_JOB_TO_TRY_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB_TO_TRY_POOL(v: Py) { ensure_init(); G_NOT_A_JOB_TO_TRY_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TREE() -> Py { ensure_init(); G_NOT_A_TREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TREE(v: Py) { ensure_init(); G_NOT_A_TREE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TRYING_IT() -> Py { ensure_init(); G_NOT_TRYING_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TRYING_IT(v: Py) { ensure_init(); G_NOT_TRYING_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOW_WORDS() -> Py { ensure_init(); G_NOW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOW_WORDS(v: Py) { ensure_init(); G_NOW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_REGISTER_HINT() -> Py { ensure_init(); G_REGISTER_HINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGISTER_HINT(v: Py) { ensure_init(); G_REGISTER_HINT.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_WORDS() -> Py { ensure_init(); G_SELF_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_WORDS(v: Py) { ensure_init(); G_SELF_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WIRING_ITSELF() -> Py { ensure_init(); G_THE_WIRING_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WIRING_ITSELF(v: Py) { ensure_init(); G_THE_WIRING_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_TRY_IT_MYSELF() -> Py { ensure_init(); G_TRY_IT_MYSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRY_IT_MYSELF(v: Py) { ensure_init(); G_TRY_IT_MYSELF.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A job nobody should try first is answered with the trade that does it, and the reason.\n\n\"how do i repoint a chimney\", \"can i rewire the fuse box myself\" and \"can i take out a chimney\nbreast myself\" name a job the bot holds no card for, so they fell to the hedge - \"that's beyond\nwhat I know well, sorry. Searching for it will turn up better sources\" - which is true of every\nline and hands a person a ladder. `STYLE.md` asks for the part that can be known and then where\nthe rest lives: for these jobs the trade is not a matter of taste, and neither is the reason.\n\nThe reading is a job from the table below plus the line saying it is the person's own hands on it\n(\"how do i ...\", \"can i ... myself\", \"on my own\"), so it is a shape and not a list of wordings.\nTwo general rules, both tested:\n\n* **A hazard word in front of its own noun names the job**; standing alone it is a report. \"fix a\n  leaking skylight\" is a plan for a roofer, \"the water is leaking through the skylight\" is\n  happening now and keeps the answer it already has.\n* **A line that hires, prices or books somebody stands down**, because the reader for that names\n  a directory of people near you rather than the trade as such.\n\nNothing here is a fact about the world that could be wrong: the trade is the one that does that\nkind of work, and the reason is why it is not a weekend's job. The priority sits above the\ncomposer's intake form and hedge (-1000) and below every card, so a repair answer, a safety card\nor a knowledge entry keeps its turn, and the fences are as much the point as the table.\n");
    s_PRIORITY(op_neg(&Py::Int(999)));
    s_NOT_A_JOB_TO_TRY(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:chimney\\s+breast|(?:remove|take|knock|cut)\\s+(?:the\\s+|out\\s+the\\s+)?chimney\\s+(?:breast|stack)|chimney\\s+breast\\s+(?:out|removed|gone))(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a builder with a structural engineer"), py_str("whether it is carrying anything above it is an engineer's call, and that call decides how the rest of the job is done")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:chimney|chimney\\s+stack|chimney\\s+pot|chimney\\s+pots|flaunching|repoint(?:ing)?\\s+(?:the\\s+|a\\s+|my\\s+|our\\s+|this\\s+)?chimney)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a chimney sweep"), py_str("it is done from a scaffold above the roofline, and the fall is the part that goes wrong")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:gas\\s+(?:hob|fire|oven|cooker|pipe|pipes|pipework|supply|valve|appliance|boiler|meter)|lpg|calor\\s+gas)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a Gas Safe registered engineer"), py_str("on gas the work belongs to somebody on the register, and the number is one anyone can look up before they start")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:fuse\\s*box|fuse\\s+board|consumer\\s+unit|rewire|rewiring|ring\\s+main|socket\\s+circuit|extra\\s+socket|new\\s+socket|the\\s+wiring|the\\s+electrics|electric\\s+shower|shower\\s+unit)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("an electrician"), py_str("the mistake that matters sits behind the plaster and starts a fire where nobody is looking")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:remove|removing|knock\\s+through|knock\\s+out|take\\s+out|taking\\s+out|widen|open\\s+up|put\\s+a\\s+door\\s+in)\\s+(?:a\\s+|the\\s+|this\\s+|that\\s+|our\\s+)?(?:wall|walls|load\\s*bearing\\s+wall|wall\\s+between|partition)(?![a-z])|(?<![a-z])(?:take|taking|knock|knocking|cut|cutting|rip|ripping)\\s+(?:a\\s+|the\\s+|this\\s+|that\\s+)?(?:wall|walls|partition)\\s+(?:out|down)(?![a-z])|(?<![a-z])(?:load\\s*bearing|load-bearing|holding\\s+(?:the\\s+)?(?:house|roof|floor|ceiling)\\s+up|holds\\s+(?:the\\s+)?(?:house|roof|floor|ceiling)\\s+up|holding\\s+anything\\s+up|structural)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a builder with a structural engineer"), py_str("whether a wall is holding the floor up above it is decided by an engineer, not by how it looks once the plaster is off")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:fell|felling|cut\\s+down|cutting\\s+down|take\\s+down|taking\\s+down|pollard|pollarding|coppice|prune|pruning|trim)\\s+(?:a\\s+|the\\s+|my\\s+|this\\s+|that\\s+|our\\s+)?(?:tree|trees|conifer|conifers|poplar|willow|oak|sycamore|leylandii)(?![a-z])|(?<![a-z])(?:tree\\s+surgery|stump\\s+grinding|grind\\s+the\\s+stump)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a tree surgeon"), py_str("a tree goes where it leans once the last cut is made, and a bad one takes a fence, a roof or a person with it")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:wasp\\s+nest|wasps\\s+nest|hornets?\\s+nest|(?:wasps|hornets|bees)\\s+in\\s+the\\s+(?:loft|roof|wall|cavity|chimney)|nest\\s+of\\s+(?:wasps|hornets))(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a pest controller"), py_str("it is a suit and a long reach, and the ones that sting come at you when it goes wrong")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:asbestos|artex|old\\s+lagging|asbestos\\s+ceiling|asbestos\\s+roof)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a licensed asbestos remover"), py_str("it has to be identified before anyone cuts into it, and the test is cheap next to guessing")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:move|moving|remove|removing|replace|replacing|swap|swapping)\\s+(?:the\\s+|a\\s+|my\\s+|this\\s+|that\\s+|one\\s+of\\s+the\\s+)?(?:leaking\\s+|old\\s+|broken\\s+|rusty\\s+|stuck\\s+|noisy\\s+)?(?:radiator|radiators|rad|rads)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a plumber"), py_str("a radiator comes off wet and goes back on needing the system filled and bled, and a joint that weeps under the floor costs more than the fitting")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:the\\s+)?(?:roof|roofing|felt\\s+on\\s+the\\s+roof|roof\\s+felt|flashing|re-?felt|re-?roof|tiles?\\s+on\\s+the\\s+roof|ridge\\s+tiles?)(?![a-z])|(?<![a-z])(?:fit|fitting|replace|replacing|repair|repairing|fix|fixing|mend|mending|install|installing|leak|leaking|broken|re-?felt|re-?roof)\\s+(?:around\\s+)?(?:the\\s+|a\\s+|my\\s+)?(?:broken\\s+|cracked\\s+|old\\s+|damaged\\s+|stuck\\s+|leaking\\s+)?(?:skylight|velux|roof\\s+window)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("a roofer"), py_str("it is scaffold and two people, and the water from a job that fails costs more than the job")])]));
    s_TRY_IT_MYSELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:how\\s+(?:do|can|could|would|should)\\s+(?:i|we|you)\\b|how\\s+to\\b|whats\\s+the\\s+best\\s+way\\s+to\\b|what's\\s+the\\s+best\\s+way\\s+to\\b|whats\\s+the\\s+right\\s+way\\s+to\\b|is\\s+it\\s+(?:easy|hard|difficult|safe|worth\\s+it)\\s+to\\b|(?:is|would)\\s+(?:this|that|it)\\s+(?:a|an)\\s+(?:job|thing)\\s+(?:for\\s+me|i\\s+can)|can\\s+(?:i|we)\\s+(?:just\\s+)?(?:do|fix|fit|replace|remove|take|knock|cut|fell|wire|rewire|re-?felt|refelt|repoint|install|cap|climb|get\\s+up|dig|sand|scrape|strip|mend|move|swap|widen|plaster|prune|pollard|bleed|drain|have\\s+a\\s+go|try)\\b|should\\s+(?:i|we)\\s+(?:do|try|have\\s+a\\s+go|tackle)\\b)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SELF_WORDS(py_tuple(vec![py_str("myself"), py_str("my own"), py_str("by myself"), py_str("on my own"), py_str("diy"), py_str("ourselves"), py_str("have a go"), py_str("have a bash"), py_str("a go at it"), py_str("a go at that"), py_str("my own self")]));
    s_NOT_TRYING_IT(py_tuple(vec![py_str("find"), py_str("find me"), py_str("hire"), py_str("hiring"), py_str("recommend"), py_str("recommended"), py_str("looking for"), py_str("need a"), py_str("need an"), py_str("someone"), py_str("somebody"), py_str("anyone"), py_str("anybody"), py_str("quote"), py_str("quotes"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("how much"), py_str("near me"), py_str("local"), py_str("book"), py_str("booked"), py_str("call out"), py_str("callout"), py_str("directory"), py_str("reviews")]));
    s_NOW_WORDS(py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelling"), py_str("leaking"), py_str("leak"), py_str("leaks"), py_str("hissing"), py_str("sparking"), py_str("sparks"), py_str("smoke"), py_str("smoking"), py_str("on fire"), py_str("emergency"), py_str("right now"), py_str("just now"), py_str("tonight"), py_str("burst"), py_str("flooding"), py_str("collapsed"), py_str("falling down")]));
    s_A_HAZARD_NAMING_THE_JOB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|the|my|our|your|his|her|their|this|that|these|those)\\s+(?:leaking|leaky|smoking|smoky|hissing|sparking|burst|collapsed|flooding|blocked)\\s+(?=[a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_TREE(py_tuple(vec![py_str("christmas"), py_str("xmas"), py_str("family tree"), py_str("decision tree"), py_str("tree diagram")]));
    s_CARRIED_AND_UNPLUGGED(py_tuple(vec![py_str("lamp"), py_str("lamps"), py_str("table lamp"), py_str("desk lamp"), py_str("floor lamp"), py_str("bedside"), py_str("plug"), py_str("plugs"), py_str("flex"), py_str("lamp holder"), py_str("bayonet"), py_str("ceiling rose")]));
    s_THE_WIRING_ITSELF(py_tuple(vec![py_str("fuse box"), py_str("fuse board"), py_str("consumer unit"), py_str("the wiring"), py_str("the electrics"), py_str("socket circuit"), py_str("ring main"), py_str("electric shower"), py_str("shower unit"), py_str("shed"), py_str("garage"), py_str("extension"), py_str("loft")]));
    s_NOT_A_JOB_TO_TRY_POOL(py_tuple(vec![py_str("That is a job for {who}, not a ladder and an afternoon: {why}. Two quotes from people near you sets the price, and the second one usually says something the first did not."), py_str("{who_cap} is who takes this one on: {why}. Ring two near you - a quote costs nothing, and it tells you whether this is a small job or a bigger one than it looks.")]));
    s_REGISTER_HINT(py_dict(vec![(py_str("a Gas Safe registered engineer"), py_str("Ask for the Gas Safe number before you book.")), (py_str("an electrician"), py_str("Ask which competent-person scheme they are with before you book."))]));
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let _ = py_str("True when one of the phrases is in the line as a whole word or phrase.");
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z0-9])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?![a-z0-9])"))); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_job(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut pattern: Py = Py::None;
    let mut who: Py = Py::None;
    let mut why: Py = Py::None;
    let _ = py_str("(who does it, why it is not a weekend's job) for the job this line names, or None.");
    let __iter1 = py_iter_or_raise(&g_NOT_A_JOB_TO_TRY());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    pattern = __unpk2[0].clone();
    who = __unpk2[1].clone();
    why = __unpk2[2].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return py_tuple(vec![who.clone(), why.clone()]);
    }
    }
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
    else { Py::None };
    let mut fresh: Py = Py::None;
    let mut hint: Py = Py::None;
    let mut job: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_pool: Py = Py::None;
    let mut who: Py = Py::None;
    let mut why: Py = Py::None;
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_NOT_TRYING_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_HAZARD_NAMING_THE_JOB(), "sub", args))(&__args) }); __args.push(g_NOW_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has(args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TRY_IT_MYSELF(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_SELF_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has(args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_NOT_A_TREE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_CARRIED_AND_UNPLUGGED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_THE_WIRING_ITSELF()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has(args))(&__args) })) } }) {
    return Py::None;
    }
    job = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_job(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&job.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&job.clone(), 2, None);
    who = __unpk1[0].clone();
    why = __unpk1[1].clone();
    said_pool = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_JOB_TO_TRY_POOL()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone()), ("who_cap".to_string(), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&who.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&who.clone(), &Py::Int(1), &Py::None, &Py::None))), ("why".to_string(), why.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    hint = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REGISTER_HINT(), "get", args))(&__args) };
    if py_truthy(&hint.clone()) {
    said_pool = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![line.clone(), hint.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); } py_list(__out) })() };
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::last_bot(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&line.clone(), &last.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = fresh.clone(); if py_truthy(&__b) { __b } else { said_pool.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("random", "choice", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_job_that_is_not_a_diy_job".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_HAZARD_NAMING_THE_JOB" => Some(g_A_HAZARD_NAMING_THE_JOB()),
            "CARRIED_AND_UNPLUGGED" => Some(g_CARRIED_AND_UNPLUGGED()),
            "NOT_A_JOB_TO_TRY" => Some(g_NOT_A_JOB_TO_TRY()),
            "NOT_A_JOB_TO_TRY_POOL" => Some(g_NOT_A_JOB_TO_TRY_POOL()),
            "NOT_A_TREE" => Some(g_NOT_A_TREE()),
            "NOT_TRYING_IT" => Some(g_NOT_TRYING_IT()),
            "NOW_WORDS" => Some(g_NOW_WORDS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "REGISTER_HINT" => Some(g_REGISTER_HINT()),
            "SELF_WORDS" => Some(g_SELF_WORDS()),
            "THE_WIRING_ITSELF" => Some(g_THE_WIRING_ITSELF()),
            "TRY_IT_MYSELF" => Some(g_TRY_IT_MYSELF()),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "_the_job" => Some(py_func("_the_job", |args: &[Py]| _the_job(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_HAZARD_NAMING_THE_JOB" => s_A_HAZARD_NAMING_THE_JOB(v),
            "CARRIED_AND_UNPLUGGED" => s_CARRIED_AND_UNPLUGGED(v),
            "NOT_A_JOB_TO_TRY" => s_NOT_A_JOB_TO_TRY(v),
            "NOT_A_JOB_TO_TRY_POOL" => s_NOT_A_JOB_TO_TRY_POOL(v),
            "NOT_A_TREE" => s_NOT_A_TREE(v),
            "NOT_TRYING_IT" => s_NOT_TRYING_IT(v),
            "NOW_WORDS" => s_NOW_WORDS(v),
            "PRIORITY" => s_PRIORITY(v),
            "REGISTER_HINT" => s_REGISTER_HINT(v),
            "SELF_WORDS" => s_SELF_WORDS(v),
            "THE_WIRING_ITSELF" => s_THE_WIRING_ITSELF(v),
            "TRY_IT_MYSELF" => s_TRY_IT_MYSELF(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_job_that_is_not_a_diy_job", module_obj);
    pyrt::xrefs::reg_fn("modules.a_job_that_is_not_a_diy_job", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("modules.a_job_that_is_not_a_diy_job", "_the_job", py_func("_the_job", |args: &[Py]| _the_job(args)));
    pyrt::xrefs::reg_fn("modules.a_job_that_is_not_a_diy_job", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "A_HAZARD_NAMING_THE_JOB", g_A_HAZARD_NAMING_THE_JOB, s_A_HAZARD_NAMING_THE_JOB);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "CARRIED_AND_UNPLUGGED", g_CARRIED_AND_UNPLUGGED, s_CARRIED_AND_UNPLUGGED);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "NOT_A_JOB_TO_TRY", g_NOT_A_JOB_TO_TRY, s_NOT_A_JOB_TO_TRY);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "NOT_A_JOB_TO_TRY_POOL", g_NOT_A_JOB_TO_TRY_POOL, s_NOT_A_JOB_TO_TRY_POOL);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "NOT_A_TREE", g_NOT_A_TREE, s_NOT_A_TREE);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "NOT_TRYING_IT", g_NOT_TRYING_IT, s_NOT_TRYING_IT);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "NOW_WORDS", g_NOW_WORDS, s_NOW_WORDS);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "REGISTER_HINT", g_REGISTER_HINT, s_REGISTER_HINT);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "SELF_WORDS", g_SELF_WORDS, s_SELF_WORDS);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "THE_WIRING_ITSELF", g_THE_WIRING_ITSELF, s_THE_WIRING_ITSELF);
    pyrt::xrefs::reg_access("modules.a_job_that_is_not_a_diy_job", "TRY_IT_MYSELF", g_TRY_IT_MYSELF, s_TRY_IT_MYSELF);
}
