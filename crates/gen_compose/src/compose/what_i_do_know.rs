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


thread_local! { static G_ANIMAL_AS_A_SOURCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BUYING_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FREQUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PURCHASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOUGHT_GENERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BY_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLINICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERAL_BY_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MODIFIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUTRITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNED_ANYWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTICLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OF_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFERS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SINGULAR_BUT_ENDS_IN_S: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_IT_LIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANIMAL_AS_A_SOURCE() -> Py { ensure_init(); G_ANIMAL_AS_A_SOURCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_AS_A_SOURCE(v: Py) { ensure_init(); G_ANIMAL_AS_A_SOURCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BUYING_NOW() -> Py { ensure_init(); G_A_BUYING_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BUYING_NOW(v: Py) { ensure_init(); G_A_BUYING_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DETERMINER() -> Py { ensure_init(); G_A_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DETERMINER(v: Py) { ensure_init(); G_A_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FREQUENCY() -> Py { ensure_init(); G_A_FREQUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FREQUENCY(v: Py) { ensure_init(); G_A_FREQUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_METHOD() -> Py { ensure_init(); G_A_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_METHOD(v: Py) { ensure_init(); G_A_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PURCHASE() -> Py { ensure_init(); G_A_PURCHASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PURCHASE(v: Py) { ensure_init(); G_A_PURCHASE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REASON() -> Py { ensure_init(); G_A_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REASON(v: Py) { ensure_init(); G_A_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME() -> Py { ensure_init(); G_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME(v: Py) { ensure_init(); G_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_BOUGHT_GENERAL() -> Py { ensure_init(); G_BOUGHT_GENERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOUGHT_GENERAL(v: Py) { ensure_init(); G_BOUGHT_GENERAL.with(|c| *c.borrow_mut() = v); }
pub fn g_BY_KIND() -> Py { ensure_init(); G_BY_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BY_KIND(v: Py) { ensure_init(); G_BY_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_CLINICAL() -> Py { ensure_init(); G_CLINICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLINICAL(v: Py) { ensure_init(); G_CLINICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING() -> Py { ensure_init(); G_COOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING(v: Py) { ensure_init(); G_COOKING.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION() -> Py { ensure_init(); G_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION(v: Py) { ensure_init(); G_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERAL_BY_PART() -> Py { ensure_init(); G_GENERAL_BY_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERAL_BY_PART(v: Py) { ensure_init(); G_GENERAL_BY_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_GOT_IT() -> Py { ensure_init(); G_GOT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOT_IT(v: Py) { ensure_init(); G_GOT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLDS() -> Py { ensure_init(); G_HOLDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLDS(v: Py) { ensure_init(); G_HOLDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS() -> Py { ensure_init(); G_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); G_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVING() -> Py { ensure_init(); G_LIVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVING(v: Py) { ensure_init(); G_LIVING.with(|c| *c.borrow_mut() = v); }
pub fn g_MADE() -> Py { ensure_init(); G_MADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MADE(v: Py) { ensure_init(); G_MADE.with(|c| *c.borrow_mut() = v); }
pub fn g_MODIFIERS() -> Py { ensure_init(); G_MODIFIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MODIFIERS(v: Py) { ensure_init(); G_MODIFIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_OBJECT() -> Py { ensure_init(); G_NOT_AN_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_OBJECT(v: Py) { ensure_init(); G_NOT_AN_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NUTRITION() -> Py { ensure_init(); G_NUTRITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUTRITION(v: Py) { ensure_init(); G_NUTRITION.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_BODY() -> Py { ensure_init(); G_ON_THE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_BODY(v: Py) { ensure_init(); G_ON_THE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNED_ANYWAY() -> Py { ensure_init(); G_OWNED_ANYWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNED_ANYWAY(v: Py) { ensure_init(); G_OWNED_ANYWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTICLES() -> Py { ensure_init(); G_PARTICLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTICLES(v: Py) { ensure_init(); G_PARTICLES.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OF_SOMETHING() -> Py { ensure_init(); G_PART_OF_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OF_SOMETHING(v: Py) { ensure_init(); G_PART_OF_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_WORD() -> Py { ensure_init(); G_PART_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_WORD(v: Py) { ensure_init(); G_PART_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_REFERS_BACK() -> Py { ensure_init(); G_REFERS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFERS_BACK(v: Py) { ensure_init(); G_REFERS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_SINGULAR_BUT_ENDS_IN_S() -> Py { ensure_init(); G_SINGULAR_BUT_ENDS_IN_S.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SINGULAR_BUT_ENDS_IN_S(v: Py) { ensure_init(); G_SINGULAR_BUT_ENDS_IN_S.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL() -> Py { ensure_init(); G_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); G_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_IT_LIVES() -> Py { ensure_init(); G_WHERE_IT_LIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_IT_LIVES(v: Py) { ensure_init(); G_WHERE_IT_LIVES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The half of an answer a person can have: the thing, what is known about it, where the rest lives.\n\nThe composer's hedge is the same four words whatever was asked - \"That's beyond what I know well,\nsorry. Searching for it will turn up better sources than I can offer\" - and it names nothing. A\nperson who has said what the thing is gets a line that would fit any message in the deck:\n\n    how much exercise does it need        (after \"i got a puppy last week\")\n                                    \"I don't know enough to answer that honestly. A search should\n                                     sort you out quickly - and I'm happy to help with the rest\"\n    how often do i clean it               (after \"i got a humidifier\") the same hedge\n    how long till i get used to them      (after \"i got new glasses\") the same hedge\n\nThe rule is not \"name the thing and send them to a website\": a pointer on its own is not a reply,\nbecause a person who is told \"the maker's page has it\" has learned nothing they did not already\nknow. STYLE.md's \"Not knowing, honestly\" gives the shape, and this file builds a reply of four\nparts:\n\n1. the part that cannot be checked, in a few words (\"I can't check the schedule for that\n   humidifier, sorry\");\n2. **what the bot does know about that kind of thing** - the shape of the job, the usual first\n   step, what being behind looks like (`BY_KIND`, then `GENERAL_BY_PART`);\n3. the one place that keeps the exact part, by kind (the maker's page, the label and the RHS, the\n   optician, the paperwork, the venue's own page, the vet, the letter from the organisation);\n4. one question, so the turn keeps going.\n\nNone of part 2 is a fact about one model: a habit of a whole kind of thing is not a made-up fact,\nand a figure for a model nobody here has seen would be. The file knows where a kind of fact is\nkept, and the shape of the job that produces it, so it can answer a kettle it has never heard of\nwithout inventing anything.\n\nThe thing itself comes from the reading in `memory[\"understood\"]`, then from the phrase after\n`my/our/the/a`, then - for a follow-up that says only \"it\" - from the words of the turn before. It\nstands down on every line where the missing part is somebody's own knowledge instead: a person the\nbot has never met (`only_they_know.py`), a body or a live crisis (#answers, #safety), food, cooking,\na purchase, a judgement put to it, and a line that names nothing at all.\n\n`bot/modules/greeting.py` asks this only where the composer's own reply is its hedge, so a line\nwith a card of its own never reaches it.\n");
    s_A_METHOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:do|can|could|should|would) i\\b|\\bhow to\\b|\\bwhats the best way\\b|\\bwhat is the best way\\b|\\bhow do you\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat time\\b|\\bwhat day\\b|\\bwhich day\\b|\\bhow soon\\b|\\bwhat date\\b|\\bwhen\\b|\\bhow long (?:till|until|before|after|to)\\b|\\bopen (?:today|tomorrow|on)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhere\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FREQUENCY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:often|frequently)\\b|\\bhow many times\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:much|many|long|often|far|big|heavy|hot|old|deep|tall|wide)\\b|\\bhow much\\b|\\bwhat (?:size|weight|percentage)\\b|\\bhow much does\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_REASON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhy\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PART_WORD(py_dict(vec![(py_str("frequency"), py_str("the schedule")), (py_str("figure"), py_str("the figure")), (py_str("time"), py_str("the timing")), (py_str("place"), py_str("the right place")), (py_str("method"), py_str("the exact steps")), (py_str("reason"), py_str("the reason"))]));
    s_KINDS(py_tuple(vec![py_tuple(vec![py_str("clinical"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tens|nebulisers?|nebulizers?|cpap|bipap|inhalers?|spacers?|crutches?|walking (?:frame|stick|aid)|zimmer|catheters?|stoma|insulin|blood pressure monitor|glucose meter|hearing aids?|compression (?:stocking|stockings|sleeve|sock|socks|tights)|oxygen concentrator|feeding tube|peg tube)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("animal"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dog|dogs|puppy|pup|cat|cats|kitten|rabbit|hamster|guinea pig|budgie|parrot|fish|horse|pony|chickens?|hens?|ducks?|tortoise|snake|lizard|pet|pets|puppies|kittens)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("plant"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:plants?|trees?|shrubs?|roses?|tomato(?:es)?|potato(?:es)?|grass|lawn|hedge|seedlings?|houseplants?|orchids?|cactus|fern|bulbs?|flowers?|vegetables?|herbs?|garden|cuttings?|compost)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("appliance"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:boiler|kettle|washer|washing machine|dishwasher|oven|cooker|hob|fridge|freezer|fridge freezer|radiators?|heaters?|hoover|vacuum|dryer|tumble dryer|extractor|shower|taps?|sink|mowers?|lawnmower|drill|microwave|toaster|iron|mattress|humidifiers?|dehumidifiers?|air fryer|slow cooker|pressure cooker|bread maker|breadmaker|coffee machine|blender|mixer|food processor|sewing machine|stitch|strimmer|hedge trimmer|chainsaw|sander|pressure washer|jet wash|air conditioner|air con|heat pump|fan|extractor fan|freezer|tumble|washer dryer|water softener|\\w+(?:ifier|ator|iser|izer))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("device"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:phone|laptop|tablet|computer|pc|router|wifi|broadband|console|smart watch|app|account|email|password|website|browser|spreadsheet|software|printer|speaker|telly|tv|television|privacy settings?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("paper"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bill|statement|letter|form|invoice|receipt|contract|ticket|warranty|guarantee|passport|licence|license|payslip|tenancy|policy|quote|estimate|certificate|application|booking|reservation|reference number)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("place"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:museum|castle|cathedral|minster|gallery|theatre|theater|zoo|park|beach|market|library|station|airport|hotel|cafe|restaurant|pub|shop|store|supermarket|swimming pool|leisure centre|farm|stadium|arena|church|abbey|palace|pier|harbour|harbor|city|town|village|island|venue|exhibition)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("eyewear"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:glasses|specs|spectacles|contact lenses?|lenses|varifocals?|sunglasses)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("service"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gp|doctor|dentist|vet|bank|council|school|nursery|surgery|hospital|insurance|landlord|letting agent|solicitor|builder|plumber|electrician|garage|post office|jobcentre|university|college)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s_WHERE_IT_LIVES(py_dict(vec![(py_str("animal"), py_str("The vet or the practice's own page has it for the breed and the age, and the rehoming charity that matches the breed prints a figure too.")), (py_str("plant"), py_str("The label that came with that one has it, and the grower's or the RHS page keeps one by plant name and the time of year.")), (py_str("appliance"), py_str("The maker's own page has it for the model number on the rating plate, and the manual says it in the specification pages.")), (py_str("device"), py_str("The maker's support page has it for the model, and the settings screen on the thing itself shows what it is set to now.")), (py_str("paper"), py_str("The paperwork has it: the figure, the date and the reference are printed on the document itself, and the number to ring is on the back.")), (py_str("place"), py_str("The venue's own page or its map listing has it, and it changes with the day, so today's is the one that counts.")), (py_str("eyewear"), py_str("The optician who fitted them has it, and the practice's own page says what to expect and when to go back.")), (py_str("service"), py_str("The organisation's own page has it, and the number on your last letter from them reaches the desk that decides.")), (py_str("part"), py_str("The manual or the fitting sheet for the thing it belongs to has the step for that part, and the maker's page has it for the model number, which is on the rating plate or the paperwork that came with it.")), (py_str("bought"), py_str("The label or the paperwork that came with it is where its own numbers are, and the shop that sold it will tell you over the phone what it is and what it wants.")), (py_str("living"), py_str("The people who keep them have it rather than a maker: the local association, the supplier you got them from, or a keeper a few years in will tell you the calendar for your area.")), (py_str("clinical"), py_str("Whoever prescribed or fitted it has it: the plan is written on the sheet they gave you, and the clinic or the pharmacist on that sheet answers a question about it."))]));
    s_HOLDS(py_dict(vec![(py_str("animal"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("frequency"), py_str("method"), py_str("reason")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("part"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("method"), py_str("figure"), py_str("place")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("plant"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("frequency"), py_str("method"), py_str("reason"), py_str("time")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("appliance"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("frequency"), py_str("method"), py_str("reason")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("device"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("frequency"), py_str("method"), py_str("reason")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("paper"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("time")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("place"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("time"), py_str("place")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("eyewear"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("frequency"), py_str("method"), py_str("reason"), py_str("time")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("service"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("time"), py_str("place"), py_str("method")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("bought"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("figure"), py_str("frequency"), py_str("method"), py_str("reason")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("living"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("frequency"), py_str("method"), py_str("reason"), py_str("time")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("clinical"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("frequency"), py_str("method"), py_str("reason"), py_str("time")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_COOKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cook|cooks|cooked|cooking|roast|bake|boil|fry|grill|eat|eating|edible|recipe|oven|marinade|defrost|reheat|serve|serving|portion|dinner|lunch|tea|pie|soup|stock|stuffing)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PURCHASE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:new one|a new|to buy|buy|buys|buying|cheapest|price of|replace|replacement|cost of a new|how much is|how much are)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MADE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("appliance"), py_str("device"), py_str("part")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GENERAL_BY_PART(py_dict(vec![(py_str("method"), py_tuple(vec![py_str("the first step is nearly always the same one - turn it off, wait, and start again from the beginning of the maker's own instructions"), py_str("the order matters more than the steps: the maker's list is written to be done from the top, and skipping ahead is what leaves a job half finished")])), (py_str("frequency"), py_tuple(vec![py_str("how often it needs doing depends on how hard it works, so the busier it is the shorter the gap"), py_str("there is a usual interval and it moves with how much it is used, with the sign you are behind showing as a change you notice rather than a number")])), (py_str("figure"), py_tuple(vec![py_str("the figure is set by the model and the size, so two of the same kind of thing can differ by a lot"), py_str("the number on the thing itself - the plate, the box, the letter - is the one that decides it, not the one in the advert")])), (py_str("time"), py_tuple(vec![py_str("the timing is set by the calendar rather than by the person, so the season or the day is what to go by"), py_str("there is usually one date that matters and everything else is worked back from it")])), (py_str("place"), py_tuple(vec![py_str("the usual place is the one that already does that job, rather than a special trip"), py_str("anywhere that sells or services the thing will know who locally does the rest")])), (py_str("reason"), py_tuple(vec![py_str("there are usually two or three common causes, and which one it is shows in when it happens and what changes with it"), py_str("the way to tell the causes apart is to change one thing at a time and watch what changes")]))]));
    s_BY_KIND(py_dict(vec![(py_tuple(vec![py_str("bought"), py_str("frequency")]), py_tuple(vec![py_str("there is no number that fits every one of them: what it is made of, how much it is used and where it is kept all move it"), py_str("the sign you are behind shows in the thing itself rather than on a date - how it sounds, smells or looks changes before anything else does")])), (py_tuple(vec![py_str("clinical"), py_str("frequency")]), py_tuple(vec![py_str("how often it is used is set by whoever prescribed it and by how you are doing, not by a number that fits everyone: the plan on their sheet is the one that counts"), py_str("the dose of it is the part that changes, so the gap between uses comes from the review rather than from the instruction leaflet")])), (py_tuple(vec![py_str("clinical"), py_str("method")]), py_tuple(vec![py_str("the way to use it is the way it was shown to you, and the sheet or the clinic will show you again rather than leave you guessing"), py_str("the settings are the part to get right, so it is worth a phone call to the clinic rather than a guess at the dial")])), (py_tuple(vec![py_str("clinical"), py_str("time")]), py_tuple(vec![py_str("how long you keep using it is theirs to say, and it is looked at again at the review they booked"), py_str("the length of it is set by how the thing it is for is healing rather than by the calendar")])), (py_tuple(vec![py_str("living"), py_str("frequency")]), py_tuple(vec![py_str("how often it needs doing is set by the season and by how the animals are doing rather than by a maker: a look most weeks in the busy months and less in the cold, with the sign you are behind showing in them rather than in a number"), py_str("the calendar follows the year - the swarming months, the cold, the hungry gap - so two keepers in two places will not do it on the same week")])), (py_tuple(vec![py_str("plant"), py_str("time")]), py_tuple(vec![py_str("garden jobs go by the season more than the calendar: sow while the soil is warm, plant bare-root things while they are dormant, prune most shrubs after they flower"), py_str("the soil and the weather decide the week, and the packet holds the variety and the depth")])), (py_tuple(vec![py_str("plant"), py_str("frequency")]), py_tuple(vec![py_str("watering goes by the compost rather than the day: dry an inch down means water, damp means wait"), py_str("a feed every couple of weeks in the growing season is the usual, and nothing in winter")])), (py_tuple(vec![py_str("part"), py_str("method")]), py_tuple(vec![py_str("the order is what matters: isolate it first - water off at the valve, or unplugged - then take the old part out and match it, because it comes out one way and goes back the same way"), py_str("the old part is the pattern: take it to the shop and match it rather than trusting a model number alone, since the same model can be made with two different fittings")])), (py_tuple(vec![py_str("animal"), py_str("frequency")]), py_tuple(vec![py_str("how much and how often is set by the age and the weight more than the breed, and the two things worth watching are the weight and what is left in the bowl"), py_str("the amount goes up with age and down with age again, and a vet weighs the animal rather than guessing")])), (py_tuple(vec![py_str("animal"), py_str("figure")]), py_tuple(vec![py_str("the figure is set by the age and the size of the animal rather than by the breed name"), py_str("the two numbers that matter are the weight and the amount, and the bag or the tin prints both")])), (py_tuple(vec![py_str("appliance"), py_str("method")]), py_tuple(vec![py_str("nearly every appliance has a reset that the maker built in: off at the wall for a minute, then back on and let it finish starting up"), py_str("the model number is the thing to have in front of you - it is on the rating plate or inside the door - because the maker's steps are written for that model and not the one beside it")])), (py_tuple(vec![py_str("device"), py_str("method")]), py_tuple(vec![py_str("the first step on nearly all of them is the same: restart it, then look for the setting the message named rather than changing several at once"), py_str("the settings screen shows what it is set to now, which is usually the thing that is wrong")])), (py_tuple(vec![py_str("eyewear"), py_str("time")]), py_tuple(vec![py_str("most people are told a week or two to settle into new glasses, and the first thing to check is that they sit straight and level"), py_str("the first days are the ones that feel wrong, and a headache that lasts past that is worth going back with")])), (py_tuple(vec![py_str("paper"), py_str("figure")]), py_tuple(vec![py_str("the document carries the figure, the date and a reference, and that reference is what unlocks the answer on the phone"), py_str("the number to ring is printed on the letter rather than found on a website")])), (py_tuple(vec![py_str("place"), py_str("time")]), py_tuple(vec![py_str("opening hours move with the season, the day and the odd refurbishment, so the day itself is the one to check"), py_str("the hours on a listing are a snapshot, and the venue's own page is the one that is kept current")])), (py_tuple(vec![py_str("service"), py_str("method")]), py_tuple(vec![py_str("the first step with any of them is to ask in writing and quote the reference, because that is what makes a record"), py_str("the desk that decides is reached by the number on their own letter, not the switchboard")])), (py_tuple(vec![py_str("bought"), py_str("method")]), py_tuple(vec![py_str("the sheet, label or card that came with it is the shortest route, and the shop that sold it will tell you the way it is normally done"), py_str("the first step is the one written on whatever came in the box or on the label, and it is written for that thing rather than for its kind")]))]));
    s_TAIL(py_dict(vec![(py_str("frequency"), py_tuple(vec![py_str("Say how it has been going so far and I will tell you what the usual is."), py_str("Tell me what you have been doing with it and I will say whether that is enough.")])), (py_str("figure"), py_tuple(vec![py_str("Tell me the number you are working from and I will do the arithmetic with you."), py_str("Say what you are comparing it with and I will help you weigh it.")])), (py_str("time"), py_tuple(vec![py_str("Tell me when you need it by and I will work the timing back from that."), py_str("Say what you are working around and I will lay out the order to do things in.")])), (py_str("place"), py_tuple(vec![py_str("Tell me which one you have in mind and I will say what to ask them.")])), (py_str("method"), py_tuple(vec![py_str("Tell me what you have already tried and I will go through the next step with you.")])), (py_str("reason"), py_tuple(vec![py_str("Tell me what you have noticed and I will say what usually causes it.")]))]));
    s_A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:mum|mom|mother|dad|father|son|daughter|kid|kids|child|children|baby|toddler|boss|manager|partner|husband|wife|boyfriend|girlfriend|friend|neighbour|neighbor|sister|brother|gran|granny|nan|aunt|uncle|cousin|teacher|carer)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the it this that these those there here me you him her us them my your his its our their\nsomething anything nothing everything one ones any some all more most less much many\nthing things stuff\ntime times day days week weeks month months year years hour hours minute minutes\nsomebody anybody nobody everyone\nnew old secondhand second-hand little big small large same other good bad broken first next last\ndecent proper nice cheap expensive\ngot get gets got bought buy buying buyed ordered order arrived delivered picked treated\nhad have has want wants need needs keep keeps make makes made put puts use uses used\nclean cleans cleaned fix fixes fixed change changes take takes took give gives gave\nhow what when where why which who whose often much many long soon till until\ncan could should would will shall may might must does did is are was were be been being am\nabout around anything something\nat in on from with for to of by under over into near beside behind inside outside after before\nduring than like as per via onto upon within without across along towards toward through\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GOT_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:i|we|he|she|they|mum|dad)\\s+(?:have\\s+|had\\s+|just\\s+|finally\\s+)?(?:got|bought|ordered|won|were given|was given|picked up|treated myself to)|(?:it|they|this|that|the [a-z]+)\\s+(?:has\\s+|had\\s+)?(?:arrived|come|turned up)|\\bdelivered\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANIMAL_AS_A_SOURCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:pets?|dogs?|cats?|puppy|puppies|kitten|kittens|rabbit|hamster|horse|pony|tortoise|parrot|budgie)\\s+(?:'s\\s+)?(?:hair|hairs|fur|fluff|moult|moulting|mess|poo|poop|poos|wee|sick|vomit|smell|stink|stain|stains|mud|slobber|drool|spit|paw prints?|footprints?|blanket|bed|basket|cage|hutch|litter|tray|toy|toys|lead|leash|collar|bowl|bowls|food|biscuits|hairbrush|brush)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SINGULAR_BUT_ENDS_IN_S({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nmattress glass address gas bus dress class canvas tennis business harness lens iris\npress stress boss mess\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REFERS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|its|them|they|those|that one|this one|the one|that|this)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DETERMINER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|the|this|these|a|an)\\s+[a-z]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BUYING_NOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:after an?|in the market|looking at an?|thinking (?:of|about) (?:buying|getting)|should i (?:buy|get)|shall i (?:buy|get)|is it worth buying|which one (?:shall|should) i)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NUTRITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:calorie|calories|kcal|protein|protien|carbs?|carbohydrate|vitamins?|fibre|fiber|nutrients?|how many grams|grams? of)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MODIFIERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nnew old older newer secondhand second-hand used brand little small big large long short tall wide\nnarrow round square tiny huge giant same other another good bad poor decent proper nice cheap\nexpensive pricey posh fancy rubbish rubbishy broken faulty dodgy spare extra main whole front back\ntop bottom side inner outer left right first second third last next previous electric electrical\nmanual digital smart wooden metal plastic glass stainless steel leather cotton woolen woollen\nfurry fluffy heavy light hard soft dark bright white black grey gray red blue green yellow pink\npurple brown orange silver gold clear empty full wet dry hot cold warm cool clean dirty\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PARTICLES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\noff on out up down back away apart over in into loose together around about through round\nplease again now then too also as well\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i mean|i meant|i mean't|sorry i mean|no i mean|rather than|not that,?|i should have said|what i mean is)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LIVING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:beehive|bee hive|hive|colony|nuc|bees|wormery|worms|compost|pond|aquarium|fish tank|hens|chickens|quail|mushroom kit|starter|terrarium)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_AN_OBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nhair hairs head face back neck shoulder shoulders knee knees leg legs foot feet arm arms hand hands\nfinger fingers thumb skin eye eyes ear ears tooth teeth mouth tongue jaw stomach tummy belly gut\nchest cough breath voice sleep mood temper day days week weeks weekend month months year years\ntime life job work shift shifts money rent bill bills debt weight appetite headache head cold\ntoenail nails toothache backache stress anxiety energy patience brain memory\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWNED_ANYWAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the|my|our)\\s+[a-z][a-z' -]{0,26}?\\b(?:is|are|was|were|has|have|had|keeps|kept|went|gone|goes|stopped|stops|broke|broken|won't|wont|doesn't|doesnt|isn't|isnt|smells?|makes?|making|sounds?|leaks?|leaking|arrived|turned up)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ON_THE_BODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:wounds?|cuts?|grazes?|grazed|burns?|scalds?|blisters?|bruises?|rashes?|sores?|spots?|ulcers?|abscess|scar|scars|stitches|staples|sutures|tattoos?|piercings?|fillings?|crowns?|dentures?|denture|retainers?|aligners?|implants?|bridges?|veneer|veneers|dentist|gums?|tooth|teeth|stump|amputation|prosthes[ie]s|prosthetic|stoma|colostomy|dressings?|bandages?|plasters?|splints?|casts?|wig|wigs|toupee|hairpiece|injection|injections|insulin pen|catheter|drip|line|port|pacemaker|coil|implant)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLINICAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tens|tens machine|tens unit|nebuliser|nebulizer|cpap|bipap|inhaler|spacer|crutch|crutches|walking (?:frame|stick|aid)|zimmer|catheter|stoma|insulin|blood pressure monitor|blood sugar|glucose meter|hearing aid|hearing aids|compression (?:stocking|stockings|sleeve|sock|socks|tights)|oxygen|oxygen concentrator|feeding tube|peg tube|syringe driver)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PART_OF_SOMETHING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nhead seal seals filter filters drawer lid cap hose belt blade blades tray element washer valve\naerator spout handle handles sole soles heel heels zip zipper button buttons strap buckle wick\nfilament plug plugs gasket bearing impeller pump thermostat cartridge cartridges hinge catch latch\nknob dial flange nut bolt screw screws\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BOUGHT_GENERAL(py_dict(vec![(py_str("figure"), py_tuple(vec![py_str("the number on the thing itself, or on the label or paperwork that came with it, is the one that decides it rather than the one in the advert"), py_str("the size and the make are what the figure follows from, so two of the same kind of thing can differ by a lot")])), (py_str("reason"), py_tuple(vec![py_str("there are usually two or three common causes, and which one it is shows in when it happens and what changes with it"), py_str("the way to tell the causes apart is to change one thing at a time and watch what changes")]))]));
}
pub fn _parts_of(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut rule: Py = Py::None;
    let _ = py_str("The part of the answer the line is after, by the first shape that matches.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("method"), g_A_METHOD()]), py_tuple(vec![py_str("time"), g_A_TIME()]), py_tuple(vec![py_str("place"), g_A_PLACE()]), py_tuple(vec![py_str("frequency"), g_A_FREQUENCY()]), py_tuple(vec![py_str("figure"), g_A_FIGURE()]), py_tuple(vec![py_str("reason"), g_A_REASON()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    rule = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rule.clone(), "search", args))(&__args) }) {
    return name.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _kind_of(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut rule: Py = Py::None;
    let _ = py_str("The kind of thing the line names, or \"\" when it names none of them.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ANIMAL_AS_A_SOURCE(), "search", args))(&__args) }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&g_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    rule = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rule.clone(), "search", args))(&__args) }) {
    return kind.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _that(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let _ = py_str("The word to put in front of the thing, singular or plural, from how people say it.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SINGULAR_BUT_ENDS_IN_S(), &thing.clone())) } }) {
    return py_str("those");
    }
    return py_str("that");
    Py::None
}
pub fn _without_a_particle(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PARTICLES(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "pop", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _a_thing_from(args: &[Py]) -> Py {
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
    let mut determiner_only: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "determiner_only") { v.clone() }
    else { Py::Bool(false) };
    let mut best: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut run: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing a person's own line names, by the words around it, or \"\".\n\n    The understanding engine knows the words it has been taught, and the things people bring to a\n    conversation are exactly the ones no list here has: a humidifier, a bread maker, a pair of\n    glasses. So the words of the line are read here instead - the last run of ordinary words that\n    is not a function word - and a run too short to be a thing is not one.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|the|this|these|a|an)\\s+([a-z][a-z'-]+(?:\\s+[a-z][a-z'-]+){0,3})")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "split", args))(&__args) };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MODIFIERS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "pop", args))(&__args) };
    }
    kept = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MODIFIERS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } }) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)))) {
    break;
    }
    }
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_a_particle(args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&kept.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    }
    if py_truthy(&determiner_only.clone()) {
    return py_str("");
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z][a-z'-]+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    run = py_list(vec![]);
    best = py_str("");
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone()))) {
    if py_truthy(&run.clone()) {
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&run.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    run = py_list(vec![]);
    }
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&run.clone(), "append", args))(&__args) };
    }
    if py_truthy(&run.clone()) {
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&run.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&best.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_a_particle(args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&best.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isalpha", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_str("");
    }
    return best.clone();
    Py::None
}
pub fn _thing_named_in(args: &[Py]) -> Py {
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
    let mut corrected: Py = Py::None;
    let mut mark: Py = Py::None;
    let _ = py_str("The thing a line names, read the way the line means it.\n\n    A line that corrects itself means the second thing it says (\"the seal is fine, i mean the\n    little round filter at the bottom\"): read straight through, the first phrase is the one the\n    person has just ruled out, and a later question that only says \"it\" would keep pointing at a\n    thing they have already taken back.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    mark = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION(), "search", args))(&__args) };
    if py_truthy(&mark.clone()) {
    corrected = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mark.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); __kws.extend(vec![("determiner_only".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_from(args))(&__args) };
    if py_truthy(&corrected.clone()) {
    return corrected.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_from(args))(&__args) };
    Py::None
}
pub fn _the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut back: Py = Py::None;
    let mut before: Py = Py::None;
    let mut before_text: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut ent: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_before: Py = Py::None;
    let mut named_now: Py = Py::None;
    let mut points_back: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The thing the line is about, said the way the person said it, or \"\".\n\n    The engine's own subject is used when it has one, because it has already weighed the whole\n    line; then the last noun it recorded; then the words of the person's own line, which is where a\n    thing no list here knows comes from. A follow-up that says only \"it\" takes the turn before,\n    because the person has not changed what they are talking about - and that is exactly when the\n    hedge names nothing.\n    ");
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) };
    before_text = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    named_before = if py_truthy(&before_text.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing_named_in(args))(&__args) } } else { py_str("") };
    back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REFERS_BACK(), "search", args))(&__args) };
    points_back = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(back.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&back.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DETERMINER(), "search", args))(&__args) })) } };
    if py_truthy(&{ let __b = points_back.clone(); if !py_truthy(&__b) { __b } else { named_before.clone() } }) {
    return named_before.clone();
    }
    mark = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTION(), "search", args))(&__args) };
    if py_truthy(&mark.clone()) {
    corrected = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mark.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); __kws.extend(vec![("determiner_only".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_from(args))(&__args) };
    if py_truthy(&corrected.clone()) {
    return corrected.clone();
    }
    }
    named_now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("determiner_only".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_from(args))(&__args) };
    if py_truthy(&named_now.clone()) {
    return named_now.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING(), &subject.clone())) } }) {
    return subject.clone();
    }
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("entities")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    ent = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ent.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ent.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING(), &text.clone())) } }) {
    return text.clone();
    }
    }
    }
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING(), &subject.clone())) } }) {
    return subject.clone();
    }
    let __iter2 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("entities")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    ent = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ent.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ent.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING(), &text.clone())) } }) {
    return text.clone();
    }
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing_named_in(args))(&__args) };
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn before_text_from(args: &[Py]) -> Py {
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
    let _ = py_str("The words of the person's previous line, or \"\".");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let mut now: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's previous line, word for word, for a follow-up that names \"it\".\n\n    The store keeps the words of every turn (`turns`, with `user` and `bot`), and the reading of\n    the current line is on `understood`; the reading of an older line is not kept, so the words\n    themselves are what this file reads. That is enough: the thing is named in the line, and this\n    file only needs the name.\n    ");
    now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, &now.clone())) } }) {
    return py_dict(vec![(py_str("text"), said.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _kind_from_the_owning_line(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut source: Py = Py::None;
    let mut sources: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kind of thing, when the person's own line says they got it, or \"\".\n\n    A thing no list here knows still has a kind when the line that named it says it was bought or\n    arrived: whatever it is, the shop that sold it and the maker's page hold its model number, its\n    running cost and how to reset it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_str("");
    }
    sources = py_list(vec![{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, py_dict(vec![(py_str("text"), said.clone())])]);
    let __iter1 = py_iter_or_raise(&sources.clone());
    for __item in __iter1 {
    source = __item.clone();
    text = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(source.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&source.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(!py_contains(&text.clone(), &thing.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_AN_OBJECT(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &op_add(&py_str("my "), &thing.clone()))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &op_add(&py_str("our "), &thing.clone()))); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GOT_IT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWNED_ANYWAY(), "search", args))(&__args) } } } } } } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIVING(), "search", args))(&__args) }) {
    return py_str("living");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLINICAL(), "search", args))(&__args) }) {
    return py_str("clinical");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ON_THE_BODY(), "search", args))(&__args) }) {
    return py_str("");
    }
    return py_str("bought");
    }
    }
    return py_str("");
    Py::None
}
pub fn _known(args: &[Py]) -> Py {
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
    let mut part: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let _ = py_str("What this file can say about the kind of thing, before it says where the rest is.\n\n    A kind only speaks about itself. `BY_KIND` is the sharpest and always wins; then the machine\n    kinds, whose habits `GENERAL_BY_PART` describes; then `bought`, which may say only what is\n    true of anything a person came by. Everything else returns nothing, and the hedge stands.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_BY_KIND(), &py_tuple(vec![kind.clone(), part.clone()])))) {
    return geti(&g_BY_KIND(), &py_tuple(vec![kind.clone(), part.clone()]));
    }
    if py_truthy(&Py::Bool(py_contains(&g_MADE(), &kind.clone()))) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GENERAL_BY_PART(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("bought")))) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BOUGHT_GENERAL(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { py_str("") };
    let mut before: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut known: Py = Py::None;
    let mut knowns: Py = Py::None;
    let mut options: Py = Py::None;
    let mut part: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut tails: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What is known about the thing, where the rest of it lives, and one question - or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::is_a_hedge(args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::in_a_safety_thread(args))(&__args) }) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_the_body(args))(&__args) }) {
    return Py::None;
    }
    read = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } else { Py::None };
    read = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { read.clone() } else { py_dict(vec![]) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COOKING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PURCHASE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NUTRITION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| before_text_from(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BUYING_NOW(), "search", args))(&__args) } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ANIMAL_AS_A_SOURCE(), "search", args))(&__args) }) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(said.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ANIMAL_AS_A_SOURCE(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }) {
    return Py::None;
    }
    kind = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PART_OF_SOMETHING(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { py_str("part") } else { py_str("") };
    kind = { let __b = kind.clone(); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _kind_of(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _kind_of(args))(&__args) } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    before = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_before(args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); __args.push(py_dict(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _kind_of(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _kind_of(args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(thing.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _kind_from_the_owning_line(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _parts_of(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOLDS(), "get", args))(&__args) }, &part.clone()))) {
    return Py::None;
    }
    where_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHERE_IT_LIVES(), "get", args))(&__args) };
    knowns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _known(args))(&__args) };
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PART_WORD(), "get", args))(&__args) };
    tails = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TAIL(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&where_.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&knowns.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&tails.clone())) } } } } } }) {
    return Py::None;
    }
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&knowns.clone()) { let mut known: Py = __item.clone(); for __item in py_iter_or_raise(&tails.clone()) { let mut tail: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("word".to_string(), word.clone()), ("dem".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _that(args))(&__args) }), ("thing".to_string(), thing.clone()), ("known".to_string(), known.clone()), ("where".to_string(), where_.clone()), ("tail".to_string(), tail.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("I can't check {word} for {dem} {thing}, sorry. What I can say is that {known}. {where} {tail}"), "format", args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push(read.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.what_i_do_know".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANIMAL_AS_A_SOURCE" => Some(g_ANIMAL_AS_A_SOURCE()),
            "A_BUYING_NOW" => Some(g_A_BUYING_NOW()),
            "A_DETERMINER" => Some(g_A_DETERMINER()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_FREQUENCY" => Some(g_A_FREQUENCY()),
            "A_METHOD" => Some(g_A_METHOD()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_PURCHASE" => Some(g_A_PURCHASE()),
            "A_REASON" => Some(g_A_REASON()),
            "A_TIME" => Some(g_A_TIME()),
            "BOUGHT_GENERAL" => Some(g_BOUGHT_GENERAL()),
            "BY_KIND" => Some(g_BY_KIND()),
            "CLINICAL" => Some(g_CLINICAL()),
            "COOKING" => Some(g_COOKING()),
            "CORRECTION" => Some(g_CORRECTION()),
            "GENERAL_BY_PART" => Some(g_GENERAL_BY_PART()),
            "GOT_IT" => Some(g_GOT_IT()),
            "HOLDS" => Some(g_HOLDS()),
            "KINDS" => Some(g_KINDS()),
            "LIVING" => Some(g_LIVING()),
            "MADE" => Some(g_MADE()),
            "MODIFIERS" => Some(g_MODIFIERS()),
            "NOT_AN_OBJECT" => Some(g_NOT_AN_OBJECT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NUTRITION" => Some(g_NUTRITION()),
            "ON_THE_BODY" => Some(g_ON_THE_BODY()),
            "OWNED_ANYWAY" => Some(g_OWNED_ANYWAY()),
            "PARTICLES" => Some(g_PARTICLES()),
            "PART_OF_SOMETHING" => Some(g_PART_OF_SOMETHING()),
            "PART_WORD" => Some(g_PART_WORD()),
            "REFERS_BACK" => Some(g_REFERS_BACK()),
            "SINGULAR_BUT_ENDS_IN_S" => Some(g_SINGULAR_BUT_ENDS_IN_S()),
            "TAIL" => Some(g_TAIL()),
            "WHERE_IT_LIVES" => Some(g_WHERE_IT_LIVES()),
            "_a_thing_from" => Some(py_func("_a_thing_from", |args: &[Py]| _a_thing_from(args))),
            "_kind_from_the_owning_line" => Some(py_func("_kind_from_the_owning_line", |args: &[Py]| _kind_from_the_owning_line(args))),
            "_kind_of" => Some(py_func("_kind_of", |args: &[Py]| _kind_of(args))),
            "_known" => Some(py_func("_known", |args: &[Py]| _known(args))),
            "_parts_of" => Some(py_func("_parts_of", |args: &[Py]| _parts_of(args))),
            "_that" => Some(py_func("_that", |args: &[Py]| _that(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_thing_named_in" => Some(py_func("_thing_named_in", |args: &[Py]| _thing_named_in(args))),
            "_without_a_particle" => Some(py_func("_without_a_particle", |args: &[Py]| _without_a_particle(args))),
            "before_text_from" => Some(py_func("before_text_from", |args: &[Py]| before_text_from(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANIMAL_AS_A_SOURCE" => s_ANIMAL_AS_A_SOURCE(v),
            "A_BUYING_NOW" => s_A_BUYING_NOW(v),
            "A_DETERMINER" => s_A_DETERMINER(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_FREQUENCY" => s_A_FREQUENCY(v),
            "A_METHOD" => s_A_METHOD(v),
            "A_PERSON" => s_A_PERSON(v),
            "A_PLACE" => s_A_PLACE(v),
            "A_PURCHASE" => s_A_PURCHASE(v),
            "A_REASON" => s_A_REASON(v),
            "A_TIME" => s_A_TIME(v),
            "BOUGHT_GENERAL" => s_BOUGHT_GENERAL(v),
            "BY_KIND" => s_BY_KIND(v),
            "CLINICAL" => s_CLINICAL(v),
            "COOKING" => s_COOKING(v),
            "CORRECTION" => s_CORRECTION(v),
            "GENERAL_BY_PART" => s_GENERAL_BY_PART(v),
            "GOT_IT" => s_GOT_IT(v),
            "HOLDS" => s_HOLDS(v),
            "KINDS" => s_KINDS(v),
            "LIVING" => s_LIVING(v),
            "MADE" => s_MADE(v),
            "MODIFIERS" => s_MODIFIERS(v),
            "NOT_AN_OBJECT" => s_NOT_AN_OBJECT(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NUTRITION" => s_NUTRITION(v),
            "ON_THE_BODY" => s_ON_THE_BODY(v),
            "OWNED_ANYWAY" => s_OWNED_ANYWAY(v),
            "PARTICLES" => s_PARTICLES(v),
            "PART_OF_SOMETHING" => s_PART_OF_SOMETHING(v),
            "PART_WORD" => s_PART_WORD(v),
            "REFERS_BACK" => s_REFERS_BACK(v),
            "SINGULAR_BUT_ENDS_IN_S" => s_SINGULAR_BUT_ENDS_IN_S(v),
            "TAIL" => s_TAIL(v),
            "WHERE_IT_LIVES" => s_WHERE_IT_LIVES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.what_i_do_know", module_obj);
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_parts_of", py_func("_parts_of", |args: &[Py]| _parts_of(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_kind_of", py_func("_kind_of", |args: &[Py]| _kind_of(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_that", py_func("_that", |args: &[Py]| _that(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_without_a_particle", py_func("_without_a_particle", |args: &[Py]| _without_a_particle(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_a_thing_from", py_func("_a_thing_from", |args: &[Py]| _a_thing_from(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_thing_named_in", py_func("_thing_named_in", |args: &[Py]| _thing_named_in(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "before_text_from", py_func("before_text_from", |args: &[Py]| before_text_from(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_kind_from_the_owning_line", py_func("_kind_from_the_owning_line", |args: &[Py]| _kind_from_the_owning_line(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "_known", py_func("_known", |args: &[Py]| _known(args)));
    pyrt::xrefs::reg_fn("compose.what_i_do_know", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.what_i_do_know", "ANIMAL_AS_A_SOURCE", g_ANIMAL_AS_A_SOURCE, s_ANIMAL_AS_A_SOURCE);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_BUYING_NOW", g_A_BUYING_NOW, s_A_BUYING_NOW);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_DETERMINER", g_A_DETERMINER, s_A_DETERMINER);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_FREQUENCY", g_A_FREQUENCY, s_A_FREQUENCY);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_METHOD", g_A_METHOD, s_A_METHOD);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_PURCHASE", g_A_PURCHASE, s_A_PURCHASE);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_REASON", g_A_REASON, s_A_REASON);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "A_TIME", g_A_TIME, s_A_TIME);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "BOUGHT_GENERAL", g_BOUGHT_GENERAL, s_BOUGHT_GENERAL);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "BY_KIND", g_BY_KIND, s_BY_KIND);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "CLINICAL", g_CLINICAL, s_CLINICAL);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "COOKING", g_COOKING, s_COOKING);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "CORRECTION", g_CORRECTION, s_CORRECTION);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "GENERAL_BY_PART", g_GENERAL_BY_PART, s_GENERAL_BY_PART);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "GOT_IT", g_GOT_IT, s_GOT_IT);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "HOLDS", g_HOLDS, s_HOLDS);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "KINDS", g_KINDS, s_KINDS);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "LIVING", g_LIVING, s_LIVING);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "MADE", g_MADE, s_MADE);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "MODIFIERS", g_MODIFIERS, s_MODIFIERS);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "NOT_AN_OBJECT", g_NOT_AN_OBJECT, s_NOT_AN_OBJECT);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "NUTRITION", g_NUTRITION, s_NUTRITION);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "ON_THE_BODY", g_ON_THE_BODY, s_ON_THE_BODY);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "OWNED_ANYWAY", g_OWNED_ANYWAY, s_OWNED_ANYWAY);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "PARTICLES", g_PARTICLES, s_PARTICLES);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "PART_OF_SOMETHING", g_PART_OF_SOMETHING, s_PART_OF_SOMETHING);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "PART_WORD", g_PART_WORD, s_PART_WORD);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "REFERS_BACK", g_REFERS_BACK, s_REFERS_BACK);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "SINGULAR_BUT_ENDS_IN_S", g_SINGULAR_BUT_ENDS_IN_S, s_SINGULAR_BUT_ENDS_IN_S);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "TAIL", g_TAIL, s_TAIL);
    pyrt::xrefs::reg_access("compose.what_i_do_know", "WHERE_IT_LIVES", g_WHERE_IT_LIVES, s_WHERE_IT_LIVES);
}
