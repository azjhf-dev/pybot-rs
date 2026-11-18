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


thread_local! { static G__BAKE_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAKE_FAULT_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAKE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.bake_faults: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g__BAKE_FAULT() -> Py { ensure_init(); G__BAKE_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKE_FAULT(v: Py) { ensure_init(); G__BAKE_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__BAKE_FAULT_FAMILIES() -> Py { ensure_init(); G__BAKE_FAULT_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKE_FAULT_FAMILIES(v: Py) { ensure_init(); G__BAKE_FAULT_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g__BAKE_THING() -> Py { ensure_init(); G__BAKE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKE_THING(v: Py) { ensure_init(); G__BAKE_THING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The baking faults, kept apart from the rest of the cooking cards.\n\nA baking fault is the fault *and* the bake it happened in: a crack in a cheesecake is not a crack\nin a macaron, a meringue that weeps is not a pastry that shrinks. `cooking.py` keeps the one entry\n(`bake-fault`) that reaches the dispatcher below; the tables live here because that file sits on\nthe uploader's 1 MiB ceiling, and a card nobody can upload helps nobody.\n\nNothing changed in the move: the same keys, the same paragraph per bake, and `_same` is the same\nloose match the cooking cards use (eggs/egg, meringues/meringue).\n");
    fn __data1() -> Py { py_tuple(vec![py_str("cheesecake"), py_str("cheesecakes"), py_str("ganache"), py_str("meringue"), py_str("meringues"), py_str("macaron"), py_str("macarons"), py_str("choux"), py_str("profiteroles"), py_str("profiterole"), py_str("eclairs"), py_str("eclair"), py_str("pastry"), py_str("pastry case"), py_str("tart"), py_str("tarts"), py_str("pie crust"), py_str("pie shell"), py_str("brownie"), py_str("brownies"), py_str("cake mixture"), py_str("cake batter"), py_str("sponge mixture")]) }
    s__BAKE_THING(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("crack"), py_str("cracked"), py_str("cracks"), py_str("cracking"), py_str("split"), py_str("splits"), py_str("curdled"), py_str("curdles"), py_str("curdling"), py_str("weeping"), py_str("wept"), py_str("weeps"), py_str("beading"), py_str("beads"), py_str("shrank"), py_str("shrunk"), py_str("shrinking"), py_str("didnt puff"), py_str("didn't puff"), py_str("did not puff"), py_str("didnt rise"), py_str("didn't rise"), py_str("did not rise"), py_str("no feet"), py_str("hollow"), py_str("didnt set"), py_str("didn't set"), py_str("not set"), py_str("wont set"), py_str("won't set"), py_str("sank"), py_str("sunken"), py_str("watery"), py_str("grainy"), py_str("gritty"), py_str("soft"), py_str("softened"), py_str("softening"), py_str("went soft"), py_str("goes soft"), py_str("gone soft"), py_str("soggy"), py_str("sticky"), py_str("sticking to"), py_str("chewy"), py_str("floppy"), py_str("limp"), py_str("collapsed")]) }
    s__BAKE_FAULT(__data2());
    s__BAKE_FAULT_FAMILIES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("dry"), py_str("dried out"), py_str("overcooked"), py_str("over-cooked"), py_str("hard"), py_str("tough")]), py_str("It is nearly always the time in the oven, or flour scooped rather than weighed"), py_str("take it out 5 minutes before the recipe says, weigh the flour instead of scooping it, and brush the top with milk or syrup while it is still warm"), py_str("serve it with custard, cream or a spoon of jam, or crumble it into a trifle")]), py_tuple(vec![py_tuple(vec![py_str("crumbly"), py_str("crumbs"), py_str("crumbles"), py_str("falling apart"), py_str("fell apart"), py_str("sandy"), py_str("dry and crumbly")]), py_str("Too little fat or egg to hold it together, or it was cut while it was still warm"), py_str("weigh the butter and sugar, add an extra yolk, and leave it 10 minutes in the tin before turning it out"), py_str("eat it in a bowl with fruit and cream, or use it as the base of a trifle")]), py_tuple(vec![py_tuple(vec![py_str("sank"), py_str("sunk"), py_str("sunken"), py_str("sinking"), py_str("sink"), py_str("collapsed")]), py_str("It is underbaked, or the oven door was opened in the first 20 minutes, or there is too much raising agent in it, and all three make a bake rise fast and then drop"), py_str("bake it at 180C (160C fan) without opening the door, fill the tin two-thirds full, and check it with a skewer before it comes out"), py_str("fill the dip with jam and cream, or serve it warm as a pudding")]), py_tuple(vec![py_tuple(vec![py_str("flat"), py_str("didnt rise"), py_str("didn't rise"), py_str("not risen"), py_str("no rise")]), py_str("The baking powder is past its date, the flour was beaten in too hard, or the oven was not up to heat"), py_str("use a teaspoon of baking powder to 150g of plain flour, fold the flour in with a spoon, and let the oven come fully up to heat first"), py_str("split and fill it, tear it into a trifle, or serve it warm with custard")]), py_tuple(vec![py_tuple(vec![py_str("gummy"), py_str("clammy"), py_str("wet in the middle"), py_str("soggy in the middle"), py_str("stodgy"), py_str("doughy"), py_str("raw in the middle"), py_str("soggy"), py_str("wet inside"), py_str("squidgy"), py_str("not cooked in the middle")]), py_str("It needed longer, not a hotter oven: the outside set while the middle was still wet, and a mixture weighed by eye goes in wetter than the recipe meant"), py_str("give it 5 to 10 more minutes at the same heat with foil over the top, weigh the milk, butter and flour, and use a skewer in the centre not at the edge"), py_str("back in the oven at 160C for 10 minutes with foil over the top, or eat it warm as a pudding with custard")]), py_tuple(vec![py_tuple(vec![py_str("pale"), py_str("palest"), py_str("no colour"), py_str("white on top"), py_str("pale on top"), py_str("blond"), py_str("not brown")]), py_str("It is short of heat or short of sugar: a pale bake was on too low a shelf or the oven was under its temperature, and a recipe cut down on sugar browns slowly"), py_str("bake it one shelf up at the heat the recipe says, and check the oven with a thermometer - an oven that reads 160C when it says 180C gives a pale bake every time"), py_str("it is still good to eat, and a minute under the grill browns the top of a sponge")]), py_tuple(vec![py_tuple(vec![py_str("bicarbonate"), py_str("bicarb"), py_str("soda"), py_str("soapy"), py_str("tastes of soap"), py_str("tastes of baking soda"), py_str("tastes of bicarbonate"), py_str("chemical taste"), py_str("metallic")]), py_str("There is too much bicarbonate in it, or nothing acid for it to react with, and both leave that soapy taste behind"), py_str("use a level teaspoon to 150g of plain flour, keep it for recipes with buttermilk, yoghurt, lemon or cocoa in them, and use baking powder when nothing in the mix is acid"), py_str("a soapy taste is not dangerous - a spoon of jam or a glaze covers most of it")]), py_tuple(vec![py_tuple(vec![py_str("stuck"), py_str("sticking"), py_str("sticks"), py_str("broke"), py_str("broken")]), py_str("The tin was not greased all over, or it was turned out while it was still hot"), py_str("grease the tin and cut a strip of baking paper across the base and up two sides, then run a knife round the edge and wait 10 minutes before turning it out"), py_str("glue the broken pieces back together with jam or buttercream and nobody will know")])]));
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
    let _ = py_str("One word matches another loosely: eggs/egg, potatoes/potato, minced/mince.");
    return { let __b = call_builtin("bool", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&word.clone(), &wanted.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[wanted.clone()]) } } } } } };
    Py::None
}
pub fn _bake_fault(args: &[Py]) -> Py {
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
    let mut has: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says: Py = Py::None;
    let mut weeping: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Eight baking faults that came back as a shrug, each with its own cause and its own fix.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    said = mcall(&py_str(" "), "join", &[words.clone()]);
pub fn __nested_has_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut weeping: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut wanted: Py = py_list(__pos.get(8..).unwrap_or(&[]).to_vec());
    let mut want: Py = Py::None;
    let mut wanted: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&wanted.clone()) { let mut want: Py = __item.clone(); __out.push(_same(&[word.clone(), want.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    has = {{ let has = has.clone(); let memory = memory.clone(); let said = said.clone(); let says = says.clone(); let understood = understood.clone(); let weeping = weeping.clone(); let word = word.clone(); let words = words.clone(); py_func("has", move |args: &[Py]| { let mut full = vec![has.clone(), memory.clone(), said.clone(), says.clone(), understood.clone(), weeping.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_has_0(&full) }) }};
pub fn __nested_says_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut weeping: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut phrases: Py = py_list(__pos.get(8..).unwrap_or(&[]).to_vec());
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    says = {{ let has = has.clone(); let memory = memory.clone(); let said = said.clone(); let says = says.clone(); let understood = understood.clone(); let weeping = weeping.clone(); let word = word.clone(); let words = words.clone(); py_func("says", move |args: &[Py]| { let mut full = vec![has.clone(), memory.clone(), said.clone(), says.clone(), understood.clone(), weeping.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_says_1(&full) }) }};
    if py_truthy(&py_call(&has, &[py_str("cheesecake"), py_str("cheesecakes")])) {
    if py_truthy(&py_call(&has, &[py_str("grainy"), py_str("gritty"), py_str("curdled")])) {
    return py_str("A grainy cheesecake has curdled: the oven was too hot, or it was beaten too long after the eggs went in. Bake it at 150C (130C fan) in a water bath, mix only until each thing is just in, and let it cool in the oven with the door ajar so it sets slowly. It is still good to eat - a grainy one is a texture, not a fault to throw away.");
    }
    if py_truthy(&py_call(&has, &[py_str("watery"), py_str("soggy"), py_str("set")])) {
    return py_str("A watery cheesecake was underbaked or it went into the fridge too soon: it needs to set in the oven until only the middle wobbles, then cool in the tin an hour before it sees the cold. Bake it in a water bath at 150C (130C fan), because the steam is what makes it dense and creamy rather than grainy, and drain the tin well before it is chilled.");
    }
    return py_str("A cheesecake cracks when it is baked too hot or cooled too fast: bake it at 150C (130C fan) in a water bath, and when only the middle still wobbles turn the oven off and leave it in there with the door ajar for an hour. A crack is cosmetic - a cold cheesecake and a spoon of sour cream over the top covers it - and the next one will not crack if it cools slowly.");
    }
    if py_truthy(&py_call(&has, &[py_str("ganache")])) {
    return py_str("A split ganache got too hot, or cold cream went into warm chocolate: warm it gently over barely simmering water, off the heat, and stir in a tablespoon of warm milk or cream until it comes back together. Next time pour hot cream over chopped chocolate, wait a minute, then stir from the middle outwards, and keep every spoon and bowl dry.");
    }
    if py_truthy(&py_call(&has, &[py_str("meringue"), py_str("meringues")])) {
    weeping = py_str("A meringue weeps when the sugar was not fully dissolved or it was underbaked: rub a little between your fingers and it should feel smooth, not gritty, before it goes in. Bake it low and slow, 100C (80C fan) for an hour and a half, then leave it in the oven with the door ajar to cool, and make it on a dry day.");
    if py_truthy(&py_call(&has, &[py_str("weeping"), py_str("weeped"), py_str("weeps"), py_str("wept"), py_str("beading"), py_str("beaded"), py_str("beads"), py_str("gritty"), py_str("grainy")])) {
    return weeping.clone();
    }
    if py_truthy(&py_call(&has, &[py_str("soft"), py_str("softened"), py_str("soggy"), py_str("sticky"), py_str("chewy"), py_str("floppy"), py_str("limp"), py_str("collapsed")])) {
    return py_str("A meringue goes soft when it has not dried out enough, or it was stored where the air is damp: bake it at 100C (80C fan) for an hour and a half and lift one off the paper to check it comes away clean, then cool it in the oven with the door ajar and keep it in an airtight tin - never the fridge, which is what makes it collapse. A sticky one wants 15 minutes more at the same heat, and a chewy one wants the sugar whisked in more slowly and a longer, slower bake.");
    }
    return weeping.clone();
    }
    if py_truthy(&py_call(&has, &[py_str("macaron"), py_str("macarons")])) {
    if py_truthy(&py_call(&has, &[py_str("hollow")])) {
    return py_str("A hollow macaron was underbaked, or the whites were whipped too stiff and the shell set before the inside could come up: whip to soft to medium peaks, fold the batter until it ribbons and settles back in about 20 seconds, and bake at 150C (130C fan) for a minute or two longer, resting the shells only until a skin forms - much longer than that and they go hollow instead of getting feet.");
    }
    if py_truthy(&py_call(&has, &[py_str("soft"), py_str("softened"), py_str("softening"), py_str("soggy"), py_str("sticky"), py_str("chewy"), py_str("limp"), py_str("floppy"), py_str("collapsed"), py_str("sank")])) {
    return py_str("A macaron that goes soft a day after it is filled is the filling's moisture working into the shell, which is what a macaron is meant to do: keep them in an airtight box and eat them within two or three days, and take them out of the fridge 20 minutes before you do, because cold makes a shell chewy. One that comes out of the oven soft was underbaked or the shells were not left long enough to form a skin: rest them 30 to 60 minutes until they are dry to the touch, then bake at 150C (130C fan) for 2 minutes longer than you did and check one at the back of the tray.");
    }
    return py_str("Macarons crack when the oven is too hot or the piped shells were not rested: tap the tray, leave them 30 to 60 minutes until a skin forms, then bake at 150C (130C fan) with the door propped a crack. Fold the batter until it ribbons slowly off the spatula - overmixed and they spread, undermixed and they crack - and a hollow one usually wants 5C less and two minutes more.");
    }
    if py_truthy(&py_call(&has, &[py_str("choux"), py_str("profiteroles"), py_str("profiterole"), py_str("eclairs"), py_str("eclair")])) {
    return py_str("Choux does not puff when the dough is too wet or the oven door is opened: dry the pan over the heat for a minute once the flour is in, beat in the eggs until it just falls off the spoon in a V, and keep the door shut for the first 20 minutes at 200C (180C fan). A wet dough steams instead of setting, and a cool oven lets it slide.");
    }
    if py_truthy(&py_call(&has, &[py_str("brownie"), py_str("brownies")])) {
    return py_str("Brownies that will not set are underbaked: 20 to 25 minutes at 180C (160C fan), until a skewer comes out with a few moist crumbs and no wet batter, then cool them completely in the tin - an hour in the fridge if you are in a hurry - because they firm up as they cool. A runny middle is warm butter and sugar, so give them 5 more minutes rather than more flour.");
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("pastry"), py_str("tart"), py_str("tarts"), py_str("pie")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("pie crust"), py_str("pie shell"), py_str("pastry case")]) } }) {
    if py_truthy(&py_call(&has, &[py_str("soft"), py_str("softened"), py_str("soggy"), py_str("sticky"), py_str("greasy"), py_str("wet"), py_str("limp")])) {
    return py_str("Soft pastry is too much fat or water, or the fat got warm: use about half the weight of butter to flour, cut it in cold until it looks like breadcrumbs, and add iced water a spoon at a time until it only just comes together. Handle it as little as you can, rest it 20 minutes in the fridge, and roll it on a floured surface rather than pressing it - warm butter is what makes it limp, so cool the bowl and your hands if the kitchen is hot.");
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("crack"), py_str("cracked"), py_str("cracks")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&has, &[py_str("shrank"), py_str("shrunk"), py_str("shrinking")]))) } }) {
    return py_str("");
    }
    return py_str("Pastry shrinks when the gluten is worked too much or it is stretched into the tin: rest it 20 minutes in the fridge before rolling, ease it in rather than pulling it, and let it hang over the edge instead of pressing it up the sides. Chill the lined tin 15 minutes before it bakes and it will hold its shape.");
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("cake mixture"), py_str("cake batter"), py_str("sponge mixture")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("cake mixture"), py_str("cake batter")]) } }) {
    return py_str("A curdled cake mixture is cold eggs going into warm butter too fast, and it comes back easily: beat in a tablespoon of the weighed flour, and stand the bowl in warm water for a minute. Next time use eggs at room temperature and add them a little at a time, beating well after each one, and it will not split.");
    }
    return py_str("");
    Py::None
}
pub fn _bake_always(args: &[Py]) -> Py {
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
    let mut _use: Py = Py::None;
    let mut both: Py = Py::None;
    let mut cause: Py = Py::None;
    let mut fix: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The cause and the next-time fix for the fault in \"everything i bake comes out ...\".");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]);
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    both = op_add(&op_add(&said.clone(), &py_str(" ")), &line.clone());
    let __iter1 = py_iter_or_raise(&g__BAKE_FAULT_FAMILIES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    words = __unpk2[0].clone();
    cause = __unpk2[1].clone();
    fix = __unpk2[2].clone();
    _use = __unpk2[3].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&both.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_modulo(&py_str("%s. Next time: %s."), &py_tuple(vec![cause.clone(), fix.clone()]));
    }
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.bake_faults".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "_BAKE_FAULT" => Some(g__BAKE_FAULT()),
            "_BAKE_FAULT_FAMILIES" => Some(g__BAKE_FAULT_FAMILIES()),
            "_BAKE_THING" => Some(g__BAKE_THING()),
            "_bake_always" => Some(py_func("_bake_always", |args: &[Py]| _bake_always(args))),
            "_bake_fault" => Some(py_func("_bake_fault", |args: &[Py]| _bake_fault(args))),
            "_same" => Some(py_func("_same", |args: &[Py]| _same(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "_BAKE_FAULT" => s__BAKE_FAULT(v),
            "_BAKE_FAULT_FAMILIES" => s__BAKE_FAULT_FAMILIES(v),
            "_BAKE_THING" => s__BAKE_THING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.bake_faults", module_obj);
    pyrt::xrefs::reg_fn("knowledge.bake_faults", "_same", py_func("_same", |args: &[Py]| _same(args)));
    pyrt::xrefs::reg_fn("knowledge.bake_faults", "_bake_fault", py_func("_bake_fault", |args: &[Py]| _bake_fault(args)));
    pyrt::xrefs::reg_fn("knowledge.bake_faults", "_bake_always", py_func("_bake_always", |args: &[Py]| _bake_always(args)));
    pyrt::xrefs::reg_access("knowledge.bake_faults", "_BAKE_FAULT", g__BAKE_FAULT, s__BAKE_FAULT);
    pyrt::xrefs::reg_access("knowledge.bake_faults", "_BAKE_FAULT_FAMILIES", g__BAKE_FAULT_FAMILIES, s__BAKE_FAULT_FAMILIES);
    pyrt::xrefs::reg_access("knowledge.bake_faults", "_BAKE_THING", g__BAKE_THING, s__BAKE_THING);
}
