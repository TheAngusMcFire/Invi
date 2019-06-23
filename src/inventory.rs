use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs;

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

pub fn save_inventory(inventory : &Inventory, file_name : String) -> Result<(), Box<dyn Error>> 
{
    let serialized = serde_json::to_string(&inventory)?;
    fs::write(file_name, serialized)?;
    return Ok(());
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
        cnt_compartment : 1,
        cnt_container   : 1,
        cnt_item        : 1,
        cnt_tag         : 0,
    };

    let serialized = serde_json::to_string(&new_inventory)?;
    fs::write(&file_name, serialized)?;
    return Ok(());
}

