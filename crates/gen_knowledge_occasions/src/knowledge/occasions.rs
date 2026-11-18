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
thread_local! { static G_BRING_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_CARRY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_GUESTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_HOST_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_NOT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANY_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COOKING_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOST_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MAKE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DISH_TO_CARRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPREAD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_crosses_a_border: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BRING_ASKS() -> Py { ensure_init(); G_BRING_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_ASKS(v: Py) { ensure_init(); G_BRING_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_CARRY_WORDS() -> Py { ensure_init(); G_BRING_CARRY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_CARRY_WORDS(v: Py) { ensure_init(); G_BRING_CARRY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_DISH() -> Py { ensure_init(); G_BRING_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_DISH(v: Py) { ensure_init(); G_BRING_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_GUESTS() -> Py { ensure_init(); G_BRING_GUESTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_GUESTS(v: Py) { ensure_init(); G_BRING_GUESTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_HOST_NAMES() -> Py { ensure_init(); G_BRING_HOST_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_HOST_NAMES(v: Py) { ensure_init(); G_BRING_HOST_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_NOT_FOOD() -> Py { ensure_init(); G_BRING_NOT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_NOT_FOOD(v: Py) { ensure_init(); G_BRING_NOT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_WORDS() -> Py { ensure_init(); G_BRING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_WORDS(v: Py) { ensure_init(); G_BRING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC_FOOD() -> Py { ensure_init(); G_GENERIC_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC_FOOD(v: Py) { ensure_init(); G_GENERIC_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION_NAMES() -> Py { ensure_init(); G_OCCASION_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION_NAMES(v: Py) { ensure_init(); G_OCCASION_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__ANY_DISH() -> Py { ensure_init(); G__ANY_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANY_DISH(v: Py) { ensure_init(); G__ANY_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__COOKING_ASKS() -> Py { ensure_init(); G__COOKING_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COOKING_ASKS(v: Py) { ensure_init(); G__COOKING_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__HOST_OWN() -> Py { ensure_init(); G__HOST_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOST_OWN(v: Py) { ensure_init(); G__HOST_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__MAKE_ASKS() -> Py { ensure_init(); G__MAKE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MAKE_ASKS(v: Py) { ensure_init(); G__MAKE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DISH() -> Py { ensure_init(); G__NOT_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DISH(v: Py) { ensure_init(); G__NOT_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DISH_TO_CARRY() -> Py { ensure_init(); G__NOT_A_DISH_TO_CARRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DISH_TO_CARRY(v: Py) { ensure_init(); G__NOT_A_DISH_TO_CARRY.with(|c| *c.borrow_mut() = v); }
pub fn g__SPREAD_WORDS() -> Py { ensure_init(); G__SPREAD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPREAD_WORDS(v: Py) { ensure_init(); G__SPREAD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIP() -> Py { ensure_init(); G__TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIP(v: Py) { ensure_init(); G__TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_crosses_a_border() -> Py { ensure_init(); G_crosses_a_border.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_crosses_a_border(v: Py) { ensure_init(); G_crosses_a_border.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Occasions: what to take when the food is for someone else's do.\n\nA bring-a-dish question is not a cook-a-dinner question. \"What can I bring to a picnic\" names no\nfood and no meal, so nothing reached it and it got the shrug; \"what can i make for a bake sale\"\nreached the ten-minute dinners; \"what can i bring to a christening\" reached what-to-wear; and\n\"what can i bring to a friends house for dinner\" reached the friend who still owes you money. The\noccasion is what the ask is about, so the occasion is the key and the hint is what travels, what\ncan sit out and what to leave at home.\n\nA line that mentions an occasion without asking what to bring or make is not this entry's line:\n\"i always take a bottle to a picnic\" is a statement, and \"what should i take to a job interview\"\nis not food, so both are left to whatever answers them now.\n");
    s_crosses_a_border(py_func("crosses_a_border", |args: &[Py]| pyrt::xrefs::xcall("knowledge.geography", "crosses_a_border", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("bring"), py_str("brings"), py_str("bringing"), py_str("take"), py_str("takes"), py_str("taking"), py_str("make"), py_str("makes"), py_str("making"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("bake"), py_str("bakes"), py_str("baking"), py_str("food"), py_str("foods")]) }
    s_BRING_WORDS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("what can i bring"), py_str("what should i bring"), py_str("what do i bring"), py_str("what to bring"), py_str("anything i can bring"), py_str("shall i bring"), py_str("should i bring"), py_str("do i need to bring"), py_str("what can i take"), py_str("what should i take"), py_str("what do i take"), py_str("what to take"), py_str("shall i take"), py_str("should i take"), py_str("what can i make"), py_str("what should i make"), py_str("what do i make"), py_str("what to make"), py_str("shall i make"), py_str("should i make"), py_str("what can i bake"), py_str("what should i bake"), py_str("what to bake"), py_str("should i bake"), py_str("what can i cook"), py_str("what should i cook"), py_str("what do i cook"), py_str("what to cook"), py_str("shall i cook"), py_str("should i cook"), py_str("food for"), py_str("food to"), py_str("what food"), py_str("which food"), py_str("bring food"), py_str("bringing food"), py_str("bring some food"), py_str("bring a plate"), py_str("bring something to eat"), py_str("take food"), py_str("taking food")]) }
    s_BRING_ASKS(__data2());
    s_BRING_DISH(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("the wake"), py_str("a wake"), py_str("for the wake"), py_str("for a wake"), py_str("after the wake"), py_str("after a wake"), py_str("at the wake"), py_str("at a wake"), py_str("funeral"), py_str("funeral tea")]), py_str("Sandwiches, a big pot of soup and a traybake: food that can sit out and be eaten whenever people arrive, and that reheats for the family afterwards. Take it in a dish you do not want back, and offer one specific job - pouring tea, washing up - rather than asking what you can do.")]), py_tuple(vec![py_tuple(vec![py_str("picnic"), py_str("picnics")]), py_str("Take something that survives a warm car: a frittata cut in wedges, a salad with the dressing in the bottom of the jar, or cold roast chicken with a bag of ice packs. Flapjacks and fruit travel better than anything with cream in it.")]), py_tuple(vec![py_tuple(vec![py_str("barbecue"), py_str("barbecues"), py_str("bbq"), py_str("bbqs"), py_str("barbeque"), py_str("cookout")]), py_str("Take something that needs nothing from the grill: a big bowl of slaw, garlic bread, or halloumi and vegetable skewers if you want something cooked. Ice and a bottle of something land harder than another bag of crisps.")]), py_tuple(vec![py_tuple(vec![py_str("potluck"), py_str("pot luck"), py_str("pot lucks"), py_str("bring and share")]), py_str("Take a dish that holds its heat and needs nothing at the last minute: a tray of roast vegetables, a big grain salad, or a chilli to plug in. Ask what is already coming first, so you are not the third pasta bake.")]), py_tuple(vec![py_tuple(vec![py_str("bake sale"), py_str("cake sale"), py_str("cake stall"), py_str("school fair"), py_str("summer fair"), py_str("fete"), py_str("fayre")]), py_str("Traybakes cut into squares travel and sell best: brownies, flapjacks or a Victoria sponge traybake. Write the allergens on a card and take a tin or a box, because a stall always runs out of somewhere to put things.")]), py_tuple(vec![py_tuple(vec![py_str("buffet"), py_str("buffets"), py_str("christening"), py_str("christenings"), py_str("hen do"), py_str("leaving do"), py_str("retirement do"), py_str("naming ceremony"), py_str("leaving party"), py_str("retirement party")]), py_str("Cold things that hold for an hour and need no oven: cheese straws, mini quiches, cocktail sausages, hummus with breadsticks, and a bowl of something green. Two hot trays are plenty, and everything else can be made the day before.")]), py_tuple(vec![py_tuple(vec![py_str("dinner party"), py_str("dinner parties"), py_str("friends house"), py_str("friends place"), py_str("friend house"), py_str("their house"), py_str("round for dinner"), py_str("over for dinner"), py_str("for dinner at"), py_str("for dinner")]), py_str("Take a bottle and something for after - a pudding, cheese and biscuits, or a box of chocolates - so you are not competing for their oven. Ask what they are cooking, then bring the thing that goes with it.")])]));
    s_BRING_HOST_NAMES(py_tuple(vec![py_str("dinner party"), py_str("dinner parties"), py_str("friends house"), py_str("friends place"), py_str("friend house"), py_str("their house"), py_str("round for dinner"), py_str("over for dinner"), py_str("for dinner at"), py_str("for dinner")]));
    s_BRING_CARRY_WORDS(py_tuple(vec![py_str("bring"), py_str("brings"), py_str("bringing"), py_str("take"), py_str("takes"), py_str("taking"), py_str("made"), py_str("bought"), py_str("picked"), py_str("ordered")]));
    fn __data3() -> Py { py_tuple(vec![py_str("take my friend"), py_str("bring my friend"), py_str("my friend to"), py_str("my friend along"), py_str("my friend come"), py_str("bring a friend"), py_str("take a friend"), py_str("my friend with me"), py_str("take my partner"), py_str("bring my partner"), py_str("my partner to"), py_str("my partner along"), py_str("take my husband"), py_str("bring my husband"), py_str("take my wife"), py_str("bring my wife"), py_str("take my mum"), py_str("bring my mum"), py_str("take my mother"), py_str("bring my mother"), py_str("take my dad"), py_str("bring my dad"), py_str("take my father"), py_str("bring my father"), py_str("take my parents"), py_str("bring my parents")]) }
    s_BRING_GUESTS(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("beach"), py_str("the beach"), py_str("road trip"), py_str("the park"), py_str("park"), py_str("walk in the woods"), py_str("the woods"), py_str("mountain"), py_str("mountains"), py_str("hike"), py_str("hiking"), py_str("rambling"), py_str("long walk"), py_str("walk"), py_str("camping"), py_str("campsite"), py_str("campervan"), py_str("caravan"), py_str("day out"), py_str("day trip"), py_str("journey"), py_str("for the journey"), py_str("car journey"), py_str("train journey"), py_str("wild swim"), py_str("on the river"), py_str("up a hill")]) }
    s__TRIP(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("my dog"), py_str("the dog"), py_str("our dog"), py_str("my cat"), py_str("the cat"), py_str("my baby"), py_str("the baby"), py_str("our baby"), py_str("my kids"), py_str("my children"), py_str("my son"), py_str("my daughter"), py_str("wheelchair"), py_str("buggy"), py_str("pushchair"), py_str("pram"), py_str("car seat"), py_str("my phone"), py_str("my laptop"), py_str("my camera")]) }
    s_BRING_NOT_FOOD(op_add(&op_add(&__data5(), &g_BRING_GUESTS()), &g__TRIP()));
    s__NOT_A_DISH(py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how often"), py_str("how long")]));
    fn __data6() -> Py { py_tuple(vec![py_str("im hosting"), py_str("i'm hosting"), py_str("we're hosting"), py_str("were hosting"), py_str("im having"), py_str("i'm having"), py_str("we're having"), py_str("were having"), py_str("im throwing"), py_str("i'm throwing"), py_str("im putting on"), py_str("i'm putting on"), py_str("im organising"), py_str("i'm organising"), py_str("im organizing"), py_str("i'm organizing"), py_str("hosting a"), py_str("hosting an")]) }
    s__HOST_OWN(__data6());
    s__MAKE_ASKS(py_tuple(vec![py_str("shall i make"), py_str("should i make"), py_str("shall i cook"), py_str("should i cook"), py_str("can i make"), py_str("should i bake"), py_str("shall i bake")]));
    s__COOKING_ASKS(op_add(&g__MAKE_ASKS(), &py_tuple(vec![py_str("can i cook"), py_str("to cook for"), py_str("to make for"), py_str("to bake for"), py_str("should i cook for"), py_str("what should i cook"), py_str("what can i cook"), py_str("what to cook"), py_str("what should i make"), py_str("what can i make"), py_str("what to make")])));
    s__SPREAD_WORDS(py_tuple(vec![py_str("tea"), py_str("spread"), py_str("buffet"), py_str("lunch"), py_str("dinner"), py_str("supper"), py_str("brunch")]));
    s_GENERIC_FOOD(py_tuple(vec![py_str("food"), py_str("foods"), py_str("dish"), py_str("dishes"), py_str("plate"), py_str("plates"), py_str("something to eat")]));
    fn __data7() -> Py { py_tuple(vec![py_str("birthday"), py_str("birthdays"), py_str("housewarming"), py_str("house warming"), py_str("engagement"), py_str("baby shower"), py_str("wedding"), py_str("christening present"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("interview"), py_str("work"), py_str("office"), py_str("fridge"), py_str("freezer"), py_str("on the plane"), py_str("on a plane"), py_str("flight"), py_str("through security"), py_str("hand luggage"), py_str("cabin bag"), py_str("restaurant"), py_str("restaurants"), py_str("cafe"), py_str("cafes"), py_str("the pub"), py_str("takeaway"), py_str("ferry"), py_str("on the ferry"), py_str("on a ferry"), py_str("on the train"), py_str("on a train"), py_str("on the coach"), py_str("on the bus")]) }
    s__NOT_A_DISH_TO_CARRY(__data7());
    s__ANY_DISH(py_str("Take something that needs nothing from anyone else's kitchen: a big bowl of pasta salad, a tray of sausages or a quiche, cheese and crackers, or a traybake cut into squares. Nothing with cream in it if it will sit out, and take it in a dish you do not need back. If the host says not to bring anything, take a bottle or something for after - and ask what is already coming so you are not the third pasta bake."));
    s_OCCASION_NAMES(op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BRING_DISH()) { let __unpk8986 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk8986[0].clone(); let mut _hint: Py = __unpk8986[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g_GENERIC_FOOD()));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bring-dish")); __args.push(py_tuple(vec![g_BRING_WORDS(), g_OCCASION_NAMES()])); __args.push(py_func("what_to_bring", |args: &[Py]| what_to_bring(args))); __kws.extend(vec![("ask".to_string(), g_BRING_ASKS()), ("avoid".to_string(), g_BRING_NOT_FOOD()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _same(args: &[Py]) -> Py {
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
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let _ = py_str("One word equals another, allowing a plural: \"picnics\" is \"picnic\".");
    word = mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str("'")]);
    wanted = mcall(&call_builtin("str", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &wanted.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&py_tuple(vec![op_add(&wanted.clone(), &py_str("s")), op_add(&wanted.clone(), &py_str("es"))]), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![op_add(&word.clone(), &py_str("s")), op_add(&word.clone(), &py_str("es"))]), &wanted.clone())) } };
    Py::None
}
pub fn _words_of(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    said = mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    }
    clean = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), op_add(&op_add(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]), &py_str(" ")), &mcall(&clean.clone(), "lower", &[]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn what_to_bring(args: &[Py]) -> Py {
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
    let mut asker: Py = Py::None;
    let mut carry: Py = Py::None;
    let mut carry_word: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut food: Py = Py::None;
    let mut hint: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut offset: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The occasion the line names decides what travels.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_DISH()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    words = _words_of(&[understood.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HOST_OWN()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MAKE_ASKS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SPREAD_WORDS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&clean.clone(), "split", &[]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("bring"), py_str("take")])) { let mut carry_word: Py = __item.clone(); __out.push(_same(&[word.clone(), carry_word.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    carry = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g_BRING_CARRY_WORDS()) { let mut carry_word: Py = __item.clone(); __out.push(_same(&[word.clone(), carry_word.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("wants")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("place")))) {
    return Py::None;
    }
    asker = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BRING_ASKS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COOKING_ASKS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&carry.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_BRING_DISH());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    hint = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BRING_HOST_NAMES(), &name.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&carry.clone())) } }) {
    continue;
    }
    parts = mcall(&name.clone(), "split", &[]);
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    start = __item.clone();
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut offset: Py = __item.clone(); __out.push(_same(&[geti(&words.clone(), &op_add(&start.clone(), &offset.clone())), geti(&parts.clone(), &offset.clone())])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return hint.clone();
    }
    }
    }
    }
    if py_truthy(&{ let __b = asker.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.geography", "crosses_a_border", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g__NOT_A_DISH_TO_CARRY(), &g__TRIP())) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g_GENERIC_FOOD()) { let mut food: Py = __item.clone(); __out.push(_same(&[word.clone(), food.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__ANY_DISH();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.occasions".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "BRING_ASKS" => Some(g_BRING_ASKS()),
            "BRING_CARRY_WORDS" => Some(g_BRING_CARRY_WORDS()),
            "BRING_DISH" => Some(g_BRING_DISH()),
            "BRING_GUESTS" => Some(g_BRING_GUESTS()),
            "BRING_HOST_NAMES" => Some(g_BRING_HOST_NAMES()),
            "BRING_NOT_FOOD" => Some(g_BRING_NOT_FOOD()),
            "BRING_WORDS" => Some(g_BRING_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "GENERIC_FOOD" => Some(g_GENERIC_FOOD()),
            "OCCASION_NAMES" => Some(g_OCCASION_NAMES()),
            "_ANY_DISH" => Some(g__ANY_DISH()),
            "_COOKING_ASKS" => Some(g__COOKING_ASKS()),
            "_HOST_OWN" => Some(g__HOST_OWN()),
            "_MAKE_ASKS" => Some(g__MAKE_ASKS()),
            "_NOT_A_DISH" => Some(g__NOT_A_DISH()),
            "_NOT_A_DISH_TO_CARRY" => Some(g__NOT_A_DISH_TO_CARRY()),
            "_SPREAD_WORDS" => Some(g__SPREAD_WORDS()),
            "_TRIP" => Some(g__TRIP()),
            "_same" => Some(py_func("_same", |args: &[Py]| _same(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            "crosses_a_border" => Some(g_crosses_a_border()),
            "entry" => Some(g_entry()),
            "what_to_bring" => Some(py_func("what_to_bring", |args: &[Py]| what_to_bring(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "BRING_ASKS" => s_BRING_ASKS(v),
            "BRING_CARRY_WORDS" => s_BRING_CARRY_WORDS(v),
            "BRING_DISH" => s_BRING_DISH(v),
            "BRING_GUESTS" => s_BRING_GUESTS(v),
            "BRING_HOST_NAMES" => s_BRING_HOST_NAMES(v),
            "BRING_NOT_FOOD" => s_BRING_NOT_FOOD(v),
            "BRING_WORDS" => s_BRING_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "GENERIC_FOOD" => s_GENERIC_FOOD(v),
            "OCCASION_NAMES" => s_OCCASION_NAMES(v),
            "_ANY_DISH" => s__ANY_DISH(v),
            "_COOKING_ASKS" => s__COOKING_ASKS(v),
            "_HOST_OWN" => s__HOST_OWN(v),
            "_MAKE_ASKS" => s__MAKE_ASKS(v),
            "_NOT_A_DISH" => s__NOT_A_DISH(v),
            "_NOT_A_DISH_TO_CARRY" => s__NOT_A_DISH_TO_CARRY(v),
            "_SPREAD_WORDS" => s__SPREAD_WORDS(v),
            "_TRIP" => s__TRIP(v),
            "crosses_a_border" => s_crosses_a_border(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.occasions", module_obj);
    pyrt::xrefs::reg_fn("knowledge.occasions", "_same", py_func("_same", |args: &[Py]| _same(args)));
    pyrt::xrefs::reg_fn("knowledge.occasions", "_words_of", py_func("_words_of", |args: &[Py]| _words_of(args)));
    pyrt::xrefs::reg_fn("knowledge.occasions", "what_to_bring", py_func("what_to_bring", |args: &[Py]| what_to_bring(args)));
    pyrt::xrefs::reg_access("knowledge.occasions", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_ASKS", g_BRING_ASKS, s_BRING_ASKS);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_CARRY_WORDS", g_BRING_CARRY_WORDS, s_BRING_CARRY_WORDS);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_DISH", g_BRING_DISH, s_BRING_DISH);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_GUESTS", g_BRING_GUESTS, s_BRING_GUESTS);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_HOST_NAMES", g_BRING_HOST_NAMES, s_BRING_HOST_NAMES);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_NOT_FOOD", g_BRING_NOT_FOOD, s_BRING_NOT_FOOD);
    pyrt::xrefs::reg_access("knowledge.occasions", "BRING_WORDS", g_BRING_WORDS, s_BRING_WORDS);
    pyrt::xrefs::reg_access("knowledge.occasions", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.occasions", "GENERIC_FOOD", g_GENERIC_FOOD, s_GENERIC_FOOD);
    pyrt::xrefs::reg_access("knowledge.occasions", "OCCASION_NAMES", g_OCCASION_NAMES, s_OCCASION_NAMES);
    pyrt::xrefs::reg_access("knowledge.occasions", "_ANY_DISH", g__ANY_DISH, s__ANY_DISH);
    pyrt::xrefs::reg_access("knowledge.occasions", "_COOKING_ASKS", g__COOKING_ASKS, s__COOKING_ASKS);
    pyrt::xrefs::reg_access("knowledge.occasions", "_HOST_OWN", g__HOST_OWN, s__HOST_OWN);
    pyrt::xrefs::reg_access("knowledge.occasions", "_MAKE_ASKS", g__MAKE_ASKS, s__MAKE_ASKS);
    pyrt::xrefs::reg_access("knowledge.occasions", "_NOT_A_DISH", g__NOT_A_DISH, s__NOT_A_DISH);
    pyrt::xrefs::reg_access("knowledge.occasions", "_NOT_A_DISH_TO_CARRY", g__NOT_A_DISH_TO_CARRY, s__NOT_A_DISH_TO_CARRY);
    pyrt::xrefs::reg_access("knowledge.occasions", "_SPREAD_WORDS", g__SPREAD_WORDS, s__SPREAD_WORDS);
    pyrt::xrefs::reg_access("knowledge.occasions", "_TRIP", g__TRIP, s__TRIP);
    pyrt::xrefs::reg_access("knowledge.occasions", "crosses_a_border", g_crosses_a_border, s_crosses_a_border);
    pyrt::xrefs::reg_access("knowledge.occasions", "entry", g_entry, s_entry);
}
