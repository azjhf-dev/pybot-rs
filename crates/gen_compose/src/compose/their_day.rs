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


thread_local! { static G_CARE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_DATE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_FIGURE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_LIFE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_POLICY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_POLICY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_TASTE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_TO_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_IDENTIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_FOR_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COMPOSITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEPOSIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIFE_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEARD_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE_AND_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DIARY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_CARE_LINES() -> Py { ensure_init(); G_CARE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_LINES(v: Py) { ensure_init(); G_CARE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES() -> Py { ensure_init(); G_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES(v: Py) { ensure_init(); G_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_DATE() -> Py { ensure_init(); G_OWN_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_DATE(v: Py) { ensure_init(); G_OWN_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_DATE_LINES() -> Py { ensure_init(); G_OWN_DATE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_DATE_LINES(v: Py) { ensure_init(); G_OWN_DATE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_FIGURE() -> Py { ensure_init(); G_OWN_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_FIGURE(v: Py) { ensure_init(); G_OWN_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_FIGURE_LINES() -> Py { ensure_init(); G_OWN_FIGURE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_FIGURE_LINES(v: Py) { ensure_init(); G_OWN_FIGURE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_LIFE_LINES() -> Py { ensure_init(); G_OWN_LIFE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_LIFE_LINES(v: Py) { ensure_init(); G_OWN_LIFE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_POLICY() -> Py { ensure_init(); G_OWN_POLICY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_POLICY(v: Py) { ensure_init(); G_OWN_POLICY.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_POLICY_LINES() -> Py { ensure_init(); G_OWN_POLICY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_POLICY_LINES(v: Py) { ensure_init(); G_OWN_POLICY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_TASTE_LINES() -> Py { ensure_init(); G_OWN_TASTE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_TASTE_LINES(v: Py) { ensure_init(); G_OWN_TASTE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_TO_ASK() -> Py { ensure_init(); G_WHO_TO_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_ASK(v: Py) { ensure_init(); G_WHO_TO_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_THE_BOT() -> Py { ensure_init(); G__ABOUT_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_BOT(v: Py) { ensure_init(); G__ABOUT_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER() -> Py { ensure_init(); G__AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER(v: Py) { ensure_init(); G__AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_IDENTIFIER() -> Py { ensure_init(); G__AN_IDENTIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_IDENTIFIER(v: Py) { ensure_init(); G__AN_IDENTIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS() -> Py { ensure_init(); G__ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); G__ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_FOR_ADVICE() -> Py { ensure_init(); G__ASKS_FOR_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_ADVICE(v: Py) { ensure_init(); G__ASKS_FOR_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COMPOSITION() -> Py { ensure_init(); G__A_COMPOSITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COMPOSITION(v: Py) { ensure_init(); G__A_COMPOSITION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEPOSIT() -> Py { ensure_init(); G__A_DEPOSIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEPOSIT(v: Py) { ensure_init(); G__A_DEPOSIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIFE_CHOICE() -> Py { ensure_init(); G__A_LIFE_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIFE_CHOICE(v: Py) { ensure_init(); G__A_LIFE_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WHEN() -> Py { ensure_init(); G__A_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WHEN(v: Py) { ensure_init(); G__A_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g__HEARD_BACK() -> Py { ensure_init(); G__HEARD_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEARD_BACK(v: Py) { ensure_init(); G__HEARD_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW() -> Py { ensure_init(); G__HOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW(v: Py) { ensure_init(); G__HOW.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE_AND_THING() -> Py { ensure_init(); G__MINE_AND_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE_AND_THING(v: Py) { ensure_init(); G__MINE_AND_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMES() -> Py { ensure_init(); G__NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMES(v: Py) { ensure_init(); G__NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DIARY() -> Py { ensure_init(); G__NOT_A_DIARY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DIARY(v: Py) { ensure_init(); G__NOT_A_DIARY.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_TASTE() -> Py { ensure_init(); G__OWN_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_TASTE(v: Py) { ensure_init(); G__OWN_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE() -> Py { ensure_init(); G__PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE(v: Py) { ensure_init(); G__PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PRONOUN() -> Py { ensure_init(); G__PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRONOUN(v: Py) { ensure_init(); G__PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID() -> Py { ensure_init(); G__SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID(v: Py) { ensure_init(); G__SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE() -> Py { ensure_init(); G__STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE(v: Py) { ensure_init(); G__STATE.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_BARE() -> Py { ensure_init(); G__STATE_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_BARE(v: Py) { ensure_init(); G__STATE_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g__TAIL() -> Py { ensure_init(); G__TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAIL(v: Py) { ensure_init(); G__TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__THING() -> Py { ensure_init(); G__THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THING(v: Py) { ensure_init(); G__THING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about something of the person's own, when only they were there.\n\n\"how was the trip\", \"how was the wedding\", \"how is work\", \"were the kids ok\", \"did the doctor\nring\", \"what did the vet say\". Nothing in the modules can answer these, because the answer is\nnot in any record: the person was there and the bot was not. What happened on main is that they\nreached the search hedge:\n\n    how was the wedding -> A quick search online will get you a solid answer\n    did the doctor ring -> A pharmacist or your GP can answer that today\n    were the kids ok    -> A quick search online will get you a solid answer\n\nTelling somebody to go and look up their own wedding is the worst line in the pool, and it is\nthe one a person gets for asking the thing they most want to talk about. The memory store\nalready answers the version of the same ask with a name in it (\"how did the interview go\" gets\n\"You tell me - how did the interview go?\"), so the bot is doing the right thing in one branch\nand the wrong thing in the other.\n\nWhat a friend does is the same thing without the name: say that it is theirs to tell, and ask\nit back in their own words. No search engine, and no guess at how it went.\n\nThe reading is deliberately a list of the things a person is present for - a trip, a shift, a\nscan, the kids, the dog - rather than any noun after \"how was\". A fact nobody had to witness\n(\"how was the titanic built\") belongs to the knowledge base, and how something was done (\"how\nwas the car fixed\", \"how was the trip paid for\") is a lookup; the trailing participle is what\ntells those apart from \"how did the car go\". A machine or a record that another lane owns\n(\"has my parcel arrived yet\", \"is the car ok now\", \"how was the boiler\") is left to that lane,\nand the bot's own day (\"how was your day\", \"hows your day going\") is chat's to answer.\n\nA fact of theirs is the same family and reaches the same hedge. \"whats my rent again\", \"is my\nholiday next week\", \"does my mum like golf\" and \"and what size did i say\" have no record in this\nbot and no place to look them up: the answer is in the person's own head, their own calendar,\ntheir own paperwork, or the head of somebody they know. Telling them to search the web for their\nown rent is the worst line in the pool for the second time, so the reading here says the bot\ncannot see it, names the thing that would have it - a statement, a confirmation email, the person\nthemselves - and asks the question that moves the talk on. It never guesses a figure, a date or a\ntaste (`own_fact_for`).\n");
    s__HOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:so|and|but|ok|okay|well|then|just|right|anyway|oh|hmm|yeah|yes|alright)\\s*,?\\s+)*(?P<ask>(?:any\\s+idea|do\\s+you\\s+know)?\\s*(?:how|what)\\s*(?:(?:was|were|did|have|has|is|are)\\s+)?)(?P<rest>.*)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:the|your|his|her|their|my|our)\\s+)?(?:(?:new|first|last|old|big)\\s+)?(?:night out|first day|last day|day out|trip|holiday|honeymoon|wedding|weekend|break|party|dinner|meal|lunch|flight|journey|drive|move|interview|exam|exams|test|tests|scan|appointment|match|game|play|concert|show|meeting|presentation|shift|course|day|week|morning|afternoon|evening|night|kids|children|boys|girls|family|mum|mom|dad|sister|brother|son|daughter|baby|dog|cat|puppy|kitten|vet|doctor|dentist|hospital|school|nursery|work|job|uni|house|flat|viewing|results|date|today|yesterday|session|class|lesson|stay|audition|driving test|theory test|appraisal|surgery|procedure|operation|court|hearing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AFTER(py_str("(?:\\s+(?:yet|now|today|tomorrow|though|then|again|back|at all|about it|with that|with it|hear|heard|reply|replied|get back|got back|call back|ring back|phone back|to you|these days|so far|alright|ok|okay|fine|good|in the end|anything|something|either|at the vets|at the hospital|at the doctor|about\\s+(?:him|her|them|it|that|this|me|us|the\\s+\\w+)|at\\s+(?:school|nursery|work|home|uni|college)|with\\s+(?:the|his|her|their|my|our)\\s+\\w+)){0,2}\\s*[?.!]*$"));
    s__TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:\\s+(?:go|went|do|going|get on|got on|get along|turn out|end up|come out|hold up)\\b)?(?:\\s+(?:today|tonight|yesterday|last night|last week|then|though|so far|in the end|again|yet|all right|alright|ok|okay|fine|good)){0,2}(?:\\s+with\\s+(?:the|his|her|their|my|our|your)\\s+\\w+)?\\s*[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PRONOUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and\\s+|so\\s+|but\\s+|ok\\s+|okay\\s+|well\\s+)?how\\s+(?:did|was|were)\\s+(?:it|that|this|they|he|she)(?:\\s+(?:go|went|get on|got on|get along|turn out|come out|work out)\\b[^?]*|(?:\\s+(?:then|though|today|tonight|last night|in the end|overall|really|so far)){0,2})[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PEOPLE(py_str("vet|doctor|dentist|optician|physio|counsel\\w*|consultant|nurse|midwife|surgeon|plumber|electrician|builder|roofer|locksmith|mechanic|garage|glazier|decorator|engineer|delivery\\s*(?:man|driver)?|courier|driver|agent|estate\\s+agent|letting\\s+agent|(?:boiler|gas|window|bathroom|kitchen|damp|carpet)\\s*(?:man|lady|fitter|guy|person|people|firm|company)|recruit\\w*|agency|recruitment\\s+agency|employer|manager|boss|hr|payroll|hospital|school|nursery|college|university|uni|council|job\\s*centre|dwp|hmrc|passport\\s+office|dvla|care\\s+home|social\\s+worker|bank|building\\s+society|landlord|solicitor|conveyanc\\w*|broker|insurer|insurance|mortgage\\s+(?:man|adviser|advisor)?"));
    s__STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("^(?:is|are|were|was|did|does|do|has|have)\\s+(?:(?:the|your|my|our)\\s+)?(?:kids|children|boys|girls|family|mum|mom|dad|sister|brother|son|daughter|baby|dog|cat|puppy|kitten|"), &g__PEOPLE()), &py_str("|scan|appointment|interview|exam|test|results|party|wedding|trip|flight|match|game|bank|council|landlord|solicitor|boiler|car|washing\\s+machine|parcel|parcels|order|delivery|package|roof|fence|extension|bathroom|kitchen|drive|driveway|paving|patio|garden|shed|loft|wall|floor|flooring|door|doors|window|windows|radiator|gutter|gutters|drain|drains|chimney|damp\\s*course)\\b\\s+(?:ok|okay|all right|alright|fine|good|bad|home|yet|back|there|asleep|awake|ring|call|phone|say|said|come|came|turn up|turned up|show|go|went|do|done|arrive|arrived|land|landed|got back|get back|hear|heard|reply|replied|say anything|say something|got there|show up|showed up|show up yet|turn up yet|come yet|came yet|got to (?:school|work|the hospital|the vets?|nursery)|get to (?:school|work)|hear back|heard back|ring back|call back|phone back|been in touch|get in touch|get back to (?:you|me)|got back to (?:you|me)|confirmed|come through|come back|come back ok|got sorted|been round|been out|hold up|held up|get done|got done|get home|got home|go ok|go alright|been done)\\b")), &g__AFTER())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__STATE_BARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("^(?:is|are|were|was|did|has|have)\\s+(?:(?:they|he|she)\\s+(?:ok|okay|all right|alright|fine|good|bad|home|yet|back|there|asleep|awake|ring|call|phone|say|said|come|came|turn up|turned up|arrive|arrived|land|landed|show|go|went|do|done|got back|got there)|it\\s+(?:ok|okay|all right|alright|fine|good|bad|home|yet|back|there|asleep|awake|ring|call|phone|said|came|turned up|arrived|landed|got back|got there))\\b"), &g__AFTER())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NAMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:were|was)\\s+[a-z']+(?:\\s+and\\s+[a-z']+)?\\s+there\\b[^?]*[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HEARD_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and\\s+|so\\s+)?(?:did|do|does|have|has|had)\\s+(?:you|they|he|she|we|anybody|anyone|somebody|someone|nobody)\\s+(?:hear|heard|get|got|have|had)\\s+(?:back|anything|something|any\\s+news|a\\s+reply|an\\s+answer|word)\\b[^?]*[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\s*(?:and\\s+)?what\\s+(?:did|do|does)\\s+(?:(?:the|your|my|our)\\s+)?(?:"), &g__PEOPLE()), &py_str("|teacher|head|she|he|they)\\s+(?:say|said|tell you)\\b[^?]*[?.!]*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ABOUT_THE_BOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:how|what)\\s+(?:was|is|were|did|does|do|has|have|'s|s|'d|d)?\\s*yours?(?:elf)?\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LINES(py_tuple(vec![py_str("You tell me - {ask}?"), py_str("I wasn't there, so you know better than me - {ask}?"), py_str("Only you know that one. {Ask}?"), py_str("No way for me to know that from here - {ask}?")]));
    s_CARE_LINES(py_tuple(vec![py_str("I have no way of knowing, and I would rather ask than guess - {ask}?"), py_str("That is yours to tell me, and I am listening - {ask}?"), py_str("I hope it went as well as it could. {Ask}?")]));
    s_WHO_TO_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback_data::g_WHO_TO_ASK()) { let __unpk2803 = py_unpack(&__item.clone(), 2, None); let mut _words: Py = __unpk2803[0].clone(); let mut who: Py = __unpk2803[1].clone(); __out.push(who.clone()); } py_set(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback_data::module_obj()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("vars", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "items", args))(&__args) }) { let __unpk7570 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk7570[0].clone(); let mut value: Py = __unpk7570[1].clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("_WHO")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {  __out.push(value.clone()); } } py_set(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWN_FIGURE(py_tuple(vec![py_str("rent"), py_str("mortgage"), py_str("council tax"), py_str("bill"), py_str("bills"), py_str("premium"), py_str("tariff"), py_str("direct debit"), py_str("payment"), py_str("payments"), py_str("balance"), py_str("deposit"), py_str("service charge"), py_str("arrears"), py_str("fee"), py_str("fees"), py_str("subscription"), py_str("charges"), py_str("salary"), py_str("wage"), py_str("wages"), py_str("pension"), py_str("loan"), py_str("overdraft"), py_str("interest"), py_str("standing order"), py_str("outgoings")]));
    s_OWN_DATE(py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("appointment"), py_str("appointments"), py_str("booking"), py_str("bookings"), py_str("flight"), py_str("renewal"), py_str("mot"), py_str("service"), py_str("deadline"), py_str("contract"), py_str("tenancy"), py_str("delivery"), py_str("check-in"), py_str("checkin"), py_str("expiry"), py_str("expires"), py_str("collection"), py_str("pick up"), py_str("pickup"), py_str("completion"), py_str("install"), py_str("installation"), py_str("viewing"), py_str("moving date"), py_str("move date"), py_str("end"), py_str("ends"), py_str("ending"), py_str("runs out"), py_str("run out"), py_str("run out date")]));
    s_OWN_POLICY(py_tuple(vec![py_str("insurance"), py_str("policy"), py_str("policies"), py_str("cover"), py_str("covered"), py_str("covers"), py_str("covering"), py_str("warranty"), py_str("guarantee"), py_str("excess"), py_str("claim"), py_str("claims"), py_str("protected"), py_str("included"), py_str("excluded")]));
    s_OWN_FIGURE_LINES(py_tuple(vec![py_str("I can't see your account, sorry. That figure is on your last statement or in the app for it, and the provider will read it out if you ring."), py_str("Nothing of yours is in front of me. Your statement or that provider's app has the exact amount; if it looks wrong, they are the ones to ring."), py_str("I've no way to check your own bills. The last email or statement has it, and the payments in your banking app usually show it too.")]));
    s_OWN_POLICY_LINES(py_tuple(vec![py_str("I can't read your policy from here, sorry. What it covers is in the schedule or the policy document, and the insurer will confirm it on the phone in a couple of minutes."), py_str("Nothing of yours is in front of me. The policy wording has it under what is covered; a call settles it faster than reading it."), py_str("I can't check a policy I can't see. The schedule that came with it lists what is covered and what the excess is, and the insurer can read it back to you.")]));
    s_OWN_DATE_LINES(py_tuple(vec![py_str("I can't see your diary, sorry. It will be in the confirmation email or the text they sent when you booked - tell me what it says and I'll help you plan around it."), py_str("No way to see your own dates from here. The booking email or the calendar on your phone has it, and whoever you booked with can confirm it in a minute if not."), py_str("I can't look at your own diary. The confirmation email is the first place: it is in there nine times out of ten.")]));
    s__A_LIFE_CHOICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\b(?:my|our)\\s+(?:\\w+\\s+){0,2}(?:job|jobs|work|career|boss|team|school|college|uni|university|course|degree|flat|house|home|town|city|country|area|class|gym|club|hobby|subject|placement|apprenticeship|business|role|position)\\b|\\bwhere\\s+i\\s+(?:live|work|am)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OWN_TASTE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:do|would|will|am|did)\\s+i\\s+(?:even\\s+)?(?:like|love|hate|enjoy|prefer)\\s+(?!it\\b|that\\b|this\\b|them\\b|those\\b|these\\b|one\\b|him\\b|her\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWN_LIFE_LINES(py_tuple(vec![py_str("Only you can say, and I don't want to guess at it for you. What is making you wonder?"), py_str("That one is yours to answer. What would make it better for you - more of something, or less of something?")]));
    s_OWN_TASTE_LINES(py_tuple(vec![py_str("You know that one better than I do - taste is the one thing nobody can check for you. The worst case is one mouthful."), py_str("Only you can say. Have it and see - what you fancy is not something anyone can tell you."), py_str("That is yours to know. Taste changes, too, so the last time is worth trying again.")]));
    s__ASKS_FOR_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:so|and|ok|okay|well|then|right|but)\\s*,?\\s+)?(?:how\\s+(?:do|can|could|should|would|to|might)\\b|what\\s+(?:should|could|can|do)\\s+i\\b|where\\s+(?:can|do|should|could)\\s+i\\b|any\\s+(?:advice|ideas|tips)\\b|is\\s+it\\s+(?:ok|okay|worth|better|cheaper|cheapest|possible)\\b|am\\s+i\\s+(?:better|allowed|able)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\?|\\b(?:what|whats|what's|when|where|which|how|is|are|was|were|do|does|did|has|have|can|could|should|am)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MINE_AND_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?P<thing>[a-z][a-z' ]{0,40})")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_COMPOSITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:comes?\\s+out\\s+of|coming\\s+out\\s+of|deduct\\w*|(?:take|takes|taken|taking)\\s+(?:\\w+\\s+){0,2}off|added\\s+(?:on|to)|on\\s+top\\s+of|worked\\s+out|calculated)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_IDENTIFIER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:numbers?|sort\\s+code|reference|ref|licence|license|pin|username|password|log\\s?in)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WHEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:when|what\\s+date|which\\s+date|how\\s+long|how\\s+soon|due)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_A_DIARY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:due|baby|pregnan\\w*|midwife|scan|test results|biopsy)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DEPOSIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bdeposits?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    let mut care: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "care") { v.clone() }
    else { Py::Bool(false) };
    let mut ask: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said_back: Py = Py::None;
    let _ = py_str("The lines for a question about something of their own, or None.\n\n    `care` is the pool after a death or a diagnosis: \"you tell me\" is the right hand-back to a\n    wedding and a cold one to a scan that might not have gone well.\n    ");
    ask = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    let mut understand: Py = py_func("understand", |args: &[Py]| gen_understanding::understanding::understand(args));
    ask = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plain")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&understand, args))(&__args) }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    return Py::None;
    }
    said_back = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&ask.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&ask.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&if py_truthy(&care.clone()) { g_CARE_LINES() } else { g_LINES() }) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("ask".to_string(), ask.clone()), ("Ask".to_string(), said_back.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { Py::None };
    let mut ask: Py = Py::None;
    let mut how: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The ask it is theirs to answer, in their own words, or None.\n\n    `repaired` is the understanding engine's spelling back of the same line. It is only read, never\n    echoed: \"hows the weddng\" is asked back as they typed it, and a line with a typo or a \"lol\" on\n    the end is the same question as the clean one.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(14)) } }) {
    return Py::None;
    }
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow'd\\b|\\bhowd\\b")); __args.push(py_str("how did")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow's\\b|\\bhows\\b")); __args.push(py_str("how is")); __args.push({ let __b = plain.clone(); if py_truthy(&__b) { __b } else { text.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ABOUT_THE_BOT(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ABOUT_THE_BOT(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ABOUT_THE_BOT(), "match", args))(&__args) } } } } }) {
    return Py::None;
    }
    ask = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?.! ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^hows\\b")); __args.push(py_str("how's")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^howd\\b")); __args.push(py_str("how'd")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    how = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOW(), "match", args))(&__args) };
    if py_truthy(&how.clone()) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&how.clone(), "group", args))(&__args) };
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THING(), "match", args))(&__args) };
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&rest.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TAIL(), "match", args))(&__args) } } }) {
    return ask.clone();
    }
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g__STATE(), g__STATE_BARE(), g__SAID(), g__HEARD_BACK(), g__PRONOUN(), g__NAMES()]));
    for __item in __iter1 {
    pattern = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "match", args))(&__args) }) {
    return ask.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn own_fact(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut order: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The bucket a question about a fact of theirs belongs in, or None.\n\n    `kind` is \"figure\", \"date\", \"policy\", \"taste\" or \"life\".\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(14)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_FOR_ADVICE(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }, &Py::None, &Py::Int(3), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS(), "match", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LIFE_CHOICE(), "search", args))(&__args) }) {
    return py_tuple(vec![py_str("life"), py_str("")]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OWN_TASTE(), "search", args))(&__args) }) {
    return py_tuple(vec![py_str("taste"), py_str("")]);
    }
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MINE_AND_THING(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mine.clone(), "group", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_A_DIARY(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DEPOSIT(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_IDENTIFIER(), "search", args))(&__args) }) {
    return Py::None;
    }
    order = py_tuple(vec![py_tuple(vec![py_str("policy"), g_OWN_POLICY()]), py_tuple(vec![py_str("figure"), g_OWN_FIGURE()]), py_tuple(vec![py_str("date"), g_OWN_DATE()])]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WHEN(), "search", args))(&__args) }) {
    order = py_tuple(vec![py_tuple(vec![py_str("date"), g_OWN_DATE()]), py_tuple(vec![py_str("policy"), g_OWN_POLICY()]), py_tuple(vec![py_str("figure"), g_OWN_FIGURE()])]);
    }
    let __iter1 = py_iter_or_raise(&order.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?![a-z])"))); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("figure"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COMPOSITION(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    return py_tuple(vec![kind.clone(), py_str("")]);
    }
    return Py::None;
    Py::None
}
pub fn own_fact_for(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut read: Py = Py::None;
    let _ = py_str("The reply for a question about a fact of theirs, or None.\n\n    Read from the line as typed; a spelling the understanding engine repairs is not needed here,\n    because the nouns that decide the bucket are short and common. Only a hedge is replaced, so a\n    line with an answer of its own never reaches this.\n    ");
    read = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| own_fact(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    return Py::None;
    }
    kind = geti(&read.clone(), &Py::Int(0));
    pool = geti(&py_dict(vec![(py_str("figure"), g_OWN_FIGURE_LINES()), (py_str("date"), g_OWN_DATE_LINES()), (py_str("policy"), g_OWN_POLICY_LINES()), (py_str("taste"), g_OWN_TASTE_LINES()), (py_str("life"), g_OWN_LIFE_LINES())]), &kind.clone());
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.their_day".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CARE_LINES" => Some(g_CARE_LINES()),
            "LINES" => Some(g_LINES()),
            "OWN_DATE" => Some(g_OWN_DATE()),
            "OWN_DATE_LINES" => Some(g_OWN_DATE_LINES()),
            "OWN_FIGURE" => Some(g_OWN_FIGURE()),
            "OWN_FIGURE_LINES" => Some(g_OWN_FIGURE_LINES()),
            "OWN_LIFE_LINES" => Some(g_OWN_LIFE_LINES()),
            "OWN_POLICY" => Some(g_OWN_POLICY()),
            "OWN_POLICY_LINES" => Some(g_OWN_POLICY_LINES()),
            "OWN_TASTE_LINES" => Some(g_OWN_TASTE_LINES()),
            "WHO_TO_ASK" => Some(g_WHO_TO_ASK()),
            "_ABOUT_THE_BOT" => Some(g__ABOUT_THE_BOT()),
            "_AFTER" => Some(g__AFTER()),
            "_AN_IDENTIFIER" => Some(g__AN_IDENTIFIER()),
            "_ASKS" => Some(g__ASKS()),
            "_ASKS_FOR_ADVICE" => Some(g__ASKS_FOR_ADVICE()),
            "_A_COMPOSITION" => Some(g__A_COMPOSITION()),
            "_A_DEPOSIT" => Some(g__A_DEPOSIT()),
            "_A_LIFE_CHOICE" => Some(g__A_LIFE_CHOICE()),
            "_A_WHEN" => Some(g__A_WHEN()),
            "_HEARD_BACK" => Some(g__HEARD_BACK()),
            "_HOW" => Some(g__HOW()),
            "_MINE_AND_THING" => Some(g__MINE_AND_THING()),
            "_NAMES" => Some(g__NAMES()),
            "_NOT_A_DIARY" => Some(g__NOT_A_DIARY()),
            "_OWN_TASTE" => Some(g__OWN_TASTE()),
            "_PEOPLE" => Some(g__PEOPLE()),
            "_PRONOUN" => Some(g__PRONOUN()),
            "_SAID" => Some(g__SAID()),
            "_STATE" => Some(g__STATE()),
            "_STATE_BARE" => Some(g__STATE_BARE()),
            "_TAIL" => Some(g__TAIL()),
            "_THING" => Some(g__THING()),
            "own_fact" => Some(py_func("own_fact", |args: &[Py]| own_fact(args))),
            "own_fact_for" => Some(py_func("own_fact_for", |args: &[Py]| own_fact_for(args))),
            "pool_for" => Some(py_func("pool_for", |args: &[Py]| pool_for(args))),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CARE_LINES" => s_CARE_LINES(v),
            "LINES" => s_LINES(v),
            "OWN_DATE" => s_OWN_DATE(v),
            "OWN_DATE_LINES" => s_OWN_DATE_LINES(v),
            "OWN_FIGURE" => s_OWN_FIGURE(v),
            "OWN_FIGURE_LINES" => s_OWN_FIGURE_LINES(v),
            "OWN_LIFE_LINES" => s_OWN_LIFE_LINES(v),
            "OWN_POLICY" => s_OWN_POLICY(v),
            "OWN_POLICY_LINES" => s_OWN_POLICY_LINES(v),
            "OWN_TASTE_LINES" => s_OWN_TASTE_LINES(v),
            "WHO_TO_ASK" => s_WHO_TO_ASK(v),
            "_ABOUT_THE_BOT" => s__ABOUT_THE_BOT(v),
            "_AFTER" => s__AFTER(v),
            "_AN_IDENTIFIER" => s__AN_IDENTIFIER(v),
            "_ASKS" => s__ASKS(v),
            "_ASKS_FOR_ADVICE" => s__ASKS_FOR_ADVICE(v),
            "_A_COMPOSITION" => s__A_COMPOSITION(v),
            "_A_DEPOSIT" => s__A_DEPOSIT(v),
            "_A_LIFE_CHOICE" => s__A_LIFE_CHOICE(v),
            "_A_WHEN" => s__A_WHEN(v),
            "_HEARD_BACK" => s__HEARD_BACK(v),
            "_HOW" => s__HOW(v),
            "_MINE_AND_THING" => s__MINE_AND_THING(v),
            "_NAMES" => s__NAMES(v),
            "_NOT_A_DIARY" => s__NOT_A_DIARY(v),
            "_OWN_TASTE" => s__OWN_TASTE(v),
            "_PEOPLE" => s__PEOPLE(v),
            "_PRONOUN" => s__PRONOUN(v),
            "_SAID" => s__SAID(v),
            "_STATE" => s__STATE(v),
            "_STATE_BARE" => s__STATE_BARE(v),
            "_TAIL" => s__TAIL(v),
            "_THING" => s__THING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.their_day", module_obj);
    pyrt::xrefs::reg_fn("compose.their_day", "pool_for", py_func("pool_for", |args: &[Py]| pool_for(args)));
    pyrt::xrefs::reg_fn("compose.their_day", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_fn("compose.their_day", "own_fact", py_func("own_fact", |args: &[Py]| own_fact(args)));
    pyrt::xrefs::reg_fn("compose.their_day", "own_fact_for", py_func("own_fact_for", |args: &[Py]| own_fact_for(args)));
    pyrt::xrefs::reg_access("compose.their_day", "CARE_LINES", g_CARE_LINES, s_CARE_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "LINES", g_LINES, s_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_DATE", g_OWN_DATE, s_OWN_DATE);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_DATE_LINES", g_OWN_DATE_LINES, s_OWN_DATE_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_FIGURE", g_OWN_FIGURE, s_OWN_FIGURE);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_FIGURE_LINES", g_OWN_FIGURE_LINES, s_OWN_FIGURE_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_LIFE_LINES", g_OWN_LIFE_LINES, s_OWN_LIFE_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_POLICY", g_OWN_POLICY, s_OWN_POLICY);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_POLICY_LINES", g_OWN_POLICY_LINES, s_OWN_POLICY_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "OWN_TASTE_LINES", g_OWN_TASTE_LINES, s_OWN_TASTE_LINES);
    pyrt::xrefs::reg_access("compose.their_day", "WHO_TO_ASK", g_WHO_TO_ASK, s_WHO_TO_ASK);
    pyrt::xrefs::reg_access("compose.their_day", "_ABOUT_THE_BOT", g__ABOUT_THE_BOT, s__ABOUT_THE_BOT);
    pyrt::xrefs::reg_access("compose.their_day", "_AFTER", g__AFTER, s__AFTER);
    pyrt::xrefs::reg_access("compose.their_day", "_AN_IDENTIFIER", g__AN_IDENTIFIER, s__AN_IDENTIFIER);
    pyrt::xrefs::reg_access("compose.their_day", "_ASKS", g__ASKS, s__ASKS);
    pyrt::xrefs::reg_access("compose.their_day", "_ASKS_FOR_ADVICE", g__ASKS_FOR_ADVICE, s__ASKS_FOR_ADVICE);
    pyrt::xrefs::reg_access("compose.their_day", "_A_COMPOSITION", g__A_COMPOSITION, s__A_COMPOSITION);
    pyrt::xrefs::reg_access("compose.their_day", "_A_DEPOSIT", g__A_DEPOSIT, s__A_DEPOSIT);
    pyrt::xrefs::reg_access("compose.their_day", "_A_LIFE_CHOICE", g__A_LIFE_CHOICE, s__A_LIFE_CHOICE);
    pyrt::xrefs::reg_access("compose.their_day", "_A_WHEN", g__A_WHEN, s__A_WHEN);
    pyrt::xrefs::reg_access("compose.their_day", "_HEARD_BACK", g__HEARD_BACK, s__HEARD_BACK);
    pyrt::xrefs::reg_access("compose.their_day", "_HOW", g__HOW, s__HOW);
    pyrt::xrefs::reg_access("compose.their_day", "_MINE_AND_THING", g__MINE_AND_THING, s__MINE_AND_THING);
    pyrt::xrefs::reg_access("compose.their_day", "_NAMES", g__NAMES, s__NAMES);
    pyrt::xrefs::reg_access("compose.their_day", "_NOT_A_DIARY", g__NOT_A_DIARY, s__NOT_A_DIARY);
    pyrt::xrefs::reg_access("compose.their_day", "_OWN_TASTE", g__OWN_TASTE, s__OWN_TASTE);
    pyrt::xrefs::reg_access("compose.their_day", "_PEOPLE", g__PEOPLE, s__PEOPLE);
    pyrt::xrefs::reg_access("compose.their_day", "_PRONOUN", g__PRONOUN, s__PRONOUN);
    pyrt::xrefs::reg_access("compose.their_day", "_SAID", g__SAID, s__SAID);
    pyrt::xrefs::reg_access("compose.their_day", "_STATE", g__STATE, s__STATE);
    pyrt::xrefs::reg_access("compose.their_day", "_STATE_BARE", g__STATE_BARE, s__STATE_BARE);
    pyrt::xrefs::reg_access("compose.their_day", "_TAIL", g__TAIL, s__TAIL);
    pyrt::xrefs::reg_access("compose.their_day", "_THING", g__THING, s__THING);
}
