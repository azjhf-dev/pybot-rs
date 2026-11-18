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


thread_local! { static G_AN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BENIGN_FINDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FINDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOOD_TURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEDICAL_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SCARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITS_THE_SPEAKERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATED_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ABOUT_THEIR_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MEDICAL_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_LANDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SERIOUS_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ILLNESS_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_THING_RULED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WAIT_AFTER_A_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_ANIMAL() -> Py { ensure_init(); G_AN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ANIMAL(v: Py) { ensure_init(); G_AN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BENIGN_FINDING() -> Py { ensure_init(); G_A_BENIGN_FINDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BENIGN_FINDING(v: Py) { ensure_init(); G_A_BENIGN_FINDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FINDING() -> Py { ensure_init(); G_A_FINDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FINDING(v: Py) { ensure_init(); G_A_FINDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOOD_TURN() -> Py { ensure_init(); G_A_GOOD_TURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_TURN(v: Py) { ensure_init(); G_A_GOOD_TURN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEDICAL_TEST() -> Py { ensure_init(); G_A_MEDICAL_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEDICAL_TEST(v: Py) { ensure_init(); G_A_MEDICAL_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_ABOUT_IT() -> Py { ensure_init(); G_A_QUESTION_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_ABOUT_IT(v: Py) { ensure_init(); G_A_QUESTION_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SCARE() -> Py { ensure_init(); G_A_SCARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SCARE(v: Py) { ensure_init(); G_A_SCARE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORRY() -> Py { ensure_init(); G_A_WORRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORRY(v: Py) { ensure_init(); G_A_WORRY.with(|c| *c.borrow_mut() = v); }
pub fn g_ITS_THE_SPEAKERS() -> Py { ensure_init(); G_ITS_THE_SPEAKERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITS_THE_SPEAKERS(v: Py) { ensure_init(); G_ITS_THE_SPEAKERS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATED_CONDITION() -> Py { ensure_init(); G_NEGATED_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATED_CONDITION(v: Py) { ensure_init(); G_NEGATED_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ABOUT_THEIR_BODY() -> Py { ensure_init(); G_NOT_ABOUT_THEIR_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_THEIR_BODY(v: Py) { ensure_init(); G_NOT_ABOUT_THEIR_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MEDICAL_TEST() -> Py { ensure_init(); G_NOT_A_MEDICAL_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MEDICAL_TEST(v: Py) { ensure_init(); G_NOT_A_MEDICAL_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_LANDED() -> Py { ensure_init(); G_NOT_LANDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_LANDED(v: Py) { ensure_init(); G_NOT_LANDED.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SERIOUS_CONDITION() -> Py { ensure_init(); G_OWN_SERIOUS_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SERIOUS_CONDITION(v: Py) { ensure_init(); G_OWN_SERIOUS_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT() -> Py { ensure_init(); G_RULED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT(v: Py) { ensure_init(); G_RULED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_TAIL() -> Py { ensure_init(); G_RULED_OUT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_TAIL(v: Py) { ensure_init(); G_RULED_OUT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_WAIT() -> Py { ensure_init(); G_THEIR_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_WAIT(v: Py) { ensure_init(); G_THEIR_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ILLNESS_GOING() -> Py { ensure_init(); G_THE_ILLNESS_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ILLNESS_GOING(v: Py) { ensure_init(); G_THE_ILLNESS_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NEWS() -> Py { ensure_init(); G_THE_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NEWS(v: Py) { ensure_init(); G_THE_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_THING_RULED_OUT() -> Py { ensure_init(); G_THE_THING_RULED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_THING_RULED_OUT(v: Py) { ensure_init(); G_THE_THING_RULED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WAIT() -> Py { ensure_init(); G_THE_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WAIT(v: Py) { ensure_init(); G_THE_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WAIT_AFTER_A_TEST() -> Py { ensure_init(); G_THE_WAIT_AFTER_A_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WAIT_AFTER_A_TEST(v: Py) { ensure_init(); G_THE_WAIT_AFTER_A_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RELATIVE() -> Py { ensure_init(); G__A_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RELATIVE(v: Py) { ensure_init(); G__A_RELATIVE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Waiting for an answer about a body is not a bad day at work.\n\nFive lines were answered by the generic pool, whose warmest line is \"That sounds rough. If you\nwant, I can help you think through what to do next.\" - the line a person gets for a missed bus, said\nto somebody in the fortnight between a scan and an answer:\n\n    they think its cancer            -> \"That sounds rough. If you want, I can help you...\"\n    theyre testing me for cancer     -> \"Ugh, that's a lot to carry.\"\n    my mum might have cancer         -> \"I'm sorry - that's frustrating. Want a hand...\"\n    im worried about my results      -> \"Nerves before something big are normal...\"  (the driving test)\n    its not cancer                   -> \"Sorry to hear that.\"   (the opposite of the news)\n\nNothing is known yet in four of them, and the fifth is the news going the right way, so no card and\nno pool for a bad day is the right thing to say. What is knowable is the wait itself and the one\nthing worth doing in it, and that is what this file says - it holds no fact about the world, so it\ncannot be wrong about it.\n\nThree readings, one direction each: the wait about the person's own body, the same wait about\nsomebody they love (a different question, and a different pool), and the serious thing ruled out.\n\nA module reads these when the composer's own reply is one of the generic ones\n(bot/modules/the_wait_about_a_body.py); a card, an entry or a pool that names the condition keeps the\nturn.\n");
    s_OWN_SERIOUS_CONDITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cancers?|tumou?rs?|leuka?emia|lymphoma|myeloma|melanoma|mesothelioma|motor neurone|multiple sclerosis|huntington\\w*|parkinson\\w*|alzheim\\w*|dementia|cerebral palsy|muscular dystrophy|cirrhosis|copd|emphysema|motor\\w* disease|heart failure|kidney failure|terminal|palliative|metasta\\w+|malignant|carcinoma|\\bm\\.?s\\.?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FINDING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a\\s+)?(?:shadow|mass|growth|nodule|spot|something)\\s+(?:on|in)\\s+(?:my|the|his|her)\\s+(?:lung|lungs|breast|kidney|kidneys|liver|bowel|brain|prostate|womb|uterus|cervix|ovary|ovaries|thyroid|skin|colon|pancreas|bladder|spine|bone|bones|chest|lymph)\\b|\\b(?:they|the\\s+doctor|the\\s+doctors|the\\s+consultant|the\\s+radiologist|the\\s+scan|the\\s+mammogram|the\\s+x-?ray|the\\s+results?)\\s+(?:found|saw|spotted|noticed|picked\\s+up|showed)\\s+(?:something|a\\s+(?:shadow|mass|growth|nodule|spot|lump))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BENIGN_FINDING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<!not\\s)\\b(?:benign|harmless|was\\s+nothing|nothing\\s+(?:to\\s+worry|serious|bad|sinister)|all\\s+(?:fine|clear)|turned\\s+out\\s+(?:fine|nothing))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ITS_THE_SPEAKERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\bit'?s\\b|\\bit is\\b|\\bive\\b|\\bi've\\b|\\bi have\\b|\\bi had\\b|\\bi got\\b|\\bim\\b|\\bi'm\\b|\\bi was\\b|\\bi am\\b|\\bthey found\\b|\\btheyve found\\b|\\bthey've found\\b|\\bfound out\\b|\\b(?:my|the|their)\\s+(?:cancer|tumou?rs?|leuka?emia|lymphoma|melanoma)\\b|\\bthe (?:biopsy|scan|results?|tests?|lump|mri|x-?ray)\\b|\\bmy (?:biopsy|scan|results?|tests?|lump|mri|x-?ray)\\b|\\bgot the results\\b|\\bdiagnosed\\b|\\bdiagnosis\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_ABOUT_THEIR_BODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sponsor\\w*|fundrais\\w*|raising|raise money|awareness|charity|marathon|race for life|donat\\w*|nurses?|nursing|(?:works?|working) (?:at|in|for)|shift|documentar\\w*|film|episode|podcast|research(?:ing)?|study|statistics?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SCARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:scare|false alarm|all clear|came back clear|turned out (?:fine|nothing|benign)|it was (?:benign|nothing)|they said it was nothing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GOOD_TURN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gone|all gone|shrank|shrunk|shrink\\w*|in remission|remission|no sign of|cured|all clear|clear scan|finished (?:the )?(?:chemo|treatment|radiotherapy))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_LANDED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:not|isnt|isn't|no longer|never|clear of|free of|benign|ruled out|test\\w* for|check\\w* for|screen\\w* for|scared|worried|afraid|terrified|think|thought|might|maybe|may have|probably|hopefully|pray|praying|what if|is it|is that|or is it|could it)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WORRY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:think|thinks|thought|might|maybe|could|may\\s+have|possibly|probably|suspect\\w*|scared|afraid|terrified|worried|worry|fear|fears|hoping|praying|waiting|wait|test\\w*(?:\\s+(?:me|him|her))?\\s+for|check\\w*(?:\\s+(?:me|him|her))?\\s+for|screen\\w*\\s+for|being\\s+tested|results\\s+(?:are|is)\\s+(?:due|back)|\\b(?:is|are|was|were)\\s+(?:having|going\\s+in|in)\\s+(?:a\\s+|an\\s+|the\\s+)?(?:scan|test|biopsy|procedure|op|operation)|\\b(?:scan|scans|test|tests|biopsy|biopsies|mammogram|smear|op|operation|procedure|appointment)\\s+(?:is|are|was|were)\\s+(?:on|at|next|this|tomorrow|in)\\b|\\b(?:have|has|having|got)\\s+(?:a\\s+|an\\s+|the\\s+|my\\s+)?(?:blood\\s+|ct\\s+|mri\\s+|x[ -]?ray\\s+|smear\\s+|pregnancy\\s+)?(?:scan|test|biopsy|mammogram|smear|op|operation|procedure|surgery)\\s+(?:on|next|this|tomorrow|in|at)\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MEDICAL_TEST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:scan|scans|biopsy|biopsies|mammogram|smear|smear test|blood test|bloods|mri|ct scan|ultrasound|x[ -]?ray|xray|ecg|swab|test results|results of (?:my|the)|my results|the results|results?|op|ops|operation|operations|procedure|procedures|surgery)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_MEDICAL_TEST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:exam|exams|gcse|gcses|a level|a levels|alevel|school|uni|university|course|driving|lottery|election|football|match)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AN_ANIMAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dog|dogs|cat|cats|puppy|kitten|pet|pets|rabbit|hamster|guinea pig|horse|pony|parrot|ferret|tortoise|lizard|snake|fish)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_QUESTION_ABOUT_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:could|might|can|is|are|was|would|will|does|do)\\s+(?:it|this|that)\\s+(?:be|mean)\\b|\\bwhat\\s+if\\s+(?:it|this)\\b|\\bdo\\s+you\\s+think\\b|\\bany\\s+chance\\b|\\bwhat\\s+are\\s+the\\s+chances\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KIN(py_str("mum|mummy|mom|mommy|mother|dad|daddy|father|son|daughter|child|children|kid|kids|wife|husband|partner|boyfriend|girlfriend|brother|sister|gran|granny|grandma|grandad|grandpa|nana|nan|aunt|auntie|uncle|cousin|friend|mate|neighbour|neighbor|baby|sister|in[- ]law|niece|nephew|stepson|stepdaughter|stepdad|stepmum|fiance|fiancee"));
    s__A_RELATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?:"), &g_KIN()), &py_str(")(?:'?s)?\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:they|he|she|we|the\\s+(?:doctor|doctors|consultant|gp|specialist|hospital|scan|tests?|results?|biopsy)|doc(?:tor)?)\\b[^.!?]{0,16}?\\bruled\\s+out\\b[^.!?]{0,20}?\\b(?:cancer|tumou?rs?|malignant|carcinoma|lymphoma|leuka?emia|melanoma)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEGATED_CONDITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:it'?s|it is|its|that'?s|that is|it|that)\\s+(?:not|isnt|isn'?t|wasnt|wasn'?t|wasn'?t\\s+not)|(?:they|the doctor|the consultant|the gp|the results?|the tests?)\\s+(?:said|say|told me|came back|were|was)?\\s*(?:it\\s+)?(?:not|isnt|isn'?t|wasnt|wasn'?t))\\s+(?:a\\s+|the\\s+|any\\s+)?(?:cancer|tumou?r|malignant|carcinoma|lymphoma|leuka?emia|melanoma)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:(?:now|today|though\\s+thankfully|thankfully|thank\\s+god|at\\s+last|finally|so\\s+far|great|brilliant|and\\s+i\\s+(?:am|feel)\\s+(?:so\\s+)?(?:relieved|glad|happy)|im\\s+(?:so\\s+)?(?:relieved|glad|happy)|which\\s+is\\s+(?:great|a\\s+relief)|what\\s+a\\s+relief|thanks|cheers|anyway)\\s*)*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_WAIT(py_tuple(vec![py_str("That wait between a test and an answer is the worst part of it, and nothing is known until somebody tells you. What have they actually said so far?"), py_str("Sitting with a maybe is harder than sitting with an answer, even a bad one. Try not to spend the wait on the worst version of it. When do you hear?"), py_str("That is a frightening thing to be carrying around. Write your questions down before the appointment - they go out of your head in the room. Who is going with you?")]));
    s_THE_WAIT_AFTER_A_TEST(slice_or_raise(&g_THE_WAIT(), &Py::None, &Py::Int(2), &Py::None));
    s_THEIR_WAIT(py_tuple(vec![py_str("Waiting on somebody else's results is a helpless feeling, and nothing is known until they are back. What have they been told about when they will know?"), py_str("Sitting with a maybe about someone you love is harder than sitting with an answer. Try not to plan around the worst version of it. Who is with them?"), py_str("That is frightening, and it is a lot to carry around. What has actually been said so far - and who is looking after you in it?")]));
    s_THE_NEWS(py_tuple(vec![py_str("I'm sorry. That is a hell of a thing to be told, and there is no right way to take it. Who is with you today?"), py_str("That is the kind of news that stops the week. How are you doing with it so far?"), py_str("You don't have to make sense of it today. Do you want to talk about it, or shall I ask about something else for a while?"), py_str("That's a heavy word to hear about yourself. Who is looking after you in it?"), py_str("I'm sorry. Do you want to tell me what they said?")]));
    s_THE_ILLNESS_GOING(py_tuple(vec![py_str("That is the best kind of news to hear, and it is worth saying out loud today. How did they tell you?"), py_str("Gone is a good word to hear about it. What does that leave for you - a check in a few months, or the whole thing behind you?"), py_str("That is a weight off. Do the people who were worried with you know yet?")]));
    s_THE_THING_RULED_OUT(py_tuple(vec![py_str("Then it is not that, and that is the news you were after. What did they say it is instead?"), py_str("Ruled out. That is a weight off, whatever else came with it. How are you doing?"), py_str("Good. Not that, then. Did they say what it is?")]));
}
pub fn _the_relative(args: &[Py]) -> Py {
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_RELATIVE(), "search", args))(&__args) };
    Py::None
}
pub fn _the_wait(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The thing the line is waiting on: a condition, a finding, or a test with an answer coming.");
    found = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWN_SERIOUS_CONDITION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FINDING(), "search", args))(&__args) } } };
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_MEDICAL_TEST(), "search", args))(&__args) }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MEDICAL_TEST(), "search", args))(&__args) };
    Py::None
}
pub fn tells_of_a_condition_of_their_own(args: &[Py]) -> Py {
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
    let mut condition: Py = Py::None;
    let mut landed: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut owner: Py = Py::None;
    let _ = py_str("Whether the line tells the bot a serious condition about the person typing it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ABOUT_THEIR_BODY(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_ANIMAL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION_ABOUT_IT(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    condition = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWN_SERIOUS_CONDITION(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&condition.clone()))) {
    condition = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FINDING(), "search", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&condition.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BENIGN_FINDING(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    }
    owner = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ITS_THE_SPEAKERS(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&owner.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SCARE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GOOD_TURN(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    landed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_LANDED(), "search", args))(&__args) };
    if py_truthy(&{ let __b = landed.clone(); if !py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&landed.clone(), "start", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&condition.clone(), "start", args))(&__args) }) } }) {
    return Py::Bool(false);
    }
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relative(args))(&__args) };
    if py_truthy(&{ let __b = mine.clone(); if !py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mine.clone(), "start", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&owner.clone(), "start", args))(&__args) }) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn reads_as_the_wait_of_their_own(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is the wait before an answer about the person's own body.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ABOUT_THEIR_BODY(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_ANIMAL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION_ABOUT_IT(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SCARE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GOOD_TURN(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WORRY(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_wait(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relative(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tells_of_a_condition_of_their_own(args))(&__args) };
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tells_of_a_condition_of_their_own(args))(&__args) }));
    Py::None
}
pub fn reads_as_the_wait_of_somebody_elses(args: &[Py]) -> Py {
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
    let _ = py_str("The same wait, about a relative: \"my mum might have cancer\", \"my dads scan is on monday\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ABOUT_THEIR_BODY(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_ANIMAL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION_ABOUT_IT(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SCARE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GOOD_TURN(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WORRY(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_wait(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relative(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_a_thing_ruled_out(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line says the serious thing was ruled out - the good direction, not a wait.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION_ABOUT_IT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:is|are|was|were|did|does|do|has|have|had|can|could|would|will|should|any)\\b")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_TAIL(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEGATED_CONDITION(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_TAIL(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_the_news_of_their_own(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line tells the news itself about the person's own body: \"i have cancer\".");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tells_of_a_condition_of_their_own(args))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WORRY(), "search", args))(&__args) }));
    Py::None
}
pub fn reads_as_the_illness_going(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the line says the condition itself is over: \"the cancer is gone\", \"my tumour shrank\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ABOUT_THEIR_BODY(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWN_SERIOUS_CONDITION(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    turn = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GOOD_TURN(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&turn.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None); if py_truthy(&__b) { __b } else { py_str(" ") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_LANDED(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:is|are|was|were|will|would|could|can|has|had|have|did|does|do|any\\s+chance|do\\s+you\\s+think)\\b")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_TAIL(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn pool_for(args: &[Py]) -> Py {
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
    let _ = py_str("The pool this line belongs to, or None when it is not one of these five readings.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_thing_ruled_out(args))(&__args) }) {
    return g_THE_THING_RULED_OUT();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_the_illness_going(args))(&__args) }) {
    return g_THE_ILLNESS_GOING();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_the_news_of_their_own(args))(&__args) }) {
    return g_THE_NEWS();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_the_wait_of_somebody_elses(args))(&__args) }) {
    return g_THEIR_WAIT();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_the_wait_of_their_own(args))(&__args) }) {
    return g_THE_WAIT();
    }
    return Py::None;
    Py::None
}
pub fn names_a_condition(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line names a condition or a finding, not just a test that is booked.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWN_SERIOUS_CONDITION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FINDING(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let mut pool: Py = Py::None;
    let _ = py_str("The line for the wait or the ruled-out thing, or None when this file has nothing to say.");
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pool_for(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &g_THE_WAIT())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| names_a_condition(args))(&__args) })) } }) {
    pool = g_THE_WAIT_AFTER_A_TEST();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { memory.clone() } else { py_dict(vec![]) }); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_wait_about_a_body".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ANIMAL" => Some(g_AN_ANIMAL()),
            "A_BENIGN_FINDING" => Some(g_A_BENIGN_FINDING()),
            "A_FINDING" => Some(g_A_FINDING()),
            "A_GOOD_TURN" => Some(g_A_GOOD_TURN()),
            "A_MEDICAL_TEST" => Some(g_A_MEDICAL_TEST()),
            "A_QUESTION_ABOUT_IT" => Some(g_A_QUESTION_ABOUT_IT()),
            "A_SCARE" => Some(g_A_SCARE()),
            "A_WORRY" => Some(g_A_WORRY()),
            "ITS_THE_SPEAKERS" => Some(g_ITS_THE_SPEAKERS()),
            "KIN" => Some(g_KIN()),
            "NEGATED_CONDITION" => Some(g_NEGATED_CONDITION()),
            "NOT_ABOUT_THEIR_BODY" => Some(g_NOT_ABOUT_THEIR_BODY()),
            "NOT_A_MEDICAL_TEST" => Some(g_NOT_A_MEDICAL_TEST()),
            "NOT_LANDED" => Some(g_NOT_LANDED()),
            "OWN_SERIOUS_CONDITION" => Some(g_OWN_SERIOUS_CONDITION()),
            "RULED_OUT" => Some(g_RULED_OUT()),
            "RULED_OUT_TAIL" => Some(g_RULED_OUT_TAIL()),
            "THEIR_WAIT" => Some(g_THEIR_WAIT()),
            "THE_ILLNESS_GOING" => Some(g_THE_ILLNESS_GOING()),
            "THE_NEWS" => Some(g_THE_NEWS()),
            "THE_THING_RULED_OUT" => Some(g_THE_THING_RULED_OUT()),
            "THE_WAIT" => Some(g_THE_WAIT()),
            "THE_WAIT_AFTER_A_TEST" => Some(g_THE_WAIT_AFTER_A_TEST()),
            "_A_RELATIVE" => Some(g__A_RELATIVE()),
            "_the_relative" => Some(py_func("_the_relative", |args: &[Py]| _the_relative(args))),
            "_the_wait" => Some(py_func("_the_wait", |args: &[Py]| _the_wait(args))),
            "names_a_condition" => Some(py_func("names_a_condition", |args: &[Py]| names_a_condition(args))),
            "pool_for" => Some(py_func("pool_for", |args: &[Py]| pool_for(args))),
            "reads_as_a_thing_ruled_out" => Some(py_func("reads_as_a_thing_ruled_out", |args: &[Py]| reads_as_a_thing_ruled_out(args))),
            "reads_as_the_illness_going" => Some(py_func("reads_as_the_illness_going", |args: &[Py]| reads_as_the_illness_going(args))),
            "reads_as_the_news_of_their_own" => Some(py_func("reads_as_the_news_of_their_own", |args: &[Py]| reads_as_the_news_of_their_own(args))),
            "reads_as_the_wait_of_somebody_elses" => Some(py_func("reads_as_the_wait_of_somebody_elses", |args: &[Py]| reads_as_the_wait_of_somebody_elses(args))),
            "reads_as_the_wait_of_their_own" => Some(py_func("reads_as_the_wait_of_their_own", |args: &[Py]| reads_as_the_wait_of_their_own(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "tells_of_a_condition_of_their_own" => Some(py_func("tells_of_a_condition_of_their_own", |args: &[Py]| tells_of_a_condition_of_their_own(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_ANIMAL" => s_AN_ANIMAL(v),
            "A_BENIGN_FINDING" => s_A_BENIGN_FINDING(v),
            "A_FINDING" => s_A_FINDING(v),
            "A_GOOD_TURN" => s_A_GOOD_TURN(v),
            "A_MEDICAL_TEST" => s_A_MEDICAL_TEST(v),
            "A_QUESTION_ABOUT_IT" => s_A_QUESTION_ABOUT_IT(v),
            "A_SCARE" => s_A_SCARE(v),
            "A_WORRY" => s_A_WORRY(v),
            "ITS_THE_SPEAKERS" => s_ITS_THE_SPEAKERS(v),
            "KIN" => s_KIN(v),
            "NEGATED_CONDITION" => s_NEGATED_CONDITION(v),
            "NOT_ABOUT_THEIR_BODY" => s_NOT_ABOUT_THEIR_BODY(v),
            "NOT_A_MEDICAL_TEST" => s_NOT_A_MEDICAL_TEST(v),
            "NOT_LANDED" => s_NOT_LANDED(v),
            "OWN_SERIOUS_CONDITION" => s_OWN_SERIOUS_CONDITION(v),
            "RULED_OUT" => s_RULED_OUT(v),
            "RULED_OUT_TAIL" => s_RULED_OUT_TAIL(v),
            "THEIR_WAIT" => s_THEIR_WAIT(v),
            "THE_ILLNESS_GOING" => s_THE_ILLNESS_GOING(v),
            "THE_NEWS" => s_THE_NEWS(v),
            "THE_THING_RULED_OUT" => s_THE_THING_RULED_OUT(v),
            "THE_WAIT" => s_THE_WAIT(v),
            "THE_WAIT_AFTER_A_TEST" => s_THE_WAIT_AFTER_A_TEST(v),
            "_A_RELATIVE" => s__A_RELATIVE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_wait_about_a_body", module_obj);
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "_the_relative", py_func("_the_relative", |args: &[Py]| _the_relative(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "_the_wait", py_func("_the_wait", |args: &[Py]| _the_wait(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "tells_of_a_condition_of_their_own", py_func("tells_of_a_condition_of_their_own", |args: &[Py]| tells_of_a_condition_of_their_own(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "reads_as_the_wait_of_their_own", py_func("reads_as_the_wait_of_their_own", |args: &[Py]| reads_as_the_wait_of_their_own(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "reads_as_the_wait_of_somebody_elses", py_func("reads_as_the_wait_of_somebody_elses", |args: &[Py]| reads_as_the_wait_of_somebody_elses(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "reads_as_a_thing_ruled_out", py_func("reads_as_a_thing_ruled_out", |args: &[Py]| reads_as_a_thing_ruled_out(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "reads_as_the_news_of_their_own", py_func("reads_as_the_news_of_their_own", |args: &[Py]| reads_as_the_news_of_their_own(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "reads_as_the_illness_going", py_func("reads_as_the_illness_going", |args: &[Py]| reads_as_the_illness_going(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "pool_for", py_func("pool_for", |args: &[Py]| pool_for(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "names_a_condition", py_func("names_a_condition", |args: &[Py]| names_a_condition(args)));
    pyrt::xrefs::reg_fn("compose.a_wait_about_a_body", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "AN_ANIMAL", g_AN_ANIMAL, s_AN_ANIMAL);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_BENIGN_FINDING", g_A_BENIGN_FINDING, s_A_BENIGN_FINDING);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_FINDING", g_A_FINDING, s_A_FINDING);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_GOOD_TURN", g_A_GOOD_TURN, s_A_GOOD_TURN);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_MEDICAL_TEST", g_A_MEDICAL_TEST, s_A_MEDICAL_TEST);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_QUESTION_ABOUT_IT", g_A_QUESTION_ABOUT_IT, s_A_QUESTION_ABOUT_IT);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_SCARE", g_A_SCARE, s_A_SCARE);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "A_WORRY", g_A_WORRY, s_A_WORRY);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "ITS_THE_SPEAKERS", g_ITS_THE_SPEAKERS, s_ITS_THE_SPEAKERS);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "NEGATED_CONDITION", g_NEGATED_CONDITION, s_NEGATED_CONDITION);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "NOT_ABOUT_THEIR_BODY", g_NOT_ABOUT_THEIR_BODY, s_NOT_ABOUT_THEIR_BODY);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "NOT_A_MEDICAL_TEST", g_NOT_A_MEDICAL_TEST, s_NOT_A_MEDICAL_TEST);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "NOT_LANDED", g_NOT_LANDED, s_NOT_LANDED);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "OWN_SERIOUS_CONDITION", g_OWN_SERIOUS_CONDITION, s_OWN_SERIOUS_CONDITION);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "RULED_OUT", g_RULED_OUT, s_RULED_OUT);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "RULED_OUT_TAIL", g_RULED_OUT_TAIL, s_RULED_OUT_TAIL);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "THEIR_WAIT", g_THEIR_WAIT, s_THEIR_WAIT);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "THE_ILLNESS_GOING", g_THE_ILLNESS_GOING, s_THE_ILLNESS_GOING);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "THE_NEWS", g_THE_NEWS, s_THE_NEWS);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "THE_THING_RULED_OUT", g_THE_THING_RULED_OUT, s_THE_THING_RULED_OUT);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "THE_WAIT", g_THE_WAIT, s_THE_WAIT);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "THE_WAIT_AFTER_A_TEST", g_THE_WAIT_AFTER_A_TEST, s_THE_WAIT_AFTER_A_TEST);
    pyrt::xrefs::reg_access("compose.a_wait_about_a_body", "_A_RELATIVE", g__A_RELATIVE, s__A_RELATIVE);
}
