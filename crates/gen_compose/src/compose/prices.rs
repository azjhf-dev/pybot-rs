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


thread_local! { static G_A_BARE_BUDGET_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BUDGET_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BUDGET_IS_THEIR_OWN_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FAIRNESS_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_JUDGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_NEARBY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BALANCE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BALANCE_CARRIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_SAID_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_ASK_HEADLESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_OF_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_OF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORTH_AN_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORTH_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_BARE_BUDGET_JUDGEMENT() -> Py { ensure_init(); G_A_BARE_BUDGET_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_BUDGET_JUDGEMENT(v: Py) { ensure_init(); G_A_BARE_BUDGET_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BUDGET_ASK() -> Py { ensure_init(); G_A_BUDGET_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BUDGET_ASK(v: Py) { ensure_init(); G_A_BUDGET_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BUDGET_IS_THEIR_OWN_SUM() -> Py { ensure_init(); G_A_BUDGET_IS_THEIR_OWN_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BUDGET_IS_THEIR_OWN_SUM(v: Py) { ensure_init(); G_A_BUDGET_IS_THEIR_OWN_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FAIRNESS_JUDGEMENT() -> Py { ensure_init(); G_A_FAIRNESS_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FAIRNESS_JUDGEMENT(v: Py) { ensure_init(); G_A_FAIRNESS_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_JUDGED() -> Py { ensure_init(); G_A_JOB_JUDGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_JUDGED(v: Py) { ensure_init(); G_A_JOB_JUDGED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_CUE() -> Py { ensure_init(); G_A_MONEY_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_CUE(v: Py) { ensure_init(); G_A_MONEY_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_NEARBY() -> Py { ensure_init(); G_A_MONEY_NEARBY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_NEARBY(v: Py) { ensure_init(); G_A_MONEY_NEARBY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE_WORD() -> Py { ensure_init(); G_A_PRICE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_WORD(v: Py) { ensure_init(); G_A_PRICE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_BALANCE_ASK() -> Py { ensure_init(); G_BALANCE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BALANCE_ASK(v: Py) { ensure_init(); G_BALANCE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BALANCE_CARRIER() -> Py { ensure_init(); G_BALANCE_CARRIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BALANCE_CARRIER(v: Py) { ensure_init(); G_BALANCE_CARRIER.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_SAID_BEFORE() -> Py { ensure_init(); G_FOOD_SAID_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_SAID_BEFORE(v: Py) { ensure_init(); G_FOOD_SAID_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_WORDS() -> Py { ensure_init(); G_JOB_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_WORDS(v: Py) { ensure_init(); G_JOB_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_JUDGEMENT() -> Py { ensure_init(); G_MONEY_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_JUDGEMENT(v: Py) { ensure_init(); G_MONEY_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PRICE() -> Py { ensure_init(); G_NOT_A_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PRICE(v: Py) { ensure_init(); G_NOT_A_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_ASK() -> Py { ensure_init(); G_PRICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK(v: Py) { ensure_init(); G_PRICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_ASK_HEADLESS() -> Py { ensure_init(); G_PRICE_ASK_HEADLESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK_HEADLESS(v: Py) { ensure_init(); G_PRICE_ASK_HEADLESS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_JUDGEMENT() -> Py { ensure_init(); G_PRICE_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_JUDGEMENT(v: Py) { ensure_init(); G_PRICE_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_OF_A_JOB() -> Py { ensure_init(); G_PRICE_OF_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_OF_A_JOB(v: Py) { ensure_init(); G_PRICE_OF_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_OF_A_THING() -> Py { ensure_init(); G_PRICE_OF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_OF_A_THING(v: Py) { ensure_init(); G_PRICE_OF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WORTH_AN_AMOUNT() -> Py { ensure_init(); G_WORTH_AN_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORTH_AN_AMOUNT(v: Py) { ensure_init(); G_WORTH_AN_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_WORTH_IT() -> Py { ensure_init(); G_WORTH_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORTH_IT(v: Py) { ensure_init(); G_WORTH_IT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A price is asked in many more ways than the live-price row lists.\n\n`fallback.LIVE_KINDS` has had an honest note for a live price for a long time - \"I can't look up\nlive prices, sorry. The seller's site or a price comparison site will have today's figure\" - and\nthree phrasings reached it. Everything else fell through to the search hedge, so a person asking\n\"what does a tattoo cost\" was told to search, one branch away from a reply that says what the bot\ncannot see and where the figure is.\n\nThe reading is here rather than in `fallback.py` because that file is at the world's size limit and\na reader that has to be added to it is a reader that cannot be filed.\n\nThree shapes, and what each keeps off:\n- `PRICE_ASK`, the ask itself in the words people use for it;\n- a judgement on an amount (`is that normal`, `is that a fair price`), which needs money in the\n  line or in the one before, because \"is that normal\" alone is as often about a child's speech;\n- a job, which no page prices (\"two quotes\"), rather than a thing on sale.\n");
    s_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:do|does|did|would|will|should|might)\\b[^?.!,]{0,36}?\\bcost|\\bwhat(?:'s|s)?\\s+(?:the\\s+)?(?:cost|price|going\\s+rate|damage|ballpark)\\b|\\bhow\\s+much\\s+(?:does|do|would|will|should|might|can)\\b[^?.!,]{0,36}?\\b(?:cost|charge|set\\s+me\\s+back|come\\s+to)\\b|\\bhow\\s+(?:much|expensive)\\s+(?:is|are|was|were)\\b|\\bhow\\s+much\\s+(?:should|shall|ought\\s+to)\\b[^?.!,]{0,24}?\\b(?:spend|pay)\\b|\\bhow\\s+much\\s+(?:do|would|will)\\s+(?:i|we|you)\\s+(?:need\\s+to\\s+)?(?:spend|pay|budget|expect)\\b|\\bhow\\s+much\\s+for\\s+(?:a|an|one|the)\\b|\\b(?:fair|normal|reasonable|typical|sensible|going|right)\\s+(?:price|rate|money|cost)\\b|\\bhow\\s+much\\b[^?.!,]{0,16}\\b(?:roughly|in\\s+total|altogether|all\\s+in)\\b|\\b(?:roughly|about)\\s+how\\s+much\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ASK_HEADLESS(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|but\\s+)?(?:ow\\s+much|much)\\s+(?:is|are|was|were|do|does|would)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_OF_A_JOB(py_tuple(vec![py_str("I can't price a job from here, sorry - it comes down to who does it and what they find when they open it up. Two quotes will give you a real range, and I can help you weigh them up."), py_str("No figure from me on a job like that, sorry: it is a quote rather than a price list. Ring two firms and compare what each one covers, not just the total.")]));
    s_A_JOB_JUDGED(py_tuple(vec![py_str("A quote is only normal against another one: ring a second firm for the same job and compare what each one covers, not just the total, because who does it and what they find when they open it up is why two figures for the same job come out apart."), py_str("What tells you whether that figure is normal is a second quote for the same job, in writing, compared on what each one includes rather than the total alone. The range moves with who does it and what they find, so the second quote is the only real test of the first.")]));
    s_JOB_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nplumber plumbing builder builders decorator decorating electrician electrics joiner carpenter\nplasterer tiler roofer handyman mechanic garage servicing fitted fitting installation install\nlabour quote quotes quoted repair repairs service gardener locksmith removals surveyor mot\nfix fixing fixed mend mending rewire rewiring replumb\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_OF_A_THING(py_tuple(vec![py_str("What decides it is the same thing priced in more than one place, and what each one includes: two or three quotes for the same job tell you whether a figure is normal. Today's prices move with where you are, so I can't see them from here."), py_str("Two or three places, the same thing, and compare what each one includes rather than the total alone - that is what answers whether a price is fair. The figures themselves move with where you are and when you go, so they are not mine to see.")]));
    s_PRICE_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\bis\\s+(?:that|this|it)\\s+(?:about\\s+)?(?:fair|normal|reasonable|right|about\\s+right|steep|a\\s+lot|too\\s+(?:much|dear|expensive))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FAIRNESS_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:fair|normal|reasonable|typical|sensible|going|right|steep|good|ok|decent)\\s+(?:price|prices|rate|rates|quote|quotation|cost|money|fee|charge)\\b|\\b(?:that|this|it)\\s+(?:about\\s+)?right\\b[^?.!]{0,14}\\b(?:price|pay|paid|charge)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|isn'?t|aren'?t|was|were)\\s+(?:that|this|it|they|those|these)(?:\\s+[a-z']+){0,2}\\s+(?:so|very|really|too|quite|pretty)?\\s*(?:expensive|pricey|dear|overpriced|costly|cheap|good\\s+value)\\b|\\b(?:that|this|it|they|those|these)\\s+(?:sounds|seems|looks)\\s+(?:so\\s+|very\\s+|really\\s+)?(?:expensive|pricey|dear|steep|a\\s+lot|overpriced)\\b|\\b(?:expensive|pricey|cheap)\\s+(?:to|for)\\s+(?:fix|repair|mend|replace|service|run|insure|install|fit|clean|heat)\\b|(?:[\\u00a3$\\u20ac]\\s?\\d+|\\b\\d+\\s?(?:pounds?|quid|euros?|dollars?|bucks))\\s+(?:a\\s+lot|too\\s+much|expensive|steep)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_CUE(stdlib_call("re", "compile", &[py_str("[\\u00a3$\\u20ac]|\\b\\d{2,}\\b|\\b(?:quid|quotes?|quoted|charged|charges?|price|prices|cost|costs|bill|invoice|fee|fees|paid|pay)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOOD_SAID_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:flour|salt|sugar|bread|loaf|loaves|dough|batter|yeast|rice|pasta|butter|milk|eggs?|cheese|mince|meat|chicken|beef|pork|lamb|fish|potato(?:es)?|onions?|carrots?|tomato(?:es)?|vegetables?|stock|oats|noodles|lentils|beans|yoghurt|yogurt|sauce|spices?|pepper|garlic|cake|biscuits?|pastry|porridge|soup|curry|roast|recipe|oven|bake|baking|loaf tin)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PRICE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:cost|costs|price|prices|priced|charge|charges|charged|quid|cheapest|expensive|cheaper|pound|pounds|dollars?|euros?|fee|bill)\\b|[\\u00a3$\\u20ac]")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:dose|dosage|ml|millilitres?|mg|tablets?|capsules?|spoons?|teaspoons?|calpol|paracetamol|ibuprofen|nurofen|medicine|medication|painkillers|insulin|vitamin\\s+[a-z])\\b|\\bhow\\s+much\\s+(?:longer|time|sleep|salt|sugar|fibre|protein|water|sun|light|noise|space|of)\\b|\\bhow\\s+(?:much|many)\\s+(?:is|are|of)\\s+(?:a|an|one|two|\\d+)?\\s*(?:grams?|grammes?|kilos?|ounces?|cups?|tablespoons?|handfuls?|portions?|servings?)\\b\\s*(?:[?.!]|$)|\\b(?:rice|pasta|mince|potato(?:es)?|meat|chicken|cheese|flour|sugar|butter|milk|lentils|beans|oats|noodles|batter|yoghurt|yogurt)\\b[^?.!,]{0,18}\\b(?:each|per\\s+person|per\\s+head|for\\s+(?:two|three|four|five|six|seven|eight|\\d+))\\b|\\bfeed(?:ing)?\\b[^?.!,]{0,30}\\b(?:family|families|kids|children|household|of us|for us|four|five)\\b|\\bwhat (?:does|do) (?:a|the) (?:week|month)(?:'s)? (?:food|shop|groceries|shopping)\\b|\\b(?:should|shall|can|could|do|does)\\b[^?.!,]{0,24}\\b(?:save|saving|savings|put\\s+(?:away|by|aside)|overpay|pay\\s+off|pay\\s+down)\\b|\\bis\\s+(?:that|this|it)\\s+(?:about\\s+)?(?:fair|right)\\s+(?:on|of|to|for|that)\\b|\\bfair\\s+enough\\b|\\b(?:you|u)\\s+charge\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BALANCE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\s+(?:is|are|was|were|have|has)\\b[^?.!,]{0,32}?\\b(?:left|remaining)\\b|\\bhow\\s+much\\s+(?:is|are|was|were)\\s+(?:on|in)\\s+(?:my|our|the|your|their)\\b|\\b(?:left|remaining)\\s+(?:on|in)\\s+(?:my|our|the|your)\\b|\\bmoney\\s+(?:left|remaining)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BALANCE_CARRIER(stdlib_call("re", "compile", &[py_str("\\b(?:card|cards|voucher|vouchers|account|balance|wallet|gift|pass|pot|points|credit)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BUDGET_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|would|will)\\b[^?.!,]{0,14}?(?:[\\u00a3$\\u20ac]\\s?)?\\d[\\d,]*(?:\\.\\d+)?\\s*(?:a|per|/)\\s*(?:month|week|fortnight|year)\\b[^?.!,]{0,28}\\b(?:enough|doable|affordable|liveable|livable|realistic|feasible|manageable|possible|ok|okay|alright)\\b|\\b(?:is|would|will)\\s+(?:it|that|this)\\s+(?:be\\s+)?(?:doable|affordable|enough|realistic|manageable|feasible)\\b[^?.!,]{0,14}?(?:[\\u00a3$\\u20ac]\\s?)?\\d[\\d,]*\\s*(?:a|per)\\s*(?:month|week|fortnight|year)\\b|\\b(?:is|would|will)\\b[^?.!,]{0,14}?\\d+(?:\\.\\d+)?\\s?k\\b[^?.!,]{0,28}\\b(?:enough|doable|affordable|liveable|livable|realistic|feasible|manageable)\\b|\\bcan\\s+(?:i|we|you)\\s+(?:do\\s+(?:it|this|that)|live|manage|afford|survive|cope|get\\s+by)\\b[^?.!,]{0,20}?\\d[\\d,]*\\s*(?:a|per)\\s*(?:month|week|fortnight|year|day)\\b|\\blive\\s+on\\s+(?:[\\u00a3$\\u20ac]\\s?)?\\d[\\d,]*\\s*(?:a|per)\\s*(?:week|month|year)\\b|\\benough\\s+to\\s+live\\s+on\\b|\\bget\\s+by\\s+on\\s+(?:[\\u00a3$\\u20ac]\\s?)?\\d[\\d,]*\\s*(?:a|per)\\s*(?:week|month)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORTH_AN_AMOUNT(stdlib_call("re", "compile", &[py_str("\\bworth\\s+(?:[\\u00a3$\\u20ac]\\s?\\d{2,}|\\d+(?:\\.\\d+)?\\s*(?:pounds?|quid|euros?|dollars?|bucks)\\b|\\d{3,}\\b)|\\bworth\\s+(?:the|that|this|its|their|his)\\s+(?:price|cost)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORTH_IT(stdlib_call("re", "compile", &[py_str("\\b(?:are|is|was|were)\\s+(?:they|those|these|it|that|this)\\s+worth\\s+it\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_BUDGET_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:is|would|will)\\s+(?:that|this|it)\\s+(?:be\\s+)?(?:doable|affordable|realistic|enough|manageable|feasible|possible|ok|okay|alright)\\b|\\b(?:is|would|will)\\s+(?:[\\u00a3$\\u20ac]\\s?)?\\d[\\d,]*\\s+(?:be\\s+)?(?:doable|affordable|enough|realistic|manageable|feasible|possible)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_NEARBY(stdlib_call("re", "compile", &[py_str("[\\u00a3$\\u20ac]|\\b\\d{3,}\\b|\\b(?:money|quid|price|prices|cost|costs|pay|paid|paying|pounds?|dollars?|euros?|bucks|cheaper|cheap|expensive|value)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BUDGET_IS_THEIR_OWN_SUM(py_tuple(vec![py_str("Housing, council tax and energy are the three that decide whether a month stretches, and the first two are fixed for the year - your council's site has your band and what it comes to. Add those, take them off the figure, and what is left is what has to cover food and travel. I can't see your bills from here, so tell me if the rent is already inside that figure."), py_str("Most of a month goes on housing, council tax, energy and food. The first three are written down somewhere - the band is on your council's site and the energy figure is on the bill - and food you can judge from one week's shop. Put those against it and the rest is arithmetic; the bills are the part I can't see, so I can't do that sum from here.")]));
}
pub fn _a_balance_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("True when the line asks what is already on a card or in an account, not what one costs.");
    return call_builtin("bool", &[{ let __b = mcall(&g_BALANCE_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_BALANCE_CARRIER(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bounded(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    return stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])"))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _has(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(mcall(&_bounded(&[phrase.clone()]), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_price_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut options: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut asked: Py = Py::None;
    let mut judged: Py = Py::None;
    let mut worth: Py = Py::None;
    let _ = py_str("The honest note for a price, or None when the line is not asking for one.\n\n    `phrases` are the wordings `fallback.LIVE_KINDS` already lists for the row, `before` is the\n    person's previous line (a judgement usually comes one turn after the amount), `options` is\n    that row's own pool.\n    ");
    if py_truthy(&{ let __b = mcall(&g_NOT_A_PRICE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _a_balance_ask(&[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_FOOD_SAID_BEFORE(), "search", &[before.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PRICE_WORD(), "search", &[clean.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_BUDGET_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_BARE_BUDGET_JUDGEMENT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_MONEY_NEARBY(), "search", &[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }) {
    return g_A_BUDGET_IS_THEIR_OWN_SUM();
    }
    judged = call_builtin("bool", &[{ let __b = mcall(&g_MONEY_JUDGEMENT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PRICE_JUDGEMENT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_FAIRNESS_JUDGEMENT(), "search", &[clean.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    worth = { let __b = call_builtin("bool", &[mcall(&g_WORTH_AN_AMOUNT(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = mcall(&g_WORTH_IT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MONEY_NEARBY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_MONEY_NEARBY(), "search", &[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }]).unwrap_or_else(|e| exc::raise(e)) } };
    asked = { let __b = _has(&[clean.clone(), phrases.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PRICE_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = worth.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PRICE_ASK_HEADLESS(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = judged.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MONEY_CUE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MONEY_CUE(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MONEY_JUDGEMENT(), "search", &[clean.clone()]) } } } } } } } } } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _has(&[clean.clone(), g_JOB_WORDS()]); if py_truthy(&__b) { __b } else { { let __b = judged.clone(); if !py_truthy(&__b) { __b } else { _has(&[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, g_JOB_WORDS()]) } } } }) {
    return if py_truthy(&judged.clone()) { g_A_JOB_JUDGED() } else { g_PRICE_OF_A_JOB() };
    }
    if py_truthy(&{ let __b = judged.clone(); if py_truthy(&__b) { __b } else { worth.clone() } }) {
    return g_PRICE_OF_A_THING();
    }
    return options.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.prices".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BARE_BUDGET_JUDGEMENT" => Some(g_A_BARE_BUDGET_JUDGEMENT()),
            "A_BUDGET_ASK" => Some(g_A_BUDGET_ASK()),
            "A_BUDGET_IS_THEIR_OWN_SUM" => Some(g_A_BUDGET_IS_THEIR_OWN_SUM()),
            "A_FAIRNESS_JUDGEMENT" => Some(g_A_FAIRNESS_JUDGEMENT()),
            "A_JOB_JUDGED" => Some(g_A_JOB_JUDGED()),
            "A_MONEY_CUE" => Some(g_A_MONEY_CUE()),
            "A_MONEY_NEARBY" => Some(g_A_MONEY_NEARBY()),
            "A_PRICE_WORD" => Some(g_A_PRICE_WORD()),
            "BALANCE_ASK" => Some(g_BALANCE_ASK()),
            "BALANCE_CARRIER" => Some(g_BALANCE_CARRIER()),
            "FOOD_SAID_BEFORE" => Some(g_FOOD_SAID_BEFORE()),
            "JOB_WORDS" => Some(g_JOB_WORDS()),
            "MONEY_JUDGEMENT" => Some(g_MONEY_JUDGEMENT()),
            "NOT_A_PRICE" => Some(g_NOT_A_PRICE()),
            "PRICE_ASK" => Some(g_PRICE_ASK()),
            "PRICE_ASK_HEADLESS" => Some(g_PRICE_ASK_HEADLESS()),
            "PRICE_JUDGEMENT" => Some(g_PRICE_JUDGEMENT()),
            "PRICE_OF_A_JOB" => Some(g_PRICE_OF_A_JOB()),
            "PRICE_OF_A_THING" => Some(g_PRICE_OF_A_THING()),
            "WORTH_AN_AMOUNT" => Some(g_WORTH_AN_AMOUNT()),
            "WORTH_IT" => Some(g_WORTH_IT()),
            "_a_balance_ask" => Some(py_func("_a_balance_ask", |args: &[Py]| _a_balance_ask(args))),
            "_bounded" => Some(py_func("_bounded", |args: &[Py]| _bounded(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "the_price_reply" => Some(py_func("the_price_reply", |args: &[Py]| the_price_reply(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BARE_BUDGET_JUDGEMENT" => s_A_BARE_BUDGET_JUDGEMENT(v),
            "A_BUDGET_ASK" => s_A_BUDGET_ASK(v),
            "A_BUDGET_IS_THEIR_OWN_SUM" => s_A_BUDGET_IS_THEIR_OWN_SUM(v),
            "A_FAIRNESS_JUDGEMENT" => s_A_FAIRNESS_JUDGEMENT(v),
            "A_JOB_JUDGED" => s_A_JOB_JUDGED(v),
            "A_MONEY_CUE" => s_A_MONEY_CUE(v),
            "A_MONEY_NEARBY" => s_A_MONEY_NEARBY(v),
            "A_PRICE_WORD" => s_A_PRICE_WORD(v),
            "BALANCE_ASK" => s_BALANCE_ASK(v),
            "BALANCE_CARRIER" => s_BALANCE_CARRIER(v),
            "FOOD_SAID_BEFORE" => s_FOOD_SAID_BEFORE(v),
            "JOB_WORDS" => s_JOB_WORDS(v),
            "MONEY_JUDGEMENT" => s_MONEY_JUDGEMENT(v),
            "NOT_A_PRICE" => s_NOT_A_PRICE(v),
            "PRICE_ASK" => s_PRICE_ASK(v),
            "PRICE_ASK_HEADLESS" => s_PRICE_ASK_HEADLESS(v),
            "PRICE_JUDGEMENT" => s_PRICE_JUDGEMENT(v),
            "PRICE_OF_A_JOB" => s_PRICE_OF_A_JOB(v),
            "PRICE_OF_A_THING" => s_PRICE_OF_A_THING(v),
            "WORTH_AN_AMOUNT" => s_WORTH_AN_AMOUNT(v),
            "WORTH_IT" => s_WORTH_IT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.prices", module_obj);
    pyrt::xrefs::reg_fn("compose.prices", "_a_balance_ask", py_func("_a_balance_ask", |args: &[Py]| _a_balance_ask(args)));
    pyrt::xrefs::reg_fn("compose.prices", "_bounded", py_func("_bounded", |args: &[Py]| _bounded(args)));
    pyrt::xrefs::reg_fn("compose.prices", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("compose.prices", "the_price_reply", py_func("the_price_reply", |args: &[Py]| the_price_reply(args)));
    pyrt::xrefs::reg_access("compose.prices", "A_BARE_BUDGET_JUDGEMENT", g_A_BARE_BUDGET_JUDGEMENT, s_A_BARE_BUDGET_JUDGEMENT);
    pyrt::xrefs::reg_access("compose.prices", "A_BUDGET_ASK", g_A_BUDGET_ASK, s_A_BUDGET_ASK);
    pyrt::xrefs::reg_access("compose.prices", "A_BUDGET_IS_THEIR_OWN_SUM", g_A_BUDGET_IS_THEIR_OWN_SUM, s_A_BUDGET_IS_THEIR_OWN_SUM);
    pyrt::xrefs::reg_access("compose.prices", "A_FAIRNESS_JUDGEMENT", g_A_FAIRNESS_JUDGEMENT, s_A_FAIRNESS_JUDGEMENT);
    pyrt::xrefs::reg_access("compose.prices", "A_JOB_JUDGED", g_A_JOB_JUDGED, s_A_JOB_JUDGED);
    pyrt::xrefs::reg_access("compose.prices", "A_MONEY_CUE", g_A_MONEY_CUE, s_A_MONEY_CUE);
    pyrt::xrefs::reg_access("compose.prices", "A_MONEY_NEARBY", g_A_MONEY_NEARBY, s_A_MONEY_NEARBY);
    pyrt::xrefs::reg_access("compose.prices", "A_PRICE_WORD", g_A_PRICE_WORD, s_A_PRICE_WORD);
    pyrt::xrefs::reg_access("compose.prices", "BALANCE_ASK", g_BALANCE_ASK, s_BALANCE_ASK);
    pyrt::xrefs::reg_access("compose.prices", "BALANCE_CARRIER", g_BALANCE_CARRIER, s_BALANCE_CARRIER);
    pyrt::xrefs::reg_access("compose.prices", "FOOD_SAID_BEFORE", g_FOOD_SAID_BEFORE, s_FOOD_SAID_BEFORE);
    pyrt::xrefs::reg_access("compose.prices", "JOB_WORDS", g_JOB_WORDS, s_JOB_WORDS);
    pyrt::xrefs::reg_access("compose.prices", "MONEY_JUDGEMENT", g_MONEY_JUDGEMENT, s_MONEY_JUDGEMENT);
    pyrt::xrefs::reg_access("compose.prices", "NOT_A_PRICE", g_NOT_A_PRICE, s_NOT_A_PRICE);
    pyrt::xrefs::reg_access("compose.prices", "PRICE_ASK", g_PRICE_ASK, s_PRICE_ASK);
    pyrt::xrefs::reg_access("compose.prices", "PRICE_ASK_HEADLESS", g_PRICE_ASK_HEADLESS, s_PRICE_ASK_HEADLESS);
    pyrt::xrefs::reg_access("compose.prices", "PRICE_JUDGEMENT", g_PRICE_JUDGEMENT, s_PRICE_JUDGEMENT);
    pyrt::xrefs::reg_access("compose.prices", "PRICE_OF_A_JOB", g_PRICE_OF_A_JOB, s_PRICE_OF_A_JOB);
    pyrt::xrefs::reg_access("compose.prices", "PRICE_OF_A_THING", g_PRICE_OF_A_THING, s_PRICE_OF_A_THING);
    pyrt::xrefs::reg_access("compose.prices", "WORTH_AN_AMOUNT", g_WORTH_AN_AMOUNT, s_WORTH_AN_AMOUNT);
    pyrt::xrefs::reg_access("compose.prices", "WORTH_IT", g_WORTH_IT, s_WORTH_IT);
}
