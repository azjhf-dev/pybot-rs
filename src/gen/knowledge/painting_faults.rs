#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_ACCIDENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FINISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LEAKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MARK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_OVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_STRIPPER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SURFACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_WAIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ON_THE_WAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PAINT_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_TRIM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g__A_ACCIDENT() -> Py { ensure_init(); _A_ACCIDENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_ACCIDENT(v: Py) { ensure_init(); if let Ok(mut w) = _A_ACCIDENT.write() { *w = v; } }
pub fn g__A_FINISH() -> Py { ensure_init(); _A_FINISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FINISH(v: Py) { ensure_init(); if let Ok(mut w) = _A_FINISH.write() { *w = v; } }
pub fn g__A_LEAKING() -> Py { ensure_init(); _A_LEAKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LEAKING(v: Py) { ensure_init(); if let Ok(mut w) = _A_LEAKING.write() { *w = v; } }
pub fn g__A_MARK() -> Py { ensure_init(); _A_MARK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MARK(v: Py) { ensure_init(); if let Ok(mut w) = _A_MARK.write() { *w = v; } }
pub fn g__A_OVER() -> Py { ensure_init(); _A_OVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_OVER(v: Py) { ensure_init(); if let Ok(mut w) = _A_OVER.write() { *w = v; } }
pub fn g__A_STRIPPER() -> Py { ensure_init(); _A_STRIPPER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_STRIPPER(v: Py) { ensure_init(); if let Ok(mut w) = _A_STRIPPER.write() { *w = v; } }
pub fn g__A_SURFACE() -> Py { ensure_init(); _A_SURFACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SURFACE(v: Py) { ensure_init(); if let Ok(mut w) = _A_SURFACE.write() { *w = v; } }
pub fn g__A_WAIT() -> Py { ensure_init(); _A_WAIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_WAIT(v: Py) { ensure_init(); if let Ok(mut w) = _A_WAIT.write() { *w = v; } }
pub fn g__ON_THE_WAY() -> Py { ensure_init(); _ON_THE_WAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ON_THE_WAY(v: Py) { ensure_init(); if let Ok(mut w) = _ON_THE_WAY.write() { *w = v; } }
pub fn g__PAINT_WORD() -> Py { ensure_init(); _PAINT_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PAINT_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _PAINT_WORD.write() { *w = v; } }
pub fn g__THE_TRIM() -> Py { ensure_init(); _THE_TRIM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_TRIM(v: Py) { ensure_init(); if let Ok(mut w) = _THE_TRIM.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Three paint questions that hold no surface word, and so fell between the cards.\n\nMost painting questions name the thing being painted - a door, a wall, a window, a radiator -\nand a row that owns that thing answers them. These three name no surface, or name one the\nother cards read the other way round, and each was answered with something from another job:\n\n  how do i paint over gloss                   a finish named on its own is paint *already on*\n  how do i stop brush marks when painting     something, and \"over gloss\" is the sanding\n  gloss                                       question. The acrylics card for a beginner\n                                              answered it instead, because the only row that\n                                              holds gloss wants a surface word beside it.\n\n  my door is painted shut                     a skin of paint bridging a door to its frame is\n  the door is stuck shut with paint           a job to undo, not a door to paint. It was told\n                                              to take the door off and paint the edges, and\n                                              once that a stuck door wanted planing because it\n                                              was swollen with damp.\n\n  im painting the banister                    the trim in a room is either the thing being\n  how do i paint skirting boards              painted or the thing to be kept clean, and the\n                                              cards hold every word of both readings. The\n                                              protect-the-skirting row answered the first, so\n                                              the trim being painted has a row of its own.\n\nWhy a file: these are the faults and the trim, not a surface, and every existing painting\ncard is keyed by its surface. Keeping them here also keeps the three readings of the same\nwords in one place, where the guards that tell them apart can be read together.\n");
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    s__PAINT_WORD(py_tuple(vec![py_str("paint"), py_str("paints"), py_str("painted"), py_str("painting"), py_str("repaint"), py_str("repainting"), py_str("re-paint"), py_str("gloss"), py_str("glossing"), py_str("glossed"), py_str("satin"), py_str("eggshell"), py_str("undercoat"), py_str("undercoating"), py_str("primer"), py_str("priming"), py_str("primed"), py_str("varnish"), py_str("varnishing"), py_str("varnished"), py_str("coating")]));
    s__A_FINISH(py_tuple(vec![py_str("gloss"), py_str("glossy"), py_str("satin"), py_str("eggshell"), py_str("sheen"), py_str("matt"), py_str("matte"), py_str("undercoat"), py_str("primer"), py_str("primed"), py_str("varnish"), py_str("varnished"), py_str("lacquer"), py_str("lacqered"), py_str("lacquered"), py_str("oil"), py_str("oiled"), py_str("mdf")]));
    s__A_SURFACE(py_tuple(vec![py_str("door"), py_str("doors"), py_str("cupboard"), py_str("cupboards"), py_str("kitchen"), py_str("skirting"), py_str("skirtings"), py_str("banister"), py_str("banisters"), py_str("bannister"), py_str("bannisters"), py_str("woodwork"), py_str("radiator"), py_str("radiators"), py_str("wall"), py_str("walls"), py_str("ceiling"), py_str("ceilings"), py_str("room"), py_str("rooms"), py_str("floor"), py_str("floors"), py_str("floorboard"), py_str("floorboards"), py_str("window"), py_str("windows"), py_str("frame"), py_str("frames"), py_str("sill"), py_str("sills"), py_str("windowsill"), py_str("furniture"), py_str("table"), py_str("tables"), py_str("chair"), py_str("chairs"), py_str("shelf"), py_str("shelves"), py_str("shed"), py_str("fence"), py_str("fences"), py_str("gate"), py_str("gates"), py_str("bedroom"), py_str("bathroom"), py_str("hallway"), py_str("stairs"), py_str("stair"), py_str("handrail"), py_str("architrave"), py_str("trim"), py_str("wardrobe"), py_str("wardrobes"), py_str("cabinet"), py_str("cabinets"), py_str("drawer"), py_str("drawers"), py_str("toy"), py_str("toys"), py_str("car"), py_str("cars"), py_str("bike"), py_str("bicycle")]));
    s__A_ACCIDENT(py_tuple(vec![py_str("carpet"), py_str("carpets"), py_str("rug"), py_str("rugs"), py_str("sofa"), py_str("sofas"), py_str("couch"), py_str("settee"), py_str("curtain"), py_str("curtains"), py_str("cushion"), py_str("cushions"), py_str("clothes"), py_str("clothing"), py_str("shirt"), py_str("shirts"), py_str("jeans"), py_str("trousers"), py_str("jacket"), py_str("jackets"), py_str("hands"), py_str("hand"), py_str("hair"), py_str("skin"), py_str("fingers"), py_str("fingernails"), py_str("nails"), py_str("washing machine"), py_str("bath"), py_str("sink"), py_str("tiles"), py_str("tile"), py_str("tiling"), py_str("grass"), py_str("patio"), py_str("drive"), py_str("driveway")]));
    s__A_WAIT(py_tuple(vec![py_str("how long"), py_str("take to dry"), py_str("takes to dry"), py_str("dry"), py_str("dries"), py_str("drying"), py_str("dried"), py_str("recoat"), py_str("re-coat"), py_str("between coats"), py_str("touch dry"), py_str("hard"), py_str("cure"), py_str("curing"), py_str("seconds"), py_str("minutes"), py_str("hours"), py_str("days")]));
    s__A_STRIPPER(py_tuple(vec![py_str("remove"), py_str("removing"), py_str("removal"), py_str("get off"), py_str("take off"), py_str("taking off"), py_str("strip"), py_str("stripping"), py_str("stripper"), py_str("sand off"), py_str("sanding off"), py_str("peel"), py_str("peeling"), py_str("peeled"), py_str("flake"), py_str("flaking"), py_str("bubble"), py_str("bubbling"), py_str("blister"), py_str("blistering")]));
    s__THE_TRIM(py_tuple(vec![py_str("skirting"), py_str("skirtings"), py_str("skirting board"), py_str("skirting boards"), py_str("banister"), py_str("banisters"), py_str("bannister"), py_str("bannisters"), py_str("woodwork"), py_str("architrave"), py_str("handrail"), py_str("handrails"), py_str("spindle"), py_str("spindles"), py_str("balustrade"), py_str("picture rail"), py_str("dado rail"), py_str("window sill"), py_str("windowsill"), py_str("window sills"), py_str("windowsills"), py_str("sill"), py_str("sills"), py_str("trim"), py_str("door frame"), py_str("door frames")]));
    s__ON_THE_WAY(py_tuple(vec![py_str("on"), py_str("onto"), py_str("off"), py_str("getting"), py_str("goes"), py_str("going"), py_str("got"), py_str("spilt"), py_str("spilled"), py_str("splash"), py_str("splashes"), py_str("splashed"), py_str("drips"), py_str("dripping"), py_str("runs"), py_str("running"), py_str("from"), py_str("under"), py_str("behind"), py_str("against"), py_str("down"), py_str("near"), py_str("round"), py_str("around")]));
    s__A_MARK(py_tuple(vec![py_str("stain"), py_str("stains"), py_str("stained"), py_str("mark"), py_str("marks"), py_str("knot"), py_str("knots"), py_str("rust"), py_str("rusty"), py_str("nicotine"), py_str("tar"), py_str("smoke"), py_str("grease"), py_str("oily"), py_str("biro"), py_str("pen"), py_str("marker"), py_str("crayon"), py_str("felt tip"), py_str("felt pen"), py_str("watermark"), py_str("watermarks"), py_str("tide mark"), py_str("tide marks")]));
    s__A_LEAKING(py_tuple(vec![py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("leaked"), py_str("damp"), py_str("damp patch"), py_str("damp patches"), py_str("water"), py_str("rain"), py_str("raining"), py_str("roof"), py_str("gutter"), py_str("gutters"), py_str("pipe"), py_str("pipes"), py_str("burst"), py_str("condensation"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("flood"), py_str("flooded")]));
    s__A_OVER(py_tuple(vec![py_str("paint over"), py_str("painting over"), py_str("paint on top of"), py_str("cover"), py_str("covering"), py_str("showing through"), py_str("shows through"), py_str("showing up"), py_str("coming through"), py_str("come through"), py_str("bleeding through"), py_str("bleeds through"), py_str("seeping through"), py_str("block"), py_str("blocking"), py_str("stain block"), py_str("stainblock"), py_str("hiding")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("painted-shut")); __args.push(py_tuple(vec![py_tuple(vec![py_str("painted shut"), py_str("stuck shut"), py_str("sealed shut"), py_str("glued shut"), py_str("paint has sealed"), py_str("sealed with paint"), py_str("stuck with paint"), py_str("glued with paint"), py_str("paint in the gap"), py_str("paint in the cracks"), py_str("wont open"), py_str("won't open"), py_str("cannot open"), py_str("cant open"), py_str("can't open"), py_str("stuck fast"), py_str("jammed"), py_str("wont close"), py_str("won't close"), py_str("wont shut"), py_str("won't shut"), py_str("hard to close"), py_str("doesnt close"), py_str("doesn't close")]), py_tuple(vec![py_str("door"), py_str("doors"), py_str("front door"), py_str("back door"), py_str("drawer"), py_str("drawers"), py_str("cupboard door"), py_str("cupboard doors"), py_str("hatch"), py_str("lid")])])); __args.push(py_str("Score round the edge with a sharp knife where the paint has bridged it to the frame - down both sides, across the top, then along the bottom - and it will come free with a push rather than a shoulder. Then sand the paint off the edge and out of the rebate back to the wood, and rub a candle, a bar of soap or a little wax along the edge so the next coat cannot glue it again. If it is still tight afterwards, take a millimetre at a time off the edge with a plane or a sanding block, and prime the bare wood so it does not swell when the room gets damp. A lock or a handle painted over wants scoring round the same way before it will turn.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("car"), py_str("cars"), py_str("van"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("microwave"), py_str("washing machine"), py_str("dishwasher"), py_str("jar"), py_str("bottle"), py_str("glue"), py_str("superglue"), py_str("adhesive"), py_str("silicone"), py_str("caulk"), py_str("key"), py_str("keys"), py_str("lock"), py_str("locks"), py_str("locked out"), py_str("shut up"), py_str("shed door"), py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("damp"), py_str("damp patch"), py_str("water"), py_str("rain"), py_str("raining"), py_str("roof"), py_str("gutter"), py_str("gutters"), py_str("pipe"), py_str("pipes"), py_str("condensation"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("plaster"), py_str("brick"), py_str("bricks"), py_str("brickwork"), py_str("ceiling"), py_str("flood"), py_str("flooded"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("spilled"), py_str("spill"), py_str("scrub"), py_str("wipe off"), py_str("get it off"), py_str("remove"), py_str("removing"), py_str("clean off"), py_str("window"), py_str("windows"), py_str("sash"), py_str("sash window"), py_str("casement"), py_str("sill"), py_str("frame"), py_str("frames")])), ("guard".to_string(), g__the_paint_has_sealed_it())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-coat-over-a-finish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("paint over"), py_str("painting over"), py_str("paint on top of"), py_str("painting on top of"), py_str("over gloss"), py_str("over the gloss"), py_str("over satin"), py_str("over eggshell"), py_str("over old paint"), py_str("over the old paint"), py_str("over a finish"), py_str("gloss over"), py_str("satin over"), py_str("matt over"), py_str("matte over"), py_str("eggshell over"), py_str("brush marks"), py_str("brushmarks"), py_str("brush strokes"), py_str("brushstrokes"), py_str("brush lines"), py_str("stopping brush marks"), py_str("no brush marks")]), py_tuple(vec![py_str("gloss"), py_str("glossy"), py_str("satin"), py_str("eggshell"), py_str("sheen"), py_str("matt"), py_str("matte"), py_str("undercoat"), py_str("primer"), py_str("priming"), py_str("primed"), py_str("varnish"), py_str("varnished"), py_str("lacquer"), py_str("lacquered"), py_str("paint"), py_str("painting"), py_str("painted"), py_str("coat"), py_str("coats"), py_str("wood"), py_str("wooden"), py_str("mdf")])])); __args.push(py_str("Sand it dull first: new paint needs a key to grip, so take the shine off old gloss with 120 grit, wipe off the dust, then one coat of undercoat and your topcoat. A water-based satin goes on after the sanding alone and dries in a couple of hours; oil gloss straight onto old gloss is the way to get peeling in a year. Brush marks come from too much paint on a cheap brush, so thin it a little if it drags and keep the brush moving one way.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), add(&add(&add(&py_tuple(vec![py_str("what colour"), py_str("what color"), py_str("which colour"), py_str("which color"), py_str("colour"), py_str("colours"), py_str("color"), py_str("colors"), py_str("better"), py_str("worse"), py_str("which is"), py_str("matt or gloss"), py_str("gloss or matt"), py_str("satin or gloss"), py_str("gloss or satin"), py_str("eggshell or gloss"), py_str("wallpaper"), py_str("wall"), py_str("walls"), py_str("ceiling"), py_str("ceilings"), py_str("room"), py_str("rooms"), py_str("bedroom"), py_str("bathroom"), py_str("hallway"), py_str("kitchen"), py_str("tiles"), py_str("tile"), py_str("tiling"), py_str("grout"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("dulux"), py_str("farrow"), py_str("little greene"), py_str("buy"), py_str("bought"), py_str("cost"), py_str("price"), py_str("cheap"), py_str("litre"), py_str("litres"), py_str("how much"), py_str("how many coats")]), &g__A_ACCIDENT()), &g__A_WAIT()), &g__A_STRIPPER())), ("guard".to_string(), g__a_finish_with_no_surface())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the-trim-is-what-is-painted")); __args.push(py_tuple(vec![py_tuple(vec![py_str("paint the skirting"), py_str("paint skirting"), py_str("paint skirting boards"), py_str("painting the skirting"), py_str("painting skirting boards"), py_str("paint my skirting"), py_str("gloss the skirting"), py_str("gloss the skirting boards"), py_str("paint the banister"), py_str("paint banisters"), py_str("painting the banister"), py_str("painting the banisters"), py_str("paint the woodwork"), py_str("painting the woodwork"), py_str("paint woodwork"), py_str("gloss the banister"), py_str("gloss the woodwork"), py_str("paint the architrave"), py_str("paint the window sill"), py_str("paint the windowsill"), py_str("paint a banister"), py_str("paint a bannister"), py_str("paint a spindle"), py_str("paint a handrail"), py_str("paint an architrave"), py_str("paint a door frame"), py_str("paint a skirting"), py_str("paint the door frame"), py_str("paint the trim"), py_str("paint the handrail"), py_str("paint the spindles"), py_str("paint"), py_str("painting"), py_str("painted"), py_str("repaint"), py_str("repainting"), py_str("gloss"), py_str("glossing"), py_str("satin"), py_str("undercoat"), py_str("primer")]), g__THE_TRIM()])); __args.push(py_str("Sand them, wipe the dust off and run masking tape along the carpet, then brush with the grain in long strokes and a light load on the brush so it does not run onto the wall. Two thin coats beat one thick one, and satin or eggshell is easier to wipe than gloss. If you are doing the walls too, do the woodwork first.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("which"), py_str("should"), py_str("can"), py_str("best"), py_str("way"), py_str("sand"), py_str("sanding"), py_str("prep"), py_str("prepare"), py_str("method"), py_str("start"), py_str("need to"), py_str("give"), py_str("im"), py_str("i'm"), py_str("i am"), py_str("im just"), py_str("just painting"), py_str("just glossing"), py_str("want to"), py_str("wanna"), py_str("going to"), py_str("gonna")])), ("avoid".to_string(), py_tuple(vec![py_str("stop paint"), py_str("paint getting"), py_str("getting paint"), py_str("getting"), py_str("keep paint"), py_str("keep"), py_str("protect"), py_str("protecting"), py_str("mask"), py_str("masking"), py_str("cut in"), py_str("cutting in"), py_str("neatly"), py_str("neat"), py_str("splash"), py_str("splashes"), py_str("splashed"), py_str("spillage"), py_str("drips"), py_str("dripping"), py_str("clean"), py_str("cleaning"), py_str("cleaner"), py_str("clean off"), py_str("off the"), py_str("dust"), py_str("dusty"), py_str("wipe"), py_str("remove"), py_str("removing"), py_str("strip"), py_str("stripping"), py_str("sanding off"), py_str("colour"), py_str("color"), py_str("what colour"), py_str("which colour"), py_str("how much"), py_str("cost"), py_str("price"), py_str("buy")])), ("guard".to_string(), g__the_trim_is_what_is_painted())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-stain-under-the-coat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("over a stain"), py_str("over the stain"), py_str("over stains"), py_str("over old stains"), py_str("over a mark"), py_str("over the mark"), py_str("over marks"), py_str("over rust"), py_str("over the rust"), py_str("over nicotine"), py_str("cover"), py_str("covering"), py_str("showing through"), py_str("shows through"), py_str("showing up"), py_str("coming through"), py_str("come through"), py_str("bleeding through"), py_str("bleeds through"), py_str("seeping through"), py_str("stain block"), py_str("stainblock"), py_str("block the stain"), py_str("block a stain"), py_str("hiding")]), py_tuple(vec![py_str("stain"), py_str("stains"), py_str("stained"), py_str("mark"), py_str("marks"), py_str("knot"), py_str("knots"), py_str("rust"), py_str("rusty"), py_str("nicotine"), py_str("tar"), py_str("smoke"), py_str("grease"), py_str("oily"), py_str("biro"), py_str("pen"), py_str("marker"), py_str("crayon"), py_str("felt tip"), py_str("felt pen"), py_str("watermark"), py_str("watermarks"), py_str("tide mark"), py_str("tide marks"), py_str("wood"), py_str("wooden"), py_str("door"), py_str("doors"), py_str("skirting"), py_str("ceiling"), py_str("wall"), py_str("walls"), py_str("table"), py_str("shelf")])])); __args.push(py_str("Scrub it back first - sugar soap for grease or nicotine, a light sand for a mark in the wood - then seal it with a stain block: shellac or an aluminium primer, because ordinary undercoat lets the mark creep back through in a month. Two thin coats of block, then your finish. A knot wants the resin scraped out and shellac over it, and rust wants sanding back to bare metal and a rust-killing primer before the topcoat.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), add(&add(&add(&py_tuple(vec![py_str("ceiling"), py_str("ceilings"), py_str("wall"), py_str("walls"), py_str("roof"), py_str("keeps"), py_str("kept"), py_str("keep coming back"), py_str("how do i get"), py_str("get out"), py_str("get off"), py_str("out of"), py_str("off the"), py_str("off my"), py_str("remove"), py_str("removing"), py_str("clean"), py_str("cleaning"), py_str("scrub out"), py_str("washing"), py_str("wash"), py_str("colour"), py_str("color"), py_str("which colour"), py_str("what colour"), py_str("how much"), py_str("cost"), py_str("price"), py_str("buy")]), &g__A_ACCIDENT()), &g__A_LEAKING()), &g__A_WAIT())), ("guard".to_string(), g__a_stain_under_the_coat())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut word: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str(",.;:!?")])); } py_list(__out) })() };
    Py::None
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
    let _ = py_str("True when the line is about paint going on a surface, not a food about to be kept.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&crate::gen::knowledge::lookup::a_food_is_being_kept(&[understood.clone()])));
    Py::None
}
pub fn _a_finish_with_no_surface(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A finish named on its own: the coat going over what is already there.\n\n    \"how do i paint over gloss\" and \"i want to paint matt gloss over satin\" name no surface, so\n    no card that owns a surface can answer them, and the coat over a finish is the sanding one.\n    A line that names the surface instead (\"how do i paint over gloss paint on a door\", \"gloss\n    on the woodwork\") belongs to the card for that surface, and a coat that has to dry, a coat\n    that landed on a carpet and a finish being taken off are three other cards' questions.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_paint_is_the_subject(&[understood.clone()])))) {
    return Py::Bool(false);
    }
    words = _words(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_FINISH(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_SURFACE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__A_ACCIDENT(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__A_WAIT(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_STRIPPER(), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_paint_has_sealed_it(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The line itself names the paint that sealed the door.\n\n    \"wont shut\" and \"wont close\" are a swollen door's words, so this row wants the paint in the\n    line, not only in the chat before it: after a turn about a tin, \"the door wont shut\" belongs\n    to the swollen-door card.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_paint_is_the_subject(&[understood.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[understood.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__PAINT_WORD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_trim_is_what_is_painted(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line paints the trim itself, rather than protecting it from a wall's paint.\n\n    Both readings hold every word of the family - \"im painting the banister\" and \"how do i stop\n    paint getting on the skirting\" - so the difference is position: the trim straight after the\n    paint word is what is being painted, and the trim after a preposition is what is to be kept\n    clean.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_paint_is_the_subject(&[understood.clone()])))) {
    return Py::Bool(false);
    }
    words = _words(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g__PAINT_WORD(), &word.clone()))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&words.clone(), &add(&at.clone(), &Py::Int(1)), &add(&at.clone(), &Py::Int(5)), &Py::None));
    for __item in __iter3 {
    said = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__ON_THE_WAY(), &said.clone()))) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g__THE_TRIM(), &said.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_stain_under_the_coat(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A mark already in the surface, with a coat going over it: the blocking question.\n\n    \"how do i paint over a stain\" was the acrylics hobby card. The stain is not paint and it is\n    not dirt to scrub off now that the coat is the subject: what stops it creaping back through\n    is a coat of stain block, and knots want shellac on them. A stain on a carpet, a sofa or a\n    shirt is the cleaning card's, and water coming through a wall is the damp card's, because a\n    stain-block primer over a leak is a patch over a fault.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_paint_is_the_subject(&[understood.clone()])))) {
    return Py::Bool(false);
    }
    words = _words(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_MARK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__A_ACCIDENT(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_LEAKING(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__A_OVER(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__A_FINISH(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&word.clone(), &py_str("paint"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("painting"))) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.painting_faults".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_ACCIDENT" => Some(g__A_ACCIDENT()),
            "_A_FINISH" => Some(g__A_FINISH()),
            "_A_LEAKING" => Some(g__A_LEAKING()),
            "_A_MARK" => Some(g__A_MARK()),
            "_A_OVER" => Some(g__A_OVER()),
            "_A_STRIPPER" => Some(g__A_STRIPPER()),
            "_A_SURFACE" => Some(g__A_SURFACE()),
            "_A_WAIT" => Some(g__A_WAIT()),
            "_ON_THE_WAY" => Some(g__ON_THE_WAY()),
            "_PAINT_WORD" => Some(g__PAINT_WORD()),
            "_THE_TRIM" => Some(g__THE_TRIM()),
            "_a_finish_with_no_surface" => Some(py_func("_a_finish_with_no_surface", |args: &[Py]| _a_finish_with_no_surface(args))),
            "_a_stain_under_the_coat" => Some(py_func("_a_stain_under_the_coat", |args: &[Py]| _a_stain_under_the_coat(args))),
            "_paint_is_the_subject" => Some(py_func("_paint_is_the_subject", |args: &[Py]| _paint_is_the_subject(args))),
            "_the_paint_has_sealed_it" => Some(py_func("_the_paint_has_sealed_it", |args: &[Py]| _the_paint_has_sealed_it(args))),
            "_the_trim_is_what_is_painted" => Some(py_func("_the_trim_is_what_is_painted", |args: &[Py]| _the_trim_is_what_is_painted(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            _ => None,
        }),
        set: None,
    }))
}
