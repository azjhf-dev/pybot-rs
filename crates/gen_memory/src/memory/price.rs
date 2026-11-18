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


thread_local! { static G_A_PRICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_ASK_WITH_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_INSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_SYMBOL_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_UNDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONEY_AFTER_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONEY_CEILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PHRASE_AFTER_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PRICE_FOR_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE_IN_THE_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRICE_RANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRICE_SYMBOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRICE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_PRICE_ASK() -> Py { ensure_init(); G_A_PRICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_ASK(v: Py) { ensure_init(); G_A_PRICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_ASK_WITH_THE_THING() -> Py { ensure_init(); G_A_PRICE_ASK_WITH_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_ASK_WITH_THE_THING(v: Py) { ensure_init(); G_A_PRICE_ASK_WITH_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_BETWEEN() -> Py { ensure_init(); G_A_PRICE_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_BETWEEN(v: Py) { ensure_init(); G_A_PRICE_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_INSIDE() -> Py { ensure_init(); G_A_PRICE_INSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_INSIDE(v: Py) { ensure_init(); G_A_PRICE_INSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_OVER() -> Py { ensure_init(); G_A_PRICE_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_OVER(v: Py) { ensure_init(); G_A_PRICE_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_SYMBOL_ONLY() -> Py { ensure_init(); G_A_PRICE_SYMBOL_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_SYMBOL_ONLY(v: Py) { ensure_init(); G_A_PRICE_SYMBOL_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_UNDER() -> Py { ensure_init(); G_A_PRICE_UNDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_UNDER(v: Py) { ensure_init(); G_A_PRICE_UNDER.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_A_RATE() -> Py { ensure_init(); G__AFTER_A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_A_RATE(v: Py) { ensure_init(); G__AFTER_A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_WORDS() -> Py { ensure_init(); G__ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); G__ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONEY_AFTER_THE_ASK() -> Py { ensure_init(); G__A_MONEY_AFTER_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONEY_AFTER_THE_ASK(v: Py) { ensure_init(); G__A_MONEY_AFTER_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONEY_CEILING() -> Py { ensure_init(); G__A_MONEY_CEILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONEY_CEILING(v: Py) { ensure_init(); G__A_MONEY_CEILING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PHRASE_AFTER_THE_ASK() -> Py { ensure_init(); G__A_PHRASE_AFTER_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PHRASE_AFTER_THE_ASK(v: Py) { ensure_init(); G__A_PHRASE_AFTER_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PRICE_FOR_THE_THING() -> Py { ensure_init(); G__A_PRICE_FOR_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PRICE_FOR_THE_THING(v: Py) { ensure_init(); G__A_PRICE_FOR_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE_IN_THE_SENTENCE() -> Py { ensure_init(); G__A_RATE_IN_THE_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE_IN_THE_SENTENCE(v: Py) { ensure_init(); G__A_RATE_IN_THE_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__PRICE_RANGE() -> Py { ensure_init(); G__PRICE_RANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRICE_RANGE(v: Py) { ensure_init(); G__PRICE_RANGE.with(|c| *c.borrow_mut() = v); }
pub fn g__PRICE_SYMBOL() -> Py { ensure_init(); G__PRICE_SYMBOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRICE_SYMBOL(v: Py) { ensure_init(); G__PRICE_SYMBOL.with(|c| *c.borrow_mut() = v); }
pub fn g__PRICE_WORD() -> Py { ensure_init(); G__PRICE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRICE_WORD(v: Py) { ensure_init(); G__PRICE_WORD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A price the person gives, read against the range the bot itself named a turn ago.\n\n\"the cheapest quote i got for a new boiler is 2400\" / \"is that a lot\" was answered \"Alright then\n- good luck with it\", and \"is that about right\" after a boiler service was answered with the\nservice card again: the range was on the screen, and nothing in the bot read the two figures\ntogether. The knowledge base can price a job it is asked about (\"is 400 too much for a roof\nrepair\"), but a follow-up names neither the job nor the question - only \"that\" - so the subject\nand the range both have to come from the answer already given.\n\nWhat comes back is only ever what the answer said: where the person's figure stands against the\nrange in it. Nothing here knows what a boiler costs, so a line whose last answer named no money\nrange is left to the modules that do (this reader returns None).\n\nThe ask is a judgement of a figure - \"is that a lot\", \"does that sound right\", \"is that\nexpensive\" - and not a question about a thing: \"how much does a boiler cost\" and \"when is it\nworth it\" are answered by the entries, and \"is 400 too much for a roof repair\" names its subject\nand stays with the price entry that owns it.\n");
    s_A_PRICE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was)\\s+(?:that|this|it|(?:that|this)\\s+[a-z]+)\\s+(?:a\\s+lot|expensive|dear|cheap|about\\s+right|the\\s+going\\s+rate|the\\s+right\\s+price|the\\s+normal\\s+price|normal|reasonable|fair|too\\s+much|steep|a\\s+rip[-\\s]?off|overpriced|in\\s+line)\\b|\\b(?:does|do|did)\\s+(?:that|this|it)\\s+(?:sound|seem)\\s+(?:about\\s+)?(?:right|ok|okay|reasonable|fair|normal|expensive)\\b|\\b(?:is|was)\\s+(?:that|this|it)\\s+(?:the\\s+)?(?:going\\s+rate|normal|what\\s+it\\s+should\\s+be)\\b|\\bam\\s+i\\s+being\\s+(?:ripped\\s+off|overcharged|done)\\b|\\bdid\\s+i\\s+(?:get|pay)\\s+(?:a\\s+)?(?:good|fair|bad)\\s+(?:price|deal|rate)\\b|\\b(?:is|was)\\s+(?:that|this|it)\\s+(?:a\\s+)?(?:good|fair|bad)\\s+(?:price|deal|rate|quote)\\b|\\b(?:that|this|it)(?:'s|s|\\s+is|\\s+was)?\\s+(?:sounds?|seems?|looks?)\\s+(?:like\\s+)?(?:a\\s+lot|expensive|dear|steep|a\\s+rip[-\\s]?off|too\\s+much|about\\s+right|the\\s+going\\s+rate|the\\s+right\\s+price)\\b|\\b(?:that|this|it)\\s*(?:'s|s|is|was)\\s+(?:(?:very|really|quite|a\\s+bit|bit|so|pretty|rather)\\s+)?(?:a\\s+lot|expensive|dear|steep|too\\s+much|a\\s+rip[-\\s]?off|overpriced)\\b|\\b(?:sounds?|seems?|looks?)\\s+(?:like\\s+)?(?:(?:very|really|quite|a\\s+bit|bit|so|pretty)\\s+)?(?:a\\s+lot|expensive|dear|steep|too\\s+much|about\\s+right|the\\s+going\\s+rate)\\b|\\bhow\\s+does\\s+that\\s+(?:sound|compare)\\b|\\b(?:is|was)\\s+(?:that|this|it)\\s+(?:more|less)\\s+than\\s+(?:usual|normal|it\\s+should\\s+be)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PRICE_SYMBOL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("£|\\$|€")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PRICE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:quid|pounds?|pence|euros?|dollars?|grand|notes?|money|cost(?:s|ing)?|costs?|price[sd]?|quoted?|quote[sd]?|charge[ds]?|(?:pay|pays|paying|paid|payed)|wants?|wanting|asking|budget|spent|spend|cheapest|dearest|expensive|cheaper|dearer|fees?|bill|bills|invoice|estimate[d]?|rates?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AFTER_A_RATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:a|an|per|each|every)\\s+(day|week|month|year|hour|visit|call[-\\s]?out|session|night)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MONEY_CEILING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:up\\s+to|no\\s+more\\s+than|at\\s+most|under|below|less\\s+than|over|more\\s+than|above|at\\s+least)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PRICE_RANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?P<low>£|\\$|€)?\\s?(?P<low_n>\\d[\\d,]*(?:\\.\\d+)?)\\s?(?:(?:-|–|—|to)\\s?|(?<![,])\\s+and\\s+)(?:(?P<high>£|\\$|€)\\s?)?(?P<high_n>\\d[\\d,]*(?:\\.\\d+)?)\\s?(?P<unit>k\\b|pounds?|quid|euros?|dollars?|grand)?")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PRICE_SYMBOL_ONLY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[£$€]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PRICE_INSIDE(py_tuple(vec![py_str("That is about right. The usual range is {aim}, and {price} is inside it."), py_str("Yes, {price} is in the right range - the answer there put it at {aim}."), py_str("{price} sits inside the usual range of {aim}, so that is the going rate.")]));
    s_A_PRICE_OVER(py_tuple(vec![py_str("That is above the going rate of {aim}, so it is worth checking what is behind it."), py_str("{price} is over the {aim} the usual price comes to, so it is worth a second quote."), py_str("That is on the high side: the range is {aim}, and {price} is above it.")]));
    s_A_PRICE_UNDER(py_tuple(vec![py_str("That is below the usual {aim}, so check what it covers - a part, a deposit, or the whole job."), py_str("{price} is under the {aim} it usually comes to, so check what the quote leaves out."), py_str("That is cheap against the {aim} the answer named, so make sure the price covers the job.")]));
    s_A_PRICE_BETWEEN(py_tuple(vec![py_str("{price} is over the {below} and under the {above}, so it depends which of those you had."), py_str("{price} falls between the {below} and the {above} - which of those you had decides whether it is fair.")]));
    s_A_PRICE_ASK_WITH_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was)\\s+(?P<figure>[^,.?!]{0,40}?)\\s*(?:too\\s+much|a\\s+lot|expensive|dear|steep|a\\s+rip[-\\s]?off|about\\s+right|reasonable|fair|the\\s+going\\s+rate|the\\s+normal\\s+price|the\\s+right\\s+price|(?:a|an)\\s+(?:good|fair|reasonable|normal|ok|okay)\\s+(?:price|deal|rate|quote))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PRICE_FOR_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:for|on)\\s+(?:a|an|the|my|our|your|his|her|their|this|that)?\\s*[a-z]")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASK_WORDS(py_set(vec![py_str("much"), py_str("lot"), py_str("expensive"), py_str("dear"), py_str("steep"), py_str("rip"), py_str("off"), py_str("right"), py_str("reasonable"), py_str("fair"), py_str("going"), py_str("rate"), py_str("normal"), py_str("price"), py_str("prices"), py_str("good"), py_str("ok"), py_str("okay"), py_str("deal"), py_str("quote"), py_str("quoted"), py_str("cost"), py_str("costs"), py_str("costing"), py_str("charge"), py_str("charged"), py_str("charges"), py_str("paid"), py_str("pay"), py_str("paying"), py_str("payed"), py_str("was"), py_str("is"), py_str("did"), py_str("does"), py_str("do"), py_str("for"), py_str("to"), py_str("the"), py_str("a"), py_str("an"), py_str("and"), py_str("on"), py_str("of"), py_str("in"), py_str("it"), py_str("that"), py_str("this"), py_str("me"), py_str("my"), py_str("we"), py_str("i"), py_str("at"), py_str("just"), py_str("only"), py_str("about"), py_str("bit"), py_str("little"), py_str("total"), py_str("altogether"), py_str("bill"), py_str("quid"), py_str("pounds"), py_str("pound"), py_str("euro"), py_str("euros"), py_str("dollar"), py_str("dollars"), py_str("worth"), py_str("sound"), py_str("sounds"), py_str("seem"), py_str("seems"), py_str("again"), py_str("too"), py_str("get"), py_str("got"), py_str("all"), py_str("so")]));
    s__A_MONEY_AFTER_THE_ASK(py_set(vec![py_str("money"), py_str("quid"), py_str("pounds"), py_str("pound"), py_str("euros"), py_str("euro"), py_str("dollars"), py_str("dollar"), py_str("pence"), py_str("grand"), py_str("notes"), py_str("cash"), py_str("dosh"), py_str("price"), py_str("prices"), py_str("rate"), py_str("rates"), py_str("bill"), py_str("total")]));
    s__A_PHRASE_AFTER_THE_ASK(py_set(vec![py_str("to"), py_str("for"), py_str("on"), py_str("in"), py_str("at"), py_str("by"), py_str("with"), py_str("per"), py_str("and"), py_str("or"), py_str("but"), py_str("if"), py_str("than"), py_str("compared"), py_str("when"), py_str("these"), py_str("those"), py_str("this"), py_str("that"), py_str("it"), py_str("now"), py_str("nowadays"), py_str("then"), py_str("though"), py_str("anyway"), py_str("honestly"), py_str("really"), py_str("actually"), py_str("do"), py_str("does"), py_str("did"), py_str("should"), py_str("would"), py_str("a"), py_str("an"), py_str("the"), py_str("so"), py_str("again"), py_str("altogether"), py_str("all"), py_str("any"), py_str("as"), py_str("from"), py_str("about"), py_str("worth"), py_str("sounds"), py_str("seems"), py_str("sound"), py_str("seem"), py_str("overall"), py_str("including")]));
    s__A_RATE_IN_THE_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|per|each)\\s+(?:day|hour|week|month|night|visit|person|load)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _money(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("The figure as it is said on the screen: 2400 in a range is 2,400.");
    if py_truthy(&op_ge(&value.clone(), &Py::Int(1000))) {
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{:,}"), "format", args))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{:,.2f}"), "format", args))(&__args) } };
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { op_modulo(&py_str("%d"), &value.clone()) } else { op_modulo(&py_str("%.2f"), &value.clone()) };
    Py::None
}
pub fn _a_value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let _ = py_str("A figure as it was written: \"2,400\" and \"2400\" are the same number.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::Float(0.0);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_money_said_back(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut mark: Py = Py::None;
    let _ = py_str("A figure keeps the symbol the line wrote; with none written it is the pound it was asked in.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[£$€]")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return said.clone();
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("£"), py_str("$"), py_str("€")]));
    for __item in __iter1 {
    mark = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &mark.clone()))) {
    return op_add(&mark.clone(), &said.clone());
    }
    }
    return op_add(&py_str("£"), &said.clone());
    Py::None
}
pub fn _the_price_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("\"2400\" said back the way it is written: \"£2,400\".\n\n    The currency is part of the figure when it is said back: a verdict that opens \"400 sits inside\n    the usual range of £150 to £400\" reads as a count beside a price, and every pool below opens\n    on the figure.\n    ");
    return op_add(&py_str("£"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) });
    Py::None
}
pub fn _the_figure_in_the_range_s_style(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut range_said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "range_said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: range_said"))) };
    let mut figure: Py = Py::None;
    let mut mark: Py = Py::None;
    let _ = py_str("The person's figure written the way the range beside it is written.");
    figure = { let __b = said.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) } } };
    figure = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d[\\d,]*(?:\\.\\d+)?")); __args.push({ let figure = figure.clone(); let mark = mark.clone(); let mine = mine.clone(); let range_said = range_said.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let figure = figure.clone(); let mark = mark.clone(); let mine = mine.clone(); let range_said = range_said.clone(); let said = said.clone(); let one: Py = __kwp(args, 0, "one").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) } }) }); __args.push(figure.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    mark = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = range_said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PRICE_SYMBOL_ONLY(), "search", args))(&__args) };
    if py_truthy(&{ let __b = mark.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(figure.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PRICE_SYMBOL_ONLY(), "search", args))(&__args) })) } }) {
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mark.clone(), "group", args))(&__args) }, &figure.clone());
    }
    return figure.clone();
    Py::None
}
pub fn _number_tokens(args: &[Py]) -> Py {
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
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d[\\d,]*(?:\\.\\d+)?")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut match_: Py = __item.clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }])); } py_list(__out) })() };
    Py::None
}
pub fn _prices_in(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut marked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut start: Py = Py::None;
    let mut thousands: Py = Py::None;
    let mut token: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("[(value, spoken)] for the figures in the line that are marked as money.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _number_tokens(args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    token = __unpk2[0].clone();
    start = __unpk2[1].clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    continue;
    }
    before = slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&start.clone(), &Py::Int(24))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &start.clone(), &Py::None);
    after = slice_or_raise(&said.clone(), &op_add(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &op_add(&op_add(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(20)), &Py::None);
    marked = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&before.clone(), &token.clone()), &slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_SYMBOL(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_WORD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s?(?:quid|pounds|euros?|dollars?|grand|k)\\b")); __args.push(after.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&marked.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AFTER_A_RATE(), "match", args))(&__args) }) {
    continue;
    }
    thousands = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s?(k|grand|thousand)\\b")); __args.push(after.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&token.clone(), "strip", args))(&__args) };
    if py_truthy(&thousands.clone()) {
    value = op_mul(&value.clone(), &Py::Int(1000));
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &start.clone(), &op_add(&op_add(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thousands.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), "strip", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![value.clone(), spoken.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    return found.clone();
    Py::None
}
pub fn _a_price_at_a_rate(args: &[Py]) -> Py {
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
    let mut figures: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the only figures in the line come with a period on them (\"£200 to £300 a day\").");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:£|\\$|€)?\\s?\\d[\\d,]*(?:\\.\\d+)?(?:\\s?(?:k|pounds|quid))?")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut match_: Py = __item.clone(); __out.push(match_.clone()); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&figures.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&figures.clone());
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::Int(20)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AFTER_A_RATE(), "match", args))(&__args) }))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _price_ranges_in(args: &[Py]) -> Py {
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
    let mut period: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut high: Py = Py::None;
    let mut high_token: Py = Py::None;
    let mut low: Py = Py::None;
    let mut low_token: Py = Py::None;
    let mut marked: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("[(start, end, low, high, spoken)] for each money range in the answer.\n\n    A range in money is two figures with money on them or a money word beside them, so \"10 to 15\n    years\", \"two or three quotes\" and \"70 to 80 per cent\" are not ranges here.\n\n    A range with a period on it (\"£200 to £300 a day\", \"£140 to £150 a month\") is only read when\n    the person said their figure in the same period: a roofer's day rate says nothing about a whole\n    job at £400, and a monthly bill says everything about a bill the person calls monthly.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_RANGE(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("low_n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("high_n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }]), 2, None);
    low_token = __unpk2[0].clone();
    high_token = __unpk2[1].clone();
    let __try3 = exc::catch(|| {
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low_token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    high = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(high_token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&high.clone())); if py_truthy(&__b) { __b } else { op_le(&high.clone(), &low.clone()) } } } }) {
    continue;
    }
    tail = slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::Int(20)), &Py::None);
    head = slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::Int(24))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None);
    unit = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    marked = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("low")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("high")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("k")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "lower", args))(&__args) })) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::Int(3))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::Int(3)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_SYMBOL(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_WORD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_WORD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&marked.clone()))) {
    continue;
    }
    rate = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AFTER_A_RATE(), "match", args))(&__args) };
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let __b = period.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rate.clone(), "group", args))(&__args) }, "lower", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "lower", args))(&__args) }, &py_str("k")))) {
    let __unpk8 = py_unpack(&py_tuple(vec![op_mul(&low.clone(), &Py::Int(1000)), op_mul(&high.clone(), &Py::Int(1000))]), 2, None);
    low = __unpk8[0].clone();
    high = __unpk8[1].clone();
    }
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+")); __args.push(py_str(" ")); __args.push(spoken.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MONEY_CEILING(), "search", args))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, low.clone(), high.clone(), spoken.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    return found.clone();
    Py::None
}
pub fn _the_price_they_gave(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut marked: Py = Py::None;
    let mut token: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(value, spoken) for the price the person gave, from this line or one of the last three.");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    lines = op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(3), &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() });
    let __iter1 = py_iter_or_raise(&lines.clone());
    for __item in __iter1 {
    line = __item.clone();
    marked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _prices_in(args))(&__args) };
    if py_truthy(&marked.clone()) {
    return geti(&marked.clone(), &op_neg(&Py::Int(1)));
    }
    bare = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _number_tokens(args))(&__args) }) { let __unpk6087 = py_unpack(&__item.clone(), 2, None); let mut token: Py = __unpk6087[0].clone(); let mut _u: Py = __unpk6087[1].clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&token.clone(), "strip", args))(&__args) }])); } py_list(__out) })() }) { let __unpk7031 = py_unpack(&__item.clone(), 2, None); let mut value: Py = __unpk7031[0].clone(); let mut token: Py = __unpk7031[1].clone(); if py_truthy(&op_ge(&value.clone(), &Py::Int(40))) {  __out.push(py_tuple(vec![value.clone(), token.clone()])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_WORD(), "search", args))(&__args) }) {
    return geti(&bare.clone(), &op_neg(&Py::Int(1)));
    }
    return py_tuple(vec![Py::None, py_str("")]);
    }
    return py_tuple(vec![Py::None, py_str("")]);
    Py::None
}
pub fn _a_bare_price_in_the_thread(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut token: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The price they gave with no unit on it: \"my boiler service was 250\".\n\n    A bare figure is only read when no line in the thread marked one as money, and the answer\n    still has to name a money range near it, so a count or a size is not turned into a price.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    lines = op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(3), &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() });
    let __iter1 = py_iter_or_raise(&lines.clone());
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _prices_in(args))(&__args) }) {
    continue;
    }
    bare = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _number_tokens(args))(&__args) }) { let __unpk105 = py_unpack(&__item.clone(), 2, None); let mut token: Py = __unpk105[0].clone(); let mut _u: Py = __unpk105[1].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(token.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }); } py_list(__out) })() }) { let mut value: Py = __item.clone(); if py_truthy(&op_ge(&value.clone(), &Py::Int(40))) {  __out.push(value.clone()); } } py_list(__out) })() };
    if py_truthy(&bare.clone()) {
    return py_tuple(vec![geti(&bare.clone(), &Py::Int(0)), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&bare.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_price_said(args))(&__args) }]);
    }
    }
    return py_tuple(vec![Py::None, py_str("")]);
    Py::None
}
pub fn _answers_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The answers the bot gave, newest first.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reply")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) } } };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_range_that_is_about_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answers: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answers"))) };
    let mut price: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "price") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: price"))) };
    let mut period: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut bands: Py = Py::None;
    let mut best: Py = Py::None;
    let mut distance: Py = Py::None;
    let mut found: Py = Py::None;
    let mut high: Py = Py::None;
    let mut item: Py = Py::None;
    let mut low: Py = Py::None;
    let mut near: Py = Py::None;
    let mut said: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut second: Py = Py::None;
    let mut spoken: Py = Py::None;
    let _ = py_str("(low, high, spoken) for the range in the newest answer that is about this price, or None.\n\n    The answer can name several: a roofer's card names a patch, a flat roof and a whole roof,\n    and 400 is the patch. The nearest range wins as long as it is within three times the price -\n    a £6,000 re-roof is not read against a £150-£400 patch - and no verdict is given when two\n    ranges are within twice as near as each other, because then the answer did not say which job\n    the person is being quoted for.\n\n    A figure under every range the answer names, or over every one, is the exception: whichever\n    range is picked the verdict is the same, so the answer has said enough and the band nearest\n    the figure is the one to name. A 900 quote for a bathroom, against a card of £3,000 to £7,000\n    fitted and £8,000 to £12,000 with the plumbing changed, is worth asking about, and no card\n    that prices one job in two sizes should leave that unsaid.\n    ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&answers.clone(), &Py::None, &Py::Int(2), &Py::None));
    for __item in __iter1 {
    said = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(period.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _price_ranges_in(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_ranges_that_are_not_rates(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    bands = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut item: Py = __item.clone(); __out.push(py_tuple(vec![geti(&item.clone(), &Py::Int(2)), geti(&item.clone(), &Py::Int(3)), geti(&item.clone(), &Py::Int(4))])); } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bands.clone()) { let __unpk552 = py_unpack(&__item.clone(), 3, None); let mut low: Py = __unpk552[0].clone(); let mut _u: Py = __unpk552[1].clone(); let mut _u: Py = __unpk552[2].clone(); __out.push(op_lt(&price.clone(), &low.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bands.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bands.clone()) { let __unpk345 = py_unpack(&__item.clone(), 3, None); let mut _u: Py = __unpk345[0].clone(); let mut high: Py = __unpk345[1].clone(); let mut _u: Py = __unpk345[2].clone(); __out.push(op_gt(&price.clone(), &high.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bands.clone()); __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answers = answers.clone(); let bands = bands.clone(); let best = best.clone(); let distance = distance.clone(); let found = found.clone(); let high = high.clone(); let item = item.clone(); let low = low.clone(); let near = near.clone(); let period = period.clone(); let price = price.clone(); let said = said.clone(); let scored = scored.clone(); let second = second.clone(); let spoken = spoken.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answers = answers.clone(); let bands = bands.clone(); let best = best.clone(); let distance = distance.clone(); let found = found.clone(); let high = high.clone(); let item = item.clone(); let low = low.clone(); let near = near.clone(); let period = period.clone(); let price = price.clone(); let said = said.clone(); let scored = scored.clone(); let second = second.clone(); let spoken = spoken.clone(); let band: Py = __kwp(args, 0, "band").unwrap_or(Py::None); geti(&band.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    scored = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&bands.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    spoken = __unpk3[2].clone();
    if py_truthy(&{ let __cmp41: Py = price.clone(); if py_truthy(&op_le(&low.clone(), &__cmp41)) { op_le(&__cmp41, &high.clone()) } else { Py::Bool(false) } }) {
    distance = Py::Float(0.0);
    } else {
    if py_truthy(&op_gt(&price.clone(), &high.clone())) {
    distance = op_sub(&price.clone(), &high.clone());
    } else {
    distance = op_sub(&low.clone(), &price.clone());
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![distance.clone(), low.clone(), high.clone(), spoken.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scored.clone(), "append", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scored.clone(), "sort", args))(&__args) };
    best = geti(&scored.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = op_gt(&price.clone(), &geti(&best.clone(), &Py::Int(2))); if !py_truthy(&__b) { __b } else { op_gt(&price.clone(), &op_mul(&geti(&best.clone(), &Py::Int(2)), &Py::Int(3))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&price.clone(), &geti(&best.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { op_lt(&op_mul(&price.clone(), &Py::Int(3)), &geti(&best.clone(), &Py::Int(1))) } }) {
    continue;
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(scored.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) {
    let __unpk5 = py_unpack(&py_tuple(vec![geti(&geti(&scored.clone(), &Py::Int(0)), &Py::Int(0)), geti(&geti(&scored.clone(), &Py::Int(1)), &Py::Int(0))]), 2, None);
    near = __unpk5[0].clone();
    second = __unpk5[1].clone();
    if py_truthy(&{ let __b = near.clone(); if !py_truthy(&__b) { __b } else { op_lt(&second.clone(), &op_mul(&near.clone(), &Py::Int(2))) } }) {
    continue;
    }
    }
    return py_tuple(vec![geti(&best.clone(), &Py::Int(1)), geti(&best.clone(), &Py::Int(2)), geti(&best.clone(), &Py::Int(3))]);
    }
    return Py::None;
    Py::None
}
pub fn _the_between_verdict(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut below: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "below") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: below"))) };
    let mut above: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "above") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: above"))) };
    let mut memory: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut price: Py = Py::None;
    let _ = py_str("The verdict for a figure that falls between two bands the card prices, or None.");
    price = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(below.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_figure_in_the_range_s_style(args))(&__args) };
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_PRICE_BETWEEN()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(op_add(&py_str("between"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("price".to_string(), price.clone()), ("below".to_string(), below.clone()), ("above".to_string(), above.clone()), ("Price".to_string(), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&price.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&price.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["KeyError", "IndexError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_verdict(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut low: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut high: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let mut spoken: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spoken") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spoken"))) };
    let mut memory: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut own: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { Py::Bool(false) };
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut price: Py = Py::None;
    let mut stood: Py = Py::None;
    let mut told: Py = Py::None;
    let _ = py_str("The judgement said in the answer's own words: inside, above or below its range.\n\n    `own` is the card this turn earns, which the person does not see when the verdict takes its\n    place: the words that point back at an answer on the screen are left out, so nothing says\n    \"the answer there\" about a card that was never shown.\n    ");
    price = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(spoken.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_figure_in_the_range_s_style(args))(&__args) };
    if py_truthy(&{ let __cmp11: Py = mine.clone(); if py_truthy(&op_le(&low.clone(), &__cmp11)) { op_le(&__cmp11, &high.clone()) } else { Py::Bool(false) } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![g_A_PRICE_INSIDE(), py_str("inside")]), 2, None);
    pool = __unpk2[0].clone();
    told = __unpk2[1].clone();
    } else {
    if py_truthy(&op_gt(&mine.clone(), &high.clone())) {
    let __unpk3 = py_unpack(&py_tuple(vec![g_A_PRICE_OVER(), py_str("over")]), 2, None);
    pool = __unpk3[0].clone();
    told = __unpk3[1].clone();
    } else {
    let __unpk4 = py_unpack(&py_tuple(vec![g_A_PRICE_UNDER(), py_str("under")]), 2, None);
    pool = __unpk4[0].clone();
    told = __unpk4[1].clone();
    }
    }
    lines = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&own.clone()) {
    stood = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("the answer")))) {  __out.push(line.clone()); } } py_list(__out) })() };
    lines = { let __b = stood.clone(); if py_truthy(&__b) { __b } else { lines.clone() } };
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); __args.push(memory.clone()); __args.push(op_add(&told.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    let __try5 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("aim".to_string(), spoken.clone()), ("price".to_string(), price.clone()), ("Price".to_string(), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&price.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&price.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret6: Py = Py::Bool(false);
    let mut __val7: Py = Py::None;
    let mut __unhandled8: Option<exc::PyExc> = None;
    match __try5 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret6 = __p[0].clone(); __val7 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled9 = false;
            if !__handled9 && exc::matches_any(&__e, &["KeyError", "IndexError", "ValueError"]) {
                __handled9 = true;
    return Py::None;
            }
            if !__handled9 { __unhandled8 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled8 { exc::raise(__e); }
    if py_truthy(&__ret6) { return __val7; }
    Py::None
}
pub fn _the_thing_priced(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line prices (\"a haircut\"), or \"\" when the line names no thing at all.");
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PRICE_SYMBOL(), "sub", args))(&__args) };
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d[\\d,.]*")); __args.push(py_str(" ")); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ASK_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _the_price_they_mean(args: &[Py]) -> Py {
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
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("(value, spoken) for the figure the line is asking a judgement on, or (None, \"\").");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_price_they_gave(args))(&__args) }, 2, None);
    mine = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_bare_price_in_the_thread(args))(&__args) }, 2, None);
    mine = __unpk2[0].clone();
    said = __unpk2[1].clone();
    }
    return py_tuple(vec![mine.clone(), said.clone()]);
    Py::None
}
pub fn _the_period_they_said(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The period the person put their own figure in - \"a month\", \"a week\" - or None.\n\n    A bill of \"200 this month\" is compared with a range the answer gave \"a month\"; a quote for a\n    whole job is not compared with a roofer's day rate.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    lines = op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(3), &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() });
    let __iter1 = py_iter_or_raise(&lines.clone());
    for __item in __iter1 {
    line = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|per|each|every|this|last)\\s+(day|week|month|year|hour|night)\\b")); __args.push(line.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&found.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_price_ask(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The ask when it is about a figure, or None when the words after it name another quantity.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PRICE_ASK(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    tail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*of\\b")); __args.push(py_str("")); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return found.clone();
    }
    first = geti(&words.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a lot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&first.clone(), &py_str("of"))) } }) {
    return if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_MONEY_AFTER_THE_ASK(), &geti(&words.clone(), &Py::Int(1)))) } }) { found.clone() } else { Py::None };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_MONEY_AFTER_THE_ASK(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_PHRASE_AFTER_THE_ASK(), &first.clone())) } }) {
    return found.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_rate_is_all_the_answer_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answers: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answers"))) };
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the newest answer prices only a day or an hour of the work, and never the job.");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&answers.clone(), &Py::None, &Py::Int(1), &Py::None));
    for __item in __iter1 {
    said = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _price_ranges_in(args))(&__args) };
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_ranges_that_are_not_rates(args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_second_quote_is_the_test(args: &[Py]) -> Py {
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
    let _ = py_str("What settles a figure for a whole job: the same job priced a second time.");
    let mut A_JOB_JUDGED: Py = pyrt::xrefs::xget("compose.prices", "A_JOB_JUDGED");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(A_JOB_JUDGED.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(py_str("jobjudged")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn judged(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut answers: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut period: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A line asking whether a figure they gave is high or low, from the answer already given.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_price_ask(args))(&__args) }))) {
    return Py::None;
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    answers = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _answers_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&answers.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_price_they_mean(args))(&__args) }, 2, None);
    mine = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_money_said_back(args))(&__args) };
    period = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_period_they_said(args))(&__args) };
    aim = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answers.clone()); __args.push(mine.clone()); __args.push(period.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_range_that_is_about_it(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&aim.clone(), &Py::None))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&period.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answers.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_rate_is_all_the_answer_names(args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_second_quote_is_the_test(args))(&__args) };
    }
    return Py::None;
    }
    let __unpk2 = py_unpack(&aim.clone(), 3, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    spoken = __unpk2[2].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(low.clone()); __args.push(high.clone()); __args.push(spoken.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict(args))(&__args) };
    Py::None
}
pub fn judged_with_the_card(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut marked: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut verdict: Py = Py::None;
    let _ = py_str("A price the line gives for a thing, judged against the range the answer for it names.\n\n    \"is 6000 too much for a new boiler\" is answered with the card for a new boiler, which says\n    what one usually costs - and the person's own figure is never put beside it. Unlike the ask a\n    turn later, this one names the thing it prices, so the card this line earns is the right card\n    and its range is the range to read.\n\n    Two conditions keep it honest. The line has to name a thing (\"is that a lot\" prices nothing\n    the card can be trusted for, and the answer a turn ago is where that ask belongs), and the\n    card has to say one thing about the figure: inside one of its ranges, or above all of them,\n    or below all of them. A card that prices several jobs - a cut, a colour, a blow dry - has the\n    figure over one and under another, and then it has not said which job is being priced, so\n    nothing is claimed here.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PRICE_ASK_WITH_THE_THING(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_priced(args))(&__args) }))) {
    return Py::None;
    }
    tokens = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("figure")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _number_tokens(args))(&__args) };
    if py_truthy(&tokens.clone()) {
    let __try1 = exc::catch(|| {
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&tokens.clone(), &op_neg(&Py::Int(1))), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = mine.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = slice_or_raise(&text.clone(), &geti(&geti(&tokens.clone(), &op_neg(&Py::Int(1))), &Py::Int(1)), &op_add(&geti(&geti(&tokens.clone(), &op_neg(&Py::Int(1))), &Py::Int(1)), &Py::Int(12)), &Py::None); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AFTER_A_RATE(), "match", args))(&__args) } } }) {
    return Py::None;
    }
    } else {
    marked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _prices_in(args))(&__args) };
    mine = if py_truthy(&marked.clone()) { geti(&geti(&marked.clone(), &op_neg(&Py::Int(1))), &Py::Int(0)) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PRICE_FOR_THE_THING(), "search", args))(&__args) }))) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_price_said(args))(&__args) };
    verdict = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(answer.clone()); __args.push(memory.clone()); __kws.extend(vec![("definite_only".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict_against_a_card(args))(&__args) };
    if py_truthy(&verdict.clone()) {
    return verdict.clone();
    }
    opening = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "strip", args))(&__args) }); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    if py_truthy(&Py::Bool(!py_eq(&opening.clone(), &answer.clone()))) {
    verdict = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(opening.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict_against_a_card(args))(&__args) };
    if py_truthy(&verdict.clone()) {
    return verdict.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(answer.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict_against_a_card(args))(&__args) };
    Py::None
}
pub fn _the_ranges_that_are_not_rates(args: &[Py]) -> Py {
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
    let mut ranges: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ranges") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ranges"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut end: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut left: Py = Py::None;
    let mut right: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The card's ranges that price the thing, leaving out a rate for a day or an hour of it.");
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&ranges.clone());
    for __item in __iter1 {
    item = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&item.clone(), &Py::Int(0)), geti(&item.clone(), &Py::Int(1))]), 2, None);
    start = __unpk2[0].clone();
    end = __unpk2[1].clone();
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); __args.push(Py::Int(0)); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "rfind", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\n")); __args.push(Py::Int(0)); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "rfind", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); __args.push(end.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "find", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\n")); __args.push(end.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "find", args))(&__args) }])) { let mut at: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&at.clone(), &op_neg(&Py::Int(1))))) {  __out.push(at.clone()); } } py_list(__out) })() };
    right = if py_truthy(&after.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    sentence = slice_or_raise(&answer.clone(), &op_add(&left.clone(), &Py::Int(1)), &right.clone(), &Py::None);
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_RATE_IN_THE_SENTENCE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&answer.clone(), &end.clone(), &op_add(&end.clone(), &Py::Int(20)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AFTER_A_RATE(), "match", args))(&__args) })) } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    }
    return kept.clone();
    Py::None
}
pub fn _the_verdict_against_a_card(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut answer: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut definite_only: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "definite_only") { v.clone() }
    else { Py::Bool(false) };
    let mut _u: Py = Py::None;
    let mut far: Py = Py::None;
    let mut high: Py = Py::None;
    let mut holds: Py = Py::None;
    let mut item: Py = Py::None;
    let mut low: Py = Py::None;
    let mut near: Py = Py::None;
    let mut over: Py = Py::None;
    let mut ranges: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut under: Py = Py::None;
    let _ = py_str("The verdict when the card's ranges say one thing about the figure, else None.\n\n    `definite_only` asks for the claim the card makes outright - inside one of its bands, or above\n    them all, or below them all - and leaves the gap between two bands to the opening sentence,\n    which is the part that prices the thing the card is about rather than what something smaller\n    costs. Without it, a boiler at 1,200 came back between the price of a boiler service and the\n    price of a boiler.\n    ");
    ranges = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _price_ranges_in(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_ranges_that_are_not_rates(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&ranges.clone()))) {
    return Py::None;
    }
    holds = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut item: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = mine.clone(); if py_truthy(&op_le(&geti(&item.clone(), &Py::Int(2)), &__cmp11)) { op_le(&__cmp11, &geti(&item.clone(), &Py::Int(3))) } else { Py::Bool(false) } }) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&holds.clone()) {
    let __unpk1 = py_unpack(&geti(&holds.clone(), &Py::Int(0)), 5, None);
    _u = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    low = __unpk1[2].clone();
    high = __unpk1[3].clone();
    spoken = __unpk1[4].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(low.clone()); __args.push(high.clone()); __args.push(spoken.clone()); __args.push(memory.clone()); __kws.extend(vec![("own".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut item: Py = __item.clone(); __out.push(op_gt(&geti(&item.clone(), &Py::Int(2)), &mine.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ranges.clone()); __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(2)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 5, None);
    _u = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    low = __unpk2[2].clone();
    high = __unpk2[3].clone();
    spoken = __unpk2[4].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(low.clone()); __args.push(high.clone()); __args.push(spoken.clone()); __args.push(memory.clone()); __kws.extend(vec![("own".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut item: Py = __item.clone(); __out.push(op_lt(&geti(&item.clone(), &Py::Int(3)), &mine.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ranges.clone()); __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(3)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 5, None);
    _u = __unpk3[0].clone();
    _u = __unpk3[1].clone();
    low = __unpk3[2].clone();
    high = __unpk3[3].clone();
    spoken = __unpk3[4].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(low.clone()); __args.push(high.clone()); __args.push(spoken.clone()); __args.push(memory.clone()); __kws.extend(vec![("own".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict(args))(&__args) };
    }
    if py_truthy(&definite_only.clone()) {
    return Py::None;
    }
    under = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_lt(&geti(&item.clone(), &Py::Int(3)), &mine.clone())) {  __out.push(item.clone()); } } py_list(__out) })() };
    over = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_gt(&geti(&item.clone(), &Py::Int(2)), &mine.clone())) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = under.clone(); if !py_truthy(&__b) { __b } else { over.clone() } }) {
    near = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(under.clone()); __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(3)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    far = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(over.clone()); __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answer = answer.clone(); let definite_only = definite_only.clone(); let far = far.clone(); let high = high.clone(); let holds = holds.clone(); let item = item.clone(); let low = low.clone(); let memory = memory.clone(); let mine = mine.clone(); let near = near.clone(); let over = over.clone(); let ranges = ranges.clone(); let said = said.clone(); let spoken = spoken.clone(); let under = under.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(2)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_le(&op_sub(&geti(&far.clone(), &Py::Int(2)), &geti(&near.clone(), &Py::Int(3))), &op_sub(&geti(&near.clone(), &Py::Int(3)), &geti(&near.clone(), &Py::Int(2))))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(geti(&near.clone(), &Py::Int(4))); __args.push(geti(&far.clone(), &Py::Int(4))); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_between_verdict(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(said.clone()); __args.push(geti(&near.clone(), &Py::Int(2))); __args.push(geti(&near.clone(), &Py::Int(3))); __args.push(geti(&near.clone(), &Py::Int(4))); __args.push(memory.clone()); __kws.extend(vec![("own".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_verdict(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.price".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_PRICE_ASK" => Some(g_A_PRICE_ASK()),
            "A_PRICE_ASK_WITH_THE_THING" => Some(g_A_PRICE_ASK_WITH_THE_THING()),
            "A_PRICE_BETWEEN" => Some(g_A_PRICE_BETWEEN()),
            "A_PRICE_INSIDE" => Some(g_A_PRICE_INSIDE()),
            "A_PRICE_OVER" => Some(g_A_PRICE_OVER()),
            "A_PRICE_SYMBOL_ONLY" => Some(g_A_PRICE_SYMBOL_ONLY()),
            "A_PRICE_UNDER" => Some(g_A_PRICE_UNDER()),
            "_AFTER_A_RATE" => Some(g__AFTER_A_RATE()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_A_MONEY_AFTER_THE_ASK" => Some(g__A_MONEY_AFTER_THE_ASK()),
            "_A_MONEY_CEILING" => Some(g__A_MONEY_CEILING()),
            "_A_PHRASE_AFTER_THE_ASK" => Some(g__A_PHRASE_AFTER_THE_ASK()),
            "_A_PRICE_FOR_THE_THING" => Some(g__A_PRICE_FOR_THE_THING()),
            "_A_RATE_IN_THE_SENTENCE" => Some(g__A_RATE_IN_THE_SENTENCE()),
            "_PRICE_RANGE" => Some(g__PRICE_RANGE()),
            "_PRICE_SYMBOL" => Some(g__PRICE_SYMBOL()),
            "_PRICE_WORD" => Some(g__PRICE_WORD()),
            "_a_bare_price_in_the_thread" => Some(py_func("_a_bare_price_in_the_thread", |args: &[Py]| _a_bare_price_in_the_thread(args))),
            "_a_price_ask" => Some(py_func("_a_price_ask", |args: &[Py]| _a_price_ask(args))),
            "_a_price_at_a_rate" => Some(py_func("_a_price_at_a_rate", |args: &[Py]| _a_price_at_a_rate(args))),
            "_a_rate_is_all_the_answer_names" => Some(py_func("_a_rate_is_all_the_answer_names", |args: &[Py]| _a_rate_is_all_the_answer_names(args))),
            "_a_second_quote_is_the_test" => Some(py_func("_a_second_quote_is_the_test", |args: &[Py]| _a_second_quote_is_the_test(args))),
            "_a_value" => Some(py_func("_a_value", |args: &[Py]| _a_value(args))),
            "_answers_in" => Some(py_func("_answers_in", |args: &[Py]| _answers_in(args))),
            "_money" => Some(py_func("_money", |args: &[Py]| _money(args))),
            "_number_tokens" => Some(py_func("_number_tokens", |args: &[Py]| _number_tokens(args))),
            "_price_ranges_in" => Some(py_func("_price_ranges_in", |args: &[Py]| _price_ranges_in(args))),
            "_prices_in" => Some(py_func("_prices_in", |args: &[Py]| _prices_in(args))),
            "_the_between_verdict" => Some(py_func("_the_between_verdict", |args: &[Py]| _the_between_verdict(args))),
            "_the_figure_in_the_range_s_style" => Some(py_func("_the_figure_in_the_range_s_style", |args: &[Py]| _the_figure_in_the_range_s_style(args))),
            "_the_money_said_back" => Some(py_func("_the_money_said_back", |args: &[Py]| _the_money_said_back(args))),
            "_the_period_they_said" => Some(py_func("_the_period_they_said", |args: &[Py]| _the_period_they_said(args))),
            "_the_price_said" => Some(py_func("_the_price_said", |args: &[Py]| _the_price_said(args))),
            "_the_price_they_gave" => Some(py_func("_the_price_they_gave", |args: &[Py]| _the_price_they_gave(args))),
            "_the_price_they_mean" => Some(py_func("_the_price_they_mean", |args: &[Py]| _the_price_they_mean(args))),
            "_the_range_that_is_about_it" => Some(py_func("_the_range_that_is_about_it", |args: &[Py]| _the_range_that_is_about_it(args))),
            "_the_ranges_that_are_not_rates" => Some(py_func("_the_ranges_that_are_not_rates", |args: &[Py]| _the_ranges_that_are_not_rates(args))),
            "_the_thing_priced" => Some(py_func("_the_thing_priced", |args: &[Py]| _the_thing_priced(args))),
            "_the_verdict" => Some(py_func("_the_verdict", |args: &[Py]| _the_verdict(args))),
            "_the_verdict_against_a_card" => Some(py_func("_the_verdict_against_a_card", |args: &[Py]| _the_verdict_against_a_card(args))),
            "judged" => Some(py_func("judged", |args: &[Py]| judged(args))),
            "judged_with_the_card" => Some(py_func("judged_with_the_card", |args: &[Py]| judged_with_the_card(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_PRICE_ASK" => s_A_PRICE_ASK(v),
            "A_PRICE_ASK_WITH_THE_THING" => s_A_PRICE_ASK_WITH_THE_THING(v),
            "A_PRICE_BETWEEN" => s_A_PRICE_BETWEEN(v),
            "A_PRICE_INSIDE" => s_A_PRICE_INSIDE(v),
            "A_PRICE_OVER" => s_A_PRICE_OVER(v),
            "A_PRICE_SYMBOL_ONLY" => s_A_PRICE_SYMBOL_ONLY(v),
            "A_PRICE_UNDER" => s_A_PRICE_UNDER(v),
            "_AFTER_A_RATE" => s__AFTER_A_RATE(v),
            "_ASK_WORDS" => s__ASK_WORDS(v),
            "_A_MONEY_AFTER_THE_ASK" => s__A_MONEY_AFTER_THE_ASK(v),
            "_A_MONEY_CEILING" => s__A_MONEY_CEILING(v),
            "_A_PHRASE_AFTER_THE_ASK" => s__A_PHRASE_AFTER_THE_ASK(v),
            "_A_PRICE_FOR_THE_THING" => s__A_PRICE_FOR_THE_THING(v),
            "_A_RATE_IN_THE_SENTENCE" => s__A_RATE_IN_THE_SENTENCE(v),
            "_PRICE_RANGE" => s__PRICE_RANGE(v),
            "_PRICE_SYMBOL" => s__PRICE_SYMBOL(v),
            "_PRICE_WORD" => s__PRICE_WORD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.price", module_obj);
    pyrt::xrefs::reg_fn("memory.price", "_money", py_func("_money", |args: &[Py]| _money(args)));
    pyrt::xrefs::reg_fn("memory.price", "_a_value", py_func("_a_value", |args: &[Py]| _a_value(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_money_said_back", py_func("_the_money_said_back", |args: &[Py]| _the_money_said_back(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_price_said", py_func("_the_price_said", |args: &[Py]| _the_price_said(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_figure_in_the_range_s_style", py_func("_the_figure_in_the_range_s_style", |args: &[Py]| _the_figure_in_the_range_s_style(args)));
    pyrt::xrefs::reg_fn("memory.price", "_number_tokens", py_func("_number_tokens", |args: &[Py]| _number_tokens(args)));
    pyrt::xrefs::reg_fn("memory.price", "_prices_in", py_func("_prices_in", |args: &[Py]| _prices_in(args)));
    pyrt::xrefs::reg_fn("memory.price", "_a_price_at_a_rate", py_func("_a_price_at_a_rate", |args: &[Py]| _a_price_at_a_rate(args)));
    pyrt::xrefs::reg_fn("memory.price", "_price_ranges_in", py_func("_price_ranges_in", |args: &[Py]| _price_ranges_in(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_price_they_gave", py_func("_the_price_they_gave", |args: &[Py]| _the_price_they_gave(args)));
    pyrt::xrefs::reg_fn("memory.price", "_a_bare_price_in_the_thread", py_func("_a_bare_price_in_the_thread", |args: &[Py]| _a_bare_price_in_the_thread(args)));
    pyrt::xrefs::reg_fn("memory.price", "_answers_in", py_func("_answers_in", |args: &[Py]| _answers_in(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_range_that_is_about_it", py_func("_the_range_that_is_about_it", |args: &[Py]| _the_range_that_is_about_it(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_between_verdict", py_func("_the_between_verdict", |args: &[Py]| _the_between_verdict(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_verdict", py_func("_the_verdict", |args: &[Py]| _the_verdict(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_thing_priced", py_func("_the_thing_priced", |args: &[Py]| _the_thing_priced(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_price_they_mean", py_func("_the_price_they_mean", |args: &[Py]| _the_price_they_mean(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_period_they_said", py_func("_the_period_they_said", |args: &[Py]| _the_period_they_said(args)));
    pyrt::xrefs::reg_fn("memory.price", "_a_price_ask", py_func("_a_price_ask", |args: &[Py]| _a_price_ask(args)));
    pyrt::xrefs::reg_fn("memory.price", "_a_rate_is_all_the_answer_names", py_func("_a_rate_is_all_the_answer_names", |args: &[Py]| _a_rate_is_all_the_answer_names(args)));
    pyrt::xrefs::reg_fn("memory.price", "_a_second_quote_is_the_test", py_func("_a_second_quote_is_the_test", |args: &[Py]| _a_second_quote_is_the_test(args)));
    pyrt::xrefs::reg_fn("memory.price", "judged", py_func("judged", |args: &[Py]| judged(args)));
    pyrt::xrefs::reg_fn("memory.price", "judged_with_the_card", py_func("judged_with_the_card", |args: &[Py]| judged_with_the_card(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_ranges_that_are_not_rates", py_func("_the_ranges_that_are_not_rates", |args: &[Py]| _the_ranges_that_are_not_rates(args)));
    pyrt::xrefs::reg_fn("memory.price", "_the_verdict_against_a_card", py_func("_the_verdict_against_a_card", |args: &[Py]| _the_verdict_against_a_card(args)));
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_ASK", g_A_PRICE_ASK, s_A_PRICE_ASK);
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_ASK_WITH_THE_THING", g_A_PRICE_ASK_WITH_THE_THING, s_A_PRICE_ASK_WITH_THE_THING);
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_BETWEEN", g_A_PRICE_BETWEEN, s_A_PRICE_BETWEEN);
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_INSIDE", g_A_PRICE_INSIDE, s_A_PRICE_INSIDE);
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_OVER", g_A_PRICE_OVER, s_A_PRICE_OVER);
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_SYMBOL_ONLY", g_A_PRICE_SYMBOL_ONLY, s_A_PRICE_SYMBOL_ONLY);
    pyrt::xrefs::reg_access("memory.price", "A_PRICE_UNDER", g_A_PRICE_UNDER, s_A_PRICE_UNDER);
    pyrt::xrefs::reg_access("memory.price", "_AFTER_A_RATE", g__AFTER_A_RATE, s__AFTER_A_RATE);
    pyrt::xrefs::reg_access("memory.price", "_ASK_WORDS", g__ASK_WORDS, s__ASK_WORDS);
    pyrt::xrefs::reg_access("memory.price", "_A_MONEY_AFTER_THE_ASK", g__A_MONEY_AFTER_THE_ASK, s__A_MONEY_AFTER_THE_ASK);
    pyrt::xrefs::reg_access("memory.price", "_A_MONEY_CEILING", g__A_MONEY_CEILING, s__A_MONEY_CEILING);
    pyrt::xrefs::reg_access("memory.price", "_A_PHRASE_AFTER_THE_ASK", g__A_PHRASE_AFTER_THE_ASK, s__A_PHRASE_AFTER_THE_ASK);
    pyrt::xrefs::reg_access("memory.price", "_A_PRICE_FOR_THE_THING", g__A_PRICE_FOR_THE_THING, s__A_PRICE_FOR_THE_THING);
    pyrt::xrefs::reg_access("memory.price", "_A_RATE_IN_THE_SENTENCE", g__A_RATE_IN_THE_SENTENCE, s__A_RATE_IN_THE_SENTENCE);
    pyrt::xrefs::reg_access("memory.price", "_PRICE_RANGE", g__PRICE_RANGE, s__PRICE_RANGE);
    pyrt::xrefs::reg_access("memory.price", "_PRICE_SYMBOL", g__PRICE_SYMBOL, s__PRICE_SYMBOL);
    pyrt::xrefs::reg_access("memory.price", "_PRICE_WORD", g__PRICE_WORD, s__PRICE_WORD);
}
