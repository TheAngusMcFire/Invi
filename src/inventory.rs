use serde::{Serialize, Deserialize};
use std::error::Error;
use std::path::Path;
use std::fs;
use whoami;

pub static FILE_NAME: &str = "base.json";

pub type IdType = u32;


#[derive(Serialize, Deserialize)]
pub struct Inventory
{
    pub compartments    : Vec<Compartment>,
    pub containers      : Vec<Container>,
    pub tags            : Vec<Tag>,
    pub items           : Vec<Item>,
    cnt_compartment : IdType,
    cnt_container   : IdType,
    cnt_item        : IdType,
    cnt_tag         : IdType
}

#[derive(Serialize, Deserialize)]
pub struct Compartment
{
    pub name       : String,
    pub id         : IdType,
    containers : Vec<IdType>
}

#[derive(Serialize, Deserialize)]
pub struct Container
{
    pub name  : String,
    pub id    : IdType,
    pub id_comp   : IdType,
    items : Vec<IdType>,
    tags  : Vec<IdType>
}

#[derive(Serialize, Deserialize)]
pub struct Item
{
    pub name    : String,
    pub id      : IdType,
    pub id_cont : IdType,
}

#[derive(Serialize, Deserialize)]
pub struct Tag
{
    pub name  : String,
    pub id    : IdType
}

//trait IdObject{fn get_id(&self) -> u32; }
//impl IdObject for Tag        { fn get_id(&self) -> IdType {return self.id;} }
//impl IdObject for Item       { fn get_id(&self) -> IdType {return self.id;} }
//impl IdObject for Container  { fn get_id(&self) -> IdType {return self.id;} }
//impl IdObject for Compartment{ fn get_id(&self) -> IdType {return self.id;} }

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

impl Inventory
{
    pub fn check_tags_ids(&self, ids : &Vec<IdType>) -> Result<(),String>
    {
        'main_loop : for id in ids
        {
            for tag in self.tags.iter()
            {
                if &tag.id == id 
                {
                    continue 'main_loop;
                }
            }
            return Err(format!("{}",id));
        }
        return Ok(());
    }

    pub fn add_tag(&mut self, name : &str)
    {
        self.tags.push
        (
            Tag
            {
                name  : String::from(name),
                id : self.cnt_tag
            }
        );
        self.cnt_tag += 1;
    }

    pub fn add_item(&mut self, name : &str, con_id : IdType) -> Result<(),String>
    {
        let cont_index = con_id as usize;
        if let None = self.containers.get(cont_index)
        {
            return Err(format!("The container with the id: {} was not found!!!", con_id));
        }

        self.items.push
        (
            Item
            {
                name  : String::from(name),
                id : self.cnt_item,
                id_cont : con_id,
            }
        );

        self.containers[cont_index].items.push(self.cnt_item);
        self.cnt_item += 1;

        return Ok(());
    }

    pub fn add_container(&mut self, name : &str, com_id : IdType, tags : Vec<IdType>) -> Result<(),String>
    {
        let comp_index = com_id as usize;

        match self.check_tags_ids(&tags) {Err(e) => {return Err(format!("Check the tag ids, {} was not found!!!",e));} _=> {}}

        if let None = self.compartments.get(comp_index)
        {
            return Err(format!("The compartment with the id: {} was not found!!!", com_id));
        }

        let cont_id = self.compartments[comp_index].containers.len() as IdType;

        self.containers.push
        (
            Container
            {
                name  : String::from(name),
                id : cont_id,
                id_comp : com_id,
                items : Vec::new(),
                tags : Vec::from(tags)
            }
        );

        self.compartments[comp_index].containers.push(cont_id);

        return Ok(());
    }

    pub fn add_compartment(&mut self, name : &str)
    {
        self.compartments.push
        (
            Compartment
            {
                name  : String::from(name),
                id : self.cnt_compartment,
                containers : Vec::new()
            }
        );
        self.cnt_compartment += 1;
    }
}



pub fn save_inventory(inventory : &Inventory) -> Result<(), Box<dyn Error>> 
{
    let serialized = serde_json::to_string(&inventory)?;
    fs::write(get_file_location(FILE_NAME), serialized)?;
    return Ok(());
}

pub fn get_file_location(file_name : &str) -> String
{
    let user_name = &whoami::username();
    let file_path = format!("/home/{}/.config/invi/{}",user_name,file_name);
    return file_path;
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
        cnt_compartment : 0 as IdType,
        cnt_container   : 0 as IdType,
        cnt_item        : 0 as IdType,
        cnt_tag         : 0 as IdType,
    };

    let serialized = serde_json::to_string(&new_inventory)?;
    fs::write(&file_name, serialized)?;
    return Ok(());
}

