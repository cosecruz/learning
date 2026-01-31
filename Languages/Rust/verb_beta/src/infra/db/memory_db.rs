use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    domain::{
        ApplicationError,
        model::{ActionLog, Verb, VerbState},
        repository::{action_log_repo::ActionLogRepository, verb_repo::VerbRepository},
    },
    infra::db::Database,
};

#[derive(Clone, Default)]
pub struct InMemoryRepo {
    pub verb_store: Arc<Mutex<Vec<Verb>>>,
    pub action_log_store: Arc<Mutex<Vec<ActionLog>>>,
}

pub struct InMemoryDatabase;

impl Database for InMemoryDatabase {
    type Pool = InMemoryRepo;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    async fn connect(_: &str) -> Result<Self::Pool, Self::Error> {
        Ok(InMemoryRepo::default())
    }

    async fn migrate(_: &Self::Pool) -> Result<(), Self::Error> {
        // no-op for in-memory
        Ok(())
    }
}

/* ================================
VerbRepository Implementation
================================ */

impl VerbRepository for InMemoryRepo {
    // =============================================================
    async fn create(
        &self,
        verb: &Verb,
        action_log: &ActionLog,
    ) -> Result<(), crate::domain::error::ApplicationError> {
        // must atomically create and store verb and actionlog
        use VerbState::*;

        let valid_log = matches!(
            (
                action_log.from_state(),
                action_log.to_state(),
                action_log.verb_id() == verb.id(),
            ),
            (None, Captured, true)
        );

        if !valid_log {
            return Err(ApplicationError::Placeholder);
        }

        let mut verbs = self.verb_store.lock().await;
        let mut logs = self.action_log_store.lock().await;

        // uniqueness check
        if verbs.iter().any(|v| v.id() == verb.id()) {
            return Err(crate::domain::error::ApplicationError::Placeholder);
        }

        verbs.push(verb.clone());
        logs.push(action_log.clone());

        Ok(())
    }

    async fn get_by_id(
        &self,
        id: crate::domain::model::VerbId,
    ) -> Result<Option<Verb>, crate::domain::error::ApplicationError> {
        let verbs = self.verb_store.lock().await;

        Ok(verbs.iter().find(|v| v.id() == id).cloned())
    }

    // ==============================================================
    async fn list(
        &self,
        filter: crate::domain::repository::verb_repo::VerbFilter,
    ) -> Result<
        crate::domain::repository::verb_repo::VerbListResult,
        crate::domain::error::ApplicationError,
    > {
        let verbs = self.verb_store.lock().await;

        let results: Vec<Verb> = verbs
            .iter()
            .filter(|&v| {
                if let Some(state) = filter.state {
                    v.state() == state
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        let total = results.len() as i64;

        // pagination
        let start = filter.offset as usize;
        let end = (start + filter.limit as usize).min(results.len());

        let paged = if start < results.len() {
            results[start..end].to_vec()
        } else {
            Vec::new()
        };

        Ok(crate::domain::repository::verb_repo::VerbListResult {
            verbs: paged,
            total,
        })
    }

    // ==================================================================
    async fn update_state(
        &self,
        verb: &Verb,
        action_log: &ActionLog,
    ) -> Result<(), crate::domain::error::ApplicationError> {
        let mut verbs = self.verb_store.lock().await;
        let mut logs = self.action_log_store.lock().await;

        // find verb
        let pos = verbs.iter().position(|v| v.id() == verb.id());

        let idx = match pos {
            Some(i) => i,
            None => return Err(crate::domain::error::ApplicationError::Placeholder),
        };

        // invariant: action log must match verb
        if action_log.verb_id() != verb.id() {
            return Err(crate::domain::error::ApplicationError::Placeholder);
        }

        // replace verb snapshot
        verbs[idx] = verb.clone();

        // append log
        logs.push(action_log.clone());

        Ok(())
    }
}

/* ================================
Action_Log Repository Implementation
================================ */
impl ActionLogRepository for InMemoryRepo {
    async fn get_for_verb(
        &self,
        verb_id: crate::domain::model::VerbId,
        limit: u32,
    ) -> Result<Vec<ActionLog>, crate::domain::error::ApplicationError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_new_verb() {
        let repo = InMemoryRepo::default();

        let verb = Verb::new("Test Verb", "Test Desc").unwrap();
        let log = ActionLog::created(verb.id());

        repo.create(&verb, &log).await.unwrap();

        let stored = repo.get_by_id(verb.id()).await.unwrap();

        assert!(stored.is_some());
        let stored = stored.unwrap();

        assert_eq!(stored.title(), "Test Verb");
        assert_eq!(stored.state(), VerbState::Captured);
    }

    #[tokio::test]
    async fn update_state_creates_log() {
        let repo = InMemoryRepo::default();

        let mut verb = Verb::new("Test Verb", "Test Desc").unwrap();
        let log1 = ActionLog::created(verb.id());

        repo.create(&verb, &log1).await.unwrap();

        let log2 = verb.transition_to(VerbState::Active, None).unwrap();
        repo.update_state(&verb, &log2).await.unwrap();

        let verbs = repo.verb_store.lock().await;
        let logs = repo.action_log_store.lock().await;

        assert_eq!(verbs.len(), 1);
        assert_eq!(logs.len(), 2);
        assert_eq!(verbs[0].state(), VerbState::Active);
    }

    #[tokio::test]
    async fn list_filtering() {
        let repo = InMemoryRepo::default();

        for i in 0..3 {
            let mut verb = Verb::new(&format!("Verb {}", i), "Desc").unwrap();
            let log = ActionLog::created(verb.id());
            repo.create(&verb, &log).await.unwrap();

            if i == 1 {
                let log2 = verb.transition_to(VerbState::Active, None).unwrap();
                repo.update_state(&verb, &log2).await.unwrap();
            }
        }

        let filter =
            crate::domain::repository::verb_repo::VerbFilter::new().with_state(VerbState::Active);

        let result = repo.list(filter).await.unwrap();

        println!("{:?}", result.verbs);
        assert_eq!(result.total, 1);
        assert_eq!(result.verbs.len(), 1);
        assert_eq!(result.verbs[0].state(), VerbState::Active);
    }
}

// time storage format
// 🧠 Real-world systems do this

// Kubernetes → RFC3339 timestamps

// Kafka → epoch millis/nanos

// Postgres → TIMESTAMPTZ

// Event stores → ISO-8601

// OpenTelemetry → nanoseconds

// CloudEvents → RFC3339
