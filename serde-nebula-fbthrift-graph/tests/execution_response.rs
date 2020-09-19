use std::io;

use nebula_fbthrift_graph::{
    types::{ColumnValue, ErrorCode, RowValue},
    ExecutionResponse,
};
use serde::Deserialize;

use serde_nebula_fbthrift_graph::de::data::DataDeserializeErrorKind;
use serde_nebula_fbthrift_graph::de::deserialize_execution_response;

#[derive(Deserialize, PartialEq, Debug)]
struct Foo {
    integer: i64,
    str: String,
}

#[test]
fn with_none_column_names() -> io::Result<()> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: None,
        rows: None,
        space_name: None,
        warning_msg: None,
    };

    match deserialize_execution_response::<Foo>(&execution_response) {
        Ok(_) => assert!(true, ""),
        Err(err) => {
            assert_eq!(err.field, None);
            assert_eq!(
                err.kind,
                DataDeserializeErrorKind::Custom("column_names is none or empty".to_owned())
            );
        }
    }

    Ok(())
}

#[test]
fn with_empty_column_names() -> io::Result<()> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![]),
        rows: None,
        space_name: None,
        warning_msg: None,
    };

    match deserialize_execution_response::<Foo>(&execution_response) {
        Ok(_) => assert!(true, ""),
        Err(err) => {
            assert_eq!(err.field, None);
            assert_eq!(
                err.kind,
                DataDeserializeErrorKind::Custom("column_names is none or empty".to_owned())
            );
        }
    }

    Ok(())
}

#[test]
fn with_none_rows() -> io::Result<()> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![b"integer".to_vec(), b"id".to_vec()]),
        rows: None,
        space_name: None,
        warning_msg: None,
    };

    match deserialize_execution_response::<Foo>(&execution_response) {
        Ok(_) => assert!(true, ""),
        Err(err) => {
            assert_eq!(err.field, None);
            assert_eq!(
                err.kind,
                DataDeserializeErrorKind::Custom("rows is none".to_owned())
            );
        }
    }

    Ok(())
}

#[test]
fn simple() -> io::Result<()> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![b"integer".to_vec(), b"id".to_vec()]),
        rows: Some(vec![
            RowValue {
                columns: vec![ColumnValue::integer(1), ColumnValue::str(b"1".to_vec())],
            },
            RowValue {
                columns: vec![ColumnValue::integer(2), ColumnValue::str(b"2".to_vec())],
            },
        ]),
        space_name: None,
        warning_msg: None,
    };

    match deserialize_execution_response::<Foo>(&execution_response) {
        Ok(foo_set) => {
            assert_eq!(foo_set.len(), 2);
            let foo_first = foo_set.first().unwrap();
            assert_eq!(foo_first.integer, 1);
            assert_eq!(foo_first.str, "1");
            let foo_last = foo_set.last().unwrap();
            assert_eq!(foo_last.integer, 2);
            assert_eq!(foo_last.str, "2");
        }
        Err(err) => assert!(true, err.to_string()),
    }

    Ok(())
}

#[test]
fn with_unit() -> io::Result<()> {
    let execution_response = ExecutionResponse {
        error_code: ErrorCode::SUCCEEDED,
        latency_in_us: 1,
        error_msg: None,
        column_names: Some(vec![b"integer".to_vec(), b"id".to_vec()]),
        rows: Some(vec![
            RowValue {
                columns: vec![ColumnValue::integer(1), ColumnValue::str(b"1".to_vec())],
            },
            RowValue {
                columns: vec![ColumnValue::integer(2), ColumnValue::str(b"2".to_vec())],
            },
        ]),
        space_name: None,
        warning_msg: None,
    };

    match deserialize_execution_response::<()>(&execution_response) {
        Ok(foo_set) => {
            assert_eq!(foo_set.len(), 2);
        }
        Err(err) => assert!(true, err.to_string()),
    }

    Ok(())
}
