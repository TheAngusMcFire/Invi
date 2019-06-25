use serde::{Serialize, Deserialize};
use std::error::Error;
use std::path::Path;
use std::fs;
use whoami;

pub static FILE_NAME: &str = "base.json";

type id_type = u32;

#[derive(Serialize, Deserialize)]
pub struct Inventory
{
    compartments    : Vec<Compartment>,
    containers      : Vec<Container>,
    tags            : Vec<Tag>,
    items           : Vec<Item>,
    cnt_compartment : id_type,
    cnt_container   : id_type,
    cnt_item        : id_type,
    cnt_tag         : id_type
}

#[derive(Serialize, Deserialize)]
pub struct Compartment
{
    name       : String,
    id         : id_type,
    containers : Vec<id_type>
}

#[derive(Serialize, Deserialize)]
pub struct Container
{
    name  : String,
    id    : id_type,
    items : Vec<id_type>,
    tags  : Vec<id_type>
}

#[derive(Serialize, Deserialize)]
pub struct Item
{
    name    : String,
    notes   : String,
    id      : id_type,
    amount  : id_type,
    tags    : Vec<id_type>
}

#[derive(Serialize, Deserialize)]
pub struct Tag
{
    name  : String,
    notes : String,
    id    : id_type
}

pub struct SearchResult <'a>
{
    compartments    : Vec<&'a Compartment>,
    containers      : Vec<&'a Container>,
    tags            : Vec<&'a Tag>,
    items           : Vec<&'a Item>,
}

pub fn search<'a>(key_word : &str, inv :&'a Inventory) -> SearchResult <'a>
{
    return SearchResult 
    {
        compartments    : vec!(&inv.compartments[0]),
        containers      : Vec::new(),
        tags            : Vec::new(),
        items           : Vec::new()
    }
}

pub fn save_inventory(inventory : &Inventory, file_name : String) -> Result<(), Box<dyn Error>> 
{
    let serialized = serde_json::to_string(&inventory)?;
    fs::write(file_name, serialized)?;
    return Ok(());
}

pub fn get_file_location(file_name : &str) -> String
{
    let user_name = &whoami::username();
    let file_path = format!("/home/{}/.config/invi/{}",user_name,file_name);
    return file_path;
}

pub fn save_inventory_to_home()  -> Result<(), Box<dyn Error>> 
{


    return Ok(());
}

pub fn load_inventory_from_home() -> Result<Inventory, Box<dyn Error>> 
{
    let file_name = get_file_location(FILE_NAME);

    if Path::new(&file_name).exists()
    {
        return Ok(load_inventory(get_file_location(FILE_NAME))?);
    }

    let file_dir = get_file_location("");

    fs::create_dir_all(Path::new(&file_dir)).expect(&format!("The default file could not be found and creating the directory {} failed", file_dir));

    new_inventory(file_name.clone()).expect(&format!("Cloud not create new blank inventory database: {}",file_name));

    return Ok(load_inventory(get_file_location(FILE_NAME))?);
}



pub fn load_inventory(file_name : String) -> Result<Inventory, Box<dyn Error>> 
{
    let json_in = fs::read_to_string(file_name)?;
    let obj : Inventory = serde_json::from_str(&json_in)?;
    return Ok(obj);
}

pub fn new_inventory(file_name : String) -> Result<(), Box<dyn Error>> 
{
    let new_inventory = Inventory
    {
        compartments    : Vec::new(),
        containers      : Vec::new(),
        tags            : Vec::new(),
        items           : Vec::new(),
        cnt_compartment : 1 as id_type,
        cnt_container   : 1 as id_type,
        cnt_item        : 1 as id_type,
        cnt_tag         : 0 as id_type,
    };

    let serialized = serde_json::to_string(&new_inventory)?;
    fs::write(&file_name, serialized)?;
    return Ok(());
}

