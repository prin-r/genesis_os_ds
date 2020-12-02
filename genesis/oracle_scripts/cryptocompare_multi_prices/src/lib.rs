use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    symbol: String,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    px_krw: u64,
    px_usd: u64,
    px_mnt: u64,
    px_sdr: u64,
}

#[no_mangle]
fn prepare_impl(input: Input) {
    // cryptocompare_krw_usd_prices data source
    oei::ask_external_data(1, 62, &input.symbol.as_bytes());

    oei::ask_external_data(21, 63, "XDR USD".as_bytes());
    oei::ask_external_data(22, 63, "XDR KRW".as_bytes());
    oei::ask_external_data(31, 63, "MNT USD".as_bytes());
    oei::ask_external_data(32, 63, "MNT KRW".as_bytes());
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let multiplier: f64 = input.multiplier as f64;
    let raw: Vec<String> = ext::load_input::<String>(1);
    let luna_vec: Vec<f64> = raw
        .iter()
        .map(|x| {
            x.split(" ")
                .map(|xx| xx.parse().unwrap())
                .collect::<Vec<f64>>()
        })
        .fold(vec![0., 0.], |a, b| vec![a[0] + b[0], a[1] + b[1]])
        .iter()
        .map(|x| x / (raw.len() as f64))
        .collect::<Vec<f64>>();

    let luna_krw = luna_vec[0];
    let luna_usd = luna_vec[1];

    let usd_sdr = ext::load_median::<f64>(21).unwrap();
    let krw_sdr = ext::load_median::<f64>(22).unwrap();
    let usd_mnt = ext::load_median::<f64>(31).unwrap();
    let krw_mnt = ext::load_median::<f64>(32).unwrap();

    Output {
        px_krw: (luna_krw * (multiplier as f64)) as u64,
        px_usd: (luna_usd * (multiplier as f64)) as u64,
        px_mnt: ((luna_krw / krw_mnt + luna_usd / usd_mnt) * (0.5 * multiplier as f64)) as u64,
        px_sdr: ((luna_krw / krw_sdr + luna_usd / usd_sdr) * (0.5 * multiplier as f64)) as u64,
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
            "{symbol:string,multiplier:u64}/{px_krw:u64,px_usd:u64,px_mnt:u64,px_sdr:u64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
