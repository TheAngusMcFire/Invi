use std::io::{self};
use std::error::Error;
use std::fmt::Write;

use termion::event::Key;
use tui::backend::TermionBackend;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::cursor::Goto;
use tui::Terminal;

mod gui;
mod inventory;
mod error;
use crate::gui::{Event};

//to attach to the process use :
//echo 0 > /proc/sys/kernel/yama/ptrace_scope


fn not_main() -> Result<(), Box<dyn Error>> 
{
    let greeting_string = &inventory::get_file_location(inventory::FILE_NAME);
    let mut context = match gui::AppContext::new()
    {
        Ok(context) => context,
        Err(err) => 
        {
            inventory::new_inventory("tmp.json".to_string()).unwrap();
            let mut con = gui::AppContext::new().unwrap();
            con.write_to_terminal(&format!("Error while creating context:\n    {}\n",&err));
            con.write_to_terminal(&format!("A temporary database will be used:(tmp.json)\n"));
            con
        }
    };


    context.write_to_terminal(&format!("Using default file: {}\n",greeting_string));


    /* init all the terminal specific resources (from tui-rf example) */
    let stdout   = io::stdout().into_raw_mode().unwrap();
    let stdout   = MouseTerminal::from(stdout);
    let stdout   = AlternateScreen::from(stdout);
    let backend  = TermionBackend::new(stdout);
    let mut terminal =  Terminal::new(backend).unwrap();

    let events = gui::Events::new();

    loop
    {
        context.check_if_changed(&terminal);
        gui::draw(&mut terminal, &mut context);
        let text_field_pos = terminal.size().unwrap().height - 1;
        use std::io::Write;
        write!(terminal.backend_mut(),"{}", Goto(2 + context.cursor_pos as u16, text_field_pos)).unwrap();
        io::stdout().flush().ok();

         match events.next().unwrap()
         {
            Event::Input(input) => match input 
            {
                Key::Char('\n') => 
                {
                    let input = gui::get_input_str_and_clear(&mut context);

                    if dispatch_input(&input, &mut context){break;}
                }

                other => gui::handle_input_key(other, &mut context)
            },
            _ => {}
        }
    }

    return Ok(());
}

fn get_arguments(in_str :&str) -> Vec<String>
{
    let mut args : Vec<String> = Vec::new();
    let mut tmp_string         = String::new();
    let mut quote_start : bool = false;

    for ch in in_str.chars()
    {
        match ch
        {
            '\"' => 
            {
                if quote_start
                {
                    args.push(tmp_string.clone());
                    tmp_string.clear();
                    quote_start = false;
                }
                else{ quote_start = true; }
            }

            ' ' =>
            {
                if quote_start {tmp_string.push(ch)}
                else
                {
                    if tmp_string.len() == 0 {continue;}
                    args.push(tmp_string.clone());
                    tmp_string.clear();
                }
            }
            _   => tmp_string.push(ch)
        }
    }

    if tmp_string.len() != 0 {args.push(tmp_string.clone());}

    return args;
}


fn dispatch_input(input : &str, context : &mut gui::AppContext) -> bool
{
    let mut args = get_arguments(input);

    if args.len() == 0 {return false;}

    let first_arg = args.remove(0);

    match first_arg.as_ref()
    {
        ":q"  => 
        {
            if context.invi_dirty
            {
                writeln!(context.get_terminal_ref(),"There are unwritten changes in the inventory, write the changes(:w) or force quit(:q!) ").unwrap();
                return false;
            }
            return true;
        }
        ":q!"  => { return true; }
        ":ct" | "cls" =>{context.clear_terminal();}
        ":0"  => {context.layout = gui::InviLayout::Terminal}
        ":1"  => {context.layout = gui::InviLayout::Overview}
        ":wq" => 
        {
            if let Err(e) = write_back_file(context)
                { writeln!(context.get_terminal_ref(),"Error while saving file: {}",e).unwrap(); }
            else 
                { return true; }
        }
        ":w" => 
        {
            if let Err(e) = write_back_file(context)
            {writeln!(context.get_terminal_ref(),"Error while saving file: {}",e).unwrap();}
        }
        ":help" | ":?" | "help" | "?" | "hlp" | ":hlp" => 
            {print_help_msg(context);}

        ":aitem" => if let Err(e) = add_item(context, &args)
            {writeln!(context.get_terminal_ref(),"{}",e).unwrap();},

        ":atag"  => if let Err(e) = add_tag (context, &args)
            {writeln!(context.get_terminal_ref(),"{}",e).unwrap();},

        ":acomp"  => if let Err(e) = add_compartment(context, &args)
            {writeln!(context.get_terminal_ref(),"{}",e).unwrap();},
        
        ":acont"  => if let Err(e) = add_container(context, &args)
            {writeln!(context.get_terminal_ref(),"{}",e).unwrap();},

        _ => {context.write_to_terminal(&format!("No use for \"{}\" and args: {:?}\n",first_arg, args));}
    }

    context.scroll_items.push(input.to_string());

    return false;
}

fn write_back_file(context : &mut gui::AppContext) -> Result<(), Box<dyn Error>>
{
    inventory::save_inventory(&context.inventory)?;
    context.invi_dirty = false;
    return Ok(());
}


fn add_container(context : &mut gui::AppContext, args : &Vec<String>) -> Result<(), Box<dyn Error>>
{
    if args.len() < 2
        {return Err(Box::new(error::GenericError::new("Error invalid number of arguments".to_string())));}

    let name = &args[0];    
    let com_id : inventory::IdType = args[1].parse()?;
    let tags = get_ids_from_args(&args[2..])?; 

    context.inventory.add_container(name,  com_id, tags)?;
    context.invi_dirty = true;

    return Ok(());
}

fn add_compartment(context : &mut gui::AppContext, args : &Vec<String>) -> Result<(), Box<dyn Error>>
{
    if args.len() != 1
        {return Err(Box::new(error::GenericError::new("Error invalid number of arguments".to_string())));}

    let name = &args[0];

    context.inventory.add_compartment(name);
    context.invi_dirty = true;

    return Ok(());
}

fn add_tag(context : &mut gui::AppContext, args : &Vec<String>) -> Result<(), Box<dyn Error>>
{
    if args.len() != 1
        {return Err(Box::new(error::GenericError::new("Error invalid number of arguments".to_string())));}

    let name = &args[0];

    context.inventory.add_tag(name);
    context.invi_dirty = true;

    return Ok(());
}


fn get_ids_from_args(args : &[String]) -> Result<Vec<inventory::IdType>,Box<dyn Error>>
{
    let mut tag_ids : Vec<inventory::IdType> = Vec::new();

    for tag_str in args { tag_ids.push(tag_str.parse()?); }

    return Ok(tag_ids);
}

fn add_item(context : &mut gui::AppContext, args : &Vec<String>) -> Result<(), Box<dyn Error>>
{
    if args.len() != 2
        {return Err(Box::new(error::GenericError::new("Error invalid number of arguments".to_string())));}

    let con_id : inventory::IdType = args[1].parse()?;
    let name = &args[0];

    context.inventory.add_item(name, con_id)?;

    return Ok(());
}


fn print_help_msg(context : &mut gui::AppContext)
{
    let term = context.get_terminal_ref();

    writeln!(term,"This is the Invi help:").unwrap();
    writeln!(term,"Invi is a easy to used terminal based inventory manager").unwrap();
    writeln!(term,"Commands:").unwrap();
    writeln!(term,"    {:20}{}",":q","quit invi (save first)").unwrap();
    writeln!(term,"    {:20}{}",":ct","clear the terminals").unwrap();
    writeln!(term,"    {:20}{}","hlp | ? | help","prints this message").unwrap();
    writeln!(term,"    {:20}{}",":atag","adds a new tag <name>").unwrap();
    writeln!(term,"    {:20}{}",":acomp","adds a new compartment <name>").unwrap();
    writeln!(term,"    {:20}{}",":acont","adds a new container <name> <compartment_id> <tag_id1> <tag_id2> ... <tag_idn>").unwrap();
    writeln!(term,"    {:20}{}",":aitem","adds a new item <name> <container_id>").unwrap();
    writeln!(term,"    {:20}{}",":/<str>","used to search, items, containers, and compartments are listed also tag stuff").unwrap();
}


//fn benchmark_test()
//{
//    let mut strings : Vec<String> = Vec::new();
//
//    for index in 0..10000000
//    {
//        strings.push(format!("So this needs to be a very long string, so i will write on and on for a little bit {}",index));
//    } 
//
//    println!("Filled struct {}",strings.len());
//
//    for stri in strings.iter()
//    {
//        if stri.contains("9999999")
//        {
//            println!("found!!! {}",stri);
//            return;
//        }
//    }
//}

fn main()
{
    match not_main()
    {
        Err(err) => println!("Error while creating context:\n    {}\n",&err),
        _ =>()
    }
}

