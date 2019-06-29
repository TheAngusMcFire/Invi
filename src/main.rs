use std::io::{self, Write};
use std::error::Error;

use termion::event::Key;
use tui::backend::TermionBackend;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::cursor::Goto;
use tui::Terminal;

mod gui;
mod inventory;
use crate::gui::{Event};


fn not_main() -> Result<(), Box<dyn Error>> 
{
    let mut context = match gui::AppContext::new("test.json".to_string())
    {
        Ok(context) => context,
        Err(err) => 
        {
            inventory::new_inventory("tmp.json".to_string()).unwrap();
            let mut con = gui::AppContext::new("tmp.json".to_string()).unwrap();
            con.write_to_terminal(&format!("Error while creating context:\n    {}\n",&err));
            con.write_to_terminal(&format!("A temporary database will be used:(tmp.json)\n"));
            con
        }
    };


    let greeting_string = &inventory::get_file_location(inventory::FILE_NAME);
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
            ('\"') => 
            {
                if quote_start
                {
                    args.push(tmp_string.clone()); 
                    tmp_string.clear();
                    quote_start = false;
                }
                else{ quote_start = true; }
            }

            (' ') => 
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
    let args = get_arguments(input);

    if args.len() == 0 {return false;}

    match args[0].as_ref()
    {
        ":q" => {return true;}
        ":ct" =>{context.clear_terminal();}
        ":0" => {context.layout = gui::InviLayout::Terminal}
        ":1" => {context.layout = gui::InviLayout::Search}
        _ => {context.write_to_terminal(&format!("No use for args {:?}\n",args));}
    }

    return false;
}


fn benchmark_test()
{
    let mut strings : Vec<String> = Vec::new();

    for index in 0..10000000
    {
        strings.push(format!("So this needs to be a very long string, so i will write on and on for a little bit {}",index));
    } 

    println!("Filled struct {}",strings.len());

    for stri in strings.iter()
    {
        if stri.contains("9999999")
        {
            println!("found!!! {}",stri);
            return;
        }
    }
}

fn main()
{
    match not_main()
    {
        Err(err) => println!("Error while creating context:\n    {}\n",&err),
        _ =>()
    }
}

