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


thread_local! { static G_ACKNOWLEDGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_ALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IS_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_APOSTROPHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_VALUE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREFERS_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE_TOO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHICH_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G___all__: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ACKNOWLEDGES() -> Py { ensure_init(); G_ACKNOWLEDGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACKNOWLEDGES(v: Py) { ensure_init(); G_ACKNOWLEDGES.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ANSWER() -> Py { ensure_init(); G_AN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ANSWER(v: Py) { ensure_init(); G_AN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_EVENT() -> Py { ensure_init(); G_AN_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EVENT(v: Py) { ensure_init(); G_AN_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_ALL() -> Py { ensure_init(); G_ASKS_ALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_ALL(v: Py) { ensure_init(); G_ASKS_ALL.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_SOMETHING() -> Py { ensure_init(); G_ASKS_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_SOMETHING(v: Py) { ensure_init(); G_ASKS_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_TAIL() -> Py { ensure_init(); G_ASK_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_TAIL(v: Py) { ensure_init(); G_ASK_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE() -> Py { ensure_init(); G_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE(v: Py) { ensure_init(); G_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_AFTER() -> Py { ensure_init(); G_A_QUESTION_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_AFTER(v: Py) { ensure_init(); G_A_QUESTION_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE() -> Py { ensure_init(); G_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE(v: Py) { ensure_init(); G_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_BEST_PERSON() -> Py { ensure_init(); G_BEST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEST_PERSON(v: Py) { ensure_init(); G_BEST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTS() -> Py { ensure_init(); G_CORRECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTS(v: Py) { ensure_init(); G_CORRECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_IS_MINE() -> Py { ensure_init(); G_IS_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IS_MINE(v: Py) { ensure_init(); G_IS_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_APOSTROPHE() -> Py { ensure_init(); G_KIN_APOSTROPHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_APOSTROPHE(v: Py) { ensure_init(); G_KIN_APOSTROPHE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_WORDS() -> Py { ensure_init(); G_MY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_WORDS(v: Py) { ensure_init(); G_MY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_VALUE() -> Py { ensure_init(); G_NO_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_VALUE(v: Py) { ensure_init(); G_NO_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_VALUE_PHRASES() -> Py { ensure_init(); G_NO_VALUE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_VALUE_PHRASES(v: Py) { ensure_init(); G_NO_VALUE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_PREFERS_NOW() -> Py { ensure_init(); G_PREFERS_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREFERS_NOW(v: Py) { ensure_init(); G_PREFERS_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS() -> Py { ensure_init(); G_SAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS(v: Py) { ensure_init(); G_SAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE_TOO() -> Py { ensure_init(); G_SENTENCE_TOO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE_TOO(v: Py) { ensure_init(); G_SENTENCE_TOO.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); G_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); G_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_STUCK() -> Py { ensure_init(); G_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STUCK(v: Py) { ensure_init(); G_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL() -> Py { ensure_init(); G_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); G_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHICH_IS() -> Py { ensure_init(); G_WHICH_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHICH_IS(v: Py) { ensure_init(); G_WHICH_IS.with(|c| *c.borrow_mut() = v); }
pub fn g___all__() -> Py { ensure_init(); G___all__.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s___all__(v: Py) { ensure_init(); G___all__.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A favourite named in a line, and the ask for it back.\n\n\"my favourite colour is green\" is a fact about the person, and the conversation lane keeps\nfacts about the person. This one had no slot, so two things went wrong on the same line and\nthen on the next one:\n\n* the line was not kept at all. The composer still said it back (\"Your favourite food is curry\n  - noted.\"), so the chat recorded a favourite the store did not have. A colour was worse: the\n  knowledge base found the word \"green\" and painted a wall (\"White or cream lets green breathe,\n  then pink, mustard or brown leather\"), and the person's own line was answered with decorating\n  advice;\n* \"what is my favourite colour\" was then not a recall of anything, and the composer answered it\n  with the paragraph it had just given (\"Same as before: White or cream lets green breathe...\"),\n  which is the coldest shape the bot has: it repeats the thing that was wrong instead of\n  saying the one word they asked for.\n\nThe slot is the general one, so it covers every favourite people actually name - a colour, a\nfood, a film, a book, a team, a song, a place. Nothing here knows what any of those are: the\nthing is the word after \"favourite\" and the value is whatever the person put after \"is\", which\nis why \"my favourite film is alien\" and \"my favourite colour is a dark green\" both work.\n\nWhat counts as a value is deliberately narrow, because a wrong value is worse than no value:\nthe line has to be a statement (a \"?\" on the end passes), the value has to be a thing rather\nthan a clause, and a line that says what somebody *else* likes (\"my mum's favourite colour is\ngreen\") is not the user's own and passes.\n\nOnly the person's own favourite is read, in the shapes people type: \"my favourite X is Y\",\n\"my favourite X's Y\", \"my fav X is Y\". The ask is the same thing read backwards - \"what is my\nfavourite X\", \"whats my favourite X again\", \"do you know my favourite X\", \"what did i say my\nfavourite X was\" - and it names the thing, so the answer is that one fact and not the whole\npicture.\n");
    s_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:so|and|but|oh|ah|well|also|btw|right|ok|okay|hmm)[\\s,.!;-]+)+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAYS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:my|our)\\s+(?:favou?rite|fav|best)\\s+(?P<thing>[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,2}?)\\s*(?:is|are|'s|was|will be|has to be|would be)\\s+(?P<value>[^?.!;]{1,60}?)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEONE_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:my|our)\\s+[a-z][a-z'\\u2019-]*'?s?\\s+(?:favou?rite|fav|best)\\b|\\b(?:his|her|their|your)\\s+(?:favou?rite|fav|best)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BEST_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bbest\\s+(?:friends?|mates?|man|buddy|bud|buddies|bestie|besties|pals?|friend's|mate's|friendship)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IS_MINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?P<value>[a-z][a-z'\\u2019 -]{1,40}?)\\s+(?:is|are|has always been|'s)\\s+my\\s+(?:favou?rite|fav|best)\\s+(?P<thing>[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,2})\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\bwhat(?:'s|s| is| was| are| were)?|\\bwho(?:'s|s| is| was| are| were)?|\\bwhich(?:\\s+is|\\s+one)?|\\bdo you (?:know|remember)|\\bdid i (?:say|tell you)|\\bcan you (?:remember|tell me))\\b[^?.!]{0,40}?\\bmy\\s+(?:favou?rite|fav|best)\\s+(?P<thing>[a-z][a-z'\\u2019-]*(?:\\s+(?!is\\b|was\\b|are\\b|were\\b)[a-z][a-z'\\u2019-]*){0,2})?(?:\\s+(?:is|was|again|then|though))?\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHICH_IS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*which\\s+(?P<thing>[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,2})\\s+(?:is|was)\\s+my\\s+(?:favou?rite|fav|best)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:no|nope|actually|sorry|wait|no wait|i mean|rather|scratch that|make that|not that|that's wrong|thats wrong)\\b[\\s,.!-]+(?:its|it's|it is|i mean|make it|i meant|just)\\s+(?P<value>[^?.!;]{1,60}?)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PREFERS_NOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:i|we)\\s+(?:prefer|like)\\s+(?P<value>[^?.!,;]{1,40}?)\\s*(?:now|nowadays|these days|better now|more now|better these days|more these days|better|more)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_ALL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which)\\b[^?.!]{0,30}?\\bmy\\s+(?:favou?rite|fav)s\\b|\\bwhat\\s+(?:have\\s+i|did\\s+i)\\s+(?:told|said\\s+to)\\s+you\\b[^?.!]{0,20}\\b(?:like|love|prefer)\\b|\\bwhat\\s+do\\s+you\\s+know\\s+about\\s+my\\s+(?:favou?rite|fav)s\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AN_ANSWER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*[a-z0-9][a-z0-9'\\u2019 ,-]{0,40}\\s*[.!]?\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s,]+(?:obviously|tbh|tho|though|really|lol|haha|i think|probably|at the moment|right now|for sure|of course)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ACKNOWLEDGES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nthanks thank thankyou cheers ta ok okay k cool nice great lovely brilliant right alright fine\nsure yes yeah yep no nope nah none nothing okthanks nothanks\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_VALUE_PHRASES(py_tuple(vec![py_str("not sure"), py_str("no idea"), py_str("dont know"), py_str("don't know"), py_str("dunno"), py_str("hard to say"), py_str("cant decide"), py_str("can't decide"), py_str("not really"), py_str("nothing really"), py_str("nothing much"), py_str("i dont know"), py_str("i don't know"), py_str("a few"), py_str("loads")]));
    s_A_STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nclosed shut off over broken rubbish injured cancelled canceled finished sold bought delayed\ndirty clean empty full busy open missing ill sick late early free full gone\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AN_EVENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\b(?:right\\s+now|for\\s+now|now|today|tomorrow|tonight|again)\\b|\\ball\\s+the\\s+time\\b|\\bat\\s+the\\s+(?:moment|minute)\\b|\\bthese\\s+days\\b|\\b(?:next|last|this)\\s+[a-z]+)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_VALUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nnothing none no idea not sure unsure duno dunno dontknow whatever anything something everything\nsame hard difficult tricky easy impossible complicated tricky unclear\nwhich what whats who whose when where why how\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:to|when|if|that|because|so|how|why|what|whether)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nto be do have go going going get got make making take taking give giving see seeing say saying\nthink thinking know knowing want wanting need needing like liking love loving hate hating\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nagain is was are were then though tho now please pls exactly already anyway\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_SOMETHING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:what|which|who|when|where|why|how|should|shall|can|could|would|will|do|does|did|is|are|any|anything|someone|suggest|give|tell|help|recommend)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STUCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:can'?t|cannot|can not|don'?t know|do not know|not sure|no idea|stuck|cant|dont)\\b[^,;.!?]{0,30}?\\b(?:decide|choose|pick|watch|cook|eat|drink|read|wear|do|get|make|start)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENTENCE_TOO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*[,;]\\s*|\\s+[\\u2014-]\\s*|[.!?]+\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_QUESTION_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[.!?]+\\s+(?=(?:what|which|who|when|where|why|how|should|shall|can|could|would|will|any|anything|do|does|did|is|are)\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s___all__(py_list(vec![py_str("read"), py_str("asked"), py_str("asks_all"), py_str("prefers_now"), py_str("an_answer"), py_str("corrected"), py_str("kept"), py_str("the_favourites"), py_str("store")]));
    s_KIN_APOSTROPHE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(your (?:mum|mom|mam|mother|dad|father|son|daughter|sister|brother|wife|husband|partner|nan|nana|granny|gran|grandma|grandad|grandpa|aunt|auntie|uncle|niece|nephew|cousin|boyfriend|girlfriend|fiance|fiancee|mate|friend|boss))s\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MY_WORDS(py_dict(vec![(py_str("myself"), py_str("yourself")), (py_str("my"), py_str("your")), (py_str("me"), py_str("you")), (py_str("i"), py_str("you")), (py_str("our"), py_str("your")), (py_str("us"), py_str("you"))]));
}
pub fn _plain(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut text: Py = Py::None;
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" \t,.;:!-—")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TAIL(), "sub", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" \t,.;:!-—")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) };
    Py::None
}
pub fn _a_value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether what follows \"is\" names a favourite rather than a clause or a shrug.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _plain(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)) } }) {
    return py_str("");
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NO_VALUE(), &low.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NO_VALUE(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NO_VALUE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&low.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&phrase.clone(), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "startswith", args))(&__args) } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CLAUSE(), "match", args))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_STATE(), &low.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_EVENT(), "search", args))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9]")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    return text.clone();
    Py::None
}
pub fn _a_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word(s) the favourite is of, kept lower-case, or \"\" when it names no thing.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" \t,.;:!-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)) } }) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_TAIL(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ASK_TAIL(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("thing"), py_str("things"), py_str("one")]), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("which"), py_str("what"), py_str("who")]), &geti(&words.clone(), &Py::Int(0)))) } } } }) {
    return py_str("");
    }
    return text.clone();
    Py::None
}
pub fn _clauses(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Each clause of the line, then the whole line: a favourite said after a comma.\n\n    A clause is tried first so that a comma between two favourites does not run one value into\n    the next (\"my favourite colour is green, my favourite food is curry\" kept the food as part\n    of the colour). The whole line is last, for a value that has a comma of its own.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    out = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*[,;]\\s*|\\s+[\\u2014-]\\s+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    out = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) {
    return op_add(&out.clone(), &py_list(vec![said.clone()]));
    }
    return py_list(vec![said.clone()]);
    Py::None
}
pub fn read(args: &[Py]) -> Py {
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
    let mut before_the_question: Py = Py::None;
    let mut found: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The favourite a line states: {\"thing\": \"colour\", \"value\": \"green\"}, or None.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asked(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_ELSE(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BEST_PERSON(), "search", args))(&__args) }) {
    return Py::None;
    }
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) };
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clauses(args))(&__args) };
    before_the_question = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION_AFTER(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) };
    if py_truthy(&{ let __b = before_the_question.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&before_the_question.clone(), &plain.clone())) } }) {
    parts = op_add(&py_list(vec![before_the_question.clone()]), &parts.clone());
    }
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    said = __item.clone();
    found = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAYS(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IS_MINE(), "match", args))(&__args) } } };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) };
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { value.clone() } }) {
    return py_dict(vec![(py_str("thing"), thing.clone()), (py_str("value"), value.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn asks_something_too(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line states a favourite and asks something else as well.\n\n    \"my favourite genre is comedy, what should i watch\" is one line doing two jobs: it is a fact\n    about the person and it is a request for a suggestion. The fact is kept either way - what the\n    store must not do is answer the line with the fact alone, leaving the question unanswered.\n    Only a clause other than the one that states the favourite counts, so a line whose question\n    is part of the value (\"my favourite film is whatever is on\") is not one of these.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE_TOO(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAYS(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IS_MINE(), "match", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&plain.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_SOMETHING(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STUCK(), "search", args))(&__args) } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn asked(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut low: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The thing a line asks their favourite of (\"colour\"), or None when it asks something else.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&low.clone(), &py_str("favourite"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&low.clone(), &py_str("favorite"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&low.clone(), &py_str(" fav"))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &py_str("your favourite"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("your favorite"))) } }) {
    return Py::None;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHICH_IS(), "search", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _singular(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut text: Py = Py::None;
    let _ = py_str("The thing with a plural \"s\" taken off, for \"what are my favourite films\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "endswith", args))(&__args) })) } } } }) {
    return slice_or_raise(&text.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return text.clone();
    Py::None
}
pub fn asks_all(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Whether the line asks for every favourite at once (\"what are my favourites\").");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &py_str("your favourite"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("your favorite"))) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_ALL(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn prefers_now(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The value a \"i prefer X now\" line puts in place of the favourite last named, or \"\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return py_str("");
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PREFERS_NOW(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    Py::None
}
pub fn an_answer(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut plain: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The value somebody gives when the bot has just asked them for a favourite, or \"\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&thing.clone())) } } } }) {
    return py_str("");
    }
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_ANSWER(), "match", args))(&__args) }))) {
    return py_str("");
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("!")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plain.clone(), "lower", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_ASK_TAIL(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_ACKNOWLEDGES(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thank")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    Py::None
}
pub fn corrected(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The value a correction puts in place of a favourite just given, or \"\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&thing.clone())) } } } }) {
    return py_str("");
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTS(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    Py::None
}
pub fn kept(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut held: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The favourite on record for that thing, or \"\" when they have not said one.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&thing.clone())) } }) {
    return py_str("");
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("favourites")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _singular(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { value.clone() } else { py_str("") };
    Py::None
}
pub fn the_favourites(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every favourite on record, in the order they were given.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_list(vec![]);
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("favourites")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "items", args))(&__args) }) { let __unpk7977 = py_unpack(&__item.clone(), 2, None); let mut thing: Py = __unpk7977[0].clone(); let mut value: Py = __unpk7977[1].clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(py_tuple(vec![thing.clone(), value.clone()])); } } py_list(__out) })() };
    Py::None
}
pub fn store(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Keep one favourite in the store, replacing the same thing said again.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) };
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&value.clone())) } }) {
    return Py::None;
    }
    known = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); __args.push(py_dict(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "setdefault", args))(&__args) };
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("favourites")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    held = py_dict(vec![]);
    }
    let _ = py_setitem(&held.clone(), &thing.clone(), value.clone());
    let _ = py_setitem(&known.clone(), &py_str("favourites"), held.clone());
    Py::None
}
pub fn their_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut low: Py = Py::None;
    let mut out: Py = Py::None;
    let mut swapped: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person's own words about themselves, said back: \"my mums curry\" -> \"your mum's curry\".");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "strip", args))(&__args) };
    swapped = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MY_WORDS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&swapped.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    continue;
    }
    tail = slice_or_raise(&word.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None);
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&swapped.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&swapped.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { swapped.clone() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    if py_truthy(&tail.clone()) {
    let __aug2 = op_add(&geti(&out.clone(), &op_neg(&Py::Int(1))), &tail.clone());
    let _ = py_setitem(&out.clone(), &op_neg(&Py::Int(1)), __aug2);
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\1's")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KIN_APOSTROPHE(), "sub", args))(&__args) };
    Py::None
}
pub fn watch(args: &[Py]) -> Py {
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
    let mut answered: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut pending: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("Keep the favourite a line names, answers, corrects or changes, and mark it for the reply.\n\n    Called from the `favourites` module's `observe` (bot/modules/favourites.py), which is the one\n    place a turn is read for a favourite: the reading is here, beside the slot it writes, so the\n    wording and the store stay in one file.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| read(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    pending = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("favourite_asked")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "pop", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pending.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { pending.clone() } }) {
    answered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(pending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| an_answer(args))(&__args) };
    if py_truthy(&answered.clone()) {
    said = py_dict(vec![(py_str("thing"), pending.clone()), (py_str("value"), answered.clone())]);
    let _ = py_setitem(&memory.clone(), &py_str("favourite_told"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    recent = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("favourite_recent")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(recent.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&recent.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&recent.clone(), "get", args))(&__args) }, &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) } } } }) {
    put_right = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(geti(&recent.clone(), &py_str("thing"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| corrected(args))(&__args) };
    if py_truthy(&put_right.clone()) {
    said = py_dict(vec![(py_str("thing"), geti(&recent.clone(), &py_str("thing"))), (py_str("value"), put_right.clone())]);
    let _ = py_setitem(&memory.clone(), &py_str("favourite_told"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    recent = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("favourite_recent")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(recent.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&recent.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_le(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&recent.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(2)) } } } }) {
    moved = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| prefers_now(args))(&__args) };
    if py_truthy(&{ let __b = moved.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&moved.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(geti(&recent.clone(), &py_str("thing"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| kept(args))(&__args) })) } }) {
    was = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(geti(&recent.clone(), &py_str("thing"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| kept(args))(&__args) };
    said = py_dict(vec![(py_str("thing"), geti(&recent.clone(), &py_str("thing"))), (py_str("value"), moved.clone())]);
    let _ = py_setitem(&memory.clone(), &py_str("favourite_changed"), py_dict(vec![(py_str("thing"), geti(&recent.clone(), &py_str("thing"))), (py_str("value"), moved.clone()), (py_str("was"), was.clone())]));
    }
    }
    }
    if py_truthy(&said.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| store(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = py_setitem(&memory.clone(), &py_str("favourite_recent"), py_dict(vec![(py_str("thing"), geti(&said.clone(), &py_str("thing"))), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    }
    }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.favourite".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACKNOWLEDGES" => Some(g_ACKNOWLEDGES()),
            "AN_ANSWER" => Some(g_AN_ANSWER()),
            "AN_EVENT" => Some(g_AN_EVENT()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_ALL" => Some(g_ASKS_ALL()),
            "ASKS_SOMETHING" => Some(g_ASKS_SOMETHING()),
            "ASK_TAIL" => Some(g_ASK_TAIL()),
            "A_CLAUSE" => Some(g_A_CLAUSE()),
            "A_QUESTION_AFTER" => Some(g_A_QUESTION_AFTER()),
            "A_STATE" => Some(g_A_STATE()),
            "BEST_PERSON" => Some(g_BEST_PERSON()),
            "CORRECTS" => Some(g_CORRECTS()),
            "IS_MINE" => Some(g_IS_MINE()),
            "KIN_APOSTROPHE" => Some(g_KIN_APOSTROPHE()),
            "LEAD" => Some(g_LEAD()),
            "MY_WORDS" => Some(g_MY_WORDS()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NO_VALUE" => Some(g_NO_VALUE()),
            "NO_VALUE_PHRASES" => Some(g_NO_VALUE_PHRASES()),
            "PREFERS_NOW" => Some(g_PREFERS_NOW()),
            "SAYS" => Some(g_SAYS()),
            "SENTENCE_TOO" => Some(g_SENTENCE_TOO()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "STUCK" => Some(g_STUCK()),
            "TAIL" => Some(g_TAIL()),
            "WHICH_IS" => Some(g_WHICH_IS()),
            "__all__" => Some(g___all__()),
            "_a_thing" => Some(py_func("_a_thing", |args: &[Py]| _a_thing(args))),
            "_a_value" => Some(py_func("_a_value", |args: &[Py]| _a_value(args))),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_singular" => Some(py_func("_singular", |args: &[Py]| _singular(args))),
            "an_answer" => Some(py_func("an_answer", |args: &[Py]| an_answer(args))),
            "asked" => Some(py_func("asked", |args: &[Py]| asked(args))),
            "asks_all" => Some(py_func("asks_all", |args: &[Py]| asks_all(args))),
            "asks_something_too" => Some(py_func("asks_something_too", |args: &[Py]| asks_something_too(args))),
            "corrected" => Some(py_func("corrected", |args: &[Py]| corrected(args))),
            "kept" => Some(py_func("kept", |args: &[Py]| kept(args))),
            "prefers_now" => Some(py_func("prefers_now", |args: &[Py]| prefers_now(args))),
            "read" => Some(py_func("read", |args: &[Py]| read(args))),
            "store" => Some(py_func("store", |args: &[Py]| store(args))),
            "the_favourites" => Some(py_func("the_favourites", |args: &[Py]| the_favourites(args))),
            "their_words" => Some(py_func("their_words", |args: &[Py]| their_words(args))),
            "watch" => Some(py_func("watch", |args: &[Py]| watch(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACKNOWLEDGES" => s_ACKNOWLEDGES(v),
            "AN_ANSWER" => s_AN_ANSWER(v),
            "AN_EVENT" => s_AN_EVENT(v),
            "ASKS" => s_ASKS(v),
            "ASKS_ALL" => s_ASKS_ALL(v),
            "ASKS_SOMETHING" => s_ASKS_SOMETHING(v),
            "ASK_TAIL" => s_ASK_TAIL(v),
            "A_CLAUSE" => s_A_CLAUSE(v),
            "A_QUESTION_AFTER" => s_A_QUESTION_AFTER(v),
            "A_STATE" => s_A_STATE(v),
            "BEST_PERSON" => s_BEST_PERSON(v),
            "CORRECTS" => s_CORRECTS(v),
            "IS_MINE" => s_IS_MINE(v),
            "KIN_APOSTROPHE" => s_KIN_APOSTROPHE(v),
            "LEAD" => s_LEAD(v),
            "MY_WORDS" => s_MY_WORDS(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NO_VALUE" => s_NO_VALUE(v),
            "NO_VALUE_PHRASES" => s_NO_VALUE_PHRASES(v),
            "PREFERS_NOW" => s_PREFERS_NOW(v),
            "SAYS" => s_SAYS(v),
            "SENTENCE_TOO" => s_SENTENCE_TOO(v),
            "SOMEONE_ELSE" => s_SOMEONE_ELSE(v),
            "STUCK" => s_STUCK(v),
            "TAIL" => s_TAIL(v),
            "WHICH_IS" => s_WHICH_IS(v),
            "__all__" => s___all__(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.favourite", module_obj);
    pyrt::xrefs::reg_fn("memory.favourite", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "_a_value", py_func("_a_value", |args: &[Py]| _a_value(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "_a_thing", py_func("_a_thing", |args: &[Py]| _a_thing(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "_clauses", py_func("_clauses", |args: &[Py]| _clauses(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "read", py_func("read", |args: &[Py]| read(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "asks_something_too", py_func("asks_something_too", |args: &[Py]| asks_something_too(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "asked", py_func("asked", |args: &[Py]| asked(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "_singular", py_func("_singular", |args: &[Py]| _singular(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "asks_all", py_func("asks_all", |args: &[Py]| asks_all(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "prefers_now", py_func("prefers_now", |args: &[Py]| prefers_now(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "an_answer", py_func("an_answer", |args: &[Py]| an_answer(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "corrected", py_func("corrected", |args: &[Py]| corrected(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "kept", py_func("kept", |args: &[Py]| kept(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "the_favourites", py_func("the_favourites", |args: &[Py]| the_favourites(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "store", py_func("store", |args: &[Py]| store(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "their_words", py_func("their_words", |args: &[Py]| their_words(args)));
    pyrt::xrefs::reg_fn("memory.favourite", "watch", py_func("watch", |args: &[Py]| watch(args)));
    pyrt::xrefs::reg_access("memory.favourite", "ACKNOWLEDGES", g_ACKNOWLEDGES, s_ACKNOWLEDGES);
    pyrt::xrefs::reg_access("memory.favourite", "AN_ANSWER", g_AN_ANSWER, s_AN_ANSWER);
    pyrt::xrefs::reg_access("memory.favourite", "AN_EVENT", g_AN_EVENT, s_AN_EVENT);
    pyrt::xrefs::reg_access("memory.favourite", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.favourite", "ASKS_ALL", g_ASKS_ALL, s_ASKS_ALL);
    pyrt::xrefs::reg_access("memory.favourite", "ASKS_SOMETHING", g_ASKS_SOMETHING, s_ASKS_SOMETHING);
    pyrt::xrefs::reg_access("memory.favourite", "ASK_TAIL", g_ASK_TAIL, s_ASK_TAIL);
    pyrt::xrefs::reg_access("memory.favourite", "A_CLAUSE", g_A_CLAUSE, s_A_CLAUSE);
    pyrt::xrefs::reg_access("memory.favourite", "A_QUESTION_AFTER", g_A_QUESTION_AFTER, s_A_QUESTION_AFTER);
    pyrt::xrefs::reg_access("memory.favourite", "A_STATE", g_A_STATE, s_A_STATE);
    pyrt::xrefs::reg_access("memory.favourite", "BEST_PERSON", g_BEST_PERSON, s_BEST_PERSON);
    pyrt::xrefs::reg_access("memory.favourite", "CORRECTS", g_CORRECTS, s_CORRECTS);
    pyrt::xrefs::reg_access("memory.favourite", "IS_MINE", g_IS_MINE, s_IS_MINE);
    pyrt::xrefs::reg_access("memory.favourite", "KIN_APOSTROPHE", g_KIN_APOSTROPHE, s_KIN_APOSTROPHE);
    pyrt::xrefs::reg_access("memory.favourite", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.favourite", "MY_WORDS", g_MY_WORDS, s_MY_WORDS);
    pyrt::xrefs::reg_access("memory.favourite", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.favourite", "NO_VALUE", g_NO_VALUE, s_NO_VALUE);
    pyrt::xrefs::reg_access("memory.favourite", "NO_VALUE_PHRASES", g_NO_VALUE_PHRASES, s_NO_VALUE_PHRASES);
    pyrt::xrefs::reg_access("memory.favourite", "PREFERS_NOW", g_PREFERS_NOW, s_PREFERS_NOW);
    pyrt::xrefs::reg_access("memory.favourite", "SAYS", g_SAYS, s_SAYS);
    pyrt::xrefs::reg_access("memory.favourite", "SENTENCE_TOO", g_SENTENCE_TOO, s_SENTENCE_TOO);
    pyrt::xrefs::reg_access("memory.favourite", "SOMEONE_ELSE", g_SOMEONE_ELSE, s_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("memory.favourite", "STUCK", g_STUCK, s_STUCK);
    pyrt::xrefs::reg_access("memory.favourite", "TAIL", g_TAIL, s_TAIL);
    pyrt::xrefs::reg_access("memory.favourite", "WHICH_IS", g_WHICH_IS, s_WHICH_IS);
    pyrt::xrefs::reg_access("memory.favourite", "__all__", g___all__, s___all__);
}
