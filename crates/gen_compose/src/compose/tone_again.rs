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


thread_local! { static G_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOTH_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CASUAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CASUAL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRM_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORMAL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELLO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLITE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGN_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TONE_EXTRA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TURNED_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALREADY() -> Py { ensure_init(); G_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY(v: Py) { ensure_init(); G_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_BOTH_LINES() -> Py { ensure_init(); G_BOTH_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOTH_LINES(v: Py) { ensure_init(); G_BOTH_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CASUAL() -> Py { ensure_init(); G_CASUAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CASUAL(v: Py) { ensure_init(); G_CASUAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CASUAL_LINES() -> Py { ensure_init(); G_CASUAL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CASUAL_LINES(v: Py) { ensure_init(); G_CASUAL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRACTIONS() -> Py { ensure_init(); G_CONTRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRACTIONS(v: Py) { ensure_init(); G_CONTRACTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_ASKED() -> Py { ensure_init(); G_DATE_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_ASKED(v: Py) { ensure_init(); G_DATE_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRM() -> Py { ensure_init(); G_FIRM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRM(v: Py) { ensure_init(); G_FIRM.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRM_LINES() -> Py { ensure_init(); G_FIRM_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRM_LINES(v: Py) { ensure_init(); G_FIRM_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_FORMAL() -> Py { ensure_init(); G_FORMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORMAL(v: Py) { ensure_init(); G_FORMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_FORMAL_LINES() -> Py { ensure_init(); G_FORMAL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORMAL_LINES(v: Py) { ensure_init(); G_FORMAL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME() -> Py { ensure_init(); G_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); G_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_HELLO() -> Py { ensure_init(); G_HELLO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELLO(v: Py) { ensure_init(); G_HELLO.with(|c| *c.borrow_mut() = v); }
pub fn g_LESS() -> Py { ensure_init(); G_LESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LESS(v: Py) { ensure_init(); G_LESS.with(|c| *c.borrow_mut() = v); }
pub fn g_POLITE() -> Py { ensure_init(); G_POLITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLITE(v: Py) { ensure_init(); G_POLITE.with(|c| *c.borrow_mut() = v); }
pub fn g_POLITE_LINES() -> Py { ensure_init(); G_POLITE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLITE_LINES(v: Py) { ensure_init(); G_POLITE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_LINES() -> Py { ensure_init(); G_SHORT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_LINES(v: Py) { ensure_init(); G_SHORT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGN_OFF() -> Py { ensure_init(); G_SIGN_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGN_OFF(v: Py) { ensure_init(); G_SIGN_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT() -> Py { ensure_init(); G_SOFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT(v: Py) { ensure_init(); G_SOFT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_WORDS() -> Py { ensure_init(); G_SOFT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_WORDS(v: Py) { ensure_init(); G_SOFT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKED() -> Py { ensure_init(); G_THANKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKED(v: Py) { ensure_init(); G_THANKED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASK() -> Py { ensure_init(); G_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASK(v: Py) { ensure_init(); G_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TONE_EXTRA() -> Py { ensure_init(); G_TONE_EXTRA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TONE_EXTRA(v: Py) { ensure_init(); G_TONE_EXTRA.with(|c| *c.borrow_mut() = v); }
pub fn g_TURNED_OVER() -> Py { ensure_init(); G_TURNED_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TURNED_OVER(v: Py) { ensure_init(); G_TURNED_OVER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The draft the bot has just written, said again in the tone that was asked for.\n\n    i need to write to my landlord\n      \"Happy to write it for you. Tell me what you want them to know or do, and roughly what\n       tone, and I'll put a version together you can edit.\"\n    the boiler has been broken for 2 weeks\n      'Something like this works: \"Hi, the boiler has been broken for 2 weeks. Thanks.\"'\n    make it firm but polite\n      \"That's beyond what I know well, sorry. Searching for it will turn up better sources than I\n       can offer; tell me what you find and I'll help you weigh it up.\"\n\nNothing in that last reply is true: the ask is not a fact about the world and there is nothing to\nsearch for. It is an edit of words the bot itself printed a turn ago, and the person is not asking\nthe internet to make their message firmer - they are asking the bot. So the bot does it: the draft\ncomes back with the hedges out and the ask in, or one word shorter, or with the courtesy put in.\n\nWhat this reads, and what it keeps off, because a wrong draft is worse than the shrug it replaces:\n\n- the line has to be **tone only**. \"make it firm but polite\" names no subject; \"make the boiler\n  message firm\" does, and that is a drafting question for `draft.py`;\n- the reply just given has to **hold the draft**, in quotes. \"make it firmer\" three turns after a\n  recipe has no words to edit, and inventing a message is not an improvement;\n- a draft about a person still gets its tone changed: the edit is the tone, not the content.\n\nIt is consulted from `fallback_chain.py` before the pools, in place of the search hedge.\n");
    s_FIRM(call_builtin("frozenset", &[mcall(&py_str("\nfirm firmer firmest firmly assertive stronger strongest direct directly blunt blunter\napologetic unapologetic no-nonsense stern serious\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POLITE(call_builtin("frozenset", &[mcall(&py_str("\npolite politely courteous respectful nicer nice friendly friendlier warm warmer kinder kind\nsofter gentle gentler professional tactful diplomatic\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT(call_builtin("frozenset", &[mcall(&py_str("\nshorter short brief briefer concise tighter trim condensed\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FORMAL(call_builtin("frozenset", &[mcall(&py_str("\nformal officially\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CASUAL(call_builtin("frozenset", &[mcall(&py_str("\ncasual chatty relaxed informal\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME(stdlib_call("re", "compile", &[py_str("\\b(?:make|makes|making|say|sound|sounds|sounding|write|rewrite|word|put|say it|said|again|version|tone|it|that|this|a|an|the|bit|more|less|much|very|really|far|too|could|can|would|will|you|i|me|my|please|and|but|so|just|to|of|in|on|as|is|be|need|want|like|something|text|message|email|note|letter|reply|draft|line|lines|words|one|ones|now|then|instead|also|still|shorter|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOFT(py_tuple(vec![py_tuple(vec![py_str("\\bi\\s+was\\s+(?:just\\s+)?wondering\\s+(?:if|whether)\\b"), py_str("I need to know")]), py_tuple(vec![py_str("\\bi\\s+just\\s+wanted\\s+to\\s+(?:ask|check|say)\\b"), py_str("I am writing about")]), py_tuple(vec![py_str("\\bwould\\s+it\\s+be\\s+possible\\s+to\\b"), py_str("please")]), py_tuple(vec![py_str("\\bif\\s+you\\s+(?:get|have)\\s+a\\s+(?:chance|moment)\\b"), py_str("")]), py_tuple(vec![py_str("\\bwhen\\s+you\\s+(?:get|have)\\s+a\\s+(?:chance|moment)\\b"), py_str("")]), py_tuple(vec![py_str("\\bat\\s+your\\s+earliest\\s+convenience\\b"), py_str("")]), py_tuple(vec![py_str("\\bi\\s+hope\\s+you\\s+(?:don't|do not)\\s+mind\\b"), py_str("")]), py_tuple(vec![py_str("\\bi\\s+know\\s+you(?:'re| are)\\s+busy\\b"), py_str("")]), py_tuple(vec![py_str("\\bsorry\\s+to\\s+(?:bother|trouble|disturb)\\s+you\\b"), py_str("")]), py_tuple(vec![py_str("\\bif\\s+possible\\b"), py_str("")]), py_tuple(vec![py_str("\\bif\\s+you\\s+wouldn't\\s+mind\\b"), py_str("please")]), py_tuple(vec![py_str("\\bjust\\b"), py_str("")])]));
    s_SOFT_WORDS(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SOFT()) { let __unpk8623 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk8623[0].clone(); let mut _u: Py = __unpk8623[1].clone(); __out.push(one.clone()); } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_ASK(stdlib_call("re", "compile", &[py_str("\\?|\\bplease\\b|\\bcan you\\b|\\bcould you\\b|\\bwould you\\b|\\blet me know\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DATE_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:by|before|on|for)\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|the\\s+\\d+|\\d+\\s*(?:st|nd|rd|th)?\\b|tomorrow|today|the\\s+end\\s+of\\s+the\\s+week|next\\s+week|this\\s+week|end\\s+of\\s+month)")]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKED(stdlib_call("re", "compile", &[py_str("\\b(?:thank(?:s|\\s+you)|cheers|much\\s+appreciated|grateful)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRACTIONS(py_tuple(vec![py_tuple(vec![py_str("\\bi'm\\b"), py_str("I am")]), py_tuple(vec![py_str("\\bi've\\b"), py_str("I have")]), py_tuple(vec![py_str("\\bi'd\\b"), py_str("I would")]), py_tuple(vec![py_str("\\bdon't\\b"), py_str("do not")]), py_tuple(vec![py_str("\\bdoesn't\\b"), py_str("does not")]), py_tuple(vec![py_str("\\bdidn't\\b"), py_str("did not")]), py_tuple(vec![py_str("\\bit's\\b"), py_str("it is")]), py_tuple(vec![py_str("\\bthat's\\b"), py_str("that is")]), py_tuple(vec![py_str("\\bcan't\\b"), py_str("cannot")]), py_tuple(vec![py_str("\\bwon't\\b"), py_str("will not")]), py_tuple(vec![py_str("\\bwe're\\b"), py_str("we are")]), py_tuple(vec![py_str("\\bthey're\\b"), py_str("they are")]), py_tuple(vec![py_str("\\bhavent\\b"), py_str("have not")]), py_tuple(vec![py_str("\\bhasnt\\b"), py_str("has not")]), py_tuple(vec![py_str("\\bdont\\b"), py_str("do not")]), py_tuple(vec![py_str("\\bcant\\b"), py_str("cannot")]), py_tuple(vec![py_str("\\bim\\b"), py_str("I am")]), py_tuple(vec![py_str("\\bive\\b"), py_str("I have")])]));
    s_HELLO(stdlib_call("re", "compile", &[py_str("^(?:hi|hey|hello|dear)\\b[ ,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGN_OFF(stdlib_call("re", "compile", &[py_str("\\b(?:thanks|thank\\s+you|cheers|kind\\s+regards|many\\s+thanks|regards)[\\s.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LESS(stdlib_call("re", "compile", &[py_str("\\bless\\s+([a-z']+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TURNED_OVER(py_dict(vec![(py_str("blunt"), py_str("polite")), (py_str("blunter"), py_str("polite")), (py_str("rude"), py_str("polite")), (py_str("abrupt"), py_str("polite")), (py_str("aggressive"), py_str("polite")), (py_str("harsh"), py_str("polite")), (py_str("formal"), py_str("casual")), (py_str("stiff"), py_str("casual")), (py_str("wordy"), py_str("short")), (py_str("long"), py_str("short")), (py_str("soft"), py_str("firm")), (py_str("soppy"), py_str("firm")), (py_str("apologetic"), py_str("firm")), (py_str("wordy"), py_str("short"))]));
    s_TONE_EXTRA(call_builtin("frozenset", &[op_bitor(&call_builtin("set", &[g_TURNED_OVER()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[mcall(&g_TURNED_OVER(), "values", &[])]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRM_LINES(py_tuple(vec![py_str("Firm is the same facts with the apology and the \"just\" taken out: \"{draft}\" That leads with the fact and ends with the ask. Give me the date you want an answer by and I will put it in."), py_str("Here it is with the soft words out: \"{draft}\" The boiler and the two weeks do the arguing; a firm message only needs a date on the end - say one and I will write it in."), py_str("Stripped back: \"{draft}\" No apology, no \"whenever you get a chance\", and the ask is the last sentence. Tell me the date you want to name and I will put it in for you.")]));
    s_POLITE_LINES(py_tuple(vec![py_str("One courtesy does it: \"{draft}\" That is the same ask with a please and a thank you round it, which is all politeness is on a message like this."), py_str("Politer, same content: \"{draft}\" The please goes on the ask and the thanks on the end - any more than that and the ask gets lost in it."), py_str("With the courtesy in: \"{draft}\" Say the thing plainly and put the please and thank you around it; that reads better than a paragraph of softening.")]));
    s_SHORT_LINES(py_tuple(vec![py_str("Cut to the two sentences that do the work: \"{draft}\" A message like this only needs what is wrong and what you want - say if you want the date in it and I will add one line more."), py_str("Shorter, same ask: \"{draft}\" Everything after the first sentence and the ask was doing nothing, so it is gone.")]));
    s_FORMAL_LINES(py_tuple(vec![py_str("Formal is no contractions and the full form of address: \"{draft}\" If you have a name and an address for them I will put those in instead."), py_str("In the formal register: \"{draft}\" Same facts, no shortened words, and the salutation and sign-off to match.")]));
    s_CASUAL_LINES(py_tuple(vec![py_str("Casual keeps the facts and drops the ceremony: \"{draft}\" That is how you would say it to a neighbour rather than a council."), py_str("Loosened off: \"{draft}\" Nothing in the meaning changed - only the greeting and the sign-off.")]));
    s_BOTH_LINES(py_tuple(vec![py_str("Firm but polite is the ask in plain words with the courtesy kept: \"{draft}\" The fact first, then please, then the date. Give me the date and I will put it in."), py_str("Both at once: \"{draft}\" Nothing soft, nothing rude - the facts, a please, and the ask at the end. Say the date you want and it is ready to send.")]));
    s_ALREADY(py_dict(vec![(py_str("firm"), py_str("That draft already reads firm: \"{draft}\" The facts are there and the ask is in it. The one thing it is short of is the date you want an answer by.")), (py_str("polite"), py_str("It already reads politely: \"{draft}\" There is a request in it and no edge. If you want it warmer still, say how you know them and I will open with that.")), (py_str("short"), py_str("That is as short as it goes without losing the facts: \"{draft}\" The date and the thing that is wrong are the parts that have to stay.")), (py_str("formal"), py_str("That is already the formal version: \"{draft}\" What it still needs is their name and address at the top rather than a greeting you make up.")), (py_str("casual"), py_str("That is already casual enough: \"{draft}\" Nothing in it sounds like a letter."))]));
}
pub fn _clean(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    return stdlib_call("re", "sub", &[py_str("[^a-z' -]"), py_str(" "), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _tones(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut name: Py = Py::None;
    let mut turned: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The tones the line names, in the order they are read.");
    turned = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_LESS(), "findall", &[low.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_TURNED_OVER(), &word.clone()))) {
    let _ = mcall(&turned.clone(), "add", &[geti(&g_TURNED_OVER(), &word.clone())]);
    }
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("firm"), py_str("polite"), py_str("short"), py_str("formal"), py_str("casual")])) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&turned.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("firm"), g_FIRM()]), py_tuple(vec![py_str("polite"), g_POLITE()]), py_tuple(vec![py_str("short"), g_SHORT()]), py_tuple(vec![py_str("formal"), g_FORMAL()]), py_tuple(vec![py_str("casual"), g_CASUAL()])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    name = __unpk4[0].clone();
    group = __unpk4[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&group.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&found.clone(), &name.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[name.clone()]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _is_tone_only(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut left: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when nothing but a tone is named: no subject for anybody to look up.");
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FIRM(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_POLITE(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SHORT(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FORMAL(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CASUAL(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_TONE_EXTRA(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_FRAME(), "fullmatch", &[w.clone()]))) } } } } } } } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    return Py::Bool(!py_truthy(&left.clone()));
    Py::None
}
pub fn _the_draft(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The message the bot handed over, or \"\" when the last reply held none.");
    text = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("\"([^\"]{8,400})\""), py_str("\\u201c([^\\u201d]{8,400})\\u201d")]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = stdlib_call("re", "search", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_draft_near(args: &[Py]) -> Py {
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
    let mut draft: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The draft in the last reply, or in the one before it.\n\n    A person says thank you between the draft and the change of tone (\"cheers - can you make it\n    shorter\"), and the words to edit are still exactly as the bot wrote them. Older than that and\n    the chat has moved on, so nothing is picked up.\n    ");
    turns = slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(2)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    draft = _the_draft(&[mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")])]);
    if py_truthy(&draft.clone()) {
    return draft.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _firm(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut draft: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "draft") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: draft"))) };
    let mut instead: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The same message with the apology, the hedge and the \"whenever\" taken out.");
    said = draft.clone();
    let __iter1 = py_iter_or_raise(&g_SOFT());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    instead = __unpk2[1].clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(instead.clone()); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    said = mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    said = mcall(&mcall(&g_SIGN_OFF(), "sub", &[py_str(""), said.clone()]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_ASK(), "search", &[said.clone()])))) {
    said = op_add(&mcall(&said.clone(), "rstrip", &[py_str(" .;,")]), &py_str(". Please confirm when it will be done"));
    }
    said = mcall(&said.clone(), "rstrip", &[py_str(" .;,")]);
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("?"), py_str("!"), py_str(".")])])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    Py::None
}
pub fn _polite(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut draft: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "draft") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: draft"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The same message with one courtesy on it: the ask is already there, the thanks are not.");
    said = mcall(&mcall(&g_SIGN_OFF(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&draft.clone(), "split", &[])])]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bplease\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { mcall(&g_THE_ASK(), "search", &[said.clone()]) } }) {
    said = mcall(&said.clone(), "rstrip", &[py_str(" .;,")]);
    if py_truthy(&mcall(&said.clone(), "endswith", &[py_str("?")])) {
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), "rstrip", &[]), &py_str(", please?"));
    } else {
    said = op_add(&said.clone(), &py_str(", please"));
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_ASK(), "search", &[said.clone()])))) {
    said = op_add(&mcall(&said.clone(), "rstrip", &[py_str(" .;,")]), &py_str(". Please let me know what happens next"));
    }
    }
    said = mcall(&said.clone(), "rstrip", &[py_str(" .;,")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THANKED(), "search", &[said.clone()])))) {
    said = op_add(&said.clone(), &if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("?"), py_str("!")])])) { py_str(" Thank you.") } else { py_str(". Thank you") });
    }
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("?"), py_str("!"), py_str(".")])])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    Py::None
}
pub fn _short(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut draft: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "draft") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: draft"))) };
    let mut asked: Py = Py::None;
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The first sentence and the ask, which is what a shorter version keeps.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.?!])\\s+"), mcall(&py_str(" "), "join", &[mcall(&draft.clone(), "split", &[])])]).unwrap_or_else(|e| exc::raise(e))) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&parts.clone(), &Py::Int(0));
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&g_THE_ASK(), "search", &[p.clone()])) {  __out.push(p.clone()); } } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[op_add(&py_list(vec![geti(&parts.clone(), &Py::Int(0))]), &slice_or_raise(&asked.clone(), &Py::None, &Py::Int(1), &Py::None))]);
    Py::None
}
pub fn _formal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut draft: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "draft") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: draft"))) };
    let mut instead: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    said = mcall(&py_str(" "), "join", &[mcall(&draft.clone(), "split", &[])]);
    let __iter1 = py_iter_or_raise(&g_CONTRACTIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    instead = __unpk2[1].clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(instead.clone()); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    said = mcall(&g_HELLO(), "sub", &[py_str("Dear Sir or Madam, "), said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "startswith", &[py_str("Dear")])))) {
    said = op_add(&py_str("Dear Sir or Madam, "), &said.clone());
    }
    said = mcall(&g_SIGN_OFF(), "sub", &[py_str("Yours faithfully."), said.clone()]);
    return mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    Py::None
}
pub fn _casual(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut draft: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "draft") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: draft"))) };
    let mut said: Py = Py::None;
    said = mcall(&g_HELLO(), "sub", &[py_str("Hi, "), mcall(&py_str(" "), "join", &[mcall(&draft.clone(), "split", &[])])]);
    said = mcall(&g_SIGN_OFF(), "sub", &[py_str("Cheers."), said.clone()]);
    return mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    Py::None
}
pub fn retone(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut c: Py = Py::None;
    let mut draft: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut tones: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The draft just given, said again in the tone this line asks for, or None.");
    draft = _the_draft_near(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&draft.clone()))) {
    return Py::None;
    }
    low = _clean(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_tone_only(&[low.clone()]))) } }) {
    return Py::None;
    }
    tones = _tones(&[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&tones.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&tones.clone(), &py_str("firm"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&tones.clone(), &py_str("polite"))) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![g_BOTH_LINES(), _polite(&[_firm(&[draft.clone()])])]), 2, None);
    pool = __unpk1[0].clone();
    draft = __unpk1[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&tones.clone(), &py_str("firm")))) {
    let __unpk2 = py_unpack(&py_tuple(vec![g_FIRM_LINES(), _firm(&[draft.clone()])]), 2, None);
    pool = __unpk2[0].clone();
    draft = __unpk2[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&tones.clone(), &py_str("short")))) {
    let __unpk3 = py_unpack(&py_tuple(vec![g_SHORT_LINES(), _short(&[draft.clone()])]), 2, None);
    pool = __unpk3[0].clone();
    draft = __unpk3[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&tones.clone(), &py_str("formal")))) {
    let __unpk4 = py_unpack(&py_tuple(vec![g_FORMAL_LINES(), _formal(&[draft.clone()])]), 2, None);
    pool = __unpk4[0].clone();
    draft = __unpk4[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&tones.clone(), &py_str("casual")))) {
    let __unpk5 = py_unpack(&py_tuple(vec![g_CASUAL_LINES(), _casual(&[draft.clone()])]), 2, None);
    pool = __unpk5[0].clone();
    draft = __unpk5[1].clone();
    } else {
    let __unpk6 = py_unpack(&py_tuple(vec![g_POLITE_LINES(), _polite(&[draft.clone()])]), 2, None);
    pool = __unpk6[0].clone();
    draft = __unpk6[1].clone();
    }
    }
    }
    }
    }
    draft = mcall(&mcall(&py_str(" "), "join", &[mcall(&draft.clone(), "split", &[])]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&draft.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&draft.clone(), "strip", &[py_str(" .;,")]), "lower", &[]), &mcall(&mcall(&_the_draft_near(&[memory.clone()]), "strip", &[py_str(" .;,")]), "lower", &[])))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("draft".to_string(), draft.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mcall(&g_ALREADY(), "get", &[geti(&tones.clone(), &Py::Int(0)), geti(&g_ALREADY(), &py_str("polite"))]), "format", args))(&__args) };
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    let mut random: Py = stdlib_module("random");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("draft".to_string(), draft.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mcall(&mcall(&random.clone(), "Random", &[seed.clone()]), "choice", &[pool.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.tone_again".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALREADY" => Some(g_ALREADY()),
            "BOTH_LINES" => Some(g_BOTH_LINES()),
            "CASUAL" => Some(g_CASUAL()),
            "CASUAL_LINES" => Some(g_CASUAL_LINES()),
            "CONTRACTIONS" => Some(g_CONTRACTIONS()),
            "DATE_ASKED" => Some(g_DATE_ASKED()),
            "FIRM" => Some(g_FIRM()),
            "FIRM_LINES" => Some(g_FIRM_LINES()),
            "FORMAL" => Some(g_FORMAL()),
            "FORMAL_LINES" => Some(g_FORMAL_LINES()),
            "FRAME" => Some(g_FRAME()),
            "HELLO" => Some(g_HELLO()),
            "LESS" => Some(g_LESS()),
            "POLITE" => Some(g_POLITE()),
            "POLITE_LINES" => Some(g_POLITE_LINES()),
            "SHORT" => Some(g_SHORT()),
            "SHORT_LINES" => Some(g_SHORT_LINES()),
            "SIGN_OFF" => Some(g_SIGN_OFF()),
            "SOFT" => Some(g_SOFT()),
            "SOFT_WORDS" => Some(g_SOFT_WORDS()),
            "THANKED" => Some(g_THANKED()),
            "THE_ASK" => Some(g_THE_ASK()),
            "TONE_EXTRA" => Some(g_TONE_EXTRA()),
            "TURNED_OVER" => Some(g_TURNED_OVER()),
            "_casual" => Some(py_func("_casual", |args: &[Py]| _casual(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_firm" => Some(py_func("_firm", |args: &[Py]| _firm(args))),
            "_formal" => Some(py_func("_formal", |args: &[Py]| _formal(args))),
            "_is_tone_only" => Some(py_func("_is_tone_only", |args: &[Py]| _is_tone_only(args))),
            "_polite" => Some(py_func("_polite", |args: &[Py]| _polite(args))),
            "_short" => Some(py_func("_short", |args: &[Py]| _short(args))),
            "_the_draft" => Some(py_func("_the_draft", |args: &[Py]| _the_draft(args))),
            "_the_draft_near" => Some(py_func("_the_draft_near", |args: &[Py]| _the_draft_near(args))),
            "_tones" => Some(py_func("_tones", |args: &[Py]| _tones(args))),
            "retone" => Some(py_func("retone", |args: &[Py]| retone(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALREADY" => s_ALREADY(v),
            "BOTH_LINES" => s_BOTH_LINES(v),
            "CASUAL" => s_CASUAL(v),
            "CASUAL_LINES" => s_CASUAL_LINES(v),
            "CONTRACTIONS" => s_CONTRACTIONS(v),
            "DATE_ASKED" => s_DATE_ASKED(v),
            "FIRM" => s_FIRM(v),
            "FIRM_LINES" => s_FIRM_LINES(v),
            "FORMAL" => s_FORMAL(v),
            "FORMAL_LINES" => s_FORMAL_LINES(v),
            "FRAME" => s_FRAME(v),
            "HELLO" => s_HELLO(v),
            "LESS" => s_LESS(v),
            "POLITE" => s_POLITE(v),
            "POLITE_LINES" => s_POLITE_LINES(v),
            "SHORT" => s_SHORT(v),
            "SHORT_LINES" => s_SHORT_LINES(v),
            "SIGN_OFF" => s_SIGN_OFF(v),
            "SOFT" => s_SOFT(v),
            "SOFT_WORDS" => s_SOFT_WORDS(v),
            "THANKED" => s_THANKED(v),
            "THE_ASK" => s_THE_ASK(v),
            "TONE_EXTRA" => s_TONE_EXTRA(v),
            "TURNED_OVER" => s_TURNED_OVER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.tone_again", module_obj);
    pyrt::xrefs::reg_fn("compose.tone_again", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_tones", py_func("_tones", |args: &[Py]| _tones(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_is_tone_only", py_func("_is_tone_only", |args: &[Py]| _is_tone_only(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_the_draft", py_func("_the_draft", |args: &[Py]| _the_draft(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_the_draft_near", py_func("_the_draft_near", |args: &[Py]| _the_draft_near(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_firm", py_func("_firm", |args: &[Py]| _firm(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_polite", py_func("_polite", |args: &[Py]| _polite(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_short", py_func("_short", |args: &[Py]| _short(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_formal", py_func("_formal", |args: &[Py]| _formal(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "_casual", py_func("_casual", |args: &[Py]| _casual(args)));
    pyrt::xrefs::reg_fn("compose.tone_again", "retone", py_func("retone", |args: &[Py]| retone(args)));
    pyrt::xrefs::reg_access("compose.tone_again", "ALREADY", g_ALREADY, s_ALREADY);
    pyrt::xrefs::reg_access("compose.tone_again", "BOTH_LINES", g_BOTH_LINES, s_BOTH_LINES);
    pyrt::xrefs::reg_access("compose.tone_again", "CASUAL", g_CASUAL, s_CASUAL);
    pyrt::xrefs::reg_access("compose.tone_again", "CASUAL_LINES", g_CASUAL_LINES, s_CASUAL_LINES);
    pyrt::xrefs::reg_access("compose.tone_again", "CONTRACTIONS", g_CONTRACTIONS, s_CONTRACTIONS);
    pyrt::xrefs::reg_access("compose.tone_again", "DATE_ASKED", g_DATE_ASKED, s_DATE_ASKED);
    pyrt::xrefs::reg_access("compose.tone_again", "FIRM", g_FIRM, s_FIRM);
    pyrt::xrefs::reg_access("compose.tone_again", "FIRM_LINES", g_FIRM_LINES, s_FIRM_LINES);
    pyrt::xrefs::reg_access("compose.tone_again", "FORMAL", g_FORMAL, s_FORMAL);
    pyrt::xrefs::reg_access("compose.tone_again", "FORMAL_LINES", g_FORMAL_LINES, s_FORMAL_LINES);
    pyrt::xrefs::reg_access("compose.tone_again", "FRAME", g_FRAME, s_FRAME);
    pyrt::xrefs::reg_access("compose.tone_again", "HELLO", g_HELLO, s_HELLO);
    pyrt::xrefs::reg_access("compose.tone_again", "LESS", g_LESS, s_LESS);
    pyrt::xrefs::reg_access("compose.tone_again", "POLITE", g_POLITE, s_POLITE);
    pyrt::xrefs::reg_access("compose.tone_again", "POLITE_LINES", g_POLITE_LINES, s_POLITE_LINES);
    pyrt::xrefs::reg_access("compose.tone_again", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("compose.tone_again", "SHORT_LINES", g_SHORT_LINES, s_SHORT_LINES);
    pyrt::xrefs::reg_access("compose.tone_again", "SIGN_OFF", g_SIGN_OFF, s_SIGN_OFF);
    pyrt::xrefs::reg_access("compose.tone_again", "SOFT", g_SOFT, s_SOFT);
    pyrt::xrefs::reg_access("compose.tone_again", "SOFT_WORDS", g_SOFT_WORDS, s_SOFT_WORDS);
    pyrt::xrefs::reg_access("compose.tone_again", "THANKED", g_THANKED, s_THANKED);
    pyrt::xrefs::reg_access("compose.tone_again", "THE_ASK", g_THE_ASK, s_THE_ASK);
    pyrt::xrefs::reg_access("compose.tone_again", "TONE_EXTRA", g_TONE_EXTRA, s_TONE_EXTRA);
    pyrt::xrefs::reg_access("compose.tone_again", "TURNED_OVER", g_TURNED_OVER, s_TURNED_OVER);
}
