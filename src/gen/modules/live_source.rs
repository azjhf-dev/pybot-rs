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


pub static AIRPORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AIRPORT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_E: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_E_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BANK_HOLIDAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BANK_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_OPEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_SCHOOL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DRAW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DRAW_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCAL_DRAW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCAL_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTHLY_DRAW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTHLY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NATIONAL_DRAW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NATIONAL_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERM_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TICKET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TICKET_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BOUNDED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AIRPORT() -> Py { ensure_init(); AIRPORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AIRPORT(v: Py) { ensure_init(); if let Ok(mut w) = AIRPORT.write() { *w = v; } }
pub fn g_AIRPORT_LINES() -> Py { ensure_init(); AIRPORT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AIRPORT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = AIRPORT_LINES.write() { *w = v; } }
pub fn g_ASKS() -> Py { ensure_init(); ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS.write() { *w = v; } }
pub fn g_A_E() -> Py { ensure_init(); A_E.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_E(v: Py) { ensure_init(); if let Ok(mut w) = A_E.write() { *w = v; } }
pub fn g_A_E_LINES() -> Py { ensure_init(); A_E_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_E_LINES(v: Py) { ensure_init(); if let Ok(mut w) = A_E_LINES.write() { *w = v; } }
pub fn g_BANK_HOLIDAY() -> Py { ensure_init(); BANK_HOLIDAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BANK_HOLIDAY(v: Py) { ensure_init(); if let Ok(mut w) = BANK_HOLIDAY.write() { *w = v; } }
pub fn g_BANK_LINES() -> Py { ensure_init(); BANK_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BANK_LINES(v: Py) { ensure_init(); if let Ok(mut w) = BANK_LINES.write() { *w = v; } }
pub fn g_CALENDAR() -> Py { ensure_init(); CALENDAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR.write() { *w = v; } }
pub fn g_CALENDAR_NOT() -> Py { ensure_init(); CALENDAR_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_NOT(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_NOT.write() { *w = v; } }
pub fn g_CALENDAR_OPEN() -> Py { ensure_init(); CALENDAR_OPEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_OPEN(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_OPEN.write() { *w = v; } }
pub fn g_CALENDAR_PLAN() -> Py { ensure_init(); CALENDAR_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_PLAN.write() { *w = v; } }
pub fn g_CALENDAR_SCHOOL() -> Py { ensure_init(); CALENDAR_SCHOOL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_SCHOOL(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_SCHOOL.write() { *w = v; } }
pub fn g_DRAW() -> Py { ensure_init(); DRAW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DRAW(v: Py) { ensure_init(); if let Ok(mut w) = DRAW.write() { *w = v; } }
pub fn g_DRAW_ASK() -> Py { ensure_init(); DRAW_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DRAW_ASK(v: Py) { ensure_init(); if let Ok(mut w) = DRAW_ASK.write() { *w = v; } }
pub fn g_LEAD_IN() -> Py { ensure_init(); LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = LEAD_IN.write() { *w = v; } }
pub fn g_LOCAL_DRAW() -> Py { ensure_init(); LOCAL_DRAW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCAL_DRAW(v: Py) { ensure_init(); if let Ok(mut w) = LOCAL_DRAW.write() { *w = v; } }
pub fn g_LOCAL_LINES() -> Py { ensure_init(); LOCAL_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCAL_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LOCAL_LINES.write() { *w = v; } }
pub fn g_MONTHLY_DRAW() -> Py { ensure_init(); MONTHLY_DRAW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTHLY_DRAW(v: Py) { ensure_init(); if let Ok(mut w) = MONTHLY_DRAW.write() { *w = v; } }
pub fn g_MONTHLY_LINES() -> Py { ensure_init(); MONTHLY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTHLY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = MONTHLY_LINES.write() { *w = v; } }
pub fn g_NATIONAL_DRAW() -> Py { ensure_init(); NATIONAL_DRAW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NATIONAL_DRAW(v: Py) { ensure_init(); if let Ok(mut w) = NATIONAL_DRAW.write() { *w = v; } }
pub fn g_NATIONAL_LINES() -> Py { ensure_init(); NATIONAL_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NATIONAL_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NATIONAL_LINES.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g_TERM() -> Py { ensure_init(); TERM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERM(v: Py) { ensure_init(); if let Ok(mut w) = TERM.write() { *w = v; } }
pub fn g_TERM_LINES() -> Py { ensure_init(); TERM_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERM_LINES(v: Py) { ensure_init(); if let Ok(mut w) = TERM_LINES.write() { *w = v; } }
pub fn g_TICKET() -> Py { ensure_init(); TICKET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TICKET(v: Py) { ensure_init(); if let Ok(mut w) = TICKET.write() { *w = v; } }
pub fn g_TICKET_LINES() -> Py { ensure_init(); TICKET_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TICKET_LINES(v: Py) { ensure_init(); if let Ok(mut w) = TICKET_LINES.write() { *w = v; } }
pub fn g_WAIT() -> Py { ensure_init(); WAIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAIT(v: Py) { ensure_init(); if let Ok(mut w) = WAIT.write() { *w = v; } }
pub fn g__BOUNDED() -> Py { ensure_init(); _BOUNDED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BOUNDED(v: Py) { ensure_init(); if let Ok(mut w) = _BOUNDED.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A date in the world's diary, a live queue, and a ticket nobody can check (PRIORITY -998).\n\nThree shapes of line left a person with the generic search hedge - \"I'd rather not guess. A\nquick search online will get you a solid answer\", or \"Searching for it will turn up better\nsources than I can offer\". A search engine is the wrong place for any of them.\n\n- A date in the world's diary. \"is monday a bank holiday\", \"is the 26th a bank holiday\",\n  \"when does term end\", \"do we break up for half term on friday\", \"are the schools off next\n  week\", \"are schools closed next week\", \"do the schools close for half term\". Bank holidays\n  differ by UK nation and term dates differ by council, so there is no single page - but there\n  are exactly two, and their owners publish them: gov.uk's bank-holidays page, and the council's\n  or the school's term dates.\n- A live queue. \"whats the queue like at the airport\", \"how long is the wait at a and e\",\n  \"whats the wait like at casualty\", \"is casualty busy\". The airport's own boards and NHS 111\n  answer these in a second; a search returns last week's news. \"how long is the wait at a and e\"\n  was answered \"I can't check a figure I don't hold, sorry. Wikipedia has that number with its\n  source\" - a figure, offered for a waiting time.\n- A ticket nobody can check. \"did i win the lottery\", \"did my numbers come up\", \"are the lottery\n  results out\", \"did we win the raffle\", \"did my ticket win\". The numbers are on the ticket and\n  the result is with whoever ran it; the operator's app or the shop that sold it checks one in a\n  second. Sending somebody to a search means asking them to read a number off a page and compare\n  it themselves.\n\nSTYLE.md, \"Not knowing, honestly\": say in a few words what the bot cannot see, then the one\nplace that can. These lines do that instead of offering a search engine.\n\nThis is a module rather than a pool inside bot/compose/fallback.py because that file is over the\nworld's 1 MiB limit on main, so no pull request can change it. PRIORITY -998 puts this just above\nthe seed fallback (-1000) and below every module that can answer, so it only ever speaks where\nnothing else did; #answers' term-dates card and bank-holiday calculator keep every line they can\nanswer, and this never competes with them.\n\nThe words alone are never enough, which is the whole job here:\n\n- A date ask opens with the word that asks for a date, and a service word stands it down: \"does\n  the post run on a bank holiday\" and \"is the tip open on bank holidays\" are about the post and\n  the tip, not about which day it is. A school's own words are the one exception, because \"are\n  schools closed next week\" is a term date while \"is the pharmacy closed on a bank holiday\" is a\n  venue's hours.\n- A plan for the week names a date only in passing: \"things to do in half term\" and \"what can i\n  do with the kids in the school holidays\" want something to fill the days, and a page of term\n  dates is not an answer to them.\n- A statement is somebody's day: \"we go away every bank holiday\", \"the queue at the till was\n  huge\", \"the airport was mad busy yesterday\".\n- A ticket is not a question unless it is asked about: \"i won the lottery\" and \"i won 50 on a\n  scratchcard\" are news and keep their congratulations, and \"check my ticket\" could be a train.\n- A wait needs a place: \"how long should i boil an egg\" is not a queue.\n");
    s_PRIORITY(neg(&Py::Int(998)));
    s_BANK_HOLIDAY(py_tuple(vec![py_str("bank holiday"), py_str("bank holidays"), py_str("public holiday")]));
    s_TERM(py_tuple(vec![py_str("school holidays"), py_str("school holiday"), py_str("term dates"), py_str("term date"), py_str("school term"), py_str("half term"), py_str("half-term"), py_str("inset day"), py_str("when do the schools"), py_str("when do schools"), py_str("when does term"), py_str("when do the kids break up"), py_str("when do they break up"), py_str("when do we break up"), py_str("school go back"), py_str("schools go back"), py_str("go back to school"), py_str("school start"), py_str("schools start"), py_str("school finish"), py_str("schools finish"), py_str("break up for"), py_str("summer holidays start"), py_str("summer holiday start"), py_str("school break up"), py_str("schools off"), py_str("schools are off"), py_str("school off"), py_str("is school off"), py_str("schools closed"), py_str("are schools closed")]));
    s_CALENDAR(add(&g_BANK_HOLIDAY(), &g_TERM()));
    s_CALENDAR_NOT(py_tuple(vec![py_str("ticketed"), py_str("ticket"), py_str("fine"), py_str("fine'd"), py_str("parking"), py_str("booked off"), py_str("my shift"), py_str("rota"), py_str("overtime"), py_str("day rate"), py_str("post"), py_str("postman"), py_str("deliveries"), py_str("delivery"), py_str("buses"), py_str("bus"), py_str("trains"), py_str("train"), py_str("timetable"), py_str("shops"), py_str("shop"), py_str("supermarket"), py_str("open"), py_str("closed"), py_str("close"), py_str("runs"), py_str("run on"), py_str("running"), py_str("collections"), py_str("collection"), py_str("bin"), py_str("bins"), py_str("rubbish"), py_str("tips"), py_str("the tip"), py_str("banks"), py_str("the bank"), py_str("pharmacy"), py_str("dentist"), py_str("doctor"), py_str("swimming"), py_str("pool")]));
    s_CALENDAR_SCHOOL(py_tuple(vec![py_str("school"), py_str("schools"), py_str("term"), py_str("half term"), py_str("half-term"), py_str("inset day"), py_str("term time"), py_str("break up"), py_str("go back")]));
    s_CALENDAR_OPEN(stdlib_call("re", "compile", &[py_str("^(?:when|what|which|is|are|do|does|will|should|how)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:what (?:can|should|could) (?:i|we|you) do|things to do|something to do|ideas|activities|days out|day out|keep (?:the |them |my )?\\w* ?busy|places to go)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NATIONAL_DRAW(py_tuple(vec![py_str("lottery"), py_str("lotto"), py_str("euromillions"), py_str("euro millions"), py_str("thunderball"), py_str("scratchcard"), py_str("scratch card"), py_str("jackpot"), py_str("numbers come up"), py_str("my numbers"), py_str("the balls"), py_str("lotto numbers")]));
    s_TICKET(py_tuple(vec![py_str("ticket win"), py_str("tickets win"), py_str("ticket won"), py_str("tickets won"), py_str("ticket come up"), py_str("tickets come up"), py_str("ticket a winner"), py_str("tickets a winner"), py_str("my ticket"), py_str("our ticket")]));
    s_LOCAL_DRAW(py_tuple(vec![py_str("raffle"), py_str("sweepstake"), py_str("sweepstakes"), py_str("tombola"), py_str("bingo"), py_str("the draw"), py_str("prize draw"), py_str("draw ticket"), py_str("draw tickets"), py_str("the raffle"), py_str("the tombola"), py_str("meat raffle"), py_str("bonus ball")]));
    s_MONTHLY_DRAW(py_tuple(vec![py_str("postcode lottery"), py_str("premium bonds"), py_str("premium bond"), py_str("premium bond draw")]));
    s_DRAW(add(&add(&add(&g_NATIONAL_DRAW(), &g_LOCAL_DRAW()), &g_MONTHLY_DRAW()), &g_TICKET()));
    s_DRAW_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:did|do|does|are|is|have|has|any)\\b[^?]{0,40}?\\b(?:win|won|winner|winners|winning|come up|came up|result|results|numbers|draw|drawn|check)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WAIT(py_tuple(vec![py_str("queue"), py_str("queues"), py_str("wait"), py_str("waiting"), py_str("busy"), py_str("backed up"), py_str("how long")]));
    s_AIRPORT(py_tuple(vec![py_str("airport"), py_str("terminal"), py_str("security"), py_str("passport control"), py_str("check in"), py_str("check-in"), py_str("bag drop"), py_str("departures hall")]));
    s_A_E(py_tuple(vec![py_str("a and e"), py_str("a&e"), py_str("casualty"), py_str("accident and emergency"), py_str("emergency department"), py_str("minor injuries"), py_str("urgent care"), py_str("walk in centre"), py_str("walk-in centre")]));
    s_LEAD_IN(stdlib_call("re", "compile", &[py_str("^(?:ok|okay|oh|so|and|but|also|anyway|anyways|btw|right|well|hmm|um|er|like|sorry|quick one|one more|then|now|hi|hey|yeah|yep|please)[\\s,.!-]+")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS(stdlib_call("re", "compile", &[py_str("^(?:when|what|which|who|whose|where|why|how|is|are|was|were|do|does|did|can|could|will|would|should|shall|may|might|have|has|had|any|am)s?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BANK_LINES(py_tuple(vec![py_str("In England and Wales the bank holidays are 1 January, Good Friday, Easter Monday, the first and last Mondays in May, the last Monday in August, then 25 and 26 December - so only the Easter ones and this year's May and August Mondays need looking up. Scotland adds 2 January and 30 November and has no Easter Monday, and Northern Ireland adds 17 March and 12 July; gov.uk's bank-holidays page has the exact date for each nation."), py_str("The bank holidays that move are the Easter ones and the first and last Mondays in May and August; the rest are 1 January, 25 December and 26 December - in England and Wales. Scotland and Northern Ireland differ (2 January and St Andrew's Day, St Patrick's Day and the Twelfth), so I can't give one list for the UK: gov.uk's bank-holidays page has the date, nation by nation.")]));
    s_TERM_LINES(py_tuple(vec![py_str("English state schools run three terms - September to December, January to Easter, and Easter to July - with a week off in October, February and May and about six weeks in summer. The exact weeks are set locally, so the council's school-term-dates page or the school's own site has this year's."), py_str("A school year in England is three terms with a week's half term in October, February and May, two weeks at Christmas and about six weeks in summer, with the summer break starting in the second half of July. Which week is which is the council's or the school's to publish, because it differs by area - and Scotland and private schools run to their own calendar.")]));
    s_AIRPORT_LINES(py_tuple(vec![py_str("Most airlines want you there two hours before a short-haul flight and three before a long-haul one, and security is the queue that eats the time - the airport's own site and the boards in the terminal show that wait live, and the airline's app has the gate."), py_str("Two hours before a short-haul departure and three before long-haul is the check-in rule, and passport control is the queue that surprises people at peak times. The airport itself publishes both waits on its site and on the boards, because I can't see them from here.")]));
    s_A_E_LINES(py_tuple(vec![py_str("For anything that is not life-threatening, an urgent-care centre or a minor injuries unit usually sees you faster than A&E, and NHS 111 will say which one near you has room tonight as well as the A&E waits, which the hospital publishes itself."), py_str("A&E is for the things that cannot wait, and everything else moves quicker at urgent care or a minor injuries unit. Ring 111 - it knows which of them near you is quietest tonight and what the A&E wait is - because that changes by the hour and I can't see it.")]));
    s_NATIONAL_LINES(py_tuple(vec![py_str("Lotto draws on Wednesday and Saturday evenings and EuroMillions on Tuesday and Friday, so a result that is not up yet usually is not in yet. Your numbers are on the ticket: the app or the terminal in the shop that sold it checks a ticket in a second."), py_str("The draws are in the evening - Lotto on Wednesday and Saturday, EuroMillions on Tuesday and Friday - and the result goes up with the operator straight after. Checking is instant on the ticket's own app or at the shop terminal, because I can't see the draw from here.")]));
    s_LOCAL_LINES(py_tuple(vec![py_str("A raffle, a sweepstake or a tombola is the organiser's to confirm - they hold the numbers, and at a club the caller has them on the night or they go up on the board by the door. I can't see a draw I was not in, so whoever sold you the ticket is the one to ask."), py_str("There is no page to look a local draw up on: the organiser keeps the numbers, and a work or club sweepstake is called out on the night or pinned up afterwards. The seller or the organiser is the person who can check a ticket, not me.")]));
    s_TICKET_LINES(py_tuple(vec![py_str("Whether a ticket won depends on what it is for: a Lotto or EuroMillions ticket checks itself in the Lotto app or at the shop terminal, and a raffle or sweepstake ticket is the organiser's to confirm. The numbers are on the ticket either way, and I can't see the draw."), py_str("It depends on the kind of ticket - the national draws check in the Lotto app or at a shop terminal, and a raffle or a sweepstake is the organiser's to call. Your numbers are printed on it, which is the only part I could not see for you.")]));
    s_MONTHLY_LINES(py_tuple(vec![py_str("Premium Bonds are drawn at the start of each month and the Postcode Lottery through the month, and both check your numbers for you: NS&I's own site or app for the Bonds, the Postcode Lottery's site with your postcode. I can't see either draw from here."), py_str("The monthly draws are the operator's - NS&I publishes each Premium Bonds prize and its holder on its own site, and the Postcode Lottery's site takes your postcode and shows whether it has come up. That is the only place a result is real.")]));
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
    pattern = stdlib_call("re", "compile", &[add(&add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])"))]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&g__BOUNDED(), &phrase.clone(), stdlib_call("re", "compile", &[add(&add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])"))]).unwrap_or_else(|e| exc::raise(e)));
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
    return geti(&lines.clone(), &modulo(&call_builtin("sum", &[mcall(&clean.clone(), "encode", &[py_str("utf-8")])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e))));
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
            _ => None,
        }),
        set: None,
    }))
}
