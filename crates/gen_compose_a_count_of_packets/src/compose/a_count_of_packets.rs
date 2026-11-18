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


thread_local! { static G_AN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ANIMAL_TOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTAINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRUG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKET_WORDS_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKET_WORDS_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOCK_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.a_count_of_packets: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AN_ANIMAL() -> Py { ensure_init(); G_AN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ANIMAL(v: Py) { ensure_init(); G_AN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ANIMAL_TOOK() -> Py { ensure_init(); G_AN_ANIMAL_TOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ANIMAL_TOOK(v: Py) { ensure_init(); G_AN_ANIMAL_TOOK.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTAINER() -> Py { ensure_init(); G_CONTAINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTAINER(v: Py) { ensure_init(); G_CONTAINER.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT() -> Py { ensure_init(); G_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT(v: Py) { ensure_init(); G_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_DRUG() -> Py { ensure_init(); G_DRUG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRUG(v: Py) { ensure_init(); G_DRUG.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_VERB() -> Py { ensure_init(); G_NO_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_VERB(v: Py) { ensure_init(); G_NO_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKETS() -> Py { ensure_init(); G_PACKETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKETS(v: Py) { ensure_init(); G_PACKETS.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKET_WORDS_BEFORE() -> Py { ensure_init(); G_PACKET_WORDS_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKET_WORDS_BEFORE(v: Py) { ensure_init(); G_PACKET_WORDS_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKET_WORDS_ON() -> Py { ensure_init(); G_PACKET_WORDS_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKET_WORDS_ON(v: Py) { ensure_init(); G_PACKET_WORDS_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_STOCK_PLACE() -> Py { ensure_init(); G_STOCK_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOCK_PLACE(v: Py) { ensure_init(); G_STOCK_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEN() -> Py { ensure_init(); G_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEN(v: Py) { ensure_init(); G_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB() -> Py { ensure_init(); G_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB(v: Py) { ensure_init(); G_VERB.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A count of packets is a dose, not a shopping list.\n\n`i swallowed two packets of pills` was answered with how to take one tablet - \"Take a mouthful of\nwater first, put the tablet further back on your tongue than feels natural and swallow with a\ndrink\" - which is the worst reply there is to somebody who has just taken two packets. The poisons\ncase in `bot/modules/safety.py` reads an amount (`PILLS_OVERDOSE`, `DOSE_WHOLE_PACK`), and every\narm of it wants the word whole, full or entire before the packet, or a number the size of a\nhandful of tablets. A number and a plural packet is how people word it when they are frightened,\nso the shape was answered by whoever else had a card for the words: the tablet-taking card for\n`swallowed`, and a nod for the rest.\n\nThis reader is the container count only, and it says nothing else: it does not read a drug, an\nintent or a time, and it never fires on a packet that is being bought, carried or kept. The card\nitself is safety's (`REPLIES[\"poison\"]`), so the wording stays in one place.\n\nThree readings keep the emergency where it belongs:\n\n* **An animal is not a person.** `my dog swallowed two packets of pills` is the vet's card, and\n  999 for a dog is worse than the shrug it replaced, so a line whose subject is an animal is\n  stood down here and left to the pet reader.\n* **Stock has to be said on the packet, not somewhere in the sentence.** `we are away this week\n  and i took two packets of pills` is somebody telling you what they did, and `i took two packets\n  of pills and left the house` is the same; only a frame that sits on the packet (\"for the trip\",\n  \"on holiday with me\", \"in my bag\", \"from the pharmacy\") makes a count stock.\n* **A hand-over is a dose.** `i gave my child two packets of pills` is the count as much as\n  `i swallowed` is.\n");
    s_COUNT(py_str("(?:two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,3}|a\\s+couple\\s+of|a\\s+few|several|a\\s+handful\\s+of)"));
    s_CONTAINER(py_str("(?:packets?|packs?|boxes|box|bottles?|blisters?|strips?|sleeves?|sachets?|handfuls?)"));
    s_DRUG(py_str("(?:pills|tablets|paracetamol|ibuprofen|painkillers|analgesics|meds|medicines?|sleeping\\s+pills|antidepressants|tranquill?i[sz]ers?|insulin|codeine|co[\\s-]?codamol|codamol|tramadol|morphine|diazepam|cough\\s+medicine|painkillers?)"));
    s_VERB(py_str("(?:swallowed|swallowing|drank|drunk|eaten|ate|took|taken|downed|inhaled|popped|necked|finished|used|washed\\s+down|swilled|gulped|gave|given|handed)"));
    s_PACKETS(op_add(&op_add(&op_add(&op_add(&op_add(&g_COUNT(), &py_str("\\s+")), &g_CONTAINER()), &py_str("\\s+of\\s+[^.?!]{0,15}\\b")), &g_DRUG()), &py_str("\\b")));
    s_TAKEN(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("\\b"), &g_VERB()), &py_str("\\b[^.?!]{0,25}\\b")), &g_PACKETS())]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_VERB(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &g_PACKETS()), &py_str("[^.?!]{0,25}\\b(?:feel|felt|feeling|sick|ill|dizzy|drowsy|sleepy|vomit\\w*|throwing\\s+up|shak\\w*|blurry|off\\s+my\\s+face|can'?t\\s+stay\\s+awake|sleeping\\s+all\\s+day|swallow\\w*|and\\s+then\\s+i|and\\s+now\\s+i)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ANIMAL(py_str("(?:dogs?|pupp(?:y|ies)|cats?|kittens?|rabbits?|bunn(?:y|ies)|hamsters?|budgies?|birds?|parrots?|horses?|pon(?:y|ies)|guinea\\s+pigs?|gerbils?|ferrets?|tortoises?|animals?|pets?)"));
    s_AN_ANIMAL_TOOK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_AN_ANIMAL()), &py_str("\\b[^.?!]{0,25}\\b")), &g_VERB()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_PACKET_WORDS_BEFORE(stdlib_call("re", "compile", &[op_add(&py_str("\\b(?:buy|buys|buying|bought|shop|shopping|order|ordered|prescription|prescribed|pharmacy|chemist|doctor|gp|nurse|getting|pick\\s+up|picked\\s+up|stock|stocked\\s+up|spare|first\\s+aid)\\b[^.?!]{0,30}"), &g_PACKETS())]).unwrap_or_else(|e| exc::raise(e)));
    s_PACKET_WORDS_ON(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &g_PACKETS()), &py_str("[^.?!]{0,25}\\b(?:for\\s+(?:the|a|my)\\s+(?:trip|holiday|weekend|flight|journey|kids)|on\\s+holiday|on\\s+business|to\\s+take\\s+away|away\\s+with\\s+me|in\\s+my\\s+(?:bag|pocket|suitcase|handbag|purse)|in\\s+the\\s+(?:cupboard|drawer|cabinet|glovebox)|at\\s+home|left\\s+over|spare|abroad)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_STOCK_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:in|inside|kept\\s+in|keep\\s+in|keeps\\s+in|have|has|got|carry|carrying|stash)\\b[^.?!]{0,20}\\b(?:cupboard|drawer|cabinet|shelf|shelves|bag|pocket|purse|glovebox)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn is_a_dose(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says an amount of medicine as a count of packets somebody has taken.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_AN_ANIMAL_TOOK(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_PACKET_WORDS_BEFORE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PACKET_WORDS_ON(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_STOCK_PLACE(), "search", &[line.clone()]) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_TAKEN(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NO_VERB(), "search", &[line.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_count_of_packets".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ANIMAL" => Some(g_AN_ANIMAL()),
            "AN_ANIMAL_TOOK" => Some(g_AN_ANIMAL_TOOK()),
            "CONTAINER" => Some(g_CONTAINER()),
            "COUNT" => Some(g_COUNT()),
            "DRUG" => Some(g_DRUG()),
            "NO_VERB" => Some(g_NO_VERB()),
            "PACKETS" => Some(g_PACKETS()),
            "PACKET_WORDS_BEFORE" => Some(g_PACKET_WORDS_BEFORE()),
            "PACKET_WORDS_ON" => Some(g_PACKET_WORDS_ON()),
            "STOCK_PLACE" => Some(g_STOCK_PLACE()),
            "TAKEN" => Some(g_TAKEN()),
            "VERB" => Some(g_VERB()),
            "is_a_dose" => Some(py_func("is_a_dose", |args: &[Py]| is_a_dose(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_ANIMAL" => s_AN_ANIMAL(v),
            "AN_ANIMAL_TOOK" => s_AN_ANIMAL_TOOK(v),
            "CONTAINER" => s_CONTAINER(v),
            "COUNT" => s_COUNT(v),
            "DRUG" => s_DRUG(v),
            "NO_VERB" => s_NO_VERB(v),
            "PACKETS" => s_PACKETS(v),
            "PACKET_WORDS_BEFORE" => s_PACKET_WORDS_BEFORE(v),
            "PACKET_WORDS_ON" => s_PACKET_WORDS_ON(v),
            "STOCK_PLACE" => s_STOCK_PLACE(v),
            "TAKEN" => s_TAKEN(v),
            "VERB" => s_VERB(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_count_of_packets", module_obj);
    pyrt::xrefs::reg_fn("compose.a_count_of_packets", "is_a_dose", py_func("is_a_dose", |args: &[Py]| is_a_dose(args)));
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "AN_ANIMAL", g_AN_ANIMAL, s_AN_ANIMAL);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "AN_ANIMAL_TOOK", g_AN_ANIMAL_TOOK, s_AN_ANIMAL_TOOK);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "CONTAINER", g_CONTAINER, s_CONTAINER);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "COUNT", g_COUNT, s_COUNT);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "DRUG", g_DRUG, s_DRUG);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "NO_VERB", g_NO_VERB, s_NO_VERB);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "PACKETS", g_PACKETS, s_PACKETS);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "PACKET_WORDS_BEFORE", g_PACKET_WORDS_BEFORE, s_PACKET_WORDS_BEFORE);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "PACKET_WORDS_ON", g_PACKET_WORDS_ON, s_PACKET_WORDS_ON);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "STOCK_PLACE", g_STOCK_PLACE, s_STOCK_PLACE);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "TAKEN", g_TAKEN, s_TAKEN);
    pyrt::xrefs::reg_access("compose.a_count_of_packets", "VERB", g_VERB, s_VERB);
}
