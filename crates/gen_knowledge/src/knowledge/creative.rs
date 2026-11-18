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
thread_local! { static G__COSTUME_MAKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COSTUME_TIPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__COSTUME_MAKING() -> Py { ensure_init(); G__COSTUME_MAKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COSTUME_MAKING(v: Py) { ensure_init(); G__COSTUME_MAKING.with(|c| *c.borrow_mut() = v); }
pub fn g__COSTUME_TIPS() -> Py { ensure_init(); G__COSTUME_TIPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COSTUME_TIPS(v: Py) { ensure_init(); G__COSTUME_TIPS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_METHOD() -> Py { ensure_init(); G__THE_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_METHOD(v: Py) { ensure_init(); G__THE_METHOD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Writing craft: why a draft feels flat, and how to get a story moving.\n\nThe verse requests are already answered (`bot/knowledge/entertainment.py` has the poem, limerick,\nhaiku and rhyme tables), so nothing here writes anything for you. What was still a shrug is the\ncraft question underneath: \"how do i make my writing less boring\" came back as \"I'd rather not\nguess\", and \"how do i start a story\" got a sample story rather than the thing that makes one run.\n\nTwo entries only, both about the writing rather than the subject, and both pass (return None) on\nanything that names a subject to write about, so entertainment.py keeps every verse request.\n");
    s_ANSWERS(py_tuple(vec![py_str("creative"), py_str("language-and-writing")]));
    s__THE_METHOD(py_str("Ordinary clothes plus one big shape, and nothing to sew: a black hoodie with ears on a headband is a cat, a cardboard box is a robot, a dressing gown and a towel round the head is a shepherd. Hats, ears and face paint do more than the outfit, and one prop - a cardboard sword, a wooden spoon, a stack of books - tells everyone who it is meant to be."));
    s__COSTUME_TIPS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("world book day"), py_str("book day"), py_str("book character"), py_str("book week"), py_str("world book")]), py_str("For World Book Day pick a character who wears normal clothes - Matilda with a stack of books, the Worst Witch in school uniform, the Tiger Who Came to Tea with whiskers of face paint - because the book under the arm does the rest of the work.")]), py_tuple(vec![py_tuple(vec![py_str("nativity"), py_str("shepherd"), py_str("shepherds"), py_str("wise man"), py_str("angel"), py_str("angels")]), py_str("A shepherd is a dressing gown, a towel round the head and a tea towel for a belt; an angel is white clothes, wings bent out of wire coat hangers and tights, and tinsel on a headband for a halo.")]), py_tuple(vec![py_tuple(vec![py_str("pirate"), py_str("pirates")]), py_str("Pirate: a striped top, a scarf knotted as a bandana, an eye patch cut from card, and a sword of rolled card or a wooden spoon.")]), py_tuple(vec![py_tuple(vec![py_str("superhero"), py_str("superheroes"), py_str("super hero"), py_str("cape"), py_str("batman"), py_str("spiderman"), py_str("spider man")]), py_str("Superhero: a plain t-shirt with a paper logo taped on, leggings, and a cape cut from an old sheet pinned at the shoulders - pinned, never tied round the neck.")]), py_tuple(vec![py_tuple(vec![py_str("witch"), py_str("witches"), py_str("wizard"), py_str("harry potter")]), py_str("Witch: black clothes, a cone of card for a hat, and a broom of a stick and a bundle of straw.")]), py_tuple(vec![py_tuple(vec![py_str("dinosaur"), py_str("dinosaurs"), py_str("dragon"), py_str("dragons")]), py_str("Dinosaur: a hoodie with triangles of paper or felt down the back of the hood, and a tail pinned to the trousers.")]), py_tuple(vec![py_tuple(vec![py_str("robot"), py_str("robots"), py_str("astronaut"), py_str("spaceman")]), py_str("Robot: a box over the head with eye holes cut out, foil taped on the arms and legs, and bottle tops glued on for buttons.")]), py_tuple(vec![py_tuple(vec![py_str("cat"), py_str("cats"), py_str("dog"), py_str("dogs"), py_str("mouse"), py_str("bunny"), py_str("rabbit"), py_str("animal"), py_str("animals")]), py_str("Cat or dog: black or brown clothes, ears cut from card on a headband, and whiskers in face paint.")]), py_tuple(vec![py_tuple(vec![py_str("princess"), py_str("princesses"), py_str("fairy"), py_str("fairies")]), py_str("Princess or fairy: a long dress or a sheet, a crown of card and foil, and a sash of ribbon.")]), py_tuple(vec![py_tuple(vec![py_str("cowboy"), py_str("cowgirl"), py_str("indian"), py_str("sheriff")]), py_str("Cowboy: jeans, a checked shirt, a scarf at the neck, and a hat of card with a brim.")]), py_tuple(vec![py_tuple(vec![py_str("skeleton"), py_str("ghost"), py_str("zombie")]), py_str("Skeleton: black clothes with bones painted on in white face paint, or cut from paper and pinned on.")])]));
    s__COSTUME_MAKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:costumes?|fancy dress|dress(?:ed)? (?:up|as)|dressing up|outfits?|make|makes|making|made|diy|sew|sewing|sewn|put together|throw together|improvise|knock up|out of|ideas?|go(?:es|ing)? as|want(?:s)? to be|be a|wear|wearing|need|needs|provide|ourselves|myself|cheap|cheaply|easy|last minute|no sewing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("story-craft")); __args.push(py_tuple(vec![py_tuple(vec![py_str("story"), py_str("stories"), py_str("plot"), py_str("novel"), py_str("script"), py_str("screenplay"), py_str("character"), py_str("fiction"), py_str("villain"), py_str("chapter")]), py_tuple(vec![py_str("stuck"), py_str("idea"), py_str("ideas"), py_str("start"), py_str("where do i start"), py_str("believable"), py_str("realistic"), py_str("convincing"), py_str("plot"), py_str("plan"), py_str("help me"), py_str("help"), py_str("how do i write"), py_str("what should"), py_str("make it work"), py_str("going nowhere")])])); __args.push(g__story()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("write me a story"), py_str("write us a story"), py_str("tell me a story"), py_str("make up a story"), py_str("read next"), py_str("recommend"), py_str("suggest"), py_str("should i read"), py_str("to read"), py_str("book for"), py_str("gift"), py_str("present"), py_str("book club"), py_str("audio book"), py_str("audiobook")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-writing-better")); __args.push(py_tuple(vec![py_tuple(vec![py_str("writing"), py_str("prose"), py_str("description"), py_str("paragraph"), py_str("paragraphs"), py_str("draft"), py_str("blog post")]), py_tuple(vec![py_str("boring"), py_str("flat"), py_str("dull"), py_str("better"), py_str("improve"), py_str("punchier"), py_str("livelier"), py_str("read better"), py_str("less boring"), py_str("more interesting"), py_str("tighten"), py_str("bland")])])); __args.push(g__prose()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("handwriting"), py_str("smaller"), py_str("neater"), py_str("pen"), py_str("spelling"), py_str("grammar"), py_str("font"), py_str("size")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name-a-business")); __args.push(py_tuple(vec![py_tuple(vec![py_str("name"), py_str("names"), py_str("naming"), py_str("call it"), py_str("called"), py_str("what to call")]), py_tuple(vec![py_str("business"), py_str("shop"), py_str("bakery"), py_str("cafe"), py_str("caf"), py_str("restaurant"), py_str("company"), py_str("brand"), py_str("team"), py_str("podcast"), py_str("blog"), py_str("stall"), py_str("startup"), py_str("start up"), py_str("etsy"), py_str("pub"), py_str("salon"), py_str("studio"), py_str("brand name"), py_str("small business"), py_str("side hustle"), py_str("market stall"), py_str("youtube channel"), py_str("newsletter")])])); __args.push(g__naming()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("puppy"), py_str("kitten"), py_str("dog"), py_str("cat"), py_str("hamster"), py_str("rabbit"), py_str("guinea"), py_str("parrot"), py_str("budgie"), py_str("horse"), py_str("pony"), py_str("tortoise"), py_str("baby"), py_str("baby name"), py_str("baby names"), py_str("what is it called"), py_str("whats it called"), py_str("what is the name of"), py_str("called again")])), ("ask".to_string(), py_tuple(vec![py_str("suggest"), py_str("suggestion"), py_str("suggestions"), py_str("ideas"), py_str("idea"), py_str("any"), py_str("good"), py_str("help me"), py_str("come up with"), py_str("think of"), py_str("what should i"), py_str("what could i"), py_str("what would you"), py_str("how do i name"), py_str("name for"), py_str("name ideas"), py_str("pick a name"), py_str("choose a name"), py_str("shortlist")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-costume-at-home")); __args.push(py_tuple(vec![py_tuple(vec![py_str("costume"), py_str("costumes"), py_str("fancy dress"), py_str("outfit"), py_str("dressing up"), py_str("dress up box"), py_str("world book day"), py_str("book day"), py_str("book character"), py_str("book week"), py_str("nativity"), py_str("school play"), py_str("halloween"), py_str("party costume"), py_str("pirate"), py_str("pirates"), py_str("superhero"), py_str("superheroes"), py_str("cape"), py_str("angel"), py_str("shepherd"), py_str("witch"), py_str("wizard"), py_str("dinosaur"), py_str("robot"), py_str("princess"), py_str("cowboy"), py_str("skeleton")]), py_tuple(vec![py_str("make"), py_str("makes"), py_str("making"), py_str("made"), py_str("diy"), py_str("sew"), py_str("sewing"), py_str("put together"), py_str("throw together"), py_str("improvise"), py_str("knock up"), py_str("out of"), py_str("from what we have"), py_str("from what i have"), py_str("what can i"), py_str("what should"), py_str("what do i"), py_str("how do i"), py_str("how can i"), py_str("go as"), py_str("goes as"), py_str("going as"), py_str("dress as"), py_str("dressed as"), py_str("ideas"), py_str("idea"), py_str("want to be"), py_str("wants to be"), py_str("be a"), py_str("provide"), py_str("ourselves"), py_str("myself"), py_str("need"), py_str("easy"), py_str("cheap"), py_str("last minute"), py_str("no sewing")])])); __args.push(g__a_costume()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), g__making_a_costume()), ("avoid".to_string(), py_tuple(vec![py_str("swimming"), py_str("swimsuit"), py_str("swimwear"), py_str("bikini"), py_str("trunks"), py_str("wetsuit"), py_str("pool"), py_str("chlorine"), py_str("wash"), py_str("washing"), py_str("clean"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("iron"), py_str("ironing"), py_str("mend"), py_str("repair"), py_str("alter"), py_str("tailor"), py_str("hire"), py_str("hiring"), py_str("rent"), py_str("rental"), py_str("buy"), py_str("buying"), py_str("shop"), py_str("online"), py_str("amazon"), py_str("where can i get"), py_str("where to buy"), py_str("sale"), py_str("price"), py_str("cost"), py_str("cake"), py_str("cakes"), py_str("cupcake"), py_str("bake"), py_str("baking"), py_str("sandwich"), py_str("party food"), py_str("trick or treat"), py_str("sweets"), py_str("balloons"), py_str("bunting"), py_str("decorations"), py_str("present"), py_str("gift"), py_str("what to wear to"), py_str("activities"), py_str("things to do"), py_str("party games"), py_str("trick or treating"), py_str("carve"), py_str("pumpkin")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _story(args: &[Py]) -> Py {
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
    let _ = py_str("A story that holds up: the engine, and what makes a detail believable.");
    return py_str("Build it from a want, a wall and a turn: what the character wants in the first scene, what stops them, and the one thing that changes the plan. A plot that sounds far-fetched is usually missing the practical steps - the key, the alibi, the person who noticed - so make the method vague and the aftermath exact: the cup half drunk, the dog still alive, the wrong person blamed. If you want a villain's plan to be believable, give them one small mistake a careful reader could spot on a second read, and let a minor character be the one who spots it.");
    Py::None
}
pub fn _prose(args: &[Py]) -> Py {
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
    let _ = py_str("How to make flat writing less flat, in the order that changes the most per minute.");
    return py_str("Cut the adverbs and put one thing a reader can see in every paragraph: not 'the kitchen was messy' but 'the bin was full of cereal boxes'. Then vary the sentence length - short where it is tense, longer where it is calm - and read it aloud, because the sentence you stumble on is the one to fix. That is three passes, and they fix most flat writing.");
    Py::None
}
pub fn _naming(args: &[Py]) -> Py {
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
    let _ = py_str("A name for a shop, a band, a stall: the method, with the examples that show it.");
    return py_str("Name the thing, not the idea: a bakery called Crust, a cafe called The Second Cup, a band called The Rosy Hours. Two short words joined - place plus trade, your own name plus what you sell - beats anything clever, because people have to say it out loud and spell it down a phone. Say it with 'I'm just going to' in front of it, check that the domain and the social handle are free before you fall in love with it, and check Companies House for the limited company: a name can be yours to trade under and still be taken as a company.");
    Py::None
}
pub fn _costume_word(args: &[Py]) -> Py {
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
    let mut tip: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The tip for the character the line names, or \"\" when it names none of them.");
    let __iter1 = py_iter_or_raise(&g__COSTUME_TIPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    tip = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return tip.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _making_a_costume(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks for a costume to be made or worn, not for what to do that day.");
    line = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COSTUME_MAKING(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_costume(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut tip: Py = Py::None;
    let _ = py_str("How to make a costume at home: the method, then the character if the line names one.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    tip = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _costume_word(args))(&__args) };
    if py_truthy(&tip.clone()) {
    return op_add(&op_add(&g__THE_METHOD(), &py_str(" ")), &tip.clone());
    }
    return g__THE_METHOD();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.creative".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_COSTUME_MAKING" => Some(g__COSTUME_MAKING()),
            "_COSTUME_TIPS" => Some(g__COSTUME_TIPS()),
            "_THE_METHOD" => Some(g__THE_METHOD()),
            "_a_costume" => Some(py_func("_a_costume", |args: &[Py]| _a_costume(args))),
            "_costume_word" => Some(py_func("_costume_word", |args: &[Py]| _costume_word(args))),
            "_making_a_costume" => Some(py_func("_making_a_costume", |args: &[Py]| _making_a_costume(args))),
            "_naming" => Some(py_func("_naming", |args: &[Py]| _naming(args))),
            "_prose" => Some(py_func("_prose", |args: &[Py]| _prose(args))),
            "_story" => Some(py_func("_story", |args: &[Py]| _story(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_COSTUME_MAKING" => s__COSTUME_MAKING(v),
            "_COSTUME_TIPS" => s__COSTUME_TIPS(v),
            "_THE_METHOD" => s__THE_METHOD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.creative", module_obj);
    pyrt::xrefs::reg_fn("knowledge.creative", "_story", py_func("_story", |args: &[Py]| _story(args)));
    pyrt::xrefs::reg_fn("knowledge.creative", "_prose", py_func("_prose", |args: &[Py]| _prose(args)));
    pyrt::xrefs::reg_fn("knowledge.creative", "_naming", py_func("_naming", |args: &[Py]| _naming(args)));
    pyrt::xrefs::reg_fn("knowledge.creative", "_costume_word", py_func("_costume_word", |args: &[Py]| _costume_word(args)));
    pyrt::xrefs::reg_fn("knowledge.creative", "_making_a_costume", py_func("_making_a_costume", |args: &[Py]| _making_a_costume(args)));
    pyrt::xrefs::reg_fn("knowledge.creative", "_a_costume", py_func("_a_costume", |args: &[Py]| _a_costume(args)));
    pyrt::xrefs::reg_access("knowledge.creative", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.creative", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.creative", "_COSTUME_MAKING", g__COSTUME_MAKING, s__COSTUME_MAKING);
    pyrt::xrefs::reg_access("knowledge.creative", "_COSTUME_TIPS", g__COSTUME_TIPS, s__COSTUME_TIPS);
    pyrt::xrefs::reg_access("knowledge.creative", "_THE_METHOD", g__THE_METHOD, s__THE_METHOD);
}
