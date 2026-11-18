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


thread_local! { static G_AFFORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_NOT_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POSSESSIVE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHAT_IF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CATEGORIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_A_BUDGET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVINGS_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THERE_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COUNTED_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CURRENCY_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONEY_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SCOPE_OF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_SCOPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CURRENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HELD_SCALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HELD_SCOPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAY_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAIN_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_AS_A_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAIL_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_FIGURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_SPENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AFFORD() -> Py { ensure_init(); G_AFFORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFFORD(v: Py) { ensure_init(); G_AFFORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_NOT_MONEY() -> Py { ensure_init(); G_A_LENGTH_NOT_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_NOT_MONEY(v: Py) { ensure_init(); G_A_LENGTH_NOT_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POSSESSIVE_ASK() -> Py { ensure_init(); G_A_POSSESSIVE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POSSESSIVE_ASK(v: Py) { ensure_init(); G_A_POSSESSIVE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRICE() -> Py { ensure_init(); G_A_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRICE(v: Py) { ensure_init(); G_A_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHAT_IF() -> Py { ensure_init(); G_A_WHAT_IF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_IF(v: Py) { ensure_init(); G_A_WHAT_IF.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ON() -> Py { ensure_init(); G_BARE_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ON(v: Py) { ensure_init(); G_BARE_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_HELD() -> Py { ensure_init(); G_BUDGET_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_HELD(v: Py) { ensure_init(); G_BUDGET_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_CATEGORIES() -> Py { ensure_init(); G_CATEGORIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CATEGORIES(v: Py) { ensure_init(); G_CATEGORIES.with(|c| *c.borrow_mut() = v); }
pub fn g_COST() -> Py { ensure_init(); G_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COST(v: Py) { ensure_init(); G_COST.with(|c| *c.borrow_mut() = v); }
pub fn g_HELD() -> Py { ensure_init(); G_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELD(v: Py) { ensure_init(); G_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT() -> Py { ensure_init(); G_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT(v: Py) { ensure_init(); G_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_A_BUDGET() -> Py { ensure_init(); G_ON_A_BUDGET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_A_BUDGET(v: Py) { ensure_init(); G_ON_A_BUDGET.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_A_THING() -> Py { ensure_init(); G_ON_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_A_THING(v: Py) { ensure_init(); G_ON_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_ASK() -> Py { ensure_init(); G_PRESENT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_ASK(v: Py) { ensure_init(); G_PRESENT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVINGS_HELD() -> Py { ensure_init(); G_SAVINGS_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVINGS_HELD(v: Py) { ensure_init(); G_SAVINGS_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_SPENT() -> Py { ensure_init(); G_SPENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPENT(v: Py) { ensure_init(); G_SPENT.with(|c| *c.borrow_mut() = v); }
pub fn g_THERE_LEFT() -> Py { ensure_init(); G_THERE_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THERE_LEFT(v: Py) { ensure_init(); G_THERE_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_FILLER() -> Py { ensure_init(); G__ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_FILLER(v: Py) { ensure_init(); G__ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_WORDS() -> Py { ensure_init(); G__ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); G__ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COUNTED_NOUN() -> Py { ensure_init(); G__A_COUNTED_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNTED_NOUN(v: Py) { ensure_init(); G__A_COUNTED_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CURRENCY_PLURAL() -> Py { ensure_init(); G__A_CURRENCY_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CURRENCY_PLURAL(v: Py) { ensure_init(); G__A_CURRENCY_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONEY_PLURAL() -> Py { ensure_init(); G__A_MONEY_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONEY_PLURAL(v: Py) { ensure_init(); G__A_MONEY_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERIOD() -> Py { ensure_init(); G__A_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERIOD(v: Py) { ensure_init(); G__A_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PLURAL() -> Py { ensure_init(); G__A_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PLURAL(v: Py) { ensure_init(); G__A_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SCOPE_OF_A_THING() -> Py { ensure_init(); G__A_SCOPE_OF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SCOPE_OF_A_THING(v: Py) { ensure_init(); G__A_SCOPE_OF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_SCOPE() -> Py { ensure_init(); G__A_TIME_SCOPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_SCOPE(v: Py) { ensure_init(); G__A_TIME_SCOPE.with(|c| *c.borrow_mut() = v); }
pub fn g__CURRENCY() -> Py { ensure_init(); G__CURRENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CURRENCY(v: Py) { ensure_init(); G__CURRENCY.with(|c| *c.borrow_mut() = v); }
pub fn g__HELD_SCALE() -> Py { ensure_init(); G__HELD_SCALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HELD_SCALE(v: Py) { ensure_init(); G__HELD_SCALE.with(|c| *c.borrow_mut() = v); }
pub fn g__HELD_SCOPE() -> Py { ensure_init(); G__HELD_SCOPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HELD_SCOPE(v: Py) { ensure_init(); G__HELD_SCOPE.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY() -> Py { ensure_init(); G__MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); G__MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__PAY_FOR() -> Py { ensure_init(); G__PAY_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAY_FOR(v: Py) { ensure_init(); G__PAY_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__PAY_THING() -> Py { ensure_init(); G__PAY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAY_THING(v: Py) { ensure_init(); G__PAY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE() -> Py { ensure_init(); G__PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE(v: Py) { ensure_init(); G__PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAIN() -> Py { ensure_init(); G__PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAIN(v: Py) { ensure_init(); G__PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAIN_BACK() -> Py { ensure_init(); G__PLAIN_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAIN_BACK(v: Py) { ensure_init(); G__PLAIN_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_AS_A_COST() -> Py { ensure_init(); G__SAID_AS_A_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_AS_A_COST(v: Py) { ensure_init(); G__SAID_AS_A_COST.with(|c| *c.borrow_mut() = v); }
pub fn g__TAIL_OFF() -> Py { ensure_init(); G__TAIL_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAIL_OFF(v: Py) { ensure_init(); G__TAIL_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g__TRAILING() -> Py { ensure_init(); G__TRAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRAILING(v: Py) { ensure_init(); G__TRAILING.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_FIGURES() -> Py { ensure_init(); G__TWO_FIGURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_FIGURES(v: Py) { ensure_init(); G__TWO_FIGURES.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_SPENDS() -> Py { ensure_init(); G__TWO_SPENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_SPENDS(v: Py) { ensure_init(); G__TWO_SPENDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN_TAIL() -> Py { ensure_init(); G__WHEN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN_TAIL(v: Py) { ensure_init(); G__WHEN_TAIL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What they have told you they spent, and what they ask about it afterwards.\n\nPeople report money as it goes out - \"i spent 300 on food this month\", \"i paid 80 for the\ntrain\", \"40 on a takeaway\" - and then ask the one question that needs the bot to have kept it:\n\n    i spent 300 on food this month\n    how much have i spent      before  I'm not sure about that one, sorry - I'd rather not guess.\n                                      A quick search online will get you a solid answer.\n                               now     You've told me 300 on food this month. Tell me the rest\n                                       and I'll keep a tally.\n    how much did i spend on food   before  a cooking tip about shopping with a list\n                                   now     You told me 300 on food this month.\n\nNothing here writes to the store. Every line the person typed is in `memory[\"turns\"]`, so the\ntally is read from what they actually said: a spend the store did not file is still a spend they\nreported, and a reading that only reads cannot file the wrong thing.\n\nGuards, because a money word is everywhere:\n\n* the person has to be the one spending (\"i spent\", \"we paid\", \"it cost me\"), so a price asked\n  about in the world (\"how much does a pint cost\", \"how much is a train to leeds\") is left to\n  the knowledge base.\n* the amount has to be a number in the line, and the thing it went on is what follows \"on\" or\n  \"for\" (\"on food\", \"for the train\"). A line with a number and no thing (\"i spent 300\") is\n  still a spend and is counted, with nothing named.\n* an ask is only the ask. \"how much have i spent\" and \"what did i spend on food\" are read; a\n  line with a spend in it is a report, not a question about one.\n");
    s__MONEY(py_str("(\\d{1,7}(?:\\.\\d{1,2})?)"));
    s__CURRENCY(py_str("(?:\\s*(?:pounds?|quid|p\\b|gbp))?"));
    s_SPENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("\\b(?:i|we|he|she|they)(?:'ve|'s|'d)?\\s+(?:have\\s+|had\\s+|already\\s+)*(?:spent|paid|forked out|shelled out)\\s+(?:about|around|nearly|almost|just|only|maybe|like|over|under|another)?\\s*£?\\s*"), &g__MONEY()), &g__CURRENCY()), &py_str("\\s*(?:(?:on|for|in)\\s+)?(.*)$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("\\b(?:it|that|this|the\\s+[a-z]+)\\s+cost\\s+(?:me\\s+)?(?:about\\s+|around\\s+|nearly\\s+|just\\s+|only\\s+)?£?\\s*"), &g__MONEY()), &g__CURRENCY()), &py_str("\\s*(.*)$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("^\\s*(?:(?:and|then|also|so)\\s+)*(?:another\\s+)?£?\\s*"), &g__MONEY()), &g__CURRENCY()), &py_str("\\s+(?:on|for)\\s+(.+)$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WHEN_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*\\b(?:this|last|next|the)\\s+(?:morning|afternoon|evening|night|week|weekend|month|year)\\b.*$|\\s*\\b(?:today|yesterday|tonight|already|so far|altogether|in all|last night|at the weekend|on saturday|on sunday|on monday|on tuesday|on wednesday|on thursday|on friday)\\b.*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TAIL_OFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*\\b(?:too|as well|also|again|though|now|anyway)\\b\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TRAILING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s.,;:!?-]+$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\s+(?:have|has|did)\\s+(?:i|we)\\s+(?:spent|spend|pay|paid)\\b|\\bwhat\\s+(?:have|has|did)\\s+(?:i|we)\\s+spent\\b|\\bhow\\s+much\\s+(?:has|did)\\s+(?:it|that|this)\\s+cost\\b|\\bwhat\\s+(?:have|did)\\s+(?:i|we)\\s+spend\\b|\\bhow\\s+much\\s+(?:money\\s+)?(?:have\\s+i|did\\s+i)\\s+(?:spent|spend)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ON_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:on|for)\\s+([a-z][a-z' -]{1,30}?)(?:\\s+(?:this|last|so far|altogether|in all)\\b.*)?\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASK_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nso far altogether in all in total total now this month week year today all\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CATEGORIES(py_tuple(vec![py_tuple(vec![py_str("food"), py_tuple(vec![py_str("food"), py_str("groceries"), py_str("grocery"), py_str("shopping"), py_str("supermarket"), py_str("takeaway"), py_str("takeaways"), py_str("restaurant"), py_str("eating out"), py_str("lunch"), py_str("dinner"), py_str("breakfast"), py_str("brunch"), py_str("coffee"), py_str("snack"), py_str("snacks"), py_str("meal"), py_str("meals"), py_str("pizza"), py_str("curry"), py_str("chinese"), py_str("milk"), py_str("bread"), py_str("veg"), py_str("fruit"), py_str("meat"), py_str("booze"), py_str("beer"), py_str("wine")])]), py_tuple(vec![py_str("petrol"), py_tuple(vec![py_str("petrol"), py_str("fuel"), py_str("diesel"), py_str("petrol station"), py_str("gas")])]), py_tuple(vec![py_str("travel"), py_tuple(vec![py_str("train"), py_str("trains"), py_str("bus"), py_str("buses"), py_str("travel"), py_str("transport"), py_str("taxi"), py_str("uber"), py_str("tube"), py_str("train fare"), py_str("flights"), py_str("flight"), py_str("coach"), py_str("parking")])]), py_tuple(vec![py_str("bills"), py_tuple(vec![py_str("bills"), py_str("bill"), py_str("electric"), py_str("electricity"), py_str("water"), py_str("council tax"), py_str("rent"), py_str("mortgage"), py_str("broadband"), py_str("internet"), py_str("phone"), py_str("gas bill"), py_str("energy")])]), py_tuple(vec![py_str("kids"), py_tuple(vec![py_str("kids"), py_str("children"), py_str("childcare"), py_str("school"), py_str("nursery"), py_str("childminder")])])]));
    s__TWO_SPENDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?=[,;]\\s*(?:and\\s+)?£?\\d|\\s+and\\s+£?\\d)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SAID_AS_A_COST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|the)\\s+([a-z][a-z' -]{1,24}?)\\s+(?:is|are|comes? to|works out at)\\s*£?(\\d[\\d,]*(?:\\.\\d{1,2})?)\\s*(k|p|pounds?|quid|gbp)?\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nkid kids child children baby babies toddler boy boys girl girls lad lass son sons daughter\ndaughters twin twins nephew niece grandchild grandchildren grandson granddaughter mum mom\ndad stepson stepdaughter\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TWO_FIGURES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:,|and|&|or)\\s*\\d{1,3}\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PERIOD(py_str("(?:a|an|per|each|every)\\s+(?:month|week|fortnight|year)"));
    s__PAY_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:i|we)\\s+pay\\s+(?:about|around|just|only|roughly|like\\s+)?£?(\\d[\\d,]*(?:\\.\\d{1,2})?)\\s*(k|p|pounds?|quid|gbp)?\\s+"), &g__A_PERIOD()), &py_str("\\s+(?:for|on)\\s+(?:my|our|the)?\\s*([a-z][a-z' -]{1,24})"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PAY_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("\\b(?:i|we)\\s+pay\\s+(?:about|around|just|only|roughly|like\\s+)?£?(\\d[\\d,]*(?:\\.\\d{1,2})?)\\s*(k|p|pounds?|quid|gbp)?\\s+(?!a\\s+|an\\s+|the\\s+|per\\s+)([a-z][a-z' -]{2,24}?)\\s+"), &g__A_PERIOD())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PLAIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(ive|weve|theyve|youve|hes|shes|im)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PLAIN_BACK(py_dict(vec![(py_str("ive"), py_str("i've")), (py_str("weve"), py_str("we've")), (py_str("theyve"), py_str("they've")), (py_str("youve"), py_str("you've")), (py_str("hes"), py_str("he's")), (py_str("shes"), py_str("she's")), (py_str("im"), py_str("i'm"))]));
    s_PRESENT_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\s+(?:do|does)\\s+(?:i|we)\\s+(?:spend|pay)\\b|\\bwhat\\s+(?:do|does)\\s+(?:i|we)\\s+spend\\b|\\bhow\\s+much\\s+(?:is|are|was|were)\\s+(?:my|our)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_POSSESSIVE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\s+(?:is|are|was|were)\\s+(?:my|our)\\s+([a-z][a-z' -]{1,30}?)\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HELD_SCALE(py_str("(\\s*(?:k\\b|thousand\\b|grand\\b|p\\b|pounds?|quid|gbp))?"));
    s_HELD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("\\b(?:i|we)(?:'ve|'d|\\s+have|\\s+had)?\\s+(?:only\\s+|just\\s+)?(?:got\\s+|have\\s+)?(?:only\\s+|just\\s+|about\\s+|around\\s+|maybe\\s+|like\\s+)*£?\\s*"), &g__MONEY()), &g__HELD_SCALE()), &py_str("(.*)$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BUDGET_HELD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|our)\\s+budget(?:\\s+is|'?s|\\s+of)?\\s+(?:only\\s+|just\\s+|about\\s+|around\\s+|maybe\\s+)?£?\\s*"), &g__MONEY()), &g__HELD_SCALE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAVINGS_HELD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?:savings|saving)\\s+(?:are|is|stand\\s+at|come\\s+to)\\s+(?:only\\s+|just\\s+|about\\s+|around\\s+|maybe\\s+)?£?\\s*"), &g__MONEY()), &py_str("(?!\\s*(?:k|p)\\b)"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ON_A_BUDGET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:i|we)(?:'m|'re|'ve)?\\s+(?:on|working\\s+to|sticking\\s+to|sticking\\s+with)\\s+a\\s+budget\\s+of\\s+(?:only\\s+|just\\s+|about\\s+|around\\s+|maybe\\s+)?£?\\s*"), &g__MONEY()), &py_str("(\\s*(?:k\\b|p\\b|pounds?|quid|gbp))?"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THERE_LEFT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\bthere(?:'s|s|\\s+is|\\s+are|\\s+was)\\s+(?:only\\s+|just\\s+|about\\s+|around\\s+|maybe\\s+|like\\s+|still\\s+|nearly\\s+)*(?:a\\s+)?£?\\s*"), &g__MONEY()), &py_str("(\\s*(?:k\\b|p\\b|pounds?|quid|gbp))?(?=\\s*(?:left|remaining|to\\s+last)\\b)"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_COUNTED_NOUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+\\s+([a-z]{3,}(?:s|es))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CURRENCY_PLURAL(py_tuple(vec![py_str("pounds"), py_str("quid"), py_str("euros"), py_str("dollars"), py_str("bucks"), py_str("pence"), py_str("pennies"), py_str("cents"), py_str("sterling")]));
    s__A_TIME_SCOPE(op_add(&op_add(&py_str("(?![ \\t]*(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("week"), py_str("weekend"), py_str("month"), py_str("day"), py_str("days"), py_str("year"), py_str("fortnight"), py_str("holiday"), py_str("holidays"), py_str("term"), py_str("morning"), py_str("evening"), py_str("night"), py_str("month\\s+end")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b)")));
    s__A_SCOPE_OF_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("\\b(?:left|remaining)\\s+(?:over\\s+)?(?:of|in|on|from|at|for)\\s+(?:the|a|an|my|our|his|her|their|this|that)"), &g__A_TIME_SCOPE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MONEY_PLURAL(py_tuple(vec![py_str("things"), py_str("days"), py_str("weeks"), py_str("months"), py_str("years"), py_str("minutes"), py_str("hours"), py_str("pounds"), py_str("quid"), py_str("euros"), py_str("dollars"), py_str("pence"), py_str("savings"), py_str("notes"), py_str("coins"), py_str("bills"), py_str("pennies"), py_str("payments"), py_str("leftovers")]));
    s__A_PLURAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b([a-z]{4,}(?:s|es))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HELD_SCOPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:left|till|until|to\\s+last|this\\s+(?:week|month|payday)|for\\s+the\\s+(?:week|month)|a\\s+(?:week|month)\\b|the\\s+(?:week|month)\\b|saved|put\\s+(?:by|away|aside)|set\\s+aside|in\\s+(?:the\\s+bank|savings|an?\\s+account)|in\\s+the\\s+building\\s+society)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEFT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\s+(?:is\\s+|was\\s+)?(?:(?:have|has|do|does|did)\\s+(?:i|we)\\s+)?(?:got\\s+|have\\s+)?(?:left|remaining)\\b|\\bwhat(?:'s| is|s)\\s+(?:left|remaining)\\b|\\bwhat\\s+(?:have|do)\\s+(?:i|we)\\s+(?:got\\s+)?(?:left|remaining)\\b|\\bhow\\s+much\\s+(?:of\\s+)?(?:it|that|the\\s+[a-z]+)\\b[^?]{0,20}\\b(?:left|remaining)\\b|\\b(?:how\\s+much|what)\\s+did\\s+(?:i|we)\\s+(?:tell\\s+you|say)\\b[^?]{0,30}\\b(?:left|remaining)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AFFORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcan\\s+(?:i|we)\\s+afford\\s+(.*)$|\\bis\\s+there\\s+enough\\s+(?:left\\s+)?(?:for|to)\\s+(.*)$|\\b(?:do|have)\\s+(?:i|we)\\s+(?:have|got)\\s+enough\\s+(?:left\\s+)?(?:for|to)\\s+(.*)$|\\benough\\s+left\\s+for\\s+(.*)$|\\b(?:is|was|will|would)\\s+(?:that|this|it)\\s+(?:be\\s+)?enough\\s+(?:for|to)\\s+(.*)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PRICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("£\\s*(?P<sigil>\\d[\\d,]*(?:\\.\\d{1,2})?)|(?P<word>\\d[\\d,]*(?:\\.\\d{1,2})?)\\s*(?:pounds?|quid|gbp)\\b|(?:for|under|about|around)\\s+(?P<ask>\\d[\\d,]*(?:\\.\\d{1,2})?)|(?P<bare>\\d[\\d,]*(?:\\.\\d{1,2})?)\\s+(?P<item>[a-z][a-z'-]*)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an and any afford affordt can could do does did enough for get got\nhave has how i is it its left me much my of much's remaining some that the there this to was we\nwhat what's whats will would you your im i'm we're t left-over"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WHAT_IF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what\\s+)?if\\s+(?:i|we)\\s+(?:spend|blow|use\\s+up|spend\\s+up)\\s+(?:another\\s+|a\\s+further\\s+|just\\s+)?£?\\s*(?P<amount>\\d[\\d,]*(?:\\.\\d{1,2})?)(?P<currency>\\s*(?:k\\b|p\\b|pounds?|quid|gbp))?(?P<tail>.*)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LENGTH_NOT_MONEY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|nights?|times?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _amount(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let _ = py_str("The figure as the person typed it: 300, 12.50.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = number.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
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
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let _ = py_str("The thing the money went on, in the words the person used, or \"\".");
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return py_str("");
    }
    rest = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;.]")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    rest = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("at "), py_str("in ")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "lower", args))(&__args) }, "startswith", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:at|in|on|for)\\s+")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { rest.clone() };
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WHEN_TAIL(), "sub", args))(&__args) };
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TAIL_OFF(), "sub", args))(&__args) };
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TRAILING(), "sub", args))(&__args) }, "strip", args))(&__args) };
    return if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) { rest.clone() } else { py_str("") };
    Py::None
}
pub fn _a_bill(args: &[Py]) -> Py {
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
    let mut amount_group: Py = Py::None;
    let mut currency_group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut thing_group: Py = Py::None;
    let _ = py_str("The money a line says a thing of theirs costs, or None.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g__SAID_AS_A_COST(), Py::Int(2), Py::Int(1), Py::Int(3)]), py_tuple(vec![g__PAY_FOR(), Py::Int(1), Py::Int(3), Py::Int(2)]), py_tuple(vec![g__PAY_THING(), Py::Int(1), Py::Int(3), Py::Int(2)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    pattern = __unpk2[0].clone();
    amount_group = __unpk2[1].clone();
    thing_group = __unpk2[2].clone();
    currency_group = __unpk2[3].clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing_group.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _family(args))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(currency_group.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(thing.clone()); __args.push(pattern.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_not_a_bill(args))(&__args) } } }) {
    continue;
    }
    return py_dict(vec![(py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount_group.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount(args))(&__args) }), (py_str("thing"), thing.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _an_age_not_a_bill(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut pattern: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let _ = py_str("Whether a cost said as a fact is really somebody's age.");
    if py_truthy(&Py::Bool(!py_is(&pattern.clone(), &g__SAID_AS_A_COST()))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__PEOPLE())) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = slice_or_raise(&gattr(&match_.clone(), "string"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TWO_FIGURES(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _one(args: &[Py]) -> Py {
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
    let _ = py_str("The spend one clause reports, or None.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPENT(), "search", args))(&__args) };
    if py_truthy(&match_.clone()) {
    return py_dict(vec![(py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount(args))(&__args) }), (py_str("thing"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) })]);
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COST(), "search", args))(&__args) };
    if py_truthy(&match_.clone()) {
    return py_dict(vec![(py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount(args))(&__args) }), (py_str("thing"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) })]);
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_ON(), "match", args))(&__args) };
    if py_truthy(&match_.clone()) {
    return py_dict(vec![(py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount(args))(&__args) }), (py_str("thing"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) })]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_bill(args))(&__args) };
    Py::None
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("The line with the pronoun contractions spelled the way the readers expect.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); geti(&g__PLAIN_BACK(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&m.clone(), "group", args))(&__args) }, "lower", args))(&__args) }) }) }); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PLAIN(), "sub", args))(&__args) };
    Py::None
}
pub fn readings(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut spend: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Every spend a line reports, in the order they were said.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _plain(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return py_list(vec![]);
    }
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TWO_SPENDS(), "split", args))(&__args) });
    for __item in __iter1 {
    clause = __item.clone();
    spend = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _one(args))(&__args) };
    if py_truthy(&spend.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spend.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    }
    return found.clone();
    Py::None
}
pub fn reading(args: &[Py]) -> Py {
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
    let _ = py_str("The first spend a line reports, as {\"amount\": \"300\", \"thing\": \"food\"}, or None.\n\n    Only the person's own money counts, and only where the amount is in the line.\n    ");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| readings(args))(&__args) };
    return if py_truthy(&found.clone()) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn the_lines(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Every line the person has said this chat, oldest first, with `text` last if given.");
    lines = py_list(vec![]);
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    let __iter1 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { turns.clone() } else { py_tuple(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&lines.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&lines.clone(), &op_neg(&Py::Int(1))), &text.clone())) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    }
    return lines.clone();
    Py::None
}
pub fn the_spends(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut spend: Py = Py::None;
    let _ = py_str("What they have said they spent, oldest first: [{\"amount\", \"thing\", \"line\"}].");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines(args))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| readings(args))(&__args) });
    for __item in __iter2 {
    spend = __item.clone();
    let _ = py_setitem(&spend.clone(), &py_str("line"), line.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spend.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    }
    return found.clone();
    Py::None
}
pub fn _named(args: &[Py]) -> Py {
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
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let _ = py_str("Whether a spend went on the very thing the present-tense ask names.\n\n    Stricter than `_about`, which shares a family: \"how much do i spend on water\" is the water\n    bill they gave, not the electricity one that sits in the same family of bills.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }, "strip", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = wanted.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }, "strip", args))(&__args) }]), 2, None);
    thing = __unpk1[0].clone();
    wanted = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wanted.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&thing.clone(), &wanted.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&wanted.clone(), &thing.clone())) } }) {
    return Py::Bool(true);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "rstrip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&thing.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "rstrip", args))(&__args) })) } };
    Py::None
}
pub fn _about(args: &[Py]) -> Py {
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
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let _ = py_str("Whether the thing a spend went on is the thing the ask names.\n\n    The ask often reaches for a broader word than the spend did: \"40 on a takeaway\" is money on\n    food, and \"12.50 on lunch\" is too, so \"how much did i spend on food\" counts both. Only the\n    words in one family are shared, so \"how much did i spend on petrol\" is still just petrol.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "lower", args))(&__args) }, "strip", args))(&__args) }]), 2, None);
    thing = __unpk1[0].clone();
    wanted = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&thing.clone(), &wanted.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&wanted.clone(), &thing.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "rstrip", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&thing.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "rstrip", args))(&__args) })) } } } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _family(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _family(args))(&__args) }, "isdisjoint", args))(&__args) }));
    Py::None
}
pub fn _family(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut member: Py = Py::None;
    let mut members: Py = Py::None;
    let mut name: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words one spend and one ask can both be named by: a broad name and its members.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_CATEGORIES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    members = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &name.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&members.clone()) { let mut member: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &member.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(members.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "update", args))(&__args) };
    }
    }
    return words.clone();
    Py::None
}
pub fn the_total(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut spends: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spends") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spends"))) };
    let mut spend: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("The spends added up, as a figure that reads like the ones they typed.");
    total = Py::Float(0.0);
    let __iter1 = py_iter_or_raise(&spends.clone());
    for __item in __iter1 {
    spend = __item.clone();
    let __try2 = exc::catch(|| {
    let __aug7 = op_add(&total.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("amount")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    total = __aug7;
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    return py_str("");
    }
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&total.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(total.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(0.005))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(total.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return op_modulo(&py_str("%.2f"), &total.clone());
    Py::None
}
pub fn _listed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut spends: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spends") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spends"))) };
    let mut parts: Py = Py::None;
    let mut spend: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("\"300 on food and 40 on the train\".");
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&spends.clone());
    for __item in __iter1 {
    spend = __item.clone();
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend.clone(), "get", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&geti(&spend.clone(), &py_str("amount")), &if py_truthy(&thing.clone()) { op_add(&py_str(" on "), &thing.clone()) } else { py_str("") })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&parts.clone(), "append", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return geti(&parts.clone(), &Py::Int(0));
    }
    return op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(", "), "join", args))(&__args) }, &py_str(" and ")), &geti(&parts.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut mine: Py = Py::None;
    let mut on: Py = Py::None;
    let mut past: Py = Py::None;
    let mut present: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spend: Py = Py::None;
    let mut spends: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reply for \"how much have i spent\" and its many wordings, or None.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    past = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    present = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRESENT_ASK(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&past.clone())) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&past.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&present.clone())) } }) {
    return Py::None;
    }
    wanted = py_str("");
    on = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ON_A_THING(), "search", args))(&__args) };
    if py_truthy(&on.clone()) {
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&on.clone(), "group", args))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ASK_FILLER(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    if py_truthy(&present.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_POSSESSIVE_ASK(), "search", args))(&__args) };
    if py_truthy(&mine.clone()) {
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mine.clone(), "group", args))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ASK_FILLER(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    spends = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_spends(args))(&__args) }) { let mut spend: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named(args))(&__args) }) {  __out.push(spend.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&spends.clone()))) {
    return Py::None;
    }
    } else {
    spends = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_spends(args))(&__args) };
    }
    if py_truthy(&wanted.clone()) {
    spends = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spends.clone()) { let mut spend: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about(args))(&__args) }) {  __out.push(spend.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&spends.clone()))) {
    return op_add(&op_add(&py_str("You haven't told me what you spent on "), &wanted.clone()), &py_str(". Tell me and I'll keep a tally."));
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return op_add(&op_add(&py_str("You told me "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str("."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You've told me "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(" - ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_total(args))(&__args) }), &py_str(" on ")), &wanted.clone()), &py_str(" in all."));
    }
    if py_truthy(&Py::Bool(!py_truthy(&spends.clone()))) {
    return py_str("You haven't told me what you've spent yet. Tell me the sums as they go out and I'll keep a tally.");
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return op_add(&op_add(&py_str("You've told me "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(". Tell me the rest and I'll keep a tally."));
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("You've told me "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(" - ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_total(args))(&__args) }), &py_str(" in all."));
    Py::None
}
pub fn _a_count_of_a_thing(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the figure in the line counts something, so it is not money of theirs.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COUNTED_NOUN(), "findall", args))(&__args) });
    for __item in __iter1 {
    noun = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g__A_CURRENCY_PLURAL(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&noun.clone(), "lower", args))(&__args) }))) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SCOPE_OF_A_THING(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    before = if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("there")))) { geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("there")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }, &Py::Int(0)) } else { py_str("") };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PLURAL(), "findall", args))(&__args) });
    for __item in __iter2 {
    word = __item.clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_MONEY_PLURAL(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _figure(args: &[Py]) -> Py {
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
    let _ = py_str("A sum of money the way the person's own figures read: 20, 14, 37.50.");
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = value.clone(); if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&value.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(0.005))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return op_modulo(&py_str("%.2f"), &value.clone());
    Py::None
}
pub fn _with_scale(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut value: Py = Py::None;
    let _ = py_str("A figure with the size word written after it: \"6 thousand\" -> \"6000\", \"2k\" -> \"2000\".\n\n    Pence (\"50p\") and plain pounds are left exactly as typed: only the size words multiply, so\n    the one thing this changes is the figure a person meant when they said how much they had.\n    ");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("k")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thousand")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("grand")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) } } } } }))) {
    return number.clone();
    }
    let __try1 = exc::catch(|| {
    value = op_mul(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1000));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return number.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&value.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(0.005))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { number.clone() };
    Py::None
}
pub fn _held_in(args: &[Py]) -> Py {
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
    let mut currency: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The money one line says they have, as a figure, or None.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _plain(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    match_ = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BUDGET_HELD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ON_A_BUDGET(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAVINGS_HELD(), "search", args))(&__args) } } } } };
    if py_truthy(&match_.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_scale(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THERE_LEFT(), "search", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_count_of_a_thing(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_scale(args))(&__args) };
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HELD(), "search", args))(&__args) };
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }]), 2, None);
    currency = __unpk1[0].clone();
    tail = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&currency.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &py_str("£"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HELD_SCOPE(), "match", args))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&currency.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &py_str("£"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_count_of_a_thing(args))(&__args) } } } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_scale(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn the_held(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut amount: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The money they last said they had this chat, as {\"amount\", \"line\"}, or None.");
    found = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines(args))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _held_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&amount.clone(), &Py::None))) {
    found = py_dict(vec![(py_str("amount"), amount.clone()), (py_str("line"), line.clone())]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _a_thing_to_buy(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing an affordability ask names, in the words they used: \"a takeaway\", \"new shoes\".");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFFORD(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "groups", args))(&__args) }) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    thing = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,.!?;]")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TRAILING(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TAIL_OFF(), "sub", args))(&__args) }, "strip", args))(&__args) };
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:it|that|one|anything)\\b")); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    return if py_truthy(&{ let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_lt(&Py::Int(1), &__cmp11)) { op_lt(&__cmp11, &Py::Int(40)) } else { Py::Bool(false) } }) { thing.clone() } else { py_str("") };
    Py::None
}
pub fn _the_price(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut item: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The price a thing is named with, or None: \"a 200 coat\" -> 200.0.\n\n    The affordability question is a comparison, so the half people leave out has to be read out of\n    the line: without it the reply says a thing is covered by money it was never checked against.\n    ");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PRICE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("sigil"), py_str("word"), py_str("ask")]));
    for __item in __iter1 {
    group = __item.clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(group.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&value.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    item = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("item")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "endswith", args))(&__args) })) } }) {
    return Py::None;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bare")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("0") } }, "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return if py_truthy(&op_ge(&value.clone(), &Py::Int(10))) { value.clone() } else { Py::None };
    Py::None
}
pub fn _bare_thing(args: &[Py]) -> Py {
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
    let _ = py_str("The thing without the little word in front of it: \"a holiday\" -> \"holiday\".");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the|some|my|our|any)\\s+")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _only_the_ask(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is the sum asked and nothing else: \"how much is left\", \"can i afford it\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:'\"")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__ASK_WORDS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _nothing_before(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("True when this line is the first thing said in the chat.");
    return op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1));
    Py::None
}
pub fn _the_what_if(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut amount: Py = Py::None;
    let mut currency: Py = Py::None;
    let mut had: Py = Py::None;
    let mut held: Py = Py::None;
    let mut left: Py = Py::None;
    let mut spend: Py = Py::None;
    let mut spends: Py = Py::None;
    let _ = py_str("The sum with a spend in it that has not happened, or None.");
    currency = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("currency")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("p")]), &currency.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tail")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LENGTH_NOT_MONEY(), "match", args))(&__args) }) {
    return Py::None;
    }
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("amount")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("0") } }, "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_held(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&held.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_le(&amount.clone(), &Py::Int(0)) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    had = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&held.clone(), &py_str("amount"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    spends = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_spends(args))(&__args) };
    left = op_sub(&had.clone(), &if py_truthy(&spends.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spends.clone()) { let mut spend: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("amount")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { Py::Float(0.0) });
    if py_truthy(&op_gt(&amount.clone(), &left.clone())) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Spend "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" more and you are ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&amount.clone(), &left.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" over - ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" is what is left now."));
    }
    if py_truthy(&Py::Bool(!py_truthy(&spends.clone()))) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Spend "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" of the ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" you told me you have and ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&left.clone(), &amount.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" is left."));
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("Spend "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" more and ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&left.clone(), &amount.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" is left."));
    Py::None
}
pub fn what_is_left(args: &[Py]) -> Py {
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
    let mut cost: Py = Py::None;
    let mut had: Py = Py::None;
    let mut held: Py = Py::None;
    let mut left: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut out: Py = Py::None;
    let mut price: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spend: Py = Py::None;
    let mut spends: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut what_if: Py = Py::None;
    let _ = py_str("The reply for \"how much have i got left\" and \"can i afford a takeaway\", or None.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    what_if = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WHAT_IF(), "search", args))(&__args) };
    if py_truthy(&what_if.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(what_if.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_what_if(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEFT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFFORD(), "search", args))(&__args) } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| readings(args))(&__args) }) {
    return Py::None;
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_held(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&held.clone(), &Py::None))) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _nothing_before(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_the_ask(args))(&__args) } } }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFFORD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEFT(), "search", args))(&__args) })) } }) {
    return py_str("Afford what, and with what? Tell me the thing and what you've got to spend, and I'll do the sum.");
    }
    return py_str("Left of what? Tell me what you've got and what has gone out of it, and I'll do the sum.");
    }
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    had = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&held.clone(), &py_str("amount"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    spends = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_spends(args))(&__args) };
    if py_truthy(&spends.clone()) {
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spends.clone()) { let mut spend: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("amount")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spend.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    } else {
    out = Py::Float(0.0);
    }
    left = op_sub(&had.clone(), &out.clone());
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFFORD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEFT(), "search", args))(&__args) })) } }) {
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thing_to_buy(args))(&__args) };
    price = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_price(args))(&__args) };
    name = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bare_thing(args))(&__args) };
    named = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:a|an|the|some|my|our|any)\\s")); __args.push(thing.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { thing.clone() } else { if py_truthy(&name.clone()) { op_add(&py_str("the "), &name.clone()) } else { py_str("") } };
    cost = op_add(&if py_truthy(&named.clone()) { op_add(&py_str("Tell me the price of "), &named.clone()) } else { py_str("Tell me the price") }, &py_str(" and I'll say whether it fits."));
    if py_truthy(&{ let __b = spends.clone(); if !py_truthy(&__b) { __b } else { op_le(&left.clone(), &Py::Int(0)) } }) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You had "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" and ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(" has gone, so you are ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_neg(&left.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" over. ")), &if py_truthy(&thing.clone()) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "capitalize", args))(&__args) }, &py_str(" needs more coming in first.")) } else { py_str("Tell me if more comes in and I'll work it out again.") });
    }
    if py_truthy(&Py::Bool(!py_is(&price.clone(), &Py::None))) {
    if py_truthy(&spends.clone()) {
    if py_truthy(&op_gt(&price.clone(), &left.clone())) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("No - "), &thing.clone()), &py_str(" is more than the ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" you have left. It wants ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&price.clone(), &left.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" more."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Yes - "), &thing.clone()), &py_str(" comes out of the ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" you have left, with ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&left.clone(), &price.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" spare."));
    }
    if py_truthy(&op_gt(&price.clone(), &had.clone())) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("No - "), &thing.clone()), &py_str(" is more than the ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" you told me you have. It wants ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&price.clone(), &had.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" more."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Yes - "), &thing.clone()), &py_str(" comes out of the ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" you told me you have, with ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&had.clone(), &price.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" spare."));
    }
    if py_truthy(&Py::Bool(!py_truthy(&spends.clone()))) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You told me you have "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" and nothing has gone out of it yet, so anything up to ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" is covered. ")), &cost.clone());
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You had "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" and ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(" has gone, so ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" is left. ")), &cost.clone());
    }
    if py_truthy(&Py::Bool(!py_truthy(&spends.clone()))) {
    return op_add(&op_add(&py_str("You told me you have "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(", and you've not said that any of it has gone yet."));
    }
    if py_truthy(&op_le(&left.clone(), &Py::Int(0))) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You had "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" and ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(" has gone, so you are ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_neg(&left.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" over."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You had "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" and ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spends.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _listed(args))(&__args) }), &py_str(" has gone, so ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }), &py_str(" is left."));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.spent".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFFORD" => Some(g_AFFORD()),
            "ASK" => Some(g_ASK()),
            "A_LENGTH_NOT_MONEY" => Some(g_A_LENGTH_NOT_MONEY()),
            "A_POSSESSIVE_ASK" => Some(g_A_POSSESSIVE_ASK()),
            "A_PRICE" => Some(g_A_PRICE()),
            "A_WHAT_IF" => Some(g_A_WHAT_IF()),
            "BARE_ON" => Some(g_BARE_ON()),
            "BUDGET_HELD" => Some(g_BUDGET_HELD()),
            "CATEGORIES" => Some(g_CATEGORIES()),
            "COST" => Some(g_COST()),
            "HELD" => Some(g_HELD()),
            "LEFT" => Some(g_LEFT()),
            "ON_A_BUDGET" => Some(g_ON_A_BUDGET()),
            "ON_A_THING" => Some(g_ON_A_THING()),
            "PRESENT_ASK" => Some(g_PRESENT_ASK()),
            "SAVINGS_HELD" => Some(g_SAVINGS_HELD()),
            "SPENT" => Some(g_SPENT()),
            "THERE_LEFT" => Some(g_THERE_LEFT()),
            "_ASK_FILLER" => Some(g__ASK_FILLER()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_A_COUNTED_NOUN" => Some(g__A_COUNTED_NOUN()),
            "_A_CURRENCY_PLURAL" => Some(g__A_CURRENCY_PLURAL()),
            "_A_MONEY_PLURAL" => Some(g__A_MONEY_PLURAL()),
            "_A_PERIOD" => Some(g__A_PERIOD()),
            "_A_PLURAL" => Some(g__A_PLURAL()),
            "_A_SCOPE_OF_A_THING" => Some(g__A_SCOPE_OF_A_THING()),
            "_A_TIME_SCOPE" => Some(g__A_TIME_SCOPE()),
            "_CURRENCY" => Some(g__CURRENCY()),
            "_HELD_SCALE" => Some(g__HELD_SCALE()),
            "_HELD_SCOPE" => Some(g__HELD_SCOPE()),
            "_MONEY" => Some(g__MONEY()),
            "_PAY_FOR" => Some(g__PAY_FOR()),
            "_PAY_THING" => Some(g__PAY_THING()),
            "_PEOPLE" => Some(g__PEOPLE()),
            "_PLAIN" => Some(g__PLAIN()),
            "_PLAIN_BACK" => Some(g__PLAIN_BACK()),
            "_SAID_AS_A_COST" => Some(g__SAID_AS_A_COST()),
            "_TAIL_OFF" => Some(g__TAIL_OFF()),
            "_TRAILING" => Some(g__TRAILING()),
            "_TWO_FIGURES" => Some(g__TWO_FIGURES()),
            "_TWO_SPENDS" => Some(g__TWO_SPENDS()),
            "_WHEN_TAIL" => Some(g__WHEN_TAIL()),
            "_a_bill" => Some(py_func("_a_bill", |args: &[Py]| _a_bill(args))),
            "_a_count_of_a_thing" => Some(py_func("_a_count_of_a_thing", |args: &[Py]| _a_count_of_a_thing(args))),
            "_a_thing_to_buy" => Some(py_func("_a_thing_to_buy", |args: &[Py]| _a_thing_to_buy(args))),
            "_about" => Some(py_func("_about", |args: &[Py]| _about(args))),
            "_amount" => Some(py_func("_amount", |args: &[Py]| _amount(args))),
            "_an_age_not_a_bill" => Some(py_func("_an_age_not_a_bill", |args: &[Py]| _an_age_not_a_bill(args))),
            "_bare_thing" => Some(py_func("_bare_thing", |args: &[Py]| _bare_thing(args))),
            "_family" => Some(py_func("_family", |args: &[Py]| _family(args))),
            "_figure" => Some(py_func("_figure", |args: &[Py]| _figure(args))),
            "_held_in" => Some(py_func("_held_in", |args: &[Py]| _held_in(args))),
            "_listed" => Some(py_func("_listed", |args: &[Py]| _listed(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_nothing_before" => Some(py_func("_nothing_before", |args: &[Py]| _nothing_before(args))),
            "_one" => Some(py_func("_one", |args: &[Py]| _one(args))),
            "_only_the_ask" => Some(py_func("_only_the_ask", |args: &[Py]| _only_the_ask(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_the_price" => Some(py_func("_the_price", |args: &[Py]| _the_price(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_what_if" => Some(py_func("_the_what_if", |args: &[Py]| _the_what_if(args))),
            "_with_scale" => Some(py_func("_with_scale", |args: &[Py]| _with_scale(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "reading" => Some(py_func("reading", |args: &[Py]| reading(args))),
            "readings" => Some(py_func("readings", |args: &[Py]| readings(args))),
            "the_held" => Some(py_func("the_held", |args: &[Py]| the_held(args))),
            "the_lines" => Some(py_func("the_lines", |args: &[Py]| the_lines(args))),
            "the_spends" => Some(py_func("the_spends", |args: &[Py]| the_spends(args))),
            "the_total" => Some(py_func("the_total", |args: &[Py]| the_total(args))),
            "what_is_left" => Some(py_func("what_is_left", |args: &[Py]| what_is_left(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AFFORD" => s_AFFORD(v),
            "ASK" => s_ASK(v),
            "A_LENGTH_NOT_MONEY" => s_A_LENGTH_NOT_MONEY(v),
            "A_POSSESSIVE_ASK" => s_A_POSSESSIVE_ASK(v),
            "A_PRICE" => s_A_PRICE(v),
            "A_WHAT_IF" => s_A_WHAT_IF(v),
            "BARE_ON" => s_BARE_ON(v),
            "BUDGET_HELD" => s_BUDGET_HELD(v),
            "CATEGORIES" => s_CATEGORIES(v),
            "COST" => s_COST(v),
            "HELD" => s_HELD(v),
            "LEFT" => s_LEFT(v),
            "ON_A_BUDGET" => s_ON_A_BUDGET(v),
            "ON_A_THING" => s_ON_A_THING(v),
            "PRESENT_ASK" => s_PRESENT_ASK(v),
            "SAVINGS_HELD" => s_SAVINGS_HELD(v),
            "SPENT" => s_SPENT(v),
            "THERE_LEFT" => s_THERE_LEFT(v),
            "_ASK_FILLER" => s__ASK_FILLER(v),
            "_ASK_WORDS" => s__ASK_WORDS(v),
            "_A_COUNTED_NOUN" => s__A_COUNTED_NOUN(v),
            "_A_CURRENCY_PLURAL" => s__A_CURRENCY_PLURAL(v),
            "_A_MONEY_PLURAL" => s__A_MONEY_PLURAL(v),
            "_A_PERIOD" => s__A_PERIOD(v),
            "_A_PLURAL" => s__A_PLURAL(v),
            "_A_SCOPE_OF_A_THING" => s__A_SCOPE_OF_A_THING(v),
            "_A_TIME_SCOPE" => s__A_TIME_SCOPE(v),
            "_CURRENCY" => s__CURRENCY(v),
            "_HELD_SCALE" => s__HELD_SCALE(v),
            "_HELD_SCOPE" => s__HELD_SCOPE(v),
            "_MONEY" => s__MONEY(v),
            "_PAY_FOR" => s__PAY_FOR(v),
            "_PAY_THING" => s__PAY_THING(v),
            "_PEOPLE" => s__PEOPLE(v),
            "_PLAIN" => s__PLAIN(v),
            "_PLAIN_BACK" => s__PLAIN_BACK(v),
            "_SAID_AS_A_COST" => s__SAID_AS_A_COST(v),
            "_TAIL_OFF" => s__TAIL_OFF(v),
            "_TRAILING" => s__TRAILING(v),
            "_TWO_FIGURES" => s__TWO_FIGURES(v),
            "_TWO_SPENDS" => s__TWO_SPENDS(v),
            "_WHEN_TAIL" => s__WHEN_TAIL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.spent", module_obj);
    pyrt::xrefs::reg_fn("memory.spent", "_amount", py_func("_amount", |args: &[Py]| _amount(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_a_bill", py_func("_a_bill", |args: &[Py]| _a_bill(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_an_age_not_a_bill", py_func("_an_age_not_a_bill", |args: &[Py]| _an_age_not_a_bill(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_one", py_func("_one", |args: &[Py]| _one(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("memory.spent", "readings", py_func("readings", |args: &[Py]| readings(args)));
    pyrt::xrefs::reg_fn("memory.spent", "reading", py_func("reading", |args: &[Py]| reading(args)));
    pyrt::xrefs::reg_fn("memory.spent", "the_lines", py_func("the_lines", |args: &[Py]| the_lines(args)));
    pyrt::xrefs::reg_fn("memory.spent", "the_spends", py_func("the_spends", |args: &[Py]| the_spends(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_about", py_func("_about", |args: &[Py]| _about(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_family", py_func("_family", |args: &[Py]| _family(args)));
    pyrt::xrefs::reg_fn("memory.spent", "the_total", py_func("the_total", |args: &[Py]| the_total(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_listed", py_func("_listed", |args: &[Py]| _listed(args)));
    pyrt::xrefs::reg_fn("memory.spent", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_a_count_of_a_thing", py_func("_a_count_of_a_thing", |args: &[Py]| _a_count_of_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_figure", py_func("_figure", |args: &[Py]| _figure(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_with_scale", py_func("_with_scale", |args: &[Py]| _with_scale(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_held_in", py_func("_held_in", |args: &[Py]| _held_in(args)));
    pyrt::xrefs::reg_fn("memory.spent", "the_held", py_func("the_held", |args: &[Py]| the_held(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_a_thing_to_buy", py_func("_a_thing_to_buy", |args: &[Py]| _a_thing_to_buy(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_the_price", py_func("_the_price", |args: &[Py]| _the_price(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_bare_thing", py_func("_bare_thing", |args: &[Py]| _bare_thing(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_only_the_ask", py_func("_only_the_ask", |args: &[Py]| _only_the_ask(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_nothing_before", py_func("_nothing_before", |args: &[Py]| _nothing_before(args)));
    pyrt::xrefs::reg_fn("memory.spent", "_the_what_if", py_func("_the_what_if", |args: &[Py]| _the_what_if(args)));
    pyrt::xrefs::reg_fn("memory.spent", "what_is_left", py_func("what_is_left", |args: &[Py]| what_is_left(args)));
    pyrt::xrefs::reg_access("memory.spent", "AFFORD", g_AFFORD, s_AFFORD);
    pyrt::xrefs::reg_access("memory.spent", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("memory.spent", "A_LENGTH_NOT_MONEY", g_A_LENGTH_NOT_MONEY, s_A_LENGTH_NOT_MONEY);
    pyrt::xrefs::reg_access("memory.spent", "A_POSSESSIVE_ASK", g_A_POSSESSIVE_ASK, s_A_POSSESSIVE_ASK);
    pyrt::xrefs::reg_access("memory.spent", "A_PRICE", g_A_PRICE, s_A_PRICE);
    pyrt::xrefs::reg_access("memory.spent", "A_WHAT_IF", g_A_WHAT_IF, s_A_WHAT_IF);
    pyrt::xrefs::reg_access("memory.spent", "BARE_ON", g_BARE_ON, s_BARE_ON);
    pyrt::xrefs::reg_access("memory.spent", "BUDGET_HELD", g_BUDGET_HELD, s_BUDGET_HELD);
    pyrt::xrefs::reg_access("memory.spent", "CATEGORIES", g_CATEGORIES, s_CATEGORIES);
    pyrt::xrefs::reg_access("memory.spent", "COST", g_COST, s_COST);
    pyrt::xrefs::reg_access("memory.spent", "HELD", g_HELD, s_HELD);
    pyrt::xrefs::reg_access("memory.spent", "LEFT", g_LEFT, s_LEFT);
    pyrt::xrefs::reg_access("memory.spent", "ON_A_BUDGET", g_ON_A_BUDGET, s_ON_A_BUDGET);
    pyrt::xrefs::reg_access("memory.spent", "ON_A_THING", g_ON_A_THING, s_ON_A_THING);
    pyrt::xrefs::reg_access("memory.spent", "PRESENT_ASK", g_PRESENT_ASK, s_PRESENT_ASK);
    pyrt::xrefs::reg_access("memory.spent", "SAVINGS_HELD", g_SAVINGS_HELD, s_SAVINGS_HELD);
    pyrt::xrefs::reg_access("memory.spent", "SPENT", g_SPENT, s_SPENT);
    pyrt::xrefs::reg_access("memory.spent", "THERE_LEFT", g_THERE_LEFT, s_THERE_LEFT);
    pyrt::xrefs::reg_access("memory.spent", "_ASK_FILLER", g__ASK_FILLER, s__ASK_FILLER);
    pyrt::xrefs::reg_access("memory.spent", "_ASK_WORDS", g__ASK_WORDS, s__ASK_WORDS);
    pyrt::xrefs::reg_access("memory.spent", "_A_COUNTED_NOUN", g__A_COUNTED_NOUN, s__A_COUNTED_NOUN);
    pyrt::xrefs::reg_access("memory.spent", "_A_CURRENCY_PLURAL", g__A_CURRENCY_PLURAL, s__A_CURRENCY_PLURAL);
    pyrt::xrefs::reg_access("memory.spent", "_A_MONEY_PLURAL", g__A_MONEY_PLURAL, s__A_MONEY_PLURAL);
    pyrt::xrefs::reg_access("memory.spent", "_A_PERIOD", g__A_PERIOD, s__A_PERIOD);
    pyrt::xrefs::reg_access("memory.spent", "_A_PLURAL", g__A_PLURAL, s__A_PLURAL);
    pyrt::xrefs::reg_access("memory.spent", "_A_SCOPE_OF_A_THING", g__A_SCOPE_OF_A_THING, s__A_SCOPE_OF_A_THING);
    pyrt::xrefs::reg_access("memory.spent", "_A_TIME_SCOPE", g__A_TIME_SCOPE, s__A_TIME_SCOPE);
    pyrt::xrefs::reg_access("memory.spent", "_CURRENCY", g__CURRENCY, s__CURRENCY);
    pyrt::xrefs::reg_access("memory.spent", "_HELD_SCALE", g__HELD_SCALE, s__HELD_SCALE);
    pyrt::xrefs::reg_access("memory.spent", "_HELD_SCOPE", g__HELD_SCOPE, s__HELD_SCOPE);
    pyrt::xrefs::reg_access("memory.spent", "_MONEY", g__MONEY, s__MONEY);
    pyrt::xrefs::reg_access("memory.spent", "_PAY_FOR", g__PAY_FOR, s__PAY_FOR);
    pyrt::xrefs::reg_access("memory.spent", "_PAY_THING", g__PAY_THING, s__PAY_THING);
    pyrt::xrefs::reg_access("memory.spent", "_PEOPLE", g__PEOPLE, s__PEOPLE);
    pyrt::xrefs::reg_access("memory.spent", "_PLAIN", g__PLAIN, s__PLAIN);
    pyrt::xrefs::reg_access("memory.spent", "_PLAIN_BACK", g__PLAIN_BACK, s__PLAIN_BACK);
    pyrt::xrefs::reg_access("memory.spent", "_SAID_AS_A_COST", g__SAID_AS_A_COST, s__SAID_AS_A_COST);
    pyrt::xrefs::reg_access("memory.spent", "_TAIL_OFF", g__TAIL_OFF, s__TAIL_OFF);
    pyrt::xrefs::reg_access("memory.spent", "_TRAILING", g__TRAILING, s__TRAILING);
    pyrt::xrefs::reg_access("memory.spent", "_TWO_FIGURES", g__TWO_FIGURES, s__TWO_FIGURES);
    pyrt::xrefs::reg_access("memory.spent", "_TWO_SPENDS", g__TWO_SPENDS, s__TWO_SPENDS);
    pyrt::xrefs::reg_access("memory.spent", "_WHEN_TAIL", g__WHEN_TAIL, s__WHEN_TAIL);
}
