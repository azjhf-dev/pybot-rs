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


thread_local! { static G_A_COLOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOTHING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_SOMEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERMISSION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_ON_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_TO_WEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_COLOUR() -> Py { ensure_init(); G_A_COLOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COLOUR(v: Py) { ensure_init(); G_A_COLOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOTHING_WORDS() -> Py { ensure_init(); G_CLOTHING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOTHING_WORDS(v: Py) { ensure_init(); G_CLOTHING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_SOMEWHERE() -> Py { ensure_init(); G_GOING_SOMEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_SOMEWHERE(v: Py) { ensure_init(); G_GOING_SOMEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THIS() -> Py { ensure_init(); G_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THIS(v: Py) { ensure_init(); G_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASIONS() -> Py { ensure_init(); G_OCCASIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASIONS(v: Py) { ensure_init(); G_OCCASIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART() -> Py { ensure_init(); G_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART(v: Py) { ensure_init(); G_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_ASK() -> Py { ensure_init(); G_PART_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_ASK(v: Py) { ensure_init(); G_PART_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_OCCASION() -> Py { ensure_init(); G_PAST_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_OCCASION(v: Py) { ensure_init(); G_PAST_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_PERMISSION_ASK() -> Py { ensure_init(); G_PERMISSION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERMISSION_ASK(v: Py) { ensure_init(); G_PERMISSION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_ON_A_THING() -> Py { ensure_init(); G_PUT_ON_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_ON_A_THING(v: Py) { ensure_init(); G_PUT_ON_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_RECENT() -> Py { ensure_init(); G_RECENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECENT(v: Py) { ensure_init(); G_RECENT.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION() -> Py { ensure_init(); G_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION(v: Py) { ensure_init(); G_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_TO_WEAR() -> Py { ensure_init(); G_WHAT_TO_WEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_WEAR(v: Py) { ensure_init(); G_WHAT_TO_WEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEATH() -> Py { ensure_init(); G__A_DEATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEATH(v: Py) { ensure_init(); G__A_DEATH.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What to wear, said about the occasion already on the table.\n\nPeople do not repeat the occasion. Once they have said the thing is happening, the next question\nabout it is only the part they have not worked out:\n\n    my sister is getting married in june\n    im not sure what to wear\n    and what about shoes\n\nOn its own, \"im not sure what to wear\" is a blank - the bot answered \"Thanks for telling me - I'm\nhere if you want to dig into any of that.\", which is the reply for a feeling, not for a person\nchoosing an outfit - and \"and what about shoes\" was a search hedge. Written out with the occasion\nin the words an entry is keyed on (\"what should i wear to my sister's wedding\", \"what shoes should\ni wear to a wedding\") the same two lines reach the outfit answer: not white or cream to a wedding,\na colour they feel good in, shoes they can stand in.\n\nWhat is in here:\n\n- `the_occasion(memory)` finds the occasion the chat is on - a wedding, an interview, a party, a\n  funeral - and says it as a thing: \"a wedding\", or \"my sister's wedding\" when the chat named\n  who it is. The entries answer \"to my sister's wedding\" and \"to a wedding\" alike, so the named\n  one is only used when the relation is the person's own.\n- `written_out(text, memory)` returns the question to answer, or None. It only fires when the\n  line is about what to wear and names no occasion of its own, so \"what should i wear to a\n  wedding\" is left alone, and a chat with nothing on the calendar keeps \"Tell me what the day is\n  - work, a meal out, a long walk - and I will say what to put on.\"\n");
    s_OCCASIONS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:weddings?|getting married|getting wed|married|marrying|bride|groom|bridesmaid|best man|hen do|stag do)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("wedding"), py_str("a wedding")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:interviews?|interviewing)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("interview"), py_str("an interview")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:first day|new job|new role|job starts?|starting (?:work|the job)|start(?:ing)? (?:on )?monday)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("work"), py_str("my new job")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:part(?:y|ies)|birthdays?|anniversar(?:y|ies)|housewarmings?|retirement|celebrations?|get together)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("party"), py_str("a party")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:funerals?|burial|cremation|wake|memorial)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("funeral"), py_str("a funeral")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?=[\\s\\S]*\\b(?:mum|mom|mother|dad|father|son|daughter|sister|brother|uncle|aunt|nan|gran|granny|grandma|grandad|grandpa|cousin|niece|nephew|friend|mate|colleague|boss|wife|husband|partner|girlfriend|boyfriend|neighbour)\\b)(?=[\\s\\S]*\\b(?:died|dying|passed away|passed on|bereave\\w*|death|buried)\\b)")]).unwrap_or_else(|e| exc::raise(e)), py_str("funeral"), py_str("a funeral")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:christenings?|baptisms?|naming ceremony)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("christening"), py_str("a christening")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:graduations?|graduating|prom|leavers? do)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("graduation"), py_str("a graduation")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:festivals?|gigs?|concerts?|gig\\b)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("gig"), py_str("a gig")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:barbecues?|bbqs?|cookout)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("barbecue"), py_str("a barbecue")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:dinners? out|meal out|meal at|restaurant|night out|first date|on a date|going on a date|date night)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("meal out"), py_str("a meal out")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bchurch\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("church"), py_str("church")])]));
    s_RELATION(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(sister|brother|mum|mom|mother|dad|father|son|daughter|niece|nephew|cousin|aunt|uncle|friend|best friend|mate|colleague|boss|partner|wife|husband|girlfriend|boyfriend|in laws?|mother in law|father in law|grandma|grandad|nan|gran|granny)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_WEAR(stdlib_call("re", "compile", &[py_str("\\b(?:what|wat)\\s+(?:to|should\\s+i|do\\s+i|can\\s+i|shall\\s+i)\\s+(?:wear|put\\s+on|dress)\\b|\\b(?:not?\\s+sure|no\\s+idea|dont\\s+know|do\\s*n'?t\\s+know|havent\\s+decided|cant\\s+decide|cannot\\s+decide|stuck)\\s+(?:about\\s+)?what\\s+(?:to\\s+|i\\s+(?:should\\s+)?)?(?:wear|put\\s+on|wearing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PART(stdlib_call("re", "compile", &[py_str("\\b(?:shoes?|heels?|trainers?|boots?|sandals?|flats?|dress|dresses|suit|suits|shirt|blouse|top|jacket|coat|blazer|hat|fascinator|tie|bag|outfit|skirt|trousers|chinos|jumpsuit)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PART_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|how)\\s+about\\b|\\band\\b|\\bor\\b|\\bshould\\s+i\\s+(?:wear|go\\s+for)\\b|\\bwould\\s+(?:flats|heels|boots|trainers|sandals)\\b|\\bis\\s+it\\s+ok\\s+to\\s+wear\\b|\\bcan\\s+i\\s+(?:wear|get\\s+away\\s+with)\\b|\\bdo\\s+i\\s+need\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_ON_A_THING(stdlib_call("re", "compile", &[py_str("\\bput\\s+on\\s+(?:the|a|an|some|any|my|your|his|her|their|our|it)\\s+([a-z']+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTHS(mcall(&py_str("january february march april may june july august september october november december spring summer autumn winter christmas easter"), "split", &[]));
    s_RECENT(Py::Int(5));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_OCCASION(stdlib_call("re", "compile", &[py_str("\\b(?:had|went\\s+to|been\\s+to|was\\s+at|were\\s+at)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|dead|passed\\s+away|passed\\s+on|death|bereave\\w*|funerals?|buried|burial|cremation|memorial|wake)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOTHING_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:wear|wearing|wore|clothes|outfit|dress|suit|shoes|boots|trainers|jacket|coat|tie|shirt|colours|colors|smart|comfortable|black|white|cream|navy|grey|dark)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERMISSION_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:alright|all right|ok|okay|fine|allowed|acceptable|a problem|too much)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:not|never|avoid|don'?t|doesn'?t|no|unless)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COLOUR(stdlib_call("re", "compile", &[py_str("\\b(?:black|white|cream|navy|grey|gray|blue|red|green|pink|beige|brown|charcoal|burgundy|silver|gold|floral|patterned|light|dark)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_SOMEWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:to|for|at)\\s+(?:a\\s+|an\\s+|the\\s+|my\\s+|our\\s+)?(?:work|school|college|uni|university|office|gym|run|jog|walk|ride|cycle|swim|shops?|supermarket|church|bed|pub|bar|interview|meeting|shift|flight|train|beach|hike|climb|dance|class|lesson|lessons|tennis|football|golf|rugby|netball|yoga|gig|concert|restaurant|dinner|lunch|brunch|date|funeral|wedding|christening|baptism)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _recent(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The last few things the person said, newest first, as one lower-case string.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_RECENT()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&text.clone()) {
    let _ = mcall(&said.clone(), "append", &[text.clone()]);
    }
    }
    }
    return said.clone();
    Py::None
}
pub fn _an(args: &[Py]) -> Py {
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
    let _ = py_str("The article a word takes: \"an interview\", \"a wedding\".");
    return op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&phrase.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an ") } else { py_str("a ") }, &phrase.clone());
    Py::None
}
pub fn the_occasion(args: &[Py]) -> Py {
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
    let mut _key: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The occasion the chat is on, as \"a wedding\" or \"my sister's wedding\", or None.");
    said = _recent(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&said.clone());
    for __item in __iter1 {
    text = __item.clone();
    let __iter2 = py_iter_or_raise(&g_OCCASIONS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    pattern = __unpk3[0].clone();
    _key = __unpk3[1].clone();
    phrase = __unpk3[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[text.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_PAST_OCCASION(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_DEATH(), "search", &[text.clone()]))) } }) {
    continue;
    }
    who = mcall(&g_RELATION(), "search", &[text.clone()]);
    if py_truthy(&who.clone()) {
    return op_modulo(&py_str("my %s's %s"), &py_tuple(vec![mcall(&who.clone(), "group", &[Py::Int(1)]), geti(&mcall(&phrase.clone(), "split", &[]), &op_neg(&Py::Int(1)))]));
    }
    return phrase.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_last_answer(args: &[Py]) -> Py {
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
    let _ = py_str("What the bot said last, lower-case: the answer a part of the outfit is asked about.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _the_answer_already_gave_it(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the bot's last reply was already the outfit answer for this occasion.\n\n    The second turn earns the card (\"im not sure what to wear\" -> the wedding answer, where main\n    read a feeling back). A third turn that asks about a colour or a part of the same outfit was\n    then answered with the card again in other words - \"and what about shoes\" got the colour\n    warning, and \"will black be alright\" got \"that is everything i have on that one\" - which is\n    the bot repeating itself. The pre-pass has done its job at that point, so the follow-up is\n    left to the path it took before this module existed.\n    ");
    last = _the_last_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::Bool(false);
    }
    return op_ge(&call_builtin("len", &[call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_CLOTHING_WORDS(), "findall", &[last.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn the_sentence_answered(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence of the bot's last answer that answers a follow-up about one thing, or None.\n\n    The card for the occasion is already on the screen, and the person asks about one thing in it:\n    \"will black be alright\" after an answer that said black or dark colours are the thing. Both\n    sides left that with a shrug (\"that is everything I have on that one, so I would only repeat\n    myself\") while the sentence holding the answer was one line up. The sentence that names the\n    thing is the answer, so it is handed over as it was said, and a card whose sentence says not\n    to wear the thing still says not to: \"will white be alright\" gets the sentence that warns\n    against it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&the_occasion(&[memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_PART(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_COLOUR(), "search", &[line.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_PART_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PERMISSION_ASK(), "search", &[line.clone()]) } }))) {
    return Py::None;
    }
    last = _the_last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&last.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::None;
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), last.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    words = op_add(&call_builtin("sorted", &[call_builtin("set", &[mcall(&g_A_COLOUR(), "findall", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("sorted", &[call_builtin("set", &[mcall(&g_PART(), "findall", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), last.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&sentences.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), sentence.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&geti(&sentence.clone(), &Py::Int(0)), "isalpha", &[])) {
    sentence = op_add(&mcall(&geti(&sentence.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&sentence.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&mcall(&g_NOT_THIS(), "search", &[sentence.clone()])) {
    lead = py_str("Careful - ");
    } else {
    if py_truthy(&mcall(&g_NOT_THIS(), "search", &[line.clone()])) {
    lead = py_str("Then ");
    } else {
    lead = py_str("Yes - ");
    }
    }
    return op_add(&op_add(&lead.clone(), &mcall(&geti(&sentence.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&sentence.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _names_an_occasion(args: &[Py]) -> Py {
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
    let mut _key: Py = Py::None;
    let mut _phrase: Py = Py::None;
    let mut pattern: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OCCASIONS()) { let __unpk3922 = py_unpack(&__item.clone(), 3, None); let mut pattern: Py = __unpk3922[0].clone(); let mut _key: Py = __unpk3922[1].clone(); let mut _phrase: Py = __unpk3922[2].clone(); __out.push(mcall(&pattern.clone(), "search", &[text.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_its_own_outing(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the ask goes somewhere of its own, so the chat's occasion is not what it is about.");
    found = mcall(&g_GOING_SOMEWHERE(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&_names_an_occasion(&[text.clone()])));
    Py::None
}
pub fn written_out(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut part: Py = Py::None;
    let mut shaped: Py = Py::None;
    let mut worn: Py = Py::None;
    let _ = py_str("The outfit question the line is asking, or None when the line is not one.\n\n    \"im not sure what to wear\" after a wedding is \"what should i wear to a wedding\" and \"and\n    what about shoes\" is \"what shoes should i wear to a wedding\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::None;
    }
    worn = mcall(&g_PUT_ON_A_THING(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = worn.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CLOTHING_WORDS(), "search", &[mcall(&worn.clone(), "group", &[Py::Int(1)])]))) } }) {
    return Py::None;
    }
    shaped = { let __b = mcall(&g_WHAT_TO_WEAR(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PART(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PART_ASK(), "search", &[line.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&shaped.clone()))) {
    return Py::None;
    }
    occasion = the_occasion(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&occasion.clone()))) {
    return Py::None;
    }
    if py_truthy(&_the_answer_already_gave_it(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&_names_an_occasion(&[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&_names_its_own_outing(&[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_WHAT_TO_WEAR(), "search", &[line.clone()])) {
    return op_modulo(&py_str("what should i wear to %s"), &occasion.clone());
    }
    part = mcall(&g_PART(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_PART_ASK(), "search", &[line.clone()]) } }) {
    if py_truthy(&Py::Bool(!py_contains(&_the_last_answer(&[memory.clone()]), &mcall(&part.clone(), "group", &[Py::Int(0)])))) {
    return Py::None;
    }
    return op_modulo(&py_str("what %s should i wear to %s"), &py_tuple(vec![mcall(&part.clone(), "group", &[Py::Int(0)]), occasion.clone()]));
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.outfit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_COLOUR" => Some(g_A_COLOUR()),
            "CLOTHING_WORDS" => Some(g_CLOTHING_WORDS()),
            "GOING_SOMEWHERE" => Some(g_GOING_SOMEWHERE()),
            "MONTHS" => Some(g_MONTHS()),
            "NOT_THIS" => Some(g_NOT_THIS()),
            "OCCASIONS" => Some(g_OCCASIONS()),
            "PART" => Some(g_PART()),
            "PART_ASK" => Some(g_PART_ASK()),
            "PAST_OCCASION" => Some(g_PAST_OCCASION()),
            "PERMISSION_ASK" => Some(g_PERMISSION_ASK()),
            "PUT_ON_A_THING" => Some(g_PUT_ON_A_THING()),
            "RECENT" => Some(g_RECENT()),
            "RELATION" => Some(g_RELATION()),
            "WHAT_TO_WEAR" => Some(g_WHAT_TO_WEAR()),
            "WORD" => Some(g_WORD()),
            "_A_DEATH" => Some(g__A_DEATH()),
            "_an" => Some(py_func("_an", |args: &[Py]| _an(args))),
            "_names_an_occasion" => Some(py_func("_names_an_occasion", |args: &[Py]| _names_an_occasion(args))),
            "_names_its_own_outing" => Some(py_func("_names_its_own_outing", |args: &[Py]| _names_its_own_outing(args))),
            "_recent" => Some(py_func("_recent", |args: &[Py]| _recent(args))),
            "_the_answer_already_gave_it" => Some(py_func("_the_answer_already_gave_it", |args: &[Py]| _the_answer_already_gave_it(args))),
            "_the_last_answer" => Some(py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args))),
            "the_occasion" => Some(py_func("the_occasion", |args: &[Py]| the_occasion(args))),
            "the_sentence_answered" => Some(py_func("the_sentence_answered", |args: &[Py]| the_sentence_answered(args))),
            "written_out" => Some(py_func("written_out", |args: &[Py]| written_out(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_COLOUR" => s_A_COLOUR(v),
            "CLOTHING_WORDS" => s_CLOTHING_WORDS(v),
            "GOING_SOMEWHERE" => s_GOING_SOMEWHERE(v),
            "MONTHS" => s_MONTHS(v),
            "NOT_THIS" => s_NOT_THIS(v),
            "OCCASIONS" => s_OCCASIONS(v),
            "PART" => s_PART(v),
            "PART_ASK" => s_PART_ASK(v),
            "PAST_OCCASION" => s_PAST_OCCASION(v),
            "PERMISSION_ASK" => s_PERMISSION_ASK(v),
            "PUT_ON_A_THING" => s_PUT_ON_A_THING(v),
            "RECENT" => s_RECENT(v),
            "RELATION" => s_RELATION(v),
            "WHAT_TO_WEAR" => s_WHAT_TO_WEAR(v),
            "WORD" => s_WORD(v),
            "_A_DEATH" => s__A_DEATH(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.outfit", module_obj);
    pyrt::xrefs::reg_fn("memory.outfit", "_recent", py_func("_recent", |args: &[Py]| _recent(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "_an", py_func("_an", |args: &[Py]| _an(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "the_occasion", py_func("the_occasion", |args: &[Py]| the_occasion(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "_the_last_answer", py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "_the_answer_already_gave_it", py_func("_the_answer_already_gave_it", |args: &[Py]| _the_answer_already_gave_it(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "the_sentence_answered", py_func("the_sentence_answered", |args: &[Py]| the_sentence_answered(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "_names_an_occasion", py_func("_names_an_occasion", |args: &[Py]| _names_an_occasion(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "_names_its_own_outing", py_func("_names_its_own_outing", |args: &[Py]| _names_its_own_outing(args)));
    pyrt::xrefs::reg_fn("memory.outfit", "written_out", py_func("written_out", |args: &[Py]| written_out(args)));
    pyrt::xrefs::reg_access("memory.outfit", "A_COLOUR", g_A_COLOUR, s_A_COLOUR);
    pyrt::xrefs::reg_access("memory.outfit", "CLOTHING_WORDS", g_CLOTHING_WORDS, s_CLOTHING_WORDS);
    pyrt::xrefs::reg_access("memory.outfit", "GOING_SOMEWHERE", g_GOING_SOMEWHERE, s_GOING_SOMEWHERE);
    pyrt::xrefs::reg_access("memory.outfit", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory.outfit", "NOT_THIS", g_NOT_THIS, s_NOT_THIS);
    pyrt::xrefs::reg_access("memory.outfit", "OCCASIONS", g_OCCASIONS, s_OCCASIONS);
    pyrt::xrefs::reg_access("memory.outfit", "PART", g_PART, s_PART);
    pyrt::xrefs::reg_access("memory.outfit", "PART_ASK", g_PART_ASK, s_PART_ASK);
    pyrt::xrefs::reg_access("memory.outfit", "PAST_OCCASION", g_PAST_OCCASION, s_PAST_OCCASION);
    pyrt::xrefs::reg_access("memory.outfit", "PERMISSION_ASK", g_PERMISSION_ASK, s_PERMISSION_ASK);
    pyrt::xrefs::reg_access("memory.outfit", "PUT_ON_A_THING", g_PUT_ON_A_THING, s_PUT_ON_A_THING);
    pyrt::xrefs::reg_access("memory.outfit", "RECENT", g_RECENT, s_RECENT);
    pyrt::xrefs::reg_access("memory.outfit", "RELATION", g_RELATION, s_RELATION);
    pyrt::xrefs::reg_access("memory.outfit", "WHAT_TO_WEAR", g_WHAT_TO_WEAR, s_WHAT_TO_WEAR);
    pyrt::xrefs::reg_access("memory.outfit", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("memory.outfit", "_A_DEATH", g__A_DEATH, s__A_DEATH);
}
