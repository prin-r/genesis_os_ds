use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

use std::collections::hash_map::*;
use std::collections::HashMap;

#[derive(OBIDecode, OBISchema)]
struct Input {
  symbols: Vec<String>,
  multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
  rates: Vec<u64>,
}

const EXCHANGE_COUNT: u64 = 2;

// Token Indices
const HB10: u64 = 0;
const IIXCI: u64 = 1;
const IXBCI: u64 = 2;

// Source Indices
const HUOBIPRO: u64 = 0;
const XANGLE: u64 = 1;

// Data Source Indices
const CCXT_DS: i64 = 6;
const XANGLE_DS: i64 = 23;

fn symbol_string_to_id(symbol_string: String) -> u64 {
  match symbol_string.as_str() {
    "HB10" => HB10,
    "IIXCI" => IIXCI,
    "IXBCI" => IXBCI,
    _ => panic!(symbol_string),
  }
}

fn symbol_id_to_string(symbol_id: u64) -> String {
  match symbol_id {
    HB10 => "HB10".to_string(),
    IIXCI => "IIXCI".to_string(),
    IXBCI => "IXBCI".to_string(),
    _ => panic!("Unsupported symbol ID!"),
  }
}

fn symbol_id_to_exchange_list(symbol_id: u64) -> Vec<bool> {
  match symbol_id {
    HB10 => vec![true,false],
    IIXCI => vec![false,true],
    IXBCI => vec![false,true],
    _ => panic!("Unsupported symbol ID!"),
  }
}

fn get_symbols_from_ids(ids: Vec<u64>) -> Vec<String> {
  let mut symbol_string: Vec<String> = Vec::new();
  for id in &ids {
    let index = symbol_id_to_string(*id);
    symbol_string.push(index.to_string())
  }
  symbol_string
}

fn get_ds_input(exchange_id: u64, symbols: Vec<u64>) -> String {
  match exchange_id {
    HUOBIPRO => format!("huobipro {}", get_symbols_from_ids(symbols).join(" ")),
    XANGLE => format!("{}", get_symbols_from_ids(symbols).join(" ")),
    _ => panic!("Unsupported Exchange"),
  }
}

fn get_ds_from_exchange(exchange_id: u64) -> i64 {
  match exchange_id {
    HUOBIPRO => CCXT_DS,
    XANGLE => XANGLE_DS,
    _ => panic!("Unsupported Exchange ID"),
  }
}

// Get list of exchange that needs to be called along with the symbols to call
// given a list of input symbols
fn get_exchange_map(symbols: Vec<String>) -> HashMap<u64, Vec<u64>> {
  let mut exchange_map = HashMap::new();
  for symbol in symbols {
    let symbol_index = symbol_string_to_id((&&symbol.as_str()).to_string());
    for i in 0..(EXCHANGE_COUNT as usize) {
      if symbol_id_to_exchange_list(symbol_index)[i] {
        match exchange_map.entry(i as u64) {
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
  exchange_map
}

fn median(arr: &mut Vec<f64>) -> f64 {
  let len_arr = arr.len() as f64;
  if len_arr > 0f64 {
    arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = len_arr / 2f64;
    if len_arr as u64 % 2 == 0 {
      (arr[(mid - 1f64) as usize] + arr[mid as usize]) / 2f64
    } else {
      arr[mid as usize]
    }
  } else {
    0f64
  }
}

// Get the list of symbol strings from data source calldata input
fn get_symbols_from_input(exchange_id: u64, input: String) -> Vec<String> {
  match exchange_id {
    XANGLE => input.split(" ").map(|x| x.to_string()).collect(),
    _ => {
      let mut v: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();
      v.drain(0..1);
      v
    }
  }
}

fn prepare_impl(input: Input) {
  let exchange_map = get_exchange_map(input.symbols);
  for (exchange_id, symbols) in exchange_map.iter() {
    oei::ask_external_data(
      *exchange_id as i64,
      get_ds_from_exchange(*exchange_id),
      get_ds_input(*exchange_id, symbols.to_vec()).as_bytes(),
    )
  }
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
  // Get the required exchange and associated symbols to query
  let exchange_map = get_exchange_map((*input.symbols).to_vec());
  // store the median price of each token requested from an exchange
  let mut exchange_medians: Vec<Option<Vec<f64>>> = vec![Some(vec![]); EXCHANGE_COUNT as usize];
  for (exchange_id, symbols) in exchange_map.iter() {
    // Get the data source calldata for a given external ID
    let raw_input = ext::load_input::<String>(*exchange_id as i64);
    let mut prices = vec![vec![]; exchange_map[exchange_id].len()];
    let inputs: Vec<String> = raw_input.clone();
    if (inputs.len() == 0) {
      exchange_medians[*exchange_id as usize] = None;
      continue;
    }
    // for each validator response for the exchange,
    // split the response into individual prices
    for raw in inputs {
      let px_list: Vec<f64> = raw
        .split(",")
        .filter_map(|x| x.parse::<f64>().ok())
        .collect();
      // for each token price, add it to the list of validator responses
      // for that token and exchange
      for (idx, &px) in px_list.iter().enumerate() {
        prices[idx].push(px);
      }
    }
    let mut median_prices = vec![0f64; prices.len()];
    for (idx, price) in prices.iter().enumerate() {
      median_prices[idx] = median(&mut price.to_vec());
    }
    exchange_medians[*exchange_id as usize] = Some(median_prices);
  }

  let mut symbol_pxs = HashMap::new();
  for (exchange_id, symbols) in exchange_map.iter() {
    let exchange_median = exchange_medians[*exchange_id as usize].as_ref();
    if (exchange_median.is_none()) {
      continue;
    }
    let exchange_median = exchange_median.unwrap();
    let symbols_vec =
      get_symbols_from_input(*exchange_id, get_ds_input(*exchange_id, symbols.to_vec()));

    for (symbol_id, symbol) in symbols_vec.iter().enumerate() {
      match symbol_pxs.entry(symbol.clone()) {
        Entry::Vacant(e) => {
          e.insert(vec![exchange_median[symbol_id]]);
        }
        Entry::Occupied(mut e) => {
          e.get_mut().push(exchange_median[symbol_id]);
        }
      }
    }
  }

  let mut rates = Vec::new();
  for symbol in input.symbols.iter() {
    rates.push((median(symbol_pxs.get_mut(*&symbol).unwrap()) * (input.multiplier as f64)) as u64)
  }
  Output { rates: rates }
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
            "{symbols:[string],multiplier:u64}/{rates:[u64]:[u64]}",
            format!("{}/{}", input_schema, output_schema),
        );
    }

    // #[test]
    // fn tttt() {
    //   let input = Input {
    //     symbols: vec!["HB10".into(),"IIXCI".into()],
    //     multiplier: 1000000,
    //   };
    //   let exchange_map = get_exchange_map((*input.symbols).to_vec());
    //   // store the median price of each token requested from an exchange
    //   let mut exchange_medians: Vec<Option<Vec<f64>>> = vec![Some(vec![]); EXCHANGE_COUNT as usize];
    //   let xxx = vec![
    //     vec![String::from("0.9716")],
    //     vec![String::from("268.61859149134654")]
    //   ];
    //   let mut i = 0;
    //   for (exchange_id, symbols) in exchange_map.iter() {
    //     // Get the data source calldata for a given external ID
    //     let raw_input = xxx[i].clone();
    //     i = i + 1;
    //     let mut prices = vec![vec![]; exchange_map[exchange_id].len()];
    //     let inputs: Vec<String> = raw_input.clone();
    //     if (inputs.len() == 0) {
    //       exchange_medians[*exchange_id as usize] = None;
    //       continue;
    //     }
    //     // for each validator response for the exchange,
    //     // split the response into individual prices
    //     for raw in inputs {
    //       let px_list: Vec<f64> = raw
    //         .split(",")
    //         .filter_map(|x| x.parse::<f64>().ok())
    //         .collect();
    //       // for each token price, add it to the list of validator responses
    //       // for that token and exchange
    //       for (idx, &px) in px_list.iter().enumerate() {
    //         prices[idx].push(px);
    //       }
    //     }
    //     let mut median_prices = vec![0f64; prices.len()];
    //     for (idx, price) in prices.iter().enumerate() {
    //       median_prices[idx] = median(&mut price.to_vec());
    //     }
    //     exchange_medians[*exchange_id as usize] = Some(median_prices);
    //   }

    //   println!("{:?}", exchange_map);

    //   let mut symbol_pxs = HashMap::new();
    //   for (exchange_id, symbols) in exchange_map.iter() {
    //     let exchange_median = exchange_medians[*exchange_id as usize].as_ref();
    //     if (exchange_median.is_none()) {
    //       continue;
    //     }
    //     let exchange_median = exchange_median.unwrap();
    //     let symbols_vec =
    //       get_symbols_from_input(*exchange_id, get_ds_input(*exchange_id, symbols.to_vec()));

    //     println!("ds_input {:?}", get_ds_input(*exchange_id, symbols.to_vec()));
    //     println!("symbols_vec {:?}", symbols_vec);

    //     for (symbol_id, symbol) in symbols_vec.iter().enumerate() {
    //       match symbol_pxs.entry(symbol.clone()) {
    //         Entry::Vacant(e) => {
    //           e.insert(vec![exchange_median[symbol_id]]);
    //         }
    //         Entry::Occupied(mut e) => {
    //           e.get_mut().push(exchange_median[symbol_id]);
    //         }
    //       }
    //     }
    //   }

    //   let mut rates = Vec::new();
    //   for symbol in input.symbols.iter() {
    //     println!(">>>>>>>>>>>");
    //     println!("{:?}", symbol_pxs);
    //     rates.push((median(symbol_pxs.get_mut(*&symbol).unwrap()) * (input.multiplier as f64)) as u64)
    //   }
    // }
}

