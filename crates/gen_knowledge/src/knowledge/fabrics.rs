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
thread_local! { static G_DEFINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_DEFINE() -> Py { ensure_init(); G_DEFINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFINE(v: Py) { ensure_init(); G_DEFINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_USES() -> Py { ensure_init(); G_USES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USES(v: Py) { ensure_init(); G_USES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What bedding, clothes and towels are made of, and what the numbers on the label mean.\n\nThe answers lane owns this domain. Nothing in the tree answered it: on main `4f06dd8` \"what is a\ntog rating\", \"what is thread count\", \"what does gsm mean in fabric\", \"what is viscose\", \"what is\nlinen made from\", \"what is bamboo fabric\" and \"what is the difference between cotton and\npolyester\" were all search hedges, and they are the questions a person has in a shop with a duvet\nin one hand. The nearest cards - `clothing.py` on how to wash a thing, `home.py` on a bed - both\ntell you what to do with a fabric you already own, not what the words on the packet mean.\n\nThe facts are keyed by the term or the fibre, never by the asking: \"what is a tog rating\", \"what\ntog for winter\" and \"what tog duvet should i buy\" are one fact about togs. Every number here is\nthe ordinary retail one, because the point of the answer is that the shopper can act on it.\n\nThe `avoid` lists keep the neighbours out: a \"gsm\" line is about paper as often as fabric, \"tog\"\nis not a tog, and \"thread count\" is a sewing question as well as a sheet one.\n");
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("shopping")]));
    s_DEFINE(py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("what does"), py_str("what do"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("definition"), py_str("define"), py_str("explain"), py_str("how much"), py_str("how many"), py_str("difference"), py_str("difference between"), py_str("which"), py_str("better"), py_str("should i"), py_str("do i need"), py_str("is"), py_str("are"), py_str("explained"), py_str("stand for"), py_str("stand for"), py_str("work"), py_str("works")]));
    s_USES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("sheet"), py_str("sheets"), py_str("bedsheet"), py_str("bedsheets"), py_str("bed sheet"), py_str("bed sheets"), py_str("bedding"), py_str("duvet cover"), py_str("duvet covers"), py_str("pillowcase"), py_str("pillowcases")]), py_str("For bedding, long-staple cotton in a percale weave is the cool, crisp choice and lasts; sateen is the smoother, warmer one if you sleep cold. Linen is the coolest in summer and softens for years but creases. Polycotton is cheap and needs little ironing, and it feels it - it traps heat.")]), py_tuple(vec![py_tuple(vec![py_str("towel"), py_str("towels"), py_str("bath towel"), py_str("hand towel")]), py_str("For towels, cotton terry at 500 to 600 GSM is the sweet spot: heavier is plusher but slow to dry, and anything under 400 is thin. Egyptian or Turkish cotton is a marketing label more than a guarantee - a long-staple cotton at 550 GSM beats a short-staple one at 700.")]), py_tuple(vec![py_tuple(vec![py_str("t-shirt"), py_str("tee"), py_str("tshirt"), py_str("tshirts"), py_str("underwear"), py_str("vest"), py_str("socks")]), py_str("For something worn all day next to the skin, cotton or a cotton-rich blend: it breathes and does not hold smell the way polyester does. 180 to 220 GSM cotton is a t-shirt that keeps its shape; a 2% to 5% elastane blend is what stops underwear going baggy.")]), py_tuple(vec![py_tuple(vec![py_str("curtain"), py_str("curtains"), py_str("cushion"), py_str("cushions"), py_str("upholstery"), py_str("sofa cover")]), py_str("For curtains and cushions, weight and light matter more than the fibre: a lining makes any fabric hang better and keeps the light out, and a polycotton or a linen-look weave is cheaper and does not fade the way a printed cotton does. For a sofa that gets sat on, a tight weave in a blend beats a loose linen.")]), py_tuple(vec![py_tuple(vec![py_str("winter coat"), py_str("coat"), py_str("jacket"), py_str("raincoat"), py_str("waterproof")]), py_str("For a coat, the outer layer does one job and the lining another: a tightly woven wool or a synthetic shell for wind and rain, and a smooth lining so it goes on over a jumper. Wool keeps you warm when damp, which is why it is still the best winter fabric; down is warmer for its weight but useless wet. Check the seams for taped ones on anything sold as waterproof.")]), py_tuple(vec![py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids")]), py_str("For a baby, plain cotton next to the skin and nothing loose: cotton breathes and washes hot, and it does not hold the smell that polyester does. Avoid anything with a long pile or a loose trim, keep the bedding tight-fitting, and wash new clothes once before they go on.")]), py_tuple(vec![py_tuple(vec![py_str("gym"), py_str("sport"), py_str("running"), py_str("exercise"), py_str("workout"), py_str("cycling")]), py_str("For sport, polyester or a polyamide blend: it does not absorb sweat, so it dries fast and stays light, where cotton gets heavy and cold. Look for a mesh panel or a wicking finish, and wash it soon after - the smell that lingers in sports kit is oil trapped in the fibre, which a cool wash does not shift.")])]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tog-rating")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tog"), py_str("togs"), py_str("tog rating"), py_str("tog value"), py_str("duvet rating"), py_str("duvet tog")]), g_DEFINE()])); __args.push(py_str("A tog is how well a duvet holds warmth, not how thick it is: 4.5 is a summer duvet, 9 to 10.5 is spring and autumn, 13.5 is winter, and 15 is for a cold room. Most people are best off with a 10.5 for eight months of the year and a 4.5 for the hot ones, and two duvets - a 4.5 and a 9 - can be poppered together for winter instead of buying a 13.5. Children under one should not have a duvet at all.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("toga"), py_str("togas"), py_str("together"), py_str("toggle"), py_str("toggles"), py_str("coat"), py_str("jacket"), py_str("sleeping bag"), py_str("cot"), py_str("moses basket"), py_str("swaddle"), py_str("sleep bag"), py_str("grow bag")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thread-count")); __args.push(py_tuple(vec![py_tuple(vec![py_str("thread count"), py_str("threadcount"), py_str("thread counts"), py_str("percale"), py_str("sateen"), py_str("weave")]), g_DEFINE()])); __args.push(py_str("Thread count is the number of threads woven into one square inch, and past about 300 it stops telling you anything: the count is easy to inflate by counting plied yarns twice, so a 1,000-count sheet is often a 500-count one with two threads twisted together. The fibre and the weave matter more - long-staple cotton in a percale weave is cool and crisp, sateen is smoother and warmer, and a 200-count percale lasts longer than a thin 800.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("sewing thread"), py_str("cotton thread"), py_str("needle"), py_str("sewing machine"), py_str("embroidery"), py_str("serger"), py_str("overlock"), py_str("knitting"), py_str("wool")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gsm-fabric")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gsm"), py_str("grams per square metre"), py_str("grams per square meter"), py_str("grammage"), py_str("fabric weight"), py_str("weight of the fabric")]), g_DEFINE()])); __args.push(py_str("GSM is grams per square metre, and it is how heavy a fabric is: a thin t-shirt is 140 to 180, a good everyday tee 180 to 220, a hoodie 300 to 400, and a heavy sweatshirt 400 or more. Towels run 400 to 600 GSM, and bedding 120 to 150. Higher is thicker, heavier and warmer, not automatically better - a 600 GSM towel takes forever to dry outdoors.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("paper"), py_str("card"), py_str("cardstock"), py_str("stationery"), py_str("phone"), py_str("mobile"), py_str("network"), py_str("signal"), py_str("sim"), py_str("printer"), py_str("printing"), py_str("photocopy")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cotton-vs-polyester")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cotton and polyester"), py_str("polyester and cotton"), py_str("cotton or polyester"), py_str("polyester"), py_str("polycotton"), py_str("poly cotton"), py_str("cotton vs polyester"), py_str("difference between cotton")]), g_DEFINE()])); __args.push(py_str("Cotton breathes, takes up water and gets softer with washing; polyester does not absorb water, so it dries fast and wicks sweat, but it holds body oils and smell, which is why a cheap polyester top pongs after one wear. A blend is the compromise most clothes are: 60/40 polycotton creases less and dries faster than pure cotton, and 5% elastane in a cotton shirt is what stops it losing its shape. For sheets and summer shirts, cotton; for sportswear and rain jackets, polyester.")); __kws.extend(vec![("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("viscose-rayon")); __args.push(py_tuple(vec![py_tuple(vec![py_str("viscose"), py_str("rayon"), py_str("lyocell"), py_str("tencel"), py_str("modal"), py_str("cupro"), py_str("acetate"), py_str("bamboo fabric"), py_str("bamboo viscose"), py_str("bamboo lyocell")]), g_DEFINE()])); __args.push(py_str("Viscose, rayon, modal and lyocell are all made from wood or bamboo pulp dissolved and spun into fibre, which is why they feel cool and drape like silk; lyocell (Tencel) is the version made with a solvent that is recovered and reused. They are weak when wet, so wash them cool on a gentle cycle and never wring them, and dry them flat because they stretch out on a hanger. \"Bamboo fabric\" on a label is nearly always bamboo viscose, and it is not antibacterial however it is sold.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("bamboo plant"), py_str("bamboo cane"), py_str("bamboo screen"), py_str("garden"), py_str("shoots"), py_str("panda"), py_str("plant"), py_str("grow"), py_str("cut down")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("linen-fibre")); __args.push(py_tuple(vec![py_tuple(vec![py_str("linen"), py_str("flax"), py_str("linen fabric"), py_str("linen sheets")]), g_DEFINE()])); __args.push(py_str("Linen is woven from the fibres of the flax plant, which makes it the strongest of the plant fibres and the coolest in heat: it carries water away fast and does not cling. It creases the moment you move and feels stiff when new, and both of those go off with washing - a linen shirt gets softer every wash for years. Wash it at 30 or 40 and never tumble dry it hot, or it shrinks and goes brittle.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("linen cupboard"), py_str("linen basket"), py_str("table linen"), py_str("bed linen"), py_str("towels"), py_str("restaurant"), py_str("napkins"), py_str("hire"), py_str("laundry")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("elastane")); __args.push(py_tuple(vec![py_tuple(vec![py_str("elastane"), py_str("spandex"), py_str("lycra"), py_str("stretch fabric"), py_str("what is 5% elastane")]), g_DEFINE()])); __args.push(py_str("Elastane - sold as Lycra or Spandex - is the rubber-like fibre that makes a fabric stretch and spring back: 2% in a shirt keeps the shape, 5% in jeans lets you sit down, and 20% is sportswear. It is what wears out first, because heat and chlorine break it down: wash it cool and never tumble dry it, and it will keep its stretch for years instead of going baggy at the knees.")); __kws.extend(vec![("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("which-fabric-for")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fabric for"), py_str("material for"), py_str("what fabric"), py_str("which fabric"), py_str("best fabric"), py_str("best material"), py_str("what material"), py_str("which material")]), py_tuple(vec![py_str("should"), py_str("use"), py_str("buy"), py_str("best"), py_str("for"), py_str("which"), py_str("what")])])); __args.push(g__the_use()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); call_builtin("bool", &[_the_use(&[Py::None, reading.clone()])]).unwrap_or_else(|e| exc::raise(e)) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _the_text(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line as it was said, from the reading every entry is handed.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    text = { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&reading.clone(), "get", &[py_str("text")]) } };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    words = mcall(&reading.clone(), "get", &[py_str("words")]);
    text = if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[words.clone()]) } else { py_str("") };
    }
    return mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    Py::None
}
pub fn _the_use(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The fabric the line's own thing wants, or None when it names no use.");
    text = _the_text(&[reading.clone()]);
    let __iter1 = py_iter_or_raise(&g_USES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.fabrics".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "DEFINE" => Some(g_DEFINE()),
            "ENTRIES" => Some(g_ENTRIES()),
            "USES" => Some(g_USES()),
            "_the_text" => Some(py_func("_the_text", |args: &[Py]| _the_text(args))),
            "_the_use" => Some(py_func("_the_use", |args: &[Py]| _the_use(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "DEFINE" => s_DEFINE(v),
            "ENTRIES" => s_ENTRIES(v),
            "USES" => s_USES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.fabrics", module_obj);
    pyrt::xrefs::reg_fn("knowledge.fabrics", "_the_text", py_func("_the_text", |args: &[Py]| _the_text(args)));
    pyrt::xrefs::reg_fn("knowledge.fabrics", "_the_use", py_func("_the_use", |args: &[Py]| _the_use(args)));
    pyrt::xrefs::reg_access("knowledge.fabrics", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.fabrics", "DEFINE", g_DEFINE, s_DEFINE);
    pyrt::xrefs::reg_access("knowledge.fabrics", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.fabrics", "USES", g_USES, s_USES);
}
