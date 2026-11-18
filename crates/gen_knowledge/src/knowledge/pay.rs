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
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_AND_NET_COMPARED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_AND_NET_GROSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_AND_NET_INCLUDES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_AND_NET_INCLUDES_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_AND_NET_NET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_AND_NET_THREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROSS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUOTED_PAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_AND_NET_COMPARED() -> Py { ensure_init(); G__GROSS_AND_NET_COMPARED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_AND_NET_COMPARED(v: Py) { ensure_init(); G__GROSS_AND_NET_COMPARED.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_AND_NET_GROSS() -> Py { ensure_init(); G__GROSS_AND_NET_GROSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_AND_NET_GROSS(v: Py) { ensure_init(); G__GROSS_AND_NET_GROSS.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_AND_NET_INCLUDES() -> Py { ensure_init(); G__GROSS_AND_NET_INCLUDES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_AND_NET_INCLUDES(v: Py) { ensure_init(); G__GROSS_AND_NET_INCLUDES.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_AND_NET_INCLUDES_REPLY() -> Py { ensure_init(); G__GROSS_AND_NET_INCLUDES_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_AND_NET_INCLUDES_REPLY(v: Py) { ensure_init(); G__GROSS_AND_NET_INCLUDES_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_AND_NET_NET() -> Py { ensure_init(); G__GROSS_AND_NET_NET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_AND_NET_NET(v: Py) { ensure_init(); G__GROSS_AND_NET_NET.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_AND_NET_THREE() -> Py { ensure_init(); G__GROSS_AND_NET_THREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_AND_NET_THREE(v: Py) { ensure_init(); G__GROSS_AND_NET_THREE.with(|c| *c.borrow_mut() = v); }
pub fn g__GROSS_WORDS() -> Py { ensure_init(); G__GROSS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROSS_WORDS(v: Py) { ensure_init(); G__GROSS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NET_WORDS() -> Py { ensure_init(); G__NET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NET_WORDS(v: Py) { ensure_init(); G__NET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PAY_WORDS() -> Py { ensure_init(); G__PAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAY_WORDS(v: Py) { ensure_init(); G__PAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__QUOTED_PAY_WORDS() -> Py { ensure_init(); G__QUOTED_PAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUOTED_PAY_WORDS(v: Py) { ensure_init(); G__QUOTED_PAY_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The words on a payslip: gross, net, take home, PAYE, and the student loan.\n\nTwo numbers sit at the top of a payslip and most of what people ask about pay is the\ndifference between them. \"What is net pay\" had a card; \"what is gross pay\", \"is gross before\ntax\", \"does gross include tax\", \"gross vs net\", \"what does take home mean\", \"salary before\ntax\", \"does paye come out of my pay\" and \"does student loan come out of my wages\" did not -\nthey met a shrug, the duty-free allowance, the income-tax rates, or \"that is an order to work\nout, not a taste to pick\".\n\nSo the three questions are keyed on what the line is about rather than on the phrase the\nknowledge base happened to have:\n\n- `gross-and-net`, keyed on the two words people use and the pay they are about, so one row\n  answers every wording, and the reply leads with the half of the pair that was asked about.\n- `paye-taken-from-pay`, because an acronym asked about as a deduction from pay is the same\n  question as the acronym asked by name, and the name row only answers the name.\n- `student-loan-taken-from-pay`, for the same reason: the money row answers whether it is\n  worth paying off early, which is a different question from what comes off this month.\n\nThe avoid lists are what keeps them narrow: \"this soup is gross\", \"gross misconduct\", \"net\nworth\", \"net weight\", \"net zero\", a fishing net and a netball are not pay, and \"income tax\",\n\"tax code\", \"how much tax on 30k\", \"minimum wage\" and \"a week\" each have their own answer.\n");
    s__GROSS_WORDS(py_tuple(vec![py_str("gross"), py_str("gross pay"), py_str("gross salary"), py_str("gross income"), py_str("gross wage"), py_str("gross wages"), py_str("gross earnings"), py_str("before tax"), py_str("pre tax"), py_str("pre-tax"), py_str("pretax"), py_str("before deductions"), py_str("before its taxed"), py_str("before it is taxed")]));
    s__QUOTED_PAY_WORDS(py_tuple(vec![py_str("salary"), py_str("salaries"), py_str("wage"), py_str("wages"), py_str("pay"), py_str("income"), py_str("earnings"), py_str("pay packet"), py_str("the offer"), py_str("the advert"), py_str("the job advert"), py_str("the money"), py_str("20k"), py_str("25k"), py_str("30k"), py_str("35k"), py_str("40k"), py_str("45k"), py_str("50k"), py_str("60k"), py_str("20,000"), py_str("25,000"), py_str("30,000"), py_str("35,000"), py_str("40,000"), py_str("50,000")]));
    s__NET_WORDS(py_tuple(vec![py_str("net"), py_str("net pay"), py_str("net salary"), py_str("net income"), py_str("net wage"), py_str("net wages"), py_str("net earnings"), py_str("net of tax"), py_str("net of tax pay"), py_str("after tax"), py_str("after-tax"), py_str("after deductions"), py_str("take home"), py_str("take-home"), py_str("takehome"), py_str("take home pay"), py_str("take-home pay"), py_str("takehome pay"), py_str("what lands in the bank"), py_str("in my hand"), py_str("in my pocket")]));
    s__PAY_WORDS(py_tuple(vec![py_str("pay"), py_str("paid"), py_str("salary"), py_str("salaries"), py_str("wage"), py_str("wages"), py_str("income"), py_str("earnings"), py_str("pay packet"), py_str("paypacket"), py_str("payslip"), py_str("pay slip"), py_str("monthly pay"), py_str("weekly pay"), py_str("hourly pay"), py_str("my gross"), py_str("my net"), py_str("the gross"), py_str("the net"), py_str("a gross"), py_str("a net"), py_str("take home"), py_str("take-home"), py_str("takehome")]));
    s__GROSS_AND_NET_COMPARED(py_tuple(vec![py_str("difference"), py_str("different"), py_str("vs"), py_str("versus"), py_str("or"), py_str("between"), py_str("compare"), py_str("compared"), py_str("which is"), py_str("same"), py_str("higher"), py_str("bigger"), py_str("more than"), py_str("less than")]));
    s__GROSS_AND_NET_INCLUDES(py_tuple(vec![py_str("include"), py_str("includes"), py_str("included"), py_str("quoted"), py_str("advertised"), py_str("advert"), py_str("offer"), py_str("offered"), py_str("the number on the advert"), py_str("the figure")]));
    s__GROSS_AND_NET_THREE(py_str("Gross is what you earned before anything is taken, net (or take-home) is what lands in the bank, and the difference is income tax, National Insurance and any pension you pay in. A pay figure is always quoted gross unless it says take-home, so a job advertised at £30,000 is about £2,000 a month in your hand on a standard code, and less if you pay into a pension. Check the tax code against your HMRC coding notice (1257L is the standard one): a wrong code is the usual reason a wage comes out short."));
    s__GROSS_AND_NET_GROSS(py_str("Gross is your pay before anything is taken out of it: the number in the job advert, on the contract and at the top of the payslip, before income tax, National Insurance and pension. What actually lands in your bank is net, or take-home, and the gap between them is those deductions - about a fifth of it on an ordinary wage. So a salary quoted at £30,000 is gross unless it says take-home."));
    s__GROSS_AND_NET_NET(py_str("Net, or take-home, is what lands in your bank after income tax, National Insurance and any pension have come off. Gross is the number before those - the one in the advert and on the contract - so the two differ by the deductions on the payslip. If the net figure is less than you expected, check the tax code first: a wrong one is the commonest reason."));
    s__GROSS_AND_NET_INCLUDES_REPLY(py_str("No: a quoted salary or wage is gross, before tax, unless the advert or the offer says take-home. So £30,000 a year is about £2,000 a month after income tax and National Insurance on a standard code, and less if you pay into a pension. Ask for the figure in writing and check whether a pension is included, because that changes what you actually get."));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gross-and-net")); __args.push(py_tuple(vec![op_add(&op_add(&g__GROSS_WORDS(), &g__NET_WORDS()), &g__QUOTED_PAY_WORDS()), op_add(&g__PAY_WORDS(), &py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what does"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("difference"), py_str("different"), py_str("vs"), py_str("versus"), py_str("or"), py_str("between"), py_str("include"), py_str("includes"), py_str("included"), py_str("quoted"), py_str("advertised"), py_str("advert"), py_str("salary"), py_str("offer"), py_str("compare"), py_str("compared"), py_str("explain"), py_str("which"), py_str("is"), py_str("are"), py_str("was"), py_str("does"), py_str("do"), py_str("did"), py_str("include tax"), py_str("includes tax"), py_str("tax included"), py_str("tax on it")]))])); __args.push(g__gross_or_net()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__about_pay()), ("avoid".to_string(), py_tuple(vec![py_str("misconduct"), py_str("negligence"), py_str("grossly"), py_str("grossed out"), py_str("gross me out"), py_str("gross out"), py_str("disgusting"), py_str("net worth"), py_str("net weight"), py_str("gross weight"), py_str("drained weight"), py_str("net zero"), py_str("safety net"), py_str("fishing"), py_str("netball"), py_str("net profit"), py_str("net margin"), py_str("net asset"), py_str("turnover"), py_str("butterfly net"), py_str("tennis net"), py_str("keep net"), py_str("net curtains"), py_str("moss"), py_str("internet"), py_str("netflix"), py_str("income tax"), py_str("tax code"), py_str("tax return"), py_str("tax year"), py_str("national insurance"), py_str("salary sacrifice"), py_str("how much tax"), py_str("tax on"), py_str("tax bracket"), py_str("tax bill"), py_str("self assessment"), py_str("underpaid"), py_str("overpaid"), py_str("emergency tax"), py_str("average salary"), py_str("average wage"), py_str("median"), py_str("minimum wage"), py_str("living wage"), py_str("payslip"), py_str("a week"), py_str("a month"), py_str("an hour"), py_str("a day"), py_str("per week"), py_str("per month"), py_str("per hour"), py_str("what is that a week"), py_str("what is that a month")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("paye-taken-from-pay")); __args.push(py_tuple(vec![py_tuple(vec![py_str("paye"), py_str("paye tax"), py_str("pay as you earn")]), py_tuple(vec![py_str("come out"), py_str("comes out"), py_str("comes off"), py_str("taken off"), py_str("taken out"), py_str("taken from"), py_str("taken from my pay"), py_str("deducted"), py_str("deducted from"), py_str("deduction"), py_str("deductions"), py_str("out of my pay"), py_str("off my pay"), py_str("out of my wages"), py_str("off my wages"), py_str("from my pay"), py_str("out of my salary"), py_str("off my salary"), py_str("on my payslip"), py_str("stop"), py_str("stops"), py_str("stopped"), py_str("pay paye"), py_str("paid paye"), py_str("do i pay"), py_str("am i paying"), py_str("paye taken")])])); __args.push(py_str("Yes: PAYE is the income tax your employer takes out of your pay before it reaches you, worked out from your tax code each payday and shown on the payslip, so the number you are paid is already after it. National Insurance comes off the same way, and a pension or a student loan is listed as its own line. Compare the code on your payslip with your HMRC coding notice (1257L is the standard one): a wrong code is the usual reason PAYE takes too much.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("tax code"), py_str("wrong"), py_str("too much"), py_str("too high"), py_str("overpaid"), py_str("underpaid"), py_str("refund"), py_str("rebate"), py_str("emergency tax"), py_str("tax return"), py_str("self assessment"), py_str("how much tax"), py_str("national insurance"), py_str("p60"), py_str("p45"), py_str("self employed"), py_str("cis"), py_str("what is paye"), py_str("what does paye"), py_str("mean"), py_str("explain")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("student-loan-taken-from-pay")); __args.push(py_tuple(vec![py_tuple(vec![py_str("student loan"), py_str("student loans"), py_str("student debt"), py_str("student loan repayment"), py_str("student loan repayments")]), py_tuple(vec![py_str("come out"), py_str("comes out"), py_str("comes off"), py_str("taken off"), py_str("taken out"), py_str("deducted"), py_str("deduction"), py_str("deductions"), py_str("out of my pay"), py_str("off my pay"), py_str("out of my wages"), py_str("off my wages"), py_str("from my pay"), py_str("out of my salary"), py_str("off my salary"), py_str("on my payslip"), py_str("stop"), py_str("stops"), py_str("stopped"), py_str("pay student loan"), py_str("paid student loan"), py_str("do i pay"), py_str("am i paying")])])); __args.push(py_str("Yes: a student loan comes off your pay like tax, through PAYE, once you earn over the threshold for your plan - on plan 5 that is 9% of what you earn above about £25,000 a year (£2,082 a month), shown as its own line on the payslip. Only the part above the threshold counts, so a low month takes nothing, and it stops by itself when the loan is paid off. If the figure looks wrong, ask payroll for the plan they have you on: the wrong plan is the usual cause.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how much"), py_str("how much is"), py_str("pay off"), py_str("pay it back"), py_str("pay back"), py_str("repay early"), py_str("overpay"), py_str("overpayment"), py_str("worth"), py_str("settle"), py_str("write off"), py_str("written off"), py_str("forgiven"), py_str("cancel"), py_str("plan 1"), py_str("plan 2"), py_str("plan 4"), py_str("threshold"), py_str("what is a student loan"), py_str("interest rate")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    s_ANSWERS(py_tuple(vec![py_str("money")]));
}
pub fn _about_pay(args: &[Py]) -> Py {
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
    let mut ch: Py = Py::None;
    let mut says_gross: Py = Py::None;
    let mut says_net: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Gross or net is about pay: a bare \"gross\" is also the word for disgusting.\n\n    A pay word, \"net\", \"take home\", or the tax the pair is always about is what makes the line a\n    payslip question. \"this is gross\", \"gross misconduct\" and \"gross negligence\" have none of\n    them and are left where they were.\n    ");
    text = op_modulo(&py_str(" %s "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]));
    says_gross = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GROSS_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    says_net = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NET_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&text.clone()) { let mut ch: Py = __item.clone(); __out.push(mcall(&ch.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("include"), py_str("includes"), py_str("included"), py_str("include tax"), py_str("includes tax"), py_str("tax included"), py_str("before or after"), py_str("after or before")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = says_gross.clone(); if !py_truthy(&__b) { __b } else { says_net.clone() } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = says_gross.clone(); if py_truthy(&__b) { __b } else { says_net.clone() } }) {
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PAY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("tax"), py_str("taxed")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__QUOTED_PAY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("tax"), py_str("taxed")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("include"), py_str("includes"), py_str("included"), py_str("taxed"), py_str("before"), py_str("after"), py_str("quoted"), py_str("advertised")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _gross_or_net(args: &[Py]) -> Py {
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
    let mut says_gross: Py = Py::None;
    let mut says_net: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The half of the pair the line asked about, or the two together.");
    text = op_modulo(&py_str(" %s "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]));
    says_gross = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("gross"), py_str("gross pay"), py_str("gross salary"), py_str("gross income"), py_str("before tax"), py_str("pre tax"), py_str("pre-tax"), py_str("pretax")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    says_net = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("net"), py_str("net pay"), py_str("net income"), py_str("after tax"), py_str("take home"), py_str("take-home"), py_str("takehome"), py_str("take home pay"), py_str("net of tax")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = says_gross.clone(); if !py_truthy(&__b) { __b } else { says_net.clone() } }) {
    return g__GROSS_AND_NET_THREE();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str(" before or after "))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str(" after or before "))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str(" gross or net "))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str(" net or gross "))) } } } } } }) {
    return g__GROSS_AND_NET_THREE();
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GROSS_AND_NET_COMPARED()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { says_gross.clone() } }) {
    return g__GROSS_AND_NET_THREE();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GROSS_AND_NET_INCLUDES()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__GROSS_AND_NET_INCLUDES_REPLY();
    }
    if py_truthy(&says_net.clone()) {
    return g__GROSS_AND_NET_NET();
    }
    if py_truthy(&says_gross.clone()) {
    return g__GROSS_AND_NET_GROSS();
    }
    return g__GROSS_AND_NET_THREE();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.pay".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_GROSS_AND_NET_COMPARED" => Some(g__GROSS_AND_NET_COMPARED()),
            "_GROSS_AND_NET_GROSS" => Some(g__GROSS_AND_NET_GROSS()),
            "_GROSS_AND_NET_INCLUDES" => Some(g__GROSS_AND_NET_INCLUDES()),
            "_GROSS_AND_NET_INCLUDES_REPLY" => Some(g__GROSS_AND_NET_INCLUDES_REPLY()),
            "_GROSS_AND_NET_NET" => Some(g__GROSS_AND_NET_NET()),
            "_GROSS_AND_NET_THREE" => Some(g__GROSS_AND_NET_THREE()),
            "_GROSS_WORDS" => Some(g__GROSS_WORDS()),
            "_NET_WORDS" => Some(g__NET_WORDS()),
            "_PAY_WORDS" => Some(g__PAY_WORDS()),
            "_QUOTED_PAY_WORDS" => Some(g__QUOTED_PAY_WORDS()),
            "_about_pay" => Some(py_func("_about_pay", |args: &[Py]| _about_pay(args))),
            "_gross_or_net" => Some(py_func("_gross_or_net", |args: &[Py]| _gross_or_net(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_GROSS_AND_NET_COMPARED" => s__GROSS_AND_NET_COMPARED(v),
            "_GROSS_AND_NET_GROSS" => s__GROSS_AND_NET_GROSS(v),
            "_GROSS_AND_NET_INCLUDES" => s__GROSS_AND_NET_INCLUDES(v),
            "_GROSS_AND_NET_INCLUDES_REPLY" => s__GROSS_AND_NET_INCLUDES_REPLY(v),
            "_GROSS_AND_NET_NET" => s__GROSS_AND_NET_NET(v),
            "_GROSS_AND_NET_THREE" => s__GROSS_AND_NET_THREE(v),
            "_GROSS_WORDS" => s__GROSS_WORDS(v),
            "_NET_WORDS" => s__NET_WORDS(v),
            "_PAY_WORDS" => s__PAY_WORDS(v),
            "_QUOTED_PAY_WORDS" => s__QUOTED_PAY_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.pay", module_obj);
    pyrt::xrefs::reg_fn("knowledge.pay", "_about_pay", py_func("_about_pay", |args: &[Py]| _about_pay(args)));
    pyrt::xrefs::reg_fn("knowledge.pay", "_gross_or_net", py_func("_gross_or_net", |args: &[Py]| _gross_or_net(args)));
    pyrt::xrefs::reg_access("knowledge.pay", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.pay", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_AND_NET_COMPARED", g__GROSS_AND_NET_COMPARED, s__GROSS_AND_NET_COMPARED);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_AND_NET_GROSS", g__GROSS_AND_NET_GROSS, s__GROSS_AND_NET_GROSS);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_AND_NET_INCLUDES", g__GROSS_AND_NET_INCLUDES, s__GROSS_AND_NET_INCLUDES);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_AND_NET_INCLUDES_REPLY", g__GROSS_AND_NET_INCLUDES_REPLY, s__GROSS_AND_NET_INCLUDES_REPLY);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_AND_NET_NET", g__GROSS_AND_NET_NET, s__GROSS_AND_NET_NET);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_AND_NET_THREE", g__GROSS_AND_NET_THREE, s__GROSS_AND_NET_THREE);
    pyrt::xrefs::reg_access("knowledge.pay", "_GROSS_WORDS", g__GROSS_WORDS, s__GROSS_WORDS);
    pyrt::xrefs::reg_access("knowledge.pay", "_NET_WORDS", g__NET_WORDS, s__NET_WORDS);
    pyrt::xrefs::reg_access("knowledge.pay", "_PAY_WORDS", g__PAY_WORDS, s__PAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.pay", "_QUOTED_PAY_WORDS", g__QUOTED_PAY_WORDS, s__QUOTED_PAY_WORDS);
}
