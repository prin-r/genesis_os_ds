use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    symbols: Vec<String>,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    rates: Vec<u64>
}

#[no_mangle]
fn prepare_impl(input: Input) {
    let twap_ds_id = 22;
    for (i, symbol) in input.symbols.iter().enumerate() {
        oei::ask_external_data(
            i as i64,
            twap_ds_id,
            &format!("{} {}", symbol, "USDT").as_bytes()
        );
    }
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let multiplier: f64 = input.multiplier as f64;
    let mut rates = vec![0; input.symbols.len()];
    for (i, _symbol) in input.symbols.iter().enumerate() {
        let avg: f64 = ext::load_median::<f64>(i as i64).unwrap();
        rates[i] = (avg * multiplier as f64) as u64;
    }
    Output { rates }
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
