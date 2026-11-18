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


thread_local! { static G_ALL_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAVE_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FENCED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEMORY_STORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_GOES_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOPS_ON_AN_INITIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_MOST_A_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIED_THE_LOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLAUSE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HELD_THE_FENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__I_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OFFER_NOT_A_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OR_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE_HAS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEP_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_LOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TITLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHOLE_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALL_OF_THEM() -> Py { ensure_init(); G_ALL_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALL_OF_THEM(v: Py) { ensure_init(); G_ALL_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER() -> Py { ensure_init(); G_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER(v: Py) { ensure_init(); G_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAVE_ALREADY() -> Py { ensure_init(); G_A_HAVE_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAVE_ALREADY(v: Py) { ensure_init(); G_A_HAVE_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE() -> Py { ensure_init(); G_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE(v: Py) { ensure_init(); G_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_FENCED_OUT() -> Py { ensure_init(); G_FENCED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FENCED_OUT(v: Py) { ensure_init(); G_FENCED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGED() -> Py { ensure_init(); G_JUDGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGED(v: Py) { ensure_init(); G_JUDGED.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOWN() -> Py { ensure_init(); G_KNOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOWN(v: Py) { ensure_init(); G_KNOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_MEMORY_STORE() -> Py { ensure_init(); G_MEMORY_STORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEMORY_STORE(v: Py) { ensure_init(); G_MEMORY_STORE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_GOES_ON() -> Py { ensure_init(); G_NAME_GOES_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_GOES_ON(v: Py) { ensure_init(); G_NAME_GOES_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATION() -> Py { ensure_init(); G_NEGATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATION(v: Py) { ensure_init(); G_NEGATION.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT() -> Py { ensure_init(); G_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT(v: Py) { ensure_init(); G_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_LEFT() -> Py { ensure_init(); G_ONE_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_LEFT(v: Py) { ensure_init(); G_ONE_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_TAKEN() -> Py { ensure_init(); G_ONE_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_TAKEN(v: Py) { ensure_init(); G_ONE_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_AGAIN() -> Py { ensure_init(); G_PICK_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_AGAIN(v: Py) { ensure_init(); G_PICK_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSION() -> Py { ensure_init(); G_POSSESSION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSION(v: Py) { ensure_init(); G_POSSESSION.with(|c| *c.borrow_mut() = v); }
pub fn g_STOPS_ON_AN_INITIAL() -> Py { ensure_init(); G_STOPS_ON_AN_INITIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOPS_ON_AN_INITIAL(v: Py) { ensure_init(); G_STOPS_ON_AN_INITIAL.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_MOST_A_LIST() -> Py { ensure_init(); G_THE_MOST_A_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_MOST_A_LIST(v: Py) { ensure_init(); G_THE_MOST_A_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIED_THE_LOT() -> Py { ensure_init(); G_TRIED_THE_LOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIED_THE_LOT(v: Py) { ensure_init(); G_TRIED_THE_LOT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_THE_BOT() -> Py { ensure_init(); G__ASKS_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_THE_BOT(v: Py) { ensure_init(); G__ASKS_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_REASON() -> Py { ensure_init(); G__A_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_REASON(v: Py) { ensure_init(); G__A_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g__CLAUSE_VERB() -> Py { ensure_init(); G__CLAUSE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLAUSE_VERB(v: Py) { ensure_init(); G__CLAUSE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__FENCE() -> Py { ensure_init(); G__FENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FENCE(v: Py) { ensure_init(); G__FENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__HELD_THE_FENCE() -> Py { ensure_init(); G__HELD_THE_FENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HELD_THE_FENCE(v: Py) { ensure_init(); G__HELD_THE_FENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__I_HAVE() -> Py { ensure_init(); G__I_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__I_HAVE(v: Py) { ensure_init(); G__I_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_NAME() -> Py { ensure_init(); G__NOT_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_NAME(v: Py) { ensure_init(); G__NOT_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING_START() -> Py { ensure_init(); G__NOT_A_THING_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING_START(v: Py) { ensure_init(); G__NOT_A_THING_START.with(|c| *c.borrow_mut() = v); }
pub fn g__OFFER_NOT_A_LIST() -> Py { ensure_init(); G__OFFER_NOT_A_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OFFER_NOT_A_LIST(v: Py) { ensure_init(); G__OFFER_NOT_A_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g__OR_CHOICE() -> Py { ensure_init(); G__OR_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OR_CHOICE(v: Py) { ensure_init(); G__OR_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE_HAS_IT() -> Py { ensure_init(); G__SOMEONE_ELSE_HAS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE_HAS_IT(v: Py) { ensure_init(); G__SOMEONE_ELSE_HAS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__STEP_START() -> Py { ensure_init(); G__STEP_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEP_START(v: Py) { ensure_init(); G__STEP_START.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_LOT() -> Py { ensure_init(); G__THE_LOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_LOT(v: Py) { ensure_init(); G__THE_LOT.with(|c| *c.borrow_mut() = v); }
pub fn g__TITLE() -> Py { ensure_init(); G__TITLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TITLE(v: Py) { ensure_init(); G__TITLE.with(|c| *c.borrow_mut() = v); }
pub fn g__WHOLE_LIST() -> Py { ensure_init(); G__WHOLE_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHOLE_LIST(v: Py) { ensure_init(); G__WHOLE_LIST.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("They turned down the answer the bot just gave, and it owes them another one.\n\n\"whats a good film tonight\" / \"nah not that one\" is a conversation still going somewhere. Main\nanswers it by saying the line back - \"Got it — nah not that one.\" - which is the coldest reply\nthere is: the three films the bot named a turn ago are still on the screen, one of them has been\nruled out, and the person is asking, in four words, for the next one. The same shape arrives as\n\"dont fancy it\", \"ive seen it\", \"already read that one\", \"not for me\" and \"nah not those\".\n\nSo this reads the refusal and the answer it refuses. Two rules keep it to the lines it is good\nat:\n\n* the whole line has to be the refusal. Every word, once the chat-speak is taken off, has to be\n  a word for turning something down or one of the pronouns that point at it (\"that\", \"it\",\n  \"those\"). \"nah just eggs and some rice\" is a person saying what they have, not a refusal, and\n  one word this module does not know sends the turn back to the rest of the bot.\n* the answer has to have offered things to choose between, read out of the bot's own last reply\n  the way \"you pick\" is (bot/memory/choice.py). A refusal of advice with no choice in it (\"do i\n  need a coat\" / \"not today\") is not a choice to make again, and the ordinary pools answer it.\n\nWhat comes back is one of the things the bot itself offered, never a new fact: a title, a dish\nor a price it can read off its own last reply. When the line rules the whole list out (\"ive seen\nall those\") there is nothing left to hand over from it, so the reply asks the one thing that\nlets the bot pick again - what they did like - rather than offering the same list a second time.\n");
    s_MEMORY_STORE(py_str("memory_store"));
    s_FILLER(call_builtin("frozenset", &[mcall(&py_str("\n    ok okay oh ah well right so then but and just really sorry ta thanks thank you thx cheers\n    hmm erm uh um like actually honestly tbh though tho anyway ha haha lol x xx please pls\n    i im ive id ill we youre you u us for of about at in too either also still yet neither\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATION(call_builtin("frozenset", &[mcall(&py_str("\n    nah no nope none not dont do not doesnt didnt didnt never aint already meh rather\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OBJECT(call_builtin("frozenset", &[mcall(&py_str("\n    that this it those these them one ones both either neither any all every everything\n    thing things one either list two three\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGED(call_builtin("frozenset", &[mcall(&py_str("\n    seen saw watch watched read tried had have ate eaten got get fancy fancied like liked keen\n    bother bothered feel feeling want wanted sound sounds sounded appeal appeals my me kind\n    cup tea sort thing things type sort my mine thanks ta taste tastes\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE(call_builtin("frozenset", &[mcall(&py_str("seen saw watched read tried had ate eaten got heard done"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POSSESSION(call_builtin("frozenset", &[py_tuple(vec![py_str("got"), py_str("get")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAVE_ALREADY(stdlib_call("re", "compile", &[py_str("\\b(?:already|before|once|one|ones|last|year|years|since|when|ever|yet|ages?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__I_HAVE(stdlib_call("re", "compile", &[py_str("^\\s*(?:i\\s+have|ive|i\\s+got|we\\s+have|weve|we\\s+got)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KNOWN(op_bitor(&op_bitor(&op_bitor(&op_bitor(&g_FILLER(), &g_NEGATION()), &g_OBJECT()), &g_JUDGED()), &g_DONE()));
    s__ASKS_THE_BOT(call_builtin("frozenset", &[mcall(&py_str("\n    have has do does did can could would will shall is are was were am any anything\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__WHOLE_LIST(stdlib_call("re", "compile", &[py_str("\\b(?:all|those|them|these|both|every|everything|any|list|three|two)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OFFER_NOT_A_LIST(stdlib_call("re", "compile", &[py_str("\\?\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER(py_tuple(vec![py_str("Fair enough — then how about {option}?"), py_str("Then {option} instead."), py_str("Right, {option} is the other one worth a go."), py_str("Not that one, then — {option}?")]));
    s_PICK_AGAIN(py_tuple(vec![py_str("Fair enough, none of those then. What did you enjoy last? I'll pick something in that vein."), py_str("Right, that list is out. Tell me one you did like and I'll go from there."), py_str("Fair enough. What have you enjoyed lately? I'll keep clear of these three.")]));
    s_ALL_OF_THEM(py_tuple(vec![py_str("{which} then: {names}."), py_str("All of them: {names}."), py_str("Then take {which}: {names}."), py_str("Good — {which}: {names}.")]));
    s_ONE_LEFT(py_tuple(vec![py_str("That leaves one: {option}."), py_str("Two of those are out, so it comes down to {option}."), py_str("Then there is just {option} left.")]));
    s_ONE_TAKEN(py_tuple(vec![py_str("Then take {option}."), py_str("Then it is {option}."), py_str("Go with {option}, then.")]));
    s__THE_LOT(stdlib_call("re", "compile", &[py_str("\\b(?:all|every|everything|both)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_MOST_A_LIST(Py::Int(5));
    s__SOMEONE_ELSE_HAS_IT(stdlib_call("re", "compile", &[py_str("(?<!\\bi )(?<!\\bwe )(?<!\\byou )(?<!\\bive )(?<!\\bweve )(?<!\\byouve )(?<!\\balready )\\b(?:has|have|got)\\s+(?:everything|it all|the lot)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STOPS_ON_AN_INITIAL(stdlib_call("re", "compile", &[py_str("(?:^|\\s)[A-Z]\\.?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_GOES_ON(stdlib_call("re", "compile", &[py_str("\\.?\\s+((?:[A-Z]\\.\\s*)*(?:[A-Z][a-z'\\u2019-]+)(?:\\s+[A-Z][a-z'\\u2019-]+)?)")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_NAME(stdlib_call("re", "compile", &[py_str("[()\\[\\];:!?]")]).unwrap_or_else(|e| exc::raise(e)));
    s__STEP_START(call_builtin("frozenset", &[mcall(&py_str("\n    take put use add cook roast boil fry bake grill heat mix stir cut chop peel wash rinse spray\n    wipe clean soak leave cover simmer drain season salt rest cool freeze store keep serve pour\n    toss brush line preheat check test break slice grate whisk knead roll spread fill top drizzle\n    sprinkle reduce blend mash layer wrap seal tie trim dry hang iron vacuum dust scrub hoover\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__CLAUSE_VERB(call_builtin("frozenset", &[mcall(&py_str("\n    is are was were has have had does do did will would can could make makes keep keeps help\n    helps work works come comes go goes run runs give gives take takes last lasts win wins beat\n    beats fit fits hold holds protect protects save saves cost costs sound sounds look looks\n    taste tastes feel feels crisp crisps promise promises mean means sit sits stand stands need\n    needs seem seems turn turns stay stays\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__TITLE(stdlib_call("re", "compile", &[py_str("\\*([^*\\n]{2,40})\\*")]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIED_THE_LOT(stdlib_call("re", "compile", &[py_str("\\btried\\b[^.!?]{0,24}?\\b(?:everything|all of it|all of them|them all|it all|every one|the lot)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_THING_START(call_builtin("frozenset", &[mcall(&py_str("\n    most more less least too very so quite much many few little enough only even also still yet\n    just then now and or but if when while as because than that this these those it they we you\n    he she i is are was were be been being do does did done have has had will would can could\n    should may might must not no never always usually often sometimes there here\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REASON(stdlib_call("re", "compile", &[py_str("(?:\\bbecause\\b|\\bthe reason\\b|\\bwhich is why\\b|\\bthat'?s why\\b|\\bwhy\\b|\\bsomewhere\\b|\\bcauses?\\b|\\bcaused by\\b|\\bis down to\\b|\\bcomes down to\\b)[^.;!?]{0,70}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OR_CHOICE(stdlib_call("re", "compile", &[py_str(",\\s*or\\s+(?!(?:it|they|i|you|we|he|she|that|this|there|those)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__FENCE(stdlib_call("re", "compile", &[py_str("\\b(?:not|no|never|without|except|anything but)\\s+(?:a|an|the|any|too much|more)?\\s*([a-z][a-z'\\-]{3,})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HELD_THE_FENCE(stdlib_call("re", "compile", &[py_str("\\b(?:without|no|not|nothing|non)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FENCED_OUT(py_tuple(vec![py_str("Fair enough, nothing {kind} then. What do you fancy instead?"), py_str("Right, no {kind} ones. Tell me what you do want and I'll aim at that."), py_str("Fair enough - {kind} is out. What sort of thing would you go for?")]));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut kept: Py = Py::None;
    let mut low: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line as words, chat-speak and courtesies stripped, or None when it is not a refusal.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    low = mcall(&low.clone(), "strip", &[py_str(".,;:!-… ")]);
    low = stdlib_call("re", "sub", &[py_str("\\b(?:do not|did not|does not)\\b"), py_str("dont"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    low = stdlib_call("re", "sub", &[py_str("n['\\u2019]t\\b"), py_str(" not"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&low.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,;:!-")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("all"))); if !py_truthy(&__b) { __b } else { { let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&kept.clone(), &op_neg(&Py::Int(1))), &py_str("at"))) } } } }) {
    let _ = mcall(&kept.clone(), "pop", &[]);
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLER(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn refusal(args: &[Py]) -> Py {
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
    let mut offer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "offer") { v.clone() }
    else { py_str("") };
    let mut judged: Py = Py::None;
    let mut named: Py = Py::None;
    let mut points_at_the_whole: Py = Py::None;
    let mut unknown: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line turns something down and says nothing else.\n\n    `offer` is the bot's own last reply. It is the only place a word the module does not know can\n    come from, because the thing turned down is often named rather than pointed at: \"already had\n    the frittata\" and \"seen the others\" after the bot put a frittata and a ghost story on the\n    screen. A name the offer does not contain is a line about something else - \"nah just eggs and\n    some rice\" is a person saying what they have - and it goes back to the rest of the bot.\n    ");
    words = _words(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__SOMEONE_ELSE_HAS_IT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    unknown = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_KNOWN(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&unknown.clone()) {
    named = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = offer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[unknown.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&unknown.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&named.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g__ASKS_THE_BOT(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::Bool(false);
    }
    judged = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_DONE(), &w.clone()))) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&{ let __b = judged.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_le(&judged.clone(), &g_POSSESSION()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NEGATION(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_HAVE_ALREADY(), "search", &[mcall(&py_str(" "), "join", &[words.clone()])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__I_HAVE(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } } } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_NEGATION(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DONE(), &w.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    points_at_the_whole = { let __b = mcall(&g__WHOLE_LIST(), "search", &[mcall(&py_str(" "), "join", &[words.clone()])]); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OBJECT(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&points_at_the_whole.clone()))) {
    return Py::Bool(false);
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_OBJECT(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_JUDGED(), &w.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_whole_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut option: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "option") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: option"))) };
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut core: Py = Py::None;
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The option, finished off when the reader cut it short on the full stop of an initial.");
    option = mcall(&call_builtin("str", &[{ let __b = option.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_STOPS_ON_AN_INITIAL(), "search", &[option.clone()])))) {
    return option.clone();
    }
    core = mcall(&geti(&mcall(&option.clone(), "split", &[]), &op_neg(&Py::Int(1))), "rstrip", &[py_str(".")]);
    found = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[core.clone()]).unwrap_or_else(|e| exc::raise(e))), &gattr(&g_NAME_GOES_ON(), "pattern")), reply.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return option.clone();
    }
    rest = mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]), "rstrip", &[py_str(".")]);
    return op_add(&op_add(&if py_truthy(&mcall(&option.clone(), "endswith", &[py_str(".")])) { option.clone() } else { op_add(&option.clone(), &py_str(".")) }, &py_str(" ")), &rest.clone());
    Py::None
}
pub fn _tidy(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut option: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "option") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: option"))) };
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("One option as it can be said back: the name, without a note the reader cut in half.\n\n    The italics reader takes a short tail after a title as the bot's note on it (\"*Knives Out*\n    for a clever whodunnit\") and keeps it, which reads well. A bracketed note arrives cut off at\n    the comma inside it (\"The Thursday Murder Club (funny\"), and half a bracket is worse than no\n    note at all.\n\n    A figure is not a thing to choose: \"6 minutes for a runny yolk, 7 for jammy, 9 to 10 for\n    hard boiled\" is one answer with times in it, and the answer to \"not that\" after a boiled\n    egg was never \"7 for jammy\".\n    ");
    option = mcall(&stdlib_call("re", "sub", &[py_str("\\s*\\([^)]*$"), py_str(""), call_builtin("str", &[{ let __b = option.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&stdlib_call("re", "match", &[py_str("^(?:\\d|[£$€]\\s*\\d)"), option.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    option = stdlib_call("re", "sub", &[py_str("^(A|An|The) (?=[a-z])"), { let option = option.clone(); let w = w.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let option = option.clone(); let w = w.clone(); let words = words.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); mcall(&mcall(&m.clone(), "group", &[Py::Int(1)]), "lower", &[]) }) }, option.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_lt(&Py::Int(2), &call_builtin("len", &[option.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&option.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }))) {
    return py_str("");
    }
    words = mcall(&mcall(&option.clone(), "lower", &[]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__STEP_START(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::Int(3), &Py::None)) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__CLAUSE_VERB(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    return option.clone();
    Py::None
}
pub fn _by_title(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut titles: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "titles") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: titles"))) };
    let mut options: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut low: Py = Py::None;
    let mut o: Py = Py::None;
    let mut ordered: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut title: Py = Py::None;
    let _ = py_str("The reply's titles, in the order it gave them, each with its note when that survived.\n\n    bot/memory/choice.py drops an option whose note is a clause of its own: \"*Arrival* if you'd\n    rather think\" is not a way of saying \"Arrival\", so the third film of three never reaches the\n    list, and two refusals then run the offer out instead of naming it. The title is the thing to\n    choose whatever the note after it says, so a title the reader dropped comes back as itself -\n    a bare \"Arrival\" is what a friend would say.\n    ");
    if py_truthy(&op_lt(&call_builtin("len", &[titles.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return options.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    ordered = __unpk1[0].clone();
    seen = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&titles.clone());
    for __item in __iter2 {
    title = __item.clone();
    low = mcall(&title.clone(), "lower", &[]);
    said = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&mcall(&mcall(&o.clone(), "lower", &[]), "startswith", &[low.clone()])) {  __out.push(o.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&said.clone(), "lower", &[]), &low.clone())) } }) {
    rest = mcall(&slice_or_raise(&said.clone(), &call_builtin("len", &[title.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&stdlib_call("re", "match", &[py_str("^(?:,|or\\b|and\\b|\\*)"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    said = mcall(&slice_or_raise(&said.clone(), &Py::None, &call_builtin("len", &[title.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), "strip", &[]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&mcall(&o.clone(), "lower", &[]), &low.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    }
    said = { let __b = said.clone(); if py_truthy(&__b) { __b } else { title.clone() } };
    if py_truthy(&Py::Bool(py_contains(&seen.clone(), &mcall(&said.clone(), "lower", &[])))) {
    continue;
    }
    let _ = mcall(&seen.clone(), "add", &[mcall(&said.clone(), "lower", &[])]);
    let _ = mcall(&ordered.clone(), "append", &[said.clone()]);
    }
    return if py_truthy(&op_ge(&call_builtin("len", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { ordered.clone() } else { options.clone() };
    Py::None
}
pub fn _a_choice_of_things(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut title: Py = Py::None;
    let _ = py_str("True when the reply put things up to choose between, not advice with a list in it.\n\n    A reply can read as a list to `choice.options_in` without offering a choice: the nail-biting\n    answer ends \"give that hand something else to do: a ring to twist, a pen, a stress ball\",\n    and a person who says \"ive tried everything\" after it was told \"Fair enough, none of those\n    then. What did you enjoy last? I'll pick something in that vein.\" - the wording for a\n    shortlist of films. Those three are steps of the advice, not things to pick one of. A real\n    shortlist says \"or\" (\"... a frittata in 20, or a salmon traybake\") or stars its titles.\n    ");
    reply = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return { let __b = op_ge(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__TITLE(), "findall", &[reply.clone()])) { let mut title: Py = __item.clone(); __out.push(mcall(&title.clone(), "lower", &[])); } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__OR_CHOICE(), "search", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_offer_reply(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot's own reply that put things up to choose between, or \"\".\n\n    The refusal can be two turns after the offer (\"nah not that one\" / \"nah not that either\"),\n    so the reply that counts as the offer is found the same way `_offered` finds it, not taken\n    from the last turn alone.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[crate::memory::recent(&[memory.clone(), Py::Int(4)])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    reply = call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[_options(&[reply.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return reply.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _options(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut known: Py = Py::None;
    let mut o: Py = Py::None;
    let mut options: Py = Py::None;
    let mut short: Py = Py::None;
    let mut t: Py = Py::None;
    let mut titles: Py = Py::None;
    let _ = py_str("The things one reply offered to choose between, or [] when it offered no choice.");
    reply = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { mcall(&g__OFFER_NOT_A_LIST(), "search", &[reply.clone()]) } }) {
    return py_list(vec![]);
    }
    titles = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__TITLE(), "findall", &[reply.clone()])) { let mut t: Py = __item.clone(); __out.push(_tidy(&[t.clone()])); } py_list(__out) })() };
    titles = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&titles.clone()) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = t.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NOT_A_NAME(), "search", &[t.clone()]))) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::choice::options_in(&[reply.clone()])) { let mut o: Py = __item.clone(); __out.push(_tidy(&[_the_whole_name(&[o.clone(), reply.clone()])])); } py_list(__out) })() };
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&{ let __b = o.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NOT_A_NAME(), "search", &[o.clone()]))) } }) {  __out.push(o.clone()); } } py_list(__out) })() };
    options = _by_title(&[titles.clone(), options.clone()]);
    known = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&titles.clone()) { let mut t: Py = __item.clone(); __out.push(mcall(&t.clone(), "lower", &[])); } py_set(__out) })() };
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[mcall(&o.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &mcall(&o.clone(), "lower", &[]))) } }) {  __out.push(o.clone()); } } py_list(__out) })() };
    short = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&op_le(&call_builtin("len", &[mcall(&o.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {  __out.push(o.clone()); } } py_list(__out) })() };
    options = { let __b = short.clone(); if py_truthy(&__b) { __b } else { options.clone() } };
    if py_truthy(&op_lt(&call_builtin("len", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_list(vec![]);
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[titles.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__OR_CHOICE(), "search", &[reply.clone()]))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&reply.clone(), &py_str(":"))); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); __out.push(_a_noun_phrase(&[o.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }))) {
    return py_list(vec![]);
    }
    }
    if py_truthy(&_a_list_of_causes(&[reply.clone(), options.clone()])) {
    return py_list(vec![]);
    }
    return options.clone();
    Py::None
}
pub fn _a_list_of_causes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut options: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut option: Py = Py::None;
    let mut stop: Py = Py::None;
    let _ = py_str("True when the \"or\" list the options came from is a reason rather than an offer.\n\n    Only the sentence the option sits in is read, and only the words in front of it: a genuine\n    offer explains itself afterwards (\"Saturday market, or the library if it rains\"), which is the\n    opposite order and stays an offer.\n    ");
    flat = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    let __iter1 = py_iter_or_raise(&options.clone());
    for __item in __iter1 {
    option = __item.clone();
    at = mcall(&mcall(&flat.clone(), "lower", &[]), "find", &[mcall(&call_builtin("str", &[option.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    continue;
    }
    before = slice_or_raise(&flat.clone(), &Py::None, &at.clone(), &Py::None);
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str(". "), py_str("! "), py_str("? "), py_str("; ")]));
    for __item in __iter2 {
    stop = __item.clone();
    cut = mcall(&before.clone(), "rfind", &[stop.clone()]);
    if py_truthy(&op_gt(&cut.clone(), &Py::Int(0))) {
    before = slice_or_raise(&before.clone(), &op_add(&cut.clone(), &call_builtin("len", &[stop.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    }
    }
    if py_truthy(&mcall(&g__A_REASON(), "search", &[before.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_noun_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut option: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "option") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: option"))) };
    let mut words: Py = Py::None;
    let _ = py_str("True when an option reads as the name of a thing rather than a piece of a sentence.");
    words = mcall(&mcall(&call_builtin("str", &[{ let __b = option.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_THING_START(), &geti(&words.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _said_by_them(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut option: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "option") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: option"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut every_word: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the person's own line says every word of `option`.\n\n    A time after a dish is how long it takes, not part of its name: the offer says \"Quick: tuna\n    pasta in 15, a frittata in 20\", and a person who says \"already had the frittata\" has named\n    the frittata. The time stays in the line the module writes, because \"how about a frittata in\n    20?\" is the useful form and \"how about a frittata?\" loses how long it takes.\n    ");
    option = stdlib_call("re", "sub", &[py_str("\\s+in\\s+\\d+\\s*(?:minutes?|mins?|min)?\\s*$"), py_str(""), call_builtin("str", &[{ let __b = option.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    said = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
pub fn __nested_every_word_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut every_word: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut option: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "name").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&name.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
    every_word = {{ let every_word = every_word.clone(); let name = name.clone(); let option = option.clone(); let said = said.clone(); let text = text.clone(); py_func("every_word", move |args: &[Py]| { let mut full = vec![every_word.clone(), name.clone(), option.clone(), said.clone(), text.clone()]; full.extend_from_slice(args); __nested_every_word_0(&full) }) }};
    if py_truthy(&py_call(&every_word, &[option.clone()])) {
    return Py::Bool(true);
    }
    name = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:for|if|which|when|that|to|in|about)\\s+")); __args.push(option.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    return { let __b = op_gt(&call_builtin("len", &[mcall(&name.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { py_call(&every_word, &[name.clone()]) } };
    Py::None
}
pub fn _offered(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut left: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut option: Py = Py::None;
    let mut start: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The offer the refusal is about, as the options of it the bot has not named again.\n\n    A second refusal (\"then *Knives Out*\" / \"nah not that one either\") is about the same offer\n    as the first, so the search walks back over the replies this module itself wrote - the ones\n    that hand over another option and name no list - and stops at any other reply. Options the\n    bot has already handed over are taken out, so the third refusal gets the third film and a\n    fourth gets the question that lets the bot pick again. None when no reply in reach offered\n    a choice, which is the whole of the difference between \"not my answer\" and \"not my line\".\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::recent(&[memory.clone(), Py::Int(4)])) { let mut turn: Py = __item.clone(); __out.push(if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { turn.clone() } else { py_dict(vec![]) }); } py_list(__out) })() };
    start = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&op_ge(&call_builtin("len", &[_options(&[{ let __b = mcall(&geti(&turns.clone(), &index.clone()), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    start = index.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&start.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_add(&start.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &g_MEMORY_STORE()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    left = call_builtin("list", &[_options(&[{ let __b = mcall(&geti(&turns.clone(), &start.clone()), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_add(&start.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter2 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut option: Py = __item.clone(); if py_truthy(&_said_by_them(&[option.clone(), line.clone()])) {  __out.push(option.clone()); } } py_list(__out) })() };
    left = if py_truthy(&named.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut option: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&named.clone(), &option.clone()))) {  __out.push(option.clone()); } } py_list(__out) })() } } else { slice_or_raise(&left.clone(), &Py::Int(1), &Py::None, &Py::None) };
    }
    return left.clone();
    Py::None
}
pub fn _the_fence(args: &[Py]) -> Py {
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
    let mut fence: Py = Py::None;
    let mut index: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut offer: Py = Py::None;
    let mut option: Py = Py::None;
    let mut options: Py = Py::None;
    let mut request: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut started: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the person ruled out in the request, when the offer that came back ignored it.\n\n    `i fancy a film but not a scary one` is a request with a fence in it, and the film entry\n    answered with the list of scares (\"For a proper scare: Get Out, A Quiet Place, Hereditary\").\n    Every later turn-down then walked that same list - \"then Get Out?\", \"how about Hereditary?\" -\n    so the fence cannot be read one title at a time: the offer's own lead-in is matched against\n    the word the person ruled out, and the whole list stands down.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::recent(&[memory.clone(), Py::Int(3)])) { let mut turn: Py = __item.clone(); __out.push(if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { turn.clone() } else { py_dict(vec![]) }); } py_list(__out) })() };
    let __unpk1 = py_unpack(&py_tuple(vec![py_str(""), py_list(vec![]), py_str("")]), 3, None);
    offer = __unpk1[0].clone();
    options = __unpk1[1].clone();
    request = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    let mut __else3 = false;
    for __item in __iter2 {
    index = __item.clone();
    offer = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &index.clone()), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    options = _options(&[offer.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    request = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &index.clone()), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    return Py::None;
    ruled_out = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__FENCE(), "findall", &[request.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_KNOWN(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&ruled_out.clone()))) {
    return Py::None;
    }
    started = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut option: Py = __item.clone(); if py_truthy(&op_gt(&mcall(&offer.clone(), "find", &[option.clone()]), &Py::Int(0))) {  __out.push(mcall(&offer.clone(), "find", &[option.clone()])); } } py_list(__out) })() };
    lead = if py_truthy(&started.clone()) { slice_or_raise(&offer.clone(), &Py::None, &call_builtin("min", &[started.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None) } else { slice_or_raise(&offer.clone(), &Py::None, &Py::Int(80), &Py::None) };
    if py_truthy(&mcall(&g__HELD_THE_FENCE(), "search", &[lead.clone()])) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&lead.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter4 = py_iter_or_raise(&ruled_out.clone());
    for __item in __iter4 {
    fence = __item.clone();
    let __iter5 = py_iter_or_raise(&words.clone());
    for __item in __iter5 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(4), &Py::None), &slice_or_raise(&fence.clone(), &Py::None, &Py::Int(4), &Py::None)))) {
    return fence.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_whole_lot(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut count: Py = Py::None;
    let _ = py_str("\"both\" or \"all three\", for however many of them are still on the table.");
    count = call_builtin("len", &[{ let __b = options.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(2)))) {
    return py_str("both");
    }
    return op_add(&py_str("all "), &mcall(&py_dict(vec![(Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five"))]), "get", &[count.clone(), py_str("of them")]));
    Py::None
}
pub fn offers_a_choice(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let _ = py_str("True when the bot's last reply put two or more things up to choose between.\n\n    A line that says the whole lot means something different either side of this: after three\n    films \"ive tried them all\" says the list is out, and after a paragraph of advice it is a\n    report that they have done what the advice said. The reader of the report asks this before\n    it reads the line. A reply whose list is not a choice at all (`_a_choice_of_things`) is a\n    paragraph of advice: the nail-biting answer's \"give that hand something else to do: a ring\n    to twist, a pen, a stress ball\" was read as a shortlist, so \"ive tried everything\" after it\n    was answered as a list refused rather than as a report on the advice.\n    ");
    if py_truthy(&op_lt(&call_builtin("len", &[{ let __b = _offered(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    reply = _the_offer_reply(&[memory.clone()]);
    return if py_truthy(&reply.clone()) { _a_choice_of_things(&[reply.clone()]) } else { Py::Bool(false) };
    Py::None
}
pub fn _the_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut o: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The options as one list read aloud: \"a, b and c\".");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&o.clone()) {  __out.push(call_builtin("str", &[o.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return if py_truthy(&said.clone()) { geti(&said.clone(), &Py::Int(0)) } else { py_str("") };
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _the_lot_and_the_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut the_lot: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("(had the lot, wants the lot) for one line, read off the word that means all of it.\n\n    \"them all please\" and \"all of them\" carry no word for having had them, so they are a person\n    taking the lot. \"ive read them all\" and \"seen them all\" carry the had-it verb and are a\n    refusal of the whole list. A line with neither (\"none of those\", \"not that one\") points at\n    the list rather than covering it, and is the turn-down the pool below is for.\n    ");
    the_lot = call_builtin("bool", &[mcall(&g__THE_LOT(), "search", &[mcall(&py_str(" "), "join", &[lines.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![{ let __b = the_lot.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DONE(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }, the_lot.clone()]);
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut fence: Py = Py::None;
    let mut had_them_all: Py = Py::None;
    let mut left: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut named_by_them: Py = Py::None;
    let mut names: Py = Py::None;
    let mut o: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut option: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut wants_them_all: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("Another of the options the bot just offered, or a question that lets it pick again.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    answer = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[crate::memory::recent(&[memory.clone(), Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    answer = call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&answer.clone()) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&refusal(&[text.clone(), answer.clone()])))) {
    return Py::None;
    }
    offered = _offered(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&offered.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_TRIED_THE_LOT(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_choice_of_things(&[_the_offer_reply(&[memory.clone()])]))) } }) {
    return Py::None;
    }
    fence = _the_fence(&[memory.clone()]);
    if py_truthy(&fence.clone()) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("kind".to_string(), fence.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_FENCED_OUT(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    left = call_builtin("list", &[offered.clone()]).unwrap_or_else(|e| exc::raise(e));
    named_by_them = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut o: Py = __item.clone(); if py_truthy(&_said_by_them(&[o.clone(), text.clone()])) {  __out.push(o.clone()); } } py_list(__out) })() };
    left = if py_truthy(&named_by_them.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&named_by_them.clone(), &o.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() } } else { slice_or_raise(&left.clone(), &Py::Int(1), &Py::None, &Py::None) };
    lines = _words(&[text.clone()]);
    let __unpk2 = py_unpack(&_the_lot_and_the_verb(&[{ let __b = lines.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]), 2, None);
    had_them_all = __unpk2[0].clone();
    wants_them_all = __unpk2[1].clone();
    if py_truthy(&op_gt(&call_builtin("len", &[offered.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_THE_MOST_A_LIST())) {
    had_them_all = Py::Bool(false);
    wants_them_all = Py::Bool(false);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[offered.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    if py_truthy(&{ let __b = had_them_all.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = left.clone(); if py_truthy(&__b) { __b } else { wants_them_all.clone() } })) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_PICK_AGAIN(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    pool = if py_truthy(&wants_them_all.clone()) { g_ONE_TAKEN() } else { g_ONE_LEFT() };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("option".to_string(), geti(&{ let __b = left.clone(); if py_truthy(&__b) { __b } else { offered.clone() } }, &Py::Int(0)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let __unpk3 = py_unpack(&py_tuple(vec![_the_whole_lot(&[offered.clone()]), _the_names(&[offered.clone()])]), 2, None);
    which = __unpk3[0].clone();
    names = __unpk3[1].clone();
    if py_truthy(&had_them_all.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_PICK_AGAIN(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&wants_them_all.clone()) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("which".to_string(), which.clone()), ("names".to_string(), names.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_ALL_OF_THEM(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&left.clone())); if py_truthy(&__b) { __b } else { { let __b = lines.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__WHOLE_LIST(), "search", &[mcall(&py_str(" "), "join", &[lines.clone()])]) } } } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_PICK_AGAIN(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    option = geti(&left.clone(), &Py::Int(0));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("option".to_string(), option.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_ANOTHER(), memory.clone(), call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.reject".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALL_OF_THEM" => Some(g_ALL_OF_THEM()),
            "ANOTHER" => Some(g_ANOTHER()),
            "A_HAVE_ALREADY" => Some(g_A_HAVE_ALREADY()),
            "DONE" => Some(g_DONE()),
            "FENCED_OUT" => Some(g_FENCED_OUT()),
            "FILLER" => Some(g_FILLER()),
            "JUDGED" => Some(g_JUDGED()),
            "KNOWN" => Some(g_KNOWN()),
            "MEMORY_STORE" => Some(g_MEMORY_STORE()),
            "NAME_GOES_ON" => Some(g_NAME_GOES_ON()),
            "NEGATION" => Some(g_NEGATION()),
            "OBJECT" => Some(g_OBJECT()),
            "ONE_LEFT" => Some(g_ONE_LEFT()),
            "ONE_TAKEN" => Some(g_ONE_TAKEN()),
            "PICK_AGAIN" => Some(g_PICK_AGAIN()),
            "POSSESSION" => Some(g_POSSESSION()),
            "STOPS_ON_AN_INITIAL" => Some(g_STOPS_ON_AN_INITIAL()),
            "THE_MOST_A_LIST" => Some(g_THE_MOST_A_LIST()),
            "TRIED_THE_LOT" => Some(g_TRIED_THE_LOT()),
            "_ASKS_THE_BOT" => Some(g__ASKS_THE_BOT()),
            "_A_REASON" => Some(g__A_REASON()),
            "_CLAUSE_VERB" => Some(g__CLAUSE_VERB()),
            "_FENCE" => Some(g__FENCE()),
            "_HELD_THE_FENCE" => Some(g__HELD_THE_FENCE()),
            "_I_HAVE" => Some(g__I_HAVE()),
            "_NOT_A_NAME" => Some(g__NOT_A_NAME()),
            "_NOT_A_THING_START" => Some(g__NOT_A_THING_START()),
            "_OFFER_NOT_A_LIST" => Some(g__OFFER_NOT_A_LIST()),
            "_OR_CHOICE" => Some(g__OR_CHOICE()),
            "_SOMEONE_ELSE_HAS_IT" => Some(g__SOMEONE_ELSE_HAS_IT()),
            "_STEP_START" => Some(g__STEP_START()),
            "_THE_LOT" => Some(g__THE_LOT()),
            "_TITLE" => Some(g__TITLE()),
            "_WHOLE_LIST" => Some(g__WHOLE_LIST()),
            "_a_choice_of_things" => Some(py_func("_a_choice_of_things", |args: &[Py]| _a_choice_of_things(args))),
            "_a_list_of_causes" => Some(py_func("_a_list_of_causes", |args: &[Py]| _a_list_of_causes(args))),
            "_a_noun_phrase" => Some(py_func("_a_noun_phrase", |args: &[Py]| _a_noun_phrase(args))),
            "_by_title" => Some(py_func("_by_title", |args: &[Py]| _by_title(args))),
            "_offered" => Some(py_func("_offered", |args: &[Py]| _offered(args))),
            "_options" => Some(py_func("_options", |args: &[Py]| _options(args))),
            "_said_by_them" => Some(py_func("_said_by_them", |args: &[Py]| _said_by_them(args))),
            "_the_fence" => Some(py_func("_the_fence", |args: &[Py]| _the_fence(args))),
            "_the_lot_and_the_verb" => Some(py_func("_the_lot_and_the_verb", |args: &[Py]| _the_lot_and_the_verb(args))),
            "_the_names" => Some(py_func("_the_names", |args: &[Py]| _the_names(args))),
            "_the_offer_reply" => Some(py_func("_the_offer_reply", |args: &[Py]| _the_offer_reply(args))),
            "_the_whole_lot" => Some(py_func("_the_whole_lot", |args: &[Py]| _the_whole_lot(args))),
            "_the_whole_name" => Some(py_func("_the_whole_name", |args: &[Py]| _the_whole_name(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "offers_a_choice" => Some(py_func("offers_a_choice", |args: &[Py]| offers_a_choice(args))),
            "refusal" => Some(py_func("refusal", |args: &[Py]| refusal(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALL_OF_THEM" => s_ALL_OF_THEM(v),
            "ANOTHER" => s_ANOTHER(v),
            "A_HAVE_ALREADY" => s_A_HAVE_ALREADY(v),
            "DONE" => s_DONE(v),
            "FENCED_OUT" => s_FENCED_OUT(v),
            "FILLER" => s_FILLER(v),
            "JUDGED" => s_JUDGED(v),
            "KNOWN" => s_KNOWN(v),
            "MEMORY_STORE" => s_MEMORY_STORE(v),
            "NAME_GOES_ON" => s_NAME_GOES_ON(v),
            "NEGATION" => s_NEGATION(v),
            "OBJECT" => s_OBJECT(v),
            "ONE_LEFT" => s_ONE_LEFT(v),
            "ONE_TAKEN" => s_ONE_TAKEN(v),
            "PICK_AGAIN" => s_PICK_AGAIN(v),
            "POSSESSION" => s_POSSESSION(v),
            "STOPS_ON_AN_INITIAL" => s_STOPS_ON_AN_INITIAL(v),
            "THE_MOST_A_LIST" => s_THE_MOST_A_LIST(v),
            "TRIED_THE_LOT" => s_TRIED_THE_LOT(v),
            "_ASKS_THE_BOT" => s__ASKS_THE_BOT(v),
            "_A_REASON" => s__A_REASON(v),
            "_CLAUSE_VERB" => s__CLAUSE_VERB(v),
            "_FENCE" => s__FENCE(v),
            "_HELD_THE_FENCE" => s__HELD_THE_FENCE(v),
            "_I_HAVE" => s__I_HAVE(v),
            "_NOT_A_NAME" => s__NOT_A_NAME(v),
            "_NOT_A_THING_START" => s__NOT_A_THING_START(v),
            "_OFFER_NOT_A_LIST" => s__OFFER_NOT_A_LIST(v),
            "_OR_CHOICE" => s__OR_CHOICE(v),
            "_SOMEONE_ELSE_HAS_IT" => s__SOMEONE_ELSE_HAS_IT(v),
            "_STEP_START" => s__STEP_START(v),
            "_THE_LOT" => s__THE_LOT(v),
            "_TITLE" => s__TITLE(v),
            "_WHOLE_LIST" => s__WHOLE_LIST(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.reject", module_obj);
    pyrt::xrefs::reg_fn("memory.reject", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.reject", "refusal", py_func("refusal", |args: &[Py]| refusal(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_the_whole_name", py_func("_the_whole_name", |args: &[Py]| _the_whole_name(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_tidy", py_func("_tidy", |args: &[Py]| _tidy(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_by_title", py_func("_by_title", |args: &[Py]| _by_title(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_a_choice_of_things", py_func("_a_choice_of_things", |args: &[Py]| _a_choice_of_things(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_the_offer_reply", py_func("_the_offer_reply", |args: &[Py]| _the_offer_reply(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_options", py_func("_options", |args: &[Py]| _options(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_a_list_of_causes", py_func("_a_list_of_causes", |args: &[Py]| _a_list_of_causes(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_a_noun_phrase", py_func("_a_noun_phrase", |args: &[Py]| _a_noun_phrase(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_said_by_them", py_func("_said_by_them", |args: &[Py]| _said_by_them(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_offered", py_func("_offered", |args: &[Py]| _offered(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_the_fence", py_func("_the_fence", |args: &[Py]| _the_fence(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_the_whole_lot", py_func("_the_whole_lot", |args: &[Py]| _the_whole_lot(args)));
    pyrt::xrefs::reg_fn("memory.reject", "offers_a_choice", py_func("offers_a_choice", |args: &[Py]| offers_a_choice(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_the_names", py_func("_the_names", |args: &[Py]| _the_names(args)));
    pyrt::xrefs::reg_fn("memory.reject", "_the_lot_and_the_verb", py_func("_the_lot_and_the_verb", |args: &[Py]| _the_lot_and_the_verb(args)));
    pyrt::xrefs::reg_fn("memory.reject", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.reject", "ALL_OF_THEM", g_ALL_OF_THEM, s_ALL_OF_THEM);
    pyrt::xrefs::reg_access("memory.reject", "ANOTHER", g_ANOTHER, s_ANOTHER);
    pyrt::xrefs::reg_access("memory.reject", "A_HAVE_ALREADY", g_A_HAVE_ALREADY, s_A_HAVE_ALREADY);
    pyrt::xrefs::reg_access("memory.reject", "DONE", g_DONE, s_DONE);
    pyrt::xrefs::reg_access("memory.reject", "FENCED_OUT", g_FENCED_OUT, s_FENCED_OUT);
    pyrt::xrefs::reg_access("memory.reject", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("memory.reject", "JUDGED", g_JUDGED, s_JUDGED);
    pyrt::xrefs::reg_access("memory.reject", "KNOWN", g_KNOWN, s_KNOWN);
    pyrt::xrefs::reg_access("memory.reject", "MEMORY_STORE", g_MEMORY_STORE, s_MEMORY_STORE);
    pyrt::xrefs::reg_access("memory.reject", "NAME_GOES_ON", g_NAME_GOES_ON, s_NAME_GOES_ON);
    pyrt::xrefs::reg_access("memory.reject", "NEGATION", g_NEGATION, s_NEGATION);
    pyrt::xrefs::reg_access("memory.reject", "OBJECT", g_OBJECT, s_OBJECT);
    pyrt::xrefs::reg_access("memory.reject", "ONE_LEFT", g_ONE_LEFT, s_ONE_LEFT);
    pyrt::xrefs::reg_access("memory.reject", "ONE_TAKEN", g_ONE_TAKEN, s_ONE_TAKEN);
    pyrt::xrefs::reg_access("memory.reject", "PICK_AGAIN", g_PICK_AGAIN, s_PICK_AGAIN);
    pyrt::xrefs::reg_access("memory.reject", "POSSESSION", g_POSSESSION, s_POSSESSION);
    pyrt::xrefs::reg_access("memory.reject", "STOPS_ON_AN_INITIAL", g_STOPS_ON_AN_INITIAL, s_STOPS_ON_AN_INITIAL);
    pyrt::xrefs::reg_access("memory.reject", "THE_MOST_A_LIST", g_THE_MOST_A_LIST, s_THE_MOST_A_LIST);
    pyrt::xrefs::reg_access("memory.reject", "TRIED_THE_LOT", g_TRIED_THE_LOT, s_TRIED_THE_LOT);
    pyrt::xrefs::reg_access("memory.reject", "_ASKS_THE_BOT", g__ASKS_THE_BOT, s__ASKS_THE_BOT);
    pyrt::xrefs::reg_access("memory.reject", "_A_REASON", g__A_REASON, s__A_REASON);
    pyrt::xrefs::reg_access("memory.reject", "_CLAUSE_VERB", g__CLAUSE_VERB, s__CLAUSE_VERB);
    pyrt::xrefs::reg_access("memory.reject", "_FENCE", g__FENCE, s__FENCE);
    pyrt::xrefs::reg_access("memory.reject", "_HELD_THE_FENCE", g__HELD_THE_FENCE, s__HELD_THE_FENCE);
    pyrt::xrefs::reg_access("memory.reject", "_I_HAVE", g__I_HAVE, s__I_HAVE);
    pyrt::xrefs::reg_access("memory.reject", "_NOT_A_NAME", g__NOT_A_NAME, s__NOT_A_NAME);
    pyrt::xrefs::reg_access("memory.reject", "_NOT_A_THING_START", g__NOT_A_THING_START, s__NOT_A_THING_START);
    pyrt::xrefs::reg_access("memory.reject", "_OFFER_NOT_A_LIST", g__OFFER_NOT_A_LIST, s__OFFER_NOT_A_LIST);
    pyrt::xrefs::reg_access("memory.reject", "_OR_CHOICE", g__OR_CHOICE, s__OR_CHOICE);
    pyrt::xrefs::reg_access("memory.reject", "_SOMEONE_ELSE_HAS_IT", g__SOMEONE_ELSE_HAS_IT, s__SOMEONE_ELSE_HAS_IT);
    pyrt::xrefs::reg_access("memory.reject", "_STEP_START", g__STEP_START, s__STEP_START);
    pyrt::xrefs::reg_access("memory.reject", "_THE_LOT", g__THE_LOT, s__THE_LOT);
    pyrt::xrefs::reg_access("memory.reject", "_TITLE", g__TITLE, s__TITLE);
    pyrt::xrefs::reg_access("memory.reject", "_WHOLE_LIST", g__WHOLE_LIST, s__WHOLE_LIST);
}
