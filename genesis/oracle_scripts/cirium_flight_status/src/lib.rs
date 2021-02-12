use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    flight_number: String,
    date: String,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    status: String,
    delay: i64,
}

#[no_mangle]
fn prepare_impl(input: Input) {
    // flight status data source
    oei::ask_external_data(1, 81, format!("{} {}", input.flight_number, input.date).as_bytes());
}

#[no_mangle]
fn execute_impl(_: Input) -> Output {
    let majority = ext::load_majority::<String>(1);
    let values = majority.unwrap().split(",").map(|x| x.to_string()).collect::<Vec<String>>();

    Output {
        status: values[0].clone(),
        delay: values[1].parse().unwrap(),
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
            "{flight_number:string,date:string}/{status:string,delay:i64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
