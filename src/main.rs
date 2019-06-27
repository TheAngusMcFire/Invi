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


fn main()
{
    match not_main()
    {
        Err(err) => println!("Error while creating context:\n    {}\n",&err),
        _ =>()
    }
}

fn dispatch_input(input : &str, context : &mut gui::AppContext) -> bool
{
    match input.as_ref()
    {
        ":q" => {return true;}
        ":ct" =>{context.clear_terminal();}
        ":0" => {context.layout = gui::InviLayout::Terminal}
        ":1" => {context.layout = gui::InviLayout::Search}
        other => {context.write_to_terminal(&format!("{}\n",other));}
    }

    return false;
}
