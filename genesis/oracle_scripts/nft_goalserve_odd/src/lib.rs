use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    category: String,
    date1: String,
    date2: String,
    contest_id: String,
    odds_type: String,
    bookmaker_id: String,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    value1: u64,
    dp1: u64,
    value2: u64,
    dp2: u64,
    err_msg: String,
}

#[no_mangle]
fn prepare_impl(input: Input) {
    // nft data source
    oei::ask_external_data(
        1,
        13,
        format!(
            "{} {} {} {} {} {}",
            input.category,
            input.date1,
            input.date2,
            input.contest_id,
            input.odds_type,
            input.bookmaker_id
        )
        .as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let raw_opt = ext::load_majority::<String>(1);
    let mut output = Output {
        value1: 0,
        dp1: 0,
        value2: 0,
        dp2: 0,
        err_msg: "".into(),
    };
    match raw_opt {
        Some(raw) => {
            let values_opt = raw
                .split(" ")
                .map(|x| x.parse().ok())
                .collect::<Option<Vec<f64>>>();

            match values_opt {
                Some(values) => {
                    let m = input.multiplier;
                    output.dp1 = (values[0] * (m as f64)) as u64;
                    output.value1 = (values[1] * (m as f64)) as u64;
                    output.dp2 = (values[2] * (m as f64)) as u64;
                    output.value2 = (values[3] * (m as f64)) as u64;
                    output
                }
                None => {
                    output.err_msg = "Fail to parse float for some value".into();
                    output
                }
            }
        }
        None => {
            output.err_msg = "Fail to load majority".into();
            output
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
            "{category:string,date1:string,date2:string,contest_id:string,odds_type:string,bookmaker_id:string,multiplier:u64}/{value1:u64,dp1:u64,value2:u64,dp2:u64,err_msg:string}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
