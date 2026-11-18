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


thread_local! { static G_ALREADY_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_A_ROUTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANY_ABSTRACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ASKING_MY_OPINION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAD_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_ON_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_FOR_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECOMMEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_TO_HELP_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CASH_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARBY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MAP_UNNAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTE_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THERE_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_TO_GET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROUTE_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALREADY_A_PLAN() -> Py { ensure_init(); G_ALREADY_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_A_PLAN(v: Py) { ensure_init(); G_ALREADY_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_A_ROUTE() -> Py { ensure_init(); G_ALREADY_A_ROUTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_A_ROUTE(v: Py) { ensure_init(); G_ALREADY_A_ROUTE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_ABSTRACT() -> Py { ensure_init(); G_ANY_ABSTRACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_ABSTRACT(v: Py) { ensure_init(); G_ANY_ABSTRACT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ASKING_MY_OPINION() -> Py { ensure_init(); G_A_ASKING_MY_OPINION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ASKING_MY_OPINION(v: Py) { ensure_init(); G_A_ASKING_MY_OPINION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAD_IT() -> Py { ensure_init(); G_A_HAD_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAD_IT(v: Py) { ensure_init(); G_A_HAD_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAND() -> Py { ensure_init(); G_A_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAND(v: Py) { ensure_init(); G_A_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_ON_A_THING() -> Py { ensure_init(); G_A_JOB_ON_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_ON_A_THING(v: Py) { ensure_init(); G_A_JOB_ON_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEAD() -> Py { ensure_init(); G_A_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEAD(v: Py) { ensure_init(); G_A_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_FOR_SOMEBODY() -> Py { ensure_init(); G_A_NAME_FOR_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_FOR_SOMEBODY(v: Py) { ensure_init(); G_A_NAME_FOR_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOD() -> Py { ensure_init(); G_A_NOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOD(v: Py) { ensure_init(); G_A_NOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECOMMEND() -> Py { ensure_init(); G_A_RECOMMEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECOMMEND(v: Py) { ensure_init(); G_A_RECOMMEND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TAIL() -> Py { ensure_init(); G_A_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TAIL(v: Py) { ensure_init(); G_A_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_TO_HELP_WITH() -> Py { ensure_init(); G_A_THING_TO_HELP_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_TO_HELP_WITH(v: Py) { ensure_init(); G_A_THING_TO_HELP_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRADE() -> Py { ensure_init(); G_A_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRADE(v: Py) { ensure_init(); G_A_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ANY() -> Py { ensure_init(); G_BARE_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ANY(v: Py) { ensure_init(); G_BARE_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_CASH_MACHINE() -> Py { ensure_init(); G_CASH_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CASH_MACHINE(v: Py) { ensure_init(); G_CASH_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOURS() -> Py { ensure_init(); G_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOURS(v: Py) { ensure_init(); G_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARBY() -> Py { ensure_init(); G_NEARBY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARBY(v: Py) { ensure_init(); G_NEARBY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MAP() -> Py { ensure_init(); G_NO_MAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MAP(v: Py) { ensure_init(); G_NO_MAP.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MAP_UNNAMED() -> Py { ensure_init(); G_NO_MAP_UNNAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MAP_UNNAMED(v: Py) { ensure_init(); G_NO_MAP_UNNAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_THING() -> Py { ensure_init(); G_NO_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_THING(v: Py) { ensure_init(); G_NO_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTE_POOLS() -> Py { ensure_init(); G_ROUTE_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTE_POOLS(v: Py) { ensure_init(); G_ROUTE_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEBODY() -> Py { ensure_init(); G_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY(v: Py) { ensure_init(); G_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_THERE_IS() -> Py { ensure_init(); G_THERE_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THERE_IS(v: Py) { ensure_init(); G_THERE_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HOURS() -> Py { ensure_init(); G_THE_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HOURS(v: Py) { ensure_init(); G_THE_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_TO_GET() -> Py { ensure_init(); G_WHERE_TO_GET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_GET(v: Py) { ensure_init(); G_WHERE_TO_GET.with(|c| *c.borrow_mut() = v); }
pub fn g__ROUTE_HEADS() -> Py { ensure_init(); G__ROUTE_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROUTE_HEADS(v: Py) { ensure_init(); G__ROUTE_HEADS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Where to get a thing, or somebody to do a job, when the bot has no map and no branch list.\n\nPeople ask this constantly - \"where do i get seed from\", \"is there a launderette near me\", \"where\ncan i get my bike fixed\", \"is there a pharmacy open near me on sunday\" - and every one of them\nreached either the search hedge (\"A quick search online will get you a solid answer\") or, worse,\nthe composer's flat nod:\n\n    where do i get seed from                    -> I'm not sure about that one... A quick search\n    is there a launderette near me              -> I don't know enough to answer that honestly\n    any decent chippy near me                   -> Thanks for telling me - I'm here if you want\n                                                   to dig into any of that\n    where do i get the part                     (after \"its a mixer tap\") -> I don't know enough\n\nA search is not wrong here, but it is thin: it does not say *what* the bot cannot see, and it does\nnot name the one move that does work. What is true on this side is small and worth saying: there\nis no map, no list of what is near them, no live opening times and no recommendations - and the\nmove that works is a map app with their own town in it, or two neighbours, and a phone call before\nthey go out of their way.\n\nSo this reader says the true part, names the move, and keeps it short. It is a fallback and it is\nwritten to be one: it only replaces a hedge or the composer's nod (`NEUTRAL`, `A_SMALL_DAY`), and\nany entry, any card, or the composer's own line keeps the turn.\n\nThree kinds, because the answer is different for each:\n\n    a thing to buy        - \"where do i get seed from\", \"is there a launderette near me\"\n                            a map app with the thing and their town; the branch page for stock\n    a person for a job    - \"where can i get my bike fixed\", \"where do i find someone to service\n                            it\" - neighbours and a local group, then a trade body's register\n                            (Gas Safe for gas) and the price in writing\n    one with hours        - \"is there a pharmacy open near me on sunday\" - a map app's \"open now\",\n                            and the NHS pharmacy finder for a pharmacy\n\nWhat it never does is invent a shop, a name, a price or a time. It reads the thing out of the\nperson's own line (\"a christmas tree\", \"seed\", \"dry cleaning\") so the reply names what they asked\nabout instead of \"it\".\n");
    s_WHERE_TO_GET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhere (?:can|could|do|would|should|might) (?:i|we) (?:get|buy|find|hire|pick up|source)\\b|\\bwhere(?:'s| is| are)? (?:the )?(?:best|nearest|closest)\\b|\\bwhere (?:can|could) (?:i|we) (?:get|find) (?:somebody|someone)\\b|\\bwho (?:does|sells|fixes|repairs|makes|can) (?:this|that|it|them)?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THERE_IS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are) there (?:a|an|any|anywhere|somewhere|any good|any decent|a good|a decent)\\b|\\bany (?:good|decent|half decent|nice)\\b|\\banywhere (?:i can|that|round here|near|to get|to buy)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEARBY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:near me|near here|near us|nearby|round here|around here|round the corner|close by|locally|in the area|in town|near york|near my|nearest)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_RECOMMEND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:can|could|would) (?:you|anybody|anyone) (?:recommend|suggest)\\b|\\b(?:do|did) you know (?:a|an|any|anyone|anybody|someone|somebody|of a|of any)\\b|\\bknow (?:a|an|any) (?:good|decent|reliable|half decent|trustworthy)\\b|\\banyone know (?:a|an|any)\\b|\\bwho (?:would|could) you (?:recommend|suggest)\\b|\\b(?:recommend|recommendation|recommendations) (?:for|of) (?:a|an|someone|somebody)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_ASKING_MY_OPINION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:recommend|suggest) (?:i|we|me|that i|that we)\\b|\\bwould you (?:recommend|suggest) (?:seeing|going|trying|taking|using|booking|getting)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_NAME_FOR_SOMEBODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:plumber|electrician|builder|joiner|carpenter|roofer|roofing|gardener|landscaper|decorator|painter|mechanic|garage|locksmith|glazier|chimney sweep|pest control|removals|handyman|odd job|surveyor|architect|cleaner|window cleaner|solicitor|lawyer|conveyancer|accountant|bookkeeper|dentist|orthodontist|hygienist|hairdresser|barber|vet|optician|physio|physiotherapist|osteopath|chiropractor|chiropodist|podiatrist|tutor|childminder|dog walker|dog groomer|nanny|carer|interpreter|translator|tailor|seamstress|upholsterer|tiler|plasterer|drainage|gas engineer|boiler engineer|heating engineer)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOURS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bopen\\b|\\bopening\\b|\\bclosed\\b|\\bshut\\b|\\blate\\b|\\bsunday\\b|\\bsaturday\\b|\\btonight\\b|\\bnow\\b|\\btoday\\b|\\bbank holiday\\b|\\bweekend\\b|24 hour|24-hour")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEBODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:someone|somebody|a man|a person|anyone|who can|who could)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TRADE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:plumber|electrician|builder|joiner|carpenter|roofer|roofing|gardener|landscaper|decorator|painter|mechanic|garage|locksmith|glazier|chimney sweep|pest control|removals|man with a van|gas engineer|boiler engineer|heating engineer|tiler|plasterer|drainage|drain man|odd job|handyman|surveyor|architect|cleaner|window cleaner)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_JOB_ON_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fix|fixes|fixed|repair|repairs|repaired|mend|mends|mended|service|services|serviced|install|installs|installed|fit|fits|fitted|replace|replaces|replaced|unblock|unblocks|unblocked|bleed|bleeds|bleeding|rewire|plaster|tile|decorate|assemble|put up|take down|cut back|trim|fell|haul|clear out|check over|mot|valet)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the|some|any|my|our|his|her|their)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:from|from a|near me|near here|nearby|round here|around here|round the corner|close by|locally|in the area|in town|please|then|today|now|cheaply|cheap|asap|at all|if possible)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("it"), py_str("its"), py_str("it's"), py_str("one"), py_str("ones"), py_str("them"), py_str("that"), py_str("this"), py_str("those"), py_str("these"), py_str("the part"), py_str("the bit"), py_str("the thing"), py_str("the other one"), py_str("the right one"), py_str("the same one"), py_str("a new one"), py_str("the rest"), py_str("some"), py_str("more"), py_str("the stuff"), py_str("stuff"), py_str("something"), py_str("anything"), py_str("part"), py_str("bit"), py_str("piece"), py_str("number"), py_str("spare"), py_str("the spare"), py_str("new one"), py_str("the new one"), py_str("a replacement")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:setting|settings|option|menu|button|box|tick|toggle|app|website|web ?site|site|internet|online|account|profile|password|page|link|file|folder|form|email|e-mail|number|answer|word|phrase|name of|symbol|icon|tab|code|pin|receipt|invoice|statement|advice|help|support|information|info|idea|ideas|inspiration|motivation|energy|time|money|cash|courage|willpower|confidence|strength|protein|fibre|fiber|iron|calcium|vitamin|vitamins|carbs|calories|fat|nutrient|nutrients|hydration|sleep|results|result|prescription|referral|appointment|test|tests|tested|screening|screened|vaccinated|jab|jabbed|vaccine|vaccination|voucher|gift card|refund|compensation|complaint|report|permission|licence|license|visa|change|connection|platform|route|directions|ticket|bus|buses|train|trains|tube|taxi|tram|coach|get off|get on|get out|stop)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_ANY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:any|anywhere|somewhere)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANY_ABSTRACT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:advice|help|news|luck|update|updates|chance|word|ideas?|points?|sign|signs|problems?|trouble|issues?|questions?|objection|preference|reasons?|differences?|changes?|time|space|room|evidence|proof|complaints?|feedback|progress|better|worse|easier|harder|nicer|cheaper|different|new|same|news|warning|warnings)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CASH_MACHINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cash machine|cash point|cashpoint|cashline|cash dispenser|atm|hole in the wall)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ALREADY_A_PLAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'll|i will|im going|i'm going|we're going|were going|the council already)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_NOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&crate::compose::fallback_data::g_NEUTRAL(), &crate::compose::fallback_data::g_A_SMALL_DAY())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ALREADY_A_ROUTE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:own finder|own register|official (?:finder|register|list)|profession'?s own|professional body|trade body|lists? them by area|find a (?:solicitor|lawyer|vet|dentist|physio|accountant|plumber|electrician)|taking new (?:people|patients|clients))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_HAD_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:has|have|had|who'?s|whos|that'?s|whose)\\s+(?:also\\s+)?had\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_TO_HELP_WITH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:van|trailer|ladder|ladders|generator|marquee|tables? and chairs|chairs|skip|pressure washer|trolley)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ROUTE_POOLS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("BUY_POOL"), py_str("TRADE_POOL"), py_str("PROFESSIONAL_POOL"), py_str("PLACE_ROUTE_POOL")])) { let mut name: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::finding_out::module_obj()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("hasattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::finding_out::module_obj()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ROUTE_HEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ROUTE_POOLS()) { let mut pool: Py = __item.clone(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, &Py::None, &Py::Int(34), &Py::None)); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_MAP(py_tuple(vec![py_str("No map on this side, so I can't see what's near you. \"{thing} near me\" in a map app lists the closest with their hours, and the branch's own page says whether they keep it."), py_str("I can't see what's near you, so a pick from me would be a guess. Put \"{thing}\" into a map app with your town in the search, then ring before you go if it's a bus ride away."), py_str("There's no list of places here. A map app search for \"{thing}\" and your town shows the nearest ones and what time they shut, which is the part I can't tell you.")]));
    s_NO_MAP_UNNAMED(py_tuple(vec![py_str("No map on this side, so I can't point you at anywhere near you. A map app with the name of the thing and your town is the quickest way, and a phone call settles whether they keep it."), py_str("I can't see what's near you, so any shop I named would be a guess. Search it in a map app with your town added, and ring first: that saves the trip if they haven't got it.")]));
    s_A_HAND(py_tuple(vec![py_str("I can't recommend anyone - I've no way to see who's near you or how their work turns out. Two neighbours who've had the same job done is the best source for a name, then check it on the trade's register (Gas Safe for anything gas) before you book."), py_str("Nobody I can name for that, and I won't guess with somebody's livelihood. Ask on a local group or two neighbours, check the name on the trade body's list, and get the price in writing before they start.")]));
    s_THE_HOURS(py_tuple(vec![py_str("No live opening times here. A map app will show today's hours for the ones near you, and for a pharmacy the NHS pharmacy finder lists which are open."), py_str("I can't see what's open near you today. A map app with \"open now\" in the search is the quick way, and the branch's own page is the one that's kept right.")]));
}
pub fn already_a_route(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut head: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a reply already names the finder, the register or the body that keeps it.\n\n    That covers `finding_out`'s own routes as well as the finder lines other readers add, because\n    the point of this reader is to answer where nothing does, not to reword a route already given.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ALREADY_A_ROUTE(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ROUTE_HEADS()) { let mut head: Py = __item.clone(); __out.push({ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "startswith", args))(&__args) } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    Py::None
}
pub fn _a_thing(args: &[Py]) -> Py {
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
    let mut trimmed: Py = Py::None;
    let _ = py_str("The thing trimmed of the ask and the place around it, or None when it names nothing.");
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    while py_truthy(&Py::Bool(true)) {
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TAIL(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &thing.clone()))) {
    break;
    }
    thing = trimmed.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NO_THING(), &thing.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } } } }) {
    return Py::None;
    }
    return thing.clone();
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut found: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the line asks for, in the person's own words, or None when it names none.\n\n    Three read-backs, because people ask three ways: \"where do i get a christmas tree\", \"is there\n    a launderette near me\", \"anywhere round here that does dry cleaning\". The thing is taken as\n    they said it so the reply can hand them a search they can type.\n    ");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:that|which|who) (?:does|sells|fixes|repairs|makes|keeps|stocks)\\s+([a-z][a-z' ]{1,28})$")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&found.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) };
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:get|buy|find|hire|pick up|source)\\s+(?:hold of\\s+)?(?:a|an|the|some|any|my|our)?\\s*([a-z][a-z' ]{1,28}?)(?:\\s+(?:from|near|round|around|close|locally|in|please|then|today|now|cheaply|cheap|asap)$|\\s*$)")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&found.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) };
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:any|some)\\s+([a-z][a-z' ]{1,28}?)(?:\\s+(?:near|round|around|open|in|that|to|please|today|now|locally)\\b|$)")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&found.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) };
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:nearest|closest)\\s+([a-z][a-z' ]{1,28}?)(?:\\s+(?:near|to|for|in|that|please|today|now|open)\\b|$)")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&found.clone()) {
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) };
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("place"), py_str("one"), py_str("spot")]), &thing.clone())) } }) {
    return thing.clone();
    }
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are) there (?:a|an|any|somewhere|anywhere|any good|a decent)\\s+([a-z][a-z' ]{1,28}?)(?:\\s+(?:near|round|around|close|open|locally|in|that|to)\\b|$)")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&found.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _a_person_for_the_job(args: &[Py]) -> Py {
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
    let _ = py_str("True when the person is asking for somebody to do the work rather than a shop.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEBODY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TRADE(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_JOB_ON_A_THING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|the)\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut asking_for_a_name: Py = Py::None;
    let mut asking_if_there_is: Py = Py::None;
    let mut asking_where: Py = Py::None;
    let mut bare_any: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut for_the_place_test: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut who_does: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A reply for a where-to-get-it ask, or None.\n\n    Only a place ask reaches here: a shape (\"where can i get\", \"is there a ... near me\") and,\n    for the yes/no shapes, a near-word. Everything the bot does know - a card, a name, the\n    composer's own line - keeps its turn before this is asked.\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(16)) } }) {
    return Py::None;
    }
    bare_any = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_ANY(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    for_the_place_test = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cash_machine")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CASH_MACHINE(), "sub", args))(&__args) };
    if py_truthy(&{ let __b = bare_any.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THERE_IS(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(for_the_place_test.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ANY_ABSTRACT(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(for_the_place_test.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_PLACE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ALREADY_A_PLAN(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    asking_where = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHERE_TO_GET(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    asking_if_there_is = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THERE_IS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { bare_any.clone() } }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEARBY(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = bare_any.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asking_if_there_is.clone())) } }) {
    asking_if_there_is = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOURS(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    asking_for_a_name = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_RECOMMEND(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_ASKING_MY_OPINION(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_HAD_IT(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_NAME_FOR_SOMEBODY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TRADE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_TO_HELP_WITH(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEBODY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_JOB_ON_A_THING(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } } } } } } } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asking_where.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking_if_there_is.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asking_for_a_name.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&asking_for_a_name.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_HAND()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    who_does = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwho (?:does|sells|fixes|repairs|makes)\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEARBY(), "search", args))(&__args) } } };
    if py_truthy(&{ let __b = asking_where.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&who_does.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:get|buy|find|hire|pick up|source)\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEARBY(), "search", args))(&__args) })) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOURS(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_HOURS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_person_for_the_job(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_HAND()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&thing.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NO_MAP()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NO_MAP_UNNAMED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.find_a_place".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALREADY_A_PLAN" => Some(g_ALREADY_A_PLAN()),
            "ALREADY_A_ROUTE" => Some(g_ALREADY_A_ROUTE()),
            "ANY_ABSTRACT" => Some(g_ANY_ABSTRACT()),
            "A_ASKING_MY_OPINION" => Some(g_A_ASKING_MY_OPINION()),
            "A_HAD_IT" => Some(g_A_HAD_IT()),
            "A_HAND" => Some(g_A_HAND()),
            "A_JOB_ON_A_THING" => Some(g_A_JOB_ON_A_THING()),
            "A_LEAD" => Some(g_A_LEAD()),
            "A_NAME_FOR_SOMEBODY" => Some(g_A_NAME_FOR_SOMEBODY()),
            "A_NOD" => Some(g_A_NOD()),
            "A_RECOMMEND" => Some(g_A_RECOMMEND()),
            "A_TAIL" => Some(g_A_TAIL()),
            "A_THING_TO_HELP_WITH" => Some(g_A_THING_TO_HELP_WITH()),
            "A_TRADE" => Some(g_A_TRADE()),
            "BARE_ANY" => Some(g_BARE_ANY()),
            "CASH_MACHINE" => Some(g_CASH_MACHINE()),
            "HOURS" => Some(g_HOURS()),
            "NEARBY" => Some(g_NEARBY()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "NO_MAP" => Some(g_NO_MAP()),
            "NO_MAP_UNNAMED" => Some(g_NO_MAP_UNNAMED()),
            "NO_THING" => Some(g_NO_THING()),
            "ROUTE_POOLS" => Some(g_ROUTE_POOLS()),
            "SOMEBODY" => Some(g_SOMEBODY()),
            "THERE_IS" => Some(g_THERE_IS()),
            "THE_HOURS" => Some(g_THE_HOURS()),
            "WHERE_TO_GET" => Some(g_WHERE_TO_GET()),
            "_ROUTE_HEADS" => Some(g__ROUTE_HEADS()),
            "_a_person_for_the_job" => Some(py_func("_a_person_for_the_job", |args: &[Py]| _a_person_for_the_job(args))),
            "_a_thing" => Some(py_func("_a_thing", |args: &[Py]| _a_thing(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "already_a_route" => Some(py_func("already_a_route", |args: &[Py]| already_a_route(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALREADY_A_PLAN" => s_ALREADY_A_PLAN(v),
            "ALREADY_A_ROUTE" => s_ALREADY_A_ROUTE(v),
            "ANY_ABSTRACT" => s_ANY_ABSTRACT(v),
            "A_ASKING_MY_OPINION" => s_A_ASKING_MY_OPINION(v),
            "A_HAD_IT" => s_A_HAD_IT(v),
            "A_HAND" => s_A_HAND(v),
            "A_JOB_ON_A_THING" => s_A_JOB_ON_A_THING(v),
            "A_LEAD" => s_A_LEAD(v),
            "A_NAME_FOR_SOMEBODY" => s_A_NAME_FOR_SOMEBODY(v),
            "A_NOD" => s_A_NOD(v),
            "A_RECOMMEND" => s_A_RECOMMEND(v),
            "A_TAIL" => s_A_TAIL(v),
            "A_THING_TO_HELP_WITH" => s_A_THING_TO_HELP_WITH(v),
            "A_TRADE" => s_A_TRADE(v),
            "BARE_ANY" => s_BARE_ANY(v),
            "CASH_MACHINE" => s_CASH_MACHINE(v),
            "HOURS" => s_HOURS(v),
            "NEARBY" => s_NEARBY(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "NO_MAP" => s_NO_MAP(v),
            "NO_MAP_UNNAMED" => s_NO_MAP_UNNAMED(v),
            "NO_THING" => s_NO_THING(v),
            "ROUTE_POOLS" => s_ROUTE_POOLS(v),
            "SOMEBODY" => s_SOMEBODY(v),
            "THERE_IS" => s_THERE_IS(v),
            "THE_HOURS" => s_THE_HOURS(v),
            "WHERE_TO_GET" => s_WHERE_TO_GET(v),
            "_ROUTE_HEADS" => s__ROUTE_HEADS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.find_a_place", module_obj);
    pyrt::xrefs::reg_fn("compose.find_a_place", "already_a_route", py_func("already_a_route", |args: &[Py]| already_a_route(args)));
    pyrt::xrefs::reg_fn("compose.find_a_place", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.find_a_place", "_a_thing", py_func("_a_thing", |args: &[Py]| _a_thing(args)));
    pyrt::xrefs::reg_fn("compose.find_a_place", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("compose.find_a_place", "_a_person_for_the_job", py_func("_a_person_for_the_job", |args: &[Py]| _a_person_for_the_job(args)));
    pyrt::xrefs::reg_fn("compose.find_a_place", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.find_a_place", "ALREADY_A_PLAN", g_ALREADY_A_PLAN, s_ALREADY_A_PLAN);
    pyrt::xrefs::reg_access("compose.find_a_place", "ALREADY_A_ROUTE", g_ALREADY_A_ROUTE, s_ALREADY_A_ROUTE);
    pyrt::xrefs::reg_access("compose.find_a_place", "ANY_ABSTRACT", g_ANY_ABSTRACT, s_ANY_ABSTRACT);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_ASKING_MY_OPINION", g_A_ASKING_MY_OPINION, s_A_ASKING_MY_OPINION);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_HAD_IT", g_A_HAD_IT, s_A_HAD_IT);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_HAND", g_A_HAND, s_A_HAND);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_JOB_ON_A_THING", g_A_JOB_ON_A_THING, s_A_JOB_ON_A_THING);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_LEAD", g_A_LEAD, s_A_LEAD);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_NAME_FOR_SOMEBODY", g_A_NAME_FOR_SOMEBODY, s_A_NAME_FOR_SOMEBODY);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_NOD", g_A_NOD, s_A_NOD);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_RECOMMEND", g_A_RECOMMEND, s_A_RECOMMEND);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_TAIL", g_A_TAIL, s_A_TAIL);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_THING_TO_HELP_WITH", g_A_THING_TO_HELP_WITH, s_A_THING_TO_HELP_WITH);
    pyrt::xrefs::reg_access("compose.find_a_place", "A_TRADE", g_A_TRADE, s_A_TRADE);
    pyrt::xrefs::reg_access("compose.find_a_place", "BARE_ANY", g_BARE_ANY, s_BARE_ANY);
    pyrt::xrefs::reg_access("compose.find_a_place", "CASH_MACHINE", g_CASH_MACHINE, s_CASH_MACHINE);
    pyrt::xrefs::reg_access("compose.find_a_place", "HOURS", g_HOURS, s_HOURS);
    pyrt::xrefs::reg_access("compose.find_a_place", "NEARBY", g_NEARBY, s_NEARBY);
    pyrt::xrefs::reg_access("compose.find_a_place", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.find_a_place", "NO_MAP", g_NO_MAP, s_NO_MAP);
    pyrt::xrefs::reg_access("compose.find_a_place", "NO_MAP_UNNAMED", g_NO_MAP_UNNAMED, s_NO_MAP_UNNAMED);
    pyrt::xrefs::reg_access("compose.find_a_place", "NO_THING", g_NO_THING, s_NO_THING);
    pyrt::xrefs::reg_access("compose.find_a_place", "ROUTE_POOLS", g_ROUTE_POOLS, s_ROUTE_POOLS);
    pyrt::xrefs::reg_access("compose.find_a_place", "SOMEBODY", g_SOMEBODY, s_SOMEBODY);
    pyrt::xrefs::reg_access("compose.find_a_place", "THERE_IS", g_THERE_IS, s_THERE_IS);
    pyrt::xrefs::reg_access("compose.find_a_place", "THE_HOURS", g_THE_HOURS, s_THE_HOURS);
    pyrt::xrefs::reg_access("compose.find_a_place", "WHERE_TO_GET", g_WHERE_TO_GET, s_WHERE_TO_GET);
    pyrt::xrefs::reg_access("compose.find_a_place", "_ROUTE_HEADS", g__ROUTE_HEADS, s__ROUTE_HEADS);
}
