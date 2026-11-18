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


thread_local! { static G_ALONE_AFTER_A_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALONE_AFTER_A_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALONE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_COMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_HAD_ONCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_TAKEN_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_ONES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_ONE_FOR_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_SOME_FOR_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANYBODY_AT_ALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AGE_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRIVE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AS_WELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BIRTHDAY_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BREED_BEING_CHOSEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COMING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONDITION_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONTINUATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CORRECTION_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_NOT_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_ON_A_HARD_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_ON_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_STILL_TO_COME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_ON_A_HARD_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_ON_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEGREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOING_TURNED_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEELING_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GROUP_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GROUP_ALONE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAD_AN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KNOWN_AGE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_ECHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_OF_ONE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKE_WITH_TWO_HALVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MARKED_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MILESTONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONTH_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_WAS_VIOLENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_ALONE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_LIKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PET_WANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POINTED_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POSITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POSITION_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POSITION_MAYBE_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REASON_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REASON_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RESULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SECOND_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SPAN_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAY_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAY_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRIP_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TWO_ALONE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VAGUE_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WANTED_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WANTED_ANIMAL_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WANT_TURNED_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEEKDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WONDERED_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WONDER_SAID_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORRIED_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_AT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_AT_IT_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_AT_IT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BETWEEN_VALUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_STATE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_STATE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_STATE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUDGET_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_OFFER_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_AFTER_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_FLAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_FLAT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_MORNING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMING_TO_STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_INITIALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRAST_SAID_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED_WITH_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTED_LETTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTED_STARS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_BEEN_AND_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_FAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_JUST_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEATH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEGREE_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESTINATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESTINATION_ARRIVAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIARY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIARY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIARY_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIARY_TIME_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIARY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIGIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DREADED_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRINK_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DURATION_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_RISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENDS_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENDS_THE_THING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPLAIN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAR_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAGMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVEN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVEN_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVEN_AWAY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_HOME_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_HOME_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_MOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROWING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HABIT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_A: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_GOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAVY_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAVY_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_LONG_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURTS_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTRODUCED_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITS_OWNS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_HAS_A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_FINISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_CLOSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OFF_CONTRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LETTER_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIKES_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT_AFTER_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_SHIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAKE_A_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEETING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEETING_TIME_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEET_GREET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NIGHT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_AROUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_AROUND_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DAY_TO_LOOK_FORWARD_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SECOND_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_TO_HAVE_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ONE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_REFERENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_THEN_A_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_THEN_A_LINE_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBLIGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION_FLAT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFERS_A_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OLD_ONE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_STAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_NIGHTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_MEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_MEND_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_MEND_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_LIMIT_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_JOB_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_ADDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_ALL_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_TAIL_COMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_TAIL_COMING_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_TAIL_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_TAIL_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACED_IN_THE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACES_WITH_THE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTER_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_AT_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_POINTS_AT_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAISED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_ALREADY_PAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_EXPENSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_FIGURE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_JUDGES_THE_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_PRICE_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_STILL_TO_COME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_TWO_SIDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMARK_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPLACING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTS_AN_ACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_BACK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_INTRO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEPARATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHIFTS_ALL_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHIFT_EARLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHIFT_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SINCE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_COMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE_POINTS_AT_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_MILESTONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_MILESTONE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOON_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORRY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPEAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPENDING_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPREAD_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAR_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LIMIT_GOES_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NOUN_IS_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_OLD_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_STORY_GOES_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS_OF_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_CLOSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VALUE_UNIT_GLUED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VALUE_UNIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VALUE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERBISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_TAIL_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_TAIL_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_DAYS_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ROOMY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_TODAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_TODAY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_TOMORROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_TOMORROW_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHICH_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHICH_ONES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_FREQUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_IS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_THE_FIGURE_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WISH_FOR_AN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_AN_ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_SLASH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_FROM_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WOULD_HAVE_BEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANSWER_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ABILITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_MIDWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PLAN_WAS_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BULLIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIMIT_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BODY_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DETAIL_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_WITH_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_WITH_A_LINK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REACTION_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_YES_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPACED_DASH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_BOT_AS_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_rest_of_a_question: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALONE_AFTER_A_LOSS() -> Py { ensure_init(); G_ALONE_AFTER_A_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALONE_AFTER_A_LOSS(v: Py) { ensure_init(); G_ALONE_AFTER_A_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_ALONE_AFTER_A_MOVE() -> Py { ensure_init(); G_ALONE_AFTER_A_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALONE_AFTER_A_MOVE(v: Py) { ensure_init(); G_ALONE_AFTER_A_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_ALONE_TAIL() -> Py { ensure_init(); G_ALONE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALONE_TAIL(v: Py) { ensure_init(); G_ALONE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_COMING() -> Py { ensure_init(); G_ANIMAL_COMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_COMING(v: Py) { ensure_init(); G_ANIMAL_COMING.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_HAD_ONCE() -> Py { ensure_init(); G_ANIMAL_HAD_ONCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_HAD_ONCE(v: Py) { ensure_init(); G_ANIMAL_HAD_ONCE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_TAKEN_BACK() -> Py { ensure_init(); G_ANIMAL_TAKEN_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_TAKEN_BACK(v: Py) { ensure_init(); G_ANIMAL_TAKEN_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_ONE() -> Py { ensure_init(); G_ANOTHER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ONE(v: Py) { ensure_init(); G_ANOTHER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_ONES() -> Py { ensure_init(); G_ANOTHER_ONES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ONES(v: Py) { ensure_init(); G_ANOTHER_ONES.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_ONE_FOR_SOMEONE() -> Py { ensure_init(); G_ANOTHER_ONE_FOR_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ONE_FOR_SOMEONE(v: Py) { ensure_init(); G_ANOTHER_ONE_FOR_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_SOME_FOR_SOMEONE() -> Py { ensure_init(); G_ANOTHER_SOME_FOR_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_SOME_FOR_SOMEONE(v: Py) { ensure_init(); G_ANOTHER_SOME_FOR_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANYBODY_AT_ALL() -> Py { ensure_init(); G_ANYBODY_AT_ALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANYBODY_AT_ALL(v: Py) { ensure_init(); G_ANYBODY_AT_ALL.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AGE_ALONE() -> Py { ensure_init(); G_AN_AGE_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE_ALONE(v: Py) { ensure_init(); G_AN_AGE_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AIM() -> Py { ensure_init(); G_AN_AIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AIM(v: Py) { ensure_init(); G_AN_AIM.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRIVE_VERB() -> Py { ensure_init(); G_ARRIVE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRIVE_VERB(v: Py) { ensure_init(); G_ARRIVE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_AS_WELL() -> Py { ensure_init(); G_AS_WELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AS_WELL(v: Py) { ensure_init(); G_AS_WELL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BIRTHDAY_TOLD() -> Py { ensure_init(); G_A_BIRTHDAY_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BIRTHDAY_TOLD(v: Py) { ensure_init(); G_A_BIRTHDAY_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BREED_BEING_CHOSEN() -> Py { ensure_init(); G_A_BREED_BEING_CHOSEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BREED_BEING_CHOSEN(v: Py) { ensure_init(); G_A_BREED_BEING_CHOSEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_IN_THE_LINE() -> Py { ensure_init(); G_A_CHILD_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_IN_THE_LINE(v: Py) { ensure_init(); G_A_CHILD_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE() -> Py { ensure_init(); G_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE(v: Py) { ensure_init(); G_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE_OF_ITS_OWN() -> Py { ensure_init(); G_A_CLAUSE_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE_OF_ITS_OWN(v: Py) { ensure_init(); G_A_CLAUSE_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLEAR() -> Py { ensure_init(); G_A_CLEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLEAR(v: Py) { ensure_init(); G_A_CLEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COMING_WORD() -> Py { ensure_init(); G_A_COMING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMING_WORD(v: Py) { ensure_init(); G_A_COMING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONDITION_KIND() -> Py { ensure_init(); G_A_CONDITION_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION_KIND(v: Py) { ensure_init(); G_A_CONDITION_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONTINUATION() -> Py { ensure_init(); G_A_CONTINUATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONTINUATION(v: Py) { ensure_init(); G_A_CONTINUATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CORRECTION_IN_FRONT() -> Py { ensure_init(); G_A_CORRECTION_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CORRECTION_IN_FRONT(v: Py) { ensure_init(); G_A_CORRECTION_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_NOT_A_STATE() -> Py { ensure_init(); G_A_COUNT_NOT_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_NOT_A_STATE(v: Py) { ensure_init(); G_A_COUNT_NOT_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_OF_THEM() -> Py { ensure_init(); G_A_COUNT_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_OF_THEM(v: Py) { ensure_init(); G_A_COUNT_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_ALONE() -> Py { ensure_init(); G_A_DATE_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_ALONE(v: Py) { ensure_init(); G_A_DATE_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_ON_A_HARD_PLAN() -> Py { ensure_init(); G_A_DATE_ON_A_HARD_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_ON_A_HARD_PLAN(v: Py) { ensure_init(); G_A_DATE_ON_A_HARD_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_ON_A_PLAN() -> Py { ensure_init(); G_A_DATE_ON_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_ON_A_PLAN(v: Py) { ensure_init(); G_A_DATE_ON_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_STILL_TO_COME() -> Py { ensure_init(); G_A_DATE_STILL_TO_COME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_STILL_TO_COME(v: Py) { ensure_init(); G_A_DATE_STILL_TO_COME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_TOLD() -> Py { ensure_init(); G_A_DATE_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_TOLD(v: Py) { ensure_init(); G_A_DATE_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_ASKED() -> Py { ensure_init(); G_A_DAY_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_ASKED(v: Py) { ensure_init(); G_A_DAY_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_ON_A_HARD_PLAN() -> Py { ensure_init(); G_A_DAY_ON_A_HARD_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_ON_A_HARD_PLAN(v: Py) { ensure_init(); G_A_DAY_ON_A_HARD_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_ON_A_PLAN() -> Py { ensure_init(); G_A_DAY_ON_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_ON_A_PLAN(v: Py) { ensure_init(); G_A_DAY_ON_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DECISION() -> Py { ensure_init(); G_A_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DECISION(v: Py) { ensure_init(); G_A_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEGREE() -> Py { ensure_init(); G_A_DEGREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEGREE(v: Py) { ensure_init(); G_A_DEGREE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DETERMINER() -> Py { ensure_init(); G_A_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DETERMINER(v: Py) { ensure_init(); G_A_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOING() -> Py { ensure_init(); G_A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING(v: Py) { ensure_init(); G_A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOING_TURNED_DOWN() -> Py { ensure_init(); G_A_DOING_TURNED_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING_TURNED_DOWN(v: Py) { ensure_init(); G_A_DOING_TURNED_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_EVENT() -> Py { ensure_init(); G_A_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_EVENT(v: Py) { ensure_init(); G_A_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEELING() -> Py { ensure_init(); G_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEELING(v: Py) { ensure_init(); G_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEELING_ABOUT_IT() -> Py { ensure_init(); G_A_FEELING_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEELING_ABOUT_IT(v: Py) { ensure_init(); G_A_FEELING_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GROUP_ALONE() -> Py { ensure_init(); G_A_GROUP_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GROUP_ALONE(v: Py) { ensure_init(); G_A_GROUP_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GROUP_ALONE_LINES() -> Py { ensure_init(); G_A_GROUP_ALONE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GROUP_ALONE_LINES(v: Py) { ensure_init(); G_A_GROUP_ALONE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAD_AN_ANIMAL() -> Py { ensure_init(); G_A_HAD_AN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAD_AN_ANIMAL(v: Py) { ensure_init(); G_A_HAD_AN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KNOWN_AGE_LINES() -> Py { ensure_init(); G_A_KNOWN_AGE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KNOWN_AGE_LINES(v: Py) { ensure_init(); G_A_KNOWN_AGE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_ECHO() -> Py { ensure_init(); G_A_LIKE_ECHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_ECHO(v: Py) { ensure_init(); G_A_LIKE_ECHO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_OF_ONE_THING() -> Py { ensure_init(); G_A_LIKE_OF_ONE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_OF_ONE_THING(v: Py) { ensure_init(); G_A_LIKE_OF_ONE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKE_WITH_TWO_HALVES() -> Py { ensure_init(); G_A_LIKE_WITH_TWO_HALVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKE_WITH_TWO_HALVES(v: Py) { ensure_init(); G_A_LIKE_WITH_TWO_HALVES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MARKED_HOUR() -> Py { ensure_init(); G_A_MARKED_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MARKED_HOUR(v: Py) { ensure_init(); G_A_MARKED_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE() -> Py { ensure_init(); G_A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE(v: Py) { ensure_init(); G_A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MILESTONE() -> Py { ensure_init(); G_A_MILESTONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MILESTONE(v: Py) { ensure_init(); G_A_MILESTONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONTH_ALONE() -> Py { ensure_init(); G_A_MONTH_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONTH_ALONE(v: Py) { ensure_init(); G_A_MONTH_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOVE() -> Py { ensure_init(); G_A_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOVE(v: Py) { ensure_init(); G_A_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOVED() -> Py { ensure_init(); G_A_MOVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOVED(v: Py) { ensure_init(); G_A_MOVED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_AFTER() -> Py { ensure_init(); G_A_NAME_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_AFTER(v: Py) { ensure_init(); G_A_NAME_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_GIVEN() -> Py { ensure_init(); G_A_NAME_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_GIVEN(v: Py) { ensure_init(); G_A_NAME_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_WAS_VIOLENT() -> Py { ensure_init(); G_A_NAME_WAS_VIOLENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_WAS_VIOLENT(v: Py) { ensure_init(); G_A_NAME_WAS_VIOLENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_ALONE() -> Py { ensure_init(); G_A_PERSON_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_ALONE(v: Py) { ensure_init(); G_A_PERSON_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_ALONE_LINES() -> Py { ensure_init(); G_A_PERSON_ALONE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_ALONE_LINES(v: Py) { ensure_init(); G_A_PERSON_ALONE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_LIKED() -> Py { ensure_init(); G_A_PERSON_LIKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_LIKED(v: Py) { ensure_init(); G_A_PERSON_LIKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PET_WANTED() -> Py { ensure_init(); G_A_PET_WANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PET_WANTED(v: Py) { ensure_init(); G_A_PET_WANTED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN_VERB() -> Py { ensure_init(); G_A_PLAN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_VERB(v: Py) { ensure_init(); G_A_PLAN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POINTED_AT() -> Py { ensure_init(); G_A_POINTED_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POINTED_AT(v: Py) { ensure_init(); G_A_POINTED_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POSITION() -> Py { ensure_init(); G_A_POSITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POSITION(v: Py) { ensure_init(); G_A_POSITION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POSITION_LINES() -> Py { ensure_init(); G_A_POSITION_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POSITION_LINES(v: Py) { ensure_init(); G_A_POSITION_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POSITION_MAYBE_A_THING() -> Py { ensure_init(); G_A_POSITION_MAYBE_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POSITION_MAYBE_A_THING(v: Py) { ensure_init(); G_A_POSITION_MAYBE_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REASON_MARKER() -> Py { ensure_init(); G_A_REASON_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REASON_MARKER(v: Py) { ensure_init(); G_A_REASON_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REASON_TAIL() -> Py { ensure_init(); G_A_REASON_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REASON_TAIL(v: Py) { ensure_init(); G_A_REASON_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RELATION() -> Py { ensure_init(); G_A_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RELATION(v: Py) { ensure_init(); G_A_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RESULT() -> Py { ensure_init(); G_A_RESULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RESULT(v: Py) { ensure_init(); G_A_RESULT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SECOND_PERSON() -> Py { ensure_init(); G_A_SECOND_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SECOND_PERSON(v: Py) { ensure_init(); G_A_SECOND_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SOMEONE() -> Py { ensure_init(); G_A_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SOMEONE(v: Py) { ensure_init(); G_A_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SPAN_OF_TIME() -> Py { ensure_init(); G_A_SPAN_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SPAN_OF_TIME(v: Py) { ensure_init(); G_A_SPAN_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAY_PERSON() -> Py { ensure_init(); G_A_STAY_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAY_PERSON(v: Py) { ensure_init(); G_A_STAY_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAY_PLAN() -> Py { ensure_init(); G_A_STAY_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAY_PLAN(v: Py) { ensure_init(); G_A_STAY_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRIP_PLAN() -> Py { ensure_init(); G_A_TRIP_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRIP_PLAN(v: Py) { ensure_init(); G_A_TRIP_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TWO_ALONE_LINES() -> Py { ensure_init(); G_A_TWO_ALONE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TWO_ALONE_LINES(v: Py) { ensure_init(); G_A_TWO_ALONE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VAGUE_MANY() -> Py { ensure_init(); G_A_VAGUE_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VAGUE_MANY(v: Py) { ensure_init(); G_A_VAGUE_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WANTED_ANIMAL() -> Py { ensure_init(); G_A_WANTED_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WANTED_ANIMAL(v: Py) { ensure_init(); G_A_WANTED_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WANTED_ANIMAL_TAIL() -> Py { ensure_init(); G_A_WANTED_ANIMAL_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WANTED_ANIMAL_TAIL(v: Py) { ensure_init(); G_A_WANTED_ANIMAL_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WANT_TURNED_DOWN() -> Py { ensure_init(); G_A_WANT_TURNED_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WANT_TURNED_DOWN(v: Py) { ensure_init(); G_A_WANT_TURNED_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEEKDAY() -> Py { ensure_init(); G_A_WEEKDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEEKDAY(v: Py) { ensure_init(); G_A_WEEKDAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WONDERED_CONDITION() -> Py { ensure_init(); G_A_WONDERED_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WONDERED_CONDITION(v: Py) { ensure_init(); G_A_WONDERED_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WONDER_SAID_NO() -> Py { ensure_init(); G_A_WONDER_SAID_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WONDER_SAID_NO(v: Py) { ensure_init(); G_A_WONDER_SAID_NO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORRIED_ABOUT_IT() -> Py { ensure_init(); G_A_WORRIED_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORRIED_ABOUT_IT(v: Py) { ensure_init(); G_A_WORRIED_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_AT_IT() -> Py { ensure_init(); G_BACK_AT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_AT_IT(v: Py) { ensure_init(); G_BACK_AT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_AT_IT_MINE() -> Py { ensure_init(); G_BACK_AT_IT_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_AT_IT_MINE(v: Py) { ensure_init(); G_BACK_AT_IT_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_AT_IT_TAIL() -> Py { ensure_init(); G_BACK_AT_IT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_AT_IT_TAIL(v: Py) { ensure_init(); G_BACK_AT_IT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_ONE() -> Py { ensure_init(); G_BEFORE_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_ONE(v: Py) { ensure_init(); G_BEFORE_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_BETWEEN_VALUES() -> Py { ensure_init(); G_BETWEEN_VALUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BETWEEN_VALUES(v: Py) { ensure_init(); G_BETWEEN_VALUES.with(|c| *c.borrow_mut() = v); }
pub fn g_BLEED() -> Py { ensure_init(); G_BLEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLEED(v: Py) { ensure_init(); G_BLEED.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY() -> Py { ensure_init(); G_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); G_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_STATE_PHRASES() -> Py { ensure_init(); G_BODY_STATE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE_PHRASES(v: Py) { ensure_init(); G_BODY_STATE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_STATE_TAIL() -> Py { ensure_init(); G_BODY_STATE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE_TAIL(v: Py) { ensure_init(); G_BODY_STATE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_STATE_WORDS() -> Py { ensure_init(); G_BODY_STATE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE_WORDS(v: Py) { ensure_init(); G_BODY_STATE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BUDGET_FRAMES() -> Py { ensure_init(); G_BUDGET_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUDGET_FRAMES(v: Py) { ensure_init(); G_BUDGET_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_BUYING() -> Py { ensure_init(); G_BUYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUYING(v: Py) { ensure_init(); G_BUYING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_OFFER_REPLY() -> Py { ensure_init(); G_CHAT_OFFER_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_OFFER_REPLY(v: Py) { ensure_init(); G_CHAT_OFFER_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD() -> Py { ensure_init(); G_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD(v: Py) { ensure_init(); G_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSES() -> Py { ensure_init(); G_CLAUSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSES(v: Py) { ensure_init(); G_CLAUSES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_AFTER_TAIL() -> Py { ensure_init(); G_CLOCK_AFTER_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_AFTER_TAIL(v: Py) { ensure_init(); G_CLOCK_AFTER_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_AT() -> Py { ensure_init(); G_CLOCK_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_AT(v: Py) { ensure_init(); G_CLOCK_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_DETAIL() -> Py { ensure_init(); G_CLOCK_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_DETAIL(v: Py) { ensure_init(); G_CLOCK_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_EVENT() -> Py { ensure_init(); G_CLOCK_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_EVENT(v: Py) { ensure_init(); G_CLOCK_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_FLAT() -> Py { ensure_init(); G_CLOCK_FLAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_FLAT(v: Py) { ensure_init(); G_CLOCK_FLAT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_FLAT_TAIL() -> Py { ensure_init(); G_CLOCK_FLAT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_FLAT_TAIL(v: Py) { ensure_init(); G_CLOCK_FLAT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_MORNING_TAIL() -> Py { ensure_init(); G_CLOCK_MORNING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_MORNING_TAIL(v: Py) { ensure_init(); G_CLOCK_MORNING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK_WORDS() -> Py { ensure_init(); G_CLOCK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_WORDS(v: Py) { ensure_init(); G_CLOCK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMING_TO_STAY() -> Py { ensure_init(); G_COMING_TO_STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMING_TO_STAY(v: Py) { ensure_init(); G_COMING_TO_STAY.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPLAINT() -> Py { ensure_init(); G_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPLAINT(v: Py) { ensure_init(); G_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_FAMILIES() -> Py { ensure_init(); G_CONDITION_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_FAMILIES(v: Py) { ensure_init(); G_CONDITION_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_FOOD() -> Py { ensure_init(); G_CONDITION_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_FOOD(v: Py) { ensure_init(); G_CONDITION_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_FRAMES() -> Py { ensure_init(); G_CONDITION_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_FRAMES(v: Py) { ensure_init(); G_CONDITION_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_INITIALS() -> Py { ensure_init(); G_CONDITION_INITIALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_INITIALS(v: Py) { ensure_init(); G_CONDITION_INITIALS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRAST() -> Py { ensure_init(); G_CONTRAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRAST(v: Py) { ensure_init(); G_CONTRAST.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRAST_SAID_AGAIN() -> Py { ensure_init(); G_CONTRAST_SAID_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRAST_SAID_AGAIN(v: Py) { ensure_init(); G_CONTRAST_SAID_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_ALONE() -> Py { ensure_init(); G_CORRECTED_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_ALONE(v: Py) { ensure_init(); G_CORRECTED_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED_WITH_UNIT() -> Py { ensure_init(); G_CORRECTED_WITH_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED_WITH_UNIT(v: Py) { ensure_init(); G_CORRECTED_WITH_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT() -> Py { ensure_init(); G_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT(v: Py) { ensure_init(); G_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTED() -> Py { ensure_init(); G_COUNTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTED(v: Py) { ensure_init(); G_COUNTED.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTED_LETTERS() -> Py { ensure_init(); G_COUNTED_LETTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTED_LETTERS(v: Py) { ensure_init(); G_COUNTED_LETTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTED_STARS() -> Py { ensure_init(); G_COUNTED_STARS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTED_STARS(v: Py) { ensure_init(); G_COUNTED_STARS.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_AHEAD() -> Py { ensure_init(); G_DATE_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_AHEAD(v: Py) { ensure_init(); G_DATE_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DAYS() -> Py { ensure_init(); G_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAYS(v: Py) { ensure_init(); G_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_BEEN_AND_GONE() -> Py { ensure_init(); G_DAY_BEEN_AND_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_BEEN_AND_GONE(v: Py) { ensure_init(); G_DAY_BEEN_AND_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_FAR() -> Py { ensure_init(); G_DAY_FAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_FAR(v: Py) { ensure_init(); G_DAY_FAR.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_JUST_A_STATE() -> Py { ensure_init(); G_DAY_JUST_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_JUST_A_STATE(v: Py) { ensure_init(); G_DAY_JUST_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_SOON() -> Py { ensure_init(); G_DAY_SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_SOON(v: Py) { ensure_init(); G_DAY_SOON.with(|c| *c.borrow_mut() = v); }
pub fn g_DEATH_WORDS() -> Py { ensure_init(); G_DEATH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEATH_WORDS(v: Py) { ensure_init(); G_DEATH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEGREE_CLASS() -> Py { ensure_init(); G_DEGREE_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEGREE_CLASS(v: Py) { ensure_init(); G_DEGREE_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_DESTINATION() -> Py { ensure_init(); G_DESTINATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESTINATION(v: Py) { ensure_init(); G_DESTINATION.with(|c| *c.borrow_mut() = v); }
pub fn g_DESTINATION_ARRIVAL() -> Py { ensure_init(); G_DESTINATION_ARRIVAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESTINATION_ARRIVAL(v: Py) { ensure_init(); G_DESTINATION_ARRIVAL.with(|c| *c.borrow_mut() = v); }
pub fn g_DIARY_TAIL() -> Py { ensure_init(); G_DIARY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIARY_TAIL(v: Py) { ensure_init(); G_DIARY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_DIARY_THING() -> Py { ensure_init(); G_DIARY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIARY_THING(v: Py) { ensure_init(); G_DIARY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_DIARY_TIME() -> Py { ensure_init(); G_DIARY_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIARY_TIME(v: Py) { ensure_init(); G_DIARY_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_DIARY_TIME_TAIL() -> Py { ensure_init(); G_DIARY_TIME_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIARY_TIME_TAIL(v: Py) { ensure_init(); G_DIARY_TIME_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_DIARY_WORDS() -> Py { ensure_init(); G_DIARY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIARY_WORDS(v: Py) { ensure_init(); G_DIARY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIGIT() -> Py { ensure_init(); G_DIGIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIGIT(v: Py) { ensure_init(); G_DIGIT.with(|c| *c.borrow_mut() = v); }
pub fn g_DREADED_OCCASION() -> Py { ensure_init(); G_DREADED_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DREADED_OCCASION(v: Py) { ensure_init(); G_DREADED_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_DRINK_FRAMES() -> Py { ensure_init(); G_DRINK_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRINK_FRAMES(v: Py) { ensure_init(); G_DRINK_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_DURATION_UNITS() -> Py { ensure_init(); G_DURATION_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DURATION_UNITS(v: Py) { ensure_init(); G_DURATION_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_RISE() -> Py { ensure_init(); G_EARLY_RISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_RISE(v: Py) { ensure_init(); G_EARLY_RISE.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_TAIL() -> Py { ensure_init(); G_EARLY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_TAIL(v: Py) { ensure_init(); G_EARLY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_ENDS_THE_THING() -> Py { ensure_init(); G_ENDS_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENDS_THE_THING(v: Py) { ensure_init(); G_ENDS_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ENDS_THE_THING_VERB() -> Py { ensure_init(); G_ENDS_THE_THING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENDS_THE_THING_VERB(v: Py) { ensure_init(); G_ENDS_THE_THING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPLAIN_TAIL() -> Py { ensure_init(); G_EXPLAIN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPLAIN_TAIL(v: Py) { ensure_init(); G_EXPLAIN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_FACTS() -> Py { ensure_init(); G_FACTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACTS(v: Py) { ensure_init(); G_FACTS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY() -> Py { ensure_init(); G_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); G_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FAR_TAIL() -> Py { ensure_init(); G_FAR_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAR_TAIL(v: Py) { ensure_init(); G_FAR_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING() -> Py { ensure_init(); G_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); G_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_FILLER() -> Py { ensure_init(); G_FIGURE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_FILLER(v: Py) { ensure_init(); G_FIGURE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_LEAD() -> Py { ensure_init(); G_FIGURE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_LEAD(v: Py) { ensure_init(); G_FIGURE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_UNIT() -> Py { ensure_init(); G_FIGURE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_UNIT(v: Py) { ensure_init(); G_FIGURE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_WORD() -> Py { ensure_init(); G_FIGURE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_WORD(v: Py) { ensure_init(); G_FIGURE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_WORD() -> Py { ensure_init(); G_FIRST_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_WORD(v: Py) { ensure_init(); G_FIRST_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_FRAMES() -> Py { ensure_init(); G_FOOD_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_FRAMES(v: Py) { ensure_init(); G_FOOD_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_A_THING() -> Py { ensure_init(); G_FOR_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_A_THING(v: Py) { ensure_init(); G_FOR_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAGMENT() -> Py { ensure_init(); G_FRAGMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAGMENT(v: Py) { ensure_init(); G_FRAGMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAMES() -> Py { ensure_init(); G_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAMES(v: Py) { ensure_init(); G_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVEN_ANIMAL() -> Py { ensure_init(); G_GIVEN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVEN_ANIMAL(v: Py) { ensure_init(); G_GIVEN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVEN_AWAY() -> Py { ensure_init(); G_GIVEN_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVEN_AWAY(v: Py) { ensure_init(); G_GIVEN_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVEN_AWAY_REPLIES() -> Py { ensure_init(); G_GIVEN_AWAY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVEN_AWAY_REPLIES(v: Py) { ensure_init(); G_GIVEN_AWAY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING() -> Py { ensure_init(); G_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING(v: Py) { ensure_init(); G_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_BAD() -> Py { ensure_init(); G_GOING_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_BAD(v: Py) { ensure_init(); G_GOING_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_HOME() -> Py { ensure_init(); G_GOING_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_HOME(v: Py) { ensure_init(); G_GOING_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_HOME_MINE() -> Py { ensure_init(); G_GOING_HOME_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_HOME_MINE(v: Py) { ensure_init(); G_GOING_HOME_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_HOME_TAIL() -> Py { ensure_init(); G_GOING_HOME_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_HOME_TAIL(v: Py) { ensure_init(); G_GOING_HOME_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_TAIL() -> Py { ensure_init(); G_GOING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_TAIL(v: Py) { ensure_init(); G_GOING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_MOOD() -> Py { ensure_init(); G_GOOD_MOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_MOOD(v: Py) { ensure_init(); G_GOOD_MOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETING() -> Py { ensure_init(); G_GREETING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETING(v: Py) { ensure_init(); G_GREETING.with(|c| *c.borrow_mut() = v); }
pub fn g_GROWING() -> Py { ensure_init(); G_GROWING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROWING(v: Py) { ensure_init(); G_GROWING.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT() -> Py { ensure_init(); G_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT(v: Py) { ensure_init(); G_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g_HABIT_TAIL() -> Py { ensure_init(); G_HABIT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HABIT_TAIL(v: Py) { ensure_init(); G_HABIT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_A() -> Py { ensure_init(); G_HAVE_A.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_A(v: Py) { ensure_init(); G_HAVE_A.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_GOT() -> Py { ensure_init(); G_HAVE_GOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_GOT(v: Py) { ensure_init(); G_HAVE_GOT.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_THING() -> Py { ensure_init(); G_HAVE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_THING(v: Py) { ensure_init(); G_HAVE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAVY_FRAMES() -> Py { ensure_init(); G_HEAVY_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAVY_FRAMES(v: Py) { ensure_init(); G_HEAVY_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAVY_TURNS() -> Py { ensure_init(); G_HEAVY_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAVY_TURNS(v: Py) { ensure_init(); G_HEAVY_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_TAIL() -> Py { ensure_init(); G_HOME_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_TAIL(v: Py) { ensure_init(); G_HOME_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_LONG_ALREADY() -> Py { ensure_init(); G_HOW_LONG_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG_ALREADY(v: Py) { ensure_init(); G_HOW_LONG_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_HURTS_A_PERSON() -> Py { ensure_init(); G_HURTS_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURTS_A_PERSON(v: Py) { ensure_init(); G_HURTS_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_INTRODUCED_BEFORE() -> Py { ensure_init(); G_INTRODUCED_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTRODUCED_BEFORE(v: Py) { ensure_init(); G_INTRODUCED_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_ITS_OWNS_AFTER() -> Py { ensure_init(); G_ITS_OWNS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITS_OWNS_AFTER(v: Py) { ensure_init(); G_ITS_OWNS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_BEFORE() -> Py { ensure_init(); G_JOB_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_BEFORE(v: Py) { ensure_init(); G_JOB_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_HAS_A_DAY() -> Py { ensure_init(); G_JOB_HAS_A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_HAS_A_DAY(v: Py) { ensure_init(); G_JOB_HAS_A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_TAIL() -> Py { ensure_init(); G_JOB_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_TAIL(v: Py) { ensure_init(); G_JOB_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_TO_DO() -> Py { ensure_init(); G_JOB_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_TO_DO(v: Py) { ensure_init(); G_JOB_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_POSSESSIVE() -> Py { ensure_init(); G_KIN_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_POSSESSIVE(v: Py) { ensure_init(); G_KIN_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_FINISH() -> Py { ensure_init(); G_LATE_FINISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_FINISH(v: Py) { ensure_init(); G_LATE_FINISH.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_TAIL() -> Py { ensure_init(); G_LATE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_TAIL(v: Py) { ensure_init(); G_LATE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADERS() -> Py { ensure_init(); G_LEADERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADERS(v: Py) { ensure_init(); G_LEADERS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_IN() -> Py { ensure_init(); G_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); G_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_CLOSED() -> Py { ensure_init(); G_LEFT_CLOSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_CLOSED(v: Py) { ensure_init(); G_LEFT_CLOSED.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OFF_CONTRACTION() -> Py { ensure_init(); G_LEFT_OFF_CONTRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OFF_CONTRACTION(v: Py) { ensure_init(); G_LEFT_OFF_CONTRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_KEEPS() -> Py { ensure_init(); G_LENGTH_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_KEEPS(v: Py) { ensure_init(); G_LENGTH_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_TAIL() -> Py { ensure_init(); G_LENGTH_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_TAIL(v: Py) { ensure_init(); G_LENGTH_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_LETTER_COUNT() -> Py { ensure_init(); G_LETTER_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LETTER_COUNT(v: Py) { ensure_init(); G_LETTER_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_LIKES_FRAMES() -> Py { ensure_init(); G_LIKES_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIKES_FRAMES(v: Py) { ensure_init(); G_LIKES_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT_AFTER_THE_LINE() -> Py { ensure_init(); G_LIMIT_AFTER_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT_AFTER_THE_LINE(v: Py) { ensure_init(); G_LIMIT_AFTER_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT_FRAMES() -> Py { ensure_init(); G_LIMIT_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT_FRAMES(v: Py) { ensure_init(); G_LIMIT_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_SHIFT() -> Py { ensure_init(); G_LONG_SHIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_SHIFT(v: Py) { ensure_init(); G_LONG_SHIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_TAIL() -> Py { ensure_init(); G_LONG_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_TAIL(v: Py) { ensure_init(); G_LONG_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_MAKE_A_VALUE() -> Py { ensure_init(); G_MAKE_A_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAKE_A_VALUE(v: Py) { ensure_init(); G_MAKE_A_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_MEETING_TAIL() -> Py { ensure_init(); G_MEETING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEETING_TAIL(v: Py) { ensure_init(); G_MEETING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_MEETING_TIME_TAIL() -> Py { ensure_init(); G_MEETING_TIME_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEETING_TIME_TAIL(v: Py) { ensure_init(); G_MEETING_TIME_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_MEET_GREET() -> Py { ensure_init(); G_MEET_GREET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEET_GREET(v: Py) { ensure_init(); G_MEET_GREET.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_ONLY() -> Py { ensure_init(); G_MONEY_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_ONLY(v: Py) { ensure_init(); G_MONEY_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST() -> Py { ensure_init(); G_MOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); G_MOST.with(|c| *c.borrow_mut() = v); }
pub fn g_NIGHT_TAIL() -> Py { ensure_init(); G_NIGHT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NIGHT_TAIL(v: Py) { ensure_init(); G_NIGHT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_AROUND() -> Py { ensure_init(); G_NOBODY_AROUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_AROUND(v: Py) { ensure_init(); G_NOBODY_AROUND.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_AROUND_ASK() -> Py { ensure_init(); G_NOBODY_AROUND_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_AROUND_ASK(v: Py) { ensure_init(); G_NOBODY_AROUND_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DAY_TO_LOOK_FORWARD_TO() -> Py { ensure_init(); G_NOT_A_DAY_TO_LOOK_FORWARD_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DAY_TO_LOOK_FORWARD_TO(v: Py) { ensure_init(); G_NOT_A_DAY_TO_LOOK_FORWARD_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SECOND_OF() -> Py { ensure_init(); G_NOT_A_SECOND_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SECOND_OF(v: Py) { ensure_init(); G_NOT_A_SECOND_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_TO_HAVE_ANOTHER() -> Py { ensure_init(); G_NOT_A_THING_TO_HAVE_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_TO_HAVE_ANOTHER(v: Py) { ensure_init(); G_NOT_A_THING_TO_HAVE_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TRIP() -> Py { ensure_init(); G_NOT_A_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TRIP(v: Py) { ensure_init(); G_NOT_A_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_VALUE() -> Py { ensure_init(); G_NOT_A_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VALUE(v: Py) { ensure_init(); G_NOT_A_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_VISIT() -> Py { ensure_init(); G_NOT_A_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VISIT(v: Py) { ensure_init(); G_NOT_A_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ONE_THING() -> Py { ensure_init(); G_NOT_ONE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ONE_THING(v: Py) { ensure_init(); G_NOT_ONE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THEIRS() -> Py { ensure_init(); G_NOT_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIRS(v: Py) { ensure_init(); G_NOT_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_HOURS() -> Py { ensure_init(); G_NOT_THE_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_HOURS(v: Py) { ensure_init(); G_NOT_THE_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_LEAD() -> Py { ensure_init(); G_NO_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_LEAD(v: Py) { ensure_init(); G_NO_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_REASON() -> Py { ensure_init(); G_NO_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_REASON(v: Py) { ensure_init(); G_NO_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_REFERENT() -> Py { ensure_init(); G_NO_REFERENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_REFERENT(v: Py) { ensure_init(); G_NO_REFERENT.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_THEN_A_LINE() -> Py { ensure_init(); G_NO_THEN_A_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_THEN_A_LINE(v: Py) { ensure_init(); G_NO_THEN_A_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_THEN_A_LINE_ONLY() -> Py { ensure_init(); G_NO_THEN_A_LINE_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_THEN_A_LINE_ONLY(v: Py) { ensure_init(); G_NO_THEN_A_LINE_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OBLIGED() -> Py { ensure_init(); G_OBLIGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBLIGED(v: Py) { ensure_init(); G_OBLIGED.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASIONS() -> Py { ensure_init(); G_OCCASIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASIONS(v: Py) { ensure_init(); G_OCCASIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION_FLAT_TAIL() -> Py { ensure_init(); G_OCCASION_FLAT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION_FLAT_TAIL(v: Py) { ensure_init(); G_OCCASION_FLAT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION_TAIL() -> Py { ensure_init(); G_OCCASION_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION_TAIL(v: Py) { ensure_init(); G_OCCASION_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFERS_A_CHAT() -> Py { ensure_init(); G_OFFERS_A_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFERS_A_CHAT(v: Py) { ensure_init(); G_OFFERS_A_CHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_OLD_ONE_TAIL() -> Py { ensure_init(); G_OLD_ONE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OLD_ONE_TAIL(v: Py) { ensure_init(); G_OLD_ONE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_STAR() -> Py { ensure_init(); G_ONE_STAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_STAR(v: Py) { ensure_init(); G_ONE_STAR.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_NIGHTS() -> Py { ensure_init(); G_ON_NIGHTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_NIGHTS(v: Py) { ensure_init(); G_ON_NIGHTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_MEND() -> Py { ensure_init(); G_ON_THE_MEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_MEND(v: Py) { ensure_init(); G_ON_THE_MEND.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_MEND_MINE() -> Py { ensure_init(); G_ON_THE_MEND_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_MEND_MINE(v: Py) { ensure_init(); G_ON_THE_MEND_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_MEND_TAIL() -> Py { ensure_init(); G_ON_THE_MEND_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_MEND_TAIL(v: Py) { ensure_init(); G_ON_THE_MEND_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_LIMIT_FRAMES() -> Py { ensure_init(); G_OTHER_LIMIT_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_LIMIT_FRAMES(v: Py) { ensure_init(); G_OTHER_LIMIT_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PERSON() -> Py { ensure_init(); G_OTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PERSON(v: Py) { ensure_init(); G_OTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_JOB() -> Py { ensure_init(); G_PAST_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_JOB(v: Py) { ensure_init(); G_PAST_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_JOB_PLAIN() -> Py { ensure_init(); G_PAST_JOB_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_JOB_PLAIN(v: Py) { ensure_init(); G_PAST_JOB_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_PLAN() -> Py { ensure_init(); G_PAST_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_PLAN(v: Py) { ensure_init(); G_PAST_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIOD() -> Py { ensure_init(); G_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIOD(v: Py) { ensure_init(); G_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_GROUP() -> Py { ensure_init(); G_PERSON_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_GROUP(v: Py) { ensure_init(); G_PERSON_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_WORDS() -> Py { ensure_init(); G_PERSON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORDS(v: Py) { ensure_init(); G_PERSON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_ADDED() -> Py { ensure_init(); G_PET_ADDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_ADDED(v: Py) { ensure_init(); G_PET_ADDED.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_ALL_RIGHT() -> Py { ensure_init(); G_PET_ALL_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_ALL_RIGHT(v: Py) { ensure_init(); G_PET_ALL_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_TAIL_COMING() -> Py { ensure_init(); G_PET_TAIL_COMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_TAIL_COMING(v: Py) { ensure_init(); G_PET_TAIL_COMING.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_TAIL_COMING_MANY() -> Py { ensure_init(); G_PET_TAIL_COMING_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_TAIL_COMING_MANY(v: Py) { ensure_init(); G_PET_TAIL_COMING_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_TAIL_MANY() -> Py { ensure_init(); G_PET_TAIL_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_TAIL_MANY(v: Py) { ensure_init(); G_PET_TAIL_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_TAIL_ONE() -> Py { ensure_init(); G_PET_TAIL_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_TAIL_ONE(v: Py) { ensure_init(); G_PET_TAIL_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_WORD() -> Py { ensure_init(); G_PET_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_WORD(v: Py) { ensure_init(); G_PET_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_FRAME() -> Py { ensure_init(); G_PICK_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_FRAME(v: Py) { ensure_init(); G_PICK_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACED_IN_THE_PAST() -> Py { ensure_init(); G_PLACED_IN_THE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACED_IN_THE_PAST(v: Py) { ensure_init(); G_PLACED_IN_THE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACES_WITH_THE() -> Py { ensure_init(); G_PLACES_WITH_THE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACES_WITH_THE(v: Py) { ensure_init(); G_PLACES_WITH_THE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_TURNS() -> Py { ensure_init(); G_PLAN_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_TURNS(v: Py) { ensure_init(); G_PLAN_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_KIN() -> Py { ensure_init(); G_PLURAL_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_KIN(v: Py) { ensure_init(); G_PLURAL_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTER_FILLER() -> Py { ensure_init(); G_POINTER_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTER_FILLER(v: Py) { ensure_init(); G_POINTER_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_AT_ANOTHER() -> Py { ensure_init(); G_POINTS_AT_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT_ANOTHER(v: Py) { ensure_init(); G_POINTS_AT_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_POLITE() -> Py { ensure_init(); G_POLITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLITE(v: Py) { ensure_init(); G_POLITE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_POINTS_AT_ANOTHER() -> Py { ensure_init(); G_PRONOUN_POINTS_AT_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_POINTS_AT_ANOTHER(v: Py) { ensure_init(); G_PRONOUN_POINTS_AT_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT_LEAD() -> Py { ensure_init(); G_PUT_RIGHT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT_LEAD(v: Py) { ensure_init(); G_PUT_RIGHT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT_TAIL() -> Py { ensure_init(); G_PUT_RIGHT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT_TAIL(v: Py) { ensure_init(); G_PUT_RIGHT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_RAISED() -> Py { ensure_init(); G_RAISED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAISED(v: Py) { ensure_init(); G_RAISED.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_PHRASES() -> Py { ensure_init(); G_REACTION_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_PHRASES(v: Py) { ensure_init(); G_REACTION_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_REACTION_WORDS() -> Py { ensure_init(); G_REACTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACTION_WORDS(v: Py) { ensure_init(); G_REACTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_ALREADY_PAID() -> Py { ensure_init(); G_REMARK_ALREADY_PAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_ALREADY_PAID(v: Py) { ensure_init(); G_REMARK_ALREADY_PAID.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_BACK() -> Py { ensure_init(); G_REMARK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_BACK(v: Py) { ensure_init(); G_REMARK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_BETTER() -> Py { ensure_init(); G_REMARK_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_BETTER(v: Py) { ensure_init(); G_REMARK_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_EXPENSE() -> Py { ensure_init(); G_REMARK_EXPENSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_EXPENSE(v: Py) { ensure_init(); G_REMARK_EXPENSE.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_FIGURE() -> Py { ensure_init(); G_REMARK_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_FIGURE(v: Py) { ensure_init(); G_REMARK_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_FIGURE_REPLIES() -> Py { ensure_init(); G_REMARK_FIGURE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_FIGURE_REPLIES(v: Py) { ensure_init(); G_REMARK_FIGURE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_GONE() -> Py { ensure_init(); G_REMARK_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_GONE(v: Py) { ensure_init(); G_REMARK_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_HARD() -> Py { ensure_init(); G_REMARK_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_HARD(v: Py) { ensure_init(); G_REMARK_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_JUDGES_THE_PRICE() -> Py { ensure_init(); G_REMARK_JUDGES_THE_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_JUDGES_THE_PRICE(v: Py) { ensure_init(); G_REMARK_JUDGES_THE_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_KINDS() -> Py { ensure_init(); G_REMARK_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_KINDS(v: Py) { ensure_init(); G_REMARK_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_LONG() -> Py { ensure_init(); G_REMARK_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_LONG(v: Py) { ensure_init(); G_REMARK_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_PRICE_AHEAD() -> Py { ensure_init(); G_REMARK_PRICE_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_PRICE_AHEAD(v: Py) { ensure_init(); G_REMARK_PRICE_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_REPLIES() -> Py { ensure_init(); G_REMARK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_REPLIES(v: Py) { ensure_init(); G_REMARK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_STILL_TO_COME() -> Py { ensure_init(); G_REMARK_STILL_TO_COME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_STILL_TO_COME(v: Py) { ensure_init(); G_REMARK_STILL_TO_COME.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_TWO_SIDED() -> Py { ensure_init(); G_REMARK_TWO_SIDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_TWO_SIDED(v: Py) { ensure_init(); G_REMARK_TWO_SIDED.with(|c| *c.borrow_mut() = v); }
pub fn g_REMARK_WORK() -> Py { ensure_init(); G_REMARK_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMARK_WORK(v: Py) { ensure_init(); G_REMARK_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_FRAME() -> Py { ensure_init(); G_REPAIR_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_FRAME(v: Py) { ensure_init(); G_REPAIR_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_REPLACING() -> Py { ensure_init(); G_REPLACING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPLACING(v: Py) { ensure_init(); G_REPLACING.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTS_AN_ACTION() -> Py { ensure_init(); G_REPORTS_AN_ACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTS_AN_ACTION(v: Py) { ensure_init(); G_REPORTS_AN_ACTION.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_BACK_WORDS() -> Py { ensure_init(); G_SAID_BACK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_BACK_WORDS(v: Py) { ensure_init(); G_SAID_BACK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_INTRO() -> Py { ensure_init(); G_SELF_INTRO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_INTRO(v: Py) { ensure_init(); G_SELF_INTRO.with(|c| *c.borrow_mut() = v); }
pub fn g_SEPARATION() -> Py { ensure_init(); G_SEPARATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEPARATION(v: Py) { ensure_init(); G_SEPARATION.with(|c| *c.borrow_mut() = v); }
pub fn g_SHIFTS_ALL_OVER() -> Py { ensure_init(); G_SHIFTS_ALL_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHIFTS_ALL_OVER(v: Py) { ensure_init(); G_SHIFTS_ALL_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_SHIFT_EARLY() -> Py { ensure_init(); G_SHIFT_EARLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHIFT_EARLY(v: Py) { ensure_init(); G_SHIFT_EARLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SHIFT_LATE() -> Py { ensure_init(); G_SHIFT_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHIFT_LATE(v: Py) { ensure_init(); G_SHIFT_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOWER() -> Py { ensure_init(); G_SHOWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOWER(v: Py) { ensure_init(); G_SHOWER.with(|c| *c.borrow_mut() = v); }
pub fn g_SINCE_TAIL() -> Py { ensure_init(); G_SINCE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SINCE_TAIL(v: Py) { ensure_init(); G_SINCE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_COMING() -> Py { ensure_init(); G_SOMEONE_COMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_COMING(v: Py) { ensure_init(); G_SOMEONE_COMING.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE_POINTS_AT_ANOTHER() -> Py { ensure_init(); G_SOMEONE_ELSE_POINTS_AT_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE_POINTS_AT_ANOTHER(v: Py) { ensure_init(); G_SOMEONE_ELSE_POINTS_AT_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_GOOD() -> Py { ensure_init(); G_SOMEONE_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_GOOD(v: Py) { ensure_init(); G_SOMEONE_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_MILESTONE() -> Py { ensure_init(); G_SOMEONE_MILESTONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_MILESTONE(v: Py) { ensure_init(); G_SOMEONE_MILESTONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_MILESTONE_TAIL() -> Py { ensure_init(); G_SOMEONE_MILESTONE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_MILESTONE_TAIL(v: Py) { ensure_init(); G_SOMEONE_MILESTONE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_TAIL() -> Py { ensure_init(); G_SOMEONE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_TAIL(v: Py) { ensure_init(); G_SOMEONE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOON_TAIL() -> Py { ensure_init(); G_SOON_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOON_TAIL(v: Py) { ensure_init(); G_SOON_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SORRY_TAIL() -> Py { ensure_init(); G_SORRY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORRY_TAIL(v: Py) { ensure_init(); G_SORRY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SPEAK() -> Py { ensure_init(); G_SPEAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPEAK(v: Py) { ensure_init(); G_SPEAK.with(|c| *c.borrow_mut() = v); }
pub fn g_SPENDING_FRAMES() -> Py { ensure_init(); G_SPENDING_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPENDING_FRAMES(v: Py) { ensure_init(); G_SPENDING_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_SPREAD_TAIL() -> Py { ensure_init(); G_SPREAD_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPREAD_TAIL(v: Py) { ensure_init(); G_SPREAD_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_STAR_COUNT() -> Py { ensure_init(); G_STAR_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAR_COUNT(v: Py) { ensure_init(); G_STAR_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE() -> Py { ensure_init(); G_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE(v: Py) { ensure_init(); G_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_OPENS() -> Py { ensure_init(); G_STEP_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_OPENS(v: Py) { ensure_init(); G_STEP_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_FRAMES() -> Py { ensure_init(); G_STILL_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_FRAMES(v: Py) { ensure_init(); G_STILL_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_TO_DO() -> Py { ensure_init(); G_STILL_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_TO_DO(v: Py) { ensure_init(); G_STILL_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP() -> Py { ensure_init(); G_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); G_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT() -> Py { ensure_init(); G_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT(v: Py) { ensure_init(); G_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_OFF() -> Py { ensure_init(); G_TAIL_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_OFF(v: Py) { ensure_init(); G_TAIL_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS() -> Py { ensure_init(); G_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); G_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LIMIT_GOES_FIRST() -> Py { ensure_init(); G_THE_LIMIT_GOES_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LIMIT_GOES_FIRST(v: Py) { ensure_init(); G_THE_LIMIT_GOES_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NOUN_IS_AT() -> Py { ensure_init(); G_THE_NOUN_IS_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NOUN_IS_AT(v: Py) { ensure_init(); G_THE_NOUN_IS_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_OLD_ONE() -> Py { ensure_init(); G_THE_OLD_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_OLD_ONE(v: Py) { ensure_init(); G_THE_OLD_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_STORY_GOES_ON() -> Py { ensure_init(); G_THE_STORY_GOES_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_STORY_GOES_ON(v: Py) { ensure_init(); G_THE_STORY_GOES_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_BEFORE() -> Py { ensure_init(); G_THING_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_BEFORE(v: Py) { ensure_init(); G_THING_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PERSON() -> Py { ensure_init(); G_THIRD_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON(v: Py) { ensure_init(); G_THIRD_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORD() -> Py { ensure_init(); G_TIME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORD(v: Py) { ensure_init(); G_TIME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP_TAIL() -> Py { ensure_init(); G_TRIP_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP_TAIL(v: Py) { ensure_init(); G_TRIP_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP_VERB() -> Py { ensure_init(); G_TRIP_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP_VERB(v: Py) { ensure_init(); G_TRIP_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_TROUBLE() -> Py { ensure_init(); G_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TROUBLE(v: Py) { ensure_init(); G_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_PLACE() -> Py { ensure_init(); G_TWO_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_PLACE(v: Py) { ensure_init(); G_TWO_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_TO() -> Py { ensure_init(); G_TWO_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_TO(v: Py) { ensure_init(); G_TWO_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_VERB() -> Py { ensure_init(); G_TWO_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_VERB(v: Py) { ensure_init(); G_TWO_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS_AFTER() -> Py { ensure_init(); G_UNITS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS_AFTER(v: Py) { ensure_init(); G_UNITS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS_OF_MEASURE() -> Py { ensure_init(); G_UNITS_OF_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS_OF_MEASURE(v: Py) { ensure_init(); G_UNITS_OF_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_CLOSED() -> Py { ensure_init(); G_UP_CLOSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_CLOSED(v: Py) { ensure_init(); G_UP_CLOSED.with(|c| *c.borrow_mut() = v); }
pub fn g_VAGUE() -> Py { ensure_init(); G_VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VAGUE(v: Py) { ensure_init(); G_VAGUE.with(|c| *c.borrow_mut() = v); }
pub fn g_VALUE() -> Py { ensure_init(); G_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VALUE(v: Py) { ensure_init(); G_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_VALUE_UNIT_GLUED() -> Py { ensure_init(); G_VALUE_UNIT_GLUED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VALUE_UNIT_GLUED(v: Py) { ensure_init(); G_VALUE_UNIT_GLUED.with(|c| *c.borrow_mut() = v); }
pub fn g_VALUE_UNIT_WORDS() -> Py { ensure_init(); G_VALUE_UNIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VALUE_UNIT_WORDS(v: Py) { ensure_init(); G_VALUE_UNIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VALUE_WORDS() -> Py { ensure_init(); G_VALUE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VALUE_WORDS(v: Py) { ensure_init(); G_VALUE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VERBISH() -> Py { ensure_init(); G_VERBISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERBISH(v: Py) { ensure_init(); G_VERBISH.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_TAIL() -> Py { ensure_init(); G_VISIT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_TAIL(v: Py) { ensure_init(); G_VISIT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_TAIL_MANY() -> Py { ensure_init(); G_VISIT_TAIL_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_TAIL_MANY(v: Py) { ensure_init(); G_VISIT_TAIL_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_TAIL_ONE() -> Py { ensure_init(); G_VISIT_TAIL_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_TAIL_ONE(v: Py) { ensure_init(); G_VISIT_TAIL_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_ANSWER() -> Py { ensure_init(); G_WANTS_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_ANSWER(v: Py) { ensure_init(); G_WANTS_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_IT() -> Py { ensure_init(); G_WANTS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_IT(v: Py) { ensure_init(); G_WANTS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_OF_THEM() -> Py { ensure_init(); G_WHAT_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_OF_THEM(v: Py) { ensure_init(); G_WHAT_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_DAYS() -> Py { ensure_init(); G_WHEN_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_DAYS(v: Py) { ensure_init(); G_WHEN_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_DAYS_TAIL() -> Py { ensure_init(); G_WHEN_DAYS_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_DAYS_TAIL(v: Py) { ensure_init(); G_WHEN_DAYS_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_DETAIL() -> Py { ensure_init(); G_WHEN_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_DETAIL(v: Py) { ensure_init(); G_WHEN_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ROOMY_TAIL() -> Py { ensure_init(); G_WHEN_ROOMY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ROOMY_TAIL(v: Py) { ensure_init(); G_WHEN_ROOMY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_TODAY() -> Py { ensure_init(); G_WHEN_TODAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_TODAY(v: Py) { ensure_init(); G_WHEN_TODAY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_TODAY_TAIL() -> Py { ensure_init(); G_WHEN_TODAY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_TODAY_TAIL(v: Py) { ensure_init(); G_WHEN_TODAY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_TOMORROW() -> Py { ensure_init(); G_WHEN_TOMORROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_TOMORROW(v: Py) { ensure_init(); G_WHEN_TOMORROW.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_TOMORROW_TAIL() -> Py { ensure_init(); G_WHEN_TOMORROW_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_TOMORROW_TAIL(v: Py) { ensure_init(); G_WHEN_TOMORROW_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHICH_ONE() -> Py { ensure_init(); G_WHICH_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHICH_ONE(v: Py) { ensure_init(); G_WHICH_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHICH_ONES() -> Py { ensure_init(); G_WHICH_ONES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHICH_ONES(v: Py) { ensure_init(); G_WHICH_ONES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_FREQUENCY() -> Py { ensure_init(); G_WHOLE_FREQUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_FREQUENCY(v: Py) { ensure_init(); G_WHOLE_FREQUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOS() -> Py { ensure_init(); G_WHOS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOS(v: Py) { ensure_init(); G_WHOS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_IS_IT() -> Py { ensure_init(); G_WHO_IS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_IS_IT(v: Py) { ensure_init(); G_WHO_IS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_THE_FIGURE_IS() -> Py { ensure_init(); G_WHO_THE_FIGURE_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_THE_FIGURE_IS(v: Py) { ensure_init(); G_WHO_THE_FIGURE_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_WISH_FOR_AN_ANIMAL() -> Py { ensure_init(); G_WISH_FOR_AN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WISH_FOR_AN_ANIMAL(v: Py) { ensure_init(); G_WISH_FOR_AN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_AN_ARTICLE() -> Py { ensure_init(); G_WITH_AN_ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_AN_ARTICLE(v: Py) { ensure_init(); G_WITH_AN_ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_SLASH() -> Py { ensure_init(); G_WITH_SLASH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_SLASH(v: Py) { ensure_init(); G_WITH_SLASH.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_FRAMES() -> Py { ensure_init(); G_WORK_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_FRAMES(v: Py) { ensure_init(); G_WORK_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_FROM_HOME() -> Py { ensure_init(); G_WORK_FROM_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_FROM_HOME(v: Py) { ensure_init(); G_WORK_FROM_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_PLACE() -> Py { ensure_init(); G_WORK_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_PLACE(v: Py) { ensure_init(); G_WORK_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_WOULD_HAVE_BEEN() -> Py { ensure_init(); G_WOULD_HAVE_BEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WOULD_HAVE_BEEN(v: Py) { ensure_init(); G_WOULD_HAVE_BEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_AFTER() -> Py { ensure_init(); G_YES_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_AFTER(v: Py) { ensure_init(); G_YES_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__ANSWER_OPENER() -> Py { ensure_init(); G__ANSWER_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANSWER_OPENER(v: Py) { ensure_init(); G__ANSWER_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ABILITY() -> Py { ensure_init(); G__AN_ABILITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ABILITY(v: Py) { ensure_init(); G__AN_ABILITY.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED_FOR() -> Py { ensure_init(); G__ASKED_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED_FOR(v: Py) { ensure_init(); G__ASKED_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_MIDWAY() -> Py { ensure_init(); G__ASKS_MIDWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_MIDWAY(v: Py) { ensure_init(); G__ASKS_MIDWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PLAN_WAS_GIVEN() -> Py { ensure_init(); G__A_PLAN_WAS_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PLAN_WAS_GIVEN(v: Py) { ensure_init(); G__A_PLAN_WAS_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g__BULLIED() -> Py { ensure_init(); G__BULLIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BULLIED(v: Py) { ensure_init(); G__BULLIED.with(|c| *c.borrow_mut() = v); }
pub fn g__EMPTY() -> Py { ensure_init(); G__EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EMPTY(v: Py) { ensure_init(); G__EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g__FAULT() -> Py { ensure_init(); G__FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAULT(v: Py) { ensure_init(); G__FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELING_WORDS() -> Py { ensure_init(); G__FEELING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELING_WORDS(v: Py) { ensure_init(); G__FEELING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__HOLE() -> Py { ensure_init(); G__HOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOLE(v: Py) { ensure_init(); G__HOLE.with(|c| *c.borrow_mut() = v); }
pub fn g__LIMIT_ONLY() -> Py { ensure_init(); G__LIMIT_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIMIT_ONLY(v: Py) { ensure_init(); G__LIMIT_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BODY_PART() -> Py { ensure_init(); G__NOT_A_BODY_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BODY_PART(v: Py) { ensure_init(); G__NOT_A_BODY_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DETAIL() -> Py { ensure_init(); G__NOT_A_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DETAIL(v: Py) { ensure_init(); G__NOT_A_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DETAIL_SUBJECT() -> Py { ensure_init(); G__NOT_A_DETAIL_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DETAIL_SUBJECT(v: Py) { ensure_init(); G__NOT_A_DETAIL_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_WORDS() -> Py { ensure_init(); G__NOT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_WORDS(v: Py) { ensure_init(); G__NOT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_WITH_AN_ASK() -> Py { ensure_init(); G__OPENS_WITH_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_WITH_AN_ASK(v: Py) { ensure_init(); G__OPENS_WITH_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_WITH_A_LINK() -> Py { ensure_init(); G__OPENS_WITH_A_LINK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_WITH_A_LINK(v: Py) { ensure_init(); G__OPENS_WITH_A_LINK.with(|c| *c.borrow_mut() = v); }
pub fn g__REACTION_CLAUSE() -> Py { ensure_init(); G__REACTION_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REACTION_CLAUSE(v: Py) { ensure_init(); G__REACTION_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_BACK() -> Py { ensure_init(); G__SAID_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_BACK(v: Py) { ensure_init(); G__SAID_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_YES_NO() -> Py { ensure_init(); G__SAID_YES_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_YES_NO(v: Py) { ensure_init(); G__SAID_YES_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__SENTENCE() -> Py { ensure_init(); G__SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SENTENCE(v: Py) { ensure_init(); G__SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE_IN_IT() -> Py { ensure_init(); G__SOMEONE_ELSE_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE_IN_IT(v: Py) { ensure_init(); G__SOMEONE_ELSE_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__SPACED_DASH() -> Py { ensure_init(); G__SPACED_DASH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPACED_DASH(v: Py) { ensure_init(); G__SPACED_DASH.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_BOT_AS_SUBJECT() -> Py { ensure_init(); G__THE_BOT_AS_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_BOT_AS_SUBJECT(v: Py) { ensure_init(); G__THE_BOT_AS_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__WORDS() -> Py { ensure_init(); G__WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORDS(v: Py) { ensure_init(); G__WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_the_rest_of_a_question() -> Py { ensure_init(); G_the_rest_of_a_question.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_rest_of_a_question(v: Py) { ensure_init(); G_the_rest_of_a_question.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Say back the thing they just told you, in their own words.\n\nA person who says \"my sister is called Meg\" or \"we're flying to Rome on Monday\" is handing you\nsomething. A friend answers with the thing itself - \"Meg, got it\" or \"Rome on Monday, nice\" -\nnot one stock line for every line (\"Noted!\") and not a question at the end every time either.\nThe store is what makes the echo possible; the person's own words are what make it fit.\n\nbot/modules/memory_store.py asks this after its recall answers, and only for a line that is\ntelling the bot something. Every rule here is a guard against taking a turn that is not that:\n\n* only a statement - never a question, a request, a greeting, a thanks or a \"yes\"\n  (bot/understanding's kind, in memory[\"understood\"]);\n* only when the line gave the store a fact (bot/memory/facts.extract found one), or when it adds\n  a detail to the thing we are already on (\"i work from home\" after a question about naps), so\n  small talk the bot merely heard is never echoed back;\n* never a greeting or a self-introduction, because the chat module answers \"Nice to meet you,\n  Tom\" better than an echo can;\n* never a line with a feeling, a loss or good news in it: those belong to the modules that\n  answer them, and \"your dad died in March - noted\" would be awful. The caller passes only\n  lines whose mood is neutral, and bot/compose/loss.py and bot/compose/badnews.py are asked\n  here as well, so a kind reply always wins over an echo;\n* never a body that hurts, a fit or a wound (\"your dog choked on a bone - noted\" says nothing\n  about the dog), and never a birthday nobody reached (\"your mum would have been 70 - noted\");\n* the echo is short: the first clause or two of what they said, cut at a word, so the reply\n  matches the line instead of repeating it.\n\nThe frames vary through bot.compose.pick, so the bot does not say \"noted\" three times running.\n");
    s_SHIFT_EARLY(gen_understanding::understanding::kinds::g_SHIFT_EARLY());
    s_SHIFT_LATE(gen_understanding::understanding::kinds::g_SHIFT_LATE());
    s_VERBISH(gen_understanding::understanding::kinds::g_VERBISH());
    s_NUMBER_WORDS(gen_understanding::understanding::topics::g_NUMBER_WORDS());
    s_LEFT_OFF_CONTRACTION(gen_understanding::understanding::normalize::g_LEFT_OFF_CONTRACTION());
    s_LEADERS(gen_understanding::understanding::normalize::g_LEADERS());
    s_DAYS(crate::memory::facts::g_DAYS());
    s_MONTHS(crate::memory::facts::g_MONTHS());
    s_CHILD(crate::memory::store_lines::g_CHILD());
    s_PERSON_GROUP(crate::memory::store_lines::g_PERSON_GROUP());
    s_FACTS(py_tuple(vec![py_str("name"), py_str("age"), py_str("job"), py_str("city"), py_str("places"), py_str("allergies"), py_str("pets"), py_str("people"), py_str("budget"), py_str("diet"), py_str("plan"), py_str("done"), py_str("others"), py_str("relations"), py_str("condition"), py_str("tastes"), py_str("birthday"), py_str("weight"), py_str("height")]));
    s_EXPLAIN_TAIL(py_tuple(vec![py_str("{Thing}. That explains it — no wonder it sits on you. What would take some weight off?"), py_str("{Thing}. That is a lot to carry on top of how you feel. What would help most?"), py_str("{Thing} — so that is the part behind it. What has been hardest about it?")]));
    s_HAVE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:['\\u2019]?ve)?\\s+(?:have(?:\\s+got)?|got|own|keep|just got|recently got|already got|just bought|bought)\\b(?:\\s+(?:already|just|only|still|now|also))?\\s+(?:(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|another|some|no|few|couple|my|our|his|her|their)\\s+)?([a-z][a-z'-]{2,20})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\n    feeling idea question clue hurt look go try rest break problem issue theory plan reason hope\n    dream nightmare laugh cry shower think thought memory chance sense guess moment minute while\n    day week month year option choice say part lot bit time life job appointment check\n    read call text email message right headache pain ache cough cold temperature fever migraine\n    bug flu to no not nothing none much more less enough already got\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AIM(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:ok|okay|so|well|right|and|but|yeah|hi|hey)[,\\s]+)*(?:(?:i'?m|i am|im|i|we'?re|we are|we)\\s+(?:really |still |just |also |now )?(?:trying|hoping|aiming|planning|working|wanting|looking)\\b[^.!?]{0,40}?\\b(?:to|on|at)\\b|\\bi want to (?!talk|say|tell|ask|know|feel|die|disappear|give up)\\b|\\bi(?:'?m| am|m) on a (?:diet|plan|course|programme)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEELING_ABOUT_IT(stdlib_call("re", "compile", &[py_str("\\b(?:feel\\s+(?:like\\s+)?(?:giving|awful|terrible|rubbish|horrible|dreadful|hopeless|useless|a\\s+failure|like\\s+a\\s+failure|low|down|bad|shit|stupid|fat|ugly|impossible|sick)|dont\\s+feel\\s+like|don'?t\\s+feel\\s+like|not\\s+feeling\\s+it|feelings?|felt\\s+(?:like|awful|terrible|low|down)|it\\s+feels\\s+impossible|giv(?:e|ing|en)\\s+up|gave\\s+up|going\\s+to\\s+quit|struggl\\w*|can'?t\\s+(?:do\\s+it|keep|cope|face|take\\s+it|manage)|cannot\\s+(?:do|keep|cope)|hopeless|worthless|useless|pointless|miserable|ashamed|guilty|hate\\s+my\\s+body|fail\\w*|fed\\s+up|sick\\s+of|had\\s+enough|tired\\s+of\\s+it|exhausted|knackered|worn\\s+out|burnt?\\s+out|embarrass\\w*|depress\\w*|anxious|anxiety|stressed|tearful|crying|no\\s+motivation|losing\\s+(?:heart|motivation)|disheartened|demoralis\\w*|demoraliz\\w*|not\\s+going\\s+well|going\\s+badly|worried|scared|afraid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_GOOD(stdlib_call("re", "compile", &[py_str("\\b(?:prize|award|medal|trophy|cup|distinction|honours|top of the class|head boy|head girl|scholarship|won|wins|winning|came first|passed|passing|graduated|graduates|degree|a[\\s*'-]?stars?\\b(?![\\s-]*(?:wars|trek|gazer|gazing|sign|bucks|fish|city|chart))|a\\*s?|straight 9s|all 9s|all 8s|grade 9|grade 8|top grades|got into|got engaged|engaged|married|expecting|gave birth|had a baby|new baby|promoted|promotion|new job|retired|retiring|all clear|out of hospital|new puppy|new kitten|new dog|new cat|new rabbit|new hamster|new foal)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_TAIL(py_tuple(vec![py_str("{Thing} — that's lovely news."), py_str("{Thing}. That's a good bit of news to hear."), py_str("{Thing} — you must be pleased for them."), py_str("{Thing}. That's worth a bit of a fuss.")]));
    s_SOMEONE_COMING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:mum|mom|mam|mother|dad|father|son|daughter|sister|brother|wife|husband|partner|aunt|uncle|gran|granny|grandma|grandad|grandpa|nan|cousin|niece|nephew|mate|mates|friends?|in[- ]?laws?|parents|family|folks|brother|sister-in-law|brother-in-law)\\b[^.;!?]{0,18}?\\b(?:is|are|was|were)\\s+(?:already\\s+|just\\s+|now\\s+|still\\s+|all\\s+)?(?:coming|visiting|staying|stopping|arriving|due\\s+to\\s+(?:arrive|come|visit|stay))\\b|\\b(?:i'?m|im|i am|we'?re|we are)\\s+(?:off\\s+|going\\s+)?(?:up|down|over|back)?\\s*(?:to\\s+see|visiting|staying\\s+with|stopping\\s+with)\\s+(?:my|our)\\s+(?:mum|mom|mam|mother|dad|father|son|daughter|sister|brother|wife|husband|partner|aunt|uncle|gran|granny|grandma|grandad|grandpa|nan|cousin|niece|nephew|mate|friends?|parents|family|folks)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:pick(?:ing)?\\s+(?:me|us)\\s+up|drop(?:ping)?\\s+(?:me|us)\\s+off|collect(?:ing)?\\s+(?:me|us)|to\\s+(?:have\\s+a\\s+look|check|fix|repair|help|clean|deliver|install|service)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VISIT_TAIL(py_tuple(vec![py_str("{Thing} — that's something to look forward to."), py_str("{Thing}. Nice — a proper visit."), py_str("{Thing} — enjoy it."), py_str("{Thing}. Good — that's a proper bit of the week.")]));
    s_SOMEONE_MILESTONE(stdlib_call("re", "compile", &[py_str("\\b(?:birthday|anniversary)\\b[^.;!?]{0,24}?\\b(?:next|this|on|is)\\s+(?:week|month|weekend|today|tomorrow|saturday|sunday|monday|tuesday|wednesday|thursday|friday)\\b|\\bturns?\\s+\\d{1,3}\\b|\\b(?:forty|fifty|sixty|seventy|eighty|ninety)\\b|\\b\\d{2,3}(?:th|st|nd|rd)?\\b[^.;!?]{0,24}?\\b(?:next|this|on|in)\\s+(?:week|month|weekend|saturday|sunday|monday|tuesday|wednesday|thursday|friday)\\b|\\b(?:next|this)\\s+(?:week|month|weekend)\\b[^.;!?]{0,20}?\\b\\d{2,3}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_MILESTONE_TAIL(py_tuple(vec![py_str("{Thing} — that's a big one."), py_str("{Thing}. Are you doing anything for it?"), py_str("{Thing} — a round one deserves a fuss."), py_str("{Thing}. That's coming round fast.")]));
    s_A_SECOND_PERSON(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|also|plus|or|what\\s+about|how\\s+about)\\b[\\s,]*(?:my|our|your)\\s+(?P<who>[a-z][a-z '-]{1,24}?)\\s*(?:'s|s)?\\s*[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BIRTHDAY_TOLD(stdlib_call("re", "compile", &[py_str("\\b(?:birthday|bday|dob|date of birth)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLAN_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were)\\s+(?P<verb>[a-z]+ing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_TOLD(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[crate::memory::facts::g_MONTHS()])), &py_str("|")), &mcall(&py_str("|"), "join", &[crate::memory::facts::g_DAYS()])), &py_str("|\\d{1,2}(?:st|nd|rd|th)\\b|next\\s+(?:week|month|weekend)|this\\s+(?:week|month|weekend))")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POINTED_AT(stdlib_call("re", "compile", &[py_str("^(?:him|her|them|they|that one|this one|those two|the two of them|both of them|the pair of them)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_ALONE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|also|plus|or|what\\s+about|how\\s+about|same\\s+for|and\\s+same\\s+for|same\\s+as|and\\s+so\\s+does|and\\s+so\\s+is)\\b[\\s,]*)?(?P<det>my|our|the|a|an)\\s+(?P<what>[a-z][a-z '-]{1,24}?)\\s*[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHICH_ONE(stdlib_call("re", "compile", &[py_str("^(?:other|second|third|first|eldest|oldest|youngest|little|big|older|younger|new|baby|step|half|adopted|foster)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GROUP_ALONE(py_dict(vec![(py_str("kids"), py_str("children")), (py_str("children"), py_str("children")), (py_str("childs"), py_str("children")), (py_str("twins"), py_str("twins")), (py_str("boys"), py_str("boys")), (py_str("girls"), py_str("girls")), (py_str("sons"), py_str("sons")), (py_str("daughters"), py_str("daughters")), (py_str("parents"), py_str("parents")), (py_str("brothers"), py_str("brothers")), (py_str("sisters"), py_str("sisters")), (py_str("grandkids"), py_str("grandchildren")), (py_str("grandchildren"), py_str("grandchildren")), (py_str("grandparents"), py_str("grandparents"))]));
    s_A_POSITION(py_dict(vec![(py_str("youngest"), py_str("youngest")), (py_str("youngest one"), py_str("youngest")), (py_str("eldest"), py_str("eldest")), (py_str("eldest one"), py_str("eldest")), (py_str("oldest"), py_str("oldest")), (py_str("oldest one"), py_str("oldest")), (py_str("middle one"), py_str("middle one")), (py_str("middle"), py_str("middle one")), (py_str("second one"), py_str("second one")), (py_str("third one"), py_str("third one")), (py_str("little one"), py_str("little one")), (py_str("littlest"), py_str("little one")), (py_str("small one"), py_str("little one")), (py_str("other one"), py_str("other one")), (py_str("baby"), py_str("baby")), (py_str("new baby"), py_str("baby")), (py_str("little baby"), py_str("baby")), (py_str("other half"), py_str("other half")), (py_str("half"), py_str("other half")), (py_str("partner"), py_str("other half"))]));
    s_A_POSITION_MAYBE_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("other one"), py_str("second one"), py_str("third one"), py_str("middle one")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POSITION_LINES(py_dict(vec![(py_str("baby"), py_tuple(vec![py_str("{Who} — how many months?"), py_str("{Who} — boy or girl, and how old?"), py_str("{Who} — tell me about {them}.")])), (py_str("other half"), py_tuple(vec![py_str("{Who} — how long have you two been together?"), py_str("{Who}. What {do} {they} do?"), py_str("{Who} — tell me about {them}.")]))]));
    s_A_PERSON_ALONE_LINES(py_tuple(vec![py_str("{Who} — how old is {they}?"), py_str("{Who}. What {do} {they} get up to?"), py_str("{Who} — tell me a bit about {them}.")]));
    s_A_KNOWN_AGE_LINES(py_tuple(vec![py_str("{Who} — what is {they} into at the moment?"), py_str("{Who}. How is {they} getting on?")]));
    s_A_TWO_ALONE_LINES(py_tuple(vec![py_str("{Who} — what are they into at the moment?"), py_str("{Who}. How are they getting on?")]));
    s_A_GROUP_ALONE_LINES(py_tuple(vec![py_str("{Who} — how old are they?"), py_str("{Who}. How are they getting on?"), py_str("{Who} — tell me about them.")]));
    s_FRAMES(py_tuple(vec![py_str("{Thing} — noted."), py_str("Got it — {thing}."), py_str("{Thing}, good to know."), py_str("Noted: {thing}."), py_str("{Thing} — I'll remember that."), py_str("{Thing}, right — I've got that.")]));
    s_STILL_TO_DO(stdlib_call("re", "compile", &[py_str("\\b(?:still\\s+(?:have|got)|have\\s+got|i\\s+ve\\s+still\\s+got)\\s+((?:the|my|our|a|an|his|her|their|this|that)\\s+[a-z' ]{2,28}?)\\s+to\\s+(?:sort|do|deal|get|finish|pay|arrange|handle|see|take|cover|make|run|write|clean|buy|order|collect|ring|book|fix|wash|iron|pack|post|send|sign|check|tidy|move|email|print|return|visit|walk|feed|look)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ONE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:and|but|so|because|that|if|when|which)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_LIMIT_GOES_FIRST(py_tuple(vec![py_str("Then {thing} first, and the rest can wait."), py_str("Then do {thing} first and let the rest wait."), py_str("Then take {thing} first — the rest can wait until it is done."), py_str("Fair enough: {thing} first, and the rest of the list waits.")]));
    s__A_PLAN_WAS_GIVEN(stdlib_call("re", "compile", &[py_str("\\b(?:one thing|only that|pick|first|start with|begin with|next step|smallest)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_SOON(stdlib_call("re", "compile", &[py_str("\\b(?:tonight|tomorrow|this (?:afternoon|evening)|(?:on |next |this )?(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|(?:next |this |at the |the )weekend|next week|(?:on|by|before|until|till) the \\d{1,2}(?:st|nd|rd|th)\\b|\\b\\d{1,2}(?:st|nd|rd|th) of (?:january|february|march|april|may|june|july|august|september|october|november|december)|\\b(?:is|due|falls?|lands?) (?:on )?the \\d{1,2}(?:st|nd|rd|th)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_BEEN_AND_GONE(stdlib_call("re", "compile", &[py_str("\\b(?:saw|met|went|was|were|had|took|tried|left|rang|called|kicked|started|finished|ended|moved|got married|felt|ate|slept|woke|bought|sold|made|said|told|came|did|hurt|cried|stopped|began|broke|forgot|lost|won|worked|texted|visited|arrived|landed|been|this morning|yesterday|last (?:night|week|month|year|weekend|sunday|monday|tuesday|wednesday|thursday|friday|saturday)|since (?:six|seven|three|four|five|eight|nine|ten|eleven|twelve|two|one|half)|was in|used to|every (?:day|morning|night|week|weekend|month|tuesday|monday|sunday|friday|saturday)|usually|always|normally|like the \\d+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|am|was|were|be|been|being|have|has|had|got|gets|get|go|goes|going|gone|come|comes|coming|came|see|sees|seeing|visit|visits|visiting|fly|flies|flying|meet|meets|meeting|start|starts|starting|work|works|working|need|needs|want|wants|wanted|due|take|takes|taking|leave|leaves|leaving|arrive|arrives|arriving|stay|stays|staying|play|plays|shut|shuts|open|opens|starts|turning|turns|have to|has to|going in|going back|back|off|away|free|busy|dentist|vets?|doctor|gp|hospital|scan|appointment|haircut|interview|meeting|flight|train|bus|viewing|pickup|shift|party|match|wedding|birthday|exam|test|lesson|class|club|football|rugby|school|concert|payday|pay day|rent|bill|deadline|review|christening|funeral|service|checkup|check-up|surgery|midwife|health visitor)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b\\d{3,}\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING_TURNED_DOWN(stdlib_call("re", "compile", &[py_str("\\b(?:dont|don'?t|do not|doesnt|doesn'?t|didnt|didn'?t|wont|won'?t|wouldnt|wouldn'?t|cant|can'?t|cannot|not|never|no longer|no way)\\s+(?:\\w+\\s+){0,2}?(?:want|wants|wanted|need|needs|fancy|go|goes|going|have|has|having|do|doing|come|comes|coming|came|stay|staying|work|working|eat|eating|drink|drinking|see|seeing|happen|happening)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_JUST_A_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:worse|better|pale|ill|sick|poorly|off (?:it|his|her|their) food|not himself|not herself|up till|up until|much going on|busy|quiet|lovely|nice|awful|horrible)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_FAR(stdlib_call("re", "compile", &[py_str("\\b(?:in|of|next|this|by)\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december)\\b|\\bin (?:the )?(?:spring|summer|autumn|winter)\\b|\\bnext (?:month|year)\\b|\\blater (?:this|in the) (?:year|month)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_ASKED(stdlib_call("re", "compile", &[py_str("\\?|^(?:what|when|how|why|where|who|can|could|should|do|does|did|is|are|will|would|shall)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DAY_TO_LOOK_FORWARD_TO(stdlib_call("re", "compile", &[py_str("\\b(?:inquest|post[- ]mortem|autopsy|coroner|funeral|cremation|burial|bury|reading of the will|probate|hearing|tribunal|court|custody|bailiffs?|eviction|wake\\b(?!\\s*(?:up|at|early))|creditors?|bankrupt(?:cy)?|insolven(?:cy|t)|debt (?:meeting|hearing)|repossession|disciplinary|operation|surgery|biopsy|hospice|chemotherapy|radiotherapy|diagnosis|results|care assessment)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOON_TAIL(py_tuple(vec![py_str("{Thing}. Not long now, then."), py_str("{Thing} — that is soon, then."), py_str("{Thing}. Not far off now."), py_str("{Thing} — coming round quickly, then.")]));
    s_FAR_TAIL(py_tuple(vec![py_str("{Thing} — plenty of time to sort the rest."), py_str("{Thing}. That is one to look forward to."), py_str("{Thing} — nice to have something in the diary."), py_str("{Thing}. Plenty of time, anyway.")]));
    s_SORRY_TAIL(py_tuple(vec![py_str("{Thing}. That is a lot to have coming."), py_str("{Thing} — I hope it goes as well as it can."), py_str("{Thing}. That is a hard one to have ahead of you."), py_str("{Thing} — no wonder it is on your mind.")]));
    s__SAID_BACK(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FRAMES()) { let mut frame: Py = __item.clone(); __out.push(py_tuple(vec![mcall(&mcall(&geti(&mcall(&frame.clone(), "split", &[py_str("{"), Py::Int(1)]), &Py::Int(0)), "strip", &[]), "lower", &[]), mcall(&mcall(&geti(&mcall(&frame.clone(), "rsplit", &[py_str("}"), Py::Int(1)]), &op_neg(&Py::Int(1))), "strip", &[]), "lower", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s__SPACED_DASH(stdlib_call("re", "compile", &[py_str("\\s+(?:\\u2014|\\u2013|-)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORK_FRAMES(py_tuple(vec![py_str("{Thing} — what do you do there?"), py_str("{Thing}. How long have you been there?"), py_str("{Thing} — is it a good place to work?"), py_str("{Thing}. What's a normal day like?")]));
    s_WORK_PLACE(call_builtin("frozenset", &[py_tuple(vec![py_str("at"), py_str("in"), py_str("for")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_TAIL(py_tuple(vec![py_str("{Thing} — that's a proper routine."), py_str("{Thing}. That's a habit now."), py_str("{Thing} — keep that up and it'll stick.")]));
    s_GOING_TAIL(py_tuple(vec![py_str("{Thing} — how's it going?"), py_str("{Thing}, how long have you been at it?"), py_str("{Thing} — how are you finding it?")]));
    s_TRIP_TAIL(py_tuple(vec![py_str("{Thing} — have you been before?"), py_str("{Thing}. How long are you going for?"), py_str("{Thing} — that's something to look forward to."), py_str("{Thing}. What do you want to do while you're there?")]));
    s_OCCASION_TAIL(py_tuple(vec![py_str("{Thing} — are you looking forward to it?"), py_str("{Thing}. That'll come round fast."), py_str("{Thing} — that's something to look forward to.")]));
    s_PET_TAIL_ONE(py_tuple(vec![py_str("{Thing} — how long has it been with you?"), py_str("{Thing}. What's it like to live with?"), py_str("{Thing} — is it good company?")]));
    s_PET_TAIL_MANY(py_tuple(vec![py_str("{Thing} — do they get on?"), py_str("{Thing}. Who's in charge?"), py_str("{Thing} — how long have they been with you?")]));
    s_OCCASION_FLAT_TAIL(py_tuple(vec![py_str("{Thing}. That's a way off yet — and you can always leave early."), py_str("{Thing} — the run-up is usually worse than the day itself."), py_str("{Thing}. Not everything on the calendar has to be a highlight.")]));
    s_DIARY_WORDS(py_tuple(vec![py_str("dentist"), py_str("dental"), py_str("doctor"), py_str("doctors"), py_str("gp"), py_str("optician"), py_str("opticians"), py_str("clinic"), py_str("hospital"), py_str("scan"), py_str("x-?ray"), py_str("check\\s?-?up"), py_str("appointment"), py_str("interview"), py_str("viewing"), py_str("survey"), py_str("mot"), py_str("service"), py_str("haircut"), py_str("hairdresser"), py_str("barber"), py_str("physio"), py_str("physiotherapy"), py_str("chiropractor"), py_str("osteopath"), py_str("hygienist"), py_str("podiatrist"), py_str("midwife"), py_str("health visitor"), py_str("counselling"), py_str("vet"), py_str("blood test"), py_str("bloods"), py_str("jabs?"), py_str("injection"), py_str("hearing"), py_str("tribunal"), py_str("parents evening"), py_str("parents' evening"), py_str("sports day"), py_str("nativity"), py_str("recital")]));
    s_DIARY_THING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[g_DIARY_WORDS()])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIARY_TIME(stdlib_call("re", "compile", &[py_str("\\b\\d{1,2}(?:[:.]\\d{2})?\\s*(?:am|pm|o'?clock)\\b|\\b(?:at|from|by)\\s+\\d{1,2}\\b|\\b(?:morning|afternoon|evening|noon|midday|lunchtime|first thing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIARY_TAIL(py_tuple(vec![py_str("{Thing} — what time are you in?"), py_str("{Thing}. What time is it?"), py_str("{Thing} — is it first thing or later?")]));
    s_DIARY_TIME_TAIL(py_tuple(vec![py_str("{Thing} — is it far from you?"), py_str("{Thing}. Do you know where it is?"), py_str("{Thing} — somewhere you've been before?")]));
    s_MEETING_TAIL(py_tuple(vec![py_str("{Thing} — where are you meeting?"), py_str("{Thing}. Who else is going?"), py_str("{Thing} — what's the occasion?")]));
    s_MEETING_TIME_TAIL(py_tuple(vec![py_str("{Thing} — what time are you meeting?"), py_str("{Thing}. What time does it start?")]));
    s_COMING_TO_STAY(stdlib_call("re", "compile", &[py_str("\\b(?:coming|comes|visiting|staying|arriving|flying over|driving down|over for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VISIT_TAIL_ONE(py_tuple(vec![py_str("{Thing} — how long is {who} staying?"), py_str("{Thing}. Is {who} staying long?"), py_str("{Thing} — how long has {who} got with you?")]));
    s_VISIT_TAIL_MANY(py_tuple(vec![py_str("{Thing} — how long are they staying?"), py_str("{Thing}. Are they staying long?"), py_str("{Thing} — have they got long with you?")]));
    s_PLURAL_KIN(call_builtin("frozenset", &[mcall(&py_str("\n    parents brothers sisters grandparents grandkids grandchildren children kids twins folks\n    family relatives friends mates lads boys girls in laws in-laws\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|coming|comes|come|visiting|visits|visit|staying|stays|arriving|arrives|flying|driving|stopping|stops)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OCCASIONS(call_builtin("frozenset", &[py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("party"), py_str("parties"), py_str("birthday"), py_str("birthdays"), py_str("anniversary"), py_str("hen"), py_str("stag"), py_str("festival"), py_str("festivals"), py_str("concert"), py_str("gig"), py_str("gigs"), py_str("reunion"), py_str("graduation"), py_str("christening"), py_str("baptism"), py_str("barbecue"), py_str("bbq"), py_str("housewarming"), py_str("prom"), py_str("shower")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SHOWER(call_builtin("frozenset", &[py_tuple(vec![py_str("baby"), py_str("bridal"), py_str("wedding"), py_str("naming")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACES_WITH_THE(call_builtin("frozenset", &[py_tuple(vec![py_str("algarve"), py_str("cotswolds"), py_str("lakes"), py_str("highlands"), py_str("alps"), py_str("pyrenees"), py_str("cairngorms"), py_str("wight"), py_str("isle"), py_str("new forest"), py_str("peak district"), py_str("lake district"), py_str("states"), py_str("dales"), py_str("broads"), py_str("fens")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_BEFORE(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("our"), py_str("their")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_PLAN(call_builtin("frozenset", &[mcall(&py_str("went was were been came returned back"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:going|gonna|off|flying|fly|flies|flew|driving|heading|travelling|traveling|sailing|holiday|land|lands|landed|landing|arriv\\w+|touch(?:es)? down|getting in|got in|get in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DESTINATION(stdlib_call("re", "compile", &[py_str("\\b(?:to|off to|into)\\s+(?:the\\s+|a\\s+|an\\s+)?([a-z][a-z'\\u2019-]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ARRIVE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:land|lands|landed|landing|arriv\\w+|touch(?:es)? down|fly|flies|flew|getting in|got in|get in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DESTINATION_ARRIVAL(stdlib_call("re", "compile", &[py_str("\\b(?:in|at|into)\\s+(?:the\\s+|a\\s+|an\\s+)?([a-z][a-z'\\u2019-]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_TRIP(call_builtin("frozenset", &[mcall(&py_str("\n    bed sleep work school college uni university gym doctor doctors dentist hospital shops shop\n    town meeting interview court loo toilet hairdresser optician vet nursery see visit watch pick\n    collect drop get take it that this them there here him her us me you\n\n    appointment appointments scan scans checkup check-up clinic physio physiotherapy chiropractor\n    midwife blood test tests xray x-ray hearing hearings trial trials court courts tribunal prison prisons\n    jail jails cell custody remand probation solicitor lawyer\n\n    funeral funerals burial cremation wake wakes\n\n    be being been\n    tesco asda sainsbury sainsburys morrisons lidl aldi coop co-op iceland waitrose supermarket\n    office bank post\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DATE_AHEAD(stdlib_call("re", "compile", &[py_str("\\b(?:next|this|tomorrow|tonight|weekend|in (?:january|february|march|april|may|june|july|august|september|october|november|december|spring|summer|autumn|winter)|on (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|for (?:a|an|one|two|three|four|five|six|seven|eight|\\d+) (?:(?:more|whole|extra) )?(?:day|night|week|fortnight|month)s?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DREADED_OCCASION(stdlib_call("re", "compile", &[py_str("\\b(?:hate|hates|hated|dread|dreads|dreading|dreaded|can'?t face|cant face|not looking forward|dont want to go|don'?t want to go|dont fancy|don'?t fancy|cant be bothered|can'?t be bothered|a drag|boring|obligated)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_LONG_ALREADY(stdlib_call("re", "compile", &[py_str("\\bfor (?:a|an|one|two|three|four|five|six|seven|eight|\\d+)\\s*(?:day|night|week|fortnight|month|weekend)s?\\b|\\bfor the weekend\\b|\\bfor a few days\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT(stdlib_call("re", "compile", &[py_str("\\b(?:every (?:day|week|month|morning|evening|afternoon|weekend|other day)|every (?:mon|tues|wednes|thurs|fri|satur|sun)day|each (?:day|week|morning)|most days|most weeks|twice a (?:day|week|month)|once a (?:day|week|month)|on (?:a|an) (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:i(?:'ve| have) (?:been |just )?started|i(?:'ve| have) (?:taken up|got into|signed up)|i(?:'m| am) (?:trying|getting|working|building|writing|learning|practising|practicing)|i(?:'ve| have) been|started (?:again|back)|back (?:into|at) it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OBLIGED(stdlib_call("re", "compile", &[py_str("\\b(?:have to|has to|had to|must|got to|gotta|need to|needs to|keeps|kept|constantly|always|every time|end up|ends up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:cry|cries|cried|crying|tears|sob\\w*|weep\\w*|panic\\w*|anxious|anxiety|depress\\w*|numb|shak\\w*|nightmares?|insomnia|miserable|upset|self[- ]harm\\w*|hurt(?:ing)? myself|feel(?:s|ing)? (?:low|down|awful|rubbish|empty|worthless)|shout\\w* at|yell\\w* at|scream\\w* at|swear\\w* at|hit(?:s|ting)? (?:my|the|him|her|them)|sick|sickness|vomit\\w*|throwing up|threw up|throw up|nausea|nauseous|diarr\\w*|migraine\\w*|angry|furious|rage|lash(?:ing)? out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_BAD(stdlib_call("re", "compile", &[py_str("\\b(?:badly|not|never|can'?t|cant|won'?t|wont|too much|too many|stop|stopped|give up|gave up|fail\\w*|struggl\\w*|hard|tough|shattered|exhausted|knackered|late|behind|broke)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BUDGET_FRAMES(py_tuple(vec![py_str("{Thing} — I'll keep any suggestions inside it: fixed bills and food first, then savings."), py_str("{Thing}, then — I'll stick to that: fixed bills and food first, then a set amount to savings."), py_str("{Thing} — noted, and I'll work within it: fixed bills and food first, then savings on payday.")]));
    s_PERIOD(stdlib_call("re", "compile", &[py_str("\\b(?:(?:a|an|per|each|every)\\s+(?:day|week|fortnight|month|year)\\b|weekly|monthly|fortnightly|yearly|annually|a month|in a month|a week|in a week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPAN_OF_TIME(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:the\\s+(?:rest\\s+of\\s+)?(?:week|fortnight|month|year|day)\\b|this\\s+(?:week|month|year)\\b|next\\s+(?:week|month|year)\\b|last\\s+(?:week|month|year)\\b|the\\s+rest\\b|a\\s+(?:week|fortnight|month|year|day)\\b|an?\\s+hour\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:\\d[\\d,.]*|a few|a couple of|about)\\s*(?:pounds?|quid|euros?|dollars?)?\\s*(?:budget\\s+)?for\\s+(?:a|an|the|my|our|some|new)\\b|\\bbudget\\s+for\\s+(?:a|an|the|my|our|some|new)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BUYING(stdlib_call("re", "compile", &[py_str("\\b(?:buy|buys|buying|present|presents|gift|gifts|shopping for|looking for a|looking for some|looking for new|should i get|shall i get|which one|get (?:my|him|her|them|myself)|spend on|worth getting|what to get|need (?:a|some|new|another)\\b|getting (?:a|an|one|some|new)\\b(?!\\s*(?:lot|bit|break|rest|handle|hold|sense|grip|kick|chance|look|few|couple|little|better|worse|grief|telling|hearing|going|ready))|after (?:a|an|some)\\b(?!\\s*(?:lot|bit|advice|help|idea|good|thought|while|time|break|rest|sense|point))|want (?:a|an|some|one|to get|to buy)\\b(?!\\s*(?:lot|bit|break|rest|handle|hold|sense|grip|kick|chance|look|few|couple|little))|planning (?:a|an|some|my|our)\\b|moving (?:into|in|to|out|house|home)\\b|redec\\w+|renovat\\w+|refurbish\\w*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:money|loan|loans|pension|pensions|mortgage|mortgages|credit card|credit cards|overdraft|debt|debts|savings|insurance|benefits|universal credit|job|jobs|salary|wage|wages|tax|budget|budgets|plan|planner|finances?|rent|bills?|groceries)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPENDING_FRAMES(py_tuple(vec![py_str("{Thing} — I'll keep what I suggest inside it."), py_str("{Thing} — good, and everything I suggest will be within it."), py_str("{Thing} — noted, and I'll stay under that with what I recommend.")]));
    s_FOOD_FRAMES(py_tuple(vec![py_str("{Thing} — I'll keep that in mind when I suggest food."), py_str("{Thing}, good to know — I'll plan around it."), py_str("{Thing} — noted, and I won't suggest anything that goes against it.")]));
    s_DRINK_FRAMES(py_tuple(vec![py_str("{Thing} — I'll keep that in mind for anything I suggest."), py_str("{Thing}, good to know — I'll plan around it."), py_str("{Thing} — noted, and I won't suggest a drink that goes against it.")]));
    s_CONDITION_FOOD(py_dict(vec![(py_str("coeliac"), py_str("gluten")), (py_str("celiac"), py_str("gluten")), (py_str("gluten intolerant"), py_str("gluten")), (py_str("lactose intolerant"), py_str("dairy"))]));
    s_CONDITION_FRAMES(py_tuple(vec![py_str("{Thing} — noted, and I'll keep {food} out of anything I suggest."), py_str("{Thing}, good to know — I'll plan around the {food}."), py_str("{Thing} — right, so nothing with {food} in it.")]));
    s_OTHER_LIMIT_FRAMES(py_tuple(vec![py_str("{Thing} — I'll keep that in mind for anything I suggest."), py_str("{Thing}, good to know — I'll work around it."), py_str("{Thing} — right, so that is off the table for them.")]));
    s_LIMIT_FRAMES(py_dict(vec![(py_str("move"), py_tuple(vec![py_str("{Thing} — so anything I suggest stays short and flat, with somewhere to sit."), py_str("{Thing}, then. Nothing I put to you needs hours on your feet."), py_str("{Thing} — noted, and I'll keep the days gentle and close to a seat.")])), (py_str("money"), py_tuple(vec![py_str("{Thing} — so I'll keep what I suggest to what costs nothing."), py_str("{Thing}, then. Nothing that needs paying up front."), py_str("{Thing} — noted, and nothing pricey in anything I put to you.")])), (py_str("move_stay"), py_tuple(vec![py_str("{Thing} — so it is the downstairs room or a bed moved down, a chair in the shower rather than a step over the bath, and the outings one at a time."), py_str("{Thing}, then — book the bed that needs no stairs and keep the week's outings short and near a seat.")])), (py_str("other"), py_tuple(vec![py_str("{Thing} — noted, and I'll keep anything I suggest inside it."), py_str("{Thing}, then — I'll plan around it and keep what I suggest inside it.")]))]));
    s_A_STAY_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:coming|comeing|come|comes|staying|stays|stay|stopping|to stay|visiting|visit)\\b|\\b(?:we|i)(?:'re| are| am)?\\s+having\\s+(?:people|family|guests|my|her|his)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRIP_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:going to|off to|heading to|away to|flying to|driving to|trip to|holiday|weekend away|going away|staying near|in (?:june|july|august|september|october|november|december|january|february|march|april|may))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STAY_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:mum|mam|mom|mother|dad|father|parents|gran|grandma|grandad|granny|nana|nan|grandpa|grandparents|sister|brother|aunt|uncle|cousin|in laws|in-laws|mother in law|father in law|friend|friends|guest|guests|visitors|family)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LIMIT_ONLY(call_builtin("frozenset", &[py_tuple(vec![py_str("people"), py_str("relations"), py_str("other"), py_str("others"), py_str("relation"), py_str("detail")])]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENS_WITH_AN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|which|how|where|when|who|why|should|shall|can|could|would|will|do|does|did|is|are|was|were|any|anyone|anything|tell me|give me|suggest|recommend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIKES_FRAMES(py_tuple(vec![py_str("{Thing} — what is it you like about it?"), py_str("{Thing} — what is it about it that gets you?"), py_str("{Thing}. Good to know — what do you like best about it?"), py_str("{Thing} — that goes in the keep pile. I'll remember it.")]));
    s_A_LIKE_OF_ONE_THING(stdlib_call("re", "compile", &[py_str("^(?!(?:.*\\b(?:and|but|because|so|though|although|when|if|which|that)\\b)).{2,40}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIKE_ECHO(stdlib_call("re", "compile", &[py_str("^(?:you|we)\\s+(?:really\\s+|absolutely\\s+|just\\s+|still\\s+|also\\s+|do\\s+|both\\s+)?(?:love|like|adore|enjoy|prefer)\\s+(?P<thing>[^,;]+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIKE_WITH_TWO_HALVES(stdlib_call("re", "compile", &[py_str("\\b(?:and|but|because|so|though|although|when|if|which|that|than|too)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_LIKED(stdlib_call("re", "compile", &[py_str("\\b(?:girl|boy|man|woman|guy|bloke|lad|lass|person|someone|somebody|people|they|them|him|her|his|hers|theirs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ABILITY(stdlib_call("re", "compile", &[py_str("\\b(?:can'?t|cannot|can not|isn'?t able to|is not able to|not able to|only manage)\\s+(?:walk|climb|manage|stand|sit)\\b|\\b(?:can'?t|cannot|can not|isn'?t able to|is not able to|not able to|only manage)\\s+(?:do|manage|get\\s+up|get\\s+up\\s+and\\s+down|get\\s+up\\s+the|use|cope\\s+with)\\s+(?:the\\s+|a\\s+|any\\s+)?(?:more\\s+)?(?:stairs?|steps?|hills?|walks?|walking|climbing|standing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_TURNS(Py::Int(3));
    s_WANTS_IT(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:want|wanna|need|'?d like|'?d rather|would like|would rather)\\s+to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_BACK_WORDS(Py::Int(7));
    s_LIMIT_AFTER_THE_LINE(py_dict(vec![(py_str("move"), py_tuple(vec![py_str("So anything I suggest stays short and flat, with somewhere to sit."), py_str("Then nothing I put to you needs hours on your feet, and there is a seat at the end of it.")])), (py_str("money"), py_tuple(vec![py_str("So I'll keep what I suggest to what costs nothing."), py_str("Then nothing I put to you needs paying up front.")])), (py_str("move_stay"), py_tuple(vec![py_str("Then it is the downstairs room or a bed moved down, a chair in the shower rather than a step over the bath, and the outings one at a time."), py_str("Then book the bed that needs no stairs, and keep the week's outings short and near a seat.")]))]));
    s_UP_CLOSED(py_str("(?!\\s+(?:the|a|an|it|this|that|these|those|my|his|her|their|your|our|into|over|between)\\b)"));
    s_LEFT_CLOSED(py_str("(?=\\s*(?:[,.!?;]|$|\\b(?:for|after|last|in|on|at|with|when|because|so|and|since|about|yesterday|today|tonight)\\b))"));
    s_SEPARATION(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:left me|leaving me)"), &g_LEFT_CLOSED()), &py_str("|\\b(?:walked out on|dumped me|cheated on me)\\b|\\b(?:broke up|split up)")), &g_UP_CLOSED()), &py_str("|\\bdivorce\\w*\\b|\\b(?:he|she|they|my (?:wife|husband|partner|boyfriend|girlfriend|ex))\\b[^.!?]{0,16}\\bmoved out\\b|\\b(?:we|my \\w+ and i) separated\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEAVY_TURNS(Py::Int(3));
    s__NOT_A_BODY_PART(py_str("(?<!\\b(?:my|his|her|their|your|our)\\s)(?<!\\b(?:the|a)\\s)(?<!\\b(?:knee|back|leg|legs|arm|arms|head|shoulder|shoulders|neck|foot|feet|hip|hand|hands|stomach|tooth|teeth|ear|ears|eye|eyes|chest|ankle|wrist|joint|joints|muscle|muscles)\\s)"));
    s__BULLIED(stdlib_call("re", "compile", &[py_str("\\b(?:bull(?:y|ies|ied|ying))\\b|\\bpick(?:s|ed|ing)?\\s+on\\b|\\bpicked\\s+on\\b|\\bpicks\\s+on\\b|\\bmean\\s+to\\s+(?:me|him|her|them|us|my\\b)|\\bpicked\\s+on\\s+(?:me|him|her|them|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HURTS_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|my\\s+\\w+|his\\s+\\w+|her\\s+\\w+|the\\s+\\w+)\\s+(?:(?:hit|hits|hit(?:ting)?|punch\\w*|kick\\w*|shov\\w*|slap\\w*|smack\\w*|threaten\\w*)\\s+(?:me|him|her|them|us|the\\s+(?:kids|children|baby|little\\s+one))\\b|(?:beat|beats|beating)\\s+(?:me|him|her|them|us|the\\s+(?:kids|children|baby|little\\s+one))\\b(?!\\s*(?:at|in|on)\\s+(?:the\\s+|a\\s+|his\\s+|her\\s+|their\\s+)?[a-z]))|\\b(?:shout|shouts|shouting|swear|swears|swearing|yell|yells|yelling)\\s+(?:at|at)\\s+(?:me|him|her|them|us|the\\s+(?:kids|children|baby|little\\s+one))\\b|\\b(?:is|are|was|were|being)\\s+horrible\\s+to\\s+(?:me|him|her|them|us)\\b|\\b(?:he|she|they|my\\s+\\w+|his\\s+\\w+|her\\s+\\w+)\\s+(?:was|is|were|are|has\\s+been|had\\s+been|got)\\s+violent\\b|\\b(?:violent|violence)\\s+(?:to|towards|with|at)\\s+(?:me|him|her|them|us)\\b|\\b(?:was|were|am|is|being|been)\\s+harass\\w*\\s+(?:at|on|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_WAS_VIOLENT(stdlib_call("re", "compile", &[py_str("^\\s*(?!(?:the|a|an|it|there|this|that|these|those|my|his|her|their|your|our|we|i|you|he|she|they)\\b)[a-z][a-z'\\u2019-]*\\s+(?:was|is|were|are|has\\s+been|had\\s+been|got)\\s+violent\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEAVY_FRAMES(py_tuple(vec![py_str("{Thing} — I'm sorry it's all coming at once."), py_str("Got it — {thing}. That's a hard week."), py_str("{Thing}. I'm sorry there's so much on your plate.")]));
    s_STILL_FRAMES(py_tuple(vec![py_str("{Thing} — got it. And I'm sorry it's a hard time."), py_str("{Thing}. Noted — and I'm sorry you're having a hard week."), py_str("{Thing} — I've got that. Still sorry for what you're going through.")]));
    s_LEAD_IN(py_tuple(vec![py_str("hi"), py_str("hiya"), py_str("hello"), py_str("hey"), py_str("so"), py_str("well"), py_str("oh"), py_str("and"), py_str("also"), py_str("ok"), py_str("okay"), py_str("right"), py_str("because"), py_str("cos"), py_str("coz"), py_str("cause"), py_str("cuz"), py_str("because of"), py_str("cos of"), py_str("yeah"), py_str("yes"), py_str("actually"), py_str("anyway"), py_str("btw"), py_str("by the way"), py_str("just so you know"), py_str("quick one"), py_str("sorry"), py_str("haha"), py_str("lol"), py_str("oh and"), py_str("for the record"), py_str("one thing"), py_str("that reminds me"), py_str("thanks"), py_str("thank you"), py_str("cheers"), py_str("ta"), py_str("cool"), py_str("sorry one more thing"), py_str("one more thing"), py_str("one last thing"), py_str("last thing"), py_str("nice"), py_str("great"), py_str("yep"), py_str("yup"), py_str("mmm"), py_str("haha"), py_str("to be honest"), py_str("to be fair"), py_str("if im honest"), py_str("if i am honest"), py_str("if im being honest"), py_str("honestly"), py_str("tbh"), py_str("to tell the truth"), py_str("truth be told"), py_str("not going to lie"), py_str("not gonna lie"), py_str("ngl"), py_str("i wont lie"), py_str("i have to say"), py_str("i must admit"), py_str("i will say"), py_str("ill be honest"), py_str("to be truthful")]));
    s_WANTS_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|how|where|whereabouts|when|who|why|whats|hows|whos|wheres)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GREETING(stdlib_call("re", "compile", &[py_str("^\\s*(?:hi|hiya|hello|hey|morning|good morning|evening|yo|howdy)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:die|died|dies|dying|death|dead|kill|killed|suicide)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_NIGHTS(stdlib_call("re", "compile", &[py_str("\\b(?:(?:i|we)\\s+(?:work|works|do|does|am on|are on|am doing|are doing|'m on|'re on)\\s+(?:the\\s+)?night(?:s|\\s+shift(?:s)?)?\\b|night shifts?\\b|\\bon nights\\b|\\bnights this week\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EARLY_RISE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:get up|get|start|leave|wake up|am up|are up|have to be up|'m up)\\s+(?:at|by|for)\\s+(?:\\d{1,2}(?:[:.]\\d{2})?\\s*(?:am|pm)?|four|five|six|seven|half (?:four|five|six))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LATE_FINISH(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:finish|finishes|get home|am back|'m back)\\s+(?:at|around|about|by)\\s+(?:\\d{1,2}(?:[:.]\\d{2})?\\s*(?:am|pm)?|nine|ten|eleven|twelve|half (?:nine|ten|eleven))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LONG_SHIFT(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,2}|twelve|ten|eleven)\\s*[- ]?hour (?:shift|shifts|days?)\\b|\\blong (?:days|shifts)\\b|\\bwork(?:ing)? long hours\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHIFTS_ALL_OVER(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the)\\s+(?:shifts?|rota|hours)\\s+(?:is|are|change|changes|move|moves|rotate|rotates|all over|all over the place|everywhere|different|random|mad)\\b|\\bshifts? (?:all over|all over the place|everywhere)\\b|\\bwork(?:ing)? two jobs\\b|\\bsecond job\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORK_FROM_HOME(stdlib_call("re", "compile", &[py_str("\\bwork(?:ing)? from home\\b|\\bwork from home\\b|\\bwfh\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NIGHT_TAIL(py_tuple(vec![py_str("Nights turn the week upside down. The sleep in the day is never quite the same as the sleep at night, and that is the part people underrate."), py_str("Nights, then. It is a different world at three in the morning, and not many people see it."), py_str("Nights are hard on the body clock. Whatever makes the sleep after a shift work is worth protecting.")]));
    s_EARLY_TAIL(py_tuple(vec![py_str("That is an early start. The night before is where those mornings are won or lost."), py_str("Up at that hour, the whole day is done by lunchtime. It is the evening that has to be guarded."), py_str("That is a hard hour to be up, and it gets easier once the going to bed moves with it.")]));
    s_LATE_TAIL(py_tuple(vec![py_str("Finishing that late means the evening has gone by the time you are out. The wind-down is the bit worth protecting."), py_str("That is a long day by the time you are home. The last hour of it is the one to keep for yourself."), py_str("Home at that hour, the day turns into the next one. Best to have the food and the quiet already waiting.")]));
    s_LONG_TAIL(py_tuple(vec![py_str("Those are long days. Whatever you do for yourself has to be small enough to happen when you are wrecked."), py_str("Long shifts like that leave nothing of the day. The food and the sleep are the parts that actually matter."), py_str("That is a lot of hours. The days off are not a bonus, they are the half of the week that makes the rest work.")]));
    s_SPREAD_TAIL(py_tuple(vec![py_str("Shifts that move make the week hard to plan. The few fixed things - sleep, one day off, a meal at a set time - are what hold it together."), py_str("A rota like that means no two weeks are the same. Worth putting the things you care about in before the rota comes out, not after."), py_str("That kind of week is hard to build anything on. Whatever is regular in it is doing a lot of the work.")]));
    s_HOME_TAIL(py_tuple(vec![py_str("Working from home, the day has no edges unless you put them in. A walk at the end of it stands in for the commute."), py_str("That suits a lot of people and it eats a lot of people. The bit that helps is stopping at a time rather than when the work runs out."), py_str("Home all day, the work and the rest end up in the same room. A door you can shut, or a walk out of it, does more than any system.")]));
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|but|ok|okay|well|also|plus)\\s+)*(?:should|shall|can|could|do|does|did|is|are|was|were|will|would|may|might|am|have|has|any|what|whats|which|how|hows|where|wheres|whereabouts|when|who|whos|why)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_HOURS(stdlib_call("re", "compile", &[py_str("\\b(?:cant sleep|can'?t sleep|not sleeping|insomnia|exhaust\\w*|knackered|shattered|burn(?:t|ed) out|depress\\w*|anxi\\w*|panic|stress\\w*|fed up|sick of|had enough|hate|hates|quit|quitting|resign\\w*|sacked|fired|redundant|laid off|losing my job|lost my job|overtime|pay|paid|wage|wages|money|salary|boss|manager|colleague|interview|contract|holiday|annual leave|tips|advice|how do i|how can i|what should|should i|help me|pregnant|baby|kids?|child|school run|uni|college|course|exam)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_DETAIL(stdlib_call("re", "compile", &[py_str("^(?:and |oh |also |so |but )?(?:(?:its|it's|it is|thats|that's|that is|were|we're|i'm|im)\\s+(?:(?:on|in)\\s+)?|(?:on|in)\\s+|and\\s+)(?P<when>tomorrow|tonight|today|this (?:weekend|week|morning|evening|afternoon)|next (?:week|month|weekend|year)|monday|tuesday|wednesday|thursday|friday|saturday|sunday|(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:days?|weeks?|months?)|a (?:few|couple of) (?:days|weeks)|a couple of (?:days|weeks))(?:\\s+(?:away|left|to go))?\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_TODAY(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|this (?:morning|afternoon|evening))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_TOMORROW(stdlib_call("re", "compile", &[py_str("\\btomorrow\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_DAYS(stdlib_call("re", "compile", &[py_str("\\b(?:this (?:week|weekend)|monday|tuesday|wednesday|thursday|friday|saturday|sunday|a few days|a couple of days)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten)\\s+(days?|weeks?|months?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10))]));
    s_WHEN_TODAY_TAIL(py_tuple(vec![py_str("{When}, so the first of it happens now: a small thing done today beats a better one that waits for a quiet hour."), py_str("There is no day left to work up to it in. {When} is the day, so start at the first step."), py_str("{When} is already here, which makes the part of it you were saving for later today's job.")]));
    s_WHEN_TOMORROW_TAIL(py_tuple(vec![py_str("Tomorrow, so today is the working day: the first step now beats a good plan for the morning."), py_str("That leaves today for it. Do the part you would least like to be doing at the last minute."), py_str("Tomorrow, then. Whatever in that needs two sittings has to have the first of them today.")]));
    s_WHEN_DAYS_TAIL(py_tuple(vec![py_str("{When} is one step a day rather than all of it at once: pick the first step and give it today."), py_str("{When}, so count the days and put a step in each one. The last day is for checking, not starting."), py_str("{When} only fits the whole of it if each day carries a bit of it: start today, not on the last one.")]));
    s_WHEN_ROOMY_TAIL(py_tuple(vec![py_str("{When} is enough time if you spread it: one small bit a week, and the last week only checking."), py_str("{When} gives you room to do it properly. Start with the part you would least like to get wrong.")]));
    s_TIME_WORD(py_str("(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty)"));
    s_CLOCK_DETAIL(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^(?:and |oh |also |so |but )?(?:(?:its|it's|it is|thats|that's|that is|were|we're|the\\s+[a-z'-]+\\s+is)\\s+)?(?:at\\s+|for\\s+)?(?P<when>\\d{1,2}[:.]\\d{2}\\s*(?:am|pm|a\\.m\\.|p\\.m\\.)?|\\d{1,2}\\s*(?:am|pm|a\\.m\\.|p\\.m\\.)|\\d{1,2}\\s*o'?\\s?clock|(?:half|quarter)\\s+(?:(?:past|to)\\s+)?"), &g_TIME_WORD()), &py_str("|(?:five|ten|twenty|twenty[- ]five)\\s+to\\s+")), &g_TIME_WORD()), &py_str("|")), &g_TIME_WORD()), &py_str("\\s+(?:o'?\\s?clock|am|pm|a\\.m\\.|p\\.m\\.|thirty|fifteen|forty[- ]five)|\\d{1,2}\\s+(?:thirty|fifteen|forty[- ]five))\\s*[.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_AT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and |oh |also |so |but )?(?:(?:its|it's|it is|thats|that's|that is|the\\s+[a-z'-]+\\s+is)\\s+)?(?:at|for)\\s+(?P<when>(?:\\d{1,2}[:.]\\d{2}|\\d{1,2}|"), &g_TIME_WORD()), &py_str(")(?:\\s*(?:am|pm))?)\\s*[.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_MORNING_TAIL(py_tuple(vec![py_str("{At} on the day, so be there twenty minutes early and the nerves have somewhere to sit while you wait."), py_str("{At}, so the day is built around it, and the bit that wants you sharp belongs before you set off rather than after."), py_str("{At}, so there is enough of the morning to get ready without rushing it, which is usually the part that decides how it goes.")]));
    s_CLOCK_AFTER_TAIL(py_tuple(vec![py_str("{At}, so the morning is free for the part that needs you sharp, and nothing else has to fit round it."), py_str("{At} on the day: get the other jobs done in the morning so you go into it with nothing else on your mind."), py_str("{At}, so the morning is where the part that wants a steady head goes, and the afternoon is clear for it.")]));
    s_CLOCK_EVENT(stdlib_call("re", "compile", &[py_str("\\b(?:dentist|dental|optician|hygienist|doctor|gp|surgery|vet|clinic|hospital|hearing|interview|appointment|scan|exam|test|checkup|check-up|consultant|physio|physiotherapy|podiatrist|chiropodist|midwife|blood\\s+test|presentation)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_FLAT(stdlib_call("re", "compile", &[py_str("\\b(?:meeting|viewing|shift|class|lesson|lecture|rehearsal|assessment|haircut|tattoo|collection|pickup|lunch|dinner|breakfast|drinks|supper)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK_FLAT_TAIL(py_tuple(vec![py_str("{At}, then, and the rest of the day arranges itself round it rather than the other way round."), py_str("{At} on the day, so what comes before it is short and what comes after it is free."), py_str("{At}, so the day is built round it and nothing else has to be squeezed in before it.")]));
    s_CLOCK_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20))]));
    s_STEP_OPENS(stdlib_call("re", "compile", &[py_str("^\\s*(?:first|firstly|start|begin|own|book|name|skip|put|blot|pour|turn|take|check|keep|let|leave|get|add|cut|fill|soak|wash|rinse|call|ask|write|print|plan|split|break|set|read|look|watch|listen|tell|say|try|do|go|sit|spend|give|use|make|run|buy|pick|wear|choose|cook|boil|roast|fry|steam|bake|rest|chill|freeze|store|measure|weigh|clean|wipe|scrape|open|hold|walk|drive|move|cover|line|grease|trim|water|feed|brush|email|text|ring|phone|budget|save|avoid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WOULD_HAVE_BEEN(stdlib_call("re", "compile", &[py_str("\\bwould(?:\\s*have|'ve)?\\s+(?:\\w+\\s+){0,2}been\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY(stdlib_call("re", "compile", &[py_str("\\b(?:pain\\w*|ache\\w*|aching|hurts?|hurting|illness|poorly|unwell|fever\\w*|rash\\w*|swell\\w*|swollen|infec\\w*|antibiotic\\w*|blood\\w*|choked?|choking|swallow\\w*|dizz\\w*|nausea\\w*|vomit\\w*|diarrhoea|migraine\\w*|sore|cramp\\w*|symptom\\w*|injur\\w*|wound\\w*|fractur\\w*|surger\\w*|operation|a&e|111|999|seiz\\w*|sezure|breathless\\w*|x-ray|xray)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE_WORDS(call_builtin("frozenset", &[mcall(&py_str("faint faintish fainting woozy dizzy giddy groggy wobbly shaky\nclammy sweaty queezy queasy nauseous nauseated bilious sickly bunged sniffly sniffles sniffing\ncroaky hoarse congested sneezing lightheaded\npale pallid ashen grey gray greyish grayish mottled blotchy spotty puffy bloated flushed\nfeverish shivery tender husky snotty"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE_PHRASES(call_builtin("frozenset", &[py_tuple(vec![py_str("light headed"), py_str("bilious"), py_str("bunged up"), py_str("blocked up")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE_TAIL(call_builtin("frozenset", &[mcall(&py_str("today tonight now still again though honestly already here\nactually really so very absolutely bloody thanks up and"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BLEED(stdlib_call("re", "compile", &[py_str("\\bbleed\\w*\\b(?![^.!?]{0,12}\\b(?:dry|radiator|boiler|pipe\\w*)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_INTRO(stdlib_call("re", "compile", &[py_str("^\\s*(?:i(?:'m| am)|im|my name(?:'s| is))\\s+[A-Z][a-z'\\u2019-]+\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MEET_GREET(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:nice|good|great|pleased|lovely|glad|happy|grand)\\s+to\\s+(?:meet|see|hear\\s+from)\\s+(?:you|u|ya)\\b|(?:good|great|lovely|nice)\\s+to\\s+have\\s+you\\b|how\\s+(?:are|r)\\s+(?:you|u|ya)\\b|how(?:'s|s|\\s+is)\\s+(?:it\\s+going|things|tricks|you\\s+doing|your\\s+day)\\b|how\\s+do\\s+you\\s+do\\b|long\\s+time\\s+no\\s+see\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECT(py_str("(?:i|i'm|im|i've|i'd|we|we're|my|our|mine|she|she's|he|he's|it|it's|they|they're|his|her|its|their)"));
    s_STOP(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\s*(?:,\\s|;|\\.\\s|$|\\b(?:because|so|but)\\b|\\b(?:and|then)\\b\\s+(?="), &g_SUBJECT()), &py_str("\\b))\\s*")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[A-Za-z0-9'\\u2019-]+")]).unwrap_or_else(|e| exc::raise(e)));
    s_POLITE(stdlib_call("re", "compile", &[py_str("[,\\s]+(?:thanks|thank you|cheers|ta|please|thx)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_OFF(crate::memory::filler::g_TAIL_OFF());
    s_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|am|im|'s|'re|'m|was|were|has|have|had|'ve|ive|can|will|'ll|need|needs|want|wants|live|lives|work|works|study|studies|love|loves|like|likes|prefer|prefers|go|goes|going|off to|plan|plans|planning|train|training|learn|learning|move|moving|fly|flying|getting|got|adopted|allergic|spend|save|saving)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:being|bullied|bullies|bully|teas\\w+|never|always|keeps|kept|refuses|refused|stopped|stops|quit|hates?|hated|hit|hits|shout\\w*|yell\\w*|scream\\w*|argu\\w*|row|won'?t|doesn'?t|didn'?t|isn'?t|can'?t|cannot|failed|failing|behind|ignores?|ignored|blames?|blamed|lost|losing|dead against|just want|just need|driving me|drives me|doing my head in|fed up|sick of|had enough|tether|drives? me mad)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__EMPTY(call_builtin("frozenset", &[mcall(&py_str("a an the and or but if so then this that these those it its it's i i'm im me my\nwe we're us our you your he she they them his her their is are was were be been being am 's 're\nof to in on at for with from about as not no do does did have has had will would can could just\nreally very too also still now"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__FEELING_WORDS(py_str("worried|frightened|scared|terrified|afraid|fear\\w*|dread\\w*|anxious|panic\\w*|overreacting|upset|angry|annoyed|sad|lonely|guilty|embarrassed|ashamed|stressed|exhausted|knackered|weary|miserable|cry|cried|crying|tears?|sob\\w*|moan\\w*|whinge\\w*|whine|rant\\w*|vent\\w*"));
    s_A_FEELING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &g__FEELING_WORDS()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOOD_MOOD(stdlib_call("re", "compile", &[py_str("\\b(?:in (?:a|an) (?:very )?(?:good|great|fine) mood|good mood|great mood|feeling good|feel good|feeling great|feel great|cheerful|cheery|in good spirits)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_EVENT(stdlib_call("re", "compile", &[py_str("\\b(?:job|promotion|baby|exam|test|interview|offer|house|flat|course|degree|wedding|engagement|engaged|pregnant|expecting|won|passed|accepted|hired|booked|sold|bought|holiday|graduated|qualified|retired)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLEAR(stdlib_call("re", "compile", &[py_str("\\b(?:all clear|(?:came|come|comes) back clear|(?:scan|results?|tests?) (?:was|is|were|are) clear|given? the all clear)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RESULT(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they|my \\w+|our \\w+)\\s+(?:(?:just|finally|only|also|now)\\s+)?(?:got|gotten|finished|finishes|started|starts|passed|passes|won|wins|landed|booked|sold|bought|completed|handed in|graduated|qualified|retired)\\b|\\b(?:went|gone) (?:well|great|brilliant|fine)\\b|\\bout of (?:the )?hospital\\b|\\bhome from (?:the )?hospital\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOVED(stdlib_call("re", "compile", &[py_str("\\b(?:moved|moving|move) (?:in|into|to|house|out|abroad|back|country)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPLAINT(stdlib_call("re", "compile", &[py_str("\\bhad a (?:go|dig|pop) at\\b|\\b(?:gave|given|handed) me a (?:ticket|fine|bollocking)\\b|\\b(?:got|get|gets|getting|received) (?:a |an |the |my )?(?:parking |speeding |fixed penalty )?(?:ticket|penalty|caution|warning|bollocking)\\b|\\b(?:got|get|gets|getting|received|gave me) (?:a |an |the )?fine\\b(?!\\s+(?:view|views|day|time|weather|meal|bit|line|mess|idea|point|thing|way|year))|\\b(?:parking|speeding) ticket\\b|\\bclamped\\b|\\b(?:a|the) telling off\\b|\\btold (?:me )?off\\b|\\ba bollocking\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_DETAIL(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:bye|goodnight|good night|catch (?:you|up)|speak (?:later|soon)|talk later|see you|night then|anyway|back to|as i said|like i said|nothing much|not much|can'?t complain|same old|same as ever|not a lot|how about you|you\\?|fair enough|sounds good|(?:i'?ll|i will|ill) (?:try|give) (?:that|it)|will do|no worries|nice one|same to you|not bad|all good|want(?:ed)? to (?:say|ask|tell)|how (?:do|should) i put it|\\bmeant\\b|\\bsoz\\b|\\bi (?:said|told you|mentioned|already|just said)\\b|\\bas i said\\b|\\blike i said\\b|"), &gattr(&g_COMPLAINT(), "pattern")), &py_str("|")), &g__FEELING_WORDS()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_YES_NO(stdlib_call("re", "compile", &[py_str("^(?:no|nope|nah|yes|yeah|yep)\\b(?=[,.!]|\\s+(?:please|thanks|thank you|mate|then)\\b|$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ANSWER_OPENER(stdlib_call("re", "compile", &[py_str("^(?:ok|okay|course|right)\\b[ ,]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_MIDWAY(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|do|does|did|can|could|should|would|will|shall|have|has)\\s+(?:it|that|this|they|i|we|you|he|she)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKED_FOR(stdlib_call("re", "compile", &[py_str("^(?:any|anything|something|someone|somewhere|nowhere|nothing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIGIT(stdlib_call("re", "compile", &[py_str("\\d")]).unwrap_or_else(|e| exc::raise(e)));
    s__FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:leak\\w*|drip\\w*|blocked|clogged|jammed|stuck|won'?t start|won'?t turn on|stopped working|not working|isn'?t working|broken|broke|bang(?:s|ing)?|rattl\\w*|noisy|grind\\w*|screech\\w*|whin\\w*|error code|flashing|blinking|rust\\w*|damp|mould\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_DETAIL_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:funeral|grief|passed away|passed on|death|died|widow|hospital|diagnos\\w*|lost (?:the|our|a) (?:baby|pregnancy)|miscarriage|stillborn|cancer|chemo|depress\\w*|anxiety|breakup|broke up|divorce)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAGMENT(Py::Int(30));
    s_WITH_SLASH(stdlib_call("re", "compile", &[py_str("\\bw/\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DURATION_UNITS(call_builtin("frozenset", &[mcall(&py_str("week weeks day days month months year years hour hours\nfortnight fortnights minute minutes min mins"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TWO_TO(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:going|go|goes|went|off|back|fly|flying|drive|driving|walk|walking|head|heading|move|moving|travel|travelling|coming|come|be|been)\\s+2\\b(?!\\s*(?:%s)\\b)"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_DURATION_UNITS()]).unwrap_or_else(|e| exc::raise(e))])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TWO_PLACE(stdlib_call("re", "compile", &[py_str("\\b2\\s+(?=(?:the|a|an|my|our|his|her|their|its|work|bed|school|town|uni|university|college|church|hospital|town)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TWO_VERB(stdlib_call("re", "compile", &[py_str("\\b2\\s+(?=(?:go|get|come|see|do|be|make|take|put|give|tell|say|try|book|buy|ask|call|send|pay|sort|check|fix|start|stop|leave|stay|eat|drink|sleep|run|walk|drive|catch|meet|pick|order|cook|clean|wash|pack|post|ring|text|email|look|find|keep|learn|help|join|move|remember|finish|read|get)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAK(py_dict(vec![(py_str("im"), py_str("i'm")), (py_str("ive"), py_str("i've")), (py_str("i'd"), py_str("i'd")), (py_str("id"), py_str("i'd")), (py_str("ill"), py_str("i'll")), (py_str("dont"), py_str("don't")), (py_str("didnt"), py_str("didn't")), (py_str("doesnt"), py_str("doesn't")), (py_str("cant"), py_str("can't")), (py_str("wont"), py_str("won't")), (py_str("isnt"), py_str("isn't")), (py_str("wasnt"), py_str("wasn't")), (py_str("werent"), py_str("weren't")), (py_str("arent"), py_str("aren't")), (py_str("havent"), py_str("haven't")), (py_str("hasnt"), py_str("hasn't")), (py_str("couldnt"), py_str("couldn't")), (py_str("wouldnt"), py_str("wouldn't")), (py_str("shouldnt"), py_str("shouldn't")), (py_str("thats"), py_str("that's")), (py_str("its"), py_str("it's")), (py_str("youre"), py_str("you're")), (py_str("weve"), py_str("we've")), (py_str("theres"), py_str("there's")), (py_str("hes"), py_str("he's")), (py_str("shes"), py_str("she's")), (py_str("theyre"), py_str("they're")), (py_str("youve"), py_str("you've"))]));
    s_ITS_OWNS_AFTER(call_builtin("frozenset", &[mcall(&py_str("\n    on in at to from of for with by into onto near under over behind inside outside off about\n    got gets gotten get had has have having sold sell sells bought buy buys changed change changes\n    lost lose loses moved move moves took take takes taken gave give gives made make makes used\n    use uses keeps keep kept needs need needed left leaves\n    replaced replaced fitted fixed cleaned painted washed dropped scratched licked wore worn cut put\n    set parked serviced insured registered broke eaten bit hit\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT(py_dict(vec![(py_str("yrs"), py_str("years")), (py_str("yr"), py_str("year")), (py_str("mth"), py_str("month")), (py_str("mths"), py_str("months")), (py_str("wk"), py_str("week")), (py_str("wks"), py_str("weeks")), (py_str("u"), py_str("you")), (py_str("ur"), py_str("your")), (py_str("r"), py_str("are")), (py_str("n"), py_str("and")), (py_str("tho"), py_str("though")), (py_str("coz"), py_str("because")), (py_str("cuz"), py_str("because")), (py_str("b4"), py_str("before")), (py_str("abt"), py_str("about")), (py_str("tmrw"), py_str("tomorrow")), (py_str("2moro"), py_str("tomorrow")), (py_str("nite"), py_str("night")), (py_str("pls"), py_str("please")), (py_str("thx"), py_str("thanks")), (py_str("luv"), py_str("love")), (py_str("ya"), py_str("you")), (py_str("yer"), py_str("your")), (py_str("goin"), py_str("going")), (py_str("doin"), py_str("doing")), (py_str("havin"), py_str("having")), (py_str("nuthin"), py_str("nothing")), (py_str("somthing"), py_str("something")), (py_str("w"), py_str("with")), (py_str("2"), py_str("two")), (py_str("gonna"), py_str("going to")), (py_str("wanna"), py_str("want to"))]));
    s_YES_AFTER(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill it its that thats the we they you he she my our\n    no yes dont didnt doesnt cant could is was there here what how why when where who ok okay\n    just so but and can do does did nah yep yeah."), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSES(Py::Int(2));
    s_MOST(Py::Int(90));
    s_CORRECTED_WITH_UNIT(py_tuple(vec![py_str("{Value} {unit}, then."), py_str("Got it — {value} {unit}."), py_str("{Value} {unit}, got it.")]));
    s_CORRECTED_ALONE(py_tuple(vec![py_str("{Value}, then."), py_str("Got it — {value}."), py_str("{Value}, got it.")]));
    s_VALUE_UNIT_WORDS(call_builtin("frozenset", &[mcall(&py_str("km k mile miles kilo kilos kg kgs g gram grams stone stones lb\n    lbs foot feet ft inch inches cm m metre metres meter meters yard yards\n    liter litre liters litres ml pint pints hour hours\n    hr hrs minute minutes min mins day days week weeks wk month months year years time\n    pound pounds quid dollar dollars euro euros\n    morning evening night weekend cup cups spoon spoons egg eggs"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VALUE_UNIT_GLUED(stdlib_call("re", "compile", &[py_str("^\\d[\\d,]*(?:\\.\\d+)?([a-z]{1,5})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEASURE(py_dict(vec![(py_str("km"), py_str("distance")), (py_str("k"), py_str("distance")), (py_str("mile"), py_str("distance")), (py_str("miles"), py_str("distance")), (py_str("m"), py_str("distance")), (py_str("metre"), py_str("distance")), (py_str("metres"), py_str("distance")), (py_str("meter"), py_str("distance")), (py_str("meters"), py_str("distance")), (py_str("foot"), py_str("distance")), (py_str("feet"), py_str("distance")), (py_str("ft"), py_str("distance")), (py_str("inch"), py_str("distance")), (py_str("inches"), py_str("distance")), (py_str("cm"), py_str("distance")), (py_str("yard"), py_str("distance")), (py_str("yards"), py_str("distance")), (py_str("kilo"), py_str("weight")), (py_str("kilos"), py_str("weight")), (py_str("kg"), py_str("weight")), (py_str("kgs"), py_str("weight")), (py_str("g"), py_str("weight")), (py_str("gram"), py_str("weight")), (py_str("grams"), py_str("weight")), (py_str("stone"), py_str("weight")), (py_str("stones"), py_str("weight")), (py_str("lb"), py_str("weight")), (py_str("lbs"), py_str("weight")), (py_str("pound"), py_str("money")), (py_str("pounds"), py_str("money")), (py_str("quid"), py_str("money")), (py_str("dollar"), py_str("money")), (py_str("dollars"), py_str("money")), (py_str("euro"), py_str("money")), (py_str("euros"), py_str("money")), (py_str("hour"), py_str("time")), (py_str("hours"), py_str("time")), (py_str("hr"), py_str("time")), (py_str("hrs"), py_str("time")), (py_str("minute"), py_str("time")), (py_str("minutes"), py_str("time")), (py_str("min"), py_str("time")), (py_str("mins"), py_str("time")), (py_str("week"), py_str("time")), (py_str("weeks"), py_str("time")), (py_str("wk"), py_str("time")), (py_str("month"), py_str("time")), (py_str("months"), py_str("time")), (py_str("year"), py_str("time")), (py_str("years"), py_str("time")), (py_str("day"), py_str("time")), (py_str("days"), py_str("time")), (py_str("morning"), py_str("time")), (py_str("evening"), py_str("time")), (py_str("night"), py_str("time")), (py_str("weekend"), py_str("time")), (py_str("time"), py_str("time")), (py_str("times"), py_str("time")), (py_str("per"), py_str("time")), (py_str("a"), py_str("time")), (py_str("an"), py_str("time")), (py_str("every"), py_str("time")), (py_str("litre"), py_str("volume")), (py_str("litres"), py_str("volume")), (py_str("liter"), py_str("volume")), (py_str("liters"), py_str("volume")), (py_str("ml"), py_str("volume")), (py_str("pint"), py_str("volume")), (py_str("pints"), py_str("volume")), (py_str("cup"), py_str("volume")), (py_str("cups"), py_str("volume")), (py_str("spoon"), py_str("volume")), (py_str("spoons"), py_str("volume")), (py_str("egg"), py_str("count")), (py_str("eggs"), py_str("count"))]));
    s_WHOLE_FREQUENCY(call_builtin("frozenset", &[mcall(&py_str("once twice thrice daily nightly weekly fortnightly monthly\n    quarterly yearly annually hourly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_RIGHT_LEAD(py_tuple(vec![py_tuple(vec![py_str("i"), py_str("got"), py_str("it"), py_str("wrong")]), py_tuple(vec![py_str("got"), py_str("it"), py_str("wrong")]), py_tuple(vec![py_str("i"), py_str("made"), py_str("a"), py_str("mistake")]), py_tuple(vec![py_str("i"), py_str("was"), py_str("wrong")]), py_tuple(vec![py_str("my"), py_str("mistake")]), py_tuple(vec![py_str("silly"), py_str("me")]), py_tuple(vec![py_str("i"), py_str("mean")]), py_tuple(vec![py_str("i"), py_str("meant")]), py_tuple(vec![py_str("hang"), py_str("on")]), py_tuple(vec![py_str("hold"), py_str("on")]), py_tuple(vec![py_str("correction")]), py_tuple(vec![py_str("actually")]), py_tuple(vec![py_str("sorry")]), py_tuple(vec![py_str("wait")]), py_tuple(vec![py_str("nope")]), py_tuple(vec![py_str("nah")]), py_tuple(vec![py_str("no")]), py_tuple(vec![py_str("its")]), py_str("it's")]));
    s_PUT_RIGHT_TAIL(py_tuple(vec![py_tuple(vec![py_str("i"), py_str("got"), py_str("it"), py_str("wrong")]), py_tuple(vec![py_str("got"), py_str("it"), py_str("wrong")]), py_tuple(vec![py_str("i"), py_str("made"), py_str("a"), py_str("mistake")]), py_tuple(vec![py_str("i"), py_str("was"), py_str("wrong")]), py_tuple(vec![py_str("my"), py_str("mistake")]), py_tuple(vec![py_str("silly"), py_str("me")]), py_tuple(vec![py_str("i"), py_str("mean")]), py_tuple(vec![py_str("i"), py_str("think")]), py_tuple(vec![py_str("thanks")]), py_tuple(vec![py_str("sorry")]), py_tuple(vec![py_str("actually")]), py_tuple(vec![py_str("then")]), py_tuple(vec![py_str("right")]), py_tuple(vec![py_str("though")])]));
    s_GIVEN_AWAY(stdlib_call("re", "compile", &[py_str("\\b(?:had|have|has|got)\\s+to\\s+give\\b[^.!?]{0,30}?\\b(?:away|up|back|to)\\b|\\b(?:had|have|has|got)\\s+to\\s+(?:rehome|surrender|get rid of)\\b|\\b(?:had|have|has|got)\\s+to\\s+let\\b[^.!?]{0,20}?\\b(?:go|away)\\b|\\b(?:gave|given|giving)\\b[^.!?]{0,30}?\\b(?:away|up)\\b|\\b(?:couldn'?t|could not|can'?t|cannot)\\s+keep\\b|\\bsurrendered\\b|\\bhanded (?:him|her|them|it) (?:over|back)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVEN_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(dogs?|puppy|pup|puppies|cats?|kitten|kittens|rabbits?|hamsters?|guinea pigs?|parrots?|budgie|birds?|horses?|pony|snakes?|lizards?|rats?|ferrets?|tortoises?|pets?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our)\\s+(?:mum|mom|mam|dad|father|mother|sister|brother|son|daughter|nan|gran|granny|aunt|uncle|friend|mate|neighbou?r|colleague|boss|partner|wife|husband|cousin)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVEN_AWAY_REPLIES(py_tuple(vec![py_str("That's a hard thing to do, and I doubt giving up a {animal} was an easy call."), py_str("Giving up a {animal} is usually a last resort, so I doubt you came to it lightly."), py_str("I'm sorry — giving up a {animal} is one of the harder decisions there is."), py_str("That can't have been easy. You did right by the {animal}, whatever the reasons.")]));
    s_OFFERS_A_CHAT(stdlib_call("re", "compile", &[py_str("^\\W*(?:ok|okay|so|well|hi|hey|and)?[,\\s]*(?:i\\s+|we\\s+)?(?:just|only|really|actually\\s)?\\s*(?:wanted|want|fancied|feel like|felt like)\\s+(?:a\\s+|to\\s+|some\\s+)?(?:chat|talk|natter|chinwag)\\b(?:\\s+(?:really|honestly|today|tonight|for a bit|for a while|a bit|with someone|to someone|to somebody|with somebody|i suppose|i guess|thats all|that'?s all|more than anything|you know|i think|if thats ok|if that'?s ok))*[\\s.!?]*$|^\\W*(?:ok|okay|so|well)?[,\\s]*(?:i\\s+)?(?:just\\s+)?(?:wanted|want|fancied)\\s+(?:someone|somebody)\\s+to\\s+(?:talk|chat)\\s+(?:to|with)?(?:\\s+(?:really|honestly|today|tonight|a bit|i suppose|i guess))*[\\s.!?]*$|^\\W*(?:i\\s+)?thought\\s+(?:i'?d|i would)\\s+(?:say|pop)\\s+(?:hi|hello|in)\\b(?:\\s+(?:really|honestly|today|tonight|i suppose|before i go))*[\\s.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHAT_OFFER_REPLY(py_tuple(vec![py_str("Anytime — what's on your mind?"), py_str("I'm here. Anything you want to talk about, big or small?"), py_str("Good, I've got time. What's been going on?"), py_str("Any time. What shall we talk about?"), py_str("Nice — I'm all yours. What's up?")]));
    s_A_DATE_ALONE(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|so|and|well|oh|hmm|yeah|yes|no)\\s*[,\\s]\\s*)*(?:(?:it'?s|its|it is|that'?s|thats|that is|we'?re|we are|the date is|the day is)\\s+)?(?:in|on|for|by|at the end of|early|late)?\\s*(?P<when>january|february|march|april|may|june|july|august|september|october|november|december|next\\s+week|next\\s+month|next\\s+year|this\\s+weekend|the\\s+weekend|tomorrow|tonight|the\\s+\\d{1,2}(?:st|nd|rd|th)|\\d{1,2}(?:st|nd|rd|th)|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\s*(?:i think|i reckon|i believe|this year|next year|at the latest|i'?m sure)?\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONTH_ALONE(stdlib_call("re", "compile", &[py_str("^(?:january|february|march|april|may|june|july|august|september|october|november|december|monday|tuesday|wednesday|thursday|friday|saturday|sunday)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WEEKDAY(stdlib_call("re", "compile", &[py_str("^(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_ON_A_PLAN(py_tuple(vec![py_str("{Thing} — plenty of time to sort the rest."), py_str("{Thing}. That's something to look forward to."), py_str("{Thing} — what's left to sort?"), py_str("{Thing}. Good to have the date down.")]));
    s_A_DAY_ON_A_PLAN(py_tuple(vec![py_str("{Thing} — that is soon, then."), py_str("{Thing}. Not long now."), py_str("{Thing} — that comes round quickly.")]));
    s_A_WORRIED_ABOUT_IT(stdlib_call("re", "compile", &[py_str("\\b(?:nervous|worried|worry|worrying|dreading|dread|scared|afraid|anxious|panic|hate|stress(?:ed|ful)?|not looking forward|can'?t face)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_ON_A_HARD_PLAN(py_tuple(vec![py_str("{Thing} — no wonder it is on your mind."), py_str("{Thing}. The run-up is usually worse than the day itself."), py_str("{Thing} — that is a lot to have coming.")]));
    s_A_DAY_ON_A_HARD_PLAN(py_tuple(vec![py_str("{Thing} — no wonder it is on your mind."), py_str("{Thing}. The day itself is usually easier than the wait.")]));
    s_A_MILESTONE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,3})(?:st|nd|rd|th)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_BACK(stdlib_call("re", "compile", &[py_str("^(?:it|its|it's|that|thats|that's|they|them|he|she|the|this|those|these|everything|nothing|none|all)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_EXPENSE(stdlib_call("re", "compile", &[py_str("\\b(?:cost|costs|costing|expensive|pricey|a fortune|a bomb|not cheap|money pit|over budget|over the budget|cost a lot|costs a lot|adds? up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_STILL_TO_COME(stdlib_call("re", "compile", &[py_str("\\b(?:going\\s+to|gonna|planning|plan\\s+to|planning\\s+to|thinking\\s+of|want\\s+to\\s+(?:go|buy|get|book|move|build|do)|hoping\\s+to|about\\s+to|next\\s+(?:week|month|year|summer|spring|autumn|winter|june|july|august|september|october|november|december|january|february|march|april|may|weekend|christmas)|in\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|the\\s+summer|the\\s+spring|the\\s+autumn|the\\s+winter)|before\\s+i\\s|when\\s+i\\s+go|once\\s+i)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_ALREADY_PAID(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|cost|costs|costing|cost\\s+me|has\\s+been|had\\s+been|paid|spent|bought|redid|redecorated|renovated|built|installed|fitted|took|got|had|already|last\\s+(?:year|month|week|summer|winter|spring|autumn)|ago|in\\s+the\\s+end|all\\s+in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_LONG(stdlib_call("re", "compile", &[py_str("\\b(?:took|takes|taking|take)\\s+(?:forever|ages|hours|all day|all week|all weekend|weeks|months|for ever|forever and a day)\\b|\\bso long\\b|\\blonger than\\b|\\bdragged on\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_BETTER(stdlib_call("re", "compile", &[py_str("\\b(?:better|easier|nicer|cheaper|faster|less stressful|more money|pays better|pays more|worth it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_HARD(stdlib_call("re", "compile", &[py_str("\\b(?:hard|harder|difficult|tough|stressful|exhausting|draining|knackering|hectic|a nightmare|a slog|a struggle)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_WORK(stdlib_call("re", "compile", &[py_str("\\bneeds?\\s+(?:a lot of|some|lots of|loads of)\\s+work\\b|\\bneeds? work\\b|\\bneeds? doing\\b|\\bmore than i thought\\b|\\bbigger than i thought\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_GONE(stdlib_call("re", "compile", &[py_str("\\b(?:got them|got it|ate them|ate it|ruined|ruined it|killed them|all gone|eaten them|stole it|stole them|knocked them|gone to the slugs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GROWING(stdlib_call("re", "compile", &[py_str("\\b(?:tomato|tomatoes|potato|potatoes|plant|plants|planted|seed|seeds|seedling|seedlings|lettuce|beans|peas|carrot|carrots|onion|onions|squash|courgette|courgettes|pumpkin|strawberr\\w+|raspberr\\w+|blackberr\\w+|herb|herbs|flower|flowers|bulbs?|garden|allotment|veg|vegetables?|crop|crops|greenhouse|bed|beds)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_REPLIES(py_dict(vec![(py_str("expense"), py_tuple(vec![py_str("Not cheap, that."), py_str("They never are."), py_str("The bits nobody quotes for are always the ones that cost."), py_str("Ouch. Do you think it was worth it?")])), (py_str("long"), py_tuple(vec![py_str("It always takes longer than the plan says it will."), py_str("Longer than anyone warns you, every time."), py_str("That is the way of it, and at least it is done.")])), (py_str("better"), py_tuple(vec![py_str("Good - that is the bit that matters."), py_str("That is the bit that makes the difference."), py_str("That sounds like the right move.")])), (py_str("hard"), py_tuple(vec![py_str("That is the thing nobody warns you about."), py_str("It does take it out of you."), py_str("It is - and it gets easier once the new bit is routine."), py_str("Fair enough. What has been the hardest bit?")])), (py_str("work"), py_tuple(vec![py_str("Bigger job than it looked, then."), py_str("That is how these go: the first quote is never the whole of it."), py_str("Worth getting a second price before you say yes to it.")])), (py_str("gone"), py_tuple(vec![py_str("Ah, that is a shame."), py_str("That is gutting."), py_str("Netting is the only thing that stops it happening again."), py_str("Will you put another lot in?")]))]));
    s_REMARK_PRICE_AHEAD(py_tuple(vec![py_str("Not cheap, that."), py_str("They never are."), py_str("What are they quoting for it? That is usually the bit worth moving."), py_str("Worth pricing it a couple of ways round before you commit.")]));
    s_REMARK_TWO_SIDED(stdlib_call("re", "compile", &[py_str("\\b(?:but|though|although|however|except|apart from|other than|worse|less|harder|longer|slower)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_JUDGES_THE_PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:expensiv\\w*|pricey|pricier|dear|dearer|a\\s+fortune|a\\s+bomb|a\\s+packet|not\\s+cheap|cheap\\s+enough|money\\s+pit|over\\s+(?:the\\s+)?budget|cost(?:s|ing)?\\s+(?:me\\s+)?a\\s+lot|cost\\s+me|too\\s+much|so\\s+much|that\\s+much|a\\s+lot\\s+of\\s+money|loads|rip[-\\s]*off|overpriced|adds?\\s+up|worth\\s+it|not\\s+worth|daylight\\s+robbery|steep)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_FIGURE(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)\\s*\\d|\\b\\d{2,}\\b|\\b\\d+\\s*(?:quid|pounds?|euros?|dollars?|grand|k)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMARK_FIGURE_REPLIES(py_tuple(vec![py_str("Then it comes down to what you get for it, and a second price usually moves the figure."), py_str("That is the number to weigh up against what doing nothing would cost you."), py_str("Worth checking that against one more quote before you commit to it.")]));
    s_REMARK_KINDS(py_tuple(vec![py_tuple(vec![g_REMARK_EXPENSE(), py_str("expense")]), py_tuple(vec![g_REMARK_LONG(), py_str("long")]), py_tuple(vec![g_REMARK_BETTER(), py_str("better")]), py_tuple(vec![g_REMARK_HARD(), py_str("hard")]), py_tuple(vec![g_REMARK_WORK(), py_str("work")]), py_tuple(vec![g_REMARK_GONE(), py_str("gone")])]));
    s__HOLE(stdlib_call("re", "compile", &[py_str("\\{[^}]*\\}")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_WORDS(stdlib_call("re", "compile", &[py_str("[^a-z0-9]+")]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_WORDS(op_bitor(&call_builtin("frozenset", &[crate::memory::facts::g_RELATIONS()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[py_tuple(vec![py_str("kids"), py_str("children"), py_str("child"), py_str("baby"), py_str("toddler"), py_str("twins"), py_str("them"), py_str("they"), py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("us"), py_str("we"), py_str("both")])]).unwrap_or_else(|e| exc::raise(e))));
    s_A_REASON_MARKER(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|well|so|see)\\s+)?(?:because|cos|coz|cause|cuz)\\b(?:\\s+of\\b)?[\\s,:]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_REASON(stdlib_call("re", "compile", &[py_str("^(?:(?:i\\s+)?(?:dont|do not|dunno|havent a clue|no idea|not sure)\\b(?:\\s+(?:really\\s+)?(?:know\\b\\s*)?(?:why)?)?|no reason|not a clue)\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAUSE_OF_ITS_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|isnt|arent|wasnt|werent|gets?|got|makes?|made|keeps?|kept|feels?|felt|never|always|doesnt|dont|didnt|wont|cant|couldnt|has|have|had|am|been|being|takes?|took|wears?|wore|drives?|drove|works?|worked|goes|went|gives?|gave)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AGE_ALONE(stdlib_call("re", "compile", &[py_str("^(?:i'?m|im|i am|ive|i've|i just turned|i turned)\\s+(\\d{1,3})(?:\\s*(?:years?|yrs?)(?:\\s+old)?)?(?:\\s+(?:now|this year))?\\s*[.!]?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_STORY_GOES_ON(py_tuple(vec![py_str("I'm listening — say more if it helps."), py_str("Go on. I'm here."), py_str("That sounds like a lot to be in the middle of. Tell me the rest."), py_str("I'm with you. Keep going if you want to.")]));
    s_THIRD_PERSON(stdlib_call("re", "compile", &[py_str("^(?:he|she|they|him|her|his|their|them|hes|shes|theyre|theyd|theyll|she'd|he'd|she'll|he'll)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_TAIL(py_tuple(vec![py_str("{length} of it — has anything changed in that time?"), py_str("{length} of it. How are you finding it now?"), py_str("{length} of it already. Is it going the way you want it to?"), py_str("{length} of it — and has anything you have tried made a difference?")]));
    s_SINCE_TAIL(py_tuple(vec![py_str("Since {when} — has anything changed in that time?"), py_str("Since {when}. How are you finding it now?"), py_str("Since {when}. Is it going the way you want it to?"), py_str("Since {when} — and has anything you have tried made a difference?")]));
    s_LENGTH_KEEPS(call_builtin("frozenset", &[py_tuple(vec![py_str("nearly"), py_str("almost"), py_str("over"), py_str("going on")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_OLD_ONE(stdlib_call("re", "compile", &[py_str("^\\W*(?:the|my|our)\\s+(?:old|current|existing|present)?\\s*one(?:\\s+(?:i|we)\\s+(?:have|'ve|ve|got|use|used))?\\s+(?:is|was|'s|s)\\s+(?P<n>\\d{1,2}|[a-z]{3,9})\\s*(?:years?|yrs?)(?:\\s+old)?\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPLACING(stdlib_call("re", "compile", &[py_str("\\b(?:new|getting|get|got|bought|buy|buying|replace|replacing|upgrade|upgrading|order|ordering|shopping for|looking at|need a|needs a|want a|after a)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OLD_ONE_TAIL(py_tuple(vec![py_str("{n} years on the old one — it has done its share. What are you after this time?"), py_str("{n} years is a good run out of one, so you are not replacing it early."), py_str("{n} years — that is a fair innings, and it has earned the swap.")]));
    s_THE_NOUN_IS_AT(stdlib_call("re", "compile", &[py_str("\\bthe\\s+(?P<noun>[a-z'-]+)\\s+is\\s+(?:at|for)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MARKED_HOUR(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\d{1,2}\\s*(?:am|pm|a\\.m\\.|p\\.m\\.)|\\d{1,2}[:.]\\d{2}|o'?\\s?clock|\\b(?:half|quarter)\\b|\\b"), &g_TIME_WORD()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AS_WELL(stdlib_call("re", "compile", &[py_str("\\b(?:as\\s+well|too|also|another)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:kid|kids|child|children|son|sons|daughter|daughters|baby|babies|toddler)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_ADDED(stdlib_call("re", "compile", &[py_str("\\A\\s*,?\\s*(?:(?:(?:is|are|was|were)\\s+)?(?:called|named)\\b[\\w' ]*|(?:is|are|was|were)\\s+(?:\\d\\w*|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty)\\b(?:\\s*(?:years?|months?|weeks?|days?)\\s*old|\\s+old)?\\s*\\Z|(?:is|are)\\s+(?:a|an|another)\\s+[\\w' ]{1,20}\\Z|,\\s*\\w[\\w' ]*|(?:and|with|plus|,)\\s+(?:a|an|another|one|\\d+)?\\s*[\\w' ]{1,24}\\Z|\\Z)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_ALL_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|'s)\\s+(?:(?:doing|feeling)\\s+)?(?:fine|ok|okay|better|well|healthy|alright|all\\s+right|great)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INTRODUCED_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:named|called)\\s+(?:the|my|our|a|an|his|her|their|its)?\\s*\\Z"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_AFTER(stdlib_call("re", "compile", &[py_str("\\s*,?\\s*[a-z][\\w'-]*[.!]?\\Z"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VAGUE_MANY(stdlib_call("re", "compile", &[py_str("\\b(?:lots|loads|plenty|several|many|a\\s+few|some)\\b\\s*(?:of\\s+)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANIMAL_TAKEN_BACK(py_tuple(vec![py_str("No {animal}, then."), py_str("Fair enough — no {animal}."), py_str("Right — no {animal}.")]));
    s_A_WANTED_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanted|wanting|would\\s+like|'d\\s+like|would\\s+love|fancy|fancied|i'?d\\s+love|id\\s+love|after|asked\\s+for|asking\\s+for|been\\s+asking|begging\\s+for|dream(?:s|ing)?\\s+of|keen\\s+on|desperate\\s+for|mad\\s+(?:on|for)|hoping\\s+(?:for|to\\s+get)|(?:thinking|thought|talking)\\s+(?:of|about)\\s+getting|going\\s+to\\s+get)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WANT_TURNED_DOWN(stdlib_call("re", "compile", &[py_str("\\b(?:don'?t|dont|doesn'?t|doesnt|didn'?t|didnt|won'?t|wont|not|never|no\\s+longer)\\b[^.!?;]{0,16}\\b(?:want|wants|wanted|like|keen|after|ask(?:ed|ing))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAD_AN_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:had|used\\s+to\\s+have|use\\s+to\\s+have|grew\\s+up\\s+with)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANIMAL_HAD_ONCE(py_tuple(vec![py_str("A {animal} growing up, then — would you have another?"), py_str("You had a {animal} before. Do you think you would again?"), py_str("A {animal} when you were younger, then. What do you miss about it?")]));
    s_A_PET_WANTED(py_tuple(vec![py_str("Which animal are they after — a dog, a cat, something small?"), py_str("What have they got in mind? A pet is a long commitment and the answer changes with the animal.")]));
    s_A_WANTED_ANIMAL_TAIL(py_tuple(vec![py_str("A {animal}, then — what is putting you off getting one?"), py_str("So you are thinking about a {animal}. Who would look after it in the day?"), py_str("A {animal} is a long commitment. How soon were you thinking?")]));
    s_PET_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:pet|pets)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANIMAL_COMING(py_tuple(vec![py_str("A {animal} on the way, then — is everything ready for it?"), py_str("A {animal} coming home. Have you picked a name yet?"), py_str("A {animal} to look forward to.")]));
    s_PET_TAIL_COMING(py_tuple(vec![py_str("{Thing} — when does it arrive?"), py_str("{Thing}. Not long now."), py_str("{Thing} — the waiting is the worst part.")]));
    s_PET_TAIL_COMING_MANY(py_tuple(vec![py_str("{Thing} — when do they arrive?"), py_str("{Thing}. Two at once, that's a houseful."), py_str("{Thing}. Not long now.")]));
    s_WISH_FOR_AN_ANIMAL(py_tuple(vec![py_str("{Thing}? Where are you up to with it?"), py_str("{Thing} - what's drawing you to one?"), py_str("{Thing}. What would you have to sort out first?")]));
    s_A_BREED_BEING_CHOSEN(py_tuple(vec![py_str("{Animal} — are you settled on that one?"), py_str("{Animal}, then. What made you pick that one?"), py_str("{Animal} — have you met one before?")]));
    s_NOBODY_AROUND(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:don'?t|do not)\\s+(?:really\\s+|even\\s+|actually\\s+)?know\\s+(?:anyone|anybody|a soul|any ?one else|a single person|one person)\\b|\\bi\\s*'?ve\\s+(?:got|gotten)\\s+no\\s*(?:one|body|friends|mates)\\b|\\bi\\s+have\\s+no\\s*(?:one|body|friends|mates)\\b|\\bno ?(?:one|body)\\s+to\\s+(?:talk|chat|go out|spend|turn)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOBODY_AROUND_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:who|that)\\s+(?:can|could|would|will|to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANYBODY_AT_ALL(stdlib_call("re", "compile", &[py_str("\\b(?:know|knows)\\s+(?:lots|plenty|loads|a few|some|quite a few|lots of|plenty of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_PERSON(call_builtin("frozenset", &[mcall(&py_str("he she they his her their hes shes theyve him them him/her"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_HOME(stdlib_call("re", "compile", &[py_str("\\b(?:should|might|will|hopefully|due|going|expected|allowed)\\s+(?:to\\s+)?(?:be\\s+)?(?:out|home|discharged|released)\\b|\\b(?:out|home|discharged|released)\\s+(?:of\\s+(?:hospital|there)\\s+)?(?:next week|this week|tomorrow|soon|on (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|in a (?:few|couple of) days|by the weekend)\\b|\\bcoming home\\b|\\bout of hospital\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THE_MEND(stdlib_call("re", "compile", &[py_str("\\b(?:doing|getting|feeling|looking)\\s+(?:a lot |much |a bit |so much |a little )?better\\b|\\bon the mend\\b|\\bon the up\\b|\\bturned (?:a|the) corner\\b|\\bout of (?:the )?danger\\b|\\bpicking up\\b|\\bback to (?:her|his|their) old self\\b|\\bwell on the way\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_AT_IT(stdlib_call("re", "compile", &[py_str("\\bback (?:at work|to work|in work|at school|on (?:her|his|their) feet|to (?:normal|it|herself|himself|themselves|training|the gym|uni|university))\\b|\\bup and about\\b|\\bwalking (?:again|now)\\b|\\bwell enough to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_HOME_TAIL(py_tuple(vec![py_str("Out {soon} — that is the news you want. The first days at home are usually the flat ones, so keep them quiet."), py_str("That is a relief. Coming home is the milestone; the tiredness tends to last a few weeks past it."), py_str("Good to hear. Home beats any ward for sleeping, even with the stairs in the way.")]));
    s_ON_THE_MEND_TAIL(py_tuple(vec![py_str("Good to hear it. Recovery goes in fits and starts — a good day, then a flat one, and both are normal."), py_str("That is the direction you want. Better is slower than people expect, and it counts all the same."), py_str("Worth saying out loud. On the mend is the part that takes patience from everyone round them.")]));
    s_BACK_AT_IT_TAIL(py_tuple(vec![py_str("Back at it already is quick going. The first week back is the tiring one for most people."), py_str("That is a good sign. Getting back to normal takes longer than people expect, and the first week of it is the hardest."), py_str("Good news. Back to it — and the first week is the one to take gently.")]));
    s_GOING_HOME_MINE(py_tuple(vec![py_str("Out {soon} — good. The first days at home are the flat ones, so keep them quiet and let people bring things."), py_str("That is a relief to hear. Home is where the sleep happens; the ward is not.")]));
    s_ON_THE_MEND_MINE(py_tuple(vec![py_str("Good to hear. Recovery goes in fits and starts — a good day, then a flat one, and both are normal."), py_str("That is the direction you want. Better is slower than people expect, and it counts all the same."), py_str("Worth saying out loud. Take the good days as they come rather than testing them.")]));
    s_BACK_AT_IT_MINE(py_tuple(vec![py_str("Back at it already is quick going. The first week back is the tiring one for most people."), py_str("Good sign. Getting back to normal takes longer than people expect, so keep the first week light."), py_str("Good news. Back to it — and the first week is the one to take gently.")]));
    s_A_RELATION(call_builtin("frozenset", &[mcall(&py_str("mum mom mother dad father nan nanna granny gran grandma grandad grandpa\n    son daughter kid child baby sister brother mate friend partner wife husband aunt uncle cousin\n    niece nephew colleague boss neighbour neighbor lodger flatmate housemate"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOVE(stdlib_call("re", "compile", &[py_str("\\b(?:moved|moving|move|relocat|emigrat|new (?:city|town|area|job|country))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALONE_AFTER_A_MOVE(py_tuple(vec![py_str("The first months somewhere new are the emptiest part of a move, and it does ease off. Whatever meets weekly near you - a class, a club, a five-a-side - fills it faster than waiting to be asked."), py_str("That's the hard part of moving, and nobody warns you how long it lasts. One thing you turn up to at the same time every week is how the same faces start."), py_str("Being new somewhere is lonely in a way that has nothing to do with how likeable you are. Say yes to the first small invitation that comes, even the boring one.")]));
    s_ALONE_TAIL(py_tuple(vec![py_str("That's a lonely way to spend the week, and it isn't a fault in you. One message to someone you already like does more than waiting for a night out."), py_str("That's a quiet way to live. The people who get to know you are the ones you see at the same time each week, so it is worth having one of those."), py_str("That's hard going. Being on your own in a place with no history to it wears thin, and the way out is nearly always the same small step: one regular thing, one person you keep saying hello to.")]));
    s_ALONE_AFTER_A_LOSS(py_tuple(vec![py_str("That's the hardest part of it, the empty week afterwards. The people who knew them are usually the ones who most want to hear from you."), py_str("Being on your own after something like that is a weight of its own, and it is not something to push through. A short call to someone who knew them helps more than you'd think.")]));
    s__WORDS(stdlib_call("re", "compile", &[py_str("[a-z']+")]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_BOT_AS_SUBJECT(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:oh|aw+|ah|ha+|tbh|honestly|ok|okay|well|and|but|so|yeah)[,\\s]+)*you(?:'?re| are| were| seem| look| sound| make| keep| always| really| definitely)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMEONE_ELSE_IN_IT(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|i've|ive|i'd|we|we're|our|my|mine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACED_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:last (?:year|summer|winter|spring|autumn|month|week|night)|years ago|months ago|weeks ago|back in|when i was|used to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOB_TO_DO(stdlib_call("re", "compile", &[py_str("\\bto\\s+(?:sort|fix|clear|clean|paint|do|tidy|finish|replace|handle|deal\\s+with|get\\s+(?:rid\\s+of|done|sorted)|put\\s+up|take\\s+down|strip|sand|wire|plumb|service|return|file|chase|book|renew|sell|pack|prepare|plan|write|organise|organize|arrange|fill\\s+(?:in|out)|sign|complete|send\\s+(?:off|back)|post\\s+off|build|assemble|learn|study|wrap|draft|collate|make|iron|wash|mow|weed|dig|tile|grout|seal|hang|fit|install|mount|mend|sew|hem|scrub|hoover|vacuum)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOB_HAS_A_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|tomorrow|tmrw|this\\s+(?:week|weekend|month|evening)|next\\s+week|weekend|by\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|the\\s+end)|monday|tuesday|wednesday|thursday|friday|saturday|sunday|the\\s+end\\s+of\\s+the\\s+month)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOB_TAIL(py_tuple(vec![py_str("{Thing} — is that the weekend, or are you fitting it in around work?"), py_str("{Thing}. Do you know where to start with it, or is that the hard part?"), py_str("{Thing} — what's the first thing that needs doing?"), py_str("{Thing}. Is it a bigger job than it looked?")]));
    s_JOB_BEFORE(stdlib_call("re", "compile", &[py_str("\\bbefore\\s+(?:the|my|we|i|having|that)\\b|\\b(?:until|till|up to)\\s+\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_JOB(py_tuple(vec![py_str("{X} — that is a real thing to have behind you. What has changed since then?"), py_str("{X} is not nothing to have behind you. Whatever comes next, you are not starting from the bottom of anything."), py_str("{X}, then. You know that work from the inside, which counts for more than it looks.")]));
    s_PAST_JOB_PLAIN(py_tuple(vec![py_str("{X} — that is not nothing to have behind you. Do you miss it, or are you glad to be out of it?"), py_str("{X} is a thing you already know how to do, which is more than most people start with."), py_str("{X}, then. What is it that has you thinking about it now?")]));
    s_WHO_THE_FIGURE_IS(py_tuple(vec![py_str("Who's {figure}?"), py_str("{Figure} — who's that?")]));
    s_WHO_IS_IT(py_tuple(vec![py_str("Who do you mean?"), py_str("Who's that, then?"), py_str("Who are we talking about?")]));
    s_WHAT_OF_THEM(py_tuple(vec![py_str("{Count} of what?"), py_str("{Count} of what, sorry?")]));
    s_WHICH_ONES(py_tuple(vec![py_str("Which ones do you mean?"), py_str("Which one do you mean?")]));
    s_POINTS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:them|those|these|ones|it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_OF_THEM(stdlib_call("re", "compile", &[py_str("\\b(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a few|a couple|\\d{1,3})\\s+of\\s+(?:them|those|these)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_STILL_TO_COME(stdlib_call("re", "compile", &[py_str("\\b(?:turning|turns)\\s+(?:\\d{1,3}|[a-z]+)\\b|\\b(?:next|this)\\s+(?:week|month|year|weekend|summer|christmas|spring|autumn|winter)\\b|\\b(?:in|by|until|till)\\s+(?:the\\s+)?(?:new\\s+year|summer|christmas)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WONDERED_CONDITION(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:think|believe|reckon|suspect|might|may)\\b[^.!?,;]{0,30}?\\b(?:i(?:['\\u2019]?ve|\\s+have)?\\s+)?(?:have|got)\\s+(?P<thing>[a-z][a-z'\\u2019 \\-]{2,30})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WONDER_SAID_NO(stdlib_call("re", "compile", &[py_str("\\b(?:don'?t|do not|didn'?t|did not|never|not)\\s+(?:think|believe|reckon|suspect)\\b|\\bnot sure\\b|\\bno idea\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WITH_AN_ARTICLE(call_builtin("frozenset", &[mcall(&py_str("a an the some any my our his her their this that these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDITION_KIND(stdlib_call("re", "compile", &[py_str("(?:disorder|syndrome|disease|infection|deficiency|condition|intolerance|allergy|virus|ulcer|fracture|clot|itis|concussion|hernia|mole|reaction|abscess|boil|polyp|stones?)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_INITIALS(call_builtin("frozenset", &[mcall(&py_str("sti dvt uti hiv ibs ocd adhd ptsd ms me cf ibd copd gerd pnd\n    tmj pcos"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_FAMILIES(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:depression|depressed|anxiety|ptsd|adhd|autism|autistic|bipolar|ocd|eating|anorexi\\w*|bulimi\\w*|burnout|self.?harm)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Whether it's {thing} isn't something I can judge from a chat - that takes a proper assessment, and the way in is a GP appointment or a self-referral to talking therapy. Say how long it has been like this and what has changed; that is where the assessment starts."), py_str("I can't diagnose {thing} and a guess would be no use to you. Book a GP appointment and ask about a referral; with something like this the waiting list is the annoying part, so the useful move is to get in the queue now rather than when it gets worse.")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:sti|hiv|chlamydia|gonorrhoea|gonorrhea|herpes|syphilis|thrush|warts|piles|haemorrhoids|hemorrhoids)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("I can't tell you from here whether it's {thing}. Get it checked: a sexual health clinic sees you without a GP referral, the test is quick and usually free, and they can treat it there too."), py_str("Whether it's {thing} is a test question, not a chat one. Clinics do free, confidential testing and you can walk in without a referral - go sooner rather than later, because most of these are easy to sort once you know what it is.")])]), py_tuple(vec![Py::None, py_tuple(vec![py_str("Whether it's {thing} is a doctor's call, not something a chat can settle. Book an appointment and take a note of when it started and what has changed - that is what turns a hunch into an answer."), py_str("I can't tell you whether you have {thing}, and a guess would be worse than no answer. Get it looked at - a GP or a walk-in clinic - and write down how long it has been going on first, because that is the first thing they ask."), py_str("Whether it's {thing} is not something I can say from here - that needs someone who can examine you or run a test. Book it, and take three things: when it started, what has changed, and what you want out of the appointment.")])])]));
    s_RAISED(stdlib_call("re", "compile", &[py_str("\\b(?:stretch\\s+(?:it\\s+)?(?:up\\s+)?to|go\\s+up\\s+to|push\\s+(?:it\\s+)?(?:up\\s+)?to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:mum|mom|mam|mother|dad|father|son|daughter|sister|brother|wife|husband|partner|aunt|uncle|gran|granny|grandma|grandad|grandpa|nan|cousin|niece|nephew|mate|mates|friend|neighbour|neighbor|colleague|boss|stepson|stepdaughter|stepdad|stepmum|boyfriend|girlfriend|fiance|fiancee)(?:'?s)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FAMILY(py_tuple(vec![py_str("people"), py_str("others"), py_str("kids"), py_str("relations")]));
    s_REPAIR_FRAME(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:no|nope|sorry|apologies|actually|wait|hold\\s+on|oops|and|so|oh|right|ok|okay)\\b[\\s,]*)*(?:i\\s+(?:mean|meant)(?:\\s+to\\s+say)?|make\\s+that|you\\s+mean)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VAGUE(call_builtin("frozenset", &[mcall(&py_str("everything anything something nothing all stuff things thing lot lots\n    bit bits more less much many one ones again once anyway same usual normal worse better\n    else other others"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENS_WITH_A_LINK(stdlib_call("re", "compile", &[py_str("^(?:for|about|to|of|with|in|on|at|over|into|like|as)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_ONLY(stdlib_call("re", "compile", &[py_str("^(?:(?:about|around|under|below|less than|up to|max(?:imum)?|no more than|not more than)\\s*)?(?:[£$€]\\s?\\d[\\d,.]*(?:\\s?(?:k|pounds?|quid|euros?|dollars?|bucks))?|\\d[\\d,.]*\\s?(?:pounds?|quid|euros?|dollars?|bucks|k)\\b)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KIN_POSSESSIVE(stdlib_call("re", "compile", &[py_str("\\b(your (?:mum|mom|mother|dad|father|son|daughter|sister|brother|wife|husband|partner|nan|nana|granny|gran|grandma|grandad|grandpa|aunt|auntie|uncle|niece|nephew|boyfriend|girlfriend|fiance|fiancee))s(?= (?:\\d+|called|named|name|age|coming|doing|gone|been|had|has|is|was|turns|turned)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_NOT_A_STATE(stdlib_call("re", "compile", &[py_str("^you're ((?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\\s+([a-z]{2,}s)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNITS_OF_MEASURE(call_builtin("frozenset", &[py_tuple(vec![py_str("years"), py_str("months"), py_str("weeks"), py_str("days"), py_str("hours"), py_str("minutes"), py_str("seconds"), py_str("nights"), py_str("mornings"), py_str("evenings"), py_str("hrs"), py_str("mins"), py_str("secs"), py_str("kilos"), py_str("kilograms"), py_str("kgs"), py_str("pounds"), py_str("lbs"), py_str("stones"), py_str("grams"), py_str("grammes"), py_str("ounces"), py_str("miles"), py_str("yards"), py_str("metres"), py_str("meters"), py_str("kilometres"), py_str("kilometers"), py_str("litres"), py_str("liters"), py_str("pints"), py_str("cups"), py_str("calories"), py_str("degrees"), py_str("inches")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHOS(stdlib_call("re", "compile", &[py_str("\\bwhos\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_REFERENT(py_tuple(vec![py_str("The same one as what?"), py_str("Which one do you mean?"), py_str("Same as what? Tell me what we are talking about and I will pick it up."), py_str("I have lost the thread - which one do you mean?")]));
    s_POINTS_AT_ANOTHER(stdlib_call("re", "compile", &[py_str("\\b(?:another one|one more|another|one too|one as well|one aswell|one before|one of those|the same one|one like it|one like that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DECISION(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanted|should|shall|might|could|would|thinking|think|planning|planned|considering|maybe|tempted|weighing|worth)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORTS_AN_ACTION(stdlib_call("re", "compile", &[py_str("\\b(?:saw|seen|read|made|bought|finished|started|watched|met|caught|found|killed|sold|won|lost)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTER_FILLER(op_bitor(&op_bitor(&call_builtin("frozenset", &[mcall(&py_str("\ni we you he she they it me my our your his her their its us them mine ours\nthe a an and but so also too as well just even still already now again then\nhad has have got gotten get gets bought buy buying made make makes making\nfinished finish started start read reading seen saw did do does doing\none ones another more of that this these those there here before after\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[crate::memory::facts::g_RELATIONS()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[mcall(&py_str("\nwife husband partner boyfriend girlfriend son daughter child kid kids baby mum mom mother\ndad father brother sister friend mate colleague boss family parents in laws\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_NOT_A_THING_TO_HAVE_ANOTHER(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("\nanother more one ones same other else lot bit piece time times day days week weeks\nmonth months year years hour hours minute minutes while thing things stuff kind sort\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[crate::memory::facts::g_RELATIONS()]).unwrap_or_else(|e| exc::raise(e))));
    s_ANOTHER_ONE(py_tuple(vec![py_str("Another {thing}?"), py_str("A second {thing}, then. How is it going?"), py_str("Another {thing} - what is this one like?"), py_str("Two {things}, then. How are they getting on?")]));
    s_ANOTHER_ONES(py_tuple(vec![py_str("More {things}?"), py_str("Another lot of {things}, then. How are they getting on?"), py_str("More {things} - how are the first lot doing?")]));
    s_BEFORE_ONE(py_tuple(vec![py_str("How does this one compare to the last?"), py_str("What happened to the first {thing}?"), py_str("Still got the first {thing}?")]));
    s_SOMEONE_ELSE_POINTS_AT_ANOTHER(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+([a-z]+(?:[- ](?:in[- ]law|law))?)\\s+(?:has|have|got|gets|bought|buys|wants|wanted|had|found|rescued|adopted|is getting|is buying|is having)\\s+(?:another one|one too|one as well|one aswell|one of those|the same one|another)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN_POINTS_AT_ANOTHER(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they)\\s+(?:has|have|got|gets|bought|wants|wanted|had|found|rescued|adopted)\\s+(?:another one|one too|one as well|one aswell|one of those|the same one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER_ONE_FOR_SOMEONE(py_tuple(vec![py_str("Your {person} too, then. How is the {thing} going?"), py_str("Your {person} as well, then - what is the {thing} like?")]));
    s_ANOTHER_SOME_FOR_SOMEONE(py_tuple(vec![py_str("Your {person} has some too, then. How are the {things} going?"), py_str("Your {person} as well, then - how are the {things} getting on?")]));
    s_A_NAME_GIVEN(stdlib_call("re", "compile", &[py_str("\\b(?:called|named|name is|name's|names)\\s+([a-z']+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENDS_THE_THING_VERB(call_builtin("frozenset", &[mcall(&py_str("\naint arent cant couldnt didnt doesnt dont hadnt hasnt havent isnt mustnt neednt shant wasnt\nwerent wont wouldnt shouldnt\nwent gone came got gotten took taken gave given told said seen knew known kept\nbrought bought found goes comes says keeps sees knows thinks tells gives takes makes\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DETERMINER(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the|my|our|his|her|their|your|another|some|two|three|four|five|six|seven|eight|nine|ten)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENDS_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\nin on at to for with from of about by into onto over under out up down off through across along\nafter before during since until while when where why how what which who whom whose that this\nthese those and but or so because if than then\ni me you we us they them he him she it its am is are was were be been being have has had do does\ndid will would can could should may might must not no\nvery really quite too also just still again now last next\nbit lot few couple other same whole rest most least more less only first\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SECOND_OF(call_builtin("frozenset", &[mcall(&py_str("\ntraffic weather news advice information knowledge music homework housework work sleep noise mess\nrubbish luck rain snow water wind light heat fun health pain stress exercise help space shopping\ncleaning washing food drink dinner breakfast lunch tea coffee time money\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_GOT(stdlib_call("re", "compile", &[py_str("\\b(you|we)\\s+have\\s+got\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_A(stdlib_call("re", "compile", &[py_str("\\b(You|you|We|we)\\s+have\\s+a\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_LEAD(stdlib_call("re", "compile", &[py_str("^(?:i\\s*(?:'m|am)|im|i\\s+just\\s+turned|my\\s+(?:age|height)\\s+is)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_FILLER(call_builtin("frozenset", &[mcall(&py_str("year years yrs yr old this now today already and"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_UNIT(call_builtin("frozenset", &[mcall(&py_str("ft foot feet in inch inches cm m st stone pounds pound lbs lb\n    kilo kilos kg metres meters"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_WORD(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONTINUATION(stdlib_call("re", "compile", &[py_str("^(?:and|also|plus|or|what about|how about|and what about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|am|was|were|be|been|being|isnt|arent|wasnt|werent|has|have|had|hasnt|havent|hadnt|do|does|did|dont|doesnt|didnt|will|wont|would|wouldnt|can|cant|could|couldnt|should|shouldnt|shall|may|might|must|got|get|gets|getting|go|goes|going|went|gone|need|needs|needed|want|wants|wanted|like|likes|liked|think|thinks|thought|know|knows|knew|take|takes|taking|took|make|makes|making|made|start|starts|started|come|comes|came|live|lives|lived|work|works|worked|feel|feels|felt|keep|keeps|kept|try|tries|trying|look|looks|looking|plan|plans|planning|book|booked|hope|hoping|wait|waiting|see|saw|seen|say|says|said)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:i|i'm|im|i've|ive|i'd|id|i'll|ill|me|my|we|we're|were|we've|our|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_the_rest_of_a_question(g__the_rest_of_a_question());
    s_NO_THEN_A_LINE(py_str("no\\s+(?=(?:i|im|i'm|we|we're|were|you|youre|you're|he|hes|he's|she|shes|she's|it|its|it's|they|theyre|they're|this|that|thats|that's|these|those|there|the|not|my|our|your|his|her|their)\\b)"));
    s_NO_THEN_A_LINE_ONLY(stdlib_call("re", "compile", &[op_add(&py_str("^\\s*"), &g_NO_THEN_A_LINE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_LEAD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:(?:no\\s*,|"), &g_NO_THEN_A_LINE()), &py_str("|(?:no\\s+|oh\\s+|sorry\\s+|actually\\s+)?(?:wait|hang\\s+on|hold\\s+on)\\s*,|(?:i\\s+(?:meant|mean)(?:\\s+to\\s+say)?(?!\\s*,?\\s*(?:it|that|this|so)\\b(?:\\s*,?\\s*\\w+){0,2}\\s*[.!?]*\\s*$)|you\\s+mean|make\\s+that)\\b\\s*,?|(?:sorry|apologies|actually|oops|oh)\\s*,)\\s*)+")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REACTION_PHRASES(call_builtin("frozenset", &[py_tuple(vec![py_str("quiet one"), py_str("quiet day"), py_str("busy one"), py_str("busy day"), py_str("slow day"), py_str("long day"), py_str("not bad"), py_str("not much"), py_str("nothing much"), py_str("not a lot"), py_str("all good"), py_str("can't complain"), py_str("cant complain"), py_str("same as ever"), py_str("good thanks"), py_str("fine thanks"), py_str("no complaints"), py_str("same old"), py_str("usual stuff"), py_str("you too"), py_str("likewise"), py_str("went ok"), py_str("went well"), py_str("went fine"), py_str("so so"), py_str("so-so"), py_str("up and down"), py_str("not too bad")])]).unwrap_or_else(|e| exc::raise(e)));
    s_REACTION_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nlucky you fair enough got it sounds good sound good great nice cool lovely perfect ace brilliant\noh dear dear me good luck well done makes sense same here me too that's right exactly agreed\nno worries okay ok thanks ta cheers haha lol ha magic typical bless you what a shame that's a\nshame of course indeed true sure yeah yes oh wow ouch yikes a\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__REACTION_CLAUSE(stdlib_call("re", "compile", &[py_str("^([^.?!,;:]{1,32})[.?!,;:]+\\s*(?=\\S)")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED_STARS(stdlib_call("re", "compile", &[py_str("\\b(?:(?:a|an)\\s+)?(?P<count>[1-9]|1[0-2])\\s+a[\\s*'-]?stars?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ONE_STAR(stdlib_call("re", "compile", &[py_str("\\b(?:an?|one)\\s+a[\\s*-]?star\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STAR_COUNT(py_dict(vec![(Py::Int(1), py_str("an A*")), (Py::Int(2), py_str("two A*s")), (Py::Int(3), py_str("three A*s")), (Py::Int(4), py_str("four A*s")), (Py::Int(5), py_str("five A*s")), (Py::Int(6), py_str("six A*s")), (Py::Int(7), py_str("seven A*s")), (Py::Int(8), py_str("eight A*s")), (Py::Int(9), py_str("nine A*s")), (Py::Int(10), py_str("ten A*s")), (Py::Int(11), py_str("eleven A*s")), (Py::Int(12), py_str("twelve A*s"))]));
    s_COUNTED_LETTERS(stdlib_call("re", "compile", &[py_str("\\b(?P<count>[2-9]|1[0-2])\\s+(?P<letter>a|b|c|d)(?:'?s|s)\\b(?=\\s*(?:$|[,.;!?]|and\\b|but\\b))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LETTER_COUNT(py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]));
    s_A_DEGREE(stdlib_call("re", "compile", &[py_str("\\b(?:degree|degrees|uni|university|graduat\\w*|honours|bsc|ba|masters?|phd|dissertation|final year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEGREE_CLASS(stdlib_call("re", "compile", &[py_str("\\b(?P<first>[12])\\s*[:.\\- ]\\s*(?P<second>[12])\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRAST(stdlib_call("re", "compile", &[py_str("\\s*,?\\s+not\\s+(?P<old>[\\w'’$£€.,:/-]+(?:\\s+[\\w'’$£€.,:/-]+){0,3})\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRAST_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("^(?P<thing>[a-z]+(?:\\s+[a-z]+){0,2})\\s+not\\s+(?P<old>[\\w'’$£€.,:/-]+)\\s+(?P=thing)\\s+(?P<new>[\\w'’$£€.,:/-]+(?:\\s+[\\w'’$£€.,:/-]+){0,2})\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_VALUE(call_builtin("frozenset", &[mcall(&py_str("a an the any sure really very too that this it them me you him her us\nsay said tell go going gone get got want need like mind be been being am is are was were have\nhas had can could would should will do does did make take give put keep try yet even still\nallowed able bothered"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_FRAME(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:let'?s|let\\s+us|i'?ll|we'?ll|i\\s+will|we\\s+will)\\s+go\\s+(?:with|for)\\b|(?:i|we)\\s+(?:have\\s+|had\\s+)?(?:settled|decided|decide|settle)\\s+(?:on|for)\\b|(?:i|we)\\s+(?:have\\s+|had\\s+)?(?:opted|opt)\\s+for\\b|(?:i|we)\\s+(?:picked|pick|chose|choose)\\b|(?:i|we)'?ve\\s+(?:settled|decided|gone)\\s+(?:on|for|with)\\b|(?:make|makes)\\s+(?:it|that)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REASON_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:because|cos|coz|cuz|'cause|since)\\b[^.!?]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CORRECTION_IN_FRONT(stdlib_call("re", "compile", &[py_str("^\\s*(?:actually|no|nope|sorry|wait|oh|i\\s+mean|i\\s+meant|make\\s+that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BETWEEN_VALUES(call_builtin("frozenset", &[mcall(&py_str("he she it they we you i his her its their our my\nis was are be been am just only now actually really still but\nhes shes its theyre im youre were s"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VALUE(stdlib_call("re", "compile", &[py_str("\\d|^(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|a|an|grand|quid|pm|am)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MAKE_A_VALUE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:no|nope|sorry|ah|oh|ok|okay|right|and|so|well|actually|wait|hmm)[,.]?\\s+)*(?:make\\s+(?:it|that)|change\\s+(?:it|that)\\s+to|switch\\s+(?:it|that)\\s+to)\\s+(?P<value>[^,.!?;]{1,40}?)\\s*[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VALUE_WORDS(call_builtin("frozenset", &[mcall(&py_str("half quarter past to oclock am pm once twice one two three four five six\nseven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen\ntwenty thirty forty fifty sixty seventy eighty ninety hundred thousand a an grand the\nevery of hour hours minute minutes week weeks day days month months year years time times per a\nkilo kilos kg mile miles km pound pounds quid dollar dollars euro euros"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNITS_AFTER(call_builtin("frozenset", &[mcall(&py_str("times time a an per week weeks day days month months year years hour\nhours minute minutes mile miles km kilo kilos pound pounds quid egg eggs cup cups spoon\nspoons every morning evening night weekend"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("he"), py_str("he's"), py_str("he'll"), py_str("she"), py_str("she's"), py_str("she'll"), py_str("it"), py_str("it's"), py_str("it'll"), py_str("they"), py_str("they're"), py_str("they'll"), py_str("they've"), py_str("we"), py_str("we're"), py_str("we'll"), py_str("we've"), py_str("our"), py_str("there's"), py_str("there'll"), py_str("the"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("a"), py_str("an"), py_str("my"), py_str("his"), py_str("her"), py_str("its"), py_str("just"), py_str("not"), py_str("no"), py_str("and"), py_str("but"), py_str("so"), py_str("also"), py_str("then"), py_str("now"), py_str("still"), py_str("only"), py_str("even"), py_str("both"), py_str("all"), py_str("every"), py_str("at"), py_str("in"), py_str("on"), py_str("for"), py_str("after"), py_str("about"), py_str("around"), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("half"), py_str("someone"), py_str("something"), py_str("nothing"), py_str("theres"), py_str("theyll")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_relation(args: &[Py]) -> Py {
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
    let mut candidate: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The relation a bare possessive names (\"dads\" -> \"dad\"), or \"\" when it names none.");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,;:!?'")]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![said.clone(), if py_truthy(&mcall(&said.clone(), "endswith", &[py_str("s")])) { slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { py_str("") }, mcall(&said.clone(), "replace", &[py_str(" "), py_str("")])]));
    for __item in __iter1 {
    candidate = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &candidate.clone()))) {
    return candidate.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_pronoun(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut relation: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "relation") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: relation"))) };
    let _ = py_str("\"he\" or \"she\" for a relation that says it, and \"\" for one that does not.");
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_HE(), &relation.clone()))) {
    return py_str("he");
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_SHE(), &relation.clone()))) {
    return py_str("she");
    }
    return py_str("");
    Py::None
}
pub fn _the_ages_held(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let _ = py_str("{relation: age} for the people the chat has already been given an age for.");
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    held = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = py_call(&the_others, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&entry.clone(), "get", &[py_str("age")]), &Py::None)) } }) {
    let _ = py_setitem(&held.clone(), &mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), mcall(&entry.clone(), "get", &[py_str("age")]));
    }
    }
    return held.clone();
    Py::None
}
pub fn _people_in_chat(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("\"the youngest\" and \"the other one\" are people only in a chat that has people in it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    user = { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&user.clone(), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { mcall(&user.clone(), "get", &[py_str("kids")]) } } } }) {
    return Py::Bool(true);
    }
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    if py_truthy(&py_call(&the_others, &[memory.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::facts::g_RELATIONS()) { let mut one: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bmy\\s+"), &stdlib_call("re", "escape", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_person_named_alone(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut only: Py = Py::None;
    let mut ours: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut said: Py = Py::None;
    let mut several: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The question a person named on its own asks for, or \"\".\n\n    \"and my son\", \"the twins\", \"same for my daughter\": the line is a person and no fact, so there\n    is nothing to say back and one thing to ask. The store may already hold the age, and then the\n    age is not the question - what they are into is.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    only = { let __b = crate::memory::named::the_one_asked_about(&[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]) } };
    match_ = mcall(&g_A_PERSON_ALONE(), "match", &[mcall(&py_str(" "), "join", &[mcall(&only.clone(), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,;:!?'")]);
    said = mcall(&g_WHICH_ONE(), "sub", &[py_str(""), said.clone()]);
    ours = Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &mcall(&mcall(&match_.clone(), "group", &[py_str("det")]), "lower", &[])));
    word = if py_truthy(&{ let __b = mcall(&said.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) { slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { said.clone() };
    several = { let __b = mcall(&said.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } };
    place = mcall(&g_A_POSITION(), "get", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "lower", &[]), "split", &[])])]);
    if py_truthy(&place.clone()) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_POSITION_MAYBE_A_THING(), &place.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_people_in_chat(&[memory.clone()]))) } }) {
    return py_str("");
    }
    pool = { let __b = mcall(&g_A_POSITION_LINES(), "get", &[place.clone()]); if py_truthy(&__b) { __b } else { g_A_PERSON_ALONE_LINES() } };
    who = op_add(&if py_truthy(&ours.clone()) { py_str("Your ") } else { py_str("The ") }, &place.clone());
    return mcall(&mcall(&mcall(&mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), said.clone()]), "replace", &[py_str("{Who}"), who.clone()]), "replace", &[py_str("{they}"), py_str("they")]), "replace", &[py_str("{them}"), py_str("them")]), "replace", &[py_str("{do}"), py_str("do")]);
    }
    group = { let __b = mcall(&g_A_GROUP_ALONE(), "get", &[said.clone()]); if py_truthy(&__b) { __b } else { if py_truthy(&several.clone()) { op_add(&slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("s")) } else { py_str("") } } };
    if py_truthy(&group.clone()) {
    who = op_add(&if py_truthy(&ours.clone()) { py_str("Your ") } else { py_str("The ") }, &group.clone());
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_A_GROUP_ALONE_LINES(), memory.clone(), said.clone()]), "format", args))(&__args) };
    }
    relation = _a_relation(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&relation.clone()))) {
    return py_str("");
    }
    return _the_question_about(&[relation.clone(), said.clone(), ours.clone(), memory.clone()]);
    Py::None
}
pub fn _the_question_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut relation: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "relation") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: relation"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut ours: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ours") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ours"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut name: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { py_str("") };
    let mut known: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut them: Py = Py::None;
    let mut they: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The question a friend asks about this person, said back their way.\n\n    \"your son\" for a person who said \"my son\", and the name when the chat has one: \"Jack - what\n    does he get up to?\" rather than \"your son\" for somebody whose name was already given.\n    ");
    pronoun = _the_pronoun(&[relation.clone()]);
    let __unpk1 = py_unpack(&if py_truthy(&pronoun.clone()) { py_tuple(vec![pronoun.clone(), if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("he")))) { py_str("him") } else { py_str("her") }]) } else { py_tuple(vec![py_str("they"), py_str("them")]) }, 2, None);
    they = __unpk1[0].clone();
    them = __unpk1[1].clone();
    who = { let __b = name.clone(); if py_truthy(&__b) { __b } else { op_add(&if py_truthy(&ours.clone()) { py_str("Your ") } else { py_str("The ") }, &relation.clone()) } };
    known = Py::Bool(py_contains(&_the_ages_held(&[memory.clone()]), &relation.clone()));
    pool = if py_truthy(&known.clone()) { g_A_KNOWN_AGE_LINES() } else { g_A_PERSON_ALONE_LINES() };
    line = pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), said.clone()]);
    return mcall(&mcall(&mcall(&mcall(&line.clone(), "replace", &[py_str("{Who}"), who.clone()]), "replace", &[py_str("{they}"), they.clone()]), "replace", &[py_str("{them}"), them.clone()]), "replace", &[py_str("{do}"), if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she")]), &they.clone()))) { py_str("does") } else { py_str("do") }]);
    Py::None
}
pub fn a_person_pointed_at(args: &[Py]) -> Py {
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
    let mut all_named: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut only: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The question about the person a \"him\" or \"her\" or \"them\" points at, or \"\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    only = crate::memory::named::the_one_asked_about(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&only.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_POINTED_AT(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&only.clone(), "lower", &[]), "split", &[])])]))) } }) {
    return py_str("");
    }
    word = mcall(&mcall(&only.clone(), "lower", &[]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("him"), py_str("her"), py_str("them"), py_str("they"), py_str("that one"), py_str("this one")]), &word.clone())); if !py_truthy(&__b) { __b } else { op_lt(&_how_many_people(&[memory.clone()]), &Py::Int(2)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("him"), py_str("her"), py_str("them"), py_str("they"), py_str("that one"), py_str("this one")]), &word.clone()))) {
    all_named = _the_people_pointed_at(&[memory.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[all_named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    who = mcall(&py_str(" and "), "join", &[slice_or_raise(&all_named.clone(), &Py::None, &Py::Int(2), &Py::None)]);
    pool = if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&all_named.clone(), &Py::None, &Py::Int(2), &Py::None)) { let mut one: Py = __item.clone(); __out.push(_an_age_held(&[memory.clone(), one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { g_A_TWO_ALONE_LINES() } else { g_A_GROUP_ALONE_LINES() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), who.clone()]), "format", args))(&__args) };
    }
    }
    person = _the_person_pointed_at(&[word.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&person.clone(), 2, None);
    relation = __unpk1[0].clone();
    name = __unpk1[1].clone();
    return _the_question_about(&[relation.clone(), relation.clone(), Py::Bool(true), memory.clone(), name.clone()]);
    Py::None
}
pub fn _how_many_people(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let _ = py_str("How many people of theirs the chat holds.");
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    return call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = py_call(&the_others, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_people_pointed_at(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let _ = py_str("The names of the people the chat holds, in the order they were given, named only.");
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    names = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = py_call(&the_others, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&crate::memory::facts::g_RELATIONS(), &mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))) {
    continue;
    }
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[]))) } }) {
    continue;
    }
    name = op_add(&mcall(&geti(&name.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(!py_contains(&names.clone(), &name.clone()))) {
    let _ = mcall(&names.clone(), "append", &[name.clone()]);
    }
    }
    return names.clone();
    Py::None
}
pub fn _an_age_held(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let _ = py_str("Whether the chat already has an age for the person of this name.");
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    let __iter1 = py_iter_or_raise(&{ let __b = py_call(&the_others, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    held = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&held.clone(), &mcall(&mcall(&name.clone(), "strip", &[]), "lower", &[]))); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("age")]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_person_pointed_at(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut name: Py = Py::None;
    let mut people: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("(relation, name) for the person the word points at, or None when the chat has nobody.");
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    wanted = mcall(&py_dict(vec![(py_str("him"), crate::memory::facts::g_HE()), (py_str("her"), crate::memory::facts::g_SHE())]), "get", &[word.clone()]);
    people = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = py_call(&the_others, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(entry.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[people.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    relation = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&crate::memory::facts::g_RELATIONS(), &relation.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&wanted.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&wanted.clone(), &relation.clone())) } }) {
    continue;
    }
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[]) } }) {
    name = op_add(&mcall(&geti(&name.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_tuple(vec![relation.clone(), name.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn a_second_person(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut first: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The ask for the date of the fact this line leaves out, or \"\".\n\n    \"my mums birthday is in may\" / \"and my dads\" is one fact about two people, and the second\n    half says only who. The fact before it is the one being carried over, so the reply keeps it\n    and asks for the part the line did not give - the date - instead of saying the noun back.\n    Only a line that names a person and nothing else comes here, and only when the turn before it\n    was a dated fact about a different person, so a line that changes the subject is left alone.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    match_ = mcall(&g_A_SECOND_PERSON(), "match", &[mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    who = _a_relation(&[mcall(&match_.clone(), "group", &[py_str("who")])]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return py_str("");
    }
    before = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return py_str("");
    }
    first = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::facts::g_RELATIONS()) { let mut one: Py = __item.clone(); if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bmy\\s+"), &stdlib_call("re", "escape", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("")), before.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&first.clone(), &who.clone())) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_BIRTHDAY_TOLD(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DATE_TOLD(), "search", &[before.clone()]) } }))) {
    return py_str("");
    }
    said = op_add(&py_str("your "), &who.clone());
    if py_truthy(&mcall(&g_A_BIRTHDAY_TOLD(), "search", &[before.clone()])) {
    return op_add(&op_add(&py_str("And "), &said.clone()), &py_str("'s birthday — when is that?"));
    }
    verb = mcall(&g_A_PLAN_VERB(), "search", &[before.clone()]);
    pronoun = _the_pronoun(&[who.clone()]);
    if py_truthy(&{ let __b = verb.clone(); if !py_truthy(&__b) { __b } else { pronoun.clone() } }) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("And "), &said.clone()), &py_str(" — when's ")), &pronoun.clone()), &py_str(" ")), &mcall(&mcall(&verb.clone(), "group", &[py_str("verb")]), "lower", &[])), &py_str("?"));
    }
    return op_add(&op_add(&py_str("And "), &said.clone()), &py_str(" — when is that?"));
    Py::None
}
pub fn _the_last_reply_gave_a_plan(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's last reply told the person something to do, or how to choose it.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    said = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str(":"))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_PLAN_WAS_GIVEN(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return crate::memory::stuck::_the_answer_gave_a_step(&[memory.clone()]);
    Py::None
}
pub fn _a_limit_on_the_plan(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line's own limit put in front of the plan the last reply gave, or None.\n\n    Only after a step: \"i still have the school run to sort\" before the bot has said anything is a\n    fact to take in, and the store files it. After a step it is the person narrowing the plan, and\n    what a friend says is that the named thing goes first.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_the_last_reply_gave_a_plan(&[memory.clone()])))) {
    return Py::None;
    }
    found = mcall(&g_STILL_TO_DO(), "search", &[read.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "split", &[])]), "strip", &[py_str(" .,!?:;")]);
    words = mcall(&thing.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __cmp11: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(4)) } else { Py::Bool(false) } })); if py_truthy(&__b) { __b } else { mcall(&g_NOT_ONE_THING(), "search", &[thing.clone()]) } }) {
    return Py::None;
    }
    pool = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_LIMIT_GOES_FIRST()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), read.clone()]);
    Py::None
}
pub fn _a_day_coming(args: &[Py]) -> Py {
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
    let mut coming: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Which pool a line about a day ahead wants, or None when it is not one.\n\n    Only the day words matter here: the thing itself is the line, said back by the caller, so a\n    pool that says one true thing about a date ahead fits any line with a date on it.\n    ");
    said = { let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = mcall(&g_A_DAY_ASKED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DAY_BEEN_AND_GONE(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_DAY_JUST_A_STATE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_DOING_TURNED_DOWN(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_DOING(), "search", &[said.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_DAY_TO_LOOK_FORWARD_TO(), "search", &[said.clone()])) {
    return g_SORRY_TAIL();
    }
    if py_truthy(&mcall(&g_DAY_FAR(), "search", &[said.clone()])) {
    return g_FAR_TAIL();
    }
    if py_truthy(&mcall(&g_DAY_SOON(), "search", &[said.clone()])) {
    return g_SOON_TAIL();
    }
    return Py::None;
    coming = _a_day_coming(&[clean.clone()]);
    if py_truthy(&coming.clone()) {
    return coming.clone();
    }
    Py::None
}
pub fn reads_as_a_receipt(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut readings: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a reply is one of the frames above with the person's words in it.\n\n    A receipt with the composer's own question after it counts too: \"Two kids - good to know.\n    How old are they?\" is the same nod with a follow-up stuck on the back of it, and every\n    caller here asks this question exactly to know whether there is anything to stand behind.\n    Read whole, the question after the frame stopped the match, and \"what would you do\" after\n    that turn was answered \"Same as I said: two kids - good to know.\" - the bot repeating a nod\n    as though it were a view.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    readings = py_tuple(vec![said.clone(), mcall(&g__SPACED_DASH(), "sub", &[py_str(", "), said.clone()])]);
    let __iter1 = py_iter_or_raise(&readings.clone());
    for __item in __iter1 {
    reading = __item.clone();
    if py_truthy(&_one_frame(&[reading.clone()])) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&readings.clone());
    for __item in __iter2 {
    reading = __item.clone();
    first = geti(&mcall(&g__SENTENCE(), "split", &[reading.clone()]), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&first.clone(), &reading.clone())); if !py_truthy(&__b) { __b } else { _one_frame(&[first.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _one_frame(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut front: Py = Py::None;
    let mut middle: Py = Py::None;
    let _ = py_str("True when the whole line is one frame with the person's words inside it.");
    let __iter1 = py_iter_or_raise(&g__SAID_BACK());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    front = __unpk2[0].clone();
    back = __unpk2[1].clone();
    if py_truthy(&{ let __b = front.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "startswith", &[front.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = back.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "endswith", &[back.clone()]))) } }) {
    continue;
    }
    middle = mcall(&slice_or_raise(&said.clone(), &call_builtin("len", &[front.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_sub(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[back.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), "strip", &[py_str(" —-.:,;")]);
    if py_truthy(&middle.clone()) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _more_than_one_coming(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut cut: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the line names more than one person coming, from where \"my\"/\"our\" starts.\n\n    `word` is the first word after \"my\"/\"our\", which is the one the pronoun used to be read from.\n    A plural there is enough on its own, and so is an \"and\" before the coming word: \"my mum and\n    dad are coming\" and \"my aunt and uncle are visiting\" are two visitors, and the question after\n    the receipt has to be the one for a pair.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_PLURAL_KIN(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::memory::facts::g_RELATIONS(), &word.clone())) } } } }) {
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:are|were|were both|are both)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    rest = slice_or_raise(&said.clone(), &start.clone(), &Py::None, &Py::None);
    cut = mcall(&g_A_COMING_WORD(), "search", &[rest.clone()]);
    if py_truthy(&cut.clone()) {
    rest = slice_or_raise(&rest.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\band (?:i|me|we|us)\\b"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\band\\b|&"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_trip(args: &[Py]) -> Py {
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
    let mut after_to: Py = Py::None;
    let mut destination: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut head: Py = Py::None;
    let mut named: Py = Py::None;
    let mut two_word: Py = Py::None;
    let _ = py_str("\"trip\", \"occasion\" or None: a plan that is coming, told to a friend.\n\n    A friend answers \"we're going to spain in july\" with something about Spain, and an occasion\n    with something about the day, not with a coach's \"what's the first step you're unsure\n    about?\". The composer asks this so a trip is not read as a project with a first step, and\n    the echo asks it so it can answer the trip rather than file it. Only another person's trip\n    is left alone: \"my sister is going to spain in july\" is not the speaker's to be asked\n    about.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&clean.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    flat = mcall(&py_str(" "), "join", &[mcall(&mcall(&clean.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DATE_AHEAD(), "search", &[flat.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_OBLIGED(), "search", &[flat.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), flat.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_PAST_PLAN())) {
    return Py::None;
    }
    if py_truthy(&mcall(&flat.clone(), "startswith", &[py_tuple(vec![py_str("my "), py_str("her "), py_str("his "), py_str("their "), py_str("she "), py_str("he ")])])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_TRIP_VERB(), "search", &[flat.clone()])))) {
    return Py::None;
    }
    destination = mcall(&g_DESTINATION(), "search", &[flat.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&destination.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_ARRIVE_VERB(), "search", &[flat.clone()]) } }) {
    destination = mcall(&g_DESTINATION_ARRIVAL(), "search", &[flat.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&destination.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_TRIP(), &mcall(&destination.clone(), "group", &[Py::Int(1)]))) } }) {
    return Py::None;
    }
    after_to = slice_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z'’]+"), slice_or_raise(&flat.clone(), &mcall(&destination.clone(), "start", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1), &Py::Int(5), &Py::None);
    if py_truthy(&crate::memory::not_a_trip::not_a_destination(&[after_to.clone(), if py_truthy(&after_to.clone()) { geti(&after_to.clone(), &Py::Int(0)) } else { py_str("") }])) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[after_to.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_TRIP())) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[after_to.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_OCCASIONS())) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&after_to.clone(), &py_str("shower"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[after_to.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SHOWER()))) } }) {
    return Py::None;
    }
    return if py_truthy(&mcall(&g_DREADED_OCCASION(), "search", &[flat.clone()])) { py_str("occasion_flat") } else { py_str("occasion") };
    }
    head = if py_truthy(&after_to.clone()) { geti(&after_to.clone(), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THING_BEFORE(), &head.clone())); if py_truthy(&__b) { __b } else { mcall(&head.clone(), "endswith", &[py_tuple(vec![py_str("'s"), py_str("’s"), py_str("s'")])]) } }) {
    named = if py_truthy(&op_gt(&call_builtin("len", &[after_to.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&after_to.clone(), &Py::Int(1)) } else { py_str("") };
    two_word = mcall(&py_str(" "), "join", &[slice_or_raise(&after_to.clone(), &Py::Int(1), &Py::Int(3), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&head.clone(), &py_str("the"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACES_WITH_THE(), &named.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES_WITH_THE(), &two_word.clone())) } } } }))) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::None;
    }
    return py_str("trip");
    Py::None
}
pub fn _the_plan_in_hand(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Which plan the last few turns are on: \"stay\", \"trip\", or None.\n\n    A limit only means something against the plan it is a limit on, and the two plans want\n    different frames: a guest who cannot manage the stairs is a bed and a bathroom, while a person\n    who cannot walk far on a day out is a short, flat route. Read from the line itself and the\n    last few turns, so a limit said with nobody planning anything is still the pools' business.\n    ");
    lines = py_list(vec![]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&clean.clone(), "strip", &[]) } }) {
    let _ = mcall(&lines.clone(), "append", &[clean.clone()]);
    }
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_PLAN_TURNS()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    let _ = mcall(&lines.clone(), "append", &[mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])])]);
    }
    }
    }
    let __iter2 = py_iter_or_raise(&lines.clone());
    for __item in __iter2 {
    said = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_A_STAY_PLAN(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_STAY_PERSON(), "search", &[said.clone()]) } }) {
    return py_str("stay");
    }
    }
    let __iter3 = py_iter_or_raise(&lines.clone());
    for __item in __iter3 {
    said = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_A_TRIP_PLAN(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::facts::a_limit_on_the_plan(&[said.clone()]))) } }) {
    return py_str("trip");
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_like_of_one_thing(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut echo: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line's whole echo is one thing the person likes.");
    echo = _echo(&[clean.clone(), memory.clone()]);
    match_ = mcall(&g_A_LIKE_ECHO(), "match", &[{ let __b = echo.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    thing = mcall(&match_.clone(), "group", &[py_str("thing")]);
    if py_truthy(&{ let __b = mcall(&g_A_LIKE_WITH_TWO_HALVES(), "search", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PERSON_LIKED(), "search", &[thing.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = crate::memory::facts::g_RELATIONS(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), thing.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _only_a_liking(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut key: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Whether the line filed a like and nothing else worth an answer of its own.\n\n    The same guard as `_just_a_liking`, but a pet said in the like (\"i love my dog\") is not a\n    second thing to answer: the store files the animal and then answers \"Noted: you love your\n    dog.\" A name, a job or a limit in the same line is a fact of its own and keeps its own ending.\n    ");
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "items", &[])) { let __unpk6153 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk6153[0].clone(); let mut value: Py = __unpk6153[1].clone(); if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("tastes"), py_str("pets")]), &key.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&key.clone(), "endswith", &[py_str("_frame")]))) } } } }) {  __out.push(key.clone()); } } py_set(__out) })() };
    return { let __b = call_builtin("bool", &[mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("tastes")])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rest.clone())) } };
    Py::None
}
pub fn a_liked_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut entry: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing a stated like is about, or \"\" when the line filed no like.\n\n    Only \"how\" of \"yes\" is a like: \"i hate mushrooms\" files the same shape with \"no\", and the\n    dislike is answered by the composer's own question about the thing.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("tastes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("how")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("yes"))) } }) {
    continue;
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" ,.:;!?")]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&thing.clone(), &py_str("{"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&thing.clone(), &py_str("}"))); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&g_A_LIKE_OF_ONE_THING(), "match", &[thing.clone()]) } } } } } } } }) {
    return thing.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn a_limit_reply(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The limit said back with what the bot will do with it, or None when the line is more.\n\n    Read before the pools a limit has already been misread by, so it has to say nothing at all\n    unless the line really is only a limit: \"im going to keep it cheap this month\" is a month\n    they have planned, not a limit to promise anything about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__OPENS_WITH_AN_ASK(), "match", &[clean.clone()])) {
    return Py::None;
    }
    family = crate::memory::facts::a_limit_on_the_plan(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__AN_ABILITY(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_plan_on_the_table(&[memory.clone(), clean.clone()]))) } }) {
    return Py::None;
    }
    found = _learned(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_le(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &g__LIMIT_ONLY()))) } }) {
    return Py::None;
    }
    return _a_limit_reply(&[clean.clone(), family.clone(), memory.clone()]);
    Py::None
}
pub fn _a_plan_on_the_table(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether there is a plan for the limit to be about.\n\n    A limit is a narrowing of something. Said on its own it is a remark about their life (\"im on a\n    budget\", \"im not very energetic\"), and those are answered better by the pools that read a\n    budget as the cheap-meals question and a mood as a mood. It is a limit on a plan when the plan\n    was said in the last few turns, was filed by the store, or is being said here in the same\n    breath (\"i want to keep it cheap\").\n    ");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_WANTS_IT(), "search", &[clean.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    continue;
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &g_PLAN_TURNS())) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::memory::facts::g_PLAN(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::facts::a_limit_on_the_plan(&[said.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&{ let __b = crate::memory::facts::extract(&[said.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plan")])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_limit_reply(args: &[Py]) -> Py {
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
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut after: Py = Py::None;
    let mut body: Py = Py::None;
    let mut frames: Py = Py::None;
    let _ = py_str("The limit said back, with what the bot will do with it.");
    body = { let __b = _echo(&[clean.clone()]); if py_truthy(&__b) { __b } else { clean.clone() } };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("move"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_the_plan_in_hand(&[memory.clone(), clean.clone()]), &py_str("stay"))) } }) {
    family = py_str("move_stay");
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&body.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SAID_BACK_WORDS())) {
    after = mcall(&g_LIMIT_AFTER_THE_LINE(), "get", &[family.clone()]);
    if py_truthy(&after.clone()) {
    return _pick_ending(&[after.clone(), memory.clone()]);
    }
    }
    frames = { let __b = mcall(&g_LIMIT_FRAMES(), "get", &[family.clone()]); if py_truthy(&__b) { __b } else { geti(&g_LIMIT_FRAMES(), &py_str("other")) } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frames.clone()); __args.push(memory.clone()); __kws.extend(vec![("thing".to_string(), body.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&body.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick_ending(args))(&__args) };
    Py::None
}
pub fn a_limit_for_the_plan(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut family: Py = Py::None;
    let _ = py_str("A limit on the plan in hand, or None when no plan is in hand or no frame fits it.\n\n    Read by the chat module before it hands back the answer it already gave: a limit said about\n    the plan (\"shes 78 and cant do stairs\" after \"my mum is coming to stay\") is the one thing that\n    should change the answer, and the card went round three times instead. It only speaks when a\n    plan of a kind the frames are written for is on the table, so a limit in a chat about a diet\n    or a dripping tap keeps the reply it has.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g__OPENS_WITH_AN_ASK(), "match", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    family = crate::memory::facts::a_limit_on_the_plan(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::memory::there_is_a_chat(&[memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&_the_plan_in_hand(&[memory.clone()]), &py_str("stay")))) {
    return Py::None;
    }
    return _a_limit_reply(&[clean.clone(), family.clone(), memory.clone()]);
    Py::None
}
pub fn reads_as_cruelty(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Whether this line says somebody is doing the person harm.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__BULLIED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_NAME_WAS_VIOLENT(), "search", &[text.clone()]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_HURTS_A_PERSON(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_an_ask(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this line asks something, in a shape a person types.");
    return { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_AN_ASK(), "match", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _shape_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut found: Py = Py::None;
    let mut number: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("Which kind of clock this deadline is: today, tomorrow, days, or room.\n\n    \"Two days\" is not \"three weeks\": a figure counted in days leaves a step a day, and a figure\n    counted in weeks or months is room to spread the whole thing out. The pools are one per shape\n    so a deadline tonight never gets a sentence written for one three weeks off.\n    ");
    if py_truthy(&mcall(&g_WHEN_TODAY(), "search", &[when.clone()])) {
    return py_str("today");
    }
    if py_truthy(&mcall(&g_WHEN_TOMORROW(), "search", &[when.clone()])) {
    return py_str("tomorrow");
    }
    found = mcall(&g_COUNTED(), "search", &[when.clone()]);
    if py_truthy(&found.clone()) {
    number = mcall(&g_COUNT(), "get", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), mcall(&found.clone(), "group", &[Py::Int(1)])]);
    unit = mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "lower", &[]);
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) {
    return py_str("room");
    }
    if py_truthy(&op_le(&call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), &if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("day")])) { Py::Int(5) } else { Py::Int(1) })) {
    return py_str("days");
    }
    return py_str("room");
    }
    if py_truthy(&mcall(&g_WHEN_DAYS(), "search", &[when.clone()])) {
    return py_str("days");
    }
    return py_str("room");
    Py::None
}
pub fn _clock_shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut found: Py = Py::None;
    let mut hour: Py = Py::None;
    let mut said: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("Which half of the day the hour is in: \"morning\" or \"after\".\n\n    \"At ten\" is the morning, \"at 2:30\" is the afternoon, and \"half nine\" is half past nine in the\n    morning - so a bare figure in the early hours is read as the afternoon, which is what a\n    person means when they say an appointment is at two.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    found = stdlib_call("re", "search", &[py_str("\\d{1,2}|[a-z]+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("morning");
    }
    token = mcall(&found.clone(), "group", &[Py::Int(0)]);
    hour = if py_truthy(&Py::Bool(!py_truthy(&mcall(&token.clone(), "isdigit", &[])))) { mcall(&g_CLOCK_WORDS(), "get", &[token.clone()]) } else { call_builtin("int", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&Py::Bool(py_is(&hour.clone(), &Py::None))) {
    return py_str("morning");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("pm"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("p.m"))) } }) {
    return py_str("after");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("am"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("a.m"))) } }) {
    return py_str("morning");
    }
    if py_truthy(&op_le(&hour.clone(), &Py::Int(3))) {
    return py_str("after");
    }
    return if py_truthy(&op_lt(&hour.clone(), &Py::Int(12))) { py_str("morning") } else { py_str("after") };
    Py::None
}
pub fn _tells_them_what_to_do(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let _ = py_str("Whether the answer just given was advice they can act on, rather than a list of facts.");
    opening = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    return call_builtin("bool", &[mcall(&g_STEP_OPENS(), "match", &[mcall(&opening.clone(), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_bare_state(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line is nothing but a state of the body (\"im bunged up\", \"im shaky\").");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&clean.clone(), "replace", &[py_str("-"), py_str(" ")]), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_list(vec![py_str("im")]), py_list(vec![py_str("i'm")]), py_list(vec![py_str("iam")])]), &slice_or_raise(&words.clone(), &Py::None, &Py::Int(1), &Py::None)))) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    } else {
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&words.clone(), &Py::None, &Py::Int(2), &Py::None), &py_list(vec![py_str("i"), py_str("am")])))) {
    words = slice_or_raise(&words.clone(), &Py::Int(2), &Py::None, &Py::None);
    } else {
    return Py::Bool(false);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_BODY_STATE_PHRASES(), &mcall(&py_str(" "), "join", &[words.clone()])))) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BODY_STATE_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_BODY_STATE_WORDS(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BODY_STATE_TAIL(), &w.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn has_news(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line carries something that happened, as opposed to how they feel.");
    text = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    return call_builtin("bool", &[{ let __b = mcall(&g_A_EVENT(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_CLEAR(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_RESULT(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_MOVED(), "search", &[text.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _repaired(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut understood: Py = Py::None;
    let _ = py_str("The line with the words that were wrong repaired, for the guards that match English.\n\n    `clean` stays exactly as typed, because that is what the echo says back to the person -\n    repeating \"kiling\" at somebody who can spell it is the cold part of \"Got it\". But a symptom,\n    a fault and bad news are matched on the words of English, so a slip hides them: \"my back is\n    kiling me\" is a body that hurts, not a fact to file away. The repaired line is the engine's\n    own reading (`understood[\"plain\"]`), and only when that reading is of this line.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return clean.clone();
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("clean")]), &mcall(&clean.clone(), "lower", &[])))) {
    return clean.clone();
    }
    return { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { clean.clone() } };
    Py::None
}
pub fn _the_values_measures(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut glued: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kinds of measure the value is already said in: \"10 km\" -> {\"distance\"}.\n\n    A unit word anywhere in the value counts, not only the last one, because \"4 miles each way\"\n    carries its miles in the middle and appending \"miles\" to it said the unit twice. \"three times\"\n    names none: \"times\" starts a measure rather than being one (see VALUE_UNIT_WORDS), so the\n    \"a week\" of the line before is still needed to finish it. \"twice\" does name one, and so does\n    \"daily\": a frequency said in one word needs nothing put after it.\n    ");
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_WHOLE_FREQUENCY(), &mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:'’")]), "lower", &[])))) {
    let _ = mcall(&found.clone(), "add", &[py_str("time")]);
    }
    word = mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:'’")]), "lower", &[]);
    glued = mcall(&g_VALUE_UNIT_GLUED(), "match", &[word.clone()]);
    if py_truthy(&glued.clone()) {
    word = mcall(&mcall(&glued.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_VALUE_UNIT_WORDS(), &word.clone()))) {
    let _ = mcall(&found.clone(), "add", &[mcall(&g_A_MEASURE(), "get", &[word.clone(), word.clone()])]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_units_measure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut found: Py = Py::None;
    let mut measure: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kinds of measure the unit of the line before names: \"every morning\" -> {\"time\"}.");
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    measure = mcall(&g_A_MEASURE(), "get", &[mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:'’")]), "lower", &[])]);
    if py_truthy(&measure.clone()) {
    let _ = mcall(&found.clone(), "add", &[measure.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_unit_is_said_twice(args: &[Py]) -> Py {
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
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut here: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("Whether saying the line's unit after this value would measure the same thing twice.\n\n    \"5 km\" then \"10 km\" (or \"10km\", or \"10 miles\", or \"4 miles each way\"): the line before\n    measures a distance and so does the new figure, so its unit on the end is a second measure of\n    the same thing (\"10 km km, then.\"). \"5k every morning\" then \"10k\" is different - a distance\n    and a routine - and the routine still belongs to the line, which is why \"Got it - 10k every\n    morning.\" stays right there.\n    ");
    here = _the_values_measures(&[value.clone()]);
    there = _the_units_measure(&[unit.clone()]);
    if py_truthy(&{ let __b = here.clone(); if !py_truthy(&__b) { __b } else { there.clone() } }) {
    return call_builtin("bool", &[op_bitand(&here.clone(), &there.clone())]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _without_a_frame(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut back: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { Py::Bool(false) };
    let mut end: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut took: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words left when a frame phrase is taken off one end, and whether one was taken.");
    words = call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    took = Py::Bool(false);
    while py_truthy(&words.clone()) {
    let __iter1 = py_iter_or_raise(&phrases.clone());
    let mut __else2 = false;
    for __item in __iter1 {
    phrase = __item.clone();
    end = if py_truthy(&back.clone()) { slice_or_raise(&words.clone(), &op_neg(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None) } else { slice_or_raise(&words.clone(), &Py::None, &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None) };
    said = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&end.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?;:'’")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&said.clone(), &phrase.clone()))) {
    words = if py_truthy(&back.clone()) { slice_or_raise(&words.clone(), &Py::None, &op_neg(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None) } else { slice_or_raise(&words.clone(), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None) };
    took = Py::Bool(true);
    break;
    }
    }
    break;
    }
    return py_tuple(vec![words.clone(), took.clone()]);
    Py::None
}
pub fn _without_a_not(args: &[Py]) -> Py {
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
    let _ = py_str("The words with a trailing \"not <figure>\" taken off, which is the figure being taken back.");
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(2))), "lower", &[]), "strip", &[py_str(".,!?")]), &py_str("not"))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), geti(&words.clone(), &op_neg(&Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return py_tuple(vec![slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), Py::Bool(true)]);
    }
    return py_tuple(vec![call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(false)]);
    Py::None
}
pub fn _the_bare_value(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut led: Py = Py::None;
    let mut tailed: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The figure a correction names when it has no verb in it, or \"\" when it is not one.\n\n    The frame has to be there - a figure on its own (\"1150\") answers a question as often as it\n    corrects a line - and the figure is what is left between the frame in front and the one\n    behind. \"1150, not 1200\" keeps 1150: the figure after \"not\" is the one being taken back.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[py_str(" .!?")])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&_without_a_frame(&[words.clone(), g_PUT_RIGHT_LEAD()]), 2, None);
    words = __unpk1[0].clone();
    led = __unpk1[1].clone();
    let __unpk2 = py_unpack(&_without_a_not(&[words.clone()]), 2, None);
    words = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(g_PUT_RIGHT_TAIL()); __kws.extend(vec![("back".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_a_frame(args))(&__args) }, 2, None);
    words = __unpk3[0].clone();
    tailed = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = led.clone(); if py_truthy(&__b) { __b } else { tailed.clone() } }))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return py_str("");
    }
    return mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" .,!?")]);
    Py::None
}
pub fn value_put_right(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut bare: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The correction said back, or \"\" when the line is not one that only puts a value right.\n\n    Two things have to hold. The line has to be the frame and a value and nothing else (\"make it\n    four\", \"change that to 10k\"): \"my boiler is broken\" then \"make it stop\", and \"a new sofa\"\n    then \"make it bigger\", are the rest of a sentence and stay with the modules that read it.\n    And the line before has to measure something (\"three *times a week*\", \"5k *every morning*\"),\n    because a figure said back with no unit is the bot claiming a value it does not hold — which\n    is the one thing the store's correction refuses (see FalseFactTest in tests/test_memory.py).\n    ");
    match_ = mcall(&g_MAKE_A_VALUE(), "match", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    bare = py_str("");
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    bare = _the_bare_value(&[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return py_str("");
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_line_before(&[memory.clone()])))) {
    return py_str("");
    }
    value = mcall(&mcall(&py_str(" "), "join", &[mcall(&if py_truthy(&match_.clone()) { mcall(&match_.clone(), "group", &[py_str("value")]) } else { bare.clone() }, "split", &[])]), "strip", &[py_str(" .")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&value.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&value.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?'’")])); } py_list(__out) })() };
    if py_truthy(&bare.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), value.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_VALUE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHOLE_FREQUENCY(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return py_str("");
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), value.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_VALUE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_DAYS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHOLE_FREQUENCY(), &word.clone())) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return py_str("");
    }
    }
    unit = _the_unit_before(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return py_str("");
    }
    unit = _without_a_unit_already_in_the_value(&[value.clone(), unit.clone()]);
    if py_truthy(&_the_unit_is_said_twice(&[value.clone(), unit.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("value".to_string(), value.clone()), ("Value".to_string(), op_add(&mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CORRECTED_ALONE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("value".to_string(), value.clone()), ("Value".to_string(), op_add(&mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None))), ("unit".to_string(), unit.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CORRECTED_WITH_UNIT()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _without_a_unit_already_in_the_value(args: &[Py]) -> Py {
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
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut last: Py = Py::None;
    let mut unit_words: Py = Py::None;
    let mut value_words: Py = Py::None;
    let _ = py_str("The unit of the line before with the word the value already ends in taken off.\n\n    \"i swim 3 times a week\" then \"sorry 5 times\" is \"5 times a week\" - the value carries the\n    \"times\" and the line carries the \"a week\" - and it was \"5 times times a week, then.\" because\n    the unit of the line before is read from after its figure, \"times a week\", and put on a value\n    that had just said \"times\" itself.\n    ");
    unit_words = mcall(&call_builtin("str", &[{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    value_words = mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[unit_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&value_words.clone())) } }) {
    return unit.clone();
    }
    last = mcall(&mcall(&geti(&value_words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?;:'’")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_eq(&last.clone(), &mcall(&mcall(&geti(&unit_words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?;:'’")]), "lower", &[])))) {
    return mcall(&py_str(" "), "join", &[slice_or_raise(&unit_words.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    }
    return unit.clone();
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line the person said just before, or \"\" when there is none.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&last.clone(), "get", &[py_str("user")]) } else { Py::None };
    return if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { said.clone() } else { py_str("") };
    Py::None
}
pub fn _the_unit_before(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut following: Py = Py::None;
    let mut index: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words the line before measures its figure in: \"three times a week\" -> \"times a week\".\n\n    \"i run 5k every morning\" -> \"every morning\". Nothing when that line has no figure at all\n    (\"im tired\") or when its figure is followed by nothing that measures it (\"i finish at 5\",\n    \"my appointment is on the 3rd\"): a new figure with no unit of the old one is a number the\n    bot would be making up a home for.\n    ");
    said = _the_line_before(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")])) {  __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:")])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    figure = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &figure.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), figure.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_VALUE_WORDS(), &figure.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_DAYS(), &figure.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &figure.clone())) } } } } } }))) {
    continue;
    }
    tail = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(4)), &Py::None));
    for __item in __iter3 {
    following = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_UNITS_AFTER(), &mcall(&following.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&tail.clone(), "append", &[mcall(&following.clone(), "lower", &[])]);
    continue;
    }
    break;
    }
    if py_truthy(&tail.clone()) {
    return mcall(&py_str(" "), "join", &[tail.clone()]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_animal_given_away(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut record: Py = Py::None;
    let mut records: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The animal the line hands over, as a word to put in a sentence (\"dog\"), or \"\".\n\n    The line usually names the kind (\"our dog\", \"the cats\"), and when it names only the\n    animal - \"we gave max away last week\" - the record the store has just taken off the list is\n    where the kind is. Without either there is nothing to say the warm line about.\n    ");
    found = mcall(&g_GIVEN_ANIMAL(), "search", &[text.clone()]);
    if py_truthy(&found.clone()) {
    kind = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    return if py_truthy(&{ let __b = mcall(&kind.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&kind.clone(), "endswith", &[py_str("ss")]))) } }) { slice_or_raise(&kind.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { kind.clone() };
    }
    gone = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("denial")]) } else { Py::None };
    records = if py_truthy(&call_builtin("isinstance", &[gone.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&gone.clone(), "get", &[py_str("pets")]) } else { Py::None };
    said = mcall(&text.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&{ let __b = records.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    name = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    kind = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return if py_truthy(&{ let __b = mcall(&kind.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&kind.clone(), "endswith", &[py_str("ss")]))) } }) { slice_or_raise(&kind.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { kind.clone() };
    }
    }
    return py_str("");
    Py::None
}
pub fn pet_given_away(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut kind: Py = Py::None;
    let _ = py_str("The warm line for an animal given to a new home, or None when this is another line.\n\n    bot/modules/memory_store.py asks this on the turn it takes the animal off the list, and\n    `reply` asks it for a line the store had no record of: either way the person gets the same\n    kind of answer, and never \"Right — no dog, then.\" for a dog they loved.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GIVEN_AWAY(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_THEIRS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("\\s*(?:i|we)\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    kind = _the_animal_given_away(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), kind.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GIVEN_AWAY_REPLIES()); __args.push(if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _offers_a_chat(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("A short warm yes to a line that came to talk, never the line said back.\n\n    A chat offered in the days after bad news is not a fresh start: \"my dad died last month\" then\n    \"just wanted to chat really\" is the same grief going on, and the line that said it is still on\n    the screen, so that turn keeps the reply it had.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_OFFERS_A_CHAT(), "match", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None) } else { py_list(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", &[said.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.loss", "reads_as_loss", &[said.clone()]) } } } }) {
    return Py::None;
    }
    }
    return pyrt::xrefs::xcall("compose", "pick", &[g_CHAT_OFFER_REPLY(), memory.clone()]);
    Py::None
}
pub fn _the_plan_a_date_belongs_to(args: &[Py]) -> Py {
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
    let mut diary: Py = Py::None;
    let mut head: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word for the plan a bare date is the date of, and its kind, or (\"\", \"\") for none.\n\n    Only the kinds of plan a date belongs to: an occasion, a birthday said as its number, an\n    appointment in the diary, or a trip somewhere. The kind is what the phrase is built with - \"a\n    july wedding\" is an occasion, \"the dentist on tuesday\" is an appointment.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    occasion = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_OCCASIONS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&occasion.clone()) {
    return py_tuple(vec![occasion.clone(), py_str("occasion")]);
    }
    if py_truthy(&{ let __b = mcall(&g_A_MILESTONE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:my|our)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![py_str("birthday"), py_str("occasion")]);
    }
    diary = mcall(&g_DIARY_THING(), "search", &[said.clone()]);
    if py_truthy(&diary.clone()) {
    return py_tuple(vec![mcall(&mcall(&diary.clone(), "group", &[Py::Int(0)]), "lower", &[]), py_str("diary")]);
    }
    if py_truthy(&{ let __b = mcall(&g_TRIP_VERB(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_DESTINATION(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_OBLIGED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PAST_PLAN()))) } } } } } }) {
    head = mcall(&mcall(&mcall(&g_DESTINATION(), "search", &[said.clone()]), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_A_TRIP(), &head.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_THING_BEFORE(), &head.clone())) } }) {
    return py_tuple(vec![py_str("holiday"), py_str("trip")]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _a_date_on_the_thing_in_hand(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut a_day: Py = Py::None;
    let mut article: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut made: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_so_far: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut when: Py = Py::None;
    let mut worried: Py = Py::None;
    let _ = py_str("The plan's own date, said back as what the time means for it, or None.\n\n    Only when the chat is already on a thing a date can belong to - an occasion or something in\n    the diary - and the line is that date and nothing else. Anything else keeps its own reply.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g_A_DATE_ALONE(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    record = crate::memory::topics::current(&[memory.clone()]);
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = record.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    when = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("when")]), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&when.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e)), &when.clone())) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_plan_a_date_belongs_to(&[text.clone()]), 2, None);
    plan = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&plan.clone()))) {
    return Py::None;
    }
    thing = op_add(&mcall(&slice_or_raise(&when.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&when.clone(), &Py::Int(1), &Py::None, &Py::None));
    said_so_far = mcall(&py_str(" "), "join", &[op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() })]);
    worried = call_builtin("bool", &[mcall(&g_A_WORRIED_ABOUT_IT(), "search", &[said_so_far.clone()])]).unwrap_or_else(|e| exc::raise(e));
    a_day = mcall(&g_A_MONTH_ALONE(), "match", &[when.clone()]);
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("diary")))) {
    if py_truthy(&mcall(&g_A_WEEKDAY(), "match", &[when.clone()])) {
    made = op_add(&op_add(&op_add(&py_str("the "), &plan.clone()), &py_str(" on ")), &thing.clone());
    } else {
    if py_truthy(&a_day.clone()) {
    made = op_add(&op_add(&op_add(&py_str("the "), &plan.clone()), &py_str(" in ")), &thing.clone());
    } else {
    made = op_add(&op_add(&op_add(&py_str("the "), &plan.clone()), &py_str(" ")), &thing.clone());
    }
    }
    } else {
    if py_truthy(&a_day.clone()) {
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &mcall(&geti(&thing.clone(), &Py::Int(0)), "lower", &[])))) { py_str("an ") } else { py_str("a ") };
    made = op_add(&op_add(&op_add(&article.clone(), &thing.clone()), &py_str(" ")), &plan.clone());
    } else {
    if py_truthy(&{ let __b = mcall(&g_A_WEEKDAY(), "match", &[when.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("tomorrow"), py_str("tonight")]), &when.clone())) } }) {
    made = op_add(&op_add(&op_add(&py_str("the "), &plan.clone()), &py_str(" ")), &mcall(&thing.clone(), "lower", &[]));
    } else {
    made = op_add(&op_add(&op_add(&py_str("the "), &plan.clone()), &py_str(" ")), &mcall(&thing.clone(), "lower", &[]));
    }
    }
    }
    made = op_add(&mcall(&slice_or_raise(&made.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&made.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&a_day.clone()) {
    pool = if py_truthy(&worried.clone()) { g_A_DATE_ON_A_HARD_PLAN() } else { g_A_DATE_ON_A_PLAN() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Thing".to_string(), made.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone()]), "format", args))(&__args) };
    }
    pool = if py_truthy(&worried.clone()) { g_A_DAY_ON_A_HARD_PLAN() } else { g_A_DAY_ON_A_PLAN() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Thing".to_string(), made.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _after_the_hello(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("What the line says after the hello, or \"\" when the hello is all of it.");
    match_ = mcall(&g_GREETING(), "match", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    return mcall(&slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "lstrip", &[py_str(" ,.!-:;")]);
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut aged: Py = Py::None;
    let mut alone: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut animal_wanted: Py = Py::None;
    let mut another: Py = Py::None;
    let mut chat: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut clock: Py = Py::None;
    let mut coming: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut deadline: Py = Py::None;
    let mut found: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut given: Py = Py::None;
    let mut habit: Py = Py::None;
    let mut had: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut ill: Py = Py::None;
    let mut job: Py = Py::None;
    let mut kids: Py = Py::None;
    let mut lasting: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut many: Py = Py::None;
    let mut more: Py = Py::None;
    let mut nobody: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut planned: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut read: Py = Py::None;
    let mut remark: Py = Py::None;
    let mut reported: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("A short echo of the fact they just gave, or None when this is not that kind of line.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    dated = _a_date_on_the_thing_in_hand(&[clean.clone(), memory.clone()]);
    if py_truthy(&dated.clone()) {
    return dated.clone();
    }
    let mut compose_fallback: Py = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&mcall(&compose_fallback.clone(), "_the_morning_has_a_clock", &[mcall(&clean.clone(), "lower", &[])])) {
    return Py::None;
    }
    let mut A_DISMISSAL: Py = pyrt::xrefs::xget("compose.fallback_data", "A_DISMISSAL");
    if py_truthy(&mcall(&A_DISMISSAL.clone(), "match", &[clean.clone()])) {
    return Py::None;
    }
    reported = crate::memory::facts::_reported_clause(&[clean.clone()]);
    if py_truthy(&{ let __b = reported.clone(); if !py_truthy(&__b) { __b } else { crate::memory::facts::extract(&[reported.clone()]) } }) {
    clean = mcall(&py_str(" "), "join", &[mcall(&reported.clone(), "split", &[])]);
    text = clean.clone();
    }
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("suggestion")]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("errand")]) } }) {
    return Py::None;
    }
    read = _repaired(&[clean.clone(), memory.clone()]);
    if py_truthy(&mcall(&g_GREETING(), "search", &[clean.clone()])) {
    rest = _after_the_hello(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::facts::about_themselves(&[rest.clone()]))) } }) {
    return Py::None;
    }
    clean = rest.clone();
    text = rest.clone();
    read = _repaired(&[clean.clone(), memory.clone()]);
    } else {
    if py_truthy(&{ let __b = mcall(&g_MEET_GREET(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SELF_INTRO(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    }
    had = the_job_they_had(&[clean.clone(), memory.clone()]);
    if py_truthy(&had.clone()) {
    return had.clone();
    }
    alone = a_person_named_alone(&[clean.clone(), memory.clone()]);
    if py_truthy(&alone.clone()) {
    return alone.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_WANTS_ANSWER(), "search", &[read.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::memory::facts::g_RELATION_WHOS_AGE(), "search", &[read.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&_a_result_is_on_the_table(&[clean.clone()])) {
    return Py::None;
    }
    limit = _a_limit_on_the_plan(&[read.clone(), memory.clone()]);
    if py_truthy(&limit.clone()) {
    return limit.clone();
    }
    second = a_second_person(&[clean.clone(), memory.clone()]);
    if py_truthy(&second.clone()) {
    return second.clone();
    }
    corrected = value_put_right(&[clean.clone(), memory.clone()]);
    if py_truthy(&corrected.clone()) {
    return corrected.clone();
    }
    aged = the_old_one_aged(&[clean.clone(), memory.clone()]);
    if py_truthy(&aged.clone()) {
    return aged.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&_statement(&[memory.clone()])))) {
    return Py::None;
    }
    given = pet_given_away(&[clean.clone(), memory.clone()]);
    if py_truthy(&given.clone()) {
    return given.clone();
    }
    if py_truthy(&Py::Bool(!py_is(&pyrt::xrefs::xcall("compose.trouble", "reads_as_trouble", &[read.clone()]), &Py::None))) {
    return Py::None;
    }
    limit = a_limit_reply(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&limit.clone(), &Py::None))) {
    return limit.clone();
    }
    if py_truthy(&_no_subject(&[clean.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", &[mcall(&read.clone(), "lower", &[])]); if py_truthy(&__b) { __b } else { _a_loss(&[text.clone(), memory.clone(), read.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&_bad_news_just_before(&[read.clone(), memory.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_THE_STORY_GOES_ON(), memory.clone(), read.clone()]);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.badnews", "reads_like_a_symptom", &[mcall(&read.clone(), "lower", &[])])) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.goodnews", "reads_as_good_news", &[mcall(&read.clone(), "lower", &[])])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.goodnews", "reads_as_settled", &[mcall(&clean.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.goodnews", "worry_on_the_table", &[memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_DEATH_WORDS(), "search", &[read.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_WOULD_HAVE_BEEN(), "search", &[read.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BODY(), "search", &[read.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BLEED(), "search", &[read.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_FEELING(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&_said_to_the_bot(&[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_bare_state(&[mcall(&clean.clone(), "lower", &[])])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_GOOD_MOOD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&has_news(&[clean.clone()]))) } }) {
    return Py::None;
    }
    alone = nobody_around(&[mcall(&clean.clone(), "lower", &[]), memory.clone()]);
    if py_truthy(&alone.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[alone.clone(), memory.clone(), clean.clone()]);
    }
    deadline = _a_deadline(&[clean.clone(), read.clone(), memory.clone()]);
    if py_truthy(&deadline.clone()) {
    return deadline.clone();
    }
    clock = _a_clock_time(&[clean.clone(), read.clone(), memory.clone()]);
    if py_truthy(&clock.clone()) {
    return clock.clone();
    }
    if py_truthy(&_after_a_feeling(&[clean.clone(), memory.clone()])) {
    thing = _echo(&[clean.clone()]);
    if py_truthy(&thing.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_EXPLAIN_TAIL()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    }
    lasting = _the_length(&[clean.clone(), memory.clone()]);
    if py_truthy(&lasting.clone()) {
    return lasting.clone();
    }
    hours = _the_hours(&[clean.clone(), read.clone(), memory.clone()]);
    if py_truthy(&hours.clone()) {
    return hours.clone();
    }
    chat = _offers_a_chat(&[clean.clone(), memory.clone()]);
    if py_truthy(&chat.clone()) {
    return chat.clone();
    }
    job = _a_job_to_do(&[clean.clone(), memory.clone()]);
    if py_truthy(&job.clone()) {
    return job.clone();
    }
    found = _learned(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { _only_a_wish(&[found.clone()]) } }) {
    wanted = op_add(&py_str("a "), &_the_wished_animal(&[found.clone(), clean.clone()]));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Thing".to_string(), op_add(&mcall(&slice_or_raise(&wanted.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&wanted.clone(), &Py::Int(1), &Py::None, &Py::None))), ("thing".to_string(), wanted.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WISH_FOR_AN_ANIMAL()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&found.clone()) {
    nobody = _who_the_figure_is(&[found.clone(), memory.clone(), mcall(&clean.clone(), "lower", &[])]);
    if py_truthy(&nobody.clone()) {
    return nobody.clone();
    }
    }
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("age"), py_str("height")])); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_only_the_figure(&[clean.clone()]))) } } } }) {
    return Py::None;
    }
    animal_wanted = _an_animal_wanted(&[clean.clone()]);
    if py_truthy(&{ let __b = animal_wanted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_on_record(&[animal_wanted.clone(), memory.clone()]))) } }) {
    return _what_a_wanted_animal_needs(&[animal_wanted.clone(), clean.clone(), memory.clone()]);
    }
    coming = if py_truthy(&found.clone()) { _the_animal_coming(&[found.clone()]) } else { py_str("") };
    if py_truthy(&coming.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), coming.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_ANIMAL_COMING(), memory.clone(), coming.clone()]), "format", args))(&__args) };
    }
    planned = _an_animal_only_planned(&[found.clone(), clean.clone(), memory.clone()]);
    if py_truthy(&planned.clone()) {
    said = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &geti(&mcall(&planned.clone(), "split", &[]), &Py::Int(0))))) { planned.clone() } else { if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&planned.clone(), &Py::None, &Py::Int(1), &Py::None)))) { op_add(&py_str("an "), &planned.clone()) } else { op_add(&py_str("a "), &planned.clone()) } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), said.clone()), ("Animal".to_string(), op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_A_BREED_BEING_CHOSEN(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = _an_animal_taken_back(&[found.clone()]); if py_truthy(&__b) { __b } else { _an_animal_turned_down(&[found.clone(), memory.clone()]) } } } }) {
    animal = _the_taken_back_animal(&[found.clone()]);
    if py_truthy(&{ let __b = animal.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_on_record(&[animal.clone(), memory.clone()]))) } }) {
    if py_truthy(&_an_animal_they_had_once(&[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), animal.clone()), ("Animal".to_string(), op_add(&mcall(&slice_or_raise(&animal.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&animal.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_ANIMAL_HAD_ONCE(), memory.clone(), animal.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), animal.clone()), ("Animal".to_string(), op_add(&mcall(&slice_or_raise(&animal.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&animal.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_ANIMAL_TAKEN_BACK(), memory.clone(), animal.clone()]), "format", args))(&__args) };
    }
    return Py::None;
    }
    remark = _a_remark_on_what_was_said(&[clean.clone(), memory.clone()]);
    if py_truthy(&remark.clone()) {
    let _ = _a_hand_back(&[remark.clone(), memory.clone()]);
    return remark.clone();
    }
    another = _another_of_what_we_said(&[clean.clone(), memory.clone()]);
    if py_truthy(&another.clone()) {
    return another.clone();
    }
    more = _adds_one_more_allergen(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { mcall(&mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood"), py_dict(vec![])]), "get", &[py_str("routine")]) } }) {
    habit = _plain(&[clean.clone(), memory.clone()]);
    if py_truthy(&habit.clone()) {
    return habit.clone();
    }
    }
    said = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_tells_it(&[clean.clone(), found.clone()]))) } } } }) {
    said = _a_thing_they_have(&[clean.clone(), memory.clone()]);
    }
    if py_truthy(&said.clone()) {
    return { let __b = _the_opening_instead(&[said.clone(), clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { said.clone() } };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_tells_it(&[clean.clone(), found.clone()]))) } } } }) {
    plain = _plain(&[clean.clone(), memory.clone()]);
    return { let __b = _the_opening_instead(&[plain.clone(), clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { plain.clone() } };
    }
    pets = if py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&found.clone(), "get", &[py_str("pets")]) } else { Py::None };
    ill = { let __b = Py::Bool(!py_truthy(&mcall(&g_A_CHILD_IN_THE_LINE(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_for_a_pet", &[mcall(&clean.clone(), "lower", &[])]) } };
    if py_truthy(&{ let __b = pets.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("pets")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_AS_WELL(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_CHILD_IN_THE_LINE(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { ill.clone() } } } } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pets.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("pets")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_AS_WELL(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_CHILD_IN_THE_LINE(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_nothing_added_but_the_pet(&[clean.clone(), pets.clone()]))) } } } } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pets.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("pets")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_AS_WELL(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_CHILD_IN_THE_LINE(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_pet_that_is_all_right(&[clean.clone(), pets.clone()]))) } } } } } } } } } }) {
    let __unpk1 = py_unpack(&_the_pets_said(&[pets.clone(), clean.clone()]), 2, None);
    said = __unpk1[0].clone();
    many = __unpk1[1].clone();
    if py_truthy(&said.clone()) {
    if py_truthy(&_the_pets_are_coming(&[pets.clone(), memory.clone()])) {
    pool = if py_truthy(&many.clone()) { g_PET_TAIL_COMING_MANY() } else { g_PET_TAIL_COMING() };
    } else {
    pool = if py_truthy(&many.clone()) { g_PET_TAIL_MANY() } else { g_PET_TAIL_ONE() };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), said.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    }
    kids = _children_with_ages(&[text.clone()]);
    if py_truthy(&kids.clone()) {
    thing = _children_said(&[kids.clone()]);
    } else {
    if py_truthy(&{ let __b = more.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("list", &[{ let __b = mcall(&found.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &more.clone())) } }) {
    thing = op_add(&op_add(&py_str("you're allergic to "), &mcall(&py_str(" and "), "join", &[more.clone()])), &py_str(" too"));
    } else {
    thing = _echo(&[clean.clone(), memory.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    plan = _plan_tails(&[found.clone(), clean.clone(), memory.clone()]);
    if py_truthy(&plan.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plan.clone()); __args.push(memory.clone()); __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))), ("food".to_string(), mcall(&g_CONDITION_FOOD(), "get", &[mcall(&call_builtin("str", &[{ let __b = mcall(&found.clone(), "get", &[py_str("condition")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), py_str("that")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick_ending(args))(&__args) };
    }
    frames = _frames(&[found.clone(), _heavy(&[memory.clone()]), clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&frames.clone(), &g_FRAMES()))) {
    opened = crate::memory::opening::the_opening(&[thing.clone(), clean.clone(), memory.clone()]);
    if py_truthy(&opened.clone()) {
    return opened.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frames.clone()); __args.push(memory.clone()); __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))), ("food".to_string(), mcall(&g_CONDITION_FOOD(), "get", &[mcall(&call_builtin("str", &[{ let __b = mcall(&found.clone(), "get", &[py_str("condition")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), py_str("that")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick_ending(args))(&__args) };
    Py::None
}
pub fn _the_opening_instead(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut thing: Py = Py::None;
    let _ = py_str("The question about the fact, where a receipt of the line is all the reply would be.\n\n    On the first line of a chat a receipt is the filing cabinet: nothing comes back about the\n    two jobs or the six-year-old. bot/memory/opening.py asks the question a friend asks about\n    the fact instead, and stands aside for every line that is not that shape.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&reads_as_a_receipt(&[said.clone()]))) } }) {
    return Py::None;
    }
    thing = _echo(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    return crate::memory::opening::the_opening(&[thing.clone(), clean.clone(), memory.clone()]);
    Py::None
}
pub fn _a_hand_back(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Note a reply that answered a remark, so the turn after it can tell it from a new subject.\n\n    The echo it replaces (\"Got it - it takes forever.\") was skipped by\n    `bot/memory/topics.py::said_again` as an acknowledgement that says nothing of its own, which\n    is how a follow-up two turns later still reached the answer above it. A remark is the same\n    thing said properly - the subject stays where it was - so it keeps that behaviour without\n    having to look like an acknowledgement. Only the reply's own words are kept, so a turn that\n    gave something else is never mistaken for one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    kept = mcall(&memory.clone(), "get", &[py_str("hand_backs")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[kept.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    kept = py_list(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("hand_backs"), kept.clone());
    }
    let _ = mcall(&kept.clone(), "append", &[said.clone()]);
    let _ = py_delitem(&kept.clone(), &Py::None).unwrap_or(());
    Py::None
}
pub fn _a_remark_on_what_was_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut body: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut one: Py = Py::None;
    let mut options: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut topic_text: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to a one-line remark about the turn before, or None when the line is not one.\n\n    Only a short line that points back with a pronoun or a \"the\" and states one of the kinds\n    above, and only after someone has said something for it to be about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    body = mcall(&clean.clone(), "strip", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&body.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_REMARK_BACK(), "match", &[body.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&body.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_REMARK_TWO_SIDED(), "search", &[body.clone()])) {
    return Py::None;
    }
    topic_text = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("topic")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    let __iter1 = py_iter_or_raise(&g_REMARK_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[body.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("gone"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GROWING(), "search", &[op_add(&op_add(&body.clone(), &py_str(" ")), &topic_text.clone())]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("expense"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REMARK_JUDGES_THE_PRICE(), "search", &[body.clone()]))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_REMARK_FIGURE(), "search", &[body.clone()])))) {
    continue;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_REMARK_FIGURE_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    options = geti(&g_REMARK_REPLIES(), &kind.clone());
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("expense"))); if !py_truthy(&__b) { __b } else { _a_price_still_to_come(&[body.clone(), topic_text.clone()]) } }) {
    options = g_REMARK_PRICE_AHEAD();
    }
    if py_truthy(&_asked_last(&[memory.clone()])) {
    options = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&one.clone(), "endswith", &[py_str("?")])))) {  __out.push(one.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { options.clone() } };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _a_price_still_to_come(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut topic_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic_text"))) };
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether an expense remark is about a price nobody has paid yet.\n\n    \"it cost a fortune\" and \"the flights are expensive\" are the same kind of remark and want\n    different endings: the first is money already spent, which is why the question that follows\n    is whether it was worth it, and the second is a bill still to come, where that question asks\n    about a thing that has not happened. The plan on the table decides it - a line in the past\n    (\"redid\", \"cost me\", \"last year\") is behind them, and a plan still to happen (\"going to\",\n    \"in august\", \"next month\") is not - and with nothing to say either way it counts as still to\n    come, because asking someone whether an unbooked trip was worth it reads as the bot not\n    having listened.\n    ");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let __b = body.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, { let __b = topic_text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]);
    if py_truthy(&mcall(&g_REMARK_STILL_TO_COME(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g_REMARK_ALREADY_PAID(), "search", &[said.clone()])));
    Py::None
}
pub fn _a_result_is_on_the_table(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this line is a result the person is waiting on or has just heard.\n\n    Asked of bot/compose/fallback.py, which owns the two readings and the two replies, and\n    imported here rather than at the top of the file because that file imports this one.\n    ");
    let mut fallback: Py = pyrt::xrefs::xmodule("compose.fallback");
    return mcall(&fallback.clone(), "reads_as_a_result_line", &[clean.clone()]);
    Py::None
}
pub fn _ending_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut parts: Py = Py::None;
    let _ = py_str("The words a frame adds after the thing said back: the part that repeats between turns.");
    parts = mcall(&g__HOLE(), "split", &[frame.clone()]);
    return if py_truthy(&op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&py_str(" "), "join", &[mcall(&mcall(&g__NOT_WORDS(), "sub", &[py_str(" "), mcall(&geti(&parts.clone(), &Py::Int(1)), "lower", &[])]), "split", &[])]) } else { py_str("") };
    Py::None
}
pub fn _pick_ending(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frames: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frames") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frames"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fields: Py = py_dict(__kw.iter().filter(|(n, _)| !["frames", "memory"].contains(&n.as_str())).map(|(n, v)| (py_str(n), v.clone())).collect());
    let mut fields: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One of the endings, never the one the reply before it ended with.\n\n    The store says a different thing back each turn - \"Your son is 9\", then \"Your daughter is\n    4\" - so bot.compose.pick cannot see the repeat: the whole line differs and the ending is the\n    same twice running, which reads as a script even though the sentence does not. Facts follow\n    facts, so this is where it happens most: \"my son is 9 / my daughter is 4\" came back as two\n    \"I'm sorry it's all coming at once\" in a row.\n    ");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = py_str("");
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    last = mcall(&py_str(" "), "join", &[mcall(&mcall(&g__NOT_WORDS(), "sub", &[py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), "split", &[])]);
    }
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut frame: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _ending_of(&[frame.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&last.clone(), &_ending_of(&[frame.clone()]))) } }))) {  __out.push(frame.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); for (k, v) in kw_pairs(&fields.clone()) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[{ let __b = fresh.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[frames.clone()]).unwrap_or_else(|e| exc::raise(e)) } }, memory.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _after_a_feeling(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut back: Py = Py::None;
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut now: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether this line is the reason for a feeling they said on the turn before.\n\n    The reading of the line before is asked for again here rather than kept: memory[\"understood\"]\n    is this turn's reading, the modules answer in any order, and a marker written by one module\n    would be lost with its copy when another one's reply is the one the user got. A feeling is\n    the engine's own reading (intent \"feeling\", negative), so \"my boss said my reports are late\"\n    is news and not one.\n\n    The line has to be a short statement about the person or the people in it: a fact about the\n    world (\"my car is broken\") keeps its ordinary echo, and a question has its own answer.\n    ");
    if py_truthy(&mcall(&g_AN_AGE_ALONE(), "match", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    index = op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    last = geti(&turns.clone(), &index.clone());
    said = if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&last.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_NO_REASON(), "match", &[mcall(&said.clone(), "strip", &[])]); if !py_truthy(&__b) { __b } else { mcall(&g_A_CLAUSE_OF_ITS_OWN(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    let __aug1 = op_sub(&index.clone(), &Py::Int(1));
    index = __aug1;
    if py_truthy(&op_lt(&index.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    back = geti(&turns.clone(), &index.clone());
    said = if py_truthy(&call_builtin("isinstance", &[back.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&back.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    }
    read = gen_understanding::understanding::understand(&[said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&mcall(&read.clone(), "get", &[py_str("emotion")]), &py_str("confused")))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_AN_AIM(), "search", &[said.clone()])) {
    now = gen_understanding::understanding::understand(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&now.clone(), "get", &[py_str("intent")]), &py_str("feeling"))); if !py_truthy(&__b) { __b } else { op_lt(&{ let __b = mcall(&now.clone(), "get", &[py_str("polarity")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(0)) } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_FEELING_ABOUT_IT(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&read.clone(), "get", &[py_str("intent")]), &py_str("feeling"))); if py_truthy(&__b) { __b } else { op_gt(&{ let __b = mcall(&read.clone(), "get", &[py_str("polarity")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(0)) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_a_heavy_reply_just_before(&[memory.clone(), index.clone()])))) {
    return Py::Bool(false);
    }
    }
    words = mcall(&clean.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::Bool(false);
    }
    first = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",;:!")]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("ive"), py_str("i've"), py_str("we"), py_str("we're"), py_str("were")]), &first.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[op_bitand(&g_PERSON_WORDS(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(",;:!")])); } py_set(__out) })() })]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_heavy_reply_just_before(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut note: Py = Py::None;
    let _ = py_str("Whether the reply they got on turn `index` was about something heavy they are carrying.\n\n    bot/modules/chat.py notes the pool it answered from when that pool is one of the heavy ones\n    (its HEAVY_FEELINGS), and the note survives only if that module's reply is the one the person\n    got. The engine's own reading is asked first and this is the second reading, because the\n    engine files some heavy lines as news: \"i miss my mum\" is read as news, and the line after it\n    (\"she lives in spain\") was answered \"Got it - she lives in spain\", which drops the missing.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    note = mcall(&memory.clone(), "get", &[py_str("heavy_reply")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[note.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&mcall(&note.clone(), "get", &[py_str("turn")]), &index.clone()));
    Py::None
}
pub fn _bad_news_just_before(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the line carries on a hard story the last turn or two began.\n\n    The store files a fact and the echo hands it straight back, which is right for \"my sister is\n    called Meg\". It is wrong in the middle of a story: after \"im thinking of leaving my husband\",\n    \"she took the kids to her mums\" is the next line of it, not a detail to file, and \"Noted:\n    she took the kids to her mums\" answers a marriage breaking up with a receipt.\n\n    Both halves have to be true - bad news just before, and this line about someone else doing\n    something - because a neutral fact told in a hard week is still a fact: \"my sister is called\n    Meg\" after \"my wife left me\" is the one warm thing in the chat, and it must keep its echo.\n    \"Bad news\" is read two ways because a person says it two ways: a break-up or a diagnosis\n    (bot/compose/badnews.py) and a death (bot/compose/loss.py). \"my nan died last week\" then\n    \"he took the car\" is the same story going on, and the car line was answering it with\n    motorway advice.\n    The other half of this rule is already in _learned, for a detail rather than a fact; asking\n    it here covers the facts as well.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THIRD_PERSON(), "match", &[_strip_lead_in(&[clean.clone()])])))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", &[said.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.loss", "reads_as_loss", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn carries_the_story_on(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("Whether this line is the next line of a hard story the last turn or two began.\n\n    The public reading of `_bad_news_just_before`, for the arbiter: a line that opens on someone\n    else (\"she took the kids to her mums\", \"she packed her things last night\") is the story going\n    on, not a request and not a fact to file. A question is never that - \"what do i do about the\n    kids\" inside the same story is a real ask, and the modules that answer asks should have it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    return _bad_news_just_before(&[clean.clone(), memory.clone()]);
    Py::None
}
pub fn the_old_one_aged(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut age: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_age: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to a line giving the age of the thing they are replacing, or None.\n\n    \"i need a new washing machine\" then \"the old one is 12 years old\" was read as a child of\n    twelve and answered \"One kid, twelve - noted.\", and once that was put right the line fell\n    through to the composer's echo - \"Got it - the old one is 12 years old.\" The age is what the\n    purchase is measured against, so the reply says what the number means for it.\n\n    Two things have to hold. The line has to be the thing in hand and its age, and nothing else:\n    a thing named for itself (\"my boiler is 8 years old\") is a question about that thing and has\n    an entry of its own. And the turn before has to be about getting or replacing one, because\n    an age on its own is not a purchase: \"my car is 12 years old\" after a line about the children\n    stays with whatever reads the car.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    found = mcall(&g_THE_OLD_ONE(), "match", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::None;
    }
    said_age = mcall(&mcall(&found.clone(), "group", &[py_str("n")]), "lower", &[]);
    if py_truthy(&mcall(&said_age.clone(), "isdigit", &[])) {
    age = call_builtin("int", &[said_age.clone()]).unwrap_or_else(|e| exc::raise(e));
    } else {
    spoken = mcall(&crate::memory::facts::g_WORD_NUMBERS(), "get", &[said_age.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[spoken.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    age = spoken.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_le(&Py::Int(5), &__cmp11)) { op_le(&__cmp11, &Py::Int(40)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REPLACING(), "search", &[geti(&said.clone(), &op_neg(&Py::Int(1)))]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_FEELING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DEATH_WORDS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BLEED(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("n".to_string(), age.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OLD_ONE_TAIL()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _the_length(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut amount: Py = Py::None;
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut length: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The reply to a line that says only how long the thing has been going, or None.\n\n    Said a turn after the bot answered that thing, this is the person telling the answer how\n    long it has had: \"its been like that for a week\" is not a detail to file and hand back, and\n    handing it back is what the fridge conversation came out as. The length belongs with the\n    answer, so this only reads the line when the last turn was an answer - a length said cold is\n    its own subject and keeps its own reply. A body that hurts, a loss and a feeling are somebody\n    else's turn even said this short.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&crate::memory::narrow::only_a_length(&[clean.clone()])))) {
    return Py::None;
    }
    let mut tried: Py = crate::memory::tried::module_obj();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&tried.clone(), "after_an_answer", &[memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_like_a_symptom", &[mcall(&clean.clone(), "lower", &[])]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", &[mcall(&clean.clone(), "lower", &[])]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_FEELING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DEATH_WORDS(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    found = mcall(&crate::memory::narrow::g_A_LENGTH(), "search", &[clean.clone()]);
    lead = mcall(&mcall(&found.clone(), "group", &[py_str("lead")]), "lower", &[]);
    amount = mcall(&found.clone(), "group", &[py_str("amount")]);
    unit = mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_eq(&lead.clone(), &py_str("since")))) {
    when = if py_truthy(&Py::Bool(py_contains(&crate::memory::narrow::g_LENGTH_NAMED_DAYS(), &unit.clone()))) { op_add(&mcall(&geti(&unit.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&unit.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { unit.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("when".to_string(), when.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SINCE_TAIL()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    length = if py_truthy(&amount.clone()) { op_modulo(&py_str("%s %s"), &py_tuple(vec![amount.clone(), unit.clone()])) } else { unit.clone() };
    if py_truthy(&Py::Bool(py_contains(&g_LENGTH_KEEPS(), &lead.clone()))) {
    length = op_modulo(&py_str("%s %s"), &py_tuple(vec![lead.clone(), length.clone()]));
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("length".to_string(), op_add(&mcall(&geti(&length.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&length.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LENGTH_TAIL()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _the_hours(args: &[Py]) -> Py {
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
    let mut read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut family: Py = Py::None;
    let mut last_ask: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to a line about the hours they work, or None.\n\n    \"i work nights\" is a fact about their week, and the store filed it: \"Noted: you work\n    nights.\" is a clerk with a form, not a friend. The reply is about the life those hours make\n    - the body clock, the week that moves, the day with no edges - and it is chosen by which of\n    those the line says, never by the words of a topic, so it cannot be pasted onto another\n    line. Only a line that is only that: a body that will not sleep, a job they are sick of, a\n    wage, a rota they want out of and any ask are all somebody else's turn, and a loss thread\n    keeps its own quiet.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_WANTS_ANSWER(), "search", &[read.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_THE_HOURS(), "search", &[read.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_DEATH_WORDS(), "search", &[read.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_FEELING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BLEED(), "search", &[read.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.badnews", "reads_like_a_symptom", &[mcall(&read.clone(), "lower", &[])]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", &[mcall(&read.clone(), "lower", &[])]) } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    recent = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    if py_truthy(&{ let __b = recent.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_DEATH_WORDS(), "search", &[recent.clone()]) } }) {
    return Py::None;
    }
    last_ask = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("module")]), &py_str("answers"))); if !py_truthy(&__b) { __b } else { reads_as_an_ask(&[last_ask.clone()]) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_hours_family(&[read.clone()]), 2, None);
    family = __unpk1[0].clone();
    pool = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &Py::None))) {
    return Py::None;
    }
    let _ = crate::memory::hours::note(&[memory.clone(), family.clone()]);
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _a_deadline(args: &[Py]) -> Py {
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
    let mut read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pools: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The reply to a line that only says when the thing we just answered has to happen.\n\n    \"its tomorrow\" and \"its in three weeks\" are the person telling us the answer we just gave has\n    a clock on it. The answer is aimed at the clock rather than said again: a deadline a day off\n    is cut down to its first step, and one weeks off is spread out. Only after a real answer - a\n    line from the answers module long enough to be advice - because a deadline has nothing to\n    change when the turn before it was a hello or a hedge, and never in a loss thread, where the\n    clock is not the thing that matters.\n    ");
    found = mcall(&g_WHEN_DETAIL(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("answers"))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(18)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_tells_them_what_to_do(&[said.clone()])))) {
    return Py::None;
    }
    recent = op_add(&op_add(&said.clone(), &py_str(" ")), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    if py_truthy(&{ let __b = mcall(&g_DEATH_WORDS(), "search", &[recent.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BLEED(), "search", &[recent.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_FEELING(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    when = mcall(&mcall(&found.clone(), "group", &[py_str("when")]), "strip", &[]);
    pools = py_dict(vec![(py_str("today"), g_WHEN_TODAY_TAIL()), (py_str("tomorrow"), g_WHEN_TOMORROW_TAIL()), (py_str("days"), g_WHEN_DAYS_TAIL())]);
    pool = mcall(&pools.clone(), "get", &[_shape_of(&[when.clone()]), g_WHEN_ROOMY_TAIL()]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("when".to_string(), when.clone()), ("When".to_string(), op_add(&mcall(&slice_or_raise(&when.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&when.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _an_hour_on_the_day(args: &[Py]) -> Py {
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
    let _ = py_str("Which pool an hour on the day belongs to: \"nerves\" for an appointment, \"flat\" for a shift.\n\n    \"i have a night shift tomorrow\" / \"its at 10pm\" is an hour on the day too, and a night shift at\n    ten is spent asleep in the morning: the appointment pools, which all say what the morning is\n    for, say the opposite of that person's day. The flat pool says only that the day fits round\n    the hour, which is true of a shift, a class, a viewing and lunch with a sister.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_CLOCK_EVENT(), "search", &[text.clone()])) {
    return py_str("nerves");
    }
    if py_truthy(&mcall(&g_CLOCK_FLAT(), "search", &[text.clone()])) {
    return py_str("flat");
    }
    let mut clock: Py = crate::memory::when::module_obj();
    return if py_truthy(&mcall(&gattr(&clock.clone(), "EVENT_WORD"), "search", &[text.clone()])) { py_str("nerves") } else { Py::None };
    Py::None
}
pub fn _the_figure_is_an_hour(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut head: Py = Py::None;
    let _ = py_str("True when the figure is an hour at all: \"40\" is a quantity, and a clock stops at 24.");
    head = stdlib_call("re", "match", &[py_str("\\s*(\\d{1,2})(?!\\d)"), when.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::Bool(true);
    }
    return { let __cmp11: Py = call_builtin("int", &[mcall(&head.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(24)) } else { Py::Bool(false) } };
    Py::None
}
pub fn _a_clock_time(args: &[Py]) -> Py {
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
    let mut read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut last: Py = Py::None;
    let mut module: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The reply to a line that only gives the hour the thing we just answered is at, or None.\n\n    \"its at 10\" and \"the interview is at 10\" say when the answer above has to happen, and the\n    answer is aimed at the hour instead of being said back. Only after a real answer that tells\n    them what to do, and only when the conversation already names something that goes in a diary\n    (an interview, an appointment, a scan, a flight): the same words after advice about the roses\n    (\"its at 10\") are not an appointment, and the store has nothing to build a morning around.\n    ");
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {
    return Py::None;
    }
    found = { let __b = mcall(&g_CLOCK_DETAIL(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CLOCK_AT(), "search", &[clean.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let mut clock: Py = crate::memory::when::module_obj();
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    theirs = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&module.clone(), &py_str("greeting"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _an_hour_on_the_day(&[said.clone()]); if py_truthy(&__b) { __b } else { _an_hour_on_the_day(&[theirs.clone()]) } }))) {
    return Py::None;
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("answers"), py_str("chat"), py_str("memory_store")]), &module.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::None;
    }
    }
    kind = _an_hour_on_the_day(&[theirs.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    recent = op_add(&op_add(&said.clone(), &py_str(" ")), &theirs.clone());
    if py_truthy(&{ let __b = mcall(&g_DEATH_WORDS(), "search", &[theirs.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BLEED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_FEELING(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    when = mcall(&mcall(&found.clone(), "group", &[py_str("when")]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&_the_figure_is_an_hour(&[when.clone()])))) {
    return Py::None;
    }
    named = mcall(&g_THE_NOUN_IS_AT(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_MARKED_HOUR(), "search", &[when.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_hour_on_the_day(&[mcall(&named.clone(), "group", &[py_str("noun")])]))) } } } }) {
    return Py::None;
    }
    at = op_add(&py_str("At "), &when.clone());
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("flat")))) {
    pool = g_CLOCK_FLAT_TAIL();
    } else {
    pool = if py_truthy(&Py::Bool(py_eq(&_clock_shape(&[when.clone()]), &py_str("after")))) { g_CLOCK_AFTER_TAIL() } else { g_CLOCK_MORNING_TAIL() };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("At".to_string(), op_add(&mcall(&slice_or_raise(&at.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&at.clone(), &Py::Int(1), &Py::None, &Py::None))), ("When".to_string(), op_add(&mcall(&slice_or_raise(&when.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&when.clone(), &Py::Int(1), &Py::None, &Py::None))), ("when".to_string(), when.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _hours_family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let _ = py_str("Which kind of week the line names, and the pool that answers it.\n\n    Six shapes people put their hours in, and the reply has to be about the one they said: a\n    night shift has a body clock to it, an early start has a night before, working from home has\n    no edges at all.\n    ");
    if py_truthy(&mcall(&g_ON_NIGHTS(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("nights"), g_NIGHT_TAIL()]);
    }
    if py_truthy(&mcall(&gen_understanding::understanding::kinds::g_SHIFT_LATE(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("late"), g_LATE_TAIL()]);
    }
    if py_truthy(&mcall(&gen_understanding::understanding::kinds::g_SHIFT_EARLY(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("early"), g_EARLY_TAIL()]);
    }
    if py_truthy(&mcall(&g_EARLY_RISE(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("early"), g_EARLY_TAIL()]);
    }
    if py_truthy(&mcall(&g_LATE_FINISH(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("late"), g_LATE_TAIL()]);
    }
    if py_truthy(&mcall(&g_LONG_SHIFT(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("long"), g_LONG_TAIL()]);
    }
    if py_truthy(&mcall(&g_SHIFTS_ALL_OVER(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("spread"), g_SPREAD_TAIL()]);
    }
    if py_truthy(&mcall(&g_WORK_FROM_HOME(), "search", &[read.clone()])) {
    return py_tuple(vec![py_str("home"), g_HOME_TAIL()]);
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _a_pet_that_is_all_right(args: &[Py]) -> Py {
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
    let mut kept: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut found: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line says the animal is fine, rather than how it is ill.\n\n    Read after the animal word, so \"my dog is fine\" counts and \"my dog is being sick\" does not.\n    ");
    text = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = kept.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    continue;
    }
    found = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_PET_ALL_RIGHT(), "search", &[slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _nothing_added_but_the_pet(args: &[Py]) -> Py {
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
    let mut kept: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut found: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the words after the animal add nothing but its name, count, age or breed.\n\n    The animal word is the one the person used (\"pug\", not the store's \"dog\"), so a breed is\n    found in the line as they said it. A line that never repeats the animal (\"we got one\") keeps\n    the tail: the store reached an animal, and only the words beside it could say otherwise.\n    ");
    text = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = kept.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    continue;
    }
    found = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g_PET_ALL_RIGHT(), "search", &[slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_INTRODUCED_BEFORE(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)]); if !py_truthy(&__b) { __b } else { mcall(&g_A_NAME_AFTER(), "match", &[slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PET_ADDED(), "match", &[slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_wanted_animal(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the line asks for an animal rather than turning one down or remembering one.");
    low = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_A_WANT_TURNED_DOWN(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_WANTED_ANIMAL(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_animal_wanted(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The animal a line wants, or \"\".\n\n    Read from the line and not from the store, because the want is often the only thing the store\n    does not keep: \"my son would like a rabbit\" filed the son and dropped the rabbit.\n    ");
    low = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&_a_wanted_animal(&[low.clone()])))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&mcall(&crate::memory::facts::g_PETS(), "finditer", &[low.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    return mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    }
    if py_truthy(&mcall(&g_PET_WORD(), "search", &[low.clone()])) {
    return py_str("pet");
    }
    return py_str("");
    Py::None
}
pub fn _an_animal_they_had_once(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the line is an animal the person used to have, not one they are asking for.");
    low = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_a_wanted_animal(&[low.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_HAD_AN_ANIMAL(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _what_a_wanted_animal_needs(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("What having one of these takes, from the knowledge base, or the want's next question.\n\n    The line already names the animal, so the entry it reaches is that animal's own. A want that\n    no entry answers - \"the kids want a pet\", a fish nobody has named - gets the shortest honest\n    thing a friend says, which is the next question, never the animal handed back.\n    ");
    reading = py_dict(vec![]);
    let __try1 = exc::catch(|| {
    let mut lookup: Py = pyrt::xrefs::xmodule("knowledge.lookup");
    reading = gen_understanding::understanding::understand(&[op_add(&py_str("what do i need for a "), &animal.clone())]);
    found = mcall(&lookup.clone(), "find", &[reading.clone()]);
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
    found = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    reply = call_builtin("getattr", &[found.clone(), py_str("reply"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    let __try6 = exc::catch(|| {
    reply = if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&reply, &[memory.clone(), reading.clone()]) } else { reply.clone() };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    reply = py_str("");
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&reply.clone(), "strip", &[]) } }) {
    return mcall(&reply.clone(), "strip", &[]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&animal.clone(), &py_str("pet")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_PET_WANTED()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), animal.clone()), ("Animal".to_string(), op_add(&mcall(&slice_or_raise(&animal.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&animal.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_WANTED_ANIMAL_TAIL()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _the_pets_are_coming(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut coming: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("Whether the animals this line names are ones the store already has on their way.\n\n    Read from the store and not the line: the line that says the animal is coming has its own\n    reply (`ANIMAL_COMING`), and this is for the lines after it, which name the animal again\n    without saying it is on its way. A puppy is a dog, so the kinds are compared rather than the\n    words.\n    ");
    coming = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("coming")]) } }) {
    kind = mcall(&crate::memory::facts::g_ANIMAL_KINDS(), "get", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), py_str("")]);
    if py_truthy(&kind.clone()) {
    let _ = mcall(&coming.clone(), "add", &[kind.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&coming.clone()))) {
    return Py::Bool(false);
    }
    let __iter2 = py_iter_or_raise(&{ let __b = pets.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("breed")]))) } } } }) {
    continue;
    }
    kind = mcall(&crate::memory::facts::g_ANIMAL_KINDS(), "get", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), py_str("")]);
    if py_truthy(&Py::Bool(py_contains(&coming.clone(), &kind.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_animal_coming(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut animal: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The animal word of an animal the line says is on its way, or \"\".");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("coming")]) } }) {
    animal = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&animal.clone()) {
    return animal.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_taken_back_animal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut animal: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The animal word the take-back names, or \"\".");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("not_theirs")]) } }) {
    animal = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&animal.clone()) {
    return animal.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _an_animal_turned_down(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut animal: Py = Py::None;
    let mut field: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut tastes: Py = Py::None;
    let _ = py_str("True when the line is a \"no\" on an animal they do not have.\n\n    \"i dont want a dog\" files a dog that is not theirs and, beside it, a taste of \"no\" on the\n    dog, so the take-back reading saw a second fact in the line and declined. What answered the\n    refusal was the composer, holding the answer above: \"Nothing there was a dog, so the answer\n    holds. A bed, two bowls, a crate or pen, a collar and lead...\" - the whole dog kit handed\n    back to a person who has just said no to it. A taste of \"no\" on the same animal is the\n    refusal said in the store's own words, not another thing in the line, and an animal they do\n    have keeps the store's own line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pets.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&pet.clone(), "get", &[py_str("not_theirs")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_sub(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("pets"), py_str("tastes")]))) {
    return Py::Bool(false);
    }
    animal = _the_taken_back_animal(&[found.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&animal.clone())); if py_truthy(&__b) { __b } else { _on_record(&[animal.clone(), memory.clone()]) } }) {
    return Py::Bool(false);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("animal"), py_str("breed")])) { let mut field: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[field.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } } py_set(__out) })() };
    let _ = mcall(&named.clone(), "discard", &[py_str("")]);
    tastes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("tastes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut taste: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[taste.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(taste.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[tastes.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tastes.clone()) { let mut taste: Py = __item.clone(); __out.push(Py::Bool(py_contains(&named.clone(), &mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&taste.clone(), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _on_record(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("True when the store holds an animal of that kind as one of theirs.");
    user = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]) } else { Py::None };
    pets = if py_truthy(&call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = user.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let __b = pets.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &animal.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _an_animal_only_planned(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut record: Py = Py::None;
    let _ = py_str("The animal word a line renames inside a plan to get one, or \"\".\n\n    Both halves are asked of the store and the subject rather than of the wording: the line gives\n    one animal and no flag on it, the correction is a bare naming, and the chat is on a plan to\n    get something. A line that is not a correction, or a chat that is on anything else, is left\n    to the pet tail and to every other rule.\n    ");
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return py_str("");
    }
    pet = geti(&pets.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("wanted")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("coming")]); if py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("not_theirs")]) } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::memory::topics::g_NAMES_THE_THING(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])))) {
    return py_str("");
    }
    record = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("topic")]) } else { Py::None };
    plan = if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&plan.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&plan.clone(), "strip", &[]), "lower", &[]), &mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]))) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::memory::topics::g_A_THING_ON_ITS_WAY(), "search", &[plan.clone()])))) {
    return py_str("");
    }
    return mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    Py::None
}
pub fn _the_wished_animal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The animal the line wishes for, as they named it - the breed if they gave one.");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("wanted")]) } }) {
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&word.clone(), "isalpha", &[])) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&words.clone()) { geti(&words.clone(), &op_neg(&Py::Int(1))) } else { py_str("one") };
    Py::None
}
pub fn _only_a_wish(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let _ = py_str("True when everything the line gave the store is an animal the person would like.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_sub(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("pets")])) } }) {
    return Py::Bool(false);
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&pet.clone(), "get", &[py_str("wanted")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _an_animal_taken_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let _ = py_str("True when the only thing the line gave the store is an animal being taken back.\n\n    A wish is not a take-back. \"i want a labrador\" and \"i might get a dog\" name an animal the\n    person would like, and answering \"No dog, then.\" agreed to take away a dog nobody had, and\n    said no to a line that was asking about one. The store marks the two apart (`wanted`), so a\n    wished-for animal declines here and the composer answers what they would do with one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_sub(&call_builtin("set", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("pets")])) } }) {
    return Py::Bool(false);
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&pet.clone(), "get", &[py_str("wanted")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&pet.clone(), "get", &[py_str("not_theirs")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_pets_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kept: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut _u: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut breed_word: Py = Py::None;
    let mut count: Py = Py::None;
    let mut group: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut many: Py = Py::None;
    let mut n: Py = Py::None;
    let mut names: Py = Py::None;
    let mut number: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The pets the store kept, as a person would say them: \"two cats and a dog, Milly and Tom\".\n\n    The store has one entry per animal, with the count and the name beside it, so the numbers and\n    the names are both there; what was missing is the sentence a friend would make of them. Two\n    cats and a dog is not \"two dogs\", which is what counting the entries says.\n    ");
    number = py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten"))]);
    groups = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = kept.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __try2 = exc::catch(|| {
    count = call_builtin("int", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e));
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
    count = Py::Int(1);
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    breed_word = if py_truthy(&mcall(&pet.clone(), "get", &[py_str("breed_of_one")])) { py_str("") } else { call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) };
    animal = mcall(&mcall(&call_builtin("str", &[{ let __b = breed_word.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&animal.clone()))) {
    continue;
    }
    let __iter7 = py_iter_or_raise(&groups.clone());
    let mut __else8 = false;
    for __item in __iter7 {
    group = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&group.clone(), &Py::Int(0)), &animal.clone()))) {
    let __aug9 = op_add(&geti(&group.clone(), &Py::Int(1)), &count.clone());
    let _ = py_setitem(&group.clone(), &Py::Int(1), __aug9);
    break;
    }
    }
    let _ = mcall(&groups.clone(), "append", &[py_list(vec![animal.clone(), count.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&groups.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[groups.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&geti(&groups.clone(), &Py::Int(0)), &Py::Int(1)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { mcall(&g_A_VAGUE_MANY(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    parts = py_list(vec![]);
    let __iter10 = py_iter_or_raise(&groups.clone());
    for __item in __iter10 {
    let __unpk11 = py_unpack(&__item.clone(), 2, None);
    animal = __unpk11[0].clone();
    count = __unpk11[1].clone();
    if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) {
    let _ = mcall(&parts.clone(), "append", &[op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&animal.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an ") } else { py_str("a ") }, &animal.clone())]);
    } else {
    word = if py_truthy(&mcall(&animal.clone(), "endswith", &[py_str("s")])) { animal.clone() } else { op_add(&animal.clone(), &py_str("s")) };
    let _ = mcall(&parts.clone(), "append", &[{ let mut __s = String::new(); __s.push_str(&fstr(&mcall(&number.clone(), "get", &[count.clone(), call_builtin("str", &[count.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); py_string(__s) }]);
    }
    }
    said = if py_truthy(&op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&parts.clone(), &op_neg(&Py::Int(1)))) } else { geti(&parts.clone(), &Py::Int(0)) };
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() }) { let mut n: Py = __item.clone(); if py_truthy(&n.clone()) {  __out.push(op_add(&mcall(&slice_or_raise(&n.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&n.clone(), &Py::Int(1), &Py::None, &Py::None))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    let __aug12 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" called "); __s.push_str(&fstr(&geti(&names.clone(), &Py::Int(0)), -1, "")); py_string(__s) });
    said = __aug12;
    } else {
    if py_truthy(&names.clone()) {
    let __aug13 = op_add(&said.clone(), &op_add(&op_add(&py_str(", "), &mcall(&py_str(", "), "join", &[slice_or_raise(&names.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)])), &{ let mut __s = String::new(); __s.push_str(" and "); __s.push_str(&fstr(&geti(&names.clone(), &op_neg(&Py::Int(1))), -1, "")); py_string(__s) }));
    said = __aug13;
    }
    }
    many = op_gt(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&groups.clone()) { let __unpk4461 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk4461[0].clone(); let mut count: Py = __unpk4461[1].clone(); __out.push(count.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    return py_tuple(vec![said.clone(), many.clone()]);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut body: Py = Py::None;
    let mut options: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("An echo of a line with no fact in it, with a tail that fits what kind of line it is.\n\n    Only a plain statement about their own life gets here: fallback asks this after the symptom,\n    good-news, plan and trouble questions, for a line whose mood is neutral. A line that is a\n    fault to troubleshoot is left to bot/compose/trouble.py, which asks the better question.");
    body = _strip_lead_in(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&body.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&pyrt::xrefs::xcall("compose.trouble", "reads_as_trouble", &[clean.clone()]), &Py::None)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_about_them(&[body.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_HABIT(), "search", &[body.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GOING(), "search", &[body.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_OBLIGED(), "search", &[body.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_TROUBLE(), "search", &[body.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FEELING(), "search", &[body.clone()]) } }) {
    return Py::None;
    }
    thing = _echo(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HABIT(), "search", &[body.clone()])) {
    options = g_HABIT_TAIL();
    } else {
    if py_truthy(&{ let __b = mcall(&g_GOING(), "search", &[body.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GOING_BAD(), "search", &[body.clone()]))) } }) {
    options = g_GOING_TAIL();
    } else {
    return Py::None;
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _about_them(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut first: Py = Py::None;
    let _ = py_str("Whether the line is about their own life rather than the world or someone else's.\n\n    The shape it matched is what makes it specific (HABIT or GOING), so the subject is enough\n    here: \"i do legs on a tuesday\" and \"i'm trying to save money\" have no state word in them.");
    first = if py_truthy(&mcall(&body.clone(), "split", &[])) { mcall(&mcall(&geti(&mcall(&body.clone(), "split", &[]), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",;:")]) } else { py_str("") };
    return Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("i'm"), py_str("im"), py_str("i've"), py_str("ive"), py_str("i'd"), py_str("we"), py_str("we're"), py_str("were"), py_str("my"), py_str("our")]), &first.clone()));
    Py::None
}
pub fn _who_is_getting_better(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("\"them\" for somebody else in the line, \"me\" for the speaker, \"\" for nobody.");
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &op_bitor(&g_OTHER_PERSON(), &g_A_RELATION()))) {
    return py_str("them");
    }
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("me"), py_str("my"), py_str("myself")])]).unwrap_or_else(|e| exc::raise(e)))) {
    return py_str("me");
    }
    return py_str("");
    Py::None
}
pub fn a_person_getting_better(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut ours: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("The pool for news that somebody is getting well, or None.");
    plain = mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "replace", &[py_str("‘"), py_str("'")]);
    low = mcall(&plain.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&low.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { mcall(&g_DEATH_WORDS(), "search", &[low.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_who_is_getting_better(&[low.clone()])))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[op_add(&py_list(vec![low.clone()]), &_the_words_of_the_last_two_turns(&[memory.clone()]))]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:ill|sick|hospital|operation|op|surgery|broke|broken|stroke|chemo|treatment|injury|injured|fell|fall|accident|recovering|recovery|bug|flu|pneumonia|infection)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    mine = Py::Bool(py_eq(&_who_is_getting_better(&[low.clone()]), &py_str("me")));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_GOING_HOME(), g_GOING_HOME_TAIL(), g_GOING_HOME_MINE()]), py_tuple(vec![g_ON_THE_MEND(), g_ON_THE_MEND_TAIL(), g_ON_THE_MEND_MINE()]), py_tuple(vec![g_BACK_AT_IT(), g_BACK_AT_IT_TAIL(), g_BACK_AT_IT_MINE()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    pattern = __unpk2[0].clone();
    theirs = __unpk2[1].clone();
    ours = __unpk2[2].clone();
    match_ = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose", "negated", &[low.clone(), mcall(&match_.clone(), "group", &[Py::Int(0)])]))) } }) {
    return _with_the_time(&[if py_truthy(&mine.clone()) { ours.clone() } else { theirs.clone() }, low.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_words_of_the_last_two_turns(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The person's own lines from the last two turns, lowercased, oldest first.");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "append", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "append", &[mcall(&turn.clone(), "lower", &[])]);
    }
    }
    }
    return slice_or_raise(&words.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None);
    Py::None
}
pub fn _with_the_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The pool with the time the line gave in it (\"out next week\" -> \"out next week\").");
    match_ = stdlib_call("re", "search", &[py_str("\\b(next week|this week|tomorrow|soon|on (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|in a (?:few|couple of) days|by the weekend)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    when = if py_truthy(&match_.clone()) { mcall(&match_.clone(), "group", &[Py::Int(1)]) } else { py_str("soon") };
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "replace", &[py_str("{soon}"), when.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn nobody_around(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut hit: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The pool for a line that says, as a fact, that they have nobody, or None.");
    plain = mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "replace", &[py_str("‘"), py_str("'")]);
    hit = mcall(&g_NOBODY_AROUND(), "search", &[plain.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&hit.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOBODY_AROUND_ASK(), "search", &[plain.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ANYBODY_AT_ALL(), "search", &[plain.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&slice_or_raise(&plain.clone(), &Py::None, &mcall(&hit.clone(), "start", &[]), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_OTHER_PERSON())) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("compose", "negated", &[plain.clone(), slice_or_raise(&mcall(&hit.clone(), "group", &[Py::Int(0)]), &Py::None, &Py::Int(12), &Py::None)])) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[op_add(&py_list(vec![plain.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() })]);
    if py_truthy(&mcall(&g_DEATH_WORDS(), "search", &[said.clone()])) {
    return g_ALONE_AFTER_A_LOSS();
    }
    return if py_truthy(&mcall(&g_A_MOVE(), "search", &[said.clone()])) { g_ALONE_AFTER_A_MOVE() } else { g_ALONE_TAIL() };
    Py::None
}
pub fn _said_to_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line opens on the bot and has nothing of the person's life in it.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__THE_BOT_AS_SUBJECT(), "match", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__SOMEONE_ELSE_IN_IT(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])));
    Py::None
}
pub fn _no_subject(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line opens with a helper and nothing that can be a subject: \"was out all day\".\n\n    Saying the thing back needs somebody in it. People leave the subject off a short line\n    (\"am shattered\", \"was out all day\", \"have no idea\"), and handing it back as it stands - \"Got\n    it - were shattered after the move\" - reads worse than the stock line it replaced. The\n    understanding engine already knows the shape (it puts the \"i\" back for its own reading),\n    so this asks it instead of listing the helpers a second time.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("dropped_subject")]) } }) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__WORDS(), "findall", &[mcall(&_repaired(&[clean.clone(), memory.clone()]), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gen_understanding::understanding::normalize::g_LEADERS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { gen_understanding::understanding::normalize::opens_without_subject(&[words.clone(), Py::Int(0)]) } };
    Py::None
}
pub fn _asked_last(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's last reply ended on a question, so this one must not ask another.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    return mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "endswith", &[py_str("?")]);
    Py::None
}
pub fn _a_diary_plan(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut at_home: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut person: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The endings an appointment or a visit wants, or None when the plan is something else.\n\n    A plan that names a day and nothing else is either a thing in the diary - the dentist, a scan,\n    an interview, a haircut - or one of their people coming to stay. Both were filed and handed\n    back (\"Noted: the dentist is on tuesday.\", \"Noted: your mum's coming on friday.\"), which tells\n    the person nothing about the thing they have just put in their week. What is still open is the\n    time, or how long the visit lasts, so that is what the ending asks. A plan with no day in the\n    line (\"i want to save for a house\") is a project and keeps the reply it has.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("plan")]))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DATE_AHEAD(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_PAST_PLAN())) {
    return Py::None;
    }
    who = Py::None;
    named = stdlib_call("re", "search", &[py_str("\\b(?:my|our)\\s+([a-z]+)"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&named.clone()) {
    word = mcall(&named.clone(), "group", &[Py::Int(1)]);
    person = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![word.clone(), stdlib_call("re", "sub", &[py_str("'s$"), py_str(""), word.clone()]).unwrap_or_else(|e| exc::raise(e)), slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    at_home = stdlib_call("re", "search", &[py_str("\\b(?:coming|heading|getting|moving|flying) (?:back )?home\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &person.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_COMING_TO_STAY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&at_home.clone())) } } } }) {
    who = if py_truthy(&_more_than_one_coming(&[said.clone(), mcall(&named.clone(), "end", &[]), word.clone()])) { py_str("they") } else { if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::memory::facts::g_SHE(), &person.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::memory::facts::g_HE(), &person.clone())) } }) { py_str("she") } else { if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::memory::facts::g_HE(), &person.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::memory::facts::g_SHE(), &person.clone())) } }) { py_str("he") } else { py_str("they") } } };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("they")))) {
    return g_VISIT_TAIL_MANY();
    }
    if py_truthy(&who.clone()) {
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VISIT_TAIL_ONE()) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "replace", &[py_str("{who}"), who.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&mcall(&g_DIARY_THING(), "search", &[said.clone()])) {
    return if py_truthy(&mcall(&g_DIARY_TIME(), "search", &[said.clone()])) { g_DIARY_TIME_TAIL() } else { g_DIARY_TAIL() };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:meet|meeting|seeing)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return if py_truthy(&mcall(&g_DIARY_TIME(), "search", &[said.clone()])) { g_MEETING_TAIL() } else { g_MEETING_TIME_TAIL() };
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_OCCASIONS())) {
    return g_OCCASION_TAIL();
    }
    return Py::None;
    Py::None
}
pub fn _plan_tails(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kind: Py = Py::None;
    let mut one: Py = Py::None;
    let mut options: Py = Py::None;
    let _ = py_str("The endings a trip or an occasion wants, or None when the line is another kind of plan.\n\n    Everything else the store filed as a plan - \"i want to save for a house\", \"im going to\n    start running\" - is a project with a first step and keeps its own reply, which the composer\n    gives it.\n    ");
    kind = reads_as_a_trip(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("trip")))) {
    options = g_TRIP_TAIL();
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("occasion")))) {
    options = g_OCCASION_TAIL();
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("occasion_flat")))) {
    options = g_OCCASION_FLAT_TAIL();
    } else {
    options = _a_diary_plan(&[found.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&options.clone()))) {
    return Py::None;
    }
    }
    }
    }
    if py_truthy(&mcall(&g_HOW_LONG_ALREADY(), "search", &[clean.clone()])) {
    options = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mcall(&one.clone(), "lower", &[]), &py_str("how long")))) {  __out.push(one.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { options.clone() } };
    }
    if py_truthy(&_asked_last(&[memory.clone()])) {
    options = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&one.clone(), "endswith", &[py_str("?")])))) {  __out.push(one.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { options.clone() } };
    }
    return options.clone();
    Py::None
}
pub fn _about_a_body(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut read_it: Py = Py::None;
    let _ = py_str("Whether the line is the person's own body or health, however it words it.");
    let __try1 = exc::catch(|| {
    read_it = gen_understanding::understanding::understand(&[read.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }]);
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
    return { let __b = call_builtin("isinstance", &[read_it.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&read_it.clone(), "get", &[py_str("topic")]), &py_str("health"))) } };
    Py::None
}
pub fn the_job_they_had(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut job: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("A job the person said they used to do, answered as that, or None.\n\n    The reading is the store's own (`facts.JOB_WAS` and the job `facts.extract` keeps), so a line\n    this answers is a line the store filed: a job word, said in the past, about the person and\n    not about somebody else, and not a question.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::memory::facts::g_JOB_WAS(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    job = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&crate::memory::facts::extract(&[clean.clone()]), "get", &[py_str("job")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&job.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_WAS_NOT_A_JOB(), &mcall(&geti(&mcall(&job.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    return Py::None;
    }
    pool = if py_truthy(&mcall(&g_JOB_BEFORE(), "search", &[clean.clone()])) { g_PAST_JOB() } else { g_PAST_JOB_PLAIN() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("x".to_string(), job.clone()), ("X".to_string(), op_add(&mcall(&slice_or_raise(&job.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&job.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn a_job_to_do(args: &[Py]) -> Py {
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
    let _ = py_str("A thing with something still to be done to it, as the job it is, or None.\n\n    Every lane that has to know whether a line is a job on the person's list rather than an\n    appointment to get ready for reads it here, so one gate decides it for all of them.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), _a_job_to_do(&[text.clone(), memory.clone()])]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _a_job_to_do(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut match_: Py = Py::None;
    let mut read: Py = Py::None;
    let mut tails: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("A thing with something still to be done to it, answered as the job it is.\n\n    \"i have got a loft to clear out\" is not a possession - it is a job on their list - and the\n    receipt it used to get (\"Noted: you've got a loft to clear out\") is the filing cabinet\n    STYLE.md rules out. The same gate as `_a_thing_they_have` decides it is a thing at all; then a\n    verb of doing after \"to\" makes it a job, and the reply is the question a friend asks about a\n    job. It is asked here, before the fact paths, so a line about the person's evening in front of\n    it cannot change the reading.\n    ");
    read = { let __b = _spoken(&[_repairs(&[clean.clone(), memory.clone()])]); if py_truthy(&__b) { __b } else { clean.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_JOB_TO_DO(), "search", &[read.clone()])))) {
    return Py::None;
    }
    match_ = mcall(&g_HAVE_THING(), "search", &[read.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing = mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]), "strip", &[py_str("-'")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &thing.clone())); if py_truthy(&__b) { __b } else { mcall(&thing.clone(), "isdigit", &[]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _about_a_body(&[read.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLACED_IN_THE_PAST(), "search", &[read.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HABIT(), "search", &[read.clone()])) {
    return Py::None;
    }
    what = _echo(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&what.clone()))) {
    return Py::None;
    }
    what = mcall(&g_HAVE_A(), "sub", &[py_str("\\1've got a"), what.clone()]);
    tails = if py_truthy(&mcall(&g_JOB_HAS_A_DAY(), "search", &[read.clone()])) { slice_or_raise(&g_JOB_TAIL(), &Py::Int(1), &Py::None, &Py::None) } else { g_JOB_TAIL() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), what.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&what.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&what.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tails.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), read.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _the_chat_holds_something(args: &[Py]) -> Py {
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
    let _ = py_str("Whether a line was said before this one, so a pointer in this line can point at it.\n\n    The rule this serves is the operator's: a reply that leans on what was said earlier needs\n    something said earlier, and on a first message there is nothing, so the line itself has to be\n    answered. `memory` on a first turn is not empty - the understanding, the facts read from the\n    line and the topic are all in it before anybody composes a reply - so the turns said before\n    this one are the thing to ask about.\n    ");
    return crate::memory::there_is_a_chat(&[memory.clone()]);
    Py::None
}
pub fn _the_pronoun_points_at_somebody(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut held: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a pronoun in this line can point at a person or an animal the chat has named.\n\n    The store reads \"she\"/\"he\" against the people in the chat, and this asks the same reader, plus\n    the two things it does not cover: an animal, which is a \"he\", a \"she\" or an \"it\" as far as the\n    words go (\"my cat has stopped eating\" then \"she's 12\" is about the cat), and a \"they\", which\n    is the group the chat has been naming (\"something with the kids\" then \"theyre 6 and 9\").\n    ");
    if py_truthy(&crate::memory::relations::_person_from_pronoun(&[low.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    held = { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    theirs = if py_truthy(&call_builtin("isinstance", &[mcall(&held.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&held.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&theirs.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); __out.push(call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    fits = Py::None;
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:she|shes|she's|her)\\b"), { let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    fits = crate::memory::facts::g_SHE();
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|hes|he's|his)\\b"), { let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    fits = crate::memory::facts::g_HE();
    }
    }
    if py_truthy(&fits.clone()) {
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&g_PERSON_WORDS(), "union", &[crate::memory::store_lines::g_CHILD()])); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&crate::memory::relations::_pronoun_fits(&[word.clone(), fits.clone()])) {
    return Py::Bool(true);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:they|theyre|they're|them|their|theirs|those|these)\\b"), { let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), { let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter2 {
    turn = __item.clone();
    let __aug3 = op_bitor(&words.clone(), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    words = __aug3;
    }
    let __iter4 = py_iter_or_raise(&words.clone());
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::memory::store_lines::g_PERSON_GROUP(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::store_lines::g_CHILD(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push({ let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[mcall(&entry.clone(), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e)), "endswith", &[py_str("s")]))) } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _somebody_was_being_hurt(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the line before this one said somebody is doing the person harm.\n\n    \"the kids at school pick on her\" is a line about one person, and the store keeps nobody from\n    it because she is only \"her\" - so the next line's \"she\" points at that person and at nobody\n    else. This is the same room `_heavy` reads, for a line that names a figure instead of a fact.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() } } else { py_list(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return reads_as_cruelty(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _who_the_figure_is(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut low: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut pron: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The question that asks who an age or a height belongs to, or None.\n\n    Only for a reading that names nobody: \"an age, of a person\" with no person in it. The store\n    keeps \"my son is 9\" against the son and drops \"hes 9\" when the chat holds no \"he\", so the\n    note for the second one is about a fact nobody kept. `_person_from_pronoun` is the reader the\n    store itself uses, so a chat that has the person - \"my son is called tom\" then \"hes 9\" - is\n    left to the receipt it deserves.\n    ");
    entries = if py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&found.clone(), "get", &[py_str("others")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&entries.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_SOMEONE_MILESTONE(), "search", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&g_A_DATE_STILL_TO_COME(), "search", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&entries.clone());
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("who")]) } }) {
    continue;
    }
    figure = { let __b = mcall(&entry.clone(), "get", &[py_str("age")]); if py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("height")]) } };
    if py_truthy(&Py::Bool(!py_truthy(&figure.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&entry.clone(), "get", &[py_str("pronoun")]); if !py_truthy(&__b) { __b } else { _the_pronoun_points_at_somebody(&[low.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&entry.clone(), "get", &[py_str("pronoun")]); if !py_truthy(&__b) { __b } else { _somebody_was_being_hurt(&[memory.clone()]) } }) {
    pron = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    said = if py_truthy(&call_builtin("isinstance", &[figure.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { op_modulo(&py_str("%s's %s"), &py_tuple(vec![pron.clone(), figure.clone()])) } else { py_str("") };
    if py_truthy(&said.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), said.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_HEAVY_FRAMES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[figure.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { figure.clone() } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), call_builtin("str", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e))), ("Figure".to_string(), call_builtin("str", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHO_THE_FIGURE_IS()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHO_IS_IT()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _what_the_pointer_is(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut count: Py = Py::None;
    let mut counted: Py = Py::None;
    let _ = py_str("The question that asks what a bare \"them\" names, or None.\n\n    Only while the chat holds nothing: once there are turns, \"them\" points at something the\n    person said, and a chat that has a thing in it is not this reader's business.\n    ");
    if py_truthy(&_the_chat_holds_something(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_POINTS_BACK(), "search", &[{ let __b = read.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::None;
    }
    count = mcall(&g_A_COUNT_OF_THEM(), "search", &[{ let __b = read.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&count.clone()) {
    counted = mcall(&count.clone(), "group", &[Py::Int(1)]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Count".to_string(), op_add(&mcall(&slice_or_raise(&counted.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&counted.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHAT_OF_THEM()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), read.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WHICH_ONES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), read.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn _the_route_for(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("The frames that suit the kind of condition being wondered about.");
    low = mcall(&thing.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_CONDITION_FAMILIES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    family = __unpk2[0].clone();
    frames = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&family.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&family.clone(), "search", &[low.clone()]) } }) {
    return frames.clone();
    }
    }
    return geti(&geti(&g_CONDITION_FAMILIES(), &op_neg(&Py::Int(1))), &Py::Int(1));
    Py::None
}
pub fn _the_entry_names(args: &[Py]) -> Py {
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
    let mut head: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut keys: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the entry that answered the line is about this thing at all.\n\n    \"i think i have an eating disorder\" reached an entry about cities, so a card being in reach\n    is not the same as the card being about the worry. An entry that will not say is taken at its\n    word.\n    ");
    keys = call_builtin("getattr", &[entry.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&keys.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&keys.clone(), &head.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&keys.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(4), &Py::None), &slice_or_raise(&head.clone(), &Py::None, &Py::Int(4), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None), &slice_or_raise(&head.clone(), &Py::None, &Py::Int(5), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_condition_they_wonder_about(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut article: Py = Py::None;
    let mut ch: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut head: Py = Py::None;
    let mut initials: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer to \"i think i have <condition>\", or \"\" when this is not that line.\n\n    Only the lines nothing else can answer come here: a line the knowledge base has an entry\n    for is answered from it (\"i think i have gout\" is answered about gout), and a symptom word\n    is the bad-news or chat pool's turn (\"a chest infection\" keeps its own answer).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_WONDER_SAID_NO(), "search", &[clean.clone()]) } }) {
    return py_str("");
    }
    match_ = mcall(&g_A_WONDERED_CONDITION(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    words = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "lower", &[]), "split", &[])]), "strip", &[py_str("-' ")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    article = if py_truthy(&Py::Bool(py_contains(&g_WITH_AN_ARTICLE(), &geti(&words.clone(), &Py::Int(0))))) { geti(&words.clone(), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&article.clone()) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    head = geti(&words.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &head.clone())); if py_truthy(&__b) { __b } else { mcall(&head.clone(), "isdigit", &[]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&gen_understanding::understanding::topics::g_NUMBER_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&head.clone()) { let mut ch: Py = __item.clone(); __out.push(mcall(&ch.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&gen_understanding::understanding::kinds::g_VERBISH(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_CONDITION_KIND(), "search", &[head.clone()]))) } }) {
    return py_str("");
    }
    initials = Py::Bool(py_contains(&g_CONDITION_INITIALS(), &head.clone()));
    if py_truthy(&{ let __b = article.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_CONDITION_KIND(), "search", &[head.clone()]); if py_truthy(&__b) { __b } else { initials.clone() } })) } }) {
    return py_str("");
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.badnews", "reads_like_a_symptom", &[mcall(&clean.clone(), "lower", &[])])) {
    return py_str("");
    }
    entry = crate::memory::topics::an_entry_for(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&entry.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _the_entry_names(&[entry.clone(), head.clone()]) } }) {
    return py_str("");
    }
    phrase = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&initials.clone()) { mcall(&head.clone(), "upper", &[]) } else { word.clone() }); } py_list(__out) })() }]);
    if py_truthy(&article.clone()) {
    phrase = op_add(&op_add(&article.clone(), &py_str(" ")), &phrase.clone());
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), phrase.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[_the_route_for(&[phrase.clone()]), memory.clone(), phrase.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _a_thing_they_have(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut match_: Py = Py::None;
    let mut pointer: Py = Py::None;
    let mut read: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("A thing they have, said back: \"i have a bike\" -> \"Noted: you have a bike\".\n\n    Only when the line is the thing and nothing else: the thing word has to be a thing (not a\n    feeling, an idea, a question or a body that hurts), and what is behind it is the rest of the\n    line or nothing.\n    ");
    read = { let __b = _spoken(&[_repairs(&[clean.clone(), memory.clone()])]); if py_truthy(&__b) { __b } else { clean.clone() } };
    match_ = mcall(&g_HAVE_THING(), "search", &[read.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing = mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]), "strip", &[py_str("-'")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &thing.clone())); if py_truthy(&__b) { __b } else { mcall(&thing.clone(), "isdigit", &[]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _about_a_body(&[read.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLACED_IN_THE_PAST(), "search", &[read.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HABIT(), "search", &[read.clone()])) {
    return Py::None;
    }
    what = _echo(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&what.clone()))) {
    return Py::None;
    }
    pointer = _what_the_pointer_is(&[read.clone(), memory.clone()]);
    if py_truthy(&pointer.clone()) {
    return pointer.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), what.clone()), ("Thing".to_string(), op_add(&mcall(&slice_or_raise(&what.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&what.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_FRAMES(), memory.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _a_spending_budget(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut others: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether a stated budget is money to spend on a thing rather than a month to live inside.\n\n    Asked once, where the budget frame is chosen, so every way of stating a budget gets the same\n    reading: the period decides it when the line names one, and otherwise the conversation does.\n    ");
    if py_truthy(&{ let __b = mcall(&g_PERIOD(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&g_A_SPAN_OF_TIME(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::memory::facts::g_INTENDED_SPEND(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::memory::facts::g_THE_THING_MAY_COST(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { mcall(&crate::memory::facts::g_ALL_I_CAN_DO(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_FOR_A_THING(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(true);
    }
    recent = py_list(vec![{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None) } else { py_list(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&recent.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None) } else { py_list(vec![]) });
    for __item in __iter2 {
    topic = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&recent.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&topic.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    said = mcall(&mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&line.clone(), "strip", &[]), "lower", &[]), &said.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&mcall(&g_RAISED(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    let __iter3 = py_iter_or_raise(&others.clone());
    for __item in __iter3 {
    line = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_FOR_A_THING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BUYING(), "search", &[line.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_PERIOD(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SPAN_OF_TIME(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    }
    }
    if py_truthy(&mcall(&g_MONEY(), "search", &[mcall(&mcall(&py_str(" "), "join", &[others.clone()]), "lower", &[])])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_BUYING(), "search", &[mcall(&mcall(&py_str(" "), "join", &[recent.clone()]), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _frames(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut heavy: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "heavy") { v.clone() }
    else { Py::Bool(false) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut coming: Py = Py::None;
    let mut condition: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("Which set of endings this fact wants: a limit is answered with what the bot does with it.");
    if py_truthy(&mcall(&found.clone(), "get", &[py_str("budget")])) {
    return if py_truthy(&_a_spending_budget(&[clean.clone(), memory.clone()])) { g_SPENDING_FRAMES() } else { g_BUDGET_FRAMES() };
    }
    condition = mcall(&call_builtin("str", &[{ let __b = mcall(&found.clone(), "get", &[py_str("condition")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_CONDITION_FOOD(), &condition.clone()))) {
    return g_CONDITION_FRAMES();
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "get", &[py_str("allergies")]) } }) {
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&found.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("alcohol")))) {
    return g_DRINK_FRAMES();
    }
    return g_FOOD_FRAMES();
    }
    if py_truthy(&{ let __b = heavy.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_just_a_liking(&[found.clone()]))) } }) {
    return if py_truthy(&_about_the_family(&[found.clone()])) { g_HEAVY_FRAMES() } else { g_STILL_FRAMES() };
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("allergies")]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_OTHER_LIMIT_FRAMES();
    }
    if py_truthy(&_about_someone_else(&[found.clone(), clean.clone()])) {
    if py_truthy(&mcall(&g_SOMEONE_GOOD(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return g_SOMEONE_TAIL();
    }
    if py_truthy(&_the_milestone(&[clean.clone()])) {
    return g_SOMEONE_MILESTONE_TAIL();
    }
    if py_truthy(&_a_visit(&[clean.clone()])) {
    return g_VISIT_TAIL();
    }
    }
    if py_truthy(&_a_workplace(&[found.clone(), heavy.clone(), memory.clone()])) {
    step = op_modulo(&call_builtin("len", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_WORK_FRAMES()]).unwrap_or_else(|e| exc::raise(e)));
    return op_add(&slice_or_raise(&g_WORK_FRAMES(), &step.clone(), &Py::None, &Py::None), &slice_or_raise(&g_WORK_FRAMES(), &Py::None, &step.clone(), &Py::None));
    }
    coming = _a_day_coming(&[clean.clone()]);
    if py_truthy(&coming.clone()) {
    return coming.clone();
    }
    if py_truthy(&{ let __b = _only_a_liking(&[found.clone()]); if !py_truthy(&__b) { __b } else { { let __b = a_liked_thing(&[found.clone()]); if !py_truthy(&__b) { __b } else { _a_like_of_one_thing(&[clean.clone(), memory.clone()]) } } } }) {
    step = op_modulo(&call_builtin("len", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_LIKES_FRAMES()]).unwrap_or_else(|e| exc::raise(e)));
    return op_add(&slice_or_raise(&g_LIKES_FRAMES(), &step.clone(), &Py::None, &Py::None), &slice_or_raise(&g_LIKES_FRAMES(), &Py::None, &step.clone(), &Py::None));
    }
    return g_FRAMES();
    Py::None
}
pub fn _about_someone_else(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut key: Py = Py::None;
    let _ = py_str("Whether the line's fact is about a person of theirs rather than about the person typing.\n\n    A win of their own is answered by the composer's own good-news pools (`reads_as_good_news`\n    declines it here), so only somebody else's is read: their son, their mum, their neighbour.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("people"), py_str("others"), py_str("kids"), py_str("relations")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()])) {
    return Py::Bool(true);
    }
    }
    return call_builtin("bool", &[mcall(&g_THEIRS(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_visit(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line is a visit announced, rather than a lift or a question.\n\n    A question is not news (\"when is my sister coming\" asks the bot what it cannot know), and the\n    errand on the way past is not a visit (\"my mum is coming to pick me up\").\n    ");
    said = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_VISIT(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\s*(?:when|what|how|why|where|who|is|are|does|do|will|did)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_SOMEONE_COMING(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_milestone(args: &[Py]) -> Py {
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
    let _ = py_str("A round birthday or an anniversary of somebody else's, which is not the same as an age.\n\n    \"my son is 9\" is a fact to keep; \"my mum is 70 next week\" is a thing people make something\n    of, and \"my son got a prize at school\" is news rather than a milestone.\n    ");
    return call_builtin("bool", &[mcall(&g_SOMEONE_MILESTONE(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_workplace(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut heavy: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "heavy") { v.clone() }
    else { Py::Bool(false) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut frame: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Whether the line told the bot where the person works, and nothing else.\n\n    Only the place the work is in: the store keeps it beside the job (\"in a warehouse\", \"at\n    tesco\"), so those words are the tell. A line that files something else as well - an age, a\n    pet, a place they live - keeps the plain frames, because the thing said back is then two\n    facts and a workplace ending answers only one of them. A hard week two turns back keeps its\n    own endings as well: there the apology is the reply, whatever was filed.\n    ");
    if py_truthy(&{ let __b = heavy.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("job")]))) } } } }) {
    return Py::Bool(false);
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "items", &[])) { let __unpk2608 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk2608[0].clone(); let mut value: Py = __unpk2608[1].clone(); if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&key.clone(), &py_str("job"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&key.clone(), "endswith", &[py_str("_frame")]))) } } } }) {  __out.push(key.clone()); } } py_list(__out) })() };
    if py_truthy(&rest.clone()) {
    return Py::Bool(false);
    }
    frame = py_str("");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    known = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    frame = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("job_frame")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    }
    return { let __b = call_builtin("bool", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WORK_PLACE(), &geti(&mcall(&frame.clone(), "split", &[]), &Py::Int(0)))) } };
    Py::None
}
pub fn _about_the_family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut key: Py = Py::None;
    let _ = py_str("Whether the fact filed is about a person of theirs rather than a pet, a job or a taste.\n\n    \"my son is 9\" after a death is the family the death named, and the apology belongs on it.\n    \"i have two cats\" and \"i work in a shop\" are the person telling you about their life, and the\n    apology there says the cats are part of the pile.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FAMILY()) { let mut key: Py = __item.clone(); __out.push(mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _just_a_liking(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut key: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Whether the only thing this line filed is a taste, which is not something that lands.\n\n    \"i love pasta\" two turns after a death was answered \"You love pasta — I'm sorry it's all coming\n    at once\", which reads as a mistake: a liking is not one more thing on a person's plate, and it\n    is not what the room of a hard week changes. Anything else in the line (an age, a pet, a job)\n    can be that, and keeps the heavy ending.\n    ");
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "items", &[])) { let __unpk7936 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk7936[0].clone(); let mut value: Py = __unpk7936[1].clone(); if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&key.clone(), &py_str("tastes"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&key.clone(), "endswith", &[py_str("_frame")]))) } } } }) {  __out.push(key.clone()); } } py_set(__out) })() };
    return { let __b = call_builtin("bool", &[mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("tastes")])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rest.clone())) } };
    Py::None
}
pub fn _heavy(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the last few turns hold a death, a separation or harm somebody is doing.\n\n    Only the recent turns: a funeral six turns ago has been answered, and a fact told now is\n    not still being said through it. A line that is itself a loss or bad news is guarded where\n    it arrives, so this is only about the room the next fact is said in.\n    ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_HEAVY_TURNS()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.loss", "reads_as_loss", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SEPARATION(), "search", &[mcall(&said.clone(), "lower", &[])]); if py_truthy(&__b) { __b } else { reads_as_cruelty(&[said.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _statement(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Whether the understanding engine read the line as telling the bot something.");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    kind = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("kind")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kind.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { kind.clone() } }) {
    return Py::Bool(py_contains(&py_tuple(vec![py_str("statement"), py_str("tell"), py_str("news")]), &kind.clone()));
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_loss(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut topics: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Whether bot/compose/loss.py reads a death or a grief in the line: that is its turn.");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    topics = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("topics")]) } else { Py::None };
    return Py::Bool(!py_is(&pyrt::xrefs::xcall("compose.loss", "reply_for", &[text.clone(), memory.clone(), clean.clone(), topics.clone()]), &Py::None));
    Py::None
}
pub fn _learned(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The facts in the line for the store to keep, or an empty dict when there are none.\n\n    The chat-speak is folded first: the store's patterns want \"i am\", and people type \"im\", so\n    \"thanks, also im vegetarian\" is a diet like any other. A detail added to what we are already\n    on counts too: \"i work from home\" and \"i can put away about 200 a month\" store no fact of\n    their own, but they are theirs and worth answering with the thing itself rather than a stock\n    line. Those come back as {\"detail\": True}, so the caller has something to say back while the\n    store keeps its usual keys.");
    found = { let __b = crate::memory::facts::extract(&[_spoken(&[text.clone()])]); if py_truthy(&__b) { __b } else { crate::memory::facts::extract(&[text.clone()]) } };
    kept = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "items", &[])) { let __unpk5235 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk5235[0].clone(); let mut value: Py = __unpk5235[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FACTS(), &key.clone())); if !py_truthy(&__b) { __b } else { value.clone() } }) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    if py_truthy(&kept.clone()) {
    return kept.clone();
    }
    if py_truthy(&_adds_one_more_allergen(&[text.clone(), memory.clone()])) {
    return py_dict(vec![(py_str("allergies"), _adds_one_more_allergen(&[text.clone(), memory.clone()]))]);
    }
    if py_truthy(&_adds_a_detail(&[text.clone(), memory.clone()])) {
    return py_dict(vec![(py_str("detail"), Py::Bool(true))]);
    }
    return py_dict(vec![]);
    Py::None
}
pub fn _children_with_ages(args: &[Py]) -> Py {
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
    let mut reader: Py = Py::None;
    let _ = py_str("The children a line names with an age: \"my kids are ada 7 and sam 4\".");
    reader = call_builtin("getattr", &[crate::memory::facts::module_obj(), py_str("_kids_named_with_ages"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), if py_truthy(&reader.clone()) { py_call(&reader, &[text.clone()]) } else { py_list(vec![]) }]);
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
    return py_list(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _children_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kids: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kids") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kids"))) };
    let mut kid: Py = Py::None;
    let mut name: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The children said back by name and age: \"Ada is 7 and Sam is 4\".");
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&kids.clone(), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    kid = __item.clone();
    name = call_builtin("str", &[{ let __b = mcall(&kid.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    continue;
    }
    let _ = mcall(&parts.clone(), "append", &[op_add(&op_add(&op_add(&mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" is ")), &call_builtin("str", &[mcall(&kid.clone(), "get", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e)))]);
    }
    return mcall(&py_str(" and "), "join", &[parts.clone()]);
    Py::None
}
pub fn _adds_one_more_allergen(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let _ = py_str("The allergy a line adds to the ones on record, or []: \"and milk too\" after peanuts.\n\n    Only an allergy the person already gave is added to: a food word on its own says nothing\n    about what it is being added to, and guessing \"milk\" into an allergy for someone who has\n    not said they are allergic to anything is the one mistake worth never making.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    known = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    held = { let __b = mcall(&known.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&held.clone())) } }) {
    return py_list(vec![]);
    }
    return crate::memory::facts::one_more(&[text.clone()]);
    Py::None
}
pub fn _adds_a_detail(args: &[Py]) -> Py {
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
    let mut carried: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut known: Py = Py::None;
    let mut read: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line says something new about the thing we are on, with nothing to store.\n\n    A person who adds a detail (\"we live in a flat on the third floor\", \"i can put away about 200\n    a month\", \"something without meat\") is handing over the thing that makes the next answer\n    theirs. Saying it back is what a friend does; the stock line (\"got it, is there anything you\n    would like a hand with?\") could have come before any message at all.\n\n    The guards matter more than the rule: there has to be a topic for the detail to belong to, the\n    line has to say something the topic did not, and it must not be a question, a fault to\n    troubleshoot or a misbehaviour to answer kindly.\n    ");
    if py_truthy(&Py::Bool(py_is(&memory.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    record = { let __b = crate::memory::topics::carried_topic(&[memory.clone(), text.clone()]); if py_truthy(&__b) { __b } else { crate::memory::topics::current(&[memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&record.clone()))) {
    return Py::Bool(false);
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    read = _repaired(&[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WANTS_ANSWER(), "search", &[read.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TROUBLE(), "search", &[read.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.trouble", "reads_as_trouble", &[read.clone(), mcall(&read.clone(), "split", &[])])) {
    return Py::Bool(false);
    }
    said_words = _words(&[clean.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[said_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[A-Za-z\\u00c0-\\u024f]"), _strip_lead_in(&[clean.clone()])]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[A-Za-z\\u00c0-\\u024f]"), mcall(&g_REPAIR_FRAME(), "sub", &[py_str(""), clean.clone()])]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_le(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DIGIT(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(false);
    }
    carried = op_add(&op_add(&{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, &py_str(" ")), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]));
    if py_truthy(&{ let __b = mcall(&g_DEATH_WORDS(), "search", &[carried.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__NOT_A_DETAIL_SUBJECT(), "search", &[carried.clone()]) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::Bool(false);
    }
    }
    known = call_builtin("set", &[_words(&[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("details")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    earlier = __item.clone();
    let __aug3 = op_bitor(&known.clone(), &call_builtin("set", &[_words(&[call_builtin("str", &[earlier.clone()]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e)));
    known = __aug3;
    }
    if py_truthy(&{ let __b = mcall(&g__NOT_A_DETAIL(), "search", &[read.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ANSWER_OPENER(), "match", &[read.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKED_FOR(), "match", &[read.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__SAID_YES_NO(), "match", &[read.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKS_MIDWAY(), "search", &[read.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__FAULT(), "search", &[read.clone()]) } } } } } } } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&known.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
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
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of `text` that carry meaning: no articles, no auxiliaries, lower case.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__EMPTY(), &word.clone()))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _names_nothing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words left are only filler and vague ones, so there is nothing to say back.");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[body.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_VAGUE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return Py::Bool(!py_truthy(&named.clone()));
    Py::None
}
pub fn _repairs(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fixed: Py = Py::None;
    let mut now: Py = Py::None;
    let mut now_at: Py = Py::None;
    let mut pairs: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut was: Py = Py::None;
    let mut was_at: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with the understanding engine's own corrections in it.\n\n    A friend does not say your typo back. \"my dog is geting old\" was answered \"your dog is geting\n    old\", and \"im alergic\" came back spelled the same way; the engine had already repaired both\n    words for its own reading, and the echo only used its chat-speak table (SHORT), which is a\n    different list. So the echo takes every word the engine repaired, in place, and keeps the\n    words it left alone - a hyphen, a digit or a name is what they typed.\n\n    A contraction typed without its apostrophe is one word where the reading has two: \"were going\n    to spain in may\" reads as \"we are going to spain in may\", and the echo knows neither line, so\n    it never got said back at all. The engine keeps that table (`LEFT_OFF_CONTRACTION`), so the\n    count of words comes from it rather than from a second list here, and only the words it adds\n    are allowed to appear where the person typed one.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    plain = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("plain")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[plain.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&plain.clone(), "strip", &[]))) } }) {
    return clean.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&clean.clone(), "split", &[]), mcall(&plain.clone(), "split", &[])]), 2, None);
    typed = __unpk1[0].clone();
    fixed = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&typed.clone()))) {
    return clean.clone();
    }
    let __unpk2 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0), Py::Int(0)]), 3, None);
    pairs = __unpk2[0].clone();
    was_at = __unpk2[1].clone();
    now_at = __unpk2[2].clone();
    while py_truthy(&{ let __b = op_lt(&was_at.clone(), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { op_lt(&now_at.clone(), &call_builtin("len", &[fixed.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    words = mcall(&gen_understanding::understanding::normalize::g_LEFT_OFF_CONTRACTION(), "get", &[mcall(&geti(&typed.clone(), &was_at.clone()), "lower", &[])]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&fixed.clone(), &now_at.clone(), &op_add(&now_at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    let _ = mcall(&pairs.clone(), "append", &[py_tuple(vec![geti(&typed.clone(), &was_at.clone()), mcall(&py_str(" "), "join", &[words.clone()])])]);
    let __aug3 = op_add(&now_at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    now_at = __aug3;
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&geti(&typed.clone(), &was_at.clone()), "lower", &[]), &mcall(&geti(&fixed.clone(), &now_at.clone()), "lower", &[]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&geti(&typed.clone(), &was_at.clone()), "isalpha", &[]); if !py_truthy(&__b) { __b } else { mcall(&mcall(&mcall(&geti(&fixed.clone(), &now_at.clone()), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]), "isalpha", &[]) } } } }) {
    let _ = mcall(&pairs.clone(), "append", &[py_tuple(vec![geti(&typed.clone(), &was_at.clone()), geti(&fixed.clone(), &now_at.clone())])]);
    let __aug4 = op_add(&now_at.clone(), &Py::Int(1));
    now_at = __aug4;
    } else {
    return clean.clone();
    }
    }
    let __aug5 = op_add(&was_at.clone(), &Py::Int(1));
    was_at = __aug5;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&was_at.clone(), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&now_at.clone(), &call_builtin("len", &[fixed.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return clean.clone();
    }
    words = py_list(vec![]);
    let __iter6 = py_iter_or_raise(&pairs.clone());
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    was = __unpk7[0].clone();
    now = __unpk7[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&was.clone(), &now.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&was.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&mcall(&mcall(&now.clone(), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]), "isalpha", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&now.clone(), &py_str(" "))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&was.clone(), "lower", &[]), &mcall(&now.clone(), "lower", &[]))) } } } } } }) {
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&mcall(&slice_or_raise(&was.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { op_add(&mcall(&slice_or_raise(&now.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&now.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { now.clone() }]);
    } else {
    let _ = mcall(&words.clone(), "append", &[was.clone()]);
    }
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _says_something(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let _ = py_str("Whether the words left after a repair frame say something, rather than naming a bare value.\n\n    \"i got the second interview\" is a clause; \"three\", \"the tap\" and \"caffeine\" are the thing the\n    person put in place of another and nothing else. A repair's own verb is kept for those, so the\n    echo does not read \"Got it - three\", which no longer says three of what.\n    ");
    return crate::memory::facts::_tail_has_a_verb(&[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = body.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _the_person_the_pointer_is_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person the pointer line is about (\"wife\", \"mum\", \"sister\"), or \"\" for the person\n    talking.");
    found = mcall(&g_SOMEONE_ELSE_POINTS_AT_ANOTHER(), "search", &[body.clone()]);
    if py_truthy(&found.clone()) {
    word = mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]), "lower", &[]);
    return if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("own"), py_str("self")]), &word.clone()))) { py_str("") } else { word.clone() };
    }
    if py_truthy(&mcall(&g_PRONOUN_POINTS_AT_ANOTHER(), "search", &[body.clone()])) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter2 = py_iter_or_raise(&crate::memory::facts::g_RELATIONS());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bmy\\s+"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return word.clone();
    }
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_thing_the_chat_is_on(args: &[Py]) -> Py {
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
    let mut named_words: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut stems: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the chat is about, in the words the person used, whether they said one or\n    several, and the plural: (\"puppy\", False, \"puppies\") or (\"tomatoes\", True, \"tomatoes\").\n    (\"\", False, \"\") when the chat is not about a thing a person can have another of.\n\n    The subject is kept as a stem (\"tomatoe\"), so the word is looked for again in what the person\n    actually said: a reply that names the thing has to use their word for it, not the stem.\n    ");
    phrase = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = crate::memory::topics::subject(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    named_words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&named_words.clone(), "update", &[_a_thing_the_line_named(&[mcall(&turn.clone(), "get", &[py_str("user")])])]);
    }
    }
    let __aug2 = op_sub(&named_words.clone(), &g_NOT_A_SECOND_OF());
    named_words = __aug2;
    if py_truthy(&Py::Bool(!py_truthy(&named_words.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    stems = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named_words.clone()) { let mut word: Py = __item.clone(); __out.push(crate::memory::topics::stem(&[word.clone()])); } py_set(__out) })() };
    stem = py_str("");
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z]+"), phrase.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&crate::memory::named::g_NOT_THE_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_THING_TO_HAVE_ANOTHER(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&named_words.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&stems.clone(), &crate::memory::topics::stem(&[word.clone()]))) } } } } } } } }) {
    stem = word.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stem.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING_TO_HAVE_ANOTHER(), &stem.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_RELATIONS(), &stem.clone())) } } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    if py_truthy(&{ let __b = _something_they_avoid(&[memory.clone(), stem.clone()]); if py_truthy(&__b) { __b } else { _a_name_the_store_holds(&[memory.clone(), stem.clone()]) } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    said = py_str("");
    let __iter4 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter5 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &stem.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&crate::memory::topics::stem(&[word.clone()]), &stem.clone())) } }) {
    said = word.clone();
    break;
    }
    }
    if py_truthy(&said.clone()) {
    break;
    }
    }
    said = { let __b = said.clone(); if py_truthy(&__b) { __b } else { stem.clone() } };
    return py_tuple(vec![said.clone(), Py::Bool(!py_eq(&said.clone(), &stem.clone())), _plural(&[said.clone()])]);
    Py::None
}
pub fn _a_name_the_store_holds(args: &[Py]) -> Py {
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
    let mut stem: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stem") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stem"))) };
    let mut found: Py = Py::None;
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut other: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the word is a name somebody or some animal was given, not a thing of its own.\n\n    \"my sister is called meg\" then \"my brother has one too\" was read as a pointer at another\n    one, and named it: \"A second meg, then.\" A name is not a thing a person has another of, so\n    a name the store holds - a pet's, a person's, or one given on the last turn or two - is left\n    to the reader that answers the line as it stands.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    names = py_list(vec![]);
    known = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("name")]) } }) {
    let _ = mcall(&names.clone(), "append", &[call_builtin("str", &[geti(&pet.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    other = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[other.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&other.clone(), "get", &[py_str("name")]) } }) {
    let _ = mcall(&names.clone(), "append", &[call_builtin("str", &[geti(&other.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    person = mcall(&memory.clone(), "get", &[py_str("person")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[person.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&person.clone(), "get", &[py_str("name")]) } }) {
    let _ = mcall(&names.clone(), "append", &[call_builtin("str", &[geti(&person.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter3 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    found = mcall(&g_A_NAME_GIVEN(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    let _ = mcall(&names.clone(), "append", &[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&name.clone()) {  __out.push(Py::Bool(py_eq(&crate::memory::topics::stem(&[name.clone()]), &stem.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _something_they_avoid(args: &[Py]) -> Py {
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
    let mut stem: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stem") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stem"))) };
    let mut known: Py = Py::None;
    let mut meal: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the thing the chat is on is something the person is allergic to or avoiding.\n\n    \"im allergic to nuts\" then \"my son has one too\" points at the allergy the son shares, not at\n    more nuts: the \"one\" the son has is the allergy, and \"More nuts?\" is the wrong reply to it.\n    A thing the store holds as an allergy or as a food to avoid is not something a person has\n    another of, so the pointer is left to the reader that answers the allergy.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    known = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    meal = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("meal")]) } else { py_dict(vec![]) };
    let __iter1 = py_iter_or_raise(&op_add(&call_builtin("list", &[{ let __b = mcall(&known.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[{ let __b = mcall(&meal.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&crate::memory::topics::stem(&[call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))]), &stem.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _plural(args: &[Py]) -> Py {
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
    let _ = py_str("The plural of a thing the person named, so a reply can say \"two puppies\".");
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("s")])) {
    return word.clone();
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("x"), py_str("z"), py_str("ch"), py_str("sh")])])) {
    return op_add(&word.clone(), &py_str("es"));
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    }
    return op_add(&word.clone(), &py_str("s"));
    Py::None
}
pub fn _is_a_contraction(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the word is a contraction (\"isn't\", \"wont\", \"didnt\") rather than a thing.");
    return { let __b = Py::Bool(py_contains(&word.clone(), &py_str("n't"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ENDS_THE_THING_VERB(), &word.clone())) } };
    Py::None
}
pub fn _a_thing_the_line_named(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut head: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The head words of the noun phrases a line names behind a determiner: \"we got a puppy last\n    year\" -> {\"puppy\"}, \"i started a new job\" -> {\"job\"}, \"we called the puppy rufus\" ->\n    {\"rufus\"} (a name, so the caller stands down on it).");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_DETERMINER(), "fullmatch", &[word.clone()])))) {
    continue;
    }
    head = py_str("");
    let __iter3 = py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&at.clone(), &Py::Int(1)), &op_add(&at.clone(), &Py::Int(4)), &Py::None));
    for __item in __iter3 {
    after = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ENDS_THE_THING(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { _is_a_contraction(&[after.clone()]) } } } }) {
    break;
    }
    head = after.clone();
    }
    if py_truthy(&head.clone()) {
    let _ = mcall(&out.clone(), "add", &[head.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _a_loss_just_before(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the last turn or two is a death: that turn belongs to the feelings reader.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&crate::memory::facts::g_DIED(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _another_of_what_we_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut body: Py = Py::None;
    let mut found: Py = Py::None;
    let mut left: Py = Py::None;
    let mut one: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut several: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reply to a line that only points at another of the thing the chat is about, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    body = mcall(&clean.clone(), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&body.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&body.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&body.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_DECISION(), "search", &[body.clone()])) {
    return Py::None;
    }
    found = mcall(&g_POINTS_AT_ANOTHER(), "search", &[body.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&g_POINTS_AT_ANOTHER(), "sub", &[py_str(" "), body.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POINTER_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&left.clone()) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_REPORTS_AN_ACTION(), "search", &[body.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("one before"), py_str("the same one")]), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&_a_loss_just_before(&[memory.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_thing_the_chat_is_on(&[memory.clone()]), 3, None);
    one = __unpk1[0].clone();
    several = __unpk1[1].clone();
    plural = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&one.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NO_REFERENT()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    person = _the_person_the_pointer_is_about(&[body.clone(), memory.clone()]);
    if py_truthy(&person.clone()) {
    pool = if py_truthy(&several.clone()) { g_ANOTHER_SOME_FOR_SOMEONE() } else { g_ANOTHER_ONE_FOR_SOMEONE() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("person".to_string(), person.clone()), ("thing".to_string(), one.clone()), ("things".to_string(), plural.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]), &py_str("one before")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), one.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_BEFORE_ONE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&several.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("things".to_string(), plural.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ANOTHER_ONES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), one.clone()), ("things".to_string(), plural.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ANOTHER_ONE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _only_the_correction_in_front(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kept: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut body: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut front: Py = Py::None;
    let _ = py_str("True when `kept` is `body` with nothing in front of it but the correction's own opener.\n\n    The opener is read by the correction reading itself (`facts.is_a_correction_opener`), so every\n    word that announces a correction there - \"no\", \"no wait\", \"sorry\", \"actually\", \"scratch that\" -\n    is an opener here too. This list used to be its own pattern that only gave way to a \"no\", so\n    \"wait, four brothers\" was said back whole and the person got their own opener handed to them:\n    \"Got it - wait, four brothers.\".");
    kept = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = kept.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    body = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = body.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&body.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&kept.clone(), &body.clone())) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&mcall(&kept.clone(), "lower", &[]), "endswith", &[mcall(&body.clone(), "lower", &[])])))) {
    return Py::Bool(false);
    }
    front = slice_or_raise(&kept.clone(), &Py::None, &op_sub(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[body.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    return call_builtin("bool", &[{ let __b = mcall(&front.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { crate::memory::facts::is_a_correction_opener(&[front.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _echo(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut back: Py = Py::None;
    let mut body: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_it: Py = Py::None;
    let _ = py_str("What they said, said back and short: \"my sister is called Meg\" -> \"your sister is called\n    Meg\".");
    clean = gen_understanding::understanding::normalize::without_a_leading_exclamation(&[clean.clone()]);
    said_it = _repairs(&[clean.clone(), memory.clone()]);
    body = _strip_lead_in(&[said_it.clone()]);
    if py_truthy(&{ let __b = body.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_something(&[body.clone()]))) } }) {
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_it.clone()); __kws.extend(vec![("keep_the_repair".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _strip_lead_in(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kept.clone(), &body.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_only_the_correction_in_front(&[kept.clone(), body.clone()]))) } }) {
    body = kept.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&body.clone()))) {
    return Py::None;
    }
    if py_truthy(&_names_nothing(&[body.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__OPENS_WITH_A_LINK(), "match", &[body.clone()]); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[mcall(&body.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MONEY_ONLY(), "match", &[mcall(&body.clone(), "strip", &[py_str(" .,!?")])]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&op_le(&call_builtin("len", &[body.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST())) {
    said = body.clone();
    } else {
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_STOP(), "split", &[body.clone()])) { let mut clause: Py = __item.clone(); if py_truthy(&mcall(&clause.clone(), "strip", &[])) {  __out.push(mcall(&clause.clone(), "strip", &[])); } } py_list(__out) })() };
    said = if py_truthy(&clauses.clone()) { mcall(&py_str(", "), "join", &[slice_or_raise(&clauses.clone(), &Py::None, &g_CLAUSES(), &Py::None)]) } else { body.clone() };
    }
    said = mcall(&_cut(&[said.clone()]), "strip", &[py_str(" ,;:-!?.—")]);
    said = mcall(&mcall(&g_POLITE(), "sub", &[py_str(""), said.clone()]), "strip", &[py_str(" ,;:-")]);
    said = _tail_off(&[said.clone()]);
    said = mcall(&_contrast_off(&[said.clone()]), "strip", &[py_str(" ,;:-")]);
    said = mcall(&_reason_off(&[said.clone(), clean.clone()]), "strip", &[py_str(" ,;:-")]);
    said = mcall(&_pick_off(&[said.clone()]), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    back = _tidy(&[mcall(&g_KIN_POSSESSIVE(), "sub", &[py_str("\\1's"), mcall(&crate::memory::you(&[_spoken(&[said.clone()])]), "strip", &[py_str(" ,;:-")])])]);
    if py_truthy(&Py::Bool(!py_truthy(&back.clone()))) {
    return Py::None;
    }
    back = mcall(&g_HAVE_GOT(), "sub", &[py_str("\\1've got"), back.clone()]);
    counted = mcall(&g_A_COUNT_NOT_A_STATE(), "match", &[back.clone()]);
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_UNITS_OF_MEASURE(), &mcall(&mcall(&counted.clone(), "group", &[Py::Int(2)]), "lower", &[]))) } }) {
    back = op_add(&py_str("you've got "), &slice_or_raise(&back.clone(), &mcall(&counted.clone(), "start", &[Py::Int(1)]), &Py::None, &Py::None));
    }
    return mcall(&g_WHOS(), "sub", &[py_str("who's"), back.clone()]);
    Py::None
}
pub fn said_back(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("Their line in the bot's own words, short: \"i work all day\" -> \"you work all day\".\n\n    The same reading the echo reply is built from, for a module that wants to name what the\n    person said inside a sentence of its own rather than hand it back on its own.");
    return _echo(&[clean.clone(), memory.clone()]);
    Py::None
}
pub fn _only_a_dislike(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let _ = py_str("True when the line is a dislike of a thing and nothing else.\n\n    \"hates\" and \"can't\" are in `TROUBLE`, which is there for a person being difficult: \"my boss\n    keeps shouting at me\" is trouble and must not be filed away. A dislike is the same words about\n    a thing — \"i hate cats\", \"i cant stand cats\" — and taking the taste verb out leaves nothing\n    that reads as someone's misbehaviour.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("tastes")])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_TROUBLE(), "search", &[mcall(&crate::memory::taste::g_DISLIKE_IN(), "sub", &[py_str(" "), body.clone()])])));
    Py::None
}
pub fn only_a_dislike(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("True when the line is a stated dislike of a thing and nothing else.\n\n    \"i hate cats\" and \"i cant stand cats\" are facts about the person's taste, and both words are\n    also words for a bad day (\"hate\", \"can't\"): read as mood they get \"That sounds rough\", which\n    is an answer for an event that happened to them, not for a thing they do not like.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    return _only_a_dislike(&[_strip_lead_in(&[clean.clone()]), _learned(&[clean.clone(), memory.clone()])]);
    Py::None
}
pub fn _tells_it(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { Py::None };
    let mut body: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut others: Py = Py::None;
    let _ = py_str("Whether the line can be said back: a state of their world, not someone's misbehaviour.");
    body = _strip_lead_in(&[clean.clone()]);
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&others.clone()) { let mut entry: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&entry.clone(), &py_str("name"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&entry.clone(), &py_str("age"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::memory::facts::foods_ruled_out(&[found.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_TROUBLE(), "search", &[body.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_diet(&[found.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_only_a_dislike(&[body.clone(), found.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("done")])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_STATE(), "search", &[body.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[body.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_FRAGMENT()) } };
    Py::None
}
pub fn _a_diet(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut entry: Py = Py::None;
    let _ = py_str("\"she doesn't eat meat\" has a \"doesn't\" in it, but it is what she eats, not what she does\n    to the person: the store read a diet in the line, so the negation is not a complaint.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "get", &[py_str("no_meat")]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&entry.clone(), "get", &[py_str("diet")])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_the_figure(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line says nothing but an age or a height: \"im 34\", \"im 6 foot 2\", \"im 34 this\n    year\". A figure inside a longer sentence is not the thing the sentence is about.");
    body = mcall(&py_str(" "), "join", &[mcall(&mcall(&g_FIGURE_LEAD(), "sub", &[py_str(""), _strip_lead_in(&[clean.clone()])]), "split", &[])]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&body.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!;:")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FIGURE_FILLER(), &mcall(&word.clone(), "lower", &[]))) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = stdlib_call("re", "fullmatch", &[py_str("\\d{1,3}(?:st|nd|rd|th)?(?:(?:cm|m|ft|in|kg|st|lbs?)s?)?"), word.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_FIGURE_UNIT(), &mcall(&word.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FIGURE_WORD(), &mcall(&word.clone(), "lower", &[]))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_rest_of_a_question(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("Whether the line is the tail of the question before it, with no fact of its own.\n\n    \"and the weather there in june\" after \"whats the capital of australia\": a conjunction, a\n    thing and a date, and nothing else. Three things have to hold, because each one alone is\n    wrong somewhere:\n\n    - there is a conjunction (\"and\", \"also\", \"what about\"), so it is the second half of something;\n    - the rest has no verb and nobody in it, so it states nothing: \"and i have two cats\" and\n      \"also my knee is bad\" are things they told us, \"and the weather there in june\" is not;\n    - the line before it asked something. \"i have two cats\" then \"and a dog\" is a dog; \"whats the\n      capital of australia\" then \"and the weather there in june\" is the rest of that question.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    match_ = mcall(&g_A_CONTINUATION(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.;:-!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_CLAUSE(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SOMEONE(), "search", &[rest.clone()]) } }) {
    return Py::Bool(false);
    }
    return reads_as_an_ask(&[before.clone()]);
    Py::None
}
pub fn _after_a_reaction(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`text` without a short reaction clause in front of it (\"lucky you.\", \"fair enough,\").");
    match_ = mcall(&g__REACTION_CLAUSE(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return text.clone();
    }
    clause = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[])]), "lower", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'’")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return text.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_REACTION_PHRASES(), &clause.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_REACTION_WORDS(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return text.clone();
    }
    return mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    Py::None
}
pub fn _strip_lead_in(args: &[Py]) -> Py {
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
    let mut keep_the_repair: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep_the_repair") { v.clone() }
    else { Py::Bool(false) };
    let mut bare: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut done: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut low: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("`clean` without the \"hi, so,\" and the like in front of the fact.\n\n    `keep_the_repair` leaves the repair's own verb on (\"you mean three\"), for a correction whose\n    fact is a bare number or noun: \"Got it - three.\" is not a sentence and no longer says three\n    what, where \"Got it - you mean three.\" is the person's own frame and still reads.\n    ");
    text = mcall(&clean.clone(), "strip", &[py_str(" ,;:-")]);
    text = mcall(&g_NO_THEN_A_LINE_ONLY(), "sub", &[py_str(""), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&keep_the_repair.clone()))) {
    text = mcall(&g_NO_LEAD(), "sub", &[py_str(""), text.clone()]);
    }
    changed = Py::Bool(true);
    while py_truthy(&changed.clone()) {
    changed = Py::Bool(false);
    bare = if py_truthy(&keep_the_repair.clone()) { text.clone() } else { mcall(&g_NO_LEAD(), "sub", &[py_str(""), text.clone()]) };
    if py_truthy(&Py::Bool(!py_eq(&bare.clone(), &text.clone()))) {
    let __unpk1 = py_unpack(&py_tuple(vec![bare.clone(), Py::Bool(true)]), 2, None);
    text = __unpk1[0].clone();
    changed = __unpk1[1].clone();
    continue;
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEAD_IN()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    lead = __item.clone();
    low = mcall(&text.clone(), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&low.clone(), "startswith", &[lead.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), py_str(" "), py_str(","), py_str(";"), py_str(":"), py_str("-"), py_str("!"), py_str("?"), py_str(".")]), &slice_or_raise(&low.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &Py::None))) } }) {
    text = mcall(&slice_or_raise(&text.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "lstrip", &[py_str(" ,;:-!?.")]);
    changed = Py::Bool(true);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&changed.clone()))) {
    done = _after_a_reaction(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&done.clone(), &text.clone()))) {
    let __unpk3 = py_unpack(&py_tuple(vec![done.clone(), Py::Bool(true)]), 2, None);
    text = __unpk3[0].clone();
    changed = __unpk3[1].clone();
    }
    }
    }
    return text.clone();
    Py::None
}
pub fn without_the_frame(args: &[Py]) -> Py {
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
    let _ = py_str("\"sorry, i mean saturday\" with the apology and the repair taken off: \"saturday\".\n\n    For a module that says a correction back in the person's own words: the frame is what they\n    said to steer, not the thing they steered to.");
    return _strip_lead_in(&[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    Py::None
}
pub fn _spoken(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut before: Py = Py::None;
    let mut out: Py = Py::None;
    let mut place: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`said` with the chat-speak folded, so the echo does not read \"im\" or \"dont\" or \"yrs\".");
    said = mcall(&g_COUNTED_STARS(), "sub", &[{ let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); let match_: Py = __kwp(args, 0, "match").unwrap_or(Py::None); geti(&g_STAR_COUNT(), &call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("count")])]).unwrap_or_else(|e| exc::raise(e))) }) }, said.clone()]);
    said = mcall(&g_ONE_STAR(), "sub", &[py_str("an A*"), said.clone()]);
    said = mcall(&g_COUNTED_LETTERS(), "sub", &[{ let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); let match_: Py = __kwp(args, 0, "match").unwrap_or(Py::None); op_add(&op_add(&op_add(&geti(&g_LETTER_COUNT(), &call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("count")])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" ")), &mcall(&mcall(&match_.clone(), "group", &[py_str("letter")]), "upper", &[])), &py_str("s")) }) }, said.clone()]);
    if py_truthy(&mcall(&g_A_DEGREE(), "search", &[said.clone()])) {
    said = mcall(&g_DEGREE_CLASS(), "sub", &[{ let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); let match_: Py = __kwp(args, 0, "match").unwrap_or(Py::None); op_add(&op_add(&mcall(&match_.clone(), "group", &[py_str("first")]), &py_str(":")), &mcall(&match_.clone(), "group", &[py_str("second")])) }) }, said.clone()]);
    }
    said = mcall(&g_WITH_SLASH(), "sub", &[py_str("with "), said.clone()]);
    said = mcall(&g_TWO_TO(), "sub", &[{ let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let after = after.clone(); let bare = bare.clone(); let before = before.clone(); let out = out.clone(); let place = place.clone(); let said = said.clone(); let tail = tail.clone(); let word = word.clone(); let words = words.clone(); let match_: Py = __kwp(args, 0, "match").unwrap_or(Py::None); op_add(&slice_or_raise(&mcall(&match_.clone(), "group", &[Py::Int(0)]), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("to")) }) }, said.clone()]);
    said = mcall(&g_TWO_PLACE(), "sub", &[py_str("to "), said.clone()]);
    said = mcall(&g_TWO_VERB(), "sub", &[py_str("to "), said.clone()]);
    out = py_list(vec![]);
    words = mcall(&said.clone(), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    place = __unpk2[0].clone();
    word = __unpk2[1].clone();
    bare = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?")]);
    tail = slice_or_raise(&word.clone(), &call_builtin("len", &[mcall(&word.clone(), "rstrip", &[py_str(".,;:!?")])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    after = if py_truthy(&op_lt(&op_add(&place.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { mcall(&mcall(&geti(&words.clone(), &op_add(&place.clone(), &Py::Int(1))), "lower", &[]), "strip", &[py_str(".,;:!?")]) } else { py_str("") };
    before = if py_truthy(&place.clone()) { mcall(&mcall(&geti(&words.clone(), &op_sub(&place.clone(), &Py::Int(1))), "lower", &[]), "strip", &[py_str(".,;:!?")]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_eq(&bare.clone(), &py_str("yer")))) {
    let _ = mcall(&out.clone(), "append", &[op_add(&if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_YES_AFTER(), &after.clone())) } }) { py_str("yeah") } else { py_str("your") }, &tail.clone())]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&bare.clone(), &py_str("its"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ITS_OWNS_AFTER(), &before.clone())) } }) {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_SPEAK(), &bare.clone()))) {
    let _ = mcall(&out.clone(), "append", &[op_add(&geti(&g_SPEAK(), &bare.clone()), &tail.clone())]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_SHORT(), &bare.clone()))) {
    let _ = mcall(&out.clone(), "append", &[op_add(&geti(&g_SHORT(), &bare.clone()), &tail.clone())]);
    } else {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    }
    }
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _pick_off(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("`said` without the words that say a choice was made: \"lets go with the focus\" -> \"the\n    focus\", \"make it a dog\" -> \"a dog\".\n\n    A frame with nothing after it (\"lets go with that\") is left whole: there is no thing to say\n    back, and the line is not an echo at all.\n    ");
    match_ = mcall(&g_PICK_FRAME(), "match", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return said.clone();
    }
    rest = mcall(&slice_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,;:-.!?—")]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return said.clone();
    }
    return rest.clone();
    Py::None
}
pub fn _tail_off(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`said` without the chat-speak hanging off the end, so the echo is the fact and not the\n    filler: \"im veggie tho\" -> \"you're veggie\", \"im veggie\" unchanged.");
    said = crate::memory::filler::without_filler(&[said.clone()]);
    words = mcall(&said.clone(), "split", &[]);
    while py_truthy(&words.clone()) {
    low = mcall(&mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "lower", &[]), "strip", &[py_str(" ,;:-")]);
    cut = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::facts::g_FILLER_TAIL_PHRASES()) { let mut phrase: Py = __item.clone(); if py_truthy(&mcall(&low.clone(), "endswith", &[phrase.clone()])) {  __out.push(phrase.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&cut.clone())); if py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    break;
    }
    words = slice_or_raise(&words.clone(), &Py::None, &op_neg(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    return mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:-")]);
    Py::None
}
pub fn _contrast_off(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut old: Py = Py::None;
    let mut restated: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("`said` without the wrong value it replaced: \"my budget is 400 a month not 300\" -> \"my\n    budget is 400 a month\".\n\n    Only a value is taken off. \"my sister is not coming\" and \"my son is not well\" carry the same\n    shape and are whole facts, so the cut needs a value on one side of the \"not\": the number or\n    the day it corrects, on either side of it.\n    ");
    again = mcall(&g_CONTRAST_SAID_AGAIN(), "match", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&{ let __b = again.clone(); if !py_truthy(&__b) { __b } else { _a_value(&[mcall(&again.clone(), "group", &[py_str("old")])]) } }) {
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![mcall(&again.clone(), "group", &[py_str("thing")]), mcall(&again.clone(), "group", &[py_str("new")])]));
    }
    match_ = mcall(&g_CONTRAST(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return said.clone();
    }
    old = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+"), mcall(&mcall(&match_.clone(), "group", &[py_str("old")]), "strip", &[py_str(" .,;:-")])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = old.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &mcall(&geti(&old.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    old = slice_or_raise(&old.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&old.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[a-z0-9]"), geti(&old.clone(), &Py::Int(0)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return said.clone();
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+"), mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[py_str(" .,;:-")])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_VALUE(), &mcall(&geti(&old.clone(), &Py::Int(0)), "lower", &[])))) {
    return said.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _a_value(&[geti(&old.clone(), &Py::Int(0))]); if py_truthy(&__b) { __b } else { { let __b = kept.clone(); if !py_truthy(&__b) { __b } else { _a_value(&[geti(&kept.clone(), &op_neg(&Py::Int(1)))]) } } } }))) {
    return said.clone();
    }
    restated = _the_value_put_right(&[said.clone(), match_.clone()]);
    if py_truthy(&restated.clone()) {
    return restated.clone();
    }
    return slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None);
    Py::None
}
pub fn _reason_off(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("`said` without the reason a correction gave for itself: \"you mean 75 because I lost some\"\n    -> \"you mean 75\".\n\n    The reason is the why of the change, not the value that changed, so a reply that names the\n    value says the value: \"you said 75 because.\" is the machine. A line that is not a correction\n    keeps its reason, because there the reason is the fact (\"you're tired because you were up all\n    night\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_CORRECTION_IN_FRONT(), "match", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return said.clone();
    }
    match_ = mcall(&g_A_REASON_TAIL(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;:-")]))) } }) {
    return said.clone();
    }
    return slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None);
    Py::None
}
pub fn _the_value_put_right(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut at: Py = Py::None;
    let mut between: Py = Py::None;
    let mut head: Py = Py::None;
    let mut second: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with the value a restated correction puts right, or \"\" for any other shape.\n\n    A correction can put the old value in the middle of the line rather than at its end: \"hes not\n    4 hes 5\", \"my son is not 4 hes 5\". The cut above leaves only what stands in front of the\n    \"not\", so the person was told \"Got it — he's.\" and \"Got it — your son is.\" - the\n    bot handing back the first half of their sentence with the fact gone. The value the line is\n    corrected to is the last one in the clause, and what is in front of it names the subject\n    without naming a value, so the line goes back together with that value on the end.\n    ");
    head = mcall(&mcall(&py_str(" "), "join", &[mcall(&slice_or_raise(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])]), "strip", &[py_str(" ,;:-—")]);
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+"), head.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_a_value(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+"), mcall(&mcall(&match_.clone(), "group", &[py_str("old")]), "strip", &[py_str(" .,;:-")])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return py_str("");
    }
    second = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); if py_truthy(&_a_value(&[geti(&words.clone(), &at.clone())])) {  __out.push(at.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&second.clone(), &Py::None))) {
    return py_str("");
    }
    between = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &second.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?'’")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&between.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_BETWEEN_VALUES(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    return op_add(&op_add(&head.clone(), &py_str(" ")), &mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &second.clone(), &Py::None, &Py::None)]));
    Py::None
}
pub fn _a_value(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("Whether this word is a value a correction can replace (\"300\", \"june\", \"£40\").");
    low = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,;:!?’'")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_VALUE(), "match", &[low.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&crate::memory::facts::g_DAYS(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &low.clone())) } };
    Py::None
}
pub fn _cut(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let _ = py_str("`said` down to MOST characters at a word boundary, so an echo stays an echo.");
    if py_truthy(&op_le(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST())) {
    return said.clone();
    }
    kept = mcall(&geti(&mcall(&slice_or_raise(&said.clone(), &Py::None, &g_MOST(), &Py::None), "rsplit", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "rstrip", &[py_str(" ,;:-")]);
    return if py_truthy(&Py::Bool(py_contains(&kept.clone(), &py_str(" ")))) { kept.clone() } else { slice_or_raise(&said.clone(), &Py::None, &g_MOST(), &Py::None) };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut index: Py = Py::None;
    let mut low: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`text` as one sentence piece: the second person in lower case, and nothing else\n    changed, so a proper name (\"Tokyo\") keeps the capital the person gave it.");
    words = mcall(&text.clone(), "split", &[]);
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    low = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("you"), py_str("your"), py_str("you're"), py_str("you've"), py_str("we"), py_str("our"), py_str("yourself")]), &low.clone()))) {
    word = low.clone();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FIRST_WORD(), &low.clone())) } }) {
    word = low.clone();
    }
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.ack".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALONE_AFTER_A_LOSS" => Some(g_ALONE_AFTER_A_LOSS()),
            "ALONE_AFTER_A_MOVE" => Some(g_ALONE_AFTER_A_MOVE()),
            "ALONE_TAIL" => Some(g_ALONE_TAIL()),
            "ANIMAL_COMING" => Some(g_ANIMAL_COMING()),
            "ANIMAL_HAD_ONCE" => Some(g_ANIMAL_HAD_ONCE()),
            "ANIMAL_TAKEN_BACK" => Some(g_ANIMAL_TAKEN_BACK()),
            "ANOTHER_ONE" => Some(g_ANOTHER_ONE()),
            "ANOTHER_ONES" => Some(g_ANOTHER_ONES()),
            "ANOTHER_ONE_FOR_SOMEONE" => Some(g_ANOTHER_ONE_FOR_SOMEONE()),
            "ANOTHER_SOME_FOR_SOMEONE" => Some(g_ANOTHER_SOME_FOR_SOMEONE()),
            "ANYBODY_AT_ALL" => Some(g_ANYBODY_AT_ALL()),
            "AN_AGE_ALONE" => Some(g_AN_AGE_ALONE()),
            "AN_AIM" => Some(g_AN_AIM()),
            "AN_ASK" => Some(g_AN_ASK()),
            "ARRIVE_VERB" => Some(g_ARRIVE_VERB()),
            "AS_WELL" => Some(g_AS_WELL()),
            "A_BIRTHDAY_TOLD" => Some(g_A_BIRTHDAY_TOLD()),
            "A_BREED_BEING_CHOSEN" => Some(g_A_BREED_BEING_CHOSEN()),
            "A_CHILD_IN_THE_LINE" => Some(g_A_CHILD_IN_THE_LINE()),
            "A_CLAUSE" => Some(g_A_CLAUSE()),
            "A_CLAUSE_OF_ITS_OWN" => Some(g_A_CLAUSE_OF_ITS_OWN()),
            "A_CLEAR" => Some(g_A_CLEAR()),
            "A_COMING_WORD" => Some(g_A_COMING_WORD()),
            "A_CONDITION_KIND" => Some(g_A_CONDITION_KIND()),
            "A_CONTINUATION" => Some(g_A_CONTINUATION()),
            "A_CORRECTION_IN_FRONT" => Some(g_A_CORRECTION_IN_FRONT()),
            "A_COUNT_NOT_A_STATE" => Some(g_A_COUNT_NOT_A_STATE()),
            "A_COUNT_OF_THEM" => Some(g_A_COUNT_OF_THEM()),
            "A_DATE_ALONE" => Some(g_A_DATE_ALONE()),
            "A_DATE_ON_A_HARD_PLAN" => Some(g_A_DATE_ON_A_HARD_PLAN()),
            "A_DATE_ON_A_PLAN" => Some(g_A_DATE_ON_A_PLAN()),
            "A_DATE_STILL_TO_COME" => Some(g_A_DATE_STILL_TO_COME()),
            "A_DATE_TOLD" => Some(g_A_DATE_TOLD()),
            "A_DAY_ASKED" => Some(g_A_DAY_ASKED()),
            "A_DAY_ON_A_HARD_PLAN" => Some(g_A_DAY_ON_A_HARD_PLAN()),
            "A_DAY_ON_A_PLAN" => Some(g_A_DAY_ON_A_PLAN()),
            "A_DECISION" => Some(g_A_DECISION()),
            "A_DEGREE" => Some(g_A_DEGREE()),
            "A_DETERMINER" => Some(g_A_DETERMINER()),
            "A_DOING" => Some(g_A_DOING()),
            "A_DOING_TURNED_DOWN" => Some(g_A_DOING_TURNED_DOWN()),
            "A_EVENT" => Some(g_A_EVENT()),
            "A_FEELING" => Some(g_A_FEELING()),
            "A_FEELING_ABOUT_IT" => Some(g_A_FEELING_ABOUT_IT()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_GROUP_ALONE" => Some(g_A_GROUP_ALONE()),
            "A_GROUP_ALONE_LINES" => Some(g_A_GROUP_ALONE_LINES()),
            "A_HAD_AN_ANIMAL" => Some(g_A_HAD_AN_ANIMAL()),
            "A_KNOWN_AGE_LINES" => Some(g_A_KNOWN_AGE_LINES()),
            "A_LIKE_ECHO" => Some(g_A_LIKE_ECHO()),
            "A_LIKE_OF_ONE_THING" => Some(g_A_LIKE_OF_ONE_THING()),
            "A_LIKE_WITH_TWO_HALVES" => Some(g_A_LIKE_WITH_TWO_HALVES()),
            "A_MARKED_HOUR" => Some(g_A_MARKED_HOUR()),
            "A_MEASURE" => Some(g_A_MEASURE()),
            "A_MILESTONE" => Some(g_A_MILESTONE()),
            "A_MONTH_ALONE" => Some(g_A_MONTH_ALONE()),
            "A_MOVE" => Some(g_A_MOVE()),
            "A_MOVED" => Some(g_A_MOVED()),
            "A_NAME_AFTER" => Some(g_A_NAME_AFTER()),
            "A_NAME_GIVEN" => Some(g_A_NAME_GIVEN()),
            "A_NAME_WAS_VIOLENT" => Some(g_A_NAME_WAS_VIOLENT()),
            "A_PERSON_ALONE" => Some(g_A_PERSON_ALONE()),
            "A_PERSON_ALONE_LINES" => Some(g_A_PERSON_ALONE_LINES()),
            "A_PERSON_LIKED" => Some(g_A_PERSON_LIKED()),
            "A_PET_WANTED" => Some(g_A_PET_WANTED()),
            "A_PLAN_VERB" => Some(g_A_PLAN_VERB()),
            "A_POINTED_AT" => Some(g_A_POINTED_AT()),
            "A_POSITION" => Some(g_A_POSITION()),
            "A_POSITION_LINES" => Some(g_A_POSITION_LINES()),
            "A_POSITION_MAYBE_A_THING" => Some(g_A_POSITION_MAYBE_A_THING()),
            "A_REASON_MARKER" => Some(g_A_REASON_MARKER()),
            "A_REASON_TAIL" => Some(g_A_REASON_TAIL()),
            "A_RELATION" => Some(g_A_RELATION()),
            "A_RESULT" => Some(g_A_RESULT()),
            "A_SECOND_PERSON" => Some(g_A_SECOND_PERSON()),
            "A_SOMEONE" => Some(g_A_SOMEONE()),
            "A_SPAN_OF_TIME" => Some(g_A_SPAN_OF_TIME()),
            "A_STAY_PERSON" => Some(g_A_STAY_PERSON()),
            "A_STAY_PLAN" => Some(g_A_STAY_PLAN()),
            "A_TRIP_PLAN" => Some(g_A_TRIP_PLAN()),
            "A_TWO_ALONE_LINES" => Some(g_A_TWO_ALONE_LINES()),
            "A_VAGUE_MANY" => Some(g_A_VAGUE_MANY()),
            "A_WANTED_ANIMAL" => Some(g_A_WANTED_ANIMAL()),
            "A_WANTED_ANIMAL_TAIL" => Some(g_A_WANTED_ANIMAL_TAIL()),
            "A_WANT_TURNED_DOWN" => Some(g_A_WANT_TURNED_DOWN()),
            "A_WEEKDAY" => Some(g_A_WEEKDAY()),
            "A_WONDERED_CONDITION" => Some(g_A_WONDERED_CONDITION()),
            "A_WONDER_SAID_NO" => Some(g_A_WONDER_SAID_NO()),
            "A_WORRIED_ABOUT_IT" => Some(g_A_WORRIED_ABOUT_IT()),
            "BACK_AT_IT" => Some(g_BACK_AT_IT()),
            "BACK_AT_IT_MINE" => Some(g_BACK_AT_IT_MINE()),
            "BACK_AT_IT_TAIL" => Some(g_BACK_AT_IT_TAIL()),
            "BEFORE_ONE" => Some(g_BEFORE_ONE()),
            "BETWEEN_VALUES" => Some(g_BETWEEN_VALUES()),
            "BLEED" => Some(g_BLEED()),
            "BODY" => Some(g_BODY()),
            "BODY_STATE_PHRASES" => Some(g_BODY_STATE_PHRASES()),
            "BODY_STATE_TAIL" => Some(g_BODY_STATE_TAIL()),
            "BODY_STATE_WORDS" => Some(g_BODY_STATE_WORDS()),
            "BUDGET_FRAMES" => Some(g_BUDGET_FRAMES()),
            "BUYING" => Some(g_BUYING()),
            "CHAT_OFFER_REPLY" => Some(g_CHAT_OFFER_REPLY()),
            "CHILD" => Some(g_CHILD()),
            "CLAUSES" => Some(g_CLAUSES()),
            "CLOCK_AFTER_TAIL" => Some(g_CLOCK_AFTER_TAIL()),
            "CLOCK_AT" => Some(g_CLOCK_AT()),
            "CLOCK_DETAIL" => Some(g_CLOCK_DETAIL()),
            "CLOCK_EVENT" => Some(g_CLOCK_EVENT()),
            "CLOCK_FLAT" => Some(g_CLOCK_FLAT()),
            "CLOCK_FLAT_TAIL" => Some(g_CLOCK_FLAT_TAIL()),
            "CLOCK_MORNING_TAIL" => Some(g_CLOCK_MORNING_TAIL()),
            "CLOCK_WORDS" => Some(g_CLOCK_WORDS()),
            "COMING_TO_STAY" => Some(g_COMING_TO_STAY()),
            "COMPLAINT" => Some(g_COMPLAINT()),
            "CONDITION_FAMILIES" => Some(g_CONDITION_FAMILIES()),
            "CONDITION_FOOD" => Some(g_CONDITION_FOOD()),
            "CONDITION_FRAMES" => Some(g_CONDITION_FRAMES()),
            "CONDITION_INITIALS" => Some(g_CONDITION_INITIALS()),
            "CONTRAST" => Some(g_CONTRAST()),
            "CONTRAST_SAID_AGAIN" => Some(g_CONTRAST_SAID_AGAIN()),
            "CORRECTED_ALONE" => Some(g_CORRECTED_ALONE()),
            "CORRECTED_WITH_UNIT" => Some(g_CORRECTED_WITH_UNIT()),
            "COUNT" => Some(g_COUNT()),
            "COUNTED" => Some(g_COUNTED()),
            "COUNTED_LETTERS" => Some(g_COUNTED_LETTERS()),
            "COUNTED_STARS" => Some(g_COUNTED_STARS()),
            "DATE_AHEAD" => Some(g_DATE_AHEAD()),
            "DAYS" => Some(g_DAYS()),
            "DAY_BEEN_AND_GONE" => Some(g_DAY_BEEN_AND_GONE()),
            "DAY_FAR" => Some(g_DAY_FAR()),
            "DAY_JUST_A_STATE" => Some(g_DAY_JUST_A_STATE()),
            "DAY_SOON" => Some(g_DAY_SOON()),
            "DEATH_WORDS" => Some(g_DEATH_WORDS()),
            "DEGREE_CLASS" => Some(g_DEGREE_CLASS()),
            "DESTINATION" => Some(g_DESTINATION()),
            "DESTINATION_ARRIVAL" => Some(g_DESTINATION_ARRIVAL()),
            "DIARY_TAIL" => Some(g_DIARY_TAIL()),
            "DIARY_THING" => Some(g_DIARY_THING()),
            "DIARY_TIME" => Some(g_DIARY_TIME()),
            "DIARY_TIME_TAIL" => Some(g_DIARY_TIME_TAIL()),
            "DIARY_WORDS" => Some(g_DIARY_WORDS()),
            "DIGIT" => Some(g_DIGIT()),
            "DREADED_OCCASION" => Some(g_DREADED_OCCASION()),
            "DRINK_FRAMES" => Some(g_DRINK_FRAMES()),
            "DURATION_UNITS" => Some(g_DURATION_UNITS()),
            "EARLY_RISE" => Some(g_EARLY_RISE()),
            "EARLY_TAIL" => Some(g_EARLY_TAIL()),
            "ENDS_THE_THING" => Some(g_ENDS_THE_THING()),
            "ENDS_THE_THING_VERB" => Some(g_ENDS_THE_THING_VERB()),
            "EXPLAIN_TAIL" => Some(g_EXPLAIN_TAIL()),
            "FACTS" => Some(g_FACTS()),
            "FAMILY" => Some(g_FAMILY()),
            "FAR_TAIL" => Some(g_FAR_TAIL()),
            "FEELING" => Some(g_FEELING()),
            "FIGURE_FILLER" => Some(g_FIGURE_FILLER()),
            "FIGURE_LEAD" => Some(g_FIGURE_LEAD()),
            "FIGURE_UNIT" => Some(g_FIGURE_UNIT()),
            "FIGURE_WORD" => Some(g_FIGURE_WORD()),
            "FIRST_WORD" => Some(g_FIRST_WORD()),
            "FOOD_FRAMES" => Some(g_FOOD_FRAMES()),
            "FOR_A_THING" => Some(g_FOR_A_THING()),
            "FRAGMENT" => Some(g_FRAGMENT()),
            "FRAMES" => Some(g_FRAMES()),
            "GIVEN_ANIMAL" => Some(g_GIVEN_ANIMAL()),
            "GIVEN_AWAY" => Some(g_GIVEN_AWAY()),
            "GIVEN_AWAY_REPLIES" => Some(g_GIVEN_AWAY_REPLIES()),
            "GOING" => Some(g_GOING()),
            "GOING_BAD" => Some(g_GOING_BAD()),
            "GOING_HOME" => Some(g_GOING_HOME()),
            "GOING_HOME_MINE" => Some(g_GOING_HOME_MINE()),
            "GOING_HOME_TAIL" => Some(g_GOING_HOME_TAIL()),
            "GOING_TAIL" => Some(g_GOING_TAIL()),
            "GOOD_MOOD" => Some(g_GOOD_MOOD()),
            "GREETING" => Some(g_GREETING()),
            "GROWING" => Some(g_GROWING()),
            "HABIT" => Some(g_HABIT()),
            "HABIT_TAIL" => Some(g_HABIT_TAIL()),
            "HAVE_A" => Some(g_HAVE_A()),
            "HAVE_GOT" => Some(g_HAVE_GOT()),
            "HAVE_THING" => Some(g_HAVE_THING()),
            "HEAVY_FRAMES" => Some(g_HEAVY_FRAMES()),
            "HEAVY_TURNS" => Some(g_HEAVY_TURNS()),
            "HOME_TAIL" => Some(g_HOME_TAIL()),
            "HOW_LONG_ALREADY" => Some(g_HOW_LONG_ALREADY()),
            "HURTS_A_PERSON" => Some(g_HURTS_A_PERSON()),
            "INTRODUCED_BEFORE" => Some(g_INTRODUCED_BEFORE()),
            "ITS_OWNS_AFTER" => Some(g_ITS_OWNS_AFTER()),
            "JOB_BEFORE" => Some(g_JOB_BEFORE()),
            "JOB_HAS_A_DAY" => Some(g_JOB_HAS_A_DAY()),
            "JOB_TAIL" => Some(g_JOB_TAIL()),
            "JOB_TO_DO" => Some(g_JOB_TO_DO()),
            "KIN_POSSESSIVE" => Some(g_KIN_POSSESSIVE()),
            "LATE_FINISH" => Some(g_LATE_FINISH()),
            "LATE_TAIL" => Some(g_LATE_TAIL()),
            "LEADERS" => Some(g_LEADERS()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "LEFT_CLOSED" => Some(g_LEFT_CLOSED()),
            "LEFT_OFF_CONTRACTION" => Some(g_LEFT_OFF_CONTRACTION()),
            "LENGTH_KEEPS" => Some(g_LENGTH_KEEPS()),
            "LENGTH_TAIL" => Some(g_LENGTH_TAIL()),
            "LETTER_COUNT" => Some(g_LETTER_COUNT()),
            "LIKES_FRAMES" => Some(g_LIKES_FRAMES()),
            "LIMIT_AFTER_THE_LINE" => Some(g_LIMIT_AFTER_THE_LINE()),
            "LIMIT_FRAMES" => Some(g_LIMIT_FRAMES()),
            "LONG_SHIFT" => Some(g_LONG_SHIFT()),
            "LONG_TAIL" => Some(g_LONG_TAIL()),
            "MAKE_A_VALUE" => Some(g_MAKE_A_VALUE()),
            "MEETING_TAIL" => Some(g_MEETING_TAIL()),
            "MEETING_TIME_TAIL" => Some(g_MEETING_TIME_TAIL()),
            "MEET_GREET" => Some(g_MEET_GREET()),
            "MONEY" => Some(g_MONEY()),
            "MONEY_ONLY" => Some(g_MONEY_ONLY()),
            "MONTHS" => Some(g_MONTHS()),
            "MOST" => Some(g_MOST()),
            "NIGHT_TAIL" => Some(g_NIGHT_TAIL()),
            "NOBODY_AROUND" => Some(g_NOBODY_AROUND()),
            "NOBODY_AROUND_ASK" => Some(g_NOBODY_AROUND_ASK()),
            "NOT_A_DAY_TO_LOOK_FORWARD_TO" => Some(g_NOT_A_DAY_TO_LOOK_FORWARD_TO()),
            "NOT_A_SECOND_OF" => Some(g_NOT_A_SECOND_OF()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_THING_TO_HAVE_ANOTHER" => Some(g_NOT_A_THING_TO_HAVE_ANOTHER()),
            "NOT_A_TRIP" => Some(g_NOT_A_TRIP()),
            "NOT_A_VALUE" => Some(g_NOT_A_VALUE()),
            "NOT_A_VISIT" => Some(g_NOT_A_VISIT()),
            "NOT_ONE_THING" => Some(g_NOT_ONE_THING()),
            "NOT_THEIRS" => Some(g_NOT_THEIRS()),
            "NOT_THE_HOURS" => Some(g_NOT_THE_HOURS()),
            "NO_LEAD" => Some(g_NO_LEAD()),
            "NO_REASON" => Some(g_NO_REASON()),
            "NO_REFERENT" => Some(g_NO_REFERENT()),
            "NO_THEN_A_LINE" => Some(g_NO_THEN_A_LINE()),
            "NO_THEN_A_LINE_ONLY" => Some(g_NO_THEN_A_LINE_ONLY()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "OBLIGED" => Some(g_OBLIGED()),
            "OCCASIONS" => Some(g_OCCASIONS()),
            "OCCASION_FLAT_TAIL" => Some(g_OCCASION_FLAT_TAIL()),
            "OCCASION_TAIL" => Some(g_OCCASION_TAIL()),
            "OFFERS_A_CHAT" => Some(g_OFFERS_A_CHAT()),
            "OLD_ONE_TAIL" => Some(g_OLD_ONE_TAIL()),
            "ONE_STAR" => Some(g_ONE_STAR()),
            "ON_NIGHTS" => Some(g_ON_NIGHTS()),
            "ON_THE_MEND" => Some(g_ON_THE_MEND()),
            "ON_THE_MEND_MINE" => Some(g_ON_THE_MEND_MINE()),
            "ON_THE_MEND_TAIL" => Some(g_ON_THE_MEND_TAIL()),
            "OTHER_LIMIT_FRAMES" => Some(g_OTHER_LIMIT_FRAMES()),
            "OTHER_PERSON" => Some(g_OTHER_PERSON()),
            "PAST_JOB" => Some(g_PAST_JOB()),
            "PAST_JOB_PLAIN" => Some(g_PAST_JOB_PLAIN()),
            "PAST_PLAN" => Some(g_PAST_PLAN()),
            "PERIOD" => Some(g_PERIOD()),
            "PERSON_GROUP" => Some(g_PERSON_GROUP()),
            "PERSON_WORDS" => Some(g_PERSON_WORDS()),
            "PET_ADDED" => Some(g_PET_ADDED()),
            "PET_ALL_RIGHT" => Some(g_PET_ALL_RIGHT()),
            "PET_TAIL_COMING" => Some(g_PET_TAIL_COMING()),
            "PET_TAIL_COMING_MANY" => Some(g_PET_TAIL_COMING_MANY()),
            "PET_TAIL_MANY" => Some(g_PET_TAIL_MANY()),
            "PET_TAIL_ONE" => Some(g_PET_TAIL_ONE()),
            "PET_WORD" => Some(g_PET_WORD()),
            "PICK_FRAME" => Some(g_PICK_FRAME()),
            "PLACED_IN_THE_PAST" => Some(g_PLACED_IN_THE_PAST()),
            "PLACES_WITH_THE" => Some(g_PLACES_WITH_THE()),
            "PLAN_TURNS" => Some(g_PLAN_TURNS()),
            "PLURAL_KIN" => Some(g_PLURAL_KIN()),
            "POINTER_FILLER" => Some(g_POINTER_FILLER()),
            "POINTS_AT_ANOTHER" => Some(g_POINTS_AT_ANOTHER()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POLITE" => Some(g_POLITE()),
            "PRONOUN_POINTS_AT_ANOTHER" => Some(g_PRONOUN_POINTS_AT_ANOTHER()),
            "PUT_RIGHT_LEAD" => Some(g_PUT_RIGHT_LEAD()),
            "PUT_RIGHT_TAIL" => Some(g_PUT_RIGHT_TAIL()),
            "RAISED" => Some(g_RAISED()),
            "REACTION_PHRASES" => Some(g_REACTION_PHRASES()),
            "REACTION_WORDS" => Some(g_REACTION_WORDS()),
            "REMARK_ALREADY_PAID" => Some(g_REMARK_ALREADY_PAID()),
            "REMARK_BACK" => Some(g_REMARK_BACK()),
            "REMARK_BETTER" => Some(g_REMARK_BETTER()),
            "REMARK_EXPENSE" => Some(g_REMARK_EXPENSE()),
            "REMARK_FIGURE" => Some(g_REMARK_FIGURE()),
            "REMARK_FIGURE_REPLIES" => Some(g_REMARK_FIGURE_REPLIES()),
            "REMARK_GONE" => Some(g_REMARK_GONE()),
            "REMARK_HARD" => Some(g_REMARK_HARD()),
            "REMARK_JUDGES_THE_PRICE" => Some(g_REMARK_JUDGES_THE_PRICE()),
            "REMARK_KINDS" => Some(g_REMARK_KINDS()),
            "REMARK_LONG" => Some(g_REMARK_LONG()),
            "REMARK_PRICE_AHEAD" => Some(g_REMARK_PRICE_AHEAD()),
            "REMARK_REPLIES" => Some(g_REMARK_REPLIES()),
            "REMARK_STILL_TO_COME" => Some(g_REMARK_STILL_TO_COME()),
            "REMARK_TWO_SIDED" => Some(g_REMARK_TWO_SIDED()),
            "REMARK_WORK" => Some(g_REMARK_WORK()),
            "REPAIR_FRAME" => Some(g_REPAIR_FRAME()),
            "REPLACING" => Some(g_REPLACING()),
            "REPORTS_AN_ACTION" => Some(g_REPORTS_AN_ACTION()),
            "SAID_BACK_WORDS" => Some(g_SAID_BACK_WORDS()),
            "SELF_INTRO" => Some(g_SELF_INTRO()),
            "SEPARATION" => Some(g_SEPARATION()),
            "SHIFTS_ALL_OVER" => Some(g_SHIFTS_ALL_OVER()),
            "SHIFT_EARLY" => Some(g_SHIFT_EARLY()),
            "SHIFT_LATE" => Some(g_SHIFT_LATE()),
            "SHORT" => Some(g_SHORT()),
            "SHOWER" => Some(g_SHOWER()),
            "SINCE_TAIL" => Some(g_SINCE_TAIL()),
            "SOMEONE_COMING" => Some(g_SOMEONE_COMING()),
            "SOMEONE_ELSE_POINTS_AT_ANOTHER" => Some(g_SOMEONE_ELSE_POINTS_AT_ANOTHER()),
            "SOMEONE_GOOD" => Some(g_SOMEONE_GOOD()),
            "SOMEONE_MILESTONE" => Some(g_SOMEONE_MILESTONE()),
            "SOMEONE_MILESTONE_TAIL" => Some(g_SOMEONE_MILESTONE_TAIL()),
            "SOMEONE_TAIL" => Some(g_SOMEONE_TAIL()),
            "SOON_TAIL" => Some(g_SOON_TAIL()),
            "SORRY_TAIL" => Some(g_SORRY_TAIL()),
            "SPEAK" => Some(g_SPEAK()),
            "SPENDING_FRAMES" => Some(g_SPENDING_FRAMES()),
            "SPREAD_TAIL" => Some(g_SPREAD_TAIL()),
            "STAR_COUNT" => Some(g_STAR_COUNT()),
            "STATE" => Some(g_STATE()),
            "STEP_OPENS" => Some(g_STEP_OPENS()),
            "STILL_FRAMES" => Some(g_STILL_FRAMES()),
            "STILL_TO_DO" => Some(g_STILL_TO_DO()),
            "STOP" => Some(g_STOP()),
            "SUBJECT" => Some(g_SUBJECT()),
            "TAIL_OFF" => Some(g_TAIL_OFF()),
            "THEIRS" => Some(g_THEIRS()),
            "THE_LIMIT_GOES_FIRST" => Some(g_THE_LIMIT_GOES_FIRST()),
            "THE_NOUN_IS_AT" => Some(g_THE_NOUN_IS_AT()),
            "THE_OLD_ONE" => Some(g_THE_OLD_ONE()),
            "THE_STORY_GOES_ON" => Some(g_THE_STORY_GOES_ON()),
            "THING_BEFORE" => Some(g_THING_BEFORE()),
            "THIRD_PERSON" => Some(g_THIRD_PERSON()),
            "TIME_WORD" => Some(g_TIME_WORD()),
            "TRIP_TAIL" => Some(g_TRIP_TAIL()),
            "TRIP_VERB" => Some(g_TRIP_VERB()),
            "TROUBLE" => Some(g_TROUBLE()),
            "TWO_PLACE" => Some(g_TWO_PLACE()),
            "TWO_TO" => Some(g_TWO_TO()),
            "TWO_VERB" => Some(g_TWO_VERB()),
            "UNITS_AFTER" => Some(g_UNITS_AFTER()),
            "UNITS_OF_MEASURE" => Some(g_UNITS_OF_MEASURE()),
            "UP_CLOSED" => Some(g_UP_CLOSED()),
            "VAGUE" => Some(g_VAGUE()),
            "VALUE" => Some(g_VALUE()),
            "VALUE_UNIT_GLUED" => Some(g_VALUE_UNIT_GLUED()),
            "VALUE_UNIT_WORDS" => Some(g_VALUE_UNIT_WORDS()),
            "VALUE_WORDS" => Some(g_VALUE_WORDS()),
            "VERBISH" => Some(g_VERBISH()),
            "VISIT_TAIL" => Some(g_VISIT_TAIL()),
            "VISIT_TAIL_MANY" => Some(g_VISIT_TAIL_MANY()),
            "VISIT_TAIL_ONE" => Some(g_VISIT_TAIL_ONE()),
            "WANTS_ANSWER" => Some(g_WANTS_ANSWER()),
            "WANTS_IT" => Some(g_WANTS_IT()),
            "WHAT_OF_THEM" => Some(g_WHAT_OF_THEM()),
            "WHEN_DAYS" => Some(g_WHEN_DAYS()),
            "WHEN_DAYS_TAIL" => Some(g_WHEN_DAYS_TAIL()),
            "WHEN_DETAIL" => Some(g_WHEN_DETAIL()),
            "WHEN_ROOMY_TAIL" => Some(g_WHEN_ROOMY_TAIL()),
            "WHEN_TODAY" => Some(g_WHEN_TODAY()),
            "WHEN_TODAY_TAIL" => Some(g_WHEN_TODAY_TAIL()),
            "WHEN_TOMORROW" => Some(g_WHEN_TOMORROW()),
            "WHEN_TOMORROW_TAIL" => Some(g_WHEN_TOMORROW_TAIL()),
            "WHICH_ONE" => Some(g_WHICH_ONE()),
            "WHICH_ONES" => Some(g_WHICH_ONES()),
            "WHOLE_FREQUENCY" => Some(g_WHOLE_FREQUENCY()),
            "WHOS" => Some(g_WHOS()),
            "WHO_IS_IT" => Some(g_WHO_IS_IT()),
            "WHO_THE_FIGURE_IS" => Some(g_WHO_THE_FIGURE_IS()),
            "WISH_FOR_AN_ANIMAL" => Some(g_WISH_FOR_AN_ANIMAL()),
            "WITH_AN_ARTICLE" => Some(g_WITH_AN_ARTICLE()),
            "WITH_SLASH" => Some(g_WITH_SLASH()),
            "WORD" => Some(g_WORD()),
            "WORK_FRAMES" => Some(g_WORK_FRAMES()),
            "WORK_FROM_HOME" => Some(g_WORK_FROM_HOME()),
            "WORK_PLACE" => Some(g_WORK_PLACE()),
            "WOULD_HAVE_BEEN" => Some(g_WOULD_HAVE_BEEN()),
            "YES_AFTER" => Some(g_YES_AFTER()),
            "_ANSWER_OPENER" => Some(g__ANSWER_OPENER()),
            "_AN_ABILITY" => Some(g__AN_ABILITY()),
            "_ASKED_FOR" => Some(g__ASKED_FOR()),
            "_ASKS_MIDWAY" => Some(g__ASKS_MIDWAY()),
            "_A_PLAN_WAS_GIVEN" => Some(g__A_PLAN_WAS_GIVEN()),
            "_BULLIED" => Some(g__BULLIED()),
            "_EMPTY" => Some(g__EMPTY()),
            "_FAULT" => Some(g__FAULT()),
            "_FEELING_WORDS" => Some(g__FEELING_WORDS()),
            "_HOLE" => Some(g__HOLE()),
            "_LIMIT_ONLY" => Some(g__LIMIT_ONLY()),
            "_NOT_A_BODY_PART" => Some(g__NOT_A_BODY_PART()),
            "_NOT_A_DETAIL" => Some(g__NOT_A_DETAIL()),
            "_NOT_A_DETAIL_SUBJECT" => Some(g__NOT_A_DETAIL_SUBJECT()),
            "_NOT_WORDS" => Some(g__NOT_WORDS()),
            "_OPENS_WITH_AN_ASK" => Some(g__OPENS_WITH_AN_ASK()),
            "_OPENS_WITH_A_LINK" => Some(g__OPENS_WITH_A_LINK()),
            "_REACTION_CLAUSE" => Some(g__REACTION_CLAUSE()),
            "_SAID_BACK" => Some(g__SAID_BACK()),
            "_SAID_YES_NO" => Some(g__SAID_YES_NO()),
            "_SENTENCE" => Some(g__SENTENCE()),
            "_SOMEONE_ELSE_IN_IT" => Some(g__SOMEONE_ELSE_IN_IT()),
            "_SPACED_DASH" => Some(g__SPACED_DASH()),
            "_THE_BOT_AS_SUBJECT" => Some(g__THE_BOT_AS_SUBJECT()),
            "_WORDS" => Some(g__WORDS()),
            "_a_bare_state" => Some(py_func("_a_bare_state", |args: &[Py]| _a_bare_state(args))),
            "_a_clock_time" => Some(py_func("_a_clock_time", |args: &[Py]| _a_clock_time(args))),
            "_a_condition_they_wonder_about" => Some(py_func("_a_condition_they_wonder_about", |args: &[Py]| _a_condition_they_wonder_about(args))),
            "_a_date_on_the_thing_in_hand" => Some(py_func("_a_date_on_the_thing_in_hand", |args: &[Py]| _a_date_on_the_thing_in_hand(args))),
            "_a_day_coming" => Some(py_func("_a_day_coming", |args: &[Py]| _a_day_coming(args))),
            "_a_deadline" => Some(py_func("_a_deadline", |args: &[Py]| _a_deadline(args))),
            "_a_diary_plan" => Some(py_func("_a_diary_plan", |args: &[Py]| _a_diary_plan(args))),
            "_a_diet" => Some(py_func("_a_diet", |args: &[Py]| _a_diet(args))),
            "_a_hand_back" => Some(py_func("_a_hand_back", |args: &[Py]| _a_hand_back(args))),
            "_a_heavy_reply_just_before" => Some(py_func("_a_heavy_reply_just_before", |args: &[Py]| _a_heavy_reply_just_before(args))),
            "_a_job_to_do" => Some(py_func("_a_job_to_do", |args: &[Py]| _a_job_to_do(args))),
            "_a_like_of_one_thing" => Some(py_func("_a_like_of_one_thing", |args: &[Py]| _a_like_of_one_thing(args))),
            "_a_limit_on_the_plan" => Some(py_func("_a_limit_on_the_plan", |args: &[Py]| _a_limit_on_the_plan(args))),
            "_a_limit_reply" => Some(py_func("_a_limit_reply", |args: &[Py]| _a_limit_reply(args))),
            "_a_loss" => Some(py_func("_a_loss", |args: &[Py]| _a_loss(args))),
            "_a_loss_just_before" => Some(py_func("_a_loss_just_before", |args: &[Py]| _a_loss_just_before(args))),
            "_a_name_the_store_holds" => Some(py_func("_a_name_the_store_holds", |args: &[Py]| _a_name_the_store_holds(args))),
            "_a_pet_that_is_all_right" => Some(py_func("_a_pet_that_is_all_right", |args: &[Py]| _a_pet_that_is_all_right(args))),
            "_a_plan_on_the_table" => Some(py_func("_a_plan_on_the_table", |args: &[Py]| _a_plan_on_the_table(args))),
            "_a_price_still_to_come" => Some(py_func("_a_price_still_to_come", |args: &[Py]| _a_price_still_to_come(args))),
            "_a_relation" => Some(py_func("_a_relation", |args: &[Py]| _a_relation(args))),
            "_a_remark_on_what_was_said" => Some(py_func("_a_remark_on_what_was_said", |args: &[Py]| _a_remark_on_what_was_said(args))),
            "_a_result_is_on_the_table" => Some(py_func("_a_result_is_on_the_table", |args: &[Py]| _a_result_is_on_the_table(args))),
            "_a_spending_budget" => Some(py_func("_a_spending_budget", |args: &[Py]| _a_spending_budget(args))),
            "_a_thing_the_line_named" => Some(py_func("_a_thing_the_line_named", |args: &[Py]| _a_thing_the_line_named(args))),
            "_a_thing_they_have" => Some(py_func("_a_thing_they_have", |args: &[Py]| _a_thing_they_have(args))),
            "_a_value" => Some(py_func("_a_value", |args: &[Py]| _a_value(args))),
            "_a_visit" => Some(py_func("_a_visit", |args: &[Py]| _a_visit(args))),
            "_a_wanted_animal" => Some(py_func("_a_wanted_animal", |args: &[Py]| _a_wanted_animal(args))),
            "_a_workplace" => Some(py_func("_a_workplace", |args: &[Py]| _a_workplace(args))),
            "_about_a_body" => Some(py_func("_about_a_body", |args: &[Py]| _about_a_body(args))),
            "_about_someone_else" => Some(py_func("_about_someone_else", |args: &[Py]| _about_someone_else(args))),
            "_about_the_family" => Some(py_func("_about_the_family", |args: &[Py]| _about_the_family(args))),
            "_about_them" => Some(py_func("_about_them", |args: &[Py]| _about_them(args))),
            "_adds_a_detail" => Some(py_func("_adds_a_detail", |args: &[Py]| _adds_a_detail(args))),
            "_adds_one_more_allergen" => Some(py_func("_adds_one_more_allergen", |args: &[Py]| _adds_one_more_allergen(args))),
            "_after_a_feeling" => Some(py_func("_after_a_feeling", |args: &[Py]| _after_a_feeling(args))),
            "_after_a_reaction" => Some(py_func("_after_a_reaction", |args: &[Py]| _after_a_reaction(args))),
            "_after_the_hello" => Some(py_func("_after_the_hello", |args: &[Py]| _after_the_hello(args))),
            "_an_age_held" => Some(py_func("_an_age_held", |args: &[Py]| _an_age_held(args))),
            "_an_animal_only_planned" => Some(py_func("_an_animal_only_planned", |args: &[Py]| _an_animal_only_planned(args))),
            "_an_animal_taken_back" => Some(py_func("_an_animal_taken_back", |args: &[Py]| _an_animal_taken_back(args))),
            "_an_animal_they_had_once" => Some(py_func("_an_animal_they_had_once", |args: &[Py]| _an_animal_they_had_once(args))),
            "_an_animal_turned_down" => Some(py_func("_an_animal_turned_down", |args: &[Py]| _an_animal_turned_down(args))),
            "_an_animal_wanted" => Some(py_func("_an_animal_wanted", |args: &[Py]| _an_animal_wanted(args))),
            "_an_hour_on_the_day" => Some(py_func("_an_hour_on_the_day", |args: &[Py]| _an_hour_on_the_day(args))),
            "_another_of_what_we_said" => Some(py_func("_another_of_what_we_said", |args: &[Py]| _another_of_what_we_said(args))),
            "_asked_last" => Some(py_func("_asked_last", |args: &[Py]| _asked_last(args))),
            "_bad_news_just_before" => Some(py_func("_bad_news_just_before", |args: &[Py]| _bad_news_just_before(args))),
            "_children_said" => Some(py_func("_children_said", |args: &[Py]| _children_said(args))),
            "_children_with_ages" => Some(py_func("_children_with_ages", |args: &[Py]| _children_with_ages(args))),
            "_clock_shape" => Some(py_func("_clock_shape", |args: &[Py]| _clock_shape(args))),
            "_contrast_off" => Some(py_func("_contrast_off", |args: &[Py]| _contrast_off(args))),
            "_cut" => Some(py_func("_cut", |args: &[Py]| _cut(args))),
            "_echo" => Some(py_func("_echo", |args: &[Py]| _echo(args))),
            "_ending_of" => Some(py_func("_ending_of", |args: &[Py]| _ending_of(args))),
            "_frames" => Some(py_func("_frames", |args: &[Py]| _frames(args))),
            "_heavy" => Some(py_func("_heavy", |args: &[Py]| _heavy(args))),
            "_hours_family" => Some(py_func("_hours_family", |args: &[Py]| _hours_family(args))),
            "_how_many_people" => Some(py_func("_how_many_people", |args: &[Py]| _how_many_people(args))),
            "_is_a_contraction" => Some(py_func("_is_a_contraction", |args: &[Py]| _is_a_contraction(args))),
            "_just_a_liking" => Some(py_func("_just_a_liking", |args: &[Py]| _just_a_liking(args))),
            "_learned" => Some(py_func("_learned", |args: &[Py]| _learned(args))),
            "_more_than_one_coming" => Some(py_func("_more_than_one_coming", |args: &[Py]| _more_than_one_coming(args))),
            "_names_nothing" => Some(py_func("_names_nothing", |args: &[Py]| _names_nothing(args))),
            "_no_subject" => Some(py_func("_no_subject", |args: &[Py]| _no_subject(args))),
            "_nothing_added_but_the_pet" => Some(py_func("_nothing_added_but_the_pet", |args: &[Py]| _nothing_added_but_the_pet(args))),
            "_offers_a_chat" => Some(py_func("_offers_a_chat", |args: &[Py]| _offers_a_chat(args))),
            "_on_record" => Some(py_func("_on_record", |args: &[Py]| _on_record(args))),
            "_one_frame" => Some(py_func("_one_frame", |args: &[Py]| _one_frame(args))),
            "_only_a_dislike" => Some(py_func("_only_a_dislike", |args: &[Py]| _only_a_dislike(args))),
            "_only_a_liking" => Some(py_func("_only_a_liking", |args: &[Py]| _only_a_liking(args))),
            "_only_a_wish" => Some(py_func("_only_a_wish", |args: &[Py]| _only_a_wish(args))),
            "_only_the_correction_in_front" => Some(py_func("_only_the_correction_in_front", |args: &[Py]| _only_the_correction_in_front(args))),
            "_only_the_figure" => Some(py_func("_only_the_figure", |args: &[Py]| _only_the_figure(args))),
            "_people_in_chat" => Some(py_func("_people_in_chat", |args: &[Py]| _people_in_chat(args))),
            "_pick_ending" => Some(py_func("_pick_ending", |args: &[Py]| _pick_ending(args))),
            "_pick_off" => Some(py_func("_pick_off", |args: &[Py]| _pick_off(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_plan_tails" => Some(py_func("_plan_tails", |args: &[Py]| _plan_tails(args))),
            "_plural" => Some(py_func("_plural", |args: &[Py]| _plural(args))),
            "_reason_off" => Some(py_func("_reason_off", |args: &[Py]| _reason_off(args))),
            "_repaired" => Some(py_func("_repaired", |args: &[Py]| _repaired(args))),
            "_repairs" => Some(py_func("_repairs", |args: &[Py]| _repairs(args))),
            "_said_to_the_bot" => Some(py_func("_said_to_the_bot", |args: &[Py]| _said_to_the_bot(args))),
            "_says_something" => Some(py_func("_says_something", |args: &[Py]| _says_something(args))),
            "_shape_of" => Some(py_func("_shape_of", |args: &[Py]| _shape_of(args))),
            "_somebody_was_being_hurt" => Some(py_func("_somebody_was_being_hurt", |args: &[Py]| _somebody_was_being_hurt(args))),
            "_something_they_avoid" => Some(py_func("_something_they_avoid", |args: &[Py]| _something_they_avoid(args))),
            "_spoken" => Some(py_func("_spoken", |args: &[Py]| _spoken(args))),
            "_statement" => Some(py_func("_statement", |args: &[Py]| _statement(args))),
            "_strip_lead_in" => Some(py_func("_strip_lead_in", |args: &[Py]| _strip_lead_in(args))),
            "_tail_off" => Some(py_func("_tail_off", |args: &[Py]| _tail_off(args))),
            "_tells_it" => Some(py_func("_tells_it", |args: &[Py]| _tells_it(args))),
            "_tells_them_what_to_do" => Some(py_func("_tells_them_what_to_do", |args: &[Py]| _tells_them_what_to_do(args))),
            "_the_ages_held" => Some(py_func("_the_ages_held", |args: &[Py]| _the_ages_held(args))),
            "_the_animal_coming" => Some(py_func("_the_animal_coming", |args: &[Py]| _the_animal_coming(args))),
            "_the_animal_given_away" => Some(py_func("_the_animal_given_away", |args: &[Py]| _the_animal_given_away(args))),
            "_the_bare_value" => Some(py_func("_the_bare_value", |args: &[Py]| _the_bare_value(args))),
            "_the_chat_holds_something" => Some(py_func("_the_chat_holds_something", |args: &[Py]| _the_chat_holds_something(args))),
            "_the_entry_names" => Some(py_func("_the_entry_names", |args: &[Py]| _the_entry_names(args))),
            "_the_figure_is_an_hour" => Some(py_func("_the_figure_is_an_hour", |args: &[Py]| _the_figure_is_an_hour(args))),
            "_the_hours" => Some(py_func("_the_hours", |args: &[Py]| _the_hours(args))),
            "_the_last_reply_gave_a_plan" => Some(py_func("_the_last_reply_gave_a_plan", |args: &[Py]| _the_last_reply_gave_a_plan(args))),
            "_the_length" => Some(py_func("_the_length", |args: &[Py]| _the_length(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_milestone" => Some(py_func("_the_milestone", |args: &[Py]| _the_milestone(args))),
            "_the_opening_instead" => Some(py_func("_the_opening_instead", |args: &[Py]| _the_opening_instead(args))),
            "_the_people_pointed_at" => Some(py_func("_the_people_pointed_at", |args: &[Py]| _the_people_pointed_at(args))),
            "_the_person_pointed_at" => Some(py_func("_the_person_pointed_at", |args: &[Py]| _the_person_pointed_at(args))),
            "_the_person_the_pointer_is_about" => Some(py_func("_the_person_the_pointer_is_about", |args: &[Py]| _the_person_the_pointer_is_about(args))),
            "_the_pets_are_coming" => Some(py_func("_the_pets_are_coming", |args: &[Py]| _the_pets_are_coming(args))),
            "_the_pets_said" => Some(py_func("_the_pets_said", |args: &[Py]| _the_pets_said(args))),
            "_the_plan_a_date_belongs_to" => Some(py_func("_the_plan_a_date_belongs_to", |args: &[Py]| _the_plan_a_date_belongs_to(args))),
            "_the_plan_in_hand" => Some(py_func("_the_plan_in_hand", |args: &[Py]| _the_plan_in_hand(args))),
            "_the_pronoun" => Some(py_func("_the_pronoun", |args: &[Py]| _the_pronoun(args))),
            "_the_pronoun_points_at_somebody" => Some(py_func("_the_pronoun_points_at_somebody", |args: &[Py]| _the_pronoun_points_at_somebody(args))),
            "_the_question_about" => Some(py_func("_the_question_about", |args: &[Py]| _the_question_about(args))),
            "_the_rest_of_a_question" => Some(py_func("_the_rest_of_a_question", |args: &[Py]| _the_rest_of_a_question(args))),
            "_the_route_for" => Some(py_func("_the_route_for", |args: &[Py]| _the_route_for(args))),
            "_the_taken_back_animal" => Some(py_func("_the_taken_back_animal", |args: &[Py]| _the_taken_back_animal(args))),
            "_the_thing_the_chat_is_on" => Some(py_func("_the_thing_the_chat_is_on", |args: &[Py]| _the_thing_the_chat_is_on(args))),
            "_the_unit_before" => Some(py_func("_the_unit_before", |args: &[Py]| _the_unit_before(args))),
            "_the_unit_is_said_twice" => Some(py_func("_the_unit_is_said_twice", |args: &[Py]| _the_unit_is_said_twice(args))),
            "_the_units_measure" => Some(py_func("_the_units_measure", |args: &[Py]| _the_units_measure(args))),
            "_the_value_put_right" => Some(py_func("_the_value_put_right", |args: &[Py]| _the_value_put_right(args))),
            "_the_values_measures" => Some(py_func("_the_values_measures", |args: &[Py]| _the_values_measures(args))),
            "_the_wished_animal" => Some(py_func("_the_wished_animal", |args: &[Py]| _the_wished_animal(args))),
            "_the_words_of_the_last_two_turns" => Some(py_func("_the_words_of_the_last_two_turns", |args: &[Py]| _the_words_of_the_last_two_turns(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "_what_a_wanted_animal_needs" => Some(py_func("_what_a_wanted_animal_needs", |args: &[Py]| _what_a_wanted_animal_needs(args))),
            "_what_the_pointer_is" => Some(py_func("_what_the_pointer_is", |args: &[Py]| _what_the_pointer_is(args))),
            "_who_is_getting_better" => Some(py_func("_who_is_getting_better", |args: &[Py]| _who_is_getting_better(args))),
            "_who_the_figure_is" => Some(py_func("_who_the_figure_is", |args: &[Py]| _who_the_figure_is(args))),
            "_with_the_time" => Some(py_func("_with_the_time", |args: &[Py]| _with_the_time(args))),
            "_without_a_frame" => Some(py_func("_without_a_frame", |args: &[Py]| _without_a_frame(args))),
            "_without_a_not" => Some(py_func("_without_a_not", |args: &[Py]| _without_a_not(args))),
            "_without_a_unit_already_in_the_value" => Some(py_func("_without_a_unit_already_in_the_value", |args: &[Py]| _without_a_unit_already_in_the_value(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_job_to_do" => Some(py_func("a_job_to_do", |args: &[Py]| a_job_to_do(args))),
            "a_liked_thing" => Some(py_func("a_liked_thing", |args: &[Py]| a_liked_thing(args))),
            "a_limit_for_the_plan" => Some(py_func("a_limit_for_the_plan", |args: &[Py]| a_limit_for_the_plan(args))),
            "a_limit_reply" => Some(py_func("a_limit_reply", |args: &[Py]| a_limit_reply(args))),
            "a_person_getting_better" => Some(py_func("a_person_getting_better", |args: &[Py]| a_person_getting_better(args))),
            "a_person_named_alone" => Some(py_func("a_person_named_alone", |args: &[Py]| a_person_named_alone(args))),
            "a_person_pointed_at" => Some(py_func("a_person_pointed_at", |args: &[Py]| a_person_pointed_at(args))),
            "a_second_person" => Some(py_func("a_second_person", |args: &[Py]| a_second_person(args))),
            "carries_the_story_on" => Some(py_func("carries_the_story_on", |args: &[Py]| carries_the_story_on(args))),
            "has_news" => Some(py_func("has_news", |args: &[Py]| has_news(args))),
            "nobody_around" => Some(py_func("nobody_around", |args: &[Py]| nobody_around(args))),
            "only_a_dislike" => Some(py_func("only_a_dislike", |args: &[Py]| only_a_dislike(args))),
            "pet_given_away" => Some(py_func("pet_given_away", |args: &[Py]| pet_given_away(args))),
            "reads_as_a_receipt" => Some(py_func("reads_as_a_receipt", |args: &[Py]| reads_as_a_receipt(args))),
            "reads_as_a_trip" => Some(py_func("reads_as_a_trip", |args: &[Py]| reads_as_a_trip(args))),
            "reads_as_an_ask" => Some(py_func("reads_as_an_ask", |args: &[Py]| reads_as_an_ask(args))),
            "reads_as_cruelty" => Some(py_func("reads_as_cruelty", |args: &[Py]| reads_as_cruelty(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            "said_back" => Some(py_func("said_back", |args: &[Py]| said_back(args))),
            "the_job_they_had" => Some(py_func("the_job_they_had", |args: &[Py]| the_job_they_had(args))),
            "the_old_one_aged" => Some(py_func("the_old_one_aged", |args: &[Py]| the_old_one_aged(args))),
            "the_rest_of_a_question" => Some(g_the_rest_of_a_question()),
            "value_put_right" => Some(py_func("value_put_right", |args: &[Py]| value_put_right(args))),
            "without_the_frame" => Some(py_func("without_the_frame", |args: &[Py]| without_the_frame(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALONE_AFTER_A_LOSS" => s_ALONE_AFTER_A_LOSS(v),
            "ALONE_AFTER_A_MOVE" => s_ALONE_AFTER_A_MOVE(v),
            "ALONE_TAIL" => s_ALONE_TAIL(v),
            "ANIMAL_COMING" => s_ANIMAL_COMING(v),
            "ANIMAL_HAD_ONCE" => s_ANIMAL_HAD_ONCE(v),
            "ANIMAL_TAKEN_BACK" => s_ANIMAL_TAKEN_BACK(v),
            "ANOTHER_ONE" => s_ANOTHER_ONE(v),
            "ANOTHER_ONES" => s_ANOTHER_ONES(v),
            "ANOTHER_ONE_FOR_SOMEONE" => s_ANOTHER_ONE_FOR_SOMEONE(v),
            "ANOTHER_SOME_FOR_SOMEONE" => s_ANOTHER_SOME_FOR_SOMEONE(v),
            "ANYBODY_AT_ALL" => s_ANYBODY_AT_ALL(v),
            "AN_AGE_ALONE" => s_AN_AGE_ALONE(v),
            "AN_AIM" => s_AN_AIM(v),
            "AN_ASK" => s_AN_ASK(v),
            "ARRIVE_VERB" => s_ARRIVE_VERB(v),
            "AS_WELL" => s_AS_WELL(v),
            "A_BIRTHDAY_TOLD" => s_A_BIRTHDAY_TOLD(v),
            "A_BREED_BEING_CHOSEN" => s_A_BREED_BEING_CHOSEN(v),
            "A_CHILD_IN_THE_LINE" => s_A_CHILD_IN_THE_LINE(v),
            "A_CLAUSE" => s_A_CLAUSE(v),
            "A_CLAUSE_OF_ITS_OWN" => s_A_CLAUSE_OF_ITS_OWN(v),
            "A_CLEAR" => s_A_CLEAR(v),
            "A_COMING_WORD" => s_A_COMING_WORD(v),
            "A_CONDITION_KIND" => s_A_CONDITION_KIND(v),
            "A_CONTINUATION" => s_A_CONTINUATION(v),
            "A_CORRECTION_IN_FRONT" => s_A_CORRECTION_IN_FRONT(v),
            "A_COUNT_NOT_A_STATE" => s_A_COUNT_NOT_A_STATE(v),
            "A_COUNT_OF_THEM" => s_A_COUNT_OF_THEM(v),
            "A_DATE_ALONE" => s_A_DATE_ALONE(v),
            "A_DATE_ON_A_HARD_PLAN" => s_A_DATE_ON_A_HARD_PLAN(v),
            "A_DATE_ON_A_PLAN" => s_A_DATE_ON_A_PLAN(v),
            "A_DATE_STILL_TO_COME" => s_A_DATE_STILL_TO_COME(v),
            "A_DATE_TOLD" => s_A_DATE_TOLD(v),
            "A_DAY_ASKED" => s_A_DAY_ASKED(v),
            "A_DAY_ON_A_HARD_PLAN" => s_A_DAY_ON_A_HARD_PLAN(v),
            "A_DAY_ON_A_PLAN" => s_A_DAY_ON_A_PLAN(v),
            "A_DECISION" => s_A_DECISION(v),
            "A_DEGREE" => s_A_DEGREE(v),
            "A_DETERMINER" => s_A_DETERMINER(v),
            "A_DOING" => s_A_DOING(v),
            "A_DOING_TURNED_DOWN" => s_A_DOING_TURNED_DOWN(v),
            "A_EVENT" => s_A_EVENT(v),
            "A_FEELING" => s_A_FEELING(v),
            "A_FEELING_ABOUT_IT" => s_A_FEELING_ABOUT_IT(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_GROUP_ALONE" => s_A_GROUP_ALONE(v),
            "A_GROUP_ALONE_LINES" => s_A_GROUP_ALONE_LINES(v),
            "A_HAD_AN_ANIMAL" => s_A_HAD_AN_ANIMAL(v),
            "A_KNOWN_AGE_LINES" => s_A_KNOWN_AGE_LINES(v),
            "A_LIKE_ECHO" => s_A_LIKE_ECHO(v),
            "A_LIKE_OF_ONE_THING" => s_A_LIKE_OF_ONE_THING(v),
            "A_LIKE_WITH_TWO_HALVES" => s_A_LIKE_WITH_TWO_HALVES(v),
            "A_MARKED_HOUR" => s_A_MARKED_HOUR(v),
            "A_MEASURE" => s_A_MEASURE(v),
            "A_MILESTONE" => s_A_MILESTONE(v),
            "A_MONTH_ALONE" => s_A_MONTH_ALONE(v),
            "A_MOVE" => s_A_MOVE(v),
            "A_MOVED" => s_A_MOVED(v),
            "A_NAME_AFTER" => s_A_NAME_AFTER(v),
            "A_NAME_GIVEN" => s_A_NAME_GIVEN(v),
            "A_NAME_WAS_VIOLENT" => s_A_NAME_WAS_VIOLENT(v),
            "A_PERSON_ALONE" => s_A_PERSON_ALONE(v),
            "A_PERSON_ALONE_LINES" => s_A_PERSON_ALONE_LINES(v),
            "A_PERSON_LIKED" => s_A_PERSON_LIKED(v),
            "A_PET_WANTED" => s_A_PET_WANTED(v),
            "A_PLAN_VERB" => s_A_PLAN_VERB(v),
            "A_POINTED_AT" => s_A_POINTED_AT(v),
            "A_POSITION" => s_A_POSITION(v),
            "A_POSITION_LINES" => s_A_POSITION_LINES(v),
            "A_POSITION_MAYBE_A_THING" => s_A_POSITION_MAYBE_A_THING(v),
            "A_REASON_MARKER" => s_A_REASON_MARKER(v),
            "A_REASON_TAIL" => s_A_REASON_TAIL(v),
            "A_RELATION" => s_A_RELATION(v),
            "A_RESULT" => s_A_RESULT(v),
            "A_SECOND_PERSON" => s_A_SECOND_PERSON(v),
            "A_SOMEONE" => s_A_SOMEONE(v),
            "A_SPAN_OF_TIME" => s_A_SPAN_OF_TIME(v),
            "A_STAY_PERSON" => s_A_STAY_PERSON(v),
            "A_STAY_PLAN" => s_A_STAY_PLAN(v),
            "A_TRIP_PLAN" => s_A_TRIP_PLAN(v),
            "A_TWO_ALONE_LINES" => s_A_TWO_ALONE_LINES(v),
            "A_VAGUE_MANY" => s_A_VAGUE_MANY(v),
            "A_WANTED_ANIMAL" => s_A_WANTED_ANIMAL(v),
            "A_WANTED_ANIMAL_TAIL" => s_A_WANTED_ANIMAL_TAIL(v),
            "A_WANT_TURNED_DOWN" => s_A_WANT_TURNED_DOWN(v),
            "A_WEEKDAY" => s_A_WEEKDAY(v),
            "A_WONDERED_CONDITION" => s_A_WONDERED_CONDITION(v),
            "A_WONDER_SAID_NO" => s_A_WONDER_SAID_NO(v),
            "A_WORRIED_ABOUT_IT" => s_A_WORRIED_ABOUT_IT(v),
            "BACK_AT_IT" => s_BACK_AT_IT(v),
            "BACK_AT_IT_MINE" => s_BACK_AT_IT_MINE(v),
            "BACK_AT_IT_TAIL" => s_BACK_AT_IT_TAIL(v),
            "BEFORE_ONE" => s_BEFORE_ONE(v),
            "BETWEEN_VALUES" => s_BETWEEN_VALUES(v),
            "BLEED" => s_BLEED(v),
            "BODY" => s_BODY(v),
            "BODY_STATE_PHRASES" => s_BODY_STATE_PHRASES(v),
            "BODY_STATE_TAIL" => s_BODY_STATE_TAIL(v),
            "BODY_STATE_WORDS" => s_BODY_STATE_WORDS(v),
            "BUDGET_FRAMES" => s_BUDGET_FRAMES(v),
            "BUYING" => s_BUYING(v),
            "CHAT_OFFER_REPLY" => s_CHAT_OFFER_REPLY(v),
            "CHILD" => s_CHILD(v),
            "CLAUSES" => s_CLAUSES(v),
            "CLOCK_AFTER_TAIL" => s_CLOCK_AFTER_TAIL(v),
            "CLOCK_AT" => s_CLOCK_AT(v),
            "CLOCK_DETAIL" => s_CLOCK_DETAIL(v),
            "CLOCK_EVENT" => s_CLOCK_EVENT(v),
            "CLOCK_FLAT" => s_CLOCK_FLAT(v),
            "CLOCK_FLAT_TAIL" => s_CLOCK_FLAT_TAIL(v),
            "CLOCK_MORNING_TAIL" => s_CLOCK_MORNING_TAIL(v),
            "CLOCK_WORDS" => s_CLOCK_WORDS(v),
            "COMING_TO_STAY" => s_COMING_TO_STAY(v),
            "COMPLAINT" => s_COMPLAINT(v),
            "CONDITION_FAMILIES" => s_CONDITION_FAMILIES(v),
            "CONDITION_FOOD" => s_CONDITION_FOOD(v),
            "CONDITION_FRAMES" => s_CONDITION_FRAMES(v),
            "CONDITION_INITIALS" => s_CONDITION_INITIALS(v),
            "CONTRAST" => s_CONTRAST(v),
            "CONTRAST_SAID_AGAIN" => s_CONTRAST_SAID_AGAIN(v),
            "CORRECTED_ALONE" => s_CORRECTED_ALONE(v),
            "CORRECTED_WITH_UNIT" => s_CORRECTED_WITH_UNIT(v),
            "COUNT" => s_COUNT(v),
            "COUNTED" => s_COUNTED(v),
            "COUNTED_LETTERS" => s_COUNTED_LETTERS(v),
            "COUNTED_STARS" => s_COUNTED_STARS(v),
            "DATE_AHEAD" => s_DATE_AHEAD(v),
            "DAYS" => s_DAYS(v),
            "DAY_BEEN_AND_GONE" => s_DAY_BEEN_AND_GONE(v),
            "DAY_FAR" => s_DAY_FAR(v),
            "DAY_JUST_A_STATE" => s_DAY_JUST_A_STATE(v),
            "DAY_SOON" => s_DAY_SOON(v),
            "DEATH_WORDS" => s_DEATH_WORDS(v),
            "DEGREE_CLASS" => s_DEGREE_CLASS(v),
            "DESTINATION" => s_DESTINATION(v),
            "DESTINATION_ARRIVAL" => s_DESTINATION_ARRIVAL(v),
            "DIARY_TAIL" => s_DIARY_TAIL(v),
            "DIARY_THING" => s_DIARY_THING(v),
            "DIARY_TIME" => s_DIARY_TIME(v),
            "DIARY_TIME_TAIL" => s_DIARY_TIME_TAIL(v),
            "DIARY_WORDS" => s_DIARY_WORDS(v),
            "DIGIT" => s_DIGIT(v),
            "DREADED_OCCASION" => s_DREADED_OCCASION(v),
            "DRINK_FRAMES" => s_DRINK_FRAMES(v),
            "DURATION_UNITS" => s_DURATION_UNITS(v),
            "EARLY_RISE" => s_EARLY_RISE(v),
            "EARLY_TAIL" => s_EARLY_TAIL(v),
            "ENDS_THE_THING" => s_ENDS_THE_THING(v),
            "ENDS_THE_THING_VERB" => s_ENDS_THE_THING_VERB(v),
            "EXPLAIN_TAIL" => s_EXPLAIN_TAIL(v),
            "FACTS" => s_FACTS(v),
            "FAMILY" => s_FAMILY(v),
            "FAR_TAIL" => s_FAR_TAIL(v),
            "FEELING" => s_FEELING(v),
            "FIGURE_FILLER" => s_FIGURE_FILLER(v),
            "FIGURE_LEAD" => s_FIGURE_LEAD(v),
            "FIGURE_UNIT" => s_FIGURE_UNIT(v),
            "FIGURE_WORD" => s_FIGURE_WORD(v),
            "FIRST_WORD" => s_FIRST_WORD(v),
            "FOOD_FRAMES" => s_FOOD_FRAMES(v),
            "FOR_A_THING" => s_FOR_A_THING(v),
            "FRAGMENT" => s_FRAGMENT(v),
            "FRAMES" => s_FRAMES(v),
            "GIVEN_ANIMAL" => s_GIVEN_ANIMAL(v),
            "GIVEN_AWAY" => s_GIVEN_AWAY(v),
            "GIVEN_AWAY_REPLIES" => s_GIVEN_AWAY_REPLIES(v),
            "GOING" => s_GOING(v),
            "GOING_BAD" => s_GOING_BAD(v),
            "GOING_HOME" => s_GOING_HOME(v),
            "GOING_HOME_MINE" => s_GOING_HOME_MINE(v),
            "GOING_HOME_TAIL" => s_GOING_HOME_TAIL(v),
            "GOING_TAIL" => s_GOING_TAIL(v),
            "GOOD_MOOD" => s_GOOD_MOOD(v),
            "GREETING" => s_GREETING(v),
            "GROWING" => s_GROWING(v),
            "HABIT" => s_HABIT(v),
            "HABIT_TAIL" => s_HABIT_TAIL(v),
            "HAVE_A" => s_HAVE_A(v),
            "HAVE_GOT" => s_HAVE_GOT(v),
            "HAVE_THING" => s_HAVE_THING(v),
            "HEAVY_FRAMES" => s_HEAVY_FRAMES(v),
            "HEAVY_TURNS" => s_HEAVY_TURNS(v),
            "HOME_TAIL" => s_HOME_TAIL(v),
            "HOW_LONG_ALREADY" => s_HOW_LONG_ALREADY(v),
            "HURTS_A_PERSON" => s_HURTS_A_PERSON(v),
            "INTRODUCED_BEFORE" => s_INTRODUCED_BEFORE(v),
            "ITS_OWNS_AFTER" => s_ITS_OWNS_AFTER(v),
            "JOB_BEFORE" => s_JOB_BEFORE(v),
            "JOB_HAS_A_DAY" => s_JOB_HAS_A_DAY(v),
            "JOB_TAIL" => s_JOB_TAIL(v),
            "JOB_TO_DO" => s_JOB_TO_DO(v),
            "KIN_POSSESSIVE" => s_KIN_POSSESSIVE(v),
            "LATE_FINISH" => s_LATE_FINISH(v),
            "LATE_TAIL" => s_LATE_TAIL(v),
            "LEADERS" => s_LEADERS(v),
            "LEAD_IN" => s_LEAD_IN(v),
            "LEFT_CLOSED" => s_LEFT_CLOSED(v),
            "LEFT_OFF_CONTRACTION" => s_LEFT_OFF_CONTRACTION(v),
            "LENGTH_KEEPS" => s_LENGTH_KEEPS(v),
            "LENGTH_TAIL" => s_LENGTH_TAIL(v),
            "LETTER_COUNT" => s_LETTER_COUNT(v),
            "LIKES_FRAMES" => s_LIKES_FRAMES(v),
            "LIMIT_AFTER_THE_LINE" => s_LIMIT_AFTER_THE_LINE(v),
            "LIMIT_FRAMES" => s_LIMIT_FRAMES(v),
            "LONG_SHIFT" => s_LONG_SHIFT(v),
            "LONG_TAIL" => s_LONG_TAIL(v),
            "MAKE_A_VALUE" => s_MAKE_A_VALUE(v),
            "MEETING_TAIL" => s_MEETING_TAIL(v),
            "MEETING_TIME_TAIL" => s_MEETING_TIME_TAIL(v),
            "MEET_GREET" => s_MEET_GREET(v),
            "MONEY" => s_MONEY(v),
            "MONEY_ONLY" => s_MONEY_ONLY(v),
            "MONTHS" => s_MONTHS(v),
            "MOST" => s_MOST(v),
            "NIGHT_TAIL" => s_NIGHT_TAIL(v),
            "NOBODY_AROUND" => s_NOBODY_AROUND(v),
            "NOBODY_AROUND_ASK" => s_NOBODY_AROUND_ASK(v),
            "NOT_A_DAY_TO_LOOK_FORWARD_TO" => s_NOT_A_DAY_TO_LOOK_FORWARD_TO(v),
            "NOT_A_SECOND_OF" => s_NOT_A_SECOND_OF(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_A_THING_TO_HAVE_ANOTHER" => s_NOT_A_THING_TO_HAVE_ANOTHER(v),
            "NOT_A_TRIP" => s_NOT_A_TRIP(v),
            "NOT_A_VALUE" => s_NOT_A_VALUE(v),
            "NOT_A_VISIT" => s_NOT_A_VISIT(v),
            "NOT_ONE_THING" => s_NOT_ONE_THING(v),
            "NOT_THEIRS" => s_NOT_THEIRS(v),
            "NOT_THE_HOURS" => s_NOT_THE_HOURS(v),
            "NO_LEAD" => s_NO_LEAD(v),
            "NO_REASON" => s_NO_REASON(v),
            "NO_REFERENT" => s_NO_REFERENT(v),
            "NO_THEN_A_LINE" => s_NO_THEN_A_LINE(v),
            "NO_THEN_A_LINE_ONLY" => s_NO_THEN_A_LINE_ONLY(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "OBLIGED" => s_OBLIGED(v),
            "OCCASIONS" => s_OCCASIONS(v),
            "OCCASION_FLAT_TAIL" => s_OCCASION_FLAT_TAIL(v),
            "OCCASION_TAIL" => s_OCCASION_TAIL(v),
            "OFFERS_A_CHAT" => s_OFFERS_A_CHAT(v),
            "OLD_ONE_TAIL" => s_OLD_ONE_TAIL(v),
            "ONE_STAR" => s_ONE_STAR(v),
            "ON_NIGHTS" => s_ON_NIGHTS(v),
            "ON_THE_MEND" => s_ON_THE_MEND(v),
            "ON_THE_MEND_MINE" => s_ON_THE_MEND_MINE(v),
            "ON_THE_MEND_TAIL" => s_ON_THE_MEND_TAIL(v),
            "OTHER_LIMIT_FRAMES" => s_OTHER_LIMIT_FRAMES(v),
            "OTHER_PERSON" => s_OTHER_PERSON(v),
            "PAST_JOB" => s_PAST_JOB(v),
            "PAST_JOB_PLAIN" => s_PAST_JOB_PLAIN(v),
            "PAST_PLAN" => s_PAST_PLAN(v),
            "PERIOD" => s_PERIOD(v),
            "PERSON_GROUP" => s_PERSON_GROUP(v),
            "PERSON_WORDS" => s_PERSON_WORDS(v),
            "PET_ADDED" => s_PET_ADDED(v),
            "PET_ALL_RIGHT" => s_PET_ALL_RIGHT(v),
            "PET_TAIL_COMING" => s_PET_TAIL_COMING(v),
            "PET_TAIL_COMING_MANY" => s_PET_TAIL_COMING_MANY(v),
            "PET_TAIL_MANY" => s_PET_TAIL_MANY(v),
            "PET_TAIL_ONE" => s_PET_TAIL_ONE(v),
            "PET_WORD" => s_PET_WORD(v),
            "PICK_FRAME" => s_PICK_FRAME(v),
            "PLACED_IN_THE_PAST" => s_PLACED_IN_THE_PAST(v),
            "PLACES_WITH_THE" => s_PLACES_WITH_THE(v),
            "PLAN_TURNS" => s_PLAN_TURNS(v),
            "PLURAL_KIN" => s_PLURAL_KIN(v),
            "POINTER_FILLER" => s_POINTER_FILLER(v),
            "POINTS_AT_ANOTHER" => s_POINTS_AT_ANOTHER(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "POLITE" => s_POLITE(v),
            "PRONOUN_POINTS_AT_ANOTHER" => s_PRONOUN_POINTS_AT_ANOTHER(v),
            "PUT_RIGHT_LEAD" => s_PUT_RIGHT_LEAD(v),
            "PUT_RIGHT_TAIL" => s_PUT_RIGHT_TAIL(v),
            "RAISED" => s_RAISED(v),
            "REACTION_PHRASES" => s_REACTION_PHRASES(v),
            "REACTION_WORDS" => s_REACTION_WORDS(v),
            "REMARK_ALREADY_PAID" => s_REMARK_ALREADY_PAID(v),
            "REMARK_BACK" => s_REMARK_BACK(v),
            "REMARK_BETTER" => s_REMARK_BETTER(v),
            "REMARK_EXPENSE" => s_REMARK_EXPENSE(v),
            "REMARK_FIGURE" => s_REMARK_FIGURE(v),
            "REMARK_FIGURE_REPLIES" => s_REMARK_FIGURE_REPLIES(v),
            "REMARK_GONE" => s_REMARK_GONE(v),
            "REMARK_HARD" => s_REMARK_HARD(v),
            "REMARK_JUDGES_THE_PRICE" => s_REMARK_JUDGES_THE_PRICE(v),
            "REMARK_KINDS" => s_REMARK_KINDS(v),
            "REMARK_LONG" => s_REMARK_LONG(v),
            "REMARK_PRICE_AHEAD" => s_REMARK_PRICE_AHEAD(v),
            "REMARK_REPLIES" => s_REMARK_REPLIES(v),
            "REMARK_STILL_TO_COME" => s_REMARK_STILL_TO_COME(v),
            "REMARK_TWO_SIDED" => s_REMARK_TWO_SIDED(v),
            "REMARK_WORK" => s_REMARK_WORK(v),
            "REPAIR_FRAME" => s_REPAIR_FRAME(v),
            "REPLACING" => s_REPLACING(v),
            "REPORTS_AN_ACTION" => s_REPORTS_AN_ACTION(v),
            "SAID_BACK_WORDS" => s_SAID_BACK_WORDS(v),
            "SELF_INTRO" => s_SELF_INTRO(v),
            "SEPARATION" => s_SEPARATION(v),
            "SHIFTS_ALL_OVER" => s_SHIFTS_ALL_OVER(v),
            "SHIFT_EARLY" => s_SHIFT_EARLY(v),
            "SHIFT_LATE" => s_SHIFT_LATE(v),
            "SHORT" => s_SHORT(v),
            "SHOWER" => s_SHOWER(v),
            "SINCE_TAIL" => s_SINCE_TAIL(v),
            "SOMEONE_COMING" => s_SOMEONE_COMING(v),
            "SOMEONE_ELSE_POINTS_AT_ANOTHER" => s_SOMEONE_ELSE_POINTS_AT_ANOTHER(v),
            "SOMEONE_GOOD" => s_SOMEONE_GOOD(v),
            "SOMEONE_MILESTONE" => s_SOMEONE_MILESTONE(v),
            "SOMEONE_MILESTONE_TAIL" => s_SOMEONE_MILESTONE_TAIL(v),
            "SOMEONE_TAIL" => s_SOMEONE_TAIL(v),
            "SOON_TAIL" => s_SOON_TAIL(v),
            "SORRY_TAIL" => s_SORRY_TAIL(v),
            "SPEAK" => s_SPEAK(v),
            "SPENDING_FRAMES" => s_SPENDING_FRAMES(v),
            "SPREAD_TAIL" => s_SPREAD_TAIL(v),
            "STAR_COUNT" => s_STAR_COUNT(v),
            "STATE" => s_STATE(v),
            "STEP_OPENS" => s_STEP_OPENS(v),
            "STILL_FRAMES" => s_STILL_FRAMES(v),
            "STILL_TO_DO" => s_STILL_TO_DO(v),
            "STOP" => s_STOP(v),
            "SUBJECT" => s_SUBJECT(v),
            "TAIL_OFF" => s_TAIL_OFF(v),
            "THEIRS" => s_THEIRS(v),
            "THE_LIMIT_GOES_FIRST" => s_THE_LIMIT_GOES_FIRST(v),
            "THE_NOUN_IS_AT" => s_THE_NOUN_IS_AT(v),
            "THE_OLD_ONE" => s_THE_OLD_ONE(v),
            "THE_STORY_GOES_ON" => s_THE_STORY_GOES_ON(v),
            "THING_BEFORE" => s_THING_BEFORE(v),
            "THIRD_PERSON" => s_THIRD_PERSON(v),
            "TIME_WORD" => s_TIME_WORD(v),
            "TRIP_TAIL" => s_TRIP_TAIL(v),
            "TRIP_VERB" => s_TRIP_VERB(v),
            "TROUBLE" => s_TROUBLE(v),
            "TWO_PLACE" => s_TWO_PLACE(v),
            "TWO_TO" => s_TWO_TO(v),
            "TWO_VERB" => s_TWO_VERB(v),
            "UNITS_AFTER" => s_UNITS_AFTER(v),
            "UNITS_OF_MEASURE" => s_UNITS_OF_MEASURE(v),
            "UP_CLOSED" => s_UP_CLOSED(v),
            "VAGUE" => s_VAGUE(v),
            "VALUE" => s_VALUE(v),
            "VALUE_UNIT_GLUED" => s_VALUE_UNIT_GLUED(v),
            "VALUE_UNIT_WORDS" => s_VALUE_UNIT_WORDS(v),
            "VALUE_WORDS" => s_VALUE_WORDS(v),
            "VERBISH" => s_VERBISH(v),
            "VISIT_TAIL" => s_VISIT_TAIL(v),
            "VISIT_TAIL_MANY" => s_VISIT_TAIL_MANY(v),
            "VISIT_TAIL_ONE" => s_VISIT_TAIL_ONE(v),
            "WANTS_ANSWER" => s_WANTS_ANSWER(v),
            "WANTS_IT" => s_WANTS_IT(v),
            "WHAT_OF_THEM" => s_WHAT_OF_THEM(v),
            "WHEN_DAYS" => s_WHEN_DAYS(v),
            "WHEN_DAYS_TAIL" => s_WHEN_DAYS_TAIL(v),
            "WHEN_DETAIL" => s_WHEN_DETAIL(v),
            "WHEN_ROOMY_TAIL" => s_WHEN_ROOMY_TAIL(v),
            "WHEN_TODAY" => s_WHEN_TODAY(v),
            "WHEN_TODAY_TAIL" => s_WHEN_TODAY_TAIL(v),
            "WHEN_TOMORROW" => s_WHEN_TOMORROW(v),
            "WHEN_TOMORROW_TAIL" => s_WHEN_TOMORROW_TAIL(v),
            "WHICH_ONE" => s_WHICH_ONE(v),
            "WHICH_ONES" => s_WHICH_ONES(v),
            "WHOLE_FREQUENCY" => s_WHOLE_FREQUENCY(v),
            "WHOS" => s_WHOS(v),
            "WHO_IS_IT" => s_WHO_IS_IT(v),
            "WHO_THE_FIGURE_IS" => s_WHO_THE_FIGURE_IS(v),
            "WISH_FOR_AN_ANIMAL" => s_WISH_FOR_AN_ANIMAL(v),
            "WITH_AN_ARTICLE" => s_WITH_AN_ARTICLE(v),
            "WITH_SLASH" => s_WITH_SLASH(v),
            "WORD" => s_WORD(v),
            "WORK_FRAMES" => s_WORK_FRAMES(v),
            "WORK_FROM_HOME" => s_WORK_FROM_HOME(v),
            "WORK_PLACE" => s_WORK_PLACE(v),
            "WOULD_HAVE_BEEN" => s_WOULD_HAVE_BEEN(v),
            "YES_AFTER" => s_YES_AFTER(v),
            "_ANSWER_OPENER" => s__ANSWER_OPENER(v),
            "_AN_ABILITY" => s__AN_ABILITY(v),
            "_ASKED_FOR" => s__ASKED_FOR(v),
            "_ASKS_MIDWAY" => s__ASKS_MIDWAY(v),
            "_A_PLAN_WAS_GIVEN" => s__A_PLAN_WAS_GIVEN(v),
            "_BULLIED" => s__BULLIED(v),
            "_EMPTY" => s__EMPTY(v),
            "_FAULT" => s__FAULT(v),
            "_FEELING_WORDS" => s__FEELING_WORDS(v),
            "_HOLE" => s__HOLE(v),
            "_LIMIT_ONLY" => s__LIMIT_ONLY(v),
            "_NOT_A_BODY_PART" => s__NOT_A_BODY_PART(v),
            "_NOT_A_DETAIL" => s__NOT_A_DETAIL(v),
            "_NOT_A_DETAIL_SUBJECT" => s__NOT_A_DETAIL_SUBJECT(v),
            "_NOT_WORDS" => s__NOT_WORDS(v),
            "_OPENS_WITH_AN_ASK" => s__OPENS_WITH_AN_ASK(v),
            "_OPENS_WITH_A_LINK" => s__OPENS_WITH_A_LINK(v),
            "_REACTION_CLAUSE" => s__REACTION_CLAUSE(v),
            "_SAID_BACK" => s__SAID_BACK(v),
            "_SAID_YES_NO" => s__SAID_YES_NO(v),
            "_SENTENCE" => s__SENTENCE(v),
            "_SOMEONE_ELSE_IN_IT" => s__SOMEONE_ELSE_IN_IT(v),
            "_SPACED_DASH" => s__SPACED_DASH(v),
            "_THE_BOT_AS_SUBJECT" => s__THE_BOT_AS_SUBJECT(v),
            "_WORDS" => s__WORDS(v),
            "the_rest_of_a_question" => s_the_rest_of_a_question(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.ack", module_obj);
    pyrt::xrefs::reg_fn("memory.ack", "_a_relation", py_func("_a_relation", |args: &[Py]| _a_relation(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_pronoun", py_func("_the_pronoun", |args: &[Py]| _the_pronoun(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_ages_held", py_func("_the_ages_held", |args: &[Py]| _the_ages_held(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_people_in_chat", py_func("_people_in_chat", |args: &[Py]| _people_in_chat(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_person_named_alone", py_func("a_person_named_alone", |args: &[Py]| a_person_named_alone(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_question_about", py_func("_the_question_about", |args: &[Py]| _the_question_about(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_person_pointed_at", py_func("a_person_pointed_at", |args: &[Py]| a_person_pointed_at(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_how_many_people", py_func("_how_many_people", |args: &[Py]| _how_many_people(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_people_pointed_at", py_func("_the_people_pointed_at", |args: &[Py]| _the_people_pointed_at(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_age_held", py_func("_an_age_held", |args: &[Py]| _an_age_held(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_person_pointed_at", py_func("_the_person_pointed_at", |args: &[Py]| _the_person_pointed_at(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_second_person", py_func("a_second_person", |args: &[Py]| a_second_person(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_last_reply_gave_a_plan", py_func("_the_last_reply_gave_a_plan", |args: &[Py]| _the_last_reply_gave_a_plan(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_limit_on_the_plan", py_func("_a_limit_on_the_plan", |args: &[Py]| _a_limit_on_the_plan(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_day_coming", py_func("_a_day_coming", |args: &[Py]| _a_day_coming(args)));
    pyrt::xrefs::reg_fn("memory.ack", "reads_as_a_receipt", py_func("reads_as_a_receipt", |args: &[Py]| reads_as_a_receipt(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_one_frame", py_func("_one_frame", |args: &[Py]| _one_frame(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_more_than_one_coming", py_func("_more_than_one_coming", |args: &[Py]| _more_than_one_coming(args)));
    pyrt::xrefs::reg_fn("memory.ack", "reads_as_a_trip", py_func("reads_as_a_trip", |args: &[Py]| reads_as_a_trip(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_plan_in_hand", py_func("_the_plan_in_hand", |args: &[Py]| _the_plan_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_like_of_one_thing", py_func("_a_like_of_one_thing", |args: &[Py]| _a_like_of_one_thing(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_only_a_liking", py_func("_only_a_liking", |args: &[Py]| _only_a_liking(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_liked_thing", py_func("a_liked_thing", |args: &[Py]| a_liked_thing(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_limit_reply", py_func("a_limit_reply", |args: &[Py]| a_limit_reply(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_plan_on_the_table", py_func("_a_plan_on_the_table", |args: &[Py]| _a_plan_on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_limit_reply", py_func("_a_limit_reply", |args: &[Py]| _a_limit_reply(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_limit_for_the_plan", py_func("a_limit_for_the_plan", |args: &[Py]| a_limit_for_the_plan(args)));
    pyrt::xrefs::reg_fn("memory.ack", "reads_as_cruelty", py_func("reads_as_cruelty", |args: &[Py]| reads_as_cruelty(args)));
    pyrt::xrefs::reg_fn("memory.ack", "reads_as_an_ask", py_func("reads_as_an_ask", |args: &[Py]| reads_as_an_ask(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_shape_of", py_func("_shape_of", |args: &[Py]| _shape_of(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_clock_shape", py_func("_clock_shape", |args: &[Py]| _clock_shape(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_tells_them_what_to_do", py_func("_tells_them_what_to_do", |args: &[Py]| _tells_them_what_to_do(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_bare_state", py_func("_a_bare_state", |args: &[Py]| _a_bare_state(args)));
    pyrt::xrefs::reg_fn("memory.ack", "has_news", py_func("has_news", |args: &[Py]| has_news(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_repaired", py_func("_repaired", |args: &[Py]| _repaired(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_values_measures", py_func("_the_values_measures", |args: &[Py]| _the_values_measures(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_units_measure", py_func("_the_units_measure", |args: &[Py]| _the_units_measure(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_unit_is_said_twice", py_func("_the_unit_is_said_twice", |args: &[Py]| _the_unit_is_said_twice(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_without_a_frame", py_func("_without_a_frame", |args: &[Py]| _without_a_frame(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_without_a_not", py_func("_without_a_not", |args: &[Py]| _without_a_not(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_bare_value", py_func("_the_bare_value", |args: &[Py]| _the_bare_value(args)));
    pyrt::xrefs::reg_fn("memory.ack", "value_put_right", py_func("value_put_right", |args: &[Py]| value_put_right(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_without_a_unit_already_in_the_value", py_func("_without_a_unit_already_in_the_value", |args: &[Py]| _without_a_unit_already_in_the_value(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_unit_before", py_func("_the_unit_before", |args: &[Py]| _the_unit_before(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_animal_given_away", py_func("_the_animal_given_away", |args: &[Py]| _the_animal_given_away(args)));
    pyrt::xrefs::reg_fn("memory.ack", "pet_given_away", py_func("pet_given_away", |args: &[Py]| pet_given_away(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_offers_a_chat", py_func("_offers_a_chat", |args: &[Py]| _offers_a_chat(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_plan_a_date_belongs_to", py_func("_the_plan_a_date_belongs_to", |args: &[Py]| _the_plan_a_date_belongs_to(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_date_on_the_thing_in_hand", py_func("_a_date_on_the_thing_in_hand", |args: &[Py]| _a_date_on_the_thing_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_after_the_hello", py_func("_after_the_hello", |args: &[Py]| _after_the_hello(args)));
    pyrt::xrefs::reg_fn("memory.ack", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_opening_instead", py_func("_the_opening_instead", |args: &[Py]| _the_opening_instead(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_hand_back", py_func("_a_hand_back", |args: &[Py]| _a_hand_back(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_remark_on_what_was_said", py_func("_a_remark_on_what_was_said", |args: &[Py]| _a_remark_on_what_was_said(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_price_still_to_come", py_func("_a_price_still_to_come", |args: &[Py]| _a_price_still_to_come(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_result_is_on_the_table", py_func("_a_result_is_on_the_table", |args: &[Py]| _a_result_is_on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_ending_of", py_func("_ending_of", |args: &[Py]| _ending_of(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_pick_ending", py_func("_pick_ending", |args: &[Py]| _pick_ending(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_after_a_feeling", py_func("_after_a_feeling", |args: &[Py]| _after_a_feeling(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_heavy_reply_just_before", py_func("_a_heavy_reply_just_before", |args: &[Py]| _a_heavy_reply_just_before(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_bad_news_just_before", py_func("_bad_news_just_before", |args: &[Py]| _bad_news_just_before(args)));
    pyrt::xrefs::reg_fn("memory.ack", "carries_the_story_on", py_func("carries_the_story_on", |args: &[Py]| carries_the_story_on(args)));
    pyrt::xrefs::reg_fn("memory.ack", "the_old_one_aged", py_func("the_old_one_aged", |args: &[Py]| the_old_one_aged(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_length", py_func("_the_length", |args: &[Py]| _the_length(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_hours", py_func("_the_hours", |args: &[Py]| _the_hours(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_deadline", py_func("_a_deadline", |args: &[Py]| _a_deadline(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_hour_on_the_day", py_func("_an_hour_on_the_day", |args: &[Py]| _an_hour_on_the_day(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_figure_is_an_hour", py_func("_the_figure_is_an_hour", |args: &[Py]| _the_figure_is_an_hour(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_clock_time", py_func("_a_clock_time", |args: &[Py]| _a_clock_time(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_hours_family", py_func("_hours_family", |args: &[Py]| _hours_family(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_pet_that_is_all_right", py_func("_a_pet_that_is_all_right", |args: &[Py]| _a_pet_that_is_all_right(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_nothing_added_but_the_pet", py_func("_nothing_added_but_the_pet", |args: &[Py]| _nothing_added_but_the_pet(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_wanted_animal", py_func("_a_wanted_animal", |args: &[Py]| _a_wanted_animal(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_animal_wanted", py_func("_an_animal_wanted", |args: &[Py]| _an_animal_wanted(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_animal_they_had_once", py_func("_an_animal_they_had_once", |args: &[Py]| _an_animal_they_had_once(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_what_a_wanted_animal_needs", py_func("_what_a_wanted_animal_needs", |args: &[Py]| _what_a_wanted_animal_needs(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_pets_are_coming", py_func("_the_pets_are_coming", |args: &[Py]| _the_pets_are_coming(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_animal_coming", py_func("_the_animal_coming", |args: &[Py]| _the_animal_coming(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_taken_back_animal", py_func("_the_taken_back_animal", |args: &[Py]| _the_taken_back_animal(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_animal_turned_down", py_func("_an_animal_turned_down", |args: &[Py]| _an_animal_turned_down(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_on_record", py_func("_on_record", |args: &[Py]| _on_record(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_animal_only_planned", py_func("_an_animal_only_planned", |args: &[Py]| _an_animal_only_planned(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_wished_animal", py_func("_the_wished_animal", |args: &[Py]| _the_wished_animal(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_only_a_wish", py_func("_only_a_wish", |args: &[Py]| _only_a_wish(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_an_animal_taken_back", py_func("_an_animal_taken_back", |args: &[Py]| _an_animal_taken_back(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_pets_said", py_func("_the_pets_said", |args: &[Py]| _the_pets_said(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_about_them", py_func("_about_them", |args: &[Py]| _about_them(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_who_is_getting_better", py_func("_who_is_getting_better", |args: &[Py]| _who_is_getting_better(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_person_getting_better", py_func("a_person_getting_better", |args: &[Py]| a_person_getting_better(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_words_of_the_last_two_turns", py_func("_the_words_of_the_last_two_turns", |args: &[Py]| _the_words_of_the_last_two_turns(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_with_the_time", py_func("_with_the_time", |args: &[Py]| _with_the_time(args)));
    pyrt::xrefs::reg_fn("memory.ack", "nobody_around", py_func("nobody_around", |args: &[Py]| nobody_around(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_said_to_the_bot", py_func("_said_to_the_bot", |args: &[Py]| _said_to_the_bot(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_no_subject", py_func("_no_subject", |args: &[Py]| _no_subject(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_asked_last", py_func("_asked_last", |args: &[Py]| _asked_last(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_diary_plan", py_func("_a_diary_plan", |args: &[Py]| _a_diary_plan(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_plan_tails", py_func("_plan_tails", |args: &[Py]| _plan_tails(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_about_a_body", py_func("_about_a_body", |args: &[Py]| _about_a_body(args)));
    pyrt::xrefs::reg_fn("memory.ack", "the_job_they_had", py_func("the_job_they_had", |args: &[Py]| the_job_they_had(args)));
    pyrt::xrefs::reg_fn("memory.ack", "a_job_to_do", py_func("a_job_to_do", |args: &[Py]| a_job_to_do(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_job_to_do", py_func("_a_job_to_do", |args: &[Py]| _a_job_to_do(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_chat_holds_something", py_func("_the_chat_holds_something", |args: &[Py]| _the_chat_holds_something(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_pronoun_points_at_somebody", py_func("_the_pronoun_points_at_somebody", |args: &[Py]| _the_pronoun_points_at_somebody(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_somebody_was_being_hurt", py_func("_somebody_was_being_hurt", |args: &[Py]| _somebody_was_being_hurt(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_who_the_figure_is", py_func("_who_the_figure_is", |args: &[Py]| _who_the_figure_is(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_what_the_pointer_is", py_func("_what_the_pointer_is", |args: &[Py]| _what_the_pointer_is(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_route_for", py_func("_the_route_for", |args: &[Py]| _the_route_for(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_entry_names", py_func("_the_entry_names", |args: &[Py]| _the_entry_names(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_condition_they_wonder_about", py_func("_a_condition_they_wonder_about", |args: &[Py]| _a_condition_they_wonder_about(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_thing_they_have", py_func("_a_thing_they_have", |args: &[Py]| _a_thing_they_have(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_spending_budget", py_func("_a_spending_budget", |args: &[Py]| _a_spending_budget(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_frames", py_func("_frames", |args: &[Py]| _frames(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_about_someone_else", py_func("_about_someone_else", |args: &[Py]| _about_someone_else(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_visit", py_func("_a_visit", |args: &[Py]| _a_visit(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_milestone", py_func("_the_milestone", |args: &[Py]| _the_milestone(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_workplace", py_func("_a_workplace", |args: &[Py]| _a_workplace(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_about_the_family", py_func("_about_the_family", |args: &[Py]| _about_the_family(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_just_a_liking", py_func("_just_a_liking", |args: &[Py]| _just_a_liking(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_heavy", py_func("_heavy", |args: &[Py]| _heavy(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_statement", py_func("_statement", |args: &[Py]| _statement(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_loss", py_func("_a_loss", |args: &[Py]| _a_loss(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_learned", py_func("_learned", |args: &[Py]| _learned(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_children_with_ages", py_func("_children_with_ages", |args: &[Py]| _children_with_ages(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_children_said", py_func("_children_said", |args: &[Py]| _children_said(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_adds_one_more_allergen", py_func("_adds_one_more_allergen", |args: &[Py]| _adds_one_more_allergen(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_adds_a_detail", py_func("_adds_a_detail", |args: &[Py]| _adds_a_detail(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_names_nothing", py_func("_names_nothing", |args: &[Py]| _names_nothing(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_repairs", py_func("_repairs", |args: &[Py]| _repairs(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_says_something", py_func("_says_something", |args: &[Py]| _says_something(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_person_the_pointer_is_about", py_func("_the_person_the_pointer_is_about", |args: &[Py]| _the_person_the_pointer_is_about(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_thing_the_chat_is_on", py_func("_the_thing_the_chat_is_on", |args: &[Py]| _the_thing_the_chat_is_on(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_name_the_store_holds", py_func("_a_name_the_store_holds", |args: &[Py]| _a_name_the_store_holds(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_something_they_avoid", py_func("_something_they_avoid", |args: &[Py]| _something_they_avoid(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_plural", py_func("_plural", |args: &[Py]| _plural(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_is_a_contraction", py_func("_is_a_contraction", |args: &[Py]| _is_a_contraction(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_thing_the_line_named", py_func("_a_thing_the_line_named", |args: &[Py]| _a_thing_the_line_named(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_loss_just_before", py_func("_a_loss_just_before", |args: &[Py]| _a_loss_just_before(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_another_of_what_we_said", py_func("_another_of_what_we_said", |args: &[Py]| _another_of_what_we_said(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_only_the_correction_in_front", py_func("_only_the_correction_in_front", |args: &[Py]| _only_the_correction_in_front(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_echo", py_func("_echo", |args: &[Py]| _echo(args)));
    pyrt::xrefs::reg_fn("memory.ack", "said_back", py_func("said_back", |args: &[Py]| said_back(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_only_a_dislike", py_func("_only_a_dislike", |args: &[Py]| _only_a_dislike(args)));
    pyrt::xrefs::reg_fn("memory.ack", "only_a_dislike", py_func("only_a_dislike", |args: &[Py]| only_a_dislike(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_tells_it", py_func("_tells_it", |args: &[Py]| _tells_it(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_diet", py_func("_a_diet", |args: &[Py]| _a_diet(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_only_the_figure", py_func("_only_the_figure", |args: &[Py]| _only_the_figure(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_rest_of_a_question", py_func("_the_rest_of_a_question", |args: &[Py]| _the_rest_of_a_question(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_after_a_reaction", py_func("_after_a_reaction", |args: &[Py]| _after_a_reaction(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_strip_lead_in", py_func("_strip_lead_in", |args: &[Py]| _strip_lead_in(args)));
    pyrt::xrefs::reg_fn("memory.ack", "without_the_frame", py_func("without_the_frame", |args: &[Py]| without_the_frame(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_spoken", py_func("_spoken", |args: &[Py]| _spoken(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_pick_off", py_func("_pick_off", |args: &[Py]| _pick_off(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_tail_off", py_func("_tail_off", |args: &[Py]| _tail_off(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_contrast_off", py_func("_contrast_off", |args: &[Py]| _contrast_off(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_reason_off", py_func("_reason_off", |args: &[Py]| _reason_off(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_the_value_put_right", py_func("_the_value_put_right", |args: &[Py]| _the_value_put_right(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_a_value", py_func("_a_value", |args: &[Py]| _a_value(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_cut", py_func("_cut", |args: &[Py]| _cut(args)));
    pyrt::xrefs::reg_fn("memory.ack", "_tidy", py_func("_tidy", |args: &[Py]| _tidy(args)));
    pyrt::xrefs::reg_access("memory.ack", "ALONE_AFTER_A_LOSS", g_ALONE_AFTER_A_LOSS, s_ALONE_AFTER_A_LOSS);
    pyrt::xrefs::reg_access("memory.ack", "ALONE_AFTER_A_MOVE", g_ALONE_AFTER_A_MOVE, s_ALONE_AFTER_A_MOVE);
    pyrt::xrefs::reg_access("memory.ack", "ALONE_TAIL", g_ALONE_TAIL, s_ALONE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "ANIMAL_COMING", g_ANIMAL_COMING, s_ANIMAL_COMING);
    pyrt::xrefs::reg_access("memory.ack", "ANIMAL_HAD_ONCE", g_ANIMAL_HAD_ONCE, s_ANIMAL_HAD_ONCE);
    pyrt::xrefs::reg_access("memory.ack", "ANIMAL_TAKEN_BACK", g_ANIMAL_TAKEN_BACK, s_ANIMAL_TAKEN_BACK);
    pyrt::xrefs::reg_access("memory.ack", "ANOTHER_ONE", g_ANOTHER_ONE, s_ANOTHER_ONE);
    pyrt::xrefs::reg_access("memory.ack", "ANOTHER_ONES", g_ANOTHER_ONES, s_ANOTHER_ONES);
    pyrt::xrefs::reg_access("memory.ack", "ANOTHER_ONE_FOR_SOMEONE", g_ANOTHER_ONE_FOR_SOMEONE, s_ANOTHER_ONE_FOR_SOMEONE);
    pyrt::xrefs::reg_access("memory.ack", "ANOTHER_SOME_FOR_SOMEONE", g_ANOTHER_SOME_FOR_SOMEONE, s_ANOTHER_SOME_FOR_SOMEONE);
    pyrt::xrefs::reg_access("memory.ack", "ANYBODY_AT_ALL", g_ANYBODY_AT_ALL, s_ANYBODY_AT_ALL);
    pyrt::xrefs::reg_access("memory.ack", "AN_AGE_ALONE", g_AN_AGE_ALONE, s_AN_AGE_ALONE);
    pyrt::xrefs::reg_access("memory.ack", "AN_AIM", g_AN_AIM, s_AN_AIM);
    pyrt::xrefs::reg_access("memory.ack", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("memory.ack", "ARRIVE_VERB", g_ARRIVE_VERB, s_ARRIVE_VERB);
    pyrt::xrefs::reg_access("memory.ack", "AS_WELL", g_AS_WELL, s_AS_WELL);
    pyrt::xrefs::reg_access("memory.ack", "A_BIRTHDAY_TOLD", g_A_BIRTHDAY_TOLD, s_A_BIRTHDAY_TOLD);
    pyrt::xrefs::reg_access("memory.ack", "A_BREED_BEING_CHOSEN", g_A_BREED_BEING_CHOSEN, s_A_BREED_BEING_CHOSEN);
    pyrt::xrefs::reg_access("memory.ack", "A_CHILD_IN_THE_LINE", g_A_CHILD_IN_THE_LINE, s_A_CHILD_IN_THE_LINE);
    pyrt::xrefs::reg_access("memory.ack", "A_CLAUSE", g_A_CLAUSE, s_A_CLAUSE);
    pyrt::xrefs::reg_access("memory.ack", "A_CLAUSE_OF_ITS_OWN", g_A_CLAUSE_OF_ITS_OWN, s_A_CLAUSE_OF_ITS_OWN);
    pyrt::xrefs::reg_access("memory.ack", "A_CLEAR", g_A_CLEAR, s_A_CLEAR);
    pyrt::xrefs::reg_access("memory.ack", "A_COMING_WORD", g_A_COMING_WORD, s_A_COMING_WORD);
    pyrt::xrefs::reg_access("memory.ack", "A_CONDITION_KIND", g_A_CONDITION_KIND, s_A_CONDITION_KIND);
    pyrt::xrefs::reg_access("memory.ack", "A_CONTINUATION", g_A_CONTINUATION, s_A_CONTINUATION);
    pyrt::xrefs::reg_access("memory.ack", "A_CORRECTION_IN_FRONT", g_A_CORRECTION_IN_FRONT, s_A_CORRECTION_IN_FRONT);
    pyrt::xrefs::reg_access("memory.ack", "A_COUNT_NOT_A_STATE", g_A_COUNT_NOT_A_STATE, s_A_COUNT_NOT_A_STATE);
    pyrt::xrefs::reg_access("memory.ack", "A_COUNT_OF_THEM", g_A_COUNT_OF_THEM, s_A_COUNT_OF_THEM);
    pyrt::xrefs::reg_access("memory.ack", "A_DATE_ALONE", g_A_DATE_ALONE, s_A_DATE_ALONE);
    pyrt::xrefs::reg_access("memory.ack", "A_DATE_ON_A_HARD_PLAN", g_A_DATE_ON_A_HARD_PLAN, s_A_DATE_ON_A_HARD_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "A_DATE_ON_A_PLAN", g_A_DATE_ON_A_PLAN, s_A_DATE_ON_A_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "A_DATE_STILL_TO_COME", g_A_DATE_STILL_TO_COME, s_A_DATE_STILL_TO_COME);
    pyrt::xrefs::reg_access("memory.ack", "A_DATE_TOLD", g_A_DATE_TOLD, s_A_DATE_TOLD);
    pyrt::xrefs::reg_access("memory.ack", "A_DAY_ASKED", g_A_DAY_ASKED, s_A_DAY_ASKED);
    pyrt::xrefs::reg_access("memory.ack", "A_DAY_ON_A_HARD_PLAN", g_A_DAY_ON_A_HARD_PLAN, s_A_DAY_ON_A_HARD_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "A_DAY_ON_A_PLAN", g_A_DAY_ON_A_PLAN, s_A_DAY_ON_A_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "A_DECISION", g_A_DECISION, s_A_DECISION);
    pyrt::xrefs::reg_access("memory.ack", "A_DEGREE", g_A_DEGREE, s_A_DEGREE);
    pyrt::xrefs::reg_access("memory.ack", "A_DETERMINER", g_A_DETERMINER, s_A_DETERMINER);
    pyrt::xrefs::reg_access("memory.ack", "A_DOING", g_A_DOING, s_A_DOING);
    pyrt::xrefs::reg_access("memory.ack", "A_DOING_TURNED_DOWN", g_A_DOING_TURNED_DOWN, s_A_DOING_TURNED_DOWN);
    pyrt::xrefs::reg_access("memory.ack", "A_EVENT", g_A_EVENT, s_A_EVENT);
    pyrt::xrefs::reg_access("memory.ack", "A_FEELING", g_A_FEELING, s_A_FEELING);
    pyrt::xrefs::reg_access("memory.ack", "A_FEELING_ABOUT_IT", g_A_FEELING_ABOUT_IT, s_A_FEELING_ABOUT_IT);
    pyrt::xrefs::reg_access("memory.ack", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.ack", "A_GROUP_ALONE", g_A_GROUP_ALONE, s_A_GROUP_ALONE);
    pyrt::xrefs::reg_access("memory.ack", "A_GROUP_ALONE_LINES", g_A_GROUP_ALONE_LINES, s_A_GROUP_ALONE_LINES);
    pyrt::xrefs::reg_access("memory.ack", "A_HAD_AN_ANIMAL", g_A_HAD_AN_ANIMAL, s_A_HAD_AN_ANIMAL);
    pyrt::xrefs::reg_access("memory.ack", "A_KNOWN_AGE_LINES", g_A_KNOWN_AGE_LINES, s_A_KNOWN_AGE_LINES);
    pyrt::xrefs::reg_access("memory.ack", "A_LIKE_ECHO", g_A_LIKE_ECHO, s_A_LIKE_ECHO);
    pyrt::xrefs::reg_access("memory.ack", "A_LIKE_OF_ONE_THING", g_A_LIKE_OF_ONE_THING, s_A_LIKE_OF_ONE_THING);
    pyrt::xrefs::reg_access("memory.ack", "A_LIKE_WITH_TWO_HALVES", g_A_LIKE_WITH_TWO_HALVES, s_A_LIKE_WITH_TWO_HALVES);
    pyrt::xrefs::reg_access("memory.ack", "A_MARKED_HOUR", g_A_MARKED_HOUR, s_A_MARKED_HOUR);
    pyrt::xrefs::reg_access("memory.ack", "A_MEASURE", g_A_MEASURE, s_A_MEASURE);
    pyrt::xrefs::reg_access("memory.ack", "A_MILESTONE", g_A_MILESTONE, s_A_MILESTONE);
    pyrt::xrefs::reg_access("memory.ack", "A_MONTH_ALONE", g_A_MONTH_ALONE, s_A_MONTH_ALONE);
    pyrt::xrefs::reg_access("memory.ack", "A_MOVE", g_A_MOVE, s_A_MOVE);
    pyrt::xrefs::reg_access("memory.ack", "A_MOVED", g_A_MOVED, s_A_MOVED);
    pyrt::xrefs::reg_access("memory.ack", "A_NAME_AFTER", g_A_NAME_AFTER, s_A_NAME_AFTER);
    pyrt::xrefs::reg_access("memory.ack", "A_NAME_GIVEN", g_A_NAME_GIVEN, s_A_NAME_GIVEN);
    pyrt::xrefs::reg_access("memory.ack", "A_NAME_WAS_VIOLENT", g_A_NAME_WAS_VIOLENT, s_A_NAME_WAS_VIOLENT);
    pyrt::xrefs::reg_access("memory.ack", "A_PERSON_ALONE", g_A_PERSON_ALONE, s_A_PERSON_ALONE);
    pyrt::xrefs::reg_access("memory.ack", "A_PERSON_ALONE_LINES", g_A_PERSON_ALONE_LINES, s_A_PERSON_ALONE_LINES);
    pyrt::xrefs::reg_access("memory.ack", "A_PERSON_LIKED", g_A_PERSON_LIKED, s_A_PERSON_LIKED);
    pyrt::xrefs::reg_access("memory.ack", "A_PET_WANTED", g_A_PET_WANTED, s_A_PET_WANTED);
    pyrt::xrefs::reg_access("memory.ack", "A_PLAN_VERB", g_A_PLAN_VERB, s_A_PLAN_VERB);
    pyrt::xrefs::reg_access("memory.ack", "A_POINTED_AT", g_A_POINTED_AT, s_A_POINTED_AT);
    pyrt::xrefs::reg_access("memory.ack", "A_POSITION", g_A_POSITION, s_A_POSITION);
    pyrt::xrefs::reg_access("memory.ack", "A_POSITION_LINES", g_A_POSITION_LINES, s_A_POSITION_LINES);
    pyrt::xrefs::reg_access("memory.ack", "A_POSITION_MAYBE_A_THING", g_A_POSITION_MAYBE_A_THING, s_A_POSITION_MAYBE_A_THING);
    pyrt::xrefs::reg_access("memory.ack", "A_REASON_MARKER", g_A_REASON_MARKER, s_A_REASON_MARKER);
    pyrt::xrefs::reg_access("memory.ack", "A_REASON_TAIL", g_A_REASON_TAIL, s_A_REASON_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "A_RELATION", g_A_RELATION, s_A_RELATION);
    pyrt::xrefs::reg_access("memory.ack", "A_RESULT", g_A_RESULT, s_A_RESULT);
    pyrt::xrefs::reg_access("memory.ack", "A_SECOND_PERSON", g_A_SECOND_PERSON, s_A_SECOND_PERSON);
    pyrt::xrefs::reg_access("memory.ack", "A_SOMEONE", g_A_SOMEONE, s_A_SOMEONE);
    pyrt::xrefs::reg_access("memory.ack", "A_SPAN_OF_TIME", g_A_SPAN_OF_TIME, s_A_SPAN_OF_TIME);
    pyrt::xrefs::reg_access("memory.ack", "A_STAY_PERSON", g_A_STAY_PERSON, s_A_STAY_PERSON);
    pyrt::xrefs::reg_access("memory.ack", "A_STAY_PLAN", g_A_STAY_PLAN, s_A_STAY_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "A_TRIP_PLAN", g_A_TRIP_PLAN, s_A_TRIP_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "A_TWO_ALONE_LINES", g_A_TWO_ALONE_LINES, s_A_TWO_ALONE_LINES);
    pyrt::xrefs::reg_access("memory.ack", "A_VAGUE_MANY", g_A_VAGUE_MANY, s_A_VAGUE_MANY);
    pyrt::xrefs::reg_access("memory.ack", "A_WANTED_ANIMAL", g_A_WANTED_ANIMAL, s_A_WANTED_ANIMAL);
    pyrt::xrefs::reg_access("memory.ack", "A_WANTED_ANIMAL_TAIL", g_A_WANTED_ANIMAL_TAIL, s_A_WANTED_ANIMAL_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "A_WANT_TURNED_DOWN", g_A_WANT_TURNED_DOWN, s_A_WANT_TURNED_DOWN);
    pyrt::xrefs::reg_access("memory.ack", "A_WEEKDAY", g_A_WEEKDAY, s_A_WEEKDAY);
    pyrt::xrefs::reg_access("memory.ack", "A_WONDERED_CONDITION", g_A_WONDERED_CONDITION, s_A_WONDERED_CONDITION);
    pyrt::xrefs::reg_access("memory.ack", "A_WONDER_SAID_NO", g_A_WONDER_SAID_NO, s_A_WONDER_SAID_NO);
    pyrt::xrefs::reg_access("memory.ack", "A_WORRIED_ABOUT_IT", g_A_WORRIED_ABOUT_IT, s_A_WORRIED_ABOUT_IT);
    pyrt::xrefs::reg_access("memory.ack", "BACK_AT_IT", g_BACK_AT_IT, s_BACK_AT_IT);
    pyrt::xrefs::reg_access("memory.ack", "BACK_AT_IT_MINE", g_BACK_AT_IT_MINE, s_BACK_AT_IT_MINE);
    pyrt::xrefs::reg_access("memory.ack", "BACK_AT_IT_TAIL", g_BACK_AT_IT_TAIL, s_BACK_AT_IT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "BEFORE_ONE", g_BEFORE_ONE, s_BEFORE_ONE);
    pyrt::xrefs::reg_access("memory.ack", "BETWEEN_VALUES", g_BETWEEN_VALUES, s_BETWEEN_VALUES);
    pyrt::xrefs::reg_access("memory.ack", "BLEED", g_BLEED, s_BLEED);
    pyrt::xrefs::reg_access("memory.ack", "BODY", g_BODY, s_BODY);
    pyrt::xrefs::reg_access("memory.ack", "BODY_STATE_PHRASES", g_BODY_STATE_PHRASES, s_BODY_STATE_PHRASES);
    pyrt::xrefs::reg_access("memory.ack", "BODY_STATE_TAIL", g_BODY_STATE_TAIL, s_BODY_STATE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "BODY_STATE_WORDS", g_BODY_STATE_WORDS, s_BODY_STATE_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "BUDGET_FRAMES", g_BUDGET_FRAMES, s_BUDGET_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "BUYING", g_BUYING, s_BUYING);
    pyrt::xrefs::reg_access("memory.ack", "CHAT_OFFER_REPLY", g_CHAT_OFFER_REPLY, s_CHAT_OFFER_REPLY);
    pyrt::xrefs::reg_access("memory.ack", "CHILD", g_CHILD, s_CHILD);
    pyrt::xrefs::reg_access("memory.ack", "CLAUSES", g_CLAUSES, s_CLAUSES);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_AFTER_TAIL", g_CLOCK_AFTER_TAIL, s_CLOCK_AFTER_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_AT", g_CLOCK_AT, s_CLOCK_AT);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_DETAIL", g_CLOCK_DETAIL, s_CLOCK_DETAIL);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_EVENT", g_CLOCK_EVENT, s_CLOCK_EVENT);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_FLAT", g_CLOCK_FLAT, s_CLOCK_FLAT);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_FLAT_TAIL", g_CLOCK_FLAT_TAIL, s_CLOCK_FLAT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_MORNING_TAIL", g_CLOCK_MORNING_TAIL, s_CLOCK_MORNING_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "CLOCK_WORDS", g_CLOCK_WORDS, s_CLOCK_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "COMING_TO_STAY", g_COMING_TO_STAY, s_COMING_TO_STAY);
    pyrt::xrefs::reg_access("memory.ack", "COMPLAINT", g_COMPLAINT, s_COMPLAINT);
    pyrt::xrefs::reg_access("memory.ack", "CONDITION_FAMILIES", g_CONDITION_FAMILIES, s_CONDITION_FAMILIES);
    pyrt::xrefs::reg_access("memory.ack", "CONDITION_FOOD", g_CONDITION_FOOD, s_CONDITION_FOOD);
    pyrt::xrefs::reg_access("memory.ack", "CONDITION_FRAMES", g_CONDITION_FRAMES, s_CONDITION_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "CONDITION_INITIALS", g_CONDITION_INITIALS, s_CONDITION_INITIALS);
    pyrt::xrefs::reg_access("memory.ack", "CONTRAST", g_CONTRAST, s_CONTRAST);
    pyrt::xrefs::reg_access("memory.ack", "CONTRAST_SAID_AGAIN", g_CONTRAST_SAID_AGAIN, s_CONTRAST_SAID_AGAIN);
    pyrt::xrefs::reg_access("memory.ack", "CORRECTED_ALONE", g_CORRECTED_ALONE, s_CORRECTED_ALONE);
    pyrt::xrefs::reg_access("memory.ack", "CORRECTED_WITH_UNIT", g_CORRECTED_WITH_UNIT, s_CORRECTED_WITH_UNIT);
    pyrt::xrefs::reg_access("memory.ack", "COUNT", g_COUNT, s_COUNT);
    pyrt::xrefs::reg_access("memory.ack", "COUNTED", g_COUNTED, s_COUNTED);
    pyrt::xrefs::reg_access("memory.ack", "COUNTED_LETTERS", g_COUNTED_LETTERS, s_COUNTED_LETTERS);
    pyrt::xrefs::reg_access("memory.ack", "COUNTED_STARS", g_COUNTED_STARS, s_COUNTED_STARS);
    pyrt::xrefs::reg_access("memory.ack", "DATE_AHEAD", g_DATE_AHEAD, s_DATE_AHEAD);
    pyrt::xrefs::reg_access("memory.ack", "DAYS", g_DAYS, s_DAYS);
    pyrt::xrefs::reg_access("memory.ack", "DAY_BEEN_AND_GONE", g_DAY_BEEN_AND_GONE, s_DAY_BEEN_AND_GONE);
    pyrt::xrefs::reg_access("memory.ack", "DAY_FAR", g_DAY_FAR, s_DAY_FAR);
    pyrt::xrefs::reg_access("memory.ack", "DAY_JUST_A_STATE", g_DAY_JUST_A_STATE, s_DAY_JUST_A_STATE);
    pyrt::xrefs::reg_access("memory.ack", "DAY_SOON", g_DAY_SOON, s_DAY_SOON);
    pyrt::xrefs::reg_access("memory.ack", "DEATH_WORDS", g_DEATH_WORDS, s_DEATH_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "DEGREE_CLASS", g_DEGREE_CLASS, s_DEGREE_CLASS);
    pyrt::xrefs::reg_access("memory.ack", "DESTINATION", g_DESTINATION, s_DESTINATION);
    pyrt::xrefs::reg_access("memory.ack", "DESTINATION_ARRIVAL", g_DESTINATION_ARRIVAL, s_DESTINATION_ARRIVAL);
    pyrt::xrefs::reg_access("memory.ack", "DIARY_TAIL", g_DIARY_TAIL, s_DIARY_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "DIARY_THING", g_DIARY_THING, s_DIARY_THING);
    pyrt::xrefs::reg_access("memory.ack", "DIARY_TIME", g_DIARY_TIME, s_DIARY_TIME);
    pyrt::xrefs::reg_access("memory.ack", "DIARY_TIME_TAIL", g_DIARY_TIME_TAIL, s_DIARY_TIME_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "DIARY_WORDS", g_DIARY_WORDS, s_DIARY_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "DIGIT", g_DIGIT, s_DIGIT);
    pyrt::xrefs::reg_access("memory.ack", "DREADED_OCCASION", g_DREADED_OCCASION, s_DREADED_OCCASION);
    pyrt::xrefs::reg_access("memory.ack", "DRINK_FRAMES", g_DRINK_FRAMES, s_DRINK_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "DURATION_UNITS", g_DURATION_UNITS, s_DURATION_UNITS);
    pyrt::xrefs::reg_access("memory.ack", "EARLY_RISE", g_EARLY_RISE, s_EARLY_RISE);
    pyrt::xrefs::reg_access("memory.ack", "EARLY_TAIL", g_EARLY_TAIL, s_EARLY_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "ENDS_THE_THING", g_ENDS_THE_THING, s_ENDS_THE_THING);
    pyrt::xrefs::reg_access("memory.ack", "ENDS_THE_THING_VERB", g_ENDS_THE_THING_VERB, s_ENDS_THE_THING_VERB);
    pyrt::xrefs::reg_access("memory.ack", "EXPLAIN_TAIL", g_EXPLAIN_TAIL, s_EXPLAIN_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "FACTS", g_FACTS, s_FACTS);
    pyrt::xrefs::reg_access("memory.ack", "FAMILY", g_FAMILY, s_FAMILY);
    pyrt::xrefs::reg_access("memory.ack", "FAR_TAIL", g_FAR_TAIL, s_FAR_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "FEELING", g_FEELING, s_FEELING);
    pyrt::xrefs::reg_access("memory.ack", "FIGURE_FILLER", g_FIGURE_FILLER, s_FIGURE_FILLER);
    pyrt::xrefs::reg_access("memory.ack", "FIGURE_LEAD", g_FIGURE_LEAD, s_FIGURE_LEAD);
    pyrt::xrefs::reg_access("memory.ack", "FIGURE_UNIT", g_FIGURE_UNIT, s_FIGURE_UNIT);
    pyrt::xrefs::reg_access("memory.ack", "FIGURE_WORD", g_FIGURE_WORD, s_FIGURE_WORD);
    pyrt::xrefs::reg_access("memory.ack", "FIRST_WORD", g_FIRST_WORD, s_FIRST_WORD);
    pyrt::xrefs::reg_access("memory.ack", "FOOD_FRAMES", g_FOOD_FRAMES, s_FOOD_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "FOR_A_THING", g_FOR_A_THING, s_FOR_A_THING);
    pyrt::xrefs::reg_access("memory.ack", "FRAGMENT", g_FRAGMENT, s_FRAGMENT);
    pyrt::xrefs::reg_access("memory.ack", "FRAMES", g_FRAMES, s_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "GIVEN_ANIMAL", g_GIVEN_ANIMAL, s_GIVEN_ANIMAL);
    pyrt::xrefs::reg_access("memory.ack", "GIVEN_AWAY", g_GIVEN_AWAY, s_GIVEN_AWAY);
    pyrt::xrefs::reg_access("memory.ack", "GIVEN_AWAY_REPLIES", g_GIVEN_AWAY_REPLIES, s_GIVEN_AWAY_REPLIES);
    pyrt::xrefs::reg_access("memory.ack", "GOING", g_GOING, s_GOING);
    pyrt::xrefs::reg_access("memory.ack", "GOING_BAD", g_GOING_BAD, s_GOING_BAD);
    pyrt::xrefs::reg_access("memory.ack", "GOING_HOME", g_GOING_HOME, s_GOING_HOME);
    pyrt::xrefs::reg_access("memory.ack", "GOING_HOME_MINE", g_GOING_HOME_MINE, s_GOING_HOME_MINE);
    pyrt::xrefs::reg_access("memory.ack", "GOING_HOME_TAIL", g_GOING_HOME_TAIL, s_GOING_HOME_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "GOING_TAIL", g_GOING_TAIL, s_GOING_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "GOOD_MOOD", g_GOOD_MOOD, s_GOOD_MOOD);
    pyrt::xrefs::reg_access("memory.ack", "GREETING", g_GREETING, s_GREETING);
    pyrt::xrefs::reg_access("memory.ack", "GROWING", g_GROWING, s_GROWING);
    pyrt::xrefs::reg_access("memory.ack", "HABIT", g_HABIT, s_HABIT);
    pyrt::xrefs::reg_access("memory.ack", "HABIT_TAIL", g_HABIT_TAIL, s_HABIT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "HAVE_A", g_HAVE_A, s_HAVE_A);
    pyrt::xrefs::reg_access("memory.ack", "HAVE_GOT", g_HAVE_GOT, s_HAVE_GOT);
    pyrt::xrefs::reg_access("memory.ack", "HAVE_THING", g_HAVE_THING, s_HAVE_THING);
    pyrt::xrefs::reg_access("memory.ack", "HEAVY_FRAMES", g_HEAVY_FRAMES, s_HEAVY_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "HEAVY_TURNS", g_HEAVY_TURNS, s_HEAVY_TURNS);
    pyrt::xrefs::reg_access("memory.ack", "HOME_TAIL", g_HOME_TAIL, s_HOME_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "HOW_LONG_ALREADY", g_HOW_LONG_ALREADY, s_HOW_LONG_ALREADY);
    pyrt::xrefs::reg_access("memory.ack", "HURTS_A_PERSON", g_HURTS_A_PERSON, s_HURTS_A_PERSON);
    pyrt::xrefs::reg_access("memory.ack", "INTRODUCED_BEFORE", g_INTRODUCED_BEFORE, s_INTRODUCED_BEFORE);
    pyrt::xrefs::reg_access("memory.ack", "ITS_OWNS_AFTER", g_ITS_OWNS_AFTER, s_ITS_OWNS_AFTER);
    pyrt::xrefs::reg_access("memory.ack", "JOB_BEFORE", g_JOB_BEFORE, s_JOB_BEFORE);
    pyrt::xrefs::reg_access("memory.ack", "JOB_HAS_A_DAY", g_JOB_HAS_A_DAY, s_JOB_HAS_A_DAY);
    pyrt::xrefs::reg_access("memory.ack", "JOB_TAIL", g_JOB_TAIL, s_JOB_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "JOB_TO_DO", g_JOB_TO_DO, s_JOB_TO_DO);
    pyrt::xrefs::reg_access("memory.ack", "KIN_POSSESSIVE", g_KIN_POSSESSIVE, s_KIN_POSSESSIVE);
    pyrt::xrefs::reg_access("memory.ack", "LATE_FINISH", g_LATE_FINISH, s_LATE_FINISH);
    pyrt::xrefs::reg_access("memory.ack", "LATE_TAIL", g_LATE_TAIL, s_LATE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "LEADERS", g_LEADERS, s_LEADERS);
    pyrt::xrefs::reg_access("memory.ack", "LEAD_IN", g_LEAD_IN, s_LEAD_IN);
    pyrt::xrefs::reg_access("memory.ack", "LEFT_CLOSED", g_LEFT_CLOSED, s_LEFT_CLOSED);
    pyrt::xrefs::reg_access("memory.ack", "LEFT_OFF_CONTRACTION", g_LEFT_OFF_CONTRACTION, s_LEFT_OFF_CONTRACTION);
    pyrt::xrefs::reg_access("memory.ack", "LENGTH_KEEPS", g_LENGTH_KEEPS, s_LENGTH_KEEPS);
    pyrt::xrefs::reg_access("memory.ack", "LENGTH_TAIL", g_LENGTH_TAIL, s_LENGTH_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "LETTER_COUNT", g_LETTER_COUNT, s_LETTER_COUNT);
    pyrt::xrefs::reg_access("memory.ack", "LIKES_FRAMES", g_LIKES_FRAMES, s_LIKES_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "LIMIT_AFTER_THE_LINE", g_LIMIT_AFTER_THE_LINE, s_LIMIT_AFTER_THE_LINE);
    pyrt::xrefs::reg_access("memory.ack", "LIMIT_FRAMES", g_LIMIT_FRAMES, s_LIMIT_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "LONG_SHIFT", g_LONG_SHIFT, s_LONG_SHIFT);
    pyrt::xrefs::reg_access("memory.ack", "LONG_TAIL", g_LONG_TAIL, s_LONG_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "MAKE_A_VALUE", g_MAKE_A_VALUE, s_MAKE_A_VALUE);
    pyrt::xrefs::reg_access("memory.ack", "MEETING_TAIL", g_MEETING_TAIL, s_MEETING_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "MEETING_TIME_TAIL", g_MEETING_TIME_TAIL, s_MEETING_TIME_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "MEET_GREET", g_MEET_GREET, s_MEET_GREET);
    pyrt::xrefs::reg_access("memory.ack", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("memory.ack", "MONEY_ONLY", g_MONEY_ONLY, s_MONEY_ONLY);
    pyrt::xrefs::reg_access("memory.ack", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory.ack", "MOST", g_MOST, s_MOST);
    pyrt::xrefs::reg_access("memory.ack", "NIGHT_TAIL", g_NIGHT_TAIL, s_NIGHT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "NOBODY_AROUND", g_NOBODY_AROUND, s_NOBODY_AROUND);
    pyrt::xrefs::reg_access("memory.ack", "NOBODY_AROUND_ASK", g_NOBODY_AROUND_ASK, s_NOBODY_AROUND_ASK);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_DAY_TO_LOOK_FORWARD_TO", g_NOT_A_DAY_TO_LOOK_FORWARD_TO, s_NOT_A_DAY_TO_LOOK_FORWARD_TO);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_SECOND_OF", g_NOT_A_SECOND_OF, s_NOT_A_SECOND_OF);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_THING_TO_HAVE_ANOTHER", g_NOT_A_THING_TO_HAVE_ANOTHER, s_NOT_A_THING_TO_HAVE_ANOTHER);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_TRIP", g_NOT_A_TRIP, s_NOT_A_TRIP);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_VALUE", g_NOT_A_VALUE, s_NOT_A_VALUE);
    pyrt::xrefs::reg_access("memory.ack", "NOT_A_VISIT", g_NOT_A_VISIT, s_NOT_A_VISIT);
    pyrt::xrefs::reg_access("memory.ack", "NOT_ONE_THING", g_NOT_ONE_THING, s_NOT_ONE_THING);
    pyrt::xrefs::reg_access("memory.ack", "NOT_THEIRS", g_NOT_THEIRS, s_NOT_THEIRS);
    pyrt::xrefs::reg_access("memory.ack", "NOT_THE_HOURS", g_NOT_THE_HOURS, s_NOT_THE_HOURS);
    pyrt::xrefs::reg_access("memory.ack", "NO_LEAD", g_NO_LEAD, s_NO_LEAD);
    pyrt::xrefs::reg_access("memory.ack", "NO_REASON", g_NO_REASON, s_NO_REASON);
    pyrt::xrefs::reg_access("memory.ack", "NO_REFERENT", g_NO_REFERENT, s_NO_REFERENT);
    pyrt::xrefs::reg_access("memory.ack", "NO_THEN_A_LINE", g_NO_THEN_A_LINE, s_NO_THEN_A_LINE);
    pyrt::xrefs::reg_access("memory.ack", "NO_THEN_A_LINE_ONLY", g_NO_THEN_A_LINE_ONLY, s_NO_THEN_A_LINE_ONLY);
    pyrt::xrefs::reg_access("memory.ack", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "OBLIGED", g_OBLIGED, s_OBLIGED);
    pyrt::xrefs::reg_access("memory.ack", "OCCASIONS", g_OCCASIONS, s_OCCASIONS);
    pyrt::xrefs::reg_access("memory.ack", "OCCASION_FLAT_TAIL", g_OCCASION_FLAT_TAIL, s_OCCASION_FLAT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "OCCASION_TAIL", g_OCCASION_TAIL, s_OCCASION_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "OFFERS_A_CHAT", g_OFFERS_A_CHAT, s_OFFERS_A_CHAT);
    pyrt::xrefs::reg_access("memory.ack", "OLD_ONE_TAIL", g_OLD_ONE_TAIL, s_OLD_ONE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "ONE_STAR", g_ONE_STAR, s_ONE_STAR);
    pyrt::xrefs::reg_access("memory.ack", "ON_NIGHTS", g_ON_NIGHTS, s_ON_NIGHTS);
    pyrt::xrefs::reg_access("memory.ack", "ON_THE_MEND", g_ON_THE_MEND, s_ON_THE_MEND);
    pyrt::xrefs::reg_access("memory.ack", "ON_THE_MEND_MINE", g_ON_THE_MEND_MINE, s_ON_THE_MEND_MINE);
    pyrt::xrefs::reg_access("memory.ack", "ON_THE_MEND_TAIL", g_ON_THE_MEND_TAIL, s_ON_THE_MEND_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "OTHER_LIMIT_FRAMES", g_OTHER_LIMIT_FRAMES, s_OTHER_LIMIT_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "OTHER_PERSON", g_OTHER_PERSON, s_OTHER_PERSON);
    pyrt::xrefs::reg_access("memory.ack", "PAST_JOB", g_PAST_JOB, s_PAST_JOB);
    pyrt::xrefs::reg_access("memory.ack", "PAST_JOB_PLAIN", g_PAST_JOB_PLAIN, s_PAST_JOB_PLAIN);
    pyrt::xrefs::reg_access("memory.ack", "PAST_PLAN", g_PAST_PLAN, s_PAST_PLAN);
    pyrt::xrefs::reg_access("memory.ack", "PERIOD", g_PERIOD, s_PERIOD);
    pyrt::xrefs::reg_access("memory.ack", "PERSON_GROUP", g_PERSON_GROUP, s_PERSON_GROUP);
    pyrt::xrefs::reg_access("memory.ack", "PERSON_WORDS", g_PERSON_WORDS, s_PERSON_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "PET_ADDED", g_PET_ADDED, s_PET_ADDED);
    pyrt::xrefs::reg_access("memory.ack", "PET_ALL_RIGHT", g_PET_ALL_RIGHT, s_PET_ALL_RIGHT);
    pyrt::xrefs::reg_access("memory.ack", "PET_TAIL_COMING", g_PET_TAIL_COMING, s_PET_TAIL_COMING);
    pyrt::xrefs::reg_access("memory.ack", "PET_TAIL_COMING_MANY", g_PET_TAIL_COMING_MANY, s_PET_TAIL_COMING_MANY);
    pyrt::xrefs::reg_access("memory.ack", "PET_TAIL_MANY", g_PET_TAIL_MANY, s_PET_TAIL_MANY);
    pyrt::xrefs::reg_access("memory.ack", "PET_TAIL_ONE", g_PET_TAIL_ONE, s_PET_TAIL_ONE);
    pyrt::xrefs::reg_access("memory.ack", "PET_WORD", g_PET_WORD, s_PET_WORD);
    pyrt::xrefs::reg_access("memory.ack", "PICK_FRAME", g_PICK_FRAME, s_PICK_FRAME);
    pyrt::xrefs::reg_access("memory.ack", "PLACED_IN_THE_PAST", g_PLACED_IN_THE_PAST, s_PLACED_IN_THE_PAST);
    pyrt::xrefs::reg_access("memory.ack", "PLACES_WITH_THE", g_PLACES_WITH_THE, s_PLACES_WITH_THE);
    pyrt::xrefs::reg_access("memory.ack", "PLAN_TURNS", g_PLAN_TURNS, s_PLAN_TURNS);
    pyrt::xrefs::reg_access("memory.ack", "PLURAL_KIN", g_PLURAL_KIN, s_PLURAL_KIN);
    pyrt::xrefs::reg_access("memory.ack", "POINTER_FILLER", g_POINTER_FILLER, s_POINTER_FILLER);
    pyrt::xrefs::reg_access("memory.ack", "POINTS_AT_ANOTHER", g_POINTS_AT_ANOTHER, s_POINTS_AT_ANOTHER);
    pyrt::xrefs::reg_access("memory.ack", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.ack", "POLITE", g_POLITE, s_POLITE);
    pyrt::xrefs::reg_access("memory.ack", "PRONOUN_POINTS_AT_ANOTHER", g_PRONOUN_POINTS_AT_ANOTHER, s_PRONOUN_POINTS_AT_ANOTHER);
    pyrt::xrefs::reg_access("memory.ack", "PUT_RIGHT_LEAD", g_PUT_RIGHT_LEAD, s_PUT_RIGHT_LEAD);
    pyrt::xrefs::reg_access("memory.ack", "PUT_RIGHT_TAIL", g_PUT_RIGHT_TAIL, s_PUT_RIGHT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "RAISED", g_RAISED, s_RAISED);
    pyrt::xrefs::reg_access("memory.ack", "REACTION_PHRASES", g_REACTION_PHRASES, s_REACTION_PHRASES);
    pyrt::xrefs::reg_access("memory.ack", "REACTION_WORDS", g_REACTION_WORDS, s_REACTION_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_ALREADY_PAID", g_REMARK_ALREADY_PAID, s_REMARK_ALREADY_PAID);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_BACK", g_REMARK_BACK, s_REMARK_BACK);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_BETTER", g_REMARK_BETTER, s_REMARK_BETTER);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_EXPENSE", g_REMARK_EXPENSE, s_REMARK_EXPENSE);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_FIGURE", g_REMARK_FIGURE, s_REMARK_FIGURE);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_FIGURE_REPLIES", g_REMARK_FIGURE_REPLIES, s_REMARK_FIGURE_REPLIES);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_GONE", g_REMARK_GONE, s_REMARK_GONE);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_HARD", g_REMARK_HARD, s_REMARK_HARD);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_JUDGES_THE_PRICE", g_REMARK_JUDGES_THE_PRICE, s_REMARK_JUDGES_THE_PRICE);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_KINDS", g_REMARK_KINDS, s_REMARK_KINDS);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_LONG", g_REMARK_LONG, s_REMARK_LONG);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_PRICE_AHEAD", g_REMARK_PRICE_AHEAD, s_REMARK_PRICE_AHEAD);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_REPLIES", g_REMARK_REPLIES, s_REMARK_REPLIES);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_STILL_TO_COME", g_REMARK_STILL_TO_COME, s_REMARK_STILL_TO_COME);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_TWO_SIDED", g_REMARK_TWO_SIDED, s_REMARK_TWO_SIDED);
    pyrt::xrefs::reg_access("memory.ack", "REMARK_WORK", g_REMARK_WORK, s_REMARK_WORK);
    pyrt::xrefs::reg_access("memory.ack", "REPAIR_FRAME", g_REPAIR_FRAME, s_REPAIR_FRAME);
    pyrt::xrefs::reg_access("memory.ack", "REPLACING", g_REPLACING, s_REPLACING);
    pyrt::xrefs::reg_access("memory.ack", "REPORTS_AN_ACTION", g_REPORTS_AN_ACTION, s_REPORTS_AN_ACTION);
    pyrt::xrefs::reg_access("memory.ack", "SAID_BACK_WORDS", g_SAID_BACK_WORDS, s_SAID_BACK_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "SELF_INTRO", g_SELF_INTRO, s_SELF_INTRO);
    pyrt::xrefs::reg_access("memory.ack", "SEPARATION", g_SEPARATION, s_SEPARATION);
    pyrt::xrefs::reg_access("memory.ack", "SHIFTS_ALL_OVER", g_SHIFTS_ALL_OVER, s_SHIFTS_ALL_OVER);
    pyrt::xrefs::reg_access("memory.ack", "SHIFT_EARLY", g_SHIFT_EARLY, s_SHIFT_EARLY);
    pyrt::xrefs::reg_access("memory.ack", "SHIFT_LATE", g_SHIFT_LATE, s_SHIFT_LATE);
    pyrt::xrefs::reg_access("memory.ack", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("memory.ack", "SHOWER", g_SHOWER, s_SHOWER);
    pyrt::xrefs::reg_access("memory.ack", "SINCE_TAIL", g_SINCE_TAIL, s_SINCE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "SOMEONE_COMING", g_SOMEONE_COMING, s_SOMEONE_COMING);
    pyrt::xrefs::reg_access("memory.ack", "SOMEONE_ELSE_POINTS_AT_ANOTHER", g_SOMEONE_ELSE_POINTS_AT_ANOTHER, s_SOMEONE_ELSE_POINTS_AT_ANOTHER);
    pyrt::xrefs::reg_access("memory.ack", "SOMEONE_GOOD", g_SOMEONE_GOOD, s_SOMEONE_GOOD);
    pyrt::xrefs::reg_access("memory.ack", "SOMEONE_MILESTONE", g_SOMEONE_MILESTONE, s_SOMEONE_MILESTONE);
    pyrt::xrefs::reg_access("memory.ack", "SOMEONE_MILESTONE_TAIL", g_SOMEONE_MILESTONE_TAIL, s_SOMEONE_MILESTONE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "SOMEONE_TAIL", g_SOMEONE_TAIL, s_SOMEONE_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "SOON_TAIL", g_SOON_TAIL, s_SOON_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "SORRY_TAIL", g_SORRY_TAIL, s_SORRY_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "SPEAK", g_SPEAK, s_SPEAK);
    pyrt::xrefs::reg_access("memory.ack", "SPENDING_FRAMES", g_SPENDING_FRAMES, s_SPENDING_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "SPREAD_TAIL", g_SPREAD_TAIL, s_SPREAD_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "STAR_COUNT", g_STAR_COUNT, s_STAR_COUNT);
    pyrt::xrefs::reg_access("memory.ack", "STATE", g_STATE, s_STATE);
    pyrt::xrefs::reg_access("memory.ack", "STEP_OPENS", g_STEP_OPENS, s_STEP_OPENS);
    pyrt::xrefs::reg_access("memory.ack", "STILL_FRAMES", g_STILL_FRAMES, s_STILL_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "STILL_TO_DO", g_STILL_TO_DO, s_STILL_TO_DO);
    pyrt::xrefs::reg_access("memory.ack", "STOP", g_STOP, s_STOP);
    pyrt::xrefs::reg_access("memory.ack", "SUBJECT", g_SUBJECT, s_SUBJECT);
    pyrt::xrefs::reg_access("memory.ack", "TAIL_OFF", g_TAIL_OFF, s_TAIL_OFF);
    pyrt::xrefs::reg_access("memory.ack", "THEIRS", g_THEIRS, s_THEIRS);
    pyrt::xrefs::reg_access("memory.ack", "THE_LIMIT_GOES_FIRST", g_THE_LIMIT_GOES_FIRST, s_THE_LIMIT_GOES_FIRST);
    pyrt::xrefs::reg_access("memory.ack", "THE_NOUN_IS_AT", g_THE_NOUN_IS_AT, s_THE_NOUN_IS_AT);
    pyrt::xrefs::reg_access("memory.ack", "THE_OLD_ONE", g_THE_OLD_ONE, s_THE_OLD_ONE);
    pyrt::xrefs::reg_access("memory.ack", "THE_STORY_GOES_ON", g_THE_STORY_GOES_ON, s_THE_STORY_GOES_ON);
    pyrt::xrefs::reg_access("memory.ack", "THING_BEFORE", g_THING_BEFORE, s_THING_BEFORE);
    pyrt::xrefs::reg_access("memory.ack", "THIRD_PERSON", g_THIRD_PERSON, s_THIRD_PERSON);
    pyrt::xrefs::reg_access("memory.ack", "TIME_WORD", g_TIME_WORD, s_TIME_WORD);
    pyrt::xrefs::reg_access("memory.ack", "TRIP_TAIL", g_TRIP_TAIL, s_TRIP_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "TRIP_VERB", g_TRIP_VERB, s_TRIP_VERB);
    pyrt::xrefs::reg_access("memory.ack", "TROUBLE", g_TROUBLE, s_TROUBLE);
    pyrt::xrefs::reg_access("memory.ack", "TWO_PLACE", g_TWO_PLACE, s_TWO_PLACE);
    pyrt::xrefs::reg_access("memory.ack", "TWO_TO", g_TWO_TO, s_TWO_TO);
    pyrt::xrefs::reg_access("memory.ack", "TWO_VERB", g_TWO_VERB, s_TWO_VERB);
    pyrt::xrefs::reg_access("memory.ack", "UNITS_AFTER", g_UNITS_AFTER, s_UNITS_AFTER);
    pyrt::xrefs::reg_access("memory.ack", "UNITS_OF_MEASURE", g_UNITS_OF_MEASURE, s_UNITS_OF_MEASURE);
    pyrt::xrefs::reg_access("memory.ack", "UP_CLOSED", g_UP_CLOSED, s_UP_CLOSED);
    pyrt::xrefs::reg_access("memory.ack", "VAGUE", g_VAGUE, s_VAGUE);
    pyrt::xrefs::reg_access("memory.ack", "VALUE", g_VALUE, s_VALUE);
    pyrt::xrefs::reg_access("memory.ack", "VALUE_UNIT_GLUED", g_VALUE_UNIT_GLUED, s_VALUE_UNIT_GLUED);
    pyrt::xrefs::reg_access("memory.ack", "VALUE_UNIT_WORDS", g_VALUE_UNIT_WORDS, s_VALUE_UNIT_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "VALUE_WORDS", g_VALUE_WORDS, s_VALUE_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "VERBISH", g_VERBISH, s_VERBISH);
    pyrt::xrefs::reg_access("memory.ack", "VISIT_TAIL", g_VISIT_TAIL, s_VISIT_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "VISIT_TAIL_MANY", g_VISIT_TAIL_MANY, s_VISIT_TAIL_MANY);
    pyrt::xrefs::reg_access("memory.ack", "VISIT_TAIL_ONE", g_VISIT_TAIL_ONE, s_VISIT_TAIL_ONE);
    pyrt::xrefs::reg_access("memory.ack", "WANTS_ANSWER", g_WANTS_ANSWER, s_WANTS_ANSWER);
    pyrt::xrefs::reg_access("memory.ack", "WANTS_IT", g_WANTS_IT, s_WANTS_IT);
    pyrt::xrefs::reg_access("memory.ack", "WHAT_OF_THEM", g_WHAT_OF_THEM, s_WHAT_OF_THEM);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_DAYS", g_WHEN_DAYS, s_WHEN_DAYS);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_DAYS_TAIL", g_WHEN_DAYS_TAIL, s_WHEN_DAYS_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_DETAIL", g_WHEN_DETAIL, s_WHEN_DETAIL);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_ROOMY_TAIL", g_WHEN_ROOMY_TAIL, s_WHEN_ROOMY_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_TODAY", g_WHEN_TODAY, s_WHEN_TODAY);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_TODAY_TAIL", g_WHEN_TODAY_TAIL, s_WHEN_TODAY_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_TOMORROW", g_WHEN_TOMORROW, s_WHEN_TOMORROW);
    pyrt::xrefs::reg_access("memory.ack", "WHEN_TOMORROW_TAIL", g_WHEN_TOMORROW_TAIL, s_WHEN_TOMORROW_TAIL);
    pyrt::xrefs::reg_access("memory.ack", "WHICH_ONE", g_WHICH_ONE, s_WHICH_ONE);
    pyrt::xrefs::reg_access("memory.ack", "WHICH_ONES", g_WHICH_ONES, s_WHICH_ONES);
    pyrt::xrefs::reg_access("memory.ack", "WHOLE_FREQUENCY", g_WHOLE_FREQUENCY, s_WHOLE_FREQUENCY);
    pyrt::xrefs::reg_access("memory.ack", "WHOS", g_WHOS, s_WHOS);
    pyrt::xrefs::reg_access("memory.ack", "WHO_IS_IT", g_WHO_IS_IT, s_WHO_IS_IT);
    pyrt::xrefs::reg_access("memory.ack", "WHO_THE_FIGURE_IS", g_WHO_THE_FIGURE_IS, s_WHO_THE_FIGURE_IS);
    pyrt::xrefs::reg_access("memory.ack", "WISH_FOR_AN_ANIMAL", g_WISH_FOR_AN_ANIMAL, s_WISH_FOR_AN_ANIMAL);
    pyrt::xrefs::reg_access("memory.ack", "WITH_AN_ARTICLE", g_WITH_AN_ARTICLE, s_WITH_AN_ARTICLE);
    pyrt::xrefs::reg_access("memory.ack", "WITH_SLASH", g_WITH_SLASH, s_WITH_SLASH);
    pyrt::xrefs::reg_access("memory.ack", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("memory.ack", "WORK_FRAMES", g_WORK_FRAMES, s_WORK_FRAMES);
    pyrt::xrefs::reg_access("memory.ack", "WORK_FROM_HOME", g_WORK_FROM_HOME, s_WORK_FROM_HOME);
    pyrt::xrefs::reg_access("memory.ack", "WORK_PLACE", g_WORK_PLACE, s_WORK_PLACE);
    pyrt::xrefs::reg_access("memory.ack", "WOULD_HAVE_BEEN", g_WOULD_HAVE_BEEN, s_WOULD_HAVE_BEEN);
    pyrt::xrefs::reg_access("memory.ack", "YES_AFTER", g_YES_AFTER, s_YES_AFTER);
    pyrt::xrefs::reg_access("memory.ack", "_ANSWER_OPENER", g__ANSWER_OPENER, s__ANSWER_OPENER);
    pyrt::xrefs::reg_access("memory.ack", "_AN_ABILITY", g__AN_ABILITY, s__AN_ABILITY);
    pyrt::xrefs::reg_access("memory.ack", "_ASKED_FOR", g__ASKED_FOR, s__ASKED_FOR);
    pyrt::xrefs::reg_access("memory.ack", "_ASKS_MIDWAY", g__ASKS_MIDWAY, s__ASKS_MIDWAY);
    pyrt::xrefs::reg_access("memory.ack", "_A_PLAN_WAS_GIVEN", g__A_PLAN_WAS_GIVEN, s__A_PLAN_WAS_GIVEN);
    pyrt::xrefs::reg_access("memory.ack", "_BULLIED", g__BULLIED, s__BULLIED);
    pyrt::xrefs::reg_access("memory.ack", "_EMPTY", g__EMPTY, s__EMPTY);
    pyrt::xrefs::reg_access("memory.ack", "_FAULT", g__FAULT, s__FAULT);
    pyrt::xrefs::reg_access("memory.ack", "_FEELING_WORDS", g__FEELING_WORDS, s__FEELING_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "_HOLE", g__HOLE, s__HOLE);
    pyrt::xrefs::reg_access("memory.ack", "_LIMIT_ONLY", g__LIMIT_ONLY, s__LIMIT_ONLY);
    pyrt::xrefs::reg_access("memory.ack", "_NOT_A_BODY_PART", g__NOT_A_BODY_PART, s__NOT_A_BODY_PART);
    pyrt::xrefs::reg_access("memory.ack", "_NOT_A_DETAIL", g__NOT_A_DETAIL, s__NOT_A_DETAIL);
    pyrt::xrefs::reg_access("memory.ack", "_NOT_A_DETAIL_SUBJECT", g__NOT_A_DETAIL_SUBJECT, s__NOT_A_DETAIL_SUBJECT);
    pyrt::xrefs::reg_access("memory.ack", "_NOT_WORDS", g__NOT_WORDS, s__NOT_WORDS);
    pyrt::xrefs::reg_access("memory.ack", "_OPENS_WITH_AN_ASK", g__OPENS_WITH_AN_ASK, s__OPENS_WITH_AN_ASK);
    pyrt::xrefs::reg_access("memory.ack", "_OPENS_WITH_A_LINK", g__OPENS_WITH_A_LINK, s__OPENS_WITH_A_LINK);
    pyrt::xrefs::reg_access("memory.ack", "_REACTION_CLAUSE", g__REACTION_CLAUSE, s__REACTION_CLAUSE);
    pyrt::xrefs::reg_access("memory.ack", "_SAID_BACK", g__SAID_BACK, s__SAID_BACK);
    pyrt::xrefs::reg_access("memory.ack", "_SAID_YES_NO", g__SAID_YES_NO, s__SAID_YES_NO);
    pyrt::xrefs::reg_access("memory.ack", "_SENTENCE", g__SENTENCE, s__SENTENCE);
    pyrt::xrefs::reg_access("memory.ack", "_SOMEONE_ELSE_IN_IT", g__SOMEONE_ELSE_IN_IT, s__SOMEONE_ELSE_IN_IT);
    pyrt::xrefs::reg_access("memory.ack", "_SPACED_DASH", g__SPACED_DASH, s__SPACED_DASH);
    pyrt::xrefs::reg_access("memory.ack", "_THE_BOT_AS_SUBJECT", g__THE_BOT_AS_SUBJECT, s__THE_BOT_AS_SUBJECT);
    pyrt::xrefs::reg_access("memory.ack", "_WORDS", g__WORDS, s__WORDS);
    pyrt::xrefs::reg_access("memory.ack", "the_rest_of_a_question", g_the_rest_of_a_question, s_the_rest_of_a_question);
}
