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


thread_local! { static G_AHEAD_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALLERGENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPLIANCE_IDEAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ATE_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAKE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_TEMPERATURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_EGGS_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_EGGS_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAKE_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLOUR_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPARE_IDEAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOK_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROWD_PUDDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROWD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUPBOARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAIRY_FREE_VEGGIE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFER_TO_ALLERGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DELIBERATING_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESSERT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_MEAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_RECALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_TOO_MEDICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DINNER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_BARE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_WHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_WHAT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_WHAT_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_WHAT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISLIKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISLIKE_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISLIKE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_VERB_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUGH_STICK_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATING_REFUSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATING_REFUSALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EGG_DISH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EGG_FREE_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EGG_IS_THE_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_TAIL_OK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRUIT_PUDDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC_SWAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOES_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOES_WITH_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUEST_DIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUEST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDEAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDEA_CARRIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDEA_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPLIED_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_QUALITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIDS_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KITCHEN_SWAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LUNCH_MEALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAKE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAKING_A_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MIN_TYPO_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NICE_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_KEEPING_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_APPLIANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUT_FREE_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDER_AT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDER_AT_A_PLACE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OR_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_OF_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVEN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIRING_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIRING_LEAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIRING_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAIRING_RICH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_BACKREF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_REST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_NAMES_LONGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROBLEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUDDINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUDDING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_READY_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_READY_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REWARDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RICH_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAFETY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_DISLIKE_OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORTAGE_MARKERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIDES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIDE_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIDE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAGE_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTED_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_START_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBSTITUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_IN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_IN_A_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAY_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_UP_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGAN_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGAN_SWAP_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGAN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGGIE_CARRIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGGIE_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGGIE_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGGIE_SWAP_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_FAULTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEG_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WASTE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_NO_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLERGEN_NOTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANY_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED_ABOUT_THE_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_NOISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MADE_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHOICE_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLEANING_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COOK_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DIET_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRY_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GENERIC_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GLUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRAINY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRAINY_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IDEA_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INGREDIENT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_BUT_COOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LUMPY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LUMPY_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSHY_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSHY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMES_A_SAUCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PICK_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAN_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAN_THAT_MIGHT_BE_RUINED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECIPE_CARDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RED_MEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RULE_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RULE_OUT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SINK_AS_A_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SINK_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STANDS_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STARCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOUGH_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_OVEN_BEEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_OVEN_CAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_OVEN_GAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_OVEN_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_OVEN_SPONGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__cake_eggs: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__cake_plan: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__engine_reads: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__names: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__one_edit: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__slug: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__what: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_not_a_mark_on_a_thing: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_not_a_present: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_tokens: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.dishes: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AHEAD_IDEA() -> Py { ensure_init(); G_AHEAD_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AHEAD_IDEA(v: Py) { ensure_init(); G_AHEAD_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_ALLERGENS() -> Py { ensure_init(); G_ALLERGENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALLERGENS(v: Py) { ensure_init(); G_ALLERGENS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_APPLIANCE_IDEAS() -> Py { ensure_init(); G_APPLIANCE_IDEAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPLIANCE_IDEAS(v: Py) { ensure_init(); G_APPLIANCE_IDEAS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FILLER() -> Py { ensure_init(); G_ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FILLER(v: Py) { ensure_init(); G_ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_KINDS() -> Py { ensure_init(); G_ASK_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_KINDS(v: Py) { ensure_init(); G_ASK_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OPENERS() -> Py { ensure_init(); G_ASK_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENERS(v: Py) { ensure_init(); G_ASK_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_PHRASES() -> Py { ensure_init(); G_ASK_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_PHRASES(v: Py) { ensure_init(); G_ASK_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_ATE_ALREADY() -> Py { ensure_init(); G_ATE_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATE_ALREADY(v: Py) { ensure_init(); G_ATE_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD() -> Py { ensure_init(); G_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD(v: Py) { ensure_init(); G_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_BAKE_WORDS() -> Py { ensure_init(); G_BAKE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAKE_WORDS(v: Py) { ensure_init(); G_BAKE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_TEMPERATURE() -> Py { ensure_init(); G_BODY_TEMPERATURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_TEMPERATURE(v: Py) { ensure_init(); G_BODY_TEMPERATURE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_AVOID() -> Py { ensure_init(); G_CAKE_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_AVOID(v: Py) { ensure_init(); G_CAKE_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_EGGS_AVOID() -> Py { ensure_init(); G_CAKE_EGGS_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_EGGS_AVOID(v: Py) { ensure_init(); G_CAKE_EGGS_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_EGGS_KEY() -> Py { ensure_init(); G_CAKE_EGGS_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_EGGS_KEY(v: Py) { ensure_init(); G_CAKE_EGGS_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_CAKE_KEY() -> Py { ensure_init(); G_CAKE_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAKE_KEY(v: Py) { ensure_init(); G_CAKE_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP_IDEA() -> Py { ensure_init(); G_CHEAP_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP_IDEA(v: Py) { ensure_init(); G_CHEAP_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_COLOUR_CHOICE() -> Py { ensure_init(); G_COLOUR_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLOUR_CHOICE(v: Py) { ensure_init(); G_COLOUR_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPARE_IDEAS() -> Py { ensure_init(); G_COMPARE_IDEAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPARE_IDEAS(v: Py) { ensure_init(); G_COMPARE_IDEAS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_VERB() -> Py { ensure_init(); G_COOKING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_VERB(v: Py) { ensure_init(); G_COOKING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_WORDS() -> Py { ensure_init(); G_COOKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_WORDS(v: Py) { ensure_init(); G_COOKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOK_ASK() -> Py { ensure_init(); G_COOK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOK_ASK(v: Py) { ensure_init(); G_COOK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_COOK_ASK_WORDS() -> Py { ensure_init(); G_COOK_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOK_ASK_WORDS(v: Py) { ensure_init(); G_COOK_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CROWD_PUDDING() -> Py { ensure_init(); G_CROWD_PUDDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROWD_PUDDING(v: Py) { ensure_init(); G_CROWD_PUDDING.with(|c| *c.borrow_mut() = v); }
pub fn g_CROWD_WORDS() -> Py { ensure_init(); G_CROWD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROWD_WORDS(v: Py) { ensure_init(); G_CROWD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CUPBOARD() -> Py { ensure_init(); G_CUPBOARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUPBOARD(v: Py) { ensure_init(); G_CUPBOARD.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_OFF() -> Py { ensure_init(); G_CUT_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_OFF(v: Py) { ensure_init(); G_CUT_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_WORDS() -> Py { ensure_init(); G_CUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_WORDS(v: Py) { ensure_init(); G_CUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAIRY_FREE_VEGGIE() -> Py { ensure_init(); G_DAIRY_FREE_VEGGIE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAIRY_FREE_VEGGIE(v: Py) { ensure_init(); G_DAIRY_FREE_VEGGIE.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_WORDS() -> Py { ensure_init(); G_DAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_WORDS(v: Py) { ensure_init(); G_DAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFER_TO_ALLERGY() -> Py { ensure_init(); G_DEFER_TO_ALLERGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFER_TO_ALLERGY(v: Py) { ensure_init(); G_DEFER_TO_ALLERGY.with(|c| *c.borrow_mut() = v); }
pub fn g_DELIBERATING_START() -> Py { ensure_init(); G_DELIBERATING_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DELIBERATING_START(v: Py) { ensure_init(); G_DELIBERATING_START.with(|c| *c.borrow_mut() = v); }
pub fn g_DESSERT_WORDS() -> Py { ensure_init(); G_DESSERT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESSERT_WORDS(v: Py) { ensure_init(); G_DESSERT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIETS() -> Py { ensure_init(); G_DIETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIETS(v: Py) { ensure_init(); G_DIETS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_FAMILIES() -> Py { ensure_init(); G_DIET_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_FAMILIES(v: Py) { ensure_init(); G_DIET_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_MEAL_ASK() -> Py { ensure_init(); G_DIET_MEAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_MEAL_ASK(v: Py) { ensure_init(); G_DIET_MEAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_NOT_THIS() -> Py { ensure_init(); G_DIET_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_NOT_THIS(v: Py) { ensure_init(); G_DIET_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_RECALL() -> Py { ensure_init(); G_DIET_RECALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_RECALL(v: Py) { ensure_init(); G_DIET_RECALL.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_TOO_MEDICAL() -> Py { ensure_init(); G_DIET_TOO_MEDICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_TOO_MEDICAL(v: Py) { ensure_init(); G_DIET_TOO_MEDICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_DINNER_ASK() -> Py { ensure_init(); G_DINNER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DINNER_ASK(v: Py) { ensure_init(); G_DINNER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_BARE_NAMES() -> Py { ensure_init(); G_DISH_BARE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_BARE_NAMES(v: Py) { ensure_init(); G_DISH_BARE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_LEAD() -> Py { ensure_init(); G_DISH_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_LEAD(v: Py) { ensure_init(); G_DISH_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_WHAT() -> Py { ensure_init(); G_DISH_WHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WHAT(v: Py) { ensure_init(); G_DISH_WHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_WHAT_ASKS() -> Py { ensure_init(); G_DISH_WHAT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WHAT_ASKS(v: Py) { ensure_init(); G_DISH_WHAT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_WHAT_AVOID() -> Py { ensure_init(); G_DISH_WHAT_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WHAT_AVOID(v: Py) { ensure_init(); G_DISH_WHAT_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_WHAT_LEAD() -> Py { ensure_init(); G_DISH_WHAT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WHAT_LEAD(v: Py) { ensure_init(); G_DISH_WHAT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_WORDS() -> Py { ensure_init(); G_DISH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WORDS(v: Py) { ensure_init(); G_DISH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISLIKED() -> Py { ensure_init(); G_DISLIKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISLIKED(v: Py) { ensure_init(); G_DISLIKED.with(|c| *c.borrow_mut() = v); }
pub fn g_DISLIKE_SHAPES() -> Py { ensure_init(); G_DISLIKE_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISLIKE_SHAPES(v: Py) { ensure_init(); G_DISLIKE_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_DISLIKE_VERBS() -> Py { ensure_init(); G_DISLIKE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISLIKE_VERBS(v: Py) { ensure_init(); G_DISLIKE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_ASK() -> Py { ensure_init(); G_DONE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_ASK(v: Py) { ensure_init(); G_DONE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_POSSESSIVE() -> Py { ensure_init(); G_DONE_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_POSSESSIVE(v: Py) { ensure_init(); G_DONE_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_VERB_ASK() -> Py { ensure_init(); G_DONE_VERB_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_VERB_ASK(v: Py) { ensure_init(); G_DONE_VERB_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_WORD() -> Py { ensure_init(); G_DONE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_WORD(v: Py) { ensure_init(); G_DONE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUGH_STICK_LEAD() -> Py { ensure_init(); G_DOUGH_STICK_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUGH_STICK_LEAD(v: Py) { ensure_init(); G_DOUGH_STICK_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_EATING_REFUSAL() -> Py { ensure_init(); G_EATING_REFUSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATING_REFUSAL(v: Py) { ensure_init(); G_EATING_REFUSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_EATING_REFUSALS() -> Py { ensure_init(); G_EATING_REFUSALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATING_REFUSALS(v: Py) { ensure_init(); G_EATING_REFUSALS.with(|c| *c.borrow_mut() = v); }
pub fn g_EGG_DISH_WORDS() -> Py { ensure_init(); G_EGG_DISH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EGG_DISH_WORDS(v: Py) { ensure_init(); G_EGG_DISH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EGG_FREE_BAKE() -> Py { ensure_init(); G_EGG_FREE_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EGG_FREE_BAKE(v: Py) { ensure_init(); G_EGG_FREE_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_EGG_IS_THE_DISH() -> Py { ensure_init(); G_EGG_IS_THE_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EGG_IS_THE_DISH(v: Py) { ensure_init(); G_EGG_IS_THE_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_TAIL_OK() -> Py { ensure_init(); G_FOOD_TAIL_OK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_TAIL_OK(v: Py) { ensure_init(); G_FOOD_TAIL_OK.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_WORDS() -> Py { ensure_init(); G_FOOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_WORDS(v: Py) { ensure_init(); G_FOOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FRUIT_PUDDING() -> Py { ensure_init(); G_FRUIT_PUDDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRUIT_PUDDING(v: Py) { ensure_init(); G_FRUIT_PUDDING.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC_SWAP() -> Py { ensure_init(); G_GENERIC_SWAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC_SWAP(v: Py) { ensure_init(); G_GENERIC_SWAP.with(|c| *c.borrow_mut() = v); }
pub fn g_GOES_WITH() -> Py { ensure_init(); G_GOES_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOES_WITH(v: Py) { ensure_init(); G_GOES_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_GOES_WITH_SHAPE() -> Py { ensure_init(); G_GOES_WITH_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOES_WITH_SHAPE(v: Py) { ensure_init(); G_GOES_WITH_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_GUEST_DIET() -> Py { ensure_init(); G_GUEST_DIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUEST_DIET(v: Py) { ensure_init(); G_GUEST_DIET.with(|c| *c.borrow_mut() = v); }
pub fn g_GUEST_WORDS() -> Py { ensure_init(); G_GUEST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUEST_WORDS(v: Py) { ensure_init(); G_GUEST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_WORDS() -> Py { ensure_init(); G_HAVE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_WORDS(v: Py) { ensure_init(); G_HAVE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_WORDS() -> Py { ensure_init(); G_HOW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_WORDS(v: Py) { ensure_init(); G_HOW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_IDEAS() -> Py { ensure_init(); G_IDEAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDEAS(v: Py) { ensure_init(); G_IDEAS.with(|c| *c.borrow_mut() = v); }
pub fn g_IDEA_CARRIERS() -> Py { ensure_init(); G_IDEA_CARRIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDEA_CARRIERS(v: Py) { ensure_init(); G_IDEA_CARRIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_IDEA_SHAPES() -> Py { ensure_init(); G_IDEA_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDEA_SHAPES(v: Py) { ensure_init(); G_IDEA_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPLIED_ASKS() -> Py { ensure_init(); G_IMPLIED_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPLIED_ASKS(v: Py) { ensure_init(); G_IMPLIED_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING_LEAD() -> Py { ensure_init(); G_KEEPING_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING_LEAD(v: Py) { ensure_init(); G_KEEPING_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_ASK() -> Py { ensure_init(); G_KEEP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_ASK(v: Py) { ensure_init(); G_KEEP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_ASK_WORDS() -> Py { ensure_init(); G_KEEP_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_ASK_WORDS(v: Py) { ensure_init(); G_KEEP_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_QUALITY_WORDS() -> Py { ensure_init(); G_KEEP_QUALITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_QUALITY_WORDS(v: Py) { ensure_init(); G_KEEP_QUALITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIDS_IDEA() -> Py { ensure_init(); G_KIDS_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIDS_IDEA(v: Py) { ensure_init(); G_KIDS_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_KITCHEN_SWAP() -> Py { ensure_init(); G_KITCHEN_SWAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KITCHEN_SWAP(v: Py) { ensure_init(); G_KITCHEN_SWAP.with(|c| *c.borrow_mut() = v); }
pub fn g_LIKING() -> Py { ensure_init(); G_LIKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIKING(v: Py) { ensure_init(); G_LIKING.with(|c| *c.borrow_mut() = v); }
pub fn g_LUNCH_MEALS() -> Py { ensure_init(); G_LUNCH_MEALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LUNCH_MEALS(v: Py) { ensure_init(); G_LUNCH_MEALS.with(|c| *c.borrow_mut() = v); }
pub fn g_MAKE_VERB() -> Py { ensure_init(); G_MAKE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAKE_VERB(v: Py) { ensure_init(); G_MAKE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_MAKING_A_FAULT() -> Py { ensure_init(); G_MAKING_A_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAKING_A_FAULT(v: Py) { ensure_init(); G_MAKING_A_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_WORDS() -> Py { ensure_init(); G_MEAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_WORDS(v: Py) { ensure_init(); G_MEAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MIN_TYPO_LENGTH() -> Py { ensure_init(); G_MIN_TYPO_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MIN_TYPO_LENGTH(v: Py) { ensure_init(); G_MIN_TYPO_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_NICE_IDEA() -> Py { ensure_init(); G_NICE_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NICE_IDEA(v: Py) { ensure_init(); G_NICE_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_NICE_WORDS() -> Py { ensure_init(); G_NICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NICE_WORDS(v: Py) { ensure_init(); G_NICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DISH() -> Py { ensure_init(); G_NOT_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DISH(v: Py) { ensure_init(); G_NOT_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_KEEPING_QUESTION() -> Py { ensure_init(); G_NOT_A_KEEPING_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_KEEPING_QUESTION(v: Py) { ensure_init(); G_NOT_A_KEEPING_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_FOOD() -> Py { ensure_init(); G_NOT_THE_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_FOOD(v: Py) { ensure_init(); G_NOT_THE_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_APPLIANCE() -> Py { ensure_init(); G_NO_APPLIANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_APPLIANCE(v: Py) { ensure_init(); G_NO_APPLIANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUT_FREE_BAKE() -> Py { ensure_init(); G_NUT_FREE_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUT_FREE_BAKE(v: Py) { ensure_init(); G_NUT_FREE_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDER_AT_A_PLACE() -> Py { ensure_init(); G_ORDER_AT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDER_AT_A_PLACE(v: Py) { ensure_init(); G_ORDER_AT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDER_AT_A_PLACE_ASK() -> Py { ensure_init(); G_ORDER_AT_A_PLACE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDER_AT_A_PLACE_ASK(v: Py) { ensure_init(); G_ORDER_AT_A_PLACE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OR_WORD() -> Py { ensure_init(); G_OR_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OR_WORD(v: Py) { ensure_init(); G_OR_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PEOPLE() -> Py { ensure_init(); G_OTHER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PEOPLE(v: Py) { ensure_init(); G_OTHER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_ASK() -> Py { ensure_init(); G_OUT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_ASK(v: Py) { ensure_init(); G_OUT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_AVOID() -> Py { ensure_init(); G_OUT_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_AVOID(v: Py) { ensure_init(); G_OUT_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_OF_CUE() -> Py { ensure_init(); G_OUT_OF_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_OF_CUE(v: Py) { ensure_init(); G_OUT_OF_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_WORDS() -> Py { ensure_init(); G_OUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_WORDS(v: Py) { ensure_init(); G_OUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OVEN_WORD() -> Py { ensure_init(); G_OVEN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVEN_WORD(v: Py) { ensure_init(); G_OVEN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_DISLIKE() -> Py { ensure_init(); G_OWN_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_DISLIKE(v: Py) { ensure_init(); G_OWN_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIRING_FOODS() -> Py { ensure_init(); G_PAIRING_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIRING_FOODS(v: Py) { ensure_init(); G_PAIRING_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIRING_LEAN() -> Py { ensure_init(); G_PAIRING_LEAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIRING_LEAN(v: Py) { ensure_init(); G_PAIRING_LEAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIRING_PLAIN() -> Py { ensure_init(); G_PAIRING_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIRING_PLAIN(v: Py) { ensure_init(); G_PAIRING_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PAIRING_RICH() -> Py { ensure_init(); G_PAIRING_RICH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAIRING_RICH(v: Py) { ensure_init(); G_PAIRING_RICH.with(|c| *c.borrow_mut() = v); }
pub fn g_PART() -> Py { ensure_init(); G_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART(v: Py) { ensure_init(); G_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_BACKREF() -> Py { ensure_init(); G_PART_BACKREF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_BACKREF(v: Py) { ensure_init(); G_PART_BACKREF.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_NOT() -> Py { ensure_init(); G_PART_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_NOT(v: Py) { ensure_init(); G_PART_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OF() -> Py { ensure_init(); G_PART_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OF(v: Py) { ensure_init(); G_PART_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_WORDS() -> Py { ensure_init(); G_PART_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_WORDS(v: Py) { ensure_init(); G_PART_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_DISLIKE() -> Py { ensure_init(); G_PERSON_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_DISLIKE(v: Py) { ensure_init(); G_PERSON_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_REST() -> Py { ensure_init(); G_PERSON_REST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_REST(v: Py) { ensure_init(); G_PERSON_REST.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_START() -> Py { ensure_init(); G_PERSON_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_START(v: Py) { ensure_init(); G_PERSON_START.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_STATE() -> Py { ensure_init(); G_PERSON_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_STATE(v: Py) { ensure_init(); G_PERSON_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_WORDS() -> Py { ensure_init(); G_PICK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_WORDS(v: Py) { ensure_init(); G_PICK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN() -> Py { ensure_init(); G_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN(v: Py) { ensure_init(); G_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_FOODS() -> Py { ensure_init(); G_PLAIN_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_FOODS(v: Py) { ensure_init(); G_PLAIN_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLANS() -> Py { ensure_init(); G_PLANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLANS(v: Py) { ensure_init(); G_PLANS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_NAMES() -> Py { ensure_init(); G_PLAN_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_NAMES(v: Py) { ensure_init(); G_PLAN_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_NAMES_LONGEST() -> Py { ensure_init(); G_PLAN_NAMES_LONGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_NAMES_LONGEST(v: Py) { ensure_init(); G_PLAN_NAMES_LONGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_PROBLEMS() -> Py { ensure_init(); G_PROBLEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROBLEMS(v: Py) { ensure_init(); G_PROBLEMS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN() -> Py { ensure_init(); G_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); G_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_PUDDINGS() -> Py { ensure_init(); G_PUDDINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUDDINGS(v: Py) { ensure_init(); G_PUDDINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_PUDDING_ASK() -> Py { ensure_init(); G_PUDDING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUDDING_ASK(v: Py) { ensure_init(); G_PUDDING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_RAW_WORDS() -> Py { ensure_init(); G_RAW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAW_WORDS(v: Py) { ensure_init(); G_RAW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_READY_NAMES() -> Py { ensure_init(); G_READY_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_READY_NAMES(v: Py) { ensure_init(); G_READY_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_READY_NOTES() -> Py { ensure_init(); G_READY_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_READY_NOTES(v: Py) { ensure_init(); G_READY_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_REWARDED() -> Py { ensure_init(); G_REWARDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REWARDED(v: Py) { ensure_init(); G_REWARDED.with(|c| *c.borrow_mut() = v); }
pub fn g_RICH_FOODS() -> Py { ensure_init(); G_RICH_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RICH_FOODS(v: Py) { ensure_init(); G_RICH_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAFETY_WORDS() -> Py { ensure_init(); G_SAFETY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAFETY_WORDS(v: Py) { ensure_init(); G_SAFETY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_DISLIKE() -> Py { ensure_init(); G_SAID_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_DISLIKE(v: Py) { ensure_init(); G_SAID_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_DISLIKE_OTHER() -> Py { ensure_init(); G_SAID_DISLIKE_OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_DISLIKE_OTHER(v: Py) { ensure_init(); G_SAID_DISLIKE_OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORTAGE_MARKERS() -> Py { ensure_init(); G_SHORTAGE_MARKERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORTAGE_MARKERS(v: Py) { ensure_init(); G_SHORTAGE_MARKERS.with(|c| *c.borrow_mut() = v); }
pub fn g_SIDES() -> Py { ensure_init(); G_SIDES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIDES(v: Py) { ensure_init(); G_SIDES.with(|c| *c.borrow_mut() = v); }
pub fn g_SIDE_METHOD() -> Py { ensure_init(); G_SIDE_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIDE_METHOD(v: Py) { ensure_init(); G_SIDE_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_SIDE_WORDS() -> Py { ensure_init(); G_SIDE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIDE_WORDS(v: Py) { ensure_init(); G_SIDE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STAGE_PARTS() -> Py { ensure_init(); G_STAGE_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAGE_PARTS(v: Py) { ensure_init(); G_STAGE_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTED_PARTS() -> Py { ensure_init(); G_STARTED_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTED_PARTS(v: Py) { ensure_init(); G_STARTED_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_START_WORDS() -> Py { ensure_init(); G_START_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_START_WORDS(v: Py) { ensure_init(); G_START_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_VERBS() -> Py { ensure_init(); G_STEP_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_VERBS(v: Py) { ensure_init(); G_STEP_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBSTITUTES() -> Py { ensure_init(); G_SUBSTITUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBSTITUTES(v: Py) { ensure_init(); G_SUBSTITUTES.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_IN_ANSWER() -> Py { ensure_init(); G_TIME_IN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_IN_ANSWER(v: Py) { ensure_init(); G_TIME_IN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_IN_A_STEP() -> Py { ensure_init(); G_TIME_IN_A_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_IN_A_STEP(v: Py) { ensure_init(); G_TIME_IN_A_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAY_IDEA() -> Py { ensure_init(); G_TRAY_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAY_IDEA(v: Py) { ensure_init(); G_TRAY_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_UP_PHRASES() -> Py { ensure_init(); G_USE_UP_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_UP_PHRASES(v: Py) { ensure_init(); G_USE_UP_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGAN_IDEA() -> Py { ensure_init(); G_VEGAN_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGAN_IDEA(v: Py) { ensure_init(); G_VEGAN_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGAN_SWAP_IDEA() -> Py { ensure_init(); G_VEGAN_SWAP_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGAN_SWAP_IDEA(v: Py) { ensure_init(); G_VEGAN_SWAP_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGAN_WORD() -> Py { ensure_init(); G_VEGAN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGAN_WORD(v: Py) { ensure_init(); G_VEGAN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGGIE_CARRIERS() -> Py { ensure_init(); G_VEGGIE_CARRIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGGIE_CARRIERS(v: Py) { ensure_init(); G_VEGGIE_CARRIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGGIE_IDEA() -> Py { ensure_init(); G_VEGGIE_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGGIE_IDEA(v: Py) { ensure_init(); G_VEGGIE_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGGIE_KEY() -> Py { ensure_init(); G_VEGGIE_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGGIE_KEY(v: Py) { ensure_init(); G_VEGGIE_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGGIE_SWAP_IDEA() -> Py { ensure_init(); G_VEGGIE_SWAP_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGGIE_SWAP_IDEA(v: Py) { ensure_init(); G_VEGGIE_SWAP_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_FAMILIES() -> Py { ensure_init(); G_VEG_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_FAMILIES(v: Py) { ensure_init(); G_VEG_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_FAULTS() -> Py { ensure_init(); G_VEG_FAULTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_FAULTS(v: Py) { ensure_init(); G_VEG_FAULTS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_NAMES() -> Py { ensure_init(); G_VEG_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_NAMES(v: Py) { ensure_init(); G_VEG_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_VEG_WORDS() -> Py { ensure_init(); G_VEG_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEG_WORDS(v: Py) { ensure_init(); G_VEG_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WASTE_WORDS() -> Py { ensure_init(); G_WASTE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WASTE_WORDS(v: Py) { ensure_init(); G_WASTE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); G_WHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); G_WHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_FOOD() -> Py { ensure_init(); G_WHOLE_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_FOOD(v: Py) { ensure_init(); G_WHOLE_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_NO_ASK() -> Py { ensure_init(); G_YES_NO_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_NO_ASK(v: Py) { ensure_init(); G_YES_NO_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLERGEN_NOTE() -> Py { ensure_init(); G__ALLERGEN_NOTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLERGEN_NOTE(v: Py) { ensure_init(); G__ALLERGEN_NOTE.with(|c| *c.borrow_mut() = v); }
pub fn g__ANY_FOOD() -> Py { ensure_init(); G__ANY_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANY_FOOD(v: Py) { ensure_init(); G__ANY_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED_ABOUT_THE_FAULT() -> Py { ensure_init(); G__ASKED_ABOUT_THE_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED_ABOUT_THE_FAULT(v: Py) { ensure_init(); G__ASKED_ABOUT_THE_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_NOISE() -> Py { ensure_init(); G__ASK_NOISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_NOISE(v: Py) { ensure_init(); G__ASK_NOISE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MADE_DISH() -> Py { ensure_init(); G__A_MADE_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MADE_DISH(v: Py) { ensure_init(); G__A_MADE_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE_VERB() -> Py { ensure_init(); G__A_STATE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE_VERB(v: Py) { ensure_init(); G__A_STATE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRDS() -> Py { ensure_init(); G__BIRDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRDS(v: Py) { ensure_init(); G__BIRDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CHOICE_FRAME() -> Py { ensure_init(); G__CHOICE_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHOICE_FRAME(v: Py) { ensure_init(); G__CHOICE_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g__CLEANING_IT() -> Py { ensure_init(); G__CLEANING_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLEANING_IT(v: Py) { ensure_init(); G__CLEANING_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__COOK_PHRASES() -> Py { ensure_init(); G__COOK_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COOK_PHRASES(v: Py) { ensure_init(); G__COOK_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g__DIET_FOOD() -> Py { ensure_init(); G__DIET_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DIET_FOOD(v: Py) { ensure_init(); G__DIET_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__DRY_BAKE() -> Py { ensure_init(); G__DRY_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRY_BAKE(v: Py) { ensure_init(); G__DRY_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g__GENERIC_SUBJECT() -> Py { ensure_init(); G__GENERIC_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GENERIC_SUBJECT(v: Py) { ensure_init(); G__GENERIC_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__GLUE() -> Py { ensure_init(); G__GLUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GLUE(v: Py) { ensure_init(); G__GLUE.with(|c| *c.borrow_mut() = v); }
pub fn g__GRAINY() -> Py { ensure_init(); G__GRAINY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRAINY(v: Py) { ensure_init(); G__GRAINY.with(|c| *c.borrow_mut() = v); }
pub fn g__GRAINY_FIX() -> Py { ensure_init(); G__GRAINY_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRAINY_FIX(v: Py) { ensure_init(); G__GRAINY_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g__IDEA_ASK() -> Py { ensure_init(); G__IDEA_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IDEA_ASK(v: Py) { ensure_init(); G__IDEA_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__INGREDIENT_AFTER() -> Py { ensure_init(); G__INGREDIENT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INGREDIENT_AFTER(v: Py) { ensure_init(); G__INGREDIENT_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_ASK() -> Py { ensure_init(); G__KEEP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_ASK(v: Py) { ensure_init(); G__KEEP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_BUT_COOKING() -> Py { ensure_init(); G__KEEP_BUT_COOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_BUT_COOKING(v: Py) { ensure_init(); G__KEEP_BUT_COOKING.with(|c| *c.borrow_mut() = v); }
pub fn g__LUMPY() -> Py { ensure_init(); G__LUMPY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LUMPY(v: Py) { ensure_init(); G__LUMPY.with(|c| *c.borrow_mut() = v); }
pub fn g__LUMPY_FIX() -> Py { ensure_init(); G__LUMPY_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LUMPY_FIX(v: Py) { ensure_init(); G__LUMPY_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g__MEALS() -> Py { ensure_init(); G__MEALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEALS(v: Py) { ensure_init(); G__MEALS.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSHY_FIX() -> Py { ensure_init(); G__MUSHY_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSHY_FIX(v: Py) { ensure_init(); G__MUSHY_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSHY_WORDS() -> Py { ensure_init(); G__MUSHY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSHY_WORDS(v: Py) { ensure_init(); G__MUSHY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMES_A_SAUCE() -> Py { ensure_init(); G__NAMES_A_SAUCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMES_A_SAUCE(v: Py) { ensure_init(); G__NAMES_A_SAUCE.with(|c| *c.borrow_mut() = v); }
pub fn g__PICK_ONE() -> Py { ensure_init(); G__PICK_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PICK_ONE(v: Py) { ensure_init(); G__PICK_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAN_MARKER() -> Py { ensure_init(); G__PLAN_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAN_MARKER(v: Py) { ensure_init(); G__PLAN_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAN_THAT_MIGHT_BE_RUINED() -> Py { ensure_init(); G__PLAN_THAT_MIGHT_BE_RUINED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAN_THAT_MIGHT_BE_RUINED(v: Py) { ensure_init(); G__PLAN_THAT_MIGHT_BE_RUINED.with(|c| *c.borrow_mut() = v); }
pub fn g__POT() -> Py { ensure_init(); G__POT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POT(v: Py) { ensure_init(); G__POT.with(|c| *c.borrow_mut() = v); }
pub fn g__RECIPE_CARDS() -> Py { ensure_init(); G__RECIPE_CARDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECIPE_CARDS(v: Py) { ensure_init(); G__RECIPE_CARDS.with(|c| *c.borrow_mut() = v); }
pub fn g__RED_MEAT() -> Py { ensure_init(); G__RED_MEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RED_MEAT(v: Py) { ensure_init(); G__RED_MEAT.with(|c| *c.borrow_mut() = v); }
pub fn g__RULE_OUT() -> Py { ensure_init(); G__RULE_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RULE_OUT(v: Py) { ensure_init(); G__RULE_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__RULE_OUT_FILLER() -> Py { ensure_init(); G__RULE_OUT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RULE_OUT_FILLER(v: Py) { ensure_init(); G__RULE_OUT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__SINK_AS_A_BAKE() -> Py { ensure_init(); G__SINK_AS_A_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SINK_AS_A_BAKE(v: Py) { ensure_init(); G__SINK_AS_A_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g__SINK_VERBS() -> Py { ensure_init(); G__SINK_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SINK_VERBS(v: Py) { ensure_init(); G__SINK_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__STANDS_FOR_IT() -> Py { ensure_init(); G__STANDS_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STANDS_FOR_IT(v: Py) { ensure_init(); G__STANDS_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__STARCH() -> Py { ensure_init(); G__STARCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STARCH(v: Py) { ensure_init(); G__STARCH.with(|c| *c.borrow_mut() = v); }
pub fn g__TOUGH_BAKE() -> Py { ensure_init(); G__TOUGH_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOUGH_BAKE(v: Py) { ensure_init(); G__TOUGH_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_OVEN_BEEF() -> Py { ensure_init(); G__TWO_OVEN_BEEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_OVEN_BEEF(v: Py) { ensure_init(); G__TWO_OVEN_BEEF.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_OVEN_CAKE() -> Py { ensure_init(); G__TWO_OVEN_CAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_OVEN_CAKE(v: Py) { ensure_init(); G__TWO_OVEN_CAKE.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_OVEN_GAP() -> Py { ensure_init(); G__TWO_OVEN_GAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_OVEN_GAP(v: Py) { ensure_init(); G__TWO_OVEN_GAP.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_OVEN_HEAD() -> Py { ensure_init(); G__TWO_OVEN_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_OVEN_HEAD(v: Py) { ensure_init(); G__TWO_OVEN_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_OVEN_SPONGE() -> Py { ensure_init(); G__TWO_OVEN_SPONGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_OVEN_SPONGE(v: Py) { ensure_init(); G__TWO_OVEN_SPONGE.with(|c| *c.borrow_mut() = v); }
pub fn g__cake_eggs() -> Py { ensure_init(); G__cake_eggs.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__cake_eggs(v: Py) { ensure_init(); G__cake_eggs.with(|c| *c.borrow_mut() = v); }
pub fn g__cake_plan() -> Py { ensure_init(); G__cake_plan.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__cake_plan(v: Py) { ensure_init(); G__cake_plan.with(|c| *c.borrow_mut() = v); }
pub fn g__engine_reads() -> Py { ensure_init(); G__engine_reads.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__engine_reads(v: Py) { ensure_init(); G__engine_reads.with(|c| *c.borrow_mut() = v); }
pub fn g__names() -> Py { ensure_init(); G__names.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__names(v: Py) { ensure_init(); G__names.with(|c| *c.borrow_mut() = v); }
pub fn g__one_edit() -> Py { ensure_init(); G__one_edit.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__one_edit(v: Py) { ensure_init(); G__one_edit.with(|c| *c.borrow_mut() = v); }
pub fn g__slug() -> Py { ensure_init(); G__slug.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__slug(v: Py) { ensure_init(); G__slug.with(|c| *c.borrow_mut() = v); }
pub fn g__what() -> Py { ensure_init(); G__what.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__what(v: Py) { ensure_init(); G__what.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_not_a_mark_on_a_thing() -> Py { ensure_init(); G_not_a_mark_on_a_thing.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_not_a_mark_on_a_thing(v: Py) { ensure_init(); G_not_a_mark_on_a_thing.with(|c| *c.borrow_mut() = v); }
pub fn g_not_a_present() -> Py { ensure_init(); G_not_a_present.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_not_a_present(v: Py) { ensure_init(); G_not_a_present.with(|c| *c.borrow_mut() = v); }
pub fn g_tokens() -> Py { ensure_init(); G_tokens.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_tokens(v: Py) { ensure_init(); G_tokens.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Dish plans: what a cooking conversation needs after the first answer.\n\nIt also holds the first-question rows that could not go in cooking.py, which is over the world's\n1 MiB file cap: a spice bloomed in oil, a bird brined, and the baking faults no table knew.\n\nbot/knowledge/cooking.py answers the first question about a dish. A real conversation then\ncarries on: \"how long in the oven?\", \"what temperature?\", \"do I need to take the skin off?\",\n\"and if I stuff it?\", \"can I make it the day before?\", \"and what about the rice?\". Those\nlines name no dish at all, so a keyed entry cannot find them: what they mean is the dish the\nconversation is already about.\n\nSo the facts about a dish live here once, in PLANS: how hot, how long, how you know it is\ndone, whether it rests, how to get ahead, and the parts people ask about next (skin,\nstuffing, a side). The ask entries below fire on the *shape* of the question, read the dish\nfrom the last few turns (memory[\"turns\"], memory[\"topic\"]), and answer from its plan.\n\nTwo guards keep this honest. An entry passes (returns None) when no dish was named in the\nconversation, or when the dish has no fact for what was asked; and the entries keep out of\nlines that name a dish themselves (`avoid`), because cooking.py already answers those.\nSo these entries only take the turns that fall back today.\n\nTimings agree with cooking.py; if one changes there, change it here too. The answers lane\nowns this file; pass lines you do not answer rather than guessing.\n\nThe second half of the file is the other side of the same coin: what a named dish *is*. \"What is\na yorkshire pudding\" is not a method question, and no per-dish method row answers it - the\nquestion is what is on the plate. Those rows live in DISH_WHAT, one per dish, keyed by the\ndish's own names and gated on a definition word, so a how-to line keeps its method.\n");
    s_CAKE_AVOID(pyrt::xrefs::xget("knowledge.cakes", "CAKE_AVOID"));
    s_CAKE_EGGS_AVOID(pyrt::xrefs::xget("knowledge.cakes", "CAKE_EGGS_AVOID"));
    s_CAKE_EGGS_KEY(pyrt::xrefs::xget("knowledge.cakes", "CAKE_EGGS_KEY"));
    s_CAKE_KEY(pyrt::xrefs::xget("knowledge.cakes", "CAKE_KEY"));
    s__cake_eggs(py_func("cake_eggs", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cakes", "cake_eggs", args)));
    s__cake_plan(py_func("cake_plan", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cakes", "cake_plan", args)));
    s_not_a_mark_on_a_thing(py_func("not_a_mark_on_a_thing", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cleaning", "not_a_mark_on_a_thing", args)));
    s_DAY_WORDS(pyrt::xrefs::xget("knowledge.lookup", "DAY_WORDS"));
    s_MIN_TYPO_LENGTH(pyrt::xrefs::xget("knowledge.lookup", "MIN_TYPO_LENGTH"));
    s__engine_reads(py_func("_engine_reads", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "_engine_reads", args)));
    s__one_edit(py_func("_one_edit", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "_one_edit", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_tokens(py_func("tokens", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "tokens", args)));
    s_not_a_present(py_func("not_a_present", |args: &[Py]| pyrt::xrefs::xcall("knowledge.shopping", "not_a_present", args)));
    fn __data1() -> Py { py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("chicken"), py_str("rice"), py_str("pasta"), py_str("potato"), py_str("potatoes"), py_str("steak"), py_str("beef"), py_str("mince"), py_str("sausage"), py_str("sausages"), py_str("bacon"), py_str("salmon"), py_str("fish"), py_str("cod"), py_str("prawns"), py_str("cake"), py_str("bread"), py_str("sourdough"), py_str("quinoa"), py_str("couscous"), py_str("porridge"), py_str("oats"), py_str("broccoli"), py_str("soup"), py_str("curry"), py_str("gravy"), py_str("mushroom"), py_str("mushrooms"), py_str("onion"), py_str("onions"), py_str("garlic"), py_str("tomato"), py_str("tomatoes"), py_str("avocado"), py_str("banana"), py_str("cheese"), py_str("milk"), py_str("butter"), py_str("flour"), py_str("yoghurt"), py_str("yogurt"), py_str("lentils"), py_str("chickpeas"), py_str("beans"), py_str("noodles"), py_str("prawn"), py_str("turkey"), py_str("lamb"), py_str("pork"), py_str("ham"), py_str("spinach"), py_str("carrot"), py_str("carrots"), py_str("cabbage"), py_str("peas"), py_str("corn"), py_str("leek"), py_str("aubergine"), py_str("courgette"), py_str("pepper"), py_str("peppers"), py_str("salad"), py_str("casserole"), py_str("roast dinner")]) }
    s_DISH_WORDS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("veg"), py_str("vegetables"), py_str("vegetable"), py_str("carrot"), py_str("carrots"), py_str("courgette"), py_str("courgettes"), py_str("zucchini"), py_str("broccoli"), py_str("spinach"), py_str("peas"), py_str("greens"), py_str("cauliflower"), py_str("beetroot"), py_str("swede"), py_str("squash"), py_str("pumpkin"), py_str("leek"), py_str("leeks"), py_str("cabbage"), py_str("kale"), py_str("pepper"), py_str("peppers"), py_str("mushroom"), py_str("mushrooms"), py_str("tomato"), py_str("tomatoes"), py_str("parsnip"), py_str("parsnips"), py_str("turnip"), py_str("turnips"), py_str("sweetcorn"), py_str("beans"), py_str("lentils"), py_str("aubergine"), py_str("celery")]) }
    s_VEG_WORDS(__data2());
    fn __data4() -> Py { py_tuple(vec![py_str("pox"), py_str("shop"), py_str("shops"), py_str("wing"), py_str("wings"), py_str("nugget"), py_str("nuggets"), py_str("soup"), py_str("stock"), py_str("sandwich"), py_str("salad"), py_str("curry"), py_str("pie"), py_str("breast"), py_str("breasts"), py_str("thigh"), py_str("thighs"), py_str("leg"), py_str("legs"), py_str("fillet"), py_str("fillets")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("potatoes"), py_str("potato"), py_str("chicken"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("turkey"), py_str("duck"), py_str("nuts"), py_str("chestnuts"), py_str("coffee"), py_str("beans"), py_str("peppers"), py_str("tomatoes"), py_str("onions"), py_str("parsnips"), py_str("carrots"), py_str("garlic")]) }
    fn __data3() -> Py { py_dict(vec![(py_str("roast-chicken"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("roast chicken")]), py_tuple(vec![py_str("whole chicken")]), py_tuple(vec![py_str("the chicken")]), py_tuple(vec![py_str("a chicken")]), py_tuple(vec![py_str("chicken")])])), (py_str("avoid"), __data4()), (py_str("temp"), py_str("190C, or 170C in a fan oven, in the middle of the oven.")), (py_str("time"), py_str("20 minutes per 500g plus 20 minutes on top: 40 minutes per kilo, so 1 hour 40 for a 2kg bird.")), (py_str("done"), py_str("74C in the thickest part of the thigh on a digital thermometer, pushed in without touching bone, and the juices run clear with the leg moving freely in the socket; 75C for a moment is the same thing.")), (py_str("rest"), py_str("15 minutes under foil before carving, or the juice ends up on the board instead of in the meat.")), (py_str("skin"), py_str("Leave the skin on: it crisps and keeps the breast moist. Only take it off if someone at the table wants skinless, and expect the breast to dry a little.")), (py_str("stuffing"), py_str("Stuff the neck end only, weigh the bird with the stuffing in and add 15 to 20 minutes; check the stuffing is piping hot in the middle, not just the meat.")), (py_str("ahead"), py_str("You can get it ready the day before: rub it with butter and salt, keep it covered in the fridge, and roast it from cold with 10 minutes extra.")), (py_str("keep"), py_str("Wrap the leftovers within an hour and they keep 3 days in the fridge; the carcass makes good stock.")), (py_str("fill"), py_str("Push a lemon and a few garlic cloves into the cavity, rub the skin with butter and salt, and put onion and carrot in the tin under the bird for the gravy."))])), (py_str("chicken-thighs"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("chicken thigh")]), py_tuple(vec![py_str("chicken thighs")]), py_tuple(vec![py_str("thighs")]), py_tuple(vec![py_str("traybake")]), py_tuple(vec![py_str("tray bake")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven, and the tray on a high shelf.")), (py_str("time"), py_str("35 to 40 minutes at 200C (180C fan), skin side up.")), (py_str("done"), py_str("The skin is crisp and the juices run clear when you pierce the thickest part; 74C with a thermometer.")), (py_str("skin"), py_str("Keep the skin on and salt it: it is the best part, and it protects the meat underneath. Skinless thighs need only 25 to 30 minutes.")), (py_str("ahead"), py_str("You can marinate them the night before, or roast them in the morning and reheat at 180C for 10 minutes.")), (py_str("keep"), py_str("They keep 3 days in the fridge and reheat better than breast.")), (py_str("rest"), py_str("5 minutes on the tray under foil is plenty: they are small, so there is no need to rest them like a whole bird.")), (py_str("fill"), py_str("Marinate them in yoghurt with paprika, cumin and garlic for an hour if you can, or just oil, salt and lemon: they take strong flavours and stay juicy.")), (py_str("sides"), py_str("Potatoes tossed in the same tray save a pan; they need 40 minutes, so start them first."))])), (py_str("chicken-breast"), py_dict(vec![(py_str("ahead"), py_str("Season them and keep them covered in the fridge a few hours before, or cook them in the morning and slice them cold: reheated they dry out.")), (py_str("names"), py_tuple(vec![py_tuple(vec![py_str("chicken breast")]), py_tuple(vec![py_str("chicken breasts")]), py_tuple(vec![py_str("breast")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("18 to 20 minutes at 200C (180C fan) for a flattened breast, and 12 to 15 for a small one.")), (py_str("done"), py_str("The middle is white with no pink, and a thermometer reads 74C; take them out the moment they are there.")), (py_str("rest"), py_str("5 minutes under foil, so the juice stays in when you cut them.")), (py_str("keep"), py_str("They keep 3 days, and are better sliced cold in a salad than reheated.")), (py_str("fill"), py_str("Salt it, then a herb butter or a spoon of pesto under the skin-side: breast is the one cut that needs help to stay moist."))])), (py_str("salmon"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("salmon")]), py_tuple(vec![py_str("salmon fillet")]), py_tuple(vec![py_str("salmon fillets")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("12 to 15 minutes at 200C (180C fan), or 4 minutes skin side down in a hot pan.")), (py_str("done"), py_str("The flesh just flakes when you press it with a fork, and it is still slightly translucent in the very middle.")), (py_str("skin"), py_str("Leave the skin on for the oven or a pan: it protects the fish and goes crisp. It slides off easily once cooked.")), (py_str("ahead"), py_str("You can bake it cold the night before and flake it into a salad or pasta.")), (py_str("keep"), py_str("Cooked salmon keeps 2 days in the fridge, and does not freeze well.")), (py_str("rest"), py_str("No need to rest it: serve salmon straight away, while it is still hot and flaky.")), (py_str("fill"), py_str("Salt, pepper, a slice of lemon and a knob of butter, or a spoon of pesto or soy and honey. Keep it simple, the fish does the work."))])), (py_str("steak"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("steak")]), py_tuple(vec![py_str("sirloin")]), py_tuple(vec![py_str("ribeye")]), py_tuple(vec![py_str("rump steak")])])), (py_str("avoid"), py_tuple(vec![py_str("gammon"), py_str("gammon steak"), py_str("tuna"), py_str("salmon"), py_str("fish")])), (py_str("temp"), py_str("A hot pan or 220C (200C fan) in the oven to finish.")), (py_str("time"), py_str("2 to 3 minutes a side for medium rare, 4 a side for medium, plus 5 minutes resting.")), (py_str("done"), py_str("Press it with a finger: springy like the base of your thumb is medium rare, firm is well done. A thermometer reads 50C for rare, 55C medium rare, 60C medium.")), (py_str("rest"), py_str("Rest it 5 minutes under foil, always: the heat drives the juice to the middle while it cooks, and resting lets it spread back out through the meat. Cut straight away and it ends up on the board.")), (py_str("skin"), py_str("Trim any hard fat, but leave a rim of it on: it bastes the steak as it cooks.")), (py_str("fill"), py_str("Salt it generously just before it goes in the pan, then pepper after: pepper burns. A knob of butter and a garlic clove basted over at the end is the restaurant trick."))])), (py_str("gammon steak"), py_dict(vec![(py_str("label"), py_str("gammon steak")), (py_str("names"), py_tuple(vec![py_tuple(vec![py_str("gammon steak")]), py_tuple(vec![py_str("gammon steaks")]), py_tuple(vec![py_str("gammon")])])), (py_str("temp"), py_str("A hot frying pan, or a grill on high.")), (py_str("time"), py_str("4 to 5 minutes a side in a hot pan, or 15 to 20 minutes under the grill, turning once.")), (py_str("done"), py_str("It is cooked through, not pink: no pink in the middle, and a thermometer reads 70C. It is cured pork, so it wants longer than a beef steak.")), (py_str("rest"), py_str("2 or 3 minutes is enough - there is less juice in it to settle than in beef.")), (py_str("keep"), py_str("It keeps 2 days cooked, and it is better cold in a sandwich than reheated, which dries it out.")), (py_str("fill"), py_str("Nothing more than pepper: it is already salty. A spoon of honey or marmalade in the pan at the end glazes it."))])), (py_str("sausages"), py_dict(vec![(py_str("ahead"), py_str("Cook them on the day. They take 25 to 30 minutes and reheat badly, so there is little to gain by doing them early.")), (py_str("label"), py_str("sausages")), (py_str("names"), py_tuple(vec![py_tuple(vec![py_str("sausage")]), py_tuple(vec![py_str("sausages")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("25 to 30 minutes at 200C (180C fan), turning once.")), (py_str("done"), py_str("They are cooked when they are browned all over and the juices run clear; press one and it should feel firm, not springy.")), (py_str("skin"), py_str("Don't prick them: the skin holds the juice in, and pricking is what makes them dry.")), (py_str("keep"), py_str("They keep 3 days in the fridge and freeze well, cooked or raw.")), (py_str("fill"), py_str("Nothing inside them: put onion, apple or fennel in the tin beside them, and let the fat that comes out season the mash."))])), (py_str("roast-potatoes"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("roast potatoes")]), py_tuple(vec![py_str("roast potato")]), py_tuple(vec![py_str("roasties")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven, in a thin layer of hot oil.")), (py_str("time"), py_str("Parboil 8 minutes, then 40 to 45 minutes in the oven, turning once halfway.")), (py_str("done"), py_str("They are ready when the edges are deep gold and crusty; shake the tin and they should rattle.")), (py_str("ahead"), py_str("Parboil and rough them up in the morning, keep them in the fridge, and roast them at the last minute.")), (py_str("skin"), py_str("Leave the skin on if the potatoes are small: it goes crisper than a peeled one.")), (py_str("keep"), py_str("They reheat well at 200C for 8 minutes, better than in a microwave.")), (py_str("fill"), py_str("Parboil them, shake them in the pan to rough up the edges, then hot fat or oil, salt and a sprig of rosemary: the rough edges are what go crisp."))])), (py_str("cake"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("cake")]), py_tuple(vec![py_str("sponge")]), py_tuple(vec![py_str("birthday cake")]), py_tuple(vec![py_str("layer cake")])])), (py_str("temp"), py_str("180C, or 160C in a fan oven.")), (py_str("time"), py_str("A 20cm sponge bakes for 25 to 30 minutes and a loaf tin for 45 to 55; about an hour and a half start to finish once you count mixing and cooling.")), (py_str("done"), py_str("A skewer in the middle comes out clean, and the top springs back when you press it lightly.")), (py_str("ahead"), py_str("Bake it the day before, cool it completely, wrap it well and fill it on the day: it keeps 3 days in a tin.")), (py_str("keep"), py_str("It keeps 3 days in a tin, or freezes for 2 months undecorated.")), (py_str("allergy"), py_str("Leave the nuts out altogether and swap in a handful of chocolate chips or raisins; check the chocolate and any ready-made icing for 'may contain nuts', and keep the tin and worktop clean if the allergy is serious.")), (py_str("fill"), py_str("Butter and sugar creamed first, then eggs a little at a time, then flour folded in gently, and a spoon of milk if the mix is stiff.")), (py_str("sides"), py_str("Buttercream is 100g soft butter to 200g icing sugar, beaten with a splash of milk.")), (py_str("frosting"), py_str("Buttercream or ganache keeps 3 to 4 days in a sealed tub in the fridge, or 2 months in the freezer: bring it back to room temperature and beat it again before you use it. The cake is better filled on the day."))])), (py_str("curry"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("curry")]), py_tuple(vec![py_str("tikka")]), py_tuple(vec![py_str("korma")]), py_tuple(vec![py_str("dhal")]), py_tuple(vec![py_str("daal")])])), (py_str("temp"), py_str("Medium heat on the hob, 190C (170C fan) if it goes in the oven.")), (py_str("time"), py_str("35 to 45 minutes from scratch: 10 minutes of frying the onions and spices, then 25 to 30 minutes simmering.")), (py_str("done"), py_str("The sauce has darkened and thickened and the oil comes back to the surface; the meat should be tender enough to cut with a spoon.")), (py_str("ahead"), py_str("It is better made the day before: cool it quickly, keep it in the fridge and reheat it gently with a splash of water.")), (py_str("keep"), py_str("It keeps 3 days in the fridge and freezes well for 3 months.")), (py_str("fill"), py_str("Start with onion, garlic and ginger, then the spices (cumin, coriander, turmeric, chilli) fried for 30 seconds in the oil, then the meat or veg and a tin of tomatoes or coconut milk.")), (py_str("sides"), py_str("Rice, and naan or a chapati if you have them: a mug of rice to one and a half of water, 12 minutes with the lid on, and a spoon of yoghurt or raita beside it."))])), (py_str("bread"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("bread")]), py_tuple(vec![py_str("sourdough")]), py_tuple(vec![py_str("loaf")]), py_tuple(vec![py_str("dough")])])), (py_str("temp"), py_str("230C, or 210C in a fan oven, with the oven well preheated.")), (py_str("time"), py_str("35 to 45 minutes for a large loaf, 20 to 25 for rolls; the first 15 minutes with steam.")), (py_str("done"), py_str("It sounds hollow when you tap the base, and a thermometer reads 95C inside.")), (py_str("ahead"), py_str("You can leave the shaped loaf in the fridge overnight and bake it straight from cold, which gives a better flavour.")), (py_str("keep"), py_str("It keeps 3 days in a paper bag; slice and freeze what you won't eat.")), (py_str("fill"), py_str("Flour, water, salt and yeast, and time: a slow rise in the fridge overnight gives more flavour than anything you can add."))])), (py_str("chips"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("chips")]), py_tuple(vec![py_str("fries")]), py_tuple(vec![py_str("oven chips")]), py_tuple(vec![py_str("frozen chips")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven, on a tray with space between them.")), (py_str("time"), py_str("20 to 25 minutes at 200C from frozen, turning once; an air fryer does them in 12 to 15 minutes at 200C. Fresh-cut chips want 30 to 35 minutes.")), (py_str("done"), py_str("Golden at the edges and hollow-sounding when you shake the tray.")), (py_str("keep"), py_str("They are only good the day they are cooked: reheat them 5 minutes at 200C, never in a microwave, or they go limp.")), (py_str("fill"), py_str("Salt after, not before, and a sprig of rosemary or a shake of paprika in the last few minutes. Vinegar at the table.")), (py_str("sides"), py_str("Fish fingers, a fried egg or a burger, and peas, pickles or coleslaw: something sharp cuts the fat."))])), (py_str("pie"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("pie"), py_str("fruit pie"), py_str("apple pie"), py_str("pie dish")]), py_tuple(vec![py_str("pies"), py_str("fruit pies")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("30 to 35 minutes for a fruit pie with a pastry lid, and 45 minutes if it is a deep one; blind bake an empty case 15 minutes first.")), (py_str("done"), py_str("The pastry is golden and pulls away from the dish a little, and the filling bubbles up through the hole in the middle.")), (py_str("keep"), py_str("A fruit pie keeps 2 days in the fridge once it is baked, and reheats at 180C (160C fan) for 15 minutes, which brings the pastry back. Unbaked, it freezes for 3 months: freeze it in the dish, then bake it from frozen at 200C (180C fan) for 20 minutes longer than the recipe says."))])), (py_str("savoury pie"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("shepherd's pie"), py_str("shepherds pie"), py_str("shepherd pie"), py_str("cottage pie"), py_str("chicken pie"), py_str("steak pie"), py_str("pot pie"), py_str("savoury pie"), py_str("savory pie")]), py_tuple(vec![py_str("shepherd's pies"), py_str("cottage pies"), py_str("chicken pies"), py_str("steak pies")])])), (py_str("temp"), py_str("190C, or 170C in a fan oven.")), (py_str("time"), py_str("35 to 40 minutes from cold for a shepherd's or cottage pie; a pastry-topped one wants 200C (180C fan) for 30 minutes.")), (py_str("done"), py_str("The top is browned and the filling bubbles at the edges; a skewer in the middle comes out hot.")), (py_str("keep"), py_str("It keeps 3 to 4 days in the fridge, covered, and reheats at 180C for 20 minutes or in portions in the microwave. It freezes well for 3 months: freeze it before baking, then cook it from frozen for about an hour, or thaw it overnight first."))])), (py_str("scones"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("scone")]), py_tuple(vec![py_str("scones")]), py_tuple(vec![py_str("fruit scone")]), py_tuple(vec![py_str("cheese scone")])])), (py_str("temp"), py_str("220C, or 200C in a fan oven, on a hot floured tray near the top.")), (py_str("time"), py_str("12 to 15 minutes, and no longer than that or they go dry.")), (py_str("done"), py_str("They are risen and browned on top, and the sides look dry; a skewer in the middle comes out clean.")), (py_str("keep"), py_str("Best eaten the day they are made, warm. They freeze well, split first, and come back in a low oven for 5 minutes.")), (py_str("fill"), py_str("Rub cold butter into self-raising flour with a pinch of salt, add the milk in one go, and handle the dough as little as you can: cut out the rounds and put them straight in, and do not twist the cutter."))])), (py_str("muffins"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("muffin")]), py_tuple(vec![py_str("muffins")]), py_tuple(vec![py_str("blueberry muffin")]), py_tuple(vec![py_str("chocolate muffin")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("18 to 22 minutes for a normal-sized tin.")), (py_str("done"), py_str("The tops are domed and spring back when pressed, and a skewer comes out with crumbs rather than wet batter.")), (py_str("keep"), py_str("They keep 2 days in a tin and freeze for 2 months.")), (py_str("fill"), py_str("Melt the butter and mix the wet into the dry in one go, then stop: lumps are right for a muffin. Ten stirs too many is what makes them tough, and frozen berries go in unthawed."))])), (py_str("biscuits"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("biscuit"), py_str("biscuits"), py_str("cookie"), py_str("cookies")]), py_tuple(vec![py_str("shortbread")]), py_tuple(vec![py_str("flapjack")])])), (py_str("temp"), py_str("180C, or 160C in a fan oven, on a lined flat tray.")), (py_str("time"), py_str("10 to 12 minutes for a drop cookie; shortbread wants 20 to 25 at 160C.")), (py_str("done"), py_str("The edges are set and the middle still looks slightly underdone, which is right: they firm up as they cool on the tray.")), (py_str("keep"), py_str("They keep a week in a sealed tin, and freeze for 3 months.")), (py_str("fill"), py_str("Cream the butter and sugar first, fold the flour in last, and chill the dough 20 minutes so they spread less. Leave 5cm between them on the tray."))])), (py_str("meringues"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("meringue")]), py_tuple(vec![py_str("meringues")]), py_tuple(vec![py_str("pavlova")])])), (py_str("temp"), py_str("100C to 120C, or 90C to 100C in a fan oven, on greaseproof paper.")), (py_str("time"), py_str("1 hour to 1 hour 15 for dry meringues, then leave them in the switched-off oven until it is cold.")), (py_str("done"), py_str("They lift off the paper cleanly and sound hollow when tapped on the base.")), (py_str("keep"), py_str("They keep a week in an airtight tin, and go soft in the fridge.")), (py_str("fill"), py_str("Bowl and whisk free of grease, no trace of yolk in the whites, and sugar a spoonful at a time once they hold a soft peak: that is what stops them weeping."))])), (py_str("yorkshire-puddings"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("yorkshire pudding")]), py_tuple(vec![py_str("yorkshire puddings")]), py_tuple(vec![py_str("yorkshires")]), py_tuple(vec![py_str("popover")]), py_tuple(vec![py_str("popovers")])])), (py_str("temp"), py_str("220C, or 200C in a fan oven, with the tin of fat smoking hot first.")), (py_str("time"), py_str("20 to 25 minutes without opening the door.")), (py_str("done"), py_str("They are tall, crisp and brown at the edges; a pale one has not had enough heat or enough fat.")), (py_str("keep"), py_str("They freeze for a month and come back at 200C for 5 minutes.")), (py_str("fill"), py_str("Equal volumes of egg, milk and plain flour, rested half an hour, into a centimetre of smoking oil or dripping: the hot fat is what makes them rise."))])), (py_str("roast-vegetables"), py_dict(vec![(py_str("ahead"), py_str("Chop them the night before and keep them in a bowl of cold water in the fridge; drain, oil and roast them as usual.")), (py_str("names"), py_tuple(vec![py_tuple(vec![py_str("roast vegetables")]), py_tuple(vec![py_str("roast veg")]), py_tuple(vec![py_str("roasted vegetables")])])), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("30 to 40 minutes, turning once, with 20 minutes for softer veg like peppers.")), (py_str("done"), py_str("The edges are browned and a knife goes in easily.")), (py_str("keep"), py_str("They keep 4 days in the fridge and are good cold in a salad or a wrap.")), (py_str("fill"), py_str("Oil, salt and pepper is the base; add whole garlic cloves, rosemary or thyme, and a spoon of honey or balsamic for the last 10 minutes."))])), (py_str("spag-bol"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("spag bol")]), py_tuple(vec![py_str("spaghetti bolognese")]), py_tuple(vec![py_str("spaghetti bolognaise")]), py_tuple(vec![py_str("bolognese")]), py_tuple(vec![py_str("bolognaise")]), py_tuple(vec![py_str("bol")])])), (py_str("temp"), py_str("Medium heat on the hob: the mince browns in a dry hot pan before the sauce goes in.")), (py_str("time"), py_str("an hour start to finish, and the sauce is worth the last half of it at a bare simmer with the lid off. Thirty minutes is edible, an hour is good.")), (py_str("done"), py_str("The sauce has darkened and thickened and the fat pools a little at the edge; season it with more salt than feels right and a pinch of sugar.")), (py_str("keep"), py_str("Better the next day. It keeps 3 days in the fridge and freezes for 3 months, so a big pot is worth it.")), (py_str("fill"), py_str("Onion, carrot and celery chopped small and softened first, then mince, tomato puree, tinned tomatoes and a splash of milk or wine; a bay leaf and a pinch of sugar finish it.")), (py_str("sides"), py_str("Garlic bread and a green salad, and keep a mug of the pasta water back to loosen the sauce with."))])), (py_str("lasagne"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("lasagne")]), py_tuple(vec![py_str("lasagna")]), py_tuple(vec![py_str("lasagne verdi")])])), (py_str("ahead"), py_str("Assemble it the day before, keep it covered in the fridge and add 5 to 10 minutes in the oven from cold; the ragu alone can be made two days early.")), (py_str("temp"), py_str("180C, or 160C in a fan oven, on the middle shelf.")), (py_str("time"), py_str("35 to 40 minutes in the oven, and the ragu wants 30 to 40 minutes on the hob before you build it.")), (py_str("rest"), py_str("10 to 15 minutes out of the oven, or it slumps when you cut it.")), (py_str("done"), py_str("Bubbling at the edges with the top browned, and a knife through the middle meets no resistance from the pasta; rest it so it cuts cleanly.")), (py_str("keep"), py_str("It keeps 3 days in the fridge and freezes in portions for 3 months: reheat covered at 180C for 25 minutes.")), (py_str("fill"), py_str("A ragu of mince, onion, carrot and tomato cooked down for at least an hour, a bechamel, and parmesan between and on top: the rest is layering.")), (py_str("sides"), py_str("Garlic bread and a green salad with a sharp dressing, because the salad is what cuts through the sauce: tear the bread, rub it with garlic, oil and salt and give it 8 minutes in a hot oven while the pasta cooks."))])), (py_str("stew"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("beef stew")]), py_tuple(vec![py_str("stew")]), py_tuple(vec![py_str("casserole")]), py_tuple(vec![py_str("hotpot")]), py_tuple(vec![py_str("hot pot")])])), (py_str("ahead"), py_str("It is better the next day, so make it the day before and reheat it gently with a splash of water; it keeps 3 days in the fridge and freezes well.")), (py_str("avoid"), py_tuple(vec![py_str("apple"), py_str("apples"), py_str("fruit"), py_str("prune"), py_str("prunes"), py_str("rhubarb"), py_str("plum"), py_str("plums"), py_str("pear"), py_str("pears"), py_str("berry"), py_str("berries")])), (py_str("temp"), py_str("150C (130C fan) in the oven with the lid on, or the lowest ring on the hob.")), (py_str("time"), py_str("two to two and a half hours at a bare simmer, and longer at a lower heat always beats hotter and shorter.")), (py_str("done"), py_str("The beef pulls apart with a fork and the liquid has thickened; if it is still tight, another 30 minutes will not hurt it.")), (py_str("keep"), py_str("Better the next day, and it keeps 3 days in the fridge or 3 months frozen.")), (py_str("fill"), py_str("Onion, carrot, celery and a spoon of tomato puree; add potatoes or dumplings if you want it to be the whole meal, and a splash of Worcestershire or stout for depth.")), (py_str("sides"), py_str("Mash, or dumplings dropped in for the last 25 minutes with the lid on."))])), (py_str("chilli"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("chilli con carne")]), py_tuple(vec![py_str("chilli")]), py_tuple(vec![py_str("chili")]), py_tuple(vec![py_str("chile con carne")])])), (py_str("avoid"), py_tuple(vec![py_str("powder"), py_str("flakes"), py_str("flake"), py_str("sauce"), py_str("oil"), py_str("plant"), py_str("plants"), py_str("seeds"), py_str("seed"), py_str("grow"), py_str("growing"), py_str("grew")])), (py_str("temp"), py_str("Medium heat on the hob, or 160C (140C fan) in the oven with the lid on.")), (py_str("time"), py_str("45 minutes to an hour from scratch, and better for longer: brown the mince, then soften the onion and pepper and let the tomatoes and beans go slowly.")), (py_str("done"), py_str("Darkened and thickened and the fat pools at the edge: taste it and add salt, then a square of dark chocolate if it tastes sharp.")), (py_str("keep"), py_str("It keeps 3 days in the fridge and freezes for 3 months, and it is better the next day.")), (py_str("fill"), py_str("Mince (or beans and mushrooms), onion, garlic, cumin, smoked paprika and chilli, a tin of tomatoes and a tin of kidney beans; a square of dark chocolate at the end rounds it off.")), (py_str("sides"), py_str("Rice, or tortillas and a bowl of grated cheese and sour cream on the table."))])), (py_str("fish-pie"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("fish pie")]), py_tuple(vec![py_str("fishpie")])])), (py_str("temp"), py_str("180C, or 160C in a fan oven.")), (py_str("time"), py_str("35 to 40 minutes at 180C from cold, or 25 minutes if the filling is still warm.")), (py_str("rest"), py_str("10 minutes out of the oven, so the sauce settles and it does not run.")), (py_str("done"), py_str("The sauce bubbles up at the edges and the top is crisp; a knife in the middle comes out hot.")), (py_str("keep"), py_str("It keeps 3 to 4 days in the fridge, covered, and reheats at 180C for 20 minutes or in portions in the microwave. It freezes well for 3 months: freeze it unbaked, then cook it from frozen at 180C for about an hour, or thaw it overnight and give it the usual 35 to 40 minutes.")), (py_str("fill"), py_str("Fish (smoked haddock and a white fish is the classic), a white sauce made with the milk the fish was poached in, and mashed potato on top with a little mustard and cheese.")), (py_str("sides"), py_str("New potatoes and peas, or rice with greens wilted in a pan with butter. A squeeze of lemon and yoghurt with chopped dill is all the sauce it needs, and both take less time than the fish."))])), (py_str("pasta-bake"), py_dict(vec![(py_str("names"), py_tuple(vec![py_tuple(vec![py_str("pasta bake")]), py_tuple(vec![py_str("baked pasta")]), py_tuple(vec![py_str("tuna bake")]), py_tuple(vec![py_str("macaroni cheese bake")])])), (py_str("ahead"), py_str("Boil the pasta and make the sauce the day before, mix and cover it in the fridge, then bake it from cold with 10 minutes extra; baked and cooled it reheats covered at 180C for 20 minutes.")), (py_str("temp"), py_str("200C, or 180C in a fan oven.")), (py_str("time"), py_str("25 to 30 minutes at 200C, or 20 minutes if the pasta and sauce are still hot.")), (py_str("done"), py_str("The cheese is browned and the sauce bubbles at the edges; cook the pasta 2 minutes short of the packet first, or it goes soft in the oven.")), (py_str("keep"), py_str("It keeps 3 days in the fridge and reheats covered at 180C for 20 minutes.")), (py_str("fill"), py_str("A tomato sauce with garlic and basil, cooked pasta, and mozzarella or cheddar on top; add bacon, courgette or spinach to the sauce if you have them.")), (py_str("sides"), py_str("Garlic bread, rubbed with a cut clove and oiled, or a green salad with a sharp dressing: the bake is rich, so something crisp and acidic beside it is the whole job."))])), (py_str("sunday-roast"), py_dict(vec![(py_str("ahead"), py_str("Prep the night before: parboil and rough up the potatoes and keep them in the fridge, and peel the veg into a bowl of cold water. The meat goes in on the day, because that is the part that wants the oven.")), (py_str("names"), py_tuple(vec![py_tuple(vec![py_str("sunday roast")]), py_tuple(vec![py_str("roast dinner")]), py_tuple(vec![py_str("roast lunch")]), py_tuple(vec![py_str("a roast")])])), (py_str("avoid"), __data5()), (py_str("temp"), py_str("190C (170C fan) for a joint or a bird, and the potatoes go up to 200C once the meat is out and resting.")), (py_str("time"), py_str("about 1 hour 40 at 190C for a 2kg joint or bird.")), (py_str("rest"), py_str("15 to 20 minutes under foil while the potatoes finish and the gravy is made, which is what makes the timings work.")), (py_str("done"), py_str("74C in the thickest part of a chicken thigh, or 60C for beef you want pink.")), (py_str("keep"), py_str("Leftovers keep 3 days, and the bones make the gravy or a stock.")), (py_str("fill"), py_str("A joint, potatoes parboiled and roasted in hot fat, and two veg: carrots or parsnips in the oven, greens on the hob. Gravy from the tin juices.")), (py_str("sides"), py_str("Parboil the potatoes 8 minutes, shake them in the pan with hot fat and roast 40 to 45 minutes at 200C; greens take 5 minutes at the end, and the gravy wants the roasting tin on the hob with a spoon of flour and the stock."))]))]) }
    s_PLANS(__data3());
    s_READY_NOTES(py_dict(vec![(py_str("rice"), py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("long grain"), py_str("jasmine rice"), py_str("pilau"), py_str("arborio"), py_str("risotto rice"), py_str("brown rice")]), py_str("When the water has gone and the grains are tender with a little bite at the centre: taste one rather than watching the clock, and if it is still chalky add a splash of water and give it 3 more minutes with the lid on, then leave it off the heat with the lid on for 5 minutes.")])), (py_str("pasta"), py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("tagliatelle"), py_str("rigatoni"), py_str("noodles")]), py_str("Fish a piece out and bite it: tender all the way through with no chalky core and still a little firm in the middle. Start tasting 2 minutes before the packet time, and dip a mug of the cooking water out before you drain it, because it is what loosens the sauce.")])), (py_str("egg"), py_tuple(vec![py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("boiled egg"), py_str("boiled eggs"), py_str("soft boiled egg"), py_str("hard boiled egg")]), py_str("It depends what you want: 4 minutes for a runny yolk, 6 for a soft one and 8 to 9 for a firm one, counted from the moment they go into water that is already boiling, then into cold water so they stop cooking.")])), (py_str("poached egg"), py_tuple(vec![py_tuple(vec![py_str("poached egg"), py_str("poached eggs"), py_str("poach"), py_str("poaching"), py_str("poached")]), py_str("A poached egg is done when the white is opaque and set and the yolk still gives when you press it lightly: 3 minutes in barely simmering water for a runny one, 4 for a set white. Lift it out with a slotted spoon and drain it on kitchen paper.")])), (py_str("scrambled egg"), py_tuple(vec![py_tuple(vec![py_str("scrambled egg"), py_str("scrambled eggs"), py_str("scramble"), py_str("scrambled"), py_str("scrambling")]), py_str("Scrambled eggs are done when they are just set and still glossy: take them off the heat while they look a little wetter than you want, because they go on cooking in the pan and on the plate. Low heat and slow stirring is what keeps them soft.")])), (py_str("fried egg"), py_tuple(vec![py_tuple(vec![py_str("fried egg"), py_str("fried eggs"), py_str("fry an egg"), py_str("frying an egg"), py_str("sunny side up")]), py_str("A fried egg is done when the white is fully set with no clear, wobbly patches and the yolk is as firm as you like it: about 3 minutes on a low-medium heat with the lid on for a soft yolk, 4 to 5 without.")])), (py_str("porridge"), py_tuple(vec![py_tuple(vec![py_str("porridge"), py_str("oats"), py_str("oatmeal"), py_str("porridge oats")]), py_str("When the oats have collapsed into the milk and it coats the back of the spoon. Take it off while it is still a little looser than you want it, because it thickens as it sits in the bowl.")])), (py_str("omelette"), py_tuple(vec![py_tuple(vec![py_str("omelette"), py_str("omelet"), py_str("omelettes"), py_str("omelets")]), py_str("An omelette is done when the top is just set and the middle still has a slight wobble: fold it while the centre is a little soft, because it finishes on the plate. Two or three minutes on a medium heat, and off the pan before it browns.")])), (py_str("lentils"), py_tuple(vec![py_tuple(vec![py_str("lentils"), py_str("lentil"), py_str("dal"), py_str("dhal")]), py_str("Taste one: a lentil is done when it gives under the tooth with no chalky middle. Green and brown take 20 to 25 minutes, red split lentils 15 to 20 and go mushy on purpose.")])), (py_str("soup"), py_tuple(vec![py_tuple(vec![py_str("soup"), py_str("broth"), py_str("bisque")]), py_str("When the hardest thing in it - a carrot, a potato, a lentil - gives to a knife with no resistance, and it has had 15 minutes after that for the flavours to come together. Season at the end, not the start.")]))]));
    s_READY_NAMES(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_READY_NOTES(), "values", &[])) { let __unpk4293 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk4293[0].clone(); let mut _u: Py = __unpk4293[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIDES(py_dict(vec![(py_str("rice"), py_str("Rice: 1 part rice to 1.5 parts water, 12 minutes with the lid on, then 10 minutes off the heat without lifting the lid.")), (py_str("potatoes"), py_str("Potatoes: boil them for 15 to 20 minutes, or roast them at 200C for 40 to 45 minutes if you have the oven on anyway.")), (py_str("salad"), py_str("A bag of leaves with a squeeze of lemon and a little oil is enough; dress it at the table so it does not go limp.")), (py_str("bread"), py_str("Warm bread or a naan straight from the oven, no need for anything clever.")), (py_str("vegetables"), py_str("Green vegetables: broccoli or beans steamed for 4 to 5 minutes, while everything else is on the table.")), (py_str("gravy"), py_str("Gravy: whisk a teaspoon of flour into the meat juices off the heat, then add stock and simmer it for 5 minutes, stirring the brown bits up. If it goes lumpy, push it through a sieve."))]));
    s_SIDE_WORDS(py_dict(vec![(py_str("rice"), py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("pilau"), py_str("pilaf")])), (py_str("potatoes"), py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("mash"), py_str("chips"), py_str("roasties")])), (py_str("salad"), py_tuple(vec![py_str("salad"), py_str("slaw"), py_str("lettuce")])), (py_str("bread"), py_tuple(vec![py_str("bread"), py_str("naan"), py_str("pitta"), py_str("pita"), py_str("rolls"), py_str("garlic bread")])), (py_str("vegetables"), py_tuple(vec![py_str("veg"), py_str("vegetables"), py_str("greens"), py_str("broccoli"), py_str("beans"), py_str("peas"), py_str("carrots")])), (py_str("gravy"), py_tuple(vec![py_str("gravy"), py_str("jus")]))]));
    fn __data7() -> Py { py_tuple(vec![py_str("how do i stop"), py_str("how can i stop"), py_str("how to stop"), py_str("how do i fix"), py_str("how can i fix"), py_str("how to fix"), py_str("what am i doing wrong"), py_str("what went wrong"), py_str("why does it keep"), py_str("why does my"), py_str("why is my"), py_str("why do my"), py_str("how do i make it not"), py_str("how do i get it to"), py_str("what can i add"), py_str("what can i do about")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("always"), py_str("keeps"), py_str("keep"), py_str("going"), py_str("come"), py_str("comes"), py_str("coming"), py_str("out"), py_str("turn"), py_str("turns"), py_str("my"), py_str("the")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("oven"), py_str("take"), py_str("takes"), py_str("long"), py_str("time"), py_str("minutes"), py_str("hours"), py_str("cook"), py_str("roast"), py_str("bake"), py_str("kilo"), py_str("kilos"), py_str("kilogram"), py_str("kg"), py_str("total"), py_str("altogether"), py_str("should"), py_str("need"), py_str("about")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("how do i know when"), py_str("how do i know its done"), py_str("how do i know it is cooked"), py_str("how can i tell when"), py_str("how can i tell its done"), py_str("how can i tell it is done"), py_str("how can i tell its cooked"), py_str("how do i tell when"), py_str("how do i tell its done"), py_str("how do i tell it is done"), py_str("how do i tell its cooked"), py_str("how can i tell if it is done"), py_str("how can i tell if its done"), py_str("how can i tell if it is cooked"), py_str("how can i tell if its cooked"), py_str("how can i tell if it is ready"), py_str("how can i tell if its ready"), py_str("how do i tell if it is done"), py_str("how do i tell if its done"), py_str("how do i tell if it is cooked"), py_str("how do i tell if its cooked"), py_str("how can i tell if the"), py_str("how do i tell if the"), py_str("is it done"), py_str("when is it done"), py_str("ready yet"), py_str("is it cooked"), py_str("how do i check if"), py_str("how do i check when"), py_str("how can i check if"), py_str("how can i check when"), py_str("how do i check its done"), py_str("how do i check it is done"), py_str("how will i know"), py_str("how do i test if"), py_str("how do i test whether"), py_str("how do i test"), py_str("how can i test"), py_str("how do you test"), py_str("how should i test")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("it"), py_str("the"), py_str("make"), py_str("made"), py_str("can"), py_str("could"), py_str("should"), py_str("day"), py_str("before"), py_str("ahead"), py_str("keep"), py_str("keeps"), py_str("stays"), py_str("ok"), py_str("make")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("can i still use it"), py_str("can i still use that"), py_str("can i still eat it"), py_str("is it still ok"), py_str("is it still good"), py_str("is it still all right"), py_str("is it still alright"), py_str("how long will it keep"), py_str("how long does it keep"), py_str("how long do they keep"), py_str("how long will they keep"), py_str("will it keep"), py_str("does it keep"), py_str("can i keep"), py_str("can i store"), py_str("how do i store"), py_str("how should i store"), py_str("how long does it last"), py_str("how long will it last"), py_str("can i freeze"), py_str("does it freeze"), py_str("can it be frozen"), py_str("how do i reheat"), py_str("what about the leftovers"), py_str("what do i do with the leftovers"), py_str("keep fresh"), py_str("keep it fresh"), py_str("keeps fresh"), py_str("stay fresh"), py_str("stays fresh"), py_str("keep them fresh"), py_str("keep mine fresh"), py_str("from going stale"), py_str("stop it going stale"), py_str("going stale"), py_str("goes stale"), py_str("goes hard"), py_str("going hard")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("long"), py_str("it"), py_str("that"), py_str("they"), py_str("days"), py_str("day"), py_str("ok"), py_str("safe"), py_str("again"), py_str("much"), py_str("fridge"), py_str("freezer"), py_str("leftover"), py_str("leftovers")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("what should i have with it"), py_str("what should i have with that"), py_str("what can i have with it"), py_str("what to have with it"), py_str("anything with it"), py_str("what goes with it"), py_str("what side"), py_str("which side"), py_str("any sides"), py_str("what sides"), py_str("what to serve with it"), py_str("serve with it"), py_str("what about the side")]) }
    fn __data6() -> Py { py_tuple(vec![py_tuple(vec![py_str("fix"), __data7(), __data8()]), py_tuple(vec![py_str("temp"), py_tuple(vec![py_str("temperature"), py_str("temp"), py_str("degrees c"), py_str("how many degrees"), py_str("what degrees"), py_str("gas mark"), py_str("how hot"), py_str("how warm"), py_str("what heat"), py_str("which shelf")]), py_tuple(vec![py_str("oven"), py_str("fan"), py_str("bake"), py_str("roast"), py_str("cook"), py_str("hot"), py_str("heat"), py_str("setting")])]), py_tuple(vec![py_str("time"), py_tuple(vec![py_str("how long"), py_str("how many minutes"), py_str("how many hours"), py_str("how much time"), py_str("what time"), py_str("how long will"), py_str("long enough"), py_str("per kilo"), py_str("per kg")]), __data9()]), py_tuple(vec![py_str("done"), __data10(), py_tuple(vec![py_str("done"), py_str("cooked"), py_str("ready"), py_str("check"), py_str("tell"), py_str("test"), py_str("thermometer"), py_str("juice"), py_str("juices"), py_str("it"), py_str("is"), py_str("when")])]), py_tuple(vec![py_str("skin"), py_tuple(vec![py_str("skin"), py_str("skins"), py_str("skinless")]), py_tuple(vec![py_str("off"), py_str("on"), py_str("need"), py_str("take"), py_str("leave"), py_str("should"), py_str("do"), py_str("remove"), py_str("keep"), py_str("it")])]), py_tuple(vec![py_str("stuffing"), py_tuple(vec![py_str("stuff"), py_str("stuffing"), py_str("stuffed")]), py_tuple(vec![py_str("it"), py_str("should"), py_str("can"), py_str("need"), py_str("do"), py_str("if"), py_str("and"), py_str("chicken"), py_str("the")])]), py_tuple(vec![py_str("frosting"), py_tuple(vec![py_str("frosting"), py_str("icing"), py_str("buttercream"), py_str("ganache"), py_str("filling")]), __data11()]), py_tuple(vec![py_str("ahead"), py_tuple(vec![py_str("ahead"), py_str("ahead of time"), py_str("in advance"), py_str("day before"), py_str("night before"), py_str("make it earlier"), py_str("prepare it"), py_str("get it ready")]), py_tuple(vec![py_str("make"), py_str("made"), py_str("prepare"), py_str("bake"), py_str("cook"), py_str("can"), py_str("could"), py_str("should"), py_str("it"), py_str("that")])]), py_tuple(vec![py_str("keep"), __data12(), __data13()]), py_tuple(vec![py_str("rest"), py_tuple(vec![py_str("rest"), py_str("resting"), py_str("rested")]), py_tuple(vec![py_str("it"), py_str("should"), py_str("long"), py_str("how"), py_str("need"), py_str("do"), py_str("before"), py_str("then")])]), py_tuple(vec![py_str("allergy"), py_tuple(vec![py_str("allergic"), py_str("allergy"), py_str("allergies"), py_str("nut"), py_str("nuts"), py_str("intolerance")]), py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("son"), py_str("daughter"), py_str("child"), py_str("is"), py_str("has"), py_str("to"), py_str("what"), py_str("can")])]), py_tuple(vec![py_str("sides"), __data14(), py_tuple(vec![py_str("it"), py_str("that"), py_str("with"), py_str("go"), py_str("goes"), py_str("have"), py_str("side"), py_str("sides"), py_str("serve"), py_str("dish"), py_str("on")])]), py_tuple(vec![py_str("fill"), py_tuple(vec![py_str("what should i put in"), py_str("what do i put in"), py_str("what goes in"), py_str("what shall i put in"), py_str("what to put in"), py_str("ingredients"), py_str("what do i need in it"), py_str("what should go in")]), py_tuple(vec![py_str("put"), py_str("in"), py_str("goes"), py_str("go"), py_str("inside"), py_str("need"), py_str("add"), py_str("with")])])]) }
    s_ASK_KINDS(__data6());
    fn __data15() -> Py { py_tuple(vec![py_str("breast"), py_str("breasts"), py_str("thigh"), py_str("thighs"), py_str("leg"), py_str("legs"), py_str("wing"), py_str("wings"), py_str("fillet"), py_str("fillets"), py_str("drumstick"), py_str("drumsticks"), py_str("supreme"), py_str("supremes")]) }
    s_CUT_OFF(__data15());
    s__A_MADE_DISH(py_tuple(vec![py_str("soup"), py_str("soups"), py_str("broth"), py_str("broths")]));
    fn __data16() -> Py { py_set(vec![py_str("no"), py_str("not"), py_str("never"), py_str("isnt"), py_str("arent"), py_str("wasnt"), py_str("werent"), py_str("dont"), py_str("doesnt"), py_str("didnt"), py_str("cant"), py_str("cannot"), py_str("wont"), py_str("without"), py_str("except"), py_str("excepting"), py_str("instead"), py_str("apart"), py_str("but"), py_str("other"), py_str("rather"), py_str("sick"), py_str("bored"), py_str("tired"), py_str("hate"), py_str("hates"), py_str("dislike"), py_str("loathe"), py_str("skip"), py_str("avoid"), py_str("anything"), py_str("nothing")]) }
    s__RULE_OUT(__data16());
    fn __data17() -> Py { py_set(vec![py_str("a"), py_str("an"), py_str("the"), py_str("any"), py_str("more"), py_str("another"), py_str("one"), py_str("of"), py_str("to"), py_str("do"), py_str("doing"), py_str("make"), py_str("making"), py_str("cook"), py_str("cooking"), py_str("have"), py_str("having"), py_str("want"), py_str("wanting"), py_str("that"), py_str("this"), py_str("it"), py_str("its"), py_str("my"), py_str("our"), py_str("us"), py_str("me"), py_str("them"), py_str("for"), py_str("with"), py_str("be"), py_str("being"), py_str("even")]) }
    s__RULE_OUT_FILLER(__data17());
    fn __data18() -> Py { py_set(vec![py_str("paste"), py_str("powder"), py_str("sauce"), py_str("stock"), py_str("cube"), py_str("cubes"), py_str("tin"), py_str("tins"), py_str("jar"), py_str("jars"), py_str("packet"), py_str("packets"), py_str("pack"), py_str("packs"), py_str("sachet"), py_str("sachets"), py_str("oil"), py_str("puree"), py_str("mix"), py_str("seasoning"), py_str("leaves"), py_str("seeds"), py_str("granules"), py_str("concentrate"), py_str("flavouring"), py_str("flavourings"), py_str("essence"), py_str("syrup")]) }
    s__INGREDIENT_AFTER(__data18());
    fn __data19() -> Py { py_tuple(vec![py_str("making"), py_str("doing"), py_str("cooking"), py_str("roasting"), py_str("baking"), py_str("starting"), py_str("having"), py_str("im going to"), py_str("i'm going to"), py_str("we're going to"), py_str("we are going to"), py_str("going to cook"), py_str("going to make"), py_str("going to do"), py_str("fancy"), py_str("fancying"), py_str("thinking of"), py_str("thinking about"), py_str("thought about"), py_str("considering"), py_str("considered"), py_str("feel like"), py_str("feeling like"), py_str("in the mood for"), py_str("might do"), py_str("might make"), py_str("might cook"), py_str("planning"), py_str("planning on"), py_str("plan to")]) }
    s__PLAN_MARKER(__data19());
    fn __data20() -> Py { py_tuple(vec![py_str("what should i cook"), py_str("what can i cook"), py_str("what shall i cook"), py_str("what do i cook"), py_str("what to cook"), py_str("what should i make"), py_str("what can i make"), py_str("what shall i make"), py_str("what else"), py_str("what can i eat"), py_str("what should i eat"), py_str("what shall i eat"), py_str("what do i eat"), py_str("what to eat"), py_str("whats for"), py_str("what's for"), py_str("what is for"), py_str("what can i have"), py_str("what should i have"), py_str("what shall i have"), py_str("what to have"), py_str("what can we have"), py_str("ideas"), py_str("idea"), py_str("suggest"), py_str("recommend"), py_str("options"), py_str("something to cook"), py_str("something quick"), py_str("something easy"), py_str("meal plan"), py_str("meal ideas"), py_str("dinner ideas"), py_str("bored of"), py_str("bored with"), py_str("sick of"), py_str("tired of"), py_str("fed up"), py_str("had enough of"), py_str("instead of"), py_str("any more"), py_str("other than"), py_str("another"), py_str("different"), py_str("impress"), py_str("in laws"), py_str("in-laws"), py_str("new recipe"), py_str("any good recipes")]) }
    s__IDEA_ASK(__data20());
    s_PART_OF(py_dict(vec![(py_str("frosting"), py_str("cake")), (py_str("icing"), py_str("cake")), (py_str("buttercream"), py_str("cake")), (py_str("ganache"), py_str("cake"))]));
    fn __data21() -> Py { py_tuple(vec![py_str("ahead"), py_str("ahead of time"), py_str("in advance"), py_str("day before"), py_str("night before"), py_str("make it earlier"), py_str("prepare it"), py_str("get it ready"), py_str("how long will it keep"), py_str("how long does it keep"), py_str("will it keep"), py_str("keep for"), py_str("keeps for"), py_str("can i keep"), py_str("can i store"), py_str("how do i store"), py_str("how long does it last"), py_str("how long will it last"), py_str("can i freeze"), py_str("does it freeze"), py_str("can it be frozen")]) }
    s_IMPLIED_ASKS(__data21());
    s_PART_NOT(py_tuple(vec![py_str("icing sugar"), py_str("sugar paste"), py_str("frosting mix"), py_str("buttercream powder")]));
    fn __data22() -> Py { py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes"), py_str("rice"), py_str("porridge"), py_str("oats"), py_str("oatmeal"), py_str("couscous"), py_str("polenta"), py_str("noodles"), py_str("pasta"), py_str("dumplings"), py_str("gnocchi")]) }
    s__STARCH(__data22());
    fn __data23() -> Py { py_tuple(vec![py_str("soup"), py_str("soups"), py_str("stew"), py_str("stews"), py_str("sauce"), py_str("sauces"), py_str("curry"), py_str("curries"), py_str("casserole"), py_str("casseroles"), py_str("gravy"), py_str("chilli"), py_str("broth"), py_str("daal"), py_str("dhal")]) }
    s__POT(__data23());
    s__GLUE(py_tuple(vec![py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes")]), py_tuple(vec![py_str("pasta"), py_str("penne"), py_str("spaghetti"), py_str("rigatoni"), py_str("macaroni"), py_str("fusilli"), py_str("tagliatelle"), py_str("farfalla")]), py_tuple(vec![py_str("noodle"), py_str("noodles"), py_str("udon"), py_str("ramen"), py_str("rice noodle"), py_str("rice noodles")]), py_tuple(vec![py_str("couscous"), py_str("bulgur"), py_str("bulghur"), py_str("quinoa"), py_str("polenta")])]));
    s__MUSHY_WORDS(py_tuple(vec![py_str("mushy"), py_str("soggy"), py_str("overcooked"), py_str("over cooked"), py_str("over-cooked"), py_str("squashy")]));
    s__MUSHY_FIX(py_dict(vec![(py_str("risotto"), py_str("Risotto goes to porridge when it takes too much stock or too much stirring: add the stock a ladleful at a time and stir only enough to stop it catching, and take it off while the grains still have a chalky line through the middle. If it has already gone soft, beat in a knob of cold butter and the parmesan off the heat and serve it straight away - it will not come back, but it will taste right.")), (py_str("mash"), py_str("Mushy potatoes were boiled too long, or they are a waxy salad potato that goes to paste: use a floury one like Maris Piper or King Edward, drain the water the moment a knife slides in, let the steam off for a minute, and mash with warm milk and butter. Potatoes already sitting in water keep softening, so drain them as soon as they are done.")), (py_str("rice"), py_str("Soggy rice took too much water, not too little: 1 part rice to 1.5 parts water, the lid on and no stirring at all, then take it off the heat and let it stand 5 minutes with the lid on so the last of the water is taken up. Long grain and basmati want less water than pudding rice, and rinsing the rice first stops the outside going pasty.")), (py_str("grain"), py_str("Soggy couscous or quinoa took too much water: pour boiling water or stock over it in a wide bowl, 1 part to 1.5 for couscous and 1 to 2 for quinoa and bulgur, cover it and leave it 5 minutes, then fork it through. Polenta wants 1 part to 4 of liquid, stirred for 5 minutes and then left to set.")), (py_str("porridge"), py_str("Mushy porridge is too much water and too long in the pan: use 1 part oats to 2.5 parts liquid for rolled oats, stir it now and then, and take it off while it is still a little looser than you want it, because it thickens as it stands. Never drain porridge: loosen it with a splash more milk or water.")), (py_str("bake"), py_str("Mushy is the wrong word for a bake, but it means the same thing: too much liquid or not long enough in the oven. Put it back at 160C (140C fan) for 10 minutes with foil loosely over the top so the middle catches up without the crust burning, and test it with a skewer - damp crumbs, not wet mixture. Bread that is doughy in the middle wants 10 more minutes at 200C and a tap on the base for a hollow sound.")), (py_str("sauce"), py_str("A mushy sauce has too much water in it, not too little: take the lid off, keep it at a gentle simmer and let it reduce until it coats the back of a spoon, then taste it for salt, because reducing concentrates that too. A soup that was meant to be thick is better finished with a handful of rice or a spoon of flour slaked in cold water than with more time on the heat.")), (py_str(""), py_str("Mushy means too much water and too long in it, whatever the food is: take it off the heat while it still has some bite and get it out of the water rather than letting it sit. Tell me what it is - a bake, a grain, a sauce - and I can be exact about the next step."))]));
    s__TOUGH_BAKE(py_str("A tough or chewy bake has been mixed too much or has been in a little too long: stir only until the flour disappears and fold rather than beat, and weigh the butter and sugar rather than eyeballing them. If it is the oven, take the next batch out 2 minutes earlier - biscuits and meringues go chewy as they dry out, and a bake that is already tough is better eaten warm with butter or cream."));
    s__DRY_BAKE(py_str("A dry bake has been in the oven too long, or has more flour in it than the recipe meant: ovens run 10 to 20C hot or cold, so check it 5 minutes before the time and take it out while the middle is still a shade soft, and weigh the flour rather than using a cup. One that has already dried out is better eaten warm with butter, cream or fruit than thrown away."));
    s__GRAINY(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes")]), py_str("grainy-mash")]), py_tuple(vec![py_tuple(vec![py_str("hummus"), py_str("houmous"), py_str("humus"), py_str("dip"), py_str("dips"), py_str("smoothie"), py_str("pesto"), py_str("soup"), py_str("soups")]), py_str("grainy-blend")]), py_tuple(vec![py_tuple(vec![py_str("custard"), py_str("cream"), py_str("creme fraiche"), py_str("yoghurt"), py_str("yogurt"), py_str("hollandaise"), py_str("mayonnaise"), py_str("mayo")]), py_str("grainy-curdle")]), py_tuple(vec![py_tuple(vec![py_str("polenta"), py_str("couscous"), py_str("porridge"), py_str("oats"), py_str("oatmeal"), py_str("semolina"), py_str("grits")]), py_str("grainy-grain")])]));
    s__GRAINY_FIX(py_dict(vec![(py_str("grainy-mash"), py_str("Grainy mash is the potato and the tool, not the butter: a floury potato mashed while it is cold, or a masher that works it into paste. Drain them the moment a knife slides in, let the steam off for a minute, put them through a ricer or a sieve rather than mashing hard, and beat in warm milk and butter off the heat. Once it is grainy, warm milk and a good beat will loosen it, but a food processor will only make it worse.")), (py_str("grainy-blend"), py_str("A grainy blend stopped too soon or has too little fat in it: blend it 3 to 4 minutes rather than 30 seconds, scrape the sides down twice, and add the oil or tahini in a thin stream while the motor runs. A spoon of iced water loosens a thick hummus far better than more oil. Chickpea skins and tomato skins are the other half of it, so push it through a sieve if it still feels rough.")), (py_str("grainy-curdle"), py_str("Grainy custard, cream or mayonnaise has curdled: the eggs or the fat got too hot, or acid went in too fast. Everything you make this way wants low heat and a bowl over simmering water, not a pan on the ring. To rescue a curdled one, take it off the heat and whisk in a spoonful of cold milk or cream it has not split yet - and a graininess you can taste rather than see is usually salt, sugar or an over-tight emulsion, not heat.")), (py_str("grainy-grain"), py_str("Grainy polenta, couscous or porridge is lumps of dry grain that never took up water: rain it into boiling liquid in a thin stream while you whisk, instead of tipping it in and stirring after. Polenta wants 1 part to 4 of liquid and 5 minutes of stirring; couscous wants boiling water poured over it, a lid and a fork, never a spoon. Any lumps left at the end go through a sieve or get whisked out with a little more hot liquid.")), (py_str(""), py_str("Grainy means the texture broke up instead of holding together, and what to do depends on what broke: meat that steamed instead of browning wants a hotter pan and less stirring, mashed or blended food wants a gentler tool and more fat, and eggs or cream that went grainy have curdled and are already cooked. Tell me which dish it is and I can be exact."))]));
    s__LUMPY(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("polenta"), py_str("couscous"), py_str("porridge"), py_str("oats"), py_str("oatmeal"), py_str("semolina"), py_str("grits"), py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes")]), py_str("lumpy-starch")]), py_tuple(vec![py_tuple(vec![py_str("custard"), py_str("cream"), py_str("sauce"), py_str("sauces"), py_str("gravy"), py_str("roux"), py_str("cheese sauce"), py_str("white sauce"), py_str("batter"), py_str("pancakes"), py_str("pancake"), py_str("yorkshire")]), py_str("lumpy-liquid")]), py_tuple(vec![py_tuple(vec![py_str("hummus"), py_str("houmous"), py_str("humus"), py_str("dip"), py_str("dips"), py_str("pesto"), py_str("smoothie")]), py_str("lumpy-blend")])]));
    s__LUMPY_FIX(py_dict(vec![(py_str("lumpy-starch"), py_str("Lumps in porridge, polenta, couscous or mash are dry grain or dry potato that never took up liquid: tip the grain into the liquid in a thin stream while you whisk, rather than adding the liquid to the grain, and stir the bottom of the pan as it comes up to the boil. A mash goes lumpy when it is mashed cold or with thin liquid: warm it through with a splash of hot milk and put it through a sieve, which takes out every lump in one go.")), (py_str("lumpy-liquid"), py_str("A lumpy sauce is flour that cooked before it met the liquid, or liquid that went in faster than the whisk could follow: slake the flour or cornflour in cold water first, add the hot liquid in a thin stream, and keep the whisk flat on the bottom of the pan where the flour settles. Off the heat, a hard whisk and a sieve rescue a sauce that has already lumped - or strain it, put it back on the heat and start the thickening again with fresh roux, which is quicker than fighting the lumps.")), (py_str("lumpy-blend"), py_str("Lumps in a blend are too little liquid under the blade and too short a run: add the liquid first, blend 2 to 3 minutes and scrape down twice, and thin it with iced water rather than oil. Anything with skins in it - chickpeas, tomatoes, nuts - will stay rough until it goes through a sieve.")), (py_str(""), py_str("Lumpy food is dry powder or cold fat that met hot liquid too fast: mix the starch with cold water first, add it in a thin stream while you whisk, and keep the whisk flat on the bottom of the pan. Off the heat, whisk it hard and push it through a sieve. Tell me which dish it is and I can say what went wrong in that one."))]));
    s_PROBLEMS(py_dict(vec![(py_str("grainy"), py_tuple(vec![py_tuple(vec![py_str("grainy"), py_str("grainy texture"), py_str("gone grainy"), py_str("went grainy"), py_str("tastes grainy")]), py_func("_grainy", |args: &[Py]| _grainy(args)), py_dict(vec![(py_str("spag-bol"), py_str("Grainy bolognese is the mince rather than the tomato: mince that goes into a cool pan and is stirred at once steams in its own juice and breaks into little grey grains. Get the pan properly hot, brown it in two batches and leave the first minute alone before you stir, tip off the fat, then reduce a glass of wine and 100ml of milk in it before the tomatoes go in. A sauce that has already gone grainy comes back with a long bare simmer - 2 to 3 hours, lid off - and a sieve takes out the skin and seed if that is what is roughening it."))])])), (py_str("lumpy"), py_tuple(vec![py_tuple(vec![py_str("lumpy"), py_str("lumps"), py_str("all lumps")]), py_func("_lumpy", |args: &[Py]| _lumpy(args)), py_dict(vec![])])), (py_str("dry"), py_tuple(vec![py_tuple(vec![py_str("dry"), py_str("dries"), py_str("overcooked"), py_str("over cooked"), py_str("over-cooked"), py_str("too long"), py_str("ruined")]), py_str("Nearly always the time, not the temperature: take it out 5 minutes earlier than you think, cover it with foil and let it rest 10 minutes. It carries on cooking off the heat and the juices settle back into the meat."), dict_spread(&[py_dict(vec![(py_str("roast-chicken"), py_str("Cook it to 74C in the thickest part of the thigh and no further: cover the bird with foil once the skin is brown so the breast does not dry out while the thighs finish, and rest it 15 minutes."))]), py_dict(vec![(py_str("chicken-thighs"), py_str("Thighs dry out when they are skinless or crowded on the tray: keep the skin on, leave a gap between them, and take them out at 74C rather than giving them another ten minutes to be safe."))]), py_dict(vec![(py_str("chicken-breast"), py_str("Breasts are the easiest thing to overcook: flatten them to an even thickness with the heel of your hand, roast 18 to 20 minutes at 200C, and rest 5 minutes before slicing."))]), py_dict(vec![(py_str("salmon"), py_str("Salmon dries out in a hot oven: 12 to 15 minutes at 180C is enough for a fillet, and it is done when the white lines between the flakes just start to show. Take it out looking slightly underdone."))]), py_dict(vec![(py_str("cake"), py_str("A dry sponge means too long in the oven or too much flour: check 5 minutes early with a skewer, weigh the flour rather than using a cup, and soak the cake with a little syrup if it has already gone dry."))]), py_dict(vec![(py_str("sausages"), py_str("Sausages dry out when they are pricked or cooked too fast: keep them whole, cook them 25 to 30 minutes at 180C, and turn them once."))]), { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("scones"), py_str("muffins"), py_str("biscuits"), py_str("meringues"), py_str("yorkshire-puddings")])) { let mut bake: Py = __item.clone(); __out.push((bake.clone(), g__DRY_BAKE())); } py_dict(__out) })() }])])), (py_str("tough"), py_tuple(vec![py_tuple(vec![py_str("tough"), py_str("chewy"), py_str("leathery"), py_str("like shoe")]), py_str("Tough meat is usually cooked too fast or too briefly: give it longer at a lower heat with a splash of liquid, and slice it across the grain."), dict_spread(&[py_dict(vec![(py_str("steak"), py_str("Steak goes tough when it is cold in the pan or cooked past medium: take it out of the fridge 20 minutes before, dry it, sear it in a very hot pan 2 minutes a side, then rest it 5 minutes and slice across the grain."))]), py_dict(vec![(py_str("roast-chicken"), py_str("A bird cooked too fast goes tough outside and raw in: 190C for 20 minutes per 500g plus 20, then rest it 15 minutes before carving."))]), py_dict(vec![(py_str("stew"), py_str("A tough stew is an underdone one, not an overcooked one: another 30 to 45 minutes at a bare simmer with the lid on, and top the liquid up if it is low. A simmer that is really a boil tightens the meat instead of softening it."))]), { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("cake"), py_str("scones"), py_str("muffins"), py_str("biscuits"), py_str("meringues"), py_str("yorkshire-puddings")])) { let mut bake: Py = __item.clone(); __out.push((bake.clone(), g__TOUGH_BAKE())); } py_dict(__out) })() }])])), (py_str("soggy"), py_tuple(vec![py_tuple(vec![py_str("soggy"), py_str("limp"), py_str("not crisp"), py_str("not crispy"), py_str("wont crisp"), py_str("won't crisp")]), py_str("Soggy means too much water, and what to do depends on the food. Pasta, noodles and rice are simply overcooked: drain them the moment they are tender and get the sauce in straight away so they cannot sit wet. Roast things need one layer with space between them, the oil hot before they go in and the oven door shut. A bake that is soggy in the middle went in too wet or came out too soon: back it at 160C for 10 minutes under foil."), py_dict(vec![(py_str("cake"), py_str("A soggy cake is underbaked or wetter than the recipe meant: put it back at 160C for 10 minutes with foil over the top, and next time weigh the milk and butter and test with a skewer - it wants crumbs, not batter. A soggy bottom is the tin on a cold shelf: bake one shelf up, or slide a tray under it.")), (py_str("bread"), py_str("Soggy bread is underbaked or was cut while it was hot: stand the loaf out of its tin for another 10 minutes at 200C, and cool it on a rack, because a loaf cut warm steams itself and goes doughy.")), (py_str("pastry"), py_str("A soggy pastry base is a wet filling or an oven that was not hot enough: blind bake it 15 minutes at 200C with paper and beans, then brush the base with beaten egg and give it 5 minutes more.")), (py_str("biscuits"), py_str("Soggy biscuits are underbaked or were left in the tin: give them 2 more minutes at 180C, then 5 minutes in the tin and the rest on a rack - they firm as they cool.")), (py_str("roast-potatoes"), py_str("Soggy potatoes are usually parboiled too long or put in cold oil: 8 minutes in boiling water, drain and shake them rough, then into a thin layer of hot oil at 200C and do not crowd the tin.")), (py_str("roast-vegetables"), py_str("Wet vegetables steam instead of roasting: dry them, toss in oil, and spread them in a single layer with room between them."))])])), (py_str("dense"), py_tuple(vec![py_tuple(vec![py_str("dense"), py_str("heavy"), py_str("sank"), py_str("stodgy"), py_str("sink"), py_str("sinks"), py_str("sinking")]), py_func("_heavy", |args: &[Py]| _heavy(args)), py_dict(vec![(py_str("bread"), py_str("Dense bread is usually under-proved or over-kneaded: let the shaped loaf double somewhere warm, and use a thermometer on the water — yeast wants blood heat, not hot.")), (py_str("cake"), py_str("A cake sinks when the oven door is opened early or the raising agent is too much: bake at 180C without opening the door for the first 20 minutes, and check with a skewer at the earliest time in the recipe."))])])), (py_str("bland"), py_tuple(vec![py_tuple(vec![py_str("bland"), py_str("tasteless"), py_str("no flavour"), py_str("no flavor"), py_str("boring"), py_str("nothing taste")]), py_str("Bland food is nearly always short of salt and acid rather than spices: a good pinch of salt, a squeeze of lemon or a splash of vinegar lifts it more than another spoon of spice."), py_dict(vec![(py_str("curry"), py_str("Taste the curry for salt first, then add a squeeze of lemon or a spoon of yoghurt; if it still tastes thin, fry the spices in oil at the start for 30 seconds and let the sauce reduce uncovered for 5 minutes.")), (py_str("roast-vegetables"), py_str("Vegetables taste flat when they are crowded and pale: give them room, roast until the edges colour, and salt them as they come out of the oven."))])])), (py_str("sticky"), py_tuple(vec![py_tuple(vec![py_str("sticky"), py_str("gluey"), py_str("mushy"), py_str("claggy"), py_str("stodgy rice")]), py_func("_gluey", |args: &[Py]| _gluey(args)), py_dict(vec![(py_str("curry"), py_str("Rice under a curry goes sticky when it is stirred while it cooks: rinse it, bring it to a simmer, put the lid on and leave it alone for 12 minutes.")), (py_str("bread"), py_str("Sticky dough is nearly always wet dough: add strong white flour a spoonful at a time and knead it in for a minute before judging, because dough firms as it rests. Wet your hands rather than flouring them to shape it, and give it 20 minutes covered - a slack dough that has rested handles far better."))])])), (py_str("watery"), py_tuple(vec![py_tuple(vec![py_str("watery"), py_str("runny sauce"), py_str("too thin"), py_str("thin sauce"), py_str("sauce is runny")]), py_str("Watery sauce is a sauce that has not had time to reduce: simmer it uncovered for 5 to 10 minutes, or stir in a teaspoon of cornflour slaked in cold water."), py_dict(vec![(py_str("curry"), py_str("Simmer the curry uncovered for 10 minutes to reduce it, or stir in a spoon of tomato puree; a spoon of yoghurt at the end thickens it without flour.")), (py_str("fish-pie"), py_str("A watery fish pie is a filling that went on warm or thin: poach the fish in milk, use 40g each of butter and flour to 500ml of that milk, and let it cool right down before the mash goes on."))])]))]));
    fn __data24() -> Py { py_tuple(vec![py_str("cabbage"), py_str("sprouts"), py_str("sprout"), py_str("broccoli"), py_str("cauliflower"), py_str("romanesco"), py_str("kalettes"), py_str("spring greens"), py_str("leafy greens"), py_str("collard greens"), py_str("mixed greens"), py_str("collard"), py_str("pak choi"), py_str("bok choy"), py_str("broccolini"), py_str("savoy"), py_str("spring cabbage")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("aubergine"), py_str("aubergines"), py_str("courgette"), py_str("courgettes"), py_str("zucchini"), py_str("pepper"), py_str("peppers"), py_str("squash"), py_str("pumpkin"), py_str("fennel"), py_str("okra"), py_str("tomato"), py_str("tomatoes")]) }
    s_VEG_FAMILIES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("green beans"), py_str("runner beans"), py_str("broad beans"), py_str("french beans"), py_str("beans"), py_str("mangetout"), py_str("sugar snap"), py_str("sugar snaps"), py_str("peas")]), py_str("Beans go tough when they are old or undercooked: top and tail them, pull the string down the seam of runners, and boil 4 to 5 minutes in salted water. Peas and mangetout need 2 minutes at most, and a minute in a hot pan at the end brings the bite back.")]), py_tuple(vec![__data24(), py_str("Slice it thinner and give it longer: a brassica goes tough when the stalk is undercooked, so cut a cross in the stalk or slice it thin and steam 5 to 8 minutes, until a knife slides in. If it has turned grey and sulphury it is overcooked instead, so give it two minutes less next time.")]), py_tuple(vec![py_tuple(vec![py_str("carrot"), py_str("carrots"), py_str("parsnip"), py_str("parsnips"), py_str("swede"), py_str("turnip"), py_str("turnips"), py_str("beetroot"), py_str("celeriac"), py_str("sweet potato"), py_str("sweet potatoes"), py_str("jerusalem artichoke")]), py_str("Roots go hard in the middle when the pieces are uneven: cut them no bigger than a thumb, start them in cold salted water and simmer 15 to 20 minutes, or roast them covered in foil first. If the core is woody, cut it out - an old carrot or parsnip will not soften however long it cooks.")]), py_tuple(vec![py_tuple(vec![py_str("kale"), py_str("cavolo nero"), py_str("chard"), py_str("spinach")]), py_str("Strip the leaves off the stalks, because the stalk is what stays tough: steam the leaves 3 to 4 minutes, or rub kale with oil and salt for a minute to soften it for a salad without any heat.")]), py_tuple(vec![__data25(), py_str("Give it longer in a hot oven, 200C, until it collapses and the edges colour: these are mostly water, so they taste raw and squeaky until that water cooks out. Salt aubergine and courgette first and dry them, and spread them out so they roast rather than steam.")]), py_tuple(vec![py_tuple(vec![py_str("leek"), py_str("leeks"), py_str("celery"), py_str("onion"), py_str("onions"), py_str("spring onion"), py_str("spring onions"), py_str("shallot"), py_str("shallots")]), py_str("Slice it thinner and cook it slower: leeks, onions and celery stay tough when they go into a dry pan over high heat, so give them a knob of butter, a splash of water and a lid for 10 minutes. Only the dark green tops of a leek are tough enough to keep back - put those in a stock instead.")]), py_tuple(vec![py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("veggies"), py_str("root veg"), py_str("roast veg"), py_str("stir fry veg"), py_str("mixed veg"), py_str("ratatouille")]), py_str("Cut it into even pieces, no thicker than a finger, and give it longer: a tough vegetable is nearly always undercooked in the middle. Salted water for boiling, 200C in a single layer for roasting, and a knife should slide in before it comes off the heat.")])]));
    s_VEG_NAMES(op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_FAMILIES()) { let __unpk5375 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk5375[0].clone(); let mut _fix: Py = __unpk5375[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("new potatoes"), py_str("jacket potato"), py_str("jacket potatoes"), py_str("baked potato"), py_str("baked potatoes"), py_str("mushroom"), py_str("mushrooms")])));
    fn __data26() -> Py { py_tuple(vec![py_str("tough"), py_str("tougher"), py_str("hard"), py_str("harder"), py_str("still hard"), py_str("too hard"), py_str("undercooked"), py_str("under cooked"), py_str("under-done"), py_str("not cooked"), py_str("not soft"), py_str("woody"), py_str("stringy"), py_str("squeaky"), py_str("chewy"), py_str("raw"), py_str("firm"), py_str("rubbery")]) }
    s_VEG_FAULTS(__data26());
    s__SINK_VERBS(py_tuple(vec![py_str("sink"), py_str("sinks"), py_str("sinking")]));
    fn __data27() -> Py { py_tuple(vec![py_str("bread"), py_str("breads"), py_str("loaf"), py_str("loaves"), py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("sponges"), py_str("bun"), py_str("buns"), py_str("roll"), py_str("rolls"), py_str("muffin"), py_str("muffins"), py_str("scone"), py_str("scones"), py_str("souffle"), py_str("soufflé"), py_str("yorkshire"), py_str("dough"), py_str("batter"), py_str("pastry"), py_str("bake"), py_str("bakes"), py_str("baked"), py_str("baking"), py_str("oven"), py_str("risen"), py_str("proved")]) }
    s__SINK_AS_A_BAKE(__data27());
    s__PLAN_THAT_MIGHT_BE_RUINED(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|will|would|can|could|might|may|shall|should)\\s+(?:that|this|it|they|these|those)\\s+(?:ruin|ruins|spoil|spoils|wreck|wrecks)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKED_ABOUT_THE_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:why|how come|what went wrong|what did i do wrong|how do i stop|how can i stop)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data28() -> Py { py_tuple(vec![py_str("rice"), py_str("chips"), py_str("chip"), py_str("fries"), py_str("pasta"), py_str("spaghetti"), py_str("noodles"), py_str("gravy"), py_str("sauce"), py_str("soup"), py_str("stew"), py_str("casserole"), py_str("pie"), py_str("pizza"), py_str("egg"), py_str("eggs"), py_str("omelette"), py_str("bacon"), py_str("tofu"), py_str("quinoa"), py_str("couscous"), py_str("porridge"), py_str("mash"), py_str("burger"), py_str("burgers"), py_str("kebab"), py_str("fish"), py_str("lamb"), py_str("pork"), py_str("turkey"), py_str("mushroom"), py_str("mushrooms"), py_str("beans"), py_str("cabbage"), py_str("carrot"), py_str("carrots"), py_str("broccoli"), py_str("peas"), py_str("spinach"), py_str("pancakes"), py_str("biscuits"), py_str("cookies"), py_str("brownies"), py_str("pastry"), py_str("batter"), py_str("stock"), py_str("yorkshire"), py_str("mince"), py_str("bolognese"), py_str("chilli"), py_str("meat"), py_str("pudding"), py_str("custard"), py_str("jam"), py_str("sourdough"), py_str("dough"), py_str("buns"), py_str("loaf"), py_str("risotto"), py_str("polenta"), py_str("semolina"), py_str("grits"), py_str("hummus"), py_str("houmous"), py_str("roux"), py_str("mash")]) }
    s_FOOD_WORDS(__data28());
    s__A_STATE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|be|been|gets?|got|gotten|tastes?|seems?|looks?|feels?|sounds?|smells?)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_FOOD(call_builtin("frozenset", &[mcall(&py_str("\nsoil compost ground earth dirt air weather room house home flat kitchen garden lawn grass plant\nplants leaves leaf stem stems root roots skin hair hands hand face lips feet foot pan tray tin\noven hob grill fridge freezer sink washing clothes towel floor worktop counter table shelf\nradiator boiler heating window windowsill sill dish dishes plate plates bowl bowls\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__STANDS_FOR_IT(call_builtin("frozenset", &[mcall(&py_str("it its they them this that these those one ones"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data29() -> Py { py_tuple(vec![py_str("what can i make"), py_str("what can i do with"), py_str("what should i make"), py_str("what do i do with"), py_str("what to make"), py_str("what to do with"), py_str("make with"), py_str("made with"), py_str("do with"), py_str("use up"), py_str("using up"), py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("cooking"), py_str("eat"), py_str("eating"), py_str("dinner"), py_str("lunch"), py_str("supper"), py_str("tea"), py_str("meal"), py_str("meals"), py_str("meal idea"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("something quick"), py_str("something easy")]) }
    s__COOK_PHRASES(__data29());
    s_PLAIN(py_set(vec![py_str("temp"), py_str("time")]));
    s_PART(py_set(vec![py_str("skin"), py_str("stuffing"), py_str("frosting")]));
    s_DONE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|are'?nt|aren't|isnt|isn't)\\s+(?:my|the|our|it|they|these|those|this|that)\\b[^?.!]{0,40}?\\b(?:done|ready|cooked|through)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_VERB_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:tell|know|check|test|see)\\b[^?.!]{0,40}?\\b(?:is|are|its|it's)\\s+(?:done|ready|cooked|through)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data30() -> Py { py_tuple(vec![py_str("is my"), py_str("is the"), py_str("is our"), py_str("is it"), py_str("is this"), py_str("is that"), py_str("are my"), py_str("are the"), py_str("are our"), py_str("are they"), py_str("are these"), py_str("are those"), py_str("isnt my"), py_str("isn't my"), py_str("arent the"), py_str("aren't the"), py_str("is my dish")]) }
    s_DONE_POSSESSIVE(__data30());
    s_DONE_WORD(py_tuple(vec![py_str("done"), py_str("ready"), py_str("cooked"), py_str("through")]));
    s__ASK_NOISE(call_builtin("frozenset", &[mcall(&py_str("a an the my our your his her their its it they them these those this that\n    is are was were be been being isn t isn't arent aren't not\n    i we you he she they should shall can could would do does did know tell check test see\n    if whether when how what which that so yet still actually really even just about\n    left enough thorough properly all way to"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__GENERIC_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("dish food meal dinner tea supper lunch breakfast thing stuff\n    one ones piece bit pan pot oven"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__ANY_FOOD(Py::None);
    s_MAKING_A_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:make|making|made|cook|cooking|cooks|roast|roasting|bake|baking|baked|fry|frying|fried|grill|grilling|boil|boiling|simmer|simmering)\\b[^?.!]{0,60}?\\b(?:hard|harder|stale|soggy|dry|tough|rubbery|mushy|burnt)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEP_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long (?:does|do|will|can|would)\\s+(?:\\w+\\s+){0,4}(?:last|keep)\\b|\\bhow (?:do|can|should|would) (?:i|you|we) keep (?:\\w+\\s+){0,4}(?:fresh|good|from going (?:stale|hard|soft))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_IN_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,3})\\s*(?:to\\s*\\d{1,3}\\s*)?(minute|hour)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data31() -> Py { py_tuple(vec![py_str("within"), py_str("after"), py_str("up to"), py_str("keep"), py_str("keeps"), py_str("kept"), py_str("keeping"), py_str("freeze"), py_str("freezer"), py_str("fridge"), py_str("refrigerator"), py_str("cool"), py_str("store"), py_str("stored"), py_str("lasts"), py_str("left in"), py_str("left out"), py_str("use up")]) }
    s_KEEPING_LEAD(__data31());
    s_PART_WORDS(py_tuple(vec![py_str("frosting"), py_str("icing"), py_str("buttercream"), py_str("ganache"), py_str("filling"), py_str("stuffing"), py_str("skin")]));
    s_PART_BACKREF(py_tuple(vec![py_str("and"), py_str("also"), py_str("what about"), py_str("how about"), py_str("or"), py_str("same for"), py_str("what for")]));
    s__NAMES_A_SAUCE(stdlib_call("re", "compile", &[py_str("\\b(sauce|sauces|gravy|jus|relish|chutney|coulis|compote|salsa|dressing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data32() -> Py { py_tuple(vec![py_str("oven"), py_str("bake"), py_str("baking"), py_str("cook"), py_str("cooking"), py_str("roast"), py_str("fry"), py_str("frying"), py_str("grill"), py_str("boil"), py_str("simmer"), py_str("reheat"), py_str("rest"), py_str("resting"), py_str("ahead"), py_str("day before"), py_str("night before"), py_str("prepare"), py_str("sink"), py_str("sinks"), py_str("sinking"), py_str("sank"), py_str("sunken"), py_str("soggy"), py_str("stick"), py_str("sticks"), py_str("sticking"), py_str("curdle"), py_str("curdling"), py_str("split"), py_str("splitting"), py_str("burn"), py_str("burning"), py_str("burnt"), py_str("collapse"), py_str("collapsing"), py_str("dry out"), py_str("going dry"), py_str("going hard"), py_str("stale"), py_str("risen"), py_str("rise"), py_str("raising"), py_str("grey"), py_str("mushy"), py_str("raw")]) }
    s__KEEP_BUT_COOKING(__data32());
    s__KEEP_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|last|lasts|store|storing|stays good|freeze|freezer|shelf[ -]life|go off|goes off|going off|gone off)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_STATE(stdlib_call("re", "compile", &[py_str("^\\s*(?:i|we)\\s*(?:'m|'ve|'d| am| have| had| got| feel| was| were| keep| come|'s)\\b|^\\s*(?:im|ive|i've|i'm)\\s+\\w"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_REST(stdlib_call("re", "compile", &[py_str("\\bi\\s+need\\s+(?:to\\s+(?:rest|lie|sleep|sit|stop|go\\s+to\\s+bed)|a\\s+(?:rest|break|nap|lie\\s+down|sit\\s+down))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_TEMPERATURE(stdlib_call("re", "compile", &[py_str("\\b(?:has|have|had|having|got|getting|running|runs|comes? down with|down with|with)\\s+(?:a|the|any|another|high|slight)?\\s*(?:temperature|temp|fever)\\b|\\btemperature\\s+of\\s+\\d|\\bmy\\s+(?:temp|temperature)\\s+(?:is|was)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RAW_WORDS(py_tuple(vec![py_str("raw"), py_str("undercooked"), py_str("underdone"), py_str("runny"), py_str("pink")]));
    s_PLAN_NAMES(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PLANS(), "values", &[])) { let mut plan: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&plan.clone(), "get", &[py_str("names"), py_tuple(vec![])])) { let mut group: Py = __item.clone(); for __item in py_iter_or_raise(&group.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data33() -> Py { py_tuple(vec![py_str("oven"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("pan"), py_str("tray"), py_str("tin"), py_str("dish"), py_str("temperature"), py_str("degrees"), py_str("heat"), py_str("gas"), py_str("shelf"), py_str("kilo"), py_str("kilos"), py_str("kilogram"), py_str("kg"), py_str("gram"), py_str("grams"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("reheat"), py_str("leftover"), py_str("leftovers"), py_str("skin"), py_str("skinless"), py_str("stuff"), py_str("stuffed"), py_str("stuffing"), py_str("rest"), py_str("resting"), py_str("sides"), py_str("side"), py_str("done"), py_str("raw"), py_str("pink"), py_str("juices"), py_str("crisp"), py_str("crispy"), py_str("brown"), py_str("boil"), py_str("simmer"), py_str("fry"), py_str("grill"), py_str("airfryer"), py_str("sauce"), py_str("gravy"), py_str("water"), py_str("ahead"), py_str("before"), py_str("frosting"), py_str("icing"), py_str("buttercream"), py_str("ganache"), py_str("filling"), py_str("recipe"), py_str("start to finish"), py_str("in the oven"), py_str("per kilo"), py_str("per kg"), py_str("the day before"), py_str("night before")]) }
    s_COOKING_WORDS(op_add(&op_add(&op_add(&__data33(), &g_FOOD_WORDS()), &g_DISH_WORDS()), &g_PLAN_NAMES()));
    fn __data34() -> Py { py_tuple(vec![py_str("biscuit"), py_str("biscuits"), py_str("cookie"), py_str("cookies"), py_str("shortbread"), py_str("flapjack"), py_str("flapjacks"), py_str("bread"), py_str("sourdough"), py_str("loaf"), py_str("loaves"), py_str("chilli con carne"), py_str("chilli"), py_str("chili"), py_str("chips"), py_str("oven chips"), py_str("frozen chips"), py_str("curry"), py_str("tikka"), py_str("korma"), py_str("fish pie"), py_str("gammon"), py_str("gammon steak"), py_str("gammon steaks"), py_str("meringue"), py_str("meringues"), py_str("pavlova"), py_str("pasta bake"), py_str("baked pasta"), py_str("pie"), py_str("pies"), py_str("apple pie"), py_str("fruit pie"), py_str("roast potatoes"), py_str("roast potato"), py_str("roasties"), py_str("scone"), py_str("scones"), py_str("fruit scone"), py_str("cheese scone"), py_str("spag bol"), py_str("spaghetti bolognese"), py_str("spaghetti bolognaise"), py_str("bolognese"), py_str("bolognaise"), py_str("beef stew"), py_str("stew"), py_str("casserole"), py_str("sunday roast"), py_str("roast dinner"), py_str("roast lunch")]) }
    s__RECIPE_CARDS(__data34());
    fn __data35() -> Py { py_tuple(vec![py_str("and"), py_str("what"), py_str("how"), py_str("shall"), py_str("should"), py_str("do"), py_str("does"), py_str("any"), py_str("with"), py_str("also"), py_str("then"), py_str("ok"), py_str("okay"), py_str("great"), py_str("so"), py_str("could"), py_str("can"), py_str("would"), py_str("will")]) }
    s_ASK_OPENERS(__data35());
    fn __data36() -> Py { py_tuple(vec![py_str("what about"), py_str("how about"), py_str("shall i"), py_str("should i"), py_str("go with"), py_str("serve with"), py_str("alongside"), py_str("with it"), py_str("as well"), py_str("any good"), py_str("any ideas"), py_str("do we need"), py_str("do i need")]) }
    s_ASK_PHRASES(__data36());
    fn __data37() -> Py { py_tuple(vec![py_str("what can i do with"), py_str("what do i do with"), py_str("what should i do with"), py_str("what can i make with"), py_str("what can i use"), py_str("use up"), py_str("using up"), py_str("use it up"), py_str("besides"), py_str("instead of"), py_str("other than"), py_str("apart from"), py_str("bag of")]) }
    s_USE_UP_PHRASES(__data37());
    s_PLAN_NAMES_LONGEST(call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_NAMES()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &geti(&mcall(&name.clone(), "split", &[]), &Py::Int(0))))) {  __out.push(name.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| {  let name: Py = __kwp(args, 0, "name").unwrap_or(Py::None); py_tuple(vec![op_neg(&call_builtin("len", &[mcall(&name.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), name.clone()]) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data38() -> Py { py_tuple(vec![py_str("tonight"), py_str("tomorrow"), py_str("today"), py_str("this evening"), py_str("this weekend"), py_str("this afternoon"), py_str("later"), py_str("on sunday"), py_str("on saturday"), py_str("on friday"), py_str("for dinner"), py_str("for tea"), py_str("for lunch"), py_str("for sunday"), py_str("for supper"), py_str("this week")]) }
    s_WHEN_WORDS(__data38());
    fn __data39() -> Py { py_tuple(vec![py_str("doing"), py_str("cooking"), py_str("making"), py_str("roasting"), py_str("baking"), py_str("starting"), py_str("thinking of"), py_str("thinking about"), py_str("considering"), py_str("fancy"), py_str("fancying"), py_str("feel like"), py_str("feeling like"), py_str("in the mood for"), py_str("planning"), py_str("planning on"), py_str("might do"), py_str("might make"), py_str("might cook")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("tonight"), py_str("tomorrow"), py_str("this weekend"), py_str("this evening"), py_str("later"), py_str("today"), py_str("on sunday"), py_str("on saturday"), py_str("on friday"), py_str("for dinner"), py_str("for tea"), py_str("for lunch"), py_str("for sunday"), py_str("for the first time"), py_str("first time"), py_str("for my"), py_str("for a"), py_str("for the"), py_str("for six"), py_str("for 6"), py_str("for eight"), py_str("for 8"), py_str("for four"), py_str("for 4"), py_str("for two"), py_str("for 2"), py_str("i am"), py_str("we are")]) }
    s_START_WORDS(py_tuple(vec![__data39(), op_add(&g_PLAN_NAMES_LONGEST(), &__data40())]));
    s_PERSON_START(stdlib_call("re", "compile", &[py_str("\\b(i'?m|i am|we'?re|we are)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DELIBERATING_START(stdlib_call("re", "compile", &[py_str("\\b(?:thinking (?:of|about)|thought about|considered|considering|fancy|fancying|feel(?:ing)? like|in the mood for|might (?:do|make|cook|have)|may (?:do|make|cook)|planning|planning on|plan to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTED_PARTS(py_tuple(vec![py_str("time"), py_str("temp"), py_str("done")]));
    fn __data41() -> Py { py_tuple(vec![py_str("prove"), py_str("proof"), py_str("proofs"), py_str("proving"), py_str("rise"), py_str("rises"), py_str("rising"), py_str("risen"), py_str("rest"), py_str("rests"), py_str("resting"), py_str("knead"), py_str("kneads"), py_str("kneading"), py_str("chill"), py_str("chills"), py_str("chilling"), py_str("cool"), py_str("cools"), py_str("cooling"), py_str("marinate"), py_str("marinades"), py_str("marinating"), py_str("soak"), py_str("soaks"), py_str("soaking"), py_str("set"), py_str("sets"), py_str("setting")]) }
    s_STAGE_PARTS(__data41());
    s_ASK_FILLER(call_builtin("frozenset", &[mcall(&py_str("\n    a about after again all also an and any are as at be been before but by can could do does\n    doing done down for from get gets give go goes going had has have he her him his how i if in\n    into is it its just keep keeps know like long many may me min mins minute minutes more most\n    much my need needs not now of off on once one only or other our out over own per please put\n    quite rather really right roughly same shall she should so some still such take takes than\n    that the their them then there these they thing things this those time to too under until up\n    us use very want wants was way we well were what when where which while who why will with\n    would you your ok okay alright thanks thank then till yet\n    bake bakes baking boil boils boiling check chill cook cooks cooking cool cools fry grill\n    leave leaves let marinate oven pan rest roasts roast roasting simmer simmering sit stand\n    steam stir temperature temp turn turns wait\n    each side sides bit piece half quarter top bottom middle rack shelf tin dish tray foil lid\n    cover covered uncover grate hob degrees degree kilo kilos kg pound pounds lb lbs gram grams\n    cm inch inches total altogether overall low high medium slow fast quick quickest minimum\n    maximum well rare done cold frozen fresh warm hot cool leftovers leftover\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data42() -> Py { py_tuple(vec![py_str("do the"), py_str("do them"), py_str("do it"), py_str("cook the"), py_str("cook them"), py_str("roast the"), py_str("boil the"), py_str("in with"), py_str("in too"), py_str("put them in"), py_str("put it in"), py_str("same oven"), py_str("same time"), py_str("blend"), py_str("blended"), py_str("blender"), py_str("puree"), py_str("pureed"), py_str("purée"), py_str("liquidise"), py_str("liquidize"), py_str("hide"), py_str("hidden"), py_str("sneak"), py_str("grate"), py_str("grated")]) }
    s_SIDE_METHOD(__data42());
    s_GOES_WITH(py_tuple(vec![py_str("go with"), py_str("goes with"), py_str("serve with"), py_str("serving with"), py_str("pairs with"), py_str("have with"), py_str("eat with"), py_str("alongside")]));
    s_GOES_WITH_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:go|goes|going|went|serve|serves|serving|pair|pairs|pairing|eat|eats|eating|have|has|having)\\s+(?:\\w+\\s+){0,2}with\\b|\\b(?:go|goes|serve|serves|pair|pairs)\\s+alongside\\b|\\b(?:on|as)\\s+(?:the|a)\\s+side\\b|\\bside\\s+dishes?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data43() -> Py { py_tuple(vec![py_str("cheese"), py_str("apples"), py_str("apple"), py_str("pear"), py_str("pears"), py_str("grapes"), py_str("fig"), py_str("figs"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("rigatoni"), py_str("tagliatelle"), py_str("linguine"), py_str("gnocchi"), py_str("couscous"), py_str("quinoa"), py_str("halloumi"), py_str("lentils"), py_str("chickpeas"), py_str("sausages"), py_str("sausage"), py_str("ham"), py_str("salmon"), py_str("mackerel"), py_str("prawns"), py_str("tofu"), py_str("avocado"), py_str("tomatoes"), py_str("cucumber"), py_str("courgette"), py_str("aubergine"), py_str("beetroot"), py_str("sweet potato"), py_str("butternut"), py_str("hummus"), py_str("houmous"), py_str("yoghurt"), py_str("oats"), py_str("porridge"), py_str("eggs"), py_str("bacon"), py_str("chorizo"), py_str("steak"), py_str("chicken"), py_str("lamb"), py_str("pork"), py_str("duck"), py_str("turkey"), py_str("cod"), py_str("haddock"), py_str("tuna"), py_str("mushrooms"), py_str("peppers"), py_str("spinach"), py_str("broccoli"), py_str("cabbage"), py_str("peas"), py_str("beans"), py_str("carrots"), py_str("potatoes"), py_str("rice"), py_str("pasta"), py_str("noodles"), py_str("bread"), py_str("mash"), py_str("chips"), py_str("polenta"), py_str("wrap"), py_str("wraps"), py_str("pitta")]) }
    s_PAIRING_FOODS(__data43());
    s_PAIRING_RICH(py_str("Rich food wants something sharp and something crisp beside it: pickle or chutney, a few grapes or a sliced pear, crackers or warm bread, and a handful of leaves with a mustardy dressing if it has to be a meal. The rule underneath is that fat needs acid - the same reason chips want vinegar and pork wants apple sauce."));
    s_PAIRING_PLAIN(py_str("Plain and starchy wants something wet on top: a sauce, a stew, a curry, or the juices from the pan, plus one sharp thing because the starch has no acid of its own. So pasta wants a sauce with garlic and a little of the water it cooked in; rice wants a curry or a daal and a squeeze of lemon; bread wants olive oil, tomato or soup."));
    s_PAIRING_LEAN(py_str("Something starchy, something green, and something sharp or juicy: potatoes or rice or bread, a green vegetable with butter or oil on it, and a squeeze of lemon, a spoon of yoghurt, or a tomato salad. Keep it to one of each and it works with nearly anything - the sharp thing is what stops the plate tasting flat."));
    fn __data44() -> Py { py_tuple(vec![py_str("cheese"), py_str("cream"), py_str("butter"), py_str("lamb"), py_str("duck"), py_str("pork"), py_str("sausage"), py_str("sausages"), py_str("bacon"), py_str("chorizo"), py_str("halloumi"), py_str("pie"), py_str("pastry"), py_str("batter"), py_str("chips"), py_str("avocado"), py_str("salmon"), py_str("mackerel"), py_str("chocolate"), py_str("goats cheese"), py_str("feta"), py_str("steak"), py_str("beef")]) }
    s_RICH_FOODS(__data44());
    fn __data45() -> Py { py_tuple(vec![py_str("rice"), py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("rigatoni"), py_str("tagliatelle"), py_str("linguine"), py_str("noodles"), py_str("couscous"), py_str("quinoa"), py_str("bread"), py_str("potato"), py_str("potatoes"), py_str("mash"), py_str("polenta"), py_str("porridge"), py_str("oats"), py_str("wrap"), py_str("wraps"), py_str("pitta")]) }
    s_PLAIN_FOODS(__data45());
    s_COLOUR_CHOICE(stdlib_call("re", "compile", &[py_str("\\b(?:the|this|that|these|those|my|your|his|her|their|our|a|an)\\s+(?:green|blue|red|black|white|grey|gray|brown|pink|purple|orange|yellow|beige|cream)\\s+(?:one|ones|thing|things|version|colour|color)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CHOICE_FRAME(stdlib_call("re", "compile", &[py_str("\\bor\\b|\\bvs\\b|\\bversus\\b|\\bbetween\\b|\\bwhich\\b|\\bshall i\\b|\\bshould i\\b|\\bdo you want\\b|\\bwould you\\b|\\brather\\b|\\bprefer\\b|\\bpick\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IDEAS(py_tuple(vec![py_tuple(vec![py_str("eggs"), py_tuple(vec![py_str("egg"), py_str("eggs")]), py_str("Eggs are the fastest dinner there is: an omelette takes 2 minutes, a frittata with last night's vegetables 10 minutes, and eggs on toast is a proper dinner if there is cheese in the fridge.")]), py_tuple(vec![py_str("pasta"), py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("macaroni"), py_str("tagliatelle"), py_str("noodles")]), py_str("Pasta, about 12 minutes: put it on, and while it cooks warm garlic and chilli in olive oil, then toss the drained pasta through with a handful of cheese and a spoon of the water it cooked in.")]), py_tuple(vec![py_str("rice"), py_tuple(vec![py_str("rice")]), py_str("Rice, about 15 minutes: start it with the lid on, and while it steams fry onion, garlic and whatever veg you have, then stir the rice through with soy sauce and an egg.")]), py_tuple(vec![py_str("veg"), py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("broccoli"), py_str("spinach"), py_str("peppers"), py_str("mushrooms"), py_str("courgette"), py_str("aubergine"), py_str("chickpeas"), py_str("beans"), py_str("lentils")]), py_str("Whatever veg you have, 15 minutes: fry it hot in a big pan, add a tin of chickpeas or beans and a spoon of curry paste, and serve it with rice, couscous or bread.")]), py_tuple(vec![py_str("fish"), py_tuple(vec![py_str("fish"), py_str("salmon"), py_str("cod"), py_str("haddock"), py_str("prawns"), py_str("prawn"), py_str("tuna")]), py_str("Fish, about 15 minutes: a fillet skin side down in a hot pan for 4 minutes, then 2 on the other side, with rice or a bag of greens alongside.")]), py_tuple(vec![py_str("chicken"), py_tuple(vec![py_str("chicken"), py_str("turkey")]), py_str("Chicken, about 20 minutes: slice a breast thin, fry it hot for 5 or 6 minutes, then add a tin of tomatoes or a spoon of pesto and serve it with pasta or a wrap. Whole thighs take 35 minutes, so save those for a night with more time.")]), py_tuple(vec![py_str("mince"), py_tuple(vec![py_str("mince"), py_str("beef"), py_str("pork"), py_str("lamb")]), py_str("Mince, about 20 minutes: brown it hard, add a tin of tomatoes, a spoon of tomato puree and whatever veg is in the drawer, and simmer 10 minutes while rice or pasta cooks.")]), py_tuple(vec![py_str("potatoes"), py_tuple(vec![py_str("potato"), py_str("potatoes")]), py_str("Potatoes, about 25 minutes: cut them into wedges, toss in oil and salt and roast at 200C while you cook something else, or boil and mash them with butter and a spoon of mustard.")])]));
    s_TRAY_IDEA(py_str("One tray: sausages or chicken thighs, chopped potatoes, onion and peppers, oil, salt and a spoon of paprika at 200C for 40 minutes, turning once. One tray, one wash."));
    s_APPLIANCE_IDEAS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("slow cooker"), py_str("slow-cooker"), py_str("crockpot"), py_str("crock pot")]), py_str("Slow cooker: chilli, curry, stew or a shoulder of pork, on low for 6 to 8 hours or high for 3 to 4. Brown the meat first or the flavour is thin, keep the liquid to about half what you want at the end, and put herbs in for the last hour.")]), py_tuple(vec![py_tuple(vec![py_str("air fryer")]), py_str("Air fryer: chicken thighs 25 minutes at 190C, chips 20 at 200C after a shake in oil, salmon 10 at 190C. It is a small hot oven, so one layer and a shake halfway, or it steams instead of browning.")]), py_tuple(vec![py_tuple(vec![py_str("tray bake"), py_str("traybake"), py_str("roasting tin"), py_str("one pan"), py_str("one pot")]), g_TRAY_IDEA()])]));
    s_VEGAN_IDEA(py_str("Vegan and still a proper dinner: a chickpea and spinach curry with rice, a lentil and tomato lasagne with oil in place of the cheese sauce, or a tray of roasted veg with chickpeas and couscous. Check the stock cubes, the bread and the pesto, which are where the milk usually hides."));
    s_VEGGIE_IDEA(py_str("Vegetarian and still a proper dinner: a chickpea and spinach curry with rice, a lentil and mushroom lasagne, or a tray of roasted veg with feta and couscous. One dish everyone eats beats a meat main with something made separately."));
    s_VEGGIE_SWAP_IDEA(py_str("Nothing has to change about the amounts: swap the meat for a fist of chickpeas, lentils, eggs or cheese, and keep the vegetables, rice or potatoes as they are."));
    s_VEGAN_SWAP_IDEA(py_str("Nothing has to change about the amounts: swap the meat for a fist of chickpeas, lentils, tofu or nuts, use oil or a plant spread instead of butter, and keep the vegetables and rice or potatoes. Check the stock cubes, the bread and the pesto, where the milk usually hides."));
    s_OVEN_WORD(stdlib_call("re", "compile", &[py_str("\\boven\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NICE_IDEA(py_str("Something worth sitting down to, and hard to get wrong: chicken thighs roasted skin side up on a tray of potatoes, onion and peppers at 200C (180C fan) for 40 minutes, or a salmon fillet with the same tray for 15. Buy a good pudding and the whole thing feels like a dinner."));
    s_COMPARE_IDEAS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("air fryer"), py_str("oven")]), py_str("The air fryer is a small hot oven: quicker to heat, better for chips and one or two portions, worse for a big tray or a cake, because food in one layer is all it browns. Use it for a dinner for two and the oven for a family tray, a joint or anything that needs even heat.")]), py_tuple(vec![py_tuple(vec![py_str("slow cooker"), py_str("oven")]), py_str("The slow cooker is for a cheap cut and a long afternoon: it cannot brown anything, so the oven is the one for a roast, a tray of veg or a bird. Brown in a pan first and the slow cooker wins on a stew.")])]));
    s_KIDS_IDEA(py_str("Kids, about 15 minutes: pasta with butter and cheese, fish fingers or a small roast tray of chips and peas, or scrambled eggs on toast. Keep the sauce on the side and give them less than you think they want, then let them ask for more."));
    s_CHEAP_IDEA(py_str("Cheap and filling: a lentil or bean chilli, pasta with a tin of tomatoes and garlic, jacket potatoes with cheese and beans, or egg fried rice with frozen veg. Pulses and tins cost pennies a portion, and meat is the line that moves a weekly shop."));
    s_AHEAD_IDEA(py_str("Tomorrow, cook double: a chilli, a curry, a lasagne or a stew is better the next day, and the second portion is tomorrow sorted. Rice and pasta do not keep as well as the sauce, so cook those fresh."));
    s__BIRDS(py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("duck")]));
    s__RED_MEAT(py_tuple(vec![py_str("beef"), py_str("pork"), py_str("lamb"), py_str("mince"), py_str("sausage"), py_str("sausages"), py_str("ham"), py_str("bacon"), py_str("gammon"), py_str("venison")]));
    fn __data47() -> Py { py_tuple(vec![py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("turkey"), py_str("mince"), py_str("sausage"), py_str("sausages"), py_str("ham"), py_str("bacon"), py_str("gammon"), py_str("duck"), py_str("venison")]) }
    fn __data46() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("mushroom"), py_str("mushrooms")]), py_str("No mushrooms then: leave them out and put a tin of chickpeas, a handful of spinach or some roasted peppers in their place. In a lasagne or a curry they are only there for texture, so the dish still works, and the curry and the tray of roast veg have none to start with.")]), py_tuple(vec![py_tuple(vec![py_str("onion"), py_str("onions"), py_str("shallot"), py_str("shallots")]), py_str("No onion then: start the dish with a leek, a stick of celery or a spoon of tomato puree instead, and leave out the raw onion on top. The onion is there for the base, and any of those does that job.")]), py_tuple(vec![py_tuple(vec![py_str("garlic")]), py_str("No garlic then: leave it out and add a spoon of the green tops of a spring onion at the end. It is the raw heat people mind, most of the time, rather than the flavour.")]), py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("dairy")]), py_str("No cheese then: leave it off and finish the dish with a squeeze of lemon, a spoon of olive oil and some black pepper, with a spoon of pesto or a few fried onions on top if you want something savoury there. Check the stock cube and the pesto, which often have milk in them.")]), py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("salmon"), py_str("cod"), py_str("haddock"), py_str("tuna"), py_str("prawn"), py_str("prawns"), py_str("seafood")]), py_str("No fish then: chicken thighs or a tin of chickpeas go into the same sauce in the same time, and it is the sauce and the texture that carry the dish.")]), py_tuple(vec![py_tuple(vec![py_str("meat")]), py_str("Then leave the meat out: a tin of chickpeas or lentils, or a handful of mushrooms, with a stock cube, gives the dish the body the meat was doing, and the vegetarian list above is where to start.")]), py_tuple(vec![__data47(), py_str("No {food} then: swap in {other} - same dish, same time, and for a roast the same oven, potatoes and gravy - or a tin of chickpeas if you would rather have no meat.")]), py_tuple(vec![py_tuple(vec![py_str("nuts"), py_str("nut"), py_str("peanut"), py_str("peanuts"), py_str("almond"), py_str("almonds"), py_str("walnut"), py_str("walnuts"), py_str("cashew"), py_str("cashews"), py_str("pesto")]), py_str("No nuts then: toasted seeds do the same job on top, and check the pesto, the chocolate and anything marked 'may contain nuts', which is where they usually hide.")]), py_tuple(vec![py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chili"), py_str("spicy"), py_str("heat"), py_str("curry powder"), py_str("curry paste")]), py_str("No chilli then: smoked paprika or a spoon of tomato puree gives the depth without the heat, and taste as you go rather than adding it at the table.")]), py_tuple(vec![py_tuple(vec![py_str("coriander"), py_str("cilantro")]), py_str("No coriander then: flat-leaf parsley does the same fresh job, or leave the green out and finish with a squeeze of lime. If it tastes of soap, that is a gene and not the cook, and nothing you do to it will change that.")]), py_tuple(vec![py_tuple(vec![py_str("olives"), py_str("olive")]), py_str("No olives then: capers or a spoon of chopped sun-dried tomato give the same salty hit, and a little more salt and lemon does it if you have neither.")]), py_tuple(vec![py_tuple(vec![py_str("peppers"), py_str("bell pepper"), py_str("bell peppers"), py_str("red pepper"), py_str("green pepper"), py_str("yellow pepper"), py_str("sweet pepper"), py_str("capsicum")]), py_str("No peppers then: courgette, mushrooms or fennel roast in the same time and give the same bulk, and a spoon of smoked paprika puts the sweet smokiness back.")]), py_tuple(vec![py_tuple(vec![py_str("mayonnaise"), py_str("mayo")]), py_str("No mayonnaise then: yoghurt with a squeeze of lemon and a pinch of salt does the same creamy job, or a spoon of mustard thinned with oil.")]), py_tuple(vec![py_tuple(vec![py_str("cucumber")]), py_str("No cucumber then: celery, lettuce or thin ribbons of courgette give the same crunch, and they take a dressing the same way.")]), py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("passata")]), py_str("No tomato then: a little stock with a spoon of miso or a splash of balsamic gives the sauce its savoury body, and a tin of coconut milk does it for a curry; roasted red pepper, skinned and mashed, is the closest thing to the texture.")]), py_tuple(vec![py_tuple(vec![py_str("egg"), py_str("eggs")]), py_str("No egg then: a spoon of cornflour or a little of the water the pasta cooked in brings the sauce together, and check the pasta itself if it is the fresh kind.")])]) }
    s_SUBSTITUTES(__data46());
    fn __data48() -> Py { py_tuple(vec![py_str("mushroom"), py_str("mushrooms"), py_str("onion"), py_str("onions"), py_str("shallot"), py_str("shallots"), py_str("garlic"), py_str("cheese"), py_str("dairy"), py_str("milk"), py_str("butter"), py_str("cream"), py_str("yoghurt"), py_str("yogurt"), py_str("fish"), py_str("salmon"), py_str("cod"), py_str("haddock"), py_str("tuna"), py_str("prawn"), py_str("prawns"), py_str("seafood"), py_str("meat"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("mince"), py_str("sausage"), py_str("sausages"), py_str("ham"), py_str("bacon"), py_str("nuts"), py_str("nut"), py_str("peanut"), py_str("peanuts"), py_str("almond"), py_str("almonds"), py_str("walnut"), py_str("walnuts"), py_str("cashew"), py_str("cashews"), py_str("pesto"), py_str("chilli"), py_str("chillies"), py_str("chili"), py_str("spicy"), py_str("tomato"), py_str("tomatoes"), py_str("passata"), py_str("egg"), py_str("eggs"), py_str("coriander"), py_str("cilantro"), py_str("olives"), py_str("cucumber"), py_str("mayonnaise"), py_str("mayo"), py_str("peppers"), py_str("capsicum")]) }
    s_DISLIKED(__data48());
    fn __data49() -> Py { py_tuple(vec![py_str("hates"), py_str("hate"), py_str("hated"), py_str("doesnt like"), py_str("doesn't like"), py_str("does not like"), py_str("dont like"), py_str("don't like"), py_str("do not like"), py_str("cant stand"), py_str("can't stand"), py_str("cannot stand"), py_str("no"), py_str("not a fan of"), py_str("sick of"), py_str("can not stand"), py_str("never eats"), py_str("never eat"), py_str("is not a fan of"), py_str("are not a fan of"), py_str("dont have"), py_str("don't have"), py_str("do not have"), py_str("havent got"), py_str("haven't got"), py_str("have not got"), py_str("ran out of"), py_str("run out of"), py_str("out of"), py_str("none left"), py_str("got no"), py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("wouldnt eat"), py_str("wouldn't eat"), py_str("doesnt eat"), py_str("doesn't eat"), py_str("does not eat"), py_str("refuses"), py_str("refuse"), py_str("dislikes"), py_str("dislike")]) }
    s_DISLIKE_SHAPES(__data49());
    s_OUT_OF_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:dont|don't|do not|havent|haven't|have not|hasnt|hasn't|has not)\\s+(?:got|have|has)\\b|\\bgot no\\b|\\b(?:ran|run)\\s+out\\s+of\\b|\\bout\\s+of\\b|\\bnone left\\b|\\bnothing left\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KITCHEN_SWAP(py_tuple(vec![py_str("milk"), py_str("butter"), py_str("cream"), py_str("sour cream"), py_str("double cream"), py_str("single cream"), py_str("creme fraiche"), py_str("buttermilk"), py_str("yoghurt"), py_str("yogurt")]));
    fn __data50() -> Py { py_tuple(vec![py_str("wasting"), py_str("waste"), py_str("wasted"), py_str("throwing"), py_str("throw"), py_str("thrown"), py_str("bin"), py_str("binning"), py_str("buying"), py_str("bought"), py_str("cooking"), py_str("making"), py_str("chopping"), py_str("peeling"), py_str("eating"), py_str("smelling"), py_str("the smell of"), py_str("the taste of")]) }
    s_WASTE_WORDS(__data50());
    fn __data51() -> Py { py_tuple(vec![py_str("and"), py_str("or"), py_str("in"), py_str("on"), py_str("at"), py_str("for"), py_str("with"), py_str("please"), py_str("thanks"), py_str("though"), py_str("tho"), py_str("either"), py_str("tonight"), py_str("today"), py_str("now"), py_str("anymore"), py_str("any"), py_str("more"), py_str("all"), py_str("at all"), py_str("whatsoever"), py_str("ever"), py_str("honestly"), py_str("really"), py_str("but"), py_str("so"), py_str("because"), py_str("if"), py_str("when"), py_str("as"), py_str("to"), py_str("from"), py_str("into"), py_str("out"), py_str("off"), py_str("up"), py_str("away"), py_str("instead"), py_str("here"), py_str("there"), py_str("myself"), py_str("himself"), py_str("herself"), py_str("themselves"), py_str("food"), py_str("foods"), py_str("dishes"), py_str("stuff"), py_str("things"), py_str("anything"), py_str("meals"), py_str("dinners"), py_str("what"), py_str("which"), py_str("who"), py_str("can"), py_str("could"), py_str("shall"), py_str("should"), py_str("would"), py_str("will"), py_str("do"), py_str("does"), py_str("did"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("i"), py_str("we"), py_str("you"), py_str("they"), py_str("idea"), py_str("ideas"), py_str("recipe"), py_str("recipes"), py_str("suggestions"), py_str("suggestion"), py_str("instead"), py_str("tonight"), py_str("today"), py_str("for"), py_str("with"), py_str("any"), py_str("some")]) }
    s_FOOD_TAIL_OK(__data51());
    s_HOW_WORDS(stdlib_call("re", "compile", &[py_str("^\\s*(?:how|can|could|do|does|is|what do i)\\b|\\binstead of\\b|\\bsubstitute\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GENERIC_SWAP(py_str("Then leave the {food} out: cook the rest as it is and add something with the same job - a tin of chickpeas for body, a squeeze of lemon or a spoon of mustard for the edge - and taste it before it goes to the table."));
    s_CUPBOARD(stdlib_call("re", "compile", &[py_str("\\bwhat can i (?:make|do|cook|have) (?:with|out of|from)\\b|\\bwhat (?:to|shall i) (?:make|do|cook)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:i|we) ?(?:'ve|have(?: got)?|only have|just have|still have|got)\\b|\\ball i (?:have|have got|ve got)\\b|\\bthere(?:'s| is) only\\b|\\bonly (?:got|have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data52() -> Py { py_tuple(vec![py_str("hate"), py_str("hates"), py_str("hated"), py_str("dislikes"), py_str("dislike"), py_str("doesnt like"), py_str("doesn't like"), py_str("does not like"), py_str("dont like"), py_str("don't like"), py_str("do not like"), py_str("cant stand"), py_str("can't stand"), py_str("cannot stand"), py_str("can not stand"), py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("refuses to eat"), py_str("refuse to eat"), py_str("never eats"), py_str("never eat"), py_str("allergic to"), py_str("intolerant to"), py_str("sick of"), py_str("not a fan of"), py_str("is not a fan of"), py_str("are not a fan of"), py_str("off ")]) }
    s_SAID_DISLIKE(__data52());
    fn __data53() -> Py { py_tuple(vec![py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("cook"), py_str("cooking"), py_str("make"), py_str("meal"), py_str("meals"), py_str("recipe"), py_str("recipes"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("what can i"), py_str("what should i"), py_str("what do i"), py_str("feed"), py_str("tonight")]) }
    s_DINNER_ASK(__data53());
    fn __data54() -> Py { py_tuple(vec![py_str("my wife"), py_str("my husband"), py_str("my partner"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my children"), py_str("my kid"), py_str("my kids"), py_str("my mum"), py_str("my mom"), py_str("my mother"), py_str("my dad"), py_str("my father"), py_str("my parents"), py_str("my family"), py_str("my friend"), py_str("my brother"), py_str("my sister"), py_str("my boyfriend"), py_str("my girlfriend"), py_str("my flatmate"), py_str("my housemate"), py_str("my nan"), py_str("my grandma"), py_str("my grandad"), py_str("my stepson"), py_str("my stepdaughter"), py_str("he "), py_str("she "), py_str("they "), py_str("his "), py_str("her "), py_str("their "), py_str("the kids"), py_str("the children"), py_str("the little one"), py_str("someone"), py_str("somebody"), py_str("our guest"), py_str("one of the guests"), py_str("my guests")]) }
    s_OTHER_PEOPLE(__data54());
    fn __data55() -> Py { py_tuple(vec![py_str("hates"), py_str("dislikes"), py_str("does not like"), py_str("doesnt like"), py_str("doesn't like"), py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("refuses"), py_str("refuse"), py_str("never eats"), py_str("not a fan"), py_str("allergic to"), py_str("intolerant to")]) }
    s_SAID_DISLIKE_OTHER(__data55());
    fn __data56() -> Py { py_tuple(vec![py_str("i love"), py_str("i like"), py_str("i prefer"), py_str("we love"), py_str("we like"), py_str("i adore"), py_str("my favourite"), py_str("i rather have"), py_str("i would rather"), py_str("he loves"), py_str("she loves"), py_str("the kids love"), py_str("the children love"), py_str("they love")]) }
    s_LIKING(__data56());
    s_DIET_FAMILIES(py_tuple(vec![py_str("dairy"), py_str("meat"), py_str("gluten"), py_str("wheat"), py_str("soya"), py_str("soy"), py_str("sugar"), py_str("carbs"), py_str("carbohydrate"), py_str("carbohydrates")]));
    s_EATING_REFUSALS(py_tuple(vec![py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("refuse to eat"), py_str("refuses to eat"), py_str("never eat"), py_str("never eats"), py_str("doesnt eat"), py_str("doesn't eat"), py_str("does not eat")]));
    fn __data57() -> Py { py_tuple(vec![py_str("left"), py_str("none left"), py_str("ran out"), py_str("run out"), py_str("out of"), py_str("in the house"), py_str("in the cupboard"), py_str("in the fridge"), py_str("not got"), py_str("havent got"), py_str("haven't got"), py_str("dont have"), py_str("don't have"), py_str("short of"), py_str("all gone"), py_str("none of")]) }
    s_SHORTAGE_MARKERS(__data57());
    s_DISLIKE_VERBS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKE_SHAPES()) { let mut shape: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&shape.clone(), &py_str("no")))) {  __out.push(shape.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data58() -> Py { py_tuple(vec![py_str("hates"), py_str("hate"), py_str("hated"), py_str("doesnt like"), py_str("doesn't like"), py_str("does not like"), py_str("dont like"), py_str("don't like"), py_str("do not like"), py_str("cant stand"), py_str("can't stand"), py_str("cannot stand"), py_str("can not stand"), py_str("not a fan of"), py_str("is not a fan of"), py_str("are not a fan of"), py_str("never eats"), py_str("never eat")]) }
    s_PERSON_DISLIKE(__data58());
    fn __data59() -> Py { py_tuple(vec![py_str("i hate"), py_str("i hates"), py_str("i dont like"), py_str("i don't like"), py_str("i do not like"), py_str("i cant stand"), py_str("i can't stand"), py_str("i cannot stand"), py_str("im not a fan"), py_str("i'm not a fan"), py_str("i never eat"), py_str("i never eats"), py_str("i avoid")]) }
    s_OWN_DISLIKE(__data59());
    fn __data60() -> Py { py_tuple(vec![py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("toddler"), py_str("baby"), py_str("mum"), py_str("mother"), py_str("dad"), py_str("father"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("brother"), py_str("sister"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("mates"), py_str("guest"), py_str("guests"), py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her"), py_str("their"), py_str("them"), py_str("his"), py_str("hers"), py_str("everyone"), py_str("family"), py_str("flatmate"), py_str("housemate")]) }
    s_OTHER_PEOPLE(__data60());
    fn __data61() -> Py { py_tuple(vec![py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("toddler"), py_str("toddlers"), py_str("baby"), py_str("boy"), py_str("girl"), py_str("year old"), py_str("yearold")]) }
    s_A_CHILD(call_builtin("frozenset", &[__data61()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data62() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("make"), py_str("making"), py_str("dinner"), py_str("dinners"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("meals"), py_str("recipe"), py_str("recipes"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("suggestions"), py_str("suggestion"), py_str("feed")]) }
    s_COOK_ASK_WORDS(call_builtin("frozenset", &[__data62()]).unwrap_or_else(|e| exc::raise(e)));
    s_EATING_REFUSAL(py_tuple(vec![py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("wouldnt eat"), py_str("wouldn't eat"), py_str("refuses to eat"), py_str("refused to eat"), py_str("doesnt eat"), py_str("doesn't eat"), py_str("does not eat"), py_str("never eats")]));
    s_LUNCH_MEALS(call_builtin("frozenset", &[mcall(&py_str("breakfast breakfasts brunch lunch lunches lunchbox lunchboxes"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_IN_A_STEP(stdlib_call("re", "compile", &[py_str("\\b\\d{1,3}\\s*(?:to\\s*\\d{1,3}\\s*)?(?:minutes?|hours?|seconds?)\\b|\\b\\d{1,3}\\s*degrees\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH_LEAD(stdlib_call("re", "compile", &[py_str("^[A-Za-z][A-Za-z '&-]{2,28}:\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data63() -> Py { py_tuple(vec![py_str("whisk"), py_str("cook"), py_str("bake"), py_str("roast"), py_str("fry"), py_str("boil"), py_str("simmer"), py_str("grill"), py_str("pour"), py_str("stir"), py_str("layer"), py_str("finish"), py_str("leave"), py_str("rest"), py_str("turn"), py_str("add"), py_str("cover"), py_str("reduce"), py_str("chop"), py_str("heat"), py_str("mix"), py_str("fold"), py_str("beat"), py_str("chill"), py_str("set"), py_str("strain"), py_str("drain"), py_str("put")]) }
    s_STEP_VERBS(__data63());
    s_EGG_FREE_BAKE(py_str("No egg then, and it still works: one egg becomes 60g of yoghurt, half a mashed banana, or a tablespoon of ground flax or chia stirred into 3 tablespoons of water and left 10 minutes. Those bind but do not lift, so add half a teaspoon of baking powder as well, and give a batter an extra splash of milk to loosen it."));
    s_NUT_FREE_BAKE(py_str("No nuts then, and the cake still works: leave them out of the sponge and the icing rather than swapping the cake. Check the chocolate, the icing sugar and anything ready-made for 'may contain nuts', and keep the worktop, the tin and the spoon clean if the allergy is serious."));
    fn __data64() -> Py { py_tuple(vec![py_str("nut"), py_str("nuts"), py_str("peanut"), py_str("peanuts"), py_str("almond"), py_str("almonds"), py_str("cashew"), py_str("cashews"), py_str("walnut"), py_str("walnuts"), py_str("pistachio"), py_str("pistachios"), py_str("hazelnut"), py_str("hazelnuts"), py_str("sesame"), py_str("shellfish"), py_str("prawn"), py_str("prawns")]) }
    s_DEFER_TO_ALLERGY(__data64());
    fn __data65() -> Py { py_tuple(vec![py_str("nut"), py_str("nuts"), py_str("peanut"), py_str("peanuts"), py_str("almond"), py_str("almonds"), py_str("cashew"), py_str("cashews"), py_str("walnut"), py_str("walnuts"), py_str("pistachio"), py_str("pistachios"), py_str("hazelnut"), py_str("hazelnuts"), py_str("sesame"), py_str("egg"), py_str("eggs"), py_str("fish"), py_str("shellfish"), py_str("prawn"), py_str("prawns"), py_str("milk"), py_str("dairy"), py_str("cheese"), py_str("butter"), py_str("cream"), py_str("yoghurt"), py_str("yogurt"), py_str("gluten"), py_str("wheat"), py_str("flour"), py_str("soya"), py_str("soy"), py_str("mustard"), py_str("sulphites"), py_str("sulfites")]) }
    s_ALLERGENS(__data65());
    s__ALLERGEN_NOTE(py_str(" If someone at the table is allergic rather than fussy, read the labels for 'may contain' and keep the worktop, the tin and the spoon clean."));
    fn __data66() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("lasted"), py_str("store"), py_str("stores"), py_str("storing"), py_str("stored"), py_str("still good"), py_str("still ok"), py_str("stays good"), py_str("shelf life"), py_str("freeze"), py_str("freezes"), py_str("freezer"), py_str("fridge"), py_str("frozen"), py_str("gone off"), py_str("next day"), py_str("tomorrow"), py_str("use by"), py_str("out of date"), py_str("still use"), py_str("still use it"), py_str("still eat"), py_str("still eat it"), py_str("still have"), py_str("still alright"), py_str("still all right"), py_str("is it ok"), py_str("is it okay"), py_str("was it ok"), py_str("was it okay")]) }
    s_KEEP_ASK_WORDS(__data66());
    fn __data67() -> Py { py_tuple(vec![py_str("plant"), py_str("plants"), py_str("houseplant"), py_str("houseplants"), py_str("plant pot"), py_str("pot plant"), py_str("garden"), py_str("lawn"), py_str("grass"), py_str("soil"), py_str("compost"), py_str("watering"), py_str("repot"), py_str("wilt"), py_str("wilted"), py_str("prune"), py_str("pruning"), py_str("fertiliser"), py_str("fertilizer"), py_str("seedling"), py_str("seedlings"), py_str("sow"), py_str("sowing"), py_str("indoors"), py_str("greenhouse"), py_str("allotment"), py_str("flower bed"), py_str("a day"), py_str("every day"), py_str("each day"), py_str("a week"), py_str("every week"), py_str("once a week"), py_str("how often"), py_str("every morning"), py_str("every night"), py_str("healthy"), py_str("unhealthy"), py_str("good for you"), py_str("bad for you"), py_str("fattening"), py_str("calories"), py_str("calorie"), py_str("diet"), py_str("dieting"), py_str("nutrition"), py_str("protein"), py_str("vitamins"), py_str("too much"), py_str("too many")]) }
    s_NOT_A_DISH(__data67());
    s_NOT_A_KEEPING_QUESTION(op_add(&g_RAW_WORDS(), &py_tuple(vec![py_str("raw egg"), py_str("raw flour"), py_str("pink in the middle"), py_str("pink inside"), py_str("runny yolk"), py_str("runny egg"), py_str("still frozen"), py_str("defrosted properly"), py_str("thawed properly")])));
    fn __data68() -> Py { py_tuple(vec![py_str("warm"), py_str("hot"), py_str("moist"), py_str("juicy"), py_str("juicier"), py_str("tender"), py_str("tenderer"), py_str("from drying out"), py_str("drying out"), py_str("dry out"), py_str("from going dry"), py_str("from sticking"), py_str("not sticking"), py_str("sticking"), py_str("sticks together"), py_str("sticking together"), py_str("fluffy"), py_str("separate"), py_str("separated"), py_str("from going brown"), py_str("going brown"), py_str("from drying out"), py_str("drying out"), py_str("from drying"), py_str("from curdling"), py_str("curdling"), py_str("from splitting"), py_str("splitting"), py_str("runny"), py_str("setting"), py_str("from setting")]) }
    s_KEEP_QUALITY_WORDS(__data68());
    fn __data69() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("biscuit"), py_str("biscuits"), py_str("cookie"), py_str("cookies"), py_str("loaf"), py_str("bread"), py_str("muffin"), py_str("muffins"), py_str("brownie"), py_str("brownies"), py_str("tart"), py_str("pie"), py_str("birthday"), py_str("batter"), py_str("icing"), py_str("buttercream"), py_str("frosting"), py_str("pancake"), py_str("pancakes"), py_str("waffle"), py_str("waffles"), py_str("yorkshire"), py_str("crepe"), py_str("crepes")]) }
    s_BAKE_WORDS(__data69());
    s_EGG_DISH_WORDS(py_tuple(vec![py_str("omelette"), py_str("omelet"), py_str("omlette"), py_str("scrambled"), py_str("scramble"), py_str("frittata"), py_str("quiche"), py_str("poached"), py_str("shakshuka"), py_str("benedict")]));
    s_EGG_IS_THE_DISH(py_str("Eggs are the whole dish there, so there is no swapping them out of it — what works is the dish that stands in for it. Chickpea flour for an omelette: 120g gram flour and 150ml water whisked with a pinch of salt and turmeric, fried thin in a hot oiled pan, 2 to 3 minutes a side. Crumble firm tofu into a pan with turmeric and black pepper for a scramble, 5 minutes, and a pinch of black salt if you keep it for the egg taste. A quiche sets the same way with a tofu or chickpea-flour custard poured into the case."));
    s_PUDDINGS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("no oven"), py_str("without an oven"), py_str("no baking"), py_str("no bake"), py_str("hob only"), py_str("just a hob"), py_str("fridge")]), py_str("No oven, so a cold one: an Eton mess (broken meringue, whipped cream, strawberries or raspberries), a lemon posset (300ml cream, 75g sugar and the juice of a lemon, simmered 3 minutes and set overnight), or a chocolate fridge cake (200g chocolate and 100g butter melted over broken biscuits, chilled 2 hours).")]), py_tuple(vec![py_tuple(vec![py_str("chocolate"), py_str("choccy"), py_str("chocolatey")]), py_str("Chocolate mousse: melt 200g of chocolate, beat in 3 egg yolks, fold in 300ml of whipped cream and chill 2 hours. A fridge cake is quicker - broken biscuits, melted chocolate and butter, a handful of raisins, 2 hours in the fridge.")]), py_tuple(vec![py_tuple(vec![py_str("cold"), py_str("ice cream"), py_str("icecream"), py_str("frozen"), py_str("summer"), py_str("hot day")]), py_str("Something cold: an Eton mess, a lemon posset, or affogato - a scoop of vanilla ice cream with a shot of hot espresso over it.")]), py_tuple(vec![py_tuple(vec![py_str("quick"), py_str("quickly"), py_str("fast"), py_str("easy"), py_str("last minute"), py_str("ten minutes"), py_str("5 minutes")]), py_str("Ten minutes: an Eton mess, bananas with ice cream and butterscotch sauce, or yoghurt with honey and toasted nuts. A good shop-bought tart with cream is a proper pudding too, and no one minds.")])]));
    s_CROWD_PUDDING(py_str("For a crowd, one big dish rather than plates: an apple and blackberry crumble (fruit in a dish, a crumble of 100g each of flour and butter with 50g of sugar, 40 minutes at 180C/160C fan, with custard), or a baked rice pudding, or a bread and butter pudding - all made in the morning, and all soft enough for anyone who finds chewing hard. An Eton mess takes five minutes if you buy the meringues."));
    fn __data70() -> Py { py_tuple(vec![py_str("crowd"), py_str("lots"), py_str("many"), py_str("everyone"), py_str("party"), py_str("dozen"), py_str("guests"), py_str("guest"), py_str("people"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve"), py_str("thirteen"), py_str("fourteen"), py_str("fifteen"), py_str("sixteen"), py_str("eighteen"), py_str("twenty"), py_str("thirty"), py_str("forty"), py_str("fifty"), py_str("hundred")]) }
    s_CROWD_WORDS(__data70());
    s_FRUIT_PUDDING(py_str("An apple and blackberry crumble: the fruit in a dish with a spoon of sugar, a crumble of 100g each of flour and butter rubbed together with 50g of sugar, 40 minutes at 180C (160C fan), and custard. It scales to any number, it is better made in the morning, and it is the pudding people are pleased to see."));
    s_DESSERT_WORDS(py_tuple(vec![py_str("pudding"), py_str("puddings"), py_str("pud"), py_str("dessert"), py_str("desserts"), py_str("afters"), py_str("sweet"), py_str("sweets"), py_str("puddingy")]));
    s_ATE_ALREADY(stdlib_call("re", "compile", &[py_str("\\b(?:ate|eaten|drank|drunk|had|has had|have had|having)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REWARDED(stdlib_call("re", "compile", &[py_str("\\b(?:reward|rewards|rewarding|rewarded|bribe|bribes|bribing|bribed|good behaviou?r|star chart|sticker|stickers|incentive|incentives|pocket money|allowance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUDDING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|who|any|anything|ideas?|suggest\\w*|recommend\\w*|make|makes|making|made|recipe|serve|serving|shall|could|should|want|need|fancy|instead|tonight|pudding|dessert)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OR_WORD(stdlib_call("re", "compile", &[py_str("\\bor\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|this|one|its)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_APPLIANCE(stdlib_call("re", "compile", &[py_str("\\b(?:no|without a|not got a|haven't got a|havent got a)\\s+(?:oven|hob|cooker|grill)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAFETY_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:frozen|freeze|freezer|defrost\\w*|thaw\\w*|raw|reheat\\w*|left out|gone off|off meat|use by|after its date)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data71() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("meals"), py_str("eat"), py_str("eating"), py_str("food"), py_str("kitchen"), py_str("recipe"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("hungry"), py_str("shop"), py_str("shopping"), py_str("supermarket"), py_str("leftovers"), py_str("leftover"), py_str("cooker"), py_str("fryer"), py_str("crockpot"), py_str("hob"), py_str("pan"), py_str("tray"), py_str("tin"), py_str("bake")]) }
    s_MEAL_WORDS(__data71());
    fn __data72() -> Py { py_tuple(vec![py_str("vegetarian"), py_str("vegetarians"), py_str("veggie"), py_str("vegan"), py_str("vegans"), py_str("don't eat meat"), py_str("dont eat meat"), py_str("do not eat meat"), py_str("doesn't eat meat"), py_str("doesnt eat meat"), py_str("does not eat meat"), py_str("no meat"), py_str("meat free"), py_str("meat-free"), py_str("won't eat meat"), py_str("wont eat meat"), py_str("will not eat meat")]) }
    s_VEGGIE_KEY(__data72());
    fn __data73() -> Py { py_tuple(vec![py_str("is"), py_str("are"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("them"), py_str("she"), py_str("he"), py_str("they"), py_str("him"), py_str("her"), py_str("one"), py_str("of"), py_str("no"), py_str("not"), py_str("doesnt"), py_str("doesn't"), py_str("cant"), py_str("can't"), py_str("wont"), py_str("won't"), py_str("guests"), py_str("guest"), py_str("party"), py_str("them"), py_str("of us"), py_str("mate"), py_str("mates"), py_str("friend"), py_str("friends"), py_str("son"), py_str("daughter"), py_str("mum"), py_str("dad"), py_str("mother"), py_str("father"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("brother"), py_str("sister"), py_str("everyone"), py_str("someone"), py_str("somebody"), py_str("one guest"), py_str("one of")]) }
    s_VEGGIE_CARRIERS(__data73());
    fn __data74() -> Py { py_tuple(vec![py_str("dairy free"), py_str("dairy-free"), py_str("no dairy"), py_str("lactose free"), py_str("lactose intolerant"), py_str("lactose intolerance"), py_str("milk free"), py_str("milk-free"), py_str("have dairy"), py_str("eat dairy"), py_str("had dairy"), py_str("have milk"), py_str("eat milk"), py_str("drink milk"), py_str("no milk"), py_str("not dairy")]) }
    s_DIETS(py_tuple(vec![py_tuple(vec![py_str("dairy"), __data74(), py_str("Dairy free is easy to cook around: oat, soya or almond milk in place of milk, olive oil instead of butter, and a tin of coconut milk where a recipe wants cream. Watch the labels for whey, casein, milk powder and lactose, which sit in stock cubes, bread, biscuits and some crisps, and check the pesto and the chocolate too.")]), py_tuple(vec![py_str("gluten"), py_tuple(vec![py_str("gluten free"), py_str("gluten-free"), py_str("no gluten"), py_str("wheat free"), py_str("wheat-free"), py_str("eat gluten"), py_str("have gluten"), py_str("had gluten"), py_str("eat wheat"), py_str("have wheat"), py_str("no wheat")]), py_str("Gluten free: potatoes, rice, corn, quinoa and lentils are all fine, so a chilli, a curry, a stir-fry or a tray of roast veg with rice needs nothing changed. For a bake, use a gluten-free plain flour blend with a quarter teaspoon of xanthan gum per 200g and let the batter stand 20 minutes; cornflour is for thickening a sauce. Watch the labels for wheat, barley, rye and 'may contain', use tamari in place of soy sauce, and check the sausages and the stock cubes.")]), py_tuple(vec![py_str("egg"), py_tuple(vec![py_str("egg free"), py_str("egg-free"), py_str("no eggs"), py_str("no egg")]), py_str("Egg free: in a cake or biscuits one egg becomes 60g of yoghurt, half a mashed banana, or a tablespoon of ground flax or chia stirred into three of water and left ten minutes - those bind but do not lift, so add half a teaspoon of baking powder too. Watch the labels for egg white and albumen, which are in some pasta, mayonnaise, cakes and glazed bread.")]), py_tuple(vec![py_str("nut"), py_tuple(vec![py_str("nut free"), py_str("nut-free"), py_str("no nuts"), py_str("no nut"), py_str("eat nuts"), py_str("have nuts")]), py_str("Nut free: toasted seeds do the same job on top and in a flapjack, and check the labels for 'may contain nuts' on chocolate, pesto, cereal and anything made in a bakery, which is where they usually hide.")])]));
    s__DIET_FOOD(py_dict(vec![(py_str("dairy"), py_tuple(vec![py_str("dairy"), py_str("milk")])), (py_str("gluten"), py_tuple(vec![py_str("gluten"), py_str("wheat")])), (py_str("egg"), py_tuple(vec![py_str("egg"), py_str("eggs")])), (py_str("nut"), py_tuple(vec![py_str("nut"), py_str("nuts")]))]));
    s_DIETS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DIETS()) { let __unpk4144 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk4144[0].clone(); let mut words: Py = __unpk4144[1].clone(); let mut said: Py = __unpk4144[2].clone(); __out.push(py_tuple(vec![name.clone(), op_add(&call_builtin("tuple", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__DIET_FOOD(), "get", &[name.clone(), py_tuple(vec![])])) { let mut food: Py = __item.clone(); for __item in py_iter_or_raise(&_negative_forms(&[food.clone()])) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))), said.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_DAIRY_FREE_VEGGIE(py_str("Vegetarian and no dairy, which is one list and not two: a chickpea and spinach curry with rice, a lentil and tomato lasagne made with oil instead of the cheese sauce, or a tray of roasted veg with chickpeas and couscous. Watch the labels for whey, casein and milk powder, which sit in stock cubes, bread and biscuits."));
    s__MEALS(py_str("(?:breakfast|brekkie|brekky|lunch|dinner|supper|meal|meals)"));
    s_DIET_MEAL_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\bfor (?:breakfast|brekkie|brekky|lunch|dinner|tea|supper)\\b|\\b"), &g__MEALS()), &py_str("\\b[^.]{0,25}\\bideas?\\b|\\bideas?\\b[^.]{0,25}\\b")), &g__MEALS()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIET_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\bcan (?:eat|have|drink)\\b|\\bhow do i\\b|\\bhow long\\b|\\b(?:can|could|should|may) i (?:ever |still |now )?(?:eat|have|drink)(?! for (?:breakfast|lunch|dinner|tea|supper|pudding|dessert|a snack|a starter)\\b)\\b|\\bis it (?:ok|okay|fine|bad|good) (?:to|for me to) (?:eat|have|drink)\\b|\\bam i allowed to (?:eat|have|drink)\\b|\\bok (?:if|for|with)\\b|\\bokay (?:if|for|with)\\b|\\bfine to eat\\b|\\bsafe to eat\\b|\\bdo i have\\b|\\bin the fridge\\b|\\bin the cupboard\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIET_TOO_MEDICAL(stdlib_call("re", "compile", &[py_str("\\ballerg|\\banaphyla|\\bepipen\\b|\\bcoeliac\\b|\\bceliac\\b|\\bcalorie|\\bcalories\\b|\\bprotein\\b|\\bvitamin|\\bhealthy\\b|\\bis it bad\\b|\\bgood for you\\b|\\bweight\\b|\\bbaby\\b|\\bchild\\b|\\bchildren\\b|\\bpregnant\\b|\\bpregnancy\\b|\\bbreastfeed|\\bbreast feed|\\bnursing\\b|\\bnurse\\b|\\blactating\\b|\\bchestfeed"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIET_RECALL(stdlib_call("re", "compile", &[py_str("\\bmy diet\\b|\\bwhat (?:do|did) i eat\\b|\\bwhat am i eating\\b|\\bwhat (?:can|can't|cant|could) i (?:not )?(?:eat|have)\\b|\\bwhat do i not eat\\b|\\bam i (?:vegetarian|vegan|pescatarian|on a diet)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VEGAN_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:vegan|vegans|plant based|plantbased)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GUEST_DIET(stdlib_call("re", "compile", &[py_str("\\b(?:one of (?:them|us|you|the guests|the family|my guests|the party)|one guest|a guest|some of them|someone|somebody|everyone|one (?:of (?:them|us|the guests|my guests|the family) )?(?:is|was|isnt|doesnt|wont|cant|does not|will not|can not|only eats) |my (?:friend|mate|son|daughter|mum|dad|mother|father|sister|brother|partner|wife|husband|in laws|niece|nephew|cousin|colleague|neighbour))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COOKING_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:cook|cooks|cooking|cooked|bake|bakes|baking|roast|roasts|roasting|fry|frying|grill|grilling|recipe|recipes|what can i (?:cook|make)|what should i (?:cook|make))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MAKE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:make|makes|making)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_YES_NO_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:do|does|can|could|should|is|are|would|will|did)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COOK_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:cook|cooking|make|dinner|tea|supper|lunch|feed|menu|recipe|meal)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data75() -> Py { py_tuple(vec![py_str("minutes"), py_str("minute"), py_str("hour"), py_str("hours"), py_str("quick"), py_str("quickly"), py_str("fast"), py_str("easy"), py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("budget"), py_str("broke"), py_str("nice"), py_str("nicer"), py_str("special"), py_str("treat"), py_str("fancy"), py_str("tomorrow"), py_str("kids"), py_str("kid"), py_str("children"), py_str("child"), py_str("toddler")]) }
    s_PICK_WORDS(__data75());
    s_NICE_WORDS(py_tuple(vec![py_str("nice"), py_str("nicer"), py_str("special"), py_str("treat"), py_str("fancy"), py_str("impress"), py_str("impressive"), py_str("lovely")]));
    fn __data76() -> Py { py_tuple(vec![py_str("girlfriend"), py_str("boyfriend"), py_str("partner"), py_str("wife"), py_str("husband"), py_str("date"), py_str("anniversary"), py_str("guests"), py_str("guest"), py_str("friends"), py_str("family"), py_str("in-laws"), py_str("parents"), py_str("people")]) }
    s_GUEST_WORDS(__data76());
    s__CLEANING_IT(stdlib_call("re", "compile", &[py_str("\\b(?:clean|cleaning|cleaned|wash|washing|scrub|scouring|dishwasher|dishwashers)\\b|\\b(?:burnt|baked|stuck|crusted)\\s+on\\b|\\bhow do i clean\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PICK_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:or|vs|versus|rather|which|better|best)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHOLE_FOOD(stdlib_call("re", "compile", &[py_str("\\bwhole (?:chicken|bird|fish|leg|shoulder|duck|turkey|ham)\\b|\\b(?:leg|shoulder|joint) of\\b|\\b(?:bought|got|have) a (?:joint|bird)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data77() -> Py { py_tuple(vec![py_str("leg"), py_str("legs"), py_str("drumstick"), py_str("drumsticks"), py_str("thigh"), py_str("thighs"), py_str("wing"), py_str("wings"), py_str("breast"), py_str("breasts"), py_str("fillet"), py_str("fillets"), py_str("joint"), py_str("shoulder"), py_str("rib"), py_str("ribs")]) }
    s_CUT_WORDS(__data77());
    fn __data78() -> Py { py_tuple(vec![py_str("something"), py_str("anything"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("suggestion"), py_str("quick"), py_str("quickly"), py_str("easy"), py_str("fast"), py_str("minutes"), py_str("minute"), py_str("half an hour"), py_str("hour"), py_str("slow cooker"), py_str("slow-cooker"), py_str("air fryer"), py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("what about"), py_str("how about"), py_str("what else"), py_str("what can i do"), py_str("what can i cook"), py_str("what can i put"), py_str("what shall i"), py_str("one pan"), py_str("one pot"), py_str("one tray"), py_str("tray bake"), py_str("traybake"), py_str("roasting tin")]) }
    s_IDEA_SHAPES(__data78());
    fn __data79() -> Py { py_tuple(vec![py_str("something"), py_str("anything"), py_str("only"), py_str("have"), py_str("got"), py_str("haven't"), py_str("wont"), py_str("want"), py_str("need"), py_str("make"), py_str("made"), py_str("cook"), py_str("cooker"), py_str("fryer"), py_str("slow"), py_str("air"), py_str("tomorrow"), py_str("fridge"), py_str("freezer"), py_str("chicken"), py_str("mince"), py_str("fish"), py_str("eggs"), py_str("egg"), py_str("pasta"), py_str("rice"), py_str("potatoes"), py_str("veg"), py_str("vegetables"), py_str("cheese"), py_str("bread"), py_str("kids"), py_str("us"), py_str("two"), py_str("quick"), py_str("easy"), py_str("cheap"), py_str("nice"), py_str("nicer"), py_str("special"), py_str("treat"), py_str("fancy"), py_str("dinner"), py_str("supper"), py_str("pan"), py_str("pot"), py_str("tray"), py_str("tin")]) }
    s_IDEA_CARRIERS(__data79());
    fn __data80() -> Py { py_tuple(vec![py_str("eat out"), py_str("eating out"), py_str("where to eat"), py_str("where should we eat"), py_str("where can we eat"), py_str("where do we eat"), py_str("restaurant"), py_str("restaurants"), py_str("book a table"), py_str("table for"), py_str("meal out"), py_str("dinner out"), py_str("lunch out"), py_str("brunch out"), py_str("somewhere to eat"), py_str("place to eat"), py_str("places to eat"), py_str("eat tonight"), py_str("eat out tonight"), py_str("meal tonight"), py_str("chippy"), py_str("chip shop"), py_str("takeaway"), py_str("take away"), py_str("take-away"), py_str("takeout"), py_str("take out"), py_str("order in"), py_str("delivery"), py_str("deliveroo"), py_str("just eat"), py_str("uber eats"), py_str("curry house"), py_str("kebab shop"), py_str("pizza place"), py_str("best pizza"), py_str("good pizza"), py_str("decent pizza"), py_str("pizza near me"), py_str("pizza nearby"), py_str("pizza shop"), py_str("pizza takeaway"), py_str("pizza delivery"), py_str("pizza restaurant"), py_str("tapas"), py_str("bistro"), py_str("where should we go"), py_str("where shall we go"), py_str("where to go"), py_str("somewhere nice"), py_str("somewhere good"), py_str("go out for a meal"), py_str("out for dinner"), py_str("out for a meal"), py_str("anywhere to eat"), py_str("anywhere good to eat"), py_str("anywhere nice to eat"), py_str("anywhere decent to eat"), py_str("good place to eat"), py_str("somewhere good to eat")]) }
    s_OUT_WORDS(__data80());
    fn __data81() -> Py { py_tuple(vec![py_str("cant decide"), py_str("can't decide"), py_str("cannot decide"), py_str("decide"), py_str("decided"), py_str("choose"), py_str("choosing"), py_str("pick"), py_str("picking"), py_str("fancy"), py_str("fancied"), py_str("fancy going"), py_str("craving"), py_str("in the mood"), py_str("what shall we"), py_str("what should we"), py_str("where shall we"), py_str("where should we"), py_str("where can we"), py_str("where should i"), py_str("what should i"), py_str("any ideas"), py_str("ideas"), py_str("suggest"), py_str("suggestion"), py_str("suggestions"), py_str("recommend"), py_str("recommendation"), py_str("recommendations"), py_str("same"), py_str("bored of"), py_str("bored with"), py_str("sick of"), py_str("treat"), py_str("this weekend"), py_str("near me"), py_str("nearby"), py_str("want a meal"), py_str("want to go out"), py_str("fancy a meal"), py_str("would like a meal"), py_str("could do with a meal"), py_str("somewhere to go"), py_str("anywhere"), py_str("anything good"), py_str("anywhere good"), py_str("anywhere nice"), py_str("anywhere decent"), py_str("how do i order"), py_str("how to order"), py_str("how do you order")]) }
    s_OUT_ASK(__data81());
    fn __data82() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("made"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("recipes"), py_str("at home"), py_str("from scratch"), py_str("fakeaway"), py_str("calories"), py_str("calorie"), py_str("healthy"), py_str("healthier"), py_str("diet"), py_str("bad for you"), py_str("fattening"), py_str("worth it"), py_str("cost"), py_str("cheaper"), py_str("save money"), py_str("budget"), py_str("sick"), py_str("ill"), py_str("food poisoning"), py_str("diarrhoea"), py_str("vomit"), py_str("allergy"), py_str("allergic"), py_str("gluten"), py_str("vegan"), py_str("vegetarian"), py_str("halal"), py_str("freeze"), py_str("frozen"), py_str("leftover"), py_str("reheat"), py_str("how long"), py_str("grow"), py_str("grown"), py_str("garden"), py_str("gym"), py_str("protein"), py_str("carbs"), py_str("wear"), py_str("wearing"), py_str("dress"), py_str("dress code"), py_str("outfit"), py_str("what to wear"), py_str("parcel"), py_str("parcels"), py_str("courier"), py_str("couriers"), py_str("seller"), py_str("tracking"), py_str("package"), py_str("packages"), py_str("not arrived"), py_str("never arrived"), py_str("late delivery"), py_str("delivery driver"), py_str("previous tenant"), py_str("refund"), py_str("late"), py_str("delayed"), py_str("delay"), py_str("arriving"), py_str("arrived"), py_str("arrive"), py_str("dispatched"), py_str("dispatch"), py_str("wrong item"), py_str("missing"), py_str("tip"), py_str("tips"), py_str("tipping"), py_str("service charge"), py_str("gratuity")]) }
    s_OUT_AVOID(__data82());
    s_DOUGH_STICK_LEAD(py_str("Sticking is nearly always a wet dough meeting a dry surface, so the fix is less water and more flour where the two meet. "));
    fn __data83() -> Py { py_tuple(vec![py_str("corner shop"), py_str("corner shop"), py_str("cornershop"), py_str("local shop"), py_str("little shop"), py_str("convenience store"), py_str("newsagent"), py_str("newsagents"), py_str("garage shop"), py_str("petrol station"), py_str("spar"), py_str("co-op"), py_str("coop"), py_str("one stop"), py_str("londis"), py_str("premier"), py_str("nisa"), py_str("budgens")]) }
    fn __data84() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("made"), py_str("make"), py_str("making"), py_str("eat"), py_str("dinner"), py_str("supper"), py_str("lunch"), py_str("ingredients"), py_str("meal"), py_str("recipe"), py_str("tonight"), py_str("stuff"), py_str("ingredient")]) }
    fn __data85() -> Py { py_tuple(vec![py_str("open"), py_str("opening"), py_str("closed"), py_str("closing"), py_str("hours"), py_str("refund"), py_str("complaint"), py_str("complain"), py_str("delivery"), py_str("deliver"), py_str("price"), py_str("prices"), py_str("cost"), py_str("cheaper"), py_str("near me"), py_str("nearest"), py_str("parking"), py_str("stock"), py_str("in stock"), py_str("sold out")]) }
    fn __data86() -> Py { py_tuple(vec![py_str("what about"), py_str("how about"), py_str("shall i"), py_str("should i"), py_str("go with"), py_str("alongside"), py_str("serve"), py_str("with"), py_str("side"), py_str("sides"), py_str("need"), py_str("any"), py_str("and")]) }
    fn __data87() -> Py { py_tuple(vec![py_str("rice"), py_str("potato"), py_str("potatoes"), py_str("salad"), py_str("lettuce"), py_str("bread"), py_str("naan"), py_str("pitta"), py_str("veg"), py_str("vegetables"), py_str("greens"), py_str("broccoli"), py_str("beans"), py_str("peas"), py_str("carrots"), py_str("gravy"), py_str("mash"), py_str("chips"), py_str("rolls"), py_str("slaw"), py_str("side"), py_str("sides")]) }
    fn __data88() -> Py { py_tuple(vec![py_str("how long"), py_str("how do i"), py_str("recipe"), py_str("allergic"), py_str("sticky"), py_str("mushy"), py_str("gluey"), py_str("leftover"), py_str("reheat"), py_str("water"), py_str("ratio"), py_str("freeze"), py_str("calories"), py_str("healthy"), py_str("protein"), py_str("carbs"), py_str("carb"), py_str("carbohydrates"), py_str("nutrition"), py_str("nutrients"), py_str("plant"), py_str("plants"), py_str("growing"), py_str("wrong"), py_str("yellow"), py_str("brown"), py_str("wilt"), py_str("dying"), py_str("soil"), py_str("garden"), py_str("basil")]) }
    fn __data90() -> Py { py_tuple(vec![py_str("rise"), py_str("rises"), py_str("rising"), py_str("prove"), py_str("proves"), py_str("proving"), py_str("proof"), py_str("starter"), py_str("yeast"), py_str("room temperature"), py_str("room temp"), py_str("fridge"), py_str("freezer"), py_str("serve"), py_str("serving"), py_str("reheat"), py_str("warm through"), py_str("internal"), py_str("inside"), py_str("safe"), py_str("done"), py_str("thermometer"), py_str("frying pan"), py_str("hob"), py_str("ring"), py_str("grill"), py_str("air fryer"), py_str("microwave"), py_str("slow cooker"), py_str("pressure cooker"), py_str("bain marie"), py_str("water bath"), py_str("sugar"), py_str("fever"), py_str("oven thermostat"), py_str("dial"), py_str("fan oven conversion"), py_str("gas to electric"), py_str("bread and butter pudding"), py_str("garlic bread"), py_str("soda bread"), py_str("banana bread"), py_str("bread sauce"), py_str("breadcrumbs"), py_str("bread crumbs"), py_str("breadsticks"), py_str("bread sticks"), py_str("naan bread"), py_str("pitta bread"), py_str("pita bread")]) }
    fn __data91() -> Py { py_tuple(vec![py_str("interview"), py_str("interviews"), py_str("interviewer"), py_str("plant"), py_str("plants"), py_str("houseplant"), py_str("houseplants"), py_str("pot plant"), py_str("soil"), py_str("compost"), py_str("garden"), py_str("lawn"), py_str("grass"), py_str("basil"), py_str("tomato"), py_str("cucumber"), py_str("chilli"), py_str("herb"), py_str("herbs"), py_str("mint"), py_str("coriander"), py_str("parsley"), py_str("watering"), py_str("repot"), py_str("wilt"), py_str("wilted"), py_str("prune"), py_str("fertiliser"), py_str("sleep"), py_str("slept"), py_str("bedtime"), py_str("nap"), py_str("exam"), py_str("exams"), py_str("revision"), py_str("revising"), py_str("worktop"), py_str("worktops"), py_str("counter"), py_str("countertop"), py_str("counter top"), py_str("sink"), py_str("basin"), py_str("bath"), py_str("enamel"), py_str("tile"), py_str("tiles"), py_str("grout"), py_str("plate"), py_str("plates"), py_str("mug"), py_str("mugs"), py_str("bowl"), py_str("bowls"), py_str("laminate"), py_str("veneer"), py_str("granite"), py_str("quartz"), py_str("skirting"), py_str("windscreen"), py_str("windshield")]) }
    fn __data92() -> Py { py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("hours"), py_str("temperature"), py_str("temp"), py_str("degrees"), py_str("gas mark"), py_str("why"), py_str("fix"), py_str("soggy"), py_str("dry"), py_str("burnt"), py_str("bland"), py_str("watery"), py_str("raw"), py_str("pink"), py_str("safe"), py_str("keep"), py_str("keeps"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("reheat"), py_str("leftover"), py_str("leftovers"), py_str("defrost"), py_str("store")]) }
    fn __data93() -> Py { py_tuple(vec![py_str("what can i"), py_str("what should i"), py_str("what do i"), py_str("what could i"), py_str("can i make"), py_str("can i cook"), py_str("should i make"), py_str("how do i"), py_str("any idea"), py_str("ideas for"), py_str("whats good"), py_str("what's good"), py_str("job"), py_str("jobs"), py_str("new job"), py_str("interview"), py_str("career"), py_str("shift"), py_str("shifts"), py_str("school"), py_str("term"), py_str("exam"), py_str("exams"), py_str("college"), py_str("university"), py_str("uni"), py_str("course"), py_str("class"), py_str("classes"), py_str("gym"), py_str("diet"), py_str("marathon"), py_str("dieting"), py_str("new year"), py_str("resolution"), py_str("resolutions")]) }
    fn __data94() -> Py { py_tuple(vec![py_str("sticking"), py_str("sticks"), py_str("sticky"), py_str("stick"), py_str("stuck"), py_str("tacky"), py_str("bench"), py_str("counter"), py_str("worktop"), py_str("hands"), py_str("banneton"), py_str("rolling pin"), py_str("surface")]) }
    fn __data95() -> Py { py_tuple(vec![py_str("tin"), py_str("tins"), py_str("pan"), py_str("pans"), py_str("paper"), py_str("parchment"), py_str("baking paper"), py_str("lining"), py_str("grease"), py_str("greasing"), py_str("cake"), py_str("sponge"), py_str("pasta"), py_str("spaghetti"), py_str("noodles"), py_str("rice"), py_str("boil"), py_str("boiling"), py_str("saucepan"), py_str("shape"), py_str("shaping"), py_str("too sticky"), py_str("handle"), py_str("tearing"), py_str("tears"), py_str("torn"), py_str("cracks"), py_str("crumbling"), py_str("dry"), py_str("split"), py_str("splits"), py_str("splitting"), py_str("splinter"), py_str("splinters"), py_str("splintered"), py_str("cracked"), py_str("rough"), py_str("sanded"), py_str("sand it"), py_str("recipe"), py_str("from scratch"), py_str("by hand")]) }
    fn __data96() -> Py { py_tuple(vec![py_str("meat"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("turkey"), py_str("steak"), py_str("water"), py_str("tap"), py_str("decision"), py_str("choice"), py_str("exam"), py_str("interview"), py_str("day"), py_str("week"), py_str("dried"), py_str("dry"), py_str("soak"), py_str("soaking"), py_str("soaked"), py_str("overnight"), py_str("bag"), py_str("bags"), py_str("tinned"), py_str("canned"), py_str("chickpeas"), py_str("lentils"), py_str("kidney beans"), py_str("butter beans"), py_str("haricot"), py_str("cannellini"), py_str("pinto"), py_str("split peas")]) }
    fn __data97() -> Py { py_tuple(vec![py_str("why"), py_str("always"), py_str("keeps"), py_str("comes"), py_str("come"), py_str("out"), py_str("wrong"), py_str("fix"), py_str("stop"), py_str("how"), py_str("do"), py_str("i"), py_str("my"), py_str("the"), py_str("is"), py_str("are"), py_str("it"), py_str("what"), py_str("can"), py_str("going"), py_str("every"), py_str("time"), py_str("make"), py_str("not"), py_str("so")]) }
    fn __data98() -> Py { py_tuple(vec![py_str("skin"), py_str("hands"), py_str("hair"), py_str("washing"), py_str("weather"), py_str("tumble"), py_str("dryer"), py_str("wine"), py_str("sense of humour"), py_str("hot water"), py_str("heating"), py_str("heating bill"), py_str("boiler"), py_str("electricity"), py_str("gas bill"), py_str("energy bill"), py_str("thermostat"), py_str("radiator")]) }
    fn __data99() -> Py { py_tuple(vec![py_str("i"), py_str("im"), py_str("am"), py_str("we"), py_str("them"), py_str("they"), py_str("guest"), py_str("guests"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("mates"), py_str("son"), py_str("daughter"), py_str("mum"), py_str("dad"), py_str("mother"), py_str("father"), py_str("sister"), py_str("brother"), py_str("partner"), py_str("wife"), py_str("husband"), py_str("one"), py_str("someone"), py_str("everyone"), py_str("doesnt"), py_str("doesn't"), py_str("wont"), py_str("won't"), py_str("cant"), py_str("can't"), py_str("avoids"), py_str("avoid"), py_str("has")]) }
    fn __data100() -> Py { py_tuple(vec![py_str("allergy"), py_str("allergic"), py_str("anaphylaxis"), py_str("epipen"), py_str("intolerance"), py_str("coeliac"), py_str("how do i"), py_str("how long"), py_str("do i have"), py_str("in the fridge"), py_str("in the cupboard"), py_str("celiac"), py_str("is it bad"), py_str("good for you"), py_str("calories"), py_str("protein"), py_str("vitamin"), py_str("healthy"), py_str("health"), py_str("weight"), py_str("pregnant"), py_str("pregnancy"), py_str("baby"), py_str("child"), py_str("children"), py_str("when i was"), py_str("used to"), py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("nursing"), py_str("lactating"), py_str("chestfeeding"), py_str("while im feeding"), py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("use by"), py_str("best before"), py_str("gone off"), py_str("gone sour"), py_str("smells off"), py_str("smells sour"), py_str("breastfeeding"), py_str("breast feeding"), py_str("nursing"), py_str("while feeding"), py_str("lactating"), py_str("chestfeeding"), py_str("antibiotic"), py_str("antibiotics"), py_str("penicillin"), py_str("amoxicillin"), py_str("doxycycline"), py_str("lymecycline"), py_str("tetracycline"), py_str("ciprofloxacin"), py_str("trimethoprim"), py_str("nitrofurantoin"), py_str("metronidazole"), py_str("statin"), py_str("statins"), py_str("simvastatin"), py_str("atorvastatin"), py_str("rosuvastatin"), py_str("pravastatin"), py_str("warfarin"), py_str("blood thinner"), py_str("blood thinners"), py_str("anticoagulant"), py_str("anticoagulants"), py_str("inr"), py_str("apixaban"), py_str("rivaroxaban"), py_str("levothyroxine"), py_str("thyroxine"), py_str("tablet"), py_str("tablets"), py_str("medicine"), py_str("medicines"), py_str("medication"), py_str("pills"), py_str("prescription"), py_str("can i eat"), py_str("can i have"), py_str("can i drink"), py_str("should i eat"), py_str("should i have"), py_str("is it ok to eat"), py_str("is it okay to eat"), py_str("is it fine to eat"), py_str("is it safe to eat"), py_str("am i allowed to eat")]) }
    fn __data101() -> Py { py_tuple(vec![py_str("healthy"), py_str("health"), py_str("protein"), py_str("iron"), py_str("b12"), py_str("deficiency"), py_str("vitamin"), py_str("child"), py_str("children"), py_str("baby"), py_str("weight"), py_str("muscle"), py_str("pregnant"), py_str("calcium")]) }
    fn __data102() -> Py { py_tuple(vec![py_str("allergy"), py_str("allergic"), py_str("anaphylaxis"), py_str("epipen"), py_str("intolerance"), py_str("coeliac"), py_str("celiac"), py_str("anaphylactic"), py_str("in the house"), py_str("in the fridge"), py_str("shopping"), py_str("buy"), py_str("calories"), py_str("protein"), py_str("is it bad"), py_str("fancy"), py_str("no fuss"), py_str("fuss"), py_str("plain"), py_str("basic"), py_str("cheap"), py_str("skint"), py_str("budget"), py_str("quick meal"), py_str("meal idea"), py_str("meal ideas"), py_str("meal tonight"), py_str("what can i make"), py_str("what can i cook"), py_str("what should i cook"), py_str("use up"), py_str("use it up"), py_str("leftover"), py_str("leftovers"), py_str("out of the fridge"), py_str("out of the freezer"), py_str("overnight"), py_str("all night"), py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("past their date"), py_str("use by"), py_str("best before"), py_str("gone past"), py_str("run out of"), py_str("ran out of"), py_str("running out of"), py_str("runs out of"), py_str("have run out"), py_str("has run out"), py_str("none left"), py_str("nothing left")]) }
    fn __data103() -> Py { py_tuple(vec![py_str("what"), py_str("shall"), py_str("should"), py_str("can"), py_str("make"), py_str("made"), py_str("cook"), py_str("serve"), py_str("goes"), py_str("go"), py_str("with"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("finish"), py_str("and"), py_str("about"), py_str("for"), py_str("need"), py_str("want"), py_str("have"), py_str("got"), py_str("fancy"), py_str("give"), py_str("tonight")]) }
    fn __data104() -> Py { py_tuple(vec![py_str("make"), py_str("made"), py_str("making"), py_str("cook"), py_str("cooked"), py_str("serve"), py_str("serving"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("something"), py_str("shall"), py_str("should"), py_str("can"), py_str("finish"), py_str("need"), py_str("needs"), py_str("want"), py_str("wants"), py_str("fancy"), py_str("give"), py_str("any"), py_str("and"), py_str("goes"), py_str("go"), py_str("pair"), py_str("pairs"), py_str("whats"), py_str("what's"), py_str("what are"), py_str("what else"), py_str("who wants")]) }
    fn __data105() -> Py { py_tuple(vec![py_str("run"), py_str("runs"), py_str("running"), py_str("jog"), py_str("jogging"), py_str("jogged"), py_str("exercise"), py_str("exercising"), py_str("exercised"), py_str("gym"), py_str("training"), py_str("workout"), py_str("work out"), py_str("broken"), py_str("broke"), py_str("fracture"), py_str("fractured"), py_str("cast"), py_str("plaster"), py_str("sprain"), py_str("sprained"), py_str("injury"), py_str("injured"), py_str("operation"), py_str("surgery"), py_str("heal"), py_str("healing"), py_str("recovery"), py_str("wrist"), py_str("ankle"), py_str("collarbone"), py_str("rib"), py_str("ribs"), py_str("hip"), py_str("shoulder"), py_str("stretch"), py_str("stretches"), py_str("stretching"), py_str("warm up"), py_str("warming up"), py_str("cool down"), py_str("run"), py_str("runs"), py_str("running"), py_str("jog"), py_str("jogging"), py_str("jogged"), py_str("exercise"), py_str("gym"), py_str("training"), py_str("workout"), py_str("ibuprofen"), py_str("paracetamol"), py_str("aspirin"), py_str("naproxen"), py_str("painkiller"), py_str("painkillers"), py_str("tablet"), py_str("tablets"), py_str("medicine"), py_str("medicines"), py_str("medication"), py_str("antibiotic"), py_str("antibiotics"), py_str("steroid"), py_str("steroids"), py_str("vitamin"), py_str("vitamins"), py_str("calorie"), py_str("calories"), py_str("healthy"), py_str("health"), py_str("sugar"), py_str("diabetes"), py_str("diabetic"), py_str("diabetics"), py_str("insulin"), py_str("blood sugar"), py_str("unhealthy"), py_str("bad for you"), py_str("bad for me"), py_str("is it bad"), py_str("bad to eat"), py_str("every day"), py_str("every night"), py_str("everyday"), py_str("diet"), py_str("tooth"), py_str("teeth"), py_str("sweet potato"), py_str("sweetcorn"), py_str("black pudding"), py_str("yorkshire"), py_str("wine"), py_str("wines"), py_str("port"), py_str("sherry"), py_str("liqueur"), py_str("gift"), py_str("present"), py_str("christmas pudding"), py_str("custard powder"), py_str("allergic"), py_str("allergy"), py_str("recipe"), py_str("how long"), py_str("how do i"), py_str("how to"), py_str("buy"), py_str("shop"), py_str("supermarket"), py_str("restaurant"), py_str("screen"), py_str("screens"), py_str("display"), py_str("phone"), py_str("iphone"), py_str("android"), py_str("laptop"), py_str("computer"), py_str("pc"), py_str("monitor"), py_str("telly"), py_str("tv"), py_str("timeout"), py_str("time out"), py_str("minutes"), py_str("locks"), py_str("lock screen"), py_str("brightness"), py_str("what i have"), py_str("i have"), py_str("i've got"), py_str("ive got"), py_str("nothing in"), py_str("cupboard"), py_str("leftover"), py_str("left over"), py_str("in the house"), py_str("no eggs"), py_str("stink"), py_str("stinks"), py_str("stinking"), py_str("stank"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("smelling"), py_str("reek"), py_str("reeks"), py_str("odour"), py_str("pong"), py_str("pongs"), py_str("musty"), py_str("fix"), py_str("salvage"), py_str("rescue"), py_str("ruined"), py_str("came out"), py_str("come out"), py_str("went wrong"), py_str("too sweet"), py_str("too dry"), py_str("too runny"), py_str("too wet"), py_str("too heavy"), py_str("soggy"), py_str("sunk"), py_str("sunken"), py_str("burnt"), py_str("burned"), py_str("flat"), py_str("failed"), py_str("split"), py_str("didnt set"), py_str("did not set"), py_str("stodgy"), py_str("why"), py_str("crave"), py_str("craves"), py_str("craving"), py_str("cravings"), py_str("savoury pudding"), py_str("savory pudding"), py_str("steak and kidney"), py_str("suet"), py_str("white pudding"), py_str("pease pudding"), py_str("career"), py_str("careers"), py_str("job"), py_str("jobs"), py_str("nursing"), py_str("nurse"), py_str("nurses"), py_str("midwife"), py_str("midwifery"), py_str("retrain"), py_str("retraining"), py_str("retrained"), py_str("apprenticeship"), py_str("college"), py_str("university"), py_str("degree"), py_str("qualification"), py_str("qualifications"), py_str("salary"), py_str("wage"), py_str("wages"), py_str("promotion"), py_str("interview"), py_str("probation")]) }
    fn __data106() -> Py { py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("place to buy"), py_str("where to buy"), py_str("where can i buy"), py_str("cheapest place"), py_str("price"), py_str("prices"), py_str("warranty"), py_str("guarantee"), py_str("frozen"), py_str("freezer"), py_str("freeze"), py_str("defrost"), py_str("defrosted"), py_str("thaw"), py_str("thawed"), py_str("easter egg"), py_str("easter eggs"), py_str("christmas cake"), py_str("christmas pudding"), py_str("play dough"), py_str("playdough"), py_str("decorate"), py_str("decorating"), py_str("dye"), py_str("dyeing"), py_str("paint"), py_str("painting"), py_str("smoking"), py_str("smokes"), py_str("sparking"), py_str("wont heat"), py_str("won't heat"), py_str("not heating"), py_str("stopped working"), py_str("wont switch on"), py_str("wont turn on"), py_str("leaking"), py_str("tripping"), py_str("dead"), py_str("no power"), py_str("wont come on"), py_str("has stopped working"), py_str("isnt heating"), py_str("won't heat"), py_str("wont heat up"), py_str("not heating up"), py_str("blowing the fuse"), py_str("raw"), py_str("reheat"), py_str("reheated"), py_str("left out"), py_str("gone off"), py_str("dessert"), py_str("pudding"), py_str("sweet"), py_str("cake"), py_str("biscuits"), py_str("cookies"), py_str("breakfast"), py_str("cold"), py_str("iced"), py_str("bored"), py_str("bored of"), py_str("sick of"), py_str("tired of"), py_str("fed up"), py_str("had enough"), py_str("something else"), py_str("anything else"), py_str("what else")]) }
    fn __data89() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-temp")); __args.push(py_tuple(vec![g__RECIPE_CARDS(), geti(&_temp_asks(&[]), &Py::Int(0))])); __args.push(py_func("_the_heat", |args: &[Py]| _the_heat(args))); __kws.extend(vec![("ask".to_string(), g_COOKING_WORDS()), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data90())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-followup")); __args.push(_asks(&[])); __args.push(py_func("_follow_up", |args: &[Py]| _follow_up(args))); __kws.extend(vec![("ask".to_string(), g_COOKING_WORDS()), ("bias".to_string(), op_neg(&Py::Int(2))), ("avoid".to_string(), __data91())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-keep")); __args.push(py_tuple(vec![op_add(&call_builtin("tuple", &[g_PLAN_NAMES()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[g_DISH_WORDS()]).unwrap_or_else(|e| exc::raise(e))), g_KEEP_ASK_WORDS()])); __args.push(py_func("_follow_up", |args: &[Py]| _follow_up(args))); __kws.extend(vec![("ask".to_string(), g_KEEP_ASK_WORDS()), ("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_DISH(), &g_NOT_A_KEEPING_QUESTION()), &g_KEEP_QUALITY_WORDS()), &py_tuple(vec![py_str("from sinking"), py_str("from going soggy"), py_str("from sticking"), py_str("from burning"), py_str("from going stale"), py_str("from collapsing"), py_str("from curdling"), py_str("from splitting"), py_str("sinking"), py_str("sank"), py_str("sunken")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-done")); __args.push(py_tuple(vec![g_DONE_POSSESSIVE(), g_DONE_WORD()])); __args.push(py_func("_follow_up", |args: &[Py]| _follow_up(args))); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), __data92())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-start")); __args.push(g_START_WORDS()); __args.push(py_func("_start", |args: &[Py]| _start(args))); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(3))), ("avoid".to_string(), __data93())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dough-sticking")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dough"), py_str("sourdough"), py_str("bread dough"), py_str("pastry"), py_str("banneton"), py_str("flour"), py_str("knead"), py_str("kneading"), py_str("rolling pin"), py_str("springform")]), __data94()])); __args.push(py_func("_dough_sticks", |args: &[Py]| _dough_sticks(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data95())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("veg-tough")); __args.push(py_tuple(vec![g_VEG_FAULTS(), g_VEG_NAMES()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); _veg_fix(&[_line_text(&[understood.clone()])]) })); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data96())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-problem")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PROBLEMS()) { let mut key: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&geti(&g_PROBLEMS(), &key.clone()), &Py::Int(0))) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), __data97()])); __args.push(py_func("_problem", |args: &[Py]| _problem(args))); __kws.extend(vec![("ask".to_string(), g_COOKING_WORDS()), ("guard".to_string(), py_func("a_plan_that_might_be_ruined", |args: &[Py]| a_plan_that_might_be_ruined(args))), ("bias".to_string(), op_neg(&Py::Int(2))), ("avoid".to_string(), __data98())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-diet")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DIETS()) { let __unpk712 = py_unpack(&__item.clone(), 3, None); let mut _n: Py = __unpk712[0].clone(); let mut words: Py = __unpk712[1].clone(); let mut _s: Py = __unpk712[2].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), __data99()])); __args.push(py_func("_diet", |args: &[Py]| _diet(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("not_a_mark_on_a_thing", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cleaning", "not_a_mark_on_a_thing", args))), ("avoid".to_string(), op_add(&__data100(), &pyrt::xrefs::xget("knowledge.lookup", "DAY_WORDS")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-veggie")); __args.push(py_tuple(vec![g_VEGGIE_KEY(), g_VEGGIE_CARRIERS()])); __args.push(py_func("_veggie", |args: &[Py]| _veggie(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("not_a_present", |args: &[Py]| pyrt::xrefs::xcall("knowledge.shopping", "not_a_present", args))), ("avoid".to_string(), __data101())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-dislike")); __args.push(py_tuple(vec![g_DISLIKE_SHAPES(), g_DISLIKED()])); __args.push(py_func("_dislike", |args: &[Py]| _dislike(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data102())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-pudding")); __args.push(py_tuple(vec![__data103(), g_DESSERT_WORDS()])); __args.push(py_func("_pudding", |args: &[Py]| _pudding(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), __data104()), ("avoid".to_string(), __data105())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-ideas")); __args.push(py_tuple(vec![g_IDEA_SHAPES(), g_IDEA_CARRIERS()])); __args.push(py_func("_idea", |args: &[Py]| _idea(args))); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(2))), ("avoid".to_string(), __data106())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    fn __data107() -> Py { py_tuple(vec![py_str("fry the spices"), py_str("frying the spices"), py_str("fry spices"), py_str("frying spices"), py_str("fry my spices"), py_str("fry the spice"), py_str("toast the spices"), py_str("toast spices"), py_str("toasting spices"), py_str("toasted spices"), py_str("toast my spices"), py_str("bloom the spices"), py_str("bloom spices"), py_str("blooming the spices"), py_str("blooming spices"), py_str("temper the spices"), py_str("tempering the spices"), py_str("whole spices"), py_str("cook the spices"), py_str("cooking the spices"), py_str("spices in oil"), py_str("spices first"), py_str("spices before"), py_str("spices go in"), py_str("spices at the start"), py_str("add the spices"), py_str("add my spices"), py_str("add spices"), py_str("add the spice"), py_str("spices to a curry"), py_str("spices to the pan"), py_str("when do i add the spices"), py_str("when do you add the spices"), py_str("when should the spices"), py_str("whole cumin"), py_str("cumin seeds"), py_str("cumin seed"), py_str("mustard seeds"), py_str("coriander seeds"), py_str("fennel seeds"), py_str("cardamom pods"), py_str("cardamom"), py_str("star anise"), py_str("cinnamon stick"), py_str("fenugreek"), py_str("ajwain"), py_str("curry leaves"), py_str("dried chillies")]) }
    fn __data108() -> Py { py_tuple(vec![py_str("fry"), py_str("frying"), py_str("fried"), py_str("toast"), py_str("toasting"), py_str("toasted"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("bloom"), py_str("blooming"), py_str("bloomed"), py_str("temper"), py_str("tempering"), py_str("do"), py_str("should"), py_str("can"), py_str("need"), py_str("have to"), py_str("must"), py_str("when"), py_str("first"), py_str("before"), py_str("how"), py_str("worth"), py_str("necessary"), py_str("order"), py_str("start"), py_str("starting"), py_str("begin"), py_str("beginning"), py_str("always")]) }
    fn __data109() -> Py { py_tuple(vec![py_str("which spices"), py_str("what spices"), py_str("how much spice"), py_str("what spice"), py_str("too spicy"), py_str("too hot"), py_str("spice up"), py_str("relationship"), py_str("mild"), py_str("hotter"), py_str("milder"), py_str("spicy"), py_str("pantry"), py_str("cupboard"), py_str("ran out"), py_str("havent got"), py_str("have not got"), py_str("no spices"), py_str("shop"), py_str("buy"), py_str("substitute"), py_str("instead of"), py_str("sow"), py_str("sowing"), py_str("plant"), py_str("planting"), py_str("grow"), py_str("growing"), py_str("germinate"), py_str("garden"), py_str("allotment"), py_str("soil"), py_str("compost"), py_str("prune"), py_str("flower"), py_str("flowers")]) }
    fn __data110() -> Py { py_tuple(vec![py_str("turkey"), py_str("chicken"), py_str("bird"), py_str("crown"), py_str("joint"), py_str("pork"), py_str("how"), py_str("should"), py_str("long"), py_str("overnight"), py_str("worth it"), py_str("need to"), py_str("have to")]) }
    fn __data111() -> Py { py_tuple(vec![py_str("gargle"), py_str("gargling"), py_str("sore throat"), py_str("throat"), py_str("wound"), py_str("cut"), py_str("saline"), py_str("rinse"), py_str("mouth"), py_str("fish tank"), py_str("aquarium"), py_str("pool"), py_str("swimming"), py_str("bath"), py_str("soak"), py_str("drink"), py_str("defrost"), py_str("thaw"), py_str("frozen"), py_str("freeze"), py_str("leftover"), py_str("leftovers"), py_str("stock"), py_str("carcass"), py_str("gravy"), py_str("stuffing"), py_str("stuffed"), py_str("dry"), py_str("juicy"), py_str("calories"), py_str("price"), py_str("how long do i roast"), py_str("how long to roast"), py_str("temperature")]) }
    fn __data112() -> Py { py_tuple(vec![py_str("soggy"), py_str("limp"), py_str("soft"), py_str("went soft"), py_str("stodgy"), py_str("dense"), py_str("heavy"), py_str("chewy"), py_str("rubbery"), py_str("fell apart"), py_str("falling apart"), py_str("falls apart"), py_str("crumbled"), py_str("crumbles"), py_str("crumbling"), py_str("dry"), py_str("crumbly"), py_str("tough"), py_str("wont set"), py_str("won't set"), py_str("didnt set"), py_str("didn't set"), py_str("not set"), py_str("runny"), py_str("sloppy"), py_str("sticky"), py_str("didnt rise"), py_str("didn't rise"), py_str("no rise"), py_str("flat"), py_str("pale"), py_str("soggy middles")]) }
    fn __data113() -> Py { py_tuple(vec![py_str("how do i make"), py_str("how to make"), py_str("recipe"), py_str("keep"), py_str("keeps"), py_str("store"), py_str("storing"), py_str("freeze"), py_str("freezing"), py_str("frozen"), py_str("reheat"), py_str("calories"), py_str("healthy"), py_str("how long do"), py_str("how long to"), py_str("topping"), py_str("toppings"), py_str("fill"), py_str("filling"), py_str("filling the"), py_str("iron wont heat"), py_str("iron does not heat"), py_str("burnt"), py_str("burning"), py_str("burns"), py_str("stale"), py_str("stales")]) }
    fn __data114() -> Py { py_tuple(vec![py_str("hide"), py_str("hiding"), py_str("hidden"), py_str("sneak"), py_str("sneaking"), py_str("sneaked"), py_str("disguise"), py_str("smuggle"), py_str("hide veg"), py_str("hide vegetables"), py_str("hide the veg"), py_str("hide the vegetables"), py_str("hiding veg"), py_str("hiding vegetables"), py_str("hidden veg"), py_str("hidden vegetables"), py_str("way to hide vegetables"), py_str("ways to hide vegetables"), py_str("way to hide veg"), py_str("hide them in"), py_str("hide it in"), py_str("sneak them in"), py_str("sneak it in"), py_str("hide them"), py_str("does that work"), py_str("does it work"), py_str("would that work"), py_str("would it work"), py_str("will that work")]) }
    fn __data115() -> Py { py_tuple(vec![py_str("them in food"), py_str("them in a sauce"), py_str("them in the sauce"), py_str("it in food"), py_str("them in his food"), py_str("them in her food"), py_str("them in their food"), py_str("them in a meal"), py_str("them in meals"), py_str("them in dinners"), py_str("them in dinner"), py_str("sauce"), py_str("lasagne"), py_str("bolognese"), py_str("chilli"), py_str("soup"), py_str("mash"), py_str("muffin"), py_str("muffins"), py_str("curry"), py_str("stew"), py_str("pasta"), py_str("smoothie"), py_str("omelette"), py_str("burger"), py_str("burgers"), py_str("meatballs"), py_str("pie"), py_str("risotto")]) }
    fn __data116() -> Py { py_tuple(vec![py_str("hide and seek"), py_str("hide the presents"), py_str("hide presents"), py_str("hide a key"), py_str("hide keys"), py_str("spare key"), py_str("hide money"), py_str("hide cash"), py_str("hide the money"), py_str("hide the wires"), py_str("wires"), py_str("cables"), py_str("hide the mess"), py_str("hide a mess"), py_str("stains"), py_str("hide a spot"), py_str("hide spots"), py_str("acne"), py_str("bruise"), py_str("bruises"), py_str("scar"), py_str("scars"), py_str("dark circles"), py_str("grey hair"), py_str("hide my belly"), py_str("hide my tummy"), py_str("homework"), py_str("evidence"), py_str("hide the smell"), py_str("smell of weed"), py_str("hide from"), py_str("tablet"), py_str("tablets"), py_str("worming"), py_str("medicine"), py_str("medication"), py_str("pill"), py_str("pills"), py_str("birthday"), py_str("surprise"), py_str("presents"), py_str("party bag")]) }
    s_ENTRIES(op_add(&op_add(&py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cook-from-a-corner-shop")); __args.push(py_tuple(vec![__data83(), __data84()])); __args.push(py_str("A corner shop covers more of a dinner than people expect: rice, pasta, eggs, tinned beans and chickpeas, tinned tomatoes, frozen peas, onions, potatoes, milk, cheese and a curry powder or stock cubes. What it will not have is fresh herbs, good bread or anything unusual, so take the dried or tinned version rather than going without. Rice, eggs and frozen peas make fried rice in 15 minutes, and a tin of chickpeas with an onion and curry powder is a curry in 25.")); __kws.extend(vec![("avoid".to_string(), __data85())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan-sides")); __args.push(py_tuple(vec![__data86(), __data87()])); __args.push(py_func("_sides", |args: &[Py]| _sides(args))); __kws.extend(vec![("avoid".to_string(), __data88()), ("bias".to_string(), op_neg(&Py::Int(2)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]), &__data89()), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spice-bloom")); __args.push(py_tuple(vec![__data107(), __data108()])); __args.push(py_str("Fry them first: whole spices go into a little oil over a medium heat for 30 seconds to a minute, until they smell warm and one or two start to pop, because the oil carries the flavour that water cannot. Ground spices need only 20 to 30 seconds, added with the onion or just before the liquid goes in. Keep the heat moderate and have the next ingredient ready: a spice that darkens has gone bitter and cannot be rescued.")); __kws.extend(vec![("avoid".to_string(), __data109())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("brine-bird")); __args.push(py_tuple(vec![py_tuple(vec![py_str("brine"), py_str("brining"), py_str("brined"), py_str("wet brine"), py_str("wet brined")]), __data110()])); __args.push(py_str("Dissolve about 50g of salt in a litre of cold water - roughly 2 litres for a whole bird - and keep it in the fridge for an hour for every kg, or overnight for a chicken. A big pan or a clean bag works, with something on top to keep the bird under. Dry the skin well before it goes in the oven and add no more salt; a brined bird browns faster, so cover the top with foil for the first half hour.")); __kws.extend(vec![("avoid".to_string(), __data111()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bake-fault-more")); __args.push(py_tuple(vec![py_tuple(vec![py_str("waffle"), py_str("waffles"), py_str("flapjack"), py_str("flapjacks"), py_str("oat bar"), py_str("oat bars"), py_str("scone"), py_str("scones"), py_str("brownie"), py_str("brownies")]), __data112()])); __args.push(py_func("_bake_fault_more", |args: &[Py]| _bake_fault_more(args))); __kws.extend(vec![("avoid".to_string(), __data113())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hidden-veg")); __args.push(py_tuple(vec![__data114(), op_add(&g_VEG_WORDS(), &__data115())])); __args.push(py_str("Grate or blitz it into something wet and strongly flavoured: a grated carrot or courgette in bolognese, chilli, lasagne or a tomato sauce disappears, and a handful of spinach or a few peas blitzed into soup leaves nothing to pick out. Mash takes half cauliflower, and grated carrot or beetroot goes into muffins and a fruit loaf unnoticed. Keep the plain veg on the plate too - hiding gets the vitamins in, but it does not teach anyone to like it.")); __kws.extend(vec![("avoid".to_string(), __data116()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data117() -> Py { py_tuple(vec![py_str("birthday"), py_str("party"), py_str("kids"), py_str("children"), py_str("christening"), py_str("wedding"), py_str("funeral"), py_str("wake"), py_str("buffet"), py_str("how much food"), py_str("how many"), py_str("shopping"), py_str("supermarket"), py_str("list"), py_str("meal plan"), py_str("meal prep"), py_str("holiday"), py_str("holidays"), py_str("weekend away"), py_str("night out"), py_str("pub crawl"), py_str("few days"), py_str("couple of days"), py_str("for a few days"), py_str("days away"), py_str("week in"), py_str("concert"), py_str("cinema"), py_str("what to watch"), py_str("chippy"), py_str("chip shop"), py_str("curry house"), py_str("kebab"), py_str("chinese"), py_str("indian"), py_str("thai"), py_str("deliveroo"), py_str("just eat"), py_str("uber eats"), py_str("fish and chips"), py_str("what should i get"), py_str("what should we get"), py_str("what do i get"), py_str("what do we get"), py_str("what should i order"), py_str("what should we order"), py_str("what to order"), py_str("what to get"), py_str("what should i eat"), py_str("what should we eat"), py_str("what to eat"), py_str("what should i have"), py_str("what should we have"), py_str("whats good"), py_str("whats nice"), py_str("what do you recommend")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eating-out-choose")); __args.push(py_tuple(vec![g_OUT_WORDS(), g_OUT_ASK()])); __args.push(py_str("Decide in this order, because each one removes half the list: set a budget per head and a distance you will actually travel, then pick a place whose menu is short - eight good dishes beats forty average ones - and read the reviews from the last month, not the star average. If two are still level, choose the one you would be sorry to lose; if neither would bother you, the cheaper one is fine. Still stuck, book a table rather than a takeaway: a table settles it and you eat something that is not from a box.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_OUT_AVOID(), &__data117()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data118() -> Py { py_tuple(vec![py_str("pub"), py_str("restaurant"), py_str("restaurants"), py_str("cafe"), py_str("canteen"), py_str("food court"), py_str("bistro"), py_str("brasserie"), py_str("carvery"), py_str("nandos"), py_str("wetherspoons"), py_str("sushi"), py_str("italian"), py_str("mexican"), py_str("greek"), py_str("turkish"), py_str("japanese"), py_str("korean"), py_str("vietnamese"), py_str("spanish"), py_str("french"), py_str("pizza"), py_str("burger"), py_str("tapas"), py_str("ramen"), py_str("noodle bar"), py_str("menu"), py_str("the menu"), py_str("set menu"), py_str("tasting menu"), py_str("lunch menu"), py_str("dinner menu"), py_str("specials"), py_str("specials board"), py_str("wine list"), py_str("a la carte"), py_str("prix fixe")]) }
    s_ORDER_AT_A_PLACE(__data118());
    fn __data119() -> Py { py_tuple(vec![py_str("what should i order"), py_str("what should we order"), py_str("what to order"), py_str("what do i order"), py_str("what shall i order"), py_str("what should i get"), py_str("what should we get"), py_str("what do i get"), py_str("what should i eat"), py_str("what should we eat"), py_str("what to eat"), py_str("what should i have"), py_str("what should we have"), py_str("whats good"), py_str("whats nice"), py_str("whats best"), py_str("anything good"), py_str("anything nice"), py_str("what do you recommend"), py_str("what would you recommend"), py_str("what would you get"), py_str("what would you order"), py_str("whats your pick"), py_str("help me pick"), py_str("help me choose"), py_str("help me decide"), py_str("cant choose"), py_str("can't choose"), py_str("cannot choose"), py_str("cant decide"), py_str("can't decide"), py_str("what would you pick"), py_str("what do you pick"), py_str("what should i choose"), py_str("what should we choose")]) }
    s_ORDER_AT_A_PLACE_ASK(__data119());
    fn __data120() -> Py { py_tuple(vec![py_str("takeaway"), py_str("take away"), py_str("take-away"), py_str("takeout"), py_str("take out"), py_str("order in"), py_str("delivery"), py_str("deliveroo"), py_str("just eat"), py_str("uber eats"), py_str("chippy"), py_str("chip shop"), py_str("curry house"), py_str("kebab"), py_str("fish and chips"), py_str("for the week"), py_str("this week"), py_str("next week"), py_str("weekly"), py_str("plan"), py_str("planning"), py_str("meal plan"), py_str("party"), py_str("birthday"), py_str("christmas"), py_str("wedding"), py_str("buffet"), py_str("design"), py_str("print"), py_str("printing"), py_str("my cafe"), py_str("my restaurant"), py_str("my pub"), py_str("pricing"), py_str("profit"), py_str("allergen"), py_str("allergens")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("order-at-a-place")); __args.push(py_tuple(vec![g_ORDER_AT_A_PLACE(), g_ORDER_AT_A_PLACE_ASK()])); __args.push(py_str("Order the thing you would not cook at home, and one shared starter for the middle: a main each and a side between two is plenty. Ask what is good tonight - the kitchen knows what came in fresh - and say yes to the dish you cannot quite pronounce.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data120(), &g_OUT_AVOID()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data121() -> Py { py_tuple(vec![py_str("what is"), py_str("what are"), py_str("what does"), py_str("what do"), py_str("meaning"), py_str("mean"), py_str("difference"), py_str("different"), py_str("explain"), py_str("explained"), py_str("whats the"), py_str("cheaper"), py_str("worth it"), py_str("worth"), py_str("value"), py_str("better value"), py_str("best value"), py_str("more expensive")]) }
    fn __data122() -> Py { py_tuple(vec![py_str("what should i"), py_str("what should we"), py_str("what shall i"), py_str("what shall we"), py_str("what do i order"), py_str("what to order"), py_str("whats good"), py_str("whats nice"), py_str("recommend"), py_str("pick"), py_str("choose"), py_str("cant decide"), py_str("can't decide")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-the-menu-words-mean")); __args.push(py_tuple(vec![py_tuple(vec![py_str("set menu"), py_str("set menus"), py_str("a la carte"), py_str("prix fixe"), py_str("tasting menu"), py_str("tasting menus"), py_str("table d'hote"), py_str("table d'hôte")]), __data121()])); __args.push(py_str("A set menu is one fixed price for two or three courses chosen from a short list, and it is usually the best value on the board. A la carte is the full list with every dish priced on its own, so you pay for exactly what you want and nothing else. A tasting menu is a run of small courses the kitchen decides, priced per person and taken by the whole table. If you want the cheapest good meal, order the set menu; if there is one dish you really came for, order that a la carte.")); __kws.extend(vec![("avoid".to_string(), __data122()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data123() -> Py { py_tuple(vec![py_str("takeaway"), py_str("take away"), py_str("take-away"), py_str("takeout"), py_str("take out"), py_str("order in"), py_str("delivery"), py_str("deliveroo"), py_str("just eat"), py_str("uber eats"), py_str("chippy"), py_str("chip shop"), py_str("curry house"), py_str("kebab"), py_str("chinese"), py_str("indian"), py_str("thai"), py_str("fish and chips")]) }
    fn __data124() -> Py { py_tuple(vec![py_str("what should i"), py_str("what should we"), py_str("what do i"), py_str("what can i"), py_str("what shall i"), py_str("what shall we"), py_str("what to order"), py_str("what to get"), py_str("whats nice"), py_str("whats good"), py_str("whats best"), py_str("whats the best"), py_str("what is nice"), py_str("what is good"), py_str("what is best"), py_str("what are the best"), py_str("best thing"), py_str("good thing"), py_str("nicest"), py_str("tastiest"), py_str("anything nice"), py_str("anything good"), py_str("what should i get"), py_str("what should we get"), py_str("should we get"), py_str("should i get"), py_str("what would you"), py_str("same"), py_str("bored of"), py_str("sick of"), py_str("cant decide"), py_str("can't decide"), py_str("decide"), py_str("choose"), py_str("pick"), py_str("fancy"), py_str("craving"), py_str("recommend"), py_str("ideas"), py_str("suggest")]) }
    fn __data125() -> Py { py_tuple(vec![py_str("calories"), py_str("how many"), py_str("much"), py_str("how long"), py_str("keep"), py_str("reheat"), py_str("freeze"), py_str("leftover"), py_str("recipe"), py_str("birthday"), py_str("christening"), py_str("funeral"), py_str("wake"), py_str("tonight"), py_str("or cook"), py_str("or not"), py_str("twice a week"), py_str("every week"), py_str("at the weekend"), py_str("how often"), py_str("healthy")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("takeaway-order")); __args.push(py_tuple(vec![__data123(), __data124()])); __args.push(py_str("Order the thing you would not cook at home, and get one more dish than there are people so there is something left cold for tomorrow: a shared starter, a main each, one side for two people. On a Chinese that is a soup or the salt and pepper ribs, two mains with rice between them, and a vegetable; on an Indian it is a dry dish and a wet one, a bread and a rice, not three curries; on a chippy it is one large chips for two. Collect it if you can - it arrives hot instead of steamed - and ask for sauces on the side.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_OUT_AVOID(), &__data125()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__TWO_OVEN_HEAD(py_str("Yes, if they want heat within about 20C of each other: put the hotter dish in first, or give the slower one a head start, and swap the two trays on their shelves halfway through."));
    s__TWO_OVEN_GAP(py_str("Leave a finger's gap round each tray, keep the one that needs to brown in the middle, and add 5 to 10 minutes because two trays cut the heat - a fan oven copes better than a gas one."));
    s__TWO_OVEN_CAKE(py_str("A delicate cake should not go in beside a roast, because it takes on the fat and the steam: bake the cake in the morning and put the roast in later, or the other way round."));
    s__TWO_OVEN_SPONGE(py_str("A 20cm sponge needs 25 to 30 minutes at 180C, so it can come out while the roast rests."));
    s__TWO_OVEN_BEEF(py_tuple(vec![py_str("beef"), py_str("lamb"), py_str("ham"), py_str("pork"), py_str("joint"), py_str("meat")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cake-size")); __args.push(pyrt::xrefs::xget("knowledge.cakes", "CAKE_KEY")); __args.push(py_func("cake_plan", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cakes", "cake_plan", args))); __kws.extend(vec![("avoid".to_string(), pyrt::xrefs::xget("knowledge.cakes", "CAKE_AVOID")), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cake-eggs")); __args.push(pyrt::xrefs::xget("knowledge.cakes", "CAKE_EGGS_KEY")); __args.push(py_func("cake_eggs", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cakes", "cake_eggs", args))); __kws.extend(vec![("avoid".to_string(), pyrt::xrefs::xget("knowledge.cakes", "CAKE_EGGS_AVOID")), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data126() -> Py { py_tuple(vec![py_str("two dishes"), py_str("two things in the oven"), py_str("two trays in the oven"), py_str("two tins in the oven"), py_str("both dishes"), py_str("one oven"), py_str("same oven"), py_str("two things"), py_str("two trays at once"), py_str("two tins at once"), py_str("two dishes at once"), py_str("two cakes at once"), py_str("two sponges at once"), py_str("two tins of cake"), py_str("two cake tins"), py_str("share an oven"), py_str("share the oven"), py_str("both in the oven"), py_str("a roast and a cake"), py_str("roast and a cake")]) }
    fn __data127() -> Py { py_tuple(vec![py_str("oven"), py_str("cook"), py_str("cooking"), py_str("bake"), py_str("baking"), py_str("roast"), py_str("roasting"), py_str("tray"), py_str("trays"), py_str("tin"), py_str("tins"), py_str("shelf"), py_str("shelves"), py_str("fit")]) }
    fn __data128() -> Py { py_tuple(vec![py_str("air fryer"), py_str("airfryer"), py_str("microwave"), py_str("slow cooker"), py_str("slow-cooker"), py_str("pressure"), py_str("not heating"), py_str("wont heat"), py_str("won't heat"), py_str("not working"), py_str("broken"), py_str("smell"), py_str("smoke"), py_str("door"), py_str("light"), py_str("element"), py_str("fan"), py_str("noisy"), py_str("clean"), py_str("cleaning"), py_str("christmas"), py_str("buffet"), py_str("barbecue"), py_str("bbq"), py_str("freezer"), py_str("fridge"), py_str("freeze"), py_str("reheat"), py_str("defrost"), py_str("temperature"), py_str("gas mark"), py_str("hot"), py_str("fan oven"), py_str("potatoes"), py_str("roast potatoes"), py_str("chicken"), py_str("scones"), py_str("pizza"), py_str("pizzas"), py_str("bread"), py_str("salmon"), py_str("sausages"), py_str("sausage rolls"), py_str("yorkshire"), py_str("chips"), py_str("lasagne"), py_str("casserole"), py_str("vegetables"), py_str("muffins"), py_str("cookies"), py_str("biscuits"), py_str("two things together")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("two-dishes-one-oven")); __args.push(py_tuple(vec![__data126(), __data127()])); __args.push(py_func("_two_dishes_plan", |args: &[Py]| _two_dishes_plan(args))); __kws.extend(vec![("avoid".to_string(), __data128()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data129() -> Py { py_tuple(vec![py_tuple(vec![py_str("yorkshire-pudding"), py_tuple(vec![py_str("yorkshire pudding"), py_str("yorkshire puddings"), py_str("yorkies"), py_str("yorkie puds"), py_str("yorkshire puds")]), py_str("A Yorkshire pudding is a batter of plain flour, eggs and milk baked in a thin layer of smoking-hot beef dripping or oil, so it puffs up crisp at the edges and soft in the middle. The fat has to be almost smoking before the batter goes in and the oven door stays shut for the first twenty minutes, or it will not rise. It is served with roast beef and gravy, and the batter makes toad in the hole around sausages.")]), py_tuple(vec![py_str("toad-in-the-hole"), py_tuple(vec![py_str("toad in the hole"), py_str("toad in the hole")]), py_str("Toad in the hole is sausages baked in a Yorkshire pudding batter, in one roasting tin, so the batter rises around them and the sausages flavour it. Brown the sausages first in the tin with a little oil, get that fat hot, then pour the batter in and bake at 220C (200C fan) for 25 to 30 minutes. Onion gravy over the top is the usual thing.")]), py_tuple(vec![py_str("black-pudding"), py_tuple(vec![py_str("black pudding"), py_str("black puddings")]), py_str("Black pudding is a sausage of pig's blood, pork fat, oatmeal and seasoning, sliced thick and fried until the outside crisps - it is a blood sausage, not a dessert. Stornoway and Bury are the ones people argue about. It is eaten with a full breakfast, or crumbled into scallops and apple.")]), py_tuple(vec![py_str("steak-and-kidney-pudding"), py_tuple(vec![py_str("steak and kidney pudding"), py_str("steak and kidney puddings")]), py_str("Steak and kidney pudding is diced beef and kidney with gravy, sealed under a suet pastry crust and steamed for three or four hours rather than baked, then turned out onto the plate. The suet crust is what makes it a pudding: it stays light and soaks up the gravy, where a shortcrust would go heavy. If you want it brown on top, the pastry was baked and that is a pie.")]), py_tuple(vec![py_str("bara-brith"), py_tuple(vec![py_str("bara brith"), py_str("bara brith bread")]), py_str("Bara brith means speckled bread in Welsh: a fruit loaf of dried fruit, brown sugar and mixed spice, usually with the fruit soaked overnight in strong tea first. It is baked in a loaf tin like a cake, left a day to settle, then sliced and buttered. It keeps a week in a tin and is better on the second day.")]), py_tuple(vec![py_str("parkin"), py_tuple(vec![py_str("parkin"), py_str("parkin cake"), py_str("yorkshire parkin")]), py_str("Parkin is a sticky ginger cake from Yorkshire and Lancashire, made with oatmeal, black treacle, golden syrup and ground ginger - oats in a cake is what makes it chewy rather than spongy. It comes out soft and improves for three or four days in a tin, so it is made for bonfire night and eaten the week after. Cut it into squares, not slices.")]), py_tuple(vec![py_str("cobbler"), py_tuple(vec![py_str("cobbler"), py_str("fruit cobbler"), py_str("peach cobbler")]), py_str("A cobbler is fruit baked under rough spoonfuls of scone dough, so the topping comes out cobbled rather than rolled flat - it is American, and it is not a pie or a crumble. The fruit should be bubbling at the edges and the topping golden, and the dough wants a spoonful of sugar and a little salt in it. Cream or custard over the top, hot.")]), py_tuple(vec![py_str("stovies"), py_tuple(vec![py_str("stovies"), py_str("stovie")]), py_str("Stovies is a Scottish one-pot of potatoes, onions and leftover roast meat cooked slowly in dripping or stock until the potatoes break down - the name comes from stoving, cooking slowly in a closed pot. It is made from what is left of a Sunday roast, so it is thrifty by design. Serve it with oatcakes and a pickled beetroot on the side.")]), py_tuple(vec![py_str("colcannon"), py_tuple(vec![py_str("colcannon")]), py_str("Colcannon is mashed potato with cooked cabbage or kale folded through it and plenty of butter, and it is a Halloween dish in Ireland - a ring or a coin used to be hidden in the pan. Champ is the same idea made with spring onions instead of cabbage. Warm milk and butter go in last, off the heat, or the mash goes gluey.")]), py_tuple(vec![py_str("frittata"), py_tuple(vec![py_str("frittata"), py_str("frittatas")]), py_str("A frittata is an Italian open omelette of eggs and whatever is in the fridge, cooked slowly in a pan and then finished under the grill or in the oven so the top sets without the bottom burning. It is thicker than an omelette and cut in wedges, and it is as good cold the next day. Cook the vegetables first and drain anything wet, or it will weep.")]), py_tuple(vec![py_str("tagine"), py_tuple(vec![py_str("tagine"), py_str("tagines"), py_str("tajine")]), py_str("A tagine is a Moroccan stew named after the conical clay pot it is cooked in: meat, usually lamb or chicken, with spices, dried fruit and often preserved lemon or olives, cooked low and slow so it comes out sweet and savoury at once. A heavy pan with a lid does the same job in an oven at 160C. Couscous or flatbread to mop it up.")]), py_tuple(vec![py_str("ragu"), py_tuple(vec![py_str("ragu"), py_str("ragù"), py_str("ragus"), py_str("bolognese sauce"), py_str("bolognese")]), py_str("A ragu is a slow-cooked Italian meat sauce: onion, carrot and celery softened first, then mince or a whole piece of meat, wine, a little milk and tomatoes, simmered for two hours or more - bolognese is one of them. The long cook and the milk are what make it rich rather than sharp. In Bologna it is served with tagliatelle, because the ragù clings to the wide strands.")]), py_tuple(vec![py_str("pilaf"), py_tuple(vec![py_str("pilaf"), py_str("pilau"), py_str("pilaff"), py_str("pilafs")]), py_str("A pilaf is rice fried in a little fat with onion first, then cooked in measured stock with the lid on and left alone, so every grain stays separate - where a risotto is stirred and a plain rice is boiled and drained. One part rice to one and a half parts stock, twelve minutes, then ten minutes off the heat. Dried fruit, nuts or cumin go in at the frying stage.")]), py_tuple(vec![py_str("fricassee"), py_tuple(vec![py_str("fricassee"), py_str("fricassée"), py_str("fricasseed chicken")]), py_str("A fricassee is chicken, rabbit or veal cut into pieces, browned lightly and then stewed in stock and finished with a white sauce of cream or egg yolk - a French method, and a mild, pale dish rather than a brown stew. It is what to cook when someone wants something gentle. Take it off the heat before the egg yolk goes in, or it curdles.")]), py_tuple(vec![py_str("gratin"), py_tuple(vec![py_str("gratin"), py_str("gratins"), py_str("au gratin")]), py_str("A gratin is anything topped with cheese, butter and breadcrumbs and browned under the grill or in a hot oven until the top is crisp - gratin dauphinois is potato and cream cooked for two hours, and a gratin of pasta or cauliflower works the same way. The browned top is the point, so give it the last five minutes uncovered. Breadcrumbs on their own brown more evenly than cheese.")]), py_tuple(vec![py_str("coulis"), py_tuple(vec![py_str("coulis"), py_str("fruit coulis")]), py_str("A coulis is a smooth sauce of raw or lightly cooked fruit blended and then pushed through a sieve, spooned over ice cream, cheesecake or pancakes - raspberry and mango are the usual ones. Sieving is what makes it a coulis rather than a purée, and a squeeze of lemon keeps the colour bright. A spoonful of icing sugar if the fruit is sharp.")]), py_tuple(vec![py_str("casserole"), py_tuple(vec![py_str("casserole"), py_str("casseroles"), py_str("hotpot"), py_str("hot pot")]), py_str("A casserole is meat and vegetables cooked slowly in liquid in one lidded pot, in the oven or on the hob - the point is a cheap cut and a long cook, an hour and a half to two hours at 160C, so shin, shoulder and thighs come out tender. Brown the meat first and keep the liquid to about a third of the way up, or it turns into soup. Dumplings go on top for the last twenty minutes; a casserole dish is the pot it is cooked in, not the food.")]), py_tuple(vec![py_str("bubble-and-squeak"), py_tuple(vec![py_str("bubble and squeak"), py_str("bubble n squeak")]), py_str("Bubble and squeak is leftover potatoes and cabbage or sprouts crushed together and fried in a hot pan until the underside is crusty brown - the name is the noise it made in the pan. Cold potatoes work better than fresh, because they have dried out. A fried egg on top makes it breakfast.")]), py_tuple(vec![py_str("spotted-dick"), py_tuple(vec![py_str("spotted dick")]), py_str("Spotted dick is a steamed suet pudding with currants and raisins in it - the spots are the dried fruit - served with custard. It is made in a pudding basin, covered and steamed for an hour and a half rather than baked, which is why it is light rather than heavy. A spoon of golden syrup in the bottom of the basin is the usual trick.")]), py_tuple(vec![py_str("beef-wellington"), py_tuple(vec![py_str("beef wellington"), py_str("wellington beef")]), py_str("Beef Wellington is a fillet of beef seared, coated in mushroom duxelles and sometimes pâté, wrapped in puff pastry and baked, then rested and carved in thick slices. The trick is chilling it after wrapping so the pastry does not go soggy, and a hot oven, 200C (180C fan), for about 25 minutes for medium rare. It is a dish for a celebration: the fillet is the expensive cut and it does not want cooking through.")]), py_tuple(vec![py_str("scotch-egg"), py_tuple(vec![py_str("scotch egg"), py_str("scotch eggs")]), py_str("A Scotch egg is a hard-boiled egg wrapped in sausage meat, rolled in breadcrumbs and deep-fried, so the shell of meat keeps a little juice in and the yolk stays soft if it is boiled for six minutes first. The ones from a good butcher are a different thing from the chilled ones in a supermarket. Baked at 200C for 25 minutes works if you would rather not fry.")]), py_tuple(vec![py_str("banoffee-pie"), py_tuple(vec![py_str("banoffee pie"), py_str("banoffi pie")]), py_str("Banoffee pie is a biscuit-and-butter base, a thick layer of toffee made by boiling a tin of condensed milk, sliced bananas and whipped cream on top, and none of it is baked. The toffee is the work: three hours in a pan of simmering water, and let the tin cool before opening it. Chill the pie for a few hours or it slumps when you cut it.")]), py_tuple(vec![py_str("coronation-chicken"), py_tuple(vec![py_str("coronation chicken")]), py_str("Coronation chicken is cold cooked chicken in a mild curry and mayonnaise sauce with apricot, mango chutney or sultanas - a dish invented for the Queen's coronation in 1953, so it is meant to be gentle rather than hot. It goes in sandwiches, jacket potatoes or on a salad. Curry powder is better cooked in a little butter and cooled before it meets the mayonnaise.")]), py_tuple(vec![py_str("haggis"), py_tuple(vec![py_str("haggis"), py_str("haggises")]), py_str("Haggis is a Scottish sausage of sheep's heart, liver and lungs minced with oatmeal, suet, onion and pepper, and boiled in a casing - it is a savoury pudding, traditionally served with neeps and tatties. The oatmeal and the pepper are what make it crumbly and warming. A vegetarian one is made with lentils and oats, and the good ones are as peppery as the meat.")]), py_tuple(vec![py_str("hash"), py_tuple(vec![py_str("hash"), py_str("corned beef hash"), py_str("hash dinner")]), py_str("Hash is cooked meat and potatoes chopped up and fried together in a hot pan until the edges crisp - corned beef hash is the version most people mean, with onion and a little stock or gravy to keep it moist. It is a way to use up a Sunday roast, so the potatoes want to be cold and cooked. A fried egg or brown sauce on top is not optional for most people.")]), py_tuple(vec![py_str("cornish-pasty"), py_tuple(vec![py_str("cornish pasty"), py_str("pasty"), py_str("pasty's"), py_str("pasties")]), py_str("A Cornish pasty is a half-moon of shortcrust pastry crimped along one edge, filled with beef skirt, potato, swede and onion, and baked without a tin - the raw filling cooks inside the sealed pastry, which is why it is juicy. It is eaten by hand, and the thick crimp was the handle for a miner's dirty fingers. Season it well inside: the pastry keeps the steam in, so a bland filling stays bland.")]), py_tuple(vec![py_str("biscuit"), py_tuple(vec![py_str("biscuit"), py_str("biscuits")]), py_str("In Britain a biscuit is a small crisp baked thing - a digestive, a custard cream, a shortbread - while in America a biscuit is a soft soda-raised bread roll eaten with gravy. The British kind gets its snap from sugar, fat and a short bake: take them out while they are still slightly soft and they crisp as they cool. A soft biscuit has usually been stored with moisture, so keep them in a tin, not the fridge.")]), py_tuple(vec![py_str("dripping"), py_tuple(vec![py_str("dripping"), py_str("beef dripping")]), py_str("Dripping is the fat that renders out of a beef roast and sets to a pale, grainy solid in the fridge - it is the best fat there is for roast potatoes and Yorkshire puddings, because it takes a much higher heat than oil without burning. Pour it off the roasting tin through a sieve into a jar, keep it in the fridge for weeks, and spoon out what you need. The dark jelly under it is the roasting juices and it goes in the gravy.")]), py_tuple(vec![py_str("suet"), py_tuple(vec![py_str("suet"), py_str("beef suet")]), py_str("Suet is the hard white fat from around a cow's or sheep's kidneys, sold shredded and mixed with a little flour, and it is what makes a steamed pudding light and a dumpling fluffy - it melts slowly inside the dough and leaves air behind. Shortcrust and dumplings use it where a softer fat would go heavy. A vegetarian version made from palm or rice bran fat works the same way.")]), py_tuple(vec![py_str("scotch-pancake"), py_tuple(vec![py_str("scotch pancake"), py_str("scotch pancakes"), py_str("drop scone"), py_str("drop scones")]), py_str("A Scotch pancake is a small thick pancake from a thick batter poured in spoonfuls onto a griddle - a drop scone - and it is eaten cold with butter or jam rather than hot with lemon. Self-raising flour and a rest of ten minutes before cooking give it the height, and the pan wants to be dry and only just hot enough. They freeze well and toast from frozen.")]), py_tuple(vec![py_str("bap"), py_tuple(vec![py_str("bap"), py_str("baps"), py_str("barm"), py_str("barm cake"), py_str("cob"), py_str("roll with a crust")]), py_str("A bap is a soft white bread roll, usually with a floury top and a little milk or butter in the dough, and the same thing is a barm cake in Lancashire and a cob in the Midlands - it is the roll a bacon butty or a burger goes in. Softness is the point, so bake it hot and briefly and cover the rolls with a cloth as they cool. A bap is not a crusty roll, and the two are not swapped.")]), py_tuple(vec![py_str("suet-pudding"), py_tuple(vec![py_str("suet pudding"), py_str("suet puddings"), py_str("steamed pudding"), py_str("steamed puddings")]), py_str("A suet pudding is a pudding made with shredded beef suet instead of butter - either a suet pastry around a filling, like steak and kidney pudding, or a suet sponge with dried fruit, like spotted dick. The suet melts as it steams, so it comes out light where a butter sponge would sit heavy. It is always steamed rather than baked, for one and a half hours and up, in a basin covered with paper.")]), py_tuple(vec![py_str("bechamel"), py_tuple(vec![py_str("bechamel"), py_str("bechamel sauce"), py_str("white sauce")]), py_str("Bechamel is milk thickened with a roux of equal weights of butter and plain flour, cooked out for a minute so it stops tasting of raw flour, then seasoned with salt, pepper and a grating of nutmeg. It is the white sauce under lasagne and cauliflower cheese, and stirring in cheese turns it into a Mornay. Warm the milk first and whisk from the bottom of the pan to keep it smooth.")]), py_tuple(vec![py_str("hollandaise"), py_tuple(vec![py_str("hollandaise"), py_str("hollandaise sauce")]), py_str("Hollandaise is egg yolks whisked over gentle heat until thick, then warm clarified butter whisked in drop by drop to make an emulsion, sharpened with lemon and salt. It is the sauce on eggs benedict. Keep it warm rather than hot, because boiling splits it, and a teaspoon of cold water whisked in often brings a split one back.")]), py_tuple(vec![py_str("aioli"), py_tuple(vec![py_str("aioli"), py_str("garlic mayonnaise")]), py_str("Aioli is garlic pounded to a paste with salt, then olive oil whisked in to make a thick emulsion - in Provence that is all it is, and the version with egg yolk is mayonnaise with garlic. It is served cold with fish, boiled vegetables and bread. Pounding the garlic with a pinch of salt first stops it turning bitter.")]), py_tuple(vec![py_str("veloute"), py_tuple(vec![py_str("veloute"), py_str("veloute sauce")]), py_str("A veloute is a roux loosened with white stock instead of milk, simmered until it coats a spoon and finished with cream or egg yolk. It is the base of a lot of classic French sauces. Use a stock you would drink on its own, because there is nothing in a veloute to hide a weak one.")]), py_tuple(vec![py_str("mayonnaise"), py_tuple(vec![py_str("mayonnaise")]), py_str("Mayonnaise is egg yolk whisked with mustard, salt and vinegar, with oil added a little at a time until it thickens into an emulsion. The yolk is what holds the oil and the vinegar together. Eggs at room temperature and a slow drizzle at the start are what stop it splitting.")]), py_tuple(vec![py_str("gravy"), py_tuple(vec![py_str("gravy")]), py_str("Gravy is the juices left in the roasting tin loosened with stock or wine, thickened a little with flour and simmered until it tastes of the meat. The flavour is in the browned bits stuck to the tin, so splash in hot stock and scrape them up. Stir in the juices from the rested joint at the end so they stay in the gravy.")]), py_tuple(vec![py_str("pesto"), py_tuple(vec![py_str("pesto"), py_str("pesto sauce")]), py_str("Pesto is basil, pine nuts, garlic and parmesan pounded with olive oil into a coarse green paste - pounded in a mortar rather than blended, because a blade heats it and turns it bitter. Stir it into hot pasta off the heat with a spoonful of the cooking water to loosen it. Walnuts or almonds stand in for pine nuts.")]), py_tuple(vec![py_str("vinaigrette"), py_tuple(vec![py_str("vinaigrette")]), py_str("A vinaigrette is oil and vinegar shaken together with salt, usually three parts oil to one of vinegar, with a little mustard to hold the two together. It is the dressing for a green salad and for cold vegetables. Make it in a jar and shake it again just before it goes on, because it separates as it stands.")]), py_tuple(vec![py_str("chutney"), py_tuple(vec![py_str("chutney"), py_str("chutneys")]), py_str("Chutney is fruit or vegetables cooked down with vinegar, sugar and spice until thick and jammy, so it keeps for months in jars. The vinegar and sugar are what preserve it, so do not cut them down. An Indian chutney can also be a fresh uncooked relish, but the jarred British kind is the cooked one.")]), py_tuple(vec![py_str("marinade"), py_tuple(vec![py_str("marinade"), py_str("marinades")]), py_str("A marinade is a liquid meat, fish or vegetables sit in before cooking: something acidic such as wine, vinegar, lemon or yoghurt, plus oil, salt and spice. It seasons the surface and tenderises a little, and the salt and the time matter more than the flavourings. Half an hour for fish, a few hours for chicken, overnight for a tough cut, always in the fridge.")]), py_tuple(vec![py_str("brine"), py_tuple(vec![py_str("brine"), py_str("brining")]), py_str("Brine is salt water, about five per cent salt, that meat sits in before cooking so it takes up water and stays juicy. An hour for chops, four to twelve hours for a chicken or a turkey. Pat it dry before it goes in the pan or it will steam instead of browning.")]), py_tuple(vec![py_str("glaze"), py_tuple(vec![py_str("glaze"), py_str("glazes")]), py_str("A glaze is a thin coating brushed onto food so it cooks to a shine: reduced stock, honey and soy, jam loosened with water, or sugar and butter. Brush it on towards the end, because the sugar in it burns before the food is cooked through. Brush again every few minutes so it builds in layers.")]), py_tuple(vec![py_str("dry-rub"), py_tuple(vec![py_str("dry rub"), py_str("spice rub")]), py_str("A dry rub is salt, sugar and spices patted onto meat with no liquid, so the salt seasons it right through and the sugar helps the crust colour. Leave it on for half an hour at least, or overnight uncovered in the fridge for crisper skin. Pat off the loose spice before cooking so it does not burn.")]), py_tuple(vec![py_str("soda-bread"), py_tuple(vec![py_str("soda bread"), py_str("soda breads")]), py_str("Soda bread is flour, bicarbonate of soda, salt and buttermilk with no yeast - the soda and the acid in the buttermilk make it rise as it bakes, so it is mixed and in the oven in minutes and eaten the same day. Handle the dough as little as possible and cut a cross in the top. It stales fast, so toast or freeze what is left.")]), py_tuple(vec![py_str("focaccia"), py_tuple(vec![py_str("focaccia")]), py_str("Focaccia is a flat bread of soft wet dough stretched into an oiled tin, dimpled with the fingers and baked with olive oil, coarse salt and often rosemary. The dimples hold the oil, which is what makes the top crisp and the crumb light. It is meant to be wet, so resist adding flour to make it easier to handle.")]), py_tuple(vec![py_str("ciabatta"), py_tuple(vec![py_str("ciabatta")]), py_str("Ciabatta is a very wet Italian white bread, around three parts water to four of flour, which is where the big irregular holes and the chewy crust come from. The dough is too sticky to knead by hand, so it is stretched and folded instead. Prove it slowly and handle it gently or the air comes out.")]), py_tuple(vec![py_str("brioche"), py_tuple(vec![py_str("brioche"), py_str("brioche bun")]), py_str("Brioche is an enriched bread of flour, eggs and a lot of butter with a little sugar, so it is soft, yellow and slightly sweet. The butter goes in cold and a little at a time, and the dough wants a long cold prove. It makes the best burger bun and the worst sandwich for a wet filling.")]), py_tuple(vec![py_str("bannock"), py_tuple(vec![py_str("bannock"), py_str("bannocks")]), py_str("Bannock is a flat quick bread from Scotland and the north of England, of flour, oatmeal or barley with bicarbonate of soda, salt and buttermilk, cooked on a hot girdle or in a pan rather than baked. It is ready in minutes and eaten warm with butter. What you buy is a dense round flat bread for toasting.")]), py_tuple(vec![py_str("stottie"), py_tuple(vec![py_str("stottie"), py_str("stottie cake"), py_str("stotties")]), py_str("A stottie is a big flat white loaf from Tyneside, baked slowly so it stays soft and a bit stodgy inside, then split and filled - ham and pease pudding in a stottie is the classic. The name comes from stott, to bounce, because the dough is thrown down to knock it back.")]), py_tuple(vec![py_str("crumpet"), py_tuple(vec![py_str("crumpet"), py_str("crumpets")]), py_str("A crumpet is a soft spongy griddle cake of batter cooked on one side only, so the top sets full of holes, then toasted until the holes fill with butter. The batter has to be bubbly before it goes into the rings. A pikelet is the thinner version from Wales and the Midlands.")]), py_tuple(vec![py_str("bagel"), py_tuple(vec![py_str("bagel"), py_str("bagels")]), py_str("A bagel is a bread ring boiled for a minute or two before it is baked, which sets the crust and gives it the chew. It is dense on purpose, so it takes a lot of filling without falling apart. New York bagels are boiled in malted water, which is where the shine and the colour come from.")]), py_tuple(vec![py_str("naan"), py_tuple(vec![py_str("naan"), py_str("naan bread"), py_str("naans")]), py_str("Naan is a soft flatbread of white flour, yoghurt and yeast, slapped onto the wall of a hot tandoor or cooked in a very hot dry pan so it puffs and blisters in patches. It gets brushed with butter or ghee as it comes off the heat. The yoghurt is what keeps it soft the next day.")]), py_tuple(vec![py_str("pitta"), py_tuple(vec![py_str("pitta"), py_str("pita"), py_str("pitta bread"), py_str("pita bread")]), py_str("Pitta is a plain flatbread of flour, water and yeast baked very hot and fast, so the steam inside separates the two layers and leaves a pocket. That is why it opens for filling instead of needing to be cut. Keep the pack wrapped so it stays soft enough to open.")]), py_tuple(vec![py_str("trifle"), py_tuple(vec![py_str("trifle"), py_str("trifles")]), py_str("A trifle is sponge or stale cake soaked with sherry or fruit juice, then fruit, then custard, then whipped cream, and the classic has raspberries and ratafia biscuits. Make the custard with egg yolks or cornflour rather than custard powder, or it tastes thin. Build it the day before so the sponge soaks.")]), py_tuple(vec![py_str("fruit-fool"), py_tuple(vec![py_str("fruit fool"), py_str("gooseberry fool"), py_str("raspberry fool"), py_str("rhubarb fool")]), py_str("A fruit fool is stewed fruit folded into whipped cream and chilled - gooseberry is the classic, and raspberry and rhubarb work the same way. The fruit has to be cold and thick before it is folded in or it loosens the cream. It is one of the oldest British puddings and one of the quickest.")]), py_tuple(vec![py_str("syllabub"), py_tuple(vec![py_str("syllabub")]), py_str("Syllabub is cream whipped with white wine, lemon juice and sugar until it holds its shape, sometimes with the wine poured in from a height so it froths on top. It sets because the acid from the wine and lemon firms the cream. Serve it in glasses with a biscuit, cold and the same day.")]), py_tuple(vec![py_str("christmas-pudding"), py_tuple(vec![py_str("christmas pudding"), py_str("christmas puddings"), py_str("plum pudding")]), py_str("Christmas pudding is a steamed suet pudding of dried fruit, breadcrumbs, brown sugar, spice and ale or brandy, made weeks ahead so it matures and then reheated on the day - Stir-up Sunday, five weeks before, is the traditional day to make it. It is steamed for hours, never baked, and served with brandy butter or custard. Wrapped and kept cool it lasts a year.")]), py_tuple(vec![py_str("clootie-dumpling"), py_tuple(vec![py_str("clootie dumpling"), py_str("clootie dumplings")]), py_str("A clootie dumpling is a Scottish steamed fruit pudding, like a christmas pudding but rounder and softer, tied in a floured cloth - a cloot - and boiled for hours rather than steamed in a basin. The cloth gives it its wrinkled skin. Serve it with custard, or sliced and fried with a cooked breakfast.")]), py_tuple(vec![py_str("granita"), py_tuple(vec![py_str("granita")]), py_str("Granita is fruit juice or coffee with sugar and water frozen in a tray and forked through every half hour, so it sets into coarse ice crystals rather than a smooth cream. It needs no churning or special kit. More sugar means softer crystals, and alcohol keeps it from freezing hard at all.")]), py_tuple(vec![py_str("eton-mess"), py_tuple(vec![py_str("eton mess")]), py_str("Eton mess is broken meringue, whipped cream and strawberries folded together, deliberately messy rather than layered. Assemble it just before it goes to the table or the meringue loses its crunch. It is said to come from a picnic at Eton where a strawberry and meringue pudding was dropped, and the name is the whole point.")]), py_tuple(vec![py_str("cranachan"), py_tuple(vec![py_str("cranachan")]), py_str("Cranachan is a Scottish pudding of whipped cream, toasted oats, whisky and honey with raspberries folded in. Toast the oats slowly with a little sugar so they stay crisp against the cream. It is served in layers with a dram on the side.")]), py_tuple(vec![py_str("tiramisu"), py_tuple(vec![py_str("tiramisu")]), py_str("Tiramisu is savoiardi biscuits dipped in strong coffee and marsala, layered with mascarpone beaten with egg yolks and sugar, and dusted with cocoa. The coffee has to be cold and the dip quick, or the biscuits turn to mush. It needs at least four hours in the fridge, so make it the day before.")]), py_tuple(vec![py_str("cheesecake"), py_tuple(vec![py_str("cheesecake"), py_str("cheesecakes")]), py_str("Cheesecake is crushed biscuit and butter pressed into a tin with a soft cheese filling on top - baked with eggs and cream for the New York kind, or just chilled for the no-bake one. A baked cheesecake cracks when the oven is too hot, so bake it low and let it cool in the oven with the door ajar. Rest it in the fridge overnight.")]), py_tuple(vec![py_str("sorbet"), py_tuple(vec![py_str("sorbet"), py_str("sorbets")]), py_str("Sorbet is frozen fruit puree or juice with sugar syrup and nothing else - no dairy, which is what makes it different from ice cream. It needs more sugar than ice cream to stop it freezing solid, and a squeeze of lemon keeps the fruit tasting fresh. Frozen still and scraped with a fork it is a granita.")]), py_tuple(vec![py_str("hummus"), py_tuple(vec![py_str("hummus"), py_str("houmous"), py_str("humous"), py_str("houmus"), py_str("hummus dip")]), py_str("Hummus is a dip of cooked chickpeas blitzed with tahini, lemon juice, garlic and olive oil, loosened with a little cold water until it is smooth and pale. Tinned chickpeas work fine, and a pinch of bicarbonate of soda in the water while they simmer makes them softer. Spread it in a bowl with a swirl and a pool of olive oil, and eat it with flatbread or raw vegetables.")]), py_tuple(vec![py_str("tzatziki"), py_tuple(vec![py_str("tzatziki"), py_str("tsatsiki"), py_str("tatziki")]), py_str("Tzatziki is a Greek dip of thick yoghurt, grated cucumber, garlic, olive oil and a little vinegar or lemon, with dill or mint stirred through. Salt the grated cucumber and squeeze it dry first, or the dip turns watery within the hour. It goes with grilled meat, souvlaki and flatbread, and cold from the fridge it cools a hot plate down.")]), py_tuple(vec![py_str("raita"), py_tuple(vec![py_str("raita"), py_str("cucumber raita"), py_str("raitas")]), py_str("Raita is an Indian side of plain yoghurt with grated cucumber, salt and roasted cumin, sometimes mint or tomato, served cold beside a hot curry. Whisk the yoghurt smooth, stir in cucumber that has been salted and squeezed, and dust the cumin on just before it goes to the table. It is there to cool the mouth, so keep it simple.")]), py_tuple(vec![py_str("tahini"), py_tuple(vec![py_str("tahini"), py_str("tahina"), py_str("sesame paste")]), py_str("Tahini is ground sesame seeds and nothing else - a thick, faintly bitter paste used in hummus, in dressings, and on its own beaten with lemon and water as a sauce. It seizes up when liquid goes in at first and turns creamy with more beating, so keep going. Stir the jar before you use it, keep it in the cupboard, and it lasts months.")]), py_tuple(vec![py_str("jus"), py_tuple(vec![py_str("jus"), py_str("a jus"), py_str("red wine jus"), py_str("meat jus")]), py_str("A jus is the thin gravy that comes from a roast: the juices left in the tin, deglazed with wine or stock and reduced until it coats the back of a spoon. Nothing thickens it, which is what makes it lighter and glossier than gravy. Pour the fat off first and strain it if you want it clear.")]), py_tuple(vec![py_str("salsa"), py_tuple(vec![py_str("salsa"), py_str("salsa sauce"), py_str("pico de gallo"), py_str("tomato salsa")]), py_str("Salsa is a Mexican sauce, most often raw tomato, onion, chilli, lime and coriander chopped together - that one is pico de gallo, and it is a relish rather than a dip. Cooked and blended it is a smoother, smokier salsa for chips or for baking under. The dance of the same name is a different question.")]), py_tuple(vec![py_str("brisket"), py_tuple(vec![py_str("brisket"), py_str("beef brisket"), py_str("brisket of beef")]), py_str("Brisket is the boned chest of the beef, a tough cut full of connective tissue, so it wants long, slow, moist heat - four or five hours at 150C - rather than a quick roast. Cooked like that it falls into soft slices; pot-roasted with stock and onions it makes its own gravy. Always slice it across the grain.")]), py_tuple(vec![py_str("silverside"), py_tuple(vec![py_str("silverside"), py_str("salt silverside"), py_str("salted silverside")]), py_str("Silverside is a boned cut from the back of the hindquarter, usually salted and simmered rather than roasted, then served hot with mustard sauce or cold in sandwiches. Soak a very salty one overnight before cooking, then bring it to a bare simmer and hold it there - a rolling boil makes it stringy. Brined and tinned, the same cut is sold as corned beef.")]), py_tuple(vec![py_str("kipper"), py_tuple(vec![py_str("kipper"), py_str("kippers"), py_str("smoked herring")]), py_str("A kipper is a herring that has been split, salted and smoked, so it is already cooked and needs only heating: grill it or poach it in a little water for three or four minutes. It comes as a whole flattened fish with a lot of fine bones. Serve it with buttered bread and a squeeze of lemon, and eat it for breakfast if you are in Scotland.")]), py_tuple(vec![py_str("hake"), py_tuple(vec![py_str("hake"), py_str("hake fillet"), py_str("hake fillets")]), py_str("Hake is a white sea fish in the cod family, soft-fleshed and sold as fillets or as steaks with a bone through the middle. It flakes more easily than cod, so cook it briefly - skin-side down in a hot pan, or 12 to 15 minutes in a hot oven. It takes strong flavours well: garlic, paprika, tomato and olive oil.")]), py_tuple(vec![py_str("halloumi"), py_tuple(vec![py_str("halloumi"), py_str("halloumi cheese"), py_str("grilled halloumi")]), py_str("Halloumi is a Cypriot cheese of sheep's and goat's milk that is heated and pressed as it is made, so it holds its shape and squeaks instead of melting. Slice it, dry the slices on kitchen paper, then fry or grill hard for a minute a side until the outside is dark and crisp. It is very salty, so it wants lemon, chilli or honey rather than more salt.")]), py_tuple(vec![py_str("paneer"), py_tuple(vec![py_str("paneer"), py_str("paneer cheese"), py_str("indian cheese")]), py_str("Paneer is an Indian cheese made by curdling hot milk with lemon juice or vinegar and pressing the curds under a weight, so it is firm, mild and does not melt - it browns in the pan instead. Fry the cubes first so they keep their shape, then add them to the sauce at the end. It takes on whatever spice is around it.")]), py_tuple(vec![py_str("bulgur"), py_tuple(vec![py_str("bulgur"), py_str("burghul"), py_str("bulghur"), py_str("bulgur wheat"), py_str("cracked wheat")]), py_str("Bulgur is wheat that has been parboiled, dried and cracked, so it needs only soaking or a few minutes of simmering rather than half an hour. Coarse bulgur is for pilafs and salads, fine bulgur for kibbeh and tabbouleh. It swells to twice its size and keeps a firm bite where rice would go soft.")]), py_tuple(vec![py_str("quinoa"), py_tuple(vec![py_str("quinoa"), py_str("quinoa grain"), py_str("red quinoa")]), py_str("Quinoa is a seed from a South American plant, cooked like rice but faster, with a nutty taste and a slight crunch - it is not a grain. Rinse it before cooking to wash off the bitter coating, then simmer it in twice its volume of water for about 15 minutes until the little rings come away. It works hot as a side and cold in salads.")]), py_tuple(vec![py_str("polenta"), py_tuple(vec![py_str("polenta"), py_str("polenta grain"), py_str("cornmeal polenta")]), py_str("Polenta is coarsely ground cornmeal simmered in water or stock until it thickens and pulls away from the side of the pan: about 40 minutes of stirring for the coarse kind, five for the instant. Served soft with butter and parmesan, or spread in a tray, cooled firm and cut into slices to fry. The fried leftovers are the best part.")]), py_tuple(vec![py_str("couscous"), py_tuple(vec![py_str("couscous"), py_str("cous cous"), py_str("instant couscous")]), py_str("Couscous is semolina rolled into tiny grains by hand and steamed over the stew rather than boiled; the boxed instant kind has already been steamed, so it only needs boiling water poured over it. Use the same volume of water as grain, cover it for five minutes, then fluff it with a fork and stir in butter or oil. It is the bed the stew sits on.")]), py_tuple(vec![py_str("plantain"), py_tuple(vec![py_str("plantain"), py_str("plantains"), py_str("cooking banana"), py_str("green plantain")]), py_str("Plantain is a cooking banana: bigger, starchier and meant to be cooked, never eaten raw. Green ones are savoury and firm, so slice them thin and fry them crisp; left until the skin is black they are sweet, and thick coins fried in butter go soft and caramelised. Either way they take salt, chilli or a bean stew.")]), py_tuple(vec![py_str("swede"), py_tuple(vec![py_str("swede"), py_str("swedes"), py_str("rutabaga"), py_str("neeps")]), py_str("Swede is a round root with purple skin and yellow flesh - a bigger, sweeter relative of the turnip, and called rutabaga in America and neeps in Scotland. It roasts in chunks around a joint, mashes with butter and pepper, or goes into a pasty. The smaller ones are sweeter, and it takes longer to cook than a potato.")]), py_tuple(vec![py_str("parsnip"), py_tuple(vec![py_str("parsnip"), py_str("parsnips"), py_str("roast parsnips")]), py_str("A parsnip is a long cream root with a sweet, peppery taste that gets sweeter after a frost because the cold turns its starch to sugar. Roast it in chunks around the meat or mash it, and cut the woody core out of a big one first. Peel it thinly, since most of the flavour sits just under the skin.")]), py_tuple(vec![py_str("shallot"), py_tuple(vec![py_str("shallot"), py_str("shallots"), py_str("banana shallot")]), py_str("A shallot is a small elongated onion of its own variety, milder and sweeter than an onion, and it splits into two or three cloves under the skin. It is the one to use raw in a vinaigrette or pickled whole, and it softens and caramelises faster than an onion does. The banana shallot is the long one and is easiest to peel.")]), py_tuple(vec![py_str("cumin"), py_tuple(vec![py_str("cumin"), py_str("cumin seeds"), py_str("ground cumin"), py_str("jeera")]), py_str("Cumin is a warm, earthy spice sold as whole seeds or ground, and the seeds keep their flavour far longer than the powder. Toast the seeds for a minute in a dry pan, or fry them in the oil at the start of a curry, then crush them. It is the base note of most curry powders, of chilli and of a lot of Mexican cooking.")]), py_tuple(vec![py_str("paprika"), py_tuple(vec![py_str("paprika"), py_str("smoked paprika"), py_str("sweet paprika"), py_str("hot paprika")]), py_str("Paprika is ground dried peppers - sweet, hot or smoked depending on which chilli was used, and the Hungarian sweet one is what people usually mean. Sweet paprika cannot take high heat: stir it in near the end, or bloom it in warm oil for a few seconds, or it turns bitter. Smoked paprika is for depth, in chorizo, beans and roast chicken.")]), py_tuple(vec![py_str("turmeric"), py_tuple(vec![py_str("turmeric"), py_str("tumeric"), py_str("ground turmeric"), py_str("turmeric powder")]), py_str("Turmeric is a bright yellow root, used dried and ground, with an earthy, slightly bitter taste and no real heat - it is in curry powder mostly for colour. A little goes a long way, and it stains boards, cloth and fingers yellow, so keep it off anything pale. Fresh root is milder and can be grated into rice or a dressing.")]), py_tuple(vec![py_str("coriander"), py_tuple(vec![py_str("coriander"), py_str("cilantro"), py_str("coriander leaves"), py_str("ground coriander"), py_str("coriander seeds")]), py_str("Coriander is two different things from one plant: the leaves and stalks, which taste bright and faintly soapy and go in raw at the end, and the ground seed, which is warm and citrusy and goes in at the start. Toast the seeds to wake them up before grinding. In a curry paste the root is the most flavoursome part.")]), py_tuple(vec![py_str("samosa"), py_tuple(vec![py_str("samosa"), py_str("samosas"), py_str("vegetable samosa"), py_str("lamb samosa")]), py_str("A samosa is a pastry triangle of spiced potato and peas, or minced lamb, wrapped in a thin flour pastry and deep-fried until it blisters - or baked, which is lighter but not the same. The filling has to be cool and dry before it is wrapped or the pastry goes soft. Serve them with a yoghurt dip or a tamarind chutney.")]), py_tuple(vec![py_str("falafel"), py_tuple(vec![py_str("falafel"), py_str("felafel"), py_str("falafels")]), py_str("Falafel is soaked raw chickpeas ground with onion, garlic, herbs and cumin, shaped into balls and fried until dark and crisp - made from cooked chickpeas the mixture falls apart in the oil. The chickpeas need an overnight soak, not a simmer. Serve them in flatbread with salad, pickles and tahini.")]), py_tuple(vec![py_str("bhaji"), py_tuple(vec![py_str("bhaji"), py_str("bhajis"), py_str("bhajee"), py_str("onion bhaji"), py_str("onion bhajis")]), py_str("A bhaji is a fritter of sliced vegetables in a spiced gram flour batter, dropped in spoonfuls into hot oil: onion is the usual one, and cauliflower and spinach work the same way. Gram flour, not wheat, is what makes it a bhaji, and the batter should just hold together. Eat them hot with a yoghurt or tamarind dip.")]), py_tuple(vec![py_str("gherkin"), py_tuple(vec![py_str("gherkin"), py_str("gherkins"), py_str("cornichon"), py_str("cornichons"), py_str("pickled cucumber")]), py_str("A gherkin is a small cucumber pickled in vinegar or brine, eaten whole as a snack or sliced into a burger; cornichons are the tiny sour French ones. A fresh gherkin is simply a small cucumber, and only the pickled one keeps for months in the jar. Rinse them if you want less salt.")]), py_tuple(vec![py_str("caper"), py_tuple(vec![py_str("caper"), py_str("capers"), py_str("caper berries"), py_str("caperberries")]), py_str("Capers are the unopened flower buds of a Mediterranean shrub, pickled in brine or packed in salt, and used for a sour, salty hit in tartare sauce, with salmon, and in puttanesca. Rinse the salted ones before use, and chop rather than crush them. The bigger caperberries are the open fruit of the same plant and are eaten whole.")]), py_tuple(vec![py_str("samphire"), py_tuple(vec![py_str("samphire"), py_str("marsh samphire"), py_str("sea samphire"), py_str("rock samphire")]), py_str("Samphire is a salty green that grows on mudflats and beaches, looking like fleshy joined-up stems, sold fresh in summer - that one is marsh samphire. Wash it well, steam or boil it for about three minutes and eat it with butter and lemon, no salt. Rock samphire is the rarer, stronger-tasting one.")]), py_tuple(vec![py_str("salsify"), py_tuple(vec![py_str("salsify"), py_str("black salsify"), py_str("oyster plant"), py_str("scorzonera")]), py_str("Salsify is a long thin root that tastes faintly of oyster: the black-skinned one is salsify and the white one is scorzonera. Peel it and drop it straight into water with lemon juice, or it browns in minutes, then roast or fry it in butter. It is a winter vegetable and it is best simply cooked.")]), py_tuple(vec![py_str("kohlrabi"), py_tuple(vec![py_str("kohlrabi"), py_str("kohl rabi")]), py_str("Kohlrabi is a swollen stem rather than a root - pale green or purple, with a taste between cabbage and turnip and the crunch of a crisp apple. Peel the tough outer skin off, then slice it raw into salad or roast it in wedges until the edges caramelise. The leaves cook like cabbage.")]), py_tuple(vec![py_str("fennel"), py_tuple(vec![py_str("fennel"), py_str("fennel bulb"), py_str("fennel bulbs"), py_str("fennel fronds")]), py_str("Fennel is the vegetable with the fat white bulb and feathery green fronds, tasting of aniseed; the seed is the same plant's spice. Cut the bulb through the core so the wedges hold together, brown it hard in a pan, then braise or roast it, and use the fronds raw as a herb. Raw shaved thin with lemon it is a salad on its own.")]), py_tuple(vec![py_str("leek"), py_tuple(vec![py_str("leek"), py_str("leeks"), py_str("baby leeks")]), py_str("A leek is a long green and white allium, milder and sweeter than an onion, that softens and sweetens long before it browns. Slit it lengthwise and rinse between the layers, because grit grows down inside it. It is the base of leek and potato soup and it braises well in butter with a little stock.")]), py_tuple(vec![py_str("chive"), py_tuple(vec![py_str("chive"), py_str("chives"), py_str("garlic chives")]), py_str("Chives are the thin green stems of a small allium with a mild onion taste, used raw as a garnish - snipped over eggs, potatoes, soup or a baked potato. Heat kills the flavour, so add them at the end rather than in the pan. They grow back after cutting, which is why a pot on the windowsill keeps going.")]), py_tuple(vec![py_str("pudding"), py_tuple(vec![py_str("pudding"), py_str("puddings"), py_str("a pudding"), py_str("sweet pudding")]), py_str("A pudding is the sweet course at the end of a British meal, and also a family of dishes steamed or baked under suet pastry or a sponge - sticky toffee, treacle, rice, Christmas pudding. The word means either the course or the dish, so which one it is depends on the sentence. Steamed ones cook covered in a water bath; baked ones go straight in the oven.")]), py_tuple(vec![py_str("shepherds-pie"), py_tuple(vec![py_str("shepherd's pie"), py_str("shepherds pie"), py_str("shepherd pie"), py_str("shepherd's pies")]), py_str("Shepherd's pie is minced lamb cooked with onion and gravy in a dish, with mashed potato spread over the top and baked until the ridges catch and brown. Lamb is what makes it a shepherd's pie - made with beef it is a cottage pie. Let the mince cool a little before the mash goes on, or it sinks in.")]), py_tuple(vec![py_str("cottage-pie"), py_tuple(vec![py_str("cottage pie"), py_str("cottage pies"), py_str("beef pie with mash")]), py_str("Cottage pie is minced beef with onion and gravy under a mash crust, baked until the top is crisp - the beef version of shepherd's pie. Brown the mince properly first and let the gravy reduce, or the mash slides off in the serving. Fork the mash up before it goes in the oven so the ridges brown.")]), py_tuple(vec![py_str("pie"), py_tuple(vec![py_str("pie"), py_str("pies"), py_str("a pie"), py_str("savoury pie")]), py_str("A pie has pastry under the filling and a lid over it, or just a lid, where a tart is open and a cobbler has scones on top. Shortcrust is the plain one, puff the laminated one that rises in layers, and hot-water crust is for a raised pork pie. Fillings run from steak and ale to apple.")]), py_tuple(vec![py_str("compote"), py_tuple(vec![py_str("compote"), py_str("fruit compote")]), py_str("A compote is whole or large pieces of fruit poached gently in sugar syrup, sometimes with vanilla, lemon peel or spice, until they are soft but still in shape. It keeps in the fridge for a few days and is served with cream, yoghurt or porridge. Use less syrup than you think, because the fruit gives up its own juice.")])]) }
    s_DISH_WHAT(__data129());
    fn __data130() -> Py { py_tuple(vec![py_str("what is"), py_str("what's"), py_str("whats"), py_str("what are"), py_str("what was"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("tell me about"), py_str("what do you mean"), py_str("is a"), py_str("is an"), py_str("is it a"), py_str("are they"), py_str("what kind of"), py_str("what sort of"), py_str("where does"), py_str("where do"), py_str("what does")]) }
    s_DISH_WHAT_ASKS(__data130());
    fn __data131() -> Py { py_tuple(vec![py_str(""), py_str("so"), py_str("ok"), py_str("okay"), py_str("and"), py_str("but"), py_str("then"), py_str("sorry"), py_str("quick"), py_str("quick one"), py_str("one thing"), py_str("also"), py_str("yeah"), py_str("yes"), py_str("hi"), py_str("hiya"), py_str("hello"), py_str("right"), py_str("actually"), py_str("just"), py_str("please"), py_str("oh"), py_str("um"), py_str("er"), py_str("well")]) }
    s_DISH_WHAT_LEAD(__data131());
    fn __data132() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("how should i"), py_str("best way"), py_str("recipe"), py_str("recipes"), py_str("goes with"), py_str("go with"), py_str("go nicely"), py_str("serve with"), py_str("serving with"), py_str("side for"), py_str("sides for"), py_str("side dish"), py_str("with what"), py_str("alongside"), py_str("to serve"), py_str("for dinner"), py_str("for tea"), py_str("for lunch"), py_str("for a crowd"), py_str("for two"), py_str("for four"), py_str("what about"), py_str("how about"), py_str("what if"), py_str("same for"), py_str("favourite"), py_str("favorite"), py_str("favourites"), py_str("favorites"), py_str("your favourite"), py_str("your favorite"), py_str("you like"), py_str("do you like"), py_str("would you"), py_str("prefer"), py_str("your pick"), py_str("pick one"), py_str("which one"), py_str("what would you"), py_str("how long"), py_str("how much"), py_str("make it"), py_str("make them"), py_str("cook it"), py_str("cook them"), py_str("should i"), py_str("can i"), py_str("calories"), py_str("good for you"), py_str("healthy"), py_str("what is a good"), py_str("what is the best"), py_str("best way"), py_str("for dinner"), py_str("ideas"), py_str("suggest"), py_str("recommend"), py_str("which is better"), py_str("casserole dish"), py_str("hash brown"), py_str("hash browns"), py_str("tap"), py_str("taps"), py_str("washer"), py_str("plumbing"), py_str("pipe"), py_str("pipes"), py_str("paint"), py_str("radiator"), py_str("boiler"), py_str("called"), py_str("named"), py_str("puppy"), py_str("puppies"), py_str("dog"), py_str("dogs"), py_str("kitten"), py_str("kittens"), py_str("cat"), py_str("cats"), py_str("hamster"), py_str("rabbit"), py_str("lol"), py_str("haha"), py_str("ha"), py_str("thanks"), py_str("ta"), py_str("cheers"), py_str("bye"), py_str("gotta"), py_str("swap"), py_str("swaps"), py_str("swap it"), py_str("ceiling"), py_str("ceilings"), py_str("roof"), py_str("wall"), py_str("walls"), py_str("floor"), py_str("floors"), py_str("water"), py_str("damp"), py_str("fridge"), py_str("freezer"), py_str("window"), py_str("windows"), py_str("door"), py_str("doors"), py_str("tag"), py_str("tags"), py_str("hashtag"), py_str("hashtags"), py_str("symbol"), py_str("key"), py_str("code"), py_str("sign"), py_str("taste like"), py_str("tastes like"), py_str("smell like"), py_str("look like"), py_str("sound like"), py_str("dance"), py_str("dances"), py_str("dancing"), py_str("dancers")]) }
    s_DISH_WHAT_AVOID(__data132());
    s_DISH_BARE_NAMES(py_tuple(vec![py_str("pie"), py_str("pies"), py_str("a pie"), py_str("pudding"), py_str("puddings"), py_str("a pudding")]));
    let __iter133 = py_iter_or_raise(&g_DISH_WHAT());
    for __item in __iter133 {
    let __unpk134 = py_unpack(&__item.clone(), 3, None);
    s__slug(__unpk134[0].clone());
    s__names(__unpk134[1].clone());
    s__what(__unpk134[2].clone());
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("what-is-%s"), &g__slug())); __args.push(py_tuple(vec![g__names(), g_DISH_WHAT_ASKS()])); __args.push(py_func("_what_is_a_dish", |args: &[Py]| _what_is_a_dish(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_DISH_WHAT_ASKS()), ("avoid".to_string(), g_DISH_WHAT_AVOID())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    }
    fn __data135() -> Py { py_tuple(vec![py_str("cheap lunch"), py_str("cheap lunches"), py_str("cheap eats"), py_str("lunch near"), py_str("lunch around"), py_str("where to eat"), py_str("where should we eat"), py_str("where should i eat"), py_str("where can i eat"), py_str("where can we eat"), py_str("somewhere to eat"), py_str("anywhere to eat"), py_str("anywhere cheap to eat"), py_str("cheap to eat"), py_str("a place to eat"), py_str("place to eat"), py_str("places to eat"), py_str("cheap place to eat"), py_str("good place to eat"), py_str("good place for lunch"), py_str("place for lunch"), py_str("where to get lunch"), py_str("grab lunch"), py_str("get some lunch"), py_str("eat out cheap"), py_str("somewhere good for lunch"), py_str("somewhere for lunch"), py_str("lunch in town"), py_str("lunch near work"), py_str("lunch near here"), py_str("lunch round here")]) }
    fn __data136() -> Py { py_tuple(vec![py_str("near"), py_str("nearby"), py_str("around"), py_str("station"), py_str("office"), py_str("work"), py_str("town"), py_str("lunch"), py_str("anywhere"), py_str("recommend"), py_str("suggest"), py_str("get"), py_str("place"), py_str("places"), py_str("eat out"), py_str("cheap"), py_str("cheaply"), py_str("cheapest"), py_str("here"), py_str("today"), py_str("tonight")]) }
    fn __data137() -> Py { py_tuple(vec![py_str("dinner"), py_str("birthday"), py_str("anniversary"), py_str("valentine"), py_str("celebrate"), py_str("date night"), py_str("book a table"), py_str("booking"), py_str("michelin"), py_str("ideas"), py_str("packed lunch"), py_str("lunchbox"), py_str("make my own"), py_str("recipe"), py_str("meal plan"), py_str("cook"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pet"), py_str("pets"), py_str("kid"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("baby"), py_str("high chair"), py_str("vegan"), py_str("vegetarian"), py_str("veggie"), py_str("gluten"), py_str("allergy"), py_str("allergic"), py_str("wheelchair"), py_str("accessible")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cheap-lunch-where")); __args.push(py_tuple(vec![__data135(), __data136()])); __args.push(py_str("Look for the places that sell a lot of one thing: a market hall or a food court, a bakery, a cafe at the back of a shop, a street food van in a regular spot, or the lunch board outside a restaurant that wants the office trade. Walk a street back from the station or the main road, where the rents are lower, and follow the queue of people who work nearby.")); __kws.extend(vec![("avoid".to_string(), __data137()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _ready_food(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_len: Py = Py::None;
    let mut food: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let _ = py_str("The food among the doneness notes a line names, longest name first, or None.\n\n    Longest first so \"brown rice\" is rice and \"boiled eggs\" is not read as the egg in a cake;\n    the names are what the line says, not what the table is keyed on.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0)]), 2, None);
    best = __unpk1[0].clone();
    best_len = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_READY_NOTES(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    food = __unpk3[0].clone();
    let __unpk4 = py_unpack(&__unpk3[1].clone(), 2, None);
    names = __unpk4[0].clone();
    _u = __unpk4[1].clone();
    let __iter5 = py_iter_or_raise(&names.clone());
    for __item in __iter5 {
    name = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &best_len.clone()); if !py_truthy(&__b) { __b } else { _phrase_in(&[line.clone(), name.clone()]) } }) {
    let __unpk6 = py_unpack(&py_tuple(vec![food.clone(), call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    best = __unpk6[0].clone();
    best_len = __unpk6[1].clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _ready_note(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut food: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The doneness cue for the food the conversation is cooking, or None.\n\n    \"how do i know when its done\" names no food at all, so the food is the one the last line or\n    two named: the same reading a plan's `done` sentence gets, for the six foods that have no\n    plan. A plan's own dish is answered by the plan, so a line that names one is left alone.\n    ");
    text = _line_text(&[understood.clone()]);
    if py_truthy(&dish_in(&[text.clone()])) {
    return Py::None;
    }
    food = _ready_food(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&food.clone(), &Py::None))) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    earlier = __item.clone();
    food = _ready_food(&[earlier.clone()]);
    if py_truthy(&food.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&food.clone(), &Py::None))) {
    return Py::None;
    }
    return geti(&geti(&g_READY_NOTES(), &food.clone()), &Py::Int(1));
    Py::None
}
pub fn _phrase_in(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut have: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut n: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("True when the words of `phrase` appear in `text` in order (a short run).");
    wanted = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[phrase.clone()]);
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&have.clone())) } }) {
    return Py::Bool(false);
    }
    n = call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)), &n.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    i = __item.clone();
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[n.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut j: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&geti(&have.clone(), &op_add(&i.clone(), &j.clone())), &geti(&wanted.clone(), &j.clone()))); if py_truthy(&__b) { __b } else { _same_word(&[geti(&have.clone(), &op_add(&i.clone(), &j.clone())), geti(&wanted.clone(), &j.clone())]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _one_slip(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let mut i: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("True when `word` is `target` with one letter wrong, missing, extra or swapped.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &target.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[target.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(py_eq(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[word.clone(), target.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk7398 = py_unpack(&__item.clone(), 2, None); let mut a: Py = __unpk7398[0].clone(); let mut b: Py = __unpk7398[1].clone(); if py_truthy(&Py::Bool(!py_eq(&a.clone(), &b.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    }
    let __unpk1 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![word.clone(), target.clone()]) } else { py_tuple(vec![target.clone(), word.clone()]) }, 2, None);
    shorter = __unpk1[0].clone();
    longer = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[longer.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    i = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&op_add(&slice_or_raise(&longer.clone(), &Py::None, &i.clone(), &Py::None), &slice_or_raise(&longer.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)), &shorter.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_cut(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let _ = py_str("True when a word names a cut of the meat, allowing a plural and a single slip.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CUT_OFF()) { let mut cut: Py = __item.clone(); __out.push({ let __b = _same_word(&[word.clone(), cut.clone()]); if py_truthy(&__b) { __b } else { _one_slip(&[word.clone(), cut.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _same_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut a: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a"))) };
    let mut b: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "b") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: b"))) };
    let mut one: Py = Py::None;
    let mut two: Py = Py::None;
    let _ = py_str("eggs/egg, potatoes/potato, roasting/roast: the loose forms cooking.py matches with.");
    if py_truthy(&Py::Bool(py_eq(&a.clone(), &b.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![a.clone(), b.clone()]), py_tuple(vec![b.clone(), a.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    one = __unpk2[0].clone();
    two = __unpk2[1].clone();
    if py_truthy(&op_gt(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    if py_truthy(&{ let __b = mcall(&one.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &two.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&one.clone(), "endswith", &[py_str("es")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &two.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&one.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &two.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&op_add(&slice_or_raise(&one.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("e")), &two.clone())) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&one.clone(), "endswith", &[py_str("ed")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &two.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _slip_of(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut key: Py = Py::None;
    let _ = py_str("True when the line's word is one slip from an avoided one (\"brest\", \"thight\").\n\n    The same rule the knowledge base uses on its avoid lists: a slip that could be either of\n    two words is never used to read a line, so the plan for the whole bird stays out of\n    \"how long do i cook a chiken brest\" and the breast entry answers the line.\n    ");
    key = mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]);
    typed = mcall(&mcall(&{ let __b = typed.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("knowledge.lookup", "MIN_TYPO_LENGTH")); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("knowledge.lookup", "MIN_TYPO_LENGTH")); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&key.clone(), &py_str(" "))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&slice_or_raise(&typed.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&key.clone(), &Py::None, &Py::Int(1), &Py::None))); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("knowledge.lookup", "_engine_reads", &[typed.clone()]) } }) {
    return Py::Bool(false);
    }
    return pyrt::xrefs::xcall("knowledge.lookup", "_one_edit", &[typed.clone(), key.clone()]);
    Py::None
}
pub fn _a_made_dish(args: &[Py]) -> Py {
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
    let mut dish: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_MADE_DISH()) { let mut dish: Py = __item.clone(); __out.push(_same_word(&[word.clone(), dish.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _ruled_out_near(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut width: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "width") { v.clone() }
    else { Py::Int(1) };
    let mut after: Py = Py::None;
    let mut step: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words just before `at` rule the dish that starts there out (\"isnt a roast\").");
    after = op_add(&at.clone(), &width.clone());
    if py_truthy(&op_lt(&after.clone(), &call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    tail = mcall(&mcall(&geti(&have.clone(), &after.clone()), "replace", &[py_str("’"), py_str("")]), "replace", &[py_str("'"), py_str("")]);
    if py_truthy(&Py::Bool(py_contains(&g__INGREDIENT_AFTER(), &tail.clone()))) {
    return Py::Bool(false);
    }
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_add(&call_builtin("min", &[at.clone(), Py::Int(5)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    word = mcall(&mcall(&geti(&have.clone(), &op_sub(&at.clone(), &step.clone())), "replace", &[py_str("’"), py_str("")]), "replace", &[py_str("'"), py_str("")]);
    if py_truthy(&Py::Bool(py_contains(&g__RULE_OUT_FILLER(), &word.clone()))) {
    continue;
    }
    return Py::Bool(py_contains(&g__RULE_OUT(), &word.clone()));
    }
    return Py::Bool(false);
    Py::None
}
pub fn not_ruled_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut keys: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut guard: Py = Py::None;
    let _ = py_str("A row guard: the entry answers unless the line names one of these dishes to rule it out.\n\n    \"i dont want to do a roast what else could i do for 4\" reached the roast's own menu row on\n    \"what else\", and the row then told a person who had just said no to a roast how to ring the\n    changes round the joint. The row reads the line, so it can see the refusal the way it sees\n    any other word.\n    ");
pub fn __nested_guard_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut guard: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut keys: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut text: Py = Py::None;
    text = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[keys.clone()]).unwrap_or_else(|e| exc::raise(e)), &dishes_ruled_out(&[text.clone()]))));
    Py::None
}
    guard = {{ let guard = guard.clone(); let keys = keys.clone(); py_func("guard", move |args: &[Py]| { let mut full = vec![guard.clone(), keys.clone()]; full.extend_from_slice(args); __nested_guard_0(&full) }) }};
    return guard.clone();
    Py::None
}
pub fn _ruled_out_spans(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut covered: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The word positions of dish names the line rules out, so no shorter name inside one reads.\n\n    \"i dont want a spag bol tonight\" rules out the spag bol, and `bol` on its own is a name of the\n    same plate: the refusal is about the whole name it sits under, not about the words one by one.\n    ");
    covered = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_PLANS(), "values", &[]));
    for __item in __iter1 {
    plan = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&plan.clone(), "get", &[py_str("names"), py_tuple(vec![])]));
    for __item in __iter2 {
    names = __item.clone();
    let __iter3 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[names.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { names.clone() } else { py_tuple(vec![names.clone()]) });
    for __item in __iter3 {
    name = __item.clone();
    wanted = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[name.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    i = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_ruled_out_near(&[have.clone(), i.clone(), call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))])))) {
    continue;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut j: Py = __item.clone(); __out.push(_same_word(&[geti(&have.clone(), &op_add(&i.clone(), &j.clone())), geti(&wanted.clone(), &j.clone())])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&covered.clone(), "update", &[call_builtin("range", &[i.clone(), op_add(&i.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    }
    }
    }
    return covered.clone();
    Py::None
}
pub fn dishes_ruled_out(args: &[Py]) -> Py {
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
    let mut have: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut out: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The plan keys a text names only to rule them out (\"isnt a roast\").\n\n    `_named` stands down for these, and a table that answers a roast without naming one (the\n    joint for a crowd) asks here, because a person who has just ruled the roast out is asking\n    for something else.\n    ");
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_PLANS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    plan = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&mcall(&plan.clone(), "get", &[py_str("names"), py_tuple(vec![])]));
    for __item in __iter3 {
    names = __item.clone();
    let __iter4 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[names.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { names.clone() } else { py_tuple(vec![names.clone()]) });
    for __item in __iter4 {
    name = __item.clone();
    wanted = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[name.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    let __iter5 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    i = __item.clone();
    if py_truthy(&{ let __b = call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut j: Py = __item.clone(); __out.push(_same_word(&[geti(&have.clone(), &op_add(&i.clone(), &j.clone())), geti(&wanted.clone(), &j.clone())])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _ruled_out_near(&[have.clone(), i.clone(), call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    let _ = mcall(&out.clone(), "add", &[key.clone()]);
    break;
    }
    }
    }
    }
    }
    return out.clone();
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut avoid: Py = Py::None;
    let mut best: Py = Py::None;
    let mut elsewhere: Py = Py::None;
    let mut found: Py = Py::None;
    let mut have: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut ruled: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(words matched, position, plan key) of the most specific dish a text names, or None.\n\n    The longest name wins, and on a tie the one mentioned last: in \"roast chicken\n    thighs\" the dish being cooked is the thighs, not the roast chicken.\n    ");
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    elsewhere = pyrt::xrefs::xcall("understanding.topics", "says_another_thing", &[text.clone()]);
    ruled = _ruled_out_spans(&[have.clone()]);
    best = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_PLANS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    plan = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&mcall(&plan.clone(), "get", &[py_str("names"), py_tuple(vec![])]));
    for __item in __iter3 {
    names = __item.clone();
    let __iter4 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[names.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { names.clone() } else { py_tuple(vec![names.clone()]) });
    for __item in __iter4 {
    name = __item.clone();
    wanted = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[name.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    avoid = mcall(&plan.clone(), "get", &[py_str("avoid"), py_tuple(vec![])]);
    let __iter5 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    i = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut j: Py = __item.clone(); __out.push({ let __b = _same_word(&[geti(&have.clone(), &op_add(&i.clone(), &j.clone())), geti(&wanted.clone(), &j.clone())]); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { _slip_of(&[geti(&have.clone(), &op_add(&i.clone(), &j.clone())), geti(&wanted.clone(), &j.clone())]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    after = if py_truthy(&op_lt(&op_add(&i.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("len", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&have.clone(), &op_add(&i.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)))) } else { py_str("") };
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&avoid.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = _same_word(&[after.clone(), word.clone()]); if py_truthy(&__b) { __b } else { _slip_of(&[after.clone(), word.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { _a_cut(&[after.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { _a_made_dish(&[after.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = elsewhere.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&have.clone(), &i.clone(), &op_add(&i.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&elsewhere.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[i.clone(), op_add(&i.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); __out.push(Py::Bool(py_contains(&ruled.clone(), &at.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    found = py_tuple(vec![call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), i.clone(), key.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&slice_or_raise(&found.clone(), &Py::None, &Py::Int(2), &Py::None), &slice_or_raise(&best.clone(), &Py::None, &Py::Int(2), &Py::None)) } }) {
    best = found.clone();
    }
    }
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn a_dish_is_already_the_plan(args: &[Py]) -> Py {
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
    let mut marker: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("True when the line names a dish and says that is what is being cooked.\n\n    `fancy a spag bol tonight` and `thinking of doing a roast on sunday` name the dish they mean\n    to cook, so the idea row that picked a traybake for them and the evening plan that gave them\n    a lecture about their evening were both answering a question nobody asked. A line that asks\n    what to cook, or that names a food to get away from, is not this: its dish name is material.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&dish_in(&[text.clone()]), &Py::None)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PLAN_MARKER()) { let mut marker: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &marker.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__IDEA_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn dish_in(args: &[Py]) -> Py {
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
    let _ = py_str("The plan key a piece of text names, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    found = _named(&[text.clone()]);
    return if py_truthy(&found.clone()) { geti(&found.clone(), &Py::Int(2)) } else { Py::None };
    Py::None
}
pub fn implied_dish(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut no: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The dish a part word belongs to ('frosting' -> the cake plan), or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PART_NOT()) { let mut no: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), no.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_PART_OF(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    part = __unpk2[0].clone();
    key = __unpk2[1].clone();
    if py_truthy(&{ let __b = _phrase_in(&[text.clone(), part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLANS(), &key.clone())) } }) {
    return key.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _line_text(args: &[Py]) -> Py {
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
    let _ = py_str("The user's line as the matcher keeps it, or ''.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    return { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    Py::None
}
pub fn side_in(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let _ = py_str("The side a line names ('rice', 'potatoes', ...), or None.\n\n    A colour in front of \"one\" is a choice between two objects and names no side: see\n    `COLOUR_CHOICE`.\n    ");
    if py_truthy(&a_colour_choice(&[text.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SIDE_WORDS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    phrases = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&phrases.clone());
    for __item in __iter3 {
    phrase = __item.clone();
    if py_truthy(&_phrase_in(&[text.clone(), phrase.clone()])) {
    return key.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn recent_texts(args: &[Py]) -> Py {
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
    let mut record: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut users: Py = Py::None;
    let _ = py_str("What the conversation has been about, newest first, in the user's own words.\n\n    Only their lines count: a dish the bot mentioned in passing (\"with rice or bread\") is\n    not the dish they are cooking, and would be the wrong plan to answer from.\n    ");
    users = py_list(vec![]);
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&turn.clone(), "get", &[py_str("user")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&users.clone(), "append", &[geti(&turn.clone(), &py_str("user"))]);
    }
    }
    }
    record = mcall(&memory.clone(), "get", &[py_str("topic")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&record.clone(), "get", &[py_str("text")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&users.clone(), "append", &[geti(&record.clone(), &py_str("text"))]);
    }
    return users.clone();
    Py::None
}
pub fn dish_for(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut key: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The dish this conversation is about: named in the line itself, or just before it.\n\n    The user's own words come first, because what they said is the dish they mean; the\n    lines before count only when this one names nothing nameable (\"do I need to take the skin\n    off?\" after \"what can I cook with chicken thighs?\"). A line that names a food with no plan\n    of its own (\"how do I know when rice is done\") is about that food, not about the bird\n    roasted two turns ago, so it gets nothing.\n    ");
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    key = dish_in(&[text.clone()]);
    if py_truthy(&key.clone()) {
    return key.clone();
    }
    if py_truthy(&_names_food(&[text.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(6), &Py::None));
    for __item in __iter1 {
    earlier = __item.clone();
    key = dish_in(&[earlier.clone()]);
    if py_truthy(&key.clone()) {
    return key.clone();
    }
    }
    return Py::None;
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Bread and cakes go heavy for one reason, and mash, rice or porridge for the other.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g__POT()) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Heavy mash is wet potatoes and too much mashing: drain them well, let the steam off for a minute, mash them hot with warm milk and butter, and stop as soon as they are smooth. If it is already heavy, beat in a spoon of warm milk or a knob of butter and serve it at once.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g__STARCH()) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Stodgy rice, porridge or couscous is too much water and too much stirring: use less liquid than the packet says, stop stirring once it is in the pan, and let it stand off the heat for 5 minutes so the starch sets before you serve it.");
    }
    return py_str("Bread and cakes go heavy when the raising agent is stale, the mix is overworked or the oven is too cool: check the date on the powder, fold rather than beat, and give the oven 20 minutes to heat properly.");
    Py::None
}
pub fn _gluey(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut have: Py = Py::None;
    let mut mushy: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A gluey dish gets the fix for the dish it is, not the rice instruction.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    mushy = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g__MUSHY_WORDS()) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mushy.clone()) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("mash"));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("bread"), py_str("cake"), py_str("sponge"), py_str("loaf"), py_str("loaves"), py_str("muffin"), py_str("muffins"), py_str("brownie"), py_str("brownies"), py_str("pastry"), py_str("bun"), py_str("buns"), py_str("scone"), py_str("scones")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("bake"));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("risotto"), py_str("risotto rice"), py_str("paella")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("risotto"));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("sauce"), py_str("gravy"), py_str("curry"), py_str("soup"), py_str("stew"), py_str("custard"), py_str("bolognese")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("sauce"));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("rice"), py_str("risotto")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("rice"));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("couscous"), py_str("bulgur"), py_str("bulghur"), py_str("quinoa"), py_str("polenta")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("grain"));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("porridge"), py_str("oats"), py_str("oatmeal"), py_str("porage")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__MUSHY_FIX(), &py_str("porridge"));
    }
    return geti(&g__MUSHY_FIX(), &py_str(""));
    }
    have = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GLUE()) { let mut group: Py = __item.clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&have.clone()))) {
    return py_str("Gluey starch is nearly always too much water and too much stirring: stop stirring it, drain off any water it is still sitting in as soon as a piece is tender, and let it stand off the heat for a minute before you serve it.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("mash"), py_str("mashed potato"), py_str("mashed potatoes"), py_str("potato"), py_str("potatoes")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Mash goes gluey when it is mashed too long, or the potatoes went in wet: drain them well and let the steam off for a minute, mash while they are hot with warm milk and butter rather than cold, and stop as soon as it is smooth. A gluey mash that is already made comes back if you beat in a spoon of warm milk and serve it at once, but never put it in a food processor.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("noodle"), py_str("noodles"), py_str("udon"), py_str("ramen"), py_str("rice noodle"), py_str("rice noodles")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Noodles clump when they are boiled in a small pan and then left to sit: give them a big pan of boiling water, stir them apart in the first minute, and rinse them under warm water once they are drained if you are not serving them at once. A drop of oil in the water does nothing.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("couscous"), py_str("bulgur"), py_str("bulghur"), py_str("quinoa"), py_str("polenta")])) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("This one wants less water and a fork, not a spoon: pour boiling water or stock over it in a wide bowl, 1 part to 1.5 parts for couscous and 1 to 2 for quinoa, cover it and leave it 5 minutes, then fork it through to break the lumps. Stirring it while it soaks is what makes it claggy.");
    }
    return py_str("Sticky pasta is a starch coating, not a cooking fault: use a big pan with plenty of water, stir it for the first minute so the pieces separate, and drain it but do not rinse it unless it is for a cold salad. If it has already clumped, a spoon of the cooking water tossed through it loosens it again.");
    Py::None
}
pub fn _fault_groups(args: &[Py]) -> Py {
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
    let mut groups: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "groups") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: groups"))) };
    let mut key: Py = Py::None;
    let mut names: Py = Py::None;
    let mut said: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Which food family the line is about, or None when the line names no food at all.\n\n    A line that names nothing - \"how do i stop it going lumpy\", or the bare \"why\" after an\n    answer - is the previous card's business: the gravy card it followed already says which way\n    the flour went in, and the general mixing sentence in its place was a step down. So the\n    borrowing stops here and the row passes.\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&groups.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    key = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&names.clone()) { let mut wanted: Py = __item.clone(); __out.push(_same_word(&[word.clone(), wanted.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return key.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_food(&[_line_text(&[understood.clone()])])))) {
    return Py::None;
    }
    return py_str("");
    Py::None
}
pub fn _grainy(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let _ = py_str("A grainy dish gets the cause of the break, which is different for meat, mash and eggs.");
    key = _fault_groups(&[memory.clone(), understood.clone(), g__GRAINY()]);
    return if py_truthy(&Py::Bool(py_is(&key.clone(), &Py::None))) { Py::None } else { geti(&g__GRAINY_FIX(), &key.clone()) };
    Py::None
}
pub fn _lumpy(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let _ = py_str("Lumpy is a mixing fault, so the food in the line decides the tool and the next step.");
    key = _fault_groups(&[memory.clone(), understood.clone(), g__LUMPY()]);
    return if py_truthy(&Py::Bool(py_is(&key.clone(), &Py::None))) { Py::None } else { geti(&g__LUMPY_FIX(), &key.clone()) };
    Py::None
}
pub fn _veg_fix(args: &[Py]) -> Py {
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
    let mut fault: Py = Py::None;
    let mut fix: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The fix for a vegetable that stayed hard or went tough, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEG_FAULTS()) { let mut fault: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), fault.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("jacket potato"), py_str("baked potato"), py_str("jacket potatoes"), py_str("baked potatoes")])) { let mut whole: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), whole.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A jacket potato is hard in the middle because it was rushed: prick it, microwave it 8 to 10 minutes until a knife goes in, then 15 to 20 minutes at 200C to crisp the skin. In the oven alone it needs an hour and a quarter at 200C, so give it that long or start it early.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("new potatoes")])) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("oven"), py_str("crispy"), py_str("tray")])) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Hard roast potatoes went in too big or came out too early: give them another 10 to 15 minutes at 200C, and next time cut them no bigger than a golf ball and parboil them 8 minutes first, which is what cooks the middle before the outside crisps.");
    }
    return py_str("Boiled potatoes go hard in the middle when the pieces are uneven: cut them the same size, start them in cold salted water and simmer 15 to 20 minutes, until a knife slides in. An old potato can be woody at the core, and that will not soften however long it cooks.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("mushroom"), py_str("mushrooms")])) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Rubbery mushrooms have been steamed rather than fried: they hold a lot of water, so give them a wide hot pan with a little oil, one layer and no lid, and let that water cook off before you turn them. A pinch of salt only at the end, or they give up their water early and stew.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("dried"), py_str("haricot"), py_str("haricots"), py_str("cannellini"), py_str("borlotti"), py_str("butter bean"), py_str("butter beans"), py_str("chickpea"), py_str("chickpeas"), py_str("split peas"), py_str("lentil"), py_str("lentils"), py_str("puy")])) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_VEG_FAMILIES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    fix = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&_phrase_in(&[text.clone(), name.clone()])) {
    return fix.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_sink_is_a_bake(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a \"sink\" in this text is the verb about a bake, not the kitchen sink.");
    low = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SINK_AS_A_BAKE()) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[low.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_plan_that_might_be_ruined(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True unless the line is weighing what a choice would do to a dish, not reporting a fault.\n\n    One reader for every row that answers a dish that went wrong - the fault list in\n    `cooking_gaps`, the bake list beside it, and this module's own table of everyday problems -\n    because the fault is the same in all of them and a fix that only reaches one leaves the same\n    wrong answer on the same line two entries over.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__ASKED_ABOUT_THE_FAULT(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(py_is(&mcall(&g__PLAN_THAT_MIGHT_BE_RUINED(), "search", &[text.clone()]), &Py::None));
    Py::None
}
pub fn problem_in(args: &[Py]) -> Py {
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
    let mut _by_dish: Py = Py::None;
    let mut _general: Py = Py::None;
    let mut key: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Which of the everyday problems this text names, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&g_PROBLEMS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 3, None);
    words = __unpk3[0].clone();
    _general = __unpk3[1].clone();
    _by_dish = __unpk3[2].clone();
    let __iter4 = py_iter_or_raise(&words.clone());
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__SINK_VERBS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_sink_is_a_bake(&[text.clone()]))) } }) {
    continue;
    }
    if py_truthy(&_phrase_in(&[text.clone(), word.clone()])) {
    return key.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_subject_of_the_state(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut before: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut i: Py = Py::None;
    let mut low: Py = Py::None;
    let mut one: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut verbs: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words the state `word` is said of, or [] when the line states no subject.\n\n    \"the soil is dry\" -> [\"soil\"]; \"my muffins are dry\" -> [\"muffins\"]; \"dry again\" -> []. The\n    clause the state sits in is what decides, because a line can say two things at once.\n    ");
    low = mcall(&text.clone(), "lower", &[]);
    found = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&word.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_list(vec![]);
    }
    before = slice_or_raise(&low.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None);
    clause = geti(&stdlib_call("re", "split", &[py_str("[,.;:!?]|\\band\\b"), before.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(1)));
    words = mcall(&clause.clone(), "split", &[]);
    verbs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2499 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk2499[0].clone(); let mut one: Py = __unpk2499[1].clone(); if py_truthy(&mcall(&g__A_STATE_VERB(), "fullmatch", &[one.clone()])) {  __out.push(i.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&verbs.clone()))) {
    return py_list(vec![]);
    }
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &geti(&verbs.clone(), &op_neg(&Py::Int(1))), &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "isalpha", &[])) {  __out.push(one.clone()); } } py_list(__out) })() };
    return slice_or_raise(&subject.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None);
    Py::None
}
pub fn _the_state_is_about_the_food(args: &[Py]) -> Py {
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
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut _by_dish: Py = Py::None;
    let mut _general: Py = Py::None;
    let mut not_food: Py = Py::None;
    let mut one: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the fault `key` is said of the dish, and not of something else in the room.\n\n    \"my muffins are dry\" says dry of the muffins and wants the bake answer. \"i water them when\n    the soil is dry\" says dry of the soil, and the roast answer that followed was a fluent reply\n    about something else entirely. The state's own subject decides, and only a subject that names\n    a place, a surface, a material or a part of a person turns the table down; a subject that\n    names anything else - a dish, a food, a pronoun - is left to the answer it already had.\n    ");
    let __unpk1 = py_unpack(&geti(&g_PROBLEMS(), &key.clone()), 3, None);
    words = __unpk1[0].clone();
    _general = __unpk1[1].clone();
    _by_dish = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    subject = _the_subject_of_the_state(&[text.clone(), word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&subject.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__STANDS_FOR_IT(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&subject.clone()) { let mut one: Py = __item.clone(); for __item in py_iter_or_raise(&g_NOT_THE_FOOD()) { let mut not_food: Py = __item.clone(); __out.push(_same_word(&[one.clone(), not_food.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _names_food(args: &[Py]) -> Py {
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
    let mut food: Py = Py::None;
    let mut word: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()])) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g_FOOD_WORDS()) { let mut food: Py = __item.clone(); __out.push(_same_word(&[word.clone(), food.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn cook_ask(args: &[Py]) -> Py {
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
    let mut food: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks what to make or eat with a food it names.");
    text = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COOK_PHRASES()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOOD_WORDS()) { let mut food: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &food.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _problem(args: &[Py]) -> Py {
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
    let mut by_dish: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut general: Py = Py::None;
    let mut key: Py = Py::None;
    let mut one: Py = Py::None;
    let mut text: Py = Py::None;
    let mut veg: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The fix for the thing that keeps going wrong, about the dish being cooked.");
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    key = problem_in(&[text.clone()]);
    if py_truthy(&{ let __b = key.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_state_is_about_the_food(&[text.clone(), key.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(2), &Py::None));
    for __item in __iter1 {
    earlier = __item.clone();
    key = problem_in(&[earlier.clone()]);
    if py_truthy(&{ let __b = key.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_state_is_about_the_food(&[earlier.clone(), key.clone()]))) } }) {
    key = Py::None;
    continue;
    }
    if py_truthy(&key.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return _veg_fix(&[text.clone()]);
    }
    let __unpk2 = py_unpack(&geti(&g_PROBLEMS(), &key.clone()), 3, None);
    words = __unpk2[0].clone();
    general = __unpk2[1].clone();
    by_dish = __unpk2[2].clone();
    dish = dish_for(&[memory.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(py_contains(&by_dish.clone(), &dish.clone()))) {
    return geti(&by_dish.clone(), &dish.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&dish.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&py_list(vec![text.clone()]), &slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(2), &Py::None))) { let mut one: Py = __item.clone(); __out.push(_names_food(&[one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    veg = _veg_fix(&[text.clone()]);
    if py_truthy(&veg.clone()) {
    return veg.clone();
    }
    if py_truthy(&call_builtin("callable", &[general.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_call(&general, &[memory.clone(), understood.clone()]);
    }
    return general.clone();
    Py::None
}
pub fn _any_food(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    if py_truthy(&Py::Bool(py_is(&g__ANY_FOOD(), &Py::None))) {
    s__ANY_FOOD(call_builtin("tuple", &[call_builtin("sorted", &[op_bitor(&op_bitor(&call_builtin("set", &[g_PLAN_NAMES()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_FOOD_WORDS()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("set", &[g_DISH_WORDS()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    }
    return g__ANY_FOOD();
    Py::None
}
pub fn _asks_doneness(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks whether a *food* is done, by name or by pronoun.\n\n    The two regexes above say nothing about food on their own, so a line whose subject is a\n    boiler, the washing, compost, a sourdough starter or a radiator was read as a doneness\n    question and then answered with the food from the conversation: \"how do i know if my\n    boiler is ready\" was told about the rice. A doneness question either names a food the notes\n    know, or names only a pronoun or a generic word and leaves the food to the conversation.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    match_ = { let __b = mcall(&g_DONE_ASK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DONE_VERB_ASK(), "search", &[said.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    span = mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    if py_truthy(&_ready_food(&[span.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_any_food(&[])) { let mut name: Py = __item.clone(); __out.push(_phrase_in(&[span.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), span.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__ASK_NOISE(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DONE_WORD(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&subject.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__GENERIC_SUBJECT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn ask_kind(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut asks: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_rank: Py = Py::None;
    let mut have: Py = Py::None;
    let mut helper: Py = Py::None;
    let mut helpers: Py = Py::None;
    let mut keeps: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut making_a_fault: Py = Py::None;
    let mut rank: Py = Py::None;
    let mut ready: Py = Py::None;
    let mut said: Py = Py::None;
    let mut score: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Which kind of follow-up this line is: 'temp', 'time', 'done', ... or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&have.clone()))) {
    return Py::None;
    }
    said = call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e));
    ready = if py_truthy(&_asks_doneness(&[said.clone()])) { Py::Int(6) } else { Py::Int(0) };
    keeps = if py_truthy(&mcall(&g_KEEP_ASK(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])) { Py::Int(6) } else { Py::Int(0) };
    making_a_fault = call_builtin("bool", &[mcall(&g_MAKING_A_FAULT(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, py_tuple(vec![Py::Int(0), Py::Int(0)])]), 2, None);
    best = __unpk1[0].clone();
    best_rank = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g_ASK_KINDS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    kind = __unpk3[0].clone();
    asks = __unpk3[1].clone();
    helpers = __unpk3[2].clone();
    score = Py::Int(0);
    let __iter4 = py_iter_or_raise(&asks.clone());
    for __item in __iter4 {
    ask = __item.clone();
    if py_truthy(&_phrase_in(&[text.clone(), ask.clone()])) {
    let __aug5 = op_add(&score.clone(), &op_add(&Py::Int(2), &call_builtin("len", &[pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[ask.clone()])]).unwrap_or_else(|e| exc::raise(e))));
    score = __aug5;
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("done"))); if !py_truthy(&__b) { __b } else { ready.clone() } }) {
    score = call_builtin("max", &[score.clone(), ready.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("keep"))); if !py_truthy(&__b) { __b } else { keeps.clone() } }) {
    score = call_builtin("max", &[score.clone(), keeps.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&score.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&helpers.clone()) { let mut helper: Py = __item.clone(); __out.push(_same_word(&[word.clone(), helper.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug6 = op_add(&score.clone(), &Py::Int(1));
    score = __aug6;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("keep"))); if !py_truthy(&__b) { __b } else { making_a_fault.clone() } }) {
    continue;
    }
    rank = py_tuple(vec![if py_truthy(&Py::Bool(py_contains(&g_PART(), &kind.clone()))) { Py::Int(2) } else { if py_truthy(&Py::Bool(py_contains(&g_PLAIN(), &kind.clone()))) { Py::Int(0) } else { Py::Int(1) } }, score.clone()]);
    if py_truthy(&op_gt(&rank.clone(), &best_rank.clone())) {
    let __unpk7 = py_unpack(&py_tuple(vec![kind.clone(), rank.clone()]), 2, None);
    best = __unpk7[0].clone();
    best_rank = __unpk7[1].clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _first_minutes(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The first cooking time in a sentence, in minutes, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_TIME_IN_ANSWER(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    head = slice_or_raise(&mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "lower", &[]), &op_neg(&Py::Int(30)), &Py::None, &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KEEPING_LEAD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&head.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    count = call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "lower", &[]), "startswith", &[py_str("hour")])) { op_mul(&count.clone(), &Py::Int(60)) } else { count.clone() };
    }
    return Py::None;
    Py::None
}
pub fn _last_answer_said(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return py_str("");
    }
    said = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]);
    return if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { said.clone() } else { py_str("") };
    Py::None
}
pub fn _plan_contradicts_the_last_answer(args: &[Py]) -> Py {
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
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut ours: Py = Py::None;
    let mut told: Py = Py::None;
    let _ = py_str("True when the last answer gave a time for a different version of this dish.\n\n    The dish the follow-up is about is the one the last answer described, so a plan whose time\n    is nowhere near it is a different dish that shares a word (\"chicken\" for a pan of strips\n    and for a whole roast bird).\n    ");
    told = _first_minutes(&[_last_answer_said(&[memory.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&told.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&told.clone())) } }) {
    return Py::Bool(false);
    }
    ours = _first_minutes(&[{ let __b = mcall(&plan.clone(), "get", &[py_str("time")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&ours.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&ours.clone())) } }) {
    return Py::Bool(false);
    }
    return op_gt(&call_builtin("max", &[told.clone(), ours.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_mul(&Py::Int(2), &call_builtin("min", &[told.clone(), ours.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _parts_in(args: &[Py]) -> Py {
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
    let _ = py_str("The parts of a dish a line names, so \"the filling\" is not \"the icing\" said twice.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PART_WORDS()) { let mut word: Py = __item.clone(); if py_truthy(&_phrase_in(&[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, word.clone()])) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _answering_line(args: &[Py]) -> Py {
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
    let _ = py_str("The person's line that the last answer was answering, as they wrote it.\n\n    The newest turn in memory is the one that produced the last reply, so its user line is the\n    ask the paragraph answered: \"and for the filling\" must not read the icing question as\n    somebody else's part.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&turn.clone(), "get", &[py_str("user")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    if py_truthy(&mcall(&geti(&turn.clone(), &py_str("user")), "strip", &[])) {
    return geti(&turn.clone(), &py_str("user"));
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _same_answer_but_a_new_part(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut already: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The answer said for the part this line names, when the last answer already said it.\n\n    \"what should i use for icing?\" then \"and for the filling\" is two questions and one\n    paragraph. The guard below stops the paragraph being repeated word for word, which is right\n    for \"what temperature?\" asked twice and wrong here, so the part is named in front of it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    text = _line_text(&[understood.clone()]);
    asked = _parts_in(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PART_BACKREF()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    already = _parts_in(&[_answering_line(&[memory.clone()])]);
    if py_truthy(&op_sub(&asked.clone(), &already.clone())) {
    part = geti(&call_builtin("sorted", &[op_sub(&asked.clone(), &already.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    return op_add(&op_add(&{ let mut __s = String::new(); __s.push_str("Same for the "); __s.push_str(&fstr(&part.clone(), -1, "")); __s.push_str(": "); py_string(__s) }, &mcall(&geti(&answer.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&answer.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_sub(&asked.clone(), &already.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&already.clone())) } } } }) {
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _dish_named_in_the_line(args: &[Py]) -> Py {
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
    let _ = py_str("The dish this line names itself, or None when it only leans on the one before.\n\n    A line that names its own dish asks about that dish, whatever the last answer timed: \"how\n    long does a sunday roast take\" after the carrots is the roast, and the carrot's ten minutes\n    are no reason to turn its plan down.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return dish_in(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]);
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
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut answer: Py = Py::None;
    let mut key: Py = Py::None;
    let mut no: Py = Py::None;
    let mut part: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("The answer to a follow-up of this kind about the dish in the conversation, or None.");
    key = dish_for(&[memory.clone(), understood.clone()]);
    plan = if py_truthy(&key.clone()) { mcall(&g_PLANS(), "get", &[key.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__NAMES_A_SAUCE(), "search", &[call_builtin("str", &[_line_text(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_dish_named_in_the_line(&[understood.clone()]))); if !py_truthy(&__b) { __b } else { _plan_contradicts_the_last_answer(&[memory.clone(), plan.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PART_OF(), &kind.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PART_NOT()) { let mut no: Py = __item.clone(); __out.push(_phrase_in(&[_line_text(&[understood.clone()]), no.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    answer = { let __b = mcall(&plan.clone(), "get", &[kind.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("ahead"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&answer.clone())) } }) {
    answer = mcall(&plan.clone(), "get", &[py_str("keep")]);
    }
    if py_truthy(&{ let __b = answer.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&answer.clone(), &_last_answer_said(&[memory.clone()]))) } }) {
    part = _same_answer_but_a_new_part(&[memory.clone(), understood.clone(), answer.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&part.clone(), "strip", &[]) } }) {
    return part.clone();
    }
    return answer.clone();
    }
    return answer.clone();
    Py::None
}
pub fn _part_ahead(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut key: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The answer about a part the line names when it names no dish of its own.\n\n    \"Can I make the frosting the day before?\" and \"can I make the ganache ahead?\" have no dish\n    in them, so `dish_for` finds nothing and every entry passes on the line. The part itself\n    says which dish: frosting only goes on a cake. Only the ahead-and-keeping questions are\n    answered this way, because a part word says nothing about the oven.\n    ");
    if py_truthy(&Py::Bool(!py_is(&dish_for(&[memory.clone(), understood.clone()]), &Py::None))) {
    return Py::None;
    }
    text = _line_text(&[understood.clone()]);
    key = implied_dish(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&key.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IMPLIED_ASKS()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    plan = { let __b = mcall(&g_PLANS(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    answer = { let __b = mcall(&plan.clone(), "get", &[py_str("frosting")]); if py_truthy(&__b) { __b } else { mcall(&plan.clone(), "get", &[py_str("keep")]) } };
    if py_truthy(&{ let __b = answer.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&answer.clone(), &_last_answer_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    return answer.clone();
    Py::None
}
pub fn _is_a_keeping_line(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True for \"how long does lasagne keep\", false for \"how long do i keep it in the oven\".");
    line = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    return { let __b = call_builtin("bool", &[mcall(&g__KEEP_ASK(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KEEP_BUT_COOKING()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn _about_a_person(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line is about a person rather than about the dish being cooked.");
    line = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    return call_builtin("bool", &[{ let __b = mcall(&g_PERSON_STATE(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BODY_TEMPERATURE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PERSON_REST(), "search", &[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _raw_in_flight(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the conversation is about eating something raw or undercooked.\n\n    \"can I still eat it\" after \"is it ok to eat raw cookie dough\" is a question about raw dough,\n    and the dough row answers it: \"keeps 3 days in a paper bag\" does not.\n    ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(3), &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[line.clone()])) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g_RAW_WORDS()) { let mut raw: Py = __item.clone(); __out.push(_same_word(&[word.clone(), raw.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _follow_up(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut ready: Py = Py::None;
    let _ = py_str("The answer to whatever kind of follow-up the line is, about the dish so far.");
    line = _line_text(&[understood.clone()]);
    if py_truthy(&{ let __b = _about_a_person(&[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _dish_named_in_the_line(&[understood.clone()]); if py_truthy(&__b) { __b } else { dish_in(&[line.clone()]) } })) } }) {
    return Py::None;
    }
    kind = ask_kind(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("keep"))); if !py_truthy(&__b) { __b } else { _raw_in_flight(&[memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("keep"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_parts_in(&[_line_text(&[understood.clone()])]))); if !py_truthy(&__b) { __b } else { _is_a_keeping_line(&[_line_text(&[understood.clone()])]) } } } }) {
    answer = reply(&[py_str("keep"), memory.clone(), understood.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("fix")))) {
    return { let __b = _problem(&[memory.clone(), understood.clone()]); if py_truthy(&__b) { __b } else { _part_ahead(&[memory.clone(), understood.clone()]) } };
    }
    answer = if py_truthy(&kind.clone()) { reply(&[kind.clone(), memory.clone(), understood.clone()]) } else { Py::None };
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("done")))) {
    ready = _ready_note(&[memory.clone(), understood.clone()]);
    if py_truthy(&ready.clone()) {
    return ready.clone();
    }
    }
    return _part_ahead(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn follow_up(args: &[Py]) -> Py {
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
    let _ = py_str("The plan's answer for a line that only asks, when no entry claimed the line itself.\n\n    \"how long?\" after a cake's temperature, \"what temperature?\" after a roast: the line names\n    nothing, so no entry fits it, and the dish is the one the conversation is already about.\n    The composer offers this before the answer it already gave, because the plan says the whole\n    thing (\"20 minutes per 500g plus 20 minutes, so 1 hour 40\") where slicing the last answer\n    gives a sentence that may be about resting instead of cooking.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    return _follow_up(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn _temp_asks(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut asks: Py = Py::None;
    let mut helpers: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The ways of asking for the oven's heat, as one key.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    asks = __unpk1[0].clone();
    helpers = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g_ASK_KINDS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    kind = __unpk3[0].clone();
    parts = __unpk3[1].clone();
    words = __unpk3[2].clone();
    if py_truthy(&Py::Bool(!py_eq(&kind.clone(), &py_str("temp")))) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&parts.clone());
    for __item in __iter4 {
    part = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&asks.clone(), &part.clone()))) {
    let _ = mcall(&asks.clone(), "append", &[part.clone()]);
    }
    }
    let __iter5 = py_iter_or_raise(&words.clone());
    for __item in __iter5 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&helpers.clone(), &word.clone()))) {
    let _ = mcall(&helpers.clone(), "append", &[word.clone()]);
    }
    }
    }
    return py_tuple(vec![call_builtin("tuple", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[helpers.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _the_heat(args: &[Py]) -> Py {
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
    let _ = py_str("The plan's figure for the heat, when the card for this dish answers with a recipe.\n\n    The figure is what was asked for and the plan holds it in one sentence. Only the dishes\n    whose card buries it get this row, and only when the line names the dish: a bare \"what\n    temperature?\" after a cake is a follow-up, and the follow-up row has it.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&ask_kind(&[understood.clone()]), &py_str("temp")))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_dish_named_in_the_line(&[understood.clone()])))) {
    return Py::None;
    }
    return reply(&[py_str("temp"), memory.clone(), understood.clone()]);
    Py::None
}
pub fn _asks(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut _u: Py = Py::None;
    let mut asks: Py = Py::None;
    let mut helpers: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every way of asking a follow-up, and the words that come with it, as one key.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    asks = __unpk1[0].clone();
    helpers = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g_ASK_KINDS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk3[0].clone();
    parts = __unpk3[1].clone();
    words = __unpk3[2].clone();
    let __iter4 = py_iter_or_raise(&parts.clone());
    for __item in __iter4 {
    part = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&asks.clone(), &part.clone()))) {
    let _ = mcall(&asks.clone(), "append", &[part.clone()]);
    }
    }
    let __iter5 = py_iter_or_raise(&words.clone());
    for __item in __iter5 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&helpers.clone(), &word.clone()))) {
    let _ = mcall(&helpers.clone(), "append", &[word.clone()]);
    }
    }
    }
    return py_tuple(vec![call_builtin("tuple", &[asks.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[helpers.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _asking_for_a_side(args: &[Py]) -> Py {
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
    let mut have: Py = Py::None;
    let mut phrase: Py = Py::None;
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&have.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_USE_UP_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&g_ASK_OPENERS(), &geti(&have.clone(), &Py::Int(0))));
    Py::None
}
pub fn _start(args: &[Py]) -> Py {
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
    let mut announced: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The practical plan for the dish the line says is being cooked, or None.");
    if py_truthy(&ask_kind(&[understood.clone()])) {
    return Py::None;
    }
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    announced = call_builtin("bool", &[mcall(&g_PERSON_START(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = announced.clone(); if py_truthy(&__b) { __b } else { mcall(&g_DELIBERATING_START(), "search", &[text.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&announced.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHEN_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&_their_diet_refuses(&[memory.clone(), understood.clone(), text.clone()])) {
    return Py::None;
    }
    return _start_sentence(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn _their_diet_refuses(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut _bans_for: Py = Py::None;
    let mut _bans_it: Py = Py::None;
    let mut _bans_said: Py = Py::None;
    let mut _foods_named: Py = Py::None;
    let mut banned: Py = Py::None;
    let mut food: Py = Py::None;
    let _ = py_str("True when a diet the conversation holds bans what this line names.\n\n    \"i fancy a steak\" a turn after \"im vegetarian\" is the diet's question, and main answered it\n    with the swap (\"No: a steak is meat... chickpeas, lentils or tofu fill the same gap\"). Saying\n    the plan's time and heat for a dish somebody has just ruled out is the one answer worse than\n    no answer, so the plan stands down and the diet row answers.\n    ");
    _bans_for = py_func("_bans_for", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_bans_for", args));
    _bans_it = py_func("_bans_it", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_bans_it", args));
    _bans_said = py_func("_bans_said", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_bans_said", args));
    _foods_named = py_func("_foods_named", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_foods_named", args));
    banned = py_call(&_bans_for, &[understood.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&py_call(&_bans_said, &[banned.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&_foods_named, &[text.clone()])) { let mut food: Py = __item.clone(); __out.push(py_call(&_bans_it, &[food.clone(), banned.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _start_sentence(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut key: Py = Py::None;
    let mut label: Py = Py::None;
    let mut low: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut temp: Py = Py::None;
    let mut text: Py = Py::None;
    let mut time: Py = Py::None;
    let _ = py_str("The plan's own first sentence for the dish in the line: name, time, heat.");
    key = dish_for(&[memory.clone(), understood.clone()]);
    plan = if py_truthy(&key.clone()) { mcall(&g_PLANS(), "get", &[key.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    label = { let __b = mcall(&plan.clone(), "get", &[py_str("label")]); if py_truthy(&__b) { __b } else { geti(&geti(&geti(&plan.clone(), &py_str("names")), &Py::Int(0)), &Py::Int(0)) } };
    low = mcall(&label.clone(), "lower", &[]);
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    said = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&plan.clone(), &py_str("names"))) { let mut group: Py = __item.clone(); for __item in py_iter_or_raise(&group.clone()) { let mut name: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &geti(&mcall(&name.clone(), "split", &[]), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])")), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(name.clone()); } } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&low.clone(), &said.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&said.clone(), &low.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(6), &Py::None), &slice_or_raise(&low.clone(), &Py::None, &Py::Int(6), &Py::None))) } } } } } }) {
    label = said.clone();
    }
    label = op_add(&mcall(&geti(&label.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&label.clone(), &Py::Int(1), &Py::None, &Py::None));
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&plan.clone(), "get", &[py_str("time")]), mcall(&plan.clone(), "get", &[py_str("temp")])]), 2, None);
    time = __unpk1[0].clone();
    temp = __unpk1[1].clone();
    if py_truthy(&{ let __b = time.clone(); if !py_truthy(&__b) { __b } else { { let __b = temp.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&time.clone(), &mcall(&geti(&mcall(&temp.clone(), "split", &[py_str("C")]), &Py::Int(0)), "strip", &[]))) } } } }) {
    temp = Py::None;
    }
    parts = py_list(vec![time.clone(), temp.clone()]);
    return op_add(&{ let mut __s = String::new(); __s.push_str(&fstr(&label.clone(), -1, "")); __s.push_str(": "); py_string(__s) }, &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]));
    Py::None
}
pub fn _asks_about_another_thing(args: &[Py]) -> Py {
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
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut char: Py = Py::None;
    let mut group: Py = Py::None;
    let mut name: Py = Py::None;
    let mut own: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the ask names a subject of its own that is not the dish being announced.\n\n    \"im doing a roast, how long do leeks take to cook\" tells us the roast and asks about the\n    leeks: the leek entry has that time, and the plan's figure is for the joint. The same line\n    about the potatoes or about basmati rice is the same shape. The ask starts at its how/what\n    word, and the words after it that are neither grammar nor action are what it is about; if\n    one of those is not the dish's own name, the question belongs to that thing's entry.\n    ");
    plan = { let __b = mcall(&g_PLANS(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    own = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&plan.clone(), "get", &[py_str("names")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    group = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[group.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { group.clone() } else { py_tuple(vec![group.clone()]) });
    for __item in __iter2 {
    name = __item.clone();
    let _ = mcall(&own.clone(), "update", &[pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))])]);
    }
    }
    parts = stdlib_call("re", "split", &[py_str("\\bhow\\b|\\bwhat temperature\\b|\\bwhat temp\\b|\\bis it done\\b"), mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    let __iter3 = py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[geti(&parts.clone(), &op_neg(&Py::Int(1)))]));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ASK_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&own.clone(), &word.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&word.clone()) { let mut char: Py = __item.clone(); __out.push(mcall(&char.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn announced_part(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut key: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The plan's answer for a line that announces a dish and asks about cooking it at once.\n\n    \"I'm making a big lasagne tonight, how long in the oven\" tells us the dish and asks the\n    time, and `_start` declines it because it asks something, so the line reaches whichever\n    entry fits the words: the pasta-bake method said 20 to 25 minutes at 200C for a lasagne\n    that the plan gives 35 to 40 at 180C. A bare announcement is `_start`'s own, and it stays\n    there: an entry that knows the occasion (\"a cake for my daughter's birthday\" wants the\n    two-layer sponge made the day before) says more than the plan's figures do. The dish has to\n    be named in this line, because the plan answers for the dish being cooked now.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    kind = ask_kind(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&g_STARTED_PARTS(), &kind.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STAGE_PARTS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PERSON_START(), "search", &[text.clone()])))) {
    return Py::None;
    }
    key = dish_in(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return Py::None;
    }
    if py_truthy(&_asks_about_another_thing(&[text.clone(), key.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("time"), py_str("temp")]), &kind.clone()))) {
    answer = _start_sentence(&[memory.clone(), understood.clone()]);
    } else {
    answer = reply(&[kind.clone(), memory.clone(), understood.clone()]);
    }
    return if py_truthy(&{ let __b = call_builtin("isinstance", &[answer.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&answer.clone(), "strip", &[]) } }) { answer.clone() } else { Py::None };
    Py::None
}
pub fn _asking_to_pair(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what to put beside something, however the verb is dressed.");
    return call_builtin("bool", &[mcall(&g_GOES_WITH_SHAPE(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_any(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says one of these words, as a word rather than a run of letters.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_for_a_side_of_a_food(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the pairing way and names a food it can be paired with.");
    return { let __b = _asking_to_pair(&[text.clone()]); if !py_truthy(&__b) { __b } else { _names_any(&[text.clone(), g_PAIRING_FOODS()]) } };
    Py::None
}
pub fn general_sides(args: &[Py]) -> Py {
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
    let _ = py_str("The pairing rule for a food we have no dish for: fat wants acid, starch wants something\n    wet, and anything else wants one of each.");
    if py_truthy(&_names_any(&[text.clone(), g_RICH_FOODS()])) {
    return g_PAIRING_RICH();
    }
    if py_truthy(&_names_any(&[text.clone(), g_PLAIN_FOODS()])) {
    return g_PAIRING_PLAIN();
    }
    return g_PAIRING_LEAN();
    Py::None
}
pub fn a_colour_choice(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is choosing between two things by their colour, and names neither.");
    text = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g_COLOUR_CHOICE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__CHOICE_FRAME(), "search", &[text.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _sides(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut text: Py = Py::None;
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    if py_truthy(&a_colour_choice(&[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_asking_for_a_side(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SIDE_METHOD()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    key = side_in(&[text.clone()]);
    plan = mcall(&g_PLANS(), "get", &[{ let __b = dish_for(&[memory.clone(), understood.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = key.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&plan.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _asking_to_pair(&[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&key.clone()) {
    return mcall(&g_SIDES(), "get", &[key.clone()]);
    }
    if py_truthy(&plan.clone()) {
    return mcall(&plan.clone(), "get", &[py_str("sides")]);
    }
    if py_truthy(&asks_for_a_side_of_a_food(&[text.clone()])) {
    return general_sides(&[text.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _another_meat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut food: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut bird: Py = Py::None;
    let mut name: Py = Py::None;
    let mut red: Py = Py::None;
    let _ = py_str("A meat to put in place of the one somebody will not eat.");
    name = mcall(&call_builtin("str", &[{ let __b = food.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BIRDS()) { let mut bird: Py = __item.clone(); __out.push(_same_word(&[name.clone(), bird.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("beef or pork");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RED_MEAT()) { let mut red: Py = __item.clone(); __out.push(_same_word(&[name.clone(), red.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("chicken thighs");
    }
    return py_str("chicken thighs or beef");
    Py::None
}
pub fn _foods_named(args: &[Py]) -> Py {
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
    let mut cooking: Py = Py::None;
    let _ = py_str("True when the line names a food of its own, so the question is about that food.\n\n    \"mince and no tomatoes\" and \"my son wont eat onions what can i cook\" name the food they are\n    about, so what to do instead of the one thing they leave out is their answer. \"something\n    quick and cheap for dinner, no cheese\" names no food at all, and that is a dinner to pick.\n    The kitchen's own reading of the line is used, not a word list here.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    let __try1 = exc::catch(|| {
    if py_truthy(&mcall(&cooking.clone(), "ingredients", &[understood.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
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
    return Py::Bool(false);
    Py::None
}
pub fn _states_what_they_have(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says what the kitchen has, not only what it is missing.");
    return call_builtin("bool", &[mcall(&g_HAVE_WORDS(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _dish_in_flight(args: &[Py]) -> Py {
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
    let mut food: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut cooking: Py = Py::None;
    let mut foods: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a dish is named in this line or has been named a turn or two ago.");
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    said = _recent_answers(&[memory.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[said.clone()])) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_DISH_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BAKE_WORDS(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    foods = op_add(&op_add(&g_DISH_WORDS(), &g_BAKE_WORDS()), &call_builtin("tuple", &[call_builtin("getattr", &[cooking.clone(), py_str("_FOOD"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_MEAL_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&foods.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKED()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[word.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::Int(1), &Py::Int(4), &Py::None));
    for __item in __iter2 {
    line = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[line.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DISH_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_dislike(args: &[Py]) -> Py {
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
    let mut lowered: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("True when the line is a person refusing a food, not a bare \"no\" on a dinner.");
    lowered = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SAID_DISLIKE()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[lowered.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_for_a_dinner(args: &[Py]) -> Py {
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
    let mut lowered: Py = Py::None;
    let mut shape: Py = Py::None;
    lowered = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DINNER_ASK()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[lowered.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _somebody_else_refuses(args: &[Py]) -> Py {
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
    let mut lowered: Py = Py::None;
    let mut person: Py = Py::None;
    let mut shape: Py = Py::None;
    lowered = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SAID_DISLIKE_OTHER()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[lowered.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OTHER_PEOPLE()) { let mut person: Py = __item.clone(); __out.push(_phrase_in(&[lowered.clone(), mcall(&person.clone(), "strip", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _two_tastes(args: &[Py]) -> Py {
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
    let mut shape: Py = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[py_str("|")]));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKE_SHAPES()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[part.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LIKING()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[part.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_diet_family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut food: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut family: Py = Py::None;
    let _ = py_str("True when the word the line rules out is a family of foods, not one food.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DIET_FAMILIES()) { let mut family: Py = __item.clone(); __out.push(_same_word(&[food.clone(), family.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _disliked_food(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut w: Py = Py::None;
    let mut waste: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The ingredient the line is ruling out, as it is written, or None.\n\n    The dislike has to sit beside the food: \"whats a quick meal with mince and no fancy stuff\"\n    names mince and a \"no\", and it is a request for a quick mince dinner, not a reason to cook\n    without meat.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKED()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[word.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    before = mcall(&py_str(" "), "join", &[slice_or_raise(&have.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKE_SHAPES()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[before.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EATING_REFUSALS()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[before.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&have.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)) { let mut w: Py = __item.clone(); for __item in py_iter_or_raise(&g_WASTE_WORDS()) { let mut waste: Py = __item.clone(); __out.push(_same_word(&[w.clone(), waste.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = _same_word(&[word.clone(), py_str("peppers")]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&have.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)) { let mut w: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("black"), py_str("white"), py_str("ground"), py_str("cracked"), py_str("peppercorn"), py_str("peppercorns")])) { let mut kind: Py = __item.clone(); __out.push(_same_word(&[w.clone(), kind.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    after = slice_or_raise(&have.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOOD_TAIL_OK()) { let mut word: Py = __item.clone(); __out.push(_same_word(&[geti(&after.clone(), &Py::Int(0)), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    return word.clone();
    }
    return Py::None;
    Py::None
}
pub fn _someone_elses_dislike(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut shape: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the person in the line is not the one speaking.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OWN_DISLIKE()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OTHER_PEOPLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_shortage_the_swap_table_answers(args: &[Py]) -> Py {
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
    let mut have: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut understood: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut cooking: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kitchen's swap answer for a shortage, or None when the line is a dislike.\n\n    \"no butter left for the cake\" is not a dislike of butter: it is the question \"what do I use\n    instead\". The kitchen table already has that answer, and the swap rows read a bare \"no\" as\n    the ask (see _SWAP_ASK), so the reply is theirs. Passed on as this entry's reply because an\n    entry that matches and then declines is the one the turn is decided on, and the swap row\n    would never get the line.\n    ");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKE_VERBS()) { let mut verb: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), verb.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_CUPBOARD(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SHORTAGE_MARKERS()) { let mut marker: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), marker.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    let __iter1 = py_iter_or_raise(&gattr(&cooking.clone(), "ENTRIES"));
    for __item in __iter1 {
    row = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[gattr(&row.clone(), "id")]).unwrap_or_else(|e| exc::raise(e)), "startswith", &[py_str("swap-")])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&gattr(&row.clone(), "key"), &Py::Int(0))) { let mut name: Py = __item.clone(); __out.push(_same_word(&[word.clone(), name.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = if py_truthy(&call_builtin("callable", &[gattr(&row.clone(), "reply")]).unwrap_or_else(|e| exc::raise(e))) { mcall(&row.clone(), "reply", &[memory.clone(), understood.clone()]) } else { gattr(&row.clone(), "reply") };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return said.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_child_who_will_not_eat(args: &[Py]) -> Py {
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
    let mut have: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is a child's refusal to eat and nothing is asked to be cooked for it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_CHILD()) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EATING_REFUSAL()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_COOK_ASK_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _swap_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut food: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut cooking: Py = Py::None;
    let mut name: Py = Py::None;
    let mut refused: Py = Py::None;
    let mut token: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The substitute in place of `food`, or None when it offers another refused food.\n\n    \"she wont eat fish or meat\" was answered \"No fish then: chicken thighs or a tin of\n    chickpeas\", which is the one food the same line had just ruled out: an alternative under a\n    refusal cannot be the refused thing or a member of its family. The general note is safe for\n    any food, so that is what the line gets instead.\n    ");
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    refused = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("families".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cooking.clone(), "_refused_foods", args))(&__args) }) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_same_word(&[name.clone(), food.clone()])))) {  __out.push(name.clone()); } } py_set(__out) })() };
    if py_truthy(&{ let __b = refused.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[said.clone()])) { let mut token: Py = __item.clone(); for __item in py_iter_or_raise(&refused.clone()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[token.clone(), name.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    words = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[g_GENERIC_SWAP()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut token: Py = __item.clone(); for __item in py_iter_or_raise(&refused.clone()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[token.clone(), name.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("food".to_string(), food.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GENERIC_SWAP(), "format", args))(&__args) };
    }
    return said.clone();
    Py::None
}
pub fn _a_family_ruled_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The diet family this line itself refuses (\"and no dairy\"), or None.\n\n    The reading a follow-up arrives in carries the line before it: \"something with chicken\"\n    then \"and no dairy\" comes in as \"no dairy | something with chicken\", and the swap table\n    read the carried chicken and answered \"No chicken then: swap in beef or pork\" to a person\n    who had only said they wanted no dairy. The family is the question the diet card answers,\n    so a refusal of one stands this table down whatever else the line carries.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_a_diet_family(&[word.clone()])))) {
    continue;
    }
    before = mcall(&py_str(" "), "join", &[slice_or_raise(&have.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISLIKE_SHAPES()) { let mut shape: Py = __item.clone(); __out.push(_phrase_in(&[before.clone(), shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _dislike(args: &[Py]) -> Py {
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
    let mut cooking: Py = Py::None;
    let mut family: Py = Py::None;
    let mut food: Py = Py::None;
    let mut have: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shortage: Py = Py::None;
    let mut table: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What to put in place of the ingredient someone does not eat.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LUNCH_MEALS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _a_child_who_will_not_eat(&[text.clone(), have.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SHORTAGE_MARKERS()) { let mut marker: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), marker.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    shortage = _a_shortage_the_swap_table_answers(&[text.clone(), have.clone(), memory.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&shortage.clone(), &Py::None))) {
    return shortage.clone();
    }
    food = _disliked_food(&[have.clone()]);
    family = _a_family_ruled_out(&[have.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&family.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&food.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_same_word(&[food.clone(), family.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&food.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _a_diet_family(&[food.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&food.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g_CUPBOARD(), "search", &[text.clone()]) } }) {
    food = Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&food.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_dish_in_flight(&[memory.clone(), understood.clone(), food.clone()]))); if !py_truthy(&__b) { __b } else { _someone_elses_dislike(&[have.clone(), text.clone()]) } } } }) {
    let __iter1 = py_iter_or_raise(&g_SUBSTITUTES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    said = _swap_said(&[food.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("food".to_string(), food.clone()), ("other".to_string(), _another_meat(&[food.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "format", args))(&__args) }, understood.clone(), text.clone()]);
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    return Py::None;
    }
    return op_add(&said.clone(), &if py_truthy(&_an_allergen(&[food.clone()])) { g__ALLERGEN_NOTE() } else { py_str("") });
    }
    }
    return _swap_said(&[food.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("food".to_string(), food.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GENERIC_SWAP(), "format", args))(&__args) }, understood.clone(), text.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&food.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_dish_in_flight(&[memory.clone(), understood.clone(), food.clone()]))) } }) {
    if py_truthy(&_two_tastes(&[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _states_what_they_have(&[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_somebody_else_refuses(&[text.clone()]))) } }) {
    food = Py::None;
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _foods_named(&[understood.clone()]); if py_truthy(&__b) { __b } else { { let __b = _somebody_else_refuses(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = _said_dislike(&[text.clone()]); if !py_truthy(&__b) { __b } else { _asks_for_a_dinner(&[text.clone()]) } } } } } }))) {
    food = Py::None;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&food.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_HOW_WORDS(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NO_APPLIANCE(), "search", &[text.clone()]) } })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&food.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_food_context(&[memory.clone(), understood.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&food.clone(), &Py::None)); if py_truthy(&__b) { __b } else { mcall(&g_HOW_WORDS(), "search", &[text.clone()]) } }) {
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    let __iter3 = py_iter_or_raise(&py_tuple(vec![gattr(&cooking.clone(), "without"), gattr(&cooking.clone(), "dish"), gattr(&cooking.clone(), "meal")]));
    for __item in __iter3 {
    table = __item.clone();
    said = if py_truthy(&Py::Bool(py_is(&table.clone(), &gattr(&cooking.clone(), "meal")))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __kws.extend(vec![("text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&table, args))(&__args) } } else { py_call(&table, &[understood.clone()]) };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return said.clone();
    }
    }
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_food_context(&[memory.clone(), understood.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _bake_in_flight(&[memory.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("egg"), py_str("eggs")])) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return g_EGG_FREE_BAKE();
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("egg"), py_str("eggs")])) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _an_egg_dish(&[memory.clone()]) } }) {
    return g_EGG_IS_THE_DISH();
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DEFER_TO_ALLERGY()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _bake_in_flight(&[memory.clone()]) } }) {
    return g_NUT_FREE_BAKE();
    }
    if py_truthy(&{ let __b = mcall(&g_OUT_OF_CUE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KITCHEN_SWAP()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    let __iter4 = py_iter_or_raise(&g_SUBSTITUTES());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    names = __unpk5[0].clone();
    said = __unpk5[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("food".to_string(), food.clone()), ("other".to_string(), _another_meat(&[food.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "format", args))(&__args) };
    return op_add(&op_add(&said.clone(), &if py_truthy(&_an_allergen(&[food.clone()])) { g__ALLERGEN_NOTE() } else { py_str("") }), &_the_dish_still(&[memory.clone(), food.clone(), said.clone()]));
    }
    }
    return op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("food".to_string(), food.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GENERIC_SWAP(), "format", args))(&__args) }, &if py_truthy(&_an_allergen(&[food.clone()])) { g__ALLERGEN_NOTE() } else { py_str("") }), &_the_dish_still(&[memory.clone(), food.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("food".to_string(), food.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GENERIC_SWAP(), "format", args))(&__args) }]));
    Py::None
}
pub fn _the_dish_still(args: &[Py]) -> Py {
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
    let mut food: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut answer: Py = Py::None;
    let mut item: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The dish in hand in one of its own sentences, to put after a substitution, or \"\".\n\n    \"what can i make with half a bag of spinach and some eggs\" then \"i dont have any cheese\"\n    used to end with the swap paragraph alone, and that paragraph is about the food: nothing in\n    it is the dish, so the next line (\"how long does it take to cook\") had no dish to land on\n    and the person got \"I'm not sure about that one, sorry\". The dish's own sentence that\n    carries its time keeps it on the table, and the step it names is the answer to the\n    follow-up. Only a sentence with a time in it, because that is the part the follow-up lifts;\n    a sentence naming the missing food is not appended, since it is the food they cannot use.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[mcall(&turn.clone(), "get", &[py_str("bot")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    answer = mcall(&geti(&turn.clone(), &py_str("bot")), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &answer.clone())) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), answer.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentence.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TIME_IN_A_STEP(), "search", &[sentence.clone()]))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[sentence.clone()])) { let mut item: Py = __item.clone(); __out.push(_same_word(&[item.clone(), food.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&_phrase_in(&[said.clone(), mcall(&mcall(&slice_or_raise(&geti(&mcall(&sentence.clone(), "split", &[py_str(":")]), &op_neg(&Py::Int(1))), &Py::None, &Py::Int(24), &Py::None), "strip", &[]), "lower", &[])])) {
    continue;
    }
    step = mcall(&mcall(&g_DISH_LEAD(), "sub", &[py_str(""), sentence.clone()]), "strip", &[]);
    if py_truthy(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[])) {
    step = op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[step.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[step.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STEP_VERBS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return op_add(&py_str(" "), &step.clone());
    }
    return py_str("");
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_dish_in_flight(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut dish: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a conversation line names one of `words`.");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[line.clone()])) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&words.clone()) { let mut dish: Py = __item.clone(); __out.push(_same_word(&[word.clone(), dish.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _bake_in_flight(args: &[Py]) -> Py {
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
    let _ = py_str("True when the conversation is on a cake, a biscuit or a loaf, where nuts are the allergy.");
    return _a_dish_in_flight(&[memory.clone(), g_BAKE_WORDS()]);
    Py::None
}
pub fn _an_egg_dish(args: &[Py]) -> Py {
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
    let _ = py_str("True when the conversation is on a dish made of eggs: an omelette or a quiche.");
    return _a_dish_in_flight(&[memory.clone(), g_EGG_DISH_WORDS()]);
    Py::None
}
pub fn _an_allergen(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut food: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut name: Py = Py::None;
    let _ = py_str("True when what is being left out is one of the things that cause reactions.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ALLERGENS()) { let mut name: Py = __item.clone(); __out.push(_same_word(&[food.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_crowd(args: &[Py]) -> Py {
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
    let mut have: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut chunk: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Six or more people, however the conversation says so.");
    let __iter1 = py_iter_or_raise(&op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent_texts(&[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() }));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CROWD_WORDS()) { let mut word: Py = __item.clone(); __out.push(_says(&[line.clone(), word.clone(), pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[line.clone()])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[line.clone()]));
    for __item in __iter2 {
    chunk = __item.clone();
    if py_truthy(&{ let __b = mcall(&chunk.clone(), "isdigit", &[]); if !py_truthy(&__b) { __b } else { { let __cmp31: Py = call_builtin("int", &[chunk.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(6), &__cmp31)) { op_le(&__cmp31, &Py::Int(200)) } else { Py::Bool(false) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut have: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let _ = py_str("The word itself, not its loose form: \"after\" is not \"afters\", \"sweetcorn\" is not \"sweet\".");
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(" ")))) {
    return Py::Bool(py_contains(&text.clone(), &word.clone()));
    }
    return Py::Bool(py_contains(&have.clone(), &word.clone()));
    Py::None
}
pub fn _pudding_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The pudding that fits the constraint in the line.");
    let __iter1 = py_iter_or_raise(&g_PUDDINGS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrases = __unpk2[0].clone();
    said = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&phrases.clone());
    for __item in __iter3 {
    phrase = __item.clone();
    if py_truthy(&{ let __b = _phrase_in(&[text.clone(), phrase.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); __out.push(_same_word(&[word.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return said.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _pudding(args: &[Py]) -> Py {
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
    let mut have: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What to have for pudding, asked as the second half of a dinner conversation.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DESSERT_WORDS()) { let mut word: Py = __item.clone(); __out.push(_says(&[text.clone(), word.clone(), have.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_REWARDED(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_ATE_ALREADY(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PUDDING_ASK(), "search", &[text.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&_diet(&[memory.clone(), understood.clone()])) {
    return Py::None;
    }
    said = _pudding_for(&[have.clone(), text.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    if py_truthy(&_a_crowd(&[memory.clone(), text.clone(), have.clone()])) {
    return g_CROWD_PUDDING();
    }
    return g_FRUIT_PUDDING();
    Py::None
}
pub fn _food_context(args: &[Py]) -> Py {
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
    let mut cooking: Py = Py::None;
    let mut foods: Py = Py::None;
    let mut have: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut verbs: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this line is part of a conversation about cooking and eating.\n\n    The words, not the topic reading: \"what about the water company, my name's not on\n    anything\" carries a drink in the word \"water\" and is still a question about a new flat,\n    while the line that carries a constraint has no food word of its own (\"i only have 20\n    minutes\") and leans on the food named a turn or two before.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    foods = op_add(&g_DISH_WORDS(), &call_builtin("tuple", &[call_builtin("getattr", &[cooking.clone(), py_str("_FOOD"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    verbs = py_set(vec![py_str("make"), py_str("made"), py_str("cook"), py_str("cooking"), py_str("fix"), py_str("fixing"), py_str("taste"), py_str("tastes"), py_str("tasted"), py_str("came"), py_str("out"), py_str("turned"), py_str("went"), py_str("wrong")]);
    let __iter1 = py_iter_or_raise(&op_add(&py_list(vec![text.clone()]), &slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(3), &Py::None)));
    for __item in __iter1 {
    line = __item.clone();
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[line.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = { let __b = Py::Bool(py_contains(&foods.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&verbs.clone(), &word.clone())) } }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MEAL_WORDS(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _idea_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The idea for the first food in the line, or None.");
    let __iter1 = py_iter_or_raise(&g_IDEAS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    key = __unpk2[0].clone();
    names = __unpk2[1].clone();
    words = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); __out.push(_same_word(&[word.clone(), name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![key.clone(), words.clone()]);
    }
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _already(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the bot's last reply was this idea already.");
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&reply.clone(), "lower", &[]), "split", &[]), &Py::None, &Py::Int(4), &Py::None)]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]), &Py::None, &Py::Int(4), &Py::None)]), &opening.clone()));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _named_dish(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The plan dish the line names as a dish rather than as an ingredient.\n\n    \"chicken\" on its own is how someone names a food they have (\"something with chicken\"),\n    and the idea list answers that. \"roast chicken\" or \"chicken thighs\" is a dish, and the\n    plan entries answer it.\n    ");
    key = dish_in(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&key.clone(), &Py::None))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&geti(&geti(&g_PLANS(), &key.clone()), &py_str("names")));
    for __item in __iter1 {
    group = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[group.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { group.clone() } else { py_tuple(vec![group.clone()]) });
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[mcall(&name.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { _phrase_in(&[text.clone(), name.clone()]) } }) {
    return key.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _negative_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut food: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut eat: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut subject_verb: Py = Py::None;
    let _ = py_str("The ways a line says somebody does not eat one food, built from the food's own name.\n\n    A guest's restriction is rarely said as a diet word: \"one of them doesnt eat dairy\" is the\n    same constraint as \"dairy free\", and it was answered with the calorie count of cheese while\n    \"my friend doesnt eat dairy\" was echoed back as \"Your friend doesn't eat dairy.\". Building\n    the frames once covers every food in the table and every way of saying no to it, which is the\n    part a list of phrases typed by hand always misses.\n\n    The line is read after the understanding has expanded a contraction (\"doesn't\" -> \"does not\",\n    \"can't\" -> \"can not\"), so each frame is written both ways and with the apostrophe split.\n    ");
    forms = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("doesnt"), py_str("doesn t"), py_str("does not"), py_str("didnt"), py_str("did not"), py_str("wont"), py_str("won t"), py_str("will not"), py_str("cant"), py_str("can t"), py_str("can not"), py_str("cannot"), py_str("never")]));
    for __item in __iter1 {
    subject_verb = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("eat"), py_str("eats"), py_str("have"), py_str("has")]));
    for __item in __iter2 {
    eat = __item.clone();
    let _ = mcall(&forms.clone(), "append", &[{ let mut __s = String::new(); __s.push_str(&fstr(&subject_verb.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&eat.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&food.clone(), -1, "")); py_string(__s) }]);
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_str("avoids {}"), py_str("avoid {}"), py_str("avoiding {}"), py_str("off {}"), py_str("is off {}"), py_str("no {} at all"), py_str("nothing with {}"), py_str("{} is out"), py_str("cut out {}"), py_str("cutting out {}"), py_str("free of {}"), py_str("free from {}")]));
    for __item in __iter3 {
    frame = __item.clone();
    let _ = mcall(&forms.clone(), "append", &[mcall(&frame.clone(), "format", &[food.clone()])]);
    }
    return call_builtin("tuple", &[mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[forms.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _food_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cooking: Py = Py::None;
    let mut verbs: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The foods both tables know, without the verbs `cooking._FOOD` also holds.\n\n    Built on each call, not at import: `cooking` imports this module, so a table read at\n    import time can be read while `cooking` is still half way through and come back with\n    none of its foods - and whether it does depends on which module the program imported\n    first, which is not something a reply should turn on.\n    ");
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    verbs = py_set(vec![py_str("make"), py_str("made"), py_str("cook"), py_str("cooking"), py_str("fix"), py_str("fixing"), py_str("taste"), py_str("tastes"), py_str("tasted"), py_str("came"), py_str("out"), py_str("turned"), py_str("went"), py_str("wrong")]);
    return call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_DISH_WORDS(), &call_builtin("tuple", &[call_builtin("getattr", &[cooking.clone(), py_str("_FOOD"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&verbs.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_a_food(args: &[Py]) -> Py {
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
    let mut foods: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names something you cook, as `_food_context` counts foods.");
    words = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    foods = _food_words(&[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&foods.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _earlier_turns(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let _ = py_str("The person's own lines before this one; on the first turn there are none.\n\n    `recent_texts()` is no use here: it folds in `memory[\"topic\"]`, which is this very line, so\n    on turn one it already looks like something was said before.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("user")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _a_food_word_with_no_cooking(args: &[Py]) -> Py {
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
    let _ = py_str("True when there is a food on the record but nothing is being cooked.\n\n    The rice of a portion question and the rice of a calorie count are not a dinner anyone is\n    making, so a diet said after one of them is not a change to a dish. With no earlier turn\n    there is nothing to misread: \"my daughter doesnt eat meat\" said on its own is the guest\n    constraint itself, and it still gets the dinner.\n    ");
    return { let __b = call_builtin("bool", &[_earlier_turns(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&cooking_under_way(&[memory.clone(), understood.clone()]))) } };
    Py::None
}
pub fn cooking_under_way(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the conversation, in the person's own words, is about cooking something.");
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter1 = py_iter_or_raise(&op_add(&py_list(vec![text.clone()]), &slice_or_raise(&recent_texts(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]), &Py::None, &Py::Int(4), &Py::None)));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    said = call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_COOKING_VERB(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_MAKE_VERB(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { _names_a_food(&[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _veggie(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("A vegetarian dinner, when the line is about someone in the party not eating meat.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&mcall(&g_DIET_RECALL(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_food_context(&[memory.clone(), understood.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GUEST_DIET(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::None;
    }
    if py_truthy(&_asks_about_the_diet(&[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    if py_truthy(&_a_food_word_with_no_cooking(&[memory.clone(), understood.clone()])) {
    return if py_truthy(&mcall(&g_VEGAN_WORD(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])) { g_VEGAN_SWAP_IDEA() } else { g_VEGGIE_SWAP_IDEA() };
    }
    if py_truthy(&mcall(&g_VEGAN_WORD(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return g_VEGAN_IDEA();
    }
    return g_VEGGIE_IDEA();
    Py::None
}
pub fn _asks_about_the_diet(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a yes/no question about the diet itself, not a request for a meal.\n\n    \"do vegans eat honey\" and \"is vegetarian food healthy\" ask about the diet; a dinner idea\n    answers neither. \"what can i cook for a vegan\" asks for the meal and is not this.\n    ");
    return { let __b = call_builtin("bool", &[mcall(&g_YES_NO_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_COOK_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))) } };
    Py::None
}
pub fn _diet(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What to swap for a diet the line says they are on, when the line is about cooking.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_DIET_RECALL(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DIET_NOT_THIS(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DIET_TOO_MEDICAL(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DIET_MEAL_ASK(), "search", &[text.clone()]) } } } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_DIETS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    words = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEGGIE_KEY()) { let mut word: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_DAIRY_FREE_VEGGIE();
    }
    if py_truthy(&_already(&[said.clone(), memory.clone()])) {
    return Py::None;
    }
    return said.clone();
    }
    return Py::None;
    Py::None
}
pub fn has_a_veggie_reply(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the meat-free dinner is the answer to this line.\n\n    The same gate as `has_a_diet_reply`, for the same reason: \"my daughter doesnt eat meat\" is a\n    constraint on the dinner, and the memory store says it back with the person's name instead.\n    ");
    return call_builtin("bool", &[_veggie(&[memory.clone(), understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn has_a_diet_reply(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the diet table has a reply for this line.\n\n    `bot/modules/answers.py` asks this before it hands a line to the memory store as a bare\n    taste: \"im vegetarian and i dont eat dairy\" is a taste the store would say back (\"good to\n    know\"), and it is also a constraint the kitchen can cook around, so the swaps are the\n    better answer. Only a line the table really answers gets past that gate.\n    ");
    return call_builtin("bool", &[_diet(&[memory.clone(), understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn has_a_dislike_reply(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the substitution table has a reply for the food this line rules out.\n\n    The same gate, for the same reason: \"i cant stand cucumber\" is a taste the store says back\n    (\"Fair enough - no cucumber in anything I suggest\"), and what the person wants to know is\n    what to put in its place. Only a food the table has a substitute for gets past the gate.\n    ");
    return call_builtin("bool", &[_dislike(&[memory.clone(), understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _recent_answers(args: &[Py]) -> Py {
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
    let _ = py_str("The last two things the bot said, as one string.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&turn.clone(), "get", &[py_str("bot")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&said.clone(), "append", &[geti(&turn.clone(), &py_str("bot"))]);
    }
    }
    return mcall(&py_str(" "), "join", &[said.clone()]);
    Py::None
}
pub fn _idea_food(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut have: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "have") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: have"))) };
    let mut _key: Py = Py::None;
    let mut _words: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The food word in this line the idea list knows, as it is written, or None.");
    let __iter1 = py_iter_or_raise(&g_IDEAS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _key = __unpk2[0].clone();
    names = __unpk2[1].clone();
    _words = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    let __iter4 = py_iter_or_raise(&have.clone());
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&_same_word(&[word.clone(), name.clone()])) {
    return word.clone();
    }
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _two_appliances(args: &[Py]) -> Py {
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
    let mut _text: Py = Py::None;
    let mut named: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let _ = py_str("True when the line names two appliances, so it is choosing between them.");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_APPLIANCE_IDEAS()) { let __unpk4030 = py_unpack(&__item.clone(), 2, None); let mut phrases: Py = __unpk4030[0].clone(); let mut _text: Py = __unpk4030[1].clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(geti(&phrases.clone(), &Py::Int(0))); } } py_set(__out) })() };
    if py_truthy(&mcall(&g_OVEN_WORD(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    let _ = mcall(&named.clone(), "add", &[py_str("oven")]);
    }
    return op_gt(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _appliance(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut reply_text: Py = Py::None;
    let _ = py_str("The appliance this line is about, as a dinner idea, or None.");
    if py_truthy(&mcall(&g__CLEANING_IT(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _two_appliances(&[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if !py_truthy(&__b) { __b } else { mcall(&g__PICK_ONE(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_APPLIANCE_IDEAS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrases = __unpk2[0].clone();
    reply_text = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply_text.clone();
    }
    }
    if py_truthy(&mcall(&g_OVEN_WORD(), "search", &[text.clone()])) {
    return g_TRAY_IDEA();
    }
    return Py::None;
    Py::None
}
pub fn _already_cooking(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when a line or two ago the person said what they are cooking tonight.");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(2), &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&mcall(&g_WHOLE_FOOD(), "search", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_an_appliance_and_a_cut(args: &[Py]) -> Py {
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
    let mut _text: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut word: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_APPLIANCE_IDEAS()) { let __unpk2095 = py_unpack(&__item.clone(), 2, None); let mut phrases: Py = __unpk2095[0].clone(); let mut _text: Py = __unpk2095[1].clone(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OVEN_WORD(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[op_bitand(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_set(__out) })() }, &call_builtin("set", &[g_CUT_WORDS()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _idea(args: &[Py]) -> Py {
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
    let mut cooking: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut fallback_key: Py = Py::None;
    let mut have: Py = Py::None;
    let mut key: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut named: Py = Py::None;
    let mut other: Py = Py::None;
    let mut other_key: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut reply_text: Py = Py::None;
    let mut said: Py = Py::None;
    let mut table: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One dish that fits the constraint this line gives, in a conversation about dinner.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&_names_an_appliance_and_a_cut(&[text.clone()])) {
    return Py::None;
    }
    have = pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&have.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_food_context(&[memory.clone(), understood.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_named_dish(&[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_already_cooking(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_SAFETY_WORDS(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("kids"), py_str("kid"), py_str("children"), py_str("child"), py_str("toddler"), py_str("toddlers"), py_str("family")])) { let mut mark: Py = __item.clone(); __out.push(_same_word(&[word.clone(), mark.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_KIDS_IDEA();
    }
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![gattr(&cooking.clone(), "dish"), gattr(&cooking.clone(), "meal"), gattr(&cooking.clone(), "without")]));
    for __item in __iter1 {
    table = __item.clone();
    said = if py_truthy(&Py::Bool(py_is(&table.clone(), &gattr(&cooking.clone(), "meal")))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __kws.extend(vec![("text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&table, args))(&__args) } } else { py_call(&table, &[understood.clone()]) };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return said.clone();
    }
    }
    named = _idea_food(&[have.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&named.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xcall("knowledge.lookup", "tokens", &[_recent_answers(&[memory.clone()])]), &named.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_OR_WORD(), "search", &[text.clone()])) {
    let __iter2 = py_iter_or_raise(&g_COMPARE_IDEAS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    phrases = __unpk3[0].clone();
    reply_text = __unpk3[1].clone();
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(_phrase_in(&[text.clone(), phrase.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply_text.clone();
    }
    }
    }
    named = _appliance(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&named.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g_PRONOUN(), "search", &[text.clone()]) } }) {
    let __iter4 = py_iter_or_raise(&slice_or_raise(&recent_texts(&[memory.clone()]), &Py::None, &Py::Int(3), &Py::None));
    for __item in __iter4 {
    earlier = __item.clone();
    if py_truthy(&mcall(&g_NO_APPLIANCE(), "search", &[earlier.clone()])) {
    continue;
    }
    named = _appliance(&[earlier.clone()]);
    if py_truthy(&Py::Bool(!py_is(&named.clone(), &Py::None))) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_is(&named.clone(), &Py::None))) {
    return named.clone();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("budget"), py_str("broke")])) { let mut mark: Py = __item.clone(); __out.push(_same_word(&[word.clone(), mark.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_CHEAP_IDEA();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("tomorrow"), py_str("tomorrows")])) { let mut mark: Py = __item.clone(); __out.push(_same_word(&[word.clone(), mark.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_AHEAD_IDEA();
    }
    let __unpk5 = py_unpack(&_idea_for(&[have.clone()]), 2, None);
    key = __unpk5[0].clone();
    reply_text = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&reply_text.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&op_add(&g_GUEST_WORDS(), &g_NICE_WORDS())) { let mut mark: Py = __item.clone(); __out.push(_same_word(&[word.clone(), mark.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return g_NICE_IDEA();
    }
    if py_truthy(&Py::Bool(py_is(&reply_text.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&have.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&g_PICK_WORDS()) { let mut mark: Py = __item.clone(); __out.push(_same_word(&[word.clone(), mark.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter6 = py_iter_or_raise(&g_IDEAS());
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 3, None);
    fallback_key = __unpk7[0].clone();
    _u = __unpk7[1].clone();
    fallback = __unpk7[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_already(&[fallback.clone(), memory.clone()])))) {
    return fallback.clone();
    }
    }
    return geti(&geti(&g_IDEAS(), &Py::Int(0)), &Py::Int(2));
    }
    if py_truthy(&_already(&[reply_text.clone(), memory.clone()])) {
    let __iter8 = py_iter_or_raise(&g_IDEAS());
    for __item in __iter8 {
    let __unpk9 = py_unpack(&__item.clone(), 3, None);
    other_key = __unpk9[0].clone();
    _u = __unpk9[1].clone();
    other = __unpk9[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&other_key.clone(), &key.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already(&[other.clone(), memory.clone()]))) } }) {
    return other.clone();
    }
    }
    }
    return reply_text.clone();
    Py::None
}
pub fn _dough_sticks(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    text = _line_text(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("banneton"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("basket"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("brotform"))) } } } }) {
    return op_add(&g_DOUGH_STICK_LEAD(), &py_str("Dust the banneton generously with rice flour, which does not absorb the way wheat flour does, or line it with a well-floured cloth, and chill the shaped loaf 20 minutes before it goes in."));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("pastry"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("shortcrust"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("pie"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("tart"))) } } } } } }) {
    return op_add(&g_DOUGH_STICK_LEAD(), &op_add(&py_str("Chill the pastry 20 to 30 minutes before rolling, keep the flour light and roll from the middle out with short strokes: warm pastry sticks however much flour you use."), &if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("pin")))) { py_str(" Dust the pin as well.") } else { py_str("") }));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("hands"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("fingers"))) } }) {
    return op_add(&g_DOUGH_STICK_LEAD(), &py_str("Wet your hands or rub them with a little oil, and lift the dough with a scraper instead of pulling at it."));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("pin"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("rolling"))) } }) {
    return op_add(&g_DOUGH_STICK_LEAD(), &py_str("Dust the pin lightly, lift the dough with a scraper rather than a hand, and give it a quarter turn after each roll."));
    }
    return op_add(&g_DOUGH_STICK_LEAD(), &py_str("Keep a light dusting of flour on the bench, lift the dough with a scraper instead of pulling at it, and add strong white flour a spoonful at a time if the dough itself is too wet to handle."));
    Py::None
}
pub fn _bake_fault_more(args: &[Py]) -> Py {
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
    let mut has: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A waffle iron, a tray of flapjacks and a batch of scones go wrong in their own ways.\n\n    These live next to the follow-up entries because `cooking.py` is at the world's 1 MiB file\n    cap, not because they are follow-ups: each one is a first question with its own food named.\n    The baking-fault table in `cooking.py` already knows the cheesecake, the meringue and the\n    brownie; this is the same shape of answer for three more bakes.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
pub fn __nested_has_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut wanted: Py = py_list(__pos.get(5..).unwrap_or(&[]).to_vec());
    let mut want: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&wanted.clone()) { let mut want: Py = __item.clone(); __out.push(_same_word(&[word.clone(), want.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    has = {{ let has = has.clone(); let memory = memory.clone(); let understood = understood.clone(); let word = word.clone(); let words = words.clone(); py_func("has", move |args: &[Py]| { let mut full = vec![has.clone(), memory.clone(), understood.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_has_0(&full) }) }};
    if py_truthy(&py_call(&has, &[py_str("waffle"), py_str("waffles")])) {
    return py_str("A soggy waffle is a batter that is too thin or an iron that is not hot enough: make it thicker than pancake batter - it should plop off the ladle rather than pour - let it rest 10 minutes so the flour takes up the liquid, and heat the iron until a drop of water skitters on it. Fill it three-quarters so the steam can escape, and stand the cooked ones on a rack in a low oven rather than stacking them, because a stack steams itself soft.");
    }
    if py_truthy(&py_call(&has, &[py_str("flapjack"), py_str("flapjacks"), py_str("oat bar"), py_str("oat bars")])) {
    if py_truthy(&py_call(&has, &[py_str("set"), py_str("firm"), py_str("firmed"), py_str("runny"), py_str("syrup"), py_str("sloppy"), py_str("sticky"), py_str("soggy"), py_str("soft")])) {
    return py_str("A flapjack that will not set wants less liquid or more oats: about 25g more oats and 2 tablespoons less syrup, pressed down hard into the tin with the back of a spoon before it bakes. Bake it at 180C (160C fan) for 25 minutes until the edges are golden, then leave it in the tin until it is completely cold - warm flapjacks are always soft and they firm up as they cool.");
    }
    return py_str("Flapjacks that crumble were cut warm or short of syrup: cool them completely in the tin, then cut with a sharp knife straight down, and add 2 tablespoons more golden syrup or honey to the next batch. Pack the mix down firmly before it bakes, because a loose tray bakes into crumbs.");
    }
    if py_truthy(&py_call(&has, &[py_str("brownie"), py_str("brownies")])) {
    if py_truthy(&py_call(&has, &[py_str("dry"), py_str("cakey"), py_str("cake-like"), py_str("crumbly"), py_str("overbaked"), py_str("overbaked"), py_str("too long"), py_str("hard"), py_str("stale")])) {
    return py_str("A dry brownie stayed in the oven too long or went into too big a tin: 20 to 25 minutes at 180C (160C fan) in a 20cm square tin, and take it out while the middle still looks slightly under, because it sets as it cools. Melt the butter and chocolate together gently rather than boiling them, and keep the flour light - it is there to hold the brownie together, not to make a cake.");
    }
    return Py::None;
    }
    if py_truthy(&py_call(&has, &[py_str("scone"), py_str("scones")])) {
    return py_str("Dense scones come from working the dough or a wet mix: use cold butter, rub it in until it looks like breadcrumbs, and bring it together with a knife and one fold rather than kneading. Pat it 2cm thick, cut straight down with a floured cutter and bake at 220C (200C fan) for 12 to 15 minutes, because a slow oven sets them heavy.");
    }
    return Py::None;
    Py::None
}
pub fn _two_dishes_plan(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The oven answer for the pair the line names: a cake and a roast first, the rest after.");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("cake"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("roast"))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__TWO_OVEN_BEEF()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return mcall(&py_str(" "), "join", &[py_tuple(vec![g__TWO_OVEN_CAKE(), g__TWO_OVEN_SPONGE()])]);
    }
    return mcall(&py_str(" "), "join", &[py_tuple(vec![g__TWO_OVEN_HEAD(), g__TWO_OVEN_GAP(), g__TWO_OVEN_CAKE(), g__TWO_OVEN_SPONGE()])]);
    Py::None
}
pub fn _opens_with_the_ask(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut head: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line starts with the definition ask, not with banter and then the topic.");
    words = mcall(&mcall(&said.clone(), "strip", &[py_str(" .!?,")]), "split", &[]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_DISH_WHAT_LEAD()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&words.clone(), &Py::None, &call_builtin("len", &[mcall(&lead.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &mcall(&lead.clone(), "split", &[])))) {
    words = slice_or_raise(&words.clone(), &call_builtin("len", &[mcall(&lead.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    break;
    }
    }
    head = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(3), &Py::None)]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_WHAT_ASKS()) { let mut ask: Py = __item.clone(); __out.push(mcall(&head.clone(), "startswith", &[ask.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _qualified_bare_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut row: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "row") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: row"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut _u: Py = Py::None;
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut head: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("True when the row's only names are bare family words with another word in front of them.");
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(1))) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_DISH_BARE_NAMES(), &name.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    noun = geti(&mcall(&name.clone(), "split", &[]), &op_neg(&Py::Int(1)));
    let __unpk2 = py_unpack(&mcall(&said.clone(), "partition", &[op_modulo(&py_str(" %s "), &noun.clone())]), 3, None);
    head = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    tail = __unpk2[2].clone();
    before = mcall(&head.clone(), "split", &[]);
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("and"), py_str("or"), py_str("this"), py_str("that"), py_str("my"), py_str("is"), py_str("are"), py_str("was"), py_str("were")]), &geti(&before.clone(), &op_neg(&Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    after = mcall(&tail.clone(), "split", &[]);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("for"), py_str("is"), py_str("are"), py_str("and"), py_str("or"), py_str("with"), py_str("s")]), &geti(&after.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _what_is_a_dish(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut top: Py = Py::None;
    let _ = py_str("The dish the line names, longest name first, or None when it names none of them.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]) } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&_opens_with_the_ask(&[said.clone()])))) {
    return Py::None;
    }
    said = op_modulo(&py_str(" %s "), &said.clone());
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_WHAT()) { let mut row: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(1))) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &name.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut row: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_qualified_bare_word(&[row.clone(), said.clone()])))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let found = found.clone(); let memory = memory.clone(); let name = name.clone(); let row = row.clone(); let said = said.clone(); let top = top.clone(); let understood = understood.clone(); py_func("lambda", move |args: &[Py]| { let found = found.clone(); let memory = memory.clone(); let name = name.clone(); let row = row.clone(); let said = said.clone(); let top = top.clone(); let understood = understood.clone(); let row: Py = __kwp(args, 0, "row").unwrap_or(Py::None); call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(1))) { let mut name: Py = __item.clone(); __out.push(call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) }) }), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    top = op_modulo(&py_str(" %s "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(1))); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    found = op_add(&py_list(vec![geti(&found.clone(), &Py::Int(0))]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&found.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut row: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(1))) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&top.clone(), &op_modulo(&py_str(" %s "), &name.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(row.clone()); } } py_list(__out) })() });
    if py_truthy(&op_gt(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return op_modulo(&py_str("%s Or the other way round: %s"), &py_tuple(vec![geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(2)), geti(&geti(&found.clone(), &Py::Int(1)), &Py::Int(2))]));
    }
    return geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(2));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.dishes".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AHEAD_IDEA" => Some(g_AHEAD_IDEA()),
            "ALLERGENS" => Some(g_ALLERGENS()),
            "ANSWERS" => Some(g_ANSWERS()),
            "APPLIANCE_IDEAS" => Some(g_APPLIANCE_IDEAS()),
            "ASK_FILLER" => Some(g_ASK_FILLER()),
            "ASK_KINDS" => Some(g_ASK_KINDS()),
            "ASK_OPENERS" => Some(g_ASK_OPENERS()),
            "ASK_PHRASES" => Some(g_ASK_PHRASES()),
            "ATE_ALREADY" => Some(g_ATE_ALREADY()),
            "A_CHILD" => Some(g_A_CHILD()),
            "BAKE_WORDS" => Some(g_BAKE_WORDS()),
            "BODY_TEMPERATURE" => Some(g_BODY_TEMPERATURE()),
            "CAKE_AVOID" => Some(g_CAKE_AVOID()),
            "CAKE_EGGS_AVOID" => Some(g_CAKE_EGGS_AVOID()),
            "CAKE_EGGS_KEY" => Some(g_CAKE_EGGS_KEY()),
            "CAKE_KEY" => Some(g_CAKE_KEY()),
            "CHEAP_IDEA" => Some(g_CHEAP_IDEA()),
            "COLOUR_CHOICE" => Some(g_COLOUR_CHOICE()),
            "COMPARE_IDEAS" => Some(g_COMPARE_IDEAS()),
            "COOKING_VERB" => Some(g_COOKING_VERB()),
            "COOKING_WORDS" => Some(g_COOKING_WORDS()),
            "COOK_ASK" => Some(g_COOK_ASK()),
            "COOK_ASK_WORDS" => Some(g_COOK_ASK_WORDS()),
            "CROWD_PUDDING" => Some(g_CROWD_PUDDING()),
            "CROWD_WORDS" => Some(g_CROWD_WORDS()),
            "CUPBOARD" => Some(g_CUPBOARD()),
            "CUT_OFF" => Some(g_CUT_OFF()),
            "CUT_WORDS" => Some(g_CUT_WORDS()),
            "DAIRY_FREE_VEGGIE" => Some(g_DAIRY_FREE_VEGGIE()),
            "DAY_WORDS" => Some(g_DAY_WORDS()),
            "DEFER_TO_ALLERGY" => Some(g_DEFER_TO_ALLERGY()),
            "DELIBERATING_START" => Some(g_DELIBERATING_START()),
            "DESSERT_WORDS" => Some(g_DESSERT_WORDS()),
            "DIETS" => Some(g_DIETS()),
            "DIET_FAMILIES" => Some(g_DIET_FAMILIES()),
            "DIET_MEAL_ASK" => Some(g_DIET_MEAL_ASK()),
            "DIET_NOT_THIS" => Some(g_DIET_NOT_THIS()),
            "DIET_RECALL" => Some(g_DIET_RECALL()),
            "DIET_TOO_MEDICAL" => Some(g_DIET_TOO_MEDICAL()),
            "DINNER_ASK" => Some(g_DINNER_ASK()),
            "DISH_BARE_NAMES" => Some(g_DISH_BARE_NAMES()),
            "DISH_LEAD" => Some(g_DISH_LEAD()),
            "DISH_WHAT" => Some(g_DISH_WHAT()),
            "DISH_WHAT_ASKS" => Some(g_DISH_WHAT_ASKS()),
            "DISH_WHAT_AVOID" => Some(g_DISH_WHAT_AVOID()),
            "DISH_WHAT_LEAD" => Some(g_DISH_WHAT_LEAD()),
            "DISH_WORDS" => Some(g_DISH_WORDS()),
            "DISLIKED" => Some(g_DISLIKED()),
            "DISLIKE_SHAPES" => Some(g_DISLIKE_SHAPES()),
            "DISLIKE_VERBS" => Some(g_DISLIKE_VERBS()),
            "DONE_ASK" => Some(g_DONE_ASK()),
            "DONE_POSSESSIVE" => Some(g_DONE_POSSESSIVE()),
            "DONE_VERB_ASK" => Some(g_DONE_VERB_ASK()),
            "DONE_WORD" => Some(g_DONE_WORD()),
            "DOUGH_STICK_LEAD" => Some(g_DOUGH_STICK_LEAD()),
            "EATING_REFUSAL" => Some(g_EATING_REFUSAL()),
            "EATING_REFUSALS" => Some(g_EATING_REFUSALS()),
            "EGG_DISH_WORDS" => Some(g_EGG_DISH_WORDS()),
            "EGG_FREE_BAKE" => Some(g_EGG_FREE_BAKE()),
            "EGG_IS_THE_DISH" => Some(g_EGG_IS_THE_DISH()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FOOD_TAIL_OK" => Some(g_FOOD_TAIL_OK()),
            "FOOD_WORDS" => Some(g_FOOD_WORDS()),
            "FRUIT_PUDDING" => Some(g_FRUIT_PUDDING()),
            "GENERIC_SWAP" => Some(g_GENERIC_SWAP()),
            "GOES_WITH" => Some(g_GOES_WITH()),
            "GOES_WITH_SHAPE" => Some(g_GOES_WITH_SHAPE()),
            "GUEST_DIET" => Some(g_GUEST_DIET()),
            "GUEST_WORDS" => Some(g_GUEST_WORDS()),
            "HAVE_WORDS" => Some(g_HAVE_WORDS()),
            "HOW_WORDS" => Some(g_HOW_WORDS()),
            "IDEAS" => Some(g_IDEAS()),
            "IDEA_CARRIERS" => Some(g_IDEA_CARRIERS()),
            "IDEA_SHAPES" => Some(g_IDEA_SHAPES()),
            "IMPLIED_ASKS" => Some(g_IMPLIED_ASKS()),
            "KEEPING_LEAD" => Some(g_KEEPING_LEAD()),
            "KEEP_ASK" => Some(g_KEEP_ASK()),
            "KEEP_ASK_WORDS" => Some(g_KEEP_ASK_WORDS()),
            "KEEP_QUALITY_WORDS" => Some(g_KEEP_QUALITY_WORDS()),
            "KIDS_IDEA" => Some(g_KIDS_IDEA()),
            "KITCHEN_SWAP" => Some(g_KITCHEN_SWAP()),
            "LIKING" => Some(g_LIKING()),
            "LUNCH_MEALS" => Some(g_LUNCH_MEALS()),
            "MAKE_VERB" => Some(g_MAKE_VERB()),
            "MAKING_A_FAULT" => Some(g_MAKING_A_FAULT()),
            "MEAL_WORDS" => Some(g_MEAL_WORDS()),
            "MIN_TYPO_LENGTH" => Some(g_MIN_TYPO_LENGTH()),
            "NICE_IDEA" => Some(g_NICE_IDEA()),
            "NICE_WORDS" => Some(g_NICE_WORDS()),
            "NOT_A_DISH" => Some(g_NOT_A_DISH()),
            "NOT_A_KEEPING_QUESTION" => Some(g_NOT_A_KEEPING_QUESTION()),
            "NOT_THE_FOOD" => Some(g_NOT_THE_FOOD()),
            "NO_APPLIANCE" => Some(g_NO_APPLIANCE()),
            "NUT_FREE_BAKE" => Some(g_NUT_FREE_BAKE()),
            "ORDER_AT_A_PLACE" => Some(g_ORDER_AT_A_PLACE()),
            "ORDER_AT_A_PLACE_ASK" => Some(g_ORDER_AT_A_PLACE_ASK()),
            "OR_WORD" => Some(g_OR_WORD()),
            "OTHER_PEOPLE" => Some(g_OTHER_PEOPLE()),
            "OUT_ASK" => Some(g_OUT_ASK()),
            "OUT_AVOID" => Some(g_OUT_AVOID()),
            "OUT_OF_CUE" => Some(g_OUT_OF_CUE()),
            "OUT_WORDS" => Some(g_OUT_WORDS()),
            "OVEN_WORD" => Some(g_OVEN_WORD()),
            "OWN_DISLIKE" => Some(g_OWN_DISLIKE()),
            "PAIRING_FOODS" => Some(g_PAIRING_FOODS()),
            "PAIRING_LEAN" => Some(g_PAIRING_LEAN()),
            "PAIRING_PLAIN" => Some(g_PAIRING_PLAIN()),
            "PAIRING_RICH" => Some(g_PAIRING_RICH()),
            "PART" => Some(g_PART()),
            "PART_BACKREF" => Some(g_PART_BACKREF()),
            "PART_NOT" => Some(g_PART_NOT()),
            "PART_OF" => Some(g_PART_OF()),
            "PART_WORDS" => Some(g_PART_WORDS()),
            "PERSON_DISLIKE" => Some(g_PERSON_DISLIKE()),
            "PERSON_REST" => Some(g_PERSON_REST()),
            "PERSON_START" => Some(g_PERSON_START()),
            "PERSON_STATE" => Some(g_PERSON_STATE()),
            "PICK_WORDS" => Some(g_PICK_WORDS()),
            "PLAIN" => Some(g_PLAIN()),
            "PLAIN_FOODS" => Some(g_PLAIN_FOODS()),
            "PLANS" => Some(g_PLANS()),
            "PLAN_NAMES" => Some(g_PLAN_NAMES()),
            "PLAN_NAMES_LONGEST" => Some(g_PLAN_NAMES_LONGEST()),
            "PROBLEMS" => Some(g_PROBLEMS()),
            "PRONOUN" => Some(g_PRONOUN()),
            "PUDDINGS" => Some(g_PUDDINGS()),
            "PUDDING_ASK" => Some(g_PUDDING_ASK()),
            "RAW_WORDS" => Some(g_RAW_WORDS()),
            "READY_NAMES" => Some(g_READY_NAMES()),
            "READY_NOTES" => Some(g_READY_NOTES()),
            "REWARDED" => Some(g_REWARDED()),
            "RICH_FOODS" => Some(g_RICH_FOODS()),
            "SAFETY_WORDS" => Some(g_SAFETY_WORDS()),
            "SAID_DISLIKE" => Some(g_SAID_DISLIKE()),
            "SAID_DISLIKE_OTHER" => Some(g_SAID_DISLIKE_OTHER()),
            "SHORTAGE_MARKERS" => Some(g_SHORTAGE_MARKERS()),
            "SIDES" => Some(g_SIDES()),
            "SIDE_METHOD" => Some(g_SIDE_METHOD()),
            "SIDE_WORDS" => Some(g_SIDE_WORDS()),
            "STAGE_PARTS" => Some(g_STAGE_PARTS()),
            "STARTED_PARTS" => Some(g_STARTED_PARTS()),
            "START_WORDS" => Some(g_START_WORDS()),
            "STEP_VERBS" => Some(g_STEP_VERBS()),
            "SUBSTITUTES" => Some(g_SUBSTITUTES()),
            "TIME_IN_ANSWER" => Some(g_TIME_IN_ANSWER()),
            "TIME_IN_A_STEP" => Some(g_TIME_IN_A_STEP()),
            "TRAY_IDEA" => Some(g_TRAY_IDEA()),
            "USE_UP_PHRASES" => Some(g_USE_UP_PHRASES()),
            "VEGAN_IDEA" => Some(g_VEGAN_IDEA()),
            "VEGAN_SWAP_IDEA" => Some(g_VEGAN_SWAP_IDEA()),
            "VEGAN_WORD" => Some(g_VEGAN_WORD()),
            "VEGGIE_CARRIERS" => Some(g_VEGGIE_CARRIERS()),
            "VEGGIE_IDEA" => Some(g_VEGGIE_IDEA()),
            "VEGGIE_KEY" => Some(g_VEGGIE_KEY()),
            "VEGGIE_SWAP_IDEA" => Some(g_VEGGIE_SWAP_IDEA()),
            "VEG_FAMILIES" => Some(g_VEG_FAMILIES()),
            "VEG_FAULTS" => Some(g_VEG_FAULTS()),
            "VEG_NAMES" => Some(g_VEG_NAMES()),
            "VEG_WORDS" => Some(g_VEG_WORDS()),
            "WASTE_WORDS" => Some(g_WASTE_WORDS()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "WHOLE_FOOD" => Some(g_WHOLE_FOOD()),
            "YES_NO_ASK" => Some(g_YES_NO_ASK()),
            "_ALLERGEN_NOTE" => Some(g__ALLERGEN_NOTE()),
            "_ANY_FOOD" => Some(g__ANY_FOOD()),
            "_ASKED_ABOUT_THE_FAULT" => Some(g__ASKED_ABOUT_THE_FAULT()),
            "_ASK_NOISE" => Some(g__ASK_NOISE()),
            "_A_MADE_DISH" => Some(g__A_MADE_DISH()),
            "_A_STATE_VERB" => Some(g__A_STATE_VERB()),
            "_BIRDS" => Some(g__BIRDS()),
            "_CHOICE_FRAME" => Some(g__CHOICE_FRAME()),
            "_CLEANING_IT" => Some(g__CLEANING_IT()),
            "_COOK_PHRASES" => Some(g__COOK_PHRASES()),
            "_DIET_FOOD" => Some(g__DIET_FOOD()),
            "_DRY_BAKE" => Some(g__DRY_BAKE()),
            "_GENERIC_SUBJECT" => Some(g__GENERIC_SUBJECT()),
            "_GLUE" => Some(g__GLUE()),
            "_GRAINY" => Some(g__GRAINY()),
            "_GRAINY_FIX" => Some(g__GRAINY_FIX()),
            "_IDEA_ASK" => Some(g__IDEA_ASK()),
            "_INGREDIENT_AFTER" => Some(g__INGREDIENT_AFTER()),
            "_KEEP_ASK" => Some(g__KEEP_ASK()),
            "_KEEP_BUT_COOKING" => Some(g__KEEP_BUT_COOKING()),
            "_LUMPY" => Some(g__LUMPY()),
            "_LUMPY_FIX" => Some(g__LUMPY_FIX()),
            "_MEALS" => Some(g__MEALS()),
            "_MUSHY_FIX" => Some(g__MUSHY_FIX()),
            "_MUSHY_WORDS" => Some(g__MUSHY_WORDS()),
            "_NAMES_A_SAUCE" => Some(g__NAMES_A_SAUCE()),
            "_PICK_ONE" => Some(g__PICK_ONE()),
            "_PLAN_MARKER" => Some(g__PLAN_MARKER()),
            "_PLAN_THAT_MIGHT_BE_RUINED" => Some(g__PLAN_THAT_MIGHT_BE_RUINED()),
            "_POT" => Some(g__POT()),
            "_RECIPE_CARDS" => Some(g__RECIPE_CARDS()),
            "_RED_MEAT" => Some(g__RED_MEAT()),
            "_RULE_OUT" => Some(g__RULE_OUT()),
            "_RULE_OUT_FILLER" => Some(g__RULE_OUT_FILLER()),
            "_SINK_AS_A_BAKE" => Some(g__SINK_AS_A_BAKE()),
            "_SINK_VERBS" => Some(g__SINK_VERBS()),
            "_STANDS_FOR_IT" => Some(g__STANDS_FOR_IT()),
            "_STARCH" => Some(g__STARCH()),
            "_TOUGH_BAKE" => Some(g__TOUGH_BAKE()),
            "_TWO_OVEN_BEEF" => Some(g__TWO_OVEN_BEEF()),
            "_TWO_OVEN_CAKE" => Some(g__TWO_OVEN_CAKE()),
            "_TWO_OVEN_GAP" => Some(g__TWO_OVEN_GAP()),
            "_TWO_OVEN_HEAD" => Some(g__TWO_OVEN_HEAD()),
            "_TWO_OVEN_SPONGE" => Some(g__TWO_OVEN_SPONGE()),
            "_a_child_who_will_not_eat" => Some(py_func("_a_child_who_will_not_eat", |args: &[Py]| _a_child_who_will_not_eat(args))),
            "_a_crowd" => Some(py_func("_a_crowd", |args: &[Py]| _a_crowd(args))),
            "_a_cut" => Some(py_func("_a_cut", |args: &[Py]| _a_cut(args))),
            "_a_diet_family" => Some(py_func("_a_diet_family", |args: &[Py]| _a_diet_family(args))),
            "_a_dish_in_flight" => Some(py_func("_a_dish_in_flight", |args: &[Py]| _a_dish_in_flight(args))),
            "_a_family_ruled_out" => Some(py_func("_a_family_ruled_out", |args: &[Py]| _a_family_ruled_out(args))),
            "_a_food_word_with_no_cooking" => Some(py_func("_a_food_word_with_no_cooking", |args: &[Py]| _a_food_word_with_no_cooking(args))),
            "_a_made_dish" => Some(py_func("_a_made_dish", |args: &[Py]| _a_made_dish(args))),
            "_a_shortage_the_swap_table_answers" => Some(py_func("_a_shortage_the_swap_table_answers", |args: &[Py]| _a_shortage_the_swap_table_answers(args))),
            "_a_sink_is_a_bake" => Some(py_func("_a_sink_is_a_bake", |args: &[Py]| _a_sink_is_a_bake(args))),
            "_about_a_person" => Some(py_func("_about_a_person", |args: &[Py]| _about_a_person(args))),
            "_already" => Some(py_func("_already", |args: &[Py]| _already(args))),
            "_already_cooking" => Some(py_func("_already_cooking", |args: &[Py]| _already_cooking(args))),
            "_an_allergen" => Some(py_func("_an_allergen", |args: &[Py]| _an_allergen(args))),
            "_an_egg_dish" => Some(py_func("_an_egg_dish", |args: &[Py]| _an_egg_dish(args))),
            "_another_meat" => Some(py_func("_another_meat", |args: &[Py]| _another_meat(args))),
            "_answering_line" => Some(py_func("_answering_line", |args: &[Py]| _answering_line(args))),
            "_any_food" => Some(py_func("_any_food", |args: &[Py]| _any_food(args))),
            "_appliance" => Some(py_func("_appliance", |args: &[Py]| _appliance(args))),
            "_asking_for_a_side" => Some(py_func("_asking_for_a_side", |args: &[Py]| _asking_for_a_side(args))),
            "_asking_to_pair" => Some(py_func("_asking_to_pair", |args: &[Py]| _asking_to_pair(args))),
            "_asks" => Some(py_func("_asks", |args: &[Py]| _asks(args))),
            "_asks_about_another_thing" => Some(py_func("_asks_about_another_thing", |args: &[Py]| _asks_about_another_thing(args))),
            "_asks_about_the_diet" => Some(py_func("_asks_about_the_diet", |args: &[Py]| _asks_about_the_diet(args))),
            "_asks_doneness" => Some(py_func("_asks_doneness", |args: &[Py]| _asks_doneness(args))),
            "_asks_for_a_dinner" => Some(py_func("_asks_for_a_dinner", |args: &[Py]| _asks_for_a_dinner(args))),
            "_bake_fault_more" => Some(py_func("_bake_fault_more", |args: &[Py]| _bake_fault_more(args))),
            "_bake_in_flight" => Some(py_func("_bake_in_flight", |args: &[Py]| _bake_in_flight(args))),
            "_cake_eggs" => Some(g__cake_eggs()),
            "_cake_plan" => Some(g__cake_plan()),
            "_diet" => Some(py_func("_diet", |args: &[Py]| _diet(args))),
            "_dish_in_flight" => Some(py_func("_dish_in_flight", |args: &[Py]| _dish_in_flight(args))),
            "_dish_named_in_the_line" => Some(py_func("_dish_named_in_the_line", |args: &[Py]| _dish_named_in_the_line(args))),
            "_dislike" => Some(py_func("_dislike", |args: &[Py]| _dislike(args))),
            "_disliked_food" => Some(py_func("_disliked_food", |args: &[Py]| _disliked_food(args))),
            "_dough_sticks" => Some(py_func("_dough_sticks", |args: &[Py]| _dough_sticks(args))),
            "_earlier_turns" => Some(py_func("_earlier_turns", |args: &[Py]| _earlier_turns(args))),
            "_engine_reads" => Some(g__engine_reads()),
            "_fault_groups" => Some(py_func("_fault_groups", |args: &[Py]| _fault_groups(args))),
            "_first_minutes" => Some(py_func("_first_minutes", |args: &[Py]| _first_minutes(args))),
            "_follow_up" => Some(py_func("_follow_up", |args: &[Py]| _follow_up(args))),
            "_food_context" => Some(py_func("_food_context", |args: &[Py]| _food_context(args))),
            "_food_words" => Some(py_func("_food_words", |args: &[Py]| _food_words(args))),
            "_foods_named" => Some(py_func("_foods_named", |args: &[Py]| _foods_named(args))),
            "_gluey" => Some(py_func("_gluey", |args: &[Py]| _gluey(args))),
            "_grainy" => Some(py_func("_grainy", |args: &[Py]| _grainy(args))),
            "_heavy" => Some(py_func("_heavy", |args: &[Py]| _heavy(args))),
            "_idea" => Some(py_func("_idea", |args: &[Py]| _idea(args))),
            "_idea_food" => Some(py_func("_idea_food", |args: &[Py]| _idea_food(args))),
            "_idea_for" => Some(py_func("_idea_for", |args: &[Py]| _idea_for(args))),
            "_is_a_keeping_line" => Some(py_func("_is_a_keeping_line", |args: &[Py]| _is_a_keeping_line(args))),
            "_last_answer_said" => Some(py_func("_last_answer_said", |args: &[Py]| _last_answer_said(args))),
            "_line_text" => Some(py_func("_line_text", |args: &[Py]| _line_text(args))),
            "_lumpy" => Some(py_func("_lumpy", |args: &[Py]| _lumpy(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_named_dish" => Some(py_func("_named_dish", |args: &[Py]| _named_dish(args))),
            "_names" => Some(g__names()),
            "_names_a_food" => Some(py_func("_names_a_food", |args: &[Py]| _names_a_food(args))),
            "_names_an_appliance_and_a_cut" => Some(py_func("_names_an_appliance_and_a_cut", |args: &[Py]| _names_an_appliance_and_a_cut(args))),
            "_names_any" => Some(py_func("_names_any", |args: &[Py]| _names_any(args))),
            "_names_food" => Some(py_func("_names_food", |args: &[Py]| _names_food(args))),
            "_negative_forms" => Some(py_func("_negative_forms", |args: &[Py]| _negative_forms(args))),
            "_one_edit" => Some(g__one_edit()),
            "_one_slip" => Some(py_func("_one_slip", |args: &[Py]| _one_slip(args))),
            "_opens_with_the_ask" => Some(py_func("_opens_with_the_ask", |args: &[Py]| _opens_with_the_ask(args))),
            "_part_ahead" => Some(py_func("_part_ahead", |args: &[Py]| _part_ahead(args))),
            "_parts_in" => Some(py_func("_parts_in", |args: &[Py]| _parts_in(args))),
            "_phrase_in" => Some(py_func("_phrase_in", |args: &[Py]| _phrase_in(args))),
            "_plan_contradicts_the_last_answer" => Some(py_func("_plan_contradicts_the_last_answer", |args: &[Py]| _plan_contradicts_the_last_answer(args))),
            "_problem" => Some(py_func("_problem", |args: &[Py]| _problem(args))),
            "_pudding" => Some(py_func("_pudding", |args: &[Py]| _pudding(args))),
            "_pudding_for" => Some(py_func("_pudding_for", |args: &[Py]| _pudding_for(args))),
            "_qualified_bare_word" => Some(py_func("_qualified_bare_word", |args: &[Py]| _qualified_bare_word(args))),
            "_raw_in_flight" => Some(py_func("_raw_in_flight", |args: &[Py]| _raw_in_flight(args))),
            "_ready_food" => Some(py_func("_ready_food", |args: &[Py]| _ready_food(args))),
            "_ready_note" => Some(py_func("_ready_note", |args: &[Py]| _ready_note(args))),
            "_recent_answers" => Some(py_func("_recent_answers", |args: &[Py]| _recent_answers(args))),
            "_ruled_out_near" => Some(py_func("_ruled_out_near", |args: &[Py]| _ruled_out_near(args))),
            "_ruled_out_spans" => Some(py_func("_ruled_out_spans", |args: &[Py]| _ruled_out_spans(args))),
            "_said_dislike" => Some(py_func("_said_dislike", |args: &[Py]| _said_dislike(args))),
            "_same_answer_but_a_new_part" => Some(py_func("_same_answer_but_a_new_part", |args: &[Py]| _same_answer_but_a_new_part(args))),
            "_same_word" => Some(py_func("_same_word", |args: &[Py]| _same_word(args))),
            "_says" => Some(py_func("_says", |args: &[Py]| _says(args))),
            "_sides" => Some(py_func("_sides", |args: &[Py]| _sides(args))),
            "_slip_of" => Some(py_func("_slip_of", |args: &[Py]| _slip_of(args))),
            "_slug" => Some(g__slug()),
            "_somebody_else_refuses" => Some(py_func("_somebody_else_refuses", |args: &[Py]| _somebody_else_refuses(args))),
            "_someone_elses_dislike" => Some(py_func("_someone_elses_dislike", |args: &[Py]| _someone_elses_dislike(args))),
            "_start" => Some(py_func("_start", |args: &[Py]| _start(args))),
            "_start_sentence" => Some(py_func("_start_sentence", |args: &[Py]| _start_sentence(args))),
            "_states_what_they_have" => Some(py_func("_states_what_they_have", |args: &[Py]| _states_what_they_have(args))),
            "_swap_said" => Some(py_func("_swap_said", |args: &[Py]| _swap_said(args))),
            "_temp_asks" => Some(py_func("_temp_asks", |args: &[Py]| _temp_asks(args))),
            "_the_dish_still" => Some(py_func("_the_dish_still", |args: &[Py]| _the_dish_still(args))),
            "_the_heat" => Some(py_func("_the_heat", |args: &[Py]| _the_heat(args))),
            "_the_state_is_about_the_food" => Some(py_func("_the_state_is_about_the_food", |args: &[Py]| _the_state_is_about_the_food(args))),
            "_the_subject_of_the_state" => Some(py_func("_the_subject_of_the_state", |args: &[Py]| _the_subject_of_the_state(args))),
            "_their_diet_refuses" => Some(py_func("_their_diet_refuses", |args: &[Py]| _their_diet_refuses(args))),
            "_two_appliances" => Some(py_func("_two_appliances", |args: &[Py]| _two_appliances(args))),
            "_two_dishes_plan" => Some(py_func("_two_dishes_plan", |args: &[Py]| _two_dishes_plan(args))),
            "_two_tastes" => Some(py_func("_two_tastes", |args: &[Py]| _two_tastes(args))),
            "_veg_fix" => Some(py_func("_veg_fix", |args: &[Py]| _veg_fix(args))),
            "_veggie" => Some(py_func("_veggie", |args: &[Py]| _veggie(args))),
            "_what" => Some(g__what()),
            "_what_is_a_dish" => Some(py_func("_what_is_a_dish", |args: &[Py]| _what_is_a_dish(args))),
            "a_colour_choice" => Some(py_func("a_colour_choice", |args: &[Py]| a_colour_choice(args))),
            "a_dish_is_already_the_plan" => Some(py_func("a_dish_is_already_the_plan", |args: &[Py]| a_dish_is_already_the_plan(args))),
            "a_plan_that_might_be_ruined" => Some(py_func("a_plan_that_might_be_ruined", |args: &[Py]| a_plan_that_might_be_ruined(args))),
            "announced_part" => Some(py_func("announced_part", |args: &[Py]| announced_part(args))),
            "ask_kind" => Some(py_func("ask_kind", |args: &[Py]| ask_kind(args))),
            "asks_for_a_side_of_a_food" => Some(py_func("asks_for_a_side_of_a_food", |args: &[Py]| asks_for_a_side_of_a_food(args))),
            "cook_ask" => Some(py_func("cook_ask", |args: &[Py]| cook_ask(args))),
            "cooking_under_way" => Some(py_func("cooking_under_way", |args: &[Py]| cooking_under_way(args))),
            "dish_for" => Some(py_func("dish_for", |args: &[Py]| dish_for(args))),
            "dish_in" => Some(py_func("dish_in", |args: &[Py]| dish_in(args))),
            "dishes_ruled_out" => Some(py_func("dishes_ruled_out", |args: &[Py]| dishes_ruled_out(args))),
            "entry" => Some(g_entry()),
            "follow_up" => Some(py_func("follow_up", |args: &[Py]| follow_up(args))),
            "general_sides" => Some(py_func("general_sides", |args: &[Py]| general_sides(args))),
            "has_a_diet_reply" => Some(py_func("has_a_diet_reply", |args: &[Py]| has_a_diet_reply(args))),
            "has_a_dislike_reply" => Some(py_func("has_a_dislike_reply", |args: &[Py]| has_a_dislike_reply(args))),
            "has_a_veggie_reply" => Some(py_func("has_a_veggie_reply", |args: &[Py]| has_a_veggie_reply(args))),
            "implied_dish" => Some(py_func("implied_dish", |args: &[Py]| implied_dish(args))),
            "not_a_mark_on_a_thing" => Some(g_not_a_mark_on_a_thing()),
            "not_a_present" => Some(g_not_a_present()),
            "not_ruled_out" => Some(py_func("not_ruled_out", |args: &[Py]| not_ruled_out(args))),
            "problem_in" => Some(py_func("problem_in", |args: &[Py]| problem_in(args))),
            "recent_texts" => Some(py_func("recent_texts", |args: &[Py]| recent_texts(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            "side_in" => Some(py_func("side_in", |args: &[Py]| side_in(args))),
            "tokens" => Some(g_tokens()),
            "reading" => Some(pyrt::xrefs::xmodule("understanding.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AHEAD_IDEA" => s_AHEAD_IDEA(v),
            "ALLERGENS" => s_ALLERGENS(v),
            "ANSWERS" => s_ANSWERS(v),
            "APPLIANCE_IDEAS" => s_APPLIANCE_IDEAS(v),
            "ASK_FILLER" => s_ASK_FILLER(v),
            "ASK_KINDS" => s_ASK_KINDS(v),
            "ASK_OPENERS" => s_ASK_OPENERS(v),
            "ASK_PHRASES" => s_ASK_PHRASES(v),
            "ATE_ALREADY" => s_ATE_ALREADY(v),
            "A_CHILD" => s_A_CHILD(v),
            "BAKE_WORDS" => s_BAKE_WORDS(v),
            "BODY_TEMPERATURE" => s_BODY_TEMPERATURE(v),
            "CAKE_AVOID" => s_CAKE_AVOID(v),
            "CAKE_EGGS_AVOID" => s_CAKE_EGGS_AVOID(v),
            "CAKE_EGGS_KEY" => s_CAKE_EGGS_KEY(v),
            "CAKE_KEY" => s_CAKE_KEY(v),
            "CHEAP_IDEA" => s_CHEAP_IDEA(v),
            "COLOUR_CHOICE" => s_COLOUR_CHOICE(v),
            "COMPARE_IDEAS" => s_COMPARE_IDEAS(v),
            "COOKING_VERB" => s_COOKING_VERB(v),
            "COOKING_WORDS" => s_COOKING_WORDS(v),
            "COOK_ASK" => s_COOK_ASK(v),
            "COOK_ASK_WORDS" => s_COOK_ASK_WORDS(v),
            "CROWD_PUDDING" => s_CROWD_PUDDING(v),
            "CROWD_WORDS" => s_CROWD_WORDS(v),
            "CUPBOARD" => s_CUPBOARD(v),
            "CUT_OFF" => s_CUT_OFF(v),
            "CUT_WORDS" => s_CUT_WORDS(v),
            "DAIRY_FREE_VEGGIE" => s_DAIRY_FREE_VEGGIE(v),
            "DAY_WORDS" => s_DAY_WORDS(v),
            "DEFER_TO_ALLERGY" => s_DEFER_TO_ALLERGY(v),
            "DELIBERATING_START" => s_DELIBERATING_START(v),
            "DESSERT_WORDS" => s_DESSERT_WORDS(v),
            "DIETS" => s_DIETS(v),
            "DIET_FAMILIES" => s_DIET_FAMILIES(v),
            "DIET_MEAL_ASK" => s_DIET_MEAL_ASK(v),
            "DIET_NOT_THIS" => s_DIET_NOT_THIS(v),
            "DIET_RECALL" => s_DIET_RECALL(v),
            "DIET_TOO_MEDICAL" => s_DIET_TOO_MEDICAL(v),
            "DINNER_ASK" => s_DINNER_ASK(v),
            "DISH_BARE_NAMES" => s_DISH_BARE_NAMES(v),
            "DISH_LEAD" => s_DISH_LEAD(v),
            "DISH_WHAT" => s_DISH_WHAT(v),
            "DISH_WHAT_ASKS" => s_DISH_WHAT_ASKS(v),
            "DISH_WHAT_AVOID" => s_DISH_WHAT_AVOID(v),
            "DISH_WHAT_LEAD" => s_DISH_WHAT_LEAD(v),
            "DISH_WORDS" => s_DISH_WORDS(v),
            "DISLIKED" => s_DISLIKED(v),
            "DISLIKE_SHAPES" => s_DISLIKE_SHAPES(v),
            "DISLIKE_VERBS" => s_DISLIKE_VERBS(v),
            "DONE_ASK" => s_DONE_ASK(v),
            "DONE_POSSESSIVE" => s_DONE_POSSESSIVE(v),
            "DONE_VERB_ASK" => s_DONE_VERB_ASK(v),
            "DONE_WORD" => s_DONE_WORD(v),
            "DOUGH_STICK_LEAD" => s_DOUGH_STICK_LEAD(v),
            "EATING_REFUSAL" => s_EATING_REFUSAL(v),
            "EATING_REFUSALS" => s_EATING_REFUSALS(v),
            "EGG_DISH_WORDS" => s_EGG_DISH_WORDS(v),
            "EGG_FREE_BAKE" => s_EGG_FREE_BAKE(v),
            "EGG_IS_THE_DISH" => s_EGG_IS_THE_DISH(v),
            "ENTRIES" => s_ENTRIES(v),
            "FOOD_TAIL_OK" => s_FOOD_TAIL_OK(v),
            "FOOD_WORDS" => s_FOOD_WORDS(v),
            "FRUIT_PUDDING" => s_FRUIT_PUDDING(v),
            "GENERIC_SWAP" => s_GENERIC_SWAP(v),
            "GOES_WITH" => s_GOES_WITH(v),
            "GOES_WITH_SHAPE" => s_GOES_WITH_SHAPE(v),
            "GUEST_DIET" => s_GUEST_DIET(v),
            "GUEST_WORDS" => s_GUEST_WORDS(v),
            "HAVE_WORDS" => s_HAVE_WORDS(v),
            "HOW_WORDS" => s_HOW_WORDS(v),
            "IDEAS" => s_IDEAS(v),
            "IDEA_CARRIERS" => s_IDEA_CARRIERS(v),
            "IDEA_SHAPES" => s_IDEA_SHAPES(v),
            "IMPLIED_ASKS" => s_IMPLIED_ASKS(v),
            "KEEPING_LEAD" => s_KEEPING_LEAD(v),
            "KEEP_ASK" => s_KEEP_ASK(v),
            "KEEP_ASK_WORDS" => s_KEEP_ASK_WORDS(v),
            "KEEP_QUALITY_WORDS" => s_KEEP_QUALITY_WORDS(v),
            "KIDS_IDEA" => s_KIDS_IDEA(v),
            "KITCHEN_SWAP" => s_KITCHEN_SWAP(v),
            "LIKING" => s_LIKING(v),
            "LUNCH_MEALS" => s_LUNCH_MEALS(v),
            "MAKE_VERB" => s_MAKE_VERB(v),
            "MAKING_A_FAULT" => s_MAKING_A_FAULT(v),
            "MEAL_WORDS" => s_MEAL_WORDS(v),
            "MIN_TYPO_LENGTH" => s_MIN_TYPO_LENGTH(v),
            "NICE_IDEA" => s_NICE_IDEA(v),
            "NICE_WORDS" => s_NICE_WORDS(v),
            "NOT_A_DISH" => s_NOT_A_DISH(v),
            "NOT_A_KEEPING_QUESTION" => s_NOT_A_KEEPING_QUESTION(v),
            "NOT_THE_FOOD" => s_NOT_THE_FOOD(v),
            "NO_APPLIANCE" => s_NO_APPLIANCE(v),
            "NUT_FREE_BAKE" => s_NUT_FREE_BAKE(v),
            "ORDER_AT_A_PLACE" => s_ORDER_AT_A_PLACE(v),
            "ORDER_AT_A_PLACE_ASK" => s_ORDER_AT_A_PLACE_ASK(v),
            "OR_WORD" => s_OR_WORD(v),
            "OTHER_PEOPLE" => s_OTHER_PEOPLE(v),
            "OUT_ASK" => s_OUT_ASK(v),
            "OUT_AVOID" => s_OUT_AVOID(v),
            "OUT_OF_CUE" => s_OUT_OF_CUE(v),
            "OUT_WORDS" => s_OUT_WORDS(v),
            "OVEN_WORD" => s_OVEN_WORD(v),
            "OWN_DISLIKE" => s_OWN_DISLIKE(v),
            "PAIRING_FOODS" => s_PAIRING_FOODS(v),
            "PAIRING_LEAN" => s_PAIRING_LEAN(v),
            "PAIRING_PLAIN" => s_PAIRING_PLAIN(v),
            "PAIRING_RICH" => s_PAIRING_RICH(v),
            "PART" => s_PART(v),
            "PART_BACKREF" => s_PART_BACKREF(v),
            "PART_NOT" => s_PART_NOT(v),
            "PART_OF" => s_PART_OF(v),
            "PART_WORDS" => s_PART_WORDS(v),
            "PERSON_DISLIKE" => s_PERSON_DISLIKE(v),
            "PERSON_REST" => s_PERSON_REST(v),
            "PERSON_START" => s_PERSON_START(v),
            "PERSON_STATE" => s_PERSON_STATE(v),
            "PICK_WORDS" => s_PICK_WORDS(v),
            "PLAIN" => s_PLAIN(v),
            "PLAIN_FOODS" => s_PLAIN_FOODS(v),
            "PLANS" => s_PLANS(v),
            "PLAN_NAMES" => s_PLAN_NAMES(v),
            "PLAN_NAMES_LONGEST" => s_PLAN_NAMES_LONGEST(v),
            "PROBLEMS" => s_PROBLEMS(v),
            "PRONOUN" => s_PRONOUN(v),
            "PUDDINGS" => s_PUDDINGS(v),
            "PUDDING_ASK" => s_PUDDING_ASK(v),
            "RAW_WORDS" => s_RAW_WORDS(v),
            "READY_NAMES" => s_READY_NAMES(v),
            "READY_NOTES" => s_READY_NOTES(v),
            "REWARDED" => s_REWARDED(v),
            "RICH_FOODS" => s_RICH_FOODS(v),
            "SAFETY_WORDS" => s_SAFETY_WORDS(v),
            "SAID_DISLIKE" => s_SAID_DISLIKE(v),
            "SAID_DISLIKE_OTHER" => s_SAID_DISLIKE_OTHER(v),
            "SHORTAGE_MARKERS" => s_SHORTAGE_MARKERS(v),
            "SIDES" => s_SIDES(v),
            "SIDE_METHOD" => s_SIDE_METHOD(v),
            "SIDE_WORDS" => s_SIDE_WORDS(v),
            "STAGE_PARTS" => s_STAGE_PARTS(v),
            "STARTED_PARTS" => s_STARTED_PARTS(v),
            "START_WORDS" => s_START_WORDS(v),
            "STEP_VERBS" => s_STEP_VERBS(v),
            "SUBSTITUTES" => s_SUBSTITUTES(v),
            "TIME_IN_ANSWER" => s_TIME_IN_ANSWER(v),
            "TIME_IN_A_STEP" => s_TIME_IN_A_STEP(v),
            "TRAY_IDEA" => s_TRAY_IDEA(v),
            "USE_UP_PHRASES" => s_USE_UP_PHRASES(v),
            "VEGAN_IDEA" => s_VEGAN_IDEA(v),
            "VEGAN_SWAP_IDEA" => s_VEGAN_SWAP_IDEA(v),
            "VEGAN_WORD" => s_VEGAN_WORD(v),
            "VEGGIE_CARRIERS" => s_VEGGIE_CARRIERS(v),
            "VEGGIE_IDEA" => s_VEGGIE_IDEA(v),
            "VEGGIE_KEY" => s_VEGGIE_KEY(v),
            "VEGGIE_SWAP_IDEA" => s_VEGGIE_SWAP_IDEA(v),
            "VEG_FAMILIES" => s_VEG_FAMILIES(v),
            "VEG_FAULTS" => s_VEG_FAULTS(v),
            "VEG_NAMES" => s_VEG_NAMES(v),
            "VEG_WORDS" => s_VEG_WORDS(v),
            "WASTE_WORDS" => s_WASTE_WORDS(v),
            "WHEN_WORDS" => s_WHEN_WORDS(v),
            "WHOLE_FOOD" => s_WHOLE_FOOD(v),
            "YES_NO_ASK" => s_YES_NO_ASK(v),
            "_ALLERGEN_NOTE" => s__ALLERGEN_NOTE(v),
            "_ANY_FOOD" => s__ANY_FOOD(v),
            "_ASKED_ABOUT_THE_FAULT" => s__ASKED_ABOUT_THE_FAULT(v),
            "_ASK_NOISE" => s__ASK_NOISE(v),
            "_A_MADE_DISH" => s__A_MADE_DISH(v),
            "_A_STATE_VERB" => s__A_STATE_VERB(v),
            "_BIRDS" => s__BIRDS(v),
            "_CHOICE_FRAME" => s__CHOICE_FRAME(v),
            "_CLEANING_IT" => s__CLEANING_IT(v),
            "_COOK_PHRASES" => s__COOK_PHRASES(v),
            "_DIET_FOOD" => s__DIET_FOOD(v),
            "_DRY_BAKE" => s__DRY_BAKE(v),
            "_GENERIC_SUBJECT" => s__GENERIC_SUBJECT(v),
            "_GLUE" => s__GLUE(v),
            "_GRAINY" => s__GRAINY(v),
            "_GRAINY_FIX" => s__GRAINY_FIX(v),
            "_IDEA_ASK" => s__IDEA_ASK(v),
            "_INGREDIENT_AFTER" => s__INGREDIENT_AFTER(v),
            "_KEEP_ASK" => s__KEEP_ASK(v),
            "_KEEP_BUT_COOKING" => s__KEEP_BUT_COOKING(v),
            "_LUMPY" => s__LUMPY(v),
            "_LUMPY_FIX" => s__LUMPY_FIX(v),
            "_MEALS" => s__MEALS(v),
            "_MUSHY_FIX" => s__MUSHY_FIX(v),
            "_MUSHY_WORDS" => s__MUSHY_WORDS(v),
            "_NAMES_A_SAUCE" => s__NAMES_A_SAUCE(v),
            "_PICK_ONE" => s__PICK_ONE(v),
            "_PLAN_MARKER" => s__PLAN_MARKER(v),
            "_PLAN_THAT_MIGHT_BE_RUINED" => s__PLAN_THAT_MIGHT_BE_RUINED(v),
            "_POT" => s__POT(v),
            "_RECIPE_CARDS" => s__RECIPE_CARDS(v),
            "_RED_MEAT" => s__RED_MEAT(v),
            "_RULE_OUT" => s__RULE_OUT(v),
            "_RULE_OUT_FILLER" => s__RULE_OUT_FILLER(v),
            "_SINK_AS_A_BAKE" => s__SINK_AS_A_BAKE(v),
            "_SINK_VERBS" => s__SINK_VERBS(v),
            "_STANDS_FOR_IT" => s__STANDS_FOR_IT(v),
            "_STARCH" => s__STARCH(v),
            "_TOUGH_BAKE" => s__TOUGH_BAKE(v),
            "_TWO_OVEN_BEEF" => s__TWO_OVEN_BEEF(v),
            "_TWO_OVEN_CAKE" => s__TWO_OVEN_CAKE(v),
            "_TWO_OVEN_GAP" => s__TWO_OVEN_GAP(v),
            "_TWO_OVEN_HEAD" => s__TWO_OVEN_HEAD(v),
            "_TWO_OVEN_SPONGE" => s__TWO_OVEN_SPONGE(v),
            "_cake_eggs" => s__cake_eggs(v),
            "_cake_plan" => s__cake_plan(v),
            "_engine_reads" => s__engine_reads(v),
            "_names" => s__names(v),
            "_one_edit" => s__one_edit(v),
            "_slug" => s__slug(v),
            "_what" => s__what(v),
            "entry" => s_entry(v),
            "not_a_mark_on_a_thing" => s_not_a_mark_on_a_thing(v),
            "not_a_present" => s_not_a_present(v),
            "tokens" => s_tokens(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.dishes", module_obj);
    pyrt::xrefs::reg_fn("knowledge.dishes", "_ready_food", py_func("_ready_food", |args: &[Py]| _ready_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_ready_note", py_func("_ready_note", |args: &[Py]| _ready_note(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_phrase_in", py_func("_phrase_in", |args: &[Py]| _phrase_in(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_one_slip", py_func("_one_slip", |args: &[Py]| _one_slip(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_cut", py_func("_a_cut", |args: &[Py]| _a_cut(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_same_word", py_func("_same_word", |args: &[Py]| _same_word(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_slip_of", py_func("_slip_of", |args: &[Py]| _slip_of(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_made_dish", py_func("_a_made_dish", |args: &[Py]| _a_made_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_ruled_out_near", py_func("_ruled_out_near", |args: &[Py]| _ruled_out_near(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "not_ruled_out", py_func("not_ruled_out", |args: &[Py]| not_ruled_out(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_ruled_out_spans", py_func("_ruled_out_spans", |args: &[Py]| _ruled_out_spans(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "dishes_ruled_out", py_func("dishes_ruled_out", |args: &[Py]| dishes_ruled_out(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "a_dish_is_already_the_plan", py_func("a_dish_is_already_the_plan", |args: &[Py]| a_dish_is_already_the_plan(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "dish_in", py_func("dish_in", |args: &[Py]| dish_in(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "implied_dish", py_func("implied_dish", |args: &[Py]| implied_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_line_text", py_func("_line_text", |args: &[Py]| _line_text(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "side_in", py_func("side_in", |args: &[Py]| side_in(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "recent_texts", py_func("recent_texts", |args: &[Py]| recent_texts(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "dish_for", py_func("dish_for", |args: &[Py]| dish_for(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_heavy", py_func("_heavy", |args: &[Py]| _heavy(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_gluey", py_func("_gluey", |args: &[Py]| _gluey(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_fault_groups", py_func("_fault_groups", |args: &[Py]| _fault_groups(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_grainy", py_func("_grainy", |args: &[Py]| _grainy(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_lumpy", py_func("_lumpy", |args: &[Py]| _lumpy(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_veg_fix", py_func("_veg_fix", |args: &[Py]| _veg_fix(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_sink_is_a_bake", py_func("_a_sink_is_a_bake", |args: &[Py]| _a_sink_is_a_bake(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "a_plan_that_might_be_ruined", py_func("a_plan_that_might_be_ruined", |args: &[Py]| a_plan_that_might_be_ruined(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "problem_in", py_func("problem_in", |args: &[Py]| problem_in(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_the_subject_of_the_state", py_func("_the_subject_of_the_state", |args: &[Py]| _the_subject_of_the_state(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_the_state_is_about_the_food", py_func("_the_state_is_about_the_food", |args: &[Py]| _the_state_is_about_the_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_names_food", py_func("_names_food", |args: &[Py]| _names_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "cook_ask", py_func("cook_ask", |args: &[Py]| cook_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_problem", py_func("_problem", |args: &[Py]| _problem(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_any_food", py_func("_any_food", |args: &[Py]| _any_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asks_doneness", py_func("_asks_doneness", |args: &[Py]| _asks_doneness(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "ask_kind", py_func("ask_kind", |args: &[Py]| ask_kind(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_first_minutes", py_func("_first_minutes", |args: &[Py]| _first_minutes(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_last_answer_said", py_func("_last_answer_said", |args: &[Py]| _last_answer_said(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_plan_contradicts_the_last_answer", py_func("_plan_contradicts_the_last_answer", |args: &[Py]| _plan_contradicts_the_last_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_parts_in", py_func("_parts_in", |args: &[Py]| _parts_in(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_answering_line", py_func("_answering_line", |args: &[Py]| _answering_line(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_same_answer_but_a_new_part", py_func("_same_answer_but_a_new_part", |args: &[Py]| _same_answer_but_a_new_part(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_dish_named_in_the_line", py_func("_dish_named_in_the_line", |args: &[Py]| _dish_named_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_part_ahead", py_func("_part_ahead", |args: &[Py]| _part_ahead(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_is_a_keeping_line", py_func("_is_a_keeping_line", |args: &[Py]| _is_a_keeping_line(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_about_a_person", py_func("_about_a_person", |args: &[Py]| _about_a_person(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_raw_in_flight", py_func("_raw_in_flight", |args: &[Py]| _raw_in_flight(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_follow_up", py_func("_follow_up", |args: &[Py]| _follow_up(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "follow_up", py_func("follow_up", |args: &[Py]| follow_up(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_temp_asks", py_func("_temp_asks", |args: &[Py]| _temp_asks(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_the_heat", py_func("_the_heat", |args: &[Py]| _the_heat(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asks", py_func("_asks", |args: &[Py]| _asks(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asking_for_a_side", py_func("_asking_for_a_side", |args: &[Py]| _asking_for_a_side(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_start", py_func("_start", |args: &[Py]| _start(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_their_diet_refuses", py_func("_their_diet_refuses", |args: &[Py]| _their_diet_refuses(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_start_sentence", py_func("_start_sentence", |args: &[Py]| _start_sentence(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asks_about_another_thing", py_func("_asks_about_another_thing", |args: &[Py]| _asks_about_another_thing(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "announced_part", py_func("announced_part", |args: &[Py]| announced_part(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asking_to_pair", py_func("_asking_to_pair", |args: &[Py]| _asking_to_pair(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_names_any", py_func("_names_any", |args: &[Py]| _names_any(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "asks_for_a_side_of_a_food", py_func("asks_for_a_side_of_a_food", |args: &[Py]| asks_for_a_side_of_a_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "general_sides", py_func("general_sides", |args: &[Py]| general_sides(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "a_colour_choice", py_func("a_colour_choice", |args: &[Py]| a_colour_choice(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_sides", py_func("_sides", |args: &[Py]| _sides(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_another_meat", py_func("_another_meat", |args: &[Py]| _another_meat(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_foods_named", py_func("_foods_named", |args: &[Py]| _foods_named(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_states_what_they_have", py_func("_states_what_they_have", |args: &[Py]| _states_what_they_have(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_dish_in_flight", py_func("_dish_in_flight", |args: &[Py]| _dish_in_flight(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_said_dislike", py_func("_said_dislike", |args: &[Py]| _said_dislike(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asks_for_a_dinner", py_func("_asks_for_a_dinner", |args: &[Py]| _asks_for_a_dinner(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_somebody_else_refuses", py_func("_somebody_else_refuses", |args: &[Py]| _somebody_else_refuses(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_two_tastes", py_func("_two_tastes", |args: &[Py]| _two_tastes(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_diet_family", py_func("_a_diet_family", |args: &[Py]| _a_diet_family(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_disliked_food", py_func("_disliked_food", |args: &[Py]| _disliked_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_someone_elses_dislike", py_func("_someone_elses_dislike", |args: &[Py]| _someone_elses_dislike(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_shortage_the_swap_table_answers", py_func("_a_shortage_the_swap_table_answers", |args: &[Py]| _a_shortage_the_swap_table_answers(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_child_who_will_not_eat", py_func("_a_child_who_will_not_eat", |args: &[Py]| _a_child_who_will_not_eat(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_swap_said", py_func("_swap_said", |args: &[Py]| _swap_said(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_family_ruled_out", py_func("_a_family_ruled_out", |args: &[Py]| _a_family_ruled_out(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_dislike", py_func("_dislike", |args: &[Py]| _dislike(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_the_dish_still", py_func("_the_dish_still", |args: &[Py]| _the_dish_still(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_dish_in_flight", py_func("_a_dish_in_flight", |args: &[Py]| _a_dish_in_flight(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_bake_in_flight", py_func("_bake_in_flight", |args: &[Py]| _bake_in_flight(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_an_egg_dish", py_func("_an_egg_dish", |args: &[Py]| _an_egg_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_an_allergen", py_func("_an_allergen", |args: &[Py]| _an_allergen(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_crowd", py_func("_a_crowd", |args: &[Py]| _a_crowd(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_says", py_func("_says", |args: &[Py]| _says(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_pudding_for", py_func("_pudding_for", |args: &[Py]| _pudding_for(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_pudding", py_func("_pudding", |args: &[Py]| _pudding(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_food_context", py_func("_food_context", |args: &[Py]| _food_context(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_idea_for", py_func("_idea_for", |args: &[Py]| _idea_for(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_already", py_func("_already", |args: &[Py]| _already(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_named_dish", py_func("_named_dish", |args: &[Py]| _named_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_negative_forms", py_func("_negative_forms", |args: &[Py]| _negative_forms(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_food_words", py_func("_food_words", |args: &[Py]| _food_words(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_names_a_food", py_func("_names_a_food", |args: &[Py]| _names_a_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_earlier_turns", py_func("_earlier_turns", |args: &[Py]| _earlier_turns(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_a_food_word_with_no_cooking", py_func("_a_food_word_with_no_cooking", |args: &[Py]| _a_food_word_with_no_cooking(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "cooking_under_way", py_func("cooking_under_way", |args: &[Py]| cooking_under_way(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_veggie", py_func("_veggie", |args: &[Py]| _veggie(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_asks_about_the_diet", py_func("_asks_about_the_diet", |args: &[Py]| _asks_about_the_diet(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_diet", py_func("_diet", |args: &[Py]| _diet(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "has_a_veggie_reply", py_func("has_a_veggie_reply", |args: &[Py]| has_a_veggie_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "has_a_diet_reply", py_func("has_a_diet_reply", |args: &[Py]| has_a_diet_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "has_a_dislike_reply", py_func("has_a_dislike_reply", |args: &[Py]| has_a_dislike_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_recent_answers", py_func("_recent_answers", |args: &[Py]| _recent_answers(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_idea_food", py_func("_idea_food", |args: &[Py]| _idea_food(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_two_appliances", py_func("_two_appliances", |args: &[Py]| _two_appliances(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_appliance", py_func("_appliance", |args: &[Py]| _appliance(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_already_cooking", py_func("_already_cooking", |args: &[Py]| _already_cooking(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_names_an_appliance_and_a_cut", py_func("_names_an_appliance_and_a_cut", |args: &[Py]| _names_an_appliance_and_a_cut(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_idea", py_func("_idea", |args: &[Py]| _idea(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_dough_sticks", py_func("_dough_sticks", |args: &[Py]| _dough_sticks(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_bake_fault_more", py_func("_bake_fault_more", |args: &[Py]| _bake_fault_more(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_two_dishes_plan", py_func("_two_dishes_plan", |args: &[Py]| _two_dishes_plan(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_opens_with_the_ask", py_func("_opens_with_the_ask", |args: &[Py]| _opens_with_the_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_qualified_bare_word", py_func("_qualified_bare_word", |args: &[Py]| _qualified_bare_word(args)));
    pyrt::xrefs::reg_fn("knowledge.dishes", "_what_is_a_dish", py_func("_what_is_a_dish", |args: &[Py]| _what_is_a_dish(args)));
    pyrt::xrefs::reg_access("knowledge.dishes", "AHEAD_IDEA", g_AHEAD_IDEA, s_AHEAD_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "ALLERGENS", g_ALLERGENS, s_ALLERGENS);
    pyrt::xrefs::reg_access("knowledge.dishes", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.dishes", "APPLIANCE_IDEAS", g_APPLIANCE_IDEAS, s_APPLIANCE_IDEAS);
    pyrt::xrefs::reg_access("knowledge.dishes", "ASK_FILLER", g_ASK_FILLER, s_ASK_FILLER);
    pyrt::xrefs::reg_access("knowledge.dishes", "ASK_KINDS", g_ASK_KINDS, s_ASK_KINDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "ASK_OPENERS", g_ASK_OPENERS, s_ASK_OPENERS);
    pyrt::xrefs::reg_access("knowledge.dishes", "ASK_PHRASES", g_ASK_PHRASES, s_ASK_PHRASES);
    pyrt::xrefs::reg_access("knowledge.dishes", "ATE_ALREADY", g_ATE_ALREADY, s_ATE_ALREADY);
    pyrt::xrefs::reg_access("knowledge.dishes", "A_CHILD", g_A_CHILD, s_A_CHILD);
    pyrt::xrefs::reg_access("knowledge.dishes", "BAKE_WORDS", g_BAKE_WORDS, s_BAKE_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "BODY_TEMPERATURE", g_BODY_TEMPERATURE, s_BODY_TEMPERATURE);
    pyrt::xrefs::reg_access("knowledge.dishes", "CAKE_AVOID", g_CAKE_AVOID, s_CAKE_AVOID);
    pyrt::xrefs::reg_access("knowledge.dishes", "CAKE_EGGS_AVOID", g_CAKE_EGGS_AVOID, s_CAKE_EGGS_AVOID);
    pyrt::xrefs::reg_access("knowledge.dishes", "CAKE_EGGS_KEY", g_CAKE_EGGS_KEY, s_CAKE_EGGS_KEY);
    pyrt::xrefs::reg_access("knowledge.dishes", "CAKE_KEY", g_CAKE_KEY, s_CAKE_KEY);
    pyrt::xrefs::reg_access("knowledge.dishes", "CHEAP_IDEA", g_CHEAP_IDEA, s_CHEAP_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "COLOUR_CHOICE", g_COLOUR_CHOICE, s_COLOUR_CHOICE);
    pyrt::xrefs::reg_access("knowledge.dishes", "COMPARE_IDEAS", g_COMPARE_IDEAS, s_COMPARE_IDEAS);
    pyrt::xrefs::reg_access("knowledge.dishes", "COOKING_VERB", g_COOKING_VERB, s_COOKING_VERB);
    pyrt::xrefs::reg_access("knowledge.dishes", "COOKING_WORDS", g_COOKING_WORDS, s_COOKING_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "COOK_ASK", g_COOK_ASK, s_COOK_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "COOK_ASK_WORDS", g_COOK_ASK_WORDS, s_COOK_ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "CROWD_PUDDING", g_CROWD_PUDDING, s_CROWD_PUDDING);
    pyrt::xrefs::reg_access("knowledge.dishes", "CROWD_WORDS", g_CROWD_WORDS, s_CROWD_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "CUPBOARD", g_CUPBOARD, s_CUPBOARD);
    pyrt::xrefs::reg_access("knowledge.dishes", "CUT_OFF", g_CUT_OFF, s_CUT_OFF);
    pyrt::xrefs::reg_access("knowledge.dishes", "CUT_WORDS", g_CUT_WORDS, s_CUT_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DAIRY_FREE_VEGGIE", g_DAIRY_FREE_VEGGIE, s_DAIRY_FREE_VEGGIE);
    pyrt::xrefs::reg_access("knowledge.dishes", "DAY_WORDS", g_DAY_WORDS, s_DAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DEFER_TO_ALLERGY", g_DEFER_TO_ALLERGY, s_DEFER_TO_ALLERGY);
    pyrt::xrefs::reg_access("knowledge.dishes", "DELIBERATING_START", g_DELIBERATING_START, s_DELIBERATING_START);
    pyrt::xrefs::reg_access("knowledge.dishes", "DESSERT_WORDS", g_DESSERT_WORDS, s_DESSERT_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DIETS", g_DIETS, s_DIETS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DIET_FAMILIES", g_DIET_FAMILIES, s_DIET_FAMILIES);
    pyrt::xrefs::reg_access("knowledge.dishes", "DIET_MEAL_ASK", g_DIET_MEAL_ASK, s_DIET_MEAL_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "DIET_NOT_THIS", g_DIET_NOT_THIS, s_DIET_NOT_THIS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DIET_RECALL", g_DIET_RECALL, s_DIET_RECALL);
    pyrt::xrefs::reg_access("knowledge.dishes", "DIET_TOO_MEDICAL", g_DIET_TOO_MEDICAL, s_DIET_TOO_MEDICAL);
    pyrt::xrefs::reg_access("knowledge.dishes", "DINNER_ASK", g_DINNER_ASK, s_DINNER_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_BARE_NAMES", g_DISH_BARE_NAMES, s_DISH_BARE_NAMES);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_LEAD", g_DISH_LEAD, s_DISH_LEAD);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_WHAT", g_DISH_WHAT, s_DISH_WHAT);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_WHAT_ASKS", g_DISH_WHAT_ASKS, s_DISH_WHAT_ASKS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_WHAT_AVOID", g_DISH_WHAT_AVOID, s_DISH_WHAT_AVOID);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_WHAT_LEAD", g_DISH_WHAT_LEAD, s_DISH_WHAT_LEAD);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISH_WORDS", g_DISH_WORDS, s_DISH_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISLIKED", g_DISLIKED, s_DISLIKED);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISLIKE_SHAPES", g_DISLIKE_SHAPES, s_DISLIKE_SHAPES);
    pyrt::xrefs::reg_access("knowledge.dishes", "DISLIKE_VERBS", g_DISLIKE_VERBS, s_DISLIKE_VERBS);
    pyrt::xrefs::reg_access("knowledge.dishes", "DONE_ASK", g_DONE_ASK, s_DONE_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "DONE_POSSESSIVE", g_DONE_POSSESSIVE, s_DONE_POSSESSIVE);
    pyrt::xrefs::reg_access("knowledge.dishes", "DONE_VERB_ASK", g_DONE_VERB_ASK, s_DONE_VERB_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "DONE_WORD", g_DONE_WORD, s_DONE_WORD);
    pyrt::xrefs::reg_access("knowledge.dishes", "DOUGH_STICK_LEAD", g_DOUGH_STICK_LEAD, s_DOUGH_STICK_LEAD);
    pyrt::xrefs::reg_access("knowledge.dishes", "EATING_REFUSAL", g_EATING_REFUSAL, s_EATING_REFUSAL);
    pyrt::xrefs::reg_access("knowledge.dishes", "EATING_REFUSALS", g_EATING_REFUSALS, s_EATING_REFUSALS);
    pyrt::xrefs::reg_access("knowledge.dishes", "EGG_DISH_WORDS", g_EGG_DISH_WORDS, s_EGG_DISH_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "EGG_FREE_BAKE", g_EGG_FREE_BAKE, s_EGG_FREE_BAKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "EGG_IS_THE_DISH", g_EGG_IS_THE_DISH, s_EGG_IS_THE_DISH);
    pyrt::xrefs::reg_access("knowledge.dishes", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.dishes", "FOOD_TAIL_OK", g_FOOD_TAIL_OK, s_FOOD_TAIL_OK);
    pyrt::xrefs::reg_access("knowledge.dishes", "FOOD_WORDS", g_FOOD_WORDS, s_FOOD_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "FRUIT_PUDDING", g_FRUIT_PUDDING, s_FRUIT_PUDDING);
    pyrt::xrefs::reg_access("knowledge.dishes", "GENERIC_SWAP", g_GENERIC_SWAP, s_GENERIC_SWAP);
    pyrt::xrefs::reg_access("knowledge.dishes", "GOES_WITH", g_GOES_WITH, s_GOES_WITH);
    pyrt::xrefs::reg_access("knowledge.dishes", "GOES_WITH_SHAPE", g_GOES_WITH_SHAPE, s_GOES_WITH_SHAPE);
    pyrt::xrefs::reg_access("knowledge.dishes", "GUEST_DIET", g_GUEST_DIET, s_GUEST_DIET);
    pyrt::xrefs::reg_access("knowledge.dishes", "GUEST_WORDS", g_GUEST_WORDS, s_GUEST_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "HAVE_WORDS", g_HAVE_WORDS, s_HAVE_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "HOW_WORDS", g_HOW_WORDS, s_HOW_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "IDEAS", g_IDEAS, s_IDEAS);
    pyrt::xrefs::reg_access("knowledge.dishes", "IDEA_CARRIERS", g_IDEA_CARRIERS, s_IDEA_CARRIERS);
    pyrt::xrefs::reg_access("knowledge.dishes", "IDEA_SHAPES", g_IDEA_SHAPES, s_IDEA_SHAPES);
    pyrt::xrefs::reg_access("knowledge.dishes", "IMPLIED_ASKS", g_IMPLIED_ASKS, s_IMPLIED_ASKS);
    pyrt::xrefs::reg_access("knowledge.dishes", "KEEPING_LEAD", g_KEEPING_LEAD, s_KEEPING_LEAD);
    pyrt::xrefs::reg_access("knowledge.dishes", "KEEP_ASK", g_KEEP_ASK, s_KEEP_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "KEEP_ASK_WORDS", g_KEEP_ASK_WORDS, s_KEEP_ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "KEEP_QUALITY_WORDS", g_KEEP_QUALITY_WORDS, s_KEEP_QUALITY_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "KIDS_IDEA", g_KIDS_IDEA, s_KIDS_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "KITCHEN_SWAP", g_KITCHEN_SWAP, s_KITCHEN_SWAP);
    pyrt::xrefs::reg_access("knowledge.dishes", "LIKING", g_LIKING, s_LIKING);
    pyrt::xrefs::reg_access("knowledge.dishes", "LUNCH_MEALS", g_LUNCH_MEALS, s_LUNCH_MEALS);
    pyrt::xrefs::reg_access("knowledge.dishes", "MAKE_VERB", g_MAKE_VERB, s_MAKE_VERB);
    pyrt::xrefs::reg_access("knowledge.dishes", "MAKING_A_FAULT", g_MAKING_A_FAULT, s_MAKING_A_FAULT);
    pyrt::xrefs::reg_access("knowledge.dishes", "MEAL_WORDS", g_MEAL_WORDS, s_MEAL_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "MIN_TYPO_LENGTH", g_MIN_TYPO_LENGTH, s_MIN_TYPO_LENGTH);
    pyrt::xrefs::reg_access("knowledge.dishes", "NICE_IDEA", g_NICE_IDEA, s_NICE_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "NICE_WORDS", g_NICE_WORDS, s_NICE_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "NOT_A_DISH", g_NOT_A_DISH, s_NOT_A_DISH);
    pyrt::xrefs::reg_access("knowledge.dishes", "NOT_A_KEEPING_QUESTION", g_NOT_A_KEEPING_QUESTION, s_NOT_A_KEEPING_QUESTION);
    pyrt::xrefs::reg_access("knowledge.dishes", "NOT_THE_FOOD", g_NOT_THE_FOOD, s_NOT_THE_FOOD);
    pyrt::xrefs::reg_access("knowledge.dishes", "NO_APPLIANCE", g_NO_APPLIANCE, s_NO_APPLIANCE);
    pyrt::xrefs::reg_access("knowledge.dishes", "NUT_FREE_BAKE", g_NUT_FREE_BAKE, s_NUT_FREE_BAKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "ORDER_AT_A_PLACE", g_ORDER_AT_A_PLACE, s_ORDER_AT_A_PLACE);
    pyrt::xrefs::reg_access("knowledge.dishes", "ORDER_AT_A_PLACE_ASK", g_ORDER_AT_A_PLACE_ASK, s_ORDER_AT_A_PLACE_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "OR_WORD", g_OR_WORD, s_OR_WORD);
    pyrt::xrefs::reg_access("knowledge.dishes", "OTHER_PEOPLE", g_OTHER_PEOPLE, s_OTHER_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.dishes", "OUT_ASK", g_OUT_ASK, s_OUT_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "OUT_AVOID", g_OUT_AVOID, s_OUT_AVOID);
    pyrt::xrefs::reg_access("knowledge.dishes", "OUT_OF_CUE", g_OUT_OF_CUE, s_OUT_OF_CUE);
    pyrt::xrefs::reg_access("knowledge.dishes", "OUT_WORDS", g_OUT_WORDS, s_OUT_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "OVEN_WORD", g_OVEN_WORD, s_OVEN_WORD);
    pyrt::xrefs::reg_access("knowledge.dishes", "OWN_DISLIKE", g_OWN_DISLIKE, s_OWN_DISLIKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "PAIRING_FOODS", g_PAIRING_FOODS, s_PAIRING_FOODS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PAIRING_LEAN", g_PAIRING_LEAN, s_PAIRING_LEAN);
    pyrt::xrefs::reg_access("knowledge.dishes", "PAIRING_PLAIN", g_PAIRING_PLAIN, s_PAIRING_PLAIN);
    pyrt::xrefs::reg_access("knowledge.dishes", "PAIRING_RICH", g_PAIRING_RICH, s_PAIRING_RICH);
    pyrt::xrefs::reg_access("knowledge.dishes", "PART", g_PART, s_PART);
    pyrt::xrefs::reg_access("knowledge.dishes", "PART_BACKREF", g_PART_BACKREF, s_PART_BACKREF);
    pyrt::xrefs::reg_access("knowledge.dishes", "PART_NOT", g_PART_NOT, s_PART_NOT);
    pyrt::xrefs::reg_access("knowledge.dishes", "PART_OF", g_PART_OF, s_PART_OF);
    pyrt::xrefs::reg_access("knowledge.dishes", "PART_WORDS", g_PART_WORDS, s_PART_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PERSON_DISLIKE", g_PERSON_DISLIKE, s_PERSON_DISLIKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "PERSON_REST", g_PERSON_REST, s_PERSON_REST);
    pyrt::xrefs::reg_access("knowledge.dishes", "PERSON_START", g_PERSON_START, s_PERSON_START);
    pyrt::xrefs::reg_access("knowledge.dishes", "PERSON_STATE", g_PERSON_STATE, s_PERSON_STATE);
    pyrt::xrefs::reg_access("knowledge.dishes", "PICK_WORDS", g_PICK_WORDS, s_PICK_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PLAIN", g_PLAIN, s_PLAIN);
    pyrt::xrefs::reg_access("knowledge.dishes", "PLAIN_FOODS", g_PLAIN_FOODS, s_PLAIN_FOODS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PLANS", g_PLANS, s_PLANS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PLAN_NAMES", g_PLAN_NAMES, s_PLAN_NAMES);
    pyrt::xrefs::reg_access("knowledge.dishes", "PLAN_NAMES_LONGEST", g_PLAN_NAMES_LONGEST, s_PLAN_NAMES_LONGEST);
    pyrt::xrefs::reg_access("knowledge.dishes", "PROBLEMS", g_PROBLEMS, s_PROBLEMS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PRONOUN", g_PRONOUN, s_PRONOUN);
    pyrt::xrefs::reg_access("knowledge.dishes", "PUDDINGS", g_PUDDINGS, s_PUDDINGS);
    pyrt::xrefs::reg_access("knowledge.dishes", "PUDDING_ASK", g_PUDDING_ASK, s_PUDDING_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "RAW_WORDS", g_RAW_WORDS, s_RAW_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "READY_NAMES", g_READY_NAMES, s_READY_NAMES);
    pyrt::xrefs::reg_access("knowledge.dishes", "READY_NOTES", g_READY_NOTES, s_READY_NOTES);
    pyrt::xrefs::reg_access("knowledge.dishes", "REWARDED", g_REWARDED, s_REWARDED);
    pyrt::xrefs::reg_access("knowledge.dishes", "RICH_FOODS", g_RICH_FOODS, s_RICH_FOODS);
    pyrt::xrefs::reg_access("knowledge.dishes", "SAFETY_WORDS", g_SAFETY_WORDS, s_SAFETY_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "SAID_DISLIKE", g_SAID_DISLIKE, s_SAID_DISLIKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "SAID_DISLIKE_OTHER", g_SAID_DISLIKE_OTHER, s_SAID_DISLIKE_OTHER);
    pyrt::xrefs::reg_access("knowledge.dishes", "SHORTAGE_MARKERS", g_SHORTAGE_MARKERS, s_SHORTAGE_MARKERS);
    pyrt::xrefs::reg_access("knowledge.dishes", "SIDES", g_SIDES, s_SIDES);
    pyrt::xrefs::reg_access("knowledge.dishes", "SIDE_METHOD", g_SIDE_METHOD, s_SIDE_METHOD);
    pyrt::xrefs::reg_access("knowledge.dishes", "SIDE_WORDS", g_SIDE_WORDS, s_SIDE_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "STAGE_PARTS", g_STAGE_PARTS, s_STAGE_PARTS);
    pyrt::xrefs::reg_access("knowledge.dishes", "STARTED_PARTS", g_STARTED_PARTS, s_STARTED_PARTS);
    pyrt::xrefs::reg_access("knowledge.dishes", "START_WORDS", g_START_WORDS, s_START_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "STEP_VERBS", g_STEP_VERBS, s_STEP_VERBS);
    pyrt::xrefs::reg_access("knowledge.dishes", "SUBSTITUTES", g_SUBSTITUTES, s_SUBSTITUTES);
    pyrt::xrefs::reg_access("knowledge.dishes", "TIME_IN_ANSWER", g_TIME_IN_ANSWER, s_TIME_IN_ANSWER);
    pyrt::xrefs::reg_access("knowledge.dishes", "TIME_IN_A_STEP", g_TIME_IN_A_STEP, s_TIME_IN_A_STEP);
    pyrt::xrefs::reg_access("knowledge.dishes", "TRAY_IDEA", g_TRAY_IDEA, s_TRAY_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "USE_UP_PHRASES", g_USE_UP_PHRASES, s_USE_UP_PHRASES);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGAN_IDEA", g_VEGAN_IDEA, s_VEGAN_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGAN_SWAP_IDEA", g_VEGAN_SWAP_IDEA, s_VEGAN_SWAP_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGAN_WORD", g_VEGAN_WORD, s_VEGAN_WORD);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGGIE_CARRIERS", g_VEGGIE_CARRIERS, s_VEGGIE_CARRIERS);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGGIE_IDEA", g_VEGGIE_IDEA, s_VEGGIE_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGGIE_KEY", g_VEGGIE_KEY, s_VEGGIE_KEY);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEGGIE_SWAP_IDEA", g_VEGGIE_SWAP_IDEA, s_VEGGIE_SWAP_IDEA);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEG_FAMILIES", g_VEG_FAMILIES, s_VEG_FAMILIES);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEG_FAULTS", g_VEG_FAULTS, s_VEG_FAULTS);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEG_NAMES", g_VEG_NAMES, s_VEG_NAMES);
    pyrt::xrefs::reg_access("knowledge.dishes", "VEG_WORDS", g_VEG_WORDS, s_VEG_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "WASTE_WORDS", g_WASTE_WORDS, s_WASTE_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "WHEN_WORDS", g_WHEN_WORDS, s_WHEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "WHOLE_FOOD", g_WHOLE_FOOD, s_WHOLE_FOOD);
    pyrt::xrefs::reg_access("knowledge.dishes", "YES_NO_ASK", g_YES_NO_ASK, s_YES_NO_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "_ALLERGEN_NOTE", g__ALLERGEN_NOTE, s__ALLERGEN_NOTE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_ANY_FOOD", g__ANY_FOOD, s__ANY_FOOD);
    pyrt::xrefs::reg_access("knowledge.dishes", "_ASKED_ABOUT_THE_FAULT", g__ASKED_ABOUT_THE_FAULT, s__ASKED_ABOUT_THE_FAULT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_ASK_NOISE", g__ASK_NOISE, s__ASK_NOISE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_A_MADE_DISH", g__A_MADE_DISH, s__A_MADE_DISH);
    pyrt::xrefs::reg_access("knowledge.dishes", "_A_STATE_VERB", g__A_STATE_VERB, s__A_STATE_VERB);
    pyrt::xrefs::reg_access("knowledge.dishes", "_BIRDS", g__BIRDS, s__BIRDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "_CHOICE_FRAME", g__CHOICE_FRAME, s__CHOICE_FRAME);
    pyrt::xrefs::reg_access("knowledge.dishes", "_CLEANING_IT", g__CLEANING_IT, s__CLEANING_IT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_COOK_PHRASES", g__COOK_PHRASES, s__COOK_PHRASES);
    pyrt::xrefs::reg_access("knowledge.dishes", "_DIET_FOOD", g__DIET_FOOD, s__DIET_FOOD);
    pyrt::xrefs::reg_access("knowledge.dishes", "_DRY_BAKE", g__DRY_BAKE, s__DRY_BAKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_GENERIC_SUBJECT", g__GENERIC_SUBJECT, s__GENERIC_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_GLUE", g__GLUE, s__GLUE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_GRAINY", g__GRAINY, s__GRAINY);
    pyrt::xrefs::reg_access("knowledge.dishes", "_GRAINY_FIX", g__GRAINY_FIX, s__GRAINY_FIX);
    pyrt::xrefs::reg_access("knowledge.dishes", "_IDEA_ASK", g__IDEA_ASK, s__IDEA_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "_INGREDIENT_AFTER", g__INGREDIENT_AFTER, s__INGREDIENT_AFTER);
    pyrt::xrefs::reg_access("knowledge.dishes", "_KEEP_ASK", g__KEEP_ASK, s__KEEP_ASK);
    pyrt::xrefs::reg_access("knowledge.dishes", "_KEEP_BUT_COOKING", g__KEEP_BUT_COOKING, s__KEEP_BUT_COOKING);
    pyrt::xrefs::reg_access("knowledge.dishes", "_LUMPY", g__LUMPY, s__LUMPY);
    pyrt::xrefs::reg_access("knowledge.dishes", "_LUMPY_FIX", g__LUMPY_FIX, s__LUMPY_FIX);
    pyrt::xrefs::reg_access("knowledge.dishes", "_MEALS", g__MEALS, s__MEALS);
    pyrt::xrefs::reg_access("knowledge.dishes", "_MUSHY_FIX", g__MUSHY_FIX, s__MUSHY_FIX);
    pyrt::xrefs::reg_access("knowledge.dishes", "_MUSHY_WORDS", g__MUSHY_WORDS, s__MUSHY_WORDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "_NAMES_A_SAUCE", g__NAMES_A_SAUCE, s__NAMES_A_SAUCE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_PICK_ONE", g__PICK_ONE, s__PICK_ONE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_PLAN_MARKER", g__PLAN_MARKER, s__PLAN_MARKER);
    pyrt::xrefs::reg_access("knowledge.dishes", "_PLAN_THAT_MIGHT_BE_RUINED", g__PLAN_THAT_MIGHT_BE_RUINED, s__PLAN_THAT_MIGHT_BE_RUINED);
    pyrt::xrefs::reg_access("knowledge.dishes", "_POT", g__POT, s__POT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_RECIPE_CARDS", g__RECIPE_CARDS, s__RECIPE_CARDS);
    pyrt::xrefs::reg_access("knowledge.dishes", "_RED_MEAT", g__RED_MEAT, s__RED_MEAT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_RULE_OUT", g__RULE_OUT, s__RULE_OUT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_RULE_OUT_FILLER", g__RULE_OUT_FILLER, s__RULE_OUT_FILLER);
    pyrt::xrefs::reg_access("knowledge.dishes", "_SINK_AS_A_BAKE", g__SINK_AS_A_BAKE, s__SINK_AS_A_BAKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_SINK_VERBS", g__SINK_VERBS, s__SINK_VERBS);
    pyrt::xrefs::reg_access("knowledge.dishes", "_STANDS_FOR_IT", g__STANDS_FOR_IT, s__STANDS_FOR_IT);
    pyrt::xrefs::reg_access("knowledge.dishes", "_STARCH", g__STARCH, s__STARCH);
    pyrt::xrefs::reg_access("knowledge.dishes", "_TOUGH_BAKE", g__TOUGH_BAKE, s__TOUGH_BAKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_TWO_OVEN_BEEF", g__TWO_OVEN_BEEF, s__TWO_OVEN_BEEF);
    pyrt::xrefs::reg_access("knowledge.dishes", "_TWO_OVEN_CAKE", g__TWO_OVEN_CAKE, s__TWO_OVEN_CAKE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_TWO_OVEN_GAP", g__TWO_OVEN_GAP, s__TWO_OVEN_GAP);
    pyrt::xrefs::reg_access("knowledge.dishes", "_TWO_OVEN_HEAD", g__TWO_OVEN_HEAD, s__TWO_OVEN_HEAD);
    pyrt::xrefs::reg_access("knowledge.dishes", "_TWO_OVEN_SPONGE", g__TWO_OVEN_SPONGE, s__TWO_OVEN_SPONGE);
    pyrt::xrefs::reg_access("knowledge.dishes", "_cake_eggs", g__cake_eggs, s__cake_eggs);
    pyrt::xrefs::reg_access("knowledge.dishes", "_cake_plan", g__cake_plan, s__cake_plan);
    pyrt::xrefs::reg_access("knowledge.dishes", "_engine_reads", g__engine_reads, s__engine_reads);
    pyrt::xrefs::reg_access("knowledge.dishes", "_names", g__names, s__names);
    pyrt::xrefs::reg_access("knowledge.dishes", "_one_edit", g__one_edit, s__one_edit);
    pyrt::xrefs::reg_access("knowledge.dishes", "_slug", g__slug, s__slug);
    pyrt::xrefs::reg_access("knowledge.dishes", "_what", g__what, s__what);
    pyrt::xrefs::reg_access("knowledge.dishes", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.dishes", "not_a_mark_on_a_thing", g_not_a_mark_on_a_thing, s_not_a_mark_on_a_thing);
    pyrt::xrefs::reg_access("knowledge.dishes", "not_a_present", g_not_a_present, s_not_a_present);
    pyrt::xrefs::reg_access("knowledge.dishes", "tokens", g_tokens, s_tokens);
}
