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


thread_local! { static G_A_DEATH_BEFORE_THE_DREAM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DREAM_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SAID_WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SCENE_OF_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WISH_TO_BE_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DREAM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DREAM_IS_HAPPENING_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DREAM_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DREAM_NAMED_LOOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DREAM_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_A_NIGHTMARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.dream: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_DEATH_BEFORE_THE_DREAM() -> Py { ensure_init(); G_A_DEATH_BEFORE_THE_DREAM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEATH_BEFORE_THE_DREAM(v: Py) { ensure_init(); G_A_DEATH_BEFORE_THE_DREAM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DREAM_AHEAD() -> Py { ensure_init(); G_A_DREAM_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DREAM_AHEAD(v: Py) { ensure_init(); G_A_DREAM_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SAID_WISH() -> Py { ensure_init(); G_A_SAID_WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SAID_WISH(v: Py) { ensure_init(); G_A_SAID_WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SCENE_OF_NOTHING() -> Py { ensure_init(); G_A_SCENE_OF_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SCENE_OF_NOTHING(v: Py) { ensure_init(); G_A_SCENE_OF_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WISH() -> Py { ensure_init(); G_A_WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WISH(v: Py) { ensure_init(); G_A_WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WISH_TO_BE_GONE() -> Py { ensure_init(); G_A_WISH_TO_BE_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WISH_TO_BE_GONE(v: Py) { ensure_init(); G_A_WISH_TO_BE_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_DREAM() -> Py { ensure_init(); G_DREAM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DREAM(v: Py) { ensure_init(); G_DREAM.with(|c| *c.borrow_mut() = v); }
pub fn g_DREAM_IS_HAPPENING_NOW() -> Py { ensure_init(); G_DREAM_IS_HAPPENING_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DREAM_IS_HAPPENING_NOW(v: Py) { ensure_init(); G_DREAM_IS_HAPPENING_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_DREAM_NAMED() -> Py { ensure_init(); G_DREAM_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DREAM_NAMED(v: Py) { ensure_init(); G_DREAM_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_DREAM_NAMED_LOOSE() -> Py { ensure_init(); G_DREAM_NAMED_LOOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DREAM_NAMED_LOOSE(v: Py) { ensure_init(); G_DREAM_NAMED_LOOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_DREAM_VERB() -> Py { ensure_init(); G_DREAM_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DREAM_VERB(v: Py) { ensure_init(); G_DREAM_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON() -> Py { ensure_init(); G_FIRST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON(v: Py) { ensure_init(); G_FIRST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_A_NIGHTMARE() -> Py { ensure_init(); G_NAMED_A_NIGHTMARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_A_NIGHTMARE(v: Py) { ensure_init(); G_NAMED_A_NIGHTMARE.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED_IT() -> Py { ensure_init(); G_SETTLED_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED_IT(v: Py) { ensure_init(); G_SETTLED_IT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reading of a line that is about a dream, and the scene the dream was made of.\n\nA dream is a scene rather than an event, and the words of one are the words of the real thing:\n\n    i dreamt my dad died              the loss composer offered sympathy for a death\n    i keep dreaming my teeth fall out a child's milk teeth were explained\n    i had a dream my house was on fire  \"ugh, that's a lot to carry\"\n\nOne reading serves all three, because the only thing that separates a dream from the event is the\nframe around it - `dreamt`, `dreamed`, `i had a dream`, `i keep dreaming`. It is here rather than\nin one module because three readers need it: `bot/modules/dreams.py` answers the dream,\n`bot/compose/loss.py` has to stand down when the death is inside one, and the knowledge entries\nhave to stand down when the tooth or the fire is inside one.\n\nThe frame is read the way the sentence is built: the dream verb comes *before* the scene it is\nabout, and the speaker is in it. That is what keeps two other lines out:\n\n    my dad died and i dreamt about him      the death is real, and it came first\n    why do we dream / what do dreams mean   nobody in the line is dreaming\n");
    s_DREAM(stdlib_call("re", "compile", &[py_str("\\b(dreamt|dreamed|dreaming|dream|dreams)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|ive|i've|id|i'd|my|me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_A_NIGHTMARE(stdlib_call("re", "compile", &[py_str("\\b(?:nightmares?|night terrors?|(?:bad|scary|terrible|awful|horrible)\\s+dreams?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WISH(stdlib_call("re", "compile", &[py_str("\\bdream(?:s|ing)?\\s+of\\b|\\b(?:i|we)\\s+have\\s+a\\s+dream\\b(?![^.]*\\b(?:last|other)\\s+night)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WISH_TO_BE_GONE(stdlib_call("re", "compile", &[py_str("\\bdream(?:s|t|ed|ing)?\\s+(?:of|about)\\s+(?:my\\s+own\\s+|me\\s+|being\\s+)?(?:ending\\s+it(?:\\s+all)?|ending\\s+things|ending\\s+my\\s+life|ending\\s+everything|kill(?:ing)?\\s+myself|suicide|suicidal|dying|death|being\\s+dead|dead|not\\s+waking|not\\s+being\\s+here|disappearing|gone\\s+for\\s+good)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DREAM_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:dreamt|dreamed|dreaming)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SAID_WISH(stdlib_call("re", "compile", &[py_str("\\b(?:ending\\s+it\\s+all|ending\\s+things|kill(?:ing)?\\s+myself|kill\\s+me|suicide|suicidal|want(?:ed|ing)?\\s+to\\s+die|wanted\\s+to\\s+be\\s+dead|wish(?:ed)?\\s+i\\s+(?:was|were)\\s+dead|not\\s+being\\s+here|not\\s+here\\s+any\\s+more|don'?t\\s+want\\s+to\\s+be\\s+here)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DREAM_AHEAD(stdlib_call("re", "compile", &[py_str("\\bdream(?:s)?\\s+(?:job|jobs|career|role|home|house|flat|holiday|holidays|trip|car|wedding|team|life|garden|kitchen|body|figure|man|woman|partner|business|company|kitchen|room|wardrobe|outfit)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SCENE_OF_NOTHING(stdlib_call("re", "compile", &[py_str("^(?:about\\s+)?(?:it|that|this|him|her|them|one)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_SETTLED_IT(stdlib_call("re", "compile", &[py_str("\\b(?:just|only)\\s+(?:a\\s+|an\\s+)?(?:bad\\s+|weird\\s+|strange\\s+|silly\\s+|daft\\s+)?(?:dream|nightmare)\\b|\\b(?:it|that|this)\\s+(?:was|were|is)\\s+(?:only\\s+|just\\s+)?(?:a\\s+)?(?:dream|nightmare)\\b|\\bnot\\s+real\\b|\\bwasn'?t\\s+real\\b|\\bnever\\s+happened\\b|\\b(?:he|she|they|it)(?:'s|\\s+is|\\s+was|\\s+were)?\\s+(?:fine|ok|okay|alright|all\\s+right|safe|still\\s+here|still\\s+alive|alive)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DREAM_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:just\\s+)?(?:had\\s+(?:a\\s+|the\\s+|this\\s+)?(?:dream|nightmare|bad\\s+dream)|(?:had|have)\\s+(?:a\\s+|this\\s+)?(?:dream|nightmare|nightmares|bad\\s+dreams?|weird\\s+dreams?)|dream(?:t|ed|ing)?(?:\\s+about|\\s+that|\\s+of|,|\\s+i\\b)|keep\\s+dreaming|kept\\s+dreaming|nightmares?\\s+(?:about|where|that)|(?:was|were)\\s+dreaming)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DREAM_NAMED_LOOSE(stdlib_call("re", "compile", &[py_str("\\b(?:in\\s+(?:my|a|the)\\s+dream|in\\s+my\\s+nightmare|in\\s+dreams?|dreamt|dreamed|dreaming|nightmares?|nightmare)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DREAM_IS_HAPPENING_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:woke\\s+up|woken\\s+up|wake\\s+up|awake|awoken|still\\s+(?:there|here|going|on|going\\s+on|burning|bleeding|hurting|happening|smoking|happens|feel|feels|have|got)|right\\s+now|just\\s+now|this\\s+minute|at\\s+the\\s+moment|and\\s+i\\s+(?:can'?t|cannot|cant)\\s+(?:breathe|move|feel|stop)|it\\s+(?:is|was)\\s+real|i\\s+(?:can|could)\\s+smell\\s+(?:smoke|gas|burning)|i\\s+(?:have|took|taken|swallowed|drank|drunk)\\s+(?:them|it|the|some|a\\s+few|\\d+|all\\s+of)|\\b\\d+\\s+(?:of\\s+them|tablets|pills)|i\\s+am\\s+(?:bleeding|burning|stuck|trapped|drowning|having|on\\s+fire)|i'?m\\s+(?:bleeding|burning|stuck|trapped|drowning|having|on\\s+fire|not\\s+breathing)|my\\s+(?:arm|leg|chest|head|back|side|stomach|belly|hand|foot|face)\\s+(?:is\\s+|feels\\s+)?(?:numb|bleeding|swollen|hot|painful|hurts|broken|blue)|there\\s+(?:is|are)\\s+(?:smoke|flames?|blood|pain)|the\\s+(?:smoke|flames?|alarm|bleeding)\\s+(?:is|are)\\s+(?:still|now|real)|(?:i\\s+|we\\s+)?(?:can\\s+)?(?:smell|smells|smelling)\\s+(?:smoke|gas|burning)|\\b(?:smoke|flames?)\\s+(?:is\\s+|are\\s+)?(?:coming|pouring|everywhere)|\\balarm\\s+(?:is\\s+|keeps\\s+)?(?:going|beeping|sounding)|\\b(?:is|are)\\s+(?:going|happening)\\s+(?:on\\s+)?now|\\b(?:he|she|they|it|the\\s+baby|my\\s+baby)(?:'s|s|\\s+is|\\s+was|\\s+are|\\s+were)?\\s+(?:not|isn'?t|wasn'?t|aren'?t|aint|ain'?t|no\\s+longer)\\s+(?:breathing|moving|responding|waking|conscious|with\\s+us|alive)|\\b(?:shes|hes|theyre|its)\\s+(?:not|isn'?t)\\s+(?:breathing|moving|responding|waking|conscious)|\\b(?:lying|lay)\\s+(?:there|on\\s+the\\s+(?:floor|ground|bed|road))\\s+(?:now|still)|\\b(?:there'?s|theres)\\s+smoke\\s+(?:in|coming|everywhere)|\\b(?:i\\s+can\\s+hear|i\\s+hear)\\s+(?:the\\s+alarm|smoke\\s+alarm|sirens))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEATH_BEFORE_THE_DREAM(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|death|dead|deaths|passed\\s+away|passed\\s+on|killed|funeral|buried|cremation|widow|widowed)\\b[^.]*?\\b(?:dream|dreams|dreamt|dreamed|dreaming|dreamed|nightmare|nightmares)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_scene(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut scene: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("What the dream was about, or None when the line is not a dream somebody had.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(400)) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NAMED_A_NIGHTMARE(), "search", &[text.clone()])) {
    return Py::None;
    }
    found = mcall(&g_DREAM(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    before = slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_FIRST_PERSON(), "search", &[before.clone()])))) {
    return Py::None;
    }
    scene = mcall(&slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.:;!-")]);
    if py_truthy(&{ let __b = mcall(&g_A_WISH(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&scene.clone(), &py_str("one day"))); if py_truthy(&__b) { __b } else { mcall(&g_A_SCENE_OF_NOTHING(), "match", &[scene.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_WISH_TO_BE_GONE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SAID_WISH(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_DREAM_AHEAD(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DREAM_VERB(), "search", &[text.clone()]))) } }) {
    return Py::None;
    }
    return { let __b = scene.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn named(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a dream the speaker had, so a reader of the scene stands down.");
    return Py::Bool(!py_is(&the_scene(&[line.clone()]), &Py::None));
    Py::None
}
pub fn dreamed_before(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut scene: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when `pattern` is found inside the dream and not before it.\n\n    `i dreamt my dad died` is a dream; `my dad died and i dreamt about him` is a death with a\n    dream after it, and the death is what the line is for.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    scene = the_scene(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&scene.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[scene.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&text.clone(), "find", &[scene.clone()]), &Py::None)])));
    Py::None
}
pub fn told_in_the_turn_before(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("True when the turn just before this one was a dream, so a line can be about that dream.\n\n    A follow-up that settles it names no frame of its own - \"hes fine its just a dream\" - and\n    every reader of `SETTLED_IT` has to know that the dream is in the turn before, not in the\n    line. One reading in one place: `bot/modules/dreams.py` takes the turn and\n    `bot/compose/loss.py` stands down, both by asking this.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }))) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("dreams")))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&the_scene(&[call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), &Py::None));
    Py::None
}
pub fn told_as_a_dream(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line tells a dream of its own and nothing in it says the room has it.\n\n    What a knowledge entry stands down for: \"i had a bad dream last night\" names no scene and the\n    sleep entry is the answer to it, and \"i dreamt my baby stopped breathing and shes not\n    breathing now\" is the room, so neither stands anything down.\n    ");
    return { let __b = named(&[line.clone()]); if !py_truthy(&__b) { __b } else { a_dream_is_not_happening(&[line.clone()]) } };
    Py::None
}
pub fn a_dream_is_not_happening(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line is about a dream and nothing in it says it is happening now.");
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    text = call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_DREAM_NAMED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DREAM_NAMED_LOOSE(), "search", &[text.clone()]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_WISH_TO_BE_GONE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SAID_WISH(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_DREAM_AHEAD(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DREAM_VERB(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_DREAM_IS_HAPPENING_NOW(), "search", &[text.clone()])));
    Py::None
}
pub fn a_death_came_first(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line names a death before it names a dream.");
    return call_builtin("bool", &[mcall(&g_A_DEATH_BEFORE_THE_DREAM(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.dream".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DEATH_BEFORE_THE_DREAM" => Some(g_A_DEATH_BEFORE_THE_DREAM()),
            "A_DREAM_AHEAD" => Some(g_A_DREAM_AHEAD()),
            "A_SAID_WISH" => Some(g_A_SAID_WISH()),
            "A_SCENE_OF_NOTHING" => Some(g_A_SCENE_OF_NOTHING()),
            "A_WISH" => Some(g_A_WISH()),
            "A_WISH_TO_BE_GONE" => Some(g_A_WISH_TO_BE_GONE()),
            "DREAM" => Some(g_DREAM()),
            "DREAM_IS_HAPPENING_NOW" => Some(g_DREAM_IS_HAPPENING_NOW()),
            "DREAM_NAMED" => Some(g_DREAM_NAMED()),
            "DREAM_NAMED_LOOSE" => Some(g_DREAM_NAMED_LOOSE()),
            "DREAM_VERB" => Some(g_DREAM_VERB()),
            "FIRST_PERSON" => Some(g_FIRST_PERSON()),
            "NAMED_A_NIGHTMARE" => Some(g_NAMED_A_NIGHTMARE()),
            "SETTLED_IT" => Some(g_SETTLED_IT()),
            "a_death_came_first" => Some(py_func("a_death_came_first", |args: &[Py]| a_death_came_first(args))),
            "a_dream_is_not_happening" => Some(py_func("a_dream_is_not_happening", |args: &[Py]| a_dream_is_not_happening(args))),
            "dreamed_before" => Some(py_func("dreamed_before", |args: &[Py]| dreamed_before(args))),
            "named" => Some(py_func("named", |args: &[Py]| named(args))),
            "the_scene" => Some(py_func("the_scene", |args: &[Py]| the_scene(args))),
            "told_as_a_dream" => Some(py_func("told_as_a_dream", |args: &[Py]| told_as_a_dream(args))),
            "told_in_the_turn_before" => Some(py_func("told_in_the_turn_before", |args: &[Py]| told_in_the_turn_before(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DEATH_BEFORE_THE_DREAM" => s_A_DEATH_BEFORE_THE_DREAM(v),
            "A_DREAM_AHEAD" => s_A_DREAM_AHEAD(v),
            "A_SAID_WISH" => s_A_SAID_WISH(v),
            "A_SCENE_OF_NOTHING" => s_A_SCENE_OF_NOTHING(v),
            "A_WISH" => s_A_WISH(v),
            "A_WISH_TO_BE_GONE" => s_A_WISH_TO_BE_GONE(v),
            "DREAM" => s_DREAM(v),
            "DREAM_IS_HAPPENING_NOW" => s_DREAM_IS_HAPPENING_NOW(v),
            "DREAM_NAMED" => s_DREAM_NAMED(v),
            "DREAM_NAMED_LOOSE" => s_DREAM_NAMED_LOOSE(v),
            "DREAM_VERB" => s_DREAM_VERB(v),
            "FIRST_PERSON" => s_FIRST_PERSON(v),
            "NAMED_A_NIGHTMARE" => s_NAMED_A_NIGHTMARE(v),
            "SETTLED_IT" => s_SETTLED_IT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.dream", module_obj);
    pyrt::xrefs::reg_fn("memory.dream", "the_scene", py_func("the_scene", |args: &[Py]| the_scene(args)));
    pyrt::xrefs::reg_fn("memory.dream", "named", py_func("named", |args: &[Py]| named(args)));
    pyrt::xrefs::reg_fn("memory.dream", "dreamed_before", py_func("dreamed_before", |args: &[Py]| dreamed_before(args)));
    pyrt::xrefs::reg_fn("memory.dream", "told_in_the_turn_before", py_func("told_in_the_turn_before", |args: &[Py]| told_in_the_turn_before(args)));
    pyrt::xrefs::reg_fn("memory.dream", "told_as_a_dream", py_func("told_as_a_dream", |args: &[Py]| told_as_a_dream(args)));
    pyrt::xrefs::reg_fn("memory.dream", "a_dream_is_not_happening", py_func("a_dream_is_not_happening", |args: &[Py]| a_dream_is_not_happening(args)));
    pyrt::xrefs::reg_fn("memory.dream", "a_death_came_first", py_func("a_death_came_first", |args: &[Py]| a_death_came_first(args)));
    pyrt::xrefs::reg_access("memory.dream", "A_DEATH_BEFORE_THE_DREAM", g_A_DEATH_BEFORE_THE_DREAM, s_A_DEATH_BEFORE_THE_DREAM);
    pyrt::xrefs::reg_access("memory.dream", "A_DREAM_AHEAD", g_A_DREAM_AHEAD, s_A_DREAM_AHEAD);
    pyrt::xrefs::reg_access("memory.dream", "A_SAID_WISH", g_A_SAID_WISH, s_A_SAID_WISH);
    pyrt::xrefs::reg_access("memory.dream", "A_SCENE_OF_NOTHING", g_A_SCENE_OF_NOTHING, s_A_SCENE_OF_NOTHING);
    pyrt::xrefs::reg_access("memory.dream", "A_WISH", g_A_WISH, s_A_WISH);
    pyrt::xrefs::reg_access("memory.dream", "A_WISH_TO_BE_GONE", g_A_WISH_TO_BE_GONE, s_A_WISH_TO_BE_GONE);
    pyrt::xrefs::reg_access("memory.dream", "DREAM", g_DREAM, s_DREAM);
    pyrt::xrefs::reg_access("memory.dream", "DREAM_IS_HAPPENING_NOW", g_DREAM_IS_HAPPENING_NOW, s_DREAM_IS_HAPPENING_NOW);
    pyrt::xrefs::reg_access("memory.dream", "DREAM_NAMED", g_DREAM_NAMED, s_DREAM_NAMED);
    pyrt::xrefs::reg_access("memory.dream", "DREAM_NAMED_LOOSE", g_DREAM_NAMED_LOOSE, s_DREAM_NAMED_LOOSE);
    pyrt::xrefs::reg_access("memory.dream", "DREAM_VERB", g_DREAM_VERB, s_DREAM_VERB);
    pyrt::xrefs::reg_access("memory.dream", "FIRST_PERSON", g_FIRST_PERSON, s_FIRST_PERSON);
    pyrt::xrefs::reg_access("memory.dream", "NAMED_A_NIGHTMARE", g_NAMED_A_NIGHTMARE, s_NAMED_A_NIGHTMARE);
    pyrt::xrefs::reg_access("memory.dream", "SETTLED_IT", g_SETTLED_IT, s_SETTLED_IT);
}
