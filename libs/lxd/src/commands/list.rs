use crate::{LxdClient, LxdContainer, Result};

impl LxdClient {
    pub fn list(&self) -> Result<Vec<LxdContainer>> {
        let response = self.invoke(&[
            "list".to_string(),
            "--format=json".to_string(),
        ])?;

        let output = response.output_sync()?;

        // @todo this is grando not nice
        let containers: Vec<LxdContainer> = serde_json::from_str(&output)
            .unwrap();

        Ok(containers)
    }
}