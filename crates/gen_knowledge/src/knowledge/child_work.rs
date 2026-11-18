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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_ABROAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_ABROAD_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_AGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_GET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_HOLIDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_HOLIDAY_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_HOURS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_JOBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_JOBS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_NOT_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_OLDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_OLDER_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_PAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_PAY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_PERMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_PERMIT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_PUB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_PUB_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_TOO_YOUNG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_TOO_YOUNG_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_ABROAD() -> Py { ensure_init(); G_CHILD_WORK_ABROAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_ABROAD(v: Py) { ensure_init(); G_CHILD_WORK_ABROAD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_ABROAD_REPLY() -> Py { ensure_init(); G_CHILD_WORK_ABROAD_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_ABROAD_REPLY(v: Py) { ensure_init(); G_CHILD_WORK_ABROAD_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_AGE() -> Py { ensure_init(); G_CHILD_WORK_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_AGE(v: Py) { ensure_init(); G_CHILD_WORK_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_AGE_ASK() -> Py { ensure_init(); G_CHILD_WORK_AGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_AGE_ASK(v: Py) { ensure_init(); G_CHILD_WORK_AGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_ASK() -> Py { ensure_init(); G_CHILD_WORK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_ASK(v: Py) { ensure_init(); G_CHILD_WORK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_CHILD() -> Py { ensure_init(); G_CHILD_WORK_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_CHILD(v: Py) { ensure_init(); G_CHILD_WORK_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_GET() -> Py { ensure_init(); G_CHILD_WORK_GET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_GET(v: Py) { ensure_init(); G_CHILD_WORK_GET.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_HOLIDAY() -> Py { ensure_init(); G_CHILD_WORK_HOLIDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_HOLIDAY(v: Py) { ensure_init(); G_CHILD_WORK_HOLIDAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_HOLIDAY_REPLY() -> Py { ensure_init(); G_CHILD_WORK_HOLIDAY_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_HOLIDAY_REPLY(v: Py) { ensure_init(); G_CHILD_WORK_HOLIDAY_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_HOURS() -> Py { ensure_init(); G_CHILD_WORK_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_HOURS(v: Py) { ensure_init(); G_CHILD_WORK_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_HOURS_ASK() -> Py { ensure_init(); G_CHILD_WORK_HOURS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_HOURS_ASK(v: Py) { ensure_init(); G_CHILD_WORK_HOURS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_JOBS() -> Py { ensure_init(); G_CHILD_WORK_JOBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_JOBS(v: Py) { ensure_init(); G_CHILD_WORK_JOBS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_JOBS_ASK() -> Py { ensure_init(); G_CHILD_WORK_JOBS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_JOBS_ASK(v: Py) { ensure_init(); G_CHILD_WORK_JOBS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_NOT_AN_AGE() -> Py { ensure_init(); G_CHILD_WORK_NOT_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_NOT_AN_AGE(v: Py) { ensure_init(); G_CHILD_WORK_NOT_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_OLDER() -> Py { ensure_init(); G_CHILD_WORK_OLDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_OLDER(v: Py) { ensure_init(); G_CHILD_WORK_OLDER.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_OLDER_REPLY() -> Py { ensure_init(); G_CHILD_WORK_OLDER_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_OLDER_REPLY(v: Py) { ensure_init(); G_CHILD_WORK_OLDER_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_PAY() -> Py { ensure_init(); G_CHILD_WORK_PAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_PAY(v: Py) { ensure_init(); G_CHILD_WORK_PAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_PAY_ASK() -> Py { ensure_init(); G_CHILD_WORK_PAY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_PAY_ASK(v: Py) { ensure_init(); G_CHILD_WORK_PAY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_PERMIT() -> Py { ensure_init(); G_CHILD_WORK_PERMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_PERMIT(v: Py) { ensure_init(); G_CHILD_WORK_PERMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_PERMIT_ASK() -> Py { ensure_init(); G_CHILD_WORK_PERMIT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_PERMIT_ASK(v: Py) { ensure_init(); G_CHILD_WORK_PERMIT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_PUB() -> Py { ensure_init(); G_CHILD_WORK_PUB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_PUB(v: Py) { ensure_init(); G_CHILD_WORK_PUB.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_PUB_REPLY() -> Py { ensure_init(); G_CHILD_WORK_PUB_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_PUB_REPLY(v: Py) { ensure_init(); G_CHILD_WORK_PUB_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_TIME() -> Py { ensure_init(); G_CHILD_WORK_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_TIME(v: Py) { ensure_init(); G_CHILD_WORK_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_TIME_ASK() -> Py { ensure_init(); G_CHILD_WORK_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_TIME_ASK(v: Py) { ensure_init(); G_CHILD_WORK_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_TOO_YOUNG() -> Py { ensure_init(); G_CHILD_WORK_TOO_YOUNG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_TOO_YOUNG(v: Py) { ensure_init(); G_CHILD_WORK_TOO_YOUNG.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_TOO_YOUNG_REPLY() -> Py { ensure_init(); G_CHILD_WORK_TOO_YOUNG_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_TOO_YOUNG_REPLY(v: Py) { ensure_init(); G_CHILD_WORK_TOO_YOUNG_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORK_WORDS() -> Py { ensure_init(); G_CHILD_WORK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORK_WORDS(v: Py) { ensure_init(); G_CHILD_WORK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Child work: a young person's job, and the hours it is allowed.\n\nKeyed by the person's age and what they are asking about it - the hours, the pay, the permit, the\nlist of jobs, whether a pub is allowed - because the answer is law rather than advice, and every\nway of asking about it needs the same figures.\n\nIt is a domain of its own rather than more rows on the end of work.py: `ANSWERS` says the topic is\nwork's, so a follow-up in a work chat is answered by it, and `bot/knowledge/answers.py` finds it\nthe same way it finds every other domain. Nothing has to be added to a shared file to keep rows\nhere, which is what lets two changes to the knowledge base be merged in either order.\n");
    s_CHILD_WORK_WORDS(py_tuple(vec![py_str("work"), py_str("works"), py_str("working"), py_str("job"), py_str("jobs"), py_str("employed"), py_str("employment"), py_str("earn"), py_str("earns"), py_str("earning"), py_str("paid"), py_str("pay")]));
    s_CHILD_WORK_CHILD(py_tuple(vec![py_str("13 year old"), py_str("14 year old"), py_str("15 year old"), py_str("16 year old"), py_str("at 13"), py_str("at 14"), py_str("at 15"), py_str("at 16"), py_str("aged 13"), py_str("aged 14"), py_str("aged 15"), py_str("aged 16"), py_str("age 13"), py_str("age 14"), py_str("age 15"), py_str("age 16"), py_str("13 years"), py_str("14 years"), py_str("15 years"), py_str("16 years"), py_str("13"), py_str("14"), py_str("15"), py_str("16"), py_str("year old"), py_str("years old"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("teenager"), py_str("teenagers"), py_str("teenage"), py_str("son"), py_str("daughter"), py_str("school leaving age"), py_str("school leaver"), py_str("school leavers"), py_str("under 16"), py_str("under 18"), py_str("gcse"), py_str("what age"), py_str("how old"), py_str("old enough"), py_str("whats the youngest"), py_str("youngest")]));
    s_CHILD_WORK_ASK(py_tuple(vec![py_str("how many hours"), py_str("hours a week"), py_str("how old"), py_str("what age"), py_str("age"), py_str("youngest"), py_str("what jobs"), py_str("which jobs"), py_str("can i"), py_str("can my"), py_str("can a"), py_str("can an"), py_str("can you"), py_str("should i let"), py_str("latest"), py_str("how late"), py_str("allowed"), py_str("legal"), py_str("law"), py_str("rules"), py_str("permit"), py_str("minimum wage"), py_str("how much"), py_str("pay"), py_str("get a job"), py_str("get one"), py_str("start work"), py_str("start a job"), py_str("wants a job"), py_str("wants to work"), py_str("wants one"), py_str("is asking"), py_str("keeps asking")]));
    s_CHILD_WORK_ABROAD(stdlib_call("re", "compile", &[py_str("\\b(?:abroad|overseas|foreign country|another country|in (?:australia|america|the usa|the us|canada|new zealand|spain|france|germany|italy|ireland|dubai|europe|japan|china)|america|australia)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_GET(stdlib_call("re", "compile", &[py_str("\\b(?:how do i (?:get|find|apply)|where (?:can|do) i (?:find|get|look)|how can i (?:get|find)|job hunt|job hunting|applications?|apply)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_PUB(stdlib_call("re", "compile", &[py_str("\\b(?:pub|pubs|bar|bars|bar work|behind the bar|barman|barmaid|off licence|off-licence|betting shop|bookmaker)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_OLDER(stdlib_call("re", "compile", &[py_str("\\b(?:17|18|seventeen|eighteen)\\s*(?:year old|years old)\\b|\\b(?:17|18) year old\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_HOLIDAY(stdlib_call("re", "compile", &[py_str("\\b(?:school holidays?|holidays?|summer|half term|christmas break|easter break)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_HOURS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how many hours|hours|week|weekly|how long|how late|what time)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_TIME_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:until|till|til|how late|how early|what time|latest|finish|finishes|night|nights|evening|evenings)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_TOO_YOUNG(stdlib_call("re", "compile", &[py_str("\\b(?:6|7|8|9|10|11|12|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:year[- ]old|years old)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_JOBS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:kind of\\s+)?(?:jobs?|work)\\b|\\bjobs?\\s+(?:can|could|are|is|for)\\b|\\bwhat\\s+can\\s+(?:a|my|they|he|she)\\s+\\w*\\s*(?:year old\\s+)?do\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_PAY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:pay|pays|paid|wage|wages|earn|earns|earning|money|how much|minimum wage|rate|rates|salary)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_PERMIT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:permit|permission|licen[cs]e|allowed to work|do they need|do i need|paperwork|register)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_AGE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:age|how old|old enough|old do you have to be|too young|youngest|when can|start)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_WORK_HOURS(py_str("Twelve hours a week is the cap in term time, at 13 and 14 and again at 15 and 16. The rest of it moves with age: two hours on a school day either way, five hours on a Saturday at 13 and 14 and eight at 15 and 16, and 25 hours a week in the school holidays at 13 and 14 and 35 at 15 and 16. Nothing before 7am or after 7pm, nothing during school hours, a break after four hours, and two weeks off a year. Once school has finished at 16 the young-worker limit of 8 hours a day and 40 a week takes over."));
    s_CHILD_WORK_OLDER_REPLY(py_str("At 17 the school rules stop and the young-worker ones start: no more than 8 hours a day or 40 hours a week while under 18, a 30-minute break after four and a half hours, and no work between 10pm and 6am unless the job is one of the exceptions, such as a hospital, a bakery or a hotel. A written risk assessment is required for anyone under 18."));
    s_CHILD_WORK_HOLIDAY_REPLY(py_str("In the school holidays the cap is 25 hours a week at 13 and 14 and 35 at 15 and 16, against 12 hours a week in term time. The 7am to 7pm limit still holds, the two-week break off work each year still applies, and a 16 year old who has left school comes under the 8-hour day and 40-hour week instead."));
    s_CHILD_WORK_JOBS(py_str("Shops, cafes and restaurants, hairdressing, an office, cleaning, garden or farm work, and a paper round are what a council normally allows at 13 or 14. Not under 16: a factory, a building site, a pub or a betting shop, anything with machinery or heavy lifting, or anything that keeps them standing for hours."));
    s_CHILD_WORK_TIME(py_str("Not before 7am or after 7pm on any day, and never during school hours - 7pm is the latest finish at any age under 16, Saturday included. On top of that the weekly cap is 12 hours in term time, with 5 hours on a Saturday at 13 and 14 and 8 at 15 and 16, and 2 hours on a school day."));
    s_CHILD_WORK_TOO_YOUNG_REPLY(py_str("No part-time work before 13 in most of the UK. A few things are allowed younger - sport, modelling, acting and a paper round - and those need a licence from the council. From 13 the usual jobs are a paper round, a shop or a cafe, capped at 12 hours a week in term time and 2 hours on a school day, with a free work permit from the council."));
    s_CHILD_WORK_AGE(py_str("Thirteen is the youngest that part-time work is allowed in most of the UK, and 14 is when employers actually take people on. Under 16 the job also needs a free work permit from the local council, and the hours are capped at 12 a week in term time."));
    s_CHILD_WORK_PAY(py_str("There is no minimum wage under 16, so the rate is whatever is agreed: fix it and the hours before the first shift, and get the pay day in writing. At 16 and 17 the lower 16-17 rate applies, and the national insurance number can be applied for three months before the sixteenth birthday."));
    s_CHILD_WORK_ABROAD_REPLY(py_str("Every country sets its own age and hours, so check that country's government page rather than the UK's: most set 14 or 15 as the youngest for part-time work, cap the hours in school weeks, and require the employer to register the job. A working-holiday or student visa has its own age and hour limits on top."));
    s_CHILD_WORK_PUB_REPLY(py_str("At 16 they can work in a pub, but not behind the bar: taking orders, clearing tables and washing up are fine, and serving alcohol is only with a meal and where the licence allows it. Under 16 there is no pub work at all - and the same list rules out a factory, a building site, a betting shop and anything with machinery."));
    s_CHILD_WORK_PERMIT(py_str("Under 16 the job needs a work permit from the local council: it is free, it names the employer and the hours, and the employer usually starts it, though a parent can ask the council's child employment officer directly. Working without one is the employer's problem, not the child's."));
    s_CHILD_WORK_NOT_AN_AGE(stdlib_call("re", "compile", &[py_str("\\b1[3-6]\\s?(?:hours?|hrs?|days?|weeks?|months?|minutes?|mins?|stone|kg|kilos?|kilos|kgs?|lbs?|pounds|miles?|mph|cm|litres?|liters?|degrees?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWERS(py_tuple(vec![py_str("work")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-child-with-a-job")); __args.push(py_tuple(vec![g_CHILD_WORK_WORDS(), g_CHILD_WORK_CHILD()])); __args.push(g__child_work()); __kws.extend(vec![("ask".to_string(), g_CHILD_WORK_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("my job"), py_str("my work"), py_str("i work"), py_str("i am a"), py_str("i'm a"), py_str("career"), py_str("careers"), py_str("salary"), py_str("promotion"), py_str("boss"), py_str("manager"), py_str("colleague"), py_str("colleagues"), py_str("interview"), py_str("cv"), py_str("resign"), py_str("quit"), py_str("redundant"), py_str("redundancy"), py_str("sacked"), py_str("apprentice"), py_str("apprenticeship"), py_str("visa"), py_str("adapter"), py_str("adaptor"), py_str("plug"), py_str("sockets"), py_str("charger"), py_str("volunteer"), py_str("volunteering"), py_str("workout"), py_str("work out"), py_str("works for"), py_str("how does it work")])), ("guard".to_string(), g__a_number_is_an_age()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _child_work(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The hours, the jobs, the pay or the permit, whichever the line asks for.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g_CHILD_WORK_GET(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_CHILD_WORK_PUB(), "search", &[said.clone()])) {
    return g_CHILD_WORK_PUB_REPLY();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_OLDER(), "search", &[said.clone()])) {
    return g_CHILD_WORK_OLDER_REPLY();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_ABROAD(), "search", &[said.clone()])) {
    return g_CHILD_WORK_ABROAD_REPLY();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_TOO_YOUNG(), "search", &[said.clone()])) {
    return g_CHILD_WORK_TOO_YOUNG_REPLY();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_PERMIT_ASK(), "search", &[said.clone()])) {
    return g_CHILD_WORK_PERMIT();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_JOBS_ASK(), "search", &[said.clone()])) {
    return g_CHILD_WORK_JOBS();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_PAY_ASK(), "search", &[said.clone()])) {
    return g_CHILD_WORK_PAY();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_HOLIDAY(), "search", &[said.clone()])) {
    return g_CHILD_WORK_HOLIDAY_REPLY();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_TIME_ASK(), "search", &[said.clone()])) {
    return g_CHILD_WORK_TIME();
    }
    if py_truthy(&mcall(&g_CHILD_WORK_HOURS_ASK(), "search", &[said.clone()])) {
    return g_CHILD_WORK_HOURS();
    }
    return g_CHILD_WORK_AGE();
    Py::None
}
pub fn _a_number_is_an_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return Py::Bool(!py_truthy(&mcall(&g_CHILD_WORK_NOT_AN_AGE(), "search", &[said.clone()])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.child_work".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "CHILD_WORK_ABROAD" => Some(g_CHILD_WORK_ABROAD()),
            "CHILD_WORK_ABROAD_REPLY" => Some(g_CHILD_WORK_ABROAD_REPLY()),
            "CHILD_WORK_AGE" => Some(g_CHILD_WORK_AGE()),
            "CHILD_WORK_AGE_ASK" => Some(g_CHILD_WORK_AGE_ASK()),
            "CHILD_WORK_ASK" => Some(g_CHILD_WORK_ASK()),
            "CHILD_WORK_CHILD" => Some(g_CHILD_WORK_CHILD()),
            "CHILD_WORK_GET" => Some(g_CHILD_WORK_GET()),
            "CHILD_WORK_HOLIDAY" => Some(g_CHILD_WORK_HOLIDAY()),
            "CHILD_WORK_HOLIDAY_REPLY" => Some(g_CHILD_WORK_HOLIDAY_REPLY()),
            "CHILD_WORK_HOURS" => Some(g_CHILD_WORK_HOURS()),
            "CHILD_WORK_HOURS_ASK" => Some(g_CHILD_WORK_HOURS_ASK()),
            "CHILD_WORK_JOBS" => Some(g_CHILD_WORK_JOBS()),
            "CHILD_WORK_JOBS_ASK" => Some(g_CHILD_WORK_JOBS_ASK()),
            "CHILD_WORK_NOT_AN_AGE" => Some(g_CHILD_WORK_NOT_AN_AGE()),
            "CHILD_WORK_OLDER" => Some(g_CHILD_WORK_OLDER()),
            "CHILD_WORK_OLDER_REPLY" => Some(g_CHILD_WORK_OLDER_REPLY()),
            "CHILD_WORK_PAY" => Some(g_CHILD_WORK_PAY()),
            "CHILD_WORK_PAY_ASK" => Some(g_CHILD_WORK_PAY_ASK()),
            "CHILD_WORK_PERMIT" => Some(g_CHILD_WORK_PERMIT()),
            "CHILD_WORK_PERMIT_ASK" => Some(g_CHILD_WORK_PERMIT_ASK()),
            "CHILD_WORK_PUB" => Some(g_CHILD_WORK_PUB()),
            "CHILD_WORK_PUB_REPLY" => Some(g_CHILD_WORK_PUB_REPLY()),
            "CHILD_WORK_TIME" => Some(g_CHILD_WORK_TIME()),
            "CHILD_WORK_TIME_ASK" => Some(g_CHILD_WORK_TIME_ASK()),
            "CHILD_WORK_TOO_YOUNG" => Some(g_CHILD_WORK_TOO_YOUNG()),
            "CHILD_WORK_TOO_YOUNG_REPLY" => Some(g_CHILD_WORK_TOO_YOUNG_REPLY()),
            "CHILD_WORK_WORDS" => Some(g_CHILD_WORK_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_a_number_is_an_age" => Some(py_func("_a_number_is_an_age", |args: &[Py]| _a_number_is_an_age(args))),
            "_child_work" => Some(py_func("_child_work", |args: &[Py]| _child_work(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "CHILD_WORK_ABROAD" => s_CHILD_WORK_ABROAD(v),
            "CHILD_WORK_ABROAD_REPLY" => s_CHILD_WORK_ABROAD_REPLY(v),
            "CHILD_WORK_AGE" => s_CHILD_WORK_AGE(v),
            "CHILD_WORK_AGE_ASK" => s_CHILD_WORK_AGE_ASK(v),
            "CHILD_WORK_ASK" => s_CHILD_WORK_ASK(v),
            "CHILD_WORK_CHILD" => s_CHILD_WORK_CHILD(v),
            "CHILD_WORK_GET" => s_CHILD_WORK_GET(v),
            "CHILD_WORK_HOLIDAY" => s_CHILD_WORK_HOLIDAY(v),
            "CHILD_WORK_HOLIDAY_REPLY" => s_CHILD_WORK_HOLIDAY_REPLY(v),
            "CHILD_WORK_HOURS" => s_CHILD_WORK_HOURS(v),
            "CHILD_WORK_HOURS_ASK" => s_CHILD_WORK_HOURS_ASK(v),
            "CHILD_WORK_JOBS" => s_CHILD_WORK_JOBS(v),
            "CHILD_WORK_JOBS_ASK" => s_CHILD_WORK_JOBS_ASK(v),
            "CHILD_WORK_NOT_AN_AGE" => s_CHILD_WORK_NOT_AN_AGE(v),
            "CHILD_WORK_OLDER" => s_CHILD_WORK_OLDER(v),
            "CHILD_WORK_OLDER_REPLY" => s_CHILD_WORK_OLDER_REPLY(v),
            "CHILD_WORK_PAY" => s_CHILD_WORK_PAY(v),
            "CHILD_WORK_PAY_ASK" => s_CHILD_WORK_PAY_ASK(v),
            "CHILD_WORK_PERMIT" => s_CHILD_WORK_PERMIT(v),
            "CHILD_WORK_PERMIT_ASK" => s_CHILD_WORK_PERMIT_ASK(v),
            "CHILD_WORK_PUB" => s_CHILD_WORK_PUB(v),
            "CHILD_WORK_PUB_REPLY" => s_CHILD_WORK_PUB_REPLY(v),
            "CHILD_WORK_TIME" => s_CHILD_WORK_TIME(v),
            "CHILD_WORK_TIME_ASK" => s_CHILD_WORK_TIME_ASK(v),
            "CHILD_WORK_TOO_YOUNG" => s_CHILD_WORK_TOO_YOUNG(v),
            "CHILD_WORK_TOO_YOUNG_REPLY" => s_CHILD_WORK_TOO_YOUNG_REPLY(v),
            "CHILD_WORK_WORDS" => s_CHILD_WORK_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.child_work", module_obj);
    pyrt::xrefs::reg_fn("knowledge.child_work", "_child_work", py_func("_child_work", |args: &[Py]| _child_work(args)));
    pyrt::xrefs::reg_fn("knowledge.child_work", "_a_number_is_an_age", py_func("_a_number_is_an_age", |args: &[Py]| _a_number_is_an_age(args)));
    pyrt::xrefs::reg_access("knowledge.child_work", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_ABROAD", g_CHILD_WORK_ABROAD, s_CHILD_WORK_ABROAD);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_ABROAD_REPLY", g_CHILD_WORK_ABROAD_REPLY, s_CHILD_WORK_ABROAD_REPLY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_AGE", g_CHILD_WORK_AGE, s_CHILD_WORK_AGE);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_AGE_ASK", g_CHILD_WORK_AGE_ASK, s_CHILD_WORK_AGE_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_ASK", g_CHILD_WORK_ASK, s_CHILD_WORK_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_CHILD", g_CHILD_WORK_CHILD, s_CHILD_WORK_CHILD);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_GET", g_CHILD_WORK_GET, s_CHILD_WORK_GET);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_HOLIDAY", g_CHILD_WORK_HOLIDAY, s_CHILD_WORK_HOLIDAY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_HOLIDAY_REPLY", g_CHILD_WORK_HOLIDAY_REPLY, s_CHILD_WORK_HOLIDAY_REPLY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_HOURS", g_CHILD_WORK_HOURS, s_CHILD_WORK_HOURS);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_HOURS_ASK", g_CHILD_WORK_HOURS_ASK, s_CHILD_WORK_HOURS_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_JOBS", g_CHILD_WORK_JOBS, s_CHILD_WORK_JOBS);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_JOBS_ASK", g_CHILD_WORK_JOBS_ASK, s_CHILD_WORK_JOBS_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_NOT_AN_AGE", g_CHILD_WORK_NOT_AN_AGE, s_CHILD_WORK_NOT_AN_AGE);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_OLDER", g_CHILD_WORK_OLDER, s_CHILD_WORK_OLDER);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_OLDER_REPLY", g_CHILD_WORK_OLDER_REPLY, s_CHILD_WORK_OLDER_REPLY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_PAY", g_CHILD_WORK_PAY, s_CHILD_WORK_PAY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_PAY_ASK", g_CHILD_WORK_PAY_ASK, s_CHILD_WORK_PAY_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_PERMIT", g_CHILD_WORK_PERMIT, s_CHILD_WORK_PERMIT);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_PERMIT_ASK", g_CHILD_WORK_PERMIT_ASK, s_CHILD_WORK_PERMIT_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_PUB", g_CHILD_WORK_PUB, s_CHILD_WORK_PUB);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_PUB_REPLY", g_CHILD_WORK_PUB_REPLY, s_CHILD_WORK_PUB_REPLY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_TIME", g_CHILD_WORK_TIME, s_CHILD_WORK_TIME);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_TIME_ASK", g_CHILD_WORK_TIME_ASK, s_CHILD_WORK_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_TOO_YOUNG", g_CHILD_WORK_TOO_YOUNG, s_CHILD_WORK_TOO_YOUNG);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_TOO_YOUNG_REPLY", g_CHILD_WORK_TOO_YOUNG_REPLY, s_CHILD_WORK_TOO_YOUNG_REPLY);
    pyrt::xrefs::reg_access("knowledge.child_work", "CHILD_WORK_WORDS", g_CHILD_WORK_WORDS, s_CHILD_WORK_WORDS);
    pyrt::xrefs::reg_access("knowledge.child_work", "ENTRIES", g_ENTRIES, s_ENTRIES);
}
