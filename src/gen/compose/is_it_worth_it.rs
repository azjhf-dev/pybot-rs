#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static AN_APPEAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_APPEAL_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_APPLIANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BODY_THREAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_OUT_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DEAL_THREAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FAULT_ALREADY_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIX_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LIVING_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAMED_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEW_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PAID_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PURCHASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PURCHASE_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPLACE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RESALE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SALE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SMALL_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SMALL_THING_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STUDY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUGGESTIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TRADE_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TRIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TROUBLE_THREAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_USED_MARK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VALUATION_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VISIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VISIT_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIX_OR_REPLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOING_ANYWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAST_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAND_DOWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_LIFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_APPEAL() -> Py { ensure_init(); AN_APPEAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_APPEAL(v: Py) { ensure_init(); if let Ok(mut w) = AN_APPEAL.write() { *w = v; } }
pub fn g_AN_APPEAL_REPLY() -> Py { ensure_init(); AN_APPEAL_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_APPEAL_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = AN_APPEAL_REPLY.write() { *w = v; } }
pub fn g_A_APPLIANCE() -> Py { ensure_init(); A_APPLIANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_APPLIANCE(v: Py) { ensure_init(); if let Ok(mut w) = A_APPLIANCE.write() { *w = v; } }
pub fn g_A_BODY_THREAD() -> Py { ensure_init(); A_BODY_THREAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BODY_THREAD(v: Py) { ensure_init(); if let Ok(mut w) = A_BODY_THREAD.write() { *w = v; } }
pub fn g_A_DAY_OUT_REPLY() -> Py { ensure_init(); A_DAY_OUT_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_OUT_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_OUT_REPLY.write() { *w = v; } }
pub fn g_A_DEAL_THREAD() -> Py { ensure_init(); A_DEAL_THREAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DEAL_THREAD(v: Py) { ensure_init(); if let Ok(mut w) = A_DEAL_THREAD.write() { *w = v; } }
pub fn g_A_FAULT_ALREADY_SAID() -> Py { ensure_init(); A_FAULT_ALREADY_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FAULT_ALREADY_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_FAULT_ALREADY_SAID.write() { *w = v; } }
pub fn g_A_FIX_REPLY() -> Py { ensure_init(); A_FIX_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIX_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_FIX_REPLY.write() { *w = v; } }
pub fn g_A_GOING() -> Py { ensure_init(); A_GOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GOING(v: Py) { ensure_init(); if let Ok(mut w) = A_GOING.write() { *w = v; } }
pub fn g_A_LIVING_THING() -> Py { ensure_init(); A_LIVING_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LIVING_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_LIVING_THING.write() { *w = v; } }
pub fn g_A_MINE() -> Py { ensure_init(); A_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MINE(v: Py) { ensure_init(); if let Ok(mut w) = A_MINE.write() { *w = v; } }
pub fn g_A_MONEY() -> Py { ensure_init(); A_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY.write() { *w = v; } }
pub fn g_A_NAMED_THING() -> Py { ensure_init(); A_NAMED_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_NAMED_THING.write() { *w = v; } }
pub fn g_A_NEW_PRICE() -> Py { ensure_init(); A_NEW_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEW_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = A_NEW_PRICE.write() { *w = v; } }
pub fn g_A_PAID_PERSON() -> Py { ensure_init(); A_PAID_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PAID_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = A_PAID_PERSON.write() { *w = v; } }
pub fn g_A_PURCHASE() -> Py { ensure_init(); A_PURCHASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PURCHASE(v: Py) { ensure_init(); if let Ok(mut w) = A_PURCHASE.write() { *w = v; } }
pub fn g_A_PURCHASE_REPLY() -> Py { ensure_init(); A_PURCHASE_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PURCHASE_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_PURCHASE_REPLY.write() { *w = v; } }
pub fn g_A_REPLACE_WORD() -> Py { ensure_init(); A_REPLACE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPLACE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_REPLACE_WORD.write() { *w = v; } }
pub fn g_A_REPORT() -> Py { ensure_init(); A_REPORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPORT(v: Py) { ensure_init(); if let Ok(mut w) = A_REPORT.write() { *w = v; } }
pub fn g_A_RESALE_THING() -> Py { ensure_init(); A_RESALE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RESALE_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_RESALE_THING.write() { *w = v; } }
pub fn g_A_SALE_ASK() -> Py { ensure_init(); A_SALE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SALE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_SALE_ASK.write() { *w = v; } }
pub fn g_A_SMALL_THING() -> Py { ensure_init(); A_SMALL_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SMALL_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_SMALL_THING.write() { *w = v; } }
pub fn g_A_SMALL_THING_REPLY() -> Py { ensure_init(); A_SMALL_THING_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SMALL_THING_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_SMALL_THING_REPLY.write() { *w = v; } }
pub fn g_A_STUDY() -> Py { ensure_init(); A_STUDY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STUDY(v: Py) { ensure_init(); if let Ok(mut w) = A_STUDY.write() { *w = v; } }
pub fn g_A_SUGGESTIONS() -> Py { ensure_init(); A_SUGGESTIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUGGESTIONS(v: Py) { ensure_init(); if let Ok(mut w) = A_SUGGESTIONS.write() { *w = v; } }
pub fn g_A_TRADE() -> Py { ensure_init(); A_TRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TRADE(v: Py) { ensure_init(); if let Ok(mut w) = A_TRADE.write() { *w = v; } }
pub fn g_A_TRADE_REPLY() -> Py { ensure_init(); A_TRADE_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TRADE_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_TRADE_REPLY.write() { *w = v; } }
pub fn g_A_TRIP() -> Py { ensure_init(); A_TRIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TRIP(v: Py) { ensure_init(); if let Ok(mut w) = A_TRIP.write() { *w = v; } }
pub fn g_A_TROUBLE_THREAD() -> Py { ensure_init(); A_TROUBLE_THREAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TROUBLE_THREAD(v: Py) { ensure_init(); if let Ok(mut w) = A_TROUBLE_THREAD.write() { *w = v; } }
pub fn g_A_USED_MARK() -> Py { ensure_init(); A_USED_MARK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_USED_MARK(v: Py) { ensure_init(); if let Ok(mut w) = A_USED_MARK.write() { *w = v; } }
pub fn g_A_VALUATION_REPLY() -> Py { ensure_init(); A_VALUATION_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VALUATION_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_VALUATION_REPLY.write() { *w = v; } }
pub fn g_A_VISIT() -> Py { ensure_init(); A_VISIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VISIT(v: Py) { ensure_init(); if let Ok(mut w) = A_VISIT.write() { *w = v; } }
pub fn g_A_VISIT_REPLY() -> Py { ensure_init(); A_VISIT_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VISIT_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_VISIT_REPLY.write() { *w = v; } }
pub fn g_FIX_OR_REPLACE() -> Py { ensure_init(); FIX_OR_REPLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIX_OR_REPLACE(v: Py) { ensure_init(); if let Ok(mut w) = FIX_OR_REPLACE.write() { *w = v; } }
pub fn g_GOING_ANYWAY() -> Py { ensure_init(); GOING_ANYWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOING_ANYWAY(v: Py) { ensure_init(); if let Ok(mut w) = GOING_ANYWAY.write() { *w = v; } }
pub fn g_LAST_ASK() -> Py { ensure_init(); LAST_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAST_ASK(v: Py) { ensure_init(); if let Ok(mut w) = LAST_ASK.write() { *w = v; } }
pub fn g_NOT_AN_ASK() -> Py { ensure_init(); NOT_AN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AN_ASK.write() { *w = v; } }
pub fn g_PLACE_WORD() -> Py { ensure_init(); PLACE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_WORD.write() { *w = v; } }
pub fn g_STAND_DOWN() -> Py { ensure_init(); STAND_DOWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAND_DOWN(v: Py) { ensure_init(); if let Ok(mut w) = STAND_DOWN.write() { *w = v; } }
pub fn g_THEIR_OWN_LIFE() -> Py { ensure_init(); THEIR_OWN_LIFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_LIFE(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_LIFE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("\"Is it worth it\" - the number to weigh, and the page that has the number.\n\n\"is it worth fixing\", \"is the castle worth the money\", \"are gore tex ones worth it\": the composer\nhas no card behind them, because none is a question about a fact. The person wants a verdict, and\na verdict without the price, the quote or the hours is a guess. Main answered them with the search\nhedge, which helps least of all: a search returns other people's verdicts and cannot know what the\nrepair costs in their town, or what the ticket price is.\n\nSTYLE.md's \"Not knowing, honestly\" has the shape that does help: say what the bot cannot see, name\nthe one place that has the figure, and stay with the person for the rest. So each family here says\nwhat decides it (the quote against a replacement, the price against the use, the price against how\nlong you would stay), where that number comes from, and the one question the bot can act on. No\nprice, no deadline and no verdict is written here - the bot can see none of them.\n\nIt is a last resort. `bot/modules/greeting.py` asks it after the composer and only when the\ncomposer's reply is a hedge, so a line any card answers never reaches it. It reads five shapes and\nreturns None on everything else, including a watched or read thing, a job, a course, a move, a\nrelationship, money trouble and anything medical or in a safety thread - those have readers of\ntheir own, and a wrong card here would be worse than the hedge it replaced.\n");
    s_FIX_OR_REPLACE(stdlib_call("re", "compile", &[py_str("\\bworth (?:fix(?:ing|in)|mend(?:ing|in)|repair(?:ing|in)|servic(?:ing|in)|descal(?:ing|in)|clean(?:ing|in)|a service|the service|replac(?:ing|in)(?: it| them)?|get(?:ting|tin) it (?:fixed|repaired|serviced|seen to|looked at)|a repair|the repair)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRADE(stdlib_call("re", "compile", &[py_str("\\bworth (?:getting someone (?:in|out|round)|getting a (?:plumber|electrician|builder|roofer|joiner|painter|decorator|gardener|handyman|mechanic)|getting it (?:done|put in)|having someone (?:in|out|round))\\b|\\bworth (?:getting|having) (?:my|our|the) \\w+ (?:rotated|serviced|checked|done|replaced|cleaned|fixed)\\b|\\b(?:a|the) (?:main )?(?:dealer|garage|mechanic)\\b[^.?]{0,20}\\bworth\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUGGESTIONS(stdlib_call("re", "compile", &[py_str("\\b(?:whats?|what is|what are|anything|something)\\b[^.?]{0,20}\\bworth (?:doing|seeing|a look|the trip)\\b|\\bworth (?:doing|seeing) there\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VISIT(stdlib_call("re", "compile", &[py_str("\\bworth (?:visiting|a visit|the trip|the journey|the drive|going to see|seeing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:castle|cathedral|minster|abbey|church|museum|gallery|exhibition|show|tour|zoo|safari park|theme park|aquarium|baths|spa|gardens?|park|palace|tower|ruins?|monument|attraction|distillery|brewery|stadium|theatre|theater|opera|festival)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PURCHASE(stdlib_call("re", "compile", &[py_str("\\bworth (?:it|the money|the price|buy(?:ing|in)|get(?:ting|tin)|paying (?:more|that|extra) for|the extra)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SMALL_THING(stdlib_call("re", "compile", &[py_str("\\b(?:kettle|toaster|iron|hoover|vacuum|microwave|phone|mobile|laptop|tablet|ipad|printer|blender|air fryer|coffee machine|hair dryer|hairdryer|straighteners|fan|heater|keyboard|monitor|headphones|earbuds|watch|remote|shredder|mower|strimmer|treadmill)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAMED_THING(stdlib_call("re", "compile", &[py_str("\\bworth (?:get(?:ting|tin)|buy(?:ing|in)|having|paying [^.?]{0,14}for|a|an) (?:a|an|the|some) (?!new\\b|another\\b|one\\b|it\\b)\\w+")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPLACE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:a new one|new one instead|replace|replacing|get another|bin it|throw it out)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_APPLIANCE(stdlib_call("re", "compile", &[py_str("\\b(?:boiler|washing machine|dishwasher|tumble dryer|fridge|freezer|oven|hob|central heating|radiators?|immersion|water tank|car|van|motorbike|bike|bicycle|roof|(?<!rice )(?<!slow )cooker|gutter|window|door|fence|drive|patio|damp|shower|toilet|tap|sink|chimney)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY(stdlib_call("re", "compile", &[py_str("£\\s?\\d[\\d,]*(?:\\.\\d{2})?|\\b\\d[\\d,]{2,}(?:\\.\\d{2})?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RESALE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:car|van|motorhome|campervan|caravan|bike|bicycle|motorbike|scooter|moped|guitar|violin|cello|piano|keyboard|drum ?kit|amp|amplifier|watch|jewellery|jewelry|ring|gold|silver|coins?|antiques?|furniture|sofa|table|wardrobe|console|playstation|xbox|nintendo|switch|camera|lens|drone|telescope|sewing machine|mower|ride on|tractor|boat|kayak|canoe|surfboard|phone|iphone|laptop|macbook|tablet|ipad|pc|desktop|gpu|graphics card|tools?|drill|saw|welder|compressor|generator)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MINE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_USED_MARK(stdlib_call("re", "compile", &[py_str("\\b(?:miles|mileage|second ?hand|used|pre-owned|reg|registration|\\d{4} reg|\\d+ years old|year old|old|vintage|antique)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SALE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:about right|too cheap|too much|too little|fair price|a fair|that fair|is that fair|good price|a good|should i (?:accept|take|sell)|worth)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEW_PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:paying|pay|buy|buying|get|getting|a new one|new|brand new|extra|more|deal|offer|quote|quotes|someone|trade|garage|dealer|repair|fix|service|servicing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_APPEAL(stdlib_call("re", "compile", &[py_str("\\bworth (?:appealing|challenging|disputing|arguing|taking it further|fighting)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_LIFE(stdlib_call("re", "compile", &[py_str("\\b(?:job|jobs|offer|career|course|degree|uni|university|college|apprenticeship|relationship|marriage|him|her|them|us|we|we're|being friends|staying friends|moving|move house|emigrat\\w*|quitting|going part time|full time|office|redundancy|settlement|watching|reading|the book|the film|the series|netflix)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIVING_THING(stdlib_call("re", "compile", &[py_str("\\b(?:goldfish|fish|dog|puppy|cat|kitten|rabbit|hamster|guinea pig|horse|pony|pet|reptile|lizard|snake)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STAND_DOWN(stdlib_call("re", "compile", &[py_str("\\b(?:afford|cant afford|can't afford|owe|owed|debt|debts|arrears|loan|overdraft|bleed\\w*|bloods?|pain|ache\\w*|sore|lump|rash|symptom\\w*|doctor|gp|hospital|111|999|scan|x-?ray|surgery|pregnant|baby|pension|annuity|retire\\w*|im \\d\\d already)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIX_REPLY(py_tuple(vec![py_str("Whether it is worth fixing comes down to the quote against what a replacement would cost, and a garage, a repair shop or the engineer for that trade will give you that figure for nothing. Ask them what else is likely to go in the next year or two as well - that is usually what decides it. How old is it, and what else has gone wrong lately?"), py_str("That one is the quote against a new one, and nobody can call it without the quote. Every garage, repair shop or engineer for that trade will price the work for free, and it is worth asking what they would check next while it is in. What would you do with it if you did not fix it?")]));
    s_A_SMALL_THING_REPLY(py_tuple(vec![py_str("These are the ones where the two sums sit close together: a new one off the shelf against the part or the cleaner and an hour of your own time. The shop's page has the new price, and the maker's page or a repair video for that exact model names what the part is. How old is it, and does it still do the job apart from that?"), py_str("Nothing here can price either side of that, and for something this size the two prices are usually nearer than people expect: the shelf price of a new one against the part and an hour of your own time. The shop's page has the one; the maker's page or a parts site has the other. What is it doing wrong?")]));
    s_A_DAY_OUT_REPLY(py_tuple(vec![py_str("I cannot see what is on near you, so the listings for the day are the first stop - the venue's own page has the times and whether it needs booking, and the newest reviews say whether people thought it was worth the trip. What sort of day are you after - a walk, something indoors, or food?"), py_str("Nothing here can see what is on there, and a guess at a name would be worthless. Start from what is on that day and check each place's own page for the price and the hours; the recent reviews are what tell you if it was worth the trip. Outside or indoors?")]));
    s_A_TRADE_REPLY(py_tuple(vec![py_str("That comes down to the quotes: two or three of them, in writing, for the same job, and what each one leaves out. Ask what is not included as well as what is, since that is where the cheap quote earns its money back. How big is the job?"), py_str("Nobody here can price work in your area, and one quote is a price rather than a market. Two or three written quotes for the same job, and the same question to each about what they would not do, tells you what it is worth. What needs doing?")]));
    s_A_VISIT_REPLY(py_tuple(vec![py_str("Only you can weigh that, and it comes down to the ticket price against how long you would stay. The place's own page has the price and the opening times, and the newest reviews are the ones that say how long people actually spend inside. What else is on the list that day?"), py_str("That is the price against the time, and nothing here can see either. Its own page carries the ticket price and the hours; the recent reviews say whether people were in and out in an hour. Are you going for the place itself, or to fill a day?")]));
    s_A_PURCHASE_REPLY(py_tuple(vec![py_str("Whether it is worth it comes down to the price against how much you would use it, and that part is yours. The shop's or the maker's page has the price and the spec; reviews from the last few months are the ones that say what it is like once the newness wears off. What would you use it for most?"), py_str("Nobody can price that for you from here. What decides it is the price against the use: its own page has the figure, and the recent reviews are where the faults show up. How often would you use it?")]));
    s_A_VALUATION_REPLY(py_tuple(vec![py_str("What a thing like that is worth is set by the ones that actually sold, not the ones still up for sale: condition, age and whatever needs doing all move it, and sold or completed listings for the same model are the figures that count, with a dealer's trade-in price as the other end of it. What is it, and what sort of state is it in?"), py_str("That figure is a market rather than a fact, so the ones that sold are what answers it - the same model, close to the same condition, on the sold or completed listings, and a second source such as a dealer's trade-in offer to check it against. What condition is it in?")]));
    s_AN_APPEAL_REPLY(py_tuple(vec![py_str("There is little to lose but the time, and the letter itself says how to ask for a review and by when - that date is the thing to find first. Citizens Advice will go through the ground you have for free, and they are quicker than the appeal is. What reason did they give?"), py_str("That is usually worth the letter it takes, and the decision letter says how to challenge it and by when. Citizens Advice or the scheme named at the bottom of the letter will check whether you have a ground for it, free. What did they say the reason was?")]));
    s_A_TRIP(stdlib_call("re", "compile", &[py_str("\\b(?:trip|holiday|weekend away|day out|visiting|on our way|travel(?:l)?ing to|staying in|booked (?:a|the)|half term|bank holiday|\\b(?:in|to) [a-z]+ for (?:a|the) (?:week|weekend|few days|fortnight|day)\\b|\\b(?:going|off) to [a-z]+ (?:next|this) week\\b|\\b(?:weekend|week|few days|day|break|holiday|time) (?:in|at) [a-z]|\\b(?:think(?:ing)? about|fancy|planning) (?:a|the) (?:trip|weekend|visit|day out))")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:go|goes|going|went|head(?:ing)?|off|fly|flying|drive|driving|travel\\w*|get) to (?:the |a |my |our )?([a-z][a-z']*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_ANYWAY(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("my"), py_str("our"), py_str("work"), py_str("bed"), py_str("school"), py_str("college"), py_str("uni"), py_str("town"), py_str("shop"), py_str("shops"), py_str("supermarket"), py_str("gym"), py_str("bank"), py_str("loo"), py_str("toilet"), py_str("pub"), py_str("doctor"), py_str("doctors"), py_str("dentist"), py_str("dentists"), py_str("hospital"), py_str("surgery"), py_str("pharmacy"), py_str("chemist"), py_str("tip"), py_str("dump"), py_str("office"), py_str("vets"), py_str("vet"), py_str("park"), py_str("post"), py_str("library"), py_str("council"), py_str("court"), py_str("airport"), py_str("station"), py_str("bus"), py_str("train"), py_str("sleep"), py_str("home"), py_str("sleep"), py_str("it"), py_str("them"), py_str("hell"), py_str("potty"), py_str("bath"), py_str("shower"), py_str("kitchen"), py_str("garage"), py_str("garden"), py_str("loft"), py_str("attic"), py_str("cellar"), py_str("skip"), py_str("bin"), py_str("bins"), py_str("charity"), py_str("car"), py_str("van"), py_str("motorway")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TROUBLE_THREAD(stdlib_call("re", "compile", &[py_str("\\b(?:marriage|on the rocks|fell out|not speaking|row(?:ed)? with|split up|divorce|relationship|cheating|affair|he left|she left|walked out|last legs)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BODY_THREAD(stdlib_call("re", "compile", &[py_str("\\b(?:back|knee|hip|shoulder|ankle|wrist|neck|spine|leg|arm|foot|stomach|chest|tooth|teeth|blood|skin|eye|ear|throat|joint)\\b[^.?]{0,24}\\b(?:hurt\\w*|ache\\w*|sore|pain\\w*|keeps? going|bad|injur\\w*|stiff|swollen|ill|infection|broke\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEAL_THREAD(stdlib_call("re", "compile", &[py_str("\\b(?:tariff|supplier|energy|switch(?:ing)? (?:supplier|tariff|deal)|fixed for two years|pension|annuity|retire\\w*|isa|savings account|mortgage deal)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STUDY(stdlib_call("re", "compile", &[py_str("\\b(?:degree|uni|university|masters|phd|doctorate|course|courses|college|apprenticeship|tuition|fees|a level|a levels|night class|night school|qualification|diploma|student)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PAID_PERSON(stdlib_call("re", "compile", &[py_str("\\bworth (?:getting|having|seeing|using) (?:a|an|the) (?:personal )?(?:trainer|coach|tutor|cleaner|accountant|lawyer|solicitor|dentist|plumber|electrician|carer|nanny|childminder|therapist|counsellor)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:it|is|was|itnt|isnt|isn t|wasnt|this|that|really|still|even|then|though|just|life)\\s+)*worth (?:it|the (?:bother|effort|pain|hassle)|anything)\\s*[.!]?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORT(stdlib_call("re", "compile", &[py_str("\\b(?:says?|said|reckons|thinks|told me)\\b[^.?]{0,30}\\bworth\\b|\\bnot worth\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FAULT_ALREADY_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:furred|scaled|limescale|broken|stopped|stopping|keeps (?:stopping|cutting)|cutting out|leaking|leaks|dripping|due (?:a|its) service|due a mot|making a noise|noisy|worn|worn out|faulty|patchy|not working|won't|wont|doesn't|doesnt|died|dead|stiff|sticking|squeaking)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAST_ASK(stdlib_call("re", "compile", &[py_str("\\s+(?:What needs doing|What is it doing wrong|How big is the job|What is it doing|What sort of day are you after[^?]*|Outside or indoors|How often would you use it)\\?$")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_destination(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("A place they say they are going to, and not one of the errands.");
    let __iter1 = py_iter_or_raise(&mcall(&g_A_GOING(), "findall", &[said.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_GOING_ANYWAY(), &word.clone())); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return word.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_trip_in_the_thread(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    return call_builtin("bool", &[mcall(&g_A_TRIP(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thread(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { Py::Int(2) };
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last turn or two of the person's own lines - the subject this ask is about.\n\n    Two, not four: \"my marriage is on the rocks\" then \"is it worth fixing\" is a marriage, but the\n    same thread two turns later (\"is it worth getting a rice cooker\") is a rice cooker, and a guard\n    that reads four turns keeps standing down on a subject the person has already left.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&back.clone()), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    Py::None
}
pub fn _is_it_worth_it(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut a_place: Py = Py::None;
    let mut appliance: Py = Py::None;
    let mut around: Py = Py::None;
    let mut reads_thread: Py = Py::None;
    let mut said: Py = Py::None;
    let mut small: Py = Py::None;
    let _ = py_str("The number to weigh for a \"worth it\" the composer hedged on, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::is_a_hedge(&[reply.clone()]))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_SALE_ASK(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_STAND_DOWN(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_MONEY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_NEW_PRICE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_STUDY(), "search", &[add(&add(&said.clone(), &py_str(" ")), &_the_thread(&[memory.clone()]))]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_RESALE_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_RESALE_THING(), "search", &[_the_thread(&[memory.clone()])]) } } } } } } } }) {
    return crate::gen::compose::pick(&[g_A_VALUATION_REPLY(), memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_THEIR_OWN_LIFE(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_STUDY(), "search", &[add(&add(&said.clone(), &py_str(" ")), &_the_thread(&[memory.clone()]))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_RESALE_THING(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_LIVING_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PAID_PERSON(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_AN_ASK(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_REPORT(), "search", &[said.clone()]) } } } } } }) {
    return Py::None;
    }
    around = _the_thread(&[memory.clone()]);
    if py_truthy(&{ let __b = mcall(&g_A_TROUBLE_THREAD(), "search", &[around.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_BODY_THREAD(), "search", &[around.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DEAL_THREAD(), "search", &[around.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_MONEY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_NEW_PRICE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_STUDY(), "search", &[add(&add(&said.clone(), &py_str(" ")), &_the_thread(&[memory.clone()]))]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MINE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_USED_MARK(), "search", &[said.clone()]) } } } } } } } }) {
    return crate::gen::compose::pick(&[g_A_VALUATION_REPLY(), memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_A_TRADE(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_A_TRADE_REPLY(), memory.clone(), said.clone()]);
    }
    reads_thread = Py::Bool(!py_truthy(&mcall(&g_A_NAMED_THING(), "search", &[said.clone()])));
    small = { let __b = mcall(&g_A_SMALL_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = reads_thread.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_SMALL_THING(), "search", &[around.clone()]) } } } };
    appliance = { let __b = mcall(&g_A_APPLIANCE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = reads_thread.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_APPLIANCE(), "search", &[around.clone()]) } } } };
    if py_truthy(&{ let __b = { let __b = small.clone(); if py_truthy(&__b) { __b } else { appliance.clone() } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_FIX_OR_REPLACE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PURCHASE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_REPLACE_WORD(), "search", &[said.clone()]) } } } } } }) {
    return crate::gen::compose::pick(&[if py_truthy(&small.clone()) { g_A_SMALL_THING_REPLY() } else { g_A_FIX_REPLY() }, memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_FIX_OR_REPLACE(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_A_FIX_REPLY(), memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_A_TRADE(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_A_TRADE_REPLY(), memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_AN_APPEAL(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_AN_APPEAL_REPLY(), memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_A_SUGGESTIONS(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_A_DAY_OUT_REPLY(), memory.clone(), said.clone()]);
    }
    a_place = { let __b = mcall(&g_A_VISIT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PLACE_WORD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_trip_in_the_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = reads_thread.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PLACE_WORD(), "search", &[around.clone()]); if py_truthy(&__b) { __b } else { _a_destination(&[around.clone()]) } } } } } } } } } };
    if py_truthy(&a_place.clone()) {
    return crate::gen::compose::pick(&[g_A_VISIT_REPLY(), memory.clone(), said.clone()]);
    }
    if py_truthy(&mcall(&g_A_PURCHASE(), "search", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_A_PURCHASE_REPLY(), memory.clone(), said.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _with_what_they_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut before: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut t: Py = Py::None;
    let _ = py_str("The same reply without its closing question when the person has just answered it.");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return reply.clone();
    }
    before = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&mcall(&g_A_FAULT_ALREADY_SAID(), "search", &[add(&add(&said.clone(), &py_str(" ")), &before.clone())])) {
    stripped = mcall(&g_LAST_ASK(), "sub", &[py_str(""), reply.clone()]);
    if py_truthy(&{ let __b = stripped.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&stripped.clone(), &reply.clone())); if !py_truthy(&__b) { __b } else { mcall(&stripped.clone(), "endswith", &[py_str(".")]) } } } }) {
    return stripped.clone();
    }
    }
    return reply.clone();
    Py::None
}
pub fn is_it_worth_it(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The pool's reply, minus a question the person has already answered this conversation.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return _with_what_they_said(&[_is_it_worth_it(&[text.clone(), memory.clone(), reply.clone()]), memory.clone(), said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.is_it_worth_it".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_APPEAL" => Some(g_AN_APPEAL()),
            "AN_APPEAL_REPLY" => Some(g_AN_APPEAL_REPLY()),
            "A_APPLIANCE" => Some(g_A_APPLIANCE()),
            "A_BODY_THREAD" => Some(g_A_BODY_THREAD()),
            "A_DAY_OUT_REPLY" => Some(g_A_DAY_OUT_REPLY()),
            "A_DEAL_THREAD" => Some(g_A_DEAL_THREAD()),
            "A_FAULT_ALREADY_SAID" => Some(g_A_FAULT_ALREADY_SAID()),
            "A_FIX_REPLY" => Some(g_A_FIX_REPLY()),
            "A_GOING" => Some(g_A_GOING()),
            "A_LIVING_THING" => Some(g_A_LIVING_THING()),
            "A_MINE" => Some(g_A_MINE()),
            "A_MONEY" => Some(g_A_MONEY()),
            "A_NAMED_THING" => Some(g_A_NAMED_THING()),
            "A_NEW_PRICE" => Some(g_A_NEW_PRICE()),
            "A_PAID_PERSON" => Some(g_A_PAID_PERSON()),
            "A_PURCHASE" => Some(g_A_PURCHASE()),
            "A_PURCHASE_REPLY" => Some(g_A_PURCHASE_REPLY()),
            "A_REPLACE_WORD" => Some(g_A_REPLACE_WORD()),
            "A_REPORT" => Some(g_A_REPORT()),
            "A_RESALE_THING" => Some(g_A_RESALE_THING()),
            "A_SALE_ASK" => Some(g_A_SALE_ASK()),
            "A_SMALL_THING" => Some(g_A_SMALL_THING()),
            "A_SMALL_THING_REPLY" => Some(g_A_SMALL_THING_REPLY()),
            "A_STUDY" => Some(g_A_STUDY()),
            "A_SUGGESTIONS" => Some(g_A_SUGGESTIONS()),
            "A_TRADE" => Some(g_A_TRADE()),
            "A_TRADE_REPLY" => Some(g_A_TRADE_REPLY()),
            "A_TRIP" => Some(g_A_TRIP()),
            "A_TROUBLE_THREAD" => Some(g_A_TROUBLE_THREAD()),
            "A_USED_MARK" => Some(g_A_USED_MARK()),
            "A_VALUATION_REPLY" => Some(g_A_VALUATION_REPLY()),
            "A_VISIT" => Some(g_A_VISIT()),
            "A_VISIT_REPLY" => Some(g_A_VISIT_REPLY()),
            "FIX_OR_REPLACE" => Some(g_FIX_OR_REPLACE()),
            "GOING_ANYWAY" => Some(g_GOING_ANYWAY()),
            "LAST_ASK" => Some(g_LAST_ASK()),
            "NOT_AN_ASK" => Some(g_NOT_AN_ASK()),
            "PLACE_WORD" => Some(g_PLACE_WORD()),
            "STAND_DOWN" => Some(g_STAND_DOWN()),
            "THEIR_OWN_LIFE" => Some(g_THEIR_OWN_LIFE()),
            "_a_destination" => Some(py_func("_a_destination", |args: &[Py]| _a_destination(args))),
            "_a_trip_in_the_thread" => Some(py_func("_a_trip_in_the_thread", |args: &[Py]| _a_trip_in_the_thread(args))),
            "_is_it_worth_it" => Some(py_func("_is_it_worth_it", |args: &[Py]| _is_it_worth_it(args))),
            "_the_thread" => Some(py_func("_the_thread", |args: &[Py]| _the_thread(args))),
            "_with_what_they_said" => Some(py_func("_with_what_they_said", |args: &[Py]| _with_what_they_said(args))),
            "is_it_worth_it" => Some(py_func("is_it_worth_it", |args: &[Py]| is_it_worth_it(args))),
            _ => None,
        }),
        set: None,
    }))
}
