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
thread_local! { static G_A_DIFFERENT_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_THE_MATERIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECORATED_SURFACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_WOODWORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ORDER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAINT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_DIFFERENT_JOB() -> Py { ensure_init(); G_A_DIFFERENT_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DIFFERENT_JOB(v: Py) { ensure_init(); G_A_DIFFERENT_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEAK() -> Py { ensure_init(); G_A_LEAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEAK(v: Py) { ensure_init(); G_A_LEAK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAIN() -> Py { ensure_init(); G_A_STAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAIN(v: Py) { ensure_init(); G_A_STAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_WRONG() -> Py { ensure_init(); G_GOING_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_WRONG(v: Py) { ensure_init(); G_GOING_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_PAINT() -> Py { ensure_init(); G_PAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAINT(v: Py) { ensure_init(); G_PAINT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_THE_MATERIAL() -> Py { ensure_init(); G__ASKS_THE_MATERIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_THE_MATERIAL(v: Py) { ensure_init(); G__ASKS_THE_MATERIAL.with(|c| *c.borrow_mut() = v); }
pub fn g__DECORATED_SURFACE() -> Py { ensure_init(); G__DECORATED_SURFACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECORATED_SURFACE(v: Py) { ensure_init(); G__DECORATED_SURFACE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_WOODWORK() -> Py { ensure_init(); G__NOT_WOODWORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_WOODWORK(v: Py) { ensure_init(); G__NOT_WOODWORK.with(|c| *c.borrow_mut() = v); }
pub fn g__ORDER_WORDS() -> Py { ensure_init(); G__ORDER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ORDER_WORDS(v: Py) { ensure_init(); G__ORDER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PAINT_WORD() -> Py { ensure_init(); G__PAINT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAINT_WORD(v: Py) { ensure_init(); G__PAINT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__STEPS() -> Py { ensure_init(); G__STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEPS(v: Py) { ensure_init(); G__STEPS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Decorating paint: the things that go wrong while a wall is being painted.\n\nSeven everyday lines about a brush and a roller, and six of them were answered about\nsomething else:\n\n  how do i stop paint dripping                  the acrylics hobby card (\"a starter set is under\n                                                twenty pounds\")\n  how do i stop paint running down the wall     most training injuries come from a jump in load\n  paint keeps dripping down the wall            the ceiling-leak card (\"a brown or yellow patch\n                                                on a ceiling is water\")\n  how do i stop the roller splattering          a search hedge\n  how do i stop paint getting on the skirting   the sand-it-and-mask-the-carpet card\n  which brush for gloss                         the key-the-old-gloss-first card\n  how do i thin gloss paint                     the acrylics hobby card again\n\nThe word \"paint\" is the whole problem: the art-and-hobbies row teaches a beginner what to buy,\nthe ceiling-leak row owns a patch on a wall, and the fitness row owns a word that is not in the\nline at all. A person with a brush in their hand asked how to stop a run, and the bot told them\nabout acrylics.\n\nSo every row here needs two things in the line: paint, or the tool it is being put on with, AND\nthe thing going wrong. `how do i paint a ceiling`, `how do i paint a door`, `how do i paint a\nwall without streaks`, `how do i paint a radiator`, `what paint for a bathroom`, `how long\nbetween coats of paint`, `how do i get paint off my hands`, `how do i get paint off a carpet` and\n`how do i clean a paintbrush` are all answered well already, and none of them names a fault, so\nthey keep their cards.\n\nA leak keeps its own: a patch of water coming through a ceiling or a wall is the damp answer's\nquestion, and \"paint\" in the line does not make it a decorating one.\n");
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    s_PAINT(py_tuple(vec![py_str("paint"), py_str("paints"), py_str("painted"), py_str("painting"), py_str("repaint"), py_str("repainting"), py_str("gloss"), py_str("glossing"), py_str("emulsion"), py_str("satin"), py_str("eggshell"), py_str("undercoat"), py_str("primer"), py_str("priming"), py_str("woodwork"), py_str("woodwork"), py_str("decorating"), py_str("decorate"), py_str("decorated"), py_str("roller"), py_str("rollers"), py_str("brush"), py_str("brushes"), py_str("paintbrush"), py_str("paintbrushes"), py_str("sleeve"), py_str("masking tape"), py_str("filler"), py_str("sander")]));
    s_GOING_WRONG(py_tuple(vec![py_str("drip"), py_str("drips"), py_str("dripping"), py_str("dripped"), py_str("run"), py_str("runs"), py_str("running"), py_str("ran"), py_str("sag"), py_str("sags"), py_str("sagging"), py_str("sagged"), py_str("curtain"), py_str("curtains"), py_str("slide"), py_str("sliding"), py_str("slid"), py_str("trickle"), py_str("trickles"), py_str("trickling"), py_str("patchy"), py_str("uneven"), py_str("transfer"), py_str("showing"), py_str("shows"), py_str("visible")]));
    s_A_LEAK(py_tuple(vec![py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("leaked"), py_str("damp"), py_str("damp patch"), py_str("damp patches"), py_str("water"), py_str("watermark"), py_str("water marks"), py_str("rain"), py_str("raining"), py_str("roof"), py_str("gutter"), py_str("gutters"), py_str("pipe"), py_str("pipes"), py_str("burst"), py_str("condensation"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("plaster"), py_str("render"), py_str("brick"), py_str("bricks"), py_str("brickwork"), py_str("ceiling tile"), py_str("flood"), py_str("flooded"), py_str("overflow"), py_str("tank"), py_str("radiator leak"), py_str("washing machine"), py_str("dishwasher")]));
    s_A_STAIN(py_tuple(vec![py_str("stain"), py_str("stains"), py_str("stained"), py_str("staining"), py_str("spilled"), py_str("spill"), py_str("spillage"), py_str("soaked"), py_str("soak"), py_str("scrub"), py_str("wipe off"), py_str("get it off"), py_str("get off"), py_str("remove"), py_str("removing"), py_str("clean off"), py_str("cleaning off"), py_str("off my hands"), py_str("off my skin"), py_str("out of my clothes"), py_str("out of a carpet"), py_str("off the floor")]));
    s_A_DIFFERENT_JOB(py_tuple(vec![py_str("hole"), py_str("holes"), py_str("crack"), py_str("cracks"), py_str("cracked"), py_str("skim"), py_str("skimming"), py_str("plastering"), py_str("plasterer"), py_str("fill a wall"), py_str("wallpaper"), py_str("wallpapering"), py_str("stripping"), py_str("strip the"), py_str("how much paint"), py_str("litre"), py_str("litres"), py_str("how many coats"), py_str("which colour"), py_str("what colour"), py_str("farrow"), py_str("dulux"), py_str("buy"), py_str("bought"), py_str("cost"), py_str("price"), py_str("cheap")]));
    s__PAINT_WORD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PAINT()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__DECORATED_SURFACE(stdlib_call("re", "compile", &[py_str("\\b(?:skirting|skirtings|skirting\\ board|skirting\\ boards|woodwork|door\\ ?frame|door\\ ?frames|architrave|architraves|picture\\ rail|banister|banisters)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__STEPS(py_dict(vec![(py_str("fill"), py_str("fill")), (py_str("filling"), py_str("fill")), (py_str("filled"), py_str("fill")), (py_str("filler"), py_str("fill")), (py_str("patch"), py_str("fill")), (py_str("patching"), py_str("fill")), (py_str("sand"), py_str("sand")), (py_str("sanding"), py_str("sand")), (py_str("sanded"), py_str("sand")), (py_str("sandpaper"), py_str("sand")), (py_str("smooth"), py_str("sand")), (py_str("smoothing"), py_str("sand")), (py_str("prime"), py_str("finish")), (py_str("priming"), py_str("finish")), (py_str("primer"), py_str("finish")), (py_str("undercoat"), py_str("finish")), (py_str("varnish"), py_str("finish")), (py_str("varnishing"), py_str("finish")), (py_str("varnished"), py_str("finish")), (py_str("oil"), py_str("finish")), (py_str("oiling"), py_str("finish")), (py_str("wax"), py_str("finish")), (py_str("waxing"), py_str("finish")), (py_str("stain"), py_str("finish")), (py_str("painting"), py_str("finish")), (py_str("paint"), py_str("finish")), (py_str("gloss"), py_str("finish")), (py_str("lacquer"), py_str("finish")), (py_str("coat"), py_str("finish")), (py_str("coats"), py_str("finish"))]));
    s__ORDER_WORDS(py_tuple(vec![py_str("first"), py_str("before"), py_str("after"), py_str("or"), py_str("between"), py_str("then")]));
    s__NOT_WOODWORK(py_tuple(vec![py_str("floor"), py_str("floors"), py_str("floorboard"), py_str("floorboards"), py_str("decking"), py_str("driveway"), py_str("path"), py_str("pavement"), py_str("wall"), py_str("walls"), py_str("ceiling"), py_str("ceiling"), py_str("plaster"), py_str("plasterboard"), py_str("brick"), py_str("bricks"), py_str("concrete"), py_str("paving"), py_str("grout"), py_str("car"), py_str("cars"), py_str("rust"), py_str("windscreen"), py_str("fence"), py_str("gate")]));
    s__ASKS_THE_MATERIAL(py_tuple(vec![py_str("what grit"), py_str("which grit"), py_str("what sandpaper"), py_str("which sandpaper"), py_str("what paper"), py_str("which paper"), py_str("what paint"), py_str("which paint"), py_str("what varnish"), py_str("which varnish"), py_str("how many coats"), py_str("how long between"), py_str("best sandpaper"), py_str("best paper"), py_str("how much paint"), py_str("how much varnish"), py_str("what filler"), py_str("which filler"), py_str("what brush"), py_str("which brush")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("paint-runs-and-sags")); __args.push(py_tuple(vec![g_PAINT(), g_GOING_WRONG()])); __args.push(py_str("Load less: a third of the bristles wet, half a roller of paint, and lay it off with light strokes in one direction rather than going back over it. A run is too much paint in one spot, so scrape the roller back over the tray's ramp every time and don't reload until it stops giving paint, and keep the brush moving in one direction instead of scrubbing back and forth. When a run has started, brush it out while it is still wet - lay the brush flat and drag the run sideways into the dry paint - then leave it alone: going back over paint as it dries is what leaves the brushmarks. On a ceiling, paint towards yourself and slightly away from the wall so the last stroke is a pulled one, and thin the paint only if it is genuinely thick.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_A_LEAK(), &g_A_STAIN()), &g_A_DIFFERENT_JOB()), &py_tuple(vec![py_str("peel"), py_str("peels"), py_str("peeling"), py_str("peeled"), py_str("peely"), py_str("flake"), py_str("flakes"), py_str("flaking"), py_str("flakey"), py_str("flaky"), py_str("bubble"), py_str("bubbles"), py_str("bubbling"), py_str("bubbled"), py_str("blister"), py_str("blisters"), py_str("blistering"), py_str("hobby"), py_str("hobbies"), py_str("acrylics"), py_str("acrylic"), py_str("watercolour"), py_str("watercolors"), py_str("canvas"), py_str("oil painting"), py_str("art"), py_str("draw"), py_str("drawing"), py_str("learn to paint"), py_str("start painting"), py_str("beginner"), py_str("nail"), py_str("nails"), py_str("nail varnish"), py_str("car"), py_str("cars"), py_str("hair"), py_str("cake"), py_str("icing"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("spray paint"), py_str("spraying"), py_str("graffiti"), py_str("drying out"), py_str("dried out"), py_str("keep paint"), py_str("keeping paint"), py_str("store paint"), py_str("storing paint"), py_str("leftover paint"), py_str("left over paint"), py_str("old paint"), py_str("tip"), py_str("tin"), py_str("tins")]))), ("guard".to_string(), g__paint_is_the_subject())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("roller-splatter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("roller"), py_str("rollers"), py_str("rolling"), py_str("sleeve"), py_str("sleeves"), py_str("rolling pin")]), py_tuple(vec![py_str("splatter"), py_str("splatters"), py_str("splattering"), py_str("spatter"), py_str("spatters"), py_str("spattering"), py_str("flick"), py_str("flicks"), py_str("flicking"), py_str("splash"), py_str("splashes"), py_str("splashed"), py_str("splashing"), py_str("spitting"), py_str("spits"), py_str("spitting paint"), py_str("flinging"), py_str("throwing"), py_str("flies"), py_str("flying"), py_str("mess"), py_str("messy"), py_str("dots"), py_str("droplets"), py_str("spray"), py_str("spraying"), py_str("splatter paint"), py_str("tracks"), py_str("lines")])])); __args.push(py_str("Half a roller of paint, a slow push, and roll the excess off on the tray's ramp before the sleeve touches the wall: a roller splatters because it is overloaded and moving too fast, and paint spins off the top of a sleeve that is turning at the ceiling. Then roll from the dry part into the wet in long strokes with light pressure. The sleeve matters too - short pile for smooth plaster, medium for a light texture, and a long pile holds so much paint that it throws it - and a brand new cheap sleeve spits fuzz for the first wall, so wash it out first or wrap tape round your hand and run it over. Sheet the floor and tape the edges, and any fleck that lands wipes off with a damp cloth while it is wet.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&g_A_LEAK(), &g_A_STAIN()), &py_tuple(vec![py_str("pastry"), py_str("baking"), py_str("dough"), py_str("lint"), py_str("deodorant"), py_str("hair"), py_str("paint off"), py_str("get paint off"), py_str("remove paint"), py_str("stripper")]))), ("guard".to_string(), g__paint_is_the_subject())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("paint-on-the-skirting")); __args.push(py_tuple(vec![py_tuple(vec![py_str("skirting"), py_str("skirtings"), py_str("skirting board"), py_str("skirting boards"), py_str("woodwork"), py_str("door frame"), py_str("doorframe"), py_str("door frames"), py_str("architrave"), py_str("picture rail"), py_str("banister"), py_str("banisters"), py_str("radiator pipe"), py_str("socket"), py_str("sockets"), py_str("light switch"), py_str("light switches")]), py_tuple(vec![py_str("paint"), py_str("paints"), py_str("painted"), py_str("painting"), py_str("roller"), py_str("rollers"), py_str("brush"), py_str("brushes"), py_str("splash"), py_str("splashes"), py_str("splashed"), py_str("drips"), py_str("dripping"), py_str("mess"), py_str("messy"), py_str("marks"), py_str("keep"), py_str("stop"), py_str("protect"), py_str("cover"), py_str("covering"), py_str("masking"), py_str("tape"), py_str("taping"), py_str("getting"), py_str("goes"), py_str("going"), py_str("cut in"), py_str("cutting in"), py_str("neat"), py_str("neatly")])])); __args.push(py_str("Press a low-tack masking tape along the top edge of the skirting, working it down into the join with a fingernail, then cut in with a 2 inch angled brush and brush the last inch slowly towards the tape rather than at it, so the paint is not pushed underneath. Pull the tape off at a shallow angle while the paint is still soft - leave it until the paint is hard and it tears the edge off with it. Anything that does get through: a wet cloth takes emulsion off a gloss skirting straight away, and once it is dry a sharp scraper or a fingernail lifts it without marking the gloss. Masking tape, not parcel tape, and not left on for a week, or it takes the gloss with it when it comes off.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_A_LEAK(), &g_A_STAIN()), &g_A_DIFFERENT_JOB()), &py_tuple(vec![py_str("paint off"), py_str("get paint off"), py_str("getting paint off"), py_str("clean paint off"), py_str("cleaning paint off"), py_str("remove paint"), py_str("removing paint"), py_str("off the door"), py_str("off the frame"), py_str("off the floor"), py_str("paint the skirting"), py_str("paint a skirting"), py_str("painting the skirting"), py_str("painting a skirting"), py_str("paint my skirting"), py_str("painting my skirting"), py_str("gloss the skirting"), py_str("glossing the skirting"), py_str("skirting tile"), py_str("tiles"), py_str("tiling"), py_str("grout"), py_str("caulk"), py_str("silicone"), py_str("rot"), py_str("woodworm"), py_str("how do i fit"), py_str("fitting"), py_str("replace"), py_str("replacing"), py_str("remove"), py_str("removing a"), py_str("how high"), py_str("height")]))), ("guard".to_string(), g__paint_is_the_subject())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("brush-for-paint")); __args.push(py_tuple(vec![py_tuple(vec![py_str("brush"), py_str("brushes"), py_str("paintbrush"), py_str("paintbrushes"), py_str("filament"), py_str("filaments"), py_str("bristle"), py_str("bristles"), py_str("sleeve"), py_str("roller")]), py_tuple(vec![py_str("gloss"), py_str("glossing"), py_str("emulsion"), py_str("satin"), py_str("eggshell"), py_str("undercoat"), py_str("primer"), py_str("paint"), py_str("paints"), py_str("painting"), py_str("water based"), py_str("waterbased"), py_str("oil based"), py_str("oilbased"), py_str("solvent"), py_str("cutting in"), py_str("cut in"), py_str("woodwork"), py_str("trim"), py_str("skirting"), py_str("door"), py_str("doors")])])); __args.push(py_str("Synthetic filaments for anything water-based - emulsion, water-based gloss and satin - and natural bristle for oil-based gloss, which holds the solvent and lays off flatter without leaving brushmarks. Width matters more than brand: a 2 inch angled sash brush for cutting in and for gloss on woodwork, a 3 or 4 inch brush for a wall, and cheap is fine for emulsion but not for gloss, where a shedding brush leaves hairs in the finish. Between coats, wrap a water-based brush in a damp cloth inside a bag rather than washing it out each time, and comb a bristle brush straight after cleaning so it dries in shape.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("hair"), py_str("hairbrush"), py_str("beard"), py_str("dog"), py_str("cat"), py_str("pet"), py_str("pastry"), py_str("basting"), py_str("makeup"), py_str("make up"), py_str("blusher"), py_str("nail"), py_str("toothbrush"), py_str("teeth"), py_str("broom"), py_str("yard brush"), py_str("clean"), py_str("cleaning"), py_str("clean a paintbrush"), py_str("washing out"), py_str("dried out"), py_str("how do i clean"), py_str("buy"), py_str("bought"), py_str("best brand"), py_str("cheap"), py_str("price"), py_str("cost")])), ("guard".to_string(), g__paint_is_the_subject())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thinning-paint")); __args.push(py_tuple(vec![py_tuple(vec![py_str("thin"), py_str("thinning"), py_str("thinned"), py_str("thinner"), py_str("thinners"), py_str("dilute"), py_str("diluting"), py_str("water down"), py_str("water it down"), py_str("too thick"), py_str("thick"), py_str("stiff")]), py_tuple(vec![py_str("gloss"), py_str("glossing"), py_str("emulsion"), py_str("satin"), py_str("eggshell"), py_str("undercoat"), py_str("primer"), py_str("paint"), py_str("paints"), py_str("painting"), py_str("varnish"), py_str("lacquer"), py_str("solvent"), py_str("white spirit"), py_str("turps"), py_str("brush marks"), py_str("brushmarks")])])); __args.push(py_str("White spirit for oil-based gloss, water for water-based paint, and only a little: about one part white spirit to ten parts paint, stirred in rather than shaken, and no more than about a tenth of water in emulsion or water-based gloss. More than that and the paint stops forming a film, so it dries patchy and the sheen goes. Thin only when the paint is genuinely stiff - a fresh tin that is hard to brush is usually too cold rather than too thick, so stand it in warm water for ten minutes first, and never thin a primer or an undercoat to stretch it, because that is what makes it fail.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_A_STAIN(), &py_tuple(vec![py_str("gravy"), py_str("sauce"), py_str("soup"), py_str("jam"), py_str("cream"), py_str("yoghurt"), py_str("smoothie"), py_str("hair"), py_str("blood"), py_str("nail varnish"), py_str("nail polish"), py_str("lose weight"), py_str("thinner person"), py_str("become thin"), py_str("too thin"), py_str("underweight"), py_str("varnish a floor"), py_str("floorboards"), py_str("french polish"), py_str("car paint"), py_str("car"), py_str("cars"), py_str("spray")]))), ("guard".to_string(), g__paint_is_the_subject())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("prep-before-finish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sand"), py_str("sanding"), py_str("sanded"), py_str("sandpaper"), py_str("sand down"), py_str("smooth"), py_str("fill"), py_str("filling"), py_str("filled"), py_str("filler"), py_str("patch"), py_str("patching")]), py_tuple(vec![py_str("varnish"), py_str("varnishing"), py_str("varnished"), py_str("oil"), py_str("oiling"), py_str("wax"), py_str("waxing"), py_str("stain"), py_str("painting"), py_str("paint"), py_str("gloss"), py_str("lacquer"), py_str("prime"), py_str("priming"), py_str("primer"), py_str("undercoat"), py_str("coat"), py_str("coats"), py_str("first"), py_str("before"), py_str("after"), py_str("between")])])); __args.push(py_str("Fill first, then sand, then finish: push the filler into the dents and holes, leave it a little proud, and sand it back level with the wood once it is hard. Then sand the whole surface - 120 grit on a block for paint or varnish, 80 to 100 on a filler patch - wipe the dust off with a tack cloth, and put the first coat on. Sand lightly between coats with 180 to 240 grit and wipe the dust off again, because that is what makes the last coat sit flat; leave the final coat alone. On a wall you only need to sand between coats if there is a run or a bit of grit.")); __kws.extend(vec![("guard".to_string(), g__the_order_of_two_steps()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("brackets-then-the-shelf")); __args.push(py_tuple(vec![py_tuple(vec![py_str("shelf"), py_str("shelves"), py_str("bracket"), py_str("brackets"), py_str("shelf bracket"), py_str("floating shelf")]), py_tuple(vec![py_str("first"), py_str("before"), py_str("or"), py_str("then"), py_str("put up"), py_str("hang"), py_str("fix"), py_str("fixing")])])); __args.push(py_str("The brackets go up first. Hold the shelf where you want it, mark the holes through the brackets with a pencil, take the shelf down, and fix the brackets to the wall with a fixing that suits it: a wall plug in brick or block, a screw into a stud, or a hollow-wall anchor in plasterboard with nothing behind it. Then screw the shelf to the brackets and check it with a spirit level along the shelf itself, not along one bracket.")); __kws.extend(vec![("guard".to_string(), g__a_shelf_and_its_brackets()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _paint_is_the_subject(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is about paint going on a surface, not paint being kept or bought.\n\n    \"how do i keep paint from drying out\" and \"where do i store paint\" are keeping questions, and\n    a food in the same line (\"how do i stop paint getting on the new rug while the cake cools\")\n    is another card's. A line that names neither the paint nor a surface that is only ever painted\n    is not this row's either, however many of its verbs the key holds.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&crate::knowledge::lookup::a_food_is_being_kept(&[understood.clone()])) {
    return Py::Bool(false);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__PAINT_WORD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__DECORATED_SURFACE(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_order_of_two_steps(args: &[Py]) -> Py {
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
    let mut step: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names two steps of finishing woodwork and asks which comes first.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_WOODWORK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKS_THE_MATERIAL()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ORDER_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return op_ge(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__STEPS(), "items", &[])) { let __unpk3844 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk3844[0].clone(); let mut step: Py = __unpk3844[1].clone(); if py_truthy(&Py::Bool(py_contains(&text.clone(), &word.clone()))) {  __out.push(step.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _a_shelf_and_its_brackets(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about fixing a shelf and asks whether the shelf or the brackets first.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&text.clone(), &py_str("shelf"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&text.clone(), &py_str("shelves"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&text.clone(), &py_str("bracket"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&text.clone(), &py_str("brackets"))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("first"), py_str("before"), py_str("or"), py_str("then"), py_str("put up"), py_str("hang")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.decorating".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "A_DIFFERENT_JOB" => Some(g_A_DIFFERENT_JOB()),
            "A_LEAK" => Some(g_A_LEAK()),
            "A_STAIN" => Some(g_A_STAIN()),
            "ENTRIES" => Some(g_ENTRIES()),
            "GOING_WRONG" => Some(g_GOING_WRONG()),
            "PAINT" => Some(g_PAINT()),
            "_ASKS_THE_MATERIAL" => Some(g__ASKS_THE_MATERIAL()),
            "_DECORATED_SURFACE" => Some(g__DECORATED_SURFACE()),
            "_NOT_WOODWORK" => Some(g__NOT_WOODWORK()),
            "_ORDER_WORDS" => Some(g__ORDER_WORDS()),
            "_PAINT_WORD" => Some(g__PAINT_WORD()),
            "_STEPS" => Some(g__STEPS()),
            "_a_shelf_and_its_brackets" => Some(py_func("_a_shelf_and_its_brackets", |args: &[Py]| _a_shelf_and_its_brackets(args))),
            "_paint_is_the_subject" => Some(py_func("_paint_is_the_subject", |args: &[Py]| _paint_is_the_subject(args))),
            "_the_order_of_two_steps" => Some(py_func("_the_order_of_two_steps", |args: &[Py]| _the_order_of_two_steps(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "A_DIFFERENT_JOB" => s_A_DIFFERENT_JOB(v),
            "A_LEAK" => s_A_LEAK(v),
            "A_STAIN" => s_A_STAIN(v),
            "ENTRIES" => s_ENTRIES(v),
            "GOING_WRONG" => s_GOING_WRONG(v),
            "PAINT" => s_PAINT(v),
            "_ASKS_THE_MATERIAL" => s__ASKS_THE_MATERIAL(v),
            "_DECORATED_SURFACE" => s__DECORATED_SURFACE(v),
            "_NOT_WOODWORK" => s__NOT_WOODWORK(v),
            "_ORDER_WORDS" => s__ORDER_WORDS(v),
            "_PAINT_WORD" => s__PAINT_WORD(v),
            "_STEPS" => s__STEPS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.decorating", module_obj);
    pyrt::xrefs::reg_fn("knowledge.decorating", "_paint_is_the_subject", py_func("_paint_is_the_subject", |args: &[Py]| _paint_is_the_subject(args)));
    pyrt::xrefs::reg_fn("knowledge.decorating", "_the_order_of_two_steps", py_func("_the_order_of_two_steps", |args: &[Py]| _the_order_of_two_steps(args)));
    pyrt::xrefs::reg_fn("knowledge.decorating", "_a_shelf_and_its_brackets", py_func("_a_shelf_and_its_brackets", |args: &[Py]| _a_shelf_and_its_brackets(args)));
    pyrt::xrefs::reg_access("knowledge.decorating", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.decorating", "A_DIFFERENT_JOB", g_A_DIFFERENT_JOB, s_A_DIFFERENT_JOB);
    pyrt::xrefs::reg_access("knowledge.decorating", "A_LEAK", g_A_LEAK, s_A_LEAK);
    pyrt::xrefs::reg_access("knowledge.decorating", "A_STAIN", g_A_STAIN, s_A_STAIN);
    pyrt::xrefs::reg_access("knowledge.decorating", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.decorating", "GOING_WRONG", g_GOING_WRONG, s_GOING_WRONG);
    pyrt::xrefs::reg_access("knowledge.decorating", "PAINT", g_PAINT, s_PAINT);
    pyrt::xrefs::reg_access("knowledge.decorating", "_ASKS_THE_MATERIAL", g__ASKS_THE_MATERIAL, s__ASKS_THE_MATERIAL);
    pyrt::xrefs::reg_access("knowledge.decorating", "_DECORATED_SURFACE", g__DECORATED_SURFACE, s__DECORATED_SURFACE);
    pyrt::xrefs::reg_access("knowledge.decorating", "_NOT_WOODWORK", g__NOT_WOODWORK, s__NOT_WOODWORK);
    pyrt::xrefs::reg_access("knowledge.decorating", "_ORDER_WORDS", g__ORDER_WORDS, s__ORDER_WORDS);
    pyrt::xrefs::reg_access("knowledge.decorating", "_PAINT_WORD", g__PAINT_WORD, s__PAINT_WORD);
    pyrt::xrefs::reg_access("knowledge.decorating", "_STEPS", g__STEPS, s__STEPS);
}
