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


thread_local! { static G_BREED_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREED_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOSE_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_CREATURE_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BREED_KIND() -> Py { ensure_init(); G_BREED_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREED_KIND(v: Py) { ensure_init(); G_BREED_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_BREED_RE() -> Py { ensure_init(); G_BREED_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREED_RE(v: Py) { ensure_init(); G_BREED_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_BY_WORD() -> Py { ensure_init(); G_BY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BY_WORD(v: Py) { ensure_init(); G_BY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME_BEFORE() -> Py { ensure_init(); G_FRAME_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME_BEFORE(v: Py) { ensure_init(); G_FRAME_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME_REACH() -> Py { ensure_init(); G_FRAME_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME_REACH(v: Py) { ensure_init(); G_FRAME_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOSE_FRAME() -> Py { ensure_init(); G_LOOSE_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOSE_FRAME(v: Py) { ensure_init(); G_LOOSE_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_CREATURE_NEXT() -> Py { ensure_init(); G_NOT_A_CREATURE_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CREATURE_NEXT(v: Py) { ensure_init(); G_NOT_A_CREATURE_NEXT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A breed is the animal it is.\n\n\"my greyhound is not eating\" names no animal any knowledge entry is keyed on: the entry for a\ndog that stops eating is keyed on \"dog\", \"puppy\" and \"pup\", and the breed word reaches none of\nthem, so the line fell all the way through to the composer and the person got \"I'm here if you\nwant to dig into any of that\" while \"my dog is not eating\" gets the vet-first answer. The same\nhole ran through every breed: a labrador, a whippet, a poodle and a jack russell all got a\ndisclosure line, and a spaniel whose food was refused got the answer about changing food over,\nbecause \"spaniel\" matched nothing and the card was chosen on the rest of the line.\n\nSo the reading carries the animal's own word beside the breed's: `words` gets \"dog\" after\n\"greyhound\", which is what the line means, and every module that reads the animal it is - the\nknowledge base most of all - sees a dog line. `clean` is untouched, so an echo or a\nconfirmation still says the person's own word back.\n\nTwo things keep this off lines that are not about an animal: a breed that is the first half of\nsome other name (\"the greyhound bus is late\", \"greyhound racing\" - the coach-delay entry was the\nright answer to that one), and the ambiguous names (\"boxer\", \"pointer\", \"setter\", \"springer\",\n\"persian\" the rug, \"bengal\" the tiger), which are left out of the table rather than guessed at.\n");
    s_BREED_KIND(py_dict(vec![(py_str("labrador"), py_str("dog")), (py_str("lurcher"), py_str("dog")), (py_str("greyhound"), py_str("dog")), (py_str("whippet"), py_str("dog")), (py_str("terrier"), py_str("dog")), (py_str("spaniel"), py_str("dog")), (py_str("retriever"), py_str("dog")), (py_str("collie"), py_str("dog")), (py_str("poodle"), py_str("dog")), (py_str("beagle"), py_str("dog")), (py_str("bulldog"), py_str("dog")), (py_str("pug"), py_str("dog")), (py_str("husky"), py_str("dog")), (py_str("chihuahua"), py_str("dog")), (py_str("dachshund"), py_str("dog")), (py_str("dalmatian"), py_str("dog")), (py_str("mastiff"), py_str("dog")), (py_str("rottweiler"), py_str("dog")), (py_str("doberman"), py_str("dog")), (py_str("dobermann"), py_str("dog")), (py_str("alsatian"), py_str("dog")), (py_str("corgi"), py_str("dog")), (py_str("staffie"), py_str("dog")), (py_str("mongrel"), py_str("dog")), (py_str("cockapoo"), py_str("dog")), (py_str("labradoodle"), py_str("dog")), (py_str("cavapoo"), py_str("dog")), (py_str("pomeranian"), py_str("dog")), (py_str("papillon"), py_str("dog")), (py_str("schnauzer"), py_str("dog")), (py_str("weimaraner"), py_str("dog")), (py_str("vizsla"), py_str("dog")), (py_str("samoyed"), py_str("dog")), (py_str("akita"), py_str("dog")), (py_str("shar pei"), py_str("dog")), (py_str("lhasa apso"), py_str("dog")), (py_str("afghan hound"), py_str("dog")), (py_str("basset hound"), py_str("dog")), (py_str("border collie"), py_str("dog")), (py_str("cocker spaniel"), py_str("dog")), (py_str("golden retriever"), py_str("dog")), (py_str("labrador retriever"), py_str("dog")), (py_str("jack russell"), py_str("dog")), (py_str("german shepherd"), py_str("dog")), (py_str("shih tzu"), py_str("dog")), (py_str("staffordshire bull terrier"), py_str("dog")), (py_str("bull terrier"), py_str("dog")), (py_str("yorkshire terrier"), py_str("dog")), (py_str("westie"), py_str("dog")), (py_str("west highland terrier"), py_str("dog")), (py_str("siamese"), py_str("cat")), (py_str("ragdoll"), py_str("cat")), (py_str("moggy"), py_str("cat")), (py_str("tabby"), py_str("cat")), (py_str("tortoiseshell"), py_str("cat")), (py_str("sphynx"), py_str("cat")), (py_str("maine coon"), py_str("cat")), (py_str("british shorthair"), py_str("cat")), (py_str("scottish fold"), py_str("cat"))]));
    s_MONEY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nmoney quid tenner tenners grand odds punter winnings bet bets betting stake stakes\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOOSE_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the a an that these those this"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_CREATURE_NEXT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nbus buses coach coaches station stations line lines service services depot depots ticket\ntickets terminal terminals timetable timetables racing race races derby stadium track kennel\nkennels club breeder breeders rescue glasses frames sunglasses sofa sofas rug rugs curtains\nupholstery twin twins person people\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FRAME_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my our his her their the a an another this that these those\ngot get gets have has had having keep keeps kept kept buy buys bought take takes took rescue\nrescued adopt adopted rehome rehomed"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FRAME_REACH(Py::Int(4));
    s_BY_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^[a-z][a-z'-]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BREED_RE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _alternation(args))(&__args) }), &py_str(")(?:s|es)?\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _singular(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("The table's spelling of a plural breed: \"greyhounds\" is a greyhound, \"collies\" a collie.");
    if py_truthy(&Py::Bool(py_contains(&g_BREED_KIND(), &word.clone()))) {
    return word.clone();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ies")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BREED_KIND(), &op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y")))) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BREED_KIND(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None))) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BREED_KIND(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return py_str("");
    Py::None
}
pub fn the_animal_a_breed_is(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut after: Py = Py::None;
    let mut filled: Py = Py::None;
    let mut found: Py = Py::None;
    let mut frame_words: Py = Py::None;
    let mut framed: Py = Py::None;
    let mut index: Py = Py::None;
    let mut length: Py = Py::None;
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reachable: Py = Py::None;
    let mut span: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's words with the animal's own word after each breed it names.\n\n    `words` is the reading's word list, which is the text in order. Every breed word keeps its\n    place and the animal goes straight after it, so a two-word key elsewhere in the line (\"not\n    eating\", \"off her food\") still reads as it did, and a phrase breed (\"jack russell\") is filled\n    once, after its last word. Nothing is added when the word after the breed makes it another\n    thing: \"the greyhound bus is late\" and \"greyhound racing\" stay as typed.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&words.clone())) } }) {
    return words.clone();
    }
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) } } else { py_str("") }); } py_list(__out) })() };
    filled = py_list(vec![]);
    index = Py::Int(0);
    while py_truthy(&op_lt(&index.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {
    found = py_str("");
    span = Py::Int(1);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(4), Py::Int(3), Py::Int(2)]));
    for __item in __iter1 {
    length = __item.clone();
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&low.clone(), &index.clone(), &op_add(&index.clone(), &length.clone()), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &length.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _singular(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BREED_KIND(), "get", args))(&__args) } } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&g_BREED_KIND(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _singular(args))(&__args) }), length.clone()]), 2, None);
    found = __unpk2[0].clone();
    span = __unpk2[1].clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&low.clone(), &index.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _singular(args))(&__args) }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BREED_KIND(), "get", args))(&__args) };
    }
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &span.clone()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&low.clone(), &op_add(&index.clone(), &span.clone())) } else { py_str("") };
    reachable = slice_or_raise(&low.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&index.clone(), &g_FRAME_REACH())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &index.clone(), &Py::None);
    frame_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&reachable.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_FRAME_BEFORE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    framed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frame_words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { framed.clone() } }) {
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_CREATURE_NEXT(), &after.clone()))) {
    framed = Py::Bool(false);
    } else {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frame_words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LOOSE_FRAME(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_MONEY(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    framed = Py::Bool(false);
    }
    }
    }
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { framed.clone() } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &index.clone(), &op_add(&index.clone(), &span.clone()), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&filled.clone(), "extend", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&filled.clone(), "append", args))(&__args) };
    let __aug3 = op_add(&index.clone(), &span.clone());
    index = __aug3;
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &index.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&filled.clone(), "append", args))(&__args) };
    let __aug4 = op_add(&index.clone(), &Py::Int(1));
    index = __aug4;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(filled.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return words.clone();
    }
    return filled.clone();
    Py::None
}
pub fn a_frame_before(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut before: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a breed at `start` is said as an animal: a determiner or a verb of having first.\n\n    The words of a line say what a breed word is being used as, and the store reads the raw line\n    rather than the reading's word list, so this is the same test as `the_animal_a_breed_is` over\n    the text before the breed. \"im husky\" is a voice, not an animal.\n    ");
    before = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&g_FRAME_REACH()), &Py::None, &Py::None);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FRAME_BEFORE(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_breed_named(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The table's spelling of the breed in `text`, or \"\" - \"greyhounds\" is \"greyhound\".");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _singular(args))(&__args) };
    Py::None
}
pub fn the_kind_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("The animal a breed word names - \"dog\", \"cat\" - or \"\" for anything else.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_breed_named(args))(&__args) }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BREED_KIND(), "get", args))(&__args) };
    Py::None
}
pub fn _alternation(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let _ = py_str("Every breed as one pattern, the longest phrase first so \"golden retriever\" wins.");
    names = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_BREED_KIND()); __kws.extend(vec![("key".to_string(), { let name = name.clone(); let names = names.clone(); py_func("lambda", move |args: &[Py]| { let name = name.clone(); let names = names.clone(); let name: Py = __kwp(args, 0, "name").unwrap_or(Py::None); py_tuple(vec![op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(py_str("\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "replace", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.breeds".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BREED_KIND" => Some(g_BREED_KIND()),
            "BREED_RE" => Some(g_BREED_RE()),
            "BY_WORD" => Some(g_BY_WORD()),
            "FRAME_BEFORE" => Some(g_FRAME_BEFORE()),
            "FRAME_REACH" => Some(g_FRAME_REACH()),
            "LOOSE_FRAME" => Some(g_LOOSE_FRAME()),
            "MONEY" => Some(g_MONEY()),
            "NOT_A_CREATURE_NEXT" => Some(g_NOT_A_CREATURE_NEXT()),
            "_alternation" => Some(py_func("_alternation", |args: &[Py]| _alternation(args))),
            "_singular" => Some(py_func("_singular", |args: &[Py]| _singular(args))),
            "a_frame_before" => Some(py_func("a_frame_before", |args: &[Py]| a_frame_before(args))),
            "the_animal_a_breed_is" => Some(py_func("the_animal_a_breed_is", |args: &[Py]| the_animal_a_breed_is(args))),
            "the_breed_named" => Some(py_func("the_breed_named", |args: &[Py]| the_breed_named(args))),
            "the_kind_of" => Some(py_func("the_kind_of", |args: &[Py]| the_kind_of(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BREED_KIND" => s_BREED_KIND(v),
            "BREED_RE" => s_BREED_RE(v),
            "BY_WORD" => s_BY_WORD(v),
            "FRAME_BEFORE" => s_FRAME_BEFORE(v),
            "FRAME_REACH" => s_FRAME_REACH(v),
            "LOOSE_FRAME" => s_LOOSE_FRAME(v),
            "MONEY" => s_MONEY(v),
            "NOT_A_CREATURE_NEXT" => s_NOT_A_CREATURE_NEXT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.breeds", module_obj);
    pyrt::xrefs::reg_fn("understanding.breeds", "_singular", py_func("_singular", |args: &[Py]| _singular(args)));
    pyrt::xrefs::reg_fn("understanding.breeds", "the_animal_a_breed_is", py_func("the_animal_a_breed_is", |args: &[Py]| the_animal_a_breed_is(args)));
    pyrt::xrefs::reg_fn("understanding.breeds", "a_frame_before", py_func("a_frame_before", |args: &[Py]| a_frame_before(args)));
    pyrt::xrefs::reg_fn("understanding.breeds", "the_breed_named", py_func("the_breed_named", |args: &[Py]| the_breed_named(args)));
    pyrt::xrefs::reg_fn("understanding.breeds", "the_kind_of", py_func("the_kind_of", |args: &[Py]| the_kind_of(args)));
    pyrt::xrefs::reg_fn("understanding.breeds", "_alternation", py_func("_alternation", |args: &[Py]| _alternation(args)));
    pyrt::xrefs::reg_access("understanding.breeds", "BREED_KIND", g_BREED_KIND, s_BREED_KIND);
    pyrt::xrefs::reg_access("understanding.breeds", "BREED_RE", g_BREED_RE, s_BREED_RE);
    pyrt::xrefs::reg_access("understanding.breeds", "BY_WORD", g_BY_WORD, s_BY_WORD);
    pyrt::xrefs::reg_access("understanding.breeds", "FRAME_BEFORE", g_FRAME_BEFORE, s_FRAME_BEFORE);
    pyrt::xrefs::reg_access("understanding.breeds", "FRAME_REACH", g_FRAME_REACH, s_FRAME_REACH);
    pyrt::xrefs::reg_access("understanding.breeds", "LOOSE_FRAME", g_LOOSE_FRAME, s_LOOSE_FRAME);
    pyrt::xrefs::reg_access("understanding.breeds", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("understanding.breeds", "NOT_A_CREATURE_NEXT", g_NOT_A_CREATURE_NEXT, s_NOT_A_CREATURE_NEXT);
}
