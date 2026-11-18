//! Registry of bot/modules entries, arbiter and cross-crate registration.
#![allow(dead_code)]
use pyrt::value::*;
use pyrt::module_entry::ModuleEntry;

pub fn register_all() {
    gen___main__::register_all();
    gen_compose::register_all();
    gen_compose_a_count_of_packets::register_all();
    gen_compose_a_detail_dropped::register_all();
    gen_compose_a_document_lives_with_an_office::register_all();
    gen_compose_a_first_time::register_all();
    gen_compose_a_number_the_bot_knows::register_all();
    gen_compose_a_place_nobody_can_see::register_all();
    gen_compose_a_thing_being_weighed::register_all();
    gen_compose_a_thing_wants_doing::register_all();
    gen_compose_a_trouble::register_all();
    gen_compose_a_wait_about_a_body::register_all();
    gen_compose_aboutness::register_all();
    gen_compose_badnews::register_all();
    gen_compose_breakup::register_all();
    gen_compose_check_with_you::register_all();
    gen_compose_checking::register_all();
    gen_compose_childwords::register_all();
    gen_compose_circumstance::register_all();
    gen_compose_clause::register_all();
    gen_compose_colour_on_the_body::register_all();
    gen_compose_doubt::register_all();
    gen_compose_draft::register_all();
    gen_compose_facts::register_all();
    gen_compose_fallback::register_all();
    gen_compose_fallback_chain::register_all();
    gen_compose_fallback_data::register_all();
    gen_compose_fallback_pools::register_all();
    gen_compose_fallback_words::register_all();
    gen_compose_find_a_place::register_all();
    gen_compose_finding_out::register_all();
    gen_compose_findout::register_all();
    gen_compose_fit::register_all();
    gen_compose_for_the_day::register_all();
    gen_compose_gap::register_all();
    gen_compose_goodnews::register_all();
    gen_compose_griefplea::register_all();
    gen_compose_habit::register_all();
    gen_compose_handoff::register_all();
    gen_compose_hands::register_all();
    gen_compose_harm::register_all();
    gen_compose_hedge::register_all();
    gen_compose_how_it_went::register_all();
    gen_compose_invitation::register_all();
    gen_compose_is_it_worth_it::register_all();
    gen_compose_judgement::register_all();
    gen_compose_kept_nearby::register_all();
    gen_compose_lead::register_all();
    gen_compose_lead_in::register_all();
    gen_compose_leaving::register_all();
    gen_compose_limit::register_all();
    gen_compose_liveask::register_all();
    gen_compose_loss::register_all();
    gen_compose_missing_half::register_all();
    gen_compose_morning_after::register_all();
    gen_compose_moving::register_all();
    gen_compose_nearby_one::register_all();
    gen_compose_news_ask::register_all();
    gen_compose_no_advice::register_all();
    gen_compose_no_eyes::register_all();
    gen_compose_no_page::register_all();
    gen_compose_not_knowing::register_all();
    gen_compose_nottheirbest::register_all();
    gen_compose_occasionplan::register_all();
    gen_compose_onething::register_all();
    gen_compose_only_they_know::register_all();
    gen_compose_onmybehalf::register_all();
    gen_compose_onscreen::register_all();
    gen_compose_onsubject::register_all();
    gen_compose_outlook::register_all();
    gen_compose_own_ask::register_all();
    gen_compose_own_facts::register_all();
    gen_compose_pace::register_all();
    gen_compose_place_facts::register_all();
    gen_compose_plan_ask::register_all();
    gen_compose_plans::register_all();
    gen_compose_pleased::register_all();
    gen_compose_postcode::register_all();
    gen_compose_prices::register_all();
    gen_compose_quantity::register_all();
    gen_compose_reaction::register_all();
    gen_compose_reading::register_all();
    gen_compose_recall::register_all();
    gen_compose_receiving_end::register_all();
    gen_compose_remark::register_all();
    gen_compose_repair::register_all();
    gen_compose_rotten_luck::register_all();
    gen_compose_routes::register_all();
    gen_compose_rule::register_all();
    gen_compose_ruledout::register_all();
    gen_compose_safety_doubt::register_all();
    gen_compose_safetyask::register_all();
    gen_compose_said_about_me::register_all();
    gen_compose_saidby::register_all();
    gen_compose_setting_off::register_all();
    gen_compose_stepwords::register_all();
    gen_compose_still::register_all();
    gen_compose_stretches::register_all();
    gen_compose_talktime::register_all();
    gen_compose_talkto::register_all();
    gen_compose_taste::register_all();
    gen_compose_the_course_of_it::register_all();
    gen_compose_the_rule_for_it::register_all();
    gen_compose_the_shape_without_the_fact::register_all();
    gen_compose_the_trouble_named::register_all();
    gen_compose_the_turn_after::register_all();
    gen_compose_their_day::register_all();
    gen_compose_their_detail::register_all();
    gen_compose_their_life::register_all();
    gen_compose_theirperson::register_all();
    gen_compose_thread_detail::register_all();
    gen_compose_toknow::register_all();
    gen_compose_tone_again::register_all();
    gen_compose_trouble::register_all();
    gen_compose_unseen::register_all();
    gen_compose_urgent::register_all();
    gen_compose_verdict::register_all();
    gen_compose_waited_for::register_all();
    gen_compose_ward::register_all();
    gen_compose_what_decides_it::register_all();
    gen_compose_what_i_do_know::register_all();
    gen_compose_what_it_asks::register_all();
    gen_compose_who::register_all();
    gen_compose_who_to_see::register_all();
    gen_compose_whole::register_all();
    gen_compose_why::register_all();
    gen_compose_worn::register_all();
    gen_knowledge::register_all();
    gen_knowledge_a_plain_noise_word_on_a_machine::register_all();
    gen_knowledge_affording::register_all();
    gen_knowledge_animals::register_all();
    gen_knowledge_appointments::register_all();
    gen_knowledge_babyfood::register_all();
    gen_knowledge_bake_faults::register_all();
    gen_knowledge_bikes::register_all();
    gen_knowledge_body::register_all();
    gen_knowledge_body_energy::register_all();
    gen_knowledge_build_jobs::register_all();
    gen_knowledge_business::register_all();
    gen_knowledge_cakes::register_all();
    gen_knowledge_caring::register_all();
    gen_knowledge_cars::register_all();
    gen_knowledge_child_work::register_all();
    gen_knowledge_chilli::register_all();
    gen_knowledge_cleaning::register_all();
    gen_knowledge_clothing::register_all();
    gen_knowledge_coats::register_all();
    gen_knowledge_comeback::register_all();
    gen_knowledge_complaints::register_all();
    gen_knowledge_conditions::register_all();
    gen_knowledge_consumer::register_all();
    gen_knowledge_containers_frost::register_all();
    gen_knowledge_cook_kit::register_all();
    gen_knowledge_cooking::register_all();
    gen_knowledge_cooking_entries::register_all();
    gen_knowledge_cooking_fats::register_all();
    gen_knowledge_cooking_gaps::register_all();
    gen_knowledge_cooking_more::register_all();
    gen_knowledge_cooking_tables::register_all();
    gen_knowledge_cooling::register_all();
    gen_knowledge_costs::register_all();
    gen_knowledge_creative::register_all();
    gen_knowledge_crisis_replies::register_all();
    gen_knowledge_dates::register_all();
    gen_knowledge_decorating::register_all();
    gen_knowledge_desk_setup::register_all();
    gen_knowledge_devices::register_all();
    gen_knowledge_differences::register_all();
    gen_knowledge_dish_verdicts::register_all();
    gen_knowledge_dishes::register_all();
    gen_knowledge_drinks::register_all();
    gen_knowledge_eithers::register_all();
    gen_knowledge_entertainment::register_all();
    gen_knowledge_everyday_lines::register_all();
    gen_knowledge_explains::register_all();
    gen_knowledge_fabrics::register_all();
    gen_knowledge_family::register_all();
    gen_knowledge_family_more::register_all();
    gen_knowledge_fears::register_all();
    gen_knowledge_fitness::register_all();
    gen_knowledge_fitweek::register_all();
    gen_knowledge_food_left_out::register_all();
    gen_knowledge_freshness::register_all();
    gen_knowledge_friends::register_all();
    gen_knowledge_fruit::register_all();
    gen_knowledge_fun::register_all();
    gen_knowledge_garden::register_all();
    gen_knowledge_garden_tools::register_all();
    gen_knowledge_geography::register_all();
    gen_knowledge_habits::register_all();
    gen_knowledge_health::register_all();
    gen_knowledge_history::register_all();
    gen_knowledge_home::register_all();
    gen_knowledge_home_craft::register_all();
    gen_knowledge_house_odds::register_all();
    gen_knowledge_how_it_works::register_all();
    gen_knowledge_icing::register_all();
    gen_knowledge_idioms::register_all();
    gen_knowledge_income_periods::register_all();
    gen_knowledge_ingredient_swaps::register_all();
    gen_knowledge_job_times::register_all();
    gen_knowledge_joint_time::register_all();
    gen_knowledge_joints::register_all();
    gen_knowledge_keeping::register_all();
    gen_knowledge_keeping_cooked::register_all();
    gen_knowledge_kept_animals::register_all();
    gen_knowledge_kit_lists::register_all();
    gen_knowledge_kitchen::register_all();
    gen_knowledge_kitchen_odds::register_all();
    gen_knowledge_kitchen_prep::register_all();
    gen_knowledge_kits_and_claims::register_all();
    gen_knowledge_land::register_all();
    gen_knowledge_language::register_all();
    gen_knowledge_left_on::register_all();
    gen_knowledge_lifeadmin::register_all();
    gen_knowledge_lookup::register_all();
    gen_knowledge_machine_door::register_all();
    gen_knowledge_materials::register_all();
    gen_knowledge_maths::register_all();
    gen_knowledge_meals::register_all();
    gen_knowledge_measures::register_all();
    gen_knowledge_meat_in_a_card::register_all();
    gen_knowledge_microwave_methods::register_all();
    gen_knowledge_money::register_all();
    gen_knowledge_money_words::register_all();
    gen_knowledge_music::register_all();
    gen_knowledge_nature::register_all();
    gen_knowledge_newborn::register_all();
    gen_knowledge_no_kit::register_all();
    gen_knowledge_notable::register_all();
    gen_knowledge_nursing::register_all();
    gen_knowledge_nutrition::register_all();
    gen_knowledge_occasions::register_all();
    gen_knowledge_odours::register_all();
    gen_knowledge_opinions::register_all();
    gen_knowledge_painting_faults::register_all();
    gen_knowledge_panic_talk::register_all();
    gen_knowledge_parking::register_all();
    gen_knowledge_pasta::register_all();
    gen_knowledge_pay::register_all();
    gen_knowledge_pensions::register_all();
    gen_knowledge_pets::register_all();
    gen_knowledge_pets_health::register_all();
    gen_knowledge_pharmacy::register_all();
    gen_knowledge_phone_use::register_all();
    gen_knowledge_photography::register_all();
    gen_knowledge_picking::register_all();
    gen_knowledge_picks::register_all();
    gen_knowledge_plans::register_all();
    gen_knowledge_power_heat::register_all();
    gen_knowledge_pregnancy::register_all();
    gen_knowledge_prices::register_all();
    gen_knowledge_procedures::register_all();
    gen_knowledge_quantity::register_all();
    gen_knowledge_rice::register_all();
    gen_knowledge_routines::register_all();
    gen_knowledge_rubbish::register_all();
    gen_knowledge_sauces::register_all();
    gen_knowledge_school_day::register_all();
    gen_knowledge_school_days::register_all();
    gen_knowledge_schoolmaths::register_all();
    gen_knowledge_science::register_all();
    gen_knowledge_seasoning::register_all();
    gen_knowledge_shopping::register_all();
    gen_knowledge_sleep::register_all();
    gen_knowledge_sleep_aids::register_all();
    gen_knowledge_spice_heat::register_all();
    gen_knowledge_sport::register_all();
    gen_knowledge_still_ok::register_all();
    gen_knowledge_study::register_all();
    gen_knowledge_swaps::register_all();
    gen_knowledge_takes::register_all();
    gen_knowledge_technology::register_all();
    gen_knowledge_tenancy::register_all();
    gen_knowledge_trades::register_all();
    gen_knowledge_travel::register_all();
    gen_knowledge_travel_health::register_all();
    gen_knowledge_units::register_all();
    gen_knowledge_vaccines::register_all();
    gen_knowledge_vegetables::register_all();
    gen_knowledge_warm_food::register_all();
    gen_knowledge_water_out::register_all();
    gen_knowledge_what_damages_what::register_all();
    gen_knowledge_whenbuy::register_all();
    gen_knowledge_wording::register_all();
    gen_knowledge_work::register_all();
    gen_memory::register_all();
    gen_memory_a_year_of_birth::register_all();
    gen_memory_aboutthem::register_all();
    gen_memory_ack::register_all();
    gen_memory_again::register_all();
    gen_memory_already::register_all();
    gen_memory_amounts::register_all();
    gen_memory_another::register_all();
    gen_memory_asked_about::register_all();
    gen_memory_asleep::register_all();
    gen_memory_back_again::register_all();
    gen_memory_baking::register_all();
    gen_memory_callback::register_all();
    gen_memory_carried_on::register_all();
    gen_memory_carry::register_all();
    gen_memory_cause::register_all();
    gen_memory_change::register_all();
    gen_memory_choice::register_all();
    gen_memory_company::register_all();
    gen_memory_condition::register_all();
    gen_memory_corrected::register_all();
    gen_memory_counts::register_all();
    gen_memory_deadline::register_all();
    gen_memory_decided::register_all();
    gen_memory_detail::register_all();
    gen_memory_doing_it::register_all();
    gen_memory_doubt::register_all();
    gen_memory_dream::register_all();
    gen_memory_earlier::register_all();
    gen_memory_enough::register_all();
    gen_memory_facts::register_all();
    gen_memory_favourite::register_all();
    gen_memory_filler::register_all();
    gen_memory_goals::register_all();
    gen_memory_groupages::register_all();
    gen_memory_guest::register_all();
    gen_memory_healing::register_all();
    gen_memory_history::register_all();
    gen_memory_holds::register_all();
    gen_memory_hope::register_all();
    gen_memory_hours::register_all();
    gen_memory_household::register_all();
    gen_memory_kidcount::register_all();
    gen_memory_kinds::register_all();
    gen_memory_limit::register_all();
    gen_memory_limits::register_all();
    gen_memory_lineback::register_all();
    gen_memory_matters::register_all();
    gen_memory_meant::register_all();
    gen_memory_measure::register_all();
    gen_memory_more::register_all();
    gen_memory_named::register_all();
    gen_memory_narrow::register_all();
    gen_memory_newborn::register_all();
    gen_memory_not_a_trip::register_all();
    gen_memory_not_this::register_all();
    gen_memory_obstacle::register_all();
    gen_memory_onset::register_all();
    gen_memory_opening::register_all();
    gen_memory_outfit::register_all();
    gen_memory_outgoings::register_all();
    gen_memory_paragraph::register_all();
    gen_memory_passing_the_time::register_all();
    gen_memory_pending::register_all();
    gen_memory_people::register_all();
    gen_memory_pile::register_all();
    gen_memory_places::register_all();
    gen_memory_plan::register_all();
    gen_memory_plans::register_all();
    gen_memory_portions::register_all();
    gen_memory_prefer::register_all();
    gen_memory_price::register_all();
    gen_memory_purse::register_all();
    gen_memory_quotes::register_all();
    gen_memory_rates::register_all();
    gen_memory_recall::register_all();
    gen_memory_reject::register_all();
    gen_memory_relations::register_all();
    gen_memory_remedy::register_all();
    gen_memory_reminded::register_all();
    gen_memory_rename::register_all();
    gen_memory_repair::register_all();
    gen_memory_repeat::register_all();
    gen_memory_resize::register_all();
    gen_memory_returned::register_all();
    gen_memory_returns::register_all();
    gen_memory_saving::register_all();
    gen_memory_setback::register_all();
    gen_memory_size::register_all();
    gen_memory_spent::register_all();
    gen_memory_spoken::register_all();
    gen_memory_stands::register_all();
    gen_memory_store_lines::register_all();
    gen_memory_store_replies::register_all();
    gen_memory_stuck::register_all();
    gen_memory_surname::register_all();
    gen_memory_swap::register_all();
    gen_memory_taking::register_all();
    gen_memory_taste::register_all();
    gen_memory_the_verdict::register_all();
    gen_memory_the_years::register_all();
    gen_memory_their_feeling::register_all();
    gen_memory_thread::register_all();
    gen_memory_times::register_all();
    gen_memory_told::register_all();
    gen_memory_too_late::register_all();
    gen_memory_topics::register_all();
    gen_memory_tried::register_all();
    gen_memory_trips::register_all();
    gen_memory_warmth::register_all();
    gen_memory_whathurts::register_all();
    gen_memory_when::register_all();
    gen_memory_whereabout::register_all();
    gen_memory_whose_body::register_all();
    gen_memory_will_it_work::register_all();
    gen_memory_workplace::register_all();
    gen_memory_years_in::register_all();
    gen_modules::register_all();
    gen_modules_a_cake_between_its_layers::register_all();
    gen_modules_a_check_with_you::register_all();
    gen_modules_a_count_of_packets::register_all();
    gen_modules_a_count_said::register_all();
    gen_modules_a_day_somebody_else_keeps::register_all();
    gen_modules_a_debt_said_flat::register_all();
    gen_modules_a_fact_asked_again::register_all();
    gen_modules_a_figure_beside_the_state::register_all();
    gen_modules_a_figure_of_a_dish::register_all();
    gen_modules_a_first_time::register_all();
    gen_modules_a_habit_is_raised::register_all();
    gen_modules_a_job_that_is_not_a_diy_job::register_all();
    gen_modules_a_letter_about_money::register_all();
    gen_modules_a_limit_over_the_answer::register_all();
    gen_modules_a_measure_asked_again::register_all();
    gen_modules_a_number_the_bot_knows::register_all();
    gen_modules_a_plan_the_thread_holds::register_all();
    gen_modules_a_question_about_somebody::register_all();
    gen_modules_a_return_brings_back_the_advice::register_all();
    gen_modules_a_shock_at_a_bill::register_all();
    gen_modules_a_thing_being_weighed::register_all();
    gen_modules_a_thing_wants_doing::register_all();
    gen_modules_a_will_it_work::register_all();
    gen_modules_aboutachild::register_all();
    gen_modules_after_a_lead_in::register_all();
    gen_modules_already::register_all();
    gen_modules_an_age_ahead::register_all();
    gen_modules_answers::register_all();
    gen_modules_arbiter::register_all();
    gen_modules_attribute_said_again::register_all();
    gen_modules_bulky::register_all();
    gen_modules_callback_about::register_all();
    gen_modules_carers_turn::register_all();
    gen_modules_chasing_a_body::register_all();
    gen_modules_chat::register_all();
    gen_modules_chat_replies::register_all();
    gen_modules_colour_on_the_body::register_all();
    gen_modules_confusing::register_all();
    gen_modules_detail_question::register_all();
    gen_modules_done_ask::register_all();
    gen_modules_doubt::register_all();
    gen_modules_dreams::register_all();
    gen_modules_familyask::register_all();
    gen_modules_favourites::register_all();
    gen_modules_figure_asked_again::register_all();
    gen_modules_first_line_open::register_all();
    gen_modules_greeting::register_all();
    gen_modules_grief::register_all();
    gen_modules_group_ages::register_all();
    gen_modules_hardpast::register_all();
    gen_modules_household::register_all();
    gen_modules_how_long_a_stay::register_all();
    gen_modules_intent::register_all();
    gen_modules_invitation::register_all();
    gen_modules_judge_ask::register_all();
    gen_modules_keeping::register_all();
    gen_modules_kid_tally::register_all();
    gen_modules_live_source::register_all();
    gen_modules_live_times::register_all();
    gen_modules_loss::register_all();
    gen_modules_memory_store::register_all();
    gen_modules_memory_tables::register_all();
    gen_modules_my_work::register_all();
    gen_modules_name_taken_back::register_all();
    gen_modules_near_me::register_all();
    gen_modules_night::register_all();
    gen_modules_no_advice::register_all();
    gen_modules_nudge::register_all();
    gen_modules_on_record::register_all();
    gen_modules_onsubject::register_all();
    gen_modules_pace::register_all();
    gen_modules_particles::register_all();
    gen_modules_recap::register_all();
    gen_modules_receiving_end::register_all();
    gen_modules_record_due::register_all();
    gen_modules_ruled_out::register_all();
    gen_modules_safety::register_all();
    gen_modules_safety_cards::register_all();
    gen_modules_safety_phrases::register_all();
    gen_modules_safety_tables::register_all();
    gen_modules_said_back::register_all();
    gen_modules_said_fact_back::register_all();
    gen_modules_saving_followup::register_all();
    gen_modules_since_it_began::register_all();
    gen_modules_steps::register_all();
    gen_modules_taking_asked_again::register_all();
    gen_modules_talkto::register_all();
    gen_modules_the_detail_the_thread_adds::register_all();
    gen_modules_the_kind_one_line_up::register_all();
    gen_modules_the_month_they_have::register_all();
    gen_modules_the_shape_of_the_answer::register_all();
    gen_modules_the_trouble_it_has::register_all();
    gen_modules_the_trouble_named::register_all();
    gen_modules_the_wait_about_a_body::register_all();
    gen_modules_their_life::register_all();
    gen_modules_their_pastime::register_all();
    gen_modules_their_taste::register_all();
    gen_modules_theirperson::register_all();
    gen_modules_thing_i_did_with::register_all();
    gen_modules_trip_date_said_again::register_all();
    gen_modules_understand::register_all();
    gen_modules_urgent::register_all();
    gen_modules_ward::register_all();
    gen_modules_what_decides_it::register_all();
    gen_modules_what_they_said::register_all();
    gen_modules_where_it_answers::register_all();
    gen_modules_who_to_see::register_all();
    gen_modules_words::register_all();
    gen_modules_years_in::register_all();
    gen_runtime::register_all();
    gen_understanding::register_all();
    gen_understanding_attempts::register_all();
    gen_understanding_body::register_all();
    gen_understanding_breeds::register_all();
    gen_understanding_car_glass::register_all();
    gen_understanding_cars::register_all();
    gen_understanding_confusion::register_all();
    gen_understanding_kinds::register_all();
    gen_understanding_lexicon::register_all();
    gen_understanding_normalize::register_all();
    gen_understanding_references::register_all();
    gen_understanding_senses::register_all();
    gen_understanding_topics::register_all();
}

pub fn registry() -> Vec<ModuleEntry> {
    vec![
        ModuleEntry { name: "a_cake_between_its_layers", priority: || match gen_modules_a_cake_between_its_layers::modules::a_cake_between_its_layers::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_cake_between_its_layers::modules::a_cake_between_its_layers::respond), after: None },
        ModuleEntry { name: "a_check_with_you", priority: || match gen_modules_a_check_with_you::modules::a_check_with_you::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_check_with_you::modules::a_check_with_you::respond), after: None },
        ModuleEntry { name: "a_count_of_packets", priority: || match gen_modules_a_count_of_packets::modules::a_count_of_packets::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_count_of_packets::modules::a_count_of_packets::respond), after: None },
        ModuleEntry { name: "a_count_said", priority: || match gen_modules_a_count_said::modules::a_count_said::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_a_count_said::modules::a_count_said::observe), respond: Some(gen_modules_a_count_said::modules::a_count_said::respond), after: None },
        ModuleEntry { name: "a_day_somebody_else_keeps", priority: || match gen_modules_a_day_somebody_else_keeps::modules::a_day_somebody_else_keeps::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_day_somebody_else_keeps::modules::a_day_somebody_else_keeps::respond), after: None },
        ModuleEntry { name: "a_debt_said_flat", priority: || match gen_modules_a_debt_said_flat::modules::a_debt_said_flat::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_a_debt_said_flat::modules::a_debt_said_flat::observe), respond: Some(gen_modules_a_debt_said_flat::modules::a_debt_said_flat::respond), after: None },
        ModuleEntry { name: "a_fact_asked_again", priority: || match gen_modules_a_fact_asked_again::modules::a_fact_asked_again::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_fact_asked_again::modules::a_fact_asked_again::respond), after: None },
        ModuleEntry { name: "a_figure_beside_the_state", priority: || match gen_modules_a_figure_beside_the_state::modules::a_figure_beside_the_state::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_figure_beside_the_state::modules::a_figure_beside_the_state::respond), after: None },
        ModuleEntry { name: "a_figure_of_a_dish", priority: || match gen_modules_a_figure_of_a_dish::modules::a_figure_of_a_dish::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_figure_of_a_dish::modules::a_figure_of_a_dish::respond), after: None },
        ModuleEntry { name: "a_first_time", priority: || match gen_modules_a_first_time::modules::a_first_time::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_first_time::modules::a_first_time::respond), after: None },
        ModuleEntry { name: "a_habit_is_raised", priority: || match gen_modules_a_habit_is_raised::modules::a_habit_is_raised::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_habit_is_raised::modules::a_habit_is_raised::respond), after: None },
        ModuleEntry { name: "a_job_that_is_not_a_diy_job", priority: || match gen_modules_a_job_that_is_not_a_diy_job::modules::a_job_that_is_not_a_diy_job::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_job_that_is_not_a_diy_job::modules::a_job_that_is_not_a_diy_job::respond), after: None },
        ModuleEntry { name: "a_letter_about_money", priority: || match gen_modules_a_letter_about_money::modules::a_letter_about_money::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_letter_about_money::modules::a_letter_about_money::respond), after: None },
        ModuleEntry { name: "a_limit_over_the_answer", priority: || match gen_modules_a_limit_over_the_answer::modules::a_limit_over_the_answer::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_a_limit_over_the_answer::modules::a_limit_over_the_answer::observe), respond: Some(gen_modules_a_limit_over_the_answer::modules::a_limit_over_the_answer::respond), after: None },
        ModuleEntry { name: "a_measure_asked_again", priority: || match gen_modules_a_measure_asked_again::modules::a_measure_asked_again::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_measure_asked_again::modules::a_measure_asked_again::respond), after: None },
        ModuleEntry { name: "a_number_the_bot_knows", priority: || match gen_modules_a_number_the_bot_knows::modules::a_number_the_bot_knows::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_number_the_bot_knows::modules::a_number_the_bot_knows::respond), after: None },
        ModuleEntry { name: "a_plan_the_thread_holds", priority: || match gen_modules_a_plan_the_thread_holds::modules::a_plan_the_thread_holds::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_plan_the_thread_holds::modules::a_plan_the_thread_holds::respond), after: None },
        ModuleEntry { name: "a_question_about_somebody", priority: || match gen_modules_a_question_about_somebody::modules::a_question_about_somebody::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_question_about_somebody::modules::a_question_about_somebody::respond), after: None },
        ModuleEntry { name: "a_return_brings_back_the_advice", priority: || match gen_modules_a_return_brings_back_the_advice::modules::a_return_brings_back_the_advice::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_return_brings_back_the_advice::modules::a_return_brings_back_the_advice::respond), after: None },
        ModuleEntry { name: "a_shock_at_a_bill", priority: || match gen_modules_a_shock_at_a_bill::modules::a_shock_at_a_bill::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_shock_at_a_bill::modules::a_shock_at_a_bill::respond), after: None },
        ModuleEntry { name: "a_thing_being_weighed", priority: || match gen_modules_a_thing_being_weighed::modules::a_thing_being_weighed::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_thing_being_weighed::modules::a_thing_being_weighed::respond), after: None },
        ModuleEntry { name: "a_thing_wants_doing", priority: || match gen_modules_a_thing_wants_doing::modules::a_thing_wants_doing::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_thing_wants_doing::modules::a_thing_wants_doing::respond), after: None },
        ModuleEntry { name: "a_will_it_work", priority: || match gen_modules_a_will_it_work::modules::a_will_it_work::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_a_will_it_work::modules::a_will_it_work::respond), after: None },
        ModuleEntry { name: "aboutachild", priority: || match gen_modules_aboutachild::modules::aboutachild::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_aboutachild::modules::aboutachild::respond), after: None },
        ModuleEntry { name: "after_a_lead_in", priority: || match gen_modules_after_a_lead_in::modules::after_a_lead_in::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_after_a_lead_in::modules::after_a_lead_in::respond), after: None },
        ModuleEntry { name: "already", priority: || match gen_modules_already::modules::already::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_already::modules::already::respond), after: None },
        ModuleEntry { name: "an_age_ahead", priority: || match gen_modules_an_age_ahead::modules::an_age_ahead::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_an_age_ahead::modules::an_age_ahead::respond), after: None },
        ModuleEntry { name: "answers", priority: || match gen_modules_answers::modules::answers::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_answers::modules::answers::observe), respond: Some(gen_modules_answers::modules::answers::respond), after: None },
        ModuleEntry { name: "arbiter", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "attribute_said_again", priority: || match gen_modules_attribute_said_again::modules::attribute_said_again::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_attribute_said_again::modules::attribute_said_again::respond), after: None },
        ModuleEntry { name: "bulky", priority: || match gen_modules_bulky::modules::bulky::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_bulky::modules::bulky::respond), after: None },
        ModuleEntry { name: "callback_about", priority: || match gen_modules_callback_about::modules::callback_about::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_callback_about::modules::callback_about::respond), after: None },
        ModuleEntry { name: "carers_turn", priority: || match gen_modules_carers_turn::modules::carers_turn::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_carers_turn::modules::carers_turn::respond), after: None },
        ModuleEntry { name: "chasing_a_body", priority: || match gen_modules_chasing_a_body::modules::chasing_a_body::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_chasing_a_body::modules::chasing_a_body::respond), after: None },
        ModuleEntry { name: "chat", priority: || match gen_modules_chat::modules::chat::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_chat::modules::chat::observe), respond: Some(gen_modules_chat::modules::chat::respond), after: Some(gen_modules_chat::modules::chat::after) },
        ModuleEntry { name: "chat_replies", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "colour_on_the_body", priority: || match gen_modules_colour_on_the_body::modules::colour_on_the_body::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_colour_on_the_body::modules::colour_on_the_body::respond), after: None },
        ModuleEntry { name: "confusing", priority: || match gen_modules_confusing::modules::confusing::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_confusing::modules::confusing::respond), after: None },
        ModuleEntry { name: "detail_question", priority: || match gen_modules_detail_question::modules::detail_question::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_detail_question::modules::detail_question::respond), after: None },
        ModuleEntry { name: "done_ask", priority: || match gen_modules_done_ask::modules::done_ask::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_done_ask::modules::done_ask::observe), respond: None, after: None },
        ModuleEntry { name: "doubt", priority: || match gen_modules_doubt::modules::doubt::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_doubt::modules::doubt::respond), after: None },
        ModuleEntry { name: "dreams", priority: || match gen_modules_dreams::modules::dreams::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_dreams::modules::dreams::respond), after: None },
        ModuleEntry { name: "familyask", priority: || match gen_modules_familyask::modules::familyask::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_familyask::modules::familyask::respond), after: None },
        ModuleEntry { name: "favourites", priority: || match gen_modules_favourites::modules::favourites::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_favourites::modules::favourites::observe), respond: Some(gen_modules_favourites::modules::favourites::respond), after: None },
        ModuleEntry { name: "figure_asked_again", priority: || match gen_modules_figure_asked_again::modules::figure_asked_again::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_figure_asked_again::modules::figure_asked_again::respond), after: None },
        ModuleEntry { name: "first_line_open", priority: || match gen_modules_first_line_open::modules::first_line_open::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_first_line_open::modules::first_line_open::respond), after: None },
        ModuleEntry { name: "greeting", priority: || match gen_modules_greeting::modules::greeting::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_greeting::modules::greeting::respond), after: None },
        ModuleEntry { name: "grief", priority: || match gen_modules_grief::modules::grief::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_grief::modules::grief::observe), respond: Some(gen_modules_grief::modules::grief::respond), after: None },
        ModuleEntry { name: "group_ages", priority: || match gen_modules_group_ages::modules::group_ages::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_group_ages::modules::group_ages::observe), respond: Some(gen_modules_group_ages::modules::group_ages::respond), after: None },
        ModuleEntry { name: "hardpast", priority: || match gen_modules_hardpast::modules::hardpast::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_hardpast::modules::hardpast::respond), after: None },
        ModuleEntry { name: "household", priority: || match gen_modules_household::modules::household::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_household::modules::household::observe), respond: Some(gen_modules_household::modules::household::respond), after: None },
        ModuleEntry { name: "how_long_a_stay", priority: || match gen_modules_how_long_a_stay::modules::how_long_a_stay::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_how_long_a_stay::modules::how_long_a_stay::respond), after: None },
        ModuleEntry { name: "intent", priority: || match gen_modules_intent::modules::intent::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_intent::modules::intent::observe), respond: None, after: None },
        ModuleEntry { name: "invitation", priority: || match gen_modules_invitation::modules::invitation::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_invitation::modules::invitation::respond), after: None },
        ModuleEntry { name: "judge_ask", priority: || match gen_modules_judge_ask::modules::judge_ask::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_judge_ask::modules::judge_ask::respond), after: None },
        ModuleEntry { name: "keeping", priority: || match gen_modules_keeping::modules::keeping::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_keeping::modules::keeping::respond), after: None },
        ModuleEntry { name: "kid_tally", priority: || match gen_modules_kid_tally::modules::kid_tally::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_kid_tally::modules::kid_tally::respond), after: None },
        ModuleEntry { name: "live_source", priority: || match gen_modules_live_source::modules::live_source::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_live_source::modules::live_source::respond), after: None },
        ModuleEntry { name: "live_times", priority: || match gen_modules_live_times::modules::live_times::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_live_times::modules::live_times::respond), after: None },
        ModuleEntry { name: "loss", priority: || match gen_modules_loss::modules::loss::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_loss::modules::loss::respond), after: None },
        ModuleEntry { name: "memory_store", priority: || 0, observe: Some(gen_modules_memory_store::modules::memory_store::observe), respond: Some(gen_modules_memory_store::modules::memory_store::respond), after: Some(gen_modules_memory_store::modules::memory_store::after) },
        ModuleEntry { name: "memory_tables", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "my_work", priority: || match gen_modules_my_work::modules::my_work::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_my_work::modules::my_work::respond), after: None },
        ModuleEntry { name: "name_taken_back", priority: || match gen_modules_name_taken_back::modules::name_taken_back::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_name_taken_back::modules::name_taken_back::observe), respond: Some(gen_modules_name_taken_back::modules::name_taken_back::respond), after: None },
        ModuleEntry { name: "near_me", priority: || match gen_modules_near_me::modules::near_me::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_near_me::modules::near_me::respond), after: None },
        ModuleEntry { name: "night", priority: || match gen_modules_night::modules::night::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_night::modules::night::respond), after: None },
        ModuleEntry { name: "no_advice", priority: || match gen_modules_no_advice::modules::no_advice::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_no_advice::modules::no_advice::respond), after: None },
        ModuleEntry { name: "nudge", priority: || match gen_modules_nudge::modules::nudge::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_nudge::modules::nudge::respond), after: None },
        ModuleEntry { name: "on_record", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "onsubject", priority: || match gen_modules_onsubject::modules::onsubject::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_onsubject::modules::onsubject::respond), after: None },
        ModuleEntry { name: "pace", priority: || match gen_modules_pace::modules::pace::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_pace::modules::pace::respond), after: None },
        ModuleEntry { name: "particles", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "recap", priority: || match gen_modules_recap::modules::recap::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_recap::modules::recap::respond), after: None },
        ModuleEntry { name: "receiving_end", priority: || match gen_modules_receiving_end::modules::receiving_end::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_receiving_end::modules::receiving_end::respond), after: None },
        ModuleEntry { name: "record_due", priority: || match gen_modules_record_due::modules::record_due::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_record_due::modules::record_due::respond), after: None },
        ModuleEntry { name: "ruled_out", priority: || match gen_modules_ruled_out::modules::ruled_out::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_ruled_out::modules::ruled_out::respond), after: None },
        ModuleEntry { name: "safety", priority: || match gen_modules_safety::modules::safety::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_safety::modules::safety::respond), after: None },
        ModuleEntry { name: "safety_cards", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "safety_phrases", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "safety_tables", priority: || 0, observe: None, respond: None, after: None },
        ModuleEntry { name: "said_back", priority: || match gen_modules_said_back::modules::said_back::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_said_back::modules::said_back::respond), after: None },
        ModuleEntry { name: "said_fact_back", priority: || match gen_modules_said_fact_back::modules::said_fact_back::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_said_fact_back::modules::said_fact_back::observe), respond: Some(gen_modules_said_fact_back::modules::said_fact_back::respond), after: None },
        ModuleEntry { name: "saving_followup", priority: || match gen_modules_saving_followup::modules::saving_followup::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_saving_followup::modules::saving_followup::respond), after: None },
        ModuleEntry { name: "since_it_began", priority: || match gen_modules_since_it_began::modules::since_it_began::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_since_it_began::modules::since_it_began::respond), after: None },
        ModuleEntry { name: "steps", priority: || match gen_modules_steps::modules::steps::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_steps::modules::steps::respond), after: None },
        ModuleEntry { name: "taking_asked_again", priority: || match gen_modules_taking_asked_again::modules::taking_asked_again::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_taking_asked_again::modules::taking_asked_again::respond), after: None },
        ModuleEntry { name: "talkto", priority: || match gen_modules_talkto::modules::talkto::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_talkto::modules::talkto::respond), after: None },
        ModuleEntry { name: "the_detail_the_thread_adds", priority: || match gen_modules_the_detail_the_thread_adds::modules::the_detail_the_thread_adds::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_detail_the_thread_adds::modules::the_detail_the_thread_adds::respond), after: None },
        ModuleEntry { name: "the_kind_one_line_up", priority: || match gen_modules_the_kind_one_line_up::modules::the_kind_one_line_up::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_kind_one_line_up::modules::the_kind_one_line_up::respond), after: None },
        ModuleEntry { name: "the_month_they_have", priority: || match gen_modules_the_month_they_have::modules::the_month_they_have::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_month_they_have::modules::the_month_they_have::respond), after: None },
        ModuleEntry { name: "the_shape_of_the_answer", priority: || match gen_modules_the_shape_of_the_answer::modules::the_shape_of_the_answer::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_shape_of_the_answer::modules::the_shape_of_the_answer::respond), after: None },
        ModuleEntry { name: "the_trouble_it_has", priority: || match gen_modules_the_trouble_it_has::modules::the_trouble_it_has::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_trouble_it_has::modules::the_trouble_it_has::respond), after: None },
        ModuleEntry { name: "the_trouble_named", priority: || match gen_modules_the_trouble_named::modules::the_trouble_named::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_trouble_named::modules::the_trouble_named::respond), after: None },
        ModuleEntry { name: "the_wait_about_a_body", priority: || match gen_modules_the_wait_about_a_body::modules::the_wait_about_a_body::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_the_wait_about_a_body::modules::the_wait_about_a_body::respond), after: None },
        ModuleEntry { name: "their_life", priority: || match gen_modules_their_life::modules::their_life::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_their_life::modules::their_life::respond), after: None },
        ModuleEntry { name: "their_pastime", priority: || match gen_modules_their_pastime::modules::their_pastime::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_their_pastime::modules::their_pastime::respond), after: None },
        ModuleEntry { name: "their_taste", priority: || match gen_modules_their_taste::modules::their_taste::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_their_taste::modules::their_taste::respond), after: None },
        ModuleEntry { name: "theirperson", priority: || match gen_modules_theirperson::modules::theirperson::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_theirperson::modules::theirperson::respond), after: None },
        ModuleEntry { name: "thing_i_did_with", priority: || match gen_modules_thing_i_did_with::modules::thing_i_did_with::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_thing_i_did_with::modules::thing_i_did_with::respond), after: None },
        ModuleEntry { name: "trip_date_said_again", priority: || match gen_modules_trip_date_said_again::modules::trip_date_said_again::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_trip_date_said_again::modules::trip_date_said_again::observe), respond: None, after: None },
        ModuleEntry { name: "understand", priority: || match gen_modules_understand::modules::understand::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: Some(gen_modules_understand::modules::understand::observe), respond: None, after: None },
        ModuleEntry { name: "urgent", priority: || match gen_modules_urgent::modules::urgent::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_urgent::modules::urgent::respond), after: None },
        ModuleEntry { name: "ward", priority: || match gen_modules_ward::modules::ward::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_ward::modules::ward::respond), after: None },
        ModuleEntry { name: "what_decides_it", priority: || match gen_modules_what_decides_it::modules::what_decides_it::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_what_decides_it::modules::what_decides_it::respond), after: None },
        ModuleEntry { name: "what_they_said", priority: || match gen_modules_what_they_said::modules::what_they_said::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_what_they_said::modules::what_they_said::respond), after: None },
        ModuleEntry { name: "where_it_answers", priority: || match gen_modules_where_it_answers::modules::where_it_answers::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_where_it_answers::modules::where_it_answers::respond), after: None },
        ModuleEntry { name: "who_to_see", priority: || match gen_modules_who_to_see::modules::who_to_see::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_who_to_see::modules::who_to_see::respond), after: None },
        ModuleEntry { name: "words", priority: || match gen_modules_words::modules::words::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_words::modules::words::respond), after: None },
        ModuleEntry { name: "years_in", priority: || match gen_modules_years_in::modules::years_in::g_PRIORITY() { Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }, observe: None, respond: Some(gen_modules_years_in::modules::years_in::respond), after: None },
    ]
}

pub fn arbiter_choose() -> Option<fn(&[Py]) -> Py> { Some(gen_modules_arbiter::modules::arbiter::choose) }
