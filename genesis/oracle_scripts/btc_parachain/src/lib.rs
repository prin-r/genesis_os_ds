#![feature(slice_partition_at_index)]
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

// This function assumed that the size of the input array must be greater than zero.
fn median_u32(arr: &mut Vec<u32>) -> u32 {
    let mid = arr.len() / 2;
    let (_, median, _) = arr.partition_at_index_by(mid, |a, b| a.partial_cmp(b).unwrap());
    median.clone()
}

#[no_mangle]
fn prepare_impl(input: Input) {
    let btc_fees_ds_id = 21;
    let secondary_ds_id = 31;
    for (i, symbol) in input.symbols.iter().enumerate() {
        let base_quote = symbol.split("_").map(|x| x.into()).collect::<Vec<String>>();
        match &base_quote[1][..] {
            "F" | "HH" | "H" => oei::ask_external_data(i as i64, btc_fees_ds_id, symbol.as_bytes()),
            _ => oei::ask_external_data(i as i64, secondary_ds_id, symbol.as_bytes())
        }
    }
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let mut rates:Vec<u64> = Vec::new();
    for (i, _symbol) in input.symbols.iter().enumerate() {
        let rate: f64 = (ext::load_median(i as i64)).unwrap();
        rates.push((rate * (input.multiplier as f64)) as u64);
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
