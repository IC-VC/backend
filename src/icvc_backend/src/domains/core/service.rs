use std::{cell::RefCell, time::Duration};

use crate::{
    domains::{
        project::types::{Project, ProjectStatus},
        step::{
            self,
            types::{AssessmentMethod, StepPhase, StepPhaseStatus},
        },
    },
    repository,
};
use ic_cdk_timers::TimerId;

thread_local! {
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}

pub fn start_update_projects_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    ic_cdk::println!("Timer canister: Starting a new timer with {secs:?} interval...");
    let timer_id = ic_cdk_timers::set_timer_interval(secs, update_projects_status_and_phases);

    TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

//TODO: Move the code bellow to another module
//Check project deadlines, update status and phases
pub fn update_projects_status_and_phases() {
    let projects = repository::retrieve_all_projects(None, None);
    let current_time = ic_cdk::api::time();

    //ic_cdk::println!("len: {} ", projects.len());

    for project in projects {
        if project.status != ProjectStatus::Open {
            continue;
        }

        let step_phases = repository::get_all_phases_by_project(project.id);
        process_phases(&project, step_phases, current_time);
    }
}

fn process_phases(project: &Project, phases: Vec<StepPhase>, current_time: u64) {
    for phase in phases {
        if project.current_phase != phase.id {
            continue;
        }

        /*
        ic_cdk::println!(
            "Checking project id {} - status: {} - phase: {} - status: {} ",
            project.id,
            project.status,
            phase.id,
            phase.status
        );
        */
        match phase.status {
            StepPhaseStatus::Open if current_time > phase.end_open_date => {
                repository::update_project_status(
                    project.user_id,
                    project.id,
                    ProjectStatus::NotSubmitted,
                );
                repository::update_step_phase_status(
                    project.id,
                    phase.id,
                    StepPhaseStatus::NotSubmitted,
                );
            }
            StepPhaseStatus::Submitted => update_submitted_phase(project, phase, current_time),
            _ => (),
        }
    }
}

fn update_submitted_phase(project: &Project, phase: StepPhase, current_time: u64) {
    // Exit if the assessment period is not over
    if current_time < phase.end_assessment_date {
        return;
    }

    let approved = match phase.assessment_method {
        //AssessmentMethod::Vote => true,
        AssessmentMethod::Grade => {
            step::service::save_and_calculate_grade_result(project.id, phase.id);
            true
        }
        _ => false,
    };

    if phase.assessment_method == AssessmentMethod::Grade {
        update_phase_status(&project, phase, approved);
    }
}

// Update the status of the project and phase based on assessment results
pub fn update_phase_status(project: &Project, phase: StepPhase, approved: bool) {
    if approved {
        if let Some(next_phase) = check_next_phase(project) {
            let _ = match step::service::create_step_phase(project.user_id, project.id, next_phase)
            {
                Ok(_) => {
                    repository::update_step_phase_status(
                        project.id,
                        phase.id,
                        StepPhaseStatus::Approved,
                    );
                    Ok(())
                }
                Err(e) => {
                    ic_cdk::println!(
                        "Error creating next step phase, for project id: {}, on step phase id: {}. This error should never happen: {:?}",
                        project.id, phase.id, e);
                    Err(())
                }
            };
        } else {
            finalize_project(&project, phase);
        }
    } else {
        repository::update_project_status(project.user_id, project.id, ProjectStatus::NotFunded);
        repository::update_step_phase_status(project.id, phase.id, StepPhaseStatus::NotApproved);
    }
}

// Finalize the project if no more phases are left
fn finalize_project(project: &Project, last_phase: StepPhase) {
    if last_phase.assessment_method == AssessmentMethod::Vote {
        repository::update_project_status(project.user_id, project.id, ProjectStatus::Funded);
        ic_cdk::println!("Project {} Funded", project.id);
    } else {
        ic_cdk::println!("The project can't be closed, since the last assessment isn't a vote");
    }
}

fn check_next_phase(project: &Project) -> Option<u64> {
    let next_phase = project.current_phase + 1;
    let phase_count = repository::get_step_phases_config_count();

    //ic_cdk::println!("Next phase: {} count: {}", next_phase, phase_count);
    if next_phase < phase_count as u64 {
        Some(next_phase)
    } else {
        None
    }
}
