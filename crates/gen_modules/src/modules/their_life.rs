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
thread_local! { static G__AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_READ_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MORE_THAN_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONE_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIBLINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIBLING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_AGE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_HOURS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_KIDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_KIDS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_WORK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_LINE_BEFORE_IS_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__AGE() -> Py { ensure_init(); G__AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE(v: Py) { ensure_init(); G__AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_LINES() -> Py { ensure_init(); G__AGE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_LINES(v: Py) { ensure_init(); G__AGE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_READ_ELSEWHERE() -> Py { ensure_init(); G__AGE_READ_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_READ_ELSEWHERE(v: Py) { ensure_init(); G__AGE_READ_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__HER() -> Py { ensure_init(); G__HER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HER(v: Py) { ensure_init(); G__HER.with(|c| *c.borrow_mut() = v); }
pub fn g__HIM() -> Py { ensure_init(); G__HIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HIM(v: Py) { ensure_init(); G__HIM.with(|c| *c.borrow_mut() = v); }
pub fn g__LIVE() -> Py { ensure_init(); G__LIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIVE(v: Py) { ensure_init(); G__LIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__MORE_THAN_ONE() -> Py { ensure_init(); G__MORE_THAN_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MORE_THAN_ONE(v: Py) { ensure_init(); G__MORE_THAN_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_AGE() -> Py { ensure_init(); G__NOT_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_AGE(v: Py) { ensure_init(); G__NOT_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_SHAPE() -> Py { ensure_init(); G__NOT_A_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SHAPE(v: Py) { ensure_init(); G__NOT_A_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORD() -> Py { ensure_init(); G__NUMBER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORD(v: Py) { ensure_init(); G__NUMBER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORDS() -> Py { ensure_init(); G__NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORDS(v: Py) { ensure_init(); G__NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__ONE_PET() -> Py { ensure_init(); G__ONE_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONE_PET(v: Py) { ensure_init(); G__ONE_PET.with(|c| *c.borrow_mut() = v); }
pub fn g__PET() -> Py { ensure_init(); G__PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET(v: Py) { ensure_init(); G__PET.with(|c| *c.borrow_mut() = v); }
pub fn g__PETS() -> Py { ensure_init(); G__PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PETS(v: Py) { ensure_init(); G__PETS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACES() -> Py { ensure_init(); G__PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACES(v: Py) { ensure_init(); G__PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE_KINDS() -> Py { ensure_init(); G__PLACE_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE_KINDS(v: Py) { ensure_init(); G__PLACE_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g__RELATION() -> Py { ensure_init(); G__RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RELATION(v: Py) { ensure_init(); G__RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g__SIBLINGS() -> Py { ensure_init(); G__SIBLINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIBLINGS(v: Py) { ensure_init(); G__SIBLINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__SIBLING_LINES() -> Py { ensure_init(); G__SIBLING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIBLING_LINES(v: Py) { ensure_init(); G__SIBLING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE() -> Py { ensure_init(); G__SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE(v: Py) { ensure_init(); G__SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_AGE() -> Py { ensure_init(); G__SOMEONE_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_AGE(v: Py) { ensure_init(); G__SOMEONE_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_AGE_LINES() -> Py { ensure_init(); G__SOMEONE_AGE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_AGE_LINES(v: Py) { ensure_init(); G__SOMEONE_AGE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_HOURS() -> Py { ensure_init(); G__SOMEONE_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_HOURS(v: Py) { ensure_init(); G__SOMEONE_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_HOURS_LINES() -> Py { ensure_init(); G__SOMEONE_HOURS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_HOURS_LINES(v: Py) { ensure_init(); G__SOMEONE_HOURS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_KIDS() -> Py { ensure_init(); G__SOMEONE_KIDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_KIDS(v: Py) { ensure_init(); G__SOMEONE_KIDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_KIDS_LINES() -> Py { ensure_init(); G__SOMEONE_KIDS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_KIDS_LINES(v: Py) { ensure_init(); G__SOMEONE_KIDS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_WORK() -> Py { ensure_init(); G__SOMEONE_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_WORK(v: Py) { ensure_init(); G__SOMEONE_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_WORK_LINES() -> Py { ensure_init(); G__SOMEONE_WORK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_WORK_LINES(v: Py) { ensure_init(); G__SOMEONE_WORK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__STORY() -> Py { ensure_init(); G__STORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STORY(v: Py) { ensure_init(); G__STORY.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_LINE_BEFORE_IS_HEAVY() -> Py { ensure_init(); G__THE_LINE_BEFORE_IS_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_LINE_BEFORE_IS_HEAVY(v: Py) { ensure_init(); G__THE_LINE_BEFORE_IS_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g__TONE() -> Py { ensure_init(); G__TONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TONE(v: Py) { ensure_init(); G__TONE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A fact about the person's own life, said on its own, gets its own turn.\n\nPeople drop the shape of their life into a conversation while it is about something else:\n\"we live near the park\", \"i live in a flat\", \"we rent\", \"we have a dog\", \"i've got two cats\".\nSaid mid-thread, none of them is a detail of the answer before it, and today two things can\nhappen instead of an answer to it:\n\n- after a knowledge answer, the rest of that answer is handed over again: \"my laptop broke\"\n  then \"we live near the park\" is answered \"If there is no light on the charger at all, try\n  another socket and cable\" (bot/modules/greeting.py, `the_step_they_have_not_had`, whose\n  `A_DETAIL_OF_IT` matches any \"in/near/on the X\" - the intended line there, \"its fine in the\n  kitchen\", has no subject of its own, and \"we live\" does);\n- the thread decides the entry: \"my back hurts\" then \"we have a dog\" is answered about a dog\n  in pain, which the person never said;\n\nand a fact with nothing else in the line is only nodded back (\"Noted: you have a sister and a\nbrother.\", \"Noted: you're 34.\" - the `FRAMES` receipt in bot/memory/ack.py, which knows the fact\nand has nothing to say about it).\n\nWhat a friend does with any of these is take the fact in and ask the one thing about it. That is\nall this module does, for the five shapes of ordinary life that turn up this way - where they\nlive, the animals in the house, the brothers and sisters they have, how old they are, and a\nplain detail about somebody else in their life - so a person who mentions their dog is not\nanswered about their back, and a person who mentions their sister is not handed their own\nsentence back. The last of those is the same fault one person out: \"my mum is 71\" and \"my wife\nworks three days a week\" were said back as \"Noted: your mum is 71.\" by the composer, which knows\nthe fact and has nothing to say about it, and a question about the person who was mentioned is\nwhat a friend asks.\n\nWhat a friend does with \"we live near the park\" is take the fact in and ask the one thing about\nit. That is all this module does, for the two shapes of ordinary life that turn up this way -\nwhere they live, and the animals in the house - so a person who mentions their dog is not left\nwith the answer to their back.\n\nOnly a whole line is one of these facts: a line that goes on to say something else (\"we have a\ndog that keeps barking at the postman\", \"we live near the park and the gate is broken\") keeps\nthe reading it has. A line in a heavy thread, or one carrying a symptom or a loss, is not a\nfact about the shape of their life, and stands this down.\n");
    s_PRIORITY(Py::Int(100));
    s__PLACES(py_tuple(vec![py_str("{Where} - what's it like there?"), py_str("How long have you been there?"), py_str("{Where}, then. Is it handy for what you need?")]));
    s__ONE_PET(py_tuple(vec![py_str("{Pet} - what's their name?"), py_str("How long have they been with you?"), py_str("{Pet} - who spoils them most?")]));
    s__PETS(py_tuple(vec![py_str("{Pets} - do they get on?"), py_str("{Pets} - what are their names?"), py_str("{Pets} - who spoils them most?")]));
    s__NUMBER_WORDS(py_dict(vec![(py_str("two"), py_str("Two")), (py_str("three"), py_str("Three")), (py_str("four"), py_str("Four")), (py_str("five"), py_str("Five")), (py_str("six"), py_str("Six"))]));
    s__NOT_A_SHAPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:pain|pains|painful|hurt|hurts|hurting|ill|sick|poorly|dying|died|dead|death|cancer|chemo|radiotherapy|tumour|tumor|dementia|alzheimers|alzheimer's|stroke|hospital|hospice|funeral|buried|passed away|lost|loss|bereave\\w*|griev\\w*|depress\\w*|anxious|anxiety|panic|suicid\\w*|self harm|crisis|emergency|broken|leaking|damp|mould|flood\\w*|evicted|eviction|arrears|debt|bailiff\\w*|divorce|separated|affair|abusive|violent|police|social services)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__STORY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:that|which|who|and|but|because|so|when|while|if|keeps?|kept|started|stopped|barking|bites|bit|chewed|escaped|ran|went|got|needs?|needed|wants?|made|makes|cost|costs|comes?|came|goes?|going|has|have|had|is|are|was|were|will|would|can|could)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\s+(?:tbh|tho|though|now|anyway|actually|really|literally|honestly|i\\s+suppose|i\\s+guess|still|also|too|by\\s+the\\s+way|btw|atm|at\\s+the\\s+moment|lol|ha|haha|lmao|so\\s+yeah|yeah|innit|mate|dude|as\\s+well))+$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<who>i|we)\\s+(?:live|stay)\\s+(?P<where>(?:in|near|on|by|above|below|behind|next\\s+to|opposite|at)\\s+(?:a|an|the|my|our|their|his|her)?\\s*[a-z][a-z' -]{0,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PLACE_KINDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:in|near|on|by|above|below|behind|next\\s+to|opposite|at)\\s+(?:a|an|the|my|our)?\\s*(?:flat|house|home|place|bungalow|cottage|farm|boat|caravan|car park|park|river|canal|sea|coast|beach|woods|wood|common|high\\s+street|village|town|city|estate|council\\s+estate|block|tower\\s+block|camper|motorhome|shop|pub|office|station|hospital\\s+car\\s+park|school|church|allotment)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<who>i|we)(?:'ve\\s+got|\\s+(?:have|got|own))\\s+(?P<num>a|an|one|two|three|four|five|six|\\d+|\\d+\\s+)?\\s*(?P<pet>dogs?|cats?|puppies|puppy|kittens?|kitten|rabbits?|hamsters?|budgies?|parrots?|horses?|pony|guinea\\s+pigs?|tortoises?|snakes?|lizards?|geckos?|fish|gerbils?|ferrets?|chickens?|hens?|ducks?|goats?|bees)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SIBLINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<who>i|we)(?:'ve\\s+got|\\s+(?:have|got))\\s+(?P<what>(?:(?:a|an|one|two|three|four|five|six|seven|eight|\\d+|older|younger|little|big|twin|step|half)[\\s-]+)*(?:brother|sister|brothers|sisters)(?:\\s*(?:,|and)\\s*)?(?:(?:a|an|one|two|three|four|five|six|seven|eight|\\d+|older|younger|little|big|twin|step|half)[\\s-]+)*(?:brothers?|sisters?)?)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SIBLING_LINES(py_tuple(vec![py_str("{What} - older or younger than you?"), py_str("{What} - do you see much of them?"), py_str("{What}, and do you get on?")]));
    s__AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:im|i'm|i\\s+am)\\s+(?P<age>\\d{1,2})(?:\\s*(?:years?\\s+old|yrs?\\s+old|y/?o|yo|now|today))?$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_AN_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:weeks?|days?|months?|pregnan\\w*|kilos?|kg|stone|pounds?|lbs?|miles?|minutes?|hours?|degrees?|percent|%|cm|feet|foot|ft)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AGE_LINES(py_tuple(vec![py_str("{Age} - how's this year treating you?"), py_str("{Age}, then. Anything you'd like to be different by {Next}?"), py_str("{Age} - a good age to be, or not?")]));
    s__RELATION(py_str("mother\\s+in\\s+law|father\\s+in\\s+law|son\\s+in\\s+law|daughter\\s+in\\s+law|sister\\s+in\\s+law|brother\\s+in\\s+law|step\\s?mum|stepmum|step\\s?dad|stepdad|step\\s?daughter|stepdaughter|step\\s?son|stepson|mum|mom|mother|mam|dad|father|gran|granny|grandma|nan|nana|grandad|grandpa|grandfather|sister|brother|son|daughter|aunt|auntie|uncle|cousin|niece|nephew|wife|husband|partner|boyfriend|girlfriend|grandson|granddaughter|grandchild"));
    s__SOMEONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^(?P<who>my|our)\\s+(?P<rel>"), &g__RELATION()), &py_str(")\\s+(?P<rest>[a-z0-9 ']+)$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMEONE_WORK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^works?\\s+(?:in|at|for)\\s+(?:a|an|the)?\\s*(?P<place>[a-z][a-z' ]{1,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMEONE_HOURS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^works?\\s+(?P<hours>nights?|days?|night\\s+shifts?|shifts|weekends?|evenings?|weekdays|part\\s+time|full\\s+time|(?:two|three|four|five|six)\\s+days\\s+a\\s+week|long\\s+hours|away\\s+a\\s+lot)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMEONE_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^is\\s+(?P<age>\\d{1,2})(?:\\s*(?:years?\\s+old|yrs?\\s+old|y/?o|yo|now|today))?$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AGE_READ_ELSEWHERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("boy"), py_str("girl"), py_str("eldest"), py_str("youngest"), py_str("grandson"), py_str("granddaughter"), py_str("grandchild"), py_str("grandchildren"), py_str("grandkids"), py_str("stepson"), py_str("stepdaughter"), py_str("step son"), py_str("step daughter"), py_str("son in law"), py_str("daughter in law"), py_str("niece"), py_str("nephew")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMEONE_KIDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^has\\s+(?P<num>\\d+|one|two|three|four|five|six|seven|eight)\\s+(?P<kids>kids|children|child|boys?|girls?|sons?|daughters?|twins?)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mother"), py_str("mam"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("nan"), py_str("nana"), py_str("sister"), py_str("daughter"), py_str("aunt"), py_str("auntie"), py_str("niece"), py_str("wife"), py_str("girlfriend"), py_str("stepmum"), py_str("step mum"), py_str("stepdaughter"), py_str("step daughter"), py_str("mother in law"), py_str("daughter in law"), py_str("sister in law"), py_str("granddaughter")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HIM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("dad"), py_str("father"), py_str("grandad"), py_str("grandpa"), py_str("grandfather"), py_str("brother"), py_str("son"), py_str("uncle"), py_str("nephew"), py_str("husband"), py_str("boyfriend"), py_str("stepdad"), py_str("step dad"), py_str("stepson"), py_str("step son"), py_str("father in law"), py_str("son in law"), py_str("brother in law"), py_str("grandson")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMEONE_WORK_LINES(py_tuple(vec![py_str("A {Place}, then. What's that like for {them}?"), py_str("{Place} - is that new for {them}?"), py_str("A {Place}, then. Do you hear much about it from {them}?")]));
    s__SOMEONE_HOURS_LINES(py_tuple(vec![py_str("{Hours} - does that leave much time for {them}?"), py_str("{Hours}, then. What's that like for {them}?"), py_str("{Hours} - does it suit {them}?")]));
    s__SOMEONE_AGE_LINES(py_tuple(vec![py_str("{Age} - do you see {them} much?"), py_str("{Age}, then. Do you get to see {them} often?"), py_str("{Age} - how often do you get to see {them}?")]));
    s__SOMEONE_KIDS_LINES(py_tuple(vec![py_str("{Num} kids - how old are they?"), py_str("{Num} kids, then. Do they live near you?"), py_str("{Num} kids - what are they into?")]));
    s__NUMBER_WORD(py_dict(vec![(py_str("1"), py_str("One")), (py_str("2"), py_str("Two")), (py_str("3"), py_str("Three")), (py_str("4"), py_str("Four")), (py_str("5"), py_str("Five")), (py_str("6"), py_str("Six")), (py_str("7"), py_str("Seven")), (py_str("8"), py_str("Eight"))]));
    s__MORE_THAN_ONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THE_LINE_BEFORE_IS_HEAVY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:ill|unwell|poorly|sick|dying|died|death|passed away|funeral|bereave\\w*|griev\\w*|cancer|chemo\\w*|radiotherap\\w*|tumou?r|dementia|alzheim\\w*|stroke|sepsis|surgery|operation|transplant|hospice|palliative|terminal|hospital|icu|ambulance|diagnos\\w*|bully\\w*|bullied|depress\\w*|suicid\\w*|self harm|miscarr\\w*|stillborn|addict\\w*|relapse\\w*|overdose\\w*|sectioned|in pain|suffering)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _more_than_one(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut num: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "num") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: num"))) };
    let mut said: Py = Py::None;
    let _ = py_str("True for \"two\", \"4\", \"10\" - the count in front of the animal, when there is one.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = num.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g__MORE_THAN_ONE(), &said.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("0"), py_str("1")]), &said.clone())) } };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut said: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let _ = py_str("The line as this reader wants it: lower case, no edge punctuation, no tone on the end.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "replace", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:-\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+")); __args.push(py_str(" ")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    while py_truthy(&Py::Bool(true)) {
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TONE(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &said.clone()))) {
    return said.clone();
    }
    said = trimmed.clone();
    }
    Py::None
}
pub fn _a_fact_about_their_life(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut lived: Py = Py::None;
    let mut num: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut siblings: Py = Py::None;
    let mut someone: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("The kind of fact this is, or None: \"place\", \"pet\", \"siblings\" or \"age\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_A_SHAPE(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    lived = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LIVE(), "match", args))(&__args) };
    if py_truthy(&{ let __b = lived.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("where")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lived.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PLACE_KINDS(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("where")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lived.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STORY(), "search", args))(&__args) })) } } } }) {
    return py_str("place");
    }
    siblings = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SIBLINGS(), "match", args))(&__args) };
    if py_truthy(&{ let __b = siblings.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&siblings.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STORY(), "search", args))(&__args) })) } }) {
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&siblings.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TONE(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&what.clone()) {
    return op_add(&py_str("siblings:"), &what.clone());
    }
    }
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AGE(), "match", args))(&__args) };
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_AN_AGE(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&age.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_le(&Py::Int(13), &__cmp11)) { op_le(&__cmp11, &Py::Int(99)) } else { Py::Bool(false) } } } } } }) {
    return op_add(&py_str("age:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&age.clone(), "group", args))(&__args) });
    }
    pet = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PET(), "match", args))(&__args) };
    if py_truthy(&{ let __b = pet.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STORY(), "search", args))(&__args) })) } }) {
    num = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("num")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    kind = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(num.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _more_than_one(args))(&__args) }) { py_str("two_pets") } else { py_str("pet") };
    return op_add(&op_add(&op_add(&op_add(&kind.clone(), &py_str(":")), &num.clone()), &py_str(":")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pet")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "group", args))(&__args) });
    }
    someone = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE(), "match", args))(&__args) };
    if py_truthy(&someone.clone()) {
    detail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rel")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&someone.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&someone.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_detail_about_someone(args))(&__args) };
    if py_truthy(&detail.clone()) {
    return op_add(&op_add(&op_add(&py_str("someone:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rel")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&someone.clone(), "group", args))(&__args) }), &py_str(":")), &detail.clone());
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_detail_about_someone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rel: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rel") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rel"))) };
    let mut rest: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut age: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut kids: Py = Py::None;
    let mut said: Py = Py::None;
    let mut worked: Py = Py::None;
    let _ = py_str("The plain detail said about somebody in their life, or None.\n\n    Three shapes, each written out as the detail the person named: \"work:a bank\", \"hours:nights\",\n    \"age:71\", \"kids:three\". Anything else about them is not this reader's - a symptom, a plan, a\n    judgement, a line with something else after the detail, and the animals or the place they live\n    in (tests/test_the_shape_of_their_life.py keeps those out of this reader). A child's age is not\n    asked here either: the composer's age bands answer it better (_AGE_READ_ELSEWHERE).\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    worked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE_WORK(), "match", args))(&__args) };
    if py_truthy(&{ let __b = worked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("place")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&worked.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STORY(), "search", args))(&__args) })) } }) {
    return op_add(&py_str("work:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("place")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&worked.clone(), "group", args))(&__args) }, "strip", args))(&__args) });
    }
    hours = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE_HOURS(), "match", args))(&__args) };
    if py_truthy(&hours.clone()) {
    return op_add(&py_str("hours:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hours")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hours.clone(), "group", args))(&__args) }, "strip", args))(&__args) });
    }
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE_AGE(), "match", args))(&__args) };
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&age.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(99)) } else { Py::Bool(false) } } } }) {
    if py_truthy(&Py::Bool(!py_contains(&g__AGE_READ_ELSEWHERE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = rel.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }))) {
    return op_add(&py_str("age:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&age.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    }
    kids = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE_KIDS(), "match", args))(&__args) };
    if py_truthy(&kids.clone()) {
    return op_add(&py_str("kids:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("num")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kids.clone(), "group", args))(&__args) }, "strip", args))(&__args) });
    }
    return Py::None;
    Py::None
}
pub fn _their_pronoun(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rel: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rel") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rel"))) };
    let mut said: Py = Py::None;
    let _ = py_str("Somebody in their life as an object: \"my mum\" -> \"her\", \"my cousin\" -> \"them\".");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+")); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = rel.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g__HER(), &said.clone()))) {
    return py_str("her");
    }
    if py_truthy(&Py::Bool(py_contains(&g__HIM(), &said.clone()))) {
    return py_str("him");
    }
    return py_str("them");
    Py::None
}
pub fn _where_words(args: &[Py]) -> Py {
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
    let mut lived: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The place said back in their own words: \"near the park\" -> \"Near the park\".");
    lived = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LIVE(), "match", args))(&__args) };
    where_ = if py_truthy(&lived.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("where")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lived.clone(), "group", args))(&__args) } } else { said.clone() };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&where_.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&where_.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    let mut _u: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut num: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line(args))(&__args) };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_fact_about_their_life(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "partition", args))(&__args) }, 3, None);
    kind = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    rest = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("siblings"), py_str("age"), py_str("someone")]), &kind.clone()))) {
    let __unpk2 = py_unpack(&py_tuple(vec![rest.clone(), py_str("")]), 2, None);
    num = __unpk2[0].clone();
    animal = __unpk2[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("pet"), py_str("two_pets")]), &kind.clone()))) {
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) }, 2, None);
    num = __unpk3[0].clone();
    animal = __unpk3[1].clone();
    } else {
    let __unpk4 = py_unpack(&py_tuple(vec![py_str(""), py_str("")]), 2, None);
    num = __unpk4[0].clone();
    animal = __unpk4[1].clone();
    }
    }
    let mut fallback: Py = gen_compose::compose::fallback::module_obj();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_the_thread_is_heavy", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_reads_as_heavy_news", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THE_LINE_BEFORE_IS_HEAVY(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push(num.clone()); __args.push(animal.clone()); __args.push(said.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_fact(args))(&__args) };
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the person said on the turn before this one, or \"\" on the first turn.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return if py_truthy(&turns.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _the_fact(args: &[Py]) -> Py {
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
    let mut num: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "num") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: num"))) };
    let mut animal: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _u: Py = Py::None;
    let mut count: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rel: Py = Py::None;
    let mut what: Py = Py::None;
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("siblings")))) {
    what = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&num.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&num.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{What}")); __args.push(what.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__SIBLING_LINES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) }, "replace", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("age")))) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__AGE_LINES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Next}")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(num.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Age}")); __args.push(num.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("place")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Where}")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _where_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__PLACES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) }, "replace", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("someone")))) {
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&num.clone(), "partition", args))(&__args) }, 3, None);
    rel = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    detail = __unpk1[2].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rel.clone()); __args.push(detail.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_someone_line(args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("two_pets")))) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__PETS()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(num.clone()); __args.push(py_str("The")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NUMBER_WORDS(), "get", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Pets}")); __args.push(op_add(&op_add(&count.clone(), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(animal.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _plural(args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) };
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__ONE_PET()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Pet}")); __args.push(op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&animal.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("An ") } else { py_str("A ") }, &animal.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) };
    Py::None
}
pub fn _the_someone_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rel: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rel") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rel"))) };
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _u: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut num: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut them: Py = Py::None;
    let _ = py_str("The question about the person who was mentioned: \"my mum is 71\" -> \"71 - do you see much\n    of her?\".");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&detail.clone(), "partition", args))(&__args) }, 3, None);
    kind = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    said = __unpk1[2].clone();
    them = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rel.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _their_pronoun(args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("work")))) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__SOMEONE_WORK_LINES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    place = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Place}")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "startswith", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "capitalize", args))(&__args) } } else { said.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{them}")); __args.push(them.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Place}")); __args.push(place.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("hours")))) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__SOMEONE_HOURS_LINES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{them}")); __args.push(them.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Hours}")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "capitalize", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("age")))) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__SOMEONE_AGE_LINES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{them}")); __args.push(them.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Age}")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__SOMEONE_KIDS_LINES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    num = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "capitalize", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NUMBER_WORD(), "get", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Num}")); __args.push(num.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) };
    Py::None
}
pub fn _plural(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The animal as more than one, whatever the person typed: \"cat\" -> \"cats\".");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "endswith", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("fish"), py_str("sheep"), py_str("deer")]), &said.clone())) } }) {
    return said.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("y")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "endswith", args))(&__args) }) {
    return op_add(&slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    }
    return op_add(&said.clone(), &py_str("s"));
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The first line of the pool this conversation has not said yet.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turn.clone(), &py_str("bot")), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "add", args))(&__args) };
    }
    }
    let __iter2 = py_iter_or_raise(&pool.clone());
    for __item in __iter2 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &line.clone()))) {
    return line.clone();
    }
    }
    return geti(&pool.clone(), &Py::Int(0));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.their_life".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "_AGE" => Some(g__AGE()),
            "_AGE_LINES" => Some(g__AGE_LINES()),
            "_AGE_READ_ELSEWHERE" => Some(g__AGE_READ_ELSEWHERE()),
            "_HER" => Some(g__HER()),
            "_HIM" => Some(g__HIM()),
            "_LIVE" => Some(g__LIVE()),
            "_MORE_THAN_ONE" => Some(g__MORE_THAN_ONE()),
            "_NOT_AN_AGE" => Some(g__NOT_AN_AGE()),
            "_NOT_A_SHAPE" => Some(g__NOT_A_SHAPE()),
            "_NUMBER_WORD" => Some(g__NUMBER_WORD()),
            "_NUMBER_WORDS" => Some(g__NUMBER_WORDS()),
            "_ONE_PET" => Some(g__ONE_PET()),
            "_PET" => Some(g__PET()),
            "_PETS" => Some(g__PETS()),
            "_PLACES" => Some(g__PLACES()),
            "_PLACE_KINDS" => Some(g__PLACE_KINDS()),
            "_RELATION" => Some(g__RELATION()),
            "_SIBLINGS" => Some(g__SIBLINGS()),
            "_SIBLING_LINES" => Some(g__SIBLING_LINES()),
            "_SOMEONE" => Some(g__SOMEONE()),
            "_SOMEONE_AGE" => Some(g__SOMEONE_AGE()),
            "_SOMEONE_AGE_LINES" => Some(g__SOMEONE_AGE_LINES()),
            "_SOMEONE_HOURS" => Some(g__SOMEONE_HOURS()),
            "_SOMEONE_HOURS_LINES" => Some(g__SOMEONE_HOURS_LINES()),
            "_SOMEONE_KIDS" => Some(g__SOMEONE_KIDS()),
            "_SOMEONE_KIDS_LINES" => Some(g__SOMEONE_KIDS_LINES()),
            "_SOMEONE_WORK" => Some(g__SOMEONE_WORK()),
            "_SOMEONE_WORK_LINES" => Some(g__SOMEONE_WORK_LINES()),
            "_STORY" => Some(g__STORY()),
            "_THE_LINE_BEFORE_IS_HEAVY" => Some(g__THE_LINE_BEFORE_IS_HEAVY()),
            "_TONE" => Some(g__TONE()),
            "_a_detail_about_someone" => Some(py_func("_a_detail_about_someone", |args: &[Py]| _a_detail_about_someone(args))),
            "_a_fact_about_their_life" => Some(py_func("_a_fact_about_their_life", |args: &[Py]| _a_fact_about_their_life(args))),
            "_line" => Some(py_func("_line", |args: &[Py]| _line(args))),
            "_more_than_one" => Some(py_func("_more_than_one", |args: &[Py]| _more_than_one(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_plural" => Some(py_func("_plural", |args: &[Py]| _plural(args))),
            "_the_fact" => Some(py_func("_the_fact", |args: &[Py]| _the_fact(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_someone_line" => Some(py_func("_the_someone_line", |args: &[Py]| _the_someone_line(args))),
            "_their_pronoun" => Some(py_func("_their_pronoun", |args: &[Py]| _their_pronoun(args))),
            "_where_words" => Some(py_func("_where_words", |args: &[Py]| _where_words(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "_AGE" => s__AGE(v),
            "_AGE_LINES" => s__AGE_LINES(v),
            "_AGE_READ_ELSEWHERE" => s__AGE_READ_ELSEWHERE(v),
            "_HER" => s__HER(v),
            "_HIM" => s__HIM(v),
            "_LIVE" => s__LIVE(v),
            "_MORE_THAN_ONE" => s__MORE_THAN_ONE(v),
            "_NOT_AN_AGE" => s__NOT_AN_AGE(v),
            "_NOT_A_SHAPE" => s__NOT_A_SHAPE(v),
            "_NUMBER_WORD" => s__NUMBER_WORD(v),
            "_NUMBER_WORDS" => s__NUMBER_WORDS(v),
            "_ONE_PET" => s__ONE_PET(v),
            "_PET" => s__PET(v),
            "_PETS" => s__PETS(v),
            "_PLACES" => s__PLACES(v),
            "_PLACE_KINDS" => s__PLACE_KINDS(v),
            "_RELATION" => s__RELATION(v),
            "_SIBLINGS" => s__SIBLINGS(v),
            "_SIBLING_LINES" => s__SIBLING_LINES(v),
            "_SOMEONE" => s__SOMEONE(v),
            "_SOMEONE_AGE" => s__SOMEONE_AGE(v),
            "_SOMEONE_AGE_LINES" => s__SOMEONE_AGE_LINES(v),
            "_SOMEONE_HOURS" => s__SOMEONE_HOURS(v),
            "_SOMEONE_HOURS_LINES" => s__SOMEONE_HOURS_LINES(v),
            "_SOMEONE_KIDS" => s__SOMEONE_KIDS(v),
            "_SOMEONE_KIDS_LINES" => s__SOMEONE_KIDS_LINES(v),
            "_SOMEONE_WORK" => s__SOMEONE_WORK(v),
            "_SOMEONE_WORK_LINES" => s__SOMEONE_WORK_LINES(v),
            "_STORY" => s__STORY(v),
            "_THE_LINE_BEFORE_IS_HEAVY" => s__THE_LINE_BEFORE_IS_HEAVY(v),
            "_TONE" => s__TONE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.their_life", module_obj);
    pyrt::xrefs::reg_fn("modules.their_life", "_more_than_one", py_func("_more_than_one", |args: &[Py]| _more_than_one(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_line", py_func("_line", |args: &[Py]| _line(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_a_fact_about_their_life", py_func("_a_fact_about_their_life", |args: &[Py]| _a_fact_about_their_life(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_a_detail_about_someone", py_func("_a_detail_about_someone", |args: &[Py]| _a_detail_about_someone(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_their_pronoun", py_func("_their_pronoun", |args: &[Py]| _their_pronoun(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_where_words", py_func("_where_words", |args: &[Py]| _where_words(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_the_fact", py_func("_the_fact", |args: &[Py]| _the_fact(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_the_someone_line", py_func("_the_someone_line", |args: &[Py]| _the_someone_line(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_plural", py_func("_plural", |args: &[Py]| _plural(args)));
    pyrt::xrefs::reg_fn("modules.their_life", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_access("modules.their_life", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.their_life", "_AGE", g__AGE, s__AGE);
    pyrt::xrefs::reg_access("modules.their_life", "_AGE_LINES", g__AGE_LINES, s__AGE_LINES);
    pyrt::xrefs::reg_access("modules.their_life", "_AGE_READ_ELSEWHERE", g__AGE_READ_ELSEWHERE, s__AGE_READ_ELSEWHERE);
    pyrt::xrefs::reg_access("modules.their_life", "_HER", g__HER, s__HER);
    pyrt::xrefs::reg_access("modules.their_life", "_HIM", g__HIM, s__HIM);
    pyrt::xrefs::reg_access("modules.their_life", "_LIVE", g__LIVE, s__LIVE);
    pyrt::xrefs::reg_access("modules.their_life", "_MORE_THAN_ONE", g__MORE_THAN_ONE, s__MORE_THAN_ONE);
    pyrt::xrefs::reg_access("modules.their_life", "_NOT_AN_AGE", g__NOT_AN_AGE, s__NOT_AN_AGE);
    pyrt::xrefs::reg_access("modules.their_life", "_NOT_A_SHAPE", g__NOT_A_SHAPE, s__NOT_A_SHAPE);
    pyrt::xrefs::reg_access("modules.their_life", "_NUMBER_WORD", g__NUMBER_WORD, s__NUMBER_WORD);
    pyrt::xrefs::reg_access("modules.their_life", "_NUMBER_WORDS", g__NUMBER_WORDS, s__NUMBER_WORDS);
    pyrt::xrefs::reg_access("modules.their_life", "_ONE_PET", g__ONE_PET, s__ONE_PET);
    pyrt::xrefs::reg_access("modules.their_life", "_PET", g__PET, s__PET);
    pyrt::xrefs::reg_access("modules.their_life", "_PETS", g__PETS, s__PETS);
    pyrt::xrefs::reg_access("modules.their_life", "_PLACES", g__PLACES, s__PLACES);
    pyrt::xrefs::reg_access("modules.their_life", "_PLACE_KINDS", g__PLACE_KINDS, s__PLACE_KINDS);
    pyrt::xrefs::reg_access("modules.their_life", "_RELATION", g__RELATION, s__RELATION);
    pyrt::xrefs::reg_access("modules.their_life", "_SIBLINGS", g__SIBLINGS, s__SIBLINGS);
    pyrt::xrefs::reg_access("modules.their_life", "_SIBLING_LINES", g__SIBLING_LINES, s__SIBLING_LINES);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE", g__SOMEONE, s__SOMEONE);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_AGE", g__SOMEONE_AGE, s__SOMEONE_AGE);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_AGE_LINES", g__SOMEONE_AGE_LINES, s__SOMEONE_AGE_LINES);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_HOURS", g__SOMEONE_HOURS, s__SOMEONE_HOURS);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_HOURS_LINES", g__SOMEONE_HOURS_LINES, s__SOMEONE_HOURS_LINES);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_KIDS", g__SOMEONE_KIDS, s__SOMEONE_KIDS);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_KIDS_LINES", g__SOMEONE_KIDS_LINES, s__SOMEONE_KIDS_LINES);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_WORK", g__SOMEONE_WORK, s__SOMEONE_WORK);
    pyrt::xrefs::reg_access("modules.their_life", "_SOMEONE_WORK_LINES", g__SOMEONE_WORK_LINES, s__SOMEONE_WORK_LINES);
    pyrt::xrefs::reg_access("modules.their_life", "_STORY", g__STORY, s__STORY);
    pyrt::xrefs::reg_access("modules.their_life", "_THE_LINE_BEFORE_IS_HEAVY", g__THE_LINE_BEFORE_IS_HEAVY, s__THE_LINE_BEFORE_IS_HEAVY);
    pyrt::xrefs::reg_access("modules.their_life", "_TONE", g__TONE, s__TONE);
}
