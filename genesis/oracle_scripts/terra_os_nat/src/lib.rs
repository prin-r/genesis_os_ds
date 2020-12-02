#![feature(slice_partition_at_index)]
use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    exchanges: Vec<String>,
    base_symbol: String,
    quote_symbol: String,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema, PartialEq, Debug)]
struct OrderBook {
    ask: i64,
    bid: i64,
    mid: i64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    order_books: Vec<OrderBook>,
}

const FX_ID_OFFSET: i64 = 999_999_999;
const FX_DS: [i64; 3] = [9, 10, 12];
const EXCHANGE_ORDER_BOOK_DS: i64 = 17;
const EXS: &'static [&'static str] = &["huobipro", "binance"];

fn prepare_impl(input: Input) {
    for i in 0..input.exchanges.len() {
        let mut quote = input.quote_symbol.clone();
        if EXS.iter().any(|&s| String::from(s) == input.exchanges[i]) {
            quote = String::from("USDT");
        }
        oei::ask_external_data(
            i as i64,
            EXCHANGE_ORDER_BOOK_DS,
            format!("{} {} {}", input.exchanges[i], input.base_symbol, quote).as_bytes(),
        );
    }
    for &i in &FX_DS {
        oei::ask_external_data(FX_ID_OFFSET + i, i, "KRW".as_bytes());
    }
}

// This function assumed that the size of the input array must be greater than zero.
fn median_float(arr: &mut Vec<f64>) -> f64 {
    let mid = arr.len() / 2;
    let (_, median, _) = arr.partition_at_index_by(mid, |a, b| a.partial_cmp(b).unwrap());
    median.clone()
}

fn ds_output_to_ask_bid(
    arr: Vec<String>,
    multiplier: u64,
    exchange: String,
    med_krw: f64,
) -> OrderBook {
    let (mut asks, mut bids): (Vec<_>, Vec<_>) = arr
        .iter()
        .map(|x| {
            x.split(",")
                .map(|y| y.parse().ok())
                .collect::<Option<Vec<f64>>>()
                .unwrap_or(vec![])
        })
        .filter(|x| x.len() == 2)
        .map(|x| (x[0], x[1]))
        .unzip();

    if asks.len() > 0 {
        let ma = median_float(&mut asks);
        let mb = median_float(&mut bids);
        let mul = match EXS.iter().any(|&s| String::from(s) == exchange) {
            true => multiplier as f64 / med_krw,
            false => multiplier as f64,
        };
        OrderBook {
            ask: (ma * mul as f64) as i64,
            bid: (mb * mul as f64) as i64,
            mid: (((ma * mul + mb * mul) as f64) as i64) / 2,
        }
    } else {
        OrderBook {
            ask: -1,
            bid: -1,
            mid: -1,
        }
    }
}

fn execute_impl(input: Input) -> Output {
    let mut krws = vec![];
    for &i in &FX_DS {
        match ext::load_median(FX_ID_OFFSET + i) {
            Some(krw_usd) => krws.push(krw_usd),
            None => (),
        }
    }
    let med_krw = median_float(&mut krws);

    return Output {
        order_books: input
            .exchanges
            .iter()
            .enumerate()
            .map(|(i, _)| {
                ds_output_to_ask_bid(
                    ext::load_input(i as i64),
                    input.multiplier,
                    input.exchanges[i].clone(),
                    med_krw,
                )
            })
            .collect(),
    };
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
            "{exchanges:[string],base_symbol:string,quote_symbol:string,multiplier:u64}/{order_books:[{ask:i64,bid:i64,mid:i64}]}",
            format!("{}/{}", input_schema, output_schema),
        );
    }

    #[test]
    fn test_median_float() {
        assert_eq!(5.03, median_float(&mut vec![5.03]));
        assert_eq!(68.53, median_float(&mut vec![68.53, 45.19]));
        assert_eq!(22.34, median_float(&mut vec![22.34, 21.83, 77.46]));
        assert_eq!(
            71.71,
            median_float(&mut vec![
                11.25, 55.45, 13.4, 71.71, 28.45, 94.51, 63.84, 79.33, 87.41, 75.47
            ])
        );
    }

    #[test]
    fn test_ds_output_to_ask_bid() {
        // success
        assert_eq!(
            OrderBook {
                ask: 1233,
                bid: 1165,
                mid: 1199
            },
            ds_output_to_ask_bid(vec!["12.33,11.65".into()], 100, "coinone".into(), 0.0)
        );
        assert_eq!(
            OrderBook {
                ask: 5457,
                bid: 3024,
                mid: (5457 + 3024) / 2
            },
            ds_output_to_ask_bid(
                vec!["29.01,30.24".into(), "54.57,17.27".into()],
                100,
                "coinone".into(),
                0.0
            )
        );
        assert_eq!(
            OrderBook {
                ask: 4903,
                bid: 4582,
                mid: (4903 + 4582) / 2
            },
            ds_output_to_ask_bid(
                vec![
                    "97.96,30.82".into(),
                    "35.94,68.08".into(),
                    "63.44,78.4".into(),
                    "46.86,45.82".into(),
                    "49.03,29.88".into()
                ],
                100,
                "coinone".into(),
                0.0
            )
        );

        // some fail
        assert_eq!(
            OrderBook {
                ask: 1233,
                bid: 1165,
                mid: 1199
            },
            ds_output_to_ask_bid(
                vec!["12.33,11.65".into(), "error1".into()],
                100,
                "coinone".into(),
                0.0
            )
        );
        assert_eq!(
            OrderBook {
                ask: 9571,
                bid: 7213,
                mid: (9571 + 7213) / 2
            },
            ds_output_to_ask_bid(
                vec!["error1".into(), "6.87,72.13".into(), "95.71,65.24".into()],
                100,
                "coinone".into(),
                0.0
            )
        );

        // all fail
        assert_eq!(
            OrderBook {
                ask: -1,
                bid: -1,
                mid: -1
            },
            ds_output_to_ask_bid(vec!["error1".into()], 100, "coinone".into(), 0.0)
        );
        assert_eq!(
            OrderBook {
                ask: -1,
                bid: -1,
                mid: -1
            },
            ds_output_to_ask_bid(
                vec!["error1".into(), "error2".into()],
                100,
                "coinone".into(),
                0.0
            )
        );
    }
}
