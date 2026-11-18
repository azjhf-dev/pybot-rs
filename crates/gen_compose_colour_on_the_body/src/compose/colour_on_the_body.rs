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


thread_local! { static G_ALSO_A_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLUSH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLUSH_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PATCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PATCH_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEWHERE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.colour_on_the_body: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ALSO_A_SYMPTOM() -> Py { ensure_init(); G_ALSO_A_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALSO_A_SYMPTOM(v: Py) { ensure_init(); G_ALSO_A_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BURN() -> Py { ensure_init(); G_A_BURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BURN(v: Py) { ensure_init(); G_A_BURN.with(|c| *c.borrow_mut() = v); }
pub fn g_BLUSH() -> Py { ensure_init(); G_BLUSH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLUSH(v: Py) { ensure_init(); G_BLUSH.with(|c| *c.borrow_mut() = v); }
pub fn g_BLUSH_POOL() -> Py { ensure_init(); G_BLUSH_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLUSH_POOL(v: Py) { ensure_init(); G_BLUSH_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PARTS() -> Py { ensure_init(); G_BODY_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PARTS(v: Py) { ensure_init(); G_BODY_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_COLOUR() -> Py { ensure_init(); G_COLOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLOUR(v: Py) { ensure_init(); G_COLOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_INSIDE() -> Py { ensure_init(); G_INSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSIDE(v: Py) { ensure_init(); G_INSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PATCH() -> Py { ensure_init(); G_NOT_A_PATCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PATCH(v: Py) { ensure_init(); G_NOT_A_PATCH.with(|c| *c.borrow_mut() = v); }
pub fn g_PATCH_POOL() -> Py { ensure_init(); G_PATCH_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PATCH_POOL(v: Py) { ensure_init(); G_PATCH_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_PETS() -> Py { ensure_init(); G_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETS(v: Py) { ensure_init(); G_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); G_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); G_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEWHERE_ELSE() -> Py { ensure_init(); G_SOMEWHERE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEWHERE_ELSE(v: Py) { ensure_init(); G_SOMEWHERE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A colour on a part of the person's own body, and a blush, are answered rather than filed.\n\n\"my back is red\", \"my neck is blotchy\", \"my face has gone bright red\" are a person showing you\nsomething. Every reading around them wants a shape of its own - a pain, an itch, a rash, a body\nword the knowledge base knows - and a colour on its own is none of those, so the line fell to the\nfloor (\"Got it. Is there anything you'd like a hand with?\", \"I'm listening — say more if it\nhelps.\"), which says nothing about the patch just mentioned. Some were worse than the floor: the\n-ed catch-all read \"red\" as a past tense and answered \"my arm is red\" as an account of a nice day,\nand \"my ears went red\" reached the laundry card.\n\nTwo readings, one module:\n\n* a patch of colour on their own body gets what a patch like that usually is (contact or heat - a\n  soap, a plant, sweat) and the one question that settles it: how long, and does it itch. Nothing\n  is named as a diagnosis and no fact is invented; a pharmacist looks at the ones that spread or\n  hurt.\n* a face, cheek, neck or ear that goes red *in a moment* is a blush, not a patch. The advice for a\n  patch - keep soap off it, let it settle - is about the wrong thing entirely for somebody telling\n  you they go red when they talk to people, so it gets its own short answer and one question.\n\nIt stands down for a colour that is not on a body (the wash, a carpet, the sky), somebody else's\nbody (a child, a partner, the dog), a symptom word beside the colour (\"red and itchy\" has its own\nquestion, and a red patch that is hot or spreading is the one to get looked at today), the inside\nof a mouth or an eye, and a colour that already has a reading of its own (a yellow skin is\njaundice, a black nail is blood under it, a purple patch is a bruise).\n\nNothing else is claimed: `reply_for` returns None on every line this cannot read, so the line keeps\nwhatever answer it had.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.COLOUR"); }
    s_COLOUR(stdlib_call("re", "compile", &[py_str("\\b(?:red|reddish|pink|pinkish|blotchy|blotches|flushed)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.BLUSH"); }
    s_BLUSH(stdlib_call("re", "compile", &[py_str("\\b(?:blush\\w*|(?:face|cheeks?|neck|ears?)\\b[^.!?]{0,24}?\\b(?:goes?|going|went|gone|turns?|turning|turned|gets?|getting|come up|came up)\\s+(?:(?:bright|very|all|so|quite|a bit|really|dead|properly)\\s+)?red\\b|\\b(?:red|redder|redden\\w*)\\b[^.!?]{0,30}?when(?:ever)? i (?:talk|speak|meet) to?\\s?people)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("head"), py_str("face"), py_str("neck"), py_str("cheek"), py_str("cheeks"), py_str("forehead"), py_str("scalp"), py_str("ear"), py_str("ears"), py_str("nose"), py_str("chin"), py_str("jaw"), py_str("lip"), py_str("lips"), py_str("eyelid"), py_str("eyelids"), py_str("shoulder"), py_str("shoulders"), py_str("arm"), py_str("arms"), py_str("elbow"), py_str("elbows"), py_str("forearm"), py_str("forearms"), py_str("wrist"), py_str("wrists"), py_str("hand"), py_str("hands"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("chest"), py_str("breast"), py_str("breasts"), py_str("back"), py_str("spine"), py_str("tummy"), py_str("belly"), py_str("waist"), py_str("hip"), py_str("hips"), py_str("thigh"), py_str("thighs"), py_str("knee"), py_str("knees"), py_str("shin"), py_str("shins"), py_str("calf"), py_str("calves"), py_str("ankle"), py_str("ankles"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("heel"), py_str("heels"), py_str("skin"), py_str("torso"), py_str("underarm"), py_str("armpit"), py_str("armpits"), py_str("bum"), py_str("bottom"), py_str("crotch"), py_str("groin"), py_str("navel"), py_str("collarbone"), py_str("knuckle"), py_str("knuckles"), py_str("bottom")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.BODY_PARTS"); }
    s_BODY_PARTS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("itch"), py_str("itchy"), py_str("itches"), py_str("itching"), py_str("sting"), py_str("stings"), py_str("stinging"), py_str("burning"), py_str("tingling"), py_str("throbbing"), py_str("weeping"), py_str("crusty"), py_str("flaky"), py_str("scaly"), py_str("peeling"), py_str("swollen"), py_str("swelling"), py_str("warm"), py_str("hot"), py_str("feverish"), py_str("fever"), py_str("temperature"), py_str("shivery"), py_str("shivering"), py_str("spreading"), py_str("spread"), py_str("painful"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("pain"), py_str("tender"), py_str("bleeding"), py_str("bled"), py_str("blood"), py_str("ooze"), py_str("oozing"), py_str("pus"), py_str("puffy"), py_str("puffiness"), py_str("puffed"), py_str("raw"), py_str("shaved"), py_str("shaving"), py_str("razor"), py_str("numb"), py_str("numbness"), py_str("white"), py_str("blue"), py_str("tingly"), py_str("pins"), py_str("streak"), py_str("streaks"), py_str("track"), py_str("tracks")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.ALSO_A_SYMPTOM"); }
    s_ALSO_A_SYMPTOM(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("singed"), py_str("singe"), py_str("singeing"), py_str("boiling"), py_str("boiled"), py_str("scald"), py_str("scalds"), py_str("scalded"), py_str("steam"), py_str("steamed"), py_str("bleach"), py_str("bleached"), py_str("chemical"), py_str("acid"), py_str("oven"), py_str("hob"), py_str("iron"), py_str("ironed"), py_str("kettle"), py_str("frying"), py_str("oil"), py_str("bonfire"), py_str("barbecue"), py_str("bbq"), py_str("fire"), py_str("flare"), py_str("sparkler")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.A_BURN"); }
    s_A_BURN(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("throat"), py_str("tonsil"), py_str("tonsils"), py_str("gum"), py_str("gums"), py_str("mouth"), py_str("tongue"), py_str("palate"), py_str("sinus"), py_str("stomach"), py_str("gullet"), py_str("uvula"), py_str("eye"), py_str("eyes"), py_str("eyeball"), py_str("eyesight"), py_str("vision")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.INSIDE"); }
    s_INSIDE(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("wine"), py_str("sock"), py_str("socks"), py_str("whites"), py_str("shirt"), py_str("shirts"), py_str("sheet"), py_str("sheets"), py_str("towel"), py_str("towels"), py_str("carpet"), py_str("rug"), py_str("stain"), py_str("stains"), py_str("wash"), py_str("washing"), py_str("laundry"), py_str("dye"), py_str("dyed"), py_str("paint"), py_str("painted"), py_str("ink"), py_str("pen"), py_str("marker"), py_str("fabric"), py_str("curtain"), py_str("curtains"), py_str("bedding"), py_str("sofa"), py_str("tablecloth"), py_str("wall"), py_str("walls"), py_str("urine"), py_str("pee"), py_str("dress"), py_str("jumper"), py_str("blouse"), py_str("shoe"), py_str("shoes"), py_str("bag"), py_str("phone"), py_str("screen"), py_str("light"), py_str("lights"), py_str("sky"), py_str("sunset"), py_str("car"), py_str("door"), py_str("window"), py_str("leaves"), py_str("tree"), py_str("flower"), py_str("flowers"), py_str("candle"), py_str("flag"), py_str("light"), py_str("traffic")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.SOMEWHERE_ELSE"); }
    s_SOMEWHERE_ELSE(call_builtin("frozenset", &[__data5()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("his"), py_str("hers"), py_str("they"), py_str("them"), py_str("their"), py_str("mum"), py_str("dad"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("boy"), py_str("girl"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("nan"), py_str("gran"), py_str("friend"), py_str("mate"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("mother"), py_str("mom"), py_str("father"), py_str("sister"), py_str("brother"), py_str("aunt"), py_str("uncle"), py_str("niece"), py_str("nephew"), py_str("cousin"), py_str("grandma"), py_str("grandad"), py_str("granny"), py_str("stepson"), py_str("stepdaughter"), py_str("flatmate"), py_str("housemate"), py_str("colleague"), py_str("childminder"), py_str("babysitter"), py_str("siblings"), py_str("sibling"), py_str("pup"), py_str("rabbit"), py_str("hamster")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.SOMEONE_ELSE"); }
    s_SOMEONE_ELSE(call_builtin("frozenset", &[__data6()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("rabbit"), py_str("hamster"), py_str("pup"), py_str("parrot"), py_str("budgie"), py_str("guinea"), py_str("horse"), py_str("pony")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.PETS"); }
    s_PETS(call_builtin("frozenset", &[__data7()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("mole"), py_str("moles"), py_str("bite"), py_str("bites"), py_str("bitten"), py_str("burn"), py_str("burns"), py_str("burnt"), py_str("scald"), py_str("scalded"), py_str("sunburn"), py_str("sunburnt"), py_str("wound"), py_str("wounds"), py_str("cut"), py_str("cuts"), py_str("graze"), py_str("grazed"), py_str("scar"), py_str("scars"), py_str("bruise"), py_str("bruises"), py_str("bruised"), py_str("lump"), py_str("lumps"), py_str("spot"), py_str("spots"), py_str("pimple"), py_str("pimples"), py_str("hives"), py_str("tattoo"), py_str("tattoos"), py_str("blister"), py_str("blisters"), py_str("cyst"), py_str("abscess"), py_str("sore"), py_str("sores"), py_str("ulcer"), py_str("ulcers"), py_str("stitches"), py_str("dressing"), py_str("bandage"), py_str("cast"), py_str("plaster"), py_str("cream"), py_str("ointment")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.NOT_A_PATCH"); }
    s_NOT_A_PATCH(call_builtin("frozenset", &[__data8()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.PATCH_POOL"); }
    s_PATCH_POOL(py_tuple(vec![py_str("A patch that comes up like that is usually contact or heat - a new soap or washing powder, a plant, sweat in a warm room - and it fades in a few days without much doing. Keep perfumed things off it, don't scrub at it, and let a pharmacist look if it spreads or starts to hurt. If there is a fever with it, or the patch stays red under a glass pressed on it, get seen today. How long has it been there, and does it itch?"), py_str("Redness like that is usually contact or heat and it settles on its own in a few days. Plain water rather than soap, leave it uncovered, and show a pharmacist if it is no better by the weekend or it is spreading. A fever with it, or red that does not blanch under a glass, is a same-day call rather than a wait. When did you first notice it?"), py_str("That sounds like contact or heat: a soap, a plant, a fabric, sweat under a coat. Cool water and no cream until you know what it is, and a pharmacist if it is still there in a few days or it starts to hurt. With a fever, or if it stays red where you press a glass on it, get seen the same day. Is it in one place or spreading?"), py_str("Patches like that are usually contact or heat and they calm down in a few days. Wash it with plain water, keep anything perfumed off it, and have a pharmacist look if it spreads or gets sore. If you also feel feverish or shivery, or the colour stays put under a glass, that is a today job. Where is it, and has it changed since you first saw it?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.BLUSH_POOL"); }
    s_BLUSH_POOL(py_tuple(vec![py_str("That is a blush rather than anything wrong: the vessels in the face open up for a moment and there is nothing to treat. It eases as you get used to the people in the room, and finding one friendly face to talk to helps more than trying to fight it. What sort of moment sets it off?"), py_str("Going red like that is a blush - your own body's alarm, and it fades in a minute or two. Most people read it as being interested rather than embarrassed. Is it situations with people you don't know, or does it happen with friends too?"), py_str("A blush is the face's own thermostat rather than a sign of anything wrong, and everybody has one. It helps to let it happen instead of fighting it - fighting it is what keeps it going. Does it bother you more at work or with people you know?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.colour_on_the_body.WORD"); }
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z']+")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _words(args: &[Py]) -> Py {
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
    return mcall(&g_WORD(), "findall", &[mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]);
    Py::None
}
pub fn _last_reply(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the bot said on the turn before, if the store has it.");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    }
    }
    return py_str("");
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
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut last: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("A wording for this line, never the whole of what the bot said last time.\n\n    The arbiter passes over a candidate that only repeats the bot's previous reply, and the line\n    after a patch is often another patch (\"my neck is blotchy\" then \"my chest is red\"): with one\n    wording chosen by the line alone the second turn would repeat the first and the arbiter would\n    hand the turn to whatever card the words matched - the wash card, for \"i go bright red when i\n    talk to people\". So the wording steps on when it would be the last one said again.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return py_str("");
    }
    start = op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&line.clone(), "lower", &[])) { let mut letter: Py = __item.clone(); __out.push(call_builtin("ord", &[letter.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)));
    last = _last_reply(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    said = geti(&pool.clone(), &op_modulo(&op_add(&start.clone(), &step.clone()), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&last.clone(), &mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]))) } }) {
    return said.clone();
    }
    }
    return geti(&pool.clone(), &start.clone());
    Py::None
}
pub fn _someone_elses(args: &[Py]) -> Py {
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
    let mut suffix: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says whose body it is and it is not the person typing.\n\n    \"my son's arm\", \"my sons arm\" and \"his arm\" are all somebody else. The plural is read by\n    dropping one trailing s, and only when what is left is a person word - \"his\" must not become\n    \"hi\" and slip through, which is how \"the skin under his belly is red\" was answered as the\n    person's own patch.\n    ");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SOMEONE_ELSE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PETS(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("'s"), py_str("'")]));
    for __item in __iter2 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[suffix.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_SOMEONE_ELSE(), &g_PETS()), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_SOMEONE_ELSE(), &g_PETS()), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _thread_names_a_pet(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few turns were about an animal, whose skin is not the person's.");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&mcall(&{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &Py::Int(4))) {
    break;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_PETS())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer to a colour on the person's own body or a blush, or None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    words = call_builtin("set", &[_words(&[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_BLUSH(), "search", &[clean.clone()])) {
    if py_truthy(&{ let __b = mcall(&clean.clone(), "startswith", &[py_str("why")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("i"), py_str("my"), py_str("me"), py_str("im"), py_str("myself")])))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_SOMEWHERE_ELSE()); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_INSIDE()) } }) {
    return Py::None;
    }
    if py_truthy(&_someone_elses(&[words.clone()])) {
    return Py::None;
    }
    return _pick(&[g_BLUSH_POOL(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&mcall(&g_COLOUR(), "search", &[clean.clone()]), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_BODY_PARTS())))) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&words.clone(), &g_INSIDE())) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&words.clone(), &g_SOMEWHERE_ELSE())) {
    return Py::None;
    }
    if py_truthy(&_someone_elses(&[words.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_ALSO_A_SYMPTOM()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_NOT_A_PATCH()); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_A_BURN()) } } } }) {
    return Py::None;
    }
    if py_truthy(&_thread_names_a_pet(&[memory.clone()])) {
    return Py::None;
    }
    return _pick(&[g_PATCH_POOL(), clean.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.colour_on_the_body".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALSO_A_SYMPTOM" => Some(g_ALSO_A_SYMPTOM()),
            "A_BURN" => Some(g_A_BURN()),
            "BLUSH" => Some(g_BLUSH()),
            "BLUSH_POOL" => Some(g_BLUSH_POOL()),
            "BODY_PARTS" => Some(g_BODY_PARTS()),
            "COLOUR" => Some(g_COLOUR()),
            "INSIDE" => Some(g_INSIDE()),
            "NOT_A_PATCH" => Some(g_NOT_A_PATCH()),
            "PATCH_POOL" => Some(g_PATCH_POOL()),
            "PETS" => Some(g_PETS()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "SOMEWHERE_ELSE" => Some(g_SOMEWHERE_ELSE()),
            "WORD" => Some(g_WORD()),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_someone_elses" => Some(py_func("_someone_elses", |args: &[Py]| _someone_elses(args))),
            "_thread_names_a_pet" => Some(py_func("_thread_names_a_pet", |args: &[Py]| _thread_names_a_pet(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALSO_A_SYMPTOM" => s_ALSO_A_SYMPTOM(v),
            "A_BURN" => s_A_BURN(v),
            "BLUSH" => s_BLUSH(v),
            "BLUSH_POOL" => s_BLUSH_POOL(v),
            "BODY_PARTS" => s_BODY_PARTS(v),
            "COLOUR" => s_COLOUR(v),
            "INSIDE" => s_INSIDE(v),
            "NOT_A_PATCH" => s_NOT_A_PATCH(v),
            "PATCH_POOL" => s_PATCH_POOL(v),
            "PETS" => s_PETS(v),
            "SOMEONE_ELSE" => s_SOMEONE_ELSE(v),
            "SOMEWHERE_ELSE" => s_SOMEWHERE_ELSE(v),
            "WORD" => s_WORD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.colour_on_the_body", module_obj);
    pyrt::xrefs::reg_fn("compose.colour_on_the_body", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.colour_on_the_body", "_last_reply", py_func("_last_reply", |args: &[Py]| _last_reply(args)));
    pyrt::xrefs::reg_fn("compose.colour_on_the_body", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("compose.colour_on_the_body", "_someone_elses", py_func("_someone_elses", |args: &[Py]| _someone_elses(args)));
    pyrt::xrefs::reg_fn("compose.colour_on_the_body", "_thread_names_a_pet", py_func("_thread_names_a_pet", |args: &[Py]| _thread_names_a_pet(args)));
    pyrt::xrefs::reg_fn("compose.colour_on_the_body", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "ALSO_A_SYMPTOM", g_ALSO_A_SYMPTOM, s_ALSO_A_SYMPTOM);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "A_BURN", g_A_BURN, s_A_BURN);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "BLUSH", g_BLUSH, s_BLUSH);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "BLUSH_POOL", g_BLUSH_POOL, s_BLUSH_POOL);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "BODY_PARTS", g_BODY_PARTS, s_BODY_PARTS);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "COLOUR", g_COLOUR, s_COLOUR);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "INSIDE", g_INSIDE, s_INSIDE);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "NOT_A_PATCH", g_NOT_A_PATCH, s_NOT_A_PATCH);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "PATCH_POOL", g_PATCH_POOL, s_PATCH_POOL);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "PETS", g_PETS, s_PETS);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "SOMEONE_ELSE", g_SOMEONE_ELSE, s_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "SOMEWHERE_ELSE", g_SOMEWHERE_ELSE, s_SOMEWHERE_ELSE);
    pyrt::xrefs::reg_access("compose.colour_on_the_body", "WORD", g_WORD, s_WORD);
}
