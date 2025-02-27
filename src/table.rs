use crate::compute::DQCompute;
use crate::error::DQError;
use crate::storage::DQStorage;
use arrow::array::RecordBatch;
use sqlparser::ast::Statement;

pub struct DQTable {
    pub storage: Box<dyn DQStorage>,
    pub compute: Box<dyn DQCompute>,
}

impl DQTable {
    pub fn new(storage: Box<dyn DQStorage>, compute: Box<dyn DQCompute>) -> Self {
        DQTable { storage, compute }
    }

    pub async fn update(&mut self) -> Result<(), DQError> {
        self.storage.update().await?;

        Ok(())
    }

    pub async fn execute(&mut self, statement: &Statement) -> Result<Vec<RecordBatch>, DQError> {
        let files = self.storage.execute(statement).await?;
        let batches = self.compute.execute(statement, files).await?;

        Ok(batches)
    }
}
