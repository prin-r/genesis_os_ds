use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm2::{prepare_entry_point, execute_entry_point, ext, oei};

#[derive(OBIDecode, OBISchema)]
struct Input {
    date: String,
    home_team: String,
    away_team: String,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    home_team_score: u32,
    away_team_score: u32,
}

#[no_mangle]
fn prepare_impl(input: Input) {
    let Input {
        date,
        home_team,
        away_team,
    } = input;
    // NBA rapid API data source
    oei::ask_external_data(1, 83, format!("{} {} {}", date, home_team, away_team).as_bytes());
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let majority = (ext::load_majority::<String>(1)).unwrap().split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
    Output {
        home_team_score: majority[0],
        away_team_score: majority[1],
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
            "{multiplier:u64}/{px:u64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
