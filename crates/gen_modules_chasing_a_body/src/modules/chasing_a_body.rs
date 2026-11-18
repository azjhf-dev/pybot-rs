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


thread_local! { static G_ANSWERING_SOMETHING_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEGATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_AFTER_THE_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_DID_THE_SILENT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_NOT_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_OWES_A_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_WHO_WORKS_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN_DID_THE_SILENT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN_OWES_A_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPLY_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_CONTINUATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_CHASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IVE_TRIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IVE_TRIED_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORGANISATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAISED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NEXT_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NEXT_STEP_UNNAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_SHAPE_UNNAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_TIMESCALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_TIMESCALE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WAIT_FOR_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERING_SOMETHING_ELSE() -> Py { ensure_init(); G_ANSWERING_SOMETHING_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERING_SOMETHING_ELSE(v: Py) { ensure_init(); G_ANSWERING_SOMETHING_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEGATIVE() -> Py { ensure_init(); G_A_NEGATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEGATIVE(v: Py) { ensure_init(); G_A_NEGATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_AFTER_THE_NAME() -> Py { ensure_init(); G_A_PERSON_AFTER_THE_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_AFTER_THE_NAME(v: Py) { ensure_init(); G_A_PERSON_AFTER_THE_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_DID_THE_SILENT_THING() -> Py { ensure_init(); G_A_PERSON_DID_THE_SILENT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_DID_THE_SILENT_THING(v: Py) { ensure_init(); G_A_PERSON_DID_THE_SILENT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_NOT_A_BODY() -> Py { ensure_init(); G_A_PERSON_NOT_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_NOT_A_BODY(v: Py) { ensure_init(); G_A_PERSON_NOT_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_OWES_A_REPLY() -> Py { ensure_init(); G_A_PERSON_OWES_A_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_OWES_A_REPLY(v: Py) { ensure_init(); G_A_PERSON_OWES_A_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_THEIRS() -> Py { ensure_init(); G_A_PERSON_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_THEIRS(v: Py) { ensure_init(); G_A_PERSON_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_WHO_WORKS_THERE() -> Py { ensure_init(); G_A_PERSON_WHO_WORKS_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_WHO_WORKS_THERE(v: Py) { ensure_init(); G_A_PERSON_WHO_WORKS_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN_DID_THE_SILENT_THING() -> Py { ensure_init(); G_A_PRONOUN_DID_THE_SILENT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN_DID_THE_SILENT_THING(v: Py) { ensure_init(); G_A_PRONOUN_DID_THE_SILENT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN_OWES_A_REPLY() -> Py { ensure_init(); G_A_PRONOUN_OWES_A_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN_OWES_A_REPLY(v: Py) { ensure_init(); G_A_PRONOUN_OWES_A_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPLY_VERB() -> Py { ensure_init(); G_A_REPLY_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPLY_VERB(v: Py) { ensure_init(); G_A_REPLY_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_CONTINUATION() -> Py { ensure_init(); G_BARE_CONTINUATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_CONTINUATION(v: Py) { ensure_init(); G_BARE_CONTINUATION.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_CHASE() -> Py { ensure_init(); G_HOW_TO_CHASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_CHASE(v: Py) { ensure_init(); G_HOW_TO_CHASE.with(|c| *c.borrow_mut() = v); }
pub fn g_IVE_TRIED() -> Py { ensure_init(); G_IVE_TRIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IVE_TRIED(v: Py) { ensure_init(); G_IVE_TRIED.with(|c| *c.borrow_mut() = v); }
pub fn g_IVE_TRIED_LEAD() -> Py { ensure_init(); G_IVE_TRIED_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IVE_TRIED_LEAD(v: Py) { ensure_init(); G_IVE_TRIED_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_DONE() -> Py { ensure_init(); G_NOT_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_DONE(v: Py) { ensure_init(); G_NOT_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ORGANISATIONS() -> Py { ensure_init(); G_ORGANISATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORGANISATIONS(v: Py) { ensure_init(); G_ORGANISATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RAISED() -> Py { ensure_init(); G_RAISED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAISED(v: Py) { ensure_init(); G_RAISED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NEXT_STEP() -> Py { ensure_init(); G_THE_NEXT_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NEXT_STEP(v: Py) { ensure_init(); G_THE_NEXT_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NEXT_STEP_UNNAMED() -> Py { ensure_init(); G_THE_NEXT_STEP_UNNAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NEXT_STEP_UNNAMED(v: Py) { ensure_init(); G_THE_NEXT_STEP_UNNAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PRONOUN() -> Py { ensure_init(); G_THE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PRONOUN(v: Py) { ensure_init(); G_THE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_SHAPE() -> Py { ensure_init(); G_THE_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_SHAPE(v: Py) { ensure_init(); G_THE_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_SHAPE_UNNAMED() -> Py { ensure_init(); G_THE_SHAPE_UNNAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_SHAPE_UNNAMED(v: Py) { ensure_init(); G_THE_SHAPE_UNNAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_TIMESCALE() -> Py { ensure_init(); G_THE_TIMESCALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_TIMESCALE(v: Py) { ensure_init(); G_THE_TIMESCALE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_TIMESCALE_ASK() -> Py { ensure_init(); G_THE_TIMESCALE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_TIMESCALE_ASK(v: Py) { ensure_init(); G_THE_TIMESCALE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WAIT_FOR_A_PERSON() -> Py { ensure_init(); G_THE_WAIT_FOR_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WAIT_FOR_A_PERSON(v: Py) { ensure_init(); G_THE_WAIT_FOR_A_PERSON.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Chasing an organisation that has not done the thing (PRIORITY -997).\n\nA person says the body they are dealing with has gone quiet, and the bot has nothing:\n\n    the dwp havent replied to me                  -> \"I'm listening - say more if it helps.\"\n    hmrc havent sent my rebate                    -> \"I'm listening - say more if it helps.\"\n    the gp surgery wont give me an appointment    -> \"Sorry to hear that. What would help most?\"\n    how long should i wait before i chase the council\n                                                  -> \"Citizens Advice covers this free, and a\n                                                      call beats searching.\"\n    my housing association is ignoring me         -> the search hedge\n\nNone of those is an answer. The first three are the worst of them: a person has said what is\nwrong and been asked to say more, which reads as a bot that did not understand. The last two\npoint at a charity and at a search engine, and a person who wanted to know how long to wait is\nno further on.\n\nWhat the bot can say about all of them is the shape of chasing anything, which is the same\nwhatever the body is called: what decides how fast it moves is the reference and the date, the\nask has to be in writing with what you want and by when, and the timescale and the next stage\nare published by the body itself. That is true of a council, a tax office, a surgery and a\nsupplier, so it can be said from here. What cannot be said is the person's own reference and\nwhat their page says, and that is the honest limit, said in a few words.\n\nThis is a module rather than a pool inside bot/compose/fallback.py because that file is over the\nworld's 1 MiB limit on main, so no pull request can change it. PRIORITY -997 puts it just above\nthe seed fallback (-1000) and below every module that can answer, so #answers' complaints cards\n(\"how do i complain to the council\", \"how do i complain about my energy supplier\", \"how do i\nescalate a complaint\") keep every line they can answer, and this only speaks where nothing else\ndid.\n\nThe words alone are never enough:\n\n- An organisation named is not a problem with it: \"i work for the council\", \"my sister works at\n  the jobcentre\", \"we voted labour\", \"the council tax is going up\" are somebody's day or a piece\n  of news, and a page on how to chase is not an answer to any of them.\n- What makes it a chase is that something has not happened (\"havent heard\", \"no reply\", \"still\n  waiting\", \"wont fix it\", \"ignoring me\") or that the person asks how to move it along (\"how do\n  i chase\", \"how long should i wait\", \"who do i escalate to\"). Both are read off the line, and a\n  line with neither is left to the modules that own it.\n- A question about what a thing *is* or what the rules are (\"when is the council tax due\", \"can\n  the council evict me\") is not a chase either, and #answers and the knowledge base own those.\n");
    s_PRIORITY(op_neg(&Py::Int(997)));
    s_A_PERSON_AFTER_THE_NAME(call_builtin("frozenset", &[mcall(&py_str("\n    brother sister mum mom mam dad father mother son daughter wife husband partner friend mate\n    cousin uncle aunt nephew niece boss colleague workmate neighbour neighbor stepson stepdaughter\n    flatmate roommate lodger lad girl bloke fella boyfriend girlfriend fiance fiancee\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:havent|hasnt|hadnt|ha(?:ve|s|d)\\s+not|didnt|did\\s+not|not\\s+(?:yet\\s+)?(?:replied|heard|fixed|paid|sent|done|come|turned\\s+up)|no\\s+(?:reply|response|answer|word)\\b|still\\s+(?:waiting|no|nothing)|nothing\\s+(?:back|yet|since|at\\s+all)|ignoring|ignored|wont|won't|refuse[sd]?|too\\s+long|weeks\\s+now|months\\s+now|and\\s+nothing\\b|heard\\s+nothing|nothing\\s+(?:ever\\s+)?came\\s+back|never\\s+(?:did|bothered|rang|replied|came|got\\s+back|gave|sent|turned\\s+up)|never\\s+(?:heard|came)\\s+back|no\\s+one\\s+(?:(?:has|had)\\s+)?(?:got|gotten|come)\\s+back|nobody\\s+(?:(?:has|had)\\s+)?(?:got|gotten|come)\\s+back|has\\s+not\\s+come\\s+back|have\\s+not\\s+come\\s+back|(?:nobody|no\\s+one)\\s+(?:\\w+\\s+){0,2}(?:rang|rung|phoned|called|replied|answered|bothered))")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_TO_CHASE(stdlib_call("re", "compile", &[py_str("\\b(?:chase|chasing|escalate|escalation|ombudsman|complain|complaint|follow\\s+(?:it\\s+)?up|how\\s+long\\s+(?:should|do|before|until|to\\s+wait|till)|who\\s+(?:do\\s+i|should\\s+i|can\\s+i)\\s+(?:ask|ring|call|contact|chase|escalate)|what\\s+(?:can|do|should)\\s+i\\s+do\\s+(?:about|now)|take\\s+(?:it|this)\\s+further)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("council tax")]), py_str("the council tax"), py_str("the account number on the bill")]), py_tuple(vec![py_tuple(vec![py_str("council"), py_str("local authority"), py_str("town hall")]), py_str("the council"), py_str("the reference on their letter")]), py_tuple(vec![py_tuple(vec![py_str("housing association"), py_str("housing office"), py_str("housing trust")]), py_str("the housing association"), py_str("their repair or complaint reference")]), py_tuple(vec![py_tuple(vec![py_str("landlord"), py_str("letting agent"), py_str("landlady")]), py_str("the landlord"), py_str("the date you reported it and your photos")]), py_tuple(vec![py_tuple(vec![py_str("dwp"), py_str("jobcentre"), py_str("job centre"), py_str("universal credit"), py_str("benefits office")]), py_str("the DWP"), py_str("the reference on the decision letter")]), py_tuple(vec![py_tuple(vec![py_str("hmrc"), py_str("tax office"), py_str("inland revenue")]), py_str("HMRC"), py_str("your taxpayer reference and the tax year")]), py_tuple(vec![py_tuple(vec![py_str("gp"), py_str("doctor"), py_str("surgery"), py_str("health centre"), py_str("dentist")]), py_str("the surgery"), py_str("the practice manager's name and the date you first asked")]), py_tuple(vec![py_tuple(vec![py_str("hospital"), py_str("nhs trust"), py_str("clinic")]), py_str("the hospital"), py_str("your appointment or referral reference")]), py_tuple(vec![py_tuple(vec![py_str("energy supplier"), py_str("electricity"), py_str("gas company"), py_str("water company"), py_str("water board")]), py_str("the supplier"), py_str("your account number and the date of the bill")]), py_tuple(vec![py_tuple(vec![py_str("bank"), py_str("building society")]), py_str("the bank"), py_str("the complaint reference they gave you")]), py_tuple(vec![py_tuple(vec![py_str("employer"), py_str("payroll"), py_str("hr")]), py_str("your employer"), py_str("your payslip dates and the hours you were short")]), py_tuple(vec![py_tuple(vec![py_str("school"), py_str("head teacher"), py_str("headteacher"), py_str("academy trust")]), py_str("the school"), py_str("the date you raised it and their letter")]), py_tuple(vec![py_tuple(vec![py_str("university"), py_str("college"), py_str("student finance")]), py_str("the university"), py_str("your student number and the decision reference")]), py_tuple(vec![py_tuple(vec![py_str("dvla"), py_str("passport office"), py_str("home office"), py_str("ukvi")]), py_str("the office"), py_str("the application reference on your receipt")]), py_tuple(vec![py_tuple(vec![py_str("parcel"), py_str("delivery"), py_str("courier"), py_str("postage"), py_str("the post"), py_str("my order"), py_str("the order")]), py_str("the seller"), py_str("the order number and the delivery date they gave you")])]) }
    s_ORGANISATIONS(__data1());
    s_THE_SHAPE(py_tuple(vec![py_str("Have {reference} in front of you, and put the ask in writing once: what has not happened, when you first raised it, and the day you want an answer by. {body}'s complaints page names the timescale and the next stage, usually an ombudsman after eight weeks, and that page is the part I cannot see."), py_str("First get {reference} together, then write once, with what has not happened, the date you first raised it and the day you need to hear by. {body}'s own complaints page has the timescale and the escalation route, and what it says for your case is the part I cannot see."), py_str("Start with {reference}, then write once: what has not happened, when you first raised it, and the day you need to hear by. {body}'s complaints page says how long they take and where it goes next, and that and your letter are what I cannot see from here.")]));
    s_RAISED(stdlib_call("re", "compile", &[py_str("\\b(?:complain(?:ed|ing|t)?|letters?|form|emails?|emailed|wrote|written|write|applied|application|claim(?:ed|s)?|asked|rang|phoned|called|contacted|chased|reported|requested|sent|posted|appealed|emailed\\s+them|in\\s+touch|heard\\s+from\\s+them|repairs?|refunds?|payments?|booking|appointment|orders?|delivery|deposits?|quote|invoices?|reimburs\\w*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_SHAPE_UNNAMED(py_tuple(vec![py_str("Write once with the reference they gave you, what has not happened, the date you first raised it and the day you need to hear by, and keep a copy. Past that day a complaint goes to an ombudsman, usually after eight weeks, and their own complaints page has the timescale - the part I cannot see from here."), py_str("The letter only needs the reference, what has not happened, the date you first asked and the day you want an answer by. That copy is what makes the next step easy: their complaints page names the timescale and who takes it on after it, and I cannot see that page from here."), py_str("Put it in writing once, with the reference, the dates and the day you need to hear by. Complaints usually go on to an ombudsman after eight weeks if nothing comes back, and the page that says how long they allow for this case is the one I cannot see.")]));
    s_IVE_TRIED(stdlib_call("re", "compile", &[py_str("\\b(?:i(?:ve|\\s+have)\\s+(?:rung|rang|called|phoned|emailed|written|been\\s+in|been\\s+down|asked|chased|tried)|(?:rung|rang|called|phoned|emailed)\\s+(?:them|him|her|the\\s+\\w+)|no\\s+one\\s+(?:answers|picks\\s+up|rings\\s+back)|keeps?\\s+passing\\s+me\\s+(?:round|around))")]).unwrap_or_else(|e| exc::raise(e)));
    s_IVE_TRIED_LEAD(py_tuple(vec![py_str("A call leaves nothing they have to answer, so the step that works now is one line in writing."), py_str("Phoning again will get the same answer; what counts is the ask in writing."), py_str("That is the point to stop ringing and put it in writing, because a call is not a record.")]));
    s_BARE_CONTINUATION(stdlib_call("re", "compile", &[py_str("^(?:so|ok|okay|and|but|right|well)?\\s*(?:what\\s+now|what\\s+next|now\\s+what|what\\s+do\\s+i\\s+do(?:\\s+now)?|where\\s+do\\s+i\\s+start|how\\s+do\\s+i\\s+start)\\s*[?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_TIMESCALE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b|\\bhow\\s+many\\s+(?:weeks|days|months|working\\s+days)\\b|\\bhow\\s+soon\\b|\\bwhen\\s+should\\s+i\\s+(?:chase|escalate|complain|write)\\b|\\bwait\\s+(?:before|until)\\b|\\bhow\\s+much\\s+longer\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_TIMESCALE(py_tuple(vec![py_str("Eight weeks is the usual marker: until then {body} works to its own published timescale, and after it a complaint can go to the ombudsman. Put the day you need an answer by in the letter, keep the copy, and the date you wrote down is what makes it their deadline."), py_str("Most schemes give {body} eight weeks to answer before a complaint can be escalated to an ombudsman, and its own page has the shorter figure it works to day to day. Your letter is what starts the clock, so give a date and keep a copy of it.")]));
    s_THE_NEXT_STEP(py_tuple(vec![py_str("Then it is two short paragraphs: what has not happened, the date you first raised it and the day you want an answer by, sent to the address on their letter with {reference} in it so there is a copy. If the date passes or the reply is a no, {body}'s complaints page names the next stage, which is usually an ombudsman after eight weeks."), py_str("Put {reference} at the top, say in two or three lines what has not happened and when you first asked, and ask for an answer by a named day. Keep the copy: {body}'s own page has the timescale, and the letter it sends back names who takes it on if they miss it."), py_str("Write it once, with {reference}, the date you first raised it and the day you need to hear by, and keep a copy. {body} publishes how long it should take and where a complaint goes next after that, and the date you put in the letter is what makes it their deadline.")]));
    s_THE_NEXT_STEP_UNNAMED(py_tuple(vec![py_str("Then write it once: the reference they gave you, what has not happened, the date you first raised it and the day you need to hear by, and keep a copy of it. Past that day a complaint usually goes on to an ombudsman after eight weeks, and their own page has the timescale for your case - the part I cannot see."), py_str("One short letter: the reference, what has not happened, the date you first asked and the day you want an answer by, with a copy kept. Their complaints page says how long they allow and who takes it on after that, and your copy is what proves the date you gave them.")]));
    s_A_PERSON_THEIRS(py_str("(?:best\\s+|old\\s+|oldest\\s+|closest\\s+|own\\s+|little\\s+|big\\s+|eldest\\s+|younger\\s+|school\\s+|uni\\s+|work\\s+|childhood\\s+)*(?:friends?|mates?|brothers?|sisters?|mum|mom|mam|mother|dad|father|sons?|daughters?|husbands?|wifes?|wives|partners?|boyfriends?|girlfriends?|cousins?|uncle|aunt|nephews?|nieces?|flatmates?|housemates?|roommates?|colleagues?|workmates?|neighbours?|neighbors?|ex|ex-?(?:wife|husband|partner|boyfriend|girlfriend))"));
    s_A_REPLY_VERB(py_str("(?:text(?:s|ed|ing)?|messag(?:e|es|ed|ing)|call(?:s|ed|ing)?|ring(?:s|ing)?|rang|rung|repl(?:y|ies|ied|ying)|answer(?:s|ed|ing)?|respond(?:s|ed|ing)?|pick(?:s|ed|ing)?\\s+up|ignor(?:e|es|ed|ing)|open(?:s|ed|ing)?|writ(?:e|es|ten|ing)?|wrote|(?:be|been)\\s+in\\s+touch|(?:come|came|get|gets|getting|got)\\s+back)"));
    s_A_NEGATIVE(py_str("(?:hasnt|havent|hadnt|has\\s+not|have\\s+not|didnt|did\\s+not|doesnt|does\\s+not|isnt|isn't|is\\s+not|wasnt|wasn't|was\\s+not|never|wont|won't|refuses?\\s+to)"));
    s_A_PERSON_OWES_A_REPLY(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?:\\w+\\s+){0,1}"), &g_A_PERSON_THEIRS()), &py_str("\\b[^.?!]{0,40}?\\b")), &g_A_NEGATIVE()), &py_str("\\s+(?:\\w+\\s+){0,2}")), &g_A_REPLY_VERB()), &py_str("\\b(?:\\s+(?:me|us|back)){0,2}")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PRONOUN(py_str("(?:(?:he|she|they)(?:s|'s|re|'re|ve|'ve)?)"));
    s_A_PRONOUN_OWES_A_REPLY(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^\\W*(?:(?:so|and|but|then|ok|okay|well)\\s+)*"), &g_THE_PRONOUN()), &py_str("\\b[^.?!]{0,30}?\\b")), &py_str("(?:")), &g_A_NEGATIVE()), &py_str("|\\bnot)\\s+(?:\\w+\\s+){0,2}")), &g_A_REPLY_VERB()), &py_str("\\b(?:\\s+(?:me|us|back)){0,2}")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PRONOUN_DID_THE_SILENT_THING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:(?:so|and|but|then|ok|okay|well)\\s+)*"), &g_THE_PRONOUN()), &py_str("\\b[^.?!]{0,30}?\\b(?:ignor(?:ed|es)|ghost(?:ed|s)|blank(?:ed|s)\\s+me|left\\s+me\\s+on\\s+read|gave\\s+me\\s+the\\s+silent\\s+treatment)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWERING_SOMETHING_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+)?(?:door|doorbell|buzzer|intercom|gate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_DID_THE_SILENT_THING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?:\\w+\\s+){0,1}"), &g_A_PERSON_THEIRS()), &py_str("\\b[^.?!]{0,30}?\\b(?:ignor(?:ed|es)|ghost(?:ed|s)|blank(?:ed|s)\\s+me|left\\s+me\\s+on\\s+read|gave\\s+me\\s+the\\s+silent\\s+treatment)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WAIT_FOR_A_PERSON(py_tuple(vec![py_str("No word back, and the waiting is the worst of it. One plain message - \"no rush, just checking you're ok\" - beats three follow-ups, and it is easy to answer, which matters more than saying it perfectly."), py_str("A silence like that does the talking on its own, so it is worth one clear message rather than guessing: say you would like to hear from them and that there is no pressure, then leave the ball with them. A first nudge reads as care and a third reads as a chase."), py_str("Waiting on somebody to answer is its own kind of tired. Send one message that is easy to answer - no list of complaints, no \"we need to talk\" - and then let a day or two pass before you read anything into it: people go quiet for reasons that have nothing to do with you.")]));
    s_A_PERSON_WHO_WORKS_THERE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?:\\w+\\s+){0,1}"), &g_A_PERSON_THEIRS()), &py_str("\\b[^.?!]{0,25}?\\b(?:works?|working|worked|is\\s+employed|has\\s+a\\s+job|got\\s+a\\s+job|does\\s+a\\s+job)\\s+(?:for|at|in|with)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_NOT_A_BODY(stdlib_call("re", "compile", &[py_str("\\b(?:brother|sister|mum|mom|mam|dad|father|mother|son|daughter|wife|husband|partner|friend|mate|cousin|uncle|aunt|nephew|niece|boss|colleague|neighbour|neighbor|stepson|stepdaughter|flatmate|roommate|landlady|lad)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_shape_unnamed(args: &[Py]) -> Py {
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
    let mut lowered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut fallback: Py = Py::None;
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    return mcall(&fallback.clone(), "_choose", &[g_THE_SHAPE_UNNAMED(), memory.clone(), lowered.clone()]);
    Py::None
}
pub fn _the_person_before(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether one of the person's own people was named in the last couple of turns.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&said.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:my|our)\\s+"), &g_A_PERSON_THEIRS()), &py_str("\\b")), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_wait_for_a_person(args: &[Py]) -> Py {
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
    let mut lowered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut fallback: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The wait answer for a friend who has not replied. None if nothing says they owe one.");
    if py_truthy(&mcall(&g_A_PERSON_OWES_A_REPLY(), "search", &[lowered.clone()])) {
    said = mcall(&g_A_PERSON_OWES_A_REPLY(), "search", &[lowered.clone()]);
    } else {
    if py_truthy(&mcall(&g_A_PERSON_DID_THE_SILENT_THING(), "search", &[lowered.clone()])) {
    said = mcall(&g_A_PERSON_DID_THE_SILENT_THING(), "search", &[lowered.clone()]);
    } else {
    if py_truthy(&{ let __b = mcall(&g_A_PRONOUN_DID_THE_SILENT_THING(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { _the_person_before(&[memory.clone()]) } }) {
    said = mcall(&g_A_PRONOUN_DID_THE_SILENT_THING(), "search", &[lowered.clone()]);
    } else {
    if py_truthy(&{ let __b = mcall(&g_A_PRONOUN_OWES_A_REPLY(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { _the_person_before(&[memory.clone()]) } }) {
    said = mcall(&g_A_PRONOUN_OWES_A_REPLY(), "search", &[lowered.clone()]);
    } else {
    return Py::None;
    }
    }
    }
    }
    if py_truthy(&mcall(&g_ANSWERING_SOMETHING_ELSE(), "search", &[lowered.clone()])) {
    return Py::None;
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    return mcall(&fallback.clone(), "_choose", &[g_THE_WAIT_FOR_A_PERSON(), memory.clone(), lowered.clone()]);
    Py::None
}
pub fn _was_an_unnamed_chase(args: &[Py]) -> Py {
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
    let mut current: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "current") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: current"))) };
    let mut line: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the line before this one was a silence with no organisation named in it.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&line.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&mcall(&line.clone(), "strip", &[]), "lower", &[]), &current.clone())) } }) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    lowered = mcall(&py_str(" "), "join", &[mcall(&mcall(&geti(&said.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "split", &[])]);
    return { let __b = Py::Bool(py_is(&_the_body(&[lowered.clone(), _words(&[lowered.clone()])]), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_names_its_own_person(&[lowered.clone()]))); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = mcall(&g_RAISED(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_DONE(), "search", &[lowered.clone()]) } }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    return call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_body(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lowered: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut owner: Py = Py::None;
    let mut reference: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trigger: Py = Py::None;
    let mut triggers: Py = Py::None;
    let _ = py_str("The organisation the line names, and the reference it goes by.");
    if py_truthy(&mcall(&g_A_PERSON_WHO_WORKS_THERE(), "search", &[lowered.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_ORGANISATIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    triggers = __unpk2[0].clone();
    said = __unpk2[1].clone();
    reference = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&triggers.clone());
    for __item in __iter3 {
    trigger = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&trigger.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&lowered.clone(), &trigger.clone()))) {
    return py_tuple(vec![said.clone(), reference.clone()]);
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &trigger.clone()))) {
    owner = _the_person_it_belongs_to(&[lowered.clone(), trigger.clone()]);
    if py_truthy(&owner.clone()) {
    continue;
    }
    return py_tuple(vec![said.clone(), reference.clone()]);
    }
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _names_its_own_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lowered: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let _ = py_str("Whether the line is about one of the person's own people rather than an organisation.");
    return call_builtin("bool", &[mcall(&g_A_PERSON_NOT_A_BODY(), "search", &[lowered.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_person_it_belongs_to(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lowered: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut trigger: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trigger") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: trigger"))) };
    let mut after: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The person's own relation said straight after the body's name (\"doctor brother\"), or None.\n\n    Only the two words after the trigger are read, so \"the bank hasnt rung my sister back\" is still\n    the bank: the relation has to sit where the body's name is, which is the noun a person puts\n    after it to say whose it is (\"bank manager mum\", \"hospital doctor mate of mine\").\n    ");
    found = stdlib_call("re", "search", &[op_modulo(&py_str("\\b%s\\s+(?:(\\w+)\\s+)?(\\w+)"), &stdlib_call("re", "escape", &[trigger.clone()]).unwrap_or_else(|e| exc::raise(e))), lowered.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&found.clone(), "groups", &[]));
    for __item in __iter1 {
    after = __item.clone();
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PERSON_AFTER_THE_NAME(), &after.clone())) } }) {
    return after.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_body_before(args: &[Py]) -> Py {
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
    let mut current: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "current") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: current"))) };
    let mut body: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The body a follow-up is about, from the person's own lines before this one.\n\n    The current line is skipped by its text rather than by position: whether the memory already\n    holds this turn when a module is asked depends on where in the turn the module runs, and\n    dropping the last entry either way loses the body when it has not been stored yet.\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&line.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&mcall(&line.clone(), "strip", &[]), "lower", &[]), &current.clone())) } }) {  __out.push(line.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&said.clone(), &Py::None, &Py::None, &op_neg(&Py::Int(1))));
    for __item in __iter1 {
    line = __item.clone();
    lowered = mcall(&py_str(" "), "join", &[mcall(&mcall(&line.clone(), "lower", &[]), "split", &[])]);
    body = _the_body(&[lowered.clone(), _words(&[lowered.clone()])]);
    if py_truthy(&Py::Bool(!py_is(&body.clone(), &Py::None))) {
    return body.clone();
    }
    }
    return Py::None;
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
    let mut before: Py = Py::None;
    let mut body: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut person_wait: Py = Py::None;
    let _ = py_str("What moves a chase along, for a body that has gone quiet. None for every other line.");
    lowered = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&lowered.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    memory = py_dict(vec![]);
    }
    body = _the_body(&[lowered.clone(), _words(&[lowered.clone()])]);
    if py_truthy(&Py::Bool(!py_is(&body.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_NOT_DONE(), "search", &[lowered.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_TO_CHASE(), "search", &[lowered.clone()]) } }))) {
    return Py::None;
    }
    return _the_shape(&[body.clone(), memory.clone(), lowered.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_THE_TIMESCALE_ASK(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_BARE_CONTINUATION(), "match", &[lowered.clone()]))) } }) {
    before = _the_body_before(&[memory.clone(), lowered.clone()]);
    if py_truthy(&Py::Bool(!py_is(&before.clone(), &Py::None))) {
    return _the_timescale(&[before.clone(), memory.clone(), lowered.clone()]);
    }
    }
    if py_truthy(&mcall(&g_BARE_CONTINUATION(), "match", &[lowered.clone()])) {
    before = _the_body_before(&[memory.clone(), lowered.clone()]);
    if py_truthy(&Py::Bool(!py_is(&before.clone(), &Py::None))) {
    return _the_next_step(&[before.clone(), memory.clone(), lowered.clone()]);
    }
    if py_truthy(&_was_an_unnamed_chase(&[memory.clone(), lowered.clone()])) {
    return _the_next_step_unnamed(&[memory.clone(), lowered.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&g_IVE_TRIED(), "search", &[lowered.clone()])) {
    before = _the_body_before(&[memory.clone(), lowered.clone()]);
    if py_truthy(&Py::Bool(!py_is(&before.clone(), &Py::None))) {
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    lead = mcall(&fallback.clone(), "_choose", &[g_IVE_TRIED_LEAD(), memory.clone(), lowered.clone()]);
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![lead.clone(), _the_shape(&[before.clone(), memory.clone(), lowered.clone()])]));
    }
    }
    person_wait = _the_wait_for_a_person(&[memory.clone(), lowered.clone()]);
    if py_truthy(&Py::Bool(!py_is(&person_wait.clone(), &Py::None))) {
    return person_wait.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_RAISED(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_DONE(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_its_own_person(&[lowered.clone()]))) } } } }) {
    return _the_shape_unnamed(&[memory.clone(), lowered.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _the_timescale(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut lowered: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut _reference: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut said: Py = Py::None;
    let mut the_body: Py = Py::None;
    let _ = py_str("How long to leave it, for a chase asked again without the body's name.");
    let __unpk1 = py_unpack(&body.clone(), 2, None);
    the_body = __unpk1[0].clone();
    _reference = __unpk1[1].clone();
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    said = mcall(&mcall(&fallback.clone(), "_choose", &[g_THE_TIMESCALE(), memory.clone(), lowered.clone()]), "replace", &[py_str("{body}"), the_body.clone()]);
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    return stdlib_call("re", "sub", &[py_str("(?<=[.!?] )([a-z])"), { let _reference = _reference.clone(); let body = body.clone(); let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let said = said.clone(); let the_body = the_body.clone(); py_func("lambda", move |args: &[Py]| { let _reference = _reference.clone(); let body = body.clone(); let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let said = said.clone(); let the_body = the_body.clone(); let hit: Py = __kwp(args, 0, "hit").unwrap_or(Py::None); mcall(&mcall(&hit.clone(), "group", &[Py::Int(1)]), "upper", &[]) }) }, said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_next_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut lowered: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut fallback: Py = Py::None;
    let mut reference: Py = Py::None;
    let mut said: Py = Py::None;
    let mut the_body: Py = Py::None;
    let _ = py_str("What to do now, for a bare continuation after a chase.");
    let __unpk1 = py_unpack(&body.clone(), 2, None);
    the_body = __unpk1[0].clone();
    reference = __unpk1[1].clone();
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    said = mcall(&fallback.clone(), "_choose", &[g_THE_NEXT_STEP(), memory.clone(), lowered.clone()]);
    said = mcall(&mcall(&said.clone(), "replace", &[py_str("{reference}"), reference.clone()]), "replace", &[py_str("{body}"), the_body.clone()]);
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    return stdlib_call("re", "sub", &[py_str("(?<=[.!?] )([a-z])"), { let body = body.clone(); let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let reference = reference.clone(); let said = said.clone(); let the_body = the_body.clone(); py_func("lambda", move |args: &[Py]| { let body = body.clone(); let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let reference = reference.clone(); let said = said.clone(); let the_body = the_body.clone(); let hit: Py = __kwp(args, 0, "hit").unwrap_or(Py::None); mcall(&mcall(&hit.clone(), "group", &[Py::Int(1)]), "upper", &[]) }) }, said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_next_step_unnamed(args: &[Py]) -> Py {
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
    let mut lowered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut fallback: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("What to do now, for a bare continuation after a chase with no body named.");
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    said = mcall(&fallback.clone(), "_choose", &[g_THE_NEXT_STEP_UNNAMED(), memory.clone(), lowered.clone()]);
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    return stdlib_call("re", "sub", &[py_str("(?<=[.!?] )([a-z])"), { let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let said = said.clone(); let hit: Py = __kwp(args, 0, "hit").unwrap_or(Py::None); mcall(&mcall(&hit.clone(), "group", &[Py::Int(1)]), "upper", &[]) }) }, said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut lowered: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lowered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lowered"))) };
    let mut fallback: Py = Py::None;
    let mut reference: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut the_body: Py = Py::None;
    let _ = py_str("The sentence for a body: the reference to have, the ask, the timescale, the limit.");
    let __unpk1 = py_unpack(&body.clone(), 2, None);
    the_body = __unpk1[0].clone();
    reference = __unpk1[1].clone();
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    shape = mcall(&fallback.clone(), "_choose", &[g_THE_SHAPE(), memory.clone(), lowered.clone()]);
    said = mcall(&mcall(&shape.clone(), "replace", &[py_str("{reference}"), reference.clone()]), "replace", &[py_str("{body}"), the_body.clone()]);
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    said = stdlib_call("re", "sub", &[py_str("(?<=[.!?] )([a-z])"), { let body = body.clone(); let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let reference = reference.clone(); let said = said.clone(); let shape = shape.clone(); let the_body = the_body.clone(); py_func("lambda", move |args: &[Py]| { let body = body.clone(); let fallback = fallback.clone(); let lowered = lowered.clone(); let memory = memory.clone(); let reference = reference.clone(); let said = said.clone(); let shape = shape.clone(); let the_body = the_body.clone(); let hit: Py = __kwp(args, 0, "hit").unwrap_or(Py::None); mcall(&mcall(&hit.clone(), "group", &[Py::Int(1)]), "upper", &[]) }) }, said.clone()]).unwrap_or_else(|e| exc::raise(e));
    return said.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.chasing_a_body".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERING_SOMETHING_ELSE" => Some(g_ANSWERING_SOMETHING_ELSE()),
            "A_NEGATIVE" => Some(g_A_NEGATIVE()),
            "A_PERSON_AFTER_THE_NAME" => Some(g_A_PERSON_AFTER_THE_NAME()),
            "A_PERSON_DID_THE_SILENT_THING" => Some(g_A_PERSON_DID_THE_SILENT_THING()),
            "A_PERSON_NOT_A_BODY" => Some(g_A_PERSON_NOT_A_BODY()),
            "A_PERSON_OWES_A_REPLY" => Some(g_A_PERSON_OWES_A_REPLY()),
            "A_PERSON_THEIRS" => Some(g_A_PERSON_THEIRS()),
            "A_PERSON_WHO_WORKS_THERE" => Some(g_A_PERSON_WHO_WORKS_THERE()),
            "A_PRONOUN_DID_THE_SILENT_THING" => Some(g_A_PRONOUN_DID_THE_SILENT_THING()),
            "A_PRONOUN_OWES_A_REPLY" => Some(g_A_PRONOUN_OWES_A_REPLY()),
            "A_REPLY_VERB" => Some(g_A_REPLY_VERB()),
            "BARE_CONTINUATION" => Some(g_BARE_CONTINUATION()),
            "HOW_TO_CHASE" => Some(g_HOW_TO_CHASE()),
            "IVE_TRIED" => Some(g_IVE_TRIED()),
            "IVE_TRIED_LEAD" => Some(g_IVE_TRIED_LEAD()),
            "NOT_DONE" => Some(g_NOT_DONE()),
            "ORGANISATIONS" => Some(g_ORGANISATIONS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "RAISED" => Some(g_RAISED()),
            "THE_NEXT_STEP" => Some(g_THE_NEXT_STEP()),
            "THE_NEXT_STEP_UNNAMED" => Some(g_THE_NEXT_STEP_UNNAMED()),
            "THE_PRONOUN" => Some(g_THE_PRONOUN()),
            "THE_SHAPE" => Some(g_THE_SHAPE()),
            "THE_SHAPE_UNNAMED" => Some(g_THE_SHAPE_UNNAMED()),
            "THE_TIMESCALE" => Some(g_THE_TIMESCALE()),
            "THE_TIMESCALE_ASK" => Some(g_THE_TIMESCALE_ASK()),
            "THE_WAIT_FOR_A_PERSON" => Some(g_THE_WAIT_FOR_A_PERSON()),
            "_names_its_own_person" => Some(py_func("_names_its_own_person", |args: &[Py]| _names_its_own_person(args))),
            "_the_body" => Some(py_func("_the_body", |args: &[Py]| _the_body(args))),
            "_the_body_before" => Some(py_func("_the_body_before", |args: &[Py]| _the_body_before(args))),
            "_the_next_step" => Some(py_func("_the_next_step", |args: &[Py]| _the_next_step(args))),
            "_the_next_step_unnamed" => Some(py_func("_the_next_step_unnamed", |args: &[Py]| _the_next_step_unnamed(args))),
            "_the_person_before" => Some(py_func("_the_person_before", |args: &[Py]| _the_person_before(args))),
            "_the_person_it_belongs_to" => Some(py_func("_the_person_it_belongs_to", |args: &[Py]| _the_person_it_belongs_to(args))),
            "_the_shape" => Some(py_func("_the_shape", |args: &[Py]| _the_shape(args))),
            "_the_shape_unnamed" => Some(py_func("_the_shape_unnamed", |args: &[Py]| _the_shape_unnamed(args))),
            "_the_timescale" => Some(py_func("_the_timescale", |args: &[Py]| _the_timescale(args))),
            "_the_wait_for_a_person" => Some(py_func("_the_wait_for_a_person", |args: &[Py]| _the_wait_for_a_person(args))),
            "_was_an_unnamed_chase" => Some(py_func("_was_an_unnamed_chase", |args: &[Py]| _was_an_unnamed_chase(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERING_SOMETHING_ELSE" => s_ANSWERING_SOMETHING_ELSE(v),
            "A_NEGATIVE" => s_A_NEGATIVE(v),
            "A_PERSON_AFTER_THE_NAME" => s_A_PERSON_AFTER_THE_NAME(v),
            "A_PERSON_DID_THE_SILENT_THING" => s_A_PERSON_DID_THE_SILENT_THING(v),
            "A_PERSON_NOT_A_BODY" => s_A_PERSON_NOT_A_BODY(v),
            "A_PERSON_OWES_A_REPLY" => s_A_PERSON_OWES_A_REPLY(v),
            "A_PERSON_THEIRS" => s_A_PERSON_THEIRS(v),
            "A_PERSON_WHO_WORKS_THERE" => s_A_PERSON_WHO_WORKS_THERE(v),
            "A_PRONOUN_DID_THE_SILENT_THING" => s_A_PRONOUN_DID_THE_SILENT_THING(v),
            "A_PRONOUN_OWES_A_REPLY" => s_A_PRONOUN_OWES_A_REPLY(v),
            "A_REPLY_VERB" => s_A_REPLY_VERB(v),
            "BARE_CONTINUATION" => s_BARE_CONTINUATION(v),
            "HOW_TO_CHASE" => s_HOW_TO_CHASE(v),
            "IVE_TRIED" => s_IVE_TRIED(v),
            "IVE_TRIED_LEAD" => s_IVE_TRIED_LEAD(v),
            "NOT_DONE" => s_NOT_DONE(v),
            "ORGANISATIONS" => s_ORGANISATIONS(v),
            "PRIORITY" => s_PRIORITY(v),
            "RAISED" => s_RAISED(v),
            "THE_NEXT_STEP" => s_THE_NEXT_STEP(v),
            "THE_NEXT_STEP_UNNAMED" => s_THE_NEXT_STEP_UNNAMED(v),
            "THE_PRONOUN" => s_THE_PRONOUN(v),
            "THE_SHAPE" => s_THE_SHAPE(v),
            "THE_SHAPE_UNNAMED" => s_THE_SHAPE_UNNAMED(v),
            "THE_TIMESCALE" => s_THE_TIMESCALE(v),
            "THE_TIMESCALE_ASK" => s_THE_TIMESCALE_ASK(v),
            "THE_WAIT_FOR_A_PERSON" => s_THE_WAIT_FOR_A_PERSON(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.chasing_a_body", module_obj);
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_shape_unnamed", py_func("_the_shape_unnamed", |args: &[Py]| _the_shape_unnamed(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_person_before", py_func("_the_person_before", |args: &[Py]| _the_person_before(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_wait_for_a_person", py_func("_the_wait_for_a_person", |args: &[Py]| _the_wait_for_a_person(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_was_an_unnamed_chase", py_func("_was_an_unnamed_chase", |args: &[Py]| _was_an_unnamed_chase(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_body", py_func("_the_body", |args: &[Py]| _the_body(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_names_its_own_person", py_func("_names_its_own_person", |args: &[Py]| _names_its_own_person(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_person_it_belongs_to", py_func("_the_person_it_belongs_to", |args: &[Py]| _the_person_it_belongs_to(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_body_before", py_func("_the_body_before", |args: &[Py]| _the_body_before(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_timescale", py_func("_the_timescale", |args: &[Py]| _the_timescale(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_next_step", py_func("_the_next_step", |args: &[Py]| _the_next_step(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_next_step_unnamed", py_func("_the_next_step_unnamed", |args: &[Py]| _the_next_step_unnamed(args)));
    pyrt::xrefs::reg_fn("modules.chasing_a_body", "_the_shape", py_func("_the_shape", |args: &[Py]| _the_shape(args)));
    pyrt::xrefs::reg_access("modules.chasing_a_body", "ANSWERING_SOMETHING_ELSE", g_ANSWERING_SOMETHING_ELSE, s_ANSWERING_SOMETHING_ELSE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_NEGATIVE", g_A_NEGATIVE, s_A_NEGATIVE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PERSON_AFTER_THE_NAME", g_A_PERSON_AFTER_THE_NAME, s_A_PERSON_AFTER_THE_NAME);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PERSON_DID_THE_SILENT_THING", g_A_PERSON_DID_THE_SILENT_THING, s_A_PERSON_DID_THE_SILENT_THING);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PERSON_NOT_A_BODY", g_A_PERSON_NOT_A_BODY, s_A_PERSON_NOT_A_BODY);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PERSON_OWES_A_REPLY", g_A_PERSON_OWES_A_REPLY, s_A_PERSON_OWES_A_REPLY);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PERSON_THEIRS", g_A_PERSON_THEIRS, s_A_PERSON_THEIRS);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PERSON_WHO_WORKS_THERE", g_A_PERSON_WHO_WORKS_THERE, s_A_PERSON_WHO_WORKS_THERE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PRONOUN_DID_THE_SILENT_THING", g_A_PRONOUN_DID_THE_SILENT_THING, s_A_PRONOUN_DID_THE_SILENT_THING);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_PRONOUN_OWES_A_REPLY", g_A_PRONOUN_OWES_A_REPLY, s_A_PRONOUN_OWES_A_REPLY);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "A_REPLY_VERB", g_A_REPLY_VERB, s_A_REPLY_VERB);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "BARE_CONTINUATION", g_BARE_CONTINUATION, s_BARE_CONTINUATION);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "HOW_TO_CHASE", g_HOW_TO_CHASE, s_HOW_TO_CHASE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "IVE_TRIED", g_IVE_TRIED, s_IVE_TRIED);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "IVE_TRIED_LEAD", g_IVE_TRIED_LEAD, s_IVE_TRIED_LEAD);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "NOT_DONE", g_NOT_DONE, s_NOT_DONE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "ORGANISATIONS", g_ORGANISATIONS, s_ORGANISATIONS);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "RAISED", g_RAISED, s_RAISED);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_NEXT_STEP", g_THE_NEXT_STEP, s_THE_NEXT_STEP);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_NEXT_STEP_UNNAMED", g_THE_NEXT_STEP_UNNAMED, s_THE_NEXT_STEP_UNNAMED);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_PRONOUN", g_THE_PRONOUN, s_THE_PRONOUN);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_SHAPE", g_THE_SHAPE, s_THE_SHAPE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_SHAPE_UNNAMED", g_THE_SHAPE_UNNAMED, s_THE_SHAPE_UNNAMED);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_TIMESCALE", g_THE_TIMESCALE, s_THE_TIMESCALE);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_TIMESCALE_ASK", g_THE_TIMESCALE_ASK, s_THE_TIMESCALE_ASK);
    pyrt::xrefs::reg_access("modules.chasing_a_body", "THE_WAIT_FOR_A_PERSON", g_THE_WAIT_FOR_A_PERSON, s_THE_WAIT_FOR_A_PERSON);
}
