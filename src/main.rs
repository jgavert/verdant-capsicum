extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use std::io::BufReader;
use std::io::Write;
use std::io::BufWriter;
use std::path::Path;
use std::fs::OpenOptions;

use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;

use std::string::String;
use std::collections::HashMap;

// read the request contents
//  - json file
//      - validate&errors&createdefault
//

// plan:

// try to find file
// try to read file
// try to parse file
// if any fails -> create file with default contents
// on success: return data ... in a class... wait. We auto serialize a class to
// be the default file! yes!


#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Item
{
    item_level: i32,
    req_level: i32,
    req_str: i32,
    req_dex: i32,
    req_int: i32,
    socket_info: String,
    armour: i32,
    evasion: i32,
    energy_shield: i32,
    max_life: i32,
    max_mana: i32,
    max_es: i32,
    str: i32,
    dex: i32,
    int: i32,
    fire_res: i32,
    cold_res: i32,
    lightning_res: i32,
    chaos_res: i32
}

impl Item {
    pub fn new() -> Item {
        Item {
            item_level: 1,
            req_level: 1,
            req_str: 0,
            req_dex: 0,
            req_int: 0,
            socket_info: String::from(""),
            armour: 0,
            evasion: 0,
            energy_shield: 0,
            max_life: 0,
            max_mana: 0,
            max_es: 0,
            str: 0,
            dex: 0,
            int: 0,
            fire_res: 0,
            cold_res: 0,
            lightning_res: 0,
            chaos_res: 0
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct PassiveTreeMods
{
    life_mod: i32,
    mana_mod: i32,
    es_mod: i32,
    armour_mod: i32,
    evasion_mod: i32
}

impl PassiveTreeMods {
    pub fn new() -> PassiveTreeMods {
        PassiveTreeMods {
            life_mod: 100,
            mana_mod: 100,
            es_mod: 100,
            armour_mod: 100,
            evasion_mod: 100
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct CharSpecs
{
    league: String,
    level: i32,
    str: i32,
    dex: i32,
    int: i32,
    mods: PassiveTreeMods,
    items: HashMap<String, Item>
}

impl CharSpecs {
    pub fn new() -> CharSpecs {
        let mut hmap: HashMap<String, Item> = HashMap::new();
        hmap.insert(String::from("chest"), Item::new());
        hmap.insert(String::from("helm"), Item::new());
        hmap.insert(String::from("gloves"), Item::new());
        hmap.insert(String::from("boots"), Item::new());
        hmap.insert(String::from("belt"), Item::new());
        hmap.insert(String::from("amulet"), Item::new());
        hmap.insert(String::from("ring1"), Item::new());
        hmap.insert(String::from("ring2"), Item::new());
        hmap.insert(String::from("shield"), Item::new());
        CharSpecs {
            league: String::from("Standard"),
            level: 0,
            str: 0,
            dex: 0,
            int: 0,
            mods: PassiveTreeMods::new(),
            // all items need to be placed into key,value map
            items: hmap,
        }
    }
}
#[derive(RustcDecodable, RustcEncodable, Debug)]
struct TargetStats
{
    fire_res: i32,
    cold_res: i32,
    lightning_res: i32,
    chaos_res: i32,
    life: i32,
    mana: i32,
    armour: i32,
    evasion: i32,
    es: i32
}

impl TargetStats {
    pub fn new() -> TargetStats {
        TargetStats {
            fire_res: 0,
            cold_res: 0,
            lightning_res: 0,
            chaos_res: 0,
            life: 0,
            mana: 0,
            armour: 0,
            evasion: 0,
            es: 0
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Request
{
    budjet_in_chaos: i32,
    update_itemslots: Vec<String>
}

impl Request {
    pub fn new() -> Request {
        Request {
            budjet_in_chaos: 0,
            update_itemslots: Vec::new()
        }
    }
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
struct ImportData
{
    specs: CharSpecs,
    target: TargetStats,
    request: Request
}

impl ImportData {
    pub fn new() -> ImportData {
        ImportData {
            specs: CharSpecs::new(),
            target: TargetStats::new(),
            request: Request::new()
        }
    }
}


fn main() {
  let config_path = Path::new("config.json");
  let mut options = OpenOptions::new();
  options.read(true).write(true).create(true);

  let imported_data = match options.open(&config_path) {
    Ok(file) =>
    {
        let mut reader = BufReader::new(&file);
        let mut buffer: String = String::new();
        reader.read_to_string(&mut buffer);
        let decode_data = json::decode::<ImportData>(&buffer);
        match decode_data {
            Ok(data) => data,
            Err(heh) =>
            {
                // Remove this to make creating default config as explicit option
                println!("overwriting config because of {}", heh.to_string());
                let freshdata = ImportData::new();
                match OpenOptions::new().write(true).truncate(true).create(true).open(&config_path)
                {
                    Ok(file2) =>
                    {
                        let mut writer = BufWriter::new(&file2);
                        writer.write_all(json::as_pretty_json(&freshdata).to_string().as_bytes());
                    },
                    Err(woot) => panic!("{}", woot.to_string()),
                }
                freshdata
            },
        }
    },
    Err(..) => panic!("Couldn't open. Probably bad permissions."),
  };

  println!("{}", json::as_pretty_json(&imported_data.specs.items["boots"]));
  println!("Userinput had request to update {} items:", imported_data.request.update_itemslots.len());
  for item in imported_data.request.update_itemslots
  {
      println!("{}", item);
  }

 // println!("{}", imported_data.request.update_itemslots.as_string());

    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://rust-lang.org/")
        // set a header
        .header(Connection::close())
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    //println!("Response: {}", body);
}

