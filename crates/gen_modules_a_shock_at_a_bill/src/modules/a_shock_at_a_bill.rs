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


thread_local! { static G_A_BILL_WITH_ITS_OWN_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BODY_ENDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOCUMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_IN_THE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_PROBLEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAGE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHOCK_AT_A_PAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALSO_A_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_ACCOUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BODY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SOMEBODY_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE_OF_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WENT_OUT_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORD_ABOUT_ANY_BILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORD_BEFORE_A_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEFORE_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BILL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_YET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONLY_THE_PAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WITHOUT_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_shock_at_a_bill: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_BILL_WITH_ITS_OWN_CARD() -> Py { ensure_init(); G_A_BILL_WITH_ITS_OWN_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BILL_WITH_ITS_OWN_CARD(v: Py) { ensure_init(); G_A_BILL_WITH_ITS_OWN_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BODY_ENDING() -> Py { ensure_init(); G_A_BODY_ENDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_ENDING(v: Py) { ensure_init(); G_A_BODY_ENDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOCUMENT() -> Py { ensure_init(); G_A_DOCUMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOCUMENT(v: Py) { ensure_init(); G_A_DOCUMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_IN_THE_ANSWER() -> Py { ensure_init(); G_A_MONEY_IN_THE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_IN_THE_ANSWER(v: Py) { ensure_init(); G_A_MONEY_IN_THE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_PROBLEM() -> Py { ensure_init(); G_A_MONEY_PROBLEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_PROBLEM(v: Py) { ensure_init(); G_A_MONEY_PROBLEM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAGE_VERB() -> Py { ensure_init(); G_A_PAGE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAGE_VERB(v: Py) { ensure_init(); G_A_PAGE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHOCK_AT_A_PAGE() -> Py { ensure_init(); G_A_SHOCK_AT_A_PAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHOCK_AT_A_PAGE(v: Py) { ensure_init(); G_A_SHOCK_AT_A_PAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__ALSO_A_COMPLAINT() -> Py { ensure_init(); G__ALSO_A_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALSO_A_COMPLAINT(v: Py) { ensure_init(); G__ALSO_A_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_ACCOUNT_WORD() -> Py { ensure_init(); G__A_ACCOUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_ACCOUNT_WORD(v: Py) { ensure_init(); G__A_ACCOUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BODY_WORD() -> Py { ensure_init(); G__A_BODY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BODY_WORD(v: Py) { ensure_init(); G__A_BODY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SOMEBODY_ELSE() -> Py { ensure_init(); G__A_SOMEBODY_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SOMEBODY_ELSE(v: Py) { ensure_init(); G__A_SOMEBODY_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE_OF_A_BODY() -> Py { ensure_init(); G__A_STATE_OF_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE_OF_A_BODY(v: Py) { ensure_init(); G__A_STATE_OF_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SYMPTOM() -> Py { ensure_init(); G__A_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SYMPTOM(v: Py) { ensure_init(); G__A_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WENT_OUT_COLD() -> Py { ensure_init(); G__A_WENT_OUT_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WENT_OUT_COLD(v: Py) { ensure_init(); G__A_WENT_OUT_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORD_ABOUT_ANY_BILL() -> Py { ensure_init(); G__A_WORD_ABOUT_ANY_BILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORD_ABOUT_ANY_BILL(v: Py) { ensure_init(); G__A_WORD_ABOUT_ANY_BILL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORD_BEFORE_A_KIND() -> Py { ensure_init(); G__A_WORD_BEFORE_A_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORD_BEFORE_A_KIND(v: Py) { ensure_init(); G__A_WORD_BEFORE_A_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__BEFORE_IT_COMES() -> Py { ensure_init(); G__BEFORE_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEFORE_IT_COMES(v: Py) { ensure_init(); G__BEFORE_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g__BILL_LINES() -> Py { ensure_init(); G__BILL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BILL_LINES(v: Py) { ensure_init(); G__BILL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_YET() -> Py { ensure_init(); G__NOT_YET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_YET(v: Py) { ensure_init(); G__NOT_YET.with(|c| *c.borrow_mut() = v); }
pub fn g__ONLY_THE_PAGE() -> Py { ensure_init(); G__ONLY_THE_PAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONLY_THE_PAGE(v: Py) { ensure_init(); G__ONLY_THE_PAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__WITHOUT_A_FIGURE() -> Py { ensure_init(); G__WITHOUT_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WITHOUT_A_FIGURE(v: Py) { ensure_init(); G__WITHOUT_A_FIGURE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A body ending said at a bill is a figure of speech, and the bill is the question.\n\nThirteen lines typed on `6f5447f3b`, and every one of them got an answer about the wrong subject:\n\n    i nearly died when i saw the bill           -> the bereavement team card: \"say the account\n                                                   holder has died and they close the account\"\n    i nearly fell over when i saw the bill      -> the falls review\n    i nearly had a heart attack at the bill      -> the dizzy-spell card\n    my heart stopped when i saw the bill        -> \"That's something only they can see\"\n    i couldnt breathe when i saw the bill      -> \"Got it. Is there anything you'd like a hand with?\"\n    i opened the statement and nearly cried     -> \"How did that go, overall?\"\n    i read the invoice and nearly died         -> \"Annoying. Is it every time, or only now and then?\"\n    i saw the bill                              -> \"Nice one. How did the rest of it go?\"\n    i opened the bill                           -> \"Good to hear. What was the best bit?\"\n\nThe knowledge base has no card for \"a bill shocked me\", so each of those lines reached whichever\ncard owned the *ending* - a death, a fall, a faint, a choking - or a greeting pool that cheered at\nit. The person is asking about the bill.\n\nSo the module answers one subject and nothing else: a body ending and a **bill, statement or\ninvoice** said of each other. Three orders count, and each has its own reason:\n\n* the ending, then the page - \"i nearly died when i saw the bill\". The page has to come after the\n  ending, so `i nearly died` on its own stays with loss and grief, and `the bill nearly killed me`\n  stays with the cards below.\n* the page, then the ending - \"i read the invoice and nearly died\", \"the bill took my breath away\".\n  A page verb has to sit in front of the document, so `i paid the bill and nearly died on the\n  motorway` is left to the safety module: there the person was not reading.\n* the page in hand and nothing else - \"i saw the bill\", \"i opened the bill\". A line that is only\n  somebody holding a bill is asking about the bill.\n\nA bill that has not come is answered for what it is (confirm the address, note the date and who\nyou spoke to, and what to look for when it lands); a bill that came with a figure is not asked\n\"how much?\" back. The answer is the page-check a friend gives: what it says you are charged for,\nthe dates it covers, the reference, ask for the breakdown in writing, keep a copy. A card that\nknows the bill itself keeps its turn, and the module stands down for it: when the line names a kind\nof bill (council tax, energy, water, gas) and the knowledge base's own reply names that kind in its\nopening sentence, the card read the bill - the band and the discounts, the tariff and the standing\ncharge - and the general page-check is the lesser answer.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.PRIORITY"); }
    s_PRIORITY(Py::Int(1000));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_BODY_ENDING"); }
    s_A_BODY_ENDING(stdlib_call("re", "compile", &[py_str("\\b(?:nearly|almost|bloody\\s+near|like\\s+to\\s+have)\\s+(?:died|fell\\s+over|fainted|passed\\s+out|choked|keeled\\s+over|collapsed)\\b|\\b(?:had|having)\\s+a\\s+heart\\s+attack\\b|\\bmy\\s+heart\\s+(?:stopped|sank)\\b|\\bcould(?:n'?t| not)\\s+(?:breathe|get\\s+my\\s+breath|catch\\s+my\\s+breath)\\b|\\bmy\\s+(?:jaw|eyes|heart)\\s+(?:dropped|fell|nearly\\s+dropped)\\b|\\b(?:was|were)\\s+(?:gasping|choking)\\b|\\bchoked\\b|\\bgasped\\b|\\bnearly\\s+(?:cried|wept)\\b|\\bwent\\s+(?:white|pale)\\b|\\bmy\\s+knees\\s+(?:went|gave)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_SHOCK_AT_A_PAGE"); }
    s_A_SHOCK_AT_A_PAGE(stdlib_call("re", "compile", &[py_str("\\btook\\s+my\\s+breath\\s+away\\b|\\bmy\\s+(?:jaw|heart)\\s+(?:dropped|fell|sank)\\b|\\bwent\\s+(?:white|pale)\\b|\\bmy\\s+knees\\s+(?:went|gave)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_DOCUMENT"); }
    s_A_DOCUMENT(stdlib_call("re", "compile", &[py_str("\\b(?:bill|bills|statement|statements|invoice|invoices|invoiced)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._NOT_YET"); }
    s__NOT_YET(stdlib_call("re", "compile", &[py_str("\\b(?:hasn'?t|hasnt|haven'?t|havent|hadn'?t|hadnt|didn'?t|didnt|never|no)\\b[^.!?]{0,30}\\b(?:arrived|come|came|turned\\s+up|showed\\s+up)\\b|\\bno\\s+(?:bill|statement|invoice)\\s+(?:yet|at\\s+all)\\b|\\b(?:not|never)\\s+(?:had|received|got)\\s+(?:it|a\\s+(?:bill|statement|invoice))\\s+yet\\b|\\bstill\\s+waiting\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_PAGE_VERB"); }
    s_A_PAGE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:saw|seen|opening|opened|read|reading|looked\\s+at|looking\\s+at|looked\\s+through|checked|checking|got|get|received|receiving|landed|arrived|through\\s+the\\s+door)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._ONLY_THE_PAGE"); }
    s__ONLY_THE_PAGE(stdlib_call("re", "compile", &[py_str("^(?:i|we)?\\s*(?:have\\s+|had\\s+|just\\s+|only\\s+just\\s+|finally\\s+|today\\s+|this\\s+morning\\s+|yesterday\\s+)*(?:saw|seen|opened|opening|read|reading|got|get|received|checked|looked\\s+at)\\s+(?:the|my|this|our|that|a)\\s+(?:bill|bills|statement|statements|invoice|invoices)\\b(?:\\s+(?:today|yesterday|this\\s+morning|last\\s+night|again|now))?[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("numb"), py_str("numbness"), py_str("weak"), py_str("weakness"), py_str("shaky"), py_str("shaking"), py_str("lightheaded"), py_str("sweating"), py_str("faint"), py_str("fainting"), py_str("dizzy"), py_str("dizziness"), py_str("breathless"), py_str("breathlessness")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._ALSO_A_COMPLAINT"); }
    s__ALSO_A_COMPLAINT(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_WENT_OUT_COLD"); }
    s__A_WENT_OUT_COLD(stdlib_call("re", "compile", &[py_str("\\b(?:blacked out|blackout|blacked|passed out|out cold|unconscious|keeled over|collapsed|fainted|fainting fits?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_STATE_OF_A_BODY"); }
    s__A_STATE_OF_A_BODY(stdlib_call("re", "compile", &[py_str("\\b(?:hurt|hurts|hurting|sore|soreness|tight|tightness|numb|numbness|ache|aches|aching|throb|throbbing|swollen|swelling|bleeding|stiff|stiffness|weak|weakness|shaky|shaking|still|worse|pounding|squeezing|burning|itchy|itching|watering|gone|goes|went)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    let __try2 = exc::catch(|| {
    s__A_BODY_WORD(pyrt::xrefs::xget("compose.fallback_data", "BODY_PARTS"));
    s__A_SYMPTOM(pyrt::xrefs::xget("compose.fallback_data", "SYMPTOM_WORDS"));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    fn __data7() -> Py { py_tuple(vec![py_str("arm"), py_str("back"), py_str("chest"), py_str("ear"), py_str("eye"), py_str("eyes"), py_str("foot"), py_str("hand"), py_str("head"), py_str("hip"), py_str("jaw"), py_str("knee"), py_str("leg"), py_str("neck"), py_str("shoulder"), py_str("skin"), py_str("stomach"), py_str("teeth"), py_str("throat"), py_str("toe"), py_str("wrist")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_BODY_WORD"); }
    s__A_BODY_WORD(call_builtin("frozenset", &[__data7()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("ache"), py_str("aches"), py_str("bleed"), py_str("bleeding"), py_str("bruise"), py_str("confused"), py_str("cough"), py_str("dizzy"), py_str("fever"), py_str("headache"), py_str("hurt"), py_str("hurting"), py_str("hurts"), py_str("nausea"), py_str("nauseous"), py_str("pain"), py_str("painful"), py_str("rash"), py_str("sick"), py_str("sore"), py_str("swollen"), py_str("temperature"), py_str("tired"), py_str("vomit"), py_str("vomiting"), py_str("wheezing")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_SYMPTOM"); }
    s__A_SYMPTOM(call_builtin("frozenset", &[__data8()]).unwrap_or_else(|e| exc::raise(e)));
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._BILL_LINES"); }
    s__BILL_LINES(py_tuple(vec![py_str("A bill is worth checking before you pay it: the dates it covers, what it says you are being charged for, and the reference. Ask for the breakdown in writing, and if it is wrong say so in writing before the date on it, keeping a copy. What is it for, and how much?"), py_str("Before paying a bill you did not expect, check the dates it covers, what it is charging for and the reference. Ask for the breakdown in writing, and answer in writing before the date on it, keeping a copy. What is it charging you for?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._WITHOUT_A_FIGURE"); }
    s__WITHOUT_A_FIGURE(py_tuple(vec![py_str("That is worth checking before you pay it: the dates it covers, what it says you are being charged for, and the reference. Ask for the breakdown in writing, and if it is wrong say so in writing before the date on it, keeping a copy. What is the reference on it?"), py_str("Before you pay it, check the dates it covers, what it says you are charged for and the reference. Ask for the breakdown in writing and keep a copy of everything. Does it say what period it covers?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._BEFORE_IT_COMES"); }
    s__BEFORE_IT_COMES(py_tuple(vec![py_str("When it comes, the three things to find are the date they want an answer by, what it says you are charged for and the reference. Ask them to confirm when it was sent and to which address, and note who you spoke to and when."), py_str("While you wait, it is worth confirming they have your right address, and noting the date and the name of whoever you speak to. When it lands, the reply-by date, what it charges for and the reference are the things to check first.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_FIGURE"); }
    s__A_FIGURE(stdlib_call("re", "compile", &[py_str("(?:\\u00a3\\s*\\d|\\b\\d+(?:[.,]\\d+)?\\s*(?:pounds?|quid|p|per\\s?cent|%)\\b|\\b(?:for|of|cost(?:s|ing)?|paid|owe[sd]?|totall?ing)\\s+\\u00a3?\\d+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_ACCOUNT_WORD"); }
    s__A_ACCOUNT_WORD(call_builtin("frozenset", &[mcall(&py_str("\naccount annual quarterly monthly bank bill card charge council credit debit electricity electric\nenergy gas insurance internet loan metered metering meter mobile mortgage phone rent service\nsewerage student supply tax tv utility water\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_WORD_ABOUT_ANY_BILL"); }
    s__A_WORD_ABOUT_ANY_BILL(call_builtin("frozenset", &[mcall(&py_str("\na an the my our your their his her its this that these those some any no\nbig small huge massive enormous enormous giant tiny little\nnew old latest last first second third final whole same other another next\nunexpected surprise shocking ridiculous stupid bloody damn awful terrible\nmonthly quarterly annual weekly daily\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_BILL_WITH_ITS_OWN_CARD"); }
    s_A_BILL_WITH_ITS_OWN_CARD(py_tuple(vec![py_str("council tax")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_WORD_BEFORE_A_KIND"); }
    s__A_WORD_BEFORE_A_KIND(call_builtin("frozenset", &[mcall(&py_str("\ni me my we us our you your he him his she her it its they them their\nsaw see seen seeing read reading reads open opened opening opens got get gets getting\nreceive received receiving arriving arrived arrive came come comes coming\npaid pay paying sent send sends sending took take taken\nin on at for from with about of to and but or when after before then now today\nthe first second last final\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill._A_SOMEBODY_ELSE"); }
    s__A_SOMEBODY_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our)\\s+(?:mum|mom|mammy|dad|father|mother|son|daughter|wife|husband|partner|boyfriend|girlfriend|nan|nana|gran|granny|grandma|granddad|grandad|grandpa|grandmother|grandfather|brother|sister|uncle|aunt|cousin|friend|kid|kids|child|baby|boy|girl|dog|cat|neighbour|neighbor)\\b|\\b(?:he|she|they|him|her|them)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_MONEY_PROBLEM"); }
    s_A_MONEY_PROBLEM(stdlib_call("re", "compile", &[py_str("\\b(?:charged\\s+(?:me\\s+)?twice|double\\s+charged|charged\\s+me\\s+double|duplicate|overcharged|over\\s*charged|cant\\s+pay|can'?t\\s+pay|cant\\s+afford|can'?t\\s+afford|cannot\\s+pay|cannot\\s+afford|owe|owes|owed|arrears|in\\s+arrears|payment\\s+plan|instal+ments?|wrong|too\\s+much|discount|rebate|refund|benefit|hardship|meter\\s+read|reading|estimated)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_shock_at_a_bill.A_MONEY_IN_THE_ANSWER"); }
    s_A_MONEY_IN_THE_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:pay|pays|paid|payment|payments|overdraft|refund|refunds|duplicate|charge|charges|charged|owe|owes|arrears|debit|instal+ments?|bill|bills|council|band|discount|discounts|rebate|tariff|standing\\s+charge|meter|account|bank|money|rent)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_complaint_added_after_the_ending(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("True when a real complaint is said after the figure of speech, which is the answer needed.\n\n    \"i nearly died when i saw the bill\" is about the bill, but \"i nearly fainted when i saw the bill\n    and now i feel sick\" and \"... and my knee hurts\" have two subjects, and the person's own body\n    is the one to answer: those lines belong to the faint card and the sore-knee card. The words\n    are the shared symptom list the composer already keeps (`bot/compose/fallback_data.py`), read\n    only after the ending, so the ending's own words (\"my heart sank\", \"couldnt breathe\") are not\n    the complaint.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    said = mcall(&g_A_BODY_ENDING(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    tail = slice_or_raise(&line.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None);
    after = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), tail.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&after.clone(), &op_bitor(&g__A_SYMPTOM(), &g__ALSO_A_COMPLAINT()))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_WENT_OUT_COLD(), "search", &[tail.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[op_bitand(&after.clone(), &g__A_BODY_WORD())]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__A_STATE_OF_A_BODY(), "search", &[tail.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_page_in_hand(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is only somebody holding or looking at a bill, statement or invoice.");
    return call_builtin("bool", &[{ let __b = mcall(&g__ONLY_THE_PAGE(), "match", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NOT_YET(), "search", &[line.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ending_and_the_page(args: &[Py]) -> Py {
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
    let mut document: Py = Py::None;
    let mut line: Py = Py::None;
    let mut page: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a body ending and a document are said of each other.\n\n    Two orders count, and each one has its own reason:\n    * the ending, then the page - \"i nearly died when i saw the bill\". This is the shape that\n      reached a bereavement card, a falls review and a dizzy spell.\n    * the page, then the ending - \"i read the invoice and nearly died\", \"the bill took my breath\n      away\". A page verb has to sit in front of the document, so \"i paid the bill and nearly died\n      on the motorway\" is left to the loss and safety modules: there the person was not reading.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_the_page_in_hand(&[line.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__NOT_YET(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_DOCUMENT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_BODY_ENDING(), "search", &[line.clone()]) } } } }) {
    return Py::Bool(true);
    }
    page = mcall(&g_A_SHOCK_AT_A_PAGE(), "search", &[line.clone()]);
    if py_truthy(&page.clone()) {
    return call_builtin("bool", &[mcall(&g_A_DOCUMENT(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    said = mcall(&g_A_BODY_ENDING(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    after = mcall(&g_A_DOCUMENT(), "search", &[line.clone(), mcall(&said.clone(), "end", &[])]);
    if py_truthy(&after.clone()) {
    return Py::Bool(true);
    }
    document = mcall(&g_A_DOCUMENT(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&document.clone()))) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&line.clone(), &Py::None, &mcall(&document.clone(), "start", &[]), &Py::None);
    return call_builtin("bool", &[mcall(&g_A_PAGE_VERB(), "search", &[slice_or_raise(&before.clone(), &op_neg(&Py::Int(24)), &Py::None, &Py::None)])]).unwrap_or_else(|e| exc::raise(e));
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
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("Two wordings, so a second bill ask does not repeat the first reply word for word.");
    let __try1 = exc::catch(|| {
    last = call_builtin("str", &[{ let __b = mcall(&geti(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![py_dict(vec![])]) } }, &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError", "IndexError", "TypeError"]) {
                __handled5 = true;
    last = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&lines.clone());
    for __item in __iter6 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&line.clone(), &last.clone()))) {
    return line.clone();
    }
    }
    return geti(&lines.clone(), &Py::Int(0));
    Py::None
}
pub fn _the_ending_is_somebody_elses(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the body ending is said of another person, whose news is the thing to answer.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    said = mcall(&g_A_BODY_ENDING(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__A_SOMEBODY_ELSE(), "search", &[slice_or_raise(&line.clone(), &Py::None, &mcall(&said.clone(), "start", &[]), &Py::None)])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_kind_word_in_front_of_the_page(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut document: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kind word the person put in front of a bill/statement/invoice, or \"\".");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_DOCUMENT(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    document = __item.clone();
    before = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&line.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&document.clone(), "start", &[]), &Py::Int(40))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&document.clone(), "start", &[]), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__A_WORD_ABOUT_ANY_BILL(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_WORD_BEFORE_A_KIND(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_ACCOUNT_WORD(), &word.clone())) } }) {
    return py_str("");
    }
    return word.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_bill_of_another_kind(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when the line names a kind of bill whose own card keeps the turn.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[_a_kind_word_in_front_of_the_page(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_BILL_WITH_ITS_OWN_CARD()) { let mut kind: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &kind.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("This line, because the module read it whole: a body ending and a document after it.\n\n    Without this the knowledge base answers first on most of these lines, and its card is about\n    the *ending* - a bereavement team, a falls review, a dizzy spell. The claim is not a band: the\n    module proposes on many lines it has no business taking (any bill at all), so it says which of\n    its own proposals is this one, on the line in front of it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_the_ending_and_the_page(&[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_complaint_added_after_the_ending(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_the_ending_is_somebody_elses(&[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_money_problem_in_the_line(&[text.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&_a_bill_of_another_kind(&[text.clone()])));
    Py::None
}
pub fn _ask_for_what_is_missing(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said_the_figure: Py = Py::None;
    let _ = py_str("The page-check, asking for the thing the line has not already said.");
    line = mcall(&py_str(" "), "join", &[mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[])]);
    if py_truthy(&mcall(&g__NOT_YET(), "search", &[line.clone()])) {
    return _pick(&[g__BEFORE_IT_COMES(), memory.clone()]);
    }
    said_the_figure = call_builtin("bool", &[mcall(&g__A_FIGURE(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    lines = if py_truthy(&said_the_figure.clone()) { g__WITHOUT_A_FIGURE() } else { g__BILL_LINES() };
    return _pick(&[lines.clone(), memory.clone()]);
    Py::None
}
pub fn _a_money_problem_in_the_line(args: &[Py]) -> Py {
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
    let mut _answers: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("True when the line names a money trouble *and* the knowledge base answers about the money.\n\n    Both halves are needed. The line alone is not enough: \"i nearly fell over when i saw the bill,\n    its been overcharged\" names a money trouble, and the only card behind it is the falls review\n    (the knowledge base read the ending), where the page-check is the better answer. The knowledge\n    base's own answer alone is not enough either: for \"i nearly died when i saw the bill\" it is the\n    bereavement team, which is the misreading this module exists to fix.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_MONEY_PROBLEM(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    _answers = pyrt::xrefs::xmodule("modules.answers");
    reply = mcall(&_answers.clone(), "respond", &[text.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return { let __b = call_builtin("bool", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_MONEY_IN_THE_ANSWER(), "search", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let _ = py_str("The bill's answer to a line that says a shock at a bill, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&_the_ending_and_the_page(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_a_complaint_added_after_the_ending(&[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_the_ending_is_somebody_elses(&[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_money_problem_in_the_line(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_bill_of_another_kind(&[text.clone()])) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), _ask_for_what_is_missing(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, text.clone()])]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_shock_at_a_bill".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BILL_WITH_ITS_OWN_CARD" => Some(g_A_BILL_WITH_ITS_OWN_CARD()),
            "A_BODY_ENDING" => Some(g_A_BODY_ENDING()),
            "A_DOCUMENT" => Some(g_A_DOCUMENT()),
            "A_MONEY_IN_THE_ANSWER" => Some(g_A_MONEY_IN_THE_ANSWER()),
            "A_MONEY_PROBLEM" => Some(g_A_MONEY_PROBLEM()),
            "A_PAGE_VERB" => Some(g_A_PAGE_VERB()),
            "A_SHOCK_AT_A_PAGE" => Some(g_A_SHOCK_AT_A_PAGE()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_ALSO_A_COMPLAINT" => Some(g__ALSO_A_COMPLAINT()),
            "_A_ACCOUNT_WORD" => Some(g__A_ACCOUNT_WORD()),
            "_A_BODY_WORD" => Some(g__A_BODY_WORD()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_SOMEBODY_ELSE" => Some(g__A_SOMEBODY_ELSE()),
            "_A_STATE_OF_A_BODY" => Some(g__A_STATE_OF_A_BODY()),
            "_A_SYMPTOM" => Some(g__A_SYMPTOM()),
            "_A_WENT_OUT_COLD" => Some(g__A_WENT_OUT_COLD()),
            "_A_WORD_ABOUT_ANY_BILL" => Some(g__A_WORD_ABOUT_ANY_BILL()),
            "_A_WORD_BEFORE_A_KIND" => Some(g__A_WORD_BEFORE_A_KIND()),
            "_BEFORE_IT_COMES" => Some(g__BEFORE_IT_COMES()),
            "_BILL_LINES" => Some(g__BILL_LINES()),
            "_NOT_YET" => Some(g__NOT_YET()),
            "_ONLY_THE_PAGE" => Some(g__ONLY_THE_PAGE()),
            "_WITHOUT_A_FIGURE" => Some(g__WITHOUT_A_FIGURE()),
            "_a_bill_of_another_kind" => Some(py_func("_a_bill_of_another_kind", |args: &[Py]| _a_bill_of_another_kind(args))),
            "_a_complaint_added_after_the_ending" => Some(py_func("_a_complaint_added_after_the_ending", |args: &[Py]| _a_complaint_added_after_the_ending(args))),
            "_a_kind_word_in_front_of_the_page" => Some(py_func("_a_kind_word_in_front_of_the_page", |args: &[Py]| _a_kind_word_in_front_of_the_page(args))),
            "_a_money_problem_in_the_line" => Some(py_func("_a_money_problem_in_the_line", |args: &[Py]| _a_money_problem_in_the_line(args))),
            "_ask_for_what_is_missing" => Some(py_func("_ask_for_what_is_missing", |args: &[Py]| _ask_for_what_is_missing(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_the_ending_and_the_page" => Some(py_func("_the_ending_and_the_page", |args: &[Py]| _the_ending_and_the_page(args))),
            "_the_ending_is_somebody_elses" => Some(py_func("_the_ending_is_somebody_elses", |args: &[Py]| _the_ending_is_somebody_elses(args))),
            "_the_page_in_hand" => Some(py_func("_the_page_in_hand", |args: &[Py]| _the_page_in_hand(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BILL_WITH_ITS_OWN_CARD" => s_A_BILL_WITH_ITS_OWN_CARD(v),
            "A_BODY_ENDING" => s_A_BODY_ENDING(v),
            "A_DOCUMENT" => s_A_DOCUMENT(v),
            "A_MONEY_IN_THE_ANSWER" => s_A_MONEY_IN_THE_ANSWER(v),
            "A_MONEY_PROBLEM" => s_A_MONEY_PROBLEM(v),
            "A_PAGE_VERB" => s_A_PAGE_VERB(v),
            "A_SHOCK_AT_A_PAGE" => s_A_SHOCK_AT_A_PAGE(v),
            "PRIORITY" => s_PRIORITY(v),
            "_ALSO_A_COMPLAINT" => s__ALSO_A_COMPLAINT(v),
            "_A_ACCOUNT_WORD" => s__A_ACCOUNT_WORD(v),
            "_A_BODY_WORD" => s__A_BODY_WORD(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            "_A_SOMEBODY_ELSE" => s__A_SOMEBODY_ELSE(v),
            "_A_STATE_OF_A_BODY" => s__A_STATE_OF_A_BODY(v),
            "_A_SYMPTOM" => s__A_SYMPTOM(v),
            "_A_WENT_OUT_COLD" => s__A_WENT_OUT_COLD(v),
            "_A_WORD_ABOUT_ANY_BILL" => s__A_WORD_ABOUT_ANY_BILL(v),
            "_A_WORD_BEFORE_A_KIND" => s__A_WORD_BEFORE_A_KIND(v),
            "_BEFORE_IT_COMES" => s__BEFORE_IT_COMES(v),
            "_BILL_LINES" => s__BILL_LINES(v),
            "_NOT_YET" => s__NOT_YET(v),
            "_ONLY_THE_PAGE" => s__ONLY_THE_PAGE(v),
            "_WITHOUT_A_FIGURE" => s__WITHOUT_A_FIGURE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_shock_at_a_bill", module_obj);
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_a_complaint_added_after_the_ending", py_func("_a_complaint_added_after_the_ending", |args: &[Py]| _a_complaint_added_after_the_ending(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_the_page_in_hand", py_func("_the_page_in_hand", |args: &[Py]| _the_page_in_hand(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_the_ending_and_the_page", py_func("_the_ending_and_the_page", |args: &[Py]| _the_ending_and_the_page(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_the_ending_is_somebody_elses", py_func("_the_ending_is_somebody_elses", |args: &[Py]| _the_ending_is_somebody_elses(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_a_kind_word_in_front_of_the_page", py_func("_a_kind_word_in_front_of_the_page", |args: &[Py]| _a_kind_word_in_front_of_the_page(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_a_bill_of_another_kind", py_func("_a_bill_of_another_kind", |args: &[Py]| _a_bill_of_another_kind(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_ask_for_what_is_missing", py_func("_ask_for_what_is_missing", |args: &[Py]| _ask_for_what_is_missing(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "_a_money_problem_in_the_line", py_func("_a_money_problem_in_the_line", |args: &[Py]| _a_money_problem_in_the_line(args)));
    pyrt::xrefs::reg_fn("modules.a_shock_at_a_bill", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_BILL_WITH_ITS_OWN_CARD", g_A_BILL_WITH_ITS_OWN_CARD, s_A_BILL_WITH_ITS_OWN_CARD);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_BODY_ENDING", g_A_BODY_ENDING, s_A_BODY_ENDING);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_DOCUMENT", g_A_DOCUMENT, s_A_DOCUMENT);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_MONEY_IN_THE_ANSWER", g_A_MONEY_IN_THE_ANSWER, s_A_MONEY_IN_THE_ANSWER);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_MONEY_PROBLEM", g_A_MONEY_PROBLEM, s_A_MONEY_PROBLEM);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_PAGE_VERB", g_A_PAGE_VERB, s_A_PAGE_VERB);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "A_SHOCK_AT_A_PAGE", g_A_SHOCK_AT_A_PAGE, s_A_SHOCK_AT_A_PAGE);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_ALSO_A_COMPLAINT", g__ALSO_A_COMPLAINT, s__ALSO_A_COMPLAINT);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_ACCOUNT_WORD", g__A_ACCOUNT_WORD, s__A_ACCOUNT_WORD);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_BODY_WORD", g__A_BODY_WORD, s__A_BODY_WORD);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_SOMEBODY_ELSE", g__A_SOMEBODY_ELSE, s__A_SOMEBODY_ELSE);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_STATE_OF_A_BODY", g__A_STATE_OF_A_BODY, s__A_STATE_OF_A_BODY);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_SYMPTOM", g__A_SYMPTOM, s__A_SYMPTOM);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_WENT_OUT_COLD", g__A_WENT_OUT_COLD, s__A_WENT_OUT_COLD);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_WORD_ABOUT_ANY_BILL", g__A_WORD_ABOUT_ANY_BILL, s__A_WORD_ABOUT_ANY_BILL);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_A_WORD_BEFORE_A_KIND", g__A_WORD_BEFORE_A_KIND, s__A_WORD_BEFORE_A_KIND);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_BEFORE_IT_COMES", g__BEFORE_IT_COMES, s__BEFORE_IT_COMES);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_BILL_LINES", g__BILL_LINES, s__BILL_LINES);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_NOT_YET", g__NOT_YET, s__NOT_YET);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_ONLY_THE_PAGE", g__ONLY_THE_PAGE, s__ONLY_THE_PAGE);
    pyrt::xrefs::reg_access("modules.a_shock_at_a_bill", "_WITHOUT_A_FIGURE", g__WITHOUT_A_FIGURE, s__WITHOUT_A_FIGURE);
}
