import os
import json
import sys
import requests
import subprocess


def gen_data_source_oracle_script(owner):
    dir = os.path.abspath(__file__)
    os_source_dir = os.path.abspath(
        os.path.join(dir, '../../oracle_scripts'))

    file = open("add_os_ds.sh", "w")
    file.write(f'DIR=`dirname "$0"`\n')

    with open("../mapping.json") as json_file:
        data = json.load(json_file)
        data_sources = data['data_sources']
        oracle_scripts = data['oracle_scripts']

        gen_data_source(file, data_sources, owner)
        gen_oracle_script(file, os_source_dir, oracle_scripts, owner)

    file.close()


def gen_data_source(file, data_sources, owner):
    for data_source in data_sources:
        name = data_source['name']
        description = data_source['description']
        filename = data_source['filename']
        path = f'$DIR/../datasources/{filename}'
        command_line = f'bandd add-data-source \\\n\t"{name}" \\\n\t"{description}" \\\n\t{owner} \\\n\t{path}\n\n'
        file.write(command_line)


def gen_oracle_script(file, os_source_dir, oracle_scripts, owner):
    for oracle_script in oracle_scripts:
        name = oracle_script['name']
        description = oracle_script['description']
        pakage = oracle_script['package']
        file_path = os.path.join(os_source_dir, pakage)
        schema = get_schema(file_path)
        url = get_url(os.path.join(
            os_source_dir, pakage, 'src', 'lib.rs'), pakage)
        path = f'$DIR/../res/{pakage}.wasm'
        command_line = f'bandd add-oracle-script \\\n\t"{name}" \\\n\t"{description}" \\\n\t"{schema}" \\\n\t"{url}" \\\n\t{owner} \\\n\t{path}\n\n'
        file.write(command_line)


def get_schema(path):
    pwd = os.getcwd()
    os.chdir(path)
    subprocess.check_output(
        'cargo test -- --nocapture | grep -v "^test" | grep -v "^running" | grep -v "^$" > schema.txt',
        stderr=subprocess.STDOUT,
        shell=True)

    with open("schema.txt", 'r') as file:
        schema = file.read().strip()
        os.remove("schema.txt")
        os.chdir(pwd)

        return schema


def get_url(file_path, filename):
    url = "https://api.pinata.cloud/pinning/pinFileToIPFS"
    headers = {
        'pinata_api_key': '5f7169a396725c53075b',
        'pinata_secret_api_key': '2fdca43a889df5602fd79dcc65a310de8a6ec85ec0aca6ab55a78aa1d45e7ce7'
    }
    payload = {'pinataMetadata': f'{{"name": "{filename}"}}',
               'pinataOptions': '{"cidVersion":0}'}

    files = [
        ('file', open(file_path, 'rb')),
    ]

    r = requests.request(
        "POST", url, headers=headers, data=payload, files=files)
    r.raise_for_status()
    return 'https://ipfs.io/ipfs/' + json.loads(r.content)['IpfsHash']


if __name__ == "__main__":
    owner = 'band1m5lq9u533qaya4q3nfyl6ulzqkpkhge9q8tpzs'
    gen_data_source_oracle_script(owner)
