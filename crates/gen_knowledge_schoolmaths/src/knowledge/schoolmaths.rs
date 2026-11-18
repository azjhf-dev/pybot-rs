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
thread_local! { static G__BRACKET_MATHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__BRACKET_MATHS() -> Py { ensure_init(); G__BRACKET_MATHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BRACKET_MATHS(v: Py) { ensure_init(); G__BRACKET_MATHS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("School maths: the method, when the question has no numbers in it.\n\n\"How do i solve for x\", \"what is pythagoras theorem\", \"how do i use a protractor\" name a method\nand give nothing to work out, so the arithmetic answers in maths.py have nothing to do and these\nused to be shrugs. Two of them had another subject's answer: a square root came back as a plant's\nroots, and the angles of a triangle came back as whatever row held \"work out\".\n\nOne row per method, keyed by what the person is asking about - the method's own name - with a\nsecond group for the wording of the request, so \"explain pythagoras\", \"how does pythagoras work\"\nand \"what is the pythagoras theorem\" are the same question. A sum with numbers in it is handled by\nmaths.py before this file is reached, which is why the square root of 144 is worked out rather\nthan explained.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("maths-and-numbers")]));
    s__BRACKET_MATHS(stdlib_call("re", "compile", &[py_str("\\b(?:expand|expanding|expanded|simplify|simplifying|simplified|multiply|multiplying|factorise|factorize|factorising|solving|solve|equation|equations|algebra|algebraic|power|powers|term|terms|coefficient|coefficients|negative|negatives|out|maths|math|sum|sums|calculate|number|numbers|expression|expressions|double|triple|understand|working out|work out)\\b|\\d"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("expand"), py_str("expanding"), py_str("expanded"), py_str("how"), py_str("algebra"), py_str("method"), py_str("out"), py_str("do"), py_str("work out"), py_str("steps"), py_str("explain"), py_str("brackets"), py_str("bracket"), py_str("simplify")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("shelf"), py_str("shelves"), py_str("wall"), py_str("drill"), py_str("picture"), py_str("tv"), py_str("teeth"), py_str("orthodontic"), py_str("braces"), py_str("seating"), py_str("rows"), py_str("cupboard"), py_str("screws"), py_str("fixing"), py_str("business"), py_str("businesses"), py_str("company"), py_str("shop"), py_str("cafe"), py_str("restaurant"), py_str("team"), py_str("staff"), py_str("family"), py_str("branch"), py_str("premises"), py_str("grow"), py_str("growing"), py_str("staffing"), py_str("photo"), py_str("photos"), py_str("image"), py_str("images"), py_str("canvas"), py_str("font"), py_str("print"), py_str("screen")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("plant"), py_str("plants"), py_str("tree"), py_str("trees"), py_str("garden"), py_str("flower"), py_str("flowers"), py_str("cutting"), py_str("cuttings"), py_str("water"), py_str("soil"), py_str("pot"), py_str("potted"), py_str("root bound"), py_str("rootbound"), py_str("carrot"), py_str("parsnip"), py_str("beetroot"), py_str("hair"), py_str("ginger"), py_str("propagate")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("change from"), py_str("change from a note"), py_str("change from a tenner"), py_str("change from a fiver"), py_str("count back change"), py_str("count up change"), py_str("count out change"), py_str("work out change"), py_str("work out the change"), py_str("give change"), py_str("giving change"), py_str("the change from"), py_str("change in a shop"), py_str("wrong change"), py_str("how much change"), py_str("much change"), py_str("change should i get"), py_str("change do i get"), py_str("change would i get"), py_str("my change")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("how"), py_str("work out"), py_str("calculate"), py_str("much"), py_str("count"), py_str("do"), py_str("note"), py_str("till"), py_str("shop"), py_str("give"), py_str("back"), py_str("work"), py_str("right"), py_str("correct"), py_str("should"), py_str("get")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("career"), py_str("job"), py_str("jobs"), py_str("climate"), py_str("address"), py_str("nappy"), py_str("nappies"), py_str("baby"), py_str("oil"), py_str("filter"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("battery"), py_str("bulb"), py_str("change my"), py_str("change the"), py_str("change of"), py_str("life"), py_str("mind"), py_str("opinion"), py_str("subject"), py_str("school"), py_str("schools"), py_str("passport"), py_str("name"), py_str("energy"), py_str("provider"), py_str("supplier"), py_str("bank"), py_str("account"), py_str("password"), py_str("lock"), py_str("train"), py_str("bus"), py_str("tube"), py_str("hair"), py_str("sheets"), py_str("bed"), py_str("diet"), py_str("lose"), py_str("weight"), py_str("doctor"), py_str("dressing"), py_str("bandage"), py_str("nurse"), py_str("shift"), py_str("rota"), py_str("clothes")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("work out the tip"), py_str("work out a tip"), py_str("work out my tip"), py_str("work out tips"), py_str("work out the service"), py_str("work out a service"), py_str("calculate the tip"), py_str("calculate a tip"), py_str("calculated the tip"), py_str("calculating the tip"), py_str("figure out the tip"), py_str("figure out a tip"), py_str("how to work out the tip"), py_str("how do you work out a tip"), py_str("do the tip in my head"), py_str("tip in my head"), py_str("work out how much to tip"), py_str("work out what to tip")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("to the tip"), py_str("at the tip"), py_str("the tip is"), py_str("the tip was"), py_str("the tip closes"), py_str("tip run"), py_str("fly tip"), py_str("fly-tip"), py_str("rubbish tip"), py_str("recycling"), py_str("tip of"), py_str("tips of"), py_str("white tips"), py_str("french tips"), py_str("fingertips"), py_str("finger tips"), py_str("split ends"), py_str("nails"), py_str("nail"), py_str("hair"), py_str("hairdresser"), py_str("barber"), py_str("tip jar")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("cant do maths"), py_str("can't do maths"), py_str("cannot do maths"), py_str("can not do maths"), py_str("no good at maths"), py_str("bad at maths"), py_str("rubbish at maths"), py_str("hopeless at maths"), py_str("maths is not my thing"), py_str("maths isnt my thing"), py_str("maths is not my strong point"), py_str("struggle with maths"), py_str("struggled with maths"), py_str("maths is hard for me"), py_str("cant do sums"), py_str("cant do arithmetic"), py_str("cant do fractions"), py_str("cannot do fractions"), py_str("dont understand fractions"), py_str("dont get fractions"), py_str("never understood fractions"), py_str("cant do my times tables"), py_str("never learned my times tables"), py_str("never got maths"), py_str("failed maths"), py_str("hate maths"), py_str("afraid of numbers"), py_str("numbers scare me"), py_str("numbers confuse me")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("cant"), py_str("can't"), py_str("cannot"), py_str("never"), py_str("no good"), py_str("bad"), py_str("rubbish"), py_str("hopeless"), py_str("dont"), py_str("don't"), py_str("struggle"), py_str("struggled"), py_str("hard"), py_str("hate"), py_str("failed"), py_str("confuse"), py_str("scare"), py_str("afraid"), py_str("start"), py_str("beginner"), py_str("at all"), py_str("help"), py_str("how"), py_str("learn"), py_str("not"), py_str("isnt"), py_str("strong point"), py_str("my thing")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("scale up"), py_str("double a recipe"), py_str("halve a recipe"), py_str("cup"), py_str("cups"), py_str("grams"), py_str("ml"), py_str("ounces"), py_str("convert"), py_str("conversion"), py_str("unit"), py_str("units"), py_str("square root"), py_str("pythagoras"), py_str("triangle"), py_str("algebra"), py_str("equation"), py_str("bracket"), py_str("mean"), py_str("decimal"), py_str("percentage increase"), py_str("tax return"), py_str("spreadsheet")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("solve-for-x")); __args.push(py_tuple(vec![py_tuple(vec![py_str("solve for x"), py_str("solve x"), py_str("solve the equation"), py_str("solve this equation"), py_str("solve an equation"), py_str("algebra"), py_str("equations"), py_str("equation"), py_str("rearrange the formula"), py_str("rearrange a formula")]), py_tuple(vec![py_str("how"), py_str("solve"), py_str("what"), py_str("steps"), py_str("method"), py_str("start"), py_str("help"), py_str("stuck"), py_str("works"), py_str("explain"), py_str("beginner"), py_str("learn")])])); __args.push(py_str("Do the same thing to both sides until x is on its own: in 3x + 4 = 19 take 4 off both sides to get 3x = 15, then divide both by 3 for x = 5. Undo the adding and subtracting first and the multiplying or dividing second, then put the answer back in to check it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("simultaneous"), py_str("quadratic"), py_str("percentage"), py_str("split"), py_str("share"), py_str("sharing"), py_str("fair share")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("expand-brackets")); __args.push(py_tuple(vec![py_tuple(vec![py_str("brackets"), py_str("bracket"), py_str("multiply out"), py_str("multiplying out"), py_str("double brackets"), py_str("expand and simplify")]), __data1()])); __args.push(py_str("Multiply everything inside the bracket by what is outside it: 3(x + 4) is 3x + 12. With two brackets, (x + 2)(x + 3), multiply every pair, then add the like terms: x squared + 3x + 2x + 6, which is x squared + 5x + 6.")); __kws.extend(vec![("avoid".to_string(), __data2()), ("guard".to_string(), py_func("_a_bracket_is_a_sum", |args: &[Py]| _a_bracket_is_a_sum(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("long-multiply")); __args.push(py_tuple(vec![py_tuple(vec![py_str("long multiplication"), py_str("long multiply"), py_str("multiply two digit"), py_str("two digit numbers"), py_str("multiplying by hand"), py_str("big numbers by hand")]), py_tuple(vec![py_str("how"), py_str("method"), py_str("do"), py_str("work out"), py_str("steps"), py_str("explain"), py_str("by hand"), py_str("without a calculator")])])); __args.push(py_str("Write one number above the other and take the digits of the bottom number one at a time: 34 x 26 is 34 x 6 = 204, then 34 x 20 = 680, and 204 + 680 = 884. Each row moves one place to the left, which is the nought on the end, so keep the columns lined up.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("times tables"), py_str("times table")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("pythagoras"), py_tuple(vec![py_tuple(vec![py_str("pythagoras"), py_str("pythagorean"), py_str("hypotenuse"), py_str("pythagoras theorem"), py_str("pythagoras' theorem")]), py_tuple(vec![py_str("what"), py_str("theorem"), py_str("how"), py_str("explain"), py_str("use"), py_str("work out"), py_str("find"), py_str("side"), py_str("mean"), py_str("rule"), py_str("triangle")])]), py_str("In a right-angled triangle the two shorter sides squared add up to the longest side squared: a squared + b squared = c squared. Sides of 3 and 4 give 9 + 16 = 25, so the longest side is 5; to find a shorter side, take one square away from the other instead.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("protractor"), py_tuple(vec![py_tuple(vec![py_str("protractor"), py_str("measure an angle"), py_str("measure angles"), py_str("measuring angles"), py_str("read an angle")]), py_tuple(vec![py_str("how"), py_str("use"), py_str("read"), py_str("measure"), py_str("angle"), py_str("angles"), py_str("correctly"), py_str("zero"), py_str("which")])]), py_str("Put the hole of the protractor on the point of the angle and line the 0 line up with one arm, then read the number the other arm crosses - use the scale that starts at 0 on that side, so the two rows of numbers do not fool you. Under 90 is acute, over 90 is obtuse, and exactly 90 is a right angle.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("square-root-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("square root"), py_str("square roots"), py_str("cube root"), py_str("root of a number"), py_str("root of 144")]), py_tuple(vec![py_str("how"), py_str("work out"), py_str("find"), py_str("calculate"), py_str("what is"), py_str("method"), py_str("mean"), py_str("without a calculator"), py_str("number"), py_str("big number")])])); __args.push(py_str("A square root is the number that times itself makes yours: the square root of 49 is 7 because 7 x 7 = 49. Learn the squares up to 12 x 12 and you can place anything between them by eye, and the root button gives the exact answer - so the square root of 50 is a little over 7.")); __kws.extend(vec![("avoid".to_string(), __data3())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("decimal-to-percent"), py_tuple(vec![py_tuple(vec![py_str("decimal to percentage"), py_str("decimal to percent"), py_str("decimal as a percentage"), py_str("decimal as a percent"), py_str("turn a decimal"), py_str("convert a decimal"), py_str("into a percentage"), py_str("to a percentage"), py_str("as a percentage")]), py_tuple(vec![py_str("how"), py_str("percentage"), py_str("percent"), py_str("convert"), py_str("turn"), py_str("change"), py_str("do")])]), py_str("Multiply the decimal by 100 and put a % sign on it: 0.35 is 35% and 0.07 is 7%. Going the other way, divide by 100 - so 62% is 0.62 - which is just the decimal point moving two places.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("counting-up-change")); __args.push(py_tuple(vec![__data4(), __data5()])); __args.push(py_str("Count up from the price to the note rather than subtracting: on £12.40 out of a £20, add 60p to reach £13, then £7 to reach £20 - so the change is £7.60. Counting up stops at the round numbers on the way, which is why it is easier to do in your head and easier to check: what you counted back plus the price should land on the note.")); __kws.extend(vec![("avoid".to_string(), __data6()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tip-arithmetic")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tip"), py_str("tips"), py_str("tipping"), py_str("gratuity")]), __data7()])); __args.push(py_str("Ten percent is the easy part: move the decimal point one place, so a £46 bill is £4.60, and half of that again is five percent, so twelve and a half is £5.75. Check the bill for a service charge first, because that is the tip and you add nothing on top.")); __kws.extend(vec![("avoid".to_string(), __data8()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("percentage-change")); __args.push(py_tuple(vec![py_tuple(vec![py_str("percentage increase"), py_str("percentage decrease"), py_str("percentage change"), py_str("percent increase"), py_str("percent decrease"), py_str("percentage difference"), py_str("percentage rise"), py_str("percentage drop")]), py_tuple(vec![py_str("how"), py_str("calculate"), py_str("work out"), py_str("formula"), py_str("find"), py_str("between"), py_str("old"), py_str("new"), py_str("do")])])); __args.push(py_str("Take the difference, divide it by the number you started with, then multiply by 100: 40 to 50 is 10 more, 10 out of 40 is 0.25, so a 25% rise. To add a percentage on, multiply by 1 plus the rate - 80 plus 20% is 80 x 1.2 = 96 - and to take one off, multiply by 1 minus the rate.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("maths-from-scratch")); __args.push(py_tuple(vec![__data9(), __data10()])); __args.push(py_str("Numbers are rules you can look up, not a talent. A fraction is a part of a whole: the bottom is the size of the piece and the top is how many you have, so 2/3 of 12 is 12 / 3 = 4, then 4 x 2 = 8. To add fractions, make the bottoms match first - 1/2 + 1/3 is 3/6 + 2/6 = 5/6 - and a percentage is per hundred, so 10% is the number with a zero knocked off, and 20% off 60 is 12 off, so 48. For times tables, learn 2, 5 and 10 first and double for the 4s and 8s.")); __kws.extend(vec![("avoid".to_string(), __data11()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _a_bracket_is_a_sum(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line about a bracket is about the sum rather than the mark.");
    said = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plain"), py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    name = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said.clone()) {
    break;
    }
    }
    return call_builtin("bool", &[mcall(&g__BRACKET_MATHS(), "search", &[mcall(&said.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.schoolmaths".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_BRACKET_MATHS" => Some(g__BRACKET_MATHS()),
            "_a_bracket_is_a_sum" => Some(py_func("_a_bracket_is_a_sum", |args: &[Py]| _a_bracket_is_a_sum(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_BRACKET_MATHS" => s__BRACKET_MATHS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.schoolmaths", module_obj);
    pyrt::xrefs::reg_fn("knowledge.schoolmaths", "_a_bracket_is_a_sum", py_func("_a_bracket_is_a_sum", |args: &[Py]| _a_bracket_is_a_sum(args)));
    pyrt::xrefs::reg_access("knowledge.schoolmaths", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.schoolmaths", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.schoolmaths", "_BRACKET_MATHS", g__BRACKET_MATHS, s__BRACKET_MATHS);
    pyrt::xrefs::reg_access("knowledge.schoolmaths", "entry", g_entry, s_entry);
}
