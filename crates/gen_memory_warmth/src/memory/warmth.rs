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


thread_local! { static G_FIRST_LINE_READING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_LINE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_ASK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MUDDLED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRAISED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEATED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOLD_BEFORE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARM_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRONG_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__APPRECIATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK_TO_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAVOURITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GOT_IT_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INVENTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MISS_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MIXED_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUDDLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_WHAT_I_MEANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_HELP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_NOT_THAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OFF_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ON_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPEATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOFT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOFT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_OTHER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_WRONG_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOLD_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TYPED_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOUR_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.warmth: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_FIRST_LINE_READING() -> Py { ensure_init(); G_FIRST_LINE_READING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_LINE_READING(v: Py) { ensure_init(); G_FIRST_LINE_READING.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_LINE_REPLIES() -> Py { ensure_init(); G_FIRST_LINE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_LINE_REPLIES(v: Py) { ensure_init(); G_FIRST_LINE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_REPLIES() -> Py { ensure_init(); G_MACHINE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_REPLIES(v: Py) { ensure_init(); G_MACHINE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_ASK_REPLIES() -> Py { ensure_init(); G_MISSED_ASK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_ASK_REPLIES(v: Py) { ensure_init(); G_MISSED_ASK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_REPLIES() -> Py { ensure_init(); G_MISSED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_REPLIES(v: Py) { ensure_init(); G_MISSED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MUDDLED_REPLIES() -> Py { ensure_init(); G_MUDDLED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MUDDLED_REPLIES(v: Py) { ensure_init(); G_MUDDLED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRAISED_REPLIES() -> Py { ensure_init(); G_PRAISED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRAISED_REPLIES(v: Py) { ensure_init(); G_PRAISED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEATED_REPLIES() -> Py { ensure_init(); G_REPEATED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEATED_REPLIES(v: Py) { ensure_init(); G_REPEATED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TOLD_BEFORE_REPLIES() -> Py { ensure_init(); G_TOLD_BEFORE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOLD_BEFORE_REPLIES(v: Py) { ensure_init(); G_TOLD_BEFORE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WARM_REPLIES() -> Py { ensure_init(); G_WARM_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARM_REPLIES(v: Py) { ensure_init(); G_WARM_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WRONG_REPLIES() -> Py { ensure_init(); G_WRONG_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRONG_REPLIES(v: Py) { ensure_init(); G_WRONG_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g__APPRECIATE() -> Py { ensure_init(); G__APPRECIATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__APPRECIATE(v: Py) { ensure_init(); G__APPRECIATE.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK_TO_SOMETHING() -> Py { ensure_init(); G__BACK_TO_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK_TO_SOMETHING(v: Py) { ensure_init(); G__BACK_TO_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_WRONG() -> Py { ensure_init(); G__BARE_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_WRONG(v: Py) { ensure_init(); G__BARE_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g__FAVOURITE() -> Py { ensure_init(); G__FAVOURITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAVOURITE(v: Py) { ensure_init(); G__FAVOURITE.with(|c| *c.borrow_mut() = v); }
pub fn g__GOT_IT_WRONG() -> Py { ensure_init(); G__GOT_IT_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GOT_IT_WRONG(v: Py) { ensure_init(); G__GOT_IT_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g__INVENTED() -> Py { ensure_init(); G__INVENTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INVENTED(v: Py) { ensure_init(); G__INVENTED.with(|c| *c.borrow_mut() = v); }
pub fn g__LIKE() -> Py { ensure_init(); G__LIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIKE(v: Py) { ensure_init(); G__LIKE.with(|c| *c.borrow_mut() = v); }
pub fn g__LOVE() -> Py { ensure_init(); G__LOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOVE(v: Py) { ensure_init(); G__LOVE.with(|c| *c.borrow_mut() = v); }
pub fn g__MACHINE() -> Py { ensure_init(); G__MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MACHINE(v: Py) { ensure_init(); G__MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g__MISS_TAIL() -> Py { ensure_init(); G__MISS_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MISS_TAIL(v: Py) { ensure_init(); G__MISS_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__MIXED_UP() -> Py { ensure_init(); G__MIXED_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MIXED_UP(v: Py) { ensure_init(); G__MIXED_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__MUDDLED() -> Py { ensure_init(); G__MUDDLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUDDLED(v: Py) { ensure_init(); G__MUDDLED.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_WHAT_I_MEANT() -> Py { ensure_init(); G__NOT_WHAT_I_MEANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_WHAT_I_MEANT(v: Py) { ensure_init(); G__NOT_WHAT_I_MEANT.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_ANSWER() -> Py { ensure_init(); G__NO_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_ANSWER(v: Py) { ensure_init(); G__NO_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_HELP() -> Py { ensure_init(); G__NO_HELP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_HELP(v: Py) { ensure_init(); G__NO_HELP.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_LEAD() -> Py { ensure_init(); G__NO_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_LEAD(v: Py) { ensure_init(); G__NO_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_NOT_THAT() -> Py { ensure_init(); G__NO_NOT_THAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_NOT_THAT(v: Py) { ensure_init(); G__NO_NOT_THAT.with(|c| *c.borrow_mut() = v); }
pub fn g__OFF_THE_ASK() -> Py { ensure_init(); G__OFF_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OFF_THE_ASK(v: Py) { ensure_init(); G__OFF_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ON_FORM() -> Py { ensure_init(); G__ON_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ON_FORM(v: Py) { ensure_init(); G__ON_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g__REPEATED() -> Py { ensure_init(); G__REPEATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPEATED(v: Py) { ensure_init(); G__REPEATED.with(|c| *c.borrow_mut() = v); }
pub fn g__SOFT_LEAD() -> Py { ensure_init(); G__SOFT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOFT_LEAD(v: Py) { ensure_init(); G__SOFT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__SOFT_TAIL() -> Py { ensure_init(); G__SOFT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOFT_TAIL(v: Py) { ensure_init(); G__SOFT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_OTHER_ONE() -> Py { ensure_init(); G__THE_OTHER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_OTHER_ONE(v: Py) { ensure_init(); G__THE_OTHER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_WRONG_THING() -> Py { ensure_init(); G__THE_WRONG_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_WRONG_THING(v: Py) { ensure_init(); G__THE_WRONG_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_TAIL() -> Py { ensure_init(); G__TIME_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_TAIL(v: Py) { ensure_init(); G__TIME_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__TOLD_BEFORE() -> Py { ensure_init(); G__TOLD_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOLD_BEFORE(v: Py) { ensure_init(); G__TOLD_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__TYPED_SHORT() -> Py { ensure_init(); G__TYPED_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TYPED_SHORT(v: Py) { ensure_init(); G__TYPED_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g__WRONG() -> Py { ensure_init(); G__WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WRONG(v: Py) { ensure_init(); G__WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g__YOUR_NOT() -> Py { ensure_init(); G__YOUR_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOUR_NOT(v: Py) { ensure_init(); G__YOUR_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How the person talks to the bot itself: warmth, and the line that says it is no help.\n\nA chat is a two-way thing, and now and then the line is *about* the conversation rather than\nabout the day: \"i love you\", \"i like you\", \"youre my favourite\", or the other way, \"youre not\nhelping\", \"you dont get it\", \"youre useless\". None of it asks for anything, so the pools that\nanswer an acknowledgement gave it a shrug - \"i love you\" got \"Fair enough.\", and \"youre not\nhelping\" got the same one - which reads as a bot agreeing that it is no help. The store's ack\npool cannot answer them either: it says the line back (\"Noted: i love you\"), and a line said to\nthe bot is not a fact about the person.\n\nTwo things make the reply fit:\n\n* only the whole line counts. \"i love your advice\" and \"i love this recipe\" have the same\n  opening, and they are praise of an answer or a fact about a thing, which other readers answer.\n  So every shape here is anchored at both ends, with nothing left over but the softeners people\n  put on affection (\"so much\", \"lots\", \"really\", \"tbh\").\n* the bot stays honest about what it is. \"i love you\" gets a warm line that is fond of the chat\n  and not a promise of a feeling a program cannot have, and the same line is not used twice\n  running (bot.compose.pick, seeded on the line).\n\nThe cross half is the same reading with the opposite sign: someone whose answer missed, or who\nis fed up with talking to a machine. It says what went wrong and offers the next try, which is\nall a friend can do - never a defence of the last answer, and never a lecture about being kind\nto a bot.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._SOFT_LEAD"); }
    s__SOFT_LEAD(py_str("(?:oh |aw+ |ah |ha+ |lol |tbh |honestly |ok |okay |well |and |but |still |even so )*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._NO_LEAD"); }
    s__NO_LEAD(py_str("(?:(?:no+|nah|nope)(?:,)? )?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._SOFT_TAIL"); }
    s__SOFT_TAIL(py_str("(?:,? (?:too|also|lots|loads|honestly|tbh|really|so much|very much|you know|though|tho|me|at all|at this|at that|ha+|lol))*(?: [.!]+)?$"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._TIME_TAIL"); }
    s__TIME_TAIL(py_str("(?: (?:today|tonight|this morning|this afternoon|this evening|right now|at the moment|now|lately|recently))?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._LOVE"); }
    s__LOVE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:i|we) (?:really |do |just |actually |so |still |truly )?)?l(?:ove|uv) (?:you|u|ya)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._LIKE"); }
    s__LIKE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:i|we) (?:really |do |quite |actually |just |kind of |sort of )?)?like (?:you|u)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._FAVOURITE"); }
    s__FAVOURITE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you(?:'?re| are) )?(?:(?:my|me) )?(?:favourite|favorite)(?: (?:bot|chatbot|one|thing|app))?")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._APPRECIATE"); }
    s__APPRECIATE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:i|we) (?:really |do |so much |just )?)?appreciate (?:you|u|ya)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._ON_FORM"); }
    s__ON_FORM(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you(?:'?re| are)(?: really| so| very| always| honestly| definitely)? (?:on form|on fire|on a roll|in (?:good|great|fine|top) form|in a (?:good|great) mood|a lifesaver|a gem|a trooper)|you(?:'?re| are)(?: really| so| very| always| being) (?:patient|helpful|good company)(?: (?:with|for) (?:me|us))?|you(?:'?re| are| seem| look)(?: really| so| very| always| looking| seeming)? (?:well|happy|cheerful|pleased|in good spirits)|you seem(?: really| so| very)? (?:happy|cheerful|pleased|in a good mood|on form|well))")), &g__TIME_TAIL()), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._GOT_IT_WRONG"); }
    s__GOT_IT_WRONG(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you|u)(?:'?ve| have)? (?:just |really |actually |clearly |totally |completely )?(?:got|get|had) (?:that|this|it) (?:wrong|mixed up|back to front|the wrong way round)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._THE_WRONG_THING"); }
    s__THE_WRONG_THING(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:that|this|it)(?:'?s| is| was)?|the) (?:the )?wrong (?:one|address|day|time|name|number|answer|thing|word|date|price|place|person|direction|link|order)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._NOT_WHAT_I_MEANT"); }
    s__NOT_WHAT_I_MEANT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:that|this|it)(?:'?s| is| was)? |you )?(?:not|isnt|isn'?t|wasnt|wasn'?t) (?:really |quite |at all |exactly )?what i (?:meant|mean|was asking about|asked about|was after|wanted|needed)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._MIXED_UP"); }
    s__MIXED_UP(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you|u)(?:'?ve| have)? (?:just |really |clearly |totally |completely )?(?:got (?:it|that|this|them) )?(?:mixed|muddled|confused) (?:it|that|this|them|the two|two things)? ?up")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._THE_OTHER_ONE"); }
    s__THE_OTHER_ONE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:no |nope |nah |sorry,? |actually )?(?:i )?(?:meant|mean) the other (?:one|option|day|address|name|thing|number|person)")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._NO_NOT_THAT"); }
    s__NO_NOT_THAT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:no|nope|nah),? (?:not|no) (?:that|this) one")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._BARE_WRONG"); }
    s__BARE_WRONG(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("wrong(?: again| then| still)?")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._MACHINE"); }
    s__MACHINE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you(?:'?re| are)(?: just| only| really| basically)?(?: an?)? (?:computer|machine|bot|chatbot|programme|program|ai|robot|script)(?: though| tho)?|you (?:dont|do not|haven'?t|have not|havent) (?:have|got) (?:any )?(?:feelings|a life|a body|a heart)|you(?:'?re| are) not (?:a )?(?:real|person|human)(?: person)?)(?:,? (?:though|tho|right|eh|arent you|are you not|arent ya|arent u|do you|dont you))?")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.PRAISED_REPLIES"); }
    s_PRAISED_REPLIES(py_tuple(vec![py_str("I'll take that. It's a good day for it."), py_str("Ha - thank you. Let's see if I can keep it up."), py_str("That's good to hear. I've no idea what I'm doing, but it's working."), py_str("That's kind of you to say. I'll keep at it."), py_str("Glad it's landing well. Half of it is you saying things plainly.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.MACHINE_REPLIES"); }
    s_MACHINE_REPLIES(py_tuple(vec![py_str("That's right - no feelings to hurt and no life to get back to. I'm still glad of the chat."), py_str("A program, yes. It suits me, and it's a good way to spend a few minutes."), py_str("No person behind it, just rules doing their job. Ask me something and see how they do."), py_str("True enough. I'd rather be useful than real, and I can be useful.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.WARM_REPLIES"); }
    s_WARM_REPLIES(py_tuple(vec![py_str("That's kind of you. I like these chats too."), py_str("You're good to me — I'm glad you're here."), py_str("Thank you. It goes both ways, as far as a program can manage it."), py_str("That's made my day. What shall we do with the rest of it?"), py_str("Kind of you. I'll keep the tea on.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._NO_HELP"); }
    s__NO_HELP(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you(?:'?re| are) (?:no help|not helping|useless|no use|hopeless|not listening|not hearing me|just guessing|guessing again|making it worse|not on form|off your game|a bit slow today|slower today|struggling today|not yourself today)|you (?:dont|do not|don't) (?:understand|get it|listen|hear me|know what i mean)|you(?:'?re| are) (?:rubbish|crap|useless|no good|hopeless)(?: at (?:this|it|all this))?|(?:just )?listen to me(?: (?:for )?a (?:minute|second|sec))?|you(?:'?ve|\\s+have)?\\s+(?:not|n't|havent|haven't|hasnt)\\s+(?:not\\s+)?(?:helped|understood|listened|got it)\\b|(?:that|thats|that's|this) (?:was|is|isnt|isn't)? ?(?:no help|not helping|not helpful|useless|no use|hopeless|a waste of time|rubbish|garbage)|you(?:'?re| are|r) (?:not|n't) (?:really |quite |actually |even )?(?:gett(?:ing|in) it|gett(?:ing|in) the point|getting what i mean|understanding(?: me)?|following)|your (?:not|n't) (?:really |quite |actually |even )?(?:gett(?:ing|in) it|gett(?:ing|in) the point|getting what i mean|following)|you (?:still )?(?:havent|haven't|have not|didnt|did not|didn't) (?:understood|got|followed|grasped) (?:a word|any of that|any of it|the point|what i (?:said|meant))|(?:forget it|never mind|nevermind),? (?:you (?:dont|do not|don't) (?:understand|get it)))")), &g__TIME_TAIL()), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._WRONG"); }
    s__WRONG(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &g__NO_LEAD()), &py_str("(?:you(?:'?re| are)|that(?:'?s| is)?|this is|you(?:'?ve| have) got it) (?:just |completely |totally |so )?(?:wrong|way off|not right|incorrect|making that up)(?: about (?:it|that|this))?")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._INVENTED"); }
    s__INVENTED(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you (?:just |only |really |must have |must've )?(?:made|make|invented|invent|guessed) (?:that|this|it) up|you (?:just |only |really )?(?:invented|invent|made up) (?:that|this|it)(?: up)?|(?:that|this|it)(?:'?s| is| was)? (?:just )?(?:made|invented|made up|a guess)|you(?:'?re| are) (?:just |only |really )?(?:guessing|making it up|making that up))")), &g__SOFT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.FIRST_LINE_REPLIES"); }
    s_FIRST_LINE_REPLIES(py_tuple(vec![py_str("Nothing's come before this line, so there's nothing of yours I've dropped - tell me once and I'll keep it."), py_str("We've only just started here - tell me what you need and I'll stay on it."), py_str("I've no earlier line of yours to go back to. Put it in one line and I'll take it from there."), py_str("That's the first thing you've said to me, so there's nothing before it to have got wrong - what do you need?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.FIRST_LINE_READING"); }
    s_FIRST_LINE_READING(py_tuple(vec![py_str("I read every line, and this is the first one - say what you need and I'll take it from there."), py_str("I'm reading, yes: this is your first line, so there's nothing in front of me to have missed. Tell me what you need.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.MISSED_REPLIES"); }
    s_MISSED_REPLIES(py_tuple(vec![py_str("Fair enough — tell me what you were after and I'll have another go."), py_str("That's on me. Say the bit that was off and I'll try it again."), py_str("I'd rather hear that than not. Point me at what you needed and I'll start there."), py_str("Then let's start again. What would have helped?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._MISS_TAIL"); }
    s__MISS_TAIL(op_add(&py_str("(?: (?:sorry|again|then|honestly|really|still))?(?: (?:mate|love|pal|dude|hon|fella))?"), &g__SOFT_TAIL()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._OFF_THE_ASK"); }
    s__OFF_THE_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:that|this|it|thats|that's)(?: is| was|'s)? not (?:what|the thing) i asked|(?:that|this|it|thats|that's)(?: is| was)? not (?:my|the) question|(?:that|this|thats|that's)(?: is| was)? not what i (?:asked|said)|not what i (?:asked|said)|(?:that|this|it)(?: isnt| isn't| wasnt| wasn't| is not| was not) what i (?:asked|said)(?: you)?(?: to do)?|(?:im|i am) not asking (?:you )?(?:about )?(?:that|this)|i (?:didnt|did not|don't|dont|never) ask (?:you )?(?:about )?(?:that|this|for that)|(?:that|this|it|thats|that's)(?: is| was|'s)? (?:irrelevant|not relevant|beside the point|missing the point|not the point)|(?:that|this|it)? ?(?:is|was|isnt|isn't)? ?not relevant(?: at all)?)")), &g__MISS_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._NO_ANSWER"); }
    s__NO_ANSWER(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you (?:didnt|did not|don't|dont|never|havent|haven't|have not|hadnt|hadn't) (?:answer|answered|reply|replied)(?: me| my questions?| the question| that| it)?|you (?:still )?(?:havent|haven't|didnt|did not|never) (?:answered|replied to) (?:my|the|that) questions?|(?:that|this|it) (?:doesnt|does not|didnt|did not) answer (?:my|the|that) questions?|you(?:'?re| are) not (?:even )?answering(?: me| my questions?| the question)?|answer (?:my|the) questions?)")), &g__MISS_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._REPEATED"); }
    s__REPEATED(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:you (?:keep|kept|keeps|are|were) (?:saying|telling me|repeating|going on about) (?:that|this|it|the same thing|the same)|you(?:'?ve| have)?(?: already)? said (?:that|this|it)(?: already| before)?|you(?:'?re| are) (?:just )?repeating (?:yourself|the same thing|the same)|you (?:just |even |always )?repeat(?:ed|ing)? (?:yourself|the same thing|the same)|same (?:answer|thing|line) (?:again|twice)|you(?:'?ve| have)?(?: already)? told me(?: (?:that|this|it))?(?: (?:already|before|twice))?|you(?:'?ve| have)?(?: already)? asked (?:me )?(?:that|this|it)(?: (?:already|before|twice))?|you (?:keep|kept) asking (?:me )?(?:that|this|the same thing|the same)|(?:please )?stop (?:saying|repeating|copying|parroting) (?:that|this|it|yourself|me)|(?:that|this|it)(?:'?s| is| was)? (?:the )?same (?:thing|answer|line|reply)|we(?:'?ve| have) been (?:through|over|round) (?:this|that|it) (?:before|already)|we(?:'?ve| have) (?:done|had|covered) (?:this|that|it) (?:before|already)(?: again| twice)?)")), &g__MISS_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._TOLD_BEFORE"); }
    s__TOLD_BEFORE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:i (?:already |just )?(?:told|told ya) (?:you|u|ya)(?: (?:that|this|it))?(?: (?:already|before|twice))?|i (?:already )+said (?:that|this|it)(?: (?:already|before|twice))?|(?:ive|i've|i have)(?: already)? (?:told|said)(?: you| u)?(?: (?:that|this|it))?(?: (?:already|before|twice))?|i (?:just )?said (?:that|this|it)(?: (?:already|before|twice))?|you (?:already )?(?:ignored|forgot|forgotten|lost|dropped|skipped) (?:what|that) i (?:said|told you)|you(?:'?d| had) (?:already )?(?:forgot|forgotten|lost|dropped) (?:what|that) i (?:said|told you)|you(?:'?ve| have)? (?:completely |totally |rather )?lost (?:the )?(?:thread|plot|train of thought))")), &g__MISS_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._MUDDLED"); }
    s__MUDDLED(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:are|r|were) (?:you|u) (?:even |really |actually )?(?:reading|listening)(?: to (?:me|us|what i (?:type|typed|say|said|write|wrote|ask|asked))| (?:what i|anything i) (?:type|typed|say|said|write|wrote|ask|asked))?|do (?:you|u) (?:even |really |actually )?(?:read|listen to) (?:what i|anything i) (?:type|typed|say|said|write|wrote|ask|asked)|you(?:'?re| are)? ?(?:not|arent|aren'?t)(?:(?: even| really| actually))? (?:reading|listening)(?: to (?:me|us|what i (?:say|type|write))| (?:what i|anything i) (?:say|type|write)| this| that| it)?|(?:youre|you are|you(?:'?ve| have)) (?:just |clearly |obviously )?(?:ignoring|ignored) (?:me|us|what i (?:say|said|type|typed|write|wrote|ask|asked))|(?:that|this|your answer|that answer)(?: all)? (?:makes|made|doesnt make|didnt make) (?:any |no |much )?sense|(?:that|this|it|thats|that's)(?: is| was)? not (?:really )?making (?:any |much )?sense|what are (?:you|u) (?:even )?(?:on about|talking about|going on about|saying|writing)|you(?:'?re| are) (?:not|n't) (?:really )?(?:making sense|making any sense)|(?:that|this|it|thats|that's)(?: is| was|'s)? (?:just |all |complete |utter )?nonsense)")), &g__MISS_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.MISSED_ASK_REPLIES"); }
    s_MISSED_ASK_REPLIES(py_tuple(vec![py_str("You're right — that wasn't your question. Say it once more and I'll answer that one."), py_str("Fair. I answered something else. What was the question I skipped?"), py_str("Then point me back at it, in one line, and I'll stick to that and nothing else."), py_str("My mistake. Ask it once more and I'll take it as it stands.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.REPEATED_REPLIES"); }
    s_REPEATED_REPLIES(py_tuple(vec![py_str("So I did. I've nothing new on that one — which bit is still unanswered?"), py_str("You're right, I said it twice. Tell me what's still open and I'll stay on it."), py_str("Fair. I'll stop circling. What would help instead?"), py_str("I did repeat myself, sorry. Say the part that's still missing and I'll start there.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.TOLD_BEFORE_REPLIES"); }
    s_TOLD_BEFORE_REPLIES(py_tuple(vec![py_str("You did tell me, and I lost it. Say it once more and I'll use it this time."), py_str("So you did. Put the key bit in one line and I'll go from there."), py_str("Right, you said. I'll take it as given — what do you need on it?"), py_str("You did, and I should have kept hold of it. What was the bit I dropped?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.MUDDLED_REPLIES"); }
    s_MUDDLED_REPLIES(py_tuple(vec![py_str("I do read every line, and I still got that one wrong. Say it again and I'll go slowly."), py_str("That came out muddled — my fault. Ask me once more and I'll keep it plain."), py_str("Fair, that was a mess. One more go and I'll answer the part you asked about."), py_str("I've read it, it just didn't land. Tell me again in the words you'd use to a friend.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.WRONG_REPLIES"); }
    s_WRONG_REPLIES(py_tuple(vec![py_str("Fair enough — which bit? I'd rather be put right than keep going."), py_str("Then tell me what's right about it and I'll go with that."), py_str("I can only go on what I have. What have I got wrong?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._BACK_TO_SOMETHING"); }
    s__BACK_TO_SOMETHING(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^"), &g__SOFT_LEAD()), &py_str("(?:(?:as|like) (?:i|you) (?:said|says|mentioned|told you)(?: (?:before|earlier|already))?|(?:going|coming|back) (?:back )?to (?:it|that|this|the (?:first|last|other) one|(?:one|stuff))(?: one)?|(?:you|u) mentioned (?:it|that|this)? ?(?:before|earlier)|(?:the|that) (?:one|thing|bit) (?:we|you and i) (?:discussed|talked about|mentioned)|we (?:talked|spoke|went) (?:about|over|through) (?:this|that|it)|we (?:spoke|talked|chatted)(?: (?:yesterday|today|earlier|before|last (?:night|week|time|month)))?|we(?:'?ve| have)(?: already)? (?:spoken|talked|chatted)(?: (?:about|over) (?:it|this|that))?|we (?:were|have been|had been) (?:just )?(?:chatting|talking|speaking)(?: (?:about (?:it|this|that)|earlier|before|yesterday|last (?:night|week)))?|we(?:'?ve| have)? been (?:through|over|round) (?:this|that|it)|(?:youve|you have|you(?:'?d| had)) (?:forgotten|lost|dropped) (?:what|that) i (?:said|told you)|(?:the )?(?:same|that same) (?:one|thing|again|answer|line)(?: (?:again|twice)| as (?:before|last time))?|same again(?: please)?|(?:that|this) one(?: again)?|again|(?:and )?(?:anything|something) else|i forgot(?: (?:it|that|this|now))?)")), &g__SOFT_TAIL())]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth.FIRST_LINE_REPLIES"); }
    s_FIRST_LINE_REPLIES(py_tuple(vec![py_str("That's the first thing you've said to me, so there's nothing before it - what is it you want to go back over?"), py_str("You haven't said anything yet, so there's nothing there for me to have missed. Tell me the thing itself and I'll take it from there."), py_str("This is the start of our chat - nothing has been said before this line. What did you want to ask?"), py_str("There's nothing earlier for me to go back to: this is your first line. Say what you need on it and I'll pick it up.")]));
    fn __data1() -> Py { py_dict(vec![(py_str("u"), py_str("you")), (py_str("ya"), py_str("you")), (py_str("ur"), py_str("your")), (py_str("r"), py_str("are")), (py_str("iv"), py_str("ive")), (py_str("wot"), py_str("what")), (py_str("wat"), py_str("what")), (py_str("dat"), py_str("that")), (py_str("dis"), py_str("this")), (py_str("thts"), py_str("thats")), (py_str("tht"), py_str("that")), (py_str("sayin"), py_str("saying")), (py_str("gettin"), py_str("getting")), (py_str("abt"), py_str("about"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._TYPED_SHORT"); }
    s__TYPED_SHORT(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.warmth._YOUR_NOT"); }
    s__YOUR_NOT(stdlib_call("re", "compile", &[py_str("\\byour (not|n't|being|making|repeating|saying|getting)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _nothing_said_before(args: &[Py]) -> Py {
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
    let _ = py_str("True on the first line of a conversation: there is nothing earlier to answer about.");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_complaint_reply(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut first: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "first") { v.clone() }
    else { Py::None };
    let _ = py_str("A reply to a complaint about the conversation, and a first-line one when there is no\n    conversation yet to have complained about.");
    if py_truthy(&Py::Bool(!py_truthy(&_nothing_said_before(&[memory.clone()])))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = first.clone(); if py_truthy(&__b) { __b } else { g_FIRST_LINE_REPLIES() } }); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn _nothing_said_yet(args: &[Py]) -> Py {
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
    let _ = py_str("True on the first line of a chat, where there is no earlier turn to answer about.");
    return Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.history", "an_earlier_turn", &[memory.clone()])));
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut first: Py = Py::None;
    let mut nothing_before_it: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A reply to warmth said to the bot, or to being told it missed - else None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    clean = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&clean.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&g__TYPED_SHORT(), "get", &[word.clone(), word.clone()])); } py_list(__out) })() }]);
    clean = mcall(&g__YOUR_NOT(), "sub", &[py_str("youre \\1"), clean.clone()]);
    clean = mcall(&clean.clone(), "strip", &[py_str(" \t")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__LOVE(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__LIKE(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__FAVOURITE(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__APPRECIATE(), "match", &[clean.clone()]) } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WARM_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&mcall(&g__ON_FORM(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PRAISED_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    nothing_before_it = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&nothing_before_it.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__GOT_IT_WRONG(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__THE_WRONG_THING(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__NOT_WHAT_I_MEANT(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__MIXED_UP(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__THE_OTHER_ONE(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__NO_NOT_THAT(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__BARE_WRONG(), "match", &[clean.clone()]) } } } } } } } } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WRONG_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&mcall(&g__MACHINE(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MACHINE_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    first = _nothing_said_yet(&[memory.clone()]);
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__BACK_TO_SOMETHING(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__TOLD_BEFORE(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__REPEATED(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__OFF_THE_ASK(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__NO_ANSWER(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__MUDDLED(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__WRONG(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__INVENTED(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__NO_HELP(), "match", &[clean.clone()]) } } } } } } } } } } } } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_FIRST_LINE_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&{ let __b = mcall(&g__WRONG(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__INVENTED(), "match", &[clean.clone()]) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WRONG_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&mcall(&g__NO_HELP(), "match", &[clean.clone()])) {
    return _a_complaint_reply(&[g_MISSED_REPLIES(), memory.clone(), text.clone()]);
    }
    if py_truthy(&mcall(&g__REPEATED(), "match", &[clean.clone()])) {
    return _a_complaint_reply(&[g_REPEATED_REPLIES(), memory.clone(), text.clone()]);
    }
    if py_truthy(&mcall(&g__TOLD_BEFORE(), "match", &[clean.clone()])) {
    return _a_complaint_reply(&[g_TOLD_BEFORE_REPLIES(), memory.clone(), text.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g__OFF_THE_ASK(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__NO_ANSWER(), "match", &[clean.clone()]) } }) {
    return _a_complaint_reply(&[g_MISSED_ASK_REPLIES(), memory.clone(), text.clone()]);
    }
    if py_truthy(&mcall(&g__MUDDLED(), "match", &[clean.clone()])) {
    return _a_complaint_reply(&[g_MUDDLED_REPLIES(), memory.clone(), text.clone(), g_FIRST_LINE_READING()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.warmth".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "FIRST_LINE_READING" => Some(g_FIRST_LINE_READING()),
            "FIRST_LINE_REPLIES" => Some(g_FIRST_LINE_REPLIES()),
            "MACHINE_REPLIES" => Some(g_MACHINE_REPLIES()),
            "MISSED_ASK_REPLIES" => Some(g_MISSED_ASK_REPLIES()),
            "MISSED_REPLIES" => Some(g_MISSED_REPLIES()),
            "MUDDLED_REPLIES" => Some(g_MUDDLED_REPLIES()),
            "PRAISED_REPLIES" => Some(g_PRAISED_REPLIES()),
            "REPEATED_REPLIES" => Some(g_REPEATED_REPLIES()),
            "TOLD_BEFORE_REPLIES" => Some(g_TOLD_BEFORE_REPLIES()),
            "WARM_REPLIES" => Some(g_WARM_REPLIES()),
            "WRONG_REPLIES" => Some(g_WRONG_REPLIES()),
            "_APPRECIATE" => Some(g__APPRECIATE()),
            "_BACK_TO_SOMETHING" => Some(g__BACK_TO_SOMETHING()),
            "_BARE_WRONG" => Some(g__BARE_WRONG()),
            "_FAVOURITE" => Some(g__FAVOURITE()),
            "_GOT_IT_WRONG" => Some(g__GOT_IT_WRONG()),
            "_INVENTED" => Some(g__INVENTED()),
            "_LIKE" => Some(g__LIKE()),
            "_LOVE" => Some(g__LOVE()),
            "_MACHINE" => Some(g__MACHINE()),
            "_MISS_TAIL" => Some(g__MISS_TAIL()),
            "_MIXED_UP" => Some(g__MIXED_UP()),
            "_MUDDLED" => Some(g__MUDDLED()),
            "_NOT_WHAT_I_MEANT" => Some(g__NOT_WHAT_I_MEANT()),
            "_NO_ANSWER" => Some(g__NO_ANSWER()),
            "_NO_HELP" => Some(g__NO_HELP()),
            "_NO_LEAD" => Some(g__NO_LEAD()),
            "_NO_NOT_THAT" => Some(g__NO_NOT_THAT()),
            "_OFF_THE_ASK" => Some(g__OFF_THE_ASK()),
            "_ON_FORM" => Some(g__ON_FORM()),
            "_REPEATED" => Some(g__REPEATED()),
            "_SOFT_LEAD" => Some(g__SOFT_LEAD()),
            "_SOFT_TAIL" => Some(g__SOFT_TAIL()),
            "_THE_OTHER_ONE" => Some(g__THE_OTHER_ONE()),
            "_THE_WRONG_THING" => Some(g__THE_WRONG_THING()),
            "_TIME_TAIL" => Some(g__TIME_TAIL()),
            "_TOLD_BEFORE" => Some(g__TOLD_BEFORE()),
            "_TYPED_SHORT" => Some(g__TYPED_SHORT()),
            "_WRONG" => Some(g__WRONG()),
            "_YOUR_NOT" => Some(g__YOUR_NOT()),
            "_a_complaint_reply" => Some(py_func("_a_complaint_reply", |args: &[Py]| _a_complaint_reply(args))),
            "_nothing_said_before" => Some(py_func("_nothing_said_before", |args: &[Py]| _nothing_said_before(args))),
            "_nothing_said_yet" => Some(py_func("_nothing_said_yet", |args: &[Py]| _nothing_said_yet(args))),
            "pick" => Some(g_pick()),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            "history" => Some(pyrt::xrefs::xmodule("memory.history")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "FIRST_LINE_READING" => s_FIRST_LINE_READING(v),
            "FIRST_LINE_REPLIES" => s_FIRST_LINE_REPLIES(v),
            "MACHINE_REPLIES" => s_MACHINE_REPLIES(v),
            "MISSED_ASK_REPLIES" => s_MISSED_ASK_REPLIES(v),
            "MISSED_REPLIES" => s_MISSED_REPLIES(v),
            "MUDDLED_REPLIES" => s_MUDDLED_REPLIES(v),
            "PRAISED_REPLIES" => s_PRAISED_REPLIES(v),
            "REPEATED_REPLIES" => s_REPEATED_REPLIES(v),
            "TOLD_BEFORE_REPLIES" => s_TOLD_BEFORE_REPLIES(v),
            "WARM_REPLIES" => s_WARM_REPLIES(v),
            "WRONG_REPLIES" => s_WRONG_REPLIES(v),
            "_APPRECIATE" => s__APPRECIATE(v),
            "_BACK_TO_SOMETHING" => s__BACK_TO_SOMETHING(v),
            "_BARE_WRONG" => s__BARE_WRONG(v),
            "_FAVOURITE" => s__FAVOURITE(v),
            "_GOT_IT_WRONG" => s__GOT_IT_WRONG(v),
            "_INVENTED" => s__INVENTED(v),
            "_LIKE" => s__LIKE(v),
            "_LOVE" => s__LOVE(v),
            "_MACHINE" => s__MACHINE(v),
            "_MISS_TAIL" => s__MISS_TAIL(v),
            "_MIXED_UP" => s__MIXED_UP(v),
            "_MUDDLED" => s__MUDDLED(v),
            "_NOT_WHAT_I_MEANT" => s__NOT_WHAT_I_MEANT(v),
            "_NO_ANSWER" => s__NO_ANSWER(v),
            "_NO_HELP" => s__NO_HELP(v),
            "_NO_LEAD" => s__NO_LEAD(v),
            "_NO_NOT_THAT" => s__NO_NOT_THAT(v),
            "_OFF_THE_ASK" => s__OFF_THE_ASK(v),
            "_ON_FORM" => s__ON_FORM(v),
            "_REPEATED" => s__REPEATED(v),
            "_SOFT_LEAD" => s__SOFT_LEAD(v),
            "_SOFT_TAIL" => s__SOFT_TAIL(v),
            "_THE_OTHER_ONE" => s__THE_OTHER_ONE(v),
            "_THE_WRONG_THING" => s__THE_WRONG_THING(v),
            "_TIME_TAIL" => s__TIME_TAIL(v),
            "_TOLD_BEFORE" => s__TOLD_BEFORE(v),
            "_TYPED_SHORT" => s__TYPED_SHORT(v),
            "_WRONG" => s__WRONG(v),
            "_YOUR_NOT" => s__YOUR_NOT(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.warmth", module_obj);
    pyrt::xrefs::reg_fn("memory.warmth", "_nothing_said_before", py_func("_nothing_said_before", |args: &[Py]| _nothing_said_before(args)));
    pyrt::xrefs::reg_fn("memory.warmth", "_a_complaint_reply", py_func("_a_complaint_reply", |args: &[Py]| _a_complaint_reply(args)));
    pyrt::xrefs::reg_fn("memory.warmth", "_nothing_said_yet", py_func("_nothing_said_yet", |args: &[Py]| _nothing_said_yet(args)));
    pyrt::xrefs::reg_fn("memory.warmth", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_access("memory.warmth", "FIRST_LINE_READING", g_FIRST_LINE_READING, s_FIRST_LINE_READING);
    pyrt::xrefs::reg_access("memory.warmth", "FIRST_LINE_REPLIES", g_FIRST_LINE_REPLIES, s_FIRST_LINE_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "MACHINE_REPLIES", g_MACHINE_REPLIES, s_MACHINE_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "MISSED_ASK_REPLIES", g_MISSED_ASK_REPLIES, s_MISSED_ASK_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "MISSED_REPLIES", g_MISSED_REPLIES, s_MISSED_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "MUDDLED_REPLIES", g_MUDDLED_REPLIES, s_MUDDLED_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "PRAISED_REPLIES", g_PRAISED_REPLIES, s_PRAISED_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "REPEATED_REPLIES", g_REPEATED_REPLIES, s_REPEATED_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "TOLD_BEFORE_REPLIES", g_TOLD_BEFORE_REPLIES, s_TOLD_BEFORE_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "WARM_REPLIES", g_WARM_REPLIES, s_WARM_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "WRONG_REPLIES", g_WRONG_REPLIES, s_WRONG_REPLIES);
    pyrt::xrefs::reg_access("memory.warmth", "_APPRECIATE", g__APPRECIATE, s__APPRECIATE);
    pyrt::xrefs::reg_access("memory.warmth", "_BACK_TO_SOMETHING", g__BACK_TO_SOMETHING, s__BACK_TO_SOMETHING);
    pyrt::xrefs::reg_access("memory.warmth", "_BARE_WRONG", g__BARE_WRONG, s__BARE_WRONG);
    pyrt::xrefs::reg_access("memory.warmth", "_FAVOURITE", g__FAVOURITE, s__FAVOURITE);
    pyrt::xrefs::reg_access("memory.warmth", "_GOT_IT_WRONG", g__GOT_IT_WRONG, s__GOT_IT_WRONG);
    pyrt::xrefs::reg_access("memory.warmth", "_INVENTED", g__INVENTED, s__INVENTED);
    pyrt::xrefs::reg_access("memory.warmth", "_LIKE", g__LIKE, s__LIKE);
    pyrt::xrefs::reg_access("memory.warmth", "_LOVE", g__LOVE, s__LOVE);
    pyrt::xrefs::reg_access("memory.warmth", "_MACHINE", g__MACHINE, s__MACHINE);
    pyrt::xrefs::reg_access("memory.warmth", "_MISS_TAIL", g__MISS_TAIL, s__MISS_TAIL);
    pyrt::xrefs::reg_access("memory.warmth", "_MIXED_UP", g__MIXED_UP, s__MIXED_UP);
    pyrt::xrefs::reg_access("memory.warmth", "_MUDDLED", g__MUDDLED, s__MUDDLED);
    pyrt::xrefs::reg_access("memory.warmth", "_NOT_WHAT_I_MEANT", g__NOT_WHAT_I_MEANT, s__NOT_WHAT_I_MEANT);
    pyrt::xrefs::reg_access("memory.warmth", "_NO_ANSWER", g__NO_ANSWER, s__NO_ANSWER);
    pyrt::xrefs::reg_access("memory.warmth", "_NO_HELP", g__NO_HELP, s__NO_HELP);
    pyrt::xrefs::reg_access("memory.warmth", "_NO_LEAD", g__NO_LEAD, s__NO_LEAD);
    pyrt::xrefs::reg_access("memory.warmth", "_NO_NOT_THAT", g__NO_NOT_THAT, s__NO_NOT_THAT);
    pyrt::xrefs::reg_access("memory.warmth", "_OFF_THE_ASK", g__OFF_THE_ASK, s__OFF_THE_ASK);
    pyrt::xrefs::reg_access("memory.warmth", "_ON_FORM", g__ON_FORM, s__ON_FORM);
    pyrt::xrefs::reg_access("memory.warmth", "_REPEATED", g__REPEATED, s__REPEATED);
    pyrt::xrefs::reg_access("memory.warmth", "_SOFT_LEAD", g__SOFT_LEAD, s__SOFT_LEAD);
    pyrt::xrefs::reg_access("memory.warmth", "_SOFT_TAIL", g__SOFT_TAIL, s__SOFT_TAIL);
    pyrt::xrefs::reg_access("memory.warmth", "_THE_OTHER_ONE", g__THE_OTHER_ONE, s__THE_OTHER_ONE);
    pyrt::xrefs::reg_access("memory.warmth", "_THE_WRONG_THING", g__THE_WRONG_THING, s__THE_WRONG_THING);
    pyrt::xrefs::reg_access("memory.warmth", "_TIME_TAIL", g__TIME_TAIL, s__TIME_TAIL);
    pyrt::xrefs::reg_access("memory.warmth", "_TOLD_BEFORE", g__TOLD_BEFORE, s__TOLD_BEFORE);
    pyrt::xrefs::reg_access("memory.warmth", "_TYPED_SHORT", g__TYPED_SHORT, s__TYPED_SHORT);
    pyrt::xrefs::reg_access("memory.warmth", "_WRONG", g__WRONG, s__WRONG);
    pyrt::xrefs::reg_access("memory.warmth", "_YOUR_NOT", g__YOUR_NOT, s__YOUR_NOT);
    pyrt::xrefs::reg_access("memory.warmth", "pick", g_pick, s_pick);
}
