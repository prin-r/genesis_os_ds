use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

const CRYPTO_PRICE_FROM_URL: i64 = 55;

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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
  T: Clone,
{
  assert!(!v.is_empty());
  (0..v[0].len())
    .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
    .collect()
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

fn transform_symbol(symbol: String) -> String {
  match &symbol.clone()[..] {
    "BTCB" => "BTC".into(),
    "BETH" => "ETH".into(),
    _ => panic!("Unsupported symbol {}", symbol),
  }
}

#[no_mangle]
fn prepare_impl(input: Input) {
  for (i, url) in urls().iter().enumerate() {
    oei::ask_external_data(
      i as i64,
      CRYPTO_PRICE_FROM_URL,
      format!(
        "{} {}",
        url,
        input
          .symbols
          .iter()
          .map(|s| transform_symbol(s.clone()).clone())
          .collect::<Vec<String>>()
          .join(" ")
      )
      .as_bytes(),
    )
  }
}

fn aggregate_data(multiplier: u64, uvs_str: &Vec<Vec<String>>) -> Vec<u64> {
  let uvs: Vec<Vec<Vec<f64>>> = uvs_str
    .iter()
    .map(|v| {
      v.iter()
        .map(|s| {
          s.split(" ")
            .map(|ss| ss.parse::<f64>().ok())
            .collect::<Option<Vec<f64>>>()
        })
        .map(|s| s.unwrap_or(vec![]))
        .collect()
    })
    .collect();

  let mut acc: Vec<Vec<f64>> = vec![];
  for u in uvs.iter() {
    for v in u.iter() {
      if !v.is_empty() {
        acc.push(v.clone());
      }
    }
  }

  transpose(acc)
    .iter_mut()
    .map(|x| (median_float(x) * (multiplier as f64)) as u64)
    .collect()
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
  let urls_validators_symbols: Vec<Vec<String>> = urls()
    .iter()
    .enumerate()
    .map(|(i, _)| ext::load_input::<String>(i as i64))
    .collect();

  Output {
    rates: aggregate_data(input.multiplier, &urls_validators_symbols),
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
    // assert_eq!(
    //   "{symbols:[string],multiplier:u64}/{rates:[u64]}",
    //   format!("{}/{}", input_schema, output_schema),
    // );
  }

  #[test]
  fn test_ad_1() {
    let data = vec![vec!["1.0 2.0".into()]];
    let output = aggregate_data(1_000_000, &data);

    assert_eq!(output, vec![1_000_000, 2_000_000]);
  }

  #[test]
  fn test_ad_2() {
    let data = vec![vec!["1.0 2.0 3.0".into(), "None".into()]];
    let output = aggregate_data(1_000_000, &data);

    assert_eq!(output, vec![1_000_000, 2_000_000, 3_000_000]);
  }

  #[test]
  fn test_ad_3() {
    let data = vec![vec!["1.0 2.0 3.0".into(), "a b c".into()]];
    let output = aggregate_data(1_000_000, &data);

    assert_eq!(output, vec![1_000_000, 2_000_000, 3_000_000]);
  }

  #[test]
  fn test_ad_4() {
    let data = vec![vec!["1.0 2.0".into()], vec!["3.0 4.0".into()]];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![2000, 3000]);
  }

  #[test]
  fn test_ad_5() {
    let data = vec![
      vec!["1.0 2.0 3.0".into()],
      vec!["4.0 5.0 6.0".into()],
      vec!["7.0 8.0 9.0".into()],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![4000, 5000, 6000]);
  }

  #[test]
  fn test_ad_6() {
    let data = vec![
      vec![
        "1.0 2.0 3.0".into(),
        "4.0 5.0 6.0".into(),
        "7.0 8.0 9.0".into(),
      ],
      vec![
        "11.0 12.0 13.0".into(),
        "14.0 15.0 16.0".into(),
        "17.0 18.0 19.0".into(),
      ],
      vec![
        "21.0 22.0 23.0".into(),
        "24.0 25.0 26.0".into(),
        "27.0 28.0 29.0".into(),
      ],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![14_000, 15_000, 16_000]);
  }

  #[test]
  fn test_ad_7() {
    let data = vec![
      vec![
        "7.0 8.0 9.0".into(),
        "4.0 5.0 6.0".into(),
        "27.0 28.0 29.0".into(),
      ],
      vec![
        "11.0 12.0 13.0".into(),
        "21.0 22.0 23.0".into(),
        "17.0 18.0 19.0".into(),
      ],
      vec![
        "14.0 15.0 16.0".into(),
        "24.0 25.0 26.0".into(),
        "1.0 2.0 3.0".into(),
      ],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![14_000, 15_000, 16_000]);
  }

  #[test]
  fn test_ad_8() {
    let data = vec![
      vec![
        "None".into(),
        "17.0 18.0 19.0".into(),
        "27.0 28.0 29.0".into(),
      ],
      vec![
        "11.0 12.0 13.0".into(),
        "Extra data: line 1 column 5 (char 4)".into(),
        "4.0 5.0 6.0".into(),
      ],
      vec![
        "14.0 15.0 16.0".into(),
        "24.0 25.0 26.0".into(),
        "'result'".into(),
      ],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![15_500, 16_500, 17_500]);
  }

  #[test]
  fn test_ad_9() {
    let data = vec![
      vec![
        "None 1.0 2.0".into(),
        "4.0 5.0 6.0".into(),
        "7.0 8.0 9.0".into(),
      ],
      vec![
        "11.0 12.0 13.0".into(),
        "14.0 None 16.0".into(),
        "17.0 18.0 19.0".into(),
      ],
      vec![
        "21.0 22.0 23.0".into(),
        "24.0 25.0 26.0".into(),
        "27.0 28.0 None".into(),
      ],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![14_000, 15_000, 16_000]);
  }

  #[test]
  fn test_ad_10() {
    let data = vec![
      vec!["{ ø }".into(), "{ ø }".into(), "{ ø }".into()],
      vec![
        "11.0 12.0 13.0".into(),
        "14.0 15.0 16.0".into(),
        "17.0 18.0 19.0".into(),
      ],
      vec![
        "21.0 22.0 23.0".into(),
        "24.0 25.0 26.0".into(),
        "27.0 28.0 29.0".into(),
      ],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![19_000, 20_000, 21_000]);
  }

  #[test]
  fn test_ad_11() {
    let data = vec![
      vec!["1.0 2.0 3.0".into(), "{ ø }".into(), "{ ø }".into()],
      vec![
        "11.0 12.0 13.0".into(),
        "14.0 15.0 16.0".into(),
        "17.0 18.0 19.0".into(),
      ],
      vec!["{ ø }".into(), "{ ø }".into(), "20.0 21.0 22.0".into()],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![14_000, 15_000, 16_000]);
  }

  #[test]
  fn test_ad_12() {
    let data = vec![
      vec!["{ ø }".into(), "{ ø }".into(), "{ ø }".into()],
      vec![
        "11.0 12.0 13.0".into(),
        "14.0 15.0 16.0".into(),
        "17.0 18.0 19.0".into(),
      ],
      vec!["{ ø }".into(), "{ ø }".into(), "{ ø }".into()],
    ];
    let output = aggregate_data(1_000, &data);

    assert_eq!(output, vec![14_000, 15_000, 16_000]);
  }
}
