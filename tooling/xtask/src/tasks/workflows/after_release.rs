use gh_workflow::*;

use crate::tasks::workflows::{
    release::{self, notify_on_failure},
    runners,
    steps::{CommonJobConditions, NamedJob, checkout_repo, named},
};

pub fn after_release() -> Workflow {
    let create_sentry_release = create_sentry_release();
    let notify_on_failure = notify_on_failure(&[&create_sentry_release]);

    named::workflow()
        .on(Event::default().release(Release::default().types(vec![ReleaseType::Published])))
        .add_job(create_sentry_release.name, create_sentry_release.job)
        .add_job(notify_on_failure.name, notify_on_failure.job)
}

fn create_sentry_release() -> NamedJob {
    let job = Job::default()
        .runs_on(runners::LINUX_SMALL)
        .with_repository_owner_guard()
        .add_step(checkout_repo())
        .add_step(release::create_sentry_release());
    named::job(job)
}
