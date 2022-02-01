use pyo3::prelude::*;
use serde_json::{Value};
//use serde_json::json; 
use crate::thick2ofn::thick_triple_parser::parse_triple as t2ofn; 
use crate::ofn_typing::ofn_parser::parse_ofn as typing; 
use crate::ofn_typing::typing::extract_typing as extract_typing; 
use crate::ofn_labeling::ofn_parser::parse_ofn as labeling; 
use crate::ofn_labeling::labeling::extract_labeling as extract_labeling; 
use crate::ofn2thick::ofn_parser::parse_ofn as ofn2t; 
use crate::ofn2man::parser::parse_ofn as ofn2man; 
use std::collections::HashMap;
use std::collections::HashSet;

#[pyfunction]
fn thick_2_ofn(t : &str) -> String { 

    let triple : Value = serde_json::from_str(t).unwrap();
    let ofn = t2ofn(triple.to_string().as_str());
    format!("{}", ofn)
}

#[pyfunction]
fn ofn_2_thick(t : &str) -> String { 

    ofn2t(t).to_string()
}

#[pyfunction] 
fn extract_types(t: &str) -> HashMap<String,HashSet<String>> { 
    extract_typing(t) 
}

#[pyfunction] 
fn ofn_typing(t: &str, m : HashMap<String, HashSet<String>>) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let typed = typing(&v, &m);
    format!("{}", typed) 
}

#[pyfunction] 
fn extract_labels(t: &str) -> HashMap<String,String> { 
    extract_labeling(t) 
}

#[pyfunction] 
fn ofn_labeling(t: &str, m : HashMap<String, String>) -> String { 

    let v : Value = serde_json::from_str(t).unwrap(); 
    let labeled = labeling(&v, &m);
    format!("{}", labeled) 
}

#[pyfunction] 
fn ofn_2_man(t: &str) -> String { 

    let man = ofn2man(t);
    format!("{}", man) 
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn wiring_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(thick_2_ofn, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_typing, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_labeling, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_2_man, m)?)?;
    m.add_function(wrap_pyfunction!(ofn_2_thick, m)?)?;
    m.add_function(wrap_pyfunction!(extract_labels, m)?)?;
    m.add_function(wrap_pyfunction!(extract_types, m)?)?;

    Ok(())
}
