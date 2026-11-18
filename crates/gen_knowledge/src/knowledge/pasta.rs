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
thread_local! { static G_DISH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOODLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASTA_FOR_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASTA_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASTA_MINUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FAULT_IN_THE_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_DISH_WORDS() -> Py { ensure_init(); G_DISH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WORDS(v: Py) { ensure_init(); G_DISH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS_ASK() -> Py { ensure_init(); G_KINDS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS_ASK(v: Py) { ensure_init(); G_KINDS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOODLES() -> Py { ensure_init(); G_NOODLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOODLES(v: Py) { ensure_init(); G_NOODLES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DISH() -> Py { ensure_init(); G_NOT_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DISH(v: Py) { ensure_init(); G_NOT_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TIME() -> Py { ensure_init(); G_NOT_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TIME(v: Py) { ensure_init(); G_NOT_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_PASTA_FOR_DISH() -> Py { ensure_init(); G_PASTA_FOR_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASTA_FOR_DISH(v: Py) { ensure_init(); G_PASTA_FOR_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_PASTA_KINDS() -> Py { ensure_init(); G_PASTA_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASTA_KINDS(v: Py) { ensure_init(); G_PASTA_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PASTA_MINUTES() -> Py { ensure_init(); G_PASTA_MINUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASTA_MINUTES(v: Py) { ensure_init(); G_PASTA_MINUTES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHAPES() -> Py { ensure_init(); G_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHAPES(v: Py) { ensure_init(); G_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ASK() -> Py { ensure_init(); G_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ASK(v: Py) { ensure_init(); G_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FAULT_IN_THE_DISH() -> Py { ensure_init(); G__A_FAULT_IN_THE_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FAULT_IN_THE_DISH(v: Py) { ensure_init(); G__A_FAULT_IN_THE_DISH.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Pasta, on its own: how long a shape takes, and the questions that were answered next door.\n\nPasta is the one everyday food whose answer depends on which shape is in the pan - a strand of\nthin spaghetti is done in eight minutes, a tube of rigatoni needs thirteen, and fresh pasta\nneeds two - so a single \"8 to 12\" answers nobody. The times live here as data, one row per\nshape, and the entry asks for a word for cooking or timing as well, so \"what goes with pasta\"\nand \"how do i keep leftover pasta\" stay with the entries that answer them.\n\nThe rest is pasta that was being answered by a neighbour: a mushy pan read as rice, lasagne\nsheets read as bed linen, a squash read as a strand, and pasta in the oven read as nothing.\nEach one names its own subject here rather than taking the neighbour's answer.\n\nThis is a domain of its own rather than a part of bot/knowledge/cooking.py because the cooking\ntable is the busiest list in the repository - every cooking entry lands at the end of it - and a\ndomain is one file with ENTRIES, per bot/knowledge/__init__.py.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_TIME_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("minute"), py_str("mins"), py_str("time"), py_str("long"), py_str("take"), py_str("takes"), py_str("took"), py_str("quickest")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PASTA_MINUTES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("thin spaghetti"), py_str("spaghettini"), py_str("angel hair"), py_str("linguine"), py_str("vermicelli"), py_str("capellini")]), py_str("8 to 9 minutes")]), py_tuple(vec![py_tuple(vec![py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("farfalle"), py_str("bow"), py_str("bows")]), py_str("10 to 11 minutes")]), py_tuple(vec![py_tuple(vec![py_str("rigatoni"), py_str("tagliatelle"), py_str("orecchiette"), py_str("cannelloni"), py_str("pappardelle"), py_str("conchiglie")]), py_str("11 to 13 minutes")]), py_tuple(vec![py_tuple(vec![py_str("tortellini"), py_str("ravioli"), py_str("gnocchi"), py_str("tortelloni")]), py_str("3 to 4 minutes")])]));
    s_NOODLES(py_str("Noodles are quick but not all the same: thin or instant ones are 3 to 4 minutes, egg noodles 4 to 5, and dried rice noodles 6 to 8, so check the packet for yours."));
    s_SHAPES(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PASTA_MINUTES()) { let __unpk4402 = py_unpack(&__item.clone(), 2, None); let mut keys: Py = __unpk4402[0].clone(); let mut _minutes: Py = __unpk4402[1].clone(); for __item in py_iter_or_raise(&keys.clone()) { let mut shape: Py = __item.clone(); __out.push(shape.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_tuple(vec![py_str("noodles"), py_str("noodle")])));
    s_NOT_A_TIME(py_tuple(vec![py_str("squash"), py_str("spaghetti squash"), py_str("butternut"), py_str("courgette"), py_str("zucchini"), py_str("pumpkin"), py_str("in the oven"), py_str("pasta bake"), py_str("baked pasta"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("how many sheets"), py_str("how many lasagne"), py_str("how much"), py_str("portion"), py_str("portions"), py_str("per person"), py_str("fridge"), py_str("freezer"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("leftover"), py_str("left over"), py_str("next day"), py_str("reheat"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("days"), py_str("day"), py_str("gone off"), py_str("sauce recipe"), py_str("salad"), py_str("sticking"), py_str("stuck"), py_str("clump"), py_str("clumps"), py_str("boil over"), py_str("boils over"), py_str("boiling over"), py_str("boiled over"), py_str("overflow"), py_str("overflows"), py_str("spill"), py_str("spills"), py_str("spilling"), py_str("spilled"), py_str("spilt")]));
    s_PASTA_KINDS(py_tuple(vec![py_tuple(vec![py_str("long strands"), py_str("spaghetti, linguine and tagliatelle"), py_str("a ragu, a carbonara or a butter sauce the strands can carry")]), py_tuple(vec![py_str("tubes"), py_str("penne and rigatoni"), py_str("a baked dish, and a thick sauce that fills the tube")]), py_tuple(vec![py_str("twists and shells"), py_str("fusilli, farfalle and conchiglie"), py_str("a sauce with bits in it that needs something to cling to, and pasta salad")]), py_tuple(vec![py_str("tiny shapes"), py_str("orzo and stelline"), py_str("soup, and a salad that should not be all bulk")]), py_tuple(vec![py_str("sheets"), py_str("lasagne"), py_str("a bake, layered instead of tossed")]), py_tuple(vec![py_str("filled"), py_str("ravioli and tortellini"), py_str("a meal on its own with butter or a light sauce")])]));
    s_KINDS_ASK(py_tuple(vec![py_str("kinds of pasta"), py_str("types of pasta"), py_str("sorts of pasta"), py_str("varieties of pasta"), py_str("kind of pasta"), py_str("type of pasta"), py_str("sort of pasta"), py_str("shapes of pasta"), py_str("shape of pasta"), py_str("what shapes"), py_str("which shapes"), py_str("what kinds"), py_str("what types"), py_str("different types"), py_str("different kinds"), py_str("how many types"), py_str("how many kinds")]));
    s_PASTA_FOR_DISH(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("ragu"), py_str("bolognese"), py_str("meat sauce"), py_str("slow cooked"), py_str("slow-cooked")]), py_str("Tagliatelle or pappardelle, or rigatoni"), py_str("a wide strand has the surface a thick meat sauce clings to, and a tube holds it; spaghetti is fine but most of the sauce ends up on the plate")]), py_tuple(vec![py_tuple(vec![py_str("carbonara"), py_str("creamy sauce"), py_str("cream sauce"), py_str("cheese sauce"), py_str("macaroni cheese"), py_str("mac and cheese")]), py_str("Spaghetti or rigatoni"), py_str("the sauce is a coating rather than a filling, so a long strand or a tube both work, and the ridges on rigatoni hold it")]), py_tuple(vec![py_tuple(vec![py_str("bake"), py_str("baked"), py_str("oven"), py_str("lasagne"), py_str("lasagna")]), py_str("Penne, rigatoni or lasagne sheets"), py_str("a tube keeps its shape through the oven and the sauce gets inside it, where a long strand turns to one lump")]), py_tuple(vec![py_tuple(vec![py_str("soup"), py_str("broth"), py_str("minestrone")]), py_str("Orzo, stelline or any tiny shape"), py_str("small shapes cook in the broth and sit on the spoon, and orzo thickens it a little")]), py_tuple(vec![py_tuple(vec![py_str("salad"), py_str("cold")]), py_str("Fusilli, farfalle or penne"), py_str("the nooks and the flat middles catch the dressing, and none of them clump when they are cold")]), py_tuple(vec![py_tuple(vec![py_str("pesto")]), py_str("Trofie, fusilli or linguine"), py_str("trofie is the shape pesto comes from, and the others give it something to cling to")]), py_tuple(vec![py_tuple(vec![py_str("seafood"), py_str("prawns"), py_str("prawn"), py_str("clams"), py_str("mussels")]), py_str("Linguine or spaghetti"), py_str("a long strand twines round the shellfish and the sauce stays light instead of collecting in a tube")])]));
    s_DISH_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PASTA_FOR_DISH()) { let __unpk4194 = py_unpack(&__item.clone(), 3, None); let mut keys: Py = __unpk4194[0].clone(); let mut _shapes: Py = __unpk4194[1].clone(); let mut _why: Py = __unpk4194[2].clone(); for __item in py_iter_or_raise(&keys.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_DISH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_TIME()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_DISH_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FAULT_IN_THE_DISH(py_tuple(vec![py_str("watery"), py_str("soggy"), py_str("limp"), py_str("dry"), py_str("drier"), py_str("bland"), py_str("tasteless"), py_str("mushy"), py_str("stodgy"), py_str("sticky"), py_str("gluey"), py_str("overcooked"), py_str("burnt"), py_str("salty"), py_str("too salty"), py_str("undercooked"), py_str("stale")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-for-dish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("rigatoni"), py_str("tagliatelle"), py_str("linguine"), py_str("farfalle"), py_str("conchiglie"), py_str("ravioli"), py_str("tortellini"), py_str("macaroni"), py_str("orzo"), py_str("trofie"), py_str("pappardelle")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_DISH_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])); __args.push(g__for_the_dish()); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_NOT_A_DISH(), &g__A_FAULT_IN_THE_DISH()), &py_tuple(vec![py_str("how long"), py_str("how much"), py_str("per person"), py_str("portion"), py_str("portions"), py_str("calories"), py_str("carbs"), py_str("instead"), py_str("substitute"), py_str("swap"), py_str("stand in"), py_str("replace"), py_str("difference between"), py_str("how do i"), py_str("recipe"), py_str("recipes"), py_str("how to make"), py_str("how do i make"), py_str("im making"), py_str("i am making"), py_str("im cooking"), py_str("i am cooking")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-kinds")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("rigatoni"), py_str("tagliatelle"), py_str("linguine"), py_str("farfalle"), py_str("conchiglie"), py_str("ravioli"), py_str("tortellini"), py_str("lasagne"), py_str("orzo"), py_str("macaroni")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_KINDS_ASK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])); __args.push(g__the_kinds()); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_A_DISH(), &py_tuple(vec![py_str("how long"), py_str("how much"), py_str("per person"), py_str("portion"), py_str("portions"), py_str("gluten"), py_str("free"), py_str("vegan"), py_str("vegetarian"), py_str("calories"), py_str("carbs"), py_str("instead"), py_str("substitute"), py_str("swap"), py_str("stand in"), py_str("replace"), py_str("which one"), py_str("which is better"), py_str("difference between")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-time")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("rigatoni"), py_str("tagliatelle"), py_str("linguine"), py_str("farfalle"), py_str("orecchiette"), py_str("tortellini"), py_str("ravioli"), py_str("gnocchi"), py_str("cannelloni"), py_str("noodles"), py_str("noodle")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_TIME_ASK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])); __args.push(g__pasta_time()); __kws.extend(vec![("avoid".to_string(), g_NOT_A_TIME()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-pan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("rigatoni"), py_str("tagliatelle"), py_str("linguine"), py_str("farfalle"), py_str("orecchiette"), py_str("tortellini"), py_str("ravioli"), py_str("gnocchi"), py_str("cannelloni"), py_str("noodles"), py_str("noodle")]), py_tuple(vec![py_str("pan"), py_str("pans"), py_str("pot"), py_str("pots"), py_str("saucepan"), py_str("saucepans"), py_str("wok"), py_str("size"), py_str("big"), py_str("large"), py_str("wide"), py_str("deep"), py_str("enough room")])])); __args.push(py_str("A big pot with room to spare: about 5 litres of water for 500g of pasta, so it comes back to the boil quickly and the strands do not stick together. A wide shallow pan or a large frying pan is fine for long shapes if you stir, and easier for a small portion; any deep saucepan works for short shapes.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_A_TIME(), &py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("non stick"), py_str("nonstick"), py_str("non-stick"), py_str("cast iron"), py_str("coating"), py_str("season"), py_str("seasoning"), py_str("clean"), py_str("wash"), py_str("scour")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-bake")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("lasagne"), py_str("lasagna"), py_str("macaroni"), py_str("cannelloni"), py_str("pasta bake")]), py_tuple(vec![py_str("in the oven"), py_str("oven"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("pasta bake")])])); __args.push(py_str("Cook the pasta 2 minutes short of the packet, mix it with a sauce that is wetter than you think, top it and bake it 20 to 25 minutes at 200C (180C fan) until the top is browned and bubbling. Cover it with foil for the first 15 minutes and the middle stays loose instead of drying out.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("boil"), py_str("boiling"), py_str("water"), py_str("portion"), py_str("portions"), py_str("per person"), py_str("dry"), py_str("dried"), py_str("leftover"), py_str("left over"), py_str("freeze"), py_str("reheat"), py_str("how many"), py_str("im making"), py_str("i am making"), py_str("im cooking"), py_str("i am cooking"), py_str("im doing"), py_str("i am doing"), py_str("im baking"), py_str("i am baking"), py_str("we are making"), py_str("we are cooking")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("squash-roast")); __args.push(py_tuple(vec![py_tuple(vec![py_str("squash"), py_str("spaghetti squash"), py_str("butternut"), py_str("pumpkin"), py_str("acorn squash")]), py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("oven"), py_str("bake"), py_str("baked"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("long"), py_str("time"), py_str("minutes")])])); __args.push(py_str("Halve it, scoop out the seeds and roast it cut side down at 200C for 35 to 40 minutes, until a knife goes into the flesh like butter; spaghetti squash wants 40 to 45, then fork it into strands. A whole one takes nearer an hour.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("blossom end"), py_str("rot"), py_str("growing"), py_str("grow"), py_str("seedling"), py_str("plant"), py_str("plot"), py_str("soil"), py_str("court"), py_str("racket"), py_str("game"), py_str("player"), py_str("drink"), py_str("cordial"), py_str("juice"), py_str("sirop"), py_str("seeds to sow"), py_str("variety")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-soft")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("macaroni"), py_str("fusilli"), py_str("tagliatelle"), py_str("rigatoni"), py_str("linguine"), py_str("noodles"), py_str("tortellini")]), py_tuple(vec![py_str("mushy"), py_str("soggy"), py_str("overcooked"), py_str("over cooked"), py_str("too soft"), py_str("soft"), py_str("slimy"), py_str("ruined"), py_str("sticky"), py_str("went wrong")])])); __args.push(py_str("Overcooked pasta has taken on water and lost its bite, so drain it now and stop the cooking under cold water. Toss it straight into a thick sauce - a ragu, a bake, or tomato with a spoon of cream cheese - rather than a thin one it cannot hold, and it will pass. Next time taste a piece a minute before the packet time.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("rice"), py_str("clump"), py_str("clumps"), py_str("together"), py_str("stuck"), py_str("sticking"), py_str("pan"), py_str("how long"), py_str("minutes"), py_str("portion"), py_str("leftover"), py_str("sauce recipe"), py_str("in the oven"), py_str("bake")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drain-the-pot")); __args.push(py_tuple(vec![py_tuple(vec![py_str("drain"), py_str("drains"), py_str("draining"), py_str("drained"), py_str("strain"), py_str("strains"), py_str("straining"), py_str("strained"), py_str("colander"), py_str("sieve")]), py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("macaroni"), py_str("linguine"), py_str("fusilli"), py_str("tagliatelle"), py_str("orzo"), py_str("noodles"), py_str("noodle"), py_str("potato"), py_str("potatoes"), py_str("new potatoes"), py_str("veg"), py_str("vegetables"), py_str("broccoli"), py_str("cauliflower"), py_str("carrots"), py_str("carrot"), py_str("greens"), py_str("peas"), py_str("green beans"), py_str("spinach"), py_str("kale"), py_str("cabbage"), py_str("courgette"), py_str("courgettes"), py_str("chickpeas"), py_str("lentils"), py_str("beans"), py_str("sweetcorn")])])); __args.push(py_str("Tip the whole pan into a colander in the sink, and keep a mugful of the water back first, because it is full of starch and it loosens a sauce that looks dry. Drain it the moment it is al dente or just tender - it keeps cooking in its own heat - then shake it dry and get it into the sauce or back into the warm pan, since anything left standing in a colander goes sticky within a couple of minutes. No colander is no problem: hold the lid on with a gap and pour slowly.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_A_TIME(), &py_tuple(vec![py_str("rinse"), py_str("rinsing"), py_str("washed"), py_str("wash"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("leftover"), py_str("left over"), py_str("reheat"), py_str("rice"), py_str("sauce"), py_str("bake"), py_str("oven"), py_str("salad"), py_str("cheese"), py_str("tin"), py_str("tinned")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lasagne-sheets")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lasagne"), py_str("lasagna")]), py_tuple(vec![py_str("sheets"), py_str("sheet"), py_str("pre cook"), py_str("precook"), py_str("pre cooked"), py_str("boil"), py_str("soak"), py_str("how long"), py_str("cook"), py_str("cooking"), py_str("fresh"), py_str("dried")])])); __args.push(py_str("Fresh lasagne sheets need no cooking at all: layer them straight in. Dried sheets made without egg go in dry too, as long as the sauce is wet enough; dried egg pasta is better for 3 or 4 minutes in boiling water first. Either way rest the finished dish 10 minutes before cutting so it holds together.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how long do i bake"), py_str("in the oven"), py_str("oven"), py_str("reheat"), py_str("freeze"), py_str("freezer"), py_str("calories"), py_str("per person"), py_str("how many"), py_str("how much"), py_str("leftover"), py_str("left over"), py_str("vegetarian"), py_str("vegan"), py_str("recipe"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("fridge"), py_str("store"), py_str("storage"), py_str("safe"), py_str("next day"), py_str("gone off")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _the_kinds(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut family: Py = Py::None;
    let mut for_what: Py = Py::None;
    let mut members: Py = Py::None;
    let mut shapes: Py = Py::None;
    let _ = py_str("The families of shape, what each is for, and the times those shapes already have.");
    members = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PASTA_KINDS()) { let __unpk1411 = py_unpack(&__item.clone(), 3, None); let mut family: Py = __unpk1411[0].clone(); let mut shapes: Py = __unpk1411[1].clone(); let mut for_what: Py = __unpk1411[2].clone(); __out.push(op_modulo(&py_str("%s (%s) for %s"), &py_tuple(vec![family.clone(), shapes.clone(), for_what.clone()]))); } py_list(__out) })() };
    return op_add(&op_add(&op_add(&op_add(&py_str("Pasta comes down to six families, and the family decides the job: "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&members.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("; "), "join", args))(&__args) }), &py_str(" and ")), &geti(&members.clone(), &op_neg(&Py::Int(1)))), &py_str(". The time goes by the shape as well: 8 to 9 minutes for thin strands, 10 to 11 for spaghetti and penne, 11 to 13 for rigatoni and tagliatelle, and 3 to 4 for the filled ones. Fresh pasta is 2 to 3 minutes, not the packet time."));
    Py::None
}
pub fn _minutes(args: &[Py]) -> Py {
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
    let mut joined: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut minutes: Py = Py::None;
    let _ = py_str("The minutes for the shape the line names, or the range that covers dried pasta.");
    joined = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&joined.clone(), &py_str("noodle"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&joined.clone(), &py_str("noodles"))) } }) {
    return g_NOODLES();
    }
    let __iter1 = py_iter_or_raise(&g_PASTA_MINUTES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    keys = __unpk2[0].clone();
    minutes = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_contains(&joined.clone(), &key.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return op_modulo(&py_str("About %s for that one."), &minutes.clone());
    }
    }
    return py_str("Dried pasta is 8 to 12 minutes, depending on the shape: 8 to 9 for thin spaghetti.");
    Py::None
}
pub fn _pasta_time(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The time by shape, then the two things that decide whether it is right.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("fresh")))) {
    first = py_str("Fresh pasta is 2 to 3 minutes, not the packet time: it is done the moment it floats and loses its chalky middle, so taste a strand at 2 minutes.");
    } else {
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _minutes(args))(&__args) };
    }
    return op_add(&first.clone(), &py_str(" Use a big pan with about 10g of salt per litre, keep a mug of the water for the sauce, and taste a piece a minute before the packet says: shapes differ, and so do pans."));
    Py::None
}
pub fn _for_the_dish(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shapes: Py = Py::None;
    let mut why: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The shapes for the dish the line names, then why those and not the ones next to them.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    said = { let __b = said.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) } } };
    let __iter1 = py_iter_or_raise(&g_PASTA_FOR_DISH());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    keys = __unpk2[0].clone();
    shapes = __unpk2[1].clone();
    why = __unpk2[2].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &key.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return op_modulo(&py_str("%s, because %s."), &py_tuple(vec![shapes.clone(), why.clone()]));
    }
    }
    return py_str("Match the shape to the sauce rather than to the packet: a thick meat sauce wants a wide strand or a tube, a thin one wants a long strand, and a bake wants penne or rigatoni.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.pasta".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "DISH_WORDS" => Some(g_DISH_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "KINDS_ASK" => Some(g_KINDS_ASK()),
            "NOODLES" => Some(g_NOODLES()),
            "NOT_A_DISH" => Some(g_NOT_A_DISH()),
            "NOT_A_TIME" => Some(g_NOT_A_TIME()),
            "PASTA_FOR_DISH" => Some(g_PASTA_FOR_DISH()),
            "PASTA_KINDS" => Some(g_PASTA_KINDS()),
            "PASTA_MINUTES" => Some(g_PASTA_MINUTES()),
            "SHAPES" => Some(g_SHAPES()),
            "TIME_ASK" => Some(g_TIME_ASK()),
            "_A_FAULT_IN_THE_DISH" => Some(g__A_FAULT_IN_THE_DISH()),
            "_for_the_dish" => Some(py_func("_for_the_dish", |args: &[Py]| _for_the_dish(args))),
            "_minutes" => Some(py_func("_minutes", |args: &[Py]| _minutes(args))),
            "_pasta_time" => Some(py_func("_pasta_time", |args: &[Py]| _pasta_time(args))),
            "_the_kinds" => Some(py_func("_the_kinds", |args: &[Py]| _the_kinds(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "DISH_WORDS" => s_DISH_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "KINDS_ASK" => s_KINDS_ASK(v),
            "NOODLES" => s_NOODLES(v),
            "NOT_A_DISH" => s_NOT_A_DISH(v),
            "NOT_A_TIME" => s_NOT_A_TIME(v),
            "PASTA_FOR_DISH" => s_PASTA_FOR_DISH(v),
            "PASTA_KINDS" => s_PASTA_KINDS(v),
            "PASTA_MINUTES" => s_PASTA_MINUTES(v),
            "SHAPES" => s_SHAPES(v),
            "TIME_ASK" => s_TIME_ASK(v),
            "_A_FAULT_IN_THE_DISH" => s__A_FAULT_IN_THE_DISH(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.pasta", module_obj);
    pyrt::xrefs::reg_fn("knowledge.pasta", "_the_kinds", py_func("_the_kinds", |args: &[Py]| _the_kinds(args)));
    pyrt::xrefs::reg_fn("knowledge.pasta", "_minutes", py_func("_minutes", |args: &[Py]| _minutes(args)));
    pyrt::xrefs::reg_fn("knowledge.pasta", "_pasta_time", py_func("_pasta_time", |args: &[Py]| _pasta_time(args)));
    pyrt::xrefs::reg_fn("knowledge.pasta", "_for_the_dish", py_func("_for_the_dish", |args: &[Py]| _for_the_dish(args)));
    pyrt::xrefs::reg_access("knowledge.pasta", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.pasta", "DISH_WORDS", g_DISH_WORDS, s_DISH_WORDS);
    pyrt::xrefs::reg_access("knowledge.pasta", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.pasta", "KINDS_ASK", g_KINDS_ASK, s_KINDS_ASK);
    pyrt::xrefs::reg_access("knowledge.pasta", "NOODLES", g_NOODLES, s_NOODLES);
    pyrt::xrefs::reg_access("knowledge.pasta", "NOT_A_DISH", g_NOT_A_DISH, s_NOT_A_DISH);
    pyrt::xrefs::reg_access("knowledge.pasta", "NOT_A_TIME", g_NOT_A_TIME, s_NOT_A_TIME);
    pyrt::xrefs::reg_access("knowledge.pasta", "PASTA_FOR_DISH", g_PASTA_FOR_DISH, s_PASTA_FOR_DISH);
    pyrt::xrefs::reg_access("knowledge.pasta", "PASTA_KINDS", g_PASTA_KINDS, s_PASTA_KINDS);
    pyrt::xrefs::reg_access("knowledge.pasta", "PASTA_MINUTES", g_PASTA_MINUTES, s_PASTA_MINUTES);
    pyrt::xrefs::reg_access("knowledge.pasta", "SHAPES", g_SHAPES, s_SHAPES);
    pyrt::xrefs::reg_access("knowledge.pasta", "TIME_ASK", g_TIME_ASK, s_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.pasta", "_A_FAULT_IN_THE_DISH", g__A_FAULT_IN_THE_DISH, s__A_FAULT_IN_THE_DISH);
}
