use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    category: String,
    date: String,
    contest_id: String,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    status: String,
    hometeam_total_score: u64,
    awayteam_total_score: u64,
    err_msg: String,
}

#[no_mangle]
fn prepare_impl(input: Input) {
    // nft data source
    oei::ask_external_data(
        1,
        20,
        format!("{} {} {}", input.category, input.date, input.contest_id,).as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(_: Input) -> Output {
    let raw_opt = ext::load_majority::<String>(1);
    let mut output = Output {
        status: "".into(),
        hometeam_total_score: 0,
        awayteam_total_score: 0,
        err_msg: "".into(),
    };
    match raw_opt {
        Some(raw) => {
            let values: Vec<&str> = raw.split(" ").collect();
            if values.len() != 3 {
                output.err_msg = "Fail to parse output".into();
                return output;
            }

            match (values[1].parse(), values[2].parse()) {
                (Ok(h), Ok(a)) => {
                    output.hometeam_total_score = h;
                    output.awayteam_total_score = a;
                    output.status = values[0].into();
                    output
                }
                _ => {
                    output.err_msg = "Fail to parse scores".into();
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
            "{category:string,date:string,contest_id:string}/{status:string,hometeam_total_score:u64,awayteam_total_score:u64,err_msg:string}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
