use crate::{
    domains::{
        icvc_configuration::types::{
            CheckBoxConfig, CheckBoxConfigCreate, DecimalValueConfig, DecimalValueConfigCreate,
            QuestionConfig, QuestionConfigCreate, StepConfigCreate, StepConfigCreateDefault,
            StepPhaseConfigCreate,
        },
        project::types::DocumentType,
    },
    repository::{self, generate_category_id},
    APIError, AssessmentMethod, ICVCConfigUpdate, StepId, StepPhaseId,
};

use super::{
    constants::MAX_CATEGORY_BYTES,
    types::{Category, CategoryCreate},
    types_storage::ICVCConfig,
};

#[allow(unused)]
const QUESTION_ID_PREFIX: &str = "ICVC_QUESTION";
#[allow(unused)]
const CHECKBOX_ID_PREFIX: &str = "ICVC_CHECKBOX";
#[allow(unused)]
const DECIMAL_VALUE_ID_PREFIX: &str = "ICVC_NUMERIC_DECIMAL";

pub fn get_open_duration() -> u64 {
    repository::get_icvc_config().open_duration
}

pub fn get_assessment_duration() -> u64 {
    repository::get_icvc_config().assessment_duration
}

pub fn get_icvc_config() -> ICVCConfig {
    repository::get_icvc_config()
}

pub fn update_icvc_config(update_icvc_config: ICVCConfigUpdate) -> Result<ICVCConfig, APIError> {
    match repository::update_icvc_config(update_icvc_config) {
        Ok(config) => Ok(config),
        Err(_) => Err(APIError::InternalServerError(
            "Unable to update config".to_string(),
        )),
    }
}

pub fn init_default_step_phases_config() {
    let default_step_phases_config = vec![
        //Step phase 0 - Application Phase
        StepPhaseConfigCreate {
            assessment_method: AssessmentMethod::Vote,
            steps: vec![
                //Step 0
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![DecimalValueConfigCreate { default_value: 0.0 }],
                    required_upload_files: vec![
                        DocumentType::PitchDeck,
                        DocumentType::Logo,
                        DocumentType::CoverPhoto,
                    ],
                },
            ],
        },
        //Step phase 1 - Evaluation Phase
        StepPhaseConfigCreate {
            assessment_method: AssessmentMethod::Grade,
            steps: vec![
                //step 0 - Business Model
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 1 - Team Evaluation
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 2 - Market analysis
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 3 - Financial Analysis
                StepConfigCreateDefault {
                    questions: vec![],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![DocumentType::FinancialModels],
                },
                //step 4 - Legal, Regulatory and Compliance
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 5 - Market Strategy and Sales
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 6 - ICP Effect
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 7 - Exit Strategies
                StepConfigCreateDefault {
                    questions: vec![QuestionConfigCreate {
                        max_num_bytes: 2000,
                    }],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 8 - Risks and Challenges
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 9 - Customer and Product validation
                StepConfigCreateDefault {
                    questions: vec![QuestionConfigCreate {
                        max_num_bytes: 2000,
                    }],
                    checkboxes: vec![
                        CheckBoxConfigCreate {
                            default_value: false,
                        },
                        CheckBoxConfigCreate {
                            default_value: false,
                        },
                    ],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                //step 10 - Technology Assessment
                StepConfigCreateDefault {
                    questions: vec![],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![DocumentType::ProductDemo],
                },
                //step 11 - 2-Year Roadmap and Expenditure
                StepConfigCreateDefault {
                    questions: vec![
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                        QuestionConfigCreate {
                            max_num_bytes: 2000,
                        },
                    ],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![DocumentType::ExpenditurePlan],
                },
                //step 12 - Money raised and valuation
                StepConfigCreateDefault {
                    questions: vec![],
                    checkboxes: vec![],
                    decimal_values: vec![
                        DecimalValueConfigCreate { default_value: 0.0 },
                        DecimalValueConfigCreate { default_value: 0.0 },
                    ],
                    required_upload_files: vec![],
                },
            ],
        },
        //Step phase 2 - Completion Phase
        StepPhaseConfigCreate {
            assessment_method: AssessmentMethod::Vote,
            steps: vec![
                StepConfigCreateDefault {
                    questions: vec![],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
                StepConfigCreateDefault {
                    questions: vec![],
                    checkboxes: vec![],
                    decimal_values: vec![],
                    required_upload_files: vec![],
                },
            ],
        },
    ];

    for (step_phase_index, step_phase_config) in default_step_phases_config.iter().enumerate() {
        let step_phase_id = step_phase_index as StepPhaseId;
        repository::insert_step_phase_config(step_phase_id, step_phase_config.clone());

        for (step_index, step_config) in step_phase_config.steps.iter().enumerate() {
            let step_id = step_index as StepId;

            let questions: Vec<QuestionConfig>;
            let checkboxes: Vec<CheckBoxConfig>;
            let decimal_values: Vec<DecimalValueConfig>;

            questions = step_config
                .questions
                .iter()
                .enumerate()
                .map(|(index, question_config)| QuestionConfig {
                    id: format!(
                        "{}_{}_{}_{}",
                        QUESTION_ID_PREFIX, step_phase_index, step_index, index
                    ),
                    max_num_bytes: question_config.max_num_bytes,
                })
                .collect();

            checkboxes = step_config
                .checkboxes
                .iter()
                .enumerate()
                .map(|(index, checkbox_config)| CheckBoxConfig {
                    id: format!(
                        "{}_{}_{}_{}",
                        CHECKBOX_ID_PREFIX, step_phase_index, step_index, index
                    ),
                    default_value: checkbox_config.default_value,
                })
                .collect();

            decimal_values = step_config
                .decimal_values
                .iter()
                .enumerate()
                .map(|(index, decimal_value_config)| DecimalValueConfig {
                    id: format!(
                        "{}_{}_{}_{}",
                        DECIMAL_VALUE_ID_PREFIX, step_phase_index, step_index, index
                    ),
                    default_value: decimal_value_config.default_value,
                })
                .collect();

            let step_config = StepConfigCreate {
                questions: questions,
                checkboxes: checkboxes,
                decimal_values: decimal_values,
                required_upload_files: step_config.required_upload_files.clone(),
            };

            repository::insert_step_config(step_phase_id, step_id, step_config.clone());
        }
    }

    ic_cdk::println!(
        "{} step phases config initialize successfully",
        default_step_phases_config.len()
    );
}

//Categories
pub fn create_category(category_create: CategoryCreate) -> Result<Category, APIError> {
    check_category_length(&category_create.name)?;

    let category_id: u64 = generate_category_id();

    match repository::insert_category(category_id, category_create) {
        Some(category) => Ok(category),
        None => Err(APIError::BadRequest(
            "Failed to save the category. Category ID already exists.".to_string(),
        )),
    }
}

pub fn get_category_by_id(category_id: u64) -> Result<Category, APIError> {
    match repository::get_category_by_id(category_id) {
        Some(category) => Ok(category),
        None => Err(APIError::NotFound(format!(
            "Category with id {} not found.",
            category_id
        ))),
    }
}

pub fn get_all_categories() -> Result<Vec<Category>, APIError> {
    let categories = repository::get_all_categories();

    Ok(categories)
}

pub fn desactivate_category_by_id(category_id: u64) -> Result<Category, APIError> {
    match repository::desactivate_category_by_id(category_id) {
        Some(category) => Ok(category),
        None => Err(APIError::NotFound(format!(
            "Category with id {} not found.",
            category_id
        ))),
    }
}

pub fn ini_default_categories() {
    let default_categories = vec![
        CategoryCreate {
            name: "Defi".to_string(),
        },
        CategoryCreate {
            name: "Dex".to_string(),
        },
        CategoryCreate {
            name: "Gaming".to_string(),
        },
    ];

    for category in default_categories.iter() {
        match create_category(category.clone()) {
            Ok(_) => (),
            Err(e) => {
                panic!("Unable to create default categories! Error: {}", e);
            }
        }
    }
}

fn check_category_length(category_name: &String) -> Result<(), APIError> {
    let category_len = category_name.len();

    if category_len > MAX_CATEGORY_BYTES {
        return Err(APIError::BadRequest(format!(
            "Category name length: {}. Max allowed length: {}.",
            category_len, MAX_CATEGORY_BYTES
        )));
    }

    Ok(())
}
