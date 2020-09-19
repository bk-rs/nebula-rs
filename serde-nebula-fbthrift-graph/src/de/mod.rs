pub mod data;
pub mod datetime;

use nebula_fbthrift_graph::ExecutionResponse;
use serde::Deserialize;

use crate::de::data::{DataDeserializeError, DataDeserializeErrorKind, DataDeserializer};

pub fn deserialize_execution_response<'de, D: Deserialize<'de>>(
    execution_response: &'de ExecutionResponse,
) -> Result<Vec<D>, DataDeserializeError> {
    let names = match &execution_response.column_names {
        Some(column_names) if !column_names.is_empty() => column_names,
        _ => {
            return Err(DataDeserializeError::new(
                None,
                DataDeserializeErrorKind::Custom("column_names is none or empty".to_owned()),
            ))
        }
    };

    let rows = match &execution_response.rows {
        Some(rows) => rows,
        None => {
            return Err(DataDeserializeError::new(
                None,
                DataDeserializeErrorKind::Custom("rows is none".to_owned()),
            ))
        }
    };

    let mut data_set: Vec<D> = vec![];
    for row in rows.iter() {
        let mut data_deserializer = DataDeserializer::new(&names, &row.columns);

        let data = D::deserialize(&mut data_deserializer)?;

        data_set.push(data);
    }

    Ok(data_set)
}
