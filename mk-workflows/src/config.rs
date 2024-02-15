use crate::{
    Features, HostOS, Job, TargetConf, Workflow, WorkflowKind, WINDOWS_JOB,
};

pub const DEFAULT_ANDROID_API_LEVEL: usize = 26;

pub fn workflows() -> Vec<Workflow> {
    let mut workflows = Vec::new();
    for kind in &[WorkflowKind::Release] {
        let kind = *kind;
        workflows.push(Workflow {
            kind,
            host_os: HostOS::Windows,
            host_target: "x86_64-pc-windows-msvc",
            job_template: WINDOWS_JOB,
            targets: windows_targets(),
            host_bin_ext: ".exe",
        });
    }
    workflows
}

pub fn jobs(workflow: &Workflow) -> Vec<Job> {
    match workflow.kind {
        WorkflowKind::Release => release_jobs(workflow),
    }
}

/// Jobs for releasing prebuilt binaries.
pub fn release_jobs(workflow: &Workflow) -> Vec<Job> {
    let mut jobs: Vec<_> = [
        release_job(""),
        release_job("gl"),
        release_job("vulkan"),
        release_job("textlayout"),
        release_job("gl,textlayout"),
        release_job("vulkan,textlayout"),
        release_job("gl,vulkan,textlayout"),
    ]
    .into();

    match workflow.host_os {
        HostOS::Windows => {
            jobs.push(release_job("d3d"));
            jobs.push(release_job("d3d,textlayout"));
            jobs.push(release_job("d3d,gl,textlayout"));
            let static_jobs: Vec<_> = jobs
                .iter()
                .cloned()
                .map(|j| Job {
                    name: j.name + "-static",
                    crt_static: true,
                    ..j
                })
                .collect();
            jobs.extend(static_jobs);
        }
    }

    jobs
}

fn release_job(features: impl Into<Features>) -> Job {
    let features = features.into();
    let name = {
        let name = features.name("-");
        if !name.is_empty() {
            format!("release-{name}")
        } else {
            "release".into()
        }
    };
    Job {
        name,
        toolchain: "stable",
        features,
        ..Job::default()
    }
}

fn windows_targets() -> Vec<TargetConf> {
    [TargetConf::new("x86_64-pc-windows-msvc", "d3d")].into()
}
