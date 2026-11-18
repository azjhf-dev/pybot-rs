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


thread_local! { static G_A_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DISLIKE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DISLIKE_MANNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DISLIKE_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_MANNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THE_BOT_DISLIKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THE_BOT_LIKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_INFINITIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLAUSE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LINKING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MANNER_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON_DESCRIBED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SECOND_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STANCE_NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BY_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEMONSTRATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISLIKE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DOUBLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IRREGULAR_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIKE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MANNER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PROPER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAIL_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_BOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_STANCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.taste: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_DISLIKE() -> Py { ensure_init(); G_A_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DISLIKE(v: Py) { ensure_init(); G_A_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DISLIKE_DOING() -> Py { ensure_init(); G_A_DISLIKE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DISLIKE_DOING(v: Py) { ensure_init(); G_A_DISLIKE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DISLIKE_MANNER() -> Py { ensure_init(); G_A_DISLIKE_MANNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DISLIKE_MANNER(v: Py) { ensure_init(); G_A_DISLIKE_MANNER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DISLIKE_PET() -> Py { ensure_init(); G_A_DISLIKE_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DISLIKE_PET(v: Py) { ensure_init(); G_A_DISLIKE_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DREAD() -> Py { ensure_init(); G_A_DREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DREAD(v: Py) { ensure_init(); G_A_DREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE() -> Py { ensure_init(); G_A_LIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE(v: Py) { ensure_init(); G_A_LIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_DOING() -> Py { ensure_init(); G_A_LIKE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_DOING(v: Py) { ensure_init(); G_A_LIKE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_MANNER() -> Py { ensure_init(); G_A_LIKE_MANNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_MANNER(v: Py) { ensure_init(); G_A_LIKE_MANNER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_OWN() -> Py { ensure_init(); G_A_LIKE_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_OWN(v: Py) { ensure_init(); G_A_LIKE_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_PET() -> Py { ensure_init(); G_A_LIKE_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_PET(v: Py) { ensure_init(); G_A_LIKE_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THE_BOT_DISLIKED() -> Py { ensure_init(); G_A_THE_BOT_DISLIKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THE_BOT_DISLIKED(v: Py) { ensure_init(); G_A_THE_BOT_DISLIKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THE_BOT_LIKED() -> Py { ensure_init(); G_A_THE_BOT_LIKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THE_BOT_LIKED(v: Py) { ensure_init(); G_A_THE_BOT_LIKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT() -> Py { ensure_init(); G_A_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT(v: Py) { ensure_init(); G_A_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_ME() -> Py { ensure_init(); G__ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_ME(v: Py) { ensure_init(); G__ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_WORK() -> Py { ensure_init(); G__ABOUT_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_WORK(v: Py) { ensure_init(); G__ABOUT_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ANIMAL() -> Py { ensure_init(); G__AN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ANIMAL(v: Py) { ensure_init(); G__AN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_INFINITIVE() -> Py { ensure_init(); G__AN_INFINITIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_INFINITIVE(v: Py) { ensure_init(); G__AN_INFINITIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_ASK() -> Py { ensure_init(); G__A_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_ASK(v: Py) { ensure_init(); G__A_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLAUSE_WORD() -> Py { ensure_init(); G__A_CLAUSE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_WORD(v: Py) { ensure_init(); G__A_CLAUSE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEED() -> Py { ensure_init(); G__A_DEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEED(v: Py) { ensure_init(); G__A_DEED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DREAD() -> Py { ensure_init(); G__A_DREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DREAD(v: Py) { ensure_init(); G__A_DREAD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LINKING_WORD() -> Py { ensure_init(); G__A_LINKING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LINKING_WORD(v: Py) { ensure_init(); G__A_LINKING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MANNER_PHRASE() -> Py { ensure_init(); G__A_MANNER_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MANNER_PHRASE(v: Py) { ensure_init(); G__A_MANNER_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON() -> Py { ensure_init(); G__A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON(v: Py) { ensure_init(); G__A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON_DESCRIBED() -> Py { ensure_init(); G__A_PERSON_DESCRIBED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON_DESCRIBED(v: Py) { ensure_init(); G__A_PERSON_DESCRIBED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PRONOUN() -> Py { ensure_init(); G__A_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PRONOUN(v: Py) { ensure_init(); G__A_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SECOND_CLAUSE() -> Py { ensure_init(); G__A_SECOND_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SECOND_CLAUSE(v: Py) { ensure_init(); G__A_SECOND_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SOMEONE() -> Py { ensure_init(); G__A_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SOMEONE(v: Py) { ensure_init(); G__A_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STANCE() -> Py { ensure_init(); G__A_STANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STANCE(v: Py) { ensure_init(); G__A_STANCE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STANCE_NO_SUBJECT() -> Py { ensure_init(); G__A_STANCE_NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STANCE_NO_SUBJECT(v: Py) { ensure_init(); G__A_STANCE_NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SYMPTOM() -> Py { ensure_init(); G__A_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SYMPTOM(v: Py) { ensure_init(); G__A_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WAIT() -> Py { ensure_init(); G__A_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WAIT(v: Py) { ensure_init(); G__A_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g__BY_KIND() -> Py { ensure_init(); G__BY_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BY_KIND(v: Py) { ensure_init(); G__BY_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__DEMONSTRATIVE() -> Py { ensure_init(); G__DEMONSTRATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEMONSTRATIVE(v: Py) { ensure_init(); G__DEMONSTRATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__DISLIKE_VERBS() -> Py { ensure_init(); G__DISLIKE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISLIKE_VERBS(v: Py) { ensure_init(); G__DISLIKE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__DOUBLING() -> Py { ensure_init(); G__DOUBLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DOUBLING(v: Py) { ensure_init(); G__DOUBLING.with(|c| *c.borrow_mut() = v); }
pub fn g__IRREGULAR_DOING() -> Py { ensure_init(); G__IRREGULAR_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IRREGULAR_DOING(v: Py) { ensure_init(); G__IRREGULAR_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD_SUBJECT() -> Py { ensure_init(); G__LEAD_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD_SUBJECT(v: Py) { ensure_init(); G__LEAD_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__LIKE_VERBS() -> Py { ensure_init(); G__LIKE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIKE_VERBS(v: Py) { ensure_init(); G__LIKE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__MANNER_WORDS() -> Py { ensure_init(); G__MANNER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MANNER_WORDS(v: Py) { ensure_init(); G__MANNER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DOING() -> Py { ensure_init(); G__NOT_A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DOING(v: Py) { ensure_init(); G__NOT_A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING() -> Py { ensure_init(); G__NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); G__NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__PROPER() -> Py { ensure_init(); G__PROPER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PROPER(v: Py) { ensure_init(); G__PROPER.with(|c| *c.borrow_mut() = v); }
pub fn g__TAIL_FILLER() -> Py { ensure_init(); G__TAIL_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAIL_FILLER(v: Py) { ensure_init(); G__TAIL_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__THEIR_OWN() -> Py { ensure_init(); G__THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEIR_OWN(v: Py) { ensure_init(); G__THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_BOTS() -> Py { ensure_init(); G__THE_BOTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_BOTS(v: Py) { ensure_init(); G__THE_BOTS.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_STANCES() -> Py { ensure_init(); G__TWO_STANCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_STANCES(v: Py) { ensure_init(); G__TWO_STANCES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A stated like or dislike of a thing is an opener, not a fact to file.\n\nEvery one of these came back as the person's own words handed to them:\n\n    i hate mondays        -> \"Noted: you hate mondays.\"\n    i hate cooking        -> \"Noted: you hate cooking.\"\n    i love the rain       -> \"Noted: you love the rain.\"\n    i hate the dark       -> \"Noted: you hate the dark.\"\n    i dont like christmas -> \"Noted: you don't like christmas.\"\n\nThe same line said with \"cant stand\" or \"dreading\" reached the listening pool instead\n(\"Thanks for telling me - I'm here if you want to dig into any of that\"), and \"i cant wait for\nfriday\" reached the shrug (\"Got it. Is there anything you'd like a hand with?\").\n\nNone of those is a reply. A like or a dislike is the easiest thing in the world to answer: a\nfriend takes the side the person took, and asks one thing about the thing. That is the whole\nreading here. When the line names a thing the reply uses it; when it does not, the reply is\nshort and warm and asks the same question without it.\n\nWhat it refuses, because other readings own those turns:\n\n  - a person, or what a person does - \"i hate my sister\", \"i hate the way he talks to me\". A\n    dislike of a person is a relationship, not a taste, and the ones worth answering are the\n    hard ones;\n  - the person themselves - \"i hate myself\", \"i hate my life\". That is not a taste and it is\n    not this module's to name;\n  - a symptom - \"i hate feeling like this\", \"i hate being ill\" - which the mood and health\n    readings answer;\n  - a question (\"why do i hate mondays\"), which asks for a reason rather than stating a taste.\n\nTwo things the reading has to get right, because the frame carries the words:\n\n  - the way they have it is not part of the thing. \"i like my coffee strong\" is a taste in\n    coffee, and \"Your coffee strong - I can see why\" is not English. The manner comes off the\n    thing and back into the reply as its own word (\"Strong, then. What is it about your coffee\n    that you like?\"). A possessive is not enough on its own: \"my new bike\" and \"my little\n    garden\" are the thing itself, so only a last word that names a way of being (\"strong\",\n    \"weak\", \"runny\", \"well done\") comes off, and never across a linking word;\n  - a taste said about the bot is a compliment. \"i like your hair\" is not the person telling\n    you about their own hair - the reply turns \"my\" round into \"your\", so a \"your\" already in\n    the line was said about the bot, and asking what they like about their own hair is the\n    wrong friend. It is answered as the compliment it is.\n\nIt runs inside the greeting module, so it only ever speaks for lines nothing else wanted\n(README.md, \"Interface\"): a module with its own answer still takes the turn.\n");
    s__A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|your)\\s+(?:mum|mom|mam|dad|father|mother|wife|husband|partner|boyfriend|girlfriend|son|daughter|kid|kids|child|children|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|friend|friends|mate|mates|boss|landlord|neighbours?|neighbors?|colleague|colleagues|coworker|in-laws?|family|stepson|stepdaughter|ex)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:her|him|them|his|hers|theirs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON_DESCRIBED(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the|this|that|some)\\s+(?:girl|boy|man|woman|bloke|guy|lad|lady|fella|person|someone|somebody|child)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ABOUT_WORK(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:job|work|shift|shifts|career|line\\s+of\\s+work)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ABOUT_ME(stdlib_call("re", "compile", &[py_str("\\b(?:myself|my\\s+life|my\\s+body|my\\s+own\\s+body|being\\s+me|my\\s+(?:arms?|legs?|thighs?|tummy|belly|waist|hips?|shape|weight|looks|face|nose|teeth|smile|skin|hair|neck|chin|chest|shoulders?|hands?|feet|knees?|reflection)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SYMPTOM(stdlib_call("re", "compile", &[py_str("\\b(?:feeling\\s+like\\s+this|feeling\\s+this\\s+way|being\\s+ill|being\\s+sick|the\\s+way\\s+i\\s+feel|how\\s+i\\s+feel)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|how|why|when|where|who|instead|should|could|would|recommend|suggest|else|any\\s+ideas?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DEED(stdlib_call("re", "compile", &[py_str("^(?:when|that|it\\s+when|the\\s+way|how)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SECOND_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:because|but|although|though|and\\s+i|and\\s+then|so\\s+i)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAD_SUBJECT(py_str("(?P<subj>i|im|i'm)\\s+(?:(?:really|just|absolutely|totally|honestly|actually|properly|still|also|kind\\s+of|sort\\s+of)\\s+)?"));
    s__LEAD(py_str("(?:i|im|i'm)\\s+(?:really\\s+|just\\s+|absolutely\\s+|honestly\\s+|actually\\s+|totally\\s+|seriously\\s+|properly\\s+|kind\\s+of\\s+|sort\\s+of\\s+)?"));
    s__A_STANCE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^"), &g__LEAD_SUBJECT()), &py_str("(?P<verb>hate|hating|love|loving|like|liking|adore|adoring|dislike|prefer|can'?t\\s+stand|cannot\\s+stand|can\\s+not\\s+stand|couldn'?t\\s+stand|(?:am\\s+)?not\\s+a\\s+(?:big\\s+)?fan\\s+of|don'?t\\s+(?:really\\s+)?like|do\\s+not\\s+(?:really\\s+)?like|never\\s+liked)\\s+(?P<thing>.+)$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_STANCE_NO_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?P<verb>not\\s+(?:2|too|that|very|terribly|overly|really|a\\s+bit|a\\s+lot|much\\s+of\\s+a|a\\s+big)?\\s*(?:keen|fussed|bothered)\\s+(?:on|about|with)|never\\s+(?:been\\s+)?(?:that|too|very|really|overly)\\s+keen\\s+(?:on|about|with)|not\\s+(?:really\\s+|usually\\s+)?(?:much\\s+of\\s+)?(?:a\\s+)?(?:(?:big|huge|massive|great|the\\s+biggest)\\s+)?fan\\s+of|never\\s+(?:been\\s+)?(?:really\\s+|actually\\s+)?(?:a\\s+)?(?:big\\s+|huge\\s+)?fan\\s+of|never\\s+(?:really\\s+|actually\\s+)?liked|can'?t\\s+stand|cannot\\s+stand|can\\s+not\\s+stand|couldn'?t\\s+stand|can'?t\\s+stick|do\\s+not\\s+(?:really\\s+)?like|don'?t\\s+(?:really\\s+)?like)\\s+(?P<thing>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DREAD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^"), &g__LEAD()), &py_str("(?:am\\s+)?(?:dreading|dread)\\s+(?P<thing>.+)$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_WAIT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^"), &g__LEAD()), &py_str("(?:can'?t\\s+wait\\s+for|cannot\\s+wait\\s+for|counting\\s+down\\s+to)\\s+(?P<thing>.+)$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TWO_STANCES(stdlib_call("re", "compile", &[py_str("\\b(?:but|and|though|although)\\s+(?:i|im|i'm)\\s+(?:really\\s+|just\\s+|also\\s+)?(?:hate|love|like|prefer|adore|can'?t\\s+stand|don'?t\\s+like|dont\\s+like|do\\s+not\\s+like)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("hate"), py_str("hating"), py_str("dislike"), py_str("can't stand"), py_str("cant stand"), py_str("cannot stand"), py_str("can not stand"), py_str("couldn't stand"), py_str("couldnt stand"), py_str("not a fan of"), py_str("am not a fan of"), py_str("not a big fan of"), py_str("am not a big fan of"), py_str("don't like"), py_str("dont like"), py_str("do not like"), py_str("don't really like"), py_str("dont really like"), py_str("do not really like"), py_str("never liked")]) }
    s__DISLIKE_VERBS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s__LIKE_VERBS(call_builtin("frozenset", &[py_tuple(vec![py_str("love"), py_str("loving"), py_str("like"), py_str("liking"), py_str("adore"), py_str("adoring"), py_str("prefer")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DISLIKE(py_tuple(vec![py_str("{thing} - I'm with you on that. What is the worst bit of it?"), py_str("{thing} would get to anyone. What is it that gets you about it?"), py_str("{thing}, yeah. That one gets a lot of people. What is it about it?"), py_str("You are not on your own there. What is it about {thing} that does your head in?"), py_str("Fair enough. What is it about {thing}?")]));
    s_A_LIKE(py_tuple(vec![py_str("{thing} - good taste. What is it you like about that?"), py_str("Ah, {thing}. What got you into that?"), py_str("{thing}, nice one. What is the draw for you?"), py_str("Good choice. What is it about {thing} that you love?"), py_str("{thing} - that is a good thing to like. What do you like most about it?")]));
    s_A_LIKE_OWN(py_tuple(vec![py_str("{thing} - I can see why. What do you like most about it?"), py_str("{thing}, that is a good thing to have. What is the best of it?"), py_str("{thing} - fair enough. What is it about it that you like?")]));
    s_A_LIKE_PET(py_tuple(vec![py_str("{thing} - I can see why. What do they get up to?"), py_str("{thing}, that is a good thing to have. How long have you had them?"), py_str("{thing} - fair enough. What is it about them that you love?"), py_str("{thing}, lovely. What is the best thing about them?")]));
    s_A_LIKE_DOING(py_tuple(vec![py_str("{thing} - good one. How often do you get to do it?"), py_str("{thing}, nice. What got you into it?"), py_str("{thing} - I can see why. What is the best part?")]));
    s_A_DISLIKE_DOING(py_tuple(vec![py_str("{thing} - I'm with you on that. What is the worst bit?"), py_str("{thing}, fair enough. What do you do instead?"), py_str("{thing} - honestly, same. How do you get out of it?")]));
    s_A_DISLIKE_PET(py_tuple(vec![py_str("{thing} - fair enough. What is it about them?"), py_str("{thing}, yeah. What do they do that gets you?")]));
    s_A_DREAD(py_tuple(vec![py_str("{thing} - I do not blame you. What is the part you are least looking forward to?"), py_str("Fair enough - nobody looks forward to {thing}. What is the worst of it going to be?"), py_str("{thing}, yeah. What would make it easier?")]));
    s_A_LIKE_MANNER(py_tuple(vec![py_str("{manner}, then. What is it about {thing} that you like?"), py_str("{thing}, {manner} - that is the only way. What is the best of it?"), py_str("Good - {manner} it is. What do you like most about it?"), py_str("{thing}, {manner} - that is the right answer. What got you that way?")]));
    s_A_DISLIKE_MANNER(py_tuple(vec![py_str("{manner} - no wonder. What is it about {thing} that gets you?"), py_str("{thing}, {manner} - I am with you. What is the worst bit of it?"), py_str("Fair enough - nobody wants {thing} {manner}. What do you have instead?")]));
    s_A_THE_BOT_LIKED(py_tuple(vec![py_str("That is kind - thank you. What is it about it you like?"), py_str("Ha - thanks. What do you like most about it?"), py_str("Thank you - nice of you to say. What is it that works for you?")]));
    s_A_THE_BOT_DISLIKED(py_tuple(vec![py_str("Fair enough - point taken. What would land better?"), py_str("Sorry about that. How would you put it instead?")]));
    s_A_WAIT(py_tuple(vec![py_str("{thing} cannot come soon enough. What have you got planned?"), py_str("Not long now. What are you looking forward to most about {thing}?"), py_str("{thing} - good. Anything nice lined up?")]));
    fn __data2() -> Py { py_dict(vec![(py_str("dislike"), g_A_DISLIKE()), (py_str("like"), g_A_LIKE()), (py_str("like_own"), g_A_LIKE_OWN()), (py_str("like_pet"), g_A_LIKE_PET()), (py_str("dislike_pet"), g_A_DISLIKE_PET()), (py_str("like_doing"), g_A_LIKE_DOING()), (py_str("dislike_doing"), g_A_DISLIKE_DOING()), (py_str("like_manner"), g_A_LIKE_MANNER()), (py_str("dislike_manner"), g_A_DISLIKE_MANNER()), (py_str("the_bot_liked"), g_A_THE_BOT_LIKED()), (py_str("the_bot_disliked"), g_A_THE_BOT_DISLIKED()), (py_str("dread"), g_A_DREAD()), (py_str("wait"), g_A_WAIT())]) }
    s__BY_KIND(__data2());
    s__THE_BOTS(stdlib_call("re", "compile", &[py_str("^(?:your|you|ur|yours|yourself)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("strong"), py_str("weak"), py_str("runny"), py_str("firm"), py_str("hard"), py_str("soft"), py_str("milky"), py_str("creamy"), py_str("sweet"), py_str("hot"), py_str("cold"), py_str("warm"), py_str("tepid"), py_str("flat"), py_str("fizzy"), py_str("sparkling"), py_str("thick"), py_str("thin"), py_str("crispy"), py_str("crunchy"), py_str("smooth"), py_str("black"), py_str("plain"), py_str("spicy"), py_str("mild"), py_str("salty"), py_str("sour"), py_str("bitter"), py_str("dry"), py_str("ripe"), py_str("rare"), py_str("tender"), py_str("tough"), py_str("loud"), py_str("quiet"), py_str("bright"), py_str("pale"), py_str("bold"), py_str("baggy"), py_str("tight"), py_str("loose"), py_str("short"), py_str("long"), py_str("heavy"), py_str("light"), py_str("soggy"), py_str("mushy"), py_str("lumpy"), py_str("gritty"), py_str("chewy"), py_str("rubbery"), py_str("juicy"), py_str("burnt"), py_str("raw"), py_str("fluffy"), py_str("dense"), py_str("chunky"), py_str("frothy"), py_str("sharp"), py_str("stodgy"), py_str("greasy")]) }
    s__MANNER_WORDS(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MANNER_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:well done|well-done|medium rare|over easy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("of"), py_str("in"), py_str("with"), py_str("and"), py_str("on"), py_str("for"), py_str("at"), py_str("to"), py_str("from"), py_str("that"), py_str("when"), py_str("but"), py_str("as"), py_str("by"), py_str("into"), py_str("over"), py_str("under")]) }
    s__A_LINKING_WORD(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    s__THEIR_OWN(stdlib_call("re", "compile", &[py_str("^(?:my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("rabbit"), py_str("bunny"), py_str("hamster"), py_str("gerbil"), py_str("guinea"), py_str("pig"), py_str("horse"), py_str("pony"), py_str("donkey"), py_str("bird"), py_str("parrot"), py_str("budgie"), py_str("fish"), py_str("tortoise"), py_str("turtle"), py_str("snake"), py_str("lizard"), py_str("gecko"), py_str("chicken"), py_str("duck"), py_str("goat"), py_str("ferret"), py_str("rat"), py_str("mouse"), py_str("hedgehog"), py_str("chinchilla"), py_str("canary"), py_str("cockatiel")]) }
    s__AN_ANIMAL(call_builtin("frozenset", &[__data5()]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_INFINITIVE(stdlib_call("re", "compile", &[py_str("^to\\s+(?P<verb>[a-z']+)(?:\\s+(?P<rest>[a-z' ]+))?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("me"), py_str("you"), py_str("he"), py_str("she"), py_str("it"), py_str("we"), py_str("they"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("there"), py_str("if"), py_str("when"), py_str("about"), py_str("of")]) }
    s__A_CLAUSE_WORD(call_builtin("frozenset", &[__data6()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_str("think"), py_str("know"), py_str("believe"), py_str("remember"), py_str("forget"), py_str("be"), py_str("have"), py_str("seem"), py_str("suppose"), py_str("guess"), py_str("imagine"), py_str("feel"), py_str("want"), py_str("need"), py_str("say"), py_str("tell"), py_str("ask"), py_str("hear"), py_str("see"), py_str("understand"), py_str("mean"), py_str("hope"), py_str("wish"), py_str("expect"), py_str("assume"), py_str("pretend"), py_str("decide"), py_str("hesitate")]) }
    s__NOT_A_DOING(call_builtin("frozenset", &[__data7()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("swim"), py_str("run"), py_str("sit"), py_str("shop"), py_str("stop"), py_str("get"), py_str("put"), py_str("cut"), py_str("hit"), py_str("dig"), py_str("plan"), py_str("chat"), py_str("jog"), py_str("nap"), py_str("hop"), py_str("win"), py_str("drop"), py_str("trim"), py_str("rub"), py_str("knit"), py_str("shut"), py_str("swing"), py_str("begin"), py_str("forget"), py_str("travel"), py_str("snorkel")]) }
    s__DOUBLING(call_builtin("frozenset", &[__data8()]).unwrap_or_else(|e| exc::raise(e)));
    s__IRREGULAR_DOING(py_dict(vec![(py_str("be"), py_str("being")), (py_str("die"), py_str("dying")), (py_str("lie"), py_str("lying")), (py_str("tie"), py_str("tying")), (py_str("see"), py_str("seeing")), (py_str("agree"), py_str("agreeing")), (py_str("ski"), py_str("skiing")), (py_str("free"), py_str("freeing"))]));
    s__A_PRONOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("these"), py_str("those"), py_str("things"), py_str("everything"), py_str("anything"), py_str("stuff")])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_THING(stdlib_call("re", "compile", &[py_str("^(?:the\\s+|this\\s+|that\\s+|your\\s+|my\\s+|our\\s+|his\\s+|her\\s+|their\\s+)?(?:sound|idea|thought|look|smell|taste|prospect|notion|plan|suggestion|thinking|point|much|more|lots|plenty)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TAIL_FILLER(stdlib_call("re", "compile", &[py_str("[\\s,]*\\b(?:btw|tbh|lol|tho|though|anyway|anyways|honestly|i\\s+guess|as\\s+well|so\\s+much|very\\s+much|a\\s+lot|to\\s+bits|heaps|loads|right\\s+now|at\\s+the\\s+moment)\\b[\\s.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DEMONSTRATIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("this"), py_str("that"), py_str("these"), py_str("those")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data9() -> Py { py_tuple(vec![py_str("monday"), py_str("mondays"), py_str("tuesday"), py_str("tuesdays"), py_str("wednesday"), py_str("wednesdays"), py_str("thursday"), py_str("thursdays"), py_str("friday"), py_str("fridays"), py_str("saturday"), py_str("saturdays"), py_str("sunday"), py_str("sundays"), py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("christmas"), py_str("xmas"), py_str("easter"), py_str("halloween"), py_str("new year"), py_str("new years"), py_str("valentines"), py_str("valentine's"), py_str("boxing day")]) }
    s__PROPER(call_builtin("frozenset", &[__data9()]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _split_manner(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut last: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(the thing, the way they have it): \"my tea weak\" is (\"my tea\", \"weak\").\n\n    A manner comes off only when the object opens on the person's possessive, the last word is\n    one that says how a thing is, and nothing between them links the two into a single phrase.\n    ");
    words = mcall(&thing.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__MANNER_WORDS(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("the"), py_str("a"), py_str("an"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("it"), py_str("that"), py_str("this")]), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } } } }) {
    return py_tuple(vec![geti(&words.clone(), &Py::Int(0)), mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[])]);
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    return py_tuple(vec![thing.clone(), py_str("")]);
    }
    said = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    phrase = mcall(&g__A_MANNER_PHRASE(), "search", &[said.clone()]);
    if py_truthy(&phrase.clone()) {
    head = mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&mcall(&said.clone(), "lower", &[]), "index", &[mcall(&phrase.clone(), "group", &[Py::Int(0)])]), &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&head.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_LINKING_WORD(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_tuple(vec![op_add(&op_add(&geti(&words.clone(), &Py::Int(0)), &py_str(" ")), &head.clone()), mcall(&mcall(&phrase.clone(), "group", &[Py::Int(0)]), "lower", &[])]);
    }
    return py_tuple(vec![thing.clone(), py_str("")]);
    }
    last = mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(".,!;:")]);
    if py_truthy(&Py::Bool(!py_contains(&g__MANNER_WORDS(), &last.clone()))) {
    return py_tuple(vec![thing.clone(), py_str("")]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &op_neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_LINKING_WORD(), &mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!;:")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![thing.clone(), py_str("")]);
    }
    return py_tuple(vec![mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), last.clone()]);
    Py::None
}
pub fn _a_manner_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let _ = py_str("The pool for a thing and a manner said together; the kind's own pool otherwise.");
    return mcall(&py_dict(vec![(py_str("like"), py_str("like_manner")), (py_str("like_own"), py_str("like_manner")), (py_str("dislike"), py_str("dislike_manner"))]), "get", &[kind.clone(), kind.clone()]);
    Py::None
}
pub fn _a_doing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut verb: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let _ = py_str("The doing said as a name: \"cook\" is \"cooking\", \"swim\" is \"swimming\".");
    verb = mcall(&verb.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__IRREGULAR_DOING(), &verb.clone()))) {
    return geti(&g__IRREGULAR_DOING(), &verb.clone());
    }
    if py_truthy(&mcall(&verb.clone(), "endswith", &[py_str("ie")])) {
    return op_add(&slice_or_raise(&verb.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &py_str("ying"));
    }
    if py_truthy(&{ let __b = mcall(&verb.clone(), "endswith", &[py_str("e")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&verb.clone(), "endswith", &[py_tuple(vec![py_str("ee"), py_str("ye"), py_str("oe")])]))) } }) {
    return op_add(&slice_or_raise(&verb.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing"));
    }
    if py_truthy(&Py::Bool(py_contains(&g__DOUBLING(), &verb.clone()))) {
    return op_add(&op_add(&verb.clone(), &geti(&verb.clone(), &op_neg(&Py::Int(1)))), &py_str("ing"));
    }
    return op_add(&verb.clone(), &py_str("ing"));
    Py::None
}
pub fn _the_doing(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The doing a stance about \"to <verb>\" is about, capitalised, or \"\" when it is not one.");
    match_ = mcall(&g__AN_INFINITIVE(), "match", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    verb = mcall(&mcall(&match_.clone(), "group", &[py_str("verb")]), "lower", &[]);
    rest = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("rest")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__NOT_A_DOING(), &verb.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&verb.clone())) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&rest.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_CLAUSE_WORD(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return py_str("");
    }
    doing = _a_doing(&[verb.clone()]);
    return mcall(&op_add(&doing.clone(), &if py_truthy(&rest.clone()) { op_add(&py_str(" "), &rest.clone()) } else { py_str("") }), "capitalize", &[]);
    Py::None
}
pub fn _is_an_animal(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing of theirs is the animal they live with.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__THEIR_OWN(), "match", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&thing.clone(), "lower", &[]), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    last = geti(&words.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g__AN_ANIMAL(), &last.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = mcall(&last.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__AN_ANIMAL(), &slice_or_raise(&last.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } };
    Py::None
}
pub fn _the_stance(args: &[Py]) -> Py {
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
    let mut liked: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "liked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: liked"))) };
    let mut doing: Py = Py::None;
    let _ = py_str("(kind, thing) for the words after the verb: a doing, an animal of theirs, or the thing.\n\n    None when the words after the verb are a clause and not a thing at all (\"to think im\n    organised\", \"to know the price first\"): those lines have an answer already, and a frame\n    around half a clause is worse than the answer it replaces.\n    ");
    doing = _the_doing(&[thing.clone()]);
    if py_truthy(&doing.clone()) {
    return py_tuple(vec![if py_truthy(&liked.clone()) { py_str("like_doing") } else { py_str("dislike_doing") }, doing.clone()]);
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("^to\\s+[a-z]"), { let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&_is_an_animal(&[thing.clone()])) {
    return py_tuple(vec![if py_truthy(&liked.clone()) { py_str("like_pet") } else { py_str("dislike_pet") }, thing.clone()]);
    }
    if py_truthy(&{ let __b = liked.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__THEIR_OWN(), "match", &[thing.clone()]) } }) {
    return py_tuple(vec![py_str("like_own"), thing.clone()]);
    }
    return py_tuple(vec![if py_truthy(&liked.clone()) { py_str("like") } else { py_str("dislike") }, thing.clone()]);
    Py::None
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The line as words: lowercased, filler in front taken off, no trailing punctuation.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    said = stdlib_call("re", "sub", &[py_str("^(?:hi|hey|hello|yo|so|well|oh|god|honestly|anyway)[,!\\s]+"), py_str(""), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&said.clone(), "strip", &[py_str(" .!,")]);
    Py::None
}
pub fn _a_pointed_at_thing(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True for a pointing word that has the thing it points at after it.");
    words = mcall(&mcall(&{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[]);
    return { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__DEMONSTRATIVE(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("one"), py_str("ones"), py_str("sort"), py_str("kind"), py_str("type")]), &geti(&words.clone(), &Py::Int(1)))) } } } };
    Py::None
}
pub fn _the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut raw: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "raw") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: raw"))) };
    let mut thing: Py = Py::None;
    let _ = py_str("The words after the verb as they were said, or \"\" when they cannot carry a reply.");
    thing = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = raw.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!,;:")]);
    thing = mcall(&mcall(&g__TAIL_FILLER(), "sub", &[py_str(""), thing.clone()]), "strip", &[py_str(" .!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_PRONOUN(), &geti(&mcall(&mcall(&thing.clone(), "lower", &[]), "split", &[]), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_pointed_at_thing(&[thing.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g__A_DEED(), "match", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_SECOND_CLAUSE(), "search", &[thing.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g__NOT_A_THING(), "match", &[thing.clone()])) {
    return py_str("");
    }
    return geti(&_split_manner(&[thing.clone()]), &Py::Int(0));
    Py::None
}
pub fn _the_manner(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut raw: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "raw") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: raw"))) };
    let mut thing: Py = Py::None;
    let _ = py_str("The way they have it, when the object names a thing and a manner: \"my tea weak\" -> \"weak\".");
    thing = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = raw.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!,;:")]);
    thing = mcall(&mcall(&g__TAIL_FILLER(), "sub", &[py_str(""), thing.clone()]), "strip", &[py_str(" .!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return py_str("");
    }
    return geti(&_split_manner(&[thing.clone()]), &Py::Int(1));
    Py::None
}
pub fn reads_as_a_taste(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let _ = py_str("(kind, thing) for a line that only states a like or a dislike, else None.\n\n    `kind` is one of \"dislike\", \"like\", \"dread\", \"wait\"; `thing` is the words after the verb, or\n    \"\" when the line reads as a taste but names nothing a reply can carry. `_read` is the same\n    reading with the manner kept, which is what `reply_for` needs.\n    ");
    read = _read(&[text.clone()]);
    return if py_truthy(&read.clone()) { py_tuple(vec![geti(&read.clone(), &Py::Int(0)), geti(&read.clone(), &Py::Int(1))]) } else { Py::None };
    Py::None
}
pub fn _read(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut manner: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("(kind, thing, manner) for a line that only states a like or a dislike, else None.");
    said = _clean(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&py_call(&pyrt::xrefs::xget("compose.fallback", "_loves_someone"), &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__A_PERSON(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SOMEONE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ABOUT_ME(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_PERSON_DESCRIBED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_SYMPTOM(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ABOUT_WORK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__TWO_STANCES(), "search", &[said.clone()]) } } } } } } } } } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g__A_STANCE(), Py::None]), py_tuple(vec![g__A_STANCE_NO_SUBJECT(), Py::None]), py_tuple(vec![g__A_DREAD(), py_str("dread")]), py_tuple(vec![g__A_WAIT(), py_str("wait")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    match_ = mcall(&pattern.clone(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_thing(&[mcall(&match_.clone(), "group", &[py_str("thing")])])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_ASK(), "search", &[mcall(&match_.clone(), "group", &[py_str("thing")])])) {
    return Py::None;
    }
    raw = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "split", &[])]), "strip", &[py_str(" .!,;:")]);
    if py_truthy(&mcall(&g__THE_BOTS(), "match", &[raw.clone()])) {
    if py_truthy(&kind.clone()) {
    return py_tuple(vec![if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("wait")))) { py_str("the_bot_liked") } else { py_str("the_bot_disliked") }, py_str(""), py_str("")]);
    }
    verb = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("verb")]), "lower", &[]), "split", &[])]);
    return py_tuple(vec![if py_truthy(&Py::Bool(py_contains(&g__LIKE_VERBS(), &verb.clone()))) { py_str("the_bot_liked") } else { py_str("the_bot_disliked") }, py_str(""), py_str("")]);
    }
    manner = _the_manner(&[raw.clone()]);
    if py_truthy(&kind.clone()) {
    return py_tuple(vec![kind.clone(), _the_thing(&[raw.clone()]), py_str("")]);
    }
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &g__A_STANCE_NO_SUBJECT()))) {
    read = _the_stance(&[_the_thing(&[raw.clone()]), Py::Bool(false)]);
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![if py_truthy(&manner.clone()) { _a_manner_kind(&[geti(&read.clone(), &Py::Int(0))]) } else { geti(&read.clone(), &Py::Int(0)) }, geti(&read.clone(), &Py::Int(1)), manner.clone()]);
    }
    verb = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("verb")]), "lower", &[]), "split", &[])]);
    subject = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("subj")]), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("im"), py_str("i'm")]), &subject.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("like"), py_str("liking")]), &verb.clone())) } }) {
    return Py::None;
    }
    thing = _the_thing(&[raw.clone()]);
    if py_truthy(&Py::Bool(py_contains(&g__DISLIKE_VERBS(), &verb.clone()))) {
    read = _the_stance(&[thing.clone(), Py::Bool(false)]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g__LIKE_VERBS(), &verb.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&verb.clone(), &py_str("prefer"))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bto\\b"), thing.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    read = _the_stance(&[thing.clone(), Py::Bool(true)]);
    } else {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![if py_truthy(&manner.clone()) { _a_manner_kind(&[geti(&read.clone(), &Py::Int(0))]) } else { geti(&read.clone(), &Py::Int(0)) }, geti(&read.clone(), &Py::Int(1)), manner.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _as_a_name(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first word of the thing with its capital back, when it is a name rather than a word.");
    words = mcall(&thing.clone(), "split", &[py_str(" "), Py::Int(1)]);
    head = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.")]);
    if py_truthy(&Py::Bool(!py_contains(&g__PROPER(), &head.clone()))) {
    return thing.clone();
    }
    return op_add(&op_add(&mcall(&geti(&geti(&words.clone(), &Py::Int(0)), &Py::Int(0)), "upper", &[]), &slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::Int(1), &Py::None, &Py::None)), &if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { op_add(&py_str(" "), &geti(&words.clone(), &Py::Int(1))) } else { py_str("") });
    Py::None
}
pub fn _in_a_sentence(args: &[Py]) -> Py {
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
    let _ = py_str("The person's own words for the thing, lowercased mid-line: \"the rain\", not \"The rain\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&_as_a_name(&[thing.clone()]), &thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PROPER(), &mcall(&mcall(&geti(&mcall(&thing.clone(), "split", &[py_str(" ")]), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.")]))) } } } }) {
    return thing.clone();
    }
    return op_add(&mcall(&geti(&thing.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _as_yours(args: &[Py]) -> Py {
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
    let _ = py_str("The person's own thing as the bot would name it: \"my flat\" is \"your flat\" to a friend.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(my|our)\\b")); __args.push(py_str("your")); __args.push(thing.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _at_the_front(args: &[Py]) -> Py {
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
    let _ = py_str("The person's own words for the thing, with a capital, for a line that opens on it.");
    return if py_truthy(&thing.clone()) { op_add(&mcall(&geti(&thing.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { py_str("") };
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
    else { Py::None };
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut manner: Py = Py::None;
    let mut options: Py = Py::None;
    let mut read: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The reply for a stated like or dislike, or None when the line is not one.");
    read = _read(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&read.clone(), 3, None);
    kind = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    manner = __unpk1[2].clone();
    thing = _as_a_name(&[_as_yours(&[thing.clone()])]);
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&g__BY_KIND(), &kind.clone())) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), if py_truthy(&mcall(&line.clone(), "startswith", &[py_str("{thing}")])) { _at_the_front(&[thing.clone()]) } else { _in_a_sentence(&[thing.clone()]) }), ("manner".to_string(), if py_truthy(&mcall(&line.clone(), "startswith", &[py_str("{manner}")])) { _at_the_front(&[manner.clone()]) } else { manner.clone() })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(_clean(&[text.clone()])); __kws.extend(vec![("seed_text".to_string(), _clean(&[text.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.taste".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DISLIKE" => Some(g_A_DISLIKE()),
            "A_DISLIKE_DOING" => Some(g_A_DISLIKE_DOING()),
            "A_DISLIKE_MANNER" => Some(g_A_DISLIKE_MANNER()),
            "A_DISLIKE_PET" => Some(g_A_DISLIKE_PET()),
            "A_DREAD" => Some(g_A_DREAD()),
            "A_LIKE" => Some(g_A_LIKE()),
            "A_LIKE_DOING" => Some(g_A_LIKE_DOING()),
            "A_LIKE_MANNER" => Some(g_A_LIKE_MANNER()),
            "A_LIKE_OWN" => Some(g_A_LIKE_OWN()),
            "A_LIKE_PET" => Some(g_A_LIKE_PET()),
            "A_THE_BOT_DISLIKED" => Some(g_A_THE_BOT_DISLIKED()),
            "A_THE_BOT_LIKED" => Some(g_A_THE_BOT_LIKED()),
            "A_WAIT" => Some(g_A_WAIT()),
            "_ABOUT_ME" => Some(g__ABOUT_ME()),
            "_ABOUT_WORK" => Some(g__ABOUT_WORK()),
            "_AN_ANIMAL" => Some(g__AN_ANIMAL()),
            "_AN_INFINITIVE" => Some(g__AN_INFINITIVE()),
            "_A_ASK" => Some(g__A_ASK()),
            "_A_CLAUSE_WORD" => Some(g__A_CLAUSE_WORD()),
            "_A_DEED" => Some(g__A_DEED()),
            "_A_DREAD" => Some(g__A_DREAD()),
            "_A_LINKING_WORD" => Some(g__A_LINKING_WORD()),
            "_A_MANNER_PHRASE" => Some(g__A_MANNER_PHRASE()),
            "_A_PERSON" => Some(g__A_PERSON()),
            "_A_PERSON_DESCRIBED" => Some(g__A_PERSON_DESCRIBED()),
            "_A_PRONOUN" => Some(g__A_PRONOUN()),
            "_A_SECOND_CLAUSE" => Some(g__A_SECOND_CLAUSE()),
            "_A_SOMEONE" => Some(g__A_SOMEONE()),
            "_A_STANCE" => Some(g__A_STANCE()),
            "_A_STANCE_NO_SUBJECT" => Some(g__A_STANCE_NO_SUBJECT()),
            "_A_SYMPTOM" => Some(g__A_SYMPTOM()),
            "_A_WAIT" => Some(g__A_WAIT()),
            "_BY_KIND" => Some(g__BY_KIND()),
            "_DEMONSTRATIVE" => Some(g__DEMONSTRATIVE()),
            "_DISLIKE_VERBS" => Some(g__DISLIKE_VERBS()),
            "_DOUBLING" => Some(g__DOUBLING()),
            "_IRREGULAR_DOING" => Some(g__IRREGULAR_DOING()),
            "_LEAD" => Some(g__LEAD()),
            "_LEAD_SUBJECT" => Some(g__LEAD_SUBJECT()),
            "_LIKE_VERBS" => Some(g__LIKE_VERBS()),
            "_MANNER_WORDS" => Some(g__MANNER_WORDS()),
            "_NOT_A_DOING" => Some(g__NOT_A_DOING()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_PROPER" => Some(g__PROPER()),
            "_TAIL_FILLER" => Some(g__TAIL_FILLER()),
            "_THEIR_OWN" => Some(g__THEIR_OWN()),
            "_THE_BOTS" => Some(g__THE_BOTS()),
            "_TWO_STANCES" => Some(g__TWO_STANCES()),
            "_a_doing" => Some(py_func("_a_doing", |args: &[Py]| _a_doing(args))),
            "_a_manner_kind" => Some(py_func("_a_manner_kind", |args: &[Py]| _a_manner_kind(args))),
            "_a_pointed_at_thing" => Some(py_func("_a_pointed_at_thing", |args: &[Py]| _a_pointed_at_thing(args))),
            "_as_a_name" => Some(py_func("_as_a_name", |args: &[Py]| _as_a_name(args))),
            "_as_yours" => Some(py_func("_as_yours", |args: &[Py]| _as_yours(args))),
            "_at_the_front" => Some(py_func("_at_the_front", |args: &[Py]| _at_the_front(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_in_a_sentence" => Some(py_func("_in_a_sentence", |args: &[Py]| _in_a_sentence(args))),
            "_is_an_animal" => Some(py_func("_is_an_animal", |args: &[Py]| _is_an_animal(args))),
            "_read" => Some(py_func("_read", |args: &[Py]| _read(args))),
            "_split_manner" => Some(py_func("_split_manner", |args: &[Py]| _split_manner(args))),
            "_the_doing" => Some(py_func("_the_doing", |args: &[Py]| _the_doing(args))),
            "_the_manner" => Some(py_func("_the_manner", |args: &[Py]| _the_manner(args))),
            "_the_stance" => Some(py_func("_the_stance", |args: &[Py]| _the_stance(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "reads_as_a_taste" => Some(py_func("reads_as_a_taste", |args: &[Py]| reads_as_a_taste(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DISLIKE" => s_A_DISLIKE(v),
            "A_DISLIKE_DOING" => s_A_DISLIKE_DOING(v),
            "A_DISLIKE_MANNER" => s_A_DISLIKE_MANNER(v),
            "A_DISLIKE_PET" => s_A_DISLIKE_PET(v),
            "A_DREAD" => s_A_DREAD(v),
            "A_LIKE" => s_A_LIKE(v),
            "A_LIKE_DOING" => s_A_LIKE_DOING(v),
            "A_LIKE_MANNER" => s_A_LIKE_MANNER(v),
            "A_LIKE_OWN" => s_A_LIKE_OWN(v),
            "A_LIKE_PET" => s_A_LIKE_PET(v),
            "A_THE_BOT_DISLIKED" => s_A_THE_BOT_DISLIKED(v),
            "A_THE_BOT_LIKED" => s_A_THE_BOT_LIKED(v),
            "A_WAIT" => s_A_WAIT(v),
            "_ABOUT_ME" => s__ABOUT_ME(v),
            "_ABOUT_WORK" => s__ABOUT_WORK(v),
            "_AN_ANIMAL" => s__AN_ANIMAL(v),
            "_AN_INFINITIVE" => s__AN_INFINITIVE(v),
            "_A_ASK" => s__A_ASK(v),
            "_A_CLAUSE_WORD" => s__A_CLAUSE_WORD(v),
            "_A_DEED" => s__A_DEED(v),
            "_A_DREAD" => s__A_DREAD(v),
            "_A_LINKING_WORD" => s__A_LINKING_WORD(v),
            "_A_MANNER_PHRASE" => s__A_MANNER_PHRASE(v),
            "_A_PERSON" => s__A_PERSON(v),
            "_A_PERSON_DESCRIBED" => s__A_PERSON_DESCRIBED(v),
            "_A_PRONOUN" => s__A_PRONOUN(v),
            "_A_SECOND_CLAUSE" => s__A_SECOND_CLAUSE(v),
            "_A_SOMEONE" => s__A_SOMEONE(v),
            "_A_STANCE" => s__A_STANCE(v),
            "_A_STANCE_NO_SUBJECT" => s__A_STANCE_NO_SUBJECT(v),
            "_A_SYMPTOM" => s__A_SYMPTOM(v),
            "_A_WAIT" => s__A_WAIT(v),
            "_BY_KIND" => s__BY_KIND(v),
            "_DEMONSTRATIVE" => s__DEMONSTRATIVE(v),
            "_DISLIKE_VERBS" => s__DISLIKE_VERBS(v),
            "_DOUBLING" => s__DOUBLING(v),
            "_IRREGULAR_DOING" => s__IRREGULAR_DOING(v),
            "_LEAD" => s__LEAD(v),
            "_LEAD_SUBJECT" => s__LEAD_SUBJECT(v),
            "_LIKE_VERBS" => s__LIKE_VERBS(v),
            "_MANNER_WORDS" => s__MANNER_WORDS(v),
            "_NOT_A_DOING" => s__NOT_A_DOING(v),
            "_NOT_A_THING" => s__NOT_A_THING(v),
            "_PROPER" => s__PROPER(v),
            "_TAIL_FILLER" => s__TAIL_FILLER(v),
            "_THEIR_OWN" => s__THEIR_OWN(v),
            "_THE_BOTS" => s__THE_BOTS(v),
            "_TWO_STANCES" => s__TWO_STANCES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.taste", module_obj);
    pyrt::xrefs::reg_fn("compose.taste", "_split_manner", py_func("_split_manner", |args: &[Py]| _split_manner(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_a_manner_kind", py_func("_a_manner_kind", |args: &[Py]| _a_manner_kind(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_a_doing", py_func("_a_doing", |args: &[Py]| _a_doing(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_the_doing", py_func("_the_doing", |args: &[Py]| _the_doing(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_is_an_animal", py_func("_is_an_animal", |args: &[Py]| _is_an_animal(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_the_stance", py_func("_the_stance", |args: &[Py]| _the_stance(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_a_pointed_at_thing", py_func("_a_pointed_at_thing", |args: &[Py]| _a_pointed_at_thing(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_the_manner", py_func("_the_manner", |args: &[Py]| _the_manner(args)));
    pyrt::xrefs::reg_fn("compose.taste", "reads_as_a_taste", py_func("reads_as_a_taste", |args: &[Py]| reads_as_a_taste(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_read", py_func("_read", |args: &[Py]| _read(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_as_a_name", py_func("_as_a_name", |args: &[Py]| _as_a_name(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_in_a_sentence", py_func("_in_a_sentence", |args: &[Py]| _in_a_sentence(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_as_yours", py_func("_as_yours", |args: &[Py]| _as_yours(args)));
    pyrt::xrefs::reg_fn("compose.taste", "_at_the_front", py_func("_at_the_front", |args: &[Py]| _at_the_front(args)));
    pyrt::xrefs::reg_fn("compose.taste", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.taste", "A_DISLIKE", g_A_DISLIKE, s_A_DISLIKE);
    pyrt::xrefs::reg_access("compose.taste", "A_DISLIKE_DOING", g_A_DISLIKE_DOING, s_A_DISLIKE_DOING);
    pyrt::xrefs::reg_access("compose.taste", "A_DISLIKE_MANNER", g_A_DISLIKE_MANNER, s_A_DISLIKE_MANNER);
    pyrt::xrefs::reg_access("compose.taste", "A_DISLIKE_PET", g_A_DISLIKE_PET, s_A_DISLIKE_PET);
    pyrt::xrefs::reg_access("compose.taste", "A_DREAD", g_A_DREAD, s_A_DREAD);
    pyrt::xrefs::reg_access("compose.taste", "A_LIKE", g_A_LIKE, s_A_LIKE);
    pyrt::xrefs::reg_access("compose.taste", "A_LIKE_DOING", g_A_LIKE_DOING, s_A_LIKE_DOING);
    pyrt::xrefs::reg_access("compose.taste", "A_LIKE_MANNER", g_A_LIKE_MANNER, s_A_LIKE_MANNER);
    pyrt::xrefs::reg_access("compose.taste", "A_LIKE_OWN", g_A_LIKE_OWN, s_A_LIKE_OWN);
    pyrt::xrefs::reg_access("compose.taste", "A_LIKE_PET", g_A_LIKE_PET, s_A_LIKE_PET);
    pyrt::xrefs::reg_access("compose.taste", "A_THE_BOT_DISLIKED", g_A_THE_BOT_DISLIKED, s_A_THE_BOT_DISLIKED);
    pyrt::xrefs::reg_access("compose.taste", "A_THE_BOT_LIKED", g_A_THE_BOT_LIKED, s_A_THE_BOT_LIKED);
    pyrt::xrefs::reg_access("compose.taste", "A_WAIT", g_A_WAIT, s_A_WAIT);
    pyrt::xrefs::reg_access("compose.taste", "_ABOUT_ME", g__ABOUT_ME, s__ABOUT_ME);
    pyrt::xrefs::reg_access("compose.taste", "_ABOUT_WORK", g__ABOUT_WORK, s__ABOUT_WORK);
    pyrt::xrefs::reg_access("compose.taste", "_AN_ANIMAL", g__AN_ANIMAL, s__AN_ANIMAL);
    pyrt::xrefs::reg_access("compose.taste", "_AN_INFINITIVE", g__AN_INFINITIVE, s__AN_INFINITIVE);
    pyrt::xrefs::reg_access("compose.taste", "_A_ASK", g__A_ASK, s__A_ASK);
    pyrt::xrefs::reg_access("compose.taste", "_A_CLAUSE_WORD", g__A_CLAUSE_WORD, s__A_CLAUSE_WORD);
    pyrt::xrefs::reg_access("compose.taste", "_A_DEED", g__A_DEED, s__A_DEED);
    pyrt::xrefs::reg_access("compose.taste", "_A_DREAD", g__A_DREAD, s__A_DREAD);
    pyrt::xrefs::reg_access("compose.taste", "_A_LINKING_WORD", g__A_LINKING_WORD, s__A_LINKING_WORD);
    pyrt::xrefs::reg_access("compose.taste", "_A_MANNER_PHRASE", g__A_MANNER_PHRASE, s__A_MANNER_PHRASE);
    pyrt::xrefs::reg_access("compose.taste", "_A_PERSON", g__A_PERSON, s__A_PERSON);
    pyrt::xrefs::reg_access("compose.taste", "_A_PERSON_DESCRIBED", g__A_PERSON_DESCRIBED, s__A_PERSON_DESCRIBED);
    pyrt::xrefs::reg_access("compose.taste", "_A_PRONOUN", g__A_PRONOUN, s__A_PRONOUN);
    pyrt::xrefs::reg_access("compose.taste", "_A_SECOND_CLAUSE", g__A_SECOND_CLAUSE, s__A_SECOND_CLAUSE);
    pyrt::xrefs::reg_access("compose.taste", "_A_SOMEONE", g__A_SOMEONE, s__A_SOMEONE);
    pyrt::xrefs::reg_access("compose.taste", "_A_STANCE", g__A_STANCE, s__A_STANCE);
    pyrt::xrefs::reg_access("compose.taste", "_A_STANCE_NO_SUBJECT", g__A_STANCE_NO_SUBJECT, s__A_STANCE_NO_SUBJECT);
    pyrt::xrefs::reg_access("compose.taste", "_A_SYMPTOM", g__A_SYMPTOM, s__A_SYMPTOM);
    pyrt::xrefs::reg_access("compose.taste", "_A_WAIT", g__A_WAIT, s__A_WAIT);
    pyrt::xrefs::reg_access("compose.taste", "_BY_KIND", g__BY_KIND, s__BY_KIND);
    pyrt::xrefs::reg_access("compose.taste", "_DEMONSTRATIVE", g__DEMONSTRATIVE, s__DEMONSTRATIVE);
    pyrt::xrefs::reg_access("compose.taste", "_DISLIKE_VERBS", g__DISLIKE_VERBS, s__DISLIKE_VERBS);
    pyrt::xrefs::reg_access("compose.taste", "_DOUBLING", g__DOUBLING, s__DOUBLING);
    pyrt::xrefs::reg_access("compose.taste", "_IRREGULAR_DOING", g__IRREGULAR_DOING, s__IRREGULAR_DOING);
    pyrt::xrefs::reg_access("compose.taste", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("compose.taste", "_LEAD_SUBJECT", g__LEAD_SUBJECT, s__LEAD_SUBJECT);
    pyrt::xrefs::reg_access("compose.taste", "_LIKE_VERBS", g__LIKE_VERBS, s__LIKE_VERBS);
    pyrt::xrefs::reg_access("compose.taste", "_MANNER_WORDS", g__MANNER_WORDS, s__MANNER_WORDS);
    pyrt::xrefs::reg_access("compose.taste", "_NOT_A_DOING", g__NOT_A_DOING, s__NOT_A_DOING);
    pyrt::xrefs::reg_access("compose.taste", "_NOT_A_THING", g__NOT_A_THING, s__NOT_A_THING);
    pyrt::xrefs::reg_access("compose.taste", "_PROPER", g__PROPER, s__PROPER);
    pyrt::xrefs::reg_access("compose.taste", "_TAIL_FILLER", g__TAIL_FILLER, s__TAIL_FILLER);
    pyrt::xrefs::reg_access("compose.taste", "_THEIR_OWN", g__THEIR_OWN, s__THEIR_OWN);
    pyrt::xrefs::reg_access("compose.taste", "_THE_BOTS", g__THE_BOTS, s__THE_BOTS);
    pyrt::xrefs::reg_access("compose.taste", "_TWO_STANCES", g__TWO_STANCES, s__TWO_STANCES);
}
