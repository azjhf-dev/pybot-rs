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
thread_local! { static G__AN_ANIMAL_CARED_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ANIMAL_WEARS_ME_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_KEPT_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARERS_ASSESSMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARERS_PAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOOKED_AFTER_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MY_FAMILY_CARER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_CARERS_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THIS_ROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_CARER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_LOAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.caring: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ANIMAL_CARED_FOR() -> Py { ensure_init(); G__AN_ANIMAL_CARED_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ANIMAL_CARED_FOR(v: Py) { ensure_init(); G__AN_ANIMAL_CARED_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ANIMAL_WEARS_ME_OUT() -> Py { ensure_init(); G__AN_ANIMAL_WEARS_ME_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ANIMAL_WEARS_ME_OUT(v: Py) { ensure_init(); G__AN_ANIMAL_WEARS_ME_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING() -> Py { ensure_init(); G__ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING(v: Py) { ensure_init(); G__ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_KEPT_ANIMAL() -> Py { ensure_init(); G__A_KEPT_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_KEPT_ANIMAL(v: Py) { ensure_init(); G__A_KEPT_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g__CARERS_ASSESSMENT() -> Py { ensure_init(); G__CARERS_ASSESSMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARERS_ASSESSMENT(v: Py) { ensure_init(); G__CARERS_ASSESSMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__CARERS_PAY() -> Py { ensure_init(); G__CARERS_PAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARERS_PAY(v: Py) { ensure_init(); G__CARERS_PAY.with(|c| *c.borrow_mut() = v); }
pub fn g__LOOKED_AFTER_ME() -> Py { ensure_init(); G__LOOKED_AFTER_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOOKED_AFTER_ME(v: Py) { ensure_init(); G__LOOKED_AFTER_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__MY_FAMILY_CARER() -> Py { ensure_init(); G__MY_FAMILY_CARER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MY_FAMILY_CARER(v: Py) { ensure_init(); G__MY_FAMILY_CARER.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_CARERS_MONEY() -> Py { ensure_init(); G__NOT_THE_CARERS_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_CARERS_MONEY(v: Py) { ensure_init(); G__NOT_THE_CARERS_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_JOB() -> Py { ensure_init(); G__NOT_THE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_JOB(v: Py) { ensure_init(); G__NOT_THE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THIS_ROW() -> Py { ensure_init(); G__NOT_THIS_ROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THIS_ROW(v: Py) { ensure_init(); G__NOT_THIS_ROW.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_CARER() -> Py { ensure_init(); G__THE_CARER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_CARER(v: Py) { ensure_init(); G__THE_CARER.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_LOAD() -> Py { ensure_init(); G__THE_LOAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_LOAD(v: Py) { ensure_init(); G__THE_LOAD.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Caring for somebody: the break from it, and the words for the last part of it.\n\nThe answers lane owns this domain. Most people who look after a parent, a partner or a\nneighbour find out what help exists years late, and the questions about the help itself are\nfew: respite, and what a hospice and palliative care actually are. Every entry is keyed by the\nthing asked about rather than by the wording, so \"what is palliative care\", \"how do i get a\nreferral to a hospice\" and \"can i get a break from caring\" each reach their own row.\n\nThe two money-and-rights rows used to be somebody else's on paper - Carer's Allowance named as\nbot/knowledge/money.py's and the carer's assessment as bot/knowledge/lifeadmin.py's - and neither\nfile ever got one, so the questions fell to the search hedge (\"what is carer's allowance\") and to\nthe disabled person's bus pass (\"can i get a carer's assessment\"). They are here now, because\nthe line carries a carer in it and this is the carer's domain. Attendance Allowance is still\nmoney.py's, with the two rates and form AA1.\n\nWhy its own domain: the money rows answer benefits as a figure, and the health rows answer a\nsymptom. These are neither - they are what a carer can claim or ask for, on behalf of somebody\nelse - and the lines carry another person in every one of them (\"my dad\", \"mum\"), so the\nsubject rows in money.py and health.py stand down where these answer.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("money"), py_str("health"), py_str("family-and-parenting")]));
    fn __data1() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("whats"), py_str("who"), py_str("which"), py_str("can i"), py_str("could i"), py_str("do i"), py_str("does"), py_str("is"), py_str("are"), py_str("should i"), py_str("much"), py_str("many"), py_str("claim"), py_str("claiming"), py_str("apply"), py_str("applying"), py_str("qualify"), py_str("qualifies"), py_str("eligible"), py_str("entitled"), py_str("worth"), py_str("explain"), py_str("mean"), py_str("means"), py_str("about"), py_str("tell"), py_str("long"), py_str("pay"), py_str("paid"), py_str("start"), py_str("receive"), py_str("receiving")]) }
    s__ASKING(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("job"), py_str("jobs"), py_str("work at"), py_str("work in"), py_str("working at"), py_str("working in"), py_str("career"), py_str("volunteer"), py_str("volunteering"), py_str("salary"), py_str("interview"), py_str("nurse do"), py_str("nurses do"), py_str("training to be"), py_str("study to be")]) }
    s__NOT_THE_JOB(__data2());
    s__A_KEPT_ANIMAL(py_str("(?:dogs?|puppies|puppy|pups?|cats?|kittens?|rabbits?|guinea pigs?|hamsters?|fish|goldfish|fish tank|tropical fish)"));
    s__AN_ANIMAL_CARED_FOR(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:caring for|care for|caring of|looking after|look after|i care for|i look after)\\s+(?:my|our|the|a|an|his|her|their|two|three|both|elderly|old|sick|poor|new|little|baby|big)?\\s*(?:\\w+\\s+){0,1}?"), &g__A_KEPT_ANIMAL()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ANIMAL_WEARS_ME_OUT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|the)\\s+(?:\\w+\\s+){0,1}?"), &g__A_KEPT_ANIMAL()), &py_str("\\s+(?:is|are|has|have|keeps?|takes|took)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("cope"), py_str("coping"), py_str("cope with"), py_str("cant cope"), py_str("cannot cope"), py_str("not coping"), py_str("how do i cope"), py_str("how do i keep going"), py_str("keep going"), py_str("exhausted"), py_str("knackered"), py_str("worn out"), py_str("wearing me out"), py_str("wears me out"), py_str("burnt out"), py_str("burned out"), py_str("burnout"), py_str("burn out"), py_str("overwhelmed"), py_str("struggling"), py_str("struggling to cope"), py_str("no time for myself"), py_str("no life"), py_str("no life of my own"), py_str("on my own with it"), py_str("all on me"), py_str("it never stops"), py_str("never get a break"), py_str("no help"), py_str("help me"), py_str("tiring"), py_str("so tiring"), py_str("draining"), py_str("is a lot"), py_str("a lot of work"), py_str("hard work"), py_str("is hard"), py_str("hard going"), py_str("knackering"), py_str("shattered"), py_str("worn down"), py_str("getting to me"), py_str("help for carers"), py_str("support for carers"), py_str("look after myself"), py_str("run ragged"), py_str("dont know how much longer")]) }
    s__THE_LOAD(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("carer"), py_str("carers"), py_str("carer burnout"), py_str("carer stress"), py_str("unpaid carer"), py_str("full time carer"), py_str("i am a carer"), py_str("im a carer"), py_str("being a carer"), py_str("caring for my"), py_str("care for my"), py_str("caring for a"), py_str("care of my")]) }
    s__THE_CARER(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("carer allowance"), py_str("carers allowance"), py_str("carer's allowance"), py_str("allowance"), py_str("allowances"), py_str("benefit"), py_str("benefits"), py_str("pip"), py_str("universal credit"), py_str("tax credit"), py_str("bus pass"), py_str("pass"), py_str("travel"), py_str("blue badge"), py_str("badge"), py_str("motability"), py_str("money"), py_str("paid"), py_str("pay"), py_str("claim"), py_str("claiming"), py_str("qualify"), py_str("assessment"), py_str("how much"), py_str("how many hours")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("respite-care")); __args.push(py_tuple(vec![py_tuple(vec![py_str("respite"), py_str("respite care"), py_str("respite break"), py_str("short break from caring"), py_str("a break from caring")]), g__ASKING()])); __args.push(py_str("Respite care is a break for the person doing the caring, and it comes in small pieces: a few hours a week at a day centre, somebody sitting with them so you can go out, or a week in a care home so you can have a holiday. Arrange it before you are desperate, because the good places have a waiting list. The council can arrange it after a carer's assessment and means-tests the person you care for, not you; hospices and charities often have free day places and short stays; and the GP or district nurse knows what exists locally, which is usually more than the council website says.")); __kws.extend(vec![("ask".to_string(), g__ASKING()), ("avoid".to_string(), g__NOT_THE_JOB()), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hospice")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hospice"), py_str("hospices"), py_str("hospice care")]), g__ASKING()])); __args.push(py_str("A hospice is for controlling pain and symptoms and for supporting the person doing the caring - it is not a place people are sent only to die. Most of what they do is day services, help at home and short stays, and people are usually referred months before the end, which is the whole point: it is easier to sort pain out early than in a crisis. The care is free (charity and NHS funded) and needs a referral from the GP, district nurse or hospital team, so ask for one; you can also ring the hospice and ask what they have, because they answer that question all day.")); __kws.extend(vec![("ask".to_string(), g__ASKING()), ("avoid".to_string(), op_add(&g__NOT_THE_JOB(), &py_tuple(vec![py_str("how does the hospice work")]))), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("palliative-care")); __args.push(py_tuple(vec![py_tuple(vec![py_str("palliative care"), py_str("palliative"), py_str("end of life care"), py_str("palliative care team")]), g__ASKING()])); __args.push(py_str("Palliative care is treatment that controls symptoms and keeps somebody comfortable when a cure is not the aim - and it can run for years alongside treatment for the illness, so it does not mean care has been given up on. It is about pain, breathlessness, sickness, sleep and worry, it happens wherever the person is (at home, in a hospice, in hospital or in a care home), and the GP or specialist nurse can refer. Asking for it early is the thing that makes the last months better rather than the last weeks.")); __kws.extend(vec![("ask".to_string(), g__ASKING()), ("avoid".to_string(), g__NOT_THE_JOB()), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("carer-burnout")); __args.push(py_tuple(vec![g__THE_CARER(), g__THE_LOAD()])); __args.push(py_str("The first thing to ask for is a carer's assessment from your council: it is free, it is about you rather than the person you care for, and it can pay for someone to sit with them so you get a few hours back each week. Then tell your GP that you are caring - it goes on your notes, it counts for your own health, and it changes what they offer you. Ring Carers UK on 0808 808 7777 once, because they know the local schemes and the grants better than any website does, and ask your employer about flexible hours and unpaid carer's leave, which is now a day-one right. Book the break before you need it: a few hours a week arranged in advance is the difference between coping and going under, and taking it is not a failure.")); __kws.extend(vec![("ask".to_string(), g__THE_LOAD()), ("guard".to_string(), py_func("_an_animal_is_the_one_cared_for", |args: &[Py]| _an_animal_is_the_one_cared_for(args))), ("avoid".to_string(), op_add(&g__NOT_THE_JOB(), &__data5())), ("bias".to_string(), Py::Int(1)), ("report".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    fn __data6() -> Py { py_tuple(vec![py_str("carer's allowance"), py_str("carers allowance"), py_str("carer allowance"), py_str("carers allowance"), py_str("carer's credit"), py_str("carers credit"), py_str("carer credit"), py_str("paid for caring"), py_str("paid to care"), py_str("pay for caring"), py_str("money for caring"), py_str("paid for looking after"), py_str("paid to look after"), py_str("get paid for caring"), py_str("get paid to care"), py_str("get paid for looking after"), py_str("get paid to look after"), py_str("get paid as a carer"), py_str("paid as a carer"), py_str("get paid to be a carer"), py_str("paid for being a carer"), py_str("carer's allowance rate"), py_str("carers allowance amount")]) }
    s__CARERS_PAY(__data6());
    s__CARERS_ASSESSMENT(py_tuple(vec![py_str("carer's assessment"), py_str("carers assessment"), py_str("carer assessment"), py_str("assessment as a carer"), py_str("carer's needs assessment"), py_str("carers needs assessment"), py_str("support as a carer"), py_str("help as a carer"), py_str("help for carers"), py_str("support for carers"), py_str("carer support"), py_str("what help can i get as a carer")]));
    fn __data7() -> Py { py_tuple(vec![py_str("childminder"), py_str("childminders"), py_str("childminding"), py_str("babysit"), py_str("babysitting"), py_str("nanny"), py_str("fostering"), py_str("foster"), py_str("adoption"), py_str("nursery"), py_str("au pair"), py_str("care worker"), py_str("care assistant"), py_str("support worker"), py_str("care home job"), py_str("agency work")]) }
    s__NOT_THE_CARERS_MONEY(op_add(&g__NOT_THE_JOB(), &__data7()));
    let __aug8 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("carers-allowance")); __args.push(py_tuple(vec![g__CARERS_PAY(), g__ASKING()])); __args.push(py_str("Carer's Allowance is about £83 a week for caring 35 hours or more for someone who gets PIP daily living, Attendance Allowance or DLA middle or higher care. You must be 16 or over, not in full-time education, and earning under about £200 a week after tax. Claim on gov.uk and date it from the day you started, because it backdates three months; earn too much and Carer's Credit keeps your NI record going instead.")); __kws.extend(vec![("ask".to_string(), g__ASKING()), ("avoid".to_string(), g__NOT_THE_CARERS_MONEY()), ("guard".to_string(), py_func("_the_care_is_for_a_grown_up", |args: &[Py]| _the_care_is_for_a_grown_up(args))), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("carers-assessment")); __args.push(py_tuple(vec![g__CARERS_ASSESSMENT(), g__ASKING()])); __args.push(py_str("Ask the council's adult social care team for a carer's assessment. It is free, it is about your needs and not theirs, and you can ask without telling them. It takes about an hour, at home or on the phone, and it can bring respite hours, a direct payment you spend yourself, training and a number to ring in the worst moments. Say \"I would like a carer's assessment\" and get what they agree in writing.")); __kws.extend(vec![("ask".to_string(), g__ASKING()), ("avoid".to_string(), op_add(&g__NOT_THE_CARERS_MONEY(), &py_tuple(vec![py_str("bus pass"), py_str("freedom pass"), py_str("travel pass"), py_str("blue badge"), py_str("companion"), py_str("motability"), py_str("disabled person")]))), ("explain".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug8);
    s__LOOKED_AFTER_ME(py_tuple(vec![py_str("cares for me"), py_str("care for me"), py_str("caring for me"), py_str("cared for me"), py_str("cared for by"), py_str("looks after me"), py_str("look after me"), py_str("looking after me"), py_str("looked after me"), py_str("looked after by")]));
    fn __data9() -> Py { py_tuple(vec![py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("mum"), py_str("mother"), py_str("dad"), py_str("father"), py_str("sister"), py_str("brother"), py_str("grandson"), py_str("granddaughter"), py_str("grandchild"), py_str("niece"), py_str("nephew"), py_str("children"), py_str("kids"), py_str("family"), py_str("friend"), py_str("friends"), py_str("neighbour"), py_str("neighbours"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("relatives"), py_str("carer"), py_str("carers")]) }
    s__MY_FAMILY_CARER(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("allowance"), py_str("allowances"), py_str("credit"), py_str("benefit"), py_str("benefits"), py_str("claim"), py_str("claiming"), py_str("qualify"), py_str("qualifies"), py_str("entitled"), py_str("pip"), py_str("attendance"), py_str("cope"), py_str("coping"), py_str("cant cope"), py_str("cannot cope"), py_str("exhausted"), py_str("burnt out"), py_str("burned out"), py_str("struggling"), py_str("overwhelmed"), py_str("tiring"), py_str("worn out"), py_str("no life"), py_str("respite"), py_str("assessment"), py_str("a break"), py_str("how do i get help")]) }
    s__NOT_THIS_ROW(op_add(&g__NOT_THE_JOB(), &__data10()));
    let __aug11 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("my-family-cares-for-me")); __args.push(py_tuple(vec![g__LOOKED_AFTER_ME(), g__MY_FAMILY_CARER()])); __args.push(py_str("Two things are worth starting this week, because neither is sent automatically. Whoever looks after you for 35 hours a week or more can claim Carer's Allowance, about £83 a week, and it keeps their National Insurance record going too. There is money for you as well: Attendance Allowance if you are over pension age, PIP if you are younger, neither of them means-tested, and both unlock more on other benefits. Then ask the council for a carer's assessment - it is free, it is about their needs, and it can pay for someone to sit with you so they get an evening back.")); __kws.extend(vec![("avoid".to_string(), g__NOT_THIS_ROW()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    s_ENTRIES(__aug11);
}
pub fn _an_animal_is_the_one_cared_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("False when the line's caring is for an animal: the pet files answer that, and this card is\n    about a person. Read from the line, because the animal only steps aside when it sits where\n    the person sits.");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text"), py_str("plain")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&{ let __b = mcall(&g__AN_ANIMAL_CARED_FOR(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__AN_ANIMAL_WEARS_ME_OUT(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_care_is_for_a_grown_up(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("A child named as the one being looked after is a paid favour, not the carer's benefit.\n\n    \"can i get paid to look after my friend's kids\" is childcare, and the allowance is for\n    caring for someone who gets PIP daily living, Attendance Allowance or DLA. A line that\n    says carer or caring with the child in it still counts: a parent caring for a disabled\n    child can claim, and that question names the benefit.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:kid|kids|child|children|childminding|toddler|toddlers|babies|baby)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bcarers?'?\\s*(?:allowance|credit)\\b|\\bcaring\\b|\\bcarer\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.caring".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_AN_ANIMAL_CARED_FOR" => Some(g__AN_ANIMAL_CARED_FOR()),
            "_AN_ANIMAL_WEARS_ME_OUT" => Some(g__AN_ANIMAL_WEARS_ME_OUT()),
            "_ASKING" => Some(g__ASKING()),
            "_A_KEPT_ANIMAL" => Some(g__A_KEPT_ANIMAL()),
            "_CARERS_ASSESSMENT" => Some(g__CARERS_ASSESSMENT()),
            "_CARERS_PAY" => Some(g__CARERS_PAY()),
            "_LOOKED_AFTER_ME" => Some(g__LOOKED_AFTER_ME()),
            "_MY_FAMILY_CARER" => Some(g__MY_FAMILY_CARER()),
            "_NOT_THE_CARERS_MONEY" => Some(g__NOT_THE_CARERS_MONEY()),
            "_NOT_THE_JOB" => Some(g__NOT_THE_JOB()),
            "_NOT_THIS_ROW" => Some(g__NOT_THIS_ROW()),
            "_THE_CARER" => Some(g__THE_CARER()),
            "_THE_LOAD" => Some(g__THE_LOAD()),
            "_an_animal_is_the_one_cared_for" => Some(py_func("_an_animal_is_the_one_cared_for", |args: &[Py]| _an_animal_is_the_one_cared_for(args))),
            "_the_care_is_for_a_grown_up" => Some(py_func("_the_care_is_for_a_grown_up", |args: &[Py]| _the_care_is_for_a_grown_up(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_AN_ANIMAL_CARED_FOR" => s__AN_ANIMAL_CARED_FOR(v),
            "_AN_ANIMAL_WEARS_ME_OUT" => s__AN_ANIMAL_WEARS_ME_OUT(v),
            "_ASKING" => s__ASKING(v),
            "_A_KEPT_ANIMAL" => s__A_KEPT_ANIMAL(v),
            "_CARERS_ASSESSMENT" => s__CARERS_ASSESSMENT(v),
            "_CARERS_PAY" => s__CARERS_PAY(v),
            "_LOOKED_AFTER_ME" => s__LOOKED_AFTER_ME(v),
            "_MY_FAMILY_CARER" => s__MY_FAMILY_CARER(v),
            "_NOT_THE_CARERS_MONEY" => s__NOT_THE_CARERS_MONEY(v),
            "_NOT_THE_JOB" => s__NOT_THE_JOB(v),
            "_NOT_THIS_ROW" => s__NOT_THIS_ROW(v),
            "_THE_CARER" => s__THE_CARER(v),
            "_THE_LOAD" => s__THE_LOAD(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.caring", module_obj);
    pyrt::xrefs::reg_fn("knowledge.caring", "_an_animal_is_the_one_cared_for", py_func("_an_animal_is_the_one_cared_for", |args: &[Py]| _an_animal_is_the_one_cared_for(args)));
    pyrt::xrefs::reg_fn("knowledge.caring", "_the_care_is_for_a_grown_up", py_func("_the_care_is_for_a_grown_up", |args: &[Py]| _the_care_is_for_a_grown_up(args)));
    pyrt::xrefs::reg_access("knowledge.caring", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.caring", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.caring", "_AN_ANIMAL_CARED_FOR", g__AN_ANIMAL_CARED_FOR, s__AN_ANIMAL_CARED_FOR);
    pyrt::xrefs::reg_access("knowledge.caring", "_AN_ANIMAL_WEARS_ME_OUT", g__AN_ANIMAL_WEARS_ME_OUT, s__AN_ANIMAL_WEARS_ME_OUT);
    pyrt::xrefs::reg_access("knowledge.caring", "_ASKING", g__ASKING, s__ASKING);
    pyrt::xrefs::reg_access("knowledge.caring", "_A_KEPT_ANIMAL", g__A_KEPT_ANIMAL, s__A_KEPT_ANIMAL);
    pyrt::xrefs::reg_access("knowledge.caring", "_CARERS_ASSESSMENT", g__CARERS_ASSESSMENT, s__CARERS_ASSESSMENT);
    pyrt::xrefs::reg_access("knowledge.caring", "_CARERS_PAY", g__CARERS_PAY, s__CARERS_PAY);
    pyrt::xrefs::reg_access("knowledge.caring", "_LOOKED_AFTER_ME", g__LOOKED_AFTER_ME, s__LOOKED_AFTER_ME);
    pyrt::xrefs::reg_access("knowledge.caring", "_MY_FAMILY_CARER", g__MY_FAMILY_CARER, s__MY_FAMILY_CARER);
    pyrt::xrefs::reg_access("knowledge.caring", "_NOT_THE_CARERS_MONEY", g__NOT_THE_CARERS_MONEY, s__NOT_THE_CARERS_MONEY);
    pyrt::xrefs::reg_access("knowledge.caring", "_NOT_THE_JOB", g__NOT_THE_JOB, s__NOT_THE_JOB);
    pyrt::xrefs::reg_access("knowledge.caring", "_NOT_THIS_ROW", g__NOT_THIS_ROW, s__NOT_THIS_ROW);
    pyrt::xrefs::reg_access("knowledge.caring", "_THE_CARER", g__THE_CARER, s__THE_CARER);
    pyrt::xrefs::reg_access("knowledge.caring", "_THE_LOAD", g__THE_LOAD, s__THE_LOAD);
    pyrt::xrefs::reg_access("knowledge.caring", "entry", g_entry, s_entry);
}
