#![feature(slice_partition_at_index)]
use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

const XBCI: i64 = 32;
const XLCI: i64 = 46;

#[derive(OBIDecode, OBISchema)]
struct Input {
  symbols: Vec<String>,
  multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
  rates: Vec<u64>,
}

fn urls() -> Vec<String> {
  vec![
    "https://us-rpc.bandchain.org/oracle/request_prices".into(),
    "https://eu-rpc.bandchain.org/oracle/request_prices".into(),
    "https://asia-rpc.bandchain.org/oracle/request_prices".into(),
    "https://aus-rpc.bandchain.org/oracle/request_prices".into(),
  ]
}

fn median_float(vals: &mut Vec<f64>) -> f64 {
  vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let mid = vals.len() / 2;
  if vals.len() % 2 == 0 {
    (vals[mid - 1].clone() + vals[mid].clone()) / 2f64
  } else {
    vals[mid].clone()
  }
}

fn call_all_urls(ds_id: i64) {
  for (i, url) in urls().iter().enumerate() {
    oei::ask_external_data((ds_id * 100) + (i as i64), ds_id, url.as_bytes())
  }
}

fn load_from_all_urls_and_median(ds_id: i64) -> f64 {
  median_float(
    &mut urls()
      .iter()
      .enumerate()
      .map(|(i, _)| ext::load_median::<f64>((ds_id * 100) + (i as i64)))
      .filter(|x| x.is_some())
      .map(|x| x.unwrap())
      .collect(),
  )
}

#[no_mangle]
fn prepare_impl(input: Input) {
  for symbol in input.symbols.iter() {
    match &symbol.clone()[..] {
      "XBCI" => call_all_urls(XBCI),
      "XLCI" => call_all_urls(XLCI),
      _ => panic!("UNKNOWN_SYMBOL_{}", symbol),
    }
  }
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
  Output {
    rates: input
      .symbols
      .iter()
      .map(|symbol| match &(symbol.clone())[..] {
        "XBCI" => (load_from_all_urls_and_median(XBCI) * (input.multiplier as f64)) as u64,
        "XLCI" => (load_from_all_urls_and_median(XLCI) * (input.multiplier as f64)) as u64,
        _ => panic!("UNKNOWN_SYMBOL_{}", symbol),
      })
      .collect(),
  }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);

#[cfg(test)]
mod tests {
  use super::*;
  use obi::get_schema;
  use std::collections::*;

  #[test]
  fn test_get_schema() {
    let mut schema = HashMap::new();
    Input::add_definitions_recursively(&mut schema);
    Output::add_definitions_recursively(&mut schema);
    let input_schema = get_schema(String::from("Input"), &schema);
    let output_schema = get_schema(String::from("Output"), &schema);
    println!("{}/{}", input_schema, output_schema);
    assert_eq!(
      "{symbols:[string],multiplier:u64}/{rates:[u64]}",
      format!("{}/{}", input_schema, output_schema),
    );
  }
}
