use std::io::{self, Write};

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


fn main()
{   
    /* init all the terminal specific resources (from tui-rf example) */
    let stdout   = io::stdout().into_raw_mode().unwrap();
    let stdout   = MouseTerminal::from(stdout);
    let stdout   = AlternateScreen::from(stdout);
    let backend  = TermionBackend::new(stdout);
    let mut terminal =  Terminal::new(backend).unwrap();

    let mut context = gui::AppContext::new();

    let events = gui::Events::new();

    loop
    {
        gui::draw(&mut terminal, &context);
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
                    
                    match input.as_ref()
                    {
                        ":q" => {break;}
                        ":0" => {context.layout = gui::InviLayout::Terminal}
                        ":1" => {context.layout = gui::InviLayout::Search}
                        other => {context.txt_terminal += &format!("{}\n",other)[..];}
                    }
                }
                other => gui::handle_input_key(other, &mut context)
            },
            _ => {}
        }
    }
}