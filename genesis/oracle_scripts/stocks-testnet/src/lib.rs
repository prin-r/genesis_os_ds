use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm::{execute_entry_point, ext, oei, prepare_entry_point};
use std::collections::hash_map::*;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

#[derive(OBIDecode, OBISchema)]
struct Input {
  symbols: Vec<String>,
  multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
  rates: Vec<u64>,
}

// Tickers
const AAPL: u64 = 0;
const GOOGL: u64 = 1;
const TSLA: u64 = 2;
const NFLX: u64 = 3;
const QQQ: u64 = 4;
const TWTR: u64 = 5;
const BABA: u64 = 6;
const IAU: u64 = 7;
const SLV: u64 = 8;
const USO: u64 = 9;
const VIXY: u64 = 10;
const AMZN: u64 = 11;
const MSFT: u64 = 12;
const FB: u64 = 13;
const ABNB: u64 = 14;
const GS: u64 = 15;

// Sources
const IEXCLOUD: u64 = 0;
const FINAGE: u64 = 1;
const TWELVE: u64 = 2;

// Data Sources
const IEXCLOUD_DS: i64 = 14;
const FINAGE_DS: i64 = 48;
const TWELVE_DS: i64 = 52;

lazy_static! {
  static ref SYMBOL_STRING_TO_ID: HashMap<&'static str, u64> = {
    let mut m = HashMap::new();
    m.insert("AAPL", AAPL);
    m.insert("GOOGL", GOOGL);
    m.insert("TSLA", TSLA);
    m.insert("NFLX", NFLX);
    m.insert("QQQ", QQQ);
    m.insert("TWTR", TWTR);
    m.insert("BABA", BABA);
    m.insert("IAU", IAU);
    m.insert("SLV", SLV);
    m.insert("USO", USO);
    m.insert("VIXY", VIXY);
    m.insert("AMZN", AMZN);
    m.insert("MSFT", MSFT);
    m.insert("FB", FB);
    m.insert("ABNB", ABNB);
    m.insert("GS", GS);
    m
  };
}

lazy_static! {
  static ref SYMBOL_ID_TO_STRING: HashMap<u64, &'static str> = {
    let mut m = HashMap::new();
    m.insert(AAPL, "AAPL");
    m.insert(GOOGL, "GOOGL");
    m.insert(TSLA, "TSLA");
    m.insert(NFLX, "NFLX");
    m.insert(QQQ, "QQQ");
    m.insert(TWTR, "TWTR");
    m.insert(BABA, "BABA");
    m.insert(IAU, "IAU");
    m.insert(SLV, "SLV");
    m.insert(USO, "USO");
    m.insert(VIXY, "VIXY");
    m.insert(AMZN, "AMZN");
    m.insert(MSFT, "MSFT");
    m.insert(FB, "FB");
    m.insert(ABNB, "ABNB");
    m.insert(GS, "GS");
    m
  };
}

const AVAILABILITY_MATRIX: [[bool; 3]; 16] = [
  [true, true, true], // AAPL
  [true, true, true], // GOOGL
  [true, true, true], // TSLA
  [true, true, true], // NFLX
  [true, true, true], // QQQ
  [true, true, true], // TWTR
  [true, true, true], // BABA
  [true, true, true], // IAU
  [true, true, true], // SLV
  [true, true, true], // USO
  [true, true, true], // VIXY
  [true, true, true], // AMZN
  [true, true, true], // MSFT
  [true, true, true], // FB
  [true, true, true], // ABNB
  [true, true, true], // GS
];

fn get_symbols_from_ids(ids: Vec<u64>) -> Vec<String> {
  let mut symbol_string: Vec<String> = Vec::new();
  for id in &ids {
    let index = SYMBOL_ID_TO_STRING[id];
    symbol_string.push(index.to_string())
  }
  symbol_string
}

fn get_sources_list(symbols: Vec<String>) -> HashMap<u64, Vec<u64>> {
  let mut sources_map = HashMap::new();
  for symbol in symbols {
    let owned_symbol = &symbol.to_owned();
    let symbol_index = SYMBOL_STRING_TO_ID[&owned_symbol[..]];
    for i in 0..3 {
      if AVAILABILITY_MATRIX[symbol_index as usize][i as usize] {
        match sources_map.entry(i) {
          Entry::Vacant(e) => {
            e.insert(vec![symbol_index]);
          }
          Entry::Occupied(mut e) => {
            e.get_mut().push(symbol_index);
          }
        }
      }
    }
  }
  sources_map
}

fn get_ds_from_source(source_id: u64) -> i64 {
  match source_id {
    IEXCLOUD => IEXCLOUD_DS,
    FINAGE => FINAGE_DS,
    TWELVE => TWELVE_DS,
    _ => panic!("Unsupported Exchange ID"),
  }
}

fn get_calldata(symbols: Vec<u64>) -> String {
  format!("{}", get_symbols_from_ids(symbols).join(" "))
}

fn get_symbols_from_calldata(calldata: String) -> Vec<String> {
  calldata.split(" ").map(|x| x.to_string()).collect()
}

fn split_ds_result(ds_result: String) -> Option<Vec<f64>> {
  let split = ds_result.split(",");
  split.map(|x| x.parse().ok()).collect()
}

fn split_input(result: Vec<String>) -> Vec<Vec<f64>> {
  let mut inputs: Vec<Vec<f64>> = Vec::new();
  for res in result {
    let ds_opt = split_ds_result(res.to_string());
    match ds_opt {
      Some(val) => inputs.push(val.clone()),
      _ => (),
    }
  }
  inputs
}

fn len(arr: &Vec<f64>) -> f64 {
  let mut l = 0f64;
  for _ in arr.iter() {
    l += 1f64
  }
  l
}

fn only_positive(arr: Vec<f64>) -> Vec<f64> {
  arr
    .iter()
    .filter(|&&x| x > 0f64)
    .map(|&x| x)
    .collect::<Vec<_>>()
}

fn median(arr: Vec<f64>) -> f64 {
  let mut pos_arr = only_positive(arr);
  let len_pos_arr = len(&pos_arr);
  if len_pos_arr > 0f64 {
    pos_arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = len_pos_arr / 2f64;
    if len_pos_arr as u64 % 2 == 0 {
      (pos_arr[(mid - 1f64) as usize] + pos_arr[mid as usize]) / 2f64
    } else {
      pos_arr[mid as usize]
    }
  } else {
    0f64
  }
}

fn prepare_impl(input: Input) {
  let source_map = get_sources_list(input.symbols);
  for (source_id, symbols) in source_map.iter() {
    oei::ask_external_data(
      *source_id as i64,
      get_ds_from_source(*source_id),
      get_calldata(symbols.to_vec()).as_bytes(),
    )
  }
}

fn execute_impl(input: Input) -> Output {
  let mut pxs_map = HashMap::new();
  let exchange_map = get_sources_list(input.symbols.clone());
  for (exchange_id, symbols) in exchange_map.iter() {
    let symbols_vec = get_symbols_from_calldata(get_calldata(symbols.to_vec()));
    let raw_input = ext::load_input::<String>(*exchange_id as i64).collect::<Vec<String>>();
    let inputs = split_input(raw_input);
    for data in inputs {
      let x: Vec<(&String, &f64)> = symbols_vec.iter().zip(data.iter()).collect();
      for pair in x.clone() {
        match pxs_map.entry(pair.0.clone()) {
          Entry::Vacant(e) => {
            e.insert(vec![pair.1.clone()]);
          }
          Entry::Occupied(mut e) => {
            e.get_mut().push(pair.1.clone());
          }
        }
      }
    }
  }
  let mut med: Vec<u64> = Vec::new();
  for symbol in input.symbols.iter() {
    med.push((median(pxs_map[symbol].clone()) * (input.multiplier as f64)) as u64)
  }

  Output { rates: med }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
