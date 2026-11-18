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


thread_local! { static G_ALREADY_DEALT_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ABSCESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHAIR_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GUT_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOOSE_STOOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOUTH_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POO_COLOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SAMPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SPREADING_SIGN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_IS_TERRIBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORD_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMMON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_BOTH_WAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_MAY_GO_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIN_ADJECTIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIN_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIN_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIN_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_NOUNS_BY_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_PAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_SKIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_SKIP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_UNWELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPEAKER_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_IT_HAPPENED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALTERNATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HELPER_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MAYBE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERB_NEEDING_ITS_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEFORE_THE_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHUCK_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COLD_SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FROM_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOT_SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__noun: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__part_name: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__part_nouns: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__part_pattern: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALREADY_DEALT_WITH() -> Py { ensure_init(); G_ALREADY_DEALT_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_DEALT_WITH(v: Py) { ensure_init(); G_ALREADY_DEALT_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_PART() -> Py { ensure_init(); G_ANOTHER_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_PART(v: Py) { ensure_init(); G_ANOTHER_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ABSCESS() -> Py { ensure_init(); G_AN_ABSCESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ABSCESS(v: Py) { ensure_init(); G_AN_ABSCESS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHAIR_WORD() -> Py { ensure_init(); G_A_CHAIR_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHAIR_WORD(v: Py) { ensure_init(); G_A_CHAIR_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GUT_ONLY() -> Py { ensure_init(); G_A_GUT_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GUT_ONLY(v: Py) { ensure_init(); G_A_GUT_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOOSE_STOOL() -> Py { ensure_init(); G_A_LOOSE_STOOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOOSE_STOOL(v: Py) { ensure_init(); G_A_LOOSE_STOOL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOUTH_WORD() -> Py { ensure_init(); G_A_MOUTH_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOUTH_WORD(v: Py) { ensure_init(); G_A_MOUTH_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POO_COLOUR() -> Py { ensure_init(); G_A_POO_COLOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POO_COLOUR(v: Py) { ensure_init(); G_A_POO_COLOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SAMPLE() -> Py { ensure_init(); G_A_SAMPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SAMPLE(v: Py) { ensure_init(); G_A_SAMPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SPREADING_SIGN() -> Py { ensure_init(); G_A_SPREADING_SIGN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SPREADING_SIGN(v: Py) { ensure_init(); G_A_SPREADING_SIGN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_IS_TERRIBLE() -> Py { ensure_init(); G_A_THING_IS_TERRIBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_IS_TERRIBLE(v: Py) { ensure_init(); G_A_THING_IS_TERRIBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORD_OF_ITS_OWN() -> Py { ensure_init(); G_A_WORD_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORD_OF_ITS_OWN(v: Py) { ensure_init(); G_A_WORD_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_COMMON_WORDS() -> Py { ensure_init(); G_COMMON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMMON_WORDS(v: Py) { ensure_init(); G_COMMON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_BOTH_WAYS() -> Py { ensure_init(); G_GOING_BOTH_WAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_BOTH_WAYS(v: Py) { ensure_init(); G_GOING_BOTH_WAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_MAY_GO_ON() -> Py { ensure_init(); G_OFF_MAY_GO_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_MAY_GO_ON(v: Py) { ensure_init(); G_OFF_MAY_GO_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIN_ADJECTIVE() -> Py { ensure_init(); G_PAIN_ADJECTIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIN_ADJECTIVE(v: Py) { ensure_init(); G_PAIN_ADJECTIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIN_BETWEEN() -> Py { ensure_init(); G_PAIN_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIN_BETWEEN(v: Py) { ensure_init(); G_PAIN_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIN_PHRASE() -> Py { ensure_init(); G_PAIN_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIN_PHRASE(v: Py) { ensure_init(); G_PAIN_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIN_REACH() -> Py { ensure_init(); G_PAIN_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIN_REACH(v: Py) { ensure_init(); G_PAIN_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTS() -> Py { ensure_init(); G_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTS(v: Py) { ensure_init(); G_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_NOUNS_BY_LENGTH() -> Py { ensure_init(); G_PART_NOUNS_BY_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_NOUNS_BY_LENGTH(v: Py) { ensure_init(); G_PART_NOUNS_BY_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_PAIN() -> Py { ensure_init(); G_PLAIN_PAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_PAIN(v: Py) { ensure_init(); G_PLAIN_PAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_SKIP() -> Py { ensure_init(); G_PLAIN_SKIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_SKIP(v: Py) { ensure_init(); G_PLAIN_SKIP.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_SKIP_WORDS() -> Py { ensure_init(); G_PLAIN_SKIP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_SKIP_WORDS(v: Py) { ensure_init(); G_PLAIN_SKIP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_UNWELL() -> Py { ensure_init(); G_PLAIN_UNWELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_UNWELL(v: Py) { ensure_init(); G_PLAIN_UNWELL.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE() -> Py { ensure_init(); G_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); G_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_SPEAKER_ILL() -> Py { ensure_init(); G_SPEAKER_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_ILL(v: Py) { ensure_init(); G_SPEAKER_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_IT_HAPPENED() -> Py { ensure_init(); G_WHEN_IT_HAPPENED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_IT_HAPPENED(v: Py) { ensure_init(); G_WHEN_IT_HAPPENED.with(|c| *c.borrow_mut() = v); }
pub fn g__ALTERNATING() -> Py { ensure_init(); G__ALTERNATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALTERNATING(v: Py) { ensure_init(); G__ALTERNATING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HELPER_BEFORE() -> Py { ensure_init(); G__A_HELPER_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HELPER_BEFORE(v: Py) { ensure_init(); G__A_HELPER_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MAYBE() -> Py { ensure_init(); G__A_MAYBE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MAYBE(v: Py) { ensure_init(); G__A_MAYBE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NO() -> Py { ensure_init(); G__A_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NO(v: Py) { ensure_init(); G__A_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERB_NEEDING_ITS_SUBJECT() -> Py { ensure_init(); G__A_VERB_NEEDING_ITS_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERB_NEEDING_ITS_SUBJECT(v: Py) { ensure_init(); G__A_VERB_NEEDING_ITS_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__BEFORE_THE_COMPLAINT() -> Py { ensure_init(); G__BEFORE_THE_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEFORE_THE_COMPLAINT(v: Py) { ensure_init(); G__BEFORE_THE_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g__CHUCK_UP() -> Py { ensure_init(); G__CHUCK_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHUCK_UP(v: Py) { ensure_init(); G__CHUCK_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__COLD_SIDE() -> Py { ensure_init(); G__COLD_SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COLD_SIDE(v: Py) { ensure_init(); G__COLD_SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__FILL() -> Py { ensure_init(); G__FILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FILL(v: Py) { ensure_init(); G__FILL.with(|c| *c.borrow_mut() = v); }
pub fn g__FROM_TO() -> Py { ensure_init(); G__FROM_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FROM_TO(v: Py) { ensure_init(); G__FROM_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__HOT_SIDE() -> Py { ensure_init(); G__HOT_SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOT_SIDE(v: Py) { ensure_init(); G__HOT_SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__JOIN() -> Py { ensure_init(); G__JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOIN(v: Py) { ensure_init(); G__JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g__noun() -> Py { ensure_init(); G__noun.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__noun(v: Py) { ensure_init(); G__noun.with(|c| *c.borrow_mut() = v); }
pub fn g__part_name() -> Py { ensure_init(); G__part_name.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__part_name(v: Py) { ensure_init(); G__part_name.with(|c| *c.borrow_mut() = v); }
pub fn g__part_nouns() -> Py { ensure_init(); G__part_nouns.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__part_nouns(v: Py) { ensure_init(); G__part_nouns.with(|c| *c.borrow_mut() = v); }
pub fn g__part_pattern() -> Py { ensure_init(); G__part_pattern.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__part_pattern(v: Py) { ensure_init(); G__part_pattern.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Which part of the body the line says is unwell, when the illness is the speaker's own.\n\nOne reading for everybody: \"i have a migraine\" and \"my throat is killing me\" are the same kind\nof line as \"i have a cold\", and the only thing that tells them apart is the part named. The\ncomposer keeps a pool of lines per part (`ill` is the cold's), so this reading says which pool\nthe line is asking for instead of every module matching the words again.\n\nThree things are checked, and all are needed:\n\n* the part, from the words people use for it (\"migraine\", \"sore throat\", \"fever\", \"throwing up\",\n  \"chest infection\");\n* whose it is. \"my son has a fever\" is the child's illness and the child's advice, \"she gets\n  migraines\" is hers, and only the speaker's own comes here. \"My\" counts only when the part\n  itself follows it - \"my head\" is the speaker's, \"my room temperature is 40\" is a room's - and\n  anything else needs the speaker in front of it (\"i have a headache\", \"ive been throwing up\");\n* that it is the body and not the word: hay fever is not a fever, and \"i feel sick about going\n  to the dentist\" is dread rather than a stomach.\n\nThe reading is `None` on every line that names no part - a cold, the flu and \"i feel ill\" stay\nwith the plain illness pool, which is what those lines want.\n");
    s_COMMON_WORDS(pyrt::xrefs::xget("understanding.normalize", "LEXICON"));
    s__COLD_SIDE(py_str("\\b(?:freezing|cold|chilly|shivering|shivery)\\b"));
    s__HOT_SIDE(py_str("\\b(?:sweating|sweaty|boiling|hot|burning(?: up)?)\\b"));
    s__FILL(py_str("(?:\\s+(?:one|two|a|an|the|some|bit|minute|minutes|or|and|then|so|very|really|sometimes|always|all|next|im|i'm|i|am|at|in|of|night|nights|morning|mornings|day|days|evening|evenings|afternoon|week|weekend|first|second))*\\s*"));
    s__JOIN(op_add(&op_add(&op_add(&g__FILL(), &py_str("(?:and\\s+then|then|and|or|but)")), &g__FILL()), &py_str("\\s*")));
    s__FROM_TO(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:"), &g__COLD_SIDE()), &py_str("\\s+to\\s+")), &g__HOT_SIDE()), &py_str("|")), &g__HOT_SIDE()), &py_str("\\s+to\\s+")), &g__COLD_SIDE()), &py_str(")")));
    s__ALTERNATING(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:"), &g__COLD_SIDE()), &g__JOIN()), &g__HOT_SIDE()), &py_str("|")), &g__HOT_SIDE()), &g__JOIN()), &g__COLD_SIDE()), &py_str("|")), &g__FROM_TO()), &py_str(")")));
    s_GOING_BOTH_WAYS(stdlib_call("re", "compile", &[g__ALTERNATING(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CHUCK_UP(py_str("\\b(?:been |kept? |keep |still )?chuck(?:s|ed|ing)? (?:it |them |his |her |my |their |our )?up\\b(?=\\s*(?:$|[,.!?;:]|(?:all|every|again|still|constantly|most|so|since|at|during|yesterday|today|tonight|tomorrow|now|once|twice|last|this|in|for|after|before|the other\\b)\\b|(?:my |the |his |her |their |your |our |some |that )?(?:lunch|dinner|breakfast|tea|supper|food|water|drink|drinks|milk|coffee|beer|wine|blood|bile|medicine|meal|it)\\b))"));
    s_PARTS(py_tuple(vec![py_tuple(vec![py_str("head"), py_str("\\bmigraines?\\b|\\bmigranes?\\b|\\bheadaches?\\b|\\bhead aches?\\b|\\bhead\\b[^.!?]{0,14}?\\b(?:is |feels? |keeps? |has )?(?:pounding|throbbing|splitting|banging|hurts?|hurting|aching|killing)\\b|\\b(?:pounding|throbbing|splitting)\\b[^.!?]{0,10}?\\bhead\\b"), call_builtin("frozenset", &[mcall(&py_str("head headache headaches migraine migraines"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("throat"), py_str("\\bsore throat\\b|\\bstrep throat\\b|\\bthroat\\b[^.!?]{0,14}?\\b(?:hurts?|hurting|aches?|is sore|is killing|feels? raw|is rough|is scratchy)\\b"), call_builtin("frozenset", &[mcall(&py_str("throat"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("temperature"), op_add(&py_str("(?<!hay )\\bfever\\w*|\\btemperature\\b|\\bshivery\\b|\\bchills\\b|\\bhot and cold\\b|\\bburning up\\b|"), &g__ALTERNATING()), call_builtin("frozenset", &[mcall(&py_str("temperature fever"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("stomach"), op_add(&op_add(&py_str("\\bstomach (?:bug|ache|cramps?|upset)\\b|\\btummy (?:ache|bug)\\b|\\b(?:stomach|tummy)\\b[^.!?]{0,12}?\\b(?:hurts?|hurting|aches?|aching|is killing|is (?:sore|cramping))\\b|\\b(?:(?:going|gonna|about)\\s+to\\s+|been |kept? |keep |still )?(?:throwing up|thrown up|threw up|throws up|throw up|vomiting|vomited|vomit|vomits|puk(?:e|es|ed|ing)|barf(?:s|ed|ing)?|"), &g__CHUCK_UP()), &py_str("|(?:be|been|being|was|were) sick(?!\\s+(?:of|to death|and tired)\\b)|nauseous|nausea)\\b|\\bsick\\b(?=\\s+(?:\\d|twice|once|three|four|five|since|all |last night|yesterday|most mornings|every))|\\b(?:can ?not|cant|can't|could not|couldnt|couldn't|unable to|not able to) keep (?:anything|much|water|fluids?|food|it) down\\b|\\bkeep(?:ing)? nothing down\\b|\\b(?:feel|feels|feeling|felt) sick\\b(?!\\s+(?:about|at|of|thinking)\\b)|\\bdiarrhoea\\b|\\bdiarrhea\\b|\\bd\\s?and\\s?v\\b")), call_builtin("frozenset", &[mcall(&py_str("stomach tummy belly gut"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("chest"), py_str("\\bchest infection\\b|\\bchesty\\b|\\bwheez\\w*"), call_builtin("frozenset", &[mcall(&py_str("chest"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("teeth"), py_str("\\btoothaches?\\b|\\btooth aches?\\b|\\bteeth\\b[^.!?]{0,14}?\\b(?:hurt|hurts|aching|ache|sensitive|killing)\\b|\\btooth\\b[^.!?]{0,14}?\\b(?:hurts?|aching|aches?|throbbing|is killing)\\b|\\b(?:dental|dentist)\\b[^.!?]{0,20}?\\b(?:pain|agony|hurts?)\\b|\\babscess\\b|\\bgums?\\b[^.!?]{0,14}?\\b(?:sore|bleeding|swollen|hurts?|aching)\\b"), call_builtin("frozenset", &[mcall(&py_str("tooth teeth gum gums mouth jaw"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("ear"), py_str("\\bearaches?\\b|\\bear aches?\\b|\\b(?:ear|ears)\\b[^.!?]{0,16}?\\b(?:hurts?|hurting|aching|aches?|throbbing|is killing|is sore)\\b|\\bear infection\\b|\\bglue ear\\b"), call_builtin("frozenset", &[mcall(&py_str("ear ears"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))])]));
    s_AN_ABSCESS(stdlib_call("re", "compile", &[py_str("\\babscess\\w*\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOUTH_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:tooth|toothache|toothaches|teeth|gums?|mouth|jaw|dental|dentist|molar)\\b[^.!?]{0,20}?\\babscess\\w*|\\babscess\\w*\\b[^.!?]{0,20}?\\b(?:tooth|toothache|teeth|gums?|mouth|jaw|dental|molar)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPREADING_SIGN(stdlib_call("re", "compile", &[py_str("\\b(?:fever|temperature|pus|pussy|puffy|swelling|swollen|spreading|swallow|swallowing|cheek|face)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER_PART(stdlib_call("re", "compile", &[py_str("\\b(?:leg|legs|arm|arms|foot|feet|toe|toes|hand|hands|finger|fingers|knee|knees|hip|hips|back|shoulder|shoulders|skin|wound|wounds|armpit|groin|bum|bottom|thumb|elbow|ankle|shin|thigh|chest|stomach|tummy|neck|scalp|eye|eyes|breast|breasts|ear|ears)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_DEALT_WITH(stdlib_call("re", "compile", &[py_str("\\b(?:drained|lanced|removed|burst|popped|gone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SAMPLE(stdlib_call("re", "compile", &[py_str("\\b(?:sample|samples|test|tests|tested|testing|kit|pot|container|form|forms)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|something)\\b[^.!?]{0,26}?\\b(?:helps?|works?|should i (?:take|do)|can i take|do i take|do for|to take|to do)\\b|\\bhow (?:do|can) i (?:get rid of|stop|shift|ease)\\b|\\bany (?:advice|tips|ideas)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOOSE_STOOL(stdlib_call("re", "compile", &[py_str("\\b(?:loose|runny|watery|liquid|soft)\\s+(?:stools?|poos?|poops?|bowels?)\\b|\\b(?:stools?|poos?|poops?|bowels?)\\b[^.!?]{0,24}?\\b(?:loose|runny|watery|liquid|like water)\\b|\\bthe (?:trots|shits|squits|skitters|squirts)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_IS_TERRIBLE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|be|being|been|seems?|looks?|feels?|gone|go(?:es|ing)?|went)\\s+(?:to\\s+|a (?:bit |total |right |proper |complete |real |crock )?of\\s+|such\\s+|right\\s+|proper\\s+|complete\\s+)?the (?:shits|trots|squits|skitters|squirts)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHAIR_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:leg|legs|joint|joints|seat|seats|wood|wooden|screw|screws|screwdriver|tighten|tight|wobbly|wobble|wobbles|rickety|bracket|glue|chair|chairs|table|tables|bench|furniture|dresser|sideboard|kitchen|carpenter|flat ?pack)\\b|\\b(?:bar|counter|pub|breakfast|dining) ?stools?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GUT_ONLY(stdlib_call("re", "compile", &[py_str("\\b(?:runny|watery|liquid|soft|hard|firm|pale|smelly|stinking|streaked|streaks|slimy|mucus|diarr?hoea|constipat\\w*|tummy|stomach|gut|guts|ache|aches|aching|pain|pains|painful|cramps?|cramping|sore|wind|bloat\\w*|toilet|loo|bum|poo|poos|pooed|pooping|bowels?|sick|nausea|nauseous|vomit\\w*|days?|weeks?|months?|since|morning|mornings|night|nights|yesterday|today|times|a day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POO_COLOUR(stdlib_call("re", "compile", &[py_str("\\b(?:pale|black|tarry|bloody|blood|red|floats?|green|colour|color|mucus|slimy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PART_NOUNS_BY_LENGTH(py_dict(vec![]));
    let __iter1 = py_iter_or_raise(&g_PARTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    s__part_name(__unpk2[0].clone());
    s__part_pattern(__unpk2[1].clone());
    s__part_nouns(__unpk2[2].clone());
    let __iter3 = py_iter_or_raise(&g__part_nouns());
    for __item in __iter3 {
    s__noun(__item.clone());
    let _ = mcall(&mcall(&g_PART_NOUNS_BY_LENGTH(), "setdefault", &[call_builtin("len", &[g__noun()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), "add", &[g__noun()]);
    }
    }
    s_A_WORD_OF_ITS_OWN(call_builtin("frozenset", &[mcall(&py_str("\n    bella bell bully chesty chess earns glut glum heady heal heap heat heed tommy threat\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BEFORE_THE_COMPLAINT(call_builtin("frozenset", &[mcall(&py_str("i we am is are was were be been being have has had got\n    get gets getting keep keeps kept stop stops stopped can cannot cant can't could couldnt\n    couldn't not dont don't never still just only also really so very badly feel feels felt\n    again night days woke up started begin began"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MAYBE(call_builtin("frozenset", &[mcall(&py_str("could might may would should must can"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NO(call_builtin("frozenset", &[mcall(&py_str("not never cant cannot couldnt couldn't didnt didn't"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("am"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("being"), py_str("have"), py_str("has"), py_str("had"), py_str("ve"), py_str("feel"), py_str("feels"), py_str("felt"), py_str("keep"), py_str("keeps")]) }
    s__A_VERB_NEEDING_ITS_SUBJECT(__data4());
    s_SPEAKER_ILL(stdlib_call("re", "compile", &[py_str("\\b(?:i|we) (?:both |also |all )?(?:have|had|has|got|get|getting|having|been|being|keep|keeps|keeping|am|was|are|were|feel|feeling|felt|suffer\\w*|with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the my some this that bit of a really very so proper proper\n    bad worse worst sore slight terrible awful horrible nasty mild chronic another again\n    another little quite too pretty still"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_SKIP(call_builtin("frozenset", &[py_tuple(vec![py_str("chest"), py_str("temperature")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_SKIP_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("jaw")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_PAIN(op_bitor(&call_builtin("frozenset", &[pyrt::xrefs::xget("understanding.lexicon", "PAIN_WORDS")]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[py_tuple(vec![py_str("bad"), py_str("cramping"), py_str("cramps"), py_str("upset")])]).unwrap_or_else(|e| exc::raise(e))));
    s_WHEN_IT_HAPPENED(call_builtin("frozenset", &[mcall(&py_str("today tonight yesterday morning mornings afternoon afternoons\n    evening evenings night nights week weeks weekend weekends month months day days all since\n    last this now again every hour hours minute minutes am pm o clock time times midnight noon\n    monday tuesday wednesday thursday friday saturday sunday once twice three four five six ten\n    couple few several"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HELPER_BEFORE(call_builtin("frozenset", &[mcall(&py_str("be been being am is are was were keep keeps kept still just"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIN_ADJECTIVE(call_builtin("frozenset", &[mcall(&py_str("sore painful aching achy bad swollen cramping crampy upset stiff\n    throbbing stinging burning numb rough dodgy awful terrible horrible rubbish queasy\n    trouble troubles"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIN_BETWEEN(call_builtin("frozenset", &[mcall(&py_str("is am are was were be been being feels feel felt gets getting got\n    gone keep keeps kept very really so too quite a bit little just now still often always"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIN_REACH(Py::Int(3));
    s_POSSESSIVE(call_builtin("frozenset", &[mcall(&py_str("my his her their our your"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_UNWELL(call_builtin("frozenset", &[mcall(&py_str("off rough bad awful terrible horrible rubbish dodgy queasy\n    trouble troubles unhappy wrong grim"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_MAY_GO_ON(call_builtin("frozenset", &[mcall(&py_str("and but for since today tonight now still again all most\n    lately recently days weeks months hours mornings afternoons evenings nights every\n    couple few several very really quite bit little"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIN_PHRASE(py_tuple(vec![py_tuple(vec![py_str("playing"), py_str("up")]), py_tuple(vec![py_str("acting"), py_str("up")]), py_tuple(vec![py_str("in"), py_str("knots")]), py_tuple(vec![py_str("in"), py_str("bits")]), py_tuple(vec![py_str("in"), py_str("a"), py_str("knot")]), py_tuple(vec![py_str("all"), py_str("over"), py_str("the"), py_str("place")]), py_tuple(vec![py_str("giving"), py_str("me"), py_str("grief")]), py_tuple(vec![py_str("giving"), py_str("me"), py_str("jip")])]));
}
pub fn _an_abscess_elsewhere(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line's abscess is not one the tooth pool should answer.\n\n    Three ways to say so. It is at another part of the body (\"an abscess on my leg\", \"an abscess\n    on my back\"); it is already dealt with (\"i had an abscess drained at the hospital\", \"my\n    abscess burst\"); or it is a mouth abscess, which is a spreading infection with its own\n    urgent answer (\"a gum abscess\", \"a tooth abscess\"), not pain to book an appointment for.\n    A line that says none of those - `i think i have an abscess` - is the tooth pool's.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AN_ABSCESS(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_A_MOUTH_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_SPREADING_SIGN(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ANOTHER_PART(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ALREADY_DEALT_WITH(), "search", &[text.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _one_letter_apart(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut spelled: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spelled") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spelled"))) };
    let mut at: Py = Py::None;
    let mut long_: Py = Py::None;
    let mut short: Py = Py::None;
    let mut spelled_at: Py = Py::None;
    let mut typed_at: Py = Py::None;
    let _ = py_str("True when the word is the part with one letter added, left out, or put in another's place.");
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[spelled.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[spelled.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(py_eq(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[typed.clone(), spelled.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk7027 = py_unpack(&__item.clone(), 2, None); let mut typed_at: Py = __unpk7027[0].clone(); let mut spelled_at: Py = __unpk7027[1].clone(); if py_truthy(&Py::Bool(!py_eq(&typed_at.clone(), &spelled_at.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![typed.clone(), spelled.clone()])); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 2, None);
    short = __unpk1[0].clone();
    long_ = __unpk1[1].clone();
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[long_.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); __out.push(Py::Bool(py_eq(&op_add(&slice_or_raise(&long_.clone(), &Py::None, &at.clone(), &Py::None), &slice_or_raise(&long_.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)), &short.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_part_typed_with_one_letter_wrong(args: &[Py]) -> Py {
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
    let mut fits: Py = Py::None;
    let mut length: Py = Py::None;
    let mut noun: Py = Py::None;
    let _ = py_str("The one part noun this word is a letter away from, or None when it is none or more than one.");
    fits = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), op_add(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]));
    for __item in __iter1 {
    length = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_PART_NOUNS_BY_LENGTH(), "get", &[length.clone(), py_tuple(vec![])]));
    for __item in __iter2 {
    noun = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&noun.clone(), &Py::Int(0)), &geti(&word.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { _one_letter_apart(&[word.clone(), noun.clone()]) } }) {
    let _ = mcall(&fits.clone(), "add", &[noun.clone()]);
    if py_truthy(&op_gt(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::None;
    }
    }
    }
    }
    return if py_truthy(&fits.clone()) { mcall(&fits.clone(), "pop", &[]) } else { Py::None };
    Py::None
}
pub fn _a_word_rather_than_a_typo(args: &[Py]) -> Py {
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
    let mut CONDITIONS: Py = Py::None;
    let _ = py_str("True when the word is a word already, so it is never read as a part typed wrong.\n\n    The conditions list is imported here rather than at the top because `topics` imports this\n    module for its gut words, and the reading is only ever asked for a word the lexicon has\n    never seen, which is rare enough for the import to be free.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.normalize", "LEXICON"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_WORD_OF_ITS_OWN(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    CONDITIONS = pyrt::xrefs::xget("understanding.topics", "CONDITIONS");
    return Py::Bool(py_contains(&CONDITIONS.clone(), &word.clone()));
    Py::None
}
pub fn _spell_the_parts(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line with a part name typed one letter wrong spelled the way the tables have it.");
    repaired = text.clone();
    let __iter1 = py_iter_or_raise(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { _a_word_rather_than_a_typo(&[word.clone()]) } }) {
    continue;
    }
    part = _a_part_typed_with_one_letter_wrong(&[word.clone()]);
    if py_truthy(&part.clone()) {
    repaired = stdlib_call("re", "sub", &[op_modulo(&py_str("\\b%s\\b"), &word.clone()), part.clone(), repaired.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return repaired.clone();
    Py::None
}
pub fn _the_gut_is_settled(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True unless the only word for the gut here is \"stool\" and nothing else settles it.\n\n    \"my stool is loose\" is a chair with a loose leg as easily as a body line, and both words in\n    it are the chair's too, so the loose rule reads the line only when something in it can be\n    nothing but the gut (\"loose stools for 3 days\", \"soft stools and stomach ache\", \"runny\n    stools\"). A line with only the shareable words is the furniture, which is what the person\n    owning a stool in their kitchen means by it.\n    ");
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("stool"), py_str("stools")]))))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[op_bitand(&words.clone(), &call_builtin("set", &[mcall(&g_A_GUT_ONLY(), "findall", &[mcall(&text.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut nouns: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut sample: Py = Py::None;
    let _ = py_str("The part of the body the speaker says is unwell, or None.\n\n    `text` is the line, lowercased and with the typed short forms spelled out.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    text = _spell_the_parts(&[mcall(&text.clone(), "replace", &[py_str("|"), py_str(" ")])]);
    if py_truthy(&{ let __b = mcall(&g_A_LOOSE_STOOL(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_SAMPLE(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_POO_COLOUR(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_CHAIR_WORD(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_THING_IS_TERRIBLE(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_someone_else(&[text.clone()]))); if !py_truthy(&__b) { __b } else { _the_gut_is_settled(&[text.clone()]) } } } } } } } } } } } }) {
    return py_str("stomach");
    }
    sample = call_builtin("bool", &[mcall(&g_A_SAMPLE(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_PARTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    nouns = __unpk2[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&name.clone(), &py_str("teeth"))); if !py_truthy(&__b) { __b } else { _an_abscess_elsewhere(&[text.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&name.clone(), &py_str("stomach"))); if !py_truthy(&__b) { __b } else { sample.clone() } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "finditer", &[pattern.clone(), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    match_ = __item.clone();
    if py_truthy(&_the_speakers(&[text.clone(), mcall(&match_.clone(), "end", &[]), nouns.clone(), mcall(&match_.clone(), "start", &[])])) {
    return name.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_ASKING(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_someone_else(&[text.clone()]))) } }) {
    return name.clone();
    }
    if py_truthy(&{ let __b = _just_the_part(&[text.clone(), match_.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_someone_else(&[text.clone()]))) } }) {
    return name.clone();
    }
    }
    }
    return _said_about_itself(&[text.clone()]);
    Py::None
}
pub fn _the_speakers(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut nouns: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "nouns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: nouns"))) };
    let mut began: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "began") { v.clone() }
    else { Py::None };
    let mut index: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut start: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words in front of the part make the illness the speaker's own.\n\n    \"my head is pounding\" and \"i have a fever\" are the speaker's. \"my son has a fever\" is the\n    child's, \"she gets migraines\" is hers, and \"my room temperature is 40\" is a room's: a \"my\"\n    only makes the part the speaker's when the part itself comes next.\n\n    `began` is where the match starts, when the caller knows it: the words between the speaker\n    and the complaint are then read too, for the complaints the person says with the verb as\n    the whole predicate (\"i threw up twice\", \"i vomited\", \"i cant stop being sick\").\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&text.clone(), &Py::None, &at.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    owner = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("my"))) } }) {
    owner = py_tuple(vec![word.clone(), index.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_is(&owner.clone(), &Py::None))) {
    let __unpk3 = py_unpack(&owner.clone(), 2, None);
    word = __unpk3[0].clone();
    index = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("my")))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&nouns.clone(), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))))) } };
    }
    start = call_builtin("max", &[Py::Int(0), op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = op_gt(&start.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_VERB_NEEDING_ITS_SUBJECT(), &geti(&words.clone(), &start.clone()))) } }) {
    let __aug4 = op_sub(&start.clone(), &Py::Int(1));
    start = __aug4;
    }
    tail = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &start.clone(), &Py::None, &Py::None)]);
    tail = stdlib_call("re", "sub", &[py_str("\\bi'?m\\b"), py_str("i am"), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    tail = stdlib_call("re", "sub", &[py_str("\\bi'?ve\\b"), py_str("i have"), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    tail = stdlib_call("re", "sub", &[py_str("\\bwe'?re\\b"), py_str("we are"), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    tail = stdlib_call("re", "sub", &[py_str("\\bwe'?ve\\b"), py_str("we have"), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_SPEAKER_ILL(), "search", &[tail.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("understanding.kinds", "_speaker_before", &[text.clone(), at.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_is(&began.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    return _the_speaker_is_the_subject(&[words.clone(), call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&text.clone(), &Py::None, &began.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _only_the_fillers_between(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut between: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "between") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: between"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when nothing but a helper, a negation or an adverb stands between speaker and part.");
    if py_truthy(&{ let __b = between.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_MAYBE(), &geti(&between.clone(), &Py::Int(0)))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_gt(&call_builtin("len", &[between.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_NO(), &geti(&between.clone(), &Py::Int(1)))) } }))) {
    return Py::Bool(false);
    }
    between = slice_or_raise(&between.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__BEFORE_THE_COMPLAINT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_speaker_is_the_subject(args: &[Py]) -> Py {
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
    let mut began: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "began") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: began"))) };
    let mut index: Py = Py::None;
    let _ = py_str("True when a first-person subject is the one the complaint is about.\n\n    \"i threw up twice last night\" and \"i vomited\" carry no helper the illness window knows, so\n    the subject was not found and the line was read as naming nobody: a person who had been\n    sick all night got the sleep pool. The first person in front of the complaint, with nothing\n    but a negation, a helper or an adverb between, is the person it happened to.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("i"), py_str("we")]), &geti(&words.clone(), &index.clone())))) {
    continue;
    }
    if py_truthy(&_only_the_fillers_between(&[slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &began.clone(), &Py::None)])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _an_unwell_word(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when this word says the part before it is unwell, not that it is a preposition.");
    word = geti(&words.clone(), &at.clone());
    if py_truthy(&Py::Bool(py_contains(&g_PLAIN_PAIN(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_PLAIN_UNWELL(), &word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("off")))) {
    return Py::Bool(true);
    }
    return { let __b = op_ge(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OFF_MAY_GO_ON(), &geti(&words.clone(), &op_add(&at.clone(), &Py::Int(1))))) } };
    Py::None
}
pub fn _pain_after(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut at: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("True when the part is what the word of pain is about: \"belly hurts\", \"throat is sore\".");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_add(&g_PAIN_REACH(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    at = op_add(&index.clone(), &step.clone());
    if py_truthy(&op_ge(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(false);
    }
    if py_truthy(&_an_unwell_word(&[words.clone(), at.clone()])) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&words.clone(), &at.clone()), &py_str("off"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POSSESSIVE(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } })) } }) {
    break;
    }
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_PAIN_BETWEEN(), &geti(&words.clone(), &at.clone())))) {
    break;
    }
    }
    return _phrase_after(&[words.clone(), index.clone()]);
    Py::None
}
pub fn _phrase_after(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut at: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when one of the phrases that says a part is unwell comes right after it.");
    let __iter1 = py_iter_or_raise(&g_PAIN_PHRASE());
    for __item in __iter1 {
    phrase = __item.clone();
    at = op_add(&index.clone(), &Py::Int(1));
    let __iter2 = py_iter_or_raise(&phrase.clone());
    let mut __else3 = false;
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PAIN_BETWEEN(), &geti(&words.clone(), &at.clone()))) } }) {
    let __aug4 = op_add(&at.clone(), &Py::Int(1));
    at = __aug4;
    }
    if py_truthy(&{ let __b = op_ge(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&words.clone(), &at.clone()), &word.clone())) } }) {
    break;
    }
    let __aug5 = op_add(&at.clone(), &Py::Int(1));
    at = __aug5;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_about_itself(args: &[Py]) -> Py {
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
    let mut _pattern: Py = Py::None;
    let mut found: Py = Py::None;
    let mut in_front: Py = Py::None;
    let mut index: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut nouns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The part from a row's own noun and a word of pain beside it, or None.\n\n    Whose it is, is the same question the patterns ask: the speaker's own part is read, and a\n    part that belongs to somebody else (\"my son has a bad tummy\", \"my husband has a sore\n    throat\") is theirs. A pain word in front of the part says the part itself is unwell, so\n    the rest of that line can say anything; a part that the pain follows has to be the\n    speaker's own (\"my belly hurts\"), or the whole line the complaint (\"belly ache\").\n    ");
    found = call_builtin("list", &[stdlib_call("re", "finditer", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut match_: Py = __item.clone(); __out.push(mcall(&match_.clone(), "group", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&g_PARTS());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 3, None);
    name = __unpk4[0].clone();
    _pattern = __unpk4[1].clone();
    nouns = __unpk4[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLAIN_SKIP(), &name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLAIN_SKIP_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_part_word(&[word.clone(), nouns.clone()]))) } } } }) {
    continue;
    }
    in_front = { let __b = op_gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PAIN_ADJECTIVE(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = in_front.clone(); if py_truthy(&__b) { __b } else { _pain_after(&[words.clone(), index.clone()]) } }))) {
    continue;
    }
    if py_truthy(&_the_speakers(&[text.clone(), mcall(&geti(&found.clone(), &index.clone()), "end", &[]), nouns.clone(), mcall(&geti(&found.clone(), &index.clone()), "start", &[])])) {
    return name.clone();
    }
    if py_truthy(&{ let __b = op_gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("me")]), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    return name.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { _pain_after(&[words.clone(), index.clone()]) } }) {
    return name.clone();
    }
    if py_truthy(&_someone_else(&[text.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = in_front.clone(); if py_truthy(&__b) { __b } else { _just_the_part_words(&[words.clone(), index.clone()]) } }) {
    return name.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_part_word(args: &[Py]) -> Py {
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
    let mut nouns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "nouns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: nouns"))) };
    let mut bare: Py = Py::None;
    let _ = py_str("The row's noun this word is, or None. A missing apostrophe and a plural both count:\n    \"bellys bad\", \"tummys rough\", \"my guts are in knots\".");
    if py_truthy(&Py::Bool(py_contains(&nouns.clone(), &word.clone()))) {
    return word.clone();
    }
    bare = word.clone();
    if py_truthy(&mcall(&bare.clone(), "endswith", &[py_str("'s")])) {
    bare = slice_or_raise(&bare.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    } else {
    if py_truthy(&mcall(&bare.clone(), "endswith", &[py_str("s")])) {
    bare = slice_or_raise(&bare.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    }
    return if py_truthy(&Py::Bool(py_contains(&nouns.clone(), &bare.clone()))) { bare.clone() } else { Py::None };
    Py::None
}
pub fn _just_the_part_words(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut allowed: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the part and words of pain are all the line says (\"sore tooth\", \"bad belly\").");
    rest = op_add(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None), &slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None));
    allowed = op_bitor(&op_bitor(&op_bitor(&g_FILLER(), &g_PLAIN_PAIN()), &g_PLAIN_UNWELL()), &g_PAIN_BETWEEN());
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&allowed.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _just_the_part(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the part and when it happened are all the line says: \"headache all week\".\n\n    A complaint with the front cut off is still the speaker's own when the rest of the line\n    only says when it happened (\"throwing up all night\", \"sore throat since tuesday\", \"sick\n    twice today\"): the time says when, not whose, and nobody describes somebody else's night\n    that way without naming them.\n    ");
    rest = mcall(&op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WHEN_IT_HAPPENED(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__A_HELPER_BEFORE(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "isdigit", &[]) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _someone_else(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when a person other than the speaker is named anywhere in the line.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "OTHER_PEOPLE"), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.body".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALREADY_DEALT_WITH" => Some(g_ALREADY_DEALT_WITH()),
            "ANOTHER_PART" => Some(g_ANOTHER_PART()),
            "AN_ABSCESS" => Some(g_AN_ABSCESS()),
            "ASKING" => Some(g_ASKING()),
            "A_CHAIR_WORD" => Some(g_A_CHAIR_WORD()),
            "A_GUT_ONLY" => Some(g_A_GUT_ONLY()),
            "A_LOOSE_STOOL" => Some(g_A_LOOSE_STOOL()),
            "A_MOUTH_WORD" => Some(g_A_MOUTH_WORD()),
            "A_POO_COLOUR" => Some(g_A_POO_COLOUR()),
            "A_SAMPLE" => Some(g_A_SAMPLE()),
            "A_SPREADING_SIGN" => Some(g_A_SPREADING_SIGN()),
            "A_THING_IS_TERRIBLE" => Some(g_A_THING_IS_TERRIBLE()),
            "A_WORD_OF_ITS_OWN" => Some(g_A_WORD_OF_ITS_OWN()),
            "COMMON_WORDS" => Some(g_COMMON_WORDS()),
            "FILLER" => Some(g_FILLER()),
            "GOING_BOTH_WAYS" => Some(g_GOING_BOTH_WAYS()),
            "OFF_MAY_GO_ON" => Some(g_OFF_MAY_GO_ON()),
            "PAIN_ADJECTIVE" => Some(g_PAIN_ADJECTIVE()),
            "PAIN_BETWEEN" => Some(g_PAIN_BETWEEN()),
            "PAIN_PHRASE" => Some(g_PAIN_PHRASE()),
            "PAIN_REACH" => Some(g_PAIN_REACH()),
            "PARTS" => Some(g_PARTS()),
            "PART_NOUNS_BY_LENGTH" => Some(g_PART_NOUNS_BY_LENGTH()),
            "PLAIN_PAIN" => Some(g_PLAIN_PAIN()),
            "PLAIN_SKIP" => Some(g_PLAIN_SKIP()),
            "PLAIN_SKIP_WORDS" => Some(g_PLAIN_SKIP_WORDS()),
            "PLAIN_UNWELL" => Some(g_PLAIN_UNWELL()),
            "POSSESSIVE" => Some(g_POSSESSIVE()),
            "SPEAKER_ILL" => Some(g_SPEAKER_ILL()),
            "WHEN_IT_HAPPENED" => Some(g_WHEN_IT_HAPPENED()),
            "_ALTERNATING" => Some(g__ALTERNATING()),
            "_A_HELPER_BEFORE" => Some(g__A_HELPER_BEFORE()),
            "_A_MAYBE" => Some(g__A_MAYBE()),
            "_A_NO" => Some(g__A_NO()),
            "_A_VERB_NEEDING_ITS_SUBJECT" => Some(g__A_VERB_NEEDING_ITS_SUBJECT()),
            "_BEFORE_THE_COMPLAINT" => Some(g__BEFORE_THE_COMPLAINT()),
            "_CHUCK_UP" => Some(g__CHUCK_UP()),
            "_COLD_SIDE" => Some(g__COLD_SIDE()),
            "_FILL" => Some(g__FILL()),
            "_FROM_TO" => Some(g__FROM_TO()),
            "_HOT_SIDE" => Some(g__HOT_SIDE()),
            "_JOIN" => Some(g__JOIN()),
            "_a_part_typed_with_one_letter_wrong" => Some(py_func("_a_part_typed_with_one_letter_wrong", |args: &[Py]| _a_part_typed_with_one_letter_wrong(args))),
            "_a_word_rather_than_a_typo" => Some(py_func("_a_word_rather_than_a_typo", |args: &[Py]| _a_word_rather_than_a_typo(args))),
            "_an_abscess_elsewhere" => Some(py_func("_an_abscess_elsewhere", |args: &[Py]| _an_abscess_elsewhere(args))),
            "_an_unwell_word" => Some(py_func("_an_unwell_word", |args: &[Py]| _an_unwell_word(args))),
            "_just_the_part" => Some(py_func("_just_the_part", |args: &[Py]| _just_the_part(args))),
            "_just_the_part_words" => Some(py_func("_just_the_part_words", |args: &[Py]| _just_the_part_words(args))),
            "_noun" => Some(g__noun()),
            "_one_letter_apart" => Some(py_func("_one_letter_apart", |args: &[Py]| _one_letter_apart(args))),
            "_only_the_fillers_between" => Some(py_func("_only_the_fillers_between", |args: &[Py]| _only_the_fillers_between(args))),
            "_pain_after" => Some(py_func("_pain_after", |args: &[Py]| _pain_after(args))),
            "_part_name" => Some(g__part_name()),
            "_part_nouns" => Some(g__part_nouns()),
            "_part_pattern" => Some(g__part_pattern()),
            "_phrase_after" => Some(py_func("_phrase_after", |args: &[Py]| _phrase_after(args))),
            "_said_about_itself" => Some(py_func("_said_about_itself", |args: &[Py]| _said_about_itself(args))),
            "_someone_else" => Some(py_func("_someone_else", |args: &[Py]| _someone_else(args))),
            "_spell_the_parts" => Some(py_func("_spell_the_parts", |args: &[Py]| _spell_the_parts(args))),
            "_the_gut_is_settled" => Some(py_func("_the_gut_is_settled", |args: &[Py]| _the_gut_is_settled(args))),
            "_the_part_word" => Some(py_func("_the_part_word", |args: &[Py]| _the_part_word(args))),
            "_the_speaker_is_the_subject" => Some(py_func("_the_speaker_is_the_subject", |args: &[Py]| _the_speaker_is_the_subject(args))),
            "_the_speakers" => Some(py_func("_the_speakers", |args: &[Py]| _the_speakers(args))),
            "read" => Some(py_func("read", |args: &[Py]| read(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALREADY_DEALT_WITH" => s_ALREADY_DEALT_WITH(v),
            "ANOTHER_PART" => s_ANOTHER_PART(v),
            "AN_ABSCESS" => s_AN_ABSCESS(v),
            "ASKING" => s_ASKING(v),
            "A_CHAIR_WORD" => s_A_CHAIR_WORD(v),
            "A_GUT_ONLY" => s_A_GUT_ONLY(v),
            "A_LOOSE_STOOL" => s_A_LOOSE_STOOL(v),
            "A_MOUTH_WORD" => s_A_MOUTH_WORD(v),
            "A_POO_COLOUR" => s_A_POO_COLOUR(v),
            "A_SAMPLE" => s_A_SAMPLE(v),
            "A_SPREADING_SIGN" => s_A_SPREADING_SIGN(v),
            "A_THING_IS_TERRIBLE" => s_A_THING_IS_TERRIBLE(v),
            "A_WORD_OF_ITS_OWN" => s_A_WORD_OF_ITS_OWN(v),
            "COMMON_WORDS" => s_COMMON_WORDS(v),
            "FILLER" => s_FILLER(v),
            "GOING_BOTH_WAYS" => s_GOING_BOTH_WAYS(v),
            "OFF_MAY_GO_ON" => s_OFF_MAY_GO_ON(v),
            "PAIN_ADJECTIVE" => s_PAIN_ADJECTIVE(v),
            "PAIN_BETWEEN" => s_PAIN_BETWEEN(v),
            "PAIN_PHRASE" => s_PAIN_PHRASE(v),
            "PAIN_REACH" => s_PAIN_REACH(v),
            "PARTS" => s_PARTS(v),
            "PART_NOUNS_BY_LENGTH" => s_PART_NOUNS_BY_LENGTH(v),
            "PLAIN_PAIN" => s_PLAIN_PAIN(v),
            "PLAIN_SKIP" => s_PLAIN_SKIP(v),
            "PLAIN_SKIP_WORDS" => s_PLAIN_SKIP_WORDS(v),
            "PLAIN_UNWELL" => s_PLAIN_UNWELL(v),
            "POSSESSIVE" => s_POSSESSIVE(v),
            "SPEAKER_ILL" => s_SPEAKER_ILL(v),
            "WHEN_IT_HAPPENED" => s_WHEN_IT_HAPPENED(v),
            "_ALTERNATING" => s__ALTERNATING(v),
            "_A_HELPER_BEFORE" => s__A_HELPER_BEFORE(v),
            "_A_MAYBE" => s__A_MAYBE(v),
            "_A_NO" => s__A_NO(v),
            "_A_VERB_NEEDING_ITS_SUBJECT" => s__A_VERB_NEEDING_ITS_SUBJECT(v),
            "_BEFORE_THE_COMPLAINT" => s__BEFORE_THE_COMPLAINT(v),
            "_CHUCK_UP" => s__CHUCK_UP(v),
            "_COLD_SIDE" => s__COLD_SIDE(v),
            "_FILL" => s__FILL(v),
            "_FROM_TO" => s__FROM_TO(v),
            "_HOT_SIDE" => s__HOT_SIDE(v),
            "_JOIN" => s__JOIN(v),
            "_noun" => s__noun(v),
            "_part_name" => s__part_name(v),
            "_part_nouns" => s__part_nouns(v),
            "_part_pattern" => s__part_pattern(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.body", module_obj);
    pyrt::xrefs::reg_fn("understanding.body", "_an_abscess_elsewhere", py_func("_an_abscess_elsewhere", |args: &[Py]| _an_abscess_elsewhere(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_one_letter_apart", py_func("_one_letter_apart", |args: &[Py]| _one_letter_apart(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_a_part_typed_with_one_letter_wrong", py_func("_a_part_typed_with_one_letter_wrong", |args: &[Py]| _a_part_typed_with_one_letter_wrong(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_a_word_rather_than_a_typo", py_func("_a_word_rather_than_a_typo", |args: &[Py]| _a_word_rather_than_a_typo(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_spell_the_parts", py_func("_spell_the_parts", |args: &[Py]| _spell_the_parts(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_the_gut_is_settled", py_func("_the_gut_is_settled", |args: &[Py]| _the_gut_is_settled(args)));
    pyrt::xrefs::reg_fn("understanding.body", "read", py_func("read", |args: &[Py]| read(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_the_speakers", py_func("_the_speakers", |args: &[Py]| _the_speakers(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_only_the_fillers_between", py_func("_only_the_fillers_between", |args: &[Py]| _only_the_fillers_between(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_the_speaker_is_the_subject", py_func("_the_speaker_is_the_subject", |args: &[Py]| _the_speaker_is_the_subject(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_an_unwell_word", py_func("_an_unwell_word", |args: &[Py]| _an_unwell_word(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_pain_after", py_func("_pain_after", |args: &[Py]| _pain_after(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_phrase_after", py_func("_phrase_after", |args: &[Py]| _phrase_after(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_said_about_itself", py_func("_said_about_itself", |args: &[Py]| _said_about_itself(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_the_part_word", py_func("_the_part_word", |args: &[Py]| _the_part_word(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_just_the_part_words", py_func("_just_the_part_words", |args: &[Py]| _just_the_part_words(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_just_the_part", py_func("_just_the_part", |args: &[Py]| _just_the_part(args)));
    pyrt::xrefs::reg_fn("understanding.body", "_someone_else", py_func("_someone_else", |args: &[Py]| _someone_else(args)));
    pyrt::xrefs::reg_access("understanding.body", "ALREADY_DEALT_WITH", g_ALREADY_DEALT_WITH, s_ALREADY_DEALT_WITH);
    pyrt::xrefs::reg_access("understanding.body", "ANOTHER_PART", g_ANOTHER_PART, s_ANOTHER_PART);
    pyrt::xrefs::reg_access("understanding.body", "AN_ABSCESS", g_AN_ABSCESS, s_AN_ABSCESS);
    pyrt::xrefs::reg_access("understanding.body", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("understanding.body", "A_CHAIR_WORD", g_A_CHAIR_WORD, s_A_CHAIR_WORD);
    pyrt::xrefs::reg_access("understanding.body", "A_GUT_ONLY", g_A_GUT_ONLY, s_A_GUT_ONLY);
    pyrt::xrefs::reg_access("understanding.body", "A_LOOSE_STOOL", g_A_LOOSE_STOOL, s_A_LOOSE_STOOL);
    pyrt::xrefs::reg_access("understanding.body", "A_MOUTH_WORD", g_A_MOUTH_WORD, s_A_MOUTH_WORD);
    pyrt::xrefs::reg_access("understanding.body", "A_POO_COLOUR", g_A_POO_COLOUR, s_A_POO_COLOUR);
    pyrt::xrefs::reg_access("understanding.body", "A_SAMPLE", g_A_SAMPLE, s_A_SAMPLE);
    pyrt::xrefs::reg_access("understanding.body", "A_SPREADING_SIGN", g_A_SPREADING_SIGN, s_A_SPREADING_SIGN);
    pyrt::xrefs::reg_access("understanding.body", "A_THING_IS_TERRIBLE", g_A_THING_IS_TERRIBLE, s_A_THING_IS_TERRIBLE);
    pyrt::xrefs::reg_access("understanding.body", "A_WORD_OF_ITS_OWN", g_A_WORD_OF_ITS_OWN, s_A_WORD_OF_ITS_OWN);
    pyrt::xrefs::reg_access("understanding.body", "COMMON_WORDS", g_COMMON_WORDS, s_COMMON_WORDS);
    pyrt::xrefs::reg_access("understanding.body", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("understanding.body", "GOING_BOTH_WAYS", g_GOING_BOTH_WAYS, s_GOING_BOTH_WAYS);
    pyrt::xrefs::reg_access("understanding.body", "OFF_MAY_GO_ON", g_OFF_MAY_GO_ON, s_OFF_MAY_GO_ON);
    pyrt::xrefs::reg_access("understanding.body", "PAIN_ADJECTIVE", g_PAIN_ADJECTIVE, s_PAIN_ADJECTIVE);
    pyrt::xrefs::reg_access("understanding.body", "PAIN_BETWEEN", g_PAIN_BETWEEN, s_PAIN_BETWEEN);
    pyrt::xrefs::reg_access("understanding.body", "PAIN_PHRASE", g_PAIN_PHRASE, s_PAIN_PHRASE);
    pyrt::xrefs::reg_access("understanding.body", "PAIN_REACH", g_PAIN_REACH, s_PAIN_REACH);
    pyrt::xrefs::reg_access("understanding.body", "PARTS", g_PARTS, s_PARTS);
    pyrt::xrefs::reg_access("understanding.body", "PART_NOUNS_BY_LENGTH", g_PART_NOUNS_BY_LENGTH, s_PART_NOUNS_BY_LENGTH);
    pyrt::xrefs::reg_access("understanding.body", "PLAIN_PAIN", g_PLAIN_PAIN, s_PLAIN_PAIN);
    pyrt::xrefs::reg_access("understanding.body", "PLAIN_SKIP", g_PLAIN_SKIP, s_PLAIN_SKIP);
    pyrt::xrefs::reg_access("understanding.body", "PLAIN_SKIP_WORDS", g_PLAIN_SKIP_WORDS, s_PLAIN_SKIP_WORDS);
    pyrt::xrefs::reg_access("understanding.body", "PLAIN_UNWELL", g_PLAIN_UNWELL, s_PLAIN_UNWELL);
    pyrt::xrefs::reg_access("understanding.body", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("understanding.body", "SPEAKER_ILL", g_SPEAKER_ILL, s_SPEAKER_ILL);
    pyrt::xrefs::reg_access("understanding.body", "WHEN_IT_HAPPENED", g_WHEN_IT_HAPPENED, s_WHEN_IT_HAPPENED);
    pyrt::xrefs::reg_access("understanding.body", "_ALTERNATING", g__ALTERNATING, s__ALTERNATING);
    pyrt::xrefs::reg_access("understanding.body", "_A_HELPER_BEFORE", g__A_HELPER_BEFORE, s__A_HELPER_BEFORE);
    pyrt::xrefs::reg_access("understanding.body", "_A_MAYBE", g__A_MAYBE, s__A_MAYBE);
    pyrt::xrefs::reg_access("understanding.body", "_A_NO", g__A_NO, s__A_NO);
    pyrt::xrefs::reg_access("understanding.body", "_A_VERB_NEEDING_ITS_SUBJECT", g__A_VERB_NEEDING_ITS_SUBJECT, s__A_VERB_NEEDING_ITS_SUBJECT);
    pyrt::xrefs::reg_access("understanding.body", "_BEFORE_THE_COMPLAINT", g__BEFORE_THE_COMPLAINT, s__BEFORE_THE_COMPLAINT);
    pyrt::xrefs::reg_access("understanding.body", "_CHUCK_UP", g__CHUCK_UP, s__CHUCK_UP);
    pyrt::xrefs::reg_access("understanding.body", "_COLD_SIDE", g__COLD_SIDE, s__COLD_SIDE);
    pyrt::xrefs::reg_access("understanding.body", "_FILL", g__FILL, s__FILL);
    pyrt::xrefs::reg_access("understanding.body", "_FROM_TO", g__FROM_TO, s__FROM_TO);
    pyrt::xrefs::reg_access("understanding.body", "_HOT_SIDE", g__HOT_SIDE, s__HOT_SIDE);
    pyrt::xrefs::reg_access("understanding.body", "_JOIN", g__JOIN, s__JOIN);
    pyrt::xrefs::reg_access("understanding.body", "_noun", g__noun, s__noun);
    pyrt::xrefs::reg_access("understanding.body", "_part_name", g__part_name, s__part_name);
    pyrt::xrefs::reg_access("understanding.body", "_part_nouns", g__part_nouns, s__part_nouns);
    pyrt::xrefs::reg_access("understanding.body", "_part_pattern", g__part_pattern, s__part_pattern);
}
