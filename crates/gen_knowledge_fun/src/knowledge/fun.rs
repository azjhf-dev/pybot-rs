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


thread_local! { static G_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAND_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIRTHDAY_TOASTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOAT_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEBATE_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRAWING_PROMPTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACTS_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FUN_FACTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_GAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAVING_TOASTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMING_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_TOASTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PODCAST_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIZ_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIZ_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIZ_NOT_A_QUIZ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETIREMENT_TOASTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RIDDLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOP_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOLVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMETHING_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TECH_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOAST_OCCASIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TONGUE_TWISTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEDDING_TOASTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRITING_PROMPTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FACT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FACT_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.fun: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT() -> Py { ensure_init(); G_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT(v: Py) { ensure_init(); G_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_NOUNS() -> Py { ensure_init(); G_ANIMAL_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_NOUNS(v: Py) { ensure_init(); G_ANIMAL_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED() -> Py { ensure_init(); G_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED(v: Py) { ensure_init(); G_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_BAND_NAMES() -> Py { ensure_init(); G_BAND_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAND_NAMES(v: Py) { ensure_init(); G_BAND_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_BIRTHDAY_TOASTS() -> Py { ensure_init(); G_BIRTHDAY_TOASTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIRTHDAY_TOASTS(v: Py) { ensure_init(); G_BIRTHDAY_TOASTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BOAT_NAMES() -> Py { ensure_init(); G_BOAT_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOAT_NAMES(v: Py) { ensure_init(); G_BOAT_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_DEBATE_TOPICS() -> Py { ensure_init(); G_DEBATE_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEBATE_TOPICS(v: Py) { ensure_init(); G_DEBATE_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBTED() -> Py { ensure_init(); G_DOUBTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBTED(v: Py) { ensure_init(); G_DOUBTED.with(|c| *c.borrow_mut() = v); }
pub fn g_DRAWING_PROMPTS() -> Py { ensure_init(); G_DRAWING_PROMPTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRAWING_PROMPTS(v: Py) { ensure_init(); G_DRAWING_PROMPTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FACTS_ABOUT() -> Py { ensure_init(); G_FACTS_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACTS_ABOUT(v: Py) { ensure_init(); G_FACTS_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_FUN_FACTS() -> Py { ensure_init(); G_FUN_FACTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FUN_FACTS(v: Py) { ensure_init(); G_FUN_FACTS.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_GAMES() -> Py { ensure_init(); G_GROUP_GAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_GAMES(v: Py) { ensure_init(); G_GROUP_GAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO() -> Py { ensure_init(); G_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); G_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAVING_TOASTS() -> Py { ensure_init(); G_LEAVING_TOASTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAVING_TOASTS(v: Py) { ensure_init(); G_LEAVING_TOASTS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMING_ASKED() -> Py { ensure_init(); G_NAMING_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMING_ASKED(v: Py) { ensure_init(); G_NAMING_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_TOASTS() -> Py { ensure_init(); G_PLAIN_TOASTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_TOASTS(v: Py) { ensure_init(); G_PLAIN_TOASTS.with(|c| *c.borrow_mut() = v); }
pub fn g_PODCAST_NAMES() -> Py { ensure_init(); G_PODCAST_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PODCAST_NAMES(v: Py) { ensure_init(); G_PODCAST_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTIONS() -> Py { ensure_init(); G_QUESTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTIONS(v: Py) { ensure_init(); G_QUESTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIZ_ASK() -> Py { ensure_init(); G_QUIZ_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIZ_ASK(v: Py) { ensure_init(); G_QUIZ_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIZ_ME() -> Py { ensure_init(); G_QUIZ_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIZ_ME(v: Py) { ensure_init(); G_QUIZ_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIZ_NOT_A_QUIZ() -> Py { ensure_init(); G_QUIZ_NOT_A_QUIZ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIZ_NOT_A_QUIZ(v: Py) { ensure_init(); G_QUIZ_NOT_A_QUIZ.with(|c| *c.borrow_mut() = v); }
pub fn g_RETIREMENT_TOASTS() -> Py { ensure_init(); G_RETIREMENT_TOASTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETIREMENT_TOASTS(v: Py) { ensure_init(); G_RETIREMENT_TOASTS.with(|c| *c.borrow_mut() = v); }
pub fn g_RIDDLES() -> Py { ensure_init(); G_RIDDLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RIDDLES(v: Py) { ensure_init(); G_RIDDLES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOP_NAMES() -> Py { ensure_init(); G_SHOP_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOP_NAMES(v: Py) { ensure_init(); G_SHOP_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_SOLVE() -> Py { ensure_init(); G_SOLVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOLVE(v: Py) { ensure_init(); G_SOLVE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMETHING_ASKED() -> Py { ensure_init(); G_SOMETHING_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMETHING_ASKED(v: Py) { ensure_init(); G_SOMETHING_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_TECH_NAMES() -> Py { ensure_init(); G_TECH_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TECH_NAMES(v: Py) { ensure_init(); G_TECH_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_NAMED() -> Py { ensure_init(); G_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_NAMED(v: Py) { ensure_init(); G_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_TOAST_OCCASIONS() -> Py { ensure_init(); G_TOAST_OCCASIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOAST_OCCASIONS(v: Py) { ensure_init(); G_TOAST_OCCASIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_TONGUE_TWISTERS() -> Py { ensure_init(); G_TONGUE_TWISTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TONGUE_TWISTERS(v: Py) { ensure_init(); G_TONGUE_TWISTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEDDING_TOASTS() -> Py { ensure_init(); G_WEDDING_TOASTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEDDING_TOASTS(v: Py) { ensure_init(); G_WEDDING_TOASTS.with(|c| *c.borrow_mut() = v); }
pub fn g_WRITING_PROMPTS() -> Py { ensure_init(); G_WRITING_PROMPTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRITING_PROMPTS(v: Py) { ensure_init(); G_WRITING_PROMPTS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_WORDS() -> Py { ensure_init(); G__ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); G__ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED() -> Py { ensure_init(); G__COUNTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED(v: Py) { ensure_init(); G__COUNTED.with(|c| *c.borrow_mut() = v); }
pub fn g__FACT_ASKS() -> Py { ensure_init(); G__FACT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FACT_ASKS(v: Py) { ensure_init(); G__FACT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__FACT_NOT() -> Py { ensure_init(); G__FACT_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FACT_NOT(v: Py) { ensure_init(); G__FACT_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Entertaining on request: a riddle, a fun fact, a quiz question, a toast, a debate.\n\n\"give me a riddle\" and \"tell me a fun fact\" are the same ask as \"tell me a joke\": the line\nwants an amusing thing to be said to it, not a fact looked up about a subject. `chat.py` keeps\nthe jokes, because a joke about a named subject has to be picked from the conversation; the\nbot had nothing at all for a riddle or a fun fact, and those two lines got the search hedge.\n\nSo this is a table, not a mechanism. Two entries, each a pool of ready-made replies; the pool\nis handed over whole and `pick` chooses from it, which is what keeps a person asking for three\nriddles in a row from getting the same one twice. Nothing here reads the line's subject: the\nentries only fire on the shape of the ask, so a line that names a subject (\"tell me facts about\ndogs\") is left to a module that knows about dogs.\n\nA riddle's answer goes in brackets right after the question. Given straight out, it is a riddle\nthe person can enjoy even if they do not want to guess; hidden behind a second turn, it would be\na question the bot cannot answer when they say \"i give up\".\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANIMAL_NOUNS(pyrt::xrefs::xget("knowledge.pets", "ANIMAL_NOUNS"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("creative")]));
    fn __data1() -> Py { py_tuple(vec![py_str("give"), py_str("tell"), py_str("know"), py_str("got"), py_str("have"), py_str("any"), py_str("another"), py_str("more"), py_str("share"), py_str("heard"), py_str("say"), py_str("suggest"), py_str("want"), py_str("wanted"), py_str("fancy"), py_str("hit"), py_str("need"), py_str("would like"), py_str("id like"), py_str("i d like"), py_str("read me")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.ASKED"); }
    s_ASKED(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.SOLVE"); }
    s_SOLVE(py_tuple(vec![py_str("answer"), py_str("solve"), py_str("solving"), py_str("solution"), py_str("figuring out"), py_str("guess"), py_str("guessing"), py_str("hint"), py_str("stuck on"), py_str("clue")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.DOUBTED"); }
    s_DOUBTED(py_tuple(vec![py_str("is that a fact"), py_str("thats a fact"), py_str("that s a fact"), py_str("is it a fact"), py_str("in fact"), py_str("fact is"), py_str("matter of fact"), py_str("as a fact"), py_str("the fact")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.ABOUT"); }
    s_ABOUT(py_tuple(vec![py_str("about"), py_str("regarding"), py_str("on the subject")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun._ASK_WORDS"); }
    s__ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nhey hi hello oi ok okay alright right so and now please go on come on then just\ncan could would will do you me us tell say give show share know\ngot have has any else\nsomething anything a an the\ninteresting surprising surprise entertain amuse true cool weird mad strange lovely nice\nonce most thing\nto hear read listen for is whats what's\nid like want wanna\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.TONGUE_TWISTERS"); }
    s_TONGUE_TWISTERS(py_tuple(vec![py_str("She sells sea shells by the sea shore. Say it three times, a bit faster each time."), py_str("Red lorry, yellow lorry, red lorry, yellow lorry. Say it out loud five times and the fourth one catches almost everybody."), py_str("Peter Piper picked a peck of pickled peppers. Start slowly and keep the p's crisp."), py_str("How much wood would a woodchuck chuck, if a woodchuck could chuck wood? Try it with a biscuit in your mouth if it gets easy."), py_str("A proper copper coffee pot. Short, and the hardest one here to say twice the same way."), py_str("The sixth sick sheikh's sixth sheep's sick. Nobody gets through this one clean on the first go, so start it slowly.")]));
    fn __data2() -> Py { py_tuple(vec![py_str("What has hands but cannot clap? (A clock.) Ask for another whenever you want one."), py_str("The more you take, the more you leave behind. What are they? (Footsteps.)"), py_str("What gets wetter the more it dries? (A towel.) Ask me for another if you fancy one."), py_str("What has a head and a tail but no body? (A coin.)"), py_str("What has keys but cannot open a single lock? (A piano.) Another riddle is one word away."), py_str("I am full of holes and still hold water. What am I? (A sponge.)"), py_str("What has to be broken before you can use it? (An egg.)"), py_str("What has one eye but cannot see? (A needle.) Ask for one more when you want it."), py_str("What belongs to you, but other people use it more than you do? (Your name.)"), py_str("What can go all the way round the world and never leave its corner? (A stamp.)"), py_str("What has words but never speaks? (A book.) There are more of these if you want another."), py_str("What goes up and never comes down? (Your age.)"), py_str("What has a neck but no head, and wears a cap? (A bottle.)"), py_str("What comes down but never goes up? (Rain.)"), py_str("Two fathers and two sons go fishing and catch three fish, one each. How? (They are a grandfather, a father and a son: three people, not four.)")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.RIDDLES"); }
    s_RIDDLES(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("Octopuses have three hearts and blue blood. Two hearts pump to the gills and one to the rest of the body, and the blood is blue because it carries oxygen on copper."), py_str("Sharks are older than trees. Sharks have been around about 450 million years, trees about 385 million."), py_str("A day on Venus is longer than its year: it turns once every 243 Earth days and goes round the sun in 225."), py_str("Bananas are berries and strawberries are not. Botanically a berry has its seeds inside, which a banana does and a strawberry, with its seeds on the outside, does not."), py_str("Wombats do cube-shaped droppings, and the corners come from the shape of the last bit of their gut."), py_str("A group of flamingos is called a flamboyance."), py_str("The Eiffel Tower is about 15cm taller in summer, because the iron expands in the heat."), py_str("A hummingbird's heart can beat more than 1,000 times a minute, and it can go into a sleep-like torpor at night to save energy."), py_str("Sea otters hold hands while they sleep so they do not drift apart."), py_str("The Moon moves about 3.8cm further away from the Earth every year. You can measure it with the mirrors the Apollo crews left behind."), py_str("Honey does not go off: it is too sugary and too acidic for bacteria to grow in, so a sealed jar keeps for years - and pots from Egyptian tombs have been found still edible."), py_str("Your body makes about a litre and a half of saliva a day, which is most of what you swallow."), py_str("There are more possible games of chess than there are atoms in the observable universe, which is why nobody has solved it."), py_str("A teaspoon of neutron star would weigh billions of tonnes on Earth, because the matter in it is packed as tightly as an atomic nucleus."), py_str("Snails have thousands of teeth, arranged in rows on a ribbon called a radula, and they use them like a file.")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.FUN_FACTS"); }
    s_FUN_FACTS(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("Which planet is tipped so far over that it rolls round the sun on its side? (Uranus - it is tilted about 98 degrees.) Ask for another whenever you like."), py_str("Which is the largest ocean on Earth? (The Pacific - it covers about a third of the planet.)"), py_str("How many bones are there in an adult human body? (206 - a baby is born with about 300, and many of them fuse together.)"), py_str("What is the chemical symbol for gold? (Au, from the Latin aurum. Silver is Ag, from argentum.)"), py_str("Which is the only mammal that can truly fly? (The bat. A flying squirrel only glides.)"), py_str("What is the capital of Australia? (Canberra - not Sydney or Melbourne.) Another one is one word away."), py_str("How long does sunlight take to reach the Earth? (About 8 minutes 20 seconds, so you always see the sun as it was eight minutes ago.)"), py_str("Which language has the most native speakers in the world? (Mandarin Chinese, with roughly 940 million.)"), py_str("What is the smallest country in the world? (Vatican City, about 0.44 square kilometres.)"), py_str("Who wrote Romeo and Juliet? (William Shakespeare, around 1595.)"), py_str("What is the hardest natural substance on Earth? (Diamond - a 10 on the Mohs scale, with corundum, the stuff of rubies and sapphires, down at 9.)"), py_str("How many players from one side are on the pitch in a football match? (Eleven, goalkeeper included.)"), py_str("At what temperature does water boil at sea level? (100°C. On a mountain it boils lower, because there is less air pressing on it.)"), py_str("What gas do plants take out of the air to grow? (Carbon dioxide - they keep the carbon and give the oxygen back.)"), py_str("How many strings does a standard guitar have? (Six.)"), py_str("In which year did people first walk on the Moon? (1969 - Apollo 11, in July.)"), py_str("Which is the biggest planet in our solar system? (Jupiter - it is more than twice as heavy as all the other planets put together.)"), py_str("Which is longer, a mile or a kilometre? (A mile: about 1.6 kilometres, which is why a 5k run is a bit over three miles.)"), py_str("What is the tallest animal in the world? (The giraffe, up to about 5.5 metres.)"), py_str("How many sides does a hexagon have? (Six.) Ask for another if you fancy a harder one."), py_str("What is the currency of Japan? (The yen.)"), py_str("Which sea has no coast? (The Sargasso Sea, in the middle of the Atlantic: it is held together by currents rather than by land.)")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.QUESTIONS"); }
    s_QUESTIONS(__data4());
    fn __data5() -> Py { py_dict(vec![(py_str("dogs"), py_tuple(vec![py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("pup"), py_str("pups"), py_str("doggy"), py_str("doggo")]), py_tuple(vec![py_str("A dog has about 300 million smell receptors in its nose, roughly forty times what a person has, which is why a walk is mostly reading."), py_str("A dog's nose print is as individual as a fingerprint, and the wetness helps it work out which direction a smell came from.")])])), (py_str("cats"), py_tuple(vec![py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("kitty"), py_str("moggy")]), py_tuple(vec![py_str("A cat has about thirty muscles in each ear and can turn them most of the way round, and it spends two thirds of its life asleep - up to sixteen hours a day."), py_str("A cat's purr runs at 25 to 150 vibrations a second, and cats also purr when they are hurt or frightened, not only when they are content.")])])), (py_str("bees"), py_tuple(vec![py_tuple(vec![py_str("bee"), py_str("bees"), py_str("honeybee"), py_str("honeybees"), py_str("bumblebee"), py_str("bumblebees"), py_str("bumble bee"), py_str("bumble bees")]), py_tuple(vec![py_str("A honeybee that finds food comes back and dances: a wiggle run straight up the comb means fly towards the sun, and how long she wiggles says how far."), py_str("One worker bee makes about a twelfth of a teaspoon of honey in her whole life, which is why a jar is a lot of flying.")])])), (py_str("elephants"), py_tuple(vec![py_tuple(vec![py_str("elephant"), py_str("elephants")]), py_tuple(vec![py_str("Elephants talk in rumbles mostly too low for us to hear, and a herd can pick one up from several kilometres away through the ground as well as the air."), py_str("An elephant's trunk has around 40,000 muscles in it and no bones at all, which is how it can pick up a single blade of grass and then lift a tree.")])])), (py_str("spiders"), py_tuple(vec![py_tuple(vec![py_str("spider"), py_str("spiders"), py_str("tarantula"), py_str("tarantulas")]), py_tuple(vec![py_str("A spider feels its web rather than sees it: one strand tells it the size of what landed and whether it is still struggling."), py_str("Spider silk is stronger than steel for its weight and can stretch about four times its length before it breaks.")])])), (py_str("birds"), py_tuple(vec![py_tuple(vec![py_str("bird"), py_str("birds"), py_str("sparrow"), py_str("sparrows"), py_str("robin"), py_str("robins")]), py_tuple(vec![py_str("A bird's lungs fill in one direction only, with air sacs behind it, which is how a bar-tailed godwit flew 12,000 km without stopping."), py_str("Most small birds weigh less than the batteries in a television remote, and a blue tit eats close to its own body weight in food every day in winter.")])])), (py_str("whales"), py_tuple(vec![py_tuple(vec![py_str("whale"), py_str("whales"), py_str("dolphin"), py_str("dolphins")]), py_tuple(vec![py_str("A blue whale's heart is about the size of a small car and slows to two beats a minute when it dives, and its call carries hundreds of kilometres through water."), py_str("Dolphins sleep one half of the brain at a time, keeping the other half and one eye on watch so they remember to breathe.")])])), (py_str("penguins"), py_tuple(vec![py_tuple(vec![py_str("penguin"), py_str("penguins")]), py_tuple(vec![py_str("Penguins cannot taste sweetness - the gene for it is broken in them - and they can drink sea water because a gland above the eye filters the salt out."), py_str("A penguin's knees and thighs are inside its body, which is why it looks like it has none and why it waddles instead of striding.")])])), (py_str("butterflies"), py_tuple(vec![py_tuple(vec![py_str("butterfly"), py_str("butterflies"), py_str("moth"), py_str("moths")]), py_tuple(vec![py_str("Butterflies taste with their feet, and they cannot fly once their body drops below about 15C, which is why they sit with their wings open in the sun."), py_str("A caterpillar turns almost all of itself into soup inside the chrysalis and rebuilds from the same cells, so the butterfly is not the caterpillar with wings on.")])])), (py_str("trees"), py_tuple(vec![py_tuple(vec![py_str("tree"), py_str("trees"), py_str("oak"), py_str("oaks"), py_str("forest"), py_str("forests"), py_str("woodland")]), py_tuple(vec![py_str("Trees pass sugar to each other through fungi in the soil: a beech can feed a shaded seedling of its own kind through that network."), py_str("A big oak can pull hundreds of litres of water a day up its trunk, and the tallest trees are limited by how high water can be lifted, not by how strong the wood is.")])])), (py_str("rain"), py_tuple(vec![py_tuple(vec![py_str("rain"), py_str("raining"), py_str("rainy"), py_str("weather"), py_str("thunder"), py_str("thunderstorm"), py_str("lightning")]), py_tuple(vec![py_str("The smell after rain is not the rain: it is geosmin, made by soil bacteria, and a human nose can pick it out at a few parts per trillion."), py_str("Lightning heats the air around it to about 30,000C, five times the surface of the sun, and the bang is that air expanding at once.")])])), (py_str("sleep"), py_tuple(vec![py_tuple(vec![py_str("sleep"), py_str("sleeping"), py_str("insomnia"), py_str("dream"), py_str("dreams"), py_str("dreaming")]), py_tuple(vec![py_str("After about seventeen hours awake, people perform about as badly as someone at the drink-drive limit, which is why a late night feels like a hangover."), py_str("The brain flushes waste out of itself mostly while you sleep, using a channel that opens up in deep sleep and barely runs when you are awake.")])])), (py_str("coffee"), py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("caffeine"), py_str("espresso")]), py_tuple(vec![py_str("Caffeine does not give you energy: it blocks the receptor that tells you you are tired, so the tiredness piles up behind it and lands when the coffee wears off."), py_str("Coffee was chewed, not drunk, for centuries before anyone thought to brew it, and the plant is a fruit stone: the bean is the seed of a cherry.")])])), (py_str("chocolate"), py_tuple(vec![py_tuple(vec![py_str("chocolate"), py_str("cocoa"), py_str("cacao")]), py_tuple(vec![py_str("Chocolate was drunk for most of its history - the Aztecs took it cold, bitter and spiced with chilli - and milk chocolate only arrived in 1875."), py_str("Cocoa trees grow in the shade of taller ones, and a tree takes four or five years before it makes a pod worth picking.")])])), (py_str("cheese"), py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("cheddar"), py_str("brie")]), py_tuple(vec![py_str("Cheese is the same few ingredients with different bacteria, and much of the flavour is made at the rind: a washed rind is wiped with brine so the smellier bacteria win."), py_str("Hard cheese keeps for months because most of the water has been pressed and salted out of it, which is what stops the mould that would spoil milk.")])])), (py_str("brain"), py_tuple(vec![py_tuple(vec![py_str("brain"), py_str("brains"), py_str("memory"), py_str("memories")]), py_tuple(vec![py_str("Your brain is about two per cent of your weight and uses about a fifth of your energy, and it has no pain receptors of its own - a headache comes from the layers and vessels around it."), py_str("A memory is not stored in one place: remembering something re-runs the same pattern across the brain, which is why a memory changes a little each time you recall it.")])])), (py_str("sun"), py_tuple(vec![py_tuple(vec![py_str("sun"), py_str("sunlight"), py_str("sunshine")]), py_tuple(vec![py_str("The sunlight on your arm is old: the energy took tens of thousands of years to work its way out of the sun's core, and then eight minutes to cross to you."), py_str("The sun is 99.8 per cent of the mass of the solar system - everything else, planets included, is the leftover dust.")])])), (py_str("sea"), py_tuple(vec![py_tuple(vec![py_str("sea"), py_str("ocean"), py_str("oceans"), py_str("seabed"), py_str("sea bed")]), py_tuple(vec![py_str("We have better maps of Mars than of the sea floor: about three quarters of it has never been surveyed in detail, and the deepest trench would swallow Everest with two kilometres to spare."), py_str("The sea is salty because rain has been washing rock into it for billions of years, and the water that leaves by evaporation takes none of the salt with it.")])])), (py_str("space"), py_tuple(vec![py_tuple(vec![py_str("space"), py_str("universe"), py_str("galaxy"), py_str("galaxies"), py_str("star"), py_str("stars"), py_str("planet"), py_str("planets")]), py_tuple(vec![py_str("There are more stars in the observable universe than grains of sand on every beach on Earth, and the space between them is still stretching."), py_str("A day on the International Space Station is 90 minutes long, because that is how long it takes to go round the Earth once.")])])), (py_str("music"), py_tuple(vec![py_tuple(vec![py_str("music"), py_str("song"), py_str("songs"), py_str("melody")]), py_tuple(vec![py_str("The shiver down your spine at a good bit of music comes before the moment you expect, not on it: the brain is rewarding itself for predicting what comes next."), py_str("Music uses the same reward circuits as food and friends, which is why a song can lift a mood in seconds and a smell cannot.")])])), (py_str("money"), py_tuple(vec![py_tuple(vec![py_str("money"), py_str("cash"), py_str("banknotes"), py_str("coins")]), py_tuple(vec![py_str("Most money is not paper: only a small fraction of what is counted as money exists as notes and coins, and the rest is numbers in accounts."), py_str("Notes are printed on cotton and linen, not wood pulp, which is why a note survives the wash better than a receipt does.")])])), (py_str("beetles"), py_tuple(vec![py_tuple(vec![py_str("beetle"), py_str("beetles"), py_str("insect"), py_str("insects"), py_str("bug"), py_str("bugs")]), py_tuple(vec![py_str("One in four of every animal species that has ever been named is a beetle - about 400,000 of them - so a beetle is the safest guess for any unnamed insect."), py_str("Insects breathe through holes in their sides instead of lungs, which is part of why there are no insects the size of a cat.")])])), (py_str("dinosaurs"), py_tuple(vec![py_tuple(vec![py_str("dinosaur"), py_str("dinosaurs"), py_str("t rex"), py_str("t-rex"), py_str("tyrannosaurus")]), py_tuple(vec![py_str("Birds are dinosaurs: the line that survived the asteroid is the one that became birds, which is why a chicken's skeleton is closer to a T. rex than a lizard's is."), py_str("Many dinosaurs were feathered and coloured, not scaly and green: the skin we have fossil evidence for on several of them is a coat of feathers.")])])), (py_str("mountains"), py_tuple(vec![py_tuple(vec![py_str("mountain"), py_str("mountains"), py_str("everest"), py_str("himalayas"), py_str("alps")]), py_tuple(vec![py_str("Everest is the highest, not the tallest: from its base on the sea floor, Mauna Kea in Hawaii is over ten kilometres tall."), py_str("Everest is still going up by a few millimetres a year as India pushes into Asia, and the summit is a marine limestone - it was sea floor once.")])])), (py_str("water"), py_tuple(vec![py_tuple(vec![py_str("water"), py_str("ice"), py_str("snow")]), py_tuple(vec![py_str("Water is the only common substance that is lighter as a solid, which is why ice floats and a pond freezes from the top down instead of from the bottom."), py_str("Hot water can freeze faster than cold in some conditions - it has a name, the Mpemba effect - and nobody has a settled explanation for it.")])])), (py_str("octopus"), py_tuple(vec![py_tuple(vec![py_str("octopus"), py_str("octopuses"), py_str("octopi")]), py_tuple(vec![py_str("An octopus can change colour and the texture of its skin in under a second, and it is colour-blind - nobody knows for certain how it matches colours it cannot see."), py_str("Two thirds of an octopus's neurons are in its arms, so an arm can taste what it touches and work on its own while the head does something else.")])])), (py_str("bananas"), py_tuple(vec![py_tuple(vec![py_str("banana"), py_str("bananas")]), py_tuple(vec![py_str("The banana plant is not a tree: it is the largest herb, and what looks like a trunk is the bases of the leaves packed together."), py_str("A banana is slightly radioactive because it is rich in potassium, and so is a potato - the dose from either is far too small to matter.")])])), (py_str("body"), py_tuple(vec![py_tuple(vec![py_str("body"), py_str("bodies"), py_str("human body"), py_str("the body")]), py_tuple(vec![py_str("Your body makes about a litre of new bone a year of the two or so kilograms it carries, which is why a broken bone heals back stronger at the break."), py_str("You are taller in the morning: the discs between your vertebrae squash flat over a day and spring back overnight, up to a centimetre and a half.")])])), (py_str("chilli"), py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chilli pepper"), py_str("chillies"), py_str("chili"), py_str("chilies"), py_str("jalapeno"), py_str("jalapenos"), py_str("hot sauce")]), py_tuple(vec![py_str("Chilli heat is not a taste: capsaicin trips the receptor that senses a hot pan, which is why your mouth reads it as burning and why milk works where water does not."), py_str("The hottest chillies are measured in Scoville units, and a habanero can be a hundred times hotter than a jalapeno that looks the same size.")])]))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.FACTS_ABOUT"); }
    s_FACTS_ABOUT(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("fun fact"), py_str("fun facts"), py_str("random fact"), py_str("random facts"), py_str("interesting fact"), py_str("interesting facts"), py_str("true fact"), py_str("cool fact"), py_str("cool facts"), py_str("weird fact"), py_str("weird facts"), py_str("surprising fact"), py_str("surprising facts"), py_str("amazing fact"), py_str("amazing facts"), py_str("facts about"), py_str("fact about"), py_str("facts on"), py_str("fact on"), py_str("something interesting"), py_str("something cool"), py_str("something surprising"), py_str("something amazing"), py_str("did you know"), py_str("learn something"), py_str("a fact"), py_str("fact"), py_str("facts")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun._FACT_ASKS"); }
    s__FACT_ASKS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("died"), py_str("dead"), py_str("dying"), py_str("passed away"), py_str("put down"), py_str("euthan"), py_str("lost my"), py_str("grieving"), py_str("buried"), py_str("buried"), py_str("sick"), py_str("ill"), py_str("poorly"), py_str("vet"), py_str("vets"), py_str("hurt"), py_str("hurting"), py_str("allergic"), py_str("bite"), py_str("bites"), py_str("bitten"), py_str("sting"), py_str("stings"), py_str("stung"), py_str("attack"), py_str("attacked"), py_str("afraid"), py_str("scared"), py_str("phobia"), py_str("hate"), py_str("hates"), py_str("how do i"), py_str("how to"), py_str("how can i"), py_str("why does"), py_str("why do"), py_str("why is"), py_str("my dog"), py_str("my cat"), py_str("my puppy"), py_str("my kitten"), py_str("my child"), py_str("my son"), py_str("my daughter"), py_str("is it true"), py_str("is that true"), py_str("true that"), py_str("myth"), py_str("fact check"), py_str("in fact"), py_str("as a fact"), py_str("matter of fact"), py_str("the fact"), py_str("that s a fact"), py_str("myth or fact")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun._FACT_NOT"); }
    s__FACT_NOT(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve"), py_str("twenty"), py_str("hundred"), py_str("thousand"), py_str("million"), py_str("billion"), py_str("how many"), py_str("how much"), py_str("how long"), py_str("how old"), py_str("how tall"), py_str("how big"), py_str("how far")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun._COUNTED"); }
    s__COUNTED(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("quiz me"), py_str("quiz"), py_str("quizzes"), py_str("quiz question"), py_str("quiz questions"), py_str("trivia"), py_str("trivia question"), py_str("trivia questions"), py_str("general knowledge"), py_str("general knowledge question"), py_str("test my knowledge"), py_str("test me"), py_str("ask me a question"), py_str("ask me some questions"), py_str("ask me something"), py_str("a question to answer"), py_str("questions to answer"), py_str("throw me a question"), py_str("throw me one"), py_str("send me a question"), py_str("give me a question"), py_str("chuck me a question"), py_str("hit me with a question"), py_str("ask us a question"), py_str("ask me another question"), py_str("ask me another one"), py_str("ask me something else")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.QUIZ_ASK"); }
    s_QUIZ_ASK(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("me"), py_str("us"), py_str("question"), py_str("questions"), py_str("quiz"), py_str("quizzes"), py_str("trivia"), py_str("something"), py_str("some"), py_str("any"), py_str("another"), py_str("more"), py_str("one"), py_str("please"), py_str("pls"), py_str("again"), py_str("knowledge")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.QUIZ_ME"); }
    s_QUIZ_ME(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("tomorrow"), py_str("tonight"), py_str("night"), py_str("pub"), py_str("starts"), py_str("started"), py_str("team"), py_str("show"), py_str("revise"), py_str("revision"), py_str("revising"), py_str("exam"), py_str("exams"), py_str("make a quiz"), py_str("writing a quiz"), py_str("write a quiz"), py_str("host"), py_str("rounds"), py_str("scores"), py_str("score"), py_str("spellings"), py_str("spelling"), py_str("times tables"), py_str("maths"), py_str("any good"), py_str("good at")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.QUIZ_NOT_A_QUIZ"); }
    s_QUIZ_NOT_A_QUIZ(__data11());
    fn __data12() -> Py { py_tuple(vec![py_str("boat"), py_str("boats"), py_str("yacht"), py_str("kayak"), py_str("canoe"), py_str("band"), py_str("bands"), py_str("pub"), py_str("cafe"), py_str("café"), py_str("shop"), py_str("business"), py_str("company"), py_str("startup"), py_str("podcast"), py_str("quiz team"), py_str("team"), py_str("sourdough starter"), py_str("sourdough"), py_str("starter"), py_str("wifi"), py_str("wi-fi"), py_str("network"), py_str("campervan"), py_str("van"), py_str("motorhome"), py_str("car"), py_str("bike"), py_str("motorbike"), py_str("allotment plot"), py_str("house"), py_str("flat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.THING_NAMED"); }
    s_THING_NAMED(__data12());
    fn __data13() -> Py { py_tuple(vec![py_str("what should i name"), py_str("what should we name"), py_str("what should i call"), py_str("what should we call"), py_str("what to name"), py_str("what could i name"), py_str("name for my"), py_str("name my"), py_str("name our"), py_str("name ideas"), py_str("ideas for a name"), py_str("any good names"), py_str("good name"), py_str("good names"), py_str("help me name"), py_str("suggest a name"), py_str("suggest some names"), py_str("what would you call"), py_str("names for my"), py_str("names for a")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.NAMING_ASKED"); }
    s_NAMING_ASKED(__data13());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.BOAT_NAMES"); }
    s_BOAT_NAMES(py_tuple(vec![py_str("Second Wind"), py_str("Kittiwake"), py_str("Slow Motion")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.BAND_NAMES"); }
    s_BAND_NAMES(py_tuple(vec![py_str("The Late Bus"), py_str("Half Hour"), py_str("Paper Lions")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.SHOP_NAMES"); }
    s_SHOP_NAMES(py_tuple(vec![py_str("Two Rivers"), py_str("The Workshop"), py_str("Day One")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.TECH_NAMES"); }
    s_TECH_NAMES(py_tuple(vec![py_str("Tin Roof"), py_str("Long Way Round"), py_str("The Quiet Bit")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.PODCAST_NAMES"); }
    s_PODCAST_NAMES(py_tuple(vec![py_str("Two Chairs"), py_str("The Long Way"), py_str("Kitchen Table")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.GROUP_GAMES"); }
    s_GROUP_GAMES(py_tuple(vec![py_str("Two truths and a lie: each of you says three things about yourself, two true and one made up, and the rest guess the lie - a point for each correct guess, and you tell them the answer before the next person goes. Three is the right number of players upwards, and the lies get better by the second round."), py_str("Would you rather: one of you puts two bad options against each other and everybody has to pick one and say why in a sentence - 'would you rather be ten minutes late every day or an hour late once a week'. The reasons are the fun part, not the choice."), py_str("Twenty questions: one person thinks of a person, a place or a thing, the others ask questions that can only be answered yes or no, and twenty questions has to be enough. It works in a car, at a table, and with children from about six."), py_str("The alphabet game for a journey: find each letter of the alphabet in order on road signs, number plates or shop fronts, and nobody may use a word twice. First to Z wins, and the hard letters are the fun ones.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.DRAWING_PROMPTS"); }
    s_DRAWING_PROMPTS(py_tuple(vec![py_str("Draw an animal wearing a hat it would never choose. Give them three minutes and no sharpener: the limit is what gets a drawing finished, and a finished silly one beats a perfect blank page."), py_str("Draw the kitchen as a mouse would see it - everything enormous and books on the floor. One shape to start with stops the 'I don't know what to draw' that a blank page causes."), py_str("Draw a house for the person sitting next to you: what is on the roof, what is behind the door. Ask them what is missing afterwards and let them add it in pen."), py_str("Draw the same mug three times: once in one line without lifting the pen, once looking only at the mug and never at the paper, once in under a minute. Three bad drawings beat one careful blank page, and the second one usually surprises you.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.WRITING_PROMPTS"); }
    s_WRITING_PROMPTS(py_tuple(vec![py_str("Someone finds a key that fits no lock they own, and keeps it for a month before they find out what it opens. Two hundred words, no planning, and do not give it an ending - stop where the interesting part starts."), py_str("Write the same small scene twice: once from the person who is late, once from the person waiting. Nothing has to happen in either; the two ways of seeing the same ten minutes are the whole exercise."), py_str("A character has to tell a lie to somebody they love, on a day that is otherwise ordinary. Write the ten minutes before they do it and leave the lie out of the page."), py_str("Describe a room from the point of view of the smell in it. Three hundred words, and no person may be named: it forces the details out of you.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.HOW_TO"); }
    s_HOW_TO(py_tuple(vec![py_str("how do i"), py_str("how do you"), py_str("how to"), py_str("why does"), py_str("why do"), py_str("what is the best way"), py_str("recipe"), py_str("make"), py_str("making"), py_str("cook"), py_str("cooking"), py_str("recipe for")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.WEDDING_TOASTS"); }
    s_WEDDING_TOASTS(py_tuple(vec![py_str("\"To Anna and James - may you always be as easy with each other as you were on the day you met.\" Raise the glass on the names and sit down: twenty seconds is a good toast."), py_str("Name the couple and one thing you have watched them do well together, then finish with four words: \"To Sarah and Tom - to a long and happy life.\""), py_str("\"To the two of you - may the good days outnumber the washing up.\" Short and warm beats clever, and everyone drinks on it.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.BIRTHDAY_TOASTS"); }
    s_BIRTHDAY_TOASTS(py_tuple(vec![py_str("\"To Dan - forty is the age where you finally stop pretending to like the music at parties.\" One line and one laugh, then everyone drinks and nobody has to answer."), py_str("Name the person and one thing you like about them, then raise the glass: \"To Ruth - the only person here who would drive across town for a cake.\"")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.RETIREMENT_TOASTS"); }
    s_RETIREMENT_TOASTS(py_tuple(vec![py_str("\"To Brian - thirty years of getting there before the rest of us, and now every day is Saturday.\" Then hand over whatever joke present you brought."), py_str("Name the years and one thing they were known for, then raise the glass: \"To Pat - the only person who could fix the printer. Enjoy the lie-ins.\"")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.LEAVING_TOASTS"); }
    s_LEAVING_TOASTS(py_tuple(vec![py_str("\"To Maya - the best thing about Mondays for four years. Don't be a stranger.\" Keep it to one detail you actually saw, and stop before it turns into a speech."), py_str("Say what the team will miss in one sentence and finish with the glass: \"To Tom - the only one who ever read the whole email. Good luck out there.\"")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.PLAIN_TOASTS"); }
    s_PLAIN_TOASTS(py_tuple(vec![py_str("Name the person, one real thing they did that you saw yourself, then raise the glass: \"To Ruth - she has fed half this room and never once let on how much work it was.\" That is the whole toast."), py_str("Three parts, in this order: who you are raising the glass to, one thing they did, and four words to finish - \"To Sarah and Tom.\" A toast is twenty seconds; a speech is longer and wants a joke at the start."), py_str("If you are nervous, write five words on your hand and say them slowly: \"To health, happiness and a quiet Sunday.\" Said warmly, a short toast lands better than a long one read out.")]));
    fn __data14() -> Py { py_tuple(vec![py_str("birthday"), py_str("birthdays"), py_str("forty"), py_str("fifty"), py_str("sixty"), py_str("seventy"), py_str("eighteenth"), py_str("eighteen"), py_str("twenty first"), py_str("21st"), py_str("40th"), py_str("50th"), py_str("60th"), py_str("70th")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.TOAST_OCCASIONS"); }
    s_TOAST_OCCASIONS(py_tuple(vec![py_tuple(vec![py_str("wedding"), py_tuple(vec![py_str("wedding"), py_str("bride"), py_str("groom"), py_str("married"), py_str("marriage"), py_str("couple"), py_str("engagement"), py_str("anniversary")]), g_WEDDING_TOASTS()]), py_tuple(vec![py_str("birthday"), __data14(), g_BIRTHDAY_TOASTS()]), py_tuple(vec![py_str("retirement"), py_tuple(vec![py_str("retirement"), py_str("retiring"), py_str("retired"), py_str("last day at work"), py_str("pension")]), g_RETIREMENT_TOASTS()]), py_tuple(vec![py_str("leaving"), py_tuple(vec![py_str("leaving"), py_str("new job"), py_str("moving on"), py_str("leaving do"), py_str("goodbye"), py_str("farewell"), py_str("colleague")]), g_LEAVING_TOASTS()])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.DEBATE_TOPICS"); }
    s_DEBATE_TOPICS(py_tuple(vec![py_str("This house would ban all cars from city centres. (Good for a school debate: clean arguments on both sides, air quality and business against freedom and disability access.)"), py_str("Social media does more harm than good to teenagers. (Evidence both ways, and everyone has a view - the safest topic for a group that does not agree on much.)"), py_str("This house believes homework should be scrapped for primary school children."), py_str("University should be free for everyone, paid for by general taxation."), py_str("This house would make voting compulsory, with a small fine for not voting."), py_str("Artificial intelligence will do more good than harm to ordinary jobs over the next twenty years."), py_str("This house believes professional athletes are paid too much."), py_str("This house would tax sugary drinks to pay for free school meals."), py_str("Zoos do more harm than good for the animals in them."), py_str("This house believes that four-day working weeks should be the standard.")]));
    fn __data15() -> Py { py_tuple(vec![py_str("something interesting"), py_str("anything interesting"), py_str("something surprising"), py_str("tell me something"), py_str("say something"), py_str("something to tell me"), py_str("surprise me"), py_str("entertain me"), py_str("amuse me"), py_str("interesting thing"), py_str("surprising thing"), py_str("something cool"), py_str("anything cool"), py_str("something mad"), py_str("anything mad"), py_str("something weird"), py_str("anything weird")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.SOMETHING_ASKED"); }
    s_SOMETHING_ASKED(__data15());
    fn __data17() -> Py { py_tuple(vec![py_str("bread"), py_str("cheese"), py_str("avocado"), py_str("butter"), py_str("toaster"), py_str("burnt"), py_str("burn"), py_str("burned"), py_str("sandwich"), py_str("breakfast"), py_str("marmite"), py_str("beans"), py_str("slice"), py_str("slices"), py_str("loaf"), py_str("egg on"), py_str("cinnamon"), py_str("cereal"), py_str("cereals"), py_str("cornflakes"), py_str("porridge"), py_str("granola"), py_str("weetabix"), py_str("jam"), py_str("marmalade"), py_str("peanut butter"), py_str("honey"), py_str("banana"), py_str("yoghurt"), py_str("yogurt"), py_str("fruit"), py_str("eggs"), py_str("scrambled"), py_str("poached"), py_str("fry up"), py_str("every morning"), py_str("each morning"), py_str("in the morning"), py_str("for breakfast"), py_str("usually have"), py_str("have toast"), py_str("had toast"), py_str("has toast"), py_str("eats toast"), py_str("eating toast"), py_str("toast and"), py_str("and toast"), py_str("toast with"), py_str("toast on"), py_str("plain toast"), py_str("dry toast"), py_str("piece of toast"), py_str("bit of toast"), py_str("draft"), py_str("speech"), py_str("script")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("wedding"), py_str("birthday"), py_str("dinner"), py_str("party"), py_str("speech"), py_str("what do i say"), py_str("say"), py_str("few words"), py_str("evening"), py_str("celebrate"), py_str("celebration")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("bread"), py_str("butter"), py_str("cheese"), py_str("avocado"), py_str("marmite"), py_str("beans"), py_str("slice"), py_str("toaster"), py_str("burnt"), py_str("breakfast"), py_str("toast fingers"), py_str("pregnant"), py_str("baby")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("essay"), py_str("essay title"), py_str("statement"), py_str("assignment"), py_str("public speaking"), py_str("nerves"), py_str("nervous"), py_str("speech"), py_str("argue with"), py_str("argument with"), py_str("my partner"), py_str("my friend"), py_str("my mum"), py_str("my dad"), py_str("my boss")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("quiz night"), py_str("pub quiz"), py_str("quiz idea"), py_str("quiz ideas"), py_str("quiz round"), py_str("quiz rounds"), py_str("quiz questions for"), py_str("quiz for a party"), py_str("family quiz"), py_str("quiz with friends"), py_str("write a quiz"), py_str("run a quiz"), py_str("host a quiz"), py_str("make a quiz"), py_str("a quiz for")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("ideas"), py_str("idea"), py_str("rounds"), py_str("round"), py_str("questions"), py_str("how"), py_str("what should"), py_str("what rounds"), py_str("suggest"), py_str("help"), py_str("give me"), py_str("want some"), py_str("need some"), py_str("plan"), py_str("tips"), py_str("any"), py_str("good"), py_str("night"), py_str("party")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("quiz me"), py_str("quiz us"), py_str("team name"), py_str("team names"), py_str("name our"), py_str("name the"), py_str("good idea"), py_str("bad idea"), py_str("worth it"), py_str("is it a good"), py_str("is a pub quiz a"), py_str("should we do"), py_str("shall we do"), py_str("we are doing"), py_str("we're doing"), py_str("im doing"), py_str("i'm doing")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("name"), py_str("names"), py_str("named"), py_str("naming"), py_str("call"), py_str("called"), py_str("suggest"), py_str("ideas"), py_str("idea"), py_str("any"), py_str("good"), py_str("clever"), py_str("help"), py_str("pick"), py_str("come up with"), py_str("think of")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("hamster"), py_str("rabbit"), py_str("bunny"), py_str("guinea pig"), py_str("parrot"), py_str("budgie"), py_str("fish"), py_str("horse"), py_str("pony"), py_str("tortoise"), py_str("pet"), py_str("pets"), py_str("baby"), py_str("babies"), py_str("child"), py_str("children"), py_str("son"), py_str("daughter"), py_str("plant"), py_str("plants"), py_str("garden"), py_str("allotment"), py_str("whats my"), py_str("what is my"), py_str("named after"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("how do i change"), py_str("change my"), py_str("rename"), py_str("band"), py_str("bands"), py_str("wifi"), py_str("wi-fi"), py_str("network"), py_str("quiz team"), py_str("quiz teams"), py_str("team"), py_str("teams"), py_str("sourdough"), py_str("sourdough starter"), py_str("starter")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("two truths and a lie"), py_str("2 truths and a lie"), py_str("truths and a lie"), py_str("truth or lie"), py_str("would you rather"), py_str("never have i ever"), py_str("twenty questions"), py_str("20 questions"), py_str("alphabet game"), py_str("the alphabet game"), py_str("party game"), py_str("party games"), py_str("car game"), py_str("car games"), py_str("game for the car"), py_str("games for the car"), py_str("game for a long journey"), py_str("games for a long journey"), py_str("game for the kids"), py_str("games for the kids"), py_str("games for a group"), py_str("game for a group"), py_str("icebreaker"), py_str("ice breaker"), py_str("icebreakers"), py_str("games to play"), py_str("game"), py_str("games"), py_str("play with my kids"), py_str("play with the kids"), py_str("play with my children"), py_str("play with the children"), py_str("play with my family"), py_str("play with the family"), py_str("play a game"), py_str("play some games"), py_str("what can we play"), py_str("what should i play"), py_str("what shall we play"), py_str("what can i play")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("good"), py_str("best"), py_str("whats"), py_str("what's"), py_str("what is"), py_str("play"), py_str("play a"), py_str("play some"), py_str("game"), py_str("games"), py_str("bored"), py_str("boring"), py_str("journey"), py_str("car"), py_str("kids"), py_str("children"), py_str("group"), py_str("family"), py_str("rainy"), py_str("flight"), py_str("question"), py_str("questions"), py_str("ask me"), py_str("ask us"), py_str("suggest"), py_str("ideas")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("bored"), py_str("boring"), py_str("nothing to do"), py_str("climbing the walls"), py_str("stuck inside"), py_str("stuck indoors"), py_str("den"), py_str("board game"), py_str("board games"), py_str("video game"), py_str("video games"), py_str("xbox"), py_str("playstation"), py_str("nintendo"), py_str("switch"), py_str("console"), py_str("minecraft"), py_str("fortnite"), py_str("game of thrones"), py_str("hunger games"), py_str("hunger"), py_str("the game"), py_str("football"), py_str("match"), py_str("ticket"), py_str("tickets"), py_str("two player"), py_str("two-player"), py_str("2 player"), py_str("2-player"), py_str("for two"), py_str("the two of us"), py_str("just us"), py_str("partner"), py_str("couple"), py_str("two people"), py_str("2 people"), py_str("two of you"), py_str("the pair of you"), py_str("scam"), py_str("gamble"), py_str("gambling"), py_str("bet"), py_str("betting"), py_str("rigged"), py_str("music"), py_str("guitar"), py_str("piano"), py_str("song"), py_str("songs"), py_str("album"), py_str("playlist"), py_str("record"), py_str("vinyl"), py_str("tennis"), py_str("golf"), py_str("cricket"), py_str("darts"), py_str("poker"), py_str("chess"), py_str("violin"), py_str("drums"), py_str("stressed"), py_str("anxious"), py_str("calm"), py_str("relax"), py_str("relaxing"), py_str("chill"), py_str("wind down"), py_str("short"), py_str("quick"), py_str("solo"), py_str("alone"), py_str("by myself"), py_str("on my own"), py_str("single player"), py_str("weeknight")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("draw"), py_str("draws"), py_str("drawing"), py_str("doodle"), py_str("doodling"), py_str("sketch"), py_str("sketching"), py_str("pencil"), py_str("crayons"), py_str("colouring"), py_str("coloring"), py_str("something to draw"), py_str("what to draw")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("prompt"), py_str("prompts"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("bored"), py_str("rainy"), py_str("easy"), py_str("simple"), py_str("quick"), py_str("fun"), py_str("what should i"), py_str("what can i"), py_str("something to draw"), py_str("to draw"), py_str("draw"), py_str("drawing"), py_str("doodle")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("bath"), py_str("bathroom"), py_str("curtain"), py_str("curtains"), py_str("blind"), py_str("blinds"), py_str("window"), py_str("money"), py_str("pension"), py_str("interest"), py_str("benefit"), py_str("salary"), py_str("diagram"), py_str("plan"), py_str("will"), py_str("contract"), py_str("sign"), py_str("signature"), py_str("paint colour"), py_str("paint color"), py_str("pick a colour"), py_str("pick a color"), py_str("colour scheme"), py_str("color scheme"), py_str("wall"), py_str("walls"), py_str("emulsion"), py_str("gloss"), py_str("woodwork"), py_str("hall"), py_str("hallway"), py_str("front room"), py_str("bedroom"), py_str("living room")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("writing prompt"), py_str("writing prompts"), py_str("story prompt"), py_str("story prompts"), py_str("creative writing"), py_str("a prompt"), py_str("prompts"), py_str("something to write"), py_str("something to write about"), py_str("what should i write"), py_str("what can i write"), py_str("story idea"), py_str("story ideas"), py_str("writing idea"), py_str("writing ideas")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("prompt"), py_str("prompts"), py_str("story"), py_str("stories"), py_str("blog"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("class"), py_str("group"), py_str("workshop"), py_str("practice")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("essay"), py_str("essays"), py_str("assignment"), py_str("homework"), py_str("coursework"), py_str("thesis"), py_str("dissertation"), py_str("cv"), py_str("resume"), py_str("cover letter"), py_str("letter"), py_str("email"), py_str("complaint"), py_str("card"), py_str("birthday"), py_str("wedding"), py_str("speech"), py_str("will"), py_str("mortgage"), py_str("landlord"), py_str("school"), py_str("university"), py_str("application"), py_str("personal statement"), py_str("diary"), py_str("journal")]) }
    fn __data16() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("toast")); __args.push(py_tuple(vec![py_tuple(vec![py_str("toast"), py_str("toasts"), py_str("few words for"), py_str("few words to say"), py_str("few words about"), py_str("few words at"), py_str("few words on")]), op_add(&g_ASKED(), &py_tuple(vec![py_str("give me"), py_str("say"), py_str("write me")]))])); __args.push(py_func("_toast", |args: &[Py]| _toast(args))); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&g_HOW_TO(), &py_tuple(vec![py_str("give him"), py_str("give her"), py_str("give them"), py_str("gives him"), py_str("gives her"), py_str("him toast"), py_str("her toast"), py_str("my baby"), py_str("my son"), py_str("my daughter"), py_str("my toddler"), py_str("toast fingers")])), &g_ANIMAL_NOUNS()), &__data17())), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("raise-a-toast")); __args.push(py_tuple(vec![py_tuple(vec![py_str("raise a toast"), py_str("raise the toast"), py_str("raise a glass"), py_str("raise the glass"), py_str("propose a toast"), py_str("propose the toast"), py_str("give a toast"), py_str("say a few words")]), __data18()])); __args.push(py_func("_toast", |args: &[Py]| _toast(args))); __kws.extend(vec![("avoid".to_string(), __data19()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("debate-topic")); __args.push(py_tuple(vec![py_tuple(vec![py_str("debate"), py_str("debates"), py_str("debate topic"), py_str("debate topics"), py_str("discussion topic"), py_str("discussion topics"), py_str("argument topic")]), op_add(&g_ASKED(), &py_tuple(vec![py_str("topic"), py_str("topics"), py_str("debate"), py_str("what should we"), py_str("suggest")]))])); __args.push(_pool(&[g_DEBATE_TOPICS()])); __kws.extend(vec![("avoid".to_string(), op_add(&g_HOW_TO(), &__data20()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("riddle")); __args.push(py_tuple(vec![py_tuple(vec![py_str("riddle"), py_str("riddles"), py_str("brain teaser"), py_str("brainteasers"), py_str("brain teasers"), py_str("word puzzle"), py_str("lateral thinking puzzle"), py_str("entertain me"), py_str("amuse me")]), op_add(&g_ASKED(), &py_tuple(vec![py_str("entertain"), py_str("amuse")]))])); __args.push(_pool(&[g_RIDDLES()])); __kws.extend(vec![("avoid".to_string(), op_add(&g_SOLVE(), &g_ABOUT()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tongue-twister")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tongue twister"), py_str("tongue twisters"), py_str("twister"), py_str("twisters")]), op_add(&g_ASKED(), &py_tuple(vec![py_str("twister"), py_str("twisters"), py_str("tongue"), py_str("say"), py_str("hard")]))])); __args.push(_pool(&[g_TONGUE_TWISTERS()])); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_SOLVE(), &g_ABOUT()), &py_tuple(vec![py_str("why do"), py_str("why does"), py_str("practice"), py_str("practising"), py_str("practicing"), py_str("write"), py_str("writing"), py_str("made up"), py_str("meaning"), py_str("mean")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fact-about-a-thing")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_FACTS_ABOUT(), "values", &[])) { let __unpk941 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk941[0].clone(); let mut _facts: Py = __unpk941[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g__FACT_ASKS()])); __args.push(py_func("_a_fact_about", |args: &[Py]| _a_fact_about(args))); __kws.extend(vec![("avoid".to_string(), g__FACT_NOT()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_the_line_does_not_state_the_fact", |args: &[Py]| _the_line_does_not_state_the_fact(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fun-fact")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fun fact"), py_str("fun facts"), py_str("random fact"), py_str("random facts"), py_str("interesting fact"), py_str("interesting facts"), py_str("a fact"), py_str("true fact")]), op_add(&g_ASKED(), &py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("what is")]))])); __args.push(_pool(&[g_FUN_FACTS()])); __kws.extend(vec![("avoid".to_string(), op_add(&g_ABOUT(), &g_DOUBTED()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("something-to-hear")); __args.push(py_tuple(vec![g_SOMETHING_ASKED(), op_add(&g_ASKED(), &g_SOMETHING_ASKED())])); __args.push(py_func("_something_to_hear", |args: &[Py]| _something_to_hear(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g_ABOUT(), &g_DOUBTED())), ("guard".to_string(), py_func("_the_line_is_only_the_ask", |args: &[Py]| _the_line_is_only_the_ask(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quiz-question")); __args.push(py_tuple(vec![g_QUIZ_ASK(), g_QUIZ_ME()])); __args.push(_pool(&[g_QUESTIONS()])); __kws.extend(vec![("avoid".to_string(), g_QUIZ_NOT_A_QUIZ()), ("guard".to_string(), py_func("_the_person_is_the_one_quizzed", |args: &[Py]| _the_person_is_the_one_quizzed(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quiz-night")); __args.push(py_tuple(vec![__data21(), __data22()])); __args.push(py_str("Six rounds of ten questions is the shape people finish: a picture round, a music intro round, general knowledge, a round about your own street or family that nobody can revise for, a true-or-false round, and one joker round where a wrong answer costs a point instead of scoring nothing. Teams of four to six, questions short enough to read aloud twice, the hardest round in the middle rather than at the end, and the answers written down as you go.")); __kws.extend(vec![("avoid".to_string(), __data23())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name-a-thing")); __args.push(py_tuple(vec![g_THING_NAMED(), g_NAMING_ASKED()])); __args.push(py_func("_a_name_for_the_thing", |args: &[Py]| _a_name_for_the_thing(args))); __kws.extend(vec![("ask".to_string(), op_add(&g_THING_NAMED(), &__data24())), ("avoid".to_string(), __data25())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("group-game")); __args.push(py_tuple(vec![__data26(), op_add(&g_ASKED(), &__data27())])); __args.push(py_func("_the_game_the_line_names", |args: &[Py]| _the_game_the_line_names(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g_HOW_TO(), &__data28()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("drawing-prompt")); __args.push(py_tuple(vec![__data29(), op_add(&g_ASKED(), &__data30())])); __args.push(_pool(&[g_DRAWING_PROMPTS()])); __kws.extend(vec![("avoid".to_string(), op_add(&g_HOW_TO(), &__data31()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("writing-prompt")); __args.push(py_tuple(vec![__data32(), op_add(&g_ASKED(), &__data33())])); __args.push(_pool(&[g_WRITING_PROMPTS()])); __kws.extend(vec![("avoid".to_string(), op_add(&g_HOW_TO(), &__data34()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.fun.ENTRIES"); }
    s_ENTRIES(__data16());
}
pub fn _the_line_is_only_the_ask(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks to be told something and names nothing itself.\n\n    The reading's clean text, not the words the conversation added: a follow-up that leans on the\n    last line is not this row's, because what it is asking about is in the other line.\n    ");
    clean = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&clean.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ASK_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return Py::Bool(!py_truthy(&words.clone()));
    Py::None
}
pub fn _something_to_hear(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut earlier: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A fact or a riddle for a line that asked for something to hear, and nothing else.\n\n    \"entertain me\" and \"amuse me\" are the two that want something to join in with, so they get a\n    riddle; everything else in the row wants to be told something, so it gets a fact.\n\n    Nothing here is given to a chat where somebody has just died. The person may still want to be\n    taken out of themselves, but a fact about neutron stars read out two lines after \"my mum died\n    last week\" is the bot talking over them, and the turn belongs to whatever reads the grief.\n    ");
    if py_truthy(&pyrt::xrefs::xcall("compose.loss", "dying_in_the_chat", &[memory.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    earlier = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[earlier.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&earlier.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.loss", "reads_as_loss", &[said.clone()]) } }) {
    return Py::None;
    }
    }
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("entertain"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("amuse"))) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_RIDDLES(), memory.clone(), text.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[g_FUN_FACTS(), memory.clone(), text.clone()]);
    Py::None
}
pub fn _said_here(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("True when `word` is said in `clean` as a word, not inside a longer one.");
    return Py::Bool(!py_is(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    Py::None
}
pub fn _the_line_does_not_state_the_fact(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when the line counts something out: that number has its own row.");
    reading = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    clean = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COUNTED()) { let mut word: Py = __item.clone(); __out.push(_said_here(&[clean.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_fact_about(args: &[Py]) -> Py {
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
    let mut _subject: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut facts: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A fact about the subject the line names, or None when it names none this table holds.");
    reading = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    clean = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FACT_ASKS()) { let mut ask: Py = __item.clone(); __out.push(_said_here(&[clean.clone(), ask.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FACT_NOT()) { let mut word: Py = __item.clone(); __out.push(_said_here(&[clean.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_FACTS_ABOUT(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    _subject = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    words = __unpk3[0].clone();
    facts = __unpk3[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_said_here(&[clean.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return pyrt::xrefs::xcall("compose", "pick", &[facts.clone(), memory.clone(), clean.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_thing_being_named(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut one: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The thing of the ones above that the line names, longest phrase first.");
    text = op_add(&op_add(&py_str(" "), &call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    best = py_str("");
    let __iter1 = py_iter_or_raise(&g_THING_NAMED());
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&text.clone(), "replace", &[py_str("'s"), py_str("")]), &op_add(&op_add(&py_str(" "), &one.clone()), &py_str(" ")))) } }) {
    best = one.clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _the_game_the_line_names(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut marks: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut text: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The game the line names, or one of the pool when it names none.\n\n    \"play two truths and a lie\" is a request for that game, and a pool that only fires on the\n    shape of the ask answered it with the alphabet game.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    text = op_add(&op_add(&py_str(" "), &call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("two truths"), py_str("2 truths"), py_str("truths and a lie"), py_str("truth or lie")]), geti(&g_GROUP_GAMES(), &Py::Int(0))]), py_tuple(vec![py_tuple(vec![py_str("would you rather")]), geti(&g_GROUP_GAMES(), &Py::Int(1))]), py_tuple(vec![py_tuple(vec![py_str("twenty questions"), py_str("20 questions")]), geti(&g_GROUP_GAMES(), &Py::Int(2))]), py_tuple(vec![py_tuple(vec![py_str("alphabet")]), geti(&g_GROUP_GAMES(), &Py::Int(3))])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    marks = __unpk2[0].clone();
    line = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&marks.clone());
    for __item in __iter3 {
    mark = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &mark.clone()))) {
    return line.clone();
    }
    }
    }
    pool = g_GROUP_GAMES();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" car"), py_str(" journey"), py_str(" travel"), py_str(" flight"), py_str(" drive"), py_str(" road"), py_str(" trip"), py_str(" train"), py_str(" plane"), py_str(" airport")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    pool = slice_or_raise(&g_GROUP_GAMES(), &Py::None, &Py::Int(3), &Py::None);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), text.clone()]);
    Py::None
}
pub fn _a_name_for_the_thing(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut lead: Py = Py::None;
    let mut names: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Three names and the rule they follow, for the thing the line names.");
    understood = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    thing = _the_thing_being_named(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("boat"), py_str("boats"), py_str("yacht"), py_str("kayak"), py_str("canoe")]), &thing.clone()))) {
    let __unpk1 = py_unpack(&py_tuple(vec![py_str("a boat"), g_BOAT_NAMES()]), 2, None);
    lead = __unpk1[0].clone();
    names = __unpk1[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("band"), py_str("bands"), py_str("quiz team"), py_str("team")]), &thing.clone()))) {
    let __unpk2 = py_unpack(&py_tuple(vec![py_str("a band"), g_BAND_NAMES()]), 2, None);
    lead = __unpk2[0].clone();
    names = __unpk2[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_eq(&thing.clone(), &py_str("podcast")))) {
    let __unpk3 = py_unpack(&py_tuple(vec![py_str("a podcast"), g_PODCAST_NAMES()]), 2, None);
    lead = __unpk3[0].clone();
    names = __unpk3[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("pub"), py_str("cafe"), py_str("café"), py_str("shop"), py_str("business"), py_str("company"), py_str("startup")]), &thing.clone()))) {
    let __unpk4 = py_unpack(&py_tuple(vec![py_str("a business"), g_SHOP_NAMES()]), 2, None);
    lead = __unpk4[0].clone();
    names = __unpk4[1].clone();
    } else {
    let __unpk5 = py_unpack(&py_tuple(vec![py_str("it"), g_TECH_NAMES()]), 2, None);
    lead = __unpk5[0].clone();
    names = __unpk5[1].clone();
    }
    }
    }
    }
    return op_modulo(&py_str("Short, sayable and easy to spell beats clever: %s, %s or %s. Say it out loud in a sentence - 'this is %s' - before you commit, and check nobody selling the same thing already has it."), &py_tuple(vec![geti(&names.clone(), &Py::Int(0)), geti(&names.clone(), &Py::Int(1)), geti(&names.clone(), &Py::Int(2)), mcall(&geti(&names.clone(), &Py::Int(0)), "lower", &[])]));
    Py::None
}
pub fn _pool(args: &[Py]) -> Py {
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
    let mut choose: Py = Py::None;
    let _ = py_str("A reply that chooses from a pool, seeded with the line.\n\n    Without the line, every first turn of every conversation chooses the same item - the seed is\n    the turn number - so \"give me a riddle\" and \"tell me a fun fact\" would open alike and a chat\n    of one riddle would always be the same riddle. With it, the choice moves with what was asked\n    and stays the same for the same words, which is what a comparison run needs.\n    ");
pub fn __nested_choose_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut choose: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut options: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut reading: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reading").map(|(_, v)| v.clone())).unwrap_or_else(|| Py::None);
    let mut text: Py = Py::None;
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), text.clone()]);
    Py::None
}
    choose = {{ let choose = choose.clone(); let options = options.clone(); py_func("choose", move |args: &[Py]| { let mut full = vec![choose.clone(), options.clone()]; full.extend_from_slice(args); __nested_choose_0(&full) }) }};
    return choose.clone();
    Py::None
}
pub fn _toast(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut _u: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut occasion_words: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The toast for the occasion the line names, or a plain one when it names none.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    joined = mcall(&py_str(" "), "join", &[words.clone()]);
    let __iter1 = py_iter_or_raise(&g_TOAST_OCCASIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk2[0].clone();
    occasion_words = __unpk2[1].clone();
    pool = __unpk2[2].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&occasion_words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&joined.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), joined.clone()]);
    }
    }
    return pyrt::xrefs::xcall("compose", "pick", &[g_PLAIN_TOASTS(), memory.clone(), joined.clone()]);
    Py::None
}
pub fn _the_person_is_the_one_quizzed(args: &[Py]) -> Py {
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
    let _ = py_str("The row answers only when the line asks for a question to answer.\n\n    The key words alone find \"a question\" and \"me\" in a line about somebody else: \"she gave me a\n    question to answer\", \"he asked me a question\", \"my daughter asked me a question\" all carry\n    them, and the person had said nothing about wanting to be quizzed. The reading says which\n    way round it is (`wants_a_quiz`: the hand-over verb opens the line, or stands behind nothing\n    but a modal or a courtesy), so a third person handing a question over gets its own answer\n    back and the ask still gets a question.\n    ");
    return call_builtin("bool", &[mcall(&understood.clone(), "get", &[py_str("wants_a_quiz")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.fun".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT" => Some(g_ABOUT()),
            "ANIMAL_NOUNS" => Some(g_ANIMAL_NOUNS()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKED" => Some(g_ASKED()),
            "BAND_NAMES" => Some(g_BAND_NAMES()),
            "BIRTHDAY_TOASTS" => Some(g_BIRTHDAY_TOASTS()),
            "BOAT_NAMES" => Some(g_BOAT_NAMES()),
            "DEBATE_TOPICS" => Some(g_DEBATE_TOPICS()),
            "DOUBTED" => Some(g_DOUBTED()),
            "DRAWING_PROMPTS" => Some(g_DRAWING_PROMPTS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FACTS_ABOUT" => Some(g_FACTS_ABOUT()),
            "FUN_FACTS" => Some(g_FUN_FACTS()),
            "GROUP_GAMES" => Some(g_GROUP_GAMES()),
            "HOW_TO" => Some(g_HOW_TO()),
            "LEAVING_TOASTS" => Some(g_LEAVING_TOASTS()),
            "NAMING_ASKED" => Some(g_NAMING_ASKED()),
            "PLAIN_TOASTS" => Some(g_PLAIN_TOASTS()),
            "PODCAST_NAMES" => Some(g_PODCAST_NAMES()),
            "QUESTIONS" => Some(g_QUESTIONS()),
            "QUIZ_ASK" => Some(g_QUIZ_ASK()),
            "QUIZ_ME" => Some(g_QUIZ_ME()),
            "QUIZ_NOT_A_QUIZ" => Some(g_QUIZ_NOT_A_QUIZ()),
            "RETIREMENT_TOASTS" => Some(g_RETIREMENT_TOASTS()),
            "RIDDLES" => Some(g_RIDDLES()),
            "SHOP_NAMES" => Some(g_SHOP_NAMES()),
            "SOLVE" => Some(g_SOLVE()),
            "SOMETHING_ASKED" => Some(g_SOMETHING_ASKED()),
            "TECH_NAMES" => Some(g_TECH_NAMES()),
            "THING_NAMED" => Some(g_THING_NAMED()),
            "TOAST_OCCASIONS" => Some(g_TOAST_OCCASIONS()),
            "TONGUE_TWISTERS" => Some(g_TONGUE_TWISTERS()),
            "WEDDING_TOASTS" => Some(g_WEDDING_TOASTS()),
            "WRITING_PROMPTS" => Some(g_WRITING_PROMPTS()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_COUNTED" => Some(g__COUNTED()),
            "_FACT_ASKS" => Some(g__FACT_ASKS()),
            "_FACT_NOT" => Some(g__FACT_NOT()),
            "_a_fact_about" => Some(py_func("_a_fact_about", |args: &[Py]| _a_fact_about(args))),
            "_a_name_for_the_thing" => Some(py_func("_a_name_for_the_thing", |args: &[Py]| _a_name_for_the_thing(args))),
            "_pool" => Some(py_func("_pool", |args: &[Py]| _pool(args))),
            "_said_here" => Some(py_func("_said_here", |args: &[Py]| _said_here(args))),
            "_something_to_hear" => Some(py_func("_something_to_hear", |args: &[Py]| _something_to_hear(args))),
            "_the_game_the_line_names" => Some(py_func("_the_game_the_line_names", |args: &[Py]| _the_game_the_line_names(args))),
            "_the_line_does_not_state_the_fact" => Some(py_func("_the_line_does_not_state_the_fact", |args: &[Py]| _the_line_does_not_state_the_fact(args))),
            "_the_line_is_only_the_ask" => Some(py_func("_the_line_is_only_the_ask", |args: &[Py]| _the_line_is_only_the_ask(args))),
            "_the_person_is_the_one_quizzed" => Some(py_func("_the_person_is_the_one_quizzed", |args: &[Py]| _the_person_is_the_one_quizzed(args))),
            "_the_thing_being_named" => Some(py_func("_the_thing_being_named", |args: &[Py]| _the_thing_being_named(args))),
            "_toast" => Some(py_func("_toast", |args: &[Py]| _toast(args))),
            "entry" => Some(g_entry()),
            "pick" => Some(g_pick()),
            "loss" => Some(pyrt::xrefs::xmodule("compose.loss")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT" => s_ABOUT(v),
            "ANIMAL_NOUNS" => s_ANIMAL_NOUNS(v),
            "ANSWERS" => s_ANSWERS(v),
            "ASKED" => s_ASKED(v),
            "BAND_NAMES" => s_BAND_NAMES(v),
            "BIRTHDAY_TOASTS" => s_BIRTHDAY_TOASTS(v),
            "BOAT_NAMES" => s_BOAT_NAMES(v),
            "DEBATE_TOPICS" => s_DEBATE_TOPICS(v),
            "DOUBTED" => s_DOUBTED(v),
            "DRAWING_PROMPTS" => s_DRAWING_PROMPTS(v),
            "ENTRIES" => s_ENTRIES(v),
            "FACTS_ABOUT" => s_FACTS_ABOUT(v),
            "FUN_FACTS" => s_FUN_FACTS(v),
            "GROUP_GAMES" => s_GROUP_GAMES(v),
            "HOW_TO" => s_HOW_TO(v),
            "LEAVING_TOASTS" => s_LEAVING_TOASTS(v),
            "NAMING_ASKED" => s_NAMING_ASKED(v),
            "PLAIN_TOASTS" => s_PLAIN_TOASTS(v),
            "PODCAST_NAMES" => s_PODCAST_NAMES(v),
            "QUESTIONS" => s_QUESTIONS(v),
            "QUIZ_ASK" => s_QUIZ_ASK(v),
            "QUIZ_ME" => s_QUIZ_ME(v),
            "QUIZ_NOT_A_QUIZ" => s_QUIZ_NOT_A_QUIZ(v),
            "RETIREMENT_TOASTS" => s_RETIREMENT_TOASTS(v),
            "RIDDLES" => s_RIDDLES(v),
            "SHOP_NAMES" => s_SHOP_NAMES(v),
            "SOLVE" => s_SOLVE(v),
            "SOMETHING_ASKED" => s_SOMETHING_ASKED(v),
            "TECH_NAMES" => s_TECH_NAMES(v),
            "THING_NAMED" => s_THING_NAMED(v),
            "TOAST_OCCASIONS" => s_TOAST_OCCASIONS(v),
            "TONGUE_TWISTERS" => s_TONGUE_TWISTERS(v),
            "WEDDING_TOASTS" => s_WEDDING_TOASTS(v),
            "WRITING_PROMPTS" => s_WRITING_PROMPTS(v),
            "_ASK_WORDS" => s__ASK_WORDS(v),
            "_COUNTED" => s__COUNTED(v),
            "_FACT_ASKS" => s__FACT_ASKS(v),
            "_FACT_NOT" => s__FACT_NOT(v),
            "entry" => s_entry(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.fun", module_obj);
    pyrt::xrefs::reg_fn("knowledge.fun", "_the_line_is_only_the_ask", py_func("_the_line_is_only_the_ask", |args: &[Py]| _the_line_is_only_the_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_something_to_hear", py_func("_something_to_hear", |args: &[Py]| _something_to_hear(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_said_here", py_func("_said_here", |args: &[Py]| _said_here(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_the_line_does_not_state_the_fact", py_func("_the_line_does_not_state_the_fact", |args: &[Py]| _the_line_does_not_state_the_fact(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_a_fact_about", py_func("_a_fact_about", |args: &[Py]| _a_fact_about(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_the_thing_being_named", py_func("_the_thing_being_named", |args: &[Py]| _the_thing_being_named(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_the_game_the_line_names", py_func("_the_game_the_line_names", |args: &[Py]| _the_game_the_line_names(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_a_name_for_the_thing", py_func("_a_name_for_the_thing", |args: &[Py]| _a_name_for_the_thing(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_pool", py_func("_pool", |args: &[Py]| _pool(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_toast", py_func("_toast", |args: &[Py]| _toast(args)));
    pyrt::xrefs::reg_fn("knowledge.fun", "_the_person_is_the_one_quizzed", py_func("_the_person_is_the_one_quizzed", |args: &[Py]| _the_person_is_the_one_quizzed(args)));
    pyrt::xrefs::reg_access("knowledge.fun", "ABOUT", g_ABOUT, s_ABOUT);
    pyrt::xrefs::reg_access("knowledge.fun", "ANIMAL_NOUNS", g_ANIMAL_NOUNS, s_ANIMAL_NOUNS);
    pyrt::xrefs::reg_access("knowledge.fun", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.fun", "ASKED", g_ASKED, s_ASKED);
    pyrt::xrefs::reg_access("knowledge.fun", "BAND_NAMES", g_BAND_NAMES, s_BAND_NAMES);
    pyrt::xrefs::reg_access("knowledge.fun", "BIRTHDAY_TOASTS", g_BIRTHDAY_TOASTS, s_BIRTHDAY_TOASTS);
    pyrt::xrefs::reg_access("knowledge.fun", "BOAT_NAMES", g_BOAT_NAMES, s_BOAT_NAMES);
    pyrt::xrefs::reg_access("knowledge.fun", "DEBATE_TOPICS", g_DEBATE_TOPICS, s_DEBATE_TOPICS);
    pyrt::xrefs::reg_access("knowledge.fun", "DOUBTED", g_DOUBTED, s_DOUBTED);
    pyrt::xrefs::reg_access("knowledge.fun", "DRAWING_PROMPTS", g_DRAWING_PROMPTS, s_DRAWING_PROMPTS);
    pyrt::xrefs::reg_access("knowledge.fun", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.fun", "FACTS_ABOUT", g_FACTS_ABOUT, s_FACTS_ABOUT);
    pyrt::xrefs::reg_access("knowledge.fun", "FUN_FACTS", g_FUN_FACTS, s_FUN_FACTS);
    pyrt::xrefs::reg_access("knowledge.fun", "GROUP_GAMES", g_GROUP_GAMES, s_GROUP_GAMES);
    pyrt::xrefs::reg_access("knowledge.fun", "HOW_TO", g_HOW_TO, s_HOW_TO);
    pyrt::xrefs::reg_access("knowledge.fun", "LEAVING_TOASTS", g_LEAVING_TOASTS, s_LEAVING_TOASTS);
    pyrt::xrefs::reg_access("knowledge.fun", "NAMING_ASKED", g_NAMING_ASKED, s_NAMING_ASKED);
    pyrt::xrefs::reg_access("knowledge.fun", "PLAIN_TOASTS", g_PLAIN_TOASTS, s_PLAIN_TOASTS);
    pyrt::xrefs::reg_access("knowledge.fun", "PODCAST_NAMES", g_PODCAST_NAMES, s_PODCAST_NAMES);
    pyrt::xrefs::reg_access("knowledge.fun", "QUESTIONS", g_QUESTIONS, s_QUESTIONS);
    pyrt::xrefs::reg_access("knowledge.fun", "QUIZ_ASK", g_QUIZ_ASK, s_QUIZ_ASK);
    pyrt::xrefs::reg_access("knowledge.fun", "QUIZ_ME", g_QUIZ_ME, s_QUIZ_ME);
    pyrt::xrefs::reg_access("knowledge.fun", "QUIZ_NOT_A_QUIZ", g_QUIZ_NOT_A_QUIZ, s_QUIZ_NOT_A_QUIZ);
    pyrt::xrefs::reg_access("knowledge.fun", "RETIREMENT_TOASTS", g_RETIREMENT_TOASTS, s_RETIREMENT_TOASTS);
    pyrt::xrefs::reg_access("knowledge.fun", "RIDDLES", g_RIDDLES, s_RIDDLES);
    pyrt::xrefs::reg_access("knowledge.fun", "SHOP_NAMES", g_SHOP_NAMES, s_SHOP_NAMES);
    pyrt::xrefs::reg_access("knowledge.fun", "SOLVE", g_SOLVE, s_SOLVE);
    pyrt::xrefs::reg_access("knowledge.fun", "SOMETHING_ASKED", g_SOMETHING_ASKED, s_SOMETHING_ASKED);
    pyrt::xrefs::reg_access("knowledge.fun", "TECH_NAMES", g_TECH_NAMES, s_TECH_NAMES);
    pyrt::xrefs::reg_access("knowledge.fun", "THING_NAMED", g_THING_NAMED, s_THING_NAMED);
    pyrt::xrefs::reg_access("knowledge.fun", "TOAST_OCCASIONS", g_TOAST_OCCASIONS, s_TOAST_OCCASIONS);
    pyrt::xrefs::reg_access("knowledge.fun", "TONGUE_TWISTERS", g_TONGUE_TWISTERS, s_TONGUE_TWISTERS);
    pyrt::xrefs::reg_access("knowledge.fun", "WEDDING_TOASTS", g_WEDDING_TOASTS, s_WEDDING_TOASTS);
    pyrt::xrefs::reg_access("knowledge.fun", "WRITING_PROMPTS", g_WRITING_PROMPTS, s_WRITING_PROMPTS);
    pyrt::xrefs::reg_access("knowledge.fun", "_ASK_WORDS", g__ASK_WORDS, s__ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.fun", "_COUNTED", g__COUNTED, s__COUNTED);
    pyrt::xrefs::reg_access("knowledge.fun", "_FACT_ASKS", g__FACT_ASKS, s__FACT_ASKS);
    pyrt::xrefs::reg_access("knowledge.fun", "_FACT_NOT", g__FACT_NOT, s__FACT_NOT);
    pyrt::xrefs::reg_access("knowledge.fun", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.fun", "pick", g_pick, s_pick);
}
