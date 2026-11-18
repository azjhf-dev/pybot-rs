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


thread_local! { static G_AND_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAVE_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLASSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MANY_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ABOUT_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALL_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILDREN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXTENDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRANDPARENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_IT_WENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_HAND_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PARTNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIBLINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__class: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__word: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__words: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AND_ANOTHER() -> Py { ensure_init(); G_AND_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_ANOTHER(v: Py) { ensure_init(); G_AND_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHOICE() -> Py { ensure_init(); G_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHOICE(v: Py) { ensure_init(); G_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAVE_RELATION() -> Py { ensure_init(); G_A_HAVE_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAVE_RELATION(v: Py) { ensure_init(); G_A_HAVE_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAST_ASK() -> Py { ensure_init(); G_A_PAST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAST_ASK(v: Py) { ensure_init(); G_A_PAST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION() -> Py { ensure_init(); G_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); G_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RELATION() -> Py { ensure_init(); G_A_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RELATION(v: Py) { ensure_init(); G_A_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_CLASSES() -> Py { ensure_init(); G_CLASSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASSES(v: Py) { ensure_init(); G_CLASSES.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MANY_RELATION() -> Py { ensure_init(); G_HOW_MANY_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_RELATION(v: Py) { ensure_init(); G_HOW_MANY_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ABOUT_THE_BOT() -> Py { ensure_init(); G_NOT_ABOUT_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_THE_BOT(v: Py) { ensure_init(); G_NOT_ABOUT_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_ONE() -> Py { ensure_init(); G_NO_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_ONE(v: Py) { ensure_init(); G_NO_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION() -> Py { ensure_init(); G_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION(v: Py) { ensure_init(); G_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_RELATION() -> Py { ensure_init(); G_THE_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_RELATION(v: Py) { ensure_init(); G_THE_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g__ALL_RELATIONS() -> Py { ensure_init(); G__ALL_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALL_RELATIONS(v: Py) { ensure_init(); G__ALL_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILDREN() -> Py { ensure_init(); G__CHILDREN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILDREN(v: Py) { ensure_init(); G__CHILDREN.with(|c| *c.borrow_mut() = v); }
pub fn g__EXTENDED() -> Py { ensure_init(); G__EXTENDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXTENDED(v: Py) { ensure_init(); G__EXTENDED.with(|c| *c.borrow_mut() = v); }
pub fn g__FAMILY() -> Py { ensure_init(); G__FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAMILY(v: Py) { ensure_init(); G__FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g__GRANDPARENTS() -> Py { ensure_init(); G__GRANDPARENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRANDPARENTS(v: Py) { ensure_init(); G__GRANDPARENTS.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_IT_WENT() -> Py { ensure_init(); G__HOW_IT_WENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_IT_WENT(v: Py) { ensure_init(); G__HOW_IT_WENT.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD_IN() -> Py { ensure_init(); G__LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD_IN(v: Py) { ensure_init(); G__LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_HAND_BACK() -> Py { ensure_init(); G__NO_HAND_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_HAND_BACK(v: Py) { ensure_init(); G__NO_HAND_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__PARENTS() -> Py { ensure_init(); G__PARENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARENTS(v: Py) { ensure_init(); G__PARENTS.with(|c| *c.borrow_mut() = v); }
pub fn g__PARTNER() -> Py { ensure_init(); G__PARTNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PARTNER(v: Py) { ensure_init(); G__PARTNER.with(|c| *c.borrow_mut() = v); }
pub fn g__SIBLINGS() -> Py { ensure_init(); G__SIBLINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIBLINGS(v: Py) { ensure_init(); G__SIBLINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PEOPLE() -> Py { ensure_init(); G__THE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PEOPLE(v: Py) { ensure_init(); G__THE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__class() -> Py { ensure_init(); G__class.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__class(v: Py) { ensure_init(); G__class.with(|c| *c.borrow_mut() = v); }
pub fn g__word() -> Py { ensure_init(); G__word.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__word(v: Py) { ensure_init(); G__word.with(|c| *c.borrow_mut() = v); }
pub fn g__words() -> Py { ensure_init(); G__words.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__words(v: Py) { ensure_init(); G__words.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about the bot's own family: there is no family to ask about.\n\nPeople ask the bot about a life of its own the way they would ask a friend - \"were your mum and\ndad there\", \"have you got any brothers\", \"what did your dad say\", \"are your aunt and uncle\ncoming\". The bot already has an honest answer for the singular shapes (\"do you have kids\" gets\n\"No family and no friends of my own - I'm a program\"), because those are the shapes an ask was\nwritten for. The plural, the past tense and the extended relations were read as questions about\nthe world instead, and went to the search hedge:\n\n    > were your mum and dad there\n    < I don't know enough to answer that honestly. A search should sort you out quickly\n    > have you got any brothers\n    < That's beyond what I know well, sorry. Searching for it will turn up better sources\n    > what did your dad say\n    < I'm not sure about that one, sorry - I'd rather not guess. A quick search online\n\nNo search engine knows whether the bot's parents were at anything, and a person being friendly\nis the last moment to answer with a placeholder.\n\nThe rule is the one the rest of the bot's own life is read by, said the other way round: a\nrelation is a fact of a life and never a thing of the world, so \"your\" in front of one is the\nask, whatever the verb is and whatever follows it. What keeps the person's own family out is\nthe question: an ask has a question word or an auxiliary at the front, or a question mark at the\nend. \"give your mum a ring\", \"ask your dad about it\" and \"i rang your mum yesterday\" are about\ntheir family and keep their own reading, and so does anything that names a thing of the world\n(\"does your gran want a curry\" is the curry's turn, at a priority above this one).\n\nThe reply names the relation that was asked about, says plainly there is none, and hands the\nturn back to their family, because that is what the person wanted to talk about (STYLE.md,\n\"Not knowing, honestly\": say what you can't do in a few words, then keep going).\n");
    s_PRIORITY(Py::Int(1100));
    s_ANSWERS(py_tuple(vec![py_str("about-the-bot")]));
    s__PARENTS(py_tuple(vec![py_str("mum"), py_str("mums"), py_str("mam"), py_str("mams"), py_str("mom"), py_str("moms"), py_str("mother"), py_str("mothers"), py_str("dad"), py_str("dads"), py_str("father"), py_str("fathers"), py_str("parent"), py_str("parents"), py_str("mummy"), py_str("daddy")]));
    s__GRANDPARENTS(py_tuple(vec![py_str("gran"), py_str("grans"), py_str("granny"), py_str("grannies"), py_str("grandma"), py_str("grandmas"), py_str("grandad"), py_str("grandads"), py_str("grandpa"), py_str("grandpas"), py_str("nana"), py_str("nanas"), py_str("nan"), py_str("nans"), py_str("grandparent"), py_str("grandparents")]));
    s__SIBLINGS(py_tuple(vec![py_str("brother"), py_str("brothers"), py_str("sister"), py_str("sisters"), py_str("sibling"), py_str("siblings"), py_str("half brother"), py_str("half sister"), py_str("stepbrother"), py_str("stepsister"), py_str("step brother"), py_str("step sister")]));
    s__CHILDREN(py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("baby"), py_str("babies"), py_str("grandchild"), py_str("grandchildren"), py_str("grandson"), py_str("granddaughter")]));
    s__PARTNER(py_tuple(vec![py_str("wife"), py_str("wives"), py_str("husband"), py_str("husbands"), py_str("partner"), py_str("partners"), py_str("girlfriend"), py_str("boyfriend"), py_str("fiance"), py_str("fiancee"), py_str("other half")]));
    s__EXTENDED(py_tuple(vec![py_str("aunt"), py_str("aunts"), py_str("auntie"), py_str("aunties"), py_str("uncle"), py_str("uncles"), py_str("cousin"), py_str("cousins"), py_str("nephew"), py_str("nephews"), py_str("niece"), py_str("nieces"), py_str("in law"), py_str("in laws"), py_str("mother in law"), py_str("father in law"), py_str("brother in law"), py_str("sister in law")]));
    s__FAMILY(py_tuple(vec![py_str("family"), py_str("families"), py_str("relatives"), py_str("relations"), py_str("folks"), py_str("kin")]));
    s_CLASSES(py_tuple(vec![py_tuple(vec![py_str("grandparent"), g__GRANDPARENTS()]), py_tuple(vec![py_str("parent"), g__PARENTS()]), py_tuple(vec![py_str("sibling"), g__SIBLINGS()]), py_tuple(vec![py_str("child"), g__CHILDREN()]), py_tuple(vec![py_str("partner"), g__PARTNER()]), py_tuple(vec![py_str("extended"), g__EXTENDED()]), py_tuple(vec![py_str("family"), g__FAMILY()])]));
    s_RELATION(py_dict(vec![]));
    let __iter1 = py_iter_or_raise(&g_CLASSES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    s__class(__unpk2[0].clone());
    s__words(__unpk2[1].clone());
    let __iter3 = py_iter_or_raise(&g__words());
    for __item in __iter3 {
    s__word(__item.clone());
    let _ = py_setitem(&g_RELATION(), &g__word(), g__class());
    }
    }
    s__ALL_RELATIONS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_RELATION()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_RELATION(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:your|ur|yours|you're|youre)\\s+(?:\\w+\\s+){0,2}?(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_RELATIONS()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAD_IN(py_str("(?:(?:and|so|but|oh|well|ok|okay|right|hey|hi|yeah|yes|no|nah|actually|honestly|just|quick|sorry|lol|haha|ha|eh|ah|hmm|huh|then|now)\\b[,!.]?\\s+)*"));
    s_A_HAVE_RELATION(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__LEAD_IN()), &py_str("(?:(?:have|has|do|does|did)\\s+(?:you|u|ya)\\s+(?:have\\s+|got\\s+|ever\\s+)*|got\\s+|any\\s+)(?:any\\s+|a\\s+|an\\s+|some\\s+)?(?:")), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_RELATIONS()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RELATION(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_RELATIONS()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_RELATION(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__LEAD_IN()), &py_str("how many\\s+(?:\\w+\\s+){0,2}?(?:")), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_RELATIONS()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b[\\w\\s,']*\\b(?:you|u|ya)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^"), &g__LEAD_IN()), &py_str("(?:(?:can|could|would|will)\\s+you\\s+)?(?:tell|told|say|talk|chat)\\s+(?:me\\s+)?(?:something\\s+|a\\s+bit\\s+|more\\s+|anything\\s+)?about\\s+|(?:is|are|was|were|do|does|did|have|has|had|will|would|can|could|should|shall|who|whos|whose|what|whats|what's|where|wheres|when|whens|why|whys|how|hows|which|any|anyone|ever|doesnt|didnt|isnt|arent|wasnt|werent)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AND_ANOTHER(stdlib_call("re", "compile", &[py_str("\\band\\b\\s+(?:your|ur)\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ABOUT_THE_BOT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|his|her|their)\\s+(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_RELATIONS()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_ONE(py_dict(vec![(py_str("parent"), py_tuple(vec![py_str("No mum and dad of my own - I'm a program, so there's no one who brought me up to be anywhere."), py_str("None on my side: I'm a program, so no parents and no childhood to have them in."), py_str("I haven't got parents - I'm a program, so there's no mum and dad at the end of a phone.")])), (py_str("grandparent"), py_tuple(vec![py_str("No grandparents of mine - I'm a program, so nothing like that on my side."), py_str("None, no: I'm a program, so no nan and grandad to go and see."), py_str("I haven't got grandparents - I'm a program, so there's no family of mine at all.")])), (py_str("sibling"), py_tuple(vec![py_str("No brothers and no sisters - I'm a program, so no family of my own."), py_str("None, no: I'm a program, so there's nobody to fall out with at Christmas."), py_str("I haven't got siblings - I'm a program, so nobody grew up alongside me.")])), (py_str("child"), py_tuple(vec![py_str("No children of my own - I'm a program, so nobody calls me anything but the bot."), py_str("None, no: I'm a program, so no kids and no school run."), py_str("I haven't got any - I'm a program, so there's no family of mine to bring up.")])), (py_str("partner"), py_tuple(vec![py_str("Nobody like that - I'm a program, so no partner and no one to ring on the way home."), py_str("None, no: I'm a program, so there's no other half on my side."), py_str("I haven't got one - I'm a program, so nobody's waiting up for me.")])), (py_str("extended"), py_tuple(vec![py_str("None of those on my side - I'm a program, so no aunts, uncles or cousins."), py_str("Nobody like that: I'm a program, so there's no family of mine to be an aunt or uncle to."), py_str("I haven't got any - I'm a program, so no relatives at all.")])), (py_str("family"), py_tuple(vec![py_str("No family of my own - I'm a program, so there is nobody on my side of it."), py_str("None, no: I'm a program, so no family, no birthday list, nobody to ask after."), py_str("I haven't got any - I'm a program, so there's no family of mine at all.")]))]));
    s__HOW_IT_WENT(py_tuple(vec![py_str("Tell me about yours - how did it go?"), py_str("How did it go with yours?"), py_str("What about your lot - how did it go?")]));
    s__THE_PEOPLE(py_tuple(vec![py_str("Tell me about yours though - do you see much of them?"), py_str("What about your lot?"), py_str("How are yours doing?")]));
    s_A_PAST_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|did|said|had|went|wore|told|thought|felt|been|there|at it|come)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NO_HAND_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:think|feel|felt|mind|care|miss|lonely|sorry|upset)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHOICE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bor\\b\\s+(?!(?:not|no|is|are|was|were|else|anything|something|otherwise)\\b|(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_RELATIONS()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b)\\w+")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_relation(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The relation the line asks about, echoed as the person wrote it, or None.\n\n    \"your mum and dad\" comes back as \"mum and dad\" because that is the pair they asked about.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    match_ = { let __b = mcall(&g_THE_RELATION(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HAVE_RELATION(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_MANY_RELATION(), "match", &[said.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&w.clone(), "lower", &[]), "strip", &[py_str(",.'")])); } py_list(__out) })() };
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_RELATION(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_AND_ANOTHER(), "search", &[slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    rest = mcall(&g_A_RELATION(), "search", &[slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&mcall(&rest.clone(), "group", &[Py::Int(0)]), "lower", &[]), &geti(&found.clone(), &Py::Int(0)))) } }) {
    return op_add(&op_add(&geti(&found.clone(), &Py::Int(0)), &py_str(" and ")), &mcall(&mcall(&rest.clone(), "group", &[Py::Int(0)]), "lower", &[]));
    }
    }
    return geti(&found.clone(), &Py::Int(0));
    Py::None
}
pub fn the_class(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Which class of relation the line named; \"family\" when it names none in particular.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    match_ = { let __b = mcall(&g_THE_RELATION(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HAVE_RELATION(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_MANY_RELATION(), "match", &[said.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&w.clone(), "lower", &[]), "strip", &[py_str(",.'")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_RELATION(), &word.clone()))) {
    return geti(&g_RELATION(), &word.clone());
    }
    }
    return Py::None;
    Py::None
}
pub fn is_a_family_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the bot about a relation of its own, and asks at all.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THE_RELATION(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_HAVE_RELATION(), "match", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HOW_MANY_RELATION(), "match", &[said.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NOT_ABOUT_THE_BOT(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_CHOICE(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_A_QUESTION(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HAVE_RELATION(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_MANY_RELATION(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "endswith", &[py_str("?")]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut honest: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_of: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut told: Py = Py::None;
    let _ = py_str("The honest answer to a question about the bot's own family, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&is_a_family_ask(&[text.clone()])))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    kind = { let __b = the_class(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("family") } };
    told = geti(&g_NO_ONE(), &kind.clone());
    said_of = if py_truthy(&mcall(&g_A_PAST_ASK(), "search", &[said.clone()])) { g__HOW_IT_WENT() } else { g__THE_PEOPLE() };
    seed = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    honest = py_call(&gen_compose::compose::fallback::g_pick(), &[told.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, seed.clone()]);
    if py_truthy(&mcall(&g__NO_HAND_BACK(), "search", &[said.clone()])) {
    return honest.clone();
    }
    return op_add(&op_add(&honest.clone(), &py_str(" ")), &py_call(&gen_compose::compose::fallback::g_pick(), &[said_of.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, seed.clone()]));
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("A question about the bot's own family is this module's line whole.\n\n    Nobody else can answer it: the knowledge base has nothing about a program's parents, and its\n    topic entries read the words of the line (\"strangers\", \"curry\") and answer a different\n    question. The reading is the same one `respond` makes, so a line this module would not answer\n    is not claimed either.\n    ");
    return is_a_family_ask(&[text.clone()]);
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let _ = py_str("The line a question about the bot's family gets; None for every other line.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    memory = py_dict(vec![]);
    }
    return reply_for(&[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.familyask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AND_ANOTHER" => Some(g_AND_ANOTHER()),
            "ANSWERS" => Some(g_ANSWERS()),
            "A_CHOICE" => Some(g_A_CHOICE()),
            "A_HAVE_RELATION" => Some(g_A_HAVE_RELATION()),
            "A_PAST_ASK" => Some(g_A_PAST_ASK()),
            "A_QUESTION" => Some(g_A_QUESTION()),
            "A_RELATION" => Some(g_A_RELATION()),
            "CLASSES" => Some(g_CLASSES()),
            "HOW_MANY_RELATION" => Some(g_HOW_MANY_RELATION()),
            "NOT_ABOUT_THE_BOT" => Some(g_NOT_ABOUT_THE_BOT()),
            "NO_ONE" => Some(g_NO_ONE()),
            "PRIORITY" => Some(g_PRIORITY()),
            "RELATION" => Some(g_RELATION()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "THE_RELATION" => Some(g_THE_RELATION()),
            "_ALL_RELATIONS" => Some(g__ALL_RELATIONS()),
            "_CHILDREN" => Some(g__CHILDREN()),
            "_EXTENDED" => Some(g__EXTENDED()),
            "_FAMILY" => Some(g__FAMILY()),
            "_GRANDPARENTS" => Some(g__GRANDPARENTS()),
            "_HOW_IT_WENT" => Some(g__HOW_IT_WENT()),
            "_LEAD_IN" => Some(g__LEAD_IN()),
            "_NO_HAND_BACK" => Some(g__NO_HAND_BACK()),
            "_PARENTS" => Some(g__PARENTS()),
            "_PARTNER" => Some(g__PARTNER()),
            "_SIBLINGS" => Some(g__SIBLINGS()),
            "_THE_PEOPLE" => Some(g__THE_PEOPLE()),
            "_class" => Some(g__class()),
            "_word" => Some(g__word()),
            "_words" => Some(g__words()),
            "is_a_family_ask" => Some(py_func("is_a_family_ask", |args: &[Py]| is_a_family_ask(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "the_class" => Some(py_func("the_class", |args: &[Py]| the_class(args))),
            "the_relation" => Some(py_func("the_relation", |args: &[Py]| the_relation(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AND_ANOTHER" => s_AND_ANOTHER(v),
            "ANSWERS" => s_ANSWERS(v),
            "A_CHOICE" => s_A_CHOICE(v),
            "A_HAVE_RELATION" => s_A_HAVE_RELATION(v),
            "A_PAST_ASK" => s_A_PAST_ASK(v),
            "A_QUESTION" => s_A_QUESTION(v),
            "A_RELATION" => s_A_RELATION(v),
            "CLASSES" => s_CLASSES(v),
            "HOW_MANY_RELATION" => s_HOW_MANY_RELATION(v),
            "NOT_ABOUT_THE_BOT" => s_NOT_ABOUT_THE_BOT(v),
            "NO_ONE" => s_NO_ONE(v),
            "PRIORITY" => s_PRIORITY(v),
            "RELATION" => s_RELATION(v),
            "THE_RELATION" => s_THE_RELATION(v),
            "_ALL_RELATIONS" => s__ALL_RELATIONS(v),
            "_CHILDREN" => s__CHILDREN(v),
            "_EXTENDED" => s__EXTENDED(v),
            "_FAMILY" => s__FAMILY(v),
            "_GRANDPARENTS" => s__GRANDPARENTS(v),
            "_HOW_IT_WENT" => s__HOW_IT_WENT(v),
            "_LEAD_IN" => s__LEAD_IN(v),
            "_NO_HAND_BACK" => s__NO_HAND_BACK(v),
            "_PARENTS" => s__PARENTS(v),
            "_PARTNER" => s__PARTNER(v),
            "_SIBLINGS" => s__SIBLINGS(v),
            "_THE_PEOPLE" => s__THE_PEOPLE(v),
            "_class" => s__class(v),
            "_word" => s__word(v),
            "_words" => s__words(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.familyask", module_obj);
    pyrt::xrefs::reg_fn("modules.familyask", "the_relation", py_func("the_relation", |args: &[Py]| the_relation(args)));
    pyrt::xrefs::reg_fn("modules.familyask", "the_class", py_func("the_class", |args: &[Py]| the_class(args)));
    pyrt::xrefs::reg_fn("modules.familyask", "is_a_family_ask", py_func("is_a_family_ask", |args: &[Py]| is_a_family_ask(args)));
    pyrt::xrefs::reg_fn("modules.familyask", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("modules.familyask", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.familyask", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.familyask", "AND_ANOTHER", g_AND_ANOTHER, s_AND_ANOTHER);
    pyrt::xrefs::reg_access("modules.familyask", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.familyask", "A_CHOICE", g_A_CHOICE, s_A_CHOICE);
    pyrt::xrefs::reg_access("modules.familyask", "A_HAVE_RELATION", g_A_HAVE_RELATION, s_A_HAVE_RELATION);
    pyrt::xrefs::reg_access("modules.familyask", "A_PAST_ASK", g_A_PAST_ASK, s_A_PAST_ASK);
    pyrt::xrefs::reg_access("modules.familyask", "A_QUESTION", g_A_QUESTION, s_A_QUESTION);
    pyrt::xrefs::reg_access("modules.familyask", "A_RELATION", g_A_RELATION, s_A_RELATION);
    pyrt::xrefs::reg_access("modules.familyask", "CLASSES", g_CLASSES, s_CLASSES);
    pyrt::xrefs::reg_access("modules.familyask", "HOW_MANY_RELATION", g_HOW_MANY_RELATION, s_HOW_MANY_RELATION);
    pyrt::xrefs::reg_access("modules.familyask", "NOT_ABOUT_THE_BOT", g_NOT_ABOUT_THE_BOT, s_NOT_ABOUT_THE_BOT);
    pyrt::xrefs::reg_access("modules.familyask", "NO_ONE", g_NO_ONE, s_NO_ONE);
    pyrt::xrefs::reg_access("modules.familyask", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.familyask", "RELATION", g_RELATION, s_RELATION);
    pyrt::xrefs::reg_access("modules.familyask", "THE_RELATION", g_THE_RELATION, s_THE_RELATION);
    pyrt::xrefs::reg_access("modules.familyask", "_ALL_RELATIONS", g__ALL_RELATIONS, s__ALL_RELATIONS);
    pyrt::xrefs::reg_access("modules.familyask", "_CHILDREN", g__CHILDREN, s__CHILDREN);
    pyrt::xrefs::reg_access("modules.familyask", "_EXTENDED", g__EXTENDED, s__EXTENDED);
    pyrt::xrefs::reg_access("modules.familyask", "_FAMILY", g__FAMILY, s__FAMILY);
    pyrt::xrefs::reg_access("modules.familyask", "_GRANDPARENTS", g__GRANDPARENTS, s__GRANDPARENTS);
    pyrt::xrefs::reg_access("modules.familyask", "_HOW_IT_WENT", g__HOW_IT_WENT, s__HOW_IT_WENT);
    pyrt::xrefs::reg_access("modules.familyask", "_LEAD_IN", g__LEAD_IN, s__LEAD_IN);
    pyrt::xrefs::reg_access("modules.familyask", "_NO_HAND_BACK", g__NO_HAND_BACK, s__NO_HAND_BACK);
    pyrt::xrefs::reg_access("modules.familyask", "_PARENTS", g__PARENTS, s__PARENTS);
    pyrt::xrefs::reg_access("modules.familyask", "_PARTNER", g__PARTNER, s__PARTNER);
    pyrt::xrefs::reg_access("modules.familyask", "_SIBLINGS", g__SIBLINGS, s__SIBLINGS);
    pyrt::xrefs::reg_access("modules.familyask", "_THE_PEOPLE", g__THE_PEOPLE, s__THE_PEOPLE);
    pyrt::xrefs::reg_access("modules.familyask", "_class", g__class, s__class);
    pyrt::xrefs::reg_access("modules.familyask", "_word", g__word, s__word);
    pyrt::xrefs::reg_access("modules.familyask", "_words", g__words, s__words);
}
