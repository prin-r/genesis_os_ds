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
    value: String
}

#[no_mangle]
fn prepare_impl(input: Input) {
    oei::ask_external_data(
        1,
        match input.category.as_str() {
            "football" => 20,
            "basketball" => 54,
            _ => 0
        },
        format!("{} {}", input.date, input.contest_id,).as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(_: Input) -> Output {
    Output {
        value: ext::load_majority::<String>(1).unwrap()
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
            "{category:string,date:string,contest_id:string}/{value:string}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
