use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Inventory
{
    compartments    : Vec<Compartment>, 
    tags            : Vec<Tag>,
    cnt_compartment : u32,
    cnt_container   : u32,
    cnt_item        : u32,
    cnt_tag         : u32
}

#[derive(Serialize, Deserialize)]
pub struct Compartment
{
    name       : String,
    id         : u32,
    containers : Vec<Container>
}

#[derive(Serialize, Deserialize)]
pub struct Container
{
    name  : String,
    id    : u32,
    items : Vec<Item>,
    tags  : Vec<Tag>
}

#[derive(Serialize, Deserialize)]
pub struct Item
{
    name    : String,
    notes   : String,
    id      : u32,
    amount  : u32,
    tags    : Vec<Tag>
}

#[derive(Serialize, Deserialize)]
pub struct Tag
{
    name  : String,
    notes : String,
    id    : u32
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
        tags            : Vec::new(),
        cnt_compartment : 1,
        cnt_container   : 1,
        cnt_item        : 1,
        cnt_tag         : 0,
    };

    let serialized = serde_json::to_string(&new_inventory)?;
    fs::write(&file_name, serialized)?;
    return Ok(());
}

