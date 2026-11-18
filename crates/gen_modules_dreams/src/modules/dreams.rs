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


thread_local! { static G_AN_EX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ATTACKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ATTACK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BREAK_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIRE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOOD_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREAK_IN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHASED_OR_FALLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHASE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEATH_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIED_MYSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EX_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_IN_THE_DREAM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_DEATH_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_HURT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_DIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEETH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEETH_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_EX() -> Py { ensure_init(); G_AN_EX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EX(v: Py) { ensure_init(); G_AN_EX.with(|c| *c.borrow_mut() = v); }
pub fn g_ATTACKED() -> Py { ensure_init(); G_ATTACKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATTACKED(v: Py) { ensure_init(); G_ATTACKED.with(|c| *c.borrow_mut() = v); }
pub fn g_ATTACK_LINES() -> Py { ensure_init(); G_ATTACK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATTACK_LINES(v: Py) { ensure_init(); G_ATTACK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BABY() -> Py { ensure_init(); G_A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BABY(v: Py) { ensure_init(); G_A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BREAK_IN() -> Py { ensure_init(); G_A_BREAK_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BREAK_IN(v: Py) { ensure_init(); G_A_BREAK_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIRE() -> Py { ensure_init(); G_A_FIRE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIRE(v: Py) { ensure_init(); G_A_FIRE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOOD_ONE() -> Py { ensure_init(); G_A_GOOD_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_ONE(v: Py) { ensure_init(); G_A_GOOD_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VISIT() -> Py { ensure_init(); G_A_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VISIT(v: Py) { ensure_init(); G_A_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_LINES() -> Py { ensure_init(); G_BABY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_LINES(v: Py) { ensure_init(); G_BABY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_BREAK_IN_LINES() -> Py { ensure_init(); G_BREAK_IN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREAK_IN_LINES(v: Py) { ensure_init(); G_BREAK_IN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHASED_OR_FALLING() -> Py { ensure_init(); G_CHASED_OR_FALLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHASED_OR_FALLING(v: Py) { ensure_init(); G_CHASED_OR_FALLING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHASE_LINES() -> Py { ensure_init(); G_CHASE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHASE_LINES(v: Py) { ensure_init(); G_CHASE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_DEATH_LINES() -> Py { ensure_init(); G_DEATH_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEATH_LINES(v: Py) { ensure_init(); G_DEATH_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_DIED_MYSELF() -> Py { ensure_init(); G_DIED_MYSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIED_MYSELF(v: Py) { ensure_init(); G_DIED_MYSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_EX_LINES() -> Py { ensure_init(); G_EX_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EX_LINES(v: Py) { ensure_init(); G_EX_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRE_LINES() -> Py { ensure_init(); G_FIRE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRE_LINES(v: Py) { ensure_init(); G_FIRE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_LINES() -> Py { ensure_init(); G_GOOD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_LINES(v: Py) { ensure_init(); G_GOOD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_IN_THE_DREAM() -> Py { ensure_init(); G_ILL_IN_THE_DREAM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_IN_THE_DREAM(v: Py) { ensure_init(); G_ILL_IN_THE_DREAM.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_LINES() -> Py { ensure_init(); G_ILL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_LINES(v: Py) { ensure_init(); G_ILL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_DEATH_LINES() -> Py { ensure_init(); G_OWN_DEATH_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_DEATH_LINES(v: Py) { ensure_init(); G_OWN_DEATH_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_HURT() -> Py { ensure_init(); G_PEOPLE_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_HURT(v: Py) { ensure_init(); G_PEOPLE_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_HURT_LINES() -> Py { ensure_init(); G_PEOPLE_HURT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_HURT_LINES(v: Py) { ensure_init(); G_PEOPLE_HURT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_LINES() -> Py { ensure_init(); G_PLAIN_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_LINES(v: Py) { ensure_init(); G_PLAIN_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED_LINES() -> Py { ensure_init(); G_SETTLED_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED_LINES(v: Py) { ensure_init(); G_SETTLED_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_DIED() -> Py { ensure_init(); G_SOMEONE_DIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_DIED(v: Py) { ensure_init(); G_SOMEONE_DIED.with(|c| *c.borrow_mut() = v); }
pub fn g_TEETH() -> Py { ensure_init(); G_TEETH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEETH(v: Py) { ensure_init(); G_TEETH.with(|c| *c.borrow_mut() = v); }
pub fn g_TEETH_LINES() -> Py { ensure_init(); G_TEETH_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEETH_LINES(v: Py) { ensure_init(); G_TEETH_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_LINES() -> Py { ensure_init(); G_VISIT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_LINES(v: Py) { ensure_init(); G_VISIT_LINES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A dream is a scene, and the line about it is answered as one.\n\n    i dreamt my dad died           ->  not a condolence: a dream about someone dying is common\n    i had a dream my house was on\n      fire                         ->  not \"ugh, that's a lot to carry\"\n    i dreamed i was drowning       ->  not \"then something has to come off the list\"\n    i keep dreaming my teeth fall\n      out                          ->  not the answer for a six year old's milk teeth\n\nEvery one of those was answered on main by a reader that took the scene for an event: the loss\ncomposer offered sympathy for a death that had not happened, the teeth entry explained a child's\nmilk teeth, and the overwhelm reply told a dreamer to shorten their to-do list. The words are the\nsame either way, so the only thing that separates them is the frame: `dreamt`, `dreamed`, `i had\na dream`, `i keep dreaming`.\n\nThe frame is what this module reads, and it reads it the way the sentence is built - the dream\nverb has to come *before* the scene it is about. \"my dad died and i dreamt about him last night\"\nis a real death with a dream after it and belongs to `bot/compose/loss.py`; \"i dreamt my dad\ndied\" is the dream and belongs here. A line with no first person in the frame (\"why do we\ndream\", \"what do dreams mean\") is not this module's question either: those are the science and\nsleep entries, and they answer them well.\n\nAbove the knowledge base (0) and above `loss` (2000) and `grief` (2100), because those two read\nthe scene as an event and offer sympathy for it, and below the memory store (7000) and the safety\nband (9000), whose turns these are not.\n");
    s_PRIORITY(Py::Int(3000));
    s_SOMEONE_DIED(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|dead|death|passed (?:away|on)|killed|funeral|buried|gone|run over|put (?:him|her|it|them) (?:down|to sleep)|euthanis\\w*|euthaniz\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIRE(stdlib_call("re", "compile", &[py_str("\\b(?:fire|burning|burnt|burned|flames|smoke|flood|flooded|collapsed|crashing down)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BREAK_IN(stdlib_call("re", "compile", &[py_str("\\b(?:robbed|robbery|burglar|burglars|burgled|intruder|break(?:ing)? in|broke in|broken into|house was broken)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHASED_OR_FALLING(stdlib_call("re", "compile", &[py_str("\\b(?:drown\\w*|falling|fell|chased|chasing|running away|trapped|stuck|suffocat\\w*|strangled|being held|can(?:no|')?t move|paralys\\w*|crash\\w*|accident|hit by a (?:car|bus|truck|train)|run over)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ILL_IN_THE_DREAM(stdlib_call("re", "compile", &[py_str("\\b(?:cancer|tumou?r|stroke|heart attack|dementia|alzheimers|aids|hiv|diagnos\\w*|ill|dying of|got (?:sick|ill)|terminal)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOOD_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:fly|flies|flying|flew|float\\w*|won|lotto|lottery|holiday|beach|love\\w*|kiss\\w*|my (?:nan|gran|grandma|grandad|mum|dad|dog|cat) (?:was|gave|said|came)|laughing|happy|best day|pregnant)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DIED_MYSELF(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:was\\s+)?(?:died|dead|dying|killed|drowned|buried|hanging|hung|overdos\\w*|killed myself|hurt myself|took my own life)\\b|\\bmy own (?:death|funeral)\\b|\\b(?:suicide|overdose)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ATTACKED(stdlib_call("re", "compile", &[py_str("\\b(?:attacking|attacked|attacks me|hurting me|hurt me|beating me|beat me|strangling me|stabbed|stabbing|shot|shooting|raped|raping|grabbed me|somebody in (?:my|the) (?:house|room)|someone in (?:my|the) (?:house|room)|broke into|breaking in|burglar|intruder|following me|chasing me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TEETH(stdlib_call("re", "compile", &[py_str("\\b(?:teeth|tooth|molar|gums)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_EX(stdlib_call("re", "compile", &[py_str("\\b(?:ex|ex-boyfriend|ex-girlfriend|ex-husband|ex-wife|my ex)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BABY(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|newborn|toddler|pregnant|infant)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE_HURT(stdlib_call("re", "compile", &[py_str("\\b(?:my (?:son|daughter|boy|girl|child|kid|children|mum|mom|mother|dad|father|partner|husband|wife|brother|sister|nan|gran|grandma|grandad|friend|niece|nephew)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_LINES(py_tuple(vec![py_str("Dreaming that someone died is one of the commonest dreams there is, and it is not a sign about them: it usually turns up when they have been on your mind, or when something between you is shifting. They are alive and ordinary this morning, and that is the part to trust."), py_str("That dream is not an omen, it is your head working on the person: most people who dream of a death say the person had been on their mind, or that something with them was changing. Nothing in the dream is a warning about them.")]));
    s_BREAK_IN_LINES(py_tuple(vec![py_str("A house broken into in a dream is the worry about being safe at home, not a warning that somebody is coming: it turns up in weeks when the flat, the money or the door has been on your mind. Check the lock in daylight and leave it there."), py_str("Dreaming about a break-in is what the head does with feeling got at, and it is one of the commonest dreams there is. Nothing in it is going to happen - the door is the same door you locked last night."), py_str("That dream is about safety rather than about burglars: the nights after a scare often bring it. What usually helps is the dull, practical thing - a light on a timer, a bolt on the door - rather than going over the dream again.")]));
    s_FIRE_LINES(py_tuple(vec![py_str("A fire in a dream usually reads as worry about the place rather than anything about fire - a bill, a repair, something you cannot quite keep an eye on. It tends to fade on its own; if the same one keeps coming back, write it down in the morning."), py_str("Dreaming the house is on fire is a stress dream, and the thing behind it is usually a practical worry rather than any danger - money, a job, a repair waiting on somebody. Nothing about the house needs doing because of it.")]));
    s_CHASE_LINES(py_tuple(vec![py_str("Being chased, drowning or falling is the standard stress dream, and it usually follows a stretch where something is out of your hands. A cooler room, less alcohol and half an hour off the screen before bed take most of the edge off it."), py_str("That is the shape a busy week takes in sleep - out of control, no footing, running late. The dream follows the week rather than the other way round, so it eases when the week does.")]));
    s_TEETH_LINES(py_tuple(vec![py_str("Dreaming your teeth fall out is one of the most reported dreams there is, and it is not about your teeth: it tends to go with a spell of worry about how you come across, or a change you cannot control. It stops when that does."), py_str("That dream is about being seen, not about your teeth - it is one of the commonest there is and it clusters in the weeks when somebody is judging you or a change is coming. No dentist needed.")]));
    s_EX_LINES(py_tuple(vec![py_str("Dreaming about an ex does not mean you want them back: your head digs up whatever is left unfinished, and it often surfaces when something in your current life is unsettled. It says more about this week than about them."), py_str("An ex in a dream is usually unfinished business rather than a wish - the brain reaches for an old face when something now feels unresolved. Nothing to act on.")]));
    s_BABY_LINES(py_tuple(vec![py_str("A dream about the baby being hurt is horrible to wake from and very common on broken sleep, and nothing in it is a warning. The waking every couple of hours is what makes those dreams so vivid and so hard to shake."), py_str("That one comes with the tiredness rather than with any danger: fragmented nights make frightening dreams much more likely. It gets better as the nights get longer.")]));
    s_GOOD_LINES(py_tuple(vec![py_str("Nothing in a dream is a warning, and that one is the good kind: flying and the people you love turn up in sleep when things are going alright. Enjoy it and let it go."), py_str("That is the sort the brain keeps - flying, a win, somebody you love turning up - and it usually follows a stretch that felt light. Nothing to do with it but have it.")]));
    s_OWN_DEATH_LINES(py_tuple(vec![py_str("Dying, or being hurt, in a dream is frightening and very common, and it is not a sign about your health: it tends to come when something in life is ending - a job, a habit, a way of doing things - rather than anything about the body."), py_str("Dreaming your own death reads as a change rather than a warning, and it clusters when something in the day to day is finishing. Anything about the body belongs to a doctor, not to a dream.")]));
    s_ATTACK_LINES(py_tuple(vec![py_str("Being attacked in a dream is not a sign of anything coming: it is what a day that felt unsafe or watched turns into at night. Uneasy sleep makes it more likely, so a door checked and a proper wind-down before bed does more for it than reading anything into it."), py_str("A dream like that follows a stretch of feeling got at rather than any real threat - a difficult person, a job, a row left hanging. Nothing about it is a warning, and it eases when the day does.")]));
    s_A_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|is|are)\\s+(?:in\\s+(?:my|the)\\s+room|here|with\\s+me|still\\s+alive|standing\\s+(?:there|in))|came\\s+to\\s+see\\s+me|visited\\s+me|sat\\s+with\\s+me|spoke\\s+to\\s+me|said\\s+hello|hugged\\s+me|holding\\s+my\\s+hand\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_VISIT_LINES(py_tuple(vec![py_str("Dreaming that somebody you love is with you is one of the kinder ones, and it usually comes when they have been on your mind. There is nothing to read into it, and nothing wrong with enjoying it."), py_str("That sort of dream is the good kind: somebody you love, there and ordinary, while you sleep. It turns up when they are on your mind, and it means nothing more than that.")]));
    s_PEOPLE_HURT_LINES(py_tuple(vec![py_str("Dreaming that somebody you love is hurt is the worry dream almost everybody has, and it is not a premonition: it turns up in the weeks when you are carrying them in your head. They are where they were when you went to bed."), py_str("That dream is the shape worry takes at night rather than anything coming to them. It is worth noting what was on your mind about them that day, because that is what the dream was made of.")]));
    s_ILL_LINES(py_tuple(vec![py_str("A dream about being ill frightens people because it names a disease, and it is not a sign about your body: dreams like that follow a scare, a wait for results, or somebody close being unwell. Anything you have actually noticed in the day to day is a doctor's question, not a dream's."), py_str("Nothing in a dream diagnoses anything. That one usually comes after a health scare in the family or a wait for a test, and it fades once the worry does.")]));
    s_PLAIN_LINES(py_tuple(vec![py_str("Dreams like that are usually the brain sorting the day out, and they come on strongest in a week with something on your mind. Writing it down in the morning is the usual trick for one that keeps coming back."), py_str("Nothing in a dream is a warning - it is the day being filed, and a vivid one usually means a lot was going on. A pen by the bed and a few lines in the morning take the sting out of the ones that keep coming back.")]));
    s_SETTLED_LINES(py_tuple(vec![py_str("Then nobody in it is any different for it - a dream is the head clearing, not a message about them. If the same one keeps coming back, that is worth saying to a GP, because broken sleep is usually the whole of it."), py_str("Good. Dreams like that are the worry being worked through at night, and they ease off when the worry does. Sleeping badly makes them louder, so a steadier night is the thing that quiets them."), py_str("That is the whole of it, then: a dream, and everyone in it where they were. Worth writing down if it comes back - the mornings after are what show whether it is a worry to see someone about or just a run of bad nights.")]));
}
pub fn _settled(args: &[Py]) -> Py {
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
    let mut choices: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "choices") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: choices"))) };
    let mut c: Py = Py::None;
    let _ = py_str("One of the lines, picked by the words so the same dream gets the same answer.");
    return geti(&choices.clone(), &op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&line.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[choices.clone()]).unwrap_or_else(|e| exc::raise(e))));
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
    let mut line: Py = Py::None;
    let mut scene: Py = Py::None;
    let _ = py_str("An answer for a dream somebody is telling, or None for every other line.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    scene = pyrt::xrefs::xcall("memory.dream", "the_scene", &[line.clone()]);
    if py_truthy(&{ let __b = scene.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.dream", "a_dream_is_not_happening", &[line.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&scene.clone()))) {
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("memory.dream", "SETTLED_IT"), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.dream", "told_in_the_turn_before", &[memory.clone()]) } }) {
    return _settled(&[line.clone(), g_SETTLED_LINES()]);
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_SOMEONE_DIED(), "search", &[slice_or_raise(&line.clone(), &Py::None, &mcall(&line.clone(), "find", &[scene.clone()]), &Py::None)]); if py_truthy(&__b) { __b } else { mcall(&g_A_FIRE(), "search", &[slice_or_raise(&line.clone(), &Py::None, &mcall(&line.clone(), "find", &[scene.clone()]), &Py::None)]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_DIED_MYSELF(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_OWN_DEATH_LINES()]);
    }
    if py_truthy(&mcall(&g_ATTACKED(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_ATTACK_LINES()]);
    }
    if py_truthy(&{ let __b = mcall(&g_ILL_IN_THE_DREAM(), "search", &[scene.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PEOPLE_HURT(), "search", &[scene.clone()]))) } }) {
    return _settled(&[line.clone(), g_ILL_LINES()]);
    }
    if py_truthy(&mcall(&g_SOMEONE_DIED(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_DEATH_LINES()]);
    }
    if py_truthy(&mcall(&g_TEETH(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_TEETH_LINES()]);
    }
    if py_truthy(&mcall(&g_A_FIRE(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_FIRE_LINES()]);
    }
    if py_truthy(&mcall(&g_A_BREAK_IN(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_BREAK_IN_LINES()]);
    }
    if py_truthy(&mcall(&g_A_BABY(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_BABY_LINES()]);
    }
    if py_truthy(&mcall(&g_AN_EX(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_EX_LINES()]);
    }
    if py_truthy(&mcall(&g_A_VISIT(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_VISIT_LINES()]);
    }
    if py_truthy(&mcall(&g_PEOPLE_HURT(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_PEOPLE_HURT_LINES()]);
    }
    if py_truthy(&mcall(&g_CHASED_OR_FALLING(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_CHASE_LINES()]);
    }
    if py_truthy(&mcall(&g_A_GOOD_ONE(), "search", &[scene.clone()])) {
    return _settled(&[line.clone(), g_GOOD_LINES()]);
    }
    return _settled(&[line.clone(), g_PLAIN_LINES()]);
    Py::None
}
pub fn CLAIMS_INSTEAD(args: &[Py]) -> Py {
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
    let _ = py_str("True on the dream lines another reader claims as it comes, where this module has the answer.\n\n    The death, the fire and the lost tooth are claims: a line with the word \"died\" in it is met\n    with a condolence before anything else is read, which is right for a death and wrong for\n    \"i dreamt my dad died\". Declaring the claim here is what lets the dream answer take that turn.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[respond(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.dreams".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_EX" => Some(g_AN_EX()),
            "ATTACKED" => Some(g_ATTACKED()),
            "ATTACK_LINES" => Some(g_ATTACK_LINES()),
            "A_BABY" => Some(g_A_BABY()),
            "A_BREAK_IN" => Some(g_A_BREAK_IN()),
            "A_FIRE" => Some(g_A_FIRE()),
            "A_GOOD_ONE" => Some(g_A_GOOD_ONE()),
            "A_VISIT" => Some(g_A_VISIT()),
            "BABY_LINES" => Some(g_BABY_LINES()),
            "BREAK_IN_LINES" => Some(g_BREAK_IN_LINES()),
            "CHASED_OR_FALLING" => Some(g_CHASED_OR_FALLING()),
            "CHASE_LINES" => Some(g_CHASE_LINES()),
            "CLAIMS_INSTEAD" => Some(py_func("CLAIMS_INSTEAD", |args: &[Py]| CLAIMS_INSTEAD(args))),
            "DEATH_LINES" => Some(g_DEATH_LINES()),
            "DIED_MYSELF" => Some(g_DIED_MYSELF()),
            "EX_LINES" => Some(g_EX_LINES()),
            "FIRE_LINES" => Some(g_FIRE_LINES()),
            "GOOD_LINES" => Some(g_GOOD_LINES()),
            "ILL_IN_THE_DREAM" => Some(g_ILL_IN_THE_DREAM()),
            "ILL_LINES" => Some(g_ILL_LINES()),
            "OWN_DEATH_LINES" => Some(g_OWN_DEATH_LINES()),
            "PEOPLE_HURT" => Some(g_PEOPLE_HURT()),
            "PEOPLE_HURT_LINES" => Some(g_PEOPLE_HURT_LINES()),
            "PLAIN_LINES" => Some(g_PLAIN_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "SETTLED_LINES" => Some(g_SETTLED_LINES()),
            "SOMEONE_DIED" => Some(g_SOMEONE_DIED()),
            "TEETH" => Some(g_TEETH()),
            "TEETH_LINES" => Some(g_TEETH_LINES()),
            "VISIT_LINES" => Some(g_VISIT_LINES()),
            "_settled" => Some(py_func("_settled", |args: &[Py]| _settled(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_EX" => s_AN_EX(v),
            "ATTACKED" => s_ATTACKED(v),
            "ATTACK_LINES" => s_ATTACK_LINES(v),
            "A_BABY" => s_A_BABY(v),
            "A_BREAK_IN" => s_A_BREAK_IN(v),
            "A_FIRE" => s_A_FIRE(v),
            "A_GOOD_ONE" => s_A_GOOD_ONE(v),
            "A_VISIT" => s_A_VISIT(v),
            "BABY_LINES" => s_BABY_LINES(v),
            "BREAK_IN_LINES" => s_BREAK_IN_LINES(v),
            "CHASED_OR_FALLING" => s_CHASED_OR_FALLING(v),
            "CHASE_LINES" => s_CHASE_LINES(v),
            "DEATH_LINES" => s_DEATH_LINES(v),
            "DIED_MYSELF" => s_DIED_MYSELF(v),
            "EX_LINES" => s_EX_LINES(v),
            "FIRE_LINES" => s_FIRE_LINES(v),
            "GOOD_LINES" => s_GOOD_LINES(v),
            "ILL_IN_THE_DREAM" => s_ILL_IN_THE_DREAM(v),
            "ILL_LINES" => s_ILL_LINES(v),
            "OWN_DEATH_LINES" => s_OWN_DEATH_LINES(v),
            "PEOPLE_HURT" => s_PEOPLE_HURT(v),
            "PEOPLE_HURT_LINES" => s_PEOPLE_HURT_LINES(v),
            "PLAIN_LINES" => s_PLAIN_LINES(v),
            "PRIORITY" => s_PRIORITY(v),
            "SETTLED_LINES" => s_SETTLED_LINES(v),
            "SOMEONE_DIED" => s_SOMEONE_DIED(v),
            "TEETH" => s_TEETH(v),
            "TEETH_LINES" => s_TEETH_LINES(v),
            "VISIT_LINES" => s_VISIT_LINES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.dreams", module_obj);
    pyrt::xrefs::reg_fn("modules.dreams", "_settled", py_func("_settled", |args: &[Py]| _settled(args)));
    pyrt::xrefs::reg_fn("modules.dreams", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.dreams", "CLAIMS_INSTEAD", py_func("CLAIMS_INSTEAD", |args: &[Py]| CLAIMS_INSTEAD(args)));
    pyrt::xrefs::reg_access("modules.dreams", "AN_EX", g_AN_EX, s_AN_EX);
    pyrt::xrefs::reg_access("modules.dreams", "ATTACKED", g_ATTACKED, s_ATTACKED);
    pyrt::xrefs::reg_access("modules.dreams", "ATTACK_LINES", g_ATTACK_LINES, s_ATTACK_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "A_BABY", g_A_BABY, s_A_BABY);
    pyrt::xrefs::reg_access("modules.dreams", "A_BREAK_IN", g_A_BREAK_IN, s_A_BREAK_IN);
    pyrt::xrefs::reg_access("modules.dreams", "A_FIRE", g_A_FIRE, s_A_FIRE);
    pyrt::xrefs::reg_access("modules.dreams", "A_GOOD_ONE", g_A_GOOD_ONE, s_A_GOOD_ONE);
    pyrt::xrefs::reg_access("modules.dreams", "A_VISIT", g_A_VISIT, s_A_VISIT);
    pyrt::xrefs::reg_access("modules.dreams", "BABY_LINES", g_BABY_LINES, s_BABY_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "BREAK_IN_LINES", g_BREAK_IN_LINES, s_BREAK_IN_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "CHASED_OR_FALLING", g_CHASED_OR_FALLING, s_CHASED_OR_FALLING);
    pyrt::xrefs::reg_access("modules.dreams", "CHASE_LINES", g_CHASE_LINES, s_CHASE_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "DEATH_LINES", g_DEATH_LINES, s_DEATH_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "DIED_MYSELF", g_DIED_MYSELF, s_DIED_MYSELF);
    pyrt::xrefs::reg_access("modules.dreams", "EX_LINES", g_EX_LINES, s_EX_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "FIRE_LINES", g_FIRE_LINES, s_FIRE_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "GOOD_LINES", g_GOOD_LINES, s_GOOD_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "ILL_IN_THE_DREAM", g_ILL_IN_THE_DREAM, s_ILL_IN_THE_DREAM);
    pyrt::xrefs::reg_access("modules.dreams", "ILL_LINES", g_ILL_LINES, s_ILL_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "OWN_DEATH_LINES", g_OWN_DEATH_LINES, s_OWN_DEATH_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "PEOPLE_HURT", g_PEOPLE_HURT, s_PEOPLE_HURT);
    pyrt::xrefs::reg_access("modules.dreams", "PEOPLE_HURT_LINES", g_PEOPLE_HURT_LINES, s_PEOPLE_HURT_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "PLAIN_LINES", g_PLAIN_LINES, s_PLAIN_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.dreams", "SETTLED_LINES", g_SETTLED_LINES, s_SETTLED_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "SOMEONE_DIED", g_SOMEONE_DIED, s_SOMEONE_DIED);
    pyrt::xrefs::reg_access("modules.dreams", "TEETH", g_TEETH, s_TEETH);
    pyrt::xrefs::reg_access("modules.dreams", "TEETH_LINES", g_TEETH_LINES, s_TEETH_LINES);
    pyrt::xrefs::reg_access("modules.dreams", "VISIT_LINES", g_VISIT_LINES, s_VISIT_LINES);
}
