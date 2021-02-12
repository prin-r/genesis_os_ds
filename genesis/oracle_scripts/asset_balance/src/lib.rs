use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    platform: String,
    asset_address: String,
    holder_address: String,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    balance: u128,
}

#[no_mangle]
fn prepare_impl(input: Input) {
    let ds_id = match input.platform.as_str() {
        "waves" => 15,
        "ethereum" => 16,
        _ => panic!("Unknown platform"),
    };
    oei::ask_external_data(
        1,
        ds_id,
        format!("{} {}", input.asset_address, input.holder_address).as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(_: Input) -> Output {
    Output {
        balance: ext::load_majority::<u128>(1).unwrap(),
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
            "{platform:string,asset_address:string,holder_address:string}/{balance:u128}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
