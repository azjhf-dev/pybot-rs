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


thread_local! { static G_AIRPORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AIRPORT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_E: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_E_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BANK_HOLIDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BANK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_SCHOOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRAW_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCAL_DRAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCAL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHLY_DRAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHLY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NATIONAL_DRAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NATIONAL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TERM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TERM_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TICKET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TICKET_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BOUNDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.live_source: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AIRPORT() -> Py { ensure_init(); G_AIRPORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIRPORT(v: Py) { ensure_init(); G_AIRPORT.with(|c| *c.borrow_mut() = v); }
pub fn g_AIRPORT_LINES() -> Py { ensure_init(); G_AIRPORT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIRPORT_LINES(v: Py) { ensure_init(); G_AIRPORT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_E() -> Py { ensure_init(); G_A_E.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_E(v: Py) { ensure_init(); G_A_E.with(|c| *c.borrow_mut() = v); }
pub fn g_A_E_LINES() -> Py { ensure_init(); G_A_E_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_E_LINES(v: Py) { ensure_init(); G_A_E_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_BANK_HOLIDAY() -> Py { ensure_init(); G_BANK_HOLIDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BANK_HOLIDAY(v: Py) { ensure_init(); G_BANK_HOLIDAY.with(|c| *c.borrow_mut() = v); }
pub fn g_BANK_LINES() -> Py { ensure_init(); G_BANK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BANK_LINES(v: Py) { ensure_init(); G_BANK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR() -> Py { ensure_init(); G_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR(v: Py) { ensure_init(); G_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_NOT() -> Py { ensure_init(); G_CALENDAR_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_NOT(v: Py) { ensure_init(); G_CALENDAR_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_OPEN() -> Py { ensure_init(); G_CALENDAR_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_OPEN(v: Py) { ensure_init(); G_CALENDAR_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_PLAN() -> Py { ensure_init(); G_CALENDAR_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_PLAN(v: Py) { ensure_init(); G_CALENDAR_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_SCHOOL() -> Py { ensure_init(); G_CALENDAR_SCHOOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_SCHOOL(v: Py) { ensure_init(); G_CALENDAR_SCHOOL.with(|c| *c.borrow_mut() = v); }
pub fn g_DRAW() -> Py { ensure_init(); G_DRAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRAW(v: Py) { ensure_init(); G_DRAW.with(|c| *c.borrow_mut() = v); }
pub fn g_DRAW_ASK() -> Py { ensure_init(); G_DRAW_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRAW_ASK(v: Py) { ensure_init(); G_DRAW_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_IN() -> Py { ensure_init(); G_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); G_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCAL_DRAW() -> Py { ensure_init(); G_LOCAL_DRAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCAL_DRAW(v: Py) { ensure_init(); G_LOCAL_DRAW.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCAL_LINES() -> Py { ensure_init(); G_LOCAL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCAL_LINES(v: Py) { ensure_init(); G_LOCAL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHLY_DRAW() -> Py { ensure_init(); G_MONTHLY_DRAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHLY_DRAW(v: Py) { ensure_init(); G_MONTHLY_DRAW.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHLY_LINES() -> Py { ensure_init(); G_MONTHLY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHLY_LINES(v: Py) { ensure_init(); G_MONTHLY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_NATIONAL_DRAW() -> Py { ensure_init(); G_NATIONAL_DRAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NATIONAL_DRAW(v: Py) { ensure_init(); G_NATIONAL_DRAW.with(|c| *c.borrow_mut() = v); }
pub fn g_NATIONAL_LINES() -> Py { ensure_init(); G_NATIONAL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NATIONAL_LINES(v: Py) { ensure_init(); G_NATIONAL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_TERM() -> Py { ensure_init(); G_TERM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TERM(v: Py) { ensure_init(); G_TERM.with(|c| *c.borrow_mut() = v); }
pub fn g_TERM_LINES() -> Py { ensure_init(); G_TERM_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TERM_LINES(v: Py) { ensure_init(); G_TERM_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_TICKET() -> Py { ensure_init(); G_TICKET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TICKET(v: Py) { ensure_init(); G_TICKET.with(|c| *c.borrow_mut() = v); }
pub fn g_TICKET_LINES() -> Py { ensure_init(); G_TICKET_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TICKET_LINES(v: Py) { ensure_init(); G_TICKET_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WAIT() -> Py { ensure_init(); G_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAIT(v: Py) { ensure_init(); G_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g__BOUNDED() -> Py { ensure_init(); G__BOUNDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BOUNDED(v: Py) { ensure_init(); G__BOUNDED.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A date in the world's diary, a live queue, and a ticket nobody can check (PRIORITY -998).\n\nThree shapes of line left a person with the generic search hedge - \"I'd rather not guess. A\nquick search online will get you a solid answer\", or \"Searching for it will turn up better\nsources than I can offer\". A search engine is the wrong place for any of them.\n\n- A date in the world's diary. \"is monday a bank holiday\", \"is the 26th a bank holiday\",\n  \"when does term end\", \"do we break up for half term on friday\", \"are the schools off next\n  week\", \"are schools closed next week\", \"do the schools close for half term\". Bank holidays\n  differ by UK nation and term dates differ by council, so there is no single page - but there\n  are exactly two, and their owners publish them: gov.uk's bank-holidays page, and the council's\n  or the school's term dates.\n- A live queue. \"whats the queue like at the airport\", \"how long is the wait at a and e\",\n  \"whats the wait like at casualty\", \"is casualty busy\". The airport's own boards and NHS 111\n  answer these in a second; a search returns last week's news. \"how long is the wait at a and e\"\n  was answered \"I can't check a figure I don't hold, sorry. Wikipedia has that number with its\n  source\" - a figure, offered for a waiting time.\n- A ticket nobody can check. \"did i win the lottery\", \"did my numbers come up\", \"are the lottery\n  results out\", \"did we win the raffle\", \"did my ticket win\". The numbers are on the ticket and\n  the result is with whoever ran it; the operator's app or the shop that sold it checks one in a\n  second. Sending somebody to a search means asking them to read a number off a page and compare\n  it themselves.\n\nSTYLE.md, \"Not knowing, honestly\": say in a few words what the bot cannot see, then the one\nplace that can. These lines do that instead of offering a search engine.\n\nThis is a module rather than a pool inside bot/compose/fallback.py because that file is over the\nworld's 1 MiB limit on main, so no pull request can change it. PRIORITY -998 puts this just above\nthe seed fallback (-1000) and below every module that can answer, so it only ever speaks where\nnothing else did; #answers' term-dates card and bank-holiday calculator keep every line they can\nanswer, and this never competes with them.\n\nThe words alone are never enough, which is the whole job here:\n\n- A date ask opens with the word that asks for a date, and a service word stands it down: \"does\n  the post run on a bank holiday\" and \"is the tip open on bank holidays\" are about the post and\n  the tip, not about which day it is. A school's own words are the one exception, because \"are\n  schools closed next week\" is a term date while \"is the pharmacy closed on a bank holiday\" is a\n  venue's hours.\n- A plan for the week names a date only in passing: \"things to do in half term\" and \"what can i\n  do with the kids in the school holidays\" want something to fill the days, and a page of term\n  dates is not an answer to them.\n- A statement is somebody's day: \"we go away every bank holiday\", \"the queue at the till was\n  huge\", \"the airport was mad busy yesterday\".\n- A ticket is not a question unless it is asked about: \"i won the lottery\" and \"i won 50 on a\n  scratchcard\" are news and keep their congratulations, and \"check my ticket\" could be a train.\n- A wait needs a place: \"how long should i boil an egg\" is not a queue.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.PRIORITY"); }
    s_PRIORITY(op_neg(&Py::Int(998)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.BANK_HOLIDAY"); }
    s_BANK_HOLIDAY(py_tuple(vec![py_str("bank holiday"), py_str("bank holidays"), py_str("public holiday")]));
    fn __data1() -> Py { py_tuple(vec![py_str("school holidays"), py_str("school holiday"), py_str("term dates"), py_str("term date"), py_str("school term"), py_str("half term"), py_str("half-term"), py_str("inset day"), py_str("when do the schools"), py_str("when do schools"), py_str("when does term"), py_str("when do the kids break up"), py_str("when do they break up"), py_str("when do we break up"), py_str("school go back"), py_str("schools go back"), py_str("go back to school"), py_str("school start"), py_str("schools start"), py_str("school finish"), py_str("schools finish"), py_str("break up for"), py_str("summer holidays start"), py_str("summer holiday start"), py_str("school break up"), py_str("schools off"), py_str("schools are off"), py_str("school off"), py_str("is school off"), py_str("schools closed"), py_str("are schools closed")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.TERM"); }
    s_TERM(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.CALENDAR"); }
    s_CALENDAR(op_add(&g_BANK_HOLIDAY(), &g_TERM()));
    fn __data2() -> Py { py_tuple(vec![py_str("ticketed"), py_str("ticket"), py_str("fine"), py_str("fine'd"), py_str("parking"), py_str("booked off"), py_str("my shift"), py_str("rota"), py_str("overtime"), py_str("day rate"), py_str("post"), py_str("postman"), py_str("deliveries"), py_str("delivery"), py_str("buses"), py_str("bus"), py_str("trains"), py_str("train"), py_str("timetable"), py_str("shops"), py_str("shop"), py_str("supermarket"), py_str("open"), py_str("closed"), py_str("close"), py_str("runs"), py_str("run on"), py_str("running"), py_str("collections"), py_str("collection"), py_str("bin"), py_str("bins"), py_str("rubbish"), py_str("tips"), py_str("the tip"), py_str("banks"), py_str("the bank"), py_str("pharmacy"), py_str("dentist"), py_str("doctor"), py_str("swimming"), py_str("pool")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.CALENDAR_NOT"); }
    s_CALENDAR_NOT(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.CALENDAR_SCHOOL"); }
    s_CALENDAR_SCHOOL(py_tuple(vec![py_str("school"), py_str("schools"), py_str("term"), py_str("half term"), py_str("half-term"), py_str("inset day"), py_str("term time"), py_str("break up"), py_str("go back")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.CALENDAR_OPEN"); }
    s_CALENDAR_OPEN(stdlib_call("re", "compile", &[py_str("^(?:when|what|which|is|are|do|does|will|should|how)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.CALENDAR_PLAN"); }
    s_CALENDAR_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:what (?:can|should|could) (?:i|we|you) do|things to do|something to do|ideas|activities|days out|day out|keep (?:the |them |my )?\\w* ?busy|places to go)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.NATIONAL_DRAW"); }
    s_NATIONAL_DRAW(py_tuple(vec![py_str("lottery"), py_str("lotto"), py_str("euromillions"), py_str("euro millions"), py_str("thunderball"), py_str("scratchcard"), py_str("scratch card"), py_str("jackpot"), py_str("numbers come up"), py_str("my numbers"), py_str("the balls"), py_str("lotto numbers")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.TICKET"); }
    s_TICKET(py_tuple(vec![py_str("ticket win"), py_str("tickets win"), py_str("ticket won"), py_str("tickets won"), py_str("ticket come up"), py_str("tickets come up"), py_str("ticket a winner"), py_str("tickets a winner"), py_str("my ticket"), py_str("our ticket")]));
    fn __data3() -> Py { py_tuple(vec![py_str("raffle"), py_str("sweepstake"), py_str("sweepstakes"), py_str("tombola"), py_str("bingo"), py_str("the draw"), py_str("prize draw"), py_str("draw ticket"), py_str("draw tickets"), py_str("the raffle"), py_str("the tombola"), py_str("meat raffle"), py_str("bonus ball")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.LOCAL_DRAW"); }
    s_LOCAL_DRAW(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.MONTHLY_DRAW"); }
    s_MONTHLY_DRAW(py_tuple(vec![py_str("postcode lottery"), py_str("premium bonds"), py_str("premium bond"), py_str("premium bond draw")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.DRAW"); }
    s_DRAW(op_add(&op_add(&op_add(&g_NATIONAL_DRAW(), &g_LOCAL_DRAW()), &g_MONTHLY_DRAW()), &g_TICKET()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.DRAW_ASK"); }
    s_DRAW_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:did|do|does|are|is|have|has|any)\\b[^?]{0,40}?\\b(?:win|won|winner|winners|winning|come up|came up|result|results|numbers|draw|drawn|check)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.WAIT"); }
    s_WAIT(py_tuple(vec![py_str("queue"), py_str("queues"), py_str("wait"), py_str("waiting"), py_str("busy"), py_str("backed up"), py_str("how long")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.AIRPORT"); }
    s_AIRPORT(py_tuple(vec![py_str("airport"), py_str("terminal"), py_str("security"), py_str("passport control"), py_str("check in"), py_str("check-in"), py_str("bag drop"), py_str("departures hall")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.A_E"); }
    s_A_E(py_tuple(vec![py_str("a and e"), py_str("a&e"), py_str("casualty"), py_str("accident and emergency"), py_str("emergency department"), py_str("minor injuries"), py_str("urgent care"), py_str("walk in centre"), py_str("walk-in centre")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.LEAD_IN"); }
    s_LEAD_IN(stdlib_call("re", "compile", &[py_str("^(?:ok|okay|oh|so|and|but|also|anyway|anyways|btw|right|well|hmm|um|er|like|sorry|quick one|one more|then|now|hi|hey|yeah|yep|please)[\\s,.!-]+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.ASKS"); }
    s_ASKS(stdlib_call("re", "compile", &[py_str("^(?:when|what|which|who|whose|where|why|how|is|are|was|were|do|does|did|can|could|will|would|should|shall|may|might|have|has|had|any|am)s?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.BANK_LINES"); }
    s_BANK_LINES(py_tuple(vec![py_str("In England and Wales the bank holidays are 1 January, Good Friday, Easter Monday, the first and last Mondays in May, the last Monday in August, then 25 and 26 December - so only the Easter ones and this year's May and August Mondays need looking up. Scotland adds 2 January and 30 November and has no Easter Monday, and Northern Ireland adds 17 March and 12 July; gov.uk's bank-holidays page has the exact date for each nation."), py_str("The bank holidays that move are the Easter ones and the first and last Mondays in May and August; the rest are 1 January, 25 December and 26 December - in England and Wales. Scotland and Northern Ireland differ (2 January and St Andrew's Day, St Patrick's Day and the Twelfth), so I can't give one list for the UK: gov.uk's bank-holidays page has the date, nation by nation.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.TERM_LINES"); }
    s_TERM_LINES(py_tuple(vec![py_str("English state schools run three terms - September to December, January to Easter, and Easter to July - with a week off in October, February and May and about six weeks in summer. The exact weeks are set locally, so the council's school-term-dates page or the school's own site has this year's."), py_str("A school year in England is three terms with a week's half term in October, February and May, two weeks at Christmas and about six weeks in summer, with the summer break starting in the second half of July. Which week is which is the council's or the school's to publish, because it differs by area - and Scotland and private schools run to their own calendar.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.AIRPORT_LINES"); }
    s_AIRPORT_LINES(py_tuple(vec![py_str("Most airlines want you there two hours before a short-haul flight and three before a long-haul one, and security is the queue that eats the time - the airport's own site and the boards in the terminal show that wait live, and the airline's app has the gate."), py_str("Two hours before a short-haul departure and three before long-haul is the check-in rule, and passport control is the queue that surprises people at peak times. The airport itself publishes both waits on its site and on the boards, because I can't see them from here.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.A_E_LINES"); }
    s_A_E_LINES(py_tuple(vec![py_str("For anything that is not life-threatening, an urgent-care centre or a minor injuries unit usually sees you faster than A&E, and NHS 111 will say which one near you has room tonight as well as the A&E waits, which the hospital publishes itself."), py_str("A&E is for the things that cannot wait, and everything else moves quicker at urgent care or a minor injuries unit. Ring 111 - it knows which of them near you is quietest tonight and what the A&E wait is - because that changes by the hour and I can't see it.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.NATIONAL_LINES"); }
    s_NATIONAL_LINES(py_tuple(vec![py_str("Lotto draws on Wednesday and Saturday evenings and EuroMillions on Tuesday and Friday, so a result that is not up yet usually is not in yet. Your numbers are on the ticket: the app or the terminal in the shop that sold it checks a ticket in a second."), py_str("The draws are in the evening - Lotto on Wednesday and Saturday, EuroMillions on Tuesday and Friday - and the result goes up with the operator straight after. Checking is instant on the ticket's own app or at the shop terminal, because I can't see the draw from here.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.LOCAL_LINES"); }
    s_LOCAL_LINES(py_tuple(vec![py_str("A raffle, a sweepstake or a tombola is the organiser's to confirm - they hold the numbers, and at a club the caller has them on the night or they go up on the board by the door. I can't see a draw I was not in, so whoever sold you the ticket is the one to ask."), py_str("There is no page to look a local draw up on: the organiser keeps the numbers, and a work or club sweepstake is called out on the night or pinned up afterwards. The seller or the organiser is the person who can check a ticket, not me.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.TICKET_LINES"); }
    s_TICKET_LINES(py_tuple(vec![py_str("Whether a ticket won depends on what it is for: a Lotto or EuroMillions ticket checks itself in the Lotto app or at the shop terminal, and a raffle or sweepstake ticket is the organiser's to confirm. The numbers are on the ticket either way, and I can't see the draw."), py_str("It depends on the kind of ticket - the national draws check in the Lotto app or at a shop terminal, and a raffle or a sweepstake is the organiser's to call. Your numbers are printed on it, which is the only part I could not see for you.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source.MONTHLY_LINES"); }
    s_MONTHLY_LINES(py_tuple(vec![py_str("Premium Bonds are drawn at the start of each month and the Postcode Lottery through the month, and both check your numbers for you: NS&I's own site or app for the Bonds, the Postcode Lottery's site with your postcode. I can't see either draw from here."), py_str("The monthly draws are the operator's - NS&I publishes each Premium Bonds prize and its holder on its own site, and the Postcode Lottery's site takes your postcode and shows whether it has come up. That is the only place a result is real.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.live_source._BOUNDED"); }
    s__BOUNDED(py_dict(vec![]));
}
pub fn _bounded(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("A phrase matched as whole words, so \"raining\" is not \"training\".");
    pattern = mcall(&g__BOUNDED(), "get", &[phrase.clone()]);
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &Py::None))) {
    let __assigned1 = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])"))]).unwrap_or_else(|e| exc::raise(e));
    pattern = __assigned1.clone();
    let _ = py_setitem(&g__BOUNDED(), &phrase.clone(), __assigned1.clone());
    }
    return pattern.clone();
    Py::None
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&mcall(&_bounded(&[phrase.clone()]), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let _ = py_str("One of the two ways of saying it, the same one every time for the same line.");
    return geti(&lines.clone(), &op_modulo(&call_builtin("sum", &[mcall(&clean.clone(), "encode", &[py_str("utf-8")])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e))));
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _a_date_ask(args: &[Py]) -> Py {
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
    let _ = py_str("The pool for a date ask, or () when the line is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_CALENDAR_OPEN(), "match", &[_without_lead_in(&[clean.clone()])])))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&mcall(&g_CALENDAR_PLAN(), "search", &[clean.clone()])) {
    return py_tuple(vec![]);
    }
    if py_truthy(&{ let __b = _has(&[clean.clone(), g_CALENDAR_NOT()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has(&[clean.clone(), g_CALENDAR_SCHOOL()]))) } }) {
    return py_tuple(vec![]);
    }
    if py_truthy(&_has(&[clean.clone(), g_TERM()])) {
    return g_TERM_LINES();
    }
    if py_truthy(&_has(&[clean.clone(), g_BANK_HOLIDAY()])) {
    return g_BANK_LINES();
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _without_lead_in(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("The line with a spoken lead-in taken off the front, for reading the ask.");
    said = clean.clone();
    while py_truthy(&Py::Bool(true)) {
    shorter = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD_IN(), "sub", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shorter.clone(), &said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&shorter.clone(), "strip", &[]))) } }) {
    return said.clone();
    }
    said = shorter.clone();
    }
    Py::None
}
pub fn _a_draw_ask(args: &[Py]) -> Py {
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
    let _ = py_str("The pool for a ticket check, or () when the line is not one.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_has(&[clean.clone(), g_DRAW()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DRAW_ASK(), "search", &[clean.clone()]))) } }) {
    return py_tuple(vec![]);
    }
    if py_truthy(&_has(&[clean.clone(), g_MONTHLY_DRAW()])) {
    return g_MONTHLY_LINES();
    }
    if py_truthy(&{ let __b = _has(&[clean.clone(), g_LOCAL_DRAW()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has(&[clean.clone(), g_NATIONAL_DRAW()]))) } }) {
    return g_LOCAL_LINES();
    }
    if py_truthy(&_has(&[clean.clone(), g_NATIONAL_DRAW()])) {
    return g_NATIONAL_LINES();
    }
    return g_TICKET_LINES();
    Py::None
}
pub fn _a_queue(args: &[Py]) -> Py {
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
    if py_truthy(&Py::Bool(!py_truthy(&_has(&[clean.clone(), g_WAIT()])))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&_has(&[clean.clone(), g_A_E()])) {
    return g_A_E_LINES();
    }
    if py_truthy(&_has(&[clean.clone(), g_AIRPORT()])) {
    return g_AIRPORT_LINES();
    }
    return py_tuple(vec![]);
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
    let mut clean: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    clean = _clean(&[text.clone()]);
    said = _without_lead_in(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_ASKS(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?"))) } })) } }) {
    return Py::None;
    }
    lines = _a_queue(&[clean.clone()]);
    if py_truthy(&lines.clone()) {
    return _pick(&[clean.clone(), lines.clone()]);
    }
    if py_truthy(&_a_draw_ask(&[clean.clone()])) {
    return _pick(&[clean.clone(), { let __b = _a_draw_ask(&[clean.clone()]); if py_truthy(&__b) { __b } else { g_LOCAL_LINES() } }]);
    }
    lines = _a_date_ask(&[clean.clone()]);
    if py_truthy(&lines.clone()) {
    return _pick(&[clean.clone(), lines.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.live_source".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AIRPORT" => Some(g_AIRPORT()),
            "AIRPORT_LINES" => Some(g_AIRPORT_LINES()),
            "ASKS" => Some(g_ASKS()),
            "A_E" => Some(g_A_E()),
            "A_E_LINES" => Some(g_A_E_LINES()),
            "BANK_HOLIDAY" => Some(g_BANK_HOLIDAY()),
            "BANK_LINES" => Some(g_BANK_LINES()),
            "CALENDAR" => Some(g_CALENDAR()),
            "CALENDAR_NOT" => Some(g_CALENDAR_NOT()),
            "CALENDAR_OPEN" => Some(g_CALENDAR_OPEN()),
            "CALENDAR_PLAN" => Some(g_CALENDAR_PLAN()),
            "CALENDAR_SCHOOL" => Some(g_CALENDAR_SCHOOL()),
            "DRAW" => Some(g_DRAW()),
            "DRAW_ASK" => Some(g_DRAW_ASK()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "LOCAL_DRAW" => Some(g_LOCAL_DRAW()),
            "LOCAL_LINES" => Some(g_LOCAL_LINES()),
            "MONTHLY_DRAW" => Some(g_MONTHLY_DRAW()),
            "MONTHLY_LINES" => Some(g_MONTHLY_LINES()),
            "NATIONAL_DRAW" => Some(g_NATIONAL_DRAW()),
            "NATIONAL_LINES" => Some(g_NATIONAL_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TERM" => Some(g_TERM()),
            "TERM_LINES" => Some(g_TERM_LINES()),
            "TICKET" => Some(g_TICKET()),
            "TICKET_LINES" => Some(g_TICKET_LINES()),
            "WAIT" => Some(g_WAIT()),
            "_BOUNDED" => Some(g__BOUNDED()),
            "_a_date_ask" => Some(py_func("_a_date_ask", |args: &[Py]| _a_date_ask(args))),
            "_a_draw_ask" => Some(py_func("_a_draw_ask", |args: &[Py]| _a_draw_ask(args))),
            "_a_queue" => Some(py_func("_a_queue", |args: &[Py]| _a_queue(args))),
            "_bounded" => Some(py_func("_bounded", |args: &[Py]| _bounded(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_without_lead_in" => Some(py_func("_without_lead_in", |args: &[Py]| _without_lead_in(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AIRPORT" => s_AIRPORT(v),
            "AIRPORT_LINES" => s_AIRPORT_LINES(v),
            "ASKS" => s_ASKS(v),
            "A_E" => s_A_E(v),
            "A_E_LINES" => s_A_E_LINES(v),
            "BANK_HOLIDAY" => s_BANK_HOLIDAY(v),
            "BANK_LINES" => s_BANK_LINES(v),
            "CALENDAR" => s_CALENDAR(v),
            "CALENDAR_NOT" => s_CALENDAR_NOT(v),
            "CALENDAR_OPEN" => s_CALENDAR_OPEN(v),
            "CALENDAR_PLAN" => s_CALENDAR_PLAN(v),
            "CALENDAR_SCHOOL" => s_CALENDAR_SCHOOL(v),
            "DRAW" => s_DRAW(v),
            "DRAW_ASK" => s_DRAW_ASK(v),
            "LEAD_IN" => s_LEAD_IN(v),
            "LOCAL_DRAW" => s_LOCAL_DRAW(v),
            "LOCAL_LINES" => s_LOCAL_LINES(v),
            "MONTHLY_DRAW" => s_MONTHLY_DRAW(v),
            "MONTHLY_LINES" => s_MONTHLY_LINES(v),
            "NATIONAL_DRAW" => s_NATIONAL_DRAW(v),
            "NATIONAL_LINES" => s_NATIONAL_LINES(v),
            "PRIORITY" => s_PRIORITY(v),
            "TERM" => s_TERM(v),
            "TERM_LINES" => s_TERM_LINES(v),
            "TICKET" => s_TICKET(v),
            "TICKET_LINES" => s_TICKET_LINES(v),
            "WAIT" => s_WAIT(v),
            "_BOUNDED" => s__BOUNDED(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.live_source", module_obj);
    pyrt::xrefs::reg_fn("modules.live_source", "_bounded", py_func("_bounded", |args: &[Py]| _bounded(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_a_date_ask", py_func("_a_date_ask", |args: &[Py]| _a_date_ask(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_without_lead_in", py_func("_without_lead_in", |args: &[Py]| _without_lead_in(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_a_draw_ask", py_func("_a_draw_ask", |args: &[Py]| _a_draw_ask(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "_a_queue", py_func("_a_queue", |args: &[Py]| _a_queue(args)));
    pyrt::xrefs::reg_fn("modules.live_source", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.live_source", "AIRPORT", g_AIRPORT, s_AIRPORT);
    pyrt::xrefs::reg_access("modules.live_source", "AIRPORT_LINES", g_AIRPORT_LINES, s_AIRPORT_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("modules.live_source", "A_E", g_A_E, s_A_E);
    pyrt::xrefs::reg_access("modules.live_source", "A_E_LINES", g_A_E_LINES, s_A_E_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "BANK_HOLIDAY", g_BANK_HOLIDAY, s_BANK_HOLIDAY);
    pyrt::xrefs::reg_access("modules.live_source", "BANK_LINES", g_BANK_LINES, s_BANK_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "CALENDAR", g_CALENDAR, s_CALENDAR);
    pyrt::xrefs::reg_access("modules.live_source", "CALENDAR_NOT", g_CALENDAR_NOT, s_CALENDAR_NOT);
    pyrt::xrefs::reg_access("modules.live_source", "CALENDAR_OPEN", g_CALENDAR_OPEN, s_CALENDAR_OPEN);
    pyrt::xrefs::reg_access("modules.live_source", "CALENDAR_PLAN", g_CALENDAR_PLAN, s_CALENDAR_PLAN);
    pyrt::xrefs::reg_access("modules.live_source", "CALENDAR_SCHOOL", g_CALENDAR_SCHOOL, s_CALENDAR_SCHOOL);
    pyrt::xrefs::reg_access("modules.live_source", "DRAW", g_DRAW, s_DRAW);
    pyrt::xrefs::reg_access("modules.live_source", "DRAW_ASK", g_DRAW_ASK, s_DRAW_ASK);
    pyrt::xrefs::reg_access("modules.live_source", "LEAD_IN", g_LEAD_IN, s_LEAD_IN);
    pyrt::xrefs::reg_access("modules.live_source", "LOCAL_DRAW", g_LOCAL_DRAW, s_LOCAL_DRAW);
    pyrt::xrefs::reg_access("modules.live_source", "LOCAL_LINES", g_LOCAL_LINES, s_LOCAL_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "MONTHLY_DRAW", g_MONTHLY_DRAW, s_MONTHLY_DRAW);
    pyrt::xrefs::reg_access("modules.live_source", "MONTHLY_LINES", g_MONTHLY_LINES, s_MONTHLY_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "NATIONAL_DRAW", g_NATIONAL_DRAW, s_NATIONAL_DRAW);
    pyrt::xrefs::reg_access("modules.live_source", "NATIONAL_LINES", g_NATIONAL_LINES, s_NATIONAL_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.live_source", "TERM", g_TERM, s_TERM);
    pyrt::xrefs::reg_access("modules.live_source", "TERM_LINES", g_TERM_LINES, s_TERM_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "TICKET", g_TICKET, s_TICKET);
    pyrt::xrefs::reg_access("modules.live_source", "TICKET_LINES", g_TICKET_LINES, s_TICKET_LINES);
    pyrt::xrefs::reg_access("modules.live_source", "WAIT", g_WAIT, s_WAIT);
    pyrt::xrefs::reg_access("modules.live_source", "_BOUNDED", g__BOUNDED, s__BOUNDED);
}
