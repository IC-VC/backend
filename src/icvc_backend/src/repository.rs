use crate::domains::canister_management::types::CanisterConfigUpdate;
use crate::domains::canister_management::types_storage::CanisterConfig;
use crate::domains::core::types_storage::CompositeKey;
use crate::domains::icvc_configuration::types::{
    Category, CategoryCreate, StepConfig, StepConfigCreate, StepPhaseConfig, StepPhaseConfigCreate,
};
use crate::domains::icvc_configuration::types_storage::{
    CategoryModel, ICVCConfig, StepConfigModel, StepPhaseConfigModel,
};
use crate::domains::project::types::{Project, ProjectCreate, ProjectStatus, ProjectUpdate};
use crate::domains::project::types_storage::ProjectModel;
use crate::domains::step::types_storage::{
    StepModel, StepPhaseGradeResultModel, StepPhaseModel, StepPhaseVoteResultModel,
};
use crate::domains::user::types::{User, UserCreate, UserId, UserUpdate};

use crate::domains::user::types_storage::UserModel;
use crate::{
    ICVCConfigUpdate, ProjectId, Step, StepCreate, StepGrade, StepId, StepPhase, StepPhaseCreate,
    StepPhaseGradeResult, StepPhaseGradeResultCreate, StepPhaseId, StepPhaseProposal,
    StepPhaseStatus, StepPhaseUpdate, StepPhaseVoteResult, StepPhaseVoteResultCreate, StepUpdate,
};

use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl, Memory, StableBTreeMap};
use std::borrow::BorrowMut;
use std::cell::RefCell;

const CANISTER_CONFIG_MEM_ID: MemoryId = MemoryId::new(0);
const ICVC_CONFIG_MEM_ID: MemoryId = MemoryId::new(1);
const PROJECT_ID_COUNTER_MEM_ID: MemoryId = MemoryId::new(2);
const USER_MAP_MEM_ID: MemoryId = MemoryId::new(3);
const USER_PROJECT_MAP_MEM_ID: MemoryId = MemoryId::new(4);
const PROJECT_STEP_PHASE_MAP_ID: MemoryId = MemoryId::new(5);
const PROJECT_STEP_MAP_MEM_ID: MemoryId = MemoryId::new(6);
const STEP_GRADE_MAP_MEM_ID: MemoryId = MemoryId::new(7);
const STEPPHASE_STEPCONFIG_MAP_MEM_ID: MemoryId = MemoryId::new(8);
const STEP_STEPCONFIG_MAP_MEM_ID: MemoryId = MemoryId::new(9);
const PHASE_PROPOSAL_MAP_MEM_ID: MemoryId = MemoryId::new(10);
const PHASE_GRADE_RESULT_MAP_MEM_ID: MemoryId = MemoryId::new(11);
const PHASE_PROPOSAL_RESULT_MAP_MEM_ID: MemoryId = MemoryId::new(12);
const CATEGORY_CONFIG_MAP_MEM_ID: MemoryId = MemoryId::new(13);
const CATEGORY_ID_COUNTER_MAP_MEM_ID: MemoryId = MemoryId::new(14);

type _Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CANISTER_CONFIG: RefCell<Cell<CanisterConfig, _Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(CANISTER_CONFIG_MEM_ID)), CanisterConfig::default()
      ).expect("Failed to initialize the canister config cell")
    );

    static ICVC_CONFIG: RefCell<Cell<ICVCConfig, _Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(ICVC_CONFIG_MEM_ID)), ICVCConfig::default()
      ).expect("Failed to initialize the icvc config cell")
    );

    static PROJECT_ID_COUNTER: RefCell<Cell<u64, _Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(PROJECT_ID_COUNTER_MEM_ID)), 0)
            .expect("Failed to initialize the project id counter cell")
    );

    static USERS_MAP: RefCell<StableBTreeMap<UserId, UserModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(USER_MAP_MEM_ID)))
    );

    static USER_PROJECT_MAP: RefCell<StableBTreeMap<(UserId, ProjectId), ProjectModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(USER_PROJECT_MAP_MEM_ID)))
    );

    static PROJECT_STEP_PHASE_MAP: RefCell<StableBTreeMap<(ProjectId, StepPhaseId), StepPhaseModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(PROJECT_STEP_PHASE_MAP_ID)))
    );

    static PROJECT_STEP_MAP: RefCell<StableBTreeMap<(ProjectId, CompositeKey), StepModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(PROJECT_STEP_MAP_MEM_ID)))
    );

    static STEP_GRADE_MAP: RefCell<StableBTreeMap<(UserId, CompositeKey), u32, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(STEP_GRADE_MAP_MEM_ID)))
    );

    static PHASE_CONFIG_MAP: RefCell<StableBTreeMap<StepPhaseId, StepPhaseConfigModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(STEPPHASE_STEPCONFIG_MAP_MEM_ID)))
    );

    static PHASE_STEP_CONFIG_MAP: RefCell<StableBTreeMap<(StepPhaseId, StepId), StepConfigModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(STEP_STEPCONFIG_MAP_MEM_ID)))
    );

    static PHASE_PROPOSAL_MAP: RefCell<StableBTreeMap<(ProjectId, StepPhaseId), u64, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(PHASE_PROPOSAL_MAP_MEM_ID)))
    );

    static PHASE_GRADE_RESULT_MAP: RefCell<StableBTreeMap<(ProjectId, StepPhaseId), StepPhaseGradeResultModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(PHASE_GRADE_RESULT_MAP_MEM_ID)))
    );

    static PHASE_PROPOSAL_RESULT_MAP: RefCell<StableBTreeMap<(ProjectId, StepPhaseId), StepPhaseVoteResultModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(PHASE_PROPOSAL_RESULT_MAP_MEM_ID)))
    );

    static CATEGORY_ID_COUNTER: RefCell<Cell<u64, _Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(CATEGORY_ID_COUNTER_MAP_MEM_ID)), 0)
            .expect("Failed to initialize the project id counter cell")
    );

    static CATEGORY_MAP: RefCell<StableBTreeMap<u64, CategoryModel, _Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(CATEGORY_CONFIG_MAP_MEM_ID)))
    );



}

// Canister config
pub fn set_canister_config(
    config: CanisterConfig,
) -> Result<CanisterConfig, ic_stable_structures::cell::ValueError> {
    CANISTER_CONFIG.with(|cell| cell.borrow_mut().set(config))
}

pub fn get_canister_config() -> CanisterConfig {
    CANISTER_CONFIG.with(|cell| cell.borrow().get().clone())
}

pub fn update_canister_config(
    update_canister_config: CanisterConfigUpdate,
) -> Result<CanisterConfig, ic_stable_structures::cell::ValueError> {
    CANISTER_CONFIG.with(|cell| {
        let mut config_model = cell.borrow().get().clone();

        if let Some(sns_governance_id) = update_canister_config.sns_governance_id {
            config_model.sns_governance_id = Some(sns_governance_id);
        };
        if let Some(subaccount) = update_canister_config.subaccount {
            config_model.subaccount = Some(subaccount);
        };
        if let Some(max_stable_memory_size) = update_canister_config.max_stable_memory_size {
            config_model.max_stable_memory_size = Some(max_stable_memory_size);
        };

        cell.borrow_mut().set(config_model.clone())?;
        Ok(config_model)
    })
}

pub fn set_owner(owner: Principal) -> Result<CanisterConfig, ic_stable_structures::cell::ValueError> {
    CANISTER_CONFIG.with(|cell| {
        let mut config_model = cell.borrow().get().clone();

        if config_model.owner.is_some() {
            config_model.owner = Some(owner);
        };

        cell.borrow_mut().set(config_model.clone())?;
        Ok(config_model)
    })
}

pub fn get_owner() -> Option<Principal> {
    CANISTER_CONFIG.with(|cell| cell.borrow().get().owner)
}

#[allow(dead_code)]
pub fn calculate_used_stable_memory() -> u64 {
    let mut total_pages: u64 = 0;

    MEMORY_MANAGER.with(|m| {
        let memory = m.borrow();

        total_pages += memory.get(CANISTER_CONFIG_MEM_ID).size();
        total_pages += memory.get(ICVC_CONFIG_MEM_ID).size();
        total_pages += memory.get(PROJECT_ID_COUNTER_MEM_ID).size();
        total_pages += memory.get(USER_MAP_MEM_ID).size();
        total_pages += memory.get(USER_PROJECT_MAP_MEM_ID).size();
        total_pages += memory.get(PROJECT_STEP_PHASE_MAP_ID).size();
        total_pages += memory.get(PROJECT_STEP_MAP_MEM_ID).size();
        total_pages += memory.get(STEP_GRADE_MAP_MEM_ID).size();
        total_pages += memory.get(STEPPHASE_STEPCONFIG_MAP_MEM_ID).size();
        total_pages += memory.get(STEP_STEPCONFIG_MAP_MEM_ID).size();
    });

    let total_bytes = total_pages * 64 * 1024;

    ic_cdk::println!("Total memory size in bytes: {}", total_bytes);

    total_bytes
}

//Icvc config
#[allow(dead_code)]
pub fn set_icvc_config(
    config: ICVCConfig,
) -> Result<ICVCConfig, ic_stable_structures::cell::ValueError> {
    ICVC_CONFIG.with(|cell| cell.borrow_mut().set(config))
}

pub fn get_icvc_config() -> ICVCConfig {
    ICVC_CONFIG.with(|cell| cell.borrow().get().clone())
}

pub fn update_icvc_config(
    update_icvc_config: ICVCConfigUpdate,
) -> Result<ICVCConfig, ic_stable_structures::cell::ValueError> {
    ICVC_CONFIG.with(|cell| {
        let mut config_model = cell.borrow().get().clone();

        if let Some(open_duration) = update_icvc_config.open_duration {
            config_model.open_duration = open_duration;
        }
        if let Some(assessment_duration) = update_icvc_config.assessment_duration {
            config_model.assessment_duration = assessment_duration;
        }
        if let Some(grade_min_value) = update_icvc_config.grade_min_value {
            config_model.grade_min_value = grade_min_value;
        }
        if let Some(grade_max_value) = update_icvc_config.grade_max_value {
            config_model.grade_max_value = grade_max_value;
        }
        if let Some(projects_update_timer_interval) =
            update_icvc_config.projects_update_timer_interval
        {
            config_model.projects_update_timer_interval = projects_update_timer_interval;
        }

        cell.borrow_mut().set(config_model.clone())?;
        Ok(config_model)
    })
}

// Projects
pub fn insert_project(
    user_id: UserId,
    project_id: ProjectId,
    project_create: ProjectCreate,
) -> Option<Project> {
    USER_PROJECT_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&(user_id, project_id)) {
            None
        } else {
            let project_model = ProjectModel {
                title: project_create.title,
                moto: project_create.moto,
                description: project_create.description,
                team_members: project_create.team_members,
                links: project_create.links,
                categories: project_create.categories,
                current_phase: u64::default(),
                status: ProjectStatus::Open,
                created_at: ic_cdk::api::time(),
                update_at: None,
                update_by: None,
            };
            map.insert((user_id, project_id), project_model.clone());
            Some(convert_model_to_project(
                user_id,
                project_id,
                project_model.clone(),
            ))
        }
    })
}

pub fn update_project(
    caller_id: UserId,
    user_id: UserId,
    project_id: ProjectId,
    project_update: ProjectUpdate,
) -> Option<Project> {
    USER_PROJECT_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(project_model) = map.get(&(user_id, project_id)).borrow_mut() {
            if let Some(title) = project_update.title {
                project_model.title = title;
            }
            if let Some(moto) = project_update.moto {
                project_model.moto = moto;
            }
            if let Some(description) = project_update.description {
                project_model.description = description;
            }
            if let Some(team_members) = project_update.team_members {
                project_model.team_members = team_members;
            }
            if let Some(links) = project_update.links {
                project_model.links = links;
            }
            match project_update.categories {
                categories => {
                    project_model.categories = categories;
                }
            }

            project_model.update_by = Some(caller_id);
            project_model.update_at = Some(ic_cdk::api::time());

            map.insert((user_id, project_id), project_model.clone());
            Some(convert_model_to_project(
                user_id,
                project_id,
                project_model.clone(),
            ))
        } else {
            None
        }
    })
}

pub fn update_project_current_phase(
    user_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Option<Project> {
    USER_PROJECT_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(project_model) = map.get(&(user_id, project_id)).borrow_mut() {
            project_model.current_phase = step_phase_id;
            map.insert((user_id, project_id), project_model.clone());
            Some(convert_model_to_project(
                user_id,
                project_id,
                project_model.clone(),
            ))
        } else {
            None
        }
    })
}

pub fn update_project_status(
    user_id: UserId,
    project_id: ProjectId,
    new_status: ProjectStatus,
) -> Option<ProjectStatus> {
    USER_PROJECT_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(project_model) = map.get(&(user_id, project_id)).borrow_mut() {
            project_model.status = new_status;
            map.insert((user_id, project_id), project_model.clone());
            Some(project_model.status.clone())
        } else {
            None
        }
    })
}

pub fn delete_project(user_id: UserId, project_id: ProjectId) -> Option<Project> {
    //TODO: DELETE ALL RELATED PROJECT DATA
    USER_PROJECT_MAP.with(|map| {
        map.borrow_mut()
            .remove(&(user_id, project_id))
            .map(|project_model| convert_model_to_project(user_id, project_id, project_model))
    })
}

pub fn retrieve_project_by_user_id_and_project_id(
    user_id: UserId,
    project_id: ProjectId,
) -> Option<Project> {
    USER_PROJECT_MAP.with(|map| {
        map.borrow()
            .get(&(user_id, project_id))
            .map(|project_model| convert_model_to_project(user_id, project_id, project_model))
    })
}

pub fn retrieve_project_by_id(project_id: ProjectId) -> Option<Project> {
    USER_PROJECT_MAP.with(|map| {
        let project_map = map.borrow();

        project_map
            .iter()
            .find(|&((_, stored_project_id), _)| stored_project_id == project_id)
            .map(|((user_id, project_id), project_model)| {
                convert_model_to_project(user_id, project_id, project_model)
            })
    })
}

pub fn retrieve_all_projects(start_at: Option<ProjectId>, limit: Option<usize>) -> Vec<Project> {
    USER_PROJECT_MAP.with(|map| {
        let project_map = map.borrow();

        project_map
            .iter()
            .filter(|&((_, project_id), _)| match start_at {
                Some(start_id) => project_id >= start_id,
                None => true,
            })
            .take(limit.unwrap_or(usize::MAX))
            .map(|((user_id, project_id), project_model)| {
                convert_model_to_project(user_id, project_id, project_model)
            })
            .collect()
    })
}

pub fn retrieve_user_projects(user_id: UserId) -> Vec<Project> {
    USER_PROJECT_MAP.with(|map| {
        let user_project_map = map.borrow_mut();

        user_project_map
            .iter()
            .filter(|(user, _project)| user.0 == user_id)
            .map(|((user_id, project_id), project_model)| {
                convert_model_to_project(user_id, project_id, project_model)
            })
            .collect()
    })
}

//Categories
pub fn insert_category(category_id: u64, category_create: CategoryCreate) -> Option<Category> {
    CATEGORY_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&category_id) {
            None
        } else {
            let category_model = CategoryModel {
                name: category_create.name,
                active: true,
            };
            map.insert(category_id, category_model.clone());
            Some(convert_model_to_category(category_id, category_model))
        }
    })
}

pub fn get_category_by_id(category_id: u64) -> Option<Category> {
    CATEGORY_MAP.with(|map| {
        map.borrow()
            .get(&category_id)
            .map(|category_model| convert_model_to_category(category_id, category_model))
    })
}

pub fn get_all_categories() -> Vec<Category> {
    CATEGORY_MAP.with(|map| {
        map.borrow()
            .iter()
            //.filter(|(_, category)| category.active == true)
            .map(|(category_id, category_model)| {
                convert_model_to_category(category_id, category_model)
            })
            .collect()
    })
}

pub fn desactivate_category_by_id(category_id: u64) -> Option<Category> {
    CATEGORY_MAP.with(|map| {
        let mut map = map.borrow_mut();

        if let Some(category) = map.get(&category_id).borrow_mut() {
            category.active = false;
            map.insert(category_id, category.clone());

            Some(convert_model_to_category(category_id, category.clone()))
        } else {
            None
        }
    })
}

// Step Phase config
pub fn insert_step_phase_config(
    step_phase_id: StepPhaseId,
    step_phase_config_create: StepPhaseConfigCreate,
) -> Option<StepPhaseConfig> {
    PHASE_CONFIG_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&step_phase_id) {
            None
        } else {
            let step_phase_config_model = StepPhaseConfigModel {
                assessement_method: step_phase_config_create.assessment_method,
            };
            map.insert(step_phase_id, step_phase_config_model.clone());
            Some(convert_model_to_step_phase_config(
                step_phase_id,
                step_phase_config_model,
            ))
        }
    })
}

pub fn get_step_phase_config_by_id(step_phase_id: StepPhaseId) -> Option<StepPhaseConfig> {
    PHASE_CONFIG_MAP.with(|map| {
        map.borrow().get(&step_phase_id).map(|step_phase_config| {
            convert_model_to_step_phase_config(step_phase_id, step_phase_config)
        })
    })
}

#[allow(dead_code)]
pub fn get_all_step_phase_configs() -> Vec<StepPhaseConfig> {
    PHASE_CONFIG_MAP.with(|map| {
        map.borrow()
            .iter()
            .map(|(step_phase_id, step_phase_config)| {
                convert_model_to_step_phase_config(step_phase_id, step_phase_config)
            })
            .collect()
    })
}

#[allow(dead_code)]
pub fn delete_step_phase_config(step_phase_id: StepPhaseId) -> Option<StepPhaseConfig> {
    //TODO: Also delete associated steps config
    PHASE_CONFIG_MAP.with(|map| {
        map.borrow_mut()
            .remove(&step_phase_id)
            .map(|step_phase_config| {
                convert_model_to_step_phase_config(step_phase_id, step_phase_config)
            })
    })
}

pub fn get_step_phases_config_count() -> usize {
    PHASE_CONFIG_MAP.with(|map| map.borrow_mut().iter().count())
}

// Step config
pub fn insert_step_config(
    step_phase_id: StepPhaseId,
    step_id: StepId,
    step_config_create: StepConfigCreate,
) -> Option<StepConfig> {
    PHASE_STEP_CONFIG_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&(step_phase_id, step_id)) {
            None
        } else {
            let step_config_model = StepConfigModel {
                questions: step_config_create.questions,
                required_upload_files: step_config_create.required_upload_files,
                checkboxes: step_config_create.checkboxes,
                numeric_values: step_config_create.decimal_values,
            };
            map.insert((step_phase_id, step_id), step_config_model.clone());
            Some(convert_model_to_step_config(
                step_phase_id,
                step_id,
                step_config_model,
            ))
        }
    })
}

#[allow(dead_code)]
pub fn get_step_config_by_id(step_phase_id: StepPhaseId, step_id: StepId) -> Option<StepConfig> {
    PHASE_STEP_CONFIG_MAP.with(|map| {
        map.borrow()
            .get(&(step_phase_id, step_id))
            .map(|step_config| convert_model_to_step_config(step_phase_id, step_id, step_config))
    })
}

pub fn get_all_phase_steps_config(step_phase_id: StepPhaseId) -> Vec<StepConfig> {
    PHASE_STEP_CONFIG_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|&((_step_phase_id, _), _)| _step_phase_id == step_phase_id)
            .map(|((step_phase_id, step_id), step_config)| {
                convert_model_to_step_config(step_phase_id, step_id, step_config)
            })
            .collect()
    })
}

#[allow(dead_code)]
pub fn delete_step_config(step_phase_id: StepPhaseId, step_id: StepId) -> Option<StepConfig> {
    PHASE_STEP_CONFIG_MAP.with(|map| {
        map.borrow_mut()
            .remove(&(step_phase_id, step_id))
            .map(|step_config| convert_model_to_step_config(step_phase_id, step_id, step_config))
    })
}

// Step phase
pub fn insert_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_phase_create: StepPhaseCreate,
) -> Option<StepPhase> {
    PROJECT_STEP_PHASE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&(project_id, step_phase_id)) {
            None
        } else {
            let model = StepPhaseModel {
                status: step_phase_create.status,
                start_open_date: step_phase_create.start_open_date,
                end_open_date: step_phase_create.end_open_date,
                submit_date: step_phase_create.submit_date,
                start_assessment_date: step_phase_create.start_assessment_date,
                end_assessment_date: step_phase_create.end_assessment_date,
                assessment_method: step_phase_create.assessment_method,
            };
            map.insert((project_id, step_phase_id), model.clone());
            Some(convert_model_to_phase(project_id, step_phase_id, model))
        }
    })
}

pub fn update_step_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_phase_update: StepPhaseUpdate,
) -> Option<StepPhase> {
    PROJECT_STEP_PHASE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(model) = map.get(&(project_id, step_phase_id)).borrow_mut() {
            if let Some(status) = step_phase_update.status {
                model.status = status;
            }
            if let Some(start_open_date) = step_phase_update.start_open_date {
                model.start_open_date = start_open_date;
            }
            if let Some(end_open_date) = step_phase_update.end_open_date {
                model.end_open_date = end_open_date;
            }
            if let Some(submit_date) = step_phase_update.submit_date {
                model.submit_date = Some(submit_date);
            }
            if let Some(start_assessment_date) = step_phase_update.start_assessment_date {
                model.start_assessment_date = start_assessment_date;
            }
            if let Some(end_assessment_date) = step_phase_update.end_assessment_date {
                model.end_assessment_date = end_assessment_date;
            }

            map.insert((project_id, step_phase_id), model.clone());

            Some(convert_model_to_phase(
                project_id,
                step_phase_id,
                model.clone(),
            ))
        } else {
            None
        }
    })
}

pub fn update_step_phase_status(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    new_status: StepPhaseStatus,
) -> Option<StepPhaseStatus> {
    PROJECT_STEP_PHASE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(model) = map.get(&(project_id, step_phase_id)).borrow_mut() {
            model.status = new_status;
            map.insert((project_id, step_phase_id), model.clone());

            Some(model.status.clone())
        } else {
            None
        }
    })
}

pub fn get_step_phase_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Option<StepPhase> {
    PROJECT_STEP_PHASE_MAP.with(|map| {
        map.borrow()
            .get(&(project_id, step_phase_id))
            .map(|model| convert_model_to_phase(project_id, step_phase_id, model))
    })
}

pub fn get_all_phases_by_project(project_id: ProjectId) -> Vec<StepPhase> {
    PROJECT_STEP_PHASE_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|&((project_id_key, _), _)| project_id == project_id_key)
            .map(|((project_id_key, step_phase_id_key), step_phase_model)| {
                convert_model_to_phase(project_id_key, step_phase_id_key, step_phase_model)
            })
            .collect()
    })
}

#[allow(dead_code)]
pub fn delete_step_phase(project_id: ProjectId, step_phase_id: StepPhaseId) -> Option<()> {
    PROJECT_STEP_PHASE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        map.remove(&(project_id, step_phase_id)).map(|_| ())
    })
}

// Steps
pub fn insert_step(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    step_create: StepCreate,
) -> Option<Step> {
    let phase_step_key = CompositeKey::construct_key(&(step_phase_id, step_id));
    PROJECT_STEP_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&(project_id, phase_step_key.clone())) {
            None
        } else {
            let model = StepModel {
                questions_submission: step_create.questions_submission,
                checkbox_submission: step_create.checkbox_submission,
                numeric_submission: step_create.numeric_submission,
                upload_files: step_create.upload_files,
                grade_end_date: None,
                update_by: None,
                update_at: None,
            };
            map.insert((project_id, phase_step_key), model.clone());
            Some(convert_model_to_step(
                project_id,
                step_phase_id,
                step_id,
                model,
            ))
        }
    })
}

pub fn update_step(
    caller_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    step_update: StepUpdate,
) -> Option<Step> {
    let phase_step_key = CompositeKey::construct_key(&(step_phase_id, step_id));
    PROJECT_STEP_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(step_model) = map.get(&(project_id, phase_step_key.clone())).borrow_mut() {
            if let Some(questions) = step_update.questions_submission {
                step_model.questions_submission = questions;
            }
            if let Some(checkbox_submission) = step_update.checkbox_submission {
                step_model.checkbox_submission = checkbox_submission;
            }
            if let Some(numeric_submission) = step_update.numeric_submission {
                step_model.numeric_submission = numeric_submission;
            }

            if let Some(upload_files) = step_update.upload_files {
                for new_file in upload_files {
                    if let Some(pos) = step_model
                        .upload_files
                        .iter()
                        .position(|file| file.document_type == new_file.document_type)
                    {
                        step_model.upload_files[pos] = new_file;
                    } else {
                        step_model.upload_files.push(new_file);
                    }
                }
            }

            step_model.update_by = Some(caller_id);
            step_model.update_at = Some(ic_cdk::api::time());

            map.insert((project_id, phase_step_key), step_model.clone());
            Some(convert_model_to_step(
                project_id,
                step_phase_id,
                step_id,
                step_model.clone(),
            ))
        } else {
            None
        }
    })
}

pub fn get_step_by_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> Option<Step> {
    let phase_step_key = CompositeKey::construct_key(&(step_phase_id, step_id));
    PROJECT_STEP_MAP.with(|map| {
        map.borrow()
            .get(&(project_id, phase_step_key))
            .map(|step_config| {
                convert_model_to_step(project_id, step_phase_id, step_id, step_config)
            })
    })
}

pub fn get_all_steps_by_phase(project_id: ProjectId, step_phase_id: StepPhaseId) -> Vec<Step> {
    PROJECT_STEP_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|((project_id_key, phase_step_key), _)| {
                let (step_phase_key, _step_key) = phase_step_key.deconstruct_key();
                *project_id_key == project_id && step_phase_key == step_phase_id
            })
            .map(|((project_id_key, phase_step_key), step_model)| {
                let (step_phase_key, _step_key) = phase_step_key.deconstruct_key();
                convert_model_to_step(project_id_key, step_phase_key, _step_key, step_model)
            })
            .collect()
    })
}

//Grades
pub fn put_step_grade(
    user_id: UserId,
    project_id: u64,
    step_phase_id: u64,
    step_id: u64,
    grade: u32,
) -> Option<u32> {
    let key = CompositeKey::construct_key(&(project_id, step_phase_id, step_id));
    STEP_GRADE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        map.insert((user_id, key), grade);
        Some(grade)
    })
}

pub fn get_step_grade_by_id(
    user_id: UserId,
    project_id: u64,
    step_phase_id: u64,
    step_id: u64,
) -> Option<StepGrade> {
    let key = CompositeKey::construct_key(&(project_id, step_phase_id, step_id));
    STEP_GRADE_MAP.with(|map| {
        map.borrow().get(&(user_id, key)).map(|grade| {
            convert_model_to_step_grade(user_id, project_id, step_phase_id, step_id, grade)
        })
    })
}

pub fn get_all_phase_steps_grade(
    user_id: UserId,
    project_id: u64,
    phase_id: u64,
) -> Vec<StepGrade> {
    STEP_GRADE_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|((user_id_key, composite_key), _)| {
                let (project_id_key, phase_id_key, _) = composite_key.deconstruct_key();
                user_id == *user_id_key && project_id_key == project_id && phase_id_key == phase_id
            })
            .map(|((user_id, composite_key), grade_model)| {
                let (project_id_key, step_phase_id_key, _step_key) =
                    composite_key.deconstruct_key();
                convert_model_to_step_grade(
                    user_id,
                    project_id_key,
                    step_phase_id_key,
                    _step_key,
                    grade_model,
                )
            })
            .collect()
    })
}

pub fn get_users_step_grades(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
) -> Vec<u32> {
    STEP_GRADE_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|((_user_id_key, composite_key), _)| {
                let (proj_id, phase_id, stp_id) = composite_key.deconstruct_key();
                proj_id == project_id && phase_id == step_phase_id && stp_id == step_id
            })
            .map(|((_, _), grade)| grade)
            .collect()
    })
}

pub fn put_step_phase_grade_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    grade_result_create: StepPhaseGradeResultCreate,
) -> StepPhaseGradeResult {
    let key = (project_id, step_phase_id);
    PHASE_GRADE_RESULT_MAP.with(|map| {
        let mut map = map.borrow_mut();

        let model = StepPhaseGradeResultModel {
            avg_result: grade_result_create.avg_result,
            total_steps_grades_count: grade_result_create.total_steps_grades_count,
            steps_grade_results: grade_result_create.steps_grade_results,
        };

        map.insert(key, model.clone());
        convert_model_to_step_phase_grade_result(project_id, step_phase_id, model)
    })
}

pub fn get_grade_result_by_step_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Option<StepPhaseGradeResult> {
    let key = (project_id, step_phase_id);
    PHASE_GRADE_RESULT_MAP.with(|map| {
        map.borrow().get(&key).map(|result| {
            convert_model_to_step_phase_grade_result(project_id, step_phase_id, result)
        })
    })
}

//Votes
pub fn put_step_phase_proposal(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    proposal_id: u64,
) -> u64 {
    let key = (project_id, step_phase_id);
    PHASE_PROPOSAL_MAP.with(|map| {
        let mut map = map.borrow_mut();
        map.insert(key, proposal_id);
        proposal_id
    })
}

pub fn get_proposal_by_step_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Option<StepPhaseProposal> {
    let key = (project_id, step_phase_id);
    PHASE_PROPOSAL_MAP.with(|map| {
        map.borrow().get(&key).map(|proposal_id| {
            convert_model_to_step_phase_proposal(project_id, step_phase_id, proposal_id)
        })
    })
}

pub fn get_all_proposals_by_step_phase(
    project_id: ProjectId,
    phase_id: StepPhaseId,
) -> Vec<StepPhaseProposal> {
    PHASE_PROPOSAL_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|((project_id_key, phase_id_key), _)| {
                project_id == *project_id_key && phase_id == *phase_id_key
            })
            .map(|((project_id_key, step_phase_id_key), proposal_id)| {
                convert_model_to_step_phase_proposal(project_id_key, step_phase_id_key, proposal_id)
            })
            .collect()
    })
}

pub fn put_step_phase_vote_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    vote_result_create: StepPhaseVoteResultCreate,
) -> StepPhaseVoteResult {
    let key = (project_id, step_phase_id);
    PHASE_PROPOSAL_RESULT_MAP.with(|map| {
        let mut map = map.borrow_mut();

        let model = StepPhaseVoteResultModel {
            yes: vote_result_create.yes,
            no: vote_result_create.no,
            total: vote_result_create.total,
            approved: vote_result_create.approved,
        };

        map.insert(key, model.clone());
        convert_model_to_step_phase_vote_result(project_id, step_phase_id, model)
    })
}

pub fn get_vote_result_by_step_phase_id(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
) -> Option<StepPhaseVoteResult> {
    let key = (project_id, step_phase_id);
    PHASE_PROPOSAL_RESULT_MAP.with(|map| {
        map.borrow().get(&key).map(|result| {
            convert_model_to_step_phase_vote_result(project_id, step_phase_id, result)
        })
    })
}

//Users
pub fn save_admin(create_admin: UserCreate) -> Option<User> {
    USERS_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if map.contains_key(&create_admin.user_id) {
            None
        } else {
            let user_model = UserModel {
                name: create_admin.name,
                is_admin: true,
            };
            map.insert(create_admin.user_id, user_model.clone());
            Some(convert_model_to_user(create_admin.user_id, user_model))
        }
    })
}

pub fn update_user(user_id: UserId, update_user: UserUpdate) -> Option<User> {
    USERS_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(user_model) = map.get(&user_id).borrow_mut() {
            user_model.name = update_user.name;

            map.insert(user_id, user_model.clone());
            Some(convert_model_to_user(user_id, user_model.clone()))
        } else {
            None
        }
    })
}

pub fn get_user_by_id(user_id: UserId) -> Option<User> {
    USERS_MAP.with(|map| {
        map.borrow()
            .get(&user_id)
            .map(|user_model| convert_model_to_user(user_id, user_model))
    })
}

pub fn get_all_admin_users() -> Vec<User> {
    USERS_MAP.with(|map| {
        map.borrow()
            .iter()
            .filter(|(_key, user_model)| user_model.is_admin)
            .map(|(user_id, user_model)| convert_model_to_user(user_id, user_model))
            .collect()
    })
}

pub fn delete_user(user_id: UserId) -> Option<User> {
    USERS_MAP.with(|map| {
        map.borrow_mut()
            .remove(&user_id)
            .map(|user_model| convert_model_to_user(user_id, user_model))
    })
}

//Helpers

pub fn generate_project_id() -> u64 {
    PROJECT_ID_COUNTER.with(|counter_cell| {
        let current_value = *counter_cell.borrow().get();
        let new_value = current_value + 1;
        counter_cell
            .borrow_mut()
            .set(new_value)
            .expect("Error incrementing project ID.");

        new_value
    })
}

pub fn generate_category_id() -> u64 {
    CATEGORY_ID_COUNTER.with(|counter_cell| {
        let current_value = *counter_cell.borrow().get();
        let new_value = current_value + 1;
        counter_cell
            .borrow_mut()
            .set(new_value)
            .expect("Error incrementing category ID.");

        new_value
    })
}

fn convert_model_to_project(
    user_id: UserId,
    project_id: ProjectId,
    project_model: ProjectModel,
) -> Project {
    Project {
        user_id,
        id: project_id,
        title: project_model.title,
        moto: project_model.moto,
        description: project_model.description,
        team_members: project_model.team_members,
        links: project_model.links,
        categories: project_model.categories,
        current_phase: project_model.current_phase,
        status: project_model.status,
        created_at: project_model.created_at,
        update_by: project_model.update_by,
        update_at: project_model.update_at,
    }
}

fn convert_model_to_category(category_id: u64, model: CategoryModel) -> Category {
    Category {
        id: category_id,
        name: model.name,
        active: model.active,
    }
}

fn convert_model_to_step_phase_config(
    step_phase_id: StepPhaseId,
    model: StepPhaseConfigModel,
) -> StepPhaseConfig {
    StepPhaseConfig {
        id: step_phase_id,
        assessment_method: model.assessement_method,
    }
}

fn convert_model_to_step_config(
    step_phase_id: StepPhaseId,
    step_id: StepId,
    model: StepConfigModel,
) -> StepConfig {
    StepConfig {
        id: step_phase_id,
        step_id,
        questions: model.questions,
        required_upload_files: model.required_upload_files,
        checkboxes: model.checkboxes,
        decimal_values: model.numeric_values,
    }
}

fn convert_model_to_phase(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    model: StepPhaseModel,
) -> StepPhase {
    StepPhase {
        project_id: project_id,
        id: step_phase_id,
        status: model.status,
        start_open_date: model.start_open_date,
        end_open_date: model.end_open_date,
        submit_date: model.submit_date,
        start_assessment_date: model.start_assessment_date,
        end_assessment_date: model.end_assessment_date,
        assessment_method: model.assessment_method,
    }
}

fn convert_model_to_step(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    model: StepModel,
) -> Step {
    Step {
        project_id: project_id,
        step_phase_id: step_phase_id,
        id: step_id,
        question_submission: model.questions_submission,
        checkbox_submission: model.checkbox_submission,
        decimal_submission: model.numeric_submission,
        upload_files: model.upload_files,
        grade_end_date: model.grade_end_date,
        update_by: model.update_by,
        update_at: model.update_at,
    }
}

fn convert_model_to_step_grade(
    user_id: UserId,
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    step_id: StepId,
    grade: u32,
) -> StepGrade {
    StepGrade {
        user_id: user_id,
        project_id: project_id,
        step_phase_id: step_phase_id,
        step_id: step_id,
        grade,
    }
}

fn convert_model_to_step_phase_proposal(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    proposal_id: u64,
) -> StepPhaseProposal {
    StepPhaseProposal {
        project_id,
        step_phase_id,
        proposal_id,
    }
}

fn convert_model_to_step_phase_grade_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    model: StepPhaseGradeResultModel,
) -> StepPhaseGradeResult {
    StepPhaseGradeResult {
        project_id,
        step_phase_id,
        avg_result: model.avg_result,
        total_steps_grades_count: model.total_steps_grades_count,
        steps_grade_results: model.steps_grade_results,
    }
}

fn convert_model_to_step_phase_vote_result(
    project_id: ProjectId,
    step_phase_id: StepPhaseId,
    model: StepPhaseVoteResultModel,
) -> StepPhaseVoteResult {
    StepPhaseVoteResult {
        project_id,
        step_phase_id,
        yes: model.yes,
        no: model.no,
        total: model.total,
        approved: model.approved,
    }
}

fn convert_model_to_user(user_id: UserId, user_model: UserModel) -> User {
    User {
        user_id,
        name: user_model.name,
        is_admin: user_model.is_admin,
    }
}
