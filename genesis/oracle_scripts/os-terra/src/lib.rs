use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm::{prepare_entry_point, execute_entry_point, ext, oei};

#[derive(OBIDecode, OBISchema)]
struct Input {
    symbol: String,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    crypto_compare_usd: i64,
    coin_gecko_usd: i64,
    huobipro_usd: i64,
    bittrex_usd: i64,
    bithumb_krw: i64,
    coinone_krw: i64,
    coinmarketcap_usd: i64,
}

const CRYPTOCOMPARE: i64 = 1;
const COINGECKO: i64 = 2;
const CCXT: i64 = 3;
const USDUSDT: i64 = 4;
const ALPHAVANTAGE: i64 = 5;
const FIXER: i64 = 6;
const OXR: i64 = 7;
const XE: i64 = 8;
const COINMARKETCAP: i64 = 15;

fn prepare_impl(input: Input) {
    if input.symbol == "LUNA" {
        // LUNA
        oei::ask_external_data(11, CRYPTOCOMPARE, "LUNA USD".as_bytes());
        oei::ask_external_data(12, COINGECKO, "LUNA USD".as_bytes());
        oei::ask_external_data(13, CCXT, "huobipro LUNA USDT".as_bytes());
        oei::ask_external_data(14, CCXT, "bittrex LUNA USDT".as_bytes());
        oei::ask_external_data(15, CCXT, "bithumb LUNA KRW".as_bytes());
        oei::ask_external_data(16, CCXT, "coinone LUNA KRW".as_bytes());
        oei::ask_external_data(17, COINMARKETCAP, "LUNA".as_bytes());
        // USDT_USD
        oei::ask_external_data(21, USDUSDT, "".as_bytes());
    } else {
        panic!("UNSUPPORTED_SYMBOL");
    }
}

fn median_with_default(data: Vec<f64>) -> f64 {
    ext::stats::median_by(data, ext::cmp::fcmp::<f64>).unwrap_or(-1f64)
}

fn to_int_with_default_1(p: f64, multiplier: u64) -> i64 {
    if p < 0f64 {
        -1i64
    } else {
        (p * (multiplier as f64)) as i64
    }
}

fn to_int_with_default_2(p: f64, usdt_usd: f64, multiplier: u64) -> i64 {
    if p < 0f64 || usdt_usd <= 0f64 {
        -1i64
    } else {
        (p * (multiplier as f64) / usdt_usd) as i64
    }
}

fn execute_impl(input: Input) -> Output {
    let price_11 = median_with_default(ext::load_input(11).collect());
    let price_12 = median_with_default(ext::load_input(12).collect());
    let price_13 = median_with_default(ext::load_input(13).collect());
    let price_14 = median_with_default(ext::load_input(14).collect());
    let price_15 = median_with_default(ext::load_input(15).collect());
    let price_16 = median_with_default(ext::load_input(16).collect());
    let price_17 = median_with_default(ext::load_input(17).collect());

    let usdt_usd = median_with_default(ext::load_input(21).collect());

    Output {
        crypto_compare_usd: to_int_with_default_1(price_11, input.multiplier),
        coin_gecko_usd: to_int_with_default_1(price_12, input.multiplier),
        huobipro_usd: to_int_with_default_2(price_13, usdt_usd, input.multiplier),
        bittrex_usd: to_int_with_default_2(price_14, usdt_usd, input.multiplier),
        bithumb_krw: to_int_with_default_1(price_15, input.multiplier),
        coinone_krw: to_int_with_default_1(price_16, input.multiplier),
        coinmarketcap_usd: to_int_with_default_1(price_17, input.multiplier),
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
            "{symbol:string,multiplier:u64}/{crypto_compare_usd:i64,coin_gecko_usd:i64,huobipro_usd:i64,bittrex_usd:i64,bithumb_krw:i64,coinone_krw:i64,coinmarketcap_usd:i64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }

    #[test]
    fn test_median_with_default() {
        assert_eq!(median_with_default(vec![]),-1f64);
        assert_eq!(median_with_default(vec![0f64]),0f64);
        assert_eq!(median_with_default(vec![1f64,3f64]),2f64);
    }

    #[test]
    fn test_to_int_with_default_1() {
        assert_eq!(to_int_with_default_1(-1f64,100), -1i64);
        assert_eq!(to_int_with_default_1(median_with_default(vec![]),100), -1i64);
        assert_eq!(to_int_with_default_1(0f64,100), 0i64);
        assert_eq!(to_int_with_default_1(1.5f64,100), 150i64);
    }

    #[test]
    fn test_to_int_with_default_2() {
        assert_eq!(to_int_with_default_2(-1f64,1f64,100), -1i64);
        assert_eq!(to_int_with_default_2(median_with_default(vec![]),1f64,100), -1i64);
        assert_eq!(to_int_with_default_2(0f64,0f64,100), -1i64);
        assert_eq!(to_int_with_default_2(1.5f64,0f64,100), -1i64);
        assert_eq!(to_int_with_default_2(10f64,-1f64,100), -1i64);

        assert_eq!(to_int_with_default_2(10f64,1f64,100), 1000i64);
        assert_eq!(to_int_with_default_2(10f64,2f64,100), 500i64);
        assert_eq!(to_int_with_default_2(0f64,2f64,100), 0i64);
    }
}
