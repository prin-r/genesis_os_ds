#![feature(slice_partition_at_index)]
use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    // TODO: remove this later
    _unused: u8,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    fastest: u32,
    half_hour: u32,
    hour: u32,
    err_msg: String,
}

// This function assumed that the size of the input array must be greater than zero.
fn median_u32(arr: &mut Vec<u32>) -> u32 {
    let mid = arr.len() / 2;
    let (_, median, _) = arr.partition_at_index_by(mid, |a, b| a.partial_cmp(b).unwrap());
    median.clone()
}

#[no_mangle]
fn prepare_impl(_: Input) {
    // Bitcoin fees data source
    oei::ask_external_data(1, 21, "".as_bytes());
}

#[no_mangle]
fn execute_impl(_: Input) -> Output {
    let raw = ext::load_input::<String>(1);
    let fees_opt = raw
        .iter()
        .map(|x0| {
            x0.split(" ")
                .map(|x1| x1.parse().ok())
                .collect::<Option<Vec<u32>>>()
                .map(|y| if y.len() == 3 { Some(y) } else { None })
                .flatten()
        })
        .collect::<Option<Vec<Vec<u32>>>>();

    let mut out = Output {
        fastest: 0,
        half_hour: 0,
        hour: 0,
        err_msg: "".into(),
    };

    match fees_opt {
        Some(fees) => {
            let l = fees.len();
            if l == 0 {
                return out;
            }
            let mut ft = vec![0; l];
            let mut hh = vec![0; l];
            let mut h = vec![0; l];
            for i in 0..l {
                ft[i] = fees[i][0];
                hh[i] = fees[i][1];
                h[i] = fees[i][2];
            }
            out.fastest = median_u32(&mut ft);
            out.half_hour = median_u32(&mut hh);
            out.hour = median_u32(&mut h);
            out
        }
        None => {
            out.err_msg = "Fail to parse some output".into();
            out
        }
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
            "{_unused:u8}/{fastest:u64,half_hour:u64,hour:u64,err_msg:string}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
