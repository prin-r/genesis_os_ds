use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    flight_number: String,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    observations: u64,
    late15:u64,
    late30:u64,
    late45:u64,
    cancelled:u64,
    diverted:u64
}

#[no_mangle]
fn prepare_impl(input: Input) {
    // flight rate data source
    oei::ask_external_data(1, 80, input.flight_number.as_bytes());
}

#[no_mangle]
fn execute_impl(_: Input) -> Output {
    let majority = ext::load_majority::<String>(1);
    let values = majority.unwrap().split(",").map(|x| x.parse().unwrap()).collect::<Vec<u64>>();

    Output {
        observations: values[0],
        late15: values[1],
        late30: values[2],
        late45: values[3],
        cancelled: values[4],
        diverted: values[5]
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
            "{flight_number:string}/{observations:u64,late15:u64,late30:u64,late45:u64,cancelled:u64,diverted:u64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
