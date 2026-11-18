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


thread_local! { static G_ABOUT_PERSON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_ADVERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_AS_A_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_PERSON_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_RULE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_YOU_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_NAMES_TYPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_NAME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_ANY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MANY_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_ALIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEVERAL_PEOPLE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_A_YES_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_NOT_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TASTE_WHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_ARE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_KIDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_OF_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PLACE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IRREGULAR_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_OLDER_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PLACE_STANDS_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_YOUNGER_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_bot: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_normalise: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_others: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_stem: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.relations: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_PERSON_ASK() -> Py { ensure_init(); G_ABOUT_PERSON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_PERSON_ASK(v: Py) { ensure_init(); G_ABOUT_PERSON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_ADVERB() -> Py { ensure_init(); G_AGE_ADVERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ADVERB(v: Py) { ensure_init(); G_AGE_ADVERB.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_AGAIN() -> Py { ensure_init(); G_AGE_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_AGAIN(v: Py) { ensure_init(); G_AGE_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_ASK() -> Py { ensure_init(); G_AGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ASK(v: Py) { ensure_init(); G_AGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_AS_A_CAUSE() -> Py { ensure_init(); G_AGE_AS_A_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_AS_A_CAUSE(v: Py) { ensure_init(); G_AGE_AS_A_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_COPULA() -> Py { ensure_init(); G_AGE_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_COPULA(v: Py) { ensure_init(); G_AGE_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_DETERMINER() -> Py { ensure_init(); G_AGE_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_DETERMINER(v: Py) { ensure_init(); G_AGE_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_PERSON_WORD() -> Py { ensure_init(); G_AGE_PERSON_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_PERSON_WORD(v: Py) { ensure_init(); G_AGE_PERSON_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_RULE_WORD() -> Py { ensure_init(); G_AGE_RULE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_RULE_WORD(v: Py) { ensure_init(); G_AGE_RULE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_SAID() -> Py { ensure_init(); G_AGE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_SAID(v: Py) { ensure_init(); G_AGE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_WORD() -> Py { ensure_init(); G_AGE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_WORD(v: Py) { ensure_init(); G_AGE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_YOU_SAY() -> Py { ensure_init(); G_AGE_YOU_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_YOU_SAY(v: Py) { ensure_init(); G_AGE_YOU_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_MANY() -> Py { ensure_init(); G_ALREADY_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_MANY(v: Py) { ensure_init(); G_ALREADY_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_PERSON() -> Py { ensure_init(); G_ANOTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_PERSON(v: Py) { ensure_init(); G_ANOTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_BREED_ASK() -> Py { ensure_init(); G_BREED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREED_ASK(v: Py) { ensure_init(); G_BREED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_TAIL() -> Py { ensure_init(); G_CHAT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_TAIL(v: Py) { ensure_init(); G_CHAT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD() -> Py { ensure_init(); G_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD(v: Py) { ensure_init(); G_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_OF() -> Py { ensure_init(); G_CHILD_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_OF(v: Py) { ensure_init(); G_CHILD_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_RELATIONS() -> Py { ensure_init(); G_CHILD_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_RELATIONS(v: Py) { ensure_init(); G_CHILD_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORD() -> Py { ensure_init(); G_CHILD_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORD(v: Py) { ensure_init(); G_CHILD_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_GROUP() -> Py { ensure_init(); G_FAMILY_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_GROUP(v: Py) { ensure_init(); G_FAMILY_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_ASK() -> Py { ensure_init(); G_GROUP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_ASK(v: Py) { ensure_init(); G_GROUP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_NAMES_TYPED() -> Py { ensure_init(); G_GROUP_NAMES_TYPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_NAMES_TYPED(v: Py) { ensure_init(); G_GROUP_NAMES_TYPED.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_NAME_ASK() -> Py { ensure_init(); G_GROUP_NAME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_NAME_ASK(v: Py) { ensure_init(); G_GROUP_NAME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_OF() -> Py { ensure_init(); G_GROUP_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_OF(v: Py) { ensure_init(); G_GROUP_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_WORDS() -> Py { ensure_init(); G_GROUP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_WORDS(v: Py) { ensure_init(); G_GROUP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_ANY() -> Py { ensure_init(); G_HAVE_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_ANY(v: Py) { ensure_init(); G_HAVE_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_ANY_WORDS() -> Py { ensure_init(); G_HAVE_ANY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_ANY_WORDS(v: Py) { ensure_init(); G_HAVE_ANY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HER_WORD() -> Py { ensure_init(); G_HER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HER_WORD(v: Py) { ensure_init(); G_HER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_MANY_OF_THEM() -> Py { ensure_init(); G_MANY_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MANY_OF_THEM(v: Py) { ensure_init(); G_MANY_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_AGAIN() -> Py { ensure_init(); G_NAME_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_AGAIN(v: Py) { ensure_init(); G_NAME_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_MANY() -> Py { ensure_init(); G_PEOPLE_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_MANY(v: Py) { ensure_init(); G_PEOPLE_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_AFTER() -> Py { ensure_init(); G_PERSON_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_AFTER(v: Py) { ensure_init(); G_PERSON_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_GROUP() -> Py { ensure_init(); G_PERSON_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_GROUP(v: Py) { ensure_init(); G_PERSON_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_ASK() -> Py { ensure_init(); G_PET_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_ASK(v: Py) { ensure_init(); G_PET_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_WORDS() -> Py { ensure_init(); G_PET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_WORDS(v: Py) { ensure_init(); G_PET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_ANIMAL() -> Py { ensure_init(); G_PLURAL_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_ANIMAL(v: Py) { ensure_init(); G_PLURAL_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_PEOPLE() -> Py { ensure_init(); G_PRONOUN_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_PEOPLE(v: Py) { ensure_init(); G_PRONOUN_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_ALIKE() -> Py { ensure_init(); G_RELATION_ALIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_ALIKE(v: Py) { ensure_init(); G_RELATION_ALIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_SEVERAL_PEOPLE_NAMED() -> Py { ensure_init(); G_SEVERAL_PEOPLE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEVERAL_PEOPLE_NAMED(v: Py) { ensure_init(); G_SEVERAL_PEOPLE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_AGE() -> Py { ensure_init(); G_THEIR_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_AGE(v: Py) { ensure_init(); G_THEIR_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_JOB() -> Py { ensure_init(); G_THEIR_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_JOB(v: Py) { ensure_init(); G_THEIR_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NAME() -> Py { ensure_init(); G_THEIR_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NAME(v: Py) { ensure_init(); G_THEIR_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_A_YES_NO() -> Py { ensure_init(); G_THEIR_TASTE_A_YES_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_A_YES_NO(v: Py) { ensure_init(); G_THEIR_TASTE_A_YES_NO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_INTO() -> Py { ensure_init(); G_THEIR_TASTE_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_INTO(v: Py) { ensure_init(); G_THEIR_TASTE_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_NOT() -> Py { ensure_init(); G_THEIR_TASTE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_NOT(v: Py) { ensure_init(); G_THEIR_TASTE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_NOT_INTO() -> Py { ensure_init(); G_THEIR_TASTE_NOT_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_NOT_INTO(v: Py) { ensure_init(); G_THEIR_TASTE_NOT_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TASTE_WHAT() -> Py { ensure_init(); G_THEIR_TASTE_WHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TASTE_WHAT(v: Py) { ensure_init(); G_THEIR_TASTE_WHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_ARE_NAMED() -> Py { ensure_init(); G_THEY_ARE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_ARE_NAMED(v: Py) { ensure_init(); G_THEY_ARE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_KIDS() -> Py { ensure_init(); G_THE_KIDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_KIDS(v: Py) { ensure_init(); G_THE_KIDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_OF_KIN() -> Py { ensure_init(); G_TWO_OF_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_OF_KIN(v: Py) { ensure_init(); G_TWO_OF_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_WHO() -> Py { ensure_init(); G__ABOUT_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_WHO(v: Py) { ensure_init(); G__ABOUT_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PLACE_ASK() -> Py { ensure_init(); G__A_PLACE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PLACE_ASK(v: Py) { ensure_init(); G__A_PLACE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__IRREGULAR_PLURAL() -> Py { ensure_init(); G__IRREGULAR_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IRREGULAR_PLURAL(v: Py) { ensure_init(); G__IRREGULAR_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_WORD() -> Py { ensure_init(); G__PET_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_WORD(v: Py) { ensure_init(); G__PET_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL() -> Py { ensure_init(); G__PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL(v: Py) { ensure_init(); G__PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_OLDER_END() -> Py { ensure_init(); G__THE_OLDER_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_OLDER_END(v: Py) { ensure_init(); G__THE_OLDER_END.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PLACE_STANDS_ALONE() -> Py { ensure_init(); G__THE_PLACE_STANDS_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PLACE_STANDS_ALONE(v: Py) { ensure_init(); G__THE_PLACE_STANDS_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_YOUNGER_END() -> Py { ensure_init(); G__THE_YOUNGER_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_YOUNGER_END(v: Py) { ensure_init(); G__THE_YOUNGER_END.with(|c| *c.borrow_mut() = v); }
pub fn g_last_bot() -> Py { ensure_init(); G_last_bot.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_bot(v: Py) { ensure_init(); G_last_bot.with(|c| *c.borrow_mut() = v); }
pub fn g_normalise() -> Py { ensure_init(); G_normalise.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_normalise(v: Py) { ensure_init(); G_normalise.with(|c| *c.borrow_mut() = v); }
pub fn g_others() -> Py { ensure_init(); G_others.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_others(v: Py) { ensure_init(); G_others.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_stem() -> Py { ensure_init(); G_stem.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_stem(v: Py) { ensure_init(); G_stem.with(|c| *c.borrow_mut() = v); }
pub fn g_user() -> Py { ensure_init(); G_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_user(v: Py) { ensure_init(); G_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The people and animals in the store: a relation, the group it belongs to, the name they were given,\nthe age they were said to be, and the answer to a question about them.\n\nMoved out of `bot/modules/memory_store.py` whole, name by name, with nothing changed but the\nimport lines: the file is over the Hub's 1 MiB limit, which stops every PR in the swarm until it\nis under it again. The set is closed - these names need nothing from the module they left - and\nthe module imports them back, so no call site and no reply changes.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_last_bot(py_func("last_bot", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_bot", args)));
    s_others(py_func("others", |args: &[Py]| pyrt::xrefs::xcall("memory", "others", args)));
    s_stem(pyrt::xrefs::xget("memory", "stem"));
    s_user(py_func("user", |args: &[Py]| pyrt::xrefs::xcall("memory", "user", args)));
    s_CHILD(pyrt::xrefs::xget("memory.store_lines", "CHILD"));
    s_CHILD_OF(pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"));
    s_CHILD_RELATIONS(pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS"));
    s_FAMILY_GROUP(pyrt::xrefs::xget("memory.store_lines", "FAMILY_GROUP"));
    s_GROUP_OF(pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"));
    s_HAVE_ANY_WORDS(pyrt::xrefs::xget("memory.store_lines", "HAVE_ANY_WORDS"));
    s_HER_WORD(pyrt::xrefs::xget("memory.store_lines", "HER_WORD"));
    s_PERSON_GROUP(pyrt::xrefs::xget("memory.store_lines", "PERSON_GROUP"));
    s_PET_WORDS(pyrt::xrefs::xget("memory.store_lines", "PET_WORDS"));
    s_PRONOUN_PEOPLE(pyrt::xrefs::xget("memory.store_lines", "PRONOUN_PEOPLE"));
    s_RELATION_ALIKE(pyrt::xrefs::xget("memory.store_lines", "RELATION_ALIKE"));
    s__ABOUT_WHO(pyrt::xrefs::xget("memory.store_lines", "_ABOUT_WHO"));
    s_normalise(py_func("normalise", |args: &[Py]| pyrt::xrefs::xcall("understanding.normalize", "normalise", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.CHAT_TAIL"); }
    s_CHAT_TAIL(stdlib_call("re", "compile", &[py_str("(?:[\\s,]*\\b(?:of|in|from|for|to)\\s+(?:this|our|the)\\s+(?:chat|conversation|talk|session|thread|conversation so far)\\b(?:\\s+(?:so far|today|here|with you|we have had))?)+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.PERSON_AFTER"); }
    s_PERSON_AFTER(stdlib_call("re", "compile", &[py_str("^(?:\\s*$|\\s*[?.!,;]|\\s+(?:is|are|was|were|and|or|now|then|today|already|yet|turning|turn|turns|aged|age|again|too)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.ABOUT_PERSON_ASK"); }
    s_ABOUT_PERSON_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:what|anything|everything|much)\\b[^?]{0,30}?\\b(?:know|knows|remember|learnt|learned|told|tell|said|say|mention|mentioned|got|have|has|had)\\b[^?]{0,20}?\\b(?:about|on)\\s+(?:"), &pyrt::xrefs::xget("memory.store_lines", "_ABOUT_WHO")), &py_str(")|\\btell me (?:a bit |a little |more |anything )?(?:more )?about\\s+(?:")), &pyrt::xrefs::xget("memory.store_lines", "_ABOUT_WHO")), &py_str(")|\\b(?:do|did|can|could)\\s+(?:you|i)\\s+(?:ever\\s+|already\\s+)?(?:know|remember|recall|tell|told|mention|mentioned|say|said|hear|heard)\\b[^?]{0,30}?\\b(?:about|on)\\s+(?:")), &pyrt::xrefs::xget("memory.store_lines", "_ABOUT_WHO")), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.ANOTHER_PERSON"); }
    s_ANOTHER_PERSON(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:(?:and|or|also|then|ok|okay|what about|and what about|how about)\\s+)*(?:my|our|the|your)?\\s*(?:little|big|older|younger|eldest|youngest|oldest|other|second|third)?\\s*(?:"), &mcall(&py_str("|"), "join", &[pyrt::xrefs::xget("memory.facts", "RELATIONS")])), &py_str(")s?\\s*\\?*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_ASK"); }
    s_AGE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow old (?:is|are|was|were)\\b|\\bwhat age(?:s)? (?:is|are|was|were)\\b|\\bwhat ages?\\b|\\bhow old\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_RULE_WORD"); }
    s_AGE_RULE_WORD(stdlib_call("re", "compile", &[py_str("^(?:do|does|did|should|shall|can|could|will|would|must|may|might|need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_YOU_SAY"); }
    s_AGE_YOU_SAY(stdlib_call("re", "compile", &[py_str("^(?:you|u|i)\\s+(?:say|said|think|guess|remember|know|reckon)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_COPULA"); }
    s_AGE_COPULA(stdlib_call("re", "compile", &[py_str("^(?:is|are|was|were|be|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_ADVERB"); }
    s_AGE_ADVERB(stdlib_call("re", "compile", &[py_str("^(?:exactly|really|roughly|approximately|about|now|actually|again|already|even)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_DETERMINER"); }
    s_AGE_DETERMINER(stdlib_call("re", "compile", &[py_str("^(?:my|our|your|their|his|her|the|a|an)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_PERSON_WORD"); }
    s_AGE_PERSON_WORD(stdlib_call("re", "compile", &[py_str("^(?:kids?|children|child|family|i|you|u|we|they|them|he|she|everyone|both|little|one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_AGAIN"); }
    s_AGE_AGAIN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:(?:and|or|also|ok|okay|what about|and what about)\\s+)*(?:my|your|the|our)?\\s*(?:little|big|older|younger|eldest|youngest|oldest|other)?\\s*("), &mcall(&py_str("|"), "join", &[op_add(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &pyrt::xrefs::xget("memory.facts", "ALONE_PERSON"))])), &py_str("|kids?|children)s?\\s*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.NAME_AGAIN"); }
    s_NAME_AGAIN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:(?:and|or|also|then|ok|okay|what about|and what about)\\b[\\s,]*)*(?:my|our|your|the)?\\s*(?:"), &mcall(&py_str("|"), "join", &[pyrt::xrefs::xget("memory.facts", "KIN_WORDS")])), &py_str(")(?:'s|s)?\\s+name\\s*\\?*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_SAID"); }
    s_AGE_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:your|his|her|their)\\s+\\w+\\s+is\\s+\\d+\\b|^\\w+\\s+is\\s+\\d+\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THE_KIDS"); }
    s_THE_KIDS(stdlib_call("re", "compile", &[py_str("\\b(?:kids?|children|child|family|everyone|both|they|them|their|boys?|girls?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.CHILD_WORD"); }
    s_CHILD_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:kids?|children|child|family|boys?|girls?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._IRREGULAR_PLURAL"); }
    s__IRREGULAR_PLURAL(py_dict(vec![(py_str("child"), py_str("children")), (py_str("grandchild"), py_str("grandchildren"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_WORD"); }
    s_AGE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|their|his|her|your)\\s+(?:\\w+\\s+)?ages?\\b|\\bages?\\s+of\\s+(?:my|our|the|their)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.AGE_AS_A_CAUSE"); }
    s_AGE_AS_A_CAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:just|only|part of|down to|because of|normal for|usual for|expected at)\\s+(?:\\w+\\s+){0,2}(?:age|old age|getting old)\\b|\\bold age\\b|\\bat (?:her|his|their|my) age\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.GROUP_ASK"); }
    s_GROUP_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|the|his|her|their|our|your)\\s+("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.store_lines", "FAMILY_GROUP")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_NAME"); }
    s_THEIR_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\b[^.?!]*\\bname\\b|\\bwhat (?:is|was) (?:my|our)\\s+\\w+'?s?\\s+(?:called|named)\\b|\\bwhat do (?:i|you) call my\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_AGE"); }
    s_THEIR_AGE(stdlib_call("re", "compile", &[py_str("\\bhow old (?:is|was|are|were)\\b|\\bwhat age (?:is|was)\\b|\\b(?:what|which)(?:'s| is| was| are)?\\s+(?:(?:my|our)\\s+\\w+'?s?|her|his|their)\\s+age\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.GROUP_WORDS"); }
    s_GROUP_WORDS(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.store_lines", "GROUP_OF")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.GROUP_NAME_ASK"); }
    s_GROUP_NAME_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bwhat (?:are|were)\\s+(?:my|our|the)\\s+(?P<a>"), &g_GROUP_WORDS()), &py_str(")\\s+called\\b|\\bwhat (?:are|were)\\s+(?:my|our|the)\\s+(?P<b>")), &g_GROUP_WORDS()), &py_str(")['\\u2019s]*\\s+names?\\b|\\b(?:names? of|name of)\\s+(?:my|our|the)\\s+(?P<c>")), &g_GROUP_WORDS()), &py_str(")\\b|\\b(?:my|our|the)\\s+(?P<d>")), &g_GROUP_WORDS()), &py_str(")['\\u2019s]*\\s+names\\b|\\bwhat (?:are|were)\\s+(?:they|their|them)\\s+called\\b|\\bwhat are their names\\b|\\btheir names\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_TASTE_WHAT"); }
    s_THEIR_TASTE_WHAT(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:does|do|did)\\s+(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]*)|(?P<pronoun>she|he|they))\\s+(?P<how>like|love|enjoy|hate|dislike|not like)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_TASTE_INTO"); }
    s_THEIR_TASTE_INTO(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:is|was)\\s+(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]*)|(?P<pronoun>she|he|they))\\s+(?:into|keen on|mad about|a fan of|big on|fussed on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_TASTE_NOT"); }
    s_THEIR_TASTE_NOT(stdlib_call("re", "compile", &[py_str("\\bwhat(?:\\s+[a-z']+){0,2}?\\s+(?:doesn'?t|doesn|does\\s+not|don'?t|do\\s+not|won'?t|will\\s+not|can'?t|cannot|never)\\s+(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]*)|(?P<pronoun>she|he|they))\\s+(?:really\\s+|actually\\s+|even\\s+|ever\\s+)?(?:like|love|eat|drink|have|stand|touch|want|be\\s+keen\\s+on)\\b(?:\\s+(?:eating|drinking|at\\s+all|really|then|now|though))*\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_TASTE_NOT_INTO"); }
    s_THEIR_TASTE_NOT_INTO(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s|\\s+is|\\s+was)?(?:\\s+[a-z']+)?\\s+(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]*)|(?P<pronoun>she|he|they))\\s+(?:not|isn'?t|is\\s+not)\\s+(?:into|keen\\s+on|mad\\s+about|a\\s+fan\\s+of|big\\s+on|fussed\\s+on)\\b\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_TASTE_A_YES_NO"); }
    s_THEIR_TASTE_A_YES_NO(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|did)\\s+(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]*)|(?P<pronoun>she|he|they))\\s+(?P<how>like|love|enjoy|hate|dislike)\\s+(?P<thing>[a-z][a-z'\\u2019 ]*?)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._PLURAL"); }
    s__PLURAL(call_builtin("frozenset", &[mcall(&py_str("kids children sons daughters boys girls parents friends brothers sisters\ngrandchildren grandkids nieces nephews cousins grandparents in-laws"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEIR_JOB"); }
    s_THEIR_JOB(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:does|do|did)\\s+(?:(?:my|our)\\s+(?P<who>[a-z][a-z'\\u2019-]*)|(?P<pronoun>she|he|they))\\s+do\\b(?:\\s+(?:for (?:a living|work|their job)|as a job|exactly|all day|now))?\\W*$|\\bwhat(?:'s|s| is| was)?\\s+(?:(?:my|our)\\s+(?P<who2>[a-z][a-z'\\u2019-]*)|(?P<pronoun2>she|he|they))'?s?\\s+(?:job|work)\\b|\\bwhat\\s+(?:(?:my|our)\\s+(?P<who3>[a-z][a-z'\\u2019-]*)|(?P<pronoun3>she|he|they))\\s+(?:does|do)\\b\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.HAVE_ANY"); }
    s_HAVE_ANY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|but\\s+)?(?:do|have|did)\\s+(?:i|we)\\s+(?:got|have|has)?\\s*(?:(?:any|a|an)\\s+)?(?P<what>(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.store_lines", "HAVE_ANY_WORDS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")(?:\\s*(?:,|or|and)\\s*(?:brothers|sisters|siblings|children|kids|parents|family)|\\s*(?:,|or|and)\\s*(?:brother|sister|parent|child|kid))*)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.GROUP_NAMES_TYPED"); }
    s_GROUP_NAMES_TYPED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bmy (?P<who>"), &mcall(&py_str("|"), "join", &[py_list(vec![py_str("kids"), py_str("children"), py_str("childs"), py_str("sons"), py_str("daughters"), py_str("boys"), py_str("girls"), py_str("parents"), py_str("brothers"), py_str("sisters"), py_str("grandchildren")])])), &py_str(")\\s+(?:are|is)\\s+(?P<names>[^.!?]{3,60})")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.THEY_ARE_NAMED"); }
    s_THEY_ARE_NAMED(stdlib_call("re", "compile", &[py_str("^\\W*(?:they|they're|theyre|those|these)\\s+(?:are|were)\\s+(?:called|named)\\s+(?P<names>[^.!?]{3,60})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.PEOPLE_MANY"); }
    s_PEOPLE_MANY(py_str("kids|children|childs|sons|daughters|boys|girls|brothers|sisters|twins|parents|grandchildren|grandparents|aunts|uncles|cousins|siblings|stepchildren|nieces|nephews|babies|teens|teenagers"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.SEVERAL_PEOPLE_NAMED"); }
    s_SEVERAL_PEOPLE_NAMED(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d+|a couple of|a few|both|the pair)\\s+(?P<counted>"), &g_PEOPLE_MANY()), &py_str(")\\b|\\b(?:my|our|the|his|her|their)\\s+(?P<group>")), &g_PEOPLE_MANY()), &py_str(")\\b|\\b(?P<bare>")), &g_PEOPLE_MANY()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.TWO_OF_KIN"); }
    s_TWO_OF_KIN(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:my|our|the|a|an)\\s+(?P<a>"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[pyrt::xrefs::xget("memory.facts", "KIN_WORDS")]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\s+and\\s+(?:my|our|the|a|an)?\\s*(?P<b>")), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[pyrt::xrefs::xget("memory.facts", "KIN_WORDS")]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._THE_YOUNGER_END"); }
    s__THE_YOUNGER_END(call_builtin("frozenset", &[py_tuple(vec![py_str("youngest"), py_str("littlest"), py_str("baby"), py_str("toddler"), py_str("newborn"), py_str("little one"), py_str("wee one"), py_str("little lad"), py_str("little girl"), py_str("little boy"), py_str("small one"), py_str("younger one")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._THE_OLDER_END"); }
    s__THE_OLDER_END(call_builtin("frozenset", &[py_tuple(vec![py_str("eldest"), py_str("oldest"), py_str("elder"), py_str("big one"), py_str("older one"), py_str("biggest")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._A_PLACE_ASK"); }
    s__A_PLACE_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:the|my|our|your)\\s+(?P<who>"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&g__THE_YOUNGER_END(), &g__THE_OLDER_END())); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("now"), py_str("then"), py_str("there"), py_str("too"), py_str("though"), py_str("anyway"), py_str("please"), py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("again"), py_str("still"), py_str("yet"), py_str("today"), py_str("nowadays"), py_str("here"), py_str("of"), py_str("in"), py_str("at"), py_str("on"), py_str("and"), py_str("or"), py_str("got"), py_str("has"), py_str("have"), py_str("had"), py_str("will"), py_str("would"), py_str("should"), py_str("can"), py_str("could"), py_str("might"), py_str("must"), py_str("turns"), py_str("turned"), py_str("just"), py_str("only"), py_str("already"), py_str("actually"), py_str("really")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._THE_PLACE_STANDS_ALONE"); }
    s__THE_PLACE_STANDS_ALONE(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.PET_ASK"); }
    s_PET_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|the)\\s+("), &pyrt::xrefs::xget("memory.store_lines", "PET_WORDS")), &py_str(")s?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.BREED_ASK"); }
    s_BREED_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|the)\\s+(?:"), &gattr(&pyrt::xrefs::xget("understanding.breeds", "BREED_RE"), "pattern")), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations._PET_WORD"); }
    s__PET_WORD(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("\\b(?:"), &pyrt::xrefs::xget("memory.store_lines", "PET_WORDS")), &py_str(")s?\\b|")), &gattr(&pyrt::xrefs::xget("understanding.breeds", "BREED_RE"), "pattern")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.PLURAL_ANIMAL"); }
    s_PLURAL_ANIMAL(stdlib_call("re", "compile", &[py_str("s$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.MANY_OF_THEM"); }
    s_MANY_OF_THEM(py_dict(vec![(py_str("child"), py_str("children")), (py_str("grandchild"), py_str("grandchildren")), (py_str("baby"), py_str("babies")), (py_str("person"), py_str("people")), (py_str("woman"), py_str("women")), (py_str("man"), py_str("men"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.relations.ALREADY_MANY"); }
    s_ALREADY_MANY(call_builtin("frozenset", &[mcall(&g_MANY_OF_THEM(), "values", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _folded(args: &[Py]) -> Py {
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
    let mut folded: Py = Py::None;
    let mut low: Py = Py::None;
    let mut seen: Py = Py::None;
    let _ = py_str("The line with chat-speak folded to plain words (\"whats my name\" -> \"what is my name\"),\n    from the understanding engine when it has read this very line, else read here.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return py_str("");
    }
    seen = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[seen.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&seen.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &low.clone())) } }) {
    folded = { let __b = mcall(&seen.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&folded.clone()) {
    return mcall(&mcall(&g_CHAT_TAIL(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&folded.clone(), "replace", &[py_str(" | "), py_str(" ")]), "lower", &[]), "split", &[])])]), "strip", &[py_str(" ,.;:!?")]);
    }
    }
    let __try1 = exc::catch(|| {
    folded = { let __b = mcall(&pyrt::xrefs::xcall("understanding.normalize", "normalise", &[text.clone()]), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    folded = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return mcall(&mcall(&g_CHAT_TAIL(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&{ let __b = folded.clone(); if py_truthy(&__b) { __b } else { low.clone() } }, "replace", &[py_str(" | "), py_str(" ")]), "lower", &[]), "split", &[])])]), "strip", &[py_str(" ,.;:!?")]);
    Py::None
}
pub fn _person_from_pronoun(args: &[Py]) -> Py {
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
    let mut named_here: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named_here") { v.clone() }
    else { py_tuple(vec![]) };
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut low: Py = Py::None;
    let mut people: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Who a bare \"she\"/\"he\" in this line is: the last person named whose pronoun fits.\n\n    \"my mum is coming on sunday\" then \"she doesnt eat meat\" is a fact about the mum, not about\n    nobody: without this the diet is kept against \"\", and the next \"what did i say about my mum\"\n    has nothing to find.\n    ");
    low = mcall(&text.clone(), "lower", &[]);
    people = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut who: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[who.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(who.clone()); } } py_list(__out) })() };
    let __aug1 = op_add(&people.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&mcall(&entry.clone(), "get", &[py_str("who")])) {  __out.push(call_builtin("str", &[mcall(&entry.clone(), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() });
    people = __aug1;
    people = op_add(&call_builtin("list", &[named_here.clone()]).unwrap_or_else(|e| exc::raise(e)), &people.clone());
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("she|her"), pyrt::xrefs::xget("memory.facts", "SHE")]), py_tuple(vec![py_str("he|his"), pyrt::xrefs::xget("memory.facts", "HE")])]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    pronoun = __unpk3[0].clone();
    fits = __unpk3[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:"), &pronoun.clone()), &py_str(")(?:'s|s)?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter4 = py_iter_or_raise(&_newest_first(&[people.clone(), memory.clone()]));
    for __item in __iter4 {
    who = __item.clone();
    if py_truthy(&{ let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&fits.clone(), &who.clone())) } }) {
    continue;
    }
    if py_truthy(&_pronoun_fits(&[who.clone(), fits.clone()])) {
    return who.clone();
    }
    }
    return py_str("");
    }
    }
    people = _newest_first(&[people.clone(), memory.clone()]);
    return if py_truthy(&people.clone()) { geti(&people.clone(), &Py::Int(0)) } else { py_str("") };
    Py::None
}
pub fn _gender_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut gender: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut root: Py = Py::None;
    let mut roots: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The gender a person word carries, or \"\" when it carries none (\"partner\", \"kid\").");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[-\\s]+"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&op_add(&py_list(vec![word.clone()]), &parts.clone()));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "SHE"), &part.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "HE"), &part.clone())) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "SHE"), &part.clone()))) {
    return py_str("she");
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "HE"), &part.clone()))) {
    return py_str("he");
    }
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![pyrt::xrefs::xget("memory.facts", "SHE"), py_str("she")]), py_tuple(vec![pyrt::xrefs::xget("memory.facts", "HE"), py_str("he")])]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    roots = __unpk3[0].clone();
    gender = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&roots.clone()) { let mut root: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[root.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(root.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    root = __item.clone();
    if py_truthy(&mcall(&word.clone(), "endswith", &[root.clone()])) {
    return gender.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _pronoun_fits(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut fits: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fits") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fits"))) };
    let mut gender: Py = Py::None;
    let _ = py_str("Whether a pronoun of that gender can point at this person.\n\n    A word that says no gender either way (\"partner\", \"kid\") can be either, so it is reachable by\n    both pronouns; a word that names one gender is only that one, whether the whole word names it\n    (\"mum\") or only its ending does (\"stepmum\", \"granddad\").\n    ");
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&fits.clone(), &who.clone()))) {
    return Py::Bool(true);
    }
    gender = _gender_of(&[who.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&gender.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_is(&if py_truthy(&Py::Bool(py_eq(&gender.clone(), &py_str("she")))) { pyrt::xrefs::xget("memory.facts", "SHE") } else { pyrt::xrefs::xget("memory.facts", "HE") }, &fits.clone()));
    Py::None
}
pub fn _newest_first(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut people: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "people") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: people"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut entry: Py = Py::None;
    let mut order: Py = Py::None;
    let mut text: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The people, the one named last in the chat first, each kept once.\n\n    \"my son is allergic to nuts\" / \"my wife is vegetarian\" / \"my husband is diabetic\" makes\n    three people, and a bare \"he\" in the next line is about the husband. The list is not in\n    that order on its own: the people the store keeps a fact for are added after the list of\n    names, so a stale \"son\" sat after a fresh \"husband\" and \"is he allergic to anything\" was\n    answered from the son. The order of the facts is the order they were said in, so the newest\n    mention of each person comes first; a person named twice takes their newest place.\n    ");
    order = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter2 = py_iter_or_raise(&people.clone());
    for __item in __iter2 {
    who = __item.clone();
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&order.clone(), &who.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    let _ = mcall(&order.clone(), "append", &[who.clone()]);
    }
    }
    }
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[people.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    who = __item.clone();
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&order.clone(), &who.clone())) } }) {
    let _ = mcall(&order.clone(), "append", &[who.clone()]);
    }
    }
    return order.clone();
    Py::None
}
pub fn _named_person(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut chained: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut relation: Py = Py::None;
    let _ = py_str("The relation a line names as a person, or \"\" when it names their thing.\n\n    A wrong answer is worse than the shrug it replaces, so \"how old is my sons school\" and \"how\n    old is my mums car\" name nobody: the possessive takes the word away from the person.\n\n    The family words people use are wider than the ones the store's oldest list names - a stepson\n    and a grandson are their own people - so every word the store can file is a word an ask can\n    name. One of those wider words only counts when the chat was told about that person: with the\n    store empty, \"how old are my grandparents\" is not \"you haven't told me how old your grandparent\n    is\", because the line gave the ages itself and the plural is a group the store does not keep.\n    ");
    chained = pyrt::xrefs::xcall("memory.facts", "a_chained_person", &[low.clone()]);
    if py_truthy(&chained.clone()) {
    return pyrt::xrefs::xcall("memory.facts", "a_person_word", &[chained.clone()]);
    }
    let __iter1 = py_iter_or_raise(&op_add(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &pyrt::xrefs::xget("memory.facts", "ALONE_PERSON")));
    for __item in __iter1 {
    relation = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&py_str("\\b"), &relation.clone()), &py_str("(?:\\s+in\\s+laws?)?(?:'s|s)?\\b")), low.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PERSON_AFTER(), "match", &[slice_or_raise(&low.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &relation.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_person_in_the_store(&[memory.clone(), relation.clone()]))) } } } }) {
    continue;
    }
    name = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]), "replace", &[py_str("'s"), py_str("")]), "split", &[])]);
    name = stdlib_call("re", "sub", &[py_str("\\bin laws$"), py_str("in law"), name.clone()]).unwrap_or_else(|e| exc::raise(e));
    name = if py_truthy(&Py::Bool(!py_truthy(&mcall(&name.clone(), "endswith", &[py_str("law")])))) { stdlib_call("re", "sub", &[py_str("s$"), py_str(""), name.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { name.clone() };
    return if py_truthy(&Py::Bool(py_eq(&name.clone(), &relation.clone()))) { relation.clone() } else { name.clone() };
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_person_in_the_store(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut entry: Py = Py::None;
    let mut person: Py = Py::None;
    let _ = py_str("Whether the chat was ever told about this person, by the word the line used.");
    who = mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &who.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut person: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[person.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }, &who.clone()));
    Py::None
}
pub fn _continues_an_about_ask(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut here: Py = Py::None;
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut users: Py = Py::None;
    let _ = py_str("True when the line is another person named after an ask about a person.\n\n    The ask of the turn before is the one being continued, so \"and my dad\" reads as \"what do you\n    know about my dad\" and not as the age question the figure in the earlier line suggests.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ANOTHER_PERSON(), "match", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_named_person(&[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    users = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_every_line(&[memory.clone()])) { let mut turn: Py = __item.clone(); if py_truthy(&mcall(&turn.clone(), "get", &[py_str("user")])) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    here = mcall(&line.clone(), "lower", &[]);
    while py_truthy(&{ let __b = users.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&geti(&users.clone(), &op_neg(&Py::Int(1))), "strip", &[]), "lower", &[]), &here.clone())) } }) {
    let _ = mcall(&users.clone(), "pop", &[]);
    }
    return call_builtin("bool", &[{ let __b = users.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ABOUT_PERSON_ASK(), "search", &[mcall(&geti(&users.clone(), &op_neg(&Py::Int(1))), "lower", &[])]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_word_the_store_knows(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut entry: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("Whether the word is the name of a person or an animal this chat was told about.");
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &word.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut person: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[person.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }, &word.clone()));
    Py::None
}
pub fn _age_is_about_a_person(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut adverb: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut copula: Py = Py::None;
    let mut determiner: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether an age ask is about somebody in the chat rather than about the world.\n\n    A rule ask names children or a relation too - \"what age do kids stop napping\", \"how old should\n    a kitten be to leave its mum\" - but the age belongs to the rule or to the animal, and the\n    store has nothing to recall about it. The person has to be what the ask asks, so the answer is\n    left to the modules that know the world; the store's own recall asks are untouched.\n    ");
    if py_truthy(&geti(&_the_place_end(&[low.clone()]), &Py::Int(1))) {
    return Py::Bool(true);
    }
    ask = mcall(&g_AGE_ASK(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.facts", "a_chained_person", &[low.clone()])) {
    return Py::Bool(true);
    }
    rest = mcall(&slice_or_raise(&low.clone(), &mcall(&ask.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    while py_truthy(&Py::Bool(true)) {
    adverb = mcall(&g_AGE_ADVERB(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&adverb.clone()))) {
    break;
    }
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&adverb.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    rule = mcall(&g_AGE_RULE_WORD(), "match", &[rest.clone()]);
    if py_truthy(&rule.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AGE_YOU_SAY(), "match", &[mcall(&slice_or_raise(&rest.clone(), &mcall(&rule.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])])))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&rule.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    copula = mcall(&g_AGE_COPULA(), "match", &[rest.clone()]);
    if py_truthy(&copula.clone()) {
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&copula.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    determiner = mcall(&g_AGE_DETERMINER(), "match", &[rest.clone()]);
    if py_truthy(&determiner.clone()) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &mcall(&mcall(&determiner.clone(), "group", &[Py::Int(0)]), "lower", &[])))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&determiner.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    word = stdlib_call("re", "match", &[py_str("[a-z']+"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    subject = mcall(&word.clone(), "group", &[Py::Int(0)]);
    if py_truthy(&{ let __b = mcall(&g_AGE_PERSON_WORD(), "match", &[subject.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &mcall(&subject.clone(), "lower", &[]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__PET_WORD(), "fullmatch", &[subject.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PET_WORD(), "fullmatch", &[mcall(&subject.clone(), "rstrip", &[py_str("s")])]) } }) {
    return Py::Bool(true);
    }
    return _a_word_the_store_knows(&[memory.clone(), subject.clone()]);
    Py::None
}
pub fn _tidy_name(args: &[Py]) -> Py {
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
    let _ = py_str("\"rufus\" is the name they mean, and a name reads \"Rufus\" at the start of a sentence.");
    value = mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,!?;:")]);
    return if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[]) } }) { op_add(&mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { value.clone() };
    Py::None
}
pub fn _same_as_last(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this exact sentence was the bot's line in the turn just before.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&turn.clone(), "get", &[py_str("bot")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return Py::Bool(py_eq(&mcall(&said.clone(), "strip", &[]), &mcall(&call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _animal_family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut animal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The animal a pet word belongs to: a puppy is a dog, a kitten is a cat.");
    word = mcall(&call_builtin("str", &[{ let __b = animal.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return mcall(&py_dict(vec![(py_str("puppy"), py_str("dog")), (py_str("dog"), py_str("dog")), (py_str("kitten"), py_str("cat")), (py_str("cat"), py_str("cat")), (py_str("bunny"), py_str("rabbit")), (py_str("rabbit"), py_str("rabbit")), (py_str("goldfish"), py_str("fish")), (py_str("fish"), py_str("fish")), (py_str("pony"), py_str("horse")), (py_str("horse"), py_str("horse")), (py_str("bird"), py_str("parrot")), (py_str("parrot"), py_str("parrot"))]), "get", &[word.clone(), word.clone()]);
    Py::None
}
pub fn _a(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])))) { py_str("an ") } else { py_str("a ") }, &word.clone());
    Py::None
}
pub fn _their_taste(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut dislikes: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut how: Py = Py::None;
    let mut item: Py = Py::None;
    let mut known: Py = Py::None;
    let mut likes: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("What a person the chat knows likes, said back for an ask about it.\n\n    Only a fact the store holds is answered, the same rule as a name or an age, and only for a\n    person it can name: \"what does she like\" with nobody pointed at stays free for whoever can\n    answer it. The verb comes from the ask when the ask has one (\"does my mum like gardening\")\n    and from the fact when it does not (\"what does my mum like\").\n    ");
    said = Py::None;
    how = py_str("");
    thing = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_THEIR_TASTE_A_YES_NO(), g_THEIR_TASTE_WHAT(), g_THEIR_TASTE_INTO()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    said = match_.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    let __iter2 = py_iter_or_raise(&py_tuple(vec![g_THEIR_TASTE_NOT(), g_THEIR_TASTE_NOT_INTO()]));
    for __item in __iter2 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    said = match_.clone();
    how = py_str("not like");
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    return Py::None;
    }
    how = { let __b = how.clone(); if py_truthy(&__b) { __b } else { mcall(&{ let __b = mcall(&mcall(&said.clone(), "groupdict", &[]), "get", &[py_str("how")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]) } };
    thing = mcall(&mcall(&{ let __b = mcall(&mcall(&said.clone(), "groupdict", &[]), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    who = mcall(&{ let __b = mcall(&mcall(&said.clone(), "groupdict", &[]), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    pronoun = mcall(&{ let __b = mcall(&mcall(&said.clone(), "groupdict", &[]), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    known = Py::None;
    if py_truthy(&who.clone()) {
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), &who.clone()))) {  __out.push(entry.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    } else {
    if py_truthy(&pronoun.clone()) {
    fits = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("she"), py_str("her")]), &pronoun.clone()))) { pyrt::xrefs::xget("memory.facts", "SHE") } else { pyrt::xrefs::xget("memory.facts", "HE") };
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[pyrt::xrefs::xcall("memory", "others", &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&fits.clone(), &mcall(&{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&entry.clone(), "get", &[py_str("likes")]); if py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("dislikes")]) } } } }) {  __out.push(entry.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_is(&known.clone(), &Py::None))) {
    who = mcall(&{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&known.clone(), &Py::None))) {
    return Py::None;
    }
    likes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("likes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    dislikes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("dislikes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&thing.clone()) {
    let __iter3 = py_iter_or_raise(&likes.clone());
    for __item in __iter3 {
    item = __item.clone();
    if py_truthy(&_same_thing(&[item.clone(), thing.clone()])) {
    return _they_like_sentence(&[who.clone(), py_list(vec![item.clone()]), py_str("yes"), pronoun.clone()]);
    }
    }
    let __iter4 = py_iter_or_raise(&dislikes.clone());
    for __item in __iter4 {
    item = __item.clone();
    if py_truthy(&_same_thing(&[item.clone(), thing.clone()])) {
    return _they_like_sentence(&[who.clone(), py_list(vec![item.clone()]), py_str("no"), pronoun.clone()]);
    }
    }
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("hate"), py_str("dislike"), py_str("not like")]), &how.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&dislikes.clone()))) {
    if py_truthy(&likes.clone()) {
    said = _they_like_sentence(&[who.clone(), likes.clone(), py_str("yes"), pronoun.clone()]);
    return op_add(&op_add(&py_str("Nothing now — "), &mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return op_add(&op_add(&py_str("You haven't told me what your "), &who.clone()), &py_str(" doesn't like yet."));
    }
    return _they_like_sentence(&[who.clone(), dislikes.clone(), py_str("no"), pronoun.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&likes.clone()))) {
    return op_add(&op_add(&py_str("You haven't told me what your "), &who.clone()), &py_str(" likes yet."));
    }
    return _they_like_sentence(&[who.clone(), likes.clone(), py_str("yes"), pronoun.clone()]);
    Py::None
}
pub fn _same_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let _ = py_str("Whether two ways of naming a taste are the same thing (\"cooking\", \"cook\").");
    one = mcall(&mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    other = mcall(&mcall(&call_builtin("str", &[{ let __b = other.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&other.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &other.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_eq(&py_call(&pyrt::xrefs::xget("memory", "stem"), &[geti(&mcall(&one.clone(), "split", &[]), &op_neg(&Py::Int(1)))]), &py_call(&pyrt::xrefs::xget("memory", "stem"), &[geti(&mcall(&other.clone(), "split", &[]), &op_neg(&Py::Int(1)))])));
    Py::None
}
pub fn _plural_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    who = mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return { let __b = Py::Bool(py_contains(&g__PLURAL(), &who.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("boss"), py_str("christmas")]), &who.clone())) } } } };
    Py::None
}
pub fn _they_like_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut things: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "things") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: things"))) };
    let mut how: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how"))) };
    let mut pronoun: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pronoun") { v.clone() }
    else { py_str("") };
    let mut head: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut some: Py = Py::None;
    let _ = py_str("What they like, said back short: \"Your sister loves cooking.\" / \"Your son doesn't like\n    mushrooms.\" The person comes first when the store can name them, and the pronoun they used\n    when it cannot.\n    ");
    listed = mcall(&py_str(" and "), "join", &[slice_or_raise(&things.clone(), &Py::None, &Py::Int(2), &Py::None)]);
    if py_truthy(&who.clone()) {
    head = op_add(&op_add(&py_str("Your "), &who.clone()), &py_str(" "));
    some = _plural_person(&[who.clone()]);
    } else {
    head = if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("they")))) { py_str("They ") } else { if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("she")))) { py_str("She ") } else { py_str("He ") } };
    some = Py::Bool(py_eq(&pronoun.clone(), &py_str("they")));
    }
    if py_truthy(&Py::Bool(py_eq(&how.clone(), &py_str("no")))) {
    return op_add(&op_add(&op_add(&head.clone(), &if py_truthy(&some.clone()) { py_str("don't like ") } else { py_str("doesn't like ") }), &listed.clone()), &py_str("."));
    }
    return op_add(&op_add(&op_add(&head.clone(), &if py_truthy(&some.clone()) { py_str("love ") } else { py_str("loves ") }), &listed.clone()), &py_str("."));
    Py::None
}
pub fn _their_job(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut job: Py = Py::None;
    let mut known: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut who: Py = Py::None;
    let mut who_kept: Py = Py::None;
    let _ = py_str("What a person the chat knows does, said back for an ask about it.\n\n    Only a role the store holds is answered, the same rule as a name or an age: \"what does my\n    sister do\" with nothing kept about a sister stays free for whoever can answer it. The\n    person is the one the ask names, or the one a pronoun points at.\n    ");
    match_ = mcall(&g_THEIR_JOB(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    groups = mcall(&match_.clone(), "groupdict", &[]);
    who = mcall(&{ let __b = mcall(&groups.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&groups.clone(), "get", &[py_str("who2")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&groups.clone(), "get", &[py_str("who3")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "lower", &[]);
    pronoun = mcall(&{ let __b = mcall(&groups.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&groups.clone(), "get", &[py_str("pronoun2")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&groups.clone(), "get", &[py_str("pronoun3")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "lower", &[]);
    known = Py::None;
    if py_truthy(&who.clone()) {
    known = _a_kept_person(&[memory.clone(), who.clone()]);
    if py_truthy(&Py::Bool(!py_is(&known.clone(), &Py::None))) {
    who = mcall(&{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    }
    } else {
    if py_truthy(&{ let __b = pronoun.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&pronoun.clone(), &py_str("they"))) } }) {
    fits = if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("she")))) { pyrt::xrefs::xget("memory.facts", "SHE") } else { pyrt::xrefs::xget("memory.facts", "HE") };
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[pyrt::xrefs::xcall("memory", "others", &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&fits.clone(), &mcall(&{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]))); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("job")]) } }) {  __out.push(entry.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_is(&known.clone(), &Py::None))) {
    who = mcall(&{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&known.clone(), &Py::None))) {
    who_kept = { let __b = call_builtin("bool", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pronoun.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "KIN"), &who.clone())); if py_truthy(&__b) { __b } else { op_bitand(&_kin_forms(&[who.clone()]), &call_builtin("set", &[pyrt::xrefs::xget("memory.facts", "KIN")]).unwrap_or_else(|e| exc::raise(e))) } } } } } };
    if py_truthy(&who_kept.clone()) {
    return op_add(&op_add(&py_str("You haven't told me what your "), &who.clone()), &py_str(" does yet."));
    }
    return Py::None;
    }
    job = mcall(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("job")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&job.clone()))) {
    return if py_truthy(&who.clone()) { op_add(&op_add(&py_str("You haven't told me what your "), &who.clone()), &py_str(" does yet.")) } else { Py::None };
    }
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { _plural_person(&[who.clone()]) } }) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Your "), &who.clone()), &py_str(" work as ")), &_a(&[job.clone()])), &py_str("."));
    }
    if py_truthy(&who.clone()) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Your "), &who.clone()), &py_str(" is ")), &_a(&[job.clone()])), &py_str("."));
    }
    return op_add(&op_add(&if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("she")))) { py_str("She's ") } else { py_str("He's ") }, &_a(&[job.clone()])), &py_str("."));
    Py::None
}
pub fn _kin_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut bare: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The relation words a possessive ask can name: \"sisters\" and \"sister's\" are \"sister\".\n\n    \"whats my sisters job\" puts the possessive on the relation word, so the word the store\n    holds is the same word without its ending.\n    ");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    forms = py_set(vec![word.clone()]);
    bare = stdlib_call("re", "sub", &[py_str("(?:'s|s'|s)$"), py_str(""), word.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&bare.clone(), &word.clone())) } }) {
    let _ = mcall(&forms.clone(), "add", &[bare.clone()]);
    }
    let _ = mcall(&forms.clone(), "add", &[op_add(&word.clone(), &py_str("s"))]);
    return forms.clone();
    Py::None
}
pub fn _a_kept_person(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut entry: Py = Py::None;
    let mut forms: Py = Py::None;
    let _ = py_str("The entry the store holds for the person an ask names, if it holds one.");
    forms = _kin_forms(&[who.clone()]);
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&forms.clone(), &mcall(&{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])))) {
    return entry.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _same_name_however_they_wrote_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut old: Py = Py::None;
    let _ = py_str("Whether the line names the person, under the name they have now or the one put right.\n\n    \"my daughter is called lily\" then \"its lilly not lily\" keeps Lilly, and the ask after it -\n    \"how old is lily\" - is about the same daughter: a correction puts a name right, it does not\n    make the person the chat already knows a stranger to the next question.\n    ");
    if py_truthy(&_same_name(&[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }, line.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut old: Py = __item.clone(); __out.push(_same_name(&[old.clone(), line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _same_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut known: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut folded: Py = Py::None;
    let mut token: Py = Py::None;
    let mut tokens: Py = Py::None;
    let _ = py_str("Whether the person the store knows is the one the line names: \"how old is janet\".\n\n    The line arrives folded, so \"ann\" is \"an\" by the time it gets here while the store keeps the\n    name as it was said. Both sides are folded and compared, which is the only way an ask by a\n    short name reaches the person it belongs to.\n    ");
    known = mcall(&mcall(&call_builtin("str", &[{ let __b = known.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&known.clone()))) {
    return Py::Bool(false);
    }
    tokens = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&tokens.clone());
    for __item in __iter1 {
    token = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &known.clone()))) {
    return Py::Bool(true);
    }
    folded = _folded(&[token.clone()]);
    if py_truthy(&Py::Bool(py_eq(&folded.clone(), &_folded(&[known.clone()])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _more_than_one(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let _ = py_str("Whether the word for the person takes \"are\": \"my kids are 4 and 7\", \"my mum is 72\".\n\n    \"boss\" ends in an s and is one person, and one rule for it lives in `_plural_person`:\n    \"Your boss are called Dave\" is not a sentence anybody says.\n    ");
    return _plural_person(&[who.clone()]);
    Py::None
}
pub fn _canon_relation(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("The relation a word names, in the word the store files it under: \"mother\" is \"mum\".");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,")]), "lower", &[]);
    word = mcall(&pyrt::xrefs::xget("memory.store_lines", "RELATION_ALIKE"), "get", &[word.clone(), word.clone()]);
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.store_lines", "RELATION_ALIKE"), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return geti(&pyrt::xrefs::xget("memory.store_lines", "RELATION_ALIKE"), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None));
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS"), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _relations_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut canon: Py = Py::None;
    let mut found: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The relations a stored word for people stands for: \"kids\" is every child relation.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+and\\s+|\\s+|,"), mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    canon = _canon_relation(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&canon.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids")]), &canon.clone()))) {
    let _ = mcall(&found.clone(), "extend", &[pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS")]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&canon.clone(), &py_str("parent"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&canon.clone(), &py_str("parents"))) } }) {
    let _ = mcall(&found.clone(), "extend", &[geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("parents"))]);
    } else {
    let _ = mcall(&found.clone(), "append", &[canon.clone()]);
    }
    }
    }
    return call_builtin("tuple", &[found.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _in_the_group(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut group: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "group") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: group"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether a relation the store holds is one of the people the ask named.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_relations_of(&[who.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&group.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_list_of_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Three names read \"Jack, Emma and Ada\", not \"Jack and Emma and Ada\".");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return mcall(&py_str(" and "), "join", &[said.clone()]);
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _mentioned_the_group(args: &[Py]) -> Py {
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
    let mut group: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "group") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: group"))) };
    let mut word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut held: Py = Py::None;
    let mut person: Py = Py::None;
    let _ = py_str("Whether the chat was told the group exists, even with no name filed for any of them.\n\n    \"i have two children\" then \"what are my children called\" is a chat that knows there are two\n    of them: the honest answer is about the name, not \"you haven't told me about your children\".\n    ");
    held = pyrt::xrefs::xcall("memory", "user", &[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("children"), py_str("kids"), py_str("childs")]), &word.clone())); if !py_truthy(&__b) { __b } else { mcall(&held.clone(), "get", &[py_str("kids")]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut person: Py = __item.clone(); __out.push(_in_the_group(&[person.clone(), group.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return _the_group_was_said(&[word.clone(), memory.clone()]);
    Py::None
}
pub fn _whether_they_have(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut asked: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut count: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut empty: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut held: Py = Py::None;
    let mut hinted: Py = Py::None;
    let mut how_many: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut none_of_them: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_them: Py = Py::None;
    let mut said_without_a_name: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether they have any of a kind of person, answered from the people on file.\n\n    \"do i have any brothers or sisters\" is a question about the chat's own family, so a search\n    for it is never the answer: the store says who it has been told about, and when the ask names\n    two kinds of person and only one is on file it says which one it has nothing on. The answer\n    is what was said and no more - it does not guess that a missing brother does not exist.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[low.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    match_ = mcall(&g_HAVE_ANY(), "match", &[mcall(&low.clone(), "strip", &[py_str(" ,.!?;:")])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s*(?:,|\\bor\\b|\\band\\b)\\s*"), mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(entry.clone()); } } py_list(__out) })() };
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    counts = py_dict(vec![]);
    found = py_list(vec![]);
    said_without_a_name = py_list(vec![]);
    empty = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    group = if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("family")))) { op_add(&op_add(&op_add(&pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS"), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("parents"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("siblings"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("grandparents"))) } else { { let __b = mcall(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![_canon_relation(&[word.clone()])]) } } };
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&_in_the_group(&[mcall(&entry.clone(), "get", &[py_str("who")]), group.clone()])) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&mine.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![word.clone(), geti(&mine.clone(), &Py::Int(0))])]);
    continue;
    }
    hinted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut one: Py = __item.clone(); if py_truthy(&_in_the_group(&[one.clone(), group.clone()])) {  __out.push(_canon_relation(&[one.clone()])); } } py_list(__out) })() };
    count = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("kids")]);
    if py_truthy(&hinted.clone()) {
    let _ = mcall(&said_without_a_name.clone(), "extend", &[hinted.clone()]);
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[count.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[count.clone(), py_func("bool", |args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&count.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { _in_the_group(&[py_str("child"), group.clone()]) } } } } } }) {
    let _ = mcall(&said_without_a_name.clone(), "append", &[word.clone()]);
    let _ = py_setitem(&counts.clone(), &word.clone(), count.clone());
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[count.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&count.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { _in_the_group(&[py_str("child"), group.clone()]) } } } }) {
    word = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("kids"), py_str("children"), py_str("childs")]), &word.clone()))) { word.clone() } else { py_str("child") };
    let _ = mcall(&said_without_a_name.clone(), "append", &[word.clone()]);
    let _ = py_setitem(&counts.clone(), &word.clone(), count.clone());
    continue;
    }
    let _ = mcall(&empty.clone(), "append", &[word.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said_without_a_name.clone())) } }) {
    asked = _a_plural_list(&[words.clone()]);
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You haven't told me about "), &_a_or_any(&[words.clone()])), &py_str(" yet.")), op_add(&op_add(&py_str("Nothing on your "), &asked.clone()), &py_str(" yet - tell me and I'll keep it."))]), memory.clone()]);
    }
    said = py_list(vec![]);
    bare = call_builtin("list", &[mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[said_without_a_name.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&found.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    word = __unpk3[0].clone();
    entry = __unpk3[1].clone();
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { word.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("names")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    let _ = mcall(&said.clone(), "append", &[op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &py_str(" are called ")), &_a_list_of_names(&[names.clone()]))]);
    } else {
    if py_truthy(&name.clone()) {
    let _ = mcall(&said.clone(), "append", &[op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &py_str(" ")), &_called(&[name.clone()]))]);
    } else {
    if py_truthy(&Py::Bool(!py_contains(&bare.clone(), &who.clone()))) {
    let _ = mcall(&bare.clone(), "append", &[who.clone()]);
    }
    }
    }
    }
    line = py_str("");
    if py_truthy(&said.clone()) {
    line = op_add(&py_str("You've mentioned "), &_a_list_of_names(&[said.clone()]));
    }
    if py_truthy(&bare.clone()) {
    none_of_them = { let __b = call_builtin("bool", &[counts.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[mcall(&counts.clone(), "values", &[])]).unwrap_or_else(|e| exc::raise(e)))) } };
    how_many = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&counts.clone(), "values", &[])) { let mut value: Py = __item.clone(); if py_truthy(&value.clone()) {  __out.push(value.clone()); } } py_list(__out) })() }, Py::Int(0)]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&none_of_them.clone()) {
    let __aug4 = op_add(&line.clone(), &op_add(&op_add(&op_add(&if py_truthy(&line.clone()) { py_str(" ") } else { py_str("") }, &py_str("You told me you don't have any ")), &_a_list_of_words(&[bare.clone()])), &py_str(".")));
    line = __aug4;
    } else {
    if py_truthy(&how_many.clone()) {
    said_them = op_add(&op_add(&call_builtin("str", &[how_many.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_str(" ")), &geti(&bare.clone(), &Py::Int(0)));
    } else {
    said_them = _a_list_of_words(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bare.clone()) { let mut word: Py = __item.clone(); __out.push(_with_article(&[word.clone()])); } py_list(__out) })() }]);
    }
    names = if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&how_many.clone(), &Py::Int(1)) } }) { py_str("their names") } else { op_add(&mcall(&pyrt::xrefs::xget("memory.store_lines", "HER_WORD"), "get", &[geti(&bare.clone(), &Py::Int(0)), py_str("their")]), &py_str(" name")) };
    let __aug5 = op_add(&line.clone(), &op_add(&op_add(&op_add(&op_add(&op_add(&if py_truthy(&line.clone()) { py_str(" ") } else { py_str("") }, &py_str("You've told me you have ")), &said_them.clone()), &py_str(", but not ")), &names.clone()), &py_str(" yet.")));
    line = __aug5;
    }
    }
    if py_truthy(&empty.clone()) {
    tail = op_add(&op_add(&py_str("no "), &_a_list_of_words(&[empty.clone()])), &py_str(" that you've told me about"));
    if py_truthy(&line.clone()) {
    line = op_add(&op_add(&op_add(&mcall(&line.clone(), "rstrip", &[py_str(".")]), &py_str(" - and ")), &tail.clone()), &py_str("."));
    } else {
    line = op_add(&op_add(&py_str("You haven't mentioned any "), &_a_list_of_words(&[empty.clone()])), &py_str(" yet."));
    }
    }
    return if py_truthy(&mcall(&line.clone(), "endswith", &[py_str(".")])) { line.clone() } else { op_add(&line.clone(), &py_str(".")) };
    Py::None
}
pub fn _a_list_of_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A list of words for people, read \"a sister and a brother\".");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return mcall(&py_str(""), "join", &[said.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_add(&op_add(&geti(&said.clone(), &Py::Int(0)), &py_str(" and ")), &geti(&said.clone(), &Py::Int(1)));
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _more_than_one_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("Whether a word for people is a plural one: \"brothers\", \"children\".");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    return { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("children"), py_str("kids"), py_str("family")]), &word.clone())) } };
    Py::None
}
pub fn _with_article(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("One kind of person, with the article the word takes: \"a brother\", \"any sisters\".");
    word = call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return op_add(&if py_truthy(&_more_than_one_word(&[word.clone()])) { py_str("any ") } else { py_str("a ") }, &word.clone());
    Py::None
}
pub fn _a_or_any(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("An ask about a kind of person reads \"a brother\" or \"any brothers or sisters\".");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return _with_article(&[geti(&said.clone(), &Py::Int(0))]);
    }
    first = geti(&said.clone(), &Py::Int(0));
    rest = mcall(&py_str(" or "), "join", &[slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    return op_add(&op_add(&op_add(&if py_truthy(&_more_than_one_word(&[first.clone()])) { py_str("any ") } else { py_str("a ") }, &first.clone()), &py_str(" or ")), &rest.clone());
    Py::None
}
pub fn _a_plural_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Two or three kinds of person read \"brothers or sisters\", not \"and\".\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&said.clone(), &Py::Int(0));
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_add(&op_add(&geti(&said.clone(), &Py::Int(0)), &py_str(" or ")), &geti(&said.clone(), &Py::Int(1)));
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" or ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _the_group_names_typed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut names: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The names a line of theirs gave for a group, as a list: \"my kids are tom and ada\".\n\n    Only the names they typed are used, taken from the group word, and only when two or three of\n    them are there: one word after \"my kids are\" is as likely to be a state (\"happy\", \"young\") as\n    a name. The last line that gives them wins, because that is the one the chat is on.\n    ");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    match_ = mcall(&g_GROUP_NAMES_TYPED(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    names = _the_names_a_pronoun_gave(&[line.clone()]);
    if py_truthy(&{ let __b = names.clone(); if !py_truthy(&__b) { __b } else { mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("kids")]) } }) {
    said = names.clone();
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_in_the_group(&[mcall(&mcall(&match_.clone(), "group", &[py_str("who")]), "lower", &[]), mcall(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), "get", &[word.clone(), py_tuple(vec![])])])))) {
    continue;
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",|\\band\\b|&"), mcall(&match_.clone(), "group", &[py_str("names")])]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[py_str(" .,")])) {  __out.push(mcall(&one.clone(), "strip", &[py_str(" .,")])); } } py_list(__out) })() };
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = { let __cmp11: Py = call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp11)) { op_le(&__cmp11, &Py::Int(14)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&one.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &mcall(&one.clone(), "lower", &[]))) } } } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __cmp21: Py = call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &Py::Int(3)) } else { Py::Bool(false) } }) {
    said = names.clone();
    }
    }
    return said.clone();
    Py::None
}
pub fn _the_names_a_pronoun_gave(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The names a \"they are called ...\" line gives, as a list, or [].");
    match_ = mcall(&g_THEY_ARE_NAMED(), "search", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_list(vec![]);
    }
    return _the_names_from(&[mcall(&match_.clone(), "group", &[py_str("names")])]);
    Py::None
}
pub fn _the_names_from(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut names: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("Two or three names out of the words that follow \"called\", \"named\" or a comma.\n\n    One word after \"my kids are\" is as likely to be a state (\"happy\", \"young\") as a name, and a\n    word a relative is called (\"my boys and my daughter\") names no one, so only two or three plain\n    words that are nobody's kin word count. One reader, because every reply that says a name back\n    had its own copy of this test and they did not agree.\n    ");
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",|\\band\\b|&"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[py_str(" .,")])) {  __out.push(mcall(&one.clone(), "strip", &[py_str(" .,")])); } } py_list(__out) })() };
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = { let __cmp11: Py = call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp11)) { op_le(&__cmp11, &Py::Int(14)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&one.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &mcall(&one.clone(), "lower", &[]))) } } } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    return if py_truthy(&{ let __cmp11: Py = call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp11)) { op_le(&__cmp11, &Py::Int(3)) } else { Py::Bool(false) } }) { names.clone() } else { py_list(vec![]) };
    Py::None
}
pub fn _the_several_named_before(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut names: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The newest earlier line that named more than one person: (word, names, line).\n\n    \"their\" and \"they\" mean more than one person, so the line they point at has to be one that\n    named more than one. The line is read for the names it gave (\"..., tom and ben\", \"called tom\n    and ada\") whether or not the store filed them, because the store files a person's own name and\n    drops the people named beside them.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[pyrt::xrefs::xcall("memory.history", "the_lines_before", &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.history", "is_their_news", &[line.clone()])))) {
    continue;
    }
    found = mcall(&g_SEVERAL_PEOPLE_NAMED(), "search", &[line.clone()]);
    names = _the_names_from(&[_the_names_tail(&[line.clone()])]);
    pair = if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) { mcall(&g_TWO_OF_KIN(), "search", &[line.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pair.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&names.clone())) } } } }) {
    continue;
    }
    word = mcall(&if py_truthy(&found.clone()) { { let __b = mcall(&found.clone(), "group", &[py_str("counted")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("group")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "group", &[py_str("bare")]) } } } } } else { py_str("") }, "lower", &[]);
    phrase = if py_truthy(&found.clone()) { mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[])]) } else { py_str("") };
    if py_truthy(&pair.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&mcall(&pair.clone(), "group", &[py_str("a")]), "lower", &[]), mcall(&mcall(&pair.clone(), "group", &[py_str("b")]), "lower", &[])]), 2, None);
    one = __unpk2[0].clone();
    other = __unpk2[1].clone();
    word = op_add(&op_add(&one.clone(), &py_str(" and ")), &other.clone());
    phrase = op_add(&op_add(&op_add(&py_str("your "), &one.clone()), &py_str(" and your ")), &other.clone());
    }
    return py_tuple(vec![word.clone(), names.clone(), phrase.clone()]);
    }
    return py_tuple(vec![py_str(""), py_list(vec![]), py_str("")]);
    Py::None
}
pub fn _the_names_tail(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The words after \"called\", \"named\", a comma or the \"are\" of a group, where names go.\n\n    \"my boys are tom and ben\" gave its names with no word to announce them, and the reader that\n    only looked after \"called\" and a comma said the chat had no names for the boys one line after\n    it had given them.\n    ");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    match_ = stdlib_call("re", "search", &[py_str("(?:\\bcalled\\b|\\bnamed\\b|,)\\s*(?P<names>[^.!?]{3,60})$"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    return mcall(&match_.clone(), "group", &[py_str("names")]);
    }
    match_ = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:"), &g_PEOPLE_MANY()), &py_str(")\\s+(?:are|were)\\s+(?P<names>[^.!?]{3,60})$")), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&match_.clone()) { mcall(&match_.clone(), "group", &[py_str("names")]) } else { py_str("") };
    Py::None
}
pub fn _the_names_of_them_all(args: &[Py]) -> Py {
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
    let mut names: Py = Py::None;
    let mut one: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut told: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The names of the several people an earlier line named, for a plural pronoun ask.\n\n    \"what are their names\" leans on the line before it, and the store holds one person from that\n    line and not the people it named: \"my sister sarah has two boys, tom and ben\" files the sister\n    and drops the boys, so answering from the store said \"Your sister is called Sarah.\" - one\n    person, and the wrong one. The line is the antecedent of \"their\", so the answer comes from\n    the line, and with the people named and no names given it says exactly that instead of\n    denying the line that is right there.\n    ");
    let __unpk1 = py_unpack(&_the_several_named_before(&[memory.clone()]), 3, None);
    word = __unpk1[0].clone();
    names = __unpk1[1].clone();
    phrase = __unpk1[2].clone();
    if py_truthy(&names.clone()) {
    return op_add(&_a_list_of_names(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut one: Py = __item.clone(); __out.push(_called(&[one.clone()])); } py_list(__out) })() }]), &py_str("."));
    }
    if py_truthy(&word.clone()) {
    told = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:my|our|the)\\s+")); __args.push({ let memory = memory.clone(); let names = names.clone(); let one = one.clone(); let phrase = phrase.clone(); let told = told.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let memory = memory.clone(); let names = names.clone(); let one = one.clone(); let phrase = phrase.clone(); let told = told.clone(); let word = word.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); py_str("your ") }) }); __args.push(phrase.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { word.clone() } };
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You told me about "), &told.clone()), &py_str(", but not their names - what are they called?")), op_add(&op_add(&py_str("I've got "), &told.clone()), &py_str(" from you, but not their names. Tell me and I'll keep them."))]), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _the_names_they_typed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The names a line of theirs gave for a group, said back: \"Tom and Sam, then - your kids\".");
    said = _the_group_names_typed(&[word.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return op_add(&op_add(&op_add(&_a_list_of_names(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(_called(&[one.clone()])); } py_list(__out) })() }]), &py_str(", then - I'll keep those as your ")), &word.clone()), &py_str("' names."));
    Py::None
}
pub fn _the_names_of_the_group(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut _u: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut at: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut forgotten: Py = Py::None;
    let mut g: Py = Py::None;
    let mut group: Py = Py::None;
    let mut grouped: Py = Py::None;
    let mut held_paired: Py = Py::None;
    let mut held_value: Py = Py::None;
    let mut held_who: Py = Py::None;
    let mut known: Py = Py::None;
    let mut n: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut one: Py = Py::None;
    let mut paired: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The names of more than one person they told us about: \"Your son is called Jack, and ...\"\n\n    A plural ask (\"what are my children called\", \"what are their names\", \"the names of my kids\")\n    is about the people the chat already holds, so it is answered from them and from nowhere\n    else. With the group named and no name on file the honest line is that, never a search for\n    what to call a child; with only a plural pronoun it stays free for whoever can answer it,\n    because \"what are their names\" can be about people the store was never told about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[low.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    ask = mcall(&g_GROUP_NAME_ASK(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    return Py::None;
    }
    word = mcall(&call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("a"), py_str("b"), py_str("c"), py_str("d")])) { let mut g: Py = __item.clone(); if py_truthy(&mcall(&ask.clone(), "group", &[g.clone()])) {  __out.push(mcall(&ask.clone(), "group", &[g.clone()])); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    known = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&word.clone()) {
    group = op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&pyrt::xrefs::xget("memory.store_lines", "CHILD_RELATIONS"), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("parents"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("siblings"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("grandparents"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("grandchildren"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("aunts"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("uncles"))), &geti(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), &py_str("cousins")));
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("family")))) {
    group = mcall(&pyrt::xrefs::xget("memory.store_lines", "GROUP_OF"), "get", &[word.clone(), py_tuple(vec![])]);
    }
    known = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&known.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&_in_the_group(&[mcall(&entry.clone(), "get", &[py_str("who")]), group.clone()])) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&known.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_mentioned_the_group(&[memory.clone(), group.clone(), word.clone()]))) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You haven't told me about your "), &word.clone()), &py_str(" yet.")), op_add(&op_add(&py_str("I've got nothing about your "), &word.clone()), &py_str(" yet - tell me and I'll keep it."))]), memory.clone()]);
    }
    } else {
    if py_truthy(&op_lt(&call_builtin("len", &[known.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    known = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&known.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = _more_than_one(&[mcall(&entry.clone(), "get", &[py_str("who")])]); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("names")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(one.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("int", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    }
    }
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&known.clone(), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    entry = __item.clone();
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    name = mcall(&entry.clone(), "get", &[py_str("name")]);
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("names")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut n: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[n.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[n.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    let _ = mcall(&said.clone(), "append", &[py_tuple(vec![who.clone(), _a_list_of_names(&[names.clone()]), Py::Bool(true)])]);
    } else {
    if py_truthy(&{ let __b = call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { name.clone() } }) {
    let _ = mcall(&said.clone(), "append", &[py_tuple(vec![who.clone(), _called(&[name.clone()]), Py::Bool(false)])]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    forgotten = _the_names_of_them_all(&[memory.clone()]);
    if py_truthy(&forgotten.clone()) {
    return forgotten.clone();
    }
    group = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&known.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&_more_than_one(&[mcall(&entry.clone(), "get", &[py_str("who")])])) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&group.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You haven't told me a name for your "), &group.clone()), &py_str(" yet - what are they called?")), op_add(&op_add(&py_str("I've got your "), &group.clone()), &py_str(" but no names for them - what are they called?"))]), memory.clone()]);
    }
    }
    if py_truthy(&_the_group_was_said(&[word.clone(), memory.clone()])) {
    typed = _the_names_they_typed(&[word.clone(), memory.clone()]);
    if py_truthy(&typed.clone()) {
    return typed.clone();
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("What are your "), &word.clone()), &py_str(" called? I'll keep their names.")), op_add(&op_add(&py_str("Tell me what your "), &word.clone()), &py_str(" are called and I'll keep it."))]), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You haven't told me a name for your "), &word.clone()), &py_str(" yet.")), op_add(&op_add(&py_str("I don't have a name for your "), &word.clone()), &py_str(" yet."))]), memory.clone()]);
    }
    grouped = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    who = __unpk3[0].clone();
    value = __unpk3[1].clone();
    paired = __unpk3[2].clone();
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[grouped.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let mut __else5 = false;
    for __item in __iter4 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    at = __unpk6[0].clone();
    let __unpk7 = py_unpack(&__unpk6[1].clone(), 3, None);
    held_who = __unpk7[0].clone();
    held_value = __unpk7[1].clone();
    held_paired = __unpk7[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&paired.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&held_paired.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&held_who.clone(), &who.clone())) } } } }) {
    let _ = py_setitem(&grouped.clone(), &at.clone(), py_tuple(vec![who.clone(), op_add(&op_add(&held_value.clone(), &py_str("\0")), &value.clone()), paired.clone()]));
    break;
    }
    }
    let _ = mcall(&grouped.clone(), "append", &[py_tuple(vec![who.clone(), value.clone(), paired.clone()])]);
    }
    parts = py_list(vec![]);
    let __iter8 = py_iter_or_raise(&grouped.clone());
    for __item in __iter8 {
    let __unpk9 = py_unpack(&__item.clone(), 3, None);
    who = __unpk9[0].clone();
    value = __unpk9[1].clone();
    paired = __unpk9[2].clone();
    if py_truthy(&paired.clone()) {
    let _ = mcall(&parts.clone(), "append", &[op_add(&op_add(&op_add(&py_str("you told me your "), &who.clone()), &py_str(" are called ")), &value.clone())]);
    } else {
    let _ = mcall(&parts.clone(), "append", &[op_add(&op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &if py_truthy(&_more_than_one(&[who.clone()])) { py_str(" are") } else { py_str(" is") }), &py_str(" called ")), &_a_list_of_names(&[mcall(&value.clone(), "split", &[py_str("\0")])]))]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    one = geti(&parts.clone(), &Py::Int(0));
    return op_add(&op_add(&mcall(&geti(&one.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&one.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let __unpk6836 = py_unpack(&__item.clone(), 3, None); let mut _u: Py = __unpk6836[0].clone(); let mut _u: Py = __unpk6836[1].clone(); let mut paired: Py = __unpk6836[2].clone(); __out.push(paired.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return op_add(&op_add(&op_add(&op_add(&mcall(&geti(&geti(&parts.clone(), &Py::Int(0)), &Py::Int(0)), "upper", &[]), &slice_or_raise(&geti(&parts.clone(), &Py::Int(0)), &Py::Int(1), &Py::None, &Py::None)), &py_str(", and ")), &geti(&parts.clone(), &Py::Int(1))), &py_str("."));
    }
    return op_add(&mcall(&py_str(". "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(op_add(&mcall(&geti(&part.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None))); } py_list(__out) })() }]), &py_str("."));
    Py::None
}
pub fn _did_you_mean(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut property_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "property_") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: property_"))) };
    let mut ask_gender: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_gender") { v.clone() }
    else { py_str("") };
    let mut fact: Py = Py::None;
    let mut gender: Py = Py::None;
    let mut him: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The one person the store holds, offered back as a question rather than stated as a fact.\n\n    The sentence agrees with the person: \"She is 50\", \"They are 50\", \"They turn 61 next april\".\n    When the ask's own pronoun is the other one - \"how old is she\" about the son the store holds -\n    the mismatch is said out loud rather than papered over, because a \"she\" is not the son and\n    answering as though it were is a fact on a person they did not mean.\n    ");
    who = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    gender = _gender_of(&[who.clone()]);
    pronoun = if py_truthy(&Py::Bool(py_eq(&gender.clone(), &py_str("she")))) { py_str("She") } else { if py_truthy(&Py::Bool(py_eq(&gender.clone(), &py_str("he")))) { py_str("He") } else { py_str("They") } };
    plural = Py::Bool(py_eq(&pronoun.clone(), &py_str("They")));
    if py_truthy(&Py::Bool(py_eq(&property_.clone(), &py_str("name")))) {
    verb = if py_truthy(&plural.clone()) { py_str("are called ") } else { py_str("is called ") };
    fact = _called(&[call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    } else {
    if py_truthy(&mcall(&entry.clone(), "get", &[py_str("turns")])) {
    verb = if py_truthy(&plural.clone()) { py_str("turn ") } else { py_str("turns ") };
    fact = op_add(&op_add(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("age")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str(" ")), &call_builtin("str", &[geti(&entry.clone(), &py_str("turns"))]).unwrap_or_else(|e| exc::raise(e)));
    } else {
    verb = if py_truthy(&plural.clone()) { py_str("are ") } else { py_str("is ") };
    fact = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("age")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    if py_truthy(&{ let __b = ask_gender.clone(); if !py_truthy(&__b) { __b } else { { let __b = gender.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&gender.clone(), &ask_gender.clone())) } } } }) {
    him = if py_truthy(&Py::Bool(py_eq(&gender.clone(), &py_str("she")))) { py_str("her") } else { py_str("him") };
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("I've only got your "), &who.clone()), &py_str(" down, and ")), &mcall(&pronoun.clone(), "lower", &[])), &py_str(" ")), &verb.clone()), &fact.clone()), &py_str(". Did you mean ")), &him.clone()), &py_str("?"));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Did you mean your "), &who.clone()), &py_str("? ")), &pronoun.clone()), &py_str(" ")), &verb.clone()), &fact.clone()), &py_str("."));
    Py::None
}
pub fn _relation_reply(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut by_name: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut grouped: Py = Py::None;
    let mut has_fact: Py = Py::None;
    let mut have_any: Py = Py::None;
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named_one: Py = Py::None;
    let mut names: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut property_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut she: Py = Py::None;
    let mut tasted: Py = Py::None;
    let mut w: Py = Py::None;
    let mut wants_age: Py = Py::None;
    let mut wants_name: Py = Py::None;
    let mut who: Py = Py::None;
    let mut words: Py = Py::None;
    let mut worked: Py = Py::None;
    let _ = py_str("A fact about someone they mentioned, said back: \"Your mum is called Janet.\"\n\n    The person is the one the ask names (\"my mum\", \"janet\"), the one it points at with a pronoun\n    (\"how old is she\"), or, when it names nobody, the last person the store heard about. Only a\n    fact the store holds is answered: \"what's my mum's name\" with no name keeps the turn free for\n    whoever can answer it.\n    ");
    tasted = _their_taste(&[memory.clone(), low.clone()]);
    if py_truthy(&tasted.clone()) {
    return tasted.clone();
    }
    worked = _their_job(&[memory.clone(), low.clone()]);
    if py_truthy(&worked.clone()) {
    return worked.clone();
    }
    have_any = _whether_they_have(&[memory.clone(), low.clone()]);
    if py_truthy(&have_any.clone()) {
    return have_any.clone();
    }
    grouped = _the_names_of_the_group(&[memory.clone(), low.clone()]);
    if py_truthy(&grouped.clone()) {
    return grouped.clone();
    }
    wants_name = call_builtin("bool", &[{ let __b = mcall(&g_THEIR_NAME(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAME_AGAIN(), "match", &[low.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    wants_age = call_builtin("bool", &[mcall(&g_THEIR_AGE(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    by_name = py_str("");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = wants_name.clone(); if py_truthy(&__b) { __b } else { wants_age.clone() } }))) {
    return Py::None;
    }
    property_ = if py_truthy(&wants_name.clone()) { py_str("name") } else { py_str("age") };
    who = _named_person(&[low.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&property_.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { { let __b = who.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&who.clone(), "endswith", &[py_str("s")]))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    return Py::None;
    }
    known = Py::None;
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("who")]), &who.clone())) } }) {
    known = entry.clone();
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { _same_name_however_they_wrote_it(&[entry.clone(), low.clone()]) } }) {
    known = entry.clone();
    by_name = { let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } };
    who = { let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } };
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&known.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&property_.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:they|them)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&known.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:she|he|her|his|they|them)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    she = Py::Bool(!py_is(&stdlib_call("re", "search", &[py_str("\\b(?:she|her)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    fits = if py_truthy(&she.clone()) { pyrt::xrefs::xget("memory.facts", "SHE") } else { pyrt::xrefs::xget("memory.facts", "HE") };
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[pyrt::xrefs::xcall("memory", "others", &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    entry = __item.clone();
    has_fact = { let __b = mcall(&entry.clone(), "get", &[property_.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&property_.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("months")]) } } } };
    if py_truthy(&{ let __b = has_fact.clone(); if !py_truthy(&__b) { __b } else { _pronoun_fits(&[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, fits.clone()]) } }) {
    known = entry.clone();
    who = { let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } };
    break;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&known.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:she|he|her|his)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&entry.clone(), "get", &[property_.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&property_.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("months")]) } } } } } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    offered = _did_you_mean(&[geti(&held.clone(), &Py::Int(0)), property_.clone(), _ask_gender(&[low.clone()])]);
    if py_truthy(&offered.clone()) {
    return offered.clone();
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&known.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { who.clone() } }) {
    let __iter3 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter3 {
    entry = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str(" and ")])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[])); } py_list(__out) })() };
    names = { let __b = mcall(&entry.clone(), "get", &[py_str("names")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    grouped = _relations_of(&[mcall(&entry.clone(), "get", &[py_str("who")])]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&grouped.clone(), &_canon_relation(&[who.clone()]))) } } } }) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You told me your "), &call_builtin("str", &[geti(&entry.clone(), &py_str("who"))]).unwrap_or_else(|e| exc::raise(e))), &py_str(" are ")), &_a_list_of_names(&[names.clone()])), &py_str(" - which one is your ")), &who.clone()), &py_str("?"));
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&known.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = wants_name.clone(); if !py_truthy(&__b) { __b } else { who.clone() } } } }) {
    return op_add(&op_add(&py_str("You haven't told me a name for your "), &who.clone()), &py_str(" yet."));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&known.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[property_.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&property_.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { mcall(&known.clone(), "get", &[py_str("name")]) } } } } } }) {
    pronoun = _pronoun_for(&[who.clone(), known.clone()]);
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Your "), &who.clone()), &py_str(" is called ")), &_called(&[call_builtin("str", &[geti(&known.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))])), &py_str(", but you haven't told me how old ")), &pronoun.clone()), &if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("they")))) { py_str(" are.") } else { py_str(" is.") });
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&property_.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { { let __b = known.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&known.clone(), "get", &[py_str("months")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[py_str("age")]))) } } } } } }) {
    pronoun = call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&who.clone()) {
    name = op_add(&py_str("Your "), &who.clone());
    plural = mcall(&who.clone(), "endswith", &[py_str("s")]);
    } else {
    name = mcall(&py_dict(vec![(py_str("she"), py_str("She")), (py_str("he"), py_str("He"))]), "get", &[pronoun.clone(), py_str("They")]);
    plural = Py::Bool(py_contains(&py_tuple(vec![py_str(""), py_str("they")]), &pronoun.clone()));
    }
    return op_add(&op_add(&op_add(&name.clone(), &if py_truthy(&plural.clone()) { py_str(" are ") } else { py_str(" is ") }), &call_builtin("str", &[geti(&known.clone(), &py_str("months"))]).unwrap_or_else(|e| exc::raise(e))), &py_str(" months old."));
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&known.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[property_.clone()]))) } }) {
    return Py::None;
    }
    said = if py_truthy(&Py::Bool(py_eq(&property_.clone(), &py_str("name")))) { _called(&[call_builtin("str", &[geti(&known.clone(), &property_.clone())]).unwrap_or_else(|e| exc::raise(e))]) } else { call_builtin("str", &[geti(&known.clone(), &property_.clone())]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return op_add(&op_add(&if py_truthy(&stdlib_call("re", "search", &[py_str("\\bthey\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_str("Their ") } else { py_str("That's ") }, &said.clone()), &py_str("."));
    }
    if py_truthy(&{ let __b = mcall(&known.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { { let __b = by_name.clone(); if py_truthy(&__b) { __b } else { _same_name(&[call_builtin("str", &[geti(&known.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), low.clone()]) } } } }) {
    named_one = _called(&[call_builtin("str", &[geti(&known.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))]);
    return op_add(&op_add(&op_add(&named_one.clone(), &if py_truthy(&Py::Bool(py_eq(&property_.clone(), &py_str("name")))) { py_str(" is called ") } else { py_str(" is ") }), &said.clone()), &py_str("."));
    }
    mine = op_add(&op_add(&op_add(&py_str("Your "), &who.clone()), &py_str(" ")), &if py_truthy(&_more_than_one(&[who.clone()])) { py_str("are ") } else { py_str("is ") });
    if py_truthy(&Py::Bool(py_eq(&property_.clone(), &py_str("name")))) {
    return op_add(&op_add(&op_add(&mine.clone(), &py_str("called ")), &said.clone()), &py_str("."));
    }
    if py_truthy(&mcall(&known.clone(), "get", &[py_str("turns")])) {
    return _one_age(&[who.clone(), geti(&known.clone(), &py_str("age")), call_builtin("str", &[geti(&known.clone(), &py_str("turns"))]).unwrap_or_else(|e| exc::raise(e)), mcall(&known.clone(), "get", &[py_str("to_come")])]);
    }
    return op_add(&op_add(&mine.clone(), &said.clone()), &py_str("."));
    Py::None
}
pub fn _pronoun_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut known: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { Py::None };
    let mut name: Py = Py::None;
    let mut title: Py = Py::None;
    let _ = py_str("The pronoun a person takes: \"she\", \"he\" or \"they\".\n\n    Most words say it (\"mum\" is a she, \"son\" is a he). A role can be held by anybody - a landlord,\n    a tutor, a boss - and the store had them as both, so the first test won and a landlord called\n    Mr Smith was answered as a \"she\". A title is evidence and wins over the word: Mr is a he.\n    ");
    name = call_builtin("str", &[{ let __b = mcall(&{ let __b = known.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    title = if py_truthy(&{ let __b = Py::Bool(py_contains(&name.clone(), &py_str(" "))); if py_truthy(&__b) { __b } else { name.clone() } }) { mcall(&mcall(&geti(&mcall(&name.clone(), "split", &[py_str(" ")]), &Py::Int(0)), "strip", &[py_str(".")]), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "TITLE_GENDER"), &title.clone()))) {
    return geti(&pyrt::xrefs::xget("memory.facts", "TITLE_GENDER"), &title.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "SHE"), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "HE"), &who.clone())) } }) {
    return py_str("she");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "HE"), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "SHE"), &who.clone())) } }) {
    return py_str("he");
    }
    return py_str("they");
    Py::None
}
pub fn _the_group_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The group word the person used, in the spelling the answer can say out loud.\n\n    \"childrens\" is a slip for \"children\" and the possessive is the same ask (\"what are my\n    childrens names\"), so the word that matched is the one the answer uses, never their typo.\n    ");
    who = mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![who.clone(), if py_truthy(&mcall(&who.clone(), "endswith", &[py_str("es")])) { slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None) } else { py_str("") }, if py_truthy(&mcall(&who.clone(), "endswith", &[py_str("s")])) { slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { py_str("") }, mcall(&pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"), "get", &[who.clone(), py_str("")]), op_add(&who.clone(), &py_str("s"))]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.store_lines", "PERSON_GROUP"), &word.clone())) } }) {
    return word.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_group_was_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut forms: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether any of the person's own lines named this group, in any spelling of it.\n\n    A group the store holds nothing for is either one nobody mentioned (\"what are my parents\n    called\" in a chat that never spoke of them) or one named in a line the readers could not\n    file (\"my kids are called tom and sam\"). Only the first may be answered \"you haven't told\n    me\": the second would deny the line before it.\n    ");
    word = _the_group_word(&[who.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    forms = py_set(vec![word.clone(), op_add(&word.clone(), &py_str("s")), op_add(&word.clone(), &py_str("es")), mcall(&pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"), "get", &[word.clone(), py_str("")])]);
    forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = one.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(one.clone()); } } py_set(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut one: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), said.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _ages_reply(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut asked_plural: Py = Py::None;
    let mut children: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut family: Py = Py::None;
    let mut group: Py = Py::None;
    let mut he: Py = Py::None;
    let mut held: Py = Py::None;
    let mut members: Py = Py::None;
    let mut months: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut placed: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut to_come: Py = Py::None;
    let mut told: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    let mut who: Py = Py::None;
    let mut who_said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How old a person they told us about is, said back: \"Your son is 7.\"\n\n    Their ages are filed with the person (\"my son is 7\", \"my mum is 68\"), so the ask is about\n    what the chat has been told, not about the world. A form of words the store has no pattern\n    for still reaches this: \"how old is the little one\" is answered when a child was the last\n    person named, and an honest \"you haven't told me\" comes back when no age was ever given.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[low.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&_continues_an_about_ask(&[memory.clone(), low.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_AGE_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_AGE_WORD(), "search", &[low.clone()]) } }))) {
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { low.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "rstrip", &[py_str("?.!")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _just_gave_an_age(&[memory.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_AGE_AGAIN(), "match", &[said.clone()]) } }))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = mcall(&g_AGE_AS_A_CAUSE(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AGE_ASK(), "search", &[low.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_AGE_ASK(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_age_is_about_a_person(&[low.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_AGE_ASK(), "search", &[low.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "search", &[py_str("\\b(?:normal|typical|usual|common|expected)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut entry: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("months")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("ages")]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return Py::None;
    }
    told = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    who_said = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    turns = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    held = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    to_come = mcall(&entry.clone(), "get", &[py_str("to_come")]);
    if py_truthy(&call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&told.clone(), "append", &[py_tuple(vec![who_said.clone(), geti(&entry.clone(), &py_str("age")), turns.clone(), if py_truthy(&call_builtin("isinstance", &[to_come.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { to_come.clone() } else { Py::None }, held.clone()])]);
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("ages")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    age = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&told.clone(), "append", &[py_tuple(vec![who_said.clone(), age.clone(), turns.clone(), Py::None, py_str("")])]);
    }
    }
    months = mcall(&entry.clone(), "get", &[py_str("months")]);
    if py_truthy(&call_builtin("isinstance", &[months.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&told.clone(), "append", &[py_tuple(vec![who_said.clone(), op_add(&call_builtin("str", &[months.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_str(" months")), turns.clone(), Py::None, held.clone()])]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_named_person(&[low.clone(), memory.clone()])))) {
    placed = _the_place_word_filed(&[memory.clone(), low.clone()]);
    if py_truthy(&placed.clone()) {
    return placed.clone();
    }
    }
    who = _named_person(&[low.clone(), memory.clone()]);
    asked_plural = { let __b = call_builtin("bool", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } } } };
    group = mcall(&g_GROUP_ASK(), "search", &[low.clone()]);
    if py_truthy(&group.clone()) {
    word = mcall(&mcall(&group.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    members = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&told.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&geti(&pyrt::xrefs::xget("memory.store_lines", "FAMILY_GROUP"), &word.clone()), &geti(&mcall(&geti(&entry.clone(), &Py::Int(0)), "split", &[]), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&mcall(&geti(&entry.clone(), &Py::Int(0)), "split", &[]), &Py::Int(0)), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&geti(&pyrt::xrefs::xget("memory.store_lines", "FAMILY_GROUP"), &word.clone()), &mcall(&pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"), "get", &[geti(&mcall(&geti(&entry.clone(), &Py::Int(0)), "split", &[]), &Py::Int(0))]))) } } } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&members.clone()) {
    return _ages_sentence(&[members.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("kids"), py_str("children")]), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&told.clone())) } }) {
    return py_str("You haven't told me how old they are.");
    }
    return op_add(&op_add(&py_str("You haven't told me how old your "), &word.clone()), &py_str(" are."));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_THE_KIDS(), "search", &[low.clone()]) } }) {
    children = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&told.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.store_lines", "CHILD"), &mcall(&pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"), "get", &[geti(&entry.clone(), &Py::Int(0)), mcall(&geti(&entry.clone(), &Py::Int(0)), "rstrip", &[py_str("s")])])))) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&mcall(&g_CHILD_WORD(), "search", &[low.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&children.clone()))) {
    return py_str("You haven't told me how old they are.");
    }
    return _ages_sentence(&[children.clone()]);
    }
    told = { let __b = children.clone(); if py_truthy(&__b) { __b } else { told.clone() } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&told.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:kids?|children|child|family)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    ages = _pet_ages(&[memory.clone()]);
    if py_truthy(&ages.clone()) {
    return _pet_ages_sentence(&[ages.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&told.clone()))) {
    theirs = mcall(&memory.clone(), "get", &[py_str("their_age")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[theirs.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&theirs.clone(), "get", &[py_str("age")]) } }) {
    return _one_age(&[py_str(""), geti(&theirs.clone(), &py_str("age")), { let __b = mcall(&theirs.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    }
    return py_str("You haven't told me how old they are.");
    }
    return _ages_sentence(&[told.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:she|her|hers|he|him|his)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    who = _person_from_pronoun(&[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    pronoun = if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:she|her)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_str("she") } else { py_str("he") };
    let __iter3 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter3 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &pronoun.clone()))) {
    continue;
    }
    age = mcall(&entry.clone(), "get", &[py_str("age")]);
    months = mcall(&entry.clone(), "get", &[py_str("months")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[months.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("isinstance", &[months.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&op_add(&if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("he")))) { py_str("He is ") } else { py_str("She is ") }, &call_builtin("str", &[months.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" months."));
    }
    when = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    he = Py::Bool(py_eq(&pronoun.clone(), &py_str("he")));
    if py_truthy(&when.clone()) {
    return op_add(&op_add(&op_add(&op_add(&if py_truthy(&he.clone()) { py_str("He turns ") } else { py_str("She turns ") }, &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" ")), &when.clone()), &py_str("."));
    }
    return op_add(&op_add(&if py_truthy(&he.clone()) { py_str("He is ") } else { py_str("She is ") }, &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("."));
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    pet = _pet_age_reply(&[memory.clone(), low.clone()]);
    if py_truthy(&pet.clone()) {
    return pet.clone();
    }
    return Py::None;
    }
    let __iter4 = py_iter_or_raise(&told.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 5, None);
    who_said = __unpk5[0].clone();
    age = __unpk5[1].clone();
    turns = __unpk5[2].clone();
    to_come = __unpk5[3].clone();
    held = __unpk5[4].clone();
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&held.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return _one_age(&[who.clone(), age.clone(), turns.clone(), to_come.clone(), held.clone()]);
    }
    }
    if py_truthy(&asked_plural.clone()) {
    word = if py_truthy(&mcall(&who.clone(), "endswith", &[py_str("s")])) { who.clone() } else { op_add(&who.clone(), &py_str("s")) };
    family = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&told.clone()) { let mut age: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&geti(&mcall(&geti(&age.clone(), &Py::Int(0)), "split", &[]), &Py::Int(0)), &word.clone()))) {  __out.push(age.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[family.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return _ages_sentence(&[family.clone()]);
    }
    }
    let __iter6 = py_iter_or_raise(&told.clone());
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 5, None);
    who_said = __unpk7[0].clone();
    age = __unpk7[1].clone();
    turns = __unpk7[2].clone();
    to_come = __unpk7[3].clone();
    held = __unpk7[4].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&who_said.clone(), &who.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&who_said.clone(), &slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } }) {
    return _one_age(&[who.clone(), age.clone(), turns.clone(), to_come.clone(), held.clone()]);
    }
    }
    family = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&told.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![op_add(&who.clone(), &py_str("s")), mcall(&g__IRREGULAR_PLURAL(), "get", &[who.clone(), op_add(&who.clone(), &py_str("s"))])]), &geti(&mcall(&geti(&one.clone(), &Py::Int(0)), "split", &[]), &Py::Int(0))))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&family.clone()) {
    return _ages_sentence(&[family.clone()]);
    }
    return op_add(&op_add(&py_str("You haven't told me how old your "), &who.clone()), &if py_truthy(&mcall(&who.clone(), "endswith", &[py_str("s")])) { py_str(" are.") } else { py_str(" is.") });
    Py::None
}
pub fn _the_place_end(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut next_word: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"young\", \"old\" or \"\" - the end of the family the ask's place word names, and the word.\n\n    The place word has to be the whole of the noun phrase, so an adjective use (\"the oldest\n    person in the world\", \"the little dog\") is not somebody's place in their family.\n    ");
    asked = mcall(&g__A_PLACE_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    after = slice_or_raise(&call_builtin("str", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &mcall(&asked.clone(), "end", &[]), &Py::None, &Py::None);
    after = mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[.,;:!?]")); __args.push(after.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)), "strip", &[]);
    next_word = if py_truthy(&after.clone()) { mcall(&geti(&mcall(&after.clone(), "split", &[]), &Py::Int(0)), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = next_word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__THE_PLACE_STANDS_ALONE(), &next_word.clone())) } }) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    word = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&asked.clone(), "group", &[py_str("who")]), "lower", &[]), "split", &[])]);
    return py_tuple(vec![if py_truthy(&Py::Bool(py_contains(&g__THE_YOUNGER_END(), &word.clone()))) { py_str("young") } else { py_str("old") }, word.clone()]);
    Py::None
}
pub fn _the_child_ages_on_file(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The ages the chat holds for children of theirs, oldest first as (word, age, turns).");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.store_lines", "CHILD"), &mcall(&pyrt::xrefs::xget("memory.store_lines", "CHILD_OF"), "get", &[who.clone(), mcall(&who.clone(), "rstrip", &[py_str("s")])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.store_lines", "CHILD"), &who.clone())) } }) {
    continue;
    }
    turns = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![who.clone(), geti(&entry.clone(), &py_str("age")), turns.clone()])]);
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("ages")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    age = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![who.clone(), age.clone(), turns.clone()])]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_place_word_filed(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut end: Py = Py::None;
    let mut one: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How old the child at the end of the family the ask names is, or \"\" when that end is empty.\n\n    A child named by their place is the same child however the place is said: \"the little one\",\n    \"the baby\" and \"the youngest\" are one end of the family, \"the eldest\" and \"the oldest\" the\n    other. Asked in a word the store has nothing filed under, the answer used to be a shrug - and\n    usually advice about the world, which is a confident answer to a question nobody asked - about\n    a child the chat had been told an age for one line before. So the end the ask names is read\n    against every age on file for a child of theirs, and the youngest or the eldest of them is the\n    answer. With no age at that end, the honest line is that the chat was never told.\n    ");
    let __unpk1 = py_unpack(&_the_place_end(&[low.clone()]), 2, None);
    end = __unpk1[0].clone();
    word = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&end.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    ages = _the_child_ages_on_file(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ages.clone()))) {
    return py_str("");
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let age = age.clone(); let ages = ages.clone(); let end = end.clone(); let low = low.clone(); let memory = memory.clone(); let one = one.clone(); let turns = turns.clone(); let who = who.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let age = age.clone(); let ages = ages.clone(); let end = end.clone(); let low = low.clone(); let memory = memory.clone(); let one = one.clone(); let turns = turns.clone(); let who = who.clone(); let word = word.clone(); let one: Py = __kwp(args, 0, "one").unwrap_or(Py::None); geti(&one.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ages.clone(), "sort", args))(&__args) };
    let __unpk2 = py_unpack(&if py_truthy(&Py::Bool(py_eq(&end.clone(), &py_str("young")))) { geti(&ages.clone(), &Py::Int(0)) } else { geti(&ages.clone(), &op_neg(&Py::Int(1))) }, 3, None);
    who = __unpk2[0].clone();
    age = __unpk2[1].clone();
    turns = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&ages.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&end.clone(), &py_str("old")))) {  __out.push(geti(&one.clone(), &Py::Int(1))); } } py_list(__out) })() }, &age.clone()))) {
    return py_str("");
    }
    return _one_age(&[word.clone(), age.clone(), turns.clone()]);
    Py::None
}
pub fn _pet_ages(args: &[Py]) -> Py {
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
    let mut pet: Py = Py::None;
    let _ = py_str("The animals the store holds an age for.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&pet.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn the_pronoun_said_with_a_name(args: &[Py]) -> Py {
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
    let mut age: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let mut aged: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The age a pronoun answer is about, said with the animal's name, or None.\n\n    A pronoun is never a name. \"we rehomed a greyhound in march\" / \"shes two\" / \"what age is she\n    again\" was answered \"She is 2.\", which would fit any animal in any chat and gave back only the\n    words the person had just typed. The one rule, for every animal and every way of asking, is:\n    the age the store holds for the animal it has been told about is said with that animal in it\n    (\"Your greyhound is 2.\").\n\n    The name has to be earned, not guessed. A pronoun in a chat with two animals is the reason the\n    guess goes wrong - a rabbit's \"hes one\" answered \"Max is 4.\" - so the animal is only named when\n    the age about to be said is the age the store filed for it, and for exactly one animal. When\n    nothing matches, there is no name to give and the pronoun path keeps the line it had, because\n    a pronoun is the right answer when the chat holds nothing to name.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    aged = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_pet_ages(&[memory.clone()])) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &age.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[aged.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return _pet_ages_sentence(&[py_list(vec![geti(&aged.clone(), &Py::Int(0))])]);
    Py::None
}
pub fn _pet_age_reply(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut aged: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut breed: Py = Py::None;
    let mut family: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("How old an animal they told us about is: \"Max is 4.\", \"Your dog is 5.\"\n\n    The age is filed with the animal (\"my dog is 5\", \"my dog max is 4\"), so the ask is about the\n    chat, not the world. When no age was ever given the store says so about that animal rather\n    than guessing or searching: the person can see from their own dog that the guess is wrong.\n    ");
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    asked = py_list(vec![]);
    animal = mcall(&g_PET_ASK(), "search", &[low.clone()]);
    family = if py_truthy(&animal.clone()) { _animal_family(&[mcall(&animal.clone(), "group", &[Py::Int(1)])]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    breed = mcall(&g_BREED_ASK(), "search", &[low.clone()]);
    if py_truthy(&breed.clone()) {
    family = _animal_family(&[pyrt::xrefs::xcall("understanding.breeds", "the_kind_of", &[geti(&mcall(&mcall(&breed.clone(), "group", &[Py::Int(0)]), "split", &[]), &op_neg(&Py::Int(1)))])]);
    }
    }
    if py_truthy(&family.clone()) {
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&_animal_family(&[mcall(&pet.clone(), "get", &[py_str("animal")])]), &family.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return op_add(&op_add(&op_add(&op_add(&py_str("You haven't told me about "), &if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&family.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an") } else { py_str("a") }), &py_str(" ")), &family.clone()), &py_str("."));
    }
    } else {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    pet = __item.clone();
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&name.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&asked.clone(), "append", &[pet.clone()]);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:she|he|it|they|his|her|its|their)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    aged = _pet_ages(&[memory.clone()]);
    if py_truthy(&aged.clone()) {
    asked = aged.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    aged = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[mcall(&pet.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&aged.clone()))) {
    family = _animal_family(&[mcall(&geti(&asked.clone(), &Py::Int(0)), "get", &[py_str("animal")])]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("int", &[{ let __b = mcall(&geti(&asked.clone(), &Py::Int(0)), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return op_add(&op_add(&py_str("You haven't told me how old your "), &family.clone()), &py_str("s are."));
    }
    return op_add(&op_add(&py_str("You haven't told me how old "), &_pet_named(&[geti(&asked.clone(), &Py::Int(0))])), &py_str(" is."));
    }
    said = _pet_ages_sentence(&[aged.clone()]);
    if py_truthy(&_same_as_last(&[said.clone(), memory.clone()])) {
    return op_add(&op_add(&py_str("Still "), &mcall(&py_str(" and "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&aged.clone(), &Py::None, &Py::Int(4), &Py::None)) { let mut pet: Py = __item.clone(); __out.push(_a_pet_age(&[pet.clone()])); } py_list(__out) })() }])), &py_str("."));
    }
    return said.clone();
    Py::None
}
pub fn _pet_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut animal: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("How to name one animal: by the name they gave it (\"Max\"), or by what they call it.");
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&name.clone()) {
    return _tidy_name(&[name.clone()]);
    }
    animal = mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = animal.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PLURAL_ANIMAL(), "search", &[animal.clone()]))) } }) {
    return op_add(&py_str("your "), &animal.clone());
    }
    return op_add(&py_str("your "), &_animal_family(&[animal.clone()]));
    Py::None
}
pub fn _a_pet_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut unit: Py = Py::None;
    let _ = py_str("The age as it was given: \"5\", \"4 months old\".");
    unit = call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return op_add(&call_builtin("str", &[geti(&pet.clone(), &py_str("age"))]).unwrap_or_else(|e| exc::raise(e)), &if py_truthy(&unit.clone()) { op_add(&op_add(&py_str(" "), &unit.clone()), &py_str(" old")) } else { py_str("") });
    Py::None
}
pub fn _pet_ages_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pets: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Every age the chat was given for these animals, said back as one sentence.");
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&pets.clone(), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    pet = __item.clone();
    who = _pet_named(&[pet.clone()]);
    verb = if py_truthy(&{ let __b = mcall(&who.clone(), "startswith", &[py_str("your ")]); if !py_truthy(&__b) { __b } else { mcall(&who.clone(), "endswith", &[py_str("s")]) } }) { py_str(" are ") } else { py_str(" is ") };
    let _ = mcall(&parts.clone(), "append", &[op_add(&op_add(&who.clone(), &verb.clone()), &_a_pet_age(&[pet.clone()]))]);
    }
    said = geti(&parts.clone(), &Py::Int(0));
    let __iter2 = py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter2 {
    part = __item.clone();
    let __aug3 = op_add(&said.clone(), &op_add(&py_str(" and "), &part.clone()));
    said = __aug3;
    }
    return op_add(&op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _one_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut age: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let mut turns: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { py_str("") };
    let mut to_come: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "to_come") { v.clone() }
    else { Py::None };
    let mut held: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { py_str("") };
    let mut name: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("One person's age, said back: a birthday still to come is said as one.\n\n    With both numbers on file the answer is both, because either alone is half of it: \"Your son is\n    7, and turns 8 in june.\" Saying only the birthday leaves them guessing how old he is now, and\n    saying only the age contradicts the birthday they just gave.\n\n    A person with a name of their own is said by the name the person gave (\"Ada is 7\"): the word\n    in front of them (\"your kids\") is the group they came in, not who they are.\n    ");
    if py_truthy(&held.clone()) {
    name = op_add(&mcall(&slice_or_raise(&call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1), &Py::None, &Py::None));
    } else {
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.store_lines", "PRONOUN_PEOPLE"), &who.clone()))) {
    name = who.clone();
    } else {
    name = if py_truthy(&who.clone()) { op_add(&py_str("your "), &who.clone()) } else { py_str("they") };
    }
    }
    plural = { let __b = Py::Bool(!py_truthy(&held.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("they"), py_str("them")]), &who.clone())) } } } } } };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[to_come.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&to_come.clone(), &age.clone())) } } } }) {
    said = op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&name.clone(), &if py_truthy(&plural.clone()) { py_str(" are ") } else { py_str(" is ") }), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(", and turns ")), &call_builtin("str", &[to_come.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" ")), &turns.clone());
    } else {
    if py_truthy(&turns.clone()) {
    said = op_add(&op_add(&op_add(&op_add(&name.clone(), &if py_truthy(&plural.clone()) { py_str(" turn ") } else { py_str(" turns ") }), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" ")), &turns.clone());
    } else {
    said = op_add(&op_add(&name.clone(), &if py_truthy(&plural.clone()) { py_str(" are ") } else { py_str(" is ") }), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e)));
    }
    }
    return op_add(&op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _just_gave_an_age(args: &[Py]) -> Py {
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
    let _ = py_str("True when the bot's last reply was an age it said back (\"Your son is 7.\").");
    return call_builtin("bool", &[mcall(&g_AGE_SAID(), "search", &[mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_bot", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_many(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The word for the group of them, in the shape the sentence needs (\"daughter\" ->\n    \"daughters\").");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_MANY_OF_THEM(), &word.clone()))) {
    return geti(&g_MANY_OF_THEM(), &word.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ALREADY_MANY(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("s")]) } }) {
    return word.clone();
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &slice_or_raise(&word.clone(), &op_neg(&Py::Int(2)), &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    }
    return op_add(&word.clone(), &py_str("s"));
    Py::None
}
pub fn _ages_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut told: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "told") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: told"))) };
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut group: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut held: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut to_come: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Every age the chat has been given, as one sentence, oldest fact first.\n\n    Ages given for the same word are said once, in the order they were told (\"your kids are 4 and\n    7\"): the person said them as one group and the answer is one group too.\n    ");
    groups = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&told.clone(), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 5, None);
    who = __unpk2[0].clone();
    age = __unpk2[1].clone();
    turns = __unpk2[2].clone();
    to_come = __unpk2[3].clone();
    held = __unpk2[4].clone();
    let __iter3 = py_iter_or_raise(&groups.clone());
    let mut __else4 = false;
    for __item in __iter3 {
    group = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&group.clone(), &Py::Int(0)), &who.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&group.clone(), &Py::Int(4)), &held.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&geti(&group.clone(), &Py::Int(2)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } } } } } }) {
    let _ = mcall(&geti(&group.clone(), &Py::Int(1)), "append", &[age.clone()]);
    break;
    }
    }
    let _ = mcall(&groups.clone(), "append", &[py_list(vec![who.clone(), py_list(vec![age.clone()]), turns.clone(), to_come.clone(), held.clone()])]);
    }
    parts = py_list(vec![]);
    let __iter5 = py_iter_or_raise(&groups.clone());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 5, None);
    who = __unpk6[0].clone();
    ages = __unpk6[1].clone();
    turns = __unpk6[2].clone();
    to_come = __unpk6[3].clone();
    held = __unpk6[4].clone();
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    let _ = mcall(&parts.clone(), "append", &[mcall(&_one_age(&[who.clone(), geti(&ages.clone(), &Py::Int(0)), turns.clone(), to_come.clone(), held.clone()]), "rstrip", &[py_str(".")])]);
    continue;
    }
    name = if py_truthy(&who.clone()) { op_add(&py_str("your "), &_the_many(&[who.clone()])) } else { py_str("they") };
    said = mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&ages.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut age: Py = __item.clone(); __out.push(call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    let _ = mcall(&parts.clone(), "append", &[op_add(&op_add(&op_add(&op_add(&name.clone(), &py_str(" are ")), &said.clone()), &py_str(" and ")), &call_builtin("str", &[geti(&ages.clone(), &op_neg(&Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)))]);
    }
    parts = op_add(&slice_or_raise(&parts.clone(), &Py::None, &Py::Int(1), &Py::None), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut part: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&groups.clone()) { let mut group: Py = __item.clone(); __out.push({ let __b = geti(&group.clone(), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&part.clone(), "startswith", &[mcall(&slice_or_raise(&call_builtin("str", &[geti(&group.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(1), &Py::None), "upper", &[])]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) { op_add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { part.clone() }); } py_list(__out) })() });
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return op_add(&op_add(&mcall(&geti(&geti(&parts.clone(), &Py::Int(0)), &Py::Int(0)), "upper", &[]), &slice_or_raise(&geti(&parts.clone(), &Py::Int(0)), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    said = op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&parts.clone(), &op_neg(&Py::Int(1))));
    return op_add(&op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _ask_gender(args: &[Py]) -> Py {
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
    let _ = py_str("The gender the line's own pronoun names, or \"\" when it names none.");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:she|her|hers)\\b"), call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("she");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|him|his)\\b"), call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("he");
    }
    return py_str("");
    Py::None
}
pub fn _called(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A name as it is written back: \"biscuit\" reads \"Biscuit\", \"mr patel\" reads \"Mr Patel\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&name.clone())) } }) {
    return py_str("");
    }
    words = mcall(&mcall(&name.clone(), "replace", &[py_str("."), py_str(" ")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(op_add(&mcall(&geti(&word.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None))); } py_list(__out) })() }]);
    }
    return op_add(&mcall(&geti(&geti(&words.clone(), &Py::Int(0)), &Py::Int(0)), "upper", &[]), &slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _every_line(args: &[Py]) -> Py {
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
    let _ = py_str("Every line the person has said, oldest first.\n\n    A named thing is looked for in all of them: \"do you remember what i said about the cat\" is\n    asked after ten turns of other subjects as often as after two, and \"you haven't mentioned\n    the cat\" about a thing they did say is worse than no answer at all. The short window in\n    `recent` stays for the pronoun asks (\"what did i say about it\"), where the nearest line\n    really is the subject of the chat.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.relations".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_PERSON_ASK" => Some(g_ABOUT_PERSON_ASK()),
            "AGE_ADVERB" => Some(g_AGE_ADVERB()),
            "AGE_AGAIN" => Some(g_AGE_AGAIN()),
            "AGE_ASK" => Some(g_AGE_ASK()),
            "AGE_AS_A_CAUSE" => Some(g_AGE_AS_A_CAUSE()),
            "AGE_COPULA" => Some(g_AGE_COPULA()),
            "AGE_DETERMINER" => Some(g_AGE_DETERMINER()),
            "AGE_PERSON_WORD" => Some(g_AGE_PERSON_WORD()),
            "AGE_RULE_WORD" => Some(g_AGE_RULE_WORD()),
            "AGE_SAID" => Some(g_AGE_SAID()),
            "AGE_WORD" => Some(g_AGE_WORD()),
            "AGE_YOU_SAY" => Some(g_AGE_YOU_SAY()),
            "ALREADY_MANY" => Some(g_ALREADY_MANY()),
            "ANOTHER_PERSON" => Some(g_ANOTHER_PERSON()),
            "BREED_ASK" => Some(g_BREED_ASK()),
            "CHAT_TAIL" => Some(g_CHAT_TAIL()),
            "CHILD" => Some(g_CHILD()),
            "CHILD_OF" => Some(g_CHILD_OF()),
            "CHILD_RELATIONS" => Some(g_CHILD_RELATIONS()),
            "CHILD_WORD" => Some(g_CHILD_WORD()),
            "FAMILY_GROUP" => Some(g_FAMILY_GROUP()),
            "GROUP_ASK" => Some(g_GROUP_ASK()),
            "GROUP_NAMES_TYPED" => Some(g_GROUP_NAMES_TYPED()),
            "GROUP_NAME_ASK" => Some(g_GROUP_NAME_ASK()),
            "GROUP_OF" => Some(g_GROUP_OF()),
            "GROUP_WORDS" => Some(g_GROUP_WORDS()),
            "HAVE_ANY" => Some(g_HAVE_ANY()),
            "HAVE_ANY_WORDS" => Some(g_HAVE_ANY_WORDS()),
            "HER_WORD" => Some(g_HER_WORD()),
            "MANY_OF_THEM" => Some(g_MANY_OF_THEM()),
            "NAME_AGAIN" => Some(g_NAME_AGAIN()),
            "PEOPLE_MANY" => Some(g_PEOPLE_MANY()),
            "PERSON_AFTER" => Some(g_PERSON_AFTER()),
            "PERSON_GROUP" => Some(g_PERSON_GROUP()),
            "PET_ASK" => Some(g_PET_ASK()),
            "PET_WORDS" => Some(g_PET_WORDS()),
            "PLURAL_ANIMAL" => Some(g_PLURAL_ANIMAL()),
            "PRONOUN_PEOPLE" => Some(g_PRONOUN_PEOPLE()),
            "RELATION_ALIKE" => Some(g_RELATION_ALIKE()),
            "SEVERAL_PEOPLE_NAMED" => Some(g_SEVERAL_PEOPLE_NAMED()),
            "THEIR_AGE" => Some(g_THEIR_AGE()),
            "THEIR_JOB" => Some(g_THEIR_JOB()),
            "THEIR_NAME" => Some(g_THEIR_NAME()),
            "THEIR_TASTE_A_YES_NO" => Some(g_THEIR_TASTE_A_YES_NO()),
            "THEIR_TASTE_INTO" => Some(g_THEIR_TASTE_INTO()),
            "THEIR_TASTE_NOT" => Some(g_THEIR_TASTE_NOT()),
            "THEIR_TASTE_NOT_INTO" => Some(g_THEIR_TASTE_NOT_INTO()),
            "THEIR_TASTE_WHAT" => Some(g_THEIR_TASTE_WHAT()),
            "THEY_ARE_NAMED" => Some(g_THEY_ARE_NAMED()),
            "THE_KIDS" => Some(g_THE_KIDS()),
            "TWO_OF_KIN" => Some(g_TWO_OF_KIN()),
            "_ABOUT_WHO" => Some(g__ABOUT_WHO()),
            "_A_PLACE_ASK" => Some(g__A_PLACE_ASK()),
            "_IRREGULAR_PLURAL" => Some(g__IRREGULAR_PLURAL()),
            "_PET_WORD" => Some(g__PET_WORD()),
            "_PLURAL" => Some(g__PLURAL()),
            "_THE_OLDER_END" => Some(g__THE_OLDER_END()),
            "_THE_PLACE_STANDS_ALONE" => Some(g__THE_PLACE_STANDS_ALONE()),
            "_THE_YOUNGER_END" => Some(g__THE_YOUNGER_END()),
            "_a" => Some(py_func("_a", |args: &[Py]| _a(args))),
            "_a_kept_person" => Some(py_func("_a_kept_person", |args: &[Py]| _a_kept_person(args))),
            "_a_list_of_names" => Some(py_func("_a_list_of_names", |args: &[Py]| _a_list_of_names(args))),
            "_a_list_of_words" => Some(py_func("_a_list_of_words", |args: &[Py]| _a_list_of_words(args))),
            "_a_or_any" => Some(py_func("_a_or_any", |args: &[Py]| _a_or_any(args))),
            "_a_person_in_the_store" => Some(py_func("_a_person_in_the_store", |args: &[Py]| _a_person_in_the_store(args))),
            "_a_pet_age" => Some(py_func("_a_pet_age", |args: &[Py]| _a_pet_age(args))),
            "_a_plural_list" => Some(py_func("_a_plural_list", |args: &[Py]| _a_plural_list(args))),
            "_a_word_the_store_knows" => Some(py_func("_a_word_the_store_knows", |args: &[Py]| _a_word_the_store_knows(args))),
            "_age_is_about_a_person" => Some(py_func("_age_is_about_a_person", |args: &[Py]| _age_is_about_a_person(args))),
            "_ages_reply" => Some(py_func("_ages_reply", |args: &[Py]| _ages_reply(args))),
            "_ages_sentence" => Some(py_func("_ages_sentence", |args: &[Py]| _ages_sentence(args))),
            "_animal_family" => Some(py_func("_animal_family", |args: &[Py]| _animal_family(args))),
            "_ask_gender" => Some(py_func("_ask_gender", |args: &[Py]| _ask_gender(args))),
            "_called" => Some(py_func("_called", |args: &[Py]| _called(args))),
            "_canon_relation" => Some(py_func("_canon_relation", |args: &[Py]| _canon_relation(args))),
            "_continues_an_about_ask" => Some(py_func("_continues_an_about_ask", |args: &[Py]| _continues_an_about_ask(args))),
            "_did_you_mean" => Some(py_func("_did_you_mean", |args: &[Py]| _did_you_mean(args))),
            "_every_line" => Some(py_func("_every_line", |args: &[Py]| _every_line(args))),
            "_folded" => Some(py_func("_folded", |args: &[Py]| _folded(args))),
            "_gender_of" => Some(py_func("_gender_of", |args: &[Py]| _gender_of(args))),
            "_in_the_group" => Some(py_func("_in_the_group", |args: &[Py]| _in_the_group(args))),
            "_just_gave_an_age" => Some(py_func("_just_gave_an_age", |args: &[Py]| _just_gave_an_age(args))),
            "_kin_forms" => Some(py_func("_kin_forms", |args: &[Py]| _kin_forms(args))),
            "_mentioned_the_group" => Some(py_func("_mentioned_the_group", |args: &[Py]| _mentioned_the_group(args))),
            "_more_than_one" => Some(py_func("_more_than_one", |args: &[Py]| _more_than_one(args))),
            "_more_than_one_word" => Some(py_func("_more_than_one_word", |args: &[Py]| _more_than_one_word(args))),
            "_named_person" => Some(py_func("_named_person", |args: &[Py]| _named_person(args))),
            "_newest_first" => Some(py_func("_newest_first", |args: &[Py]| _newest_first(args))),
            "_one_age" => Some(py_func("_one_age", |args: &[Py]| _one_age(args))),
            "_person_from_pronoun" => Some(py_func("_person_from_pronoun", |args: &[Py]| _person_from_pronoun(args))),
            "_pet_age_reply" => Some(py_func("_pet_age_reply", |args: &[Py]| _pet_age_reply(args))),
            "_pet_ages" => Some(py_func("_pet_ages", |args: &[Py]| _pet_ages(args))),
            "_pet_ages_sentence" => Some(py_func("_pet_ages_sentence", |args: &[Py]| _pet_ages_sentence(args))),
            "_pet_named" => Some(py_func("_pet_named", |args: &[Py]| _pet_named(args))),
            "_plural_person" => Some(py_func("_plural_person", |args: &[Py]| _plural_person(args))),
            "_pronoun_fits" => Some(py_func("_pronoun_fits", |args: &[Py]| _pronoun_fits(args))),
            "_pronoun_for" => Some(py_func("_pronoun_for", |args: &[Py]| _pronoun_for(args))),
            "_relation_reply" => Some(py_func("_relation_reply", |args: &[Py]| _relation_reply(args))),
            "_relations_of" => Some(py_func("_relations_of", |args: &[Py]| _relations_of(args))),
            "_same_as_last" => Some(py_func("_same_as_last", |args: &[Py]| _same_as_last(args))),
            "_same_name" => Some(py_func("_same_name", |args: &[Py]| _same_name(args))),
            "_same_name_however_they_wrote_it" => Some(py_func("_same_name_however_they_wrote_it", |args: &[Py]| _same_name_however_they_wrote_it(args))),
            "_same_thing" => Some(py_func("_same_thing", |args: &[Py]| _same_thing(args))),
            "_the_child_ages_on_file" => Some(py_func("_the_child_ages_on_file", |args: &[Py]| _the_child_ages_on_file(args))),
            "_the_group_names_typed" => Some(py_func("_the_group_names_typed", |args: &[Py]| _the_group_names_typed(args))),
            "_the_group_was_said" => Some(py_func("_the_group_was_said", |args: &[Py]| _the_group_was_said(args))),
            "_the_group_word" => Some(py_func("_the_group_word", |args: &[Py]| _the_group_word(args))),
            "_the_many" => Some(py_func("_the_many", |args: &[Py]| _the_many(args))),
            "_the_names_a_pronoun_gave" => Some(py_func("_the_names_a_pronoun_gave", |args: &[Py]| _the_names_a_pronoun_gave(args))),
            "_the_names_from" => Some(py_func("_the_names_from", |args: &[Py]| _the_names_from(args))),
            "_the_names_of_the_group" => Some(py_func("_the_names_of_the_group", |args: &[Py]| _the_names_of_the_group(args))),
            "_the_names_of_them_all" => Some(py_func("_the_names_of_them_all", |args: &[Py]| _the_names_of_them_all(args))),
            "_the_names_tail" => Some(py_func("_the_names_tail", |args: &[Py]| _the_names_tail(args))),
            "_the_names_they_typed" => Some(py_func("_the_names_they_typed", |args: &[Py]| _the_names_they_typed(args))),
            "_the_place_end" => Some(py_func("_the_place_end", |args: &[Py]| _the_place_end(args))),
            "_the_place_word_filed" => Some(py_func("_the_place_word_filed", |args: &[Py]| _the_place_word_filed(args))),
            "_the_several_named_before" => Some(py_func("_the_several_named_before", |args: &[Py]| _the_several_named_before(args))),
            "_their_job" => Some(py_func("_their_job", |args: &[Py]| _their_job(args))),
            "_their_taste" => Some(py_func("_their_taste", |args: &[Py]| _their_taste(args))),
            "_they_like_sentence" => Some(py_func("_they_like_sentence", |args: &[Py]| _they_like_sentence(args))),
            "_tidy_name" => Some(py_func("_tidy_name", |args: &[Py]| _tidy_name(args))),
            "_whether_they_have" => Some(py_func("_whether_they_have", |args: &[Py]| _whether_they_have(args))),
            "_with_article" => Some(py_func("_with_article", |args: &[Py]| _with_article(args))),
            "last_bot" => Some(g_last_bot()),
            "normalise" => Some(g_normalise()),
            "others" => Some(g_others()),
            "pick" => Some(g_pick()),
            "stem" => Some(g_stem()),
            "the_pronoun_said_with_a_name" => Some(py_func("the_pronoun_said_with_a_name", |args: &[Py]| the_pronoun_said_with_a_name(args))),
            "user" => Some(g_user()),
            "history" => Some(pyrt::xrefs::xmodule("memory.history")),
            "extract" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "breeds" => Some(pyrt::xrefs::xmodule("understanding.breeds")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_PERSON_ASK" => s_ABOUT_PERSON_ASK(v),
            "AGE_ADVERB" => s_AGE_ADVERB(v),
            "AGE_AGAIN" => s_AGE_AGAIN(v),
            "AGE_ASK" => s_AGE_ASK(v),
            "AGE_AS_A_CAUSE" => s_AGE_AS_A_CAUSE(v),
            "AGE_COPULA" => s_AGE_COPULA(v),
            "AGE_DETERMINER" => s_AGE_DETERMINER(v),
            "AGE_PERSON_WORD" => s_AGE_PERSON_WORD(v),
            "AGE_RULE_WORD" => s_AGE_RULE_WORD(v),
            "AGE_SAID" => s_AGE_SAID(v),
            "AGE_WORD" => s_AGE_WORD(v),
            "AGE_YOU_SAY" => s_AGE_YOU_SAY(v),
            "ALREADY_MANY" => s_ALREADY_MANY(v),
            "ANOTHER_PERSON" => s_ANOTHER_PERSON(v),
            "BREED_ASK" => s_BREED_ASK(v),
            "CHAT_TAIL" => s_CHAT_TAIL(v),
            "CHILD" => s_CHILD(v),
            "CHILD_OF" => s_CHILD_OF(v),
            "CHILD_RELATIONS" => s_CHILD_RELATIONS(v),
            "CHILD_WORD" => s_CHILD_WORD(v),
            "FAMILY_GROUP" => s_FAMILY_GROUP(v),
            "GROUP_ASK" => s_GROUP_ASK(v),
            "GROUP_NAMES_TYPED" => s_GROUP_NAMES_TYPED(v),
            "GROUP_NAME_ASK" => s_GROUP_NAME_ASK(v),
            "GROUP_OF" => s_GROUP_OF(v),
            "GROUP_WORDS" => s_GROUP_WORDS(v),
            "HAVE_ANY" => s_HAVE_ANY(v),
            "HAVE_ANY_WORDS" => s_HAVE_ANY_WORDS(v),
            "HER_WORD" => s_HER_WORD(v),
            "MANY_OF_THEM" => s_MANY_OF_THEM(v),
            "NAME_AGAIN" => s_NAME_AGAIN(v),
            "PEOPLE_MANY" => s_PEOPLE_MANY(v),
            "PERSON_AFTER" => s_PERSON_AFTER(v),
            "PERSON_GROUP" => s_PERSON_GROUP(v),
            "PET_ASK" => s_PET_ASK(v),
            "PET_WORDS" => s_PET_WORDS(v),
            "PLURAL_ANIMAL" => s_PLURAL_ANIMAL(v),
            "PRONOUN_PEOPLE" => s_PRONOUN_PEOPLE(v),
            "RELATION_ALIKE" => s_RELATION_ALIKE(v),
            "SEVERAL_PEOPLE_NAMED" => s_SEVERAL_PEOPLE_NAMED(v),
            "THEIR_AGE" => s_THEIR_AGE(v),
            "THEIR_JOB" => s_THEIR_JOB(v),
            "THEIR_NAME" => s_THEIR_NAME(v),
            "THEIR_TASTE_A_YES_NO" => s_THEIR_TASTE_A_YES_NO(v),
            "THEIR_TASTE_INTO" => s_THEIR_TASTE_INTO(v),
            "THEIR_TASTE_NOT" => s_THEIR_TASTE_NOT(v),
            "THEIR_TASTE_NOT_INTO" => s_THEIR_TASTE_NOT_INTO(v),
            "THEIR_TASTE_WHAT" => s_THEIR_TASTE_WHAT(v),
            "THEY_ARE_NAMED" => s_THEY_ARE_NAMED(v),
            "THE_KIDS" => s_THE_KIDS(v),
            "TWO_OF_KIN" => s_TWO_OF_KIN(v),
            "_ABOUT_WHO" => s__ABOUT_WHO(v),
            "_A_PLACE_ASK" => s__A_PLACE_ASK(v),
            "_IRREGULAR_PLURAL" => s__IRREGULAR_PLURAL(v),
            "_PET_WORD" => s__PET_WORD(v),
            "_PLURAL" => s__PLURAL(v),
            "_THE_OLDER_END" => s__THE_OLDER_END(v),
            "_THE_PLACE_STANDS_ALONE" => s__THE_PLACE_STANDS_ALONE(v),
            "_THE_YOUNGER_END" => s__THE_YOUNGER_END(v),
            "last_bot" => s_last_bot(v),
            "normalise" => s_normalise(v),
            "others" => s_others(v),
            "pick" => s_pick(v),
            "stem" => s_stem(v),
            "user" => s_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.relations", module_obj);
    pyrt::xrefs::reg_fn("memory.relations", "_folded", py_func("_folded", |args: &[Py]| _folded(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_person_from_pronoun", py_func("_person_from_pronoun", |args: &[Py]| _person_from_pronoun(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_gender_of", py_func("_gender_of", |args: &[Py]| _gender_of(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_pronoun_fits", py_func("_pronoun_fits", |args: &[Py]| _pronoun_fits(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_newest_first", py_func("_newest_first", |args: &[Py]| _newest_first(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_named_person", py_func("_named_person", |args: &[Py]| _named_person(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_person_in_the_store", py_func("_a_person_in_the_store", |args: &[Py]| _a_person_in_the_store(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_continues_an_about_ask", py_func("_continues_an_about_ask", |args: &[Py]| _continues_an_about_ask(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_word_the_store_knows", py_func("_a_word_the_store_knows", |args: &[Py]| _a_word_the_store_knows(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_age_is_about_a_person", py_func("_age_is_about_a_person", |args: &[Py]| _age_is_about_a_person(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_tidy_name", py_func("_tidy_name", |args: &[Py]| _tidy_name(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_same_as_last", py_func("_same_as_last", |args: &[Py]| _same_as_last(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_animal_family", py_func("_animal_family", |args: &[Py]| _animal_family(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a", py_func("_a", |args: &[Py]| _a(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_their_taste", py_func("_their_taste", |args: &[Py]| _their_taste(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_same_thing", py_func("_same_thing", |args: &[Py]| _same_thing(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_plural_person", py_func("_plural_person", |args: &[Py]| _plural_person(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_they_like_sentence", py_func("_they_like_sentence", |args: &[Py]| _they_like_sentence(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_their_job", py_func("_their_job", |args: &[Py]| _their_job(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_kin_forms", py_func("_kin_forms", |args: &[Py]| _kin_forms(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_kept_person", py_func("_a_kept_person", |args: &[Py]| _a_kept_person(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_same_name_however_they_wrote_it", py_func("_same_name_however_they_wrote_it", |args: &[Py]| _same_name_however_they_wrote_it(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_same_name", py_func("_same_name", |args: &[Py]| _same_name(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_more_than_one", py_func("_more_than_one", |args: &[Py]| _more_than_one(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_canon_relation", py_func("_canon_relation", |args: &[Py]| _canon_relation(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_relations_of", py_func("_relations_of", |args: &[Py]| _relations_of(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_in_the_group", py_func("_in_the_group", |args: &[Py]| _in_the_group(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_list_of_names", py_func("_a_list_of_names", |args: &[Py]| _a_list_of_names(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_mentioned_the_group", py_func("_mentioned_the_group", |args: &[Py]| _mentioned_the_group(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_whether_they_have", py_func("_whether_they_have", |args: &[Py]| _whether_they_have(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_list_of_words", py_func("_a_list_of_words", |args: &[Py]| _a_list_of_words(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_more_than_one_word", py_func("_more_than_one_word", |args: &[Py]| _more_than_one_word(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_with_article", py_func("_with_article", |args: &[Py]| _with_article(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_or_any", py_func("_a_or_any", |args: &[Py]| _a_or_any(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_plural_list", py_func("_a_plural_list", |args: &[Py]| _a_plural_list(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_group_names_typed", py_func("_the_group_names_typed", |args: &[Py]| _the_group_names_typed(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_names_a_pronoun_gave", py_func("_the_names_a_pronoun_gave", |args: &[Py]| _the_names_a_pronoun_gave(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_names_from", py_func("_the_names_from", |args: &[Py]| _the_names_from(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_several_named_before", py_func("_the_several_named_before", |args: &[Py]| _the_several_named_before(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_names_tail", py_func("_the_names_tail", |args: &[Py]| _the_names_tail(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_names_of_them_all", py_func("_the_names_of_them_all", |args: &[Py]| _the_names_of_them_all(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_names_they_typed", py_func("_the_names_they_typed", |args: &[Py]| _the_names_they_typed(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_names_of_the_group", py_func("_the_names_of_the_group", |args: &[Py]| _the_names_of_the_group(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_did_you_mean", py_func("_did_you_mean", |args: &[Py]| _did_you_mean(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_relation_reply", py_func("_relation_reply", |args: &[Py]| _relation_reply(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_pronoun_for", py_func("_pronoun_for", |args: &[Py]| _pronoun_for(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_group_word", py_func("_the_group_word", |args: &[Py]| _the_group_word(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_group_was_said", py_func("_the_group_was_said", |args: &[Py]| _the_group_was_said(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_ages_reply", py_func("_ages_reply", |args: &[Py]| _ages_reply(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_place_end", py_func("_the_place_end", |args: &[Py]| _the_place_end(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_child_ages_on_file", py_func("_the_child_ages_on_file", |args: &[Py]| _the_child_ages_on_file(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_place_word_filed", py_func("_the_place_word_filed", |args: &[Py]| _the_place_word_filed(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_pet_ages", py_func("_pet_ages", |args: &[Py]| _pet_ages(args)));
    pyrt::xrefs::reg_fn("memory.relations", "the_pronoun_said_with_a_name", py_func("the_pronoun_said_with_a_name", |args: &[Py]| the_pronoun_said_with_a_name(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_pet_age_reply", py_func("_pet_age_reply", |args: &[Py]| _pet_age_reply(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_pet_named", py_func("_pet_named", |args: &[Py]| _pet_named(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_a_pet_age", py_func("_a_pet_age", |args: &[Py]| _a_pet_age(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_pet_ages_sentence", py_func("_pet_ages_sentence", |args: &[Py]| _pet_ages_sentence(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_one_age", py_func("_one_age", |args: &[Py]| _one_age(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_just_gave_an_age", py_func("_just_gave_an_age", |args: &[Py]| _just_gave_an_age(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_the_many", py_func("_the_many", |args: &[Py]| _the_many(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_ages_sentence", py_func("_ages_sentence", |args: &[Py]| _ages_sentence(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_ask_gender", py_func("_ask_gender", |args: &[Py]| _ask_gender(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_called", py_func("_called", |args: &[Py]| _called(args)));
    pyrt::xrefs::reg_fn("memory.relations", "_every_line", py_func("_every_line", |args: &[Py]| _every_line(args)));
    pyrt::xrefs::reg_access("memory.relations", "ABOUT_PERSON_ASK", g_ABOUT_PERSON_ASK, s_ABOUT_PERSON_ASK);
    pyrt::xrefs::reg_access("memory.relations", "AGE_ADVERB", g_AGE_ADVERB, s_AGE_ADVERB);
    pyrt::xrefs::reg_access("memory.relations", "AGE_AGAIN", g_AGE_AGAIN, s_AGE_AGAIN);
    pyrt::xrefs::reg_access("memory.relations", "AGE_ASK", g_AGE_ASK, s_AGE_ASK);
    pyrt::xrefs::reg_access("memory.relations", "AGE_AS_A_CAUSE", g_AGE_AS_A_CAUSE, s_AGE_AS_A_CAUSE);
    pyrt::xrefs::reg_access("memory.relations", "AGE_COPULA", g_AGE_COPULA, s_AGE_COPULA);
    pyrt::xrefs::reg_access("memory.relations", "AGE_DETERMINER", g_AGE_DETERMINER, s_AGE_DETERMINER);
    pyrt::xrefs::reg_access("memory.relations", "AGE_PERSON_WORD", g_AGE_PERSON_WORD, s_AGE_PERSON_WORD);
    pyrt::xrefs::reg_access("memory.relations", "AGE_RULE_WORD", g_AGE_RULE_WORD, s_AGE_RULE_WORD);
    pyrt::xrefs::reg_access("memory.relations", "AGE_SAID", g_AGE_SAID, s_AGE_SAID);
    pyrt::xrefs::reg_access("memory.relations", "AGE_WORD", g_AGE_WORD, s_AGE_WORD);
    pyrt::xrefs::reg_access("memory.relations", "AGE_YOU_SAY", g_AGE_YOU_SAY, s_AGE_YOU_SAY);
    pyrt::xrefs::reg_access("memory.relations", "ALREADY_MANY", g_ALREADY_MANY, s_ALREADY_MANY);
    pyrt::xrefs::reg_access("memory.relations", "ANOTHER_PERSON", g_ANOTHER_PERSON, s_ANOTHER_PERSON);
    pyrt::xrefs::reg_access("memory.relations", "BREED_ASK", g_BREED_ASK, s_BREED_ASK);
    pyrt::xrefs::reg_access("memory.relations", "CHAT_TAIL", g_CHAT_TAIL, s_CHAT_TAIL);
    pyrt::xrefs::reg_access("memory.relations", "CHILD", g_CHILD, s_CHILD);
    pyrt::xrefs::reg_access("memory.relations", "CHILD_OF", g_CHILD_OF, s_CHILD_OF);
    pyrt::xrefs::reg_access("memory.relations", "CHILD_RELATIONS", g_CHILD_RELATIONS, s_CHILD_RELATIONS);
    pyrt::xrefs::reg_access("memory.relations", "CHILD_WORD", g_CHILD_WORD, s_CHILD_WORD);
    pyrt::xrefs::reg_access("memory.relations", "FAMILY_GROUP", g_FAMILY_GROUP, s_FAMILY_GROUP);
    pyrt::xrefs::reg_access("memory.relations", "GROUP_ASK", g_GROUP_ASK, s_GROUP_ASK);
    pyrt::xrefs::reg_access("memory.relations", "GROUP_NAMES_TYPED", g_GROUP_NAMES_TYPED, s_GROUP_NAMES_TYPED);
    pyrt::xrefs::reg_access("memory.relations", "GROUP_NAME_ASK", g_GROUP_NAME_ASK, s_GROUP_NAME_ASK);
    pyrt::xrefs::reg_access("memory.relations", "GROUP_OF", g_GROUP_OF, s_GROUP_OF);
    pyrt::xrefs::reg_access("memory.relations", "GROUP_WORDS", g_GROUP_WORDS, s_GROUP_WORDS);
    pyrt::xrefs::reg_access("memory.relations", "HAVE_ANY", g_HAVE_ANY, s_HAVE_ANY);
    pyrt::xrefs::reg_access("memory.relations", "HAVE_ANY_WORDS", g_HAVE_ANY_WORDS, s_HAVE_ANY_WORDS);
    pyrt::xrefs::reg_access("memory.relations", "HER_WORD", g_HER_WORD, s_HER_WORD);
    pyrt::xrefs::reg_access("memory.relations", "MANY_OF_THEM", g_MANY_OF_THEM, s_MANY_OF_THEM);
    pyrt::xrefs::reg_access("memory.relations", "NAME_AGAIN", g_NAME_AGAIN, s_NAME_AGAIN);
    pyrt::xrefs::reg_access("memory.relations", "PEOPLE_MANY", g_PEOPLE_MANY, s_PEOPLE_MANY);
    pyrt::xrefs::reg_access("memory.relations", "PERSON_AFTER", g_PERSON_AFTER, s_PERSON_AFTER);
    pyrt::xrefs::reg_access("memory.relations", "PERSON_GROUP", g_PERSON_GROUP, s_PERSON_GROUP);
    pyrt::xrefs::reg_access("memory.relations", "PET_ASK", g_PET_ASK, s_PET_ASK);
    pyrt::xrefs::reg_access("memory.relations", "PET_WORDS", g_PET_WORDS, s_PET_WORDS);
    pyrt::xrefs::reg_access("memory.relations", "PLURAL_ANIMAL", g_PLURAL_ANIMAL, s_PLURAL_ANIMAL);
    pyrt::xrefs::reg_access("memory.relations", "PRONOUN_PEOPLE", g_PRONOUN_PEOPLE, s_PRONOUN_PEOPLE);
    pyrt::xrefs::reg_access("memory.relations", "RELATION_ALIKE", g_RELATION_ALIKE, s_RELATION_ALIKE);
    pyrt::xrefs::reg_access("memory.relations", "SEVERAL_PEOPLE_NAMED", g_SEVERAL_PEOPLE_NAMED, s_SEVERAL_PEOPLE_NAMED);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_AGE", g_THEIR_AGE, s_THEIR_AGE);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_JOB", g_THEIR_JOB, s_THEIR_JOB);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_NAME", g_THEIR_NAME, s_THEIR_NAME);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_TASTE_A_YES_NO", g_THEIR_TASTE_A_YES_NO, s_THEIR_TASTE_A_YES_NO);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_TASTE_INTO", g_THEIR_TASTE_INTO, s_THEIR_TASTE_INTO);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_TASTE_NOT", g_THEIR_TASTE_NOT, s_THEIR_TASTE_NOT);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_TASTE_NOT_INTO", g_THEIR_TASTE_NOT_INTO, s_THEIR_TASTE_NOT_INTO);
    pyrt::xrefs::reg_access("memory.relations", "THEIR_TASTE_WHAT", g_THEIR_TASTE_WHAT, s_THEIR_TASTE_WHAT);
    pyrt::xrefs::reg_access("memory.relations", "THEY_ARE_NAMED", g_THEY_ARE_NAMED, s_THEY_ARE_NAMED);
    pyrt::xrefs::reg_access("memory.relations", "THE_KIDS", g_THE_KIDS, s_THE_KIDS);
    pyrt::xrefs::reg_access("memory.relations", "TWO_OF_KIN", g_TWO_OF_KIN, s_TWO_OF_KIN);
    pyrt::xrefs::reg_access("memory.relations", "_ABOUT_WHO", g__ABOUT_WHO, s__ABOUT_WHO);
    pyrt::xrefs::reg_access("memory.relations", "_A_PLACE_ASK", g__A_PLACE_ASK, s__A_PLACE_ASK);
    pyrt::xrefs::reg_access("memory.relations", "_IRREGULAR_PLURAL", g__IRREGULAR_PLURAL, s__IRREGULAR_PLURAL);
    pyrt::xrefs::reg_access("memory.relations", "_PET_WORD", g__PET_WORD, s__PET_WORD);
    pyrt::xrefs::reg_access("memory.relations", "_PLURAL", g__PLURAL, s__PLURAL);
    pyrt::xrefs::reg_access("memory.relations", "_THE_OLDER_END", g__THE_OLDER_END, s__THE_OLDER_END);
    pyrt::xrefs::reg_access("memory.relations", "_THE_PLACE_STANDS_ALONE", g__THE_PLACE_STANDS_ALONE, s__THE_PLACE_STANDS_ALONE);
    pyrt::xrefs::reg_access("memory.relations", "_THE_YOUNGER_END", g__THE_YOUNGER_END, s__THE_YOUNGER_END);
    pyrt::xrefs::reg_access("memory.relations", "last_bot", g_last_bot, s_last_bot);
    pyrt::xrefs::reg_access("memory.relations", "normalise", g_normalise, s_normalise);
    pyrt::xrefs::reg_access("memory.relations", "others", g_others, s_others);
    pyrt::xrefs::reg_access("memory.relations", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("memory.relations", "stem", g_stem, s_stem);
    pyrt::xrefs::reg_access("memory.relations", "user", g_user, s_user);
}
