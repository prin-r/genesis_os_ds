### Genesis data sources and oracle scripts

## Prepare environment

1. Install Python 3.0
2. Install Rust
3. run `cd oracle_scripts/bitcoin_block_count/`
4. run `wasm-pack build .`
5. run `cd ../..`
6. run `chmod +x scripts/os_to_wasm.sh` to change the access permission of os_to_wasm.script
7. run `./scripts/os_to_wasm.sh`
8. run `cd scripts`
9. create virtual environment for python `phthon3 -m venv venv
10. source new env by `source venv/bin/activate`
11. install dependency `pip install -r requirements.txt`
12. run `python gen_os_ds.py <relative_path_to_genesis_directory>` then you will get new add_os_ds.sh in scripts folder

## How to add new data source

1. Add new data source script to datasources folder
2. Add new data source to mapping.json
3. run `cd scripts`
4. run `python3 gen_os_ds.py` then you will get new add_os_ds.sh in scripts folder

## How to add new oracle script

1. Add new oracle script to oracle_scripts folder
2. Implement test for get schema in lib.rs file

Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
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
            "{symbol:string,multiplier:u64}/{volume:u64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}
```

3. run `chmod +x scripts/os_to_wasm.sh` to change the access permission of os_to_wasm.script
4. run `./scripts/os_to_wasm.sh`
5. run `cd scripts`
6. run `python3 gen_os_ds.py` then you will get new add_os_ds.sh in scripts folder
