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


thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MAP_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_LOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_OFFICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_OFFICE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOST() -> Py { ensure_init(); G_A_LOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOST(v: Py) { ensure_init(); G_A_LOST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MAP_LINE() -> Py { ensure_init(); G_A_MAP_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MAP_LINE(v: Py) { ensure_init(); G_A_MAP_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_LOST() -> Py { ensure_init(); G_A_PERSON_LOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_LOST(v: Py) { ensure_init(); G_A_PERSON_LOST.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_KINDS() -> Py { ensure_init(); G_THE_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_KINDS(v: Py) { ensure_init(); G_THE_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_OFFICE() -> Py { ensure_init(); G_THE_OFFICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_OFFICE(v: Py) { ensure_init(); G_THE_OFFICE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_OFFICE_NAMES() -> Py { ensure_init(); G_THE_OFFICE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_OFFICE_NAMES(v: Py) { ensure_init(); G_THE_OFFICE_NAMES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A document is held by an office, not found near you.\n\n\"where do i get the deeds\", \"where can i find the log book for my car\", \"where can i get a\nprobate record\" are all \"where do i get a thing\" lines, and the reader for that shape answers\nthem the way it answers a bike tyre: a map app search for the thing and a ring before setting\noff. There is no shop with a shelf of them. Somebody following that reply drives to a retail\npark for a document that has one holder, and the holder is named on a form.\n\nWhat the bot knows here is small and does not go stale: which office holds a kind of record, and\nthat the office's own service is where the copy is ordered. So the reply says the office first -\nthe Land Registry for a title register, the DVLA for a V5C, the register office for a certificate,\nthe Probate Service for a grant - and only then what to have ready. It never says the bot can\nlook it up, and it never guesses an address or a fee.\n\nNothing in here reads the world. The holder of each record is the part that is a fact, so it is\nkept as data and the wording is chosen from a pool by the line, the way the rest of the composer\ndoes it.\n\nThe turn is taken only where the composer had nothing: a hedge, or the composer's own \"I don't\nknow\". A line the knowledge base answers - \"where can i get my birth certificate\" already gets\nthe register office from a card - keeps its own answer, because this reader is asked before the\nmap-app one and after everything that knows something.\n");
    s_AN_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:where|who)\\b[^.!?]{0,40}?\\b(?:get|gets|got|hold|holds|find|order|apply|request|obtain|keep|keeps|stored|copy)\\b|\\bhow\\s+(?:do|can|could|would|should)\\s+i\\s+(?:get|find|order|apply|request|obtain|replace)\\b|\\bhow\\s+to\\s+(?:get|find|order|apply|request|obtain)\\b|\\bcan\\s+i\\s+(?:get|find|order|request|obtain|replace)\\b|\\bwhere\\s+(?:is|are)\\s+[^.!?]{0,30}?\\b(?:kept|held|stored|filed)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LOST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lost|losted|misplaced|mislaid|can'?t\\s+find|cannot\\s+find|can\\s+not\\s+find|cant\\s+find|need\\s+(?:a\\s+)?(?:new|replacement|another)\\s+copy|need\\s+a\\s+copy)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_LOST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:baby|babies|mum|mums|mom|mother|dad|father|son|daughter|child|children|boy|girl|husband|wife|partner|brother|sister|gran|granny|grandma|grandad|grandpa|nana|friend|mate|colleague|neighbou?r|pup|puppy|kitten|dog|cat|horse|pet)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_OFFICE_NAMES(py_dict(vec![(py_str("deeds"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("land\\s+registry|registers\\s+of\\s+scotland|title\\s+register")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("logbook"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bdvla\\b|\\bv5c\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("will"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("solicitor|probate\\s+service|probate\\s+record")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("lease"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("landlord|managing\\s+agent|solicitor")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("certificate"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("register\\s+office|general\\s+register\\s+office")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("probate"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("probate\\s+service|gov\\.uk|solicitor")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("records"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bgp\\b|surgery|practice")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_THE_KINDS(py_tuple(vec![py_tuple(vec![py_str("probate"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:probate|grant\\s+of\\s+probate|letters\\s+of\\s+administration|probate\\s+(?:record|records|copy|papers?))\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("logbook"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:log\\s?book|logbook|v5c|v5\\s?c|registration\\s+document)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("deeds"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:deeds?|title\\s+(?:register|deeds?|plan)|land\\s+registry|title\\s+number)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("will"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|the|my|his|her|their|our|your)\\s+will\\b|\\b[\\w'’]*s['’]?\\s+will\\b|\\b(?:grant\\s+of\\s+)?probate\\s+copy\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("lease"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lease|leases|tenancy\\s+agreement|assured\\s+shorthold)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("certificate"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:birth\\s+certificate|marriage\\s+certificate|death\\s+certificate|certified\\s+copy)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("records"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:medical\\s+records?|health\\s+records?|my\\s+records?|gp\\s+records?|patient\\s+records?)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s_THE_OFFICE(py_dict(vec![(py_str("deeds"), py_tuple(vec![py_str("The deeds are the title register, and HM Land Registry holds it for England and Wales (Registers of Scotland north of the border). Their own service will sell you a copy from the property's address, and that copy is the one that counts."), py_str("What people call the deeds is now the title register at HM Land Registry, so there is nothing to ask a solicitor for first: search their service by the property's address, pay a few pounds, and the copy comes as a download.")])), (py_str("logbook"), py_tuple(vec![py_str("The log book is the V5C and the DVLA issues it - no shop has one. If yours is lost or damaged, apply for a replacement on gov.uk with the car's details and the green slip you kept from the sale."), py_str("That one comes from the DVLA, not a counter in town: a replacement V5C is applied for on gov.uk, and the car's registration number plus the details of the keeper is what the form asks for.")])), (py_str("will"), py_tuple(vec![py_str("The will itself is with whoever drew it up or kept it - usually the solicitor - and once probate is granted a copy goes on the public record at the Probate Service. Ask the solicitor first, then search the Probate Service's own record."), py_str("Start with the solicitor named on it, or the person who kept the original, because a will is not registered anywhere while the person is alive. After probate it is a public document and the Probate Service will sell you a copy.")])), (py_str("lease"), py_tuple(vec![py_str("The lease is held by the landlord or their managing agent, and if you bought the flat your solicitor will have kept a copy - that is usually a phone call and an email. A managing agent will send one for a small fee if the solicitor cannot find it."), py_str("Ask the landlord or the managing agent for a copy, and check your own paperwork first: anyone who has bought a leasehold flat was sent one at completion, and the solicitor who did the buying keeps it on file.")])), (py_str("certificate"), py_tuple(vec![py_str("A certificate comes from the register office for the district where the birth, marriage or death was registered, or from the General Register Office for older ones. Their own site takes the order - have the date and the place ready."), py_str("The register office for the district where it happened holds it, and the General Register Office holds the older records. Order through their own site with the date and the place, and the certificate comes by post.")])), (py_str("probate"), py_tuple(vec![py_str("A grant of probate comes from the Probate Service, through gov.uk, and it is the court's copy rather than the will. An estate with a house, shares or a disagreement in it is worth a solicitor doing that part."), py_str("Probate is the Probate Service's, applied for on gov.uk, and it is worth checking whether the estate is small enough for the simpler form. If there is property or a will being argued over, hand it to a solicitor.")])), (py_str("records"), py_tuple(vec![py_str("Your medical records sit with the GP surgery you are registered at, and you can ask for a copy in writing - the surgery has to answer, usually within a month."), py_str("Ask the GP surgery that has you registered: a written request gets you a copy of the records, and the reception will say what they need from you to prove it is you.")]))]));
    s_A_MAP_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:No map on this side|I can't see what's near you|There's no list of places here|No stock lists here|I can't check stock or shops near you)")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn a_map_app_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply is the map-app reader's line about a thing to buy.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MAP_LINE(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _the_kind(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The family of record the line names, or \"\" when it names none.");
    let __iter1 = py_iter_or_raise(&g_THE_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return name.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _names_the_office(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the reply already says who holds it.");
    pattern = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_OFFICE_NAMES(), "get", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = pattern.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn a_lost_record(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { Py::None };
    let mut clean: Py = Py::None;
    let mut kind: Py = Py::None;
    let _ = py_str("True when the line is a record somebody has lost and the reply has not named its holder.\n\n    The composer reads \"we lost the log book\" as a feeling and answers it with sympathy. It is a\n    question about a replacement, and the office that issues one is the answer, so this stands\n    over that comfort line - but not over a card that already says DVLA, Land Registry or the\n    register office, and never when the thing lost is a person or a pet.\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LOST(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_LOST(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_the_office(args))(&__args) }));
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { Py::None };
    let mut asked: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut lost: Py = Py::None;
    let _ = py_str("A reply naming the office that holds the record, or None.\n\n    Only a line that asks where to get one of these records reaches here: a question shape and a\n    named record. Everything else returns None, so the map-app reader and the rest of the chain\n    keep every ordinary \"where do i get a thing\" line.\n\n    A record somebody has lost is the same question said as a statement - \"we lost the log\n    book\" - and the composer answers it as a feeling (\"That sounds rough\"). The office is the\n    answer that helps, so a lost record is taken too, unless the reply already names the office\n    or the thing lost is a person or a pet rather than a record.\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_ASK(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    lost = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LOST(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&lost.clone())) } }) {
    return Py::None;
    }
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = lost.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_LOST(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = lost.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_the_office(args))(&__args) } } } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&g_THE_OFFICE(), &kind.clone())); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_document_lives_with_an_office".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ASK" => Some(g_AN_ASK()),
            "A_LOST" => Some(g_A_LOST()),
            "A_MAP_LINE" => Some(g_A_MAP_LINE()),
            "A_PERSON_LOST" => Some(g_A_PERSON_LOST()),
            "THE_KINDS" => Some(g_THE_KINDS()),
            "THE_OFFICE" => Some(g_THE_OFFICE()),
            "THE_OFFICE_NAMES" => Some(g_THE_OFFICE_NAMES()),
            "_names_the_office" => Some(py_func("_names_the_office", |args: &[Py]| _names_the_office(args))),
            "_the_kind" => Some(py_func("_the_kind", |args: &[Py]| _the_kind(args))),
            "a_lost_record" => Some(py_func("a_lost_record", |args: &[Py]| a_lost_record(args))),
            "a_map_app_line" => Some(py_func("a_map_app_line", |args: &[Py]| a_map_app_line(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_ASK" => s_AN_ASK(v),
            "A_LOST" => s_A_LOST(v),
            "A_MAP_LINE" => s_A_MAP_LINE(v),
            "A_PERSON_LOST" => s_A_PERSON_LOST(v),
            "THE_KINDS" => s_THE_KINDS(v),
            "THE_OFFICE" => s_THE_OFFICE(v),
            "THE_OFFICE_NAMES" => s_THE_OFFICE_NAMES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_document_lives_with_an_office", module_obj);
    pyrt::xrefs::reg_fn("compose.a_document_lives_with_an_office", "a_map_app_line", py_func("a_map_app_line", |args: &[Py]| a_map_app_line(args)));
    pyrt::xrefs::reg_fn("compose.a_document_lives_with_an_office", "_the_kind", py_func("_the_kind", |args: &[Py]| _the_kind(args)));
    pyrt::xrefs::reg_fn("compose.a_document_lives_with_an_office", "_names_the_office", py_func("_names_the_office", |args: &[Py]| _names_the_office(args)));
    pyrt::xrefs::reg_fn("compose.a_document_lives_with_an_office", "a_lost_record", py_func("a_lost_record", |args: &[Py]| a_lost_record(args)));
    pyrt::xrefs::reg_fn("compose.a_document_lives_with_an_office", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "A_LOST", g_A_LOST, s_A_LOST);
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "A_MAP_LINE", g_A_MAP_LINE, s_A_MAP_LINE);
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "A_PERSON_LOST", g_A_PERSON_LOST, s_A_PERSON_LOST);
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "THE_KINDS", g_THE_KINDS, s_THE_KINDS);
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "THE_OFFICE", g_THE_OFFICE, s_THE_OFFICE);
    pyrt::xrefs::reg_access("compose.a_document_lives_with_an_office", "THE_OFFICE_NAMES", g_THE_OFFICE_NAMES, s_THE_OFFICE_NAMES);
}
