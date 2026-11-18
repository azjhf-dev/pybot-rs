#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
pub mod a_cake_between_its_layers;
pub mod a_check_with_you;
pub mod a_count_of_packets;
pub mod a_count_said;
pub mod a_day_somebody_else_keeps;
pub mod a_debt_said_flat;
pub mod a_fact_asked_again;
pub mod a_figure_beside_the_state;
pub mod a_figure_of_a_dish;
pub mod a_first_time;
pub mod a_habit_is_raised;
pub mod a_job_that_is_not_a_diy_job;
pub mod a_letter_about_money;
pub mod a_limit_over_the_answer;
pub mod a_measure_asked_again;
pub mod a_number_the_bot_knows;
pub mod a_plan_the_thread_holds;
pub mod a_question_about_somebody;
pub mod a_return_brings_back_the_advice;
pub mod a_shock_at_a_bill;
pub mod a_thing_being_weighed;
pub mod a_thing_wants_doing;
pub mod a_will_it_work;
pub mod aboutachild;
pub mod after_a_lead_in;
pub mod already;
pub mod an_age_ahead;
pub mod answers;
pub mod arbiter;
pub mod attribute_said_again;
pub mod bulky;
pub mod callback_about;
pub mod carers_turn;
pub mod chasing_a_body;
pub mod chat;
pub mod chat_replies;
pub mod colour_on_the_body;
pub mod confusing;
pub mod detail_question;
pub mod done_ask;
pub mod doubt;
pub mod dreams;
pub mod familyask;
pub mod favourites;
pub mod figure_asked_again;
pub mod first_line_open;
pub mod greeting;
pub mod grief;
pub mod group_ages;
pub mod hardpast;
pub mod household;
pub mod how_long_a_stay;
pub mod intent;
pub mod invitation;
pub mod judge_ask;
pub mod keeping;
pub mod kid_tally;
pub mod live_source;
pub mod live_times;
pub mod loss;
pub mod memory_store;
pub mod memory_tables;
pub mod my_work;
pub mod name_taken_back;
pub mod near_me;
pub mod night;
pub mod no_advice;
pub mod nudge;
pub mod on_record;
pub mod onsubject;
pub mod pace;
pub mod particles;
pub mod recap;
pub mod receiving_end;
pub mod record_due;
pub mod ruled_out;
pub mod safety;
pub mod safety_cards;
pub mod safety_phrases;
pub mod safety_tables;
pub mod said_back;
pub mod said_fact_back;
pub mod saving_followup;
pub mod since_it_began;
pub mod steps;
pub mod taking_asked_again;
pub mod talkto;
pub mod the_detail_the_thread_adds;
pub mod the_kind_one_line_up;
pub mod the_month_they_have;
pub mod the_shape_of_the_answer;
pub mod the_trouble_it_has;
pub mod the_trouble_named;
pub mod the_wait_about_a_body;
pub mod their_life;
pub mod their_pastime;
pub mod their_taste;
pub mod theirperson;
pub mod thing_i_did_with;
pub mod trip_date_said_again;
pub mod understand;
pub mod urgent;
pub mod ward;
pub mod what_decides_it;
pub mod what_they_said;
pub mod where_it_answers;
pub mod who_to_see;
pub mod words;
pub mod years_in;
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
fn module_body() {
    let _ = py_str("Add topic or feature modules here; see CONTRIBUTING.md.");
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules".to_string(),
        get: Rc::new(|attr: &str| match attr {
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules", module_obj);
}
