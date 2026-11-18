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


thread_local! { static G_BEFORE_AN_AUDIENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAVY_IN_THE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TAKEN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TAKEN_UP_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACEHOLDER_OPENINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_DOES_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_GOOD_AT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_INTO_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_LOVES_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_ROLE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_VISIT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_DOES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_GOOD_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_IS_A_ROLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_LOVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TOOK_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_THEY_ARE_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_THEY_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BEFORE_AN_AUDIENCE() -> Py { ensure_init(); G_BEFORE_AN_AUDIENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_AN_AUDIENCE(v: Py) { ensure_init(); G_BEFORE_AN_AUDIENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAVY_IN_THE_THREAD() -> Py { ensure_init(); G_HEAVY_IN_THE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAVY_IN_THE_THREAD(v: Py) { ensure_init(); G_HEAVY_IN_THE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PLURAL() -> Py { ensure_init(); G_NOT_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PLURAL(v: Py) { ensure_init(); G_NOT_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TAKEN_UP() -> Py { ensure_init(); G_NOT_TAKEN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TAKEN_UP(v: Py) { ensure_init(); G_NOT_TAKEN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TAKEN_UP_PHRASES() -> Py { ensure_init(); G_NOT_TAKEN_UP_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TAKEN_UP_PHRASES(v: Py) { ensure_init(); G_NOT_TAKEN_UP_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACEHOLDER_OPENINGS() -> Py { ensure_init(); G_PLACEHOLDER_OPENINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACEHOLDER_OPENINGS(v: Py) { ensure_init(); G_PLACEHOLDER_OPENINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_DOES_LINES() -> Py { ensure_init(); G_THEIRS_DOES_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_DOES_LINES(v: Py) { ensure_init(); G_THEIRS_DOES_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_GOOD_AT_LINES() -> Py { ensure_init(); G_THEIRS_GOOD_AT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_GOOD_AT_LINES(v: Py) { ensure_init(); G_THEIRS_GOOD_AT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_INTO_LINES() -> Py { ensure_init(); G_THEIRS_INTO_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_INTO_LINES(v: Py) { ensure_init(); G_THEIRS_INTO_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_LOVES_LINES() -> Py { ensure_init(); G_THEIRS_LOVES_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_LOVES_LINES(v: Py) { ensure_init(); G_THEIRS_LOVES_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_ROLE_LINES() -> Py { ensure_init(); G_THEIRS_ROLE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_ROLE_LINES(v: Py) { ensure_init(); G_THEIRS_ROLE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_VISIT_LINES() -> Py { ensure_init(); G_THEIRS_VISIT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_VISIT_LINES(v: Py) { ensure_init(); G_THEIRS_VISIT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_ANY() -> Py { ensure_init(); G_THEIR_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_ANY(v: Py) { ensure_init(); G_THEIR_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_DOES() -> Py { ensure_init(); G_THEIR_DOES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_DOES(v: Py) { ensure_init(); G_THEIR_DOES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_GOOD_AT() -> Py { ensure_init(); G_THEIR_GOOD_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_GOOD_AT(v: Py) { ensure_init(); G_THEIR_GOOD_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_INTO() -> Py { ensure_init(); G_THEIR_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_INTO(v: Py) { ensure_init(); G_THEIR_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_IS_A_ROLE() -> Py { ensure_init(); G_THEIR_IS_A_ROLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_IS_A_ROLE(v: Py) { ensure_init(); G_THEIR_IS_A_ROLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_KEEPS() -> Py { ensure_init(); G_THEIR_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_KEEPS(v: Py) { ensure_init(); G_THEIR_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_LOVES() -> Py { ensure_init(); G_THEIR_LOVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_LOVES(v: Py) { ensure_init(); G_THEIR_LOVES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PERSON() -> Py { ensure_init(); G_THEIR_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PERSON(v: Py) { ensure_init(); G_THEIR_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TOOK_UP() -> Py { ensure_init(); G_THEIR_TOOK_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TOOK_UP(v: Py) { ensure_init(); G_THEIR_TOOK_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_VISIT() -> Py { ensure_init(); G_THEIR_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_VISIT(v: Py) { ensure_init(); G_THEIR_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_THEY_ARE_INTO() -> Py { ensure_init(); G_WHAT_THEY_ARE_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_THEY_ARE_INTO(v: Py) { ensure_init(); G_WHAT_THEY_ARE_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_THEY_DO() -> Py { ensure_init(); G_WHAT_THEY_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_THEY_DO(v: Py) { ensure_init(); G_WHAT_THEY_DO.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What one of their people is into, does, or is coming down for: a question, not the line back.\n\nA fact about one of their people - \"my sister is into yoga\", \"my brother plays the guitar\", \"my\nmother in law is coming on sunday\" - was answered by filing it: \"Noted: your sister is into\nyoga.\" The words handed back, nothing asked. STYLE.md calls that the coldest reply there is.\n\nThis module reads the fact and asks the one thing anybody asks about a job, a hobby or a visit.\nIt answers nothing else, and it **only ever replaces a placeholder**: the composer's own reply is\nread first (bot/compose/fallback.py), and unless that reply is the receipt or the listening line\nthis module stands down. So everything the rest of the bot already says better - the store's own\njob, age and place readings, a loss, a worry, a safety line - keeps the turn.\n\nTwo things must never happen: the question must not be asked about a problem rather than a hobby\n(\"my son is into drugs\", \"my husband is into another woman\"), and it must not be asked on the\nturn after a feeling, where the fact is the reason for the feeling.\n\nIt is one module's own reading, so it can be judged on its lines alone: it proposes nothing for a\nline it does not read whole, and it passes on every line where the composer already has an\nanswer.\n");
    s_THEIR_PERSON(gen_compose::compose::remark::g_THEIR_PERSON());
    s_PRIORITY(Py::Int(300));
    s_THEIR_ANY(op_add(&op_add(&py_str("(?:"), &gen_compose::compose::remark::g_THEIR_PERSON()), &py_str("|father in law|mother in law|son in law|daughter in law|sister in law|brother in law|friend|best friend|neighbour|neighbor|mate|boss|aunty|auntie|boyfriend|girlfriend|other half|flatmate|housemate|fella|missus|stepmum|stepdad|grandson|granddaughter)")));
    s_WHAT_THEY_ARE_INTO(py_str("(?:loves|adores|likes|is obsessed with|is mad about|is nuts about)"));
    s_WHAT_THEY_DO(py_str("(?:plays|does|collects|makes|teaches|runs|swims|sings|paints|draws|bakes|cooks|gardens|reads|writes|dances)"));
    s_NOT_TAKEN_UP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("smoking"), py_str("drinking"), py_str("gambling"), py_str("drugs"), py_str("cocaine"), py_str("heroin"), py_str("betting"), py_str("vaping"), py_str("the bottle"), py_str("fighting"), py_str("biting"), py_str("wetting"), py_str("stealing"), py_str("drink"), py_str("alcohol"), py_str("weed"), py_str("pills"), py_str("porn"), py_str("scratchcards"), py_str("debt"), py_str("self-harm"), py_str("harm"), py_str("cutting"), py_str("shouting"), py_str("bottle"), py_str("cans"), py_str("booze")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_TAKEN_UP_PHRASES(py_tuple(vec![py_str("another woman"), py_str("another man"), py_str("another bloke"), py_str("another person"), py_str("another fella"), py_str("another lad"), py_str("another girl"), py_str("someone else"), py_str("somebody else"), py_str("an affair"), py_str("an ex"), py_str("his ex"), py_str("her ex"), py_str("the ex"), py_str("a row"), py_str("rows"), py_str("arguing"), py_str("arguments"), py_str("a fight"), py_str("trouble"), py_str("the police"), py_str("a gang"), py_str("crime"), py_str("money he hasnt got"), py_str("the horses")]));
    s_THEIR_INTO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? (?:is|are) (?:really |well |properly |totally |so )?into (?P<thing>[a-z][a-z' \\-]{1,22})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_LOVES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? ")), &g_WHAT_THEY_ARE_INTO()), &py_str(" (?P<thing>[a-z][a-z' \\-]{1,22})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_DOES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? ")), &g_WHAT_THEY_DO()), &py_str(" (?:in |with |for |at )?(?P<thing>[a-z][a-z' \\-]{1,22})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_TOOK_UP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? (?:has|have) (?:just |recently )?(?:taken up|got into|started) (?P<thing>[a-z][a-z' \\-]{1,22})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_KEEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? keeps (?:some |a few |two |three )?(?P<thing>[a-z]{2,12}s)$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_GOOD_AT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? (?:is|are) (?:really |very |so |quite )?good at (?P<thing>[a-z][a-z' \\-]{1,22})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_VISIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? (?:is|are) (?:coming|visiting|staying) (?P<when>(?:on |at |this |next |for |in |over )[a-z0-9' \\-]{1,24})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_IS_A_ROLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^my (?P<person>"), &g_THEIR_ANY()), &py_str(")(?:'?s)? (?:is|works as) (?:a|an) (?P<role>[a-z][a-z'\\-]{2,20})$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIRS_ROLE_LINES(py_tuple(vec![py_str("{Job}. What does a normal day look like for {them}?"), py_str("{Job}. Is that what {they} always wanted to do?"), py_str("{Job} — how long {has} {they} been at it?")]));
    s_THEIRS_INTO_LINES(py_tuple(vec![py_str("How did {they} get into that?"), py_str("Is that a new thing, or {has} {they} always been into {it}?"), py_str("Are you into {it} too, or is that {their} thing?"), py_str("What got {them} started on {it}?")]));
    s_THEIRS_LOVES_LINES(py_tuple(vec![py_str("What {do} {they} like most about {it}?"), py_str("How did that start?"), py_str("Do you get roped in too?"), py_str("Did {they} get you into {it} as well?")]));
    s_THEIRS_DOES_LINES(py_tuple(vec![py_str("How long {has} {they} been doing that?"), py_str("How often {do} {they} do it?"), py_str("Do you ever go and watch?"), py_str("What is {their} favourite bit of it?")]));
    s_THEIRS_GOOD_AT_LINES(py_tuple(vec![py_str("How long {has} {they} been at that?"), py_str("Is it a class, or something {they} {do} at home?"), py_str("Where did {they} learn that?"), py_str("Do they get much chance to show it off?")]));
    s_THEIRS_VISIT_LINES(py_tuple(vec![py_str("{is} {they} staying with you, or nearby?"), py_str("Is it a flying visit, or a long one?"), py_str("Are you looking forward to it?"), py_str("How much of the week is that taking up?")]));
    s_NOT_PLURAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("chess"), py_str("darts"), py_str("news"), py_str("gymnastics"), py_str("athletics"), py_str("bowls"), py_str("skittles"), py_str("billiards"), py_str("physics"), py_str("maths"), py_str("mathematics"), py_str("business"), py_str("class"), py_str("glass"), py_str("bass"), py_str("gas"), py_str("bus"), py_str("christmas"), py_str("tennis"), py_str("series"), py_str("species"), py_str("means"), py_str("this"), py_str("his"), py_str("hers"), py_str("its")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BEFORE_AN_AUDIENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("football"), py_str("rugby"), py_str("cricket"), py_str("tennis"), py_str("netball"), py_str("hockey"), py_str("athletics"), py_str("racing"), py_str("races"), py_str("matches"), py_str("dancing"), py_str("ballet"), py_str("singing"), py_str("choir"), py_str("gigs"), py_str("bands"), py_str("drama"), py_str("theatre"), py_str("plays"), py_str("stand up"), py_str("swimming"), py_str("speedway"), py_str("boxing"), py_str("darts"), py_str("golf")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLACEHOLDER_OPENINGS(py_tuple(vec![py_str("noted"), py_str("i'm listening"), py_str("i am listening"), py_str("got it"), py_str("ok."), py_str("that's noted"), py_str("thanks for telling me"), py_str("i don't know enough"), py_str("i dont know enough"), py_str("that's beyond what i know"), py_str("thats beyond what i know"), py_str("i'm not sure about that one"), py_str("im not sure about that one")]));
    s_HEAVY_IN_THE_THREAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bull(?:y|ied|ying)|bullies|ill|unwell|poorly|poorlier|sick|sicker|sickness|diagnos\\w*|cancer|tumou?r|chemo\\w*|radiotherap\\w*|dementia|alzheim\\w*|parkinson\\w*|stroke|sepsis|pneumonia|surgery|operation|transplant|dialysis|hospice|palliative|terminal|sectioned|sectioning|depress\\w*|anxiety|anorexi\\w*|bulimi\\w*|addict\\w*|relapse\\w*|overdose\\w*|self[- ]harm|suicidal|miscarr\\w*|stillborn|funeral|burial|cremat\\w*|griev\\w*|bereave\\w*|died|dying|death|passed away|lost (?:him|her|them|my)|hospital|icu|a&e|ambulance|in pain|suffering|struggling|carer|carers|dementia)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _starts_a_sentence(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The pool line with its first letter capitalised, since it opens the reply.\n\n    One line begins with the verb in front of the pronoun - \"{is} {they} staying with you\" - and\n    \"is she staying\" is not how a sentence starts.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return if py_truthy(&said.clone()) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { said.clone() };
    Py::None
}
pub fn _the_turn_before_was_a_feeling(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut read: Py = Py::None;
    let _ = py_str("True when the last line they said was a feeling, so this fact is the reason for it.\n\n    The reading is the store's own (bot/memory/ack.py): a fact added the turn after a feeling is\n    the explanation of that feeling, and it keeps the reply the store has for it.\n    ");
    let mut ack: Py = gen_memory::memory::ack::module_obj();
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ack.clone()); __args.push(py_str("_after_a_feeling")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = read.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&read, args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_thing_nobody_is_into(args: &[Py]) -> Py {
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
    let mut parts: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for the things that are a problem rather than a hobby (\"is into drugs\").\n\n    A phrase is read first, because the words in \"another woman\" and \"trouble at school\" are\n    ordinary words that mean nothing on their own.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_TAKEN_UP_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z\\-]+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_TAKEN_UP(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_plural_thing(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing named is plural, so the reply says \"them\" and not \"it\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z\\-]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    last = geti(&words.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_PLURAL(), &last.clone())); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return Py::Bool(false);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is"), py_str("ous")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "endswith", args))(&__args) })) } };
    Py::None
}
pub fn _it_word(args: &[Py]) -> Py {
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
    let _ = py_str("The word the pool line uses for the thing: \"them\" for dinosaurs, \"it\" for baking.");
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_plural_thing(args))(&__args) }) { py_str("them") } else { py_str("it") };
    Py::None
}
pub fn _watched(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing named is one somebody watches as well as does.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BEFORE_AN_AUDIENCE(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&g_BEFORE_AN_AUDIENCE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }));
    Py::None
}
pub fn _does_pool(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The pool for a thing they do, without the audience question for a thing done alone.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _watched(args))(&__args) }) {
    return g_THEIRS_DOES_LINES();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THEIRS_DOES_LINES()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("watch")))) {  __out.push(line.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _fill_it(args: &[Py]) -> Py {
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
    let mut person_word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person_word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person_word"))) };
    let mut it_word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "it_word") { v.clone() }
    else { py_str("it") };
    let mut are: Py = Py::None;
    let mut do_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut their: Py = Py::None;
    let mut them: Py = Py::None;
    let mut they: Py = Py::None;
    let _ = py_str("The pool line with the person's pronouns, the verb that agrees, and the thing's number.\n\n    {they} and the rest of the pronouns are the store's own fill_person (bot/compose/remark.py);\n    {has}, {is} and {it} are this pool's, because the verb in front of the pronoun and the number\n    of the thing are facts about the line rather than about the person. A line that opens on one\n    of them is capitalised once it is filled.\n    ");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(person_word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::remark::person_pronouns(args))(&__args) }, 5, None);
    they = __unpk1[0].clone();
    them = __unpk1[1].clone();
    their = __unpk1[2].clone();
    do_ = __unpk1[3].clone();
    are = __unpk1[4].clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(person_word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::remark::fill_person(args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{it}")); __args.push(it_word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{is}")); __args.push(if py_truthy(&Py::Bool(py_eq(&are.clone(), &py_str("are")))) { py_str("are") } else { py_str("is") }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{has}")); __args.push(if py_truthy(&Py::Bool(py_eq(&do_.clone(), &py_str("do")))) { py_str("have") } else { py_str("has") }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Does they")); __args.push(py_str("Do they")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("does they")); __args.push(py_str("do they")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Is they")); __args.push(py_str("Are they")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("is they")); __args.push(py_str("are they")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Has they")); __args.push(py_str("Have they")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("has they")); __args.push(py_str("have they")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) };
    return if py_truthy(&said.clone()) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { said.clone() };
    Py::None
}
pub fn reading(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut job: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The pool line for a whole-line fact about one of their people, or None.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_before_was_a_feeling(args))(&__args) }) {
    return Py::None;
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_IS_A_ROLE(), "match", args))(&__args) };
    if py_truthy(&match_.clone()) {
    job = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("role")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::remark::_the_job_in(args))(&__args) };
    if py_truthy(&job.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Job}")); __args.push(job.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_ROLE_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }, "replace", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_INTO(), "match", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_nobody_is_into(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_INTO_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _it_word(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_LOVES(), "match", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_nobody_is_into(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_LOVES_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _it_word(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_DOES(), "match", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_nobody_is_into(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _does_pool(args))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_GOOD_AT(), "match", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_nobody_is_into(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_GOOD_AT_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_KEEPS(), "match", args))(&__args) };
    if py_truthy(&match_.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_DOES_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_TOOK_UP(), "match", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_nobody_is_into(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _does_pool(args))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_VISIT(), "match", args))(&__args) };
    if py_truthy(&match_.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_VISIT_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill_it(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _is_a_placeholder(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the composer's own reply for this line is one of the placeholder shapes.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLACEHOLDER_OPENINGS()) { let mut opening: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opening.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "startswith", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_thread_is_heavy(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the line just before this one was a person ill, lost, bullied or in hospital.");
    before = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gen_compose::compose::fallback::module_obj()); __args.push(py_str("_the_line_before")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("hasattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::fallback::_the_line_before(args))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    before = if py_truthy(&turns.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::fallback::_reads_as_heavy_news(args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HEAVY_IN_THE_THREAD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let mut clean: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("The turn for a fact about one of their people, when nothing better answers it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9' ]+")); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+")); __args.push(py_str(" ")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("my ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "startswith", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thread_is_heavy(args))(&__args) }) {
    return Py::None;
    }
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reading(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    theirs = Py::None;
    let __try1 = exc::catch(|| {
    theirs = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_compose::compose::fallback::g_reply_for(), args))(&__args) };
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
    theirs = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(theirs.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_placeholder(args))(&__args) })) } }) {
    return Py::None;
    }
    return mine.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.their_pastime".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BEFORE_AN_AUDIENCE" => Some(g_BEFORE_AN_AUDIENCE()),
            "HEAVY_IN_THE_THREAD" => Some(g_HEAVY_IN_THE_THREAD()),
            "NOT_PLURAL" => Some(g_NOT_PLURAL()),
            "NOT_TAKEN_UP" => Some(g_NOT_TAKEN_UP()),
            "NOT_TAKEN_UP_PHRASES" => Some(g_NOT_TAKEN_UP_PHRASES()),
            "PLACEHOLDER_OPENINGS" => Some(g_PLACEHOLDER_OPENINGS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "THEIRS_DOES_LINES" => Some(g_THEIRS_DOES_LINES()),
            "THEIRS_GOOD_AT_LINES" => Some(g_THEIRS_GOOD_AT_LINES()),
            "THEIRS_INTO_LINES" => Some(g_THEIRS_INTO_LINES()),
            "THEIRS_LOVES_LINES" => Some(g_THEIRS_LOVES_LINES()),
            "THEIRS_ROLE_LINES" => Some(g_THEIRS_ROLE_LINES()),
            "THEIRS_VISIT_LINES" => Some(g_THEIRS_VISIT_LINES()),
            "THEIR_ANY" => Some(g_THEIR_ANY()),
            "THEIR_DOES" => Some(g_THEIR_DOES()),
            "THEIR_GOOD_AT" => Some(g_THEIR_GOOD_AT()),
            "THEIR_INTO" => Some(g_THEIR_INTO()),
            "THEIR_IS_A_ROLE" => Some(g_THEIR_IS_A_ROLE()),
            "THEIR_KEEPS" => Some(g_THEIR_KEEPS()),
            "THEIR_LOVES" => Some(g_THEIR_LOVES()),
            "THEIR_PERSON" => Some(g_THEIR_PERSON()),
            "THEIR_TOOK_UP" => Some(g_THEIR_TOOK_UP()),
            "THEIR_VISIT" => Some(g_THEIR_VISIT()),
            "WHAT_THEY_ARE_INTO" => Some(g_WHAT_THEY_ARE_INTO()),
            "WHAT_THEY_DO" => Some(g_WHAT_THEY_DO()),
            "_a_plural_thing" => Some(py_func("_a_plural_thing", |args: &[Py]| _a_plural_thing(args))),
            "_a_thing_nobody_is_into" => Some(py_func("_a_thing_nobody_is_into", |args: &[Py]| _a_thing_nobody_is_into(args))),
            "_does_pool" => Some(py_func("_does_pool", |args: &[Py]| _does_pool(args))),
            "_fill_it" => Some(py_func("_fill_it", |args: &[Py]| _fill_it(args))),
            "_is_a_placeholder" => Some(py_func("_is_a_placeholder", |args: &[Py]| _is_a_placeholder(args))),
            "_it_word" => Some(py_func("_it_word", |args: &[Py]| _it_word(args))),
            "_starts_a_sentence" => Some(py_func("_starts_a_sentence", |args: &[Py]| _starts_a_sentence(args))),
            "_the_thread_is_heavy" => Some(py_func("_the_thread_is_heavy", |args: &[Py]| _the_thread_is_heavy(args))),
            "_the_turn_before_was_a_feeling" => Some(py_func("_the_turn_before_was_a_feeling", |args: &[Py]| _the_turn_before_was_a_feeling(args))),
            "_watched" => Some(py_func("_watched", |args: &[Py]| _watched(args))),
            "reading" => Some(py_func("reading", |args: &[Py]| reading(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BEFORE_AN_AUDIENCE" => s_BEFORE_AN_AUDIENCE(v),
            "HEAVY_IN_THE_THREAD" => s_HEAVY_IN_THE_THREAD(v),
            "NOT_PLURAL" => s_NOT_PLURAL(v),
            "NOT_TAKEN_UP" => s_NOT_TAKEN_UP(v),
            "NOT_TAKEN_UP_PHRASES" => s_NOT_TAKEN_UP_PHRASES(v),
            "PLACEHOLDER_OPENINGS" => s_PLACEHOLDER_OPENINGS(v),
            "PRIORITY" => s_PRIORITY(v),
            "THEIRS_DOES_LINES" => s_THEIRS_DOES_LINES(v),
            "THEIRS_GOOD_AT_LINES" => s_THEIRS_GOOD_AT_LINES(v),
            "THEIRS_INTO_LINES" => s_THEIRS_INTO_LINES(v),
            "THEIRS_LOVES_LINES" => s_THEIRS_LOVES_LINES(v),
            "THEIRS_ROLE_LINES" => s_THEIRS_ROLE_LINES(v),
            "THEIRS_VISIT_LINES" => s_THEIRS_VISIT_LINES(v),
            "THEIR_ANY" => s_THEIR_ANY(v),
            "THEIR_DOES" => s_THEIR_DOES(v),
            "THEIR_GOOD_AT" => s_THEIR_GOOD_AT(v),
            "THEIR_INTO" => s_THEIR_INTO(v),
            "THEIR_IS_A_ROLE" => s_THEIR_IS_A_ROLE(v),
            "THEIR_KEEPS" => s_THEIR_KEEPS(v),
            "THEIR_LOVES" => s_THEIR_LOVES(v),
            "THEIR_PERSON" => s_THEIR_PERSON(v),
            "THEIR_TOOK_UP" => s_THEIR_TOOK_UP(v),
            "THEIR_VISIT" => s_THEIR_VISIT(v),
            "WHAT_THEY_ARE_INTO" => s_WHAT_THEY_ARE_INTO(v),
            "WHAT_THEY_DO" => s_WHAT_THEY_DO(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.their_pastime", module_obj);
    pyrt::xrefs::reg_fn("modules.their_pastime", "_starts_a_sentence", py_func("_starts_a_sentence", |args: &[Py]| _starts_a_sentence(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_the_turn_before_was_a_feeling", py_func("_the_turn_before_was_a_feeling", |args: &[Py]| _the_turn_before_was_a_feeling(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_a_thing_nobody_is_into", py_func("_a_thing_nobody_is_into", |args: &[Py]| _a_thing_nobody_is_into(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_a_plural_thing", py_func("_a_plural_thing", |args: &[Py]| _a_plural_thing(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_it_word", py_func("_it_word", |args: &[Py]| _it_word(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_watched", py_func("_watched", |args: &[Py]| _watched(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_does_pool", py_func("_does_pool", |args: &[Py]| _does_pool(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_fill_it", py_func("_fill_it", |args: &[Py]| _fill_it(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "reading", py_func("reading", |args: &[Py]| reading(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_is_a_placeholder", py_func("_is_a_placeholder", |args: &[Py]| _is_a_placeholder(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "_the_thread_is_heavy", py_func("_the_thread_is_heavy", |args: &[Py]| _the_thread_is_heavy(args)));
    pyrt::xrefs::reg_fn("modules.their_pastime", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.their_pastime", "BEFORE_AN_AUDIENCE", g_BEFORE_AN_AUDIENCE, s_BEFORE_AN_AUDIENCE);
    pyrt::xrefs::reg_access("modules.their_pastime", "HEAVY_IN_THE_THREAD", g_HEAVY_IN_THE_THREAD, s_HEAVY_IN_THE_THREAD);
    pyrt::xrefs::reg_access("modules.their_pastime", "NOT_PLURAL", g_NOT_PLURAL, s_NOT_PLURAL);
    pyrt::xrefs::reg_access("modules.their_pastime", "NOT_TAKEN_UP", g_NOT_TAKEN_UP, s_NOT_TAKEN_UP);
    pyrt::xrefs::reg_access("modules.their_pastime", "NOT_TAKEN_UP_PHRASES", g_NOT_TAKEN_UP_PHRASES, s_NOT_TAKEN_UP_PHRASES);
    pyrt::xrefs::reg_access("modules.their_pastime", "PLACEHOLDER_OPENINGS", g_PLACEHOLDER_OPENINGS, s_PLACEHOLDER_OPENINGS);
    pyrt::xrefs::reg_access("modules.their_pastime", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIRS_DOES_LINES", g_THEIRS_DOES_LINES, s_THEIRS_DOES_LINES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIRS_GOOD_AT_LINES", g_THEIRS_GOOD_AT_LINES, s_THEIRS_GOOD_AT_LINES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIRS_INTO_LINES", g_THEIRS_INTO_LINES, s_THEIRS_INTO_LINES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIRS_LOVES_LINES", g_THEIRS_LOVES_LINES, s_THEIRS_LOVES_LINES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIRS_ROLE_LINES", g_THEIRS_ROLE_LINES, s_THEIRS_ROLE_LINES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIRS_VISIT_LINES", g_THEIRS_VISIT_LINES, s_THEIRS_VISIT_LINES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_ANY", g_THEIR_ANY, s_THEIR_ANY);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_DOES", g_THEIR_DOES, s_THEIR_DOES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_GOOD_AT", g_THEIR_GOOD_AT, s_THEIR_GOOD_AT);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_INTO", g_THEIR_INTO, s_THEIR_INTO);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_IS_A_ROLE", g_THEIR_IS_A_ROLE, s_THEIR_IS_A_ROLE);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_KEEPS", g_THEIR_KEEPS, s_THEIR_KEEPS);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_LOVES", g_THEIR_LOVES, s_THEIR_LOVES);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_PERSON", g_THEIR_PERSON, s_THEIR_PERSON);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_TOOK_UP", g_THEIR_TOOK_UP, s_THEIR_TOOK_UP);
    pyrt::xrefs::reg_access("modules.their_pastime", "THEIR_VISIT", g_THEIR_VISIT, s_THEIR_VISIT);
    pyrt::xrefs::reg_access("modules.their_pastime", "WHAT_THEY_ARE_INTO", g_WHAT_THEY_ARE_INTO, s_WHAT_THEY_ARE_INTO);
    pyrt::xrefs::reg_access("modules.their_pastime", "WHAT_THEY_DO", g_WHAT_THEY_DO, s_WHAT_THEY_DO);
}
