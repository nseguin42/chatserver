use std::fmt::Debug;
use actix::ActorStreamExt;

use postgres_types::Type;
use tokio_postgres::{Client, Statement, ToStatement};

use crate::error::Error;

pub(crate) trait ToRepoStatement {
    fn as_string(&self) -> String;
    fn get_types(&self) -> Vec<Type>;
}

impl From<&dyn ToRepoStatement> for RepoStatement {
    fn from(s: &dyn ToRepoStatement) -> Self {
        let statement = s.as_string();
        let types = s.get_types();
        Self::new(statement, types)
    }
}

#[derive(Clone)]
pub(crate) struct RepoStatement {
    statement: String,
    prepared: Option<Statement>,
    types: Vec<Type>,
}

impl Debug for RepoStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepoStatement")
            .field("statement", &self.statement)
            .field("types", &self.types)
            .field("prepared", &self.prepared.is_some())
            .finish()
    }
}

impl RepoStatement {
    pub(crate) fn new(statement: String, types: Vec<Type>) -> Self {
        Self {
            statement,
            types,
            prepared: None,
        }
    }

    pub(crate) async fn prepare(&mut self, client: &Client) -> Result<(), Error> {
        let statement = client.prepare_typed(&self.statement, &self.types);

        // Set a timeout for the statement preparation
        let statement = tokio::time::timeout(std::time::Duration::from_secs(1), statement).await;

        match statement {
            Ok(statement) => {
                self.prepared = Some(statement.unwrap());
                Ok(())
            }
            Err(e) => Err(Error::Db(format!(
                "Failed to prepare statement {}, error: {}",
                self.statement, e
            )))?,
        }
    }

    fn to_statement(&self) -> &dyn ToStatement {
        let prepared = &self.prepared;
        match prepared {
            Some(x) => x,
            None => &self.statement,
        }
    }
}
