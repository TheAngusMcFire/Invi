
use termion::event::Key;
use termion::input::TermRead;

mod gui;
use crate::gui::{Events, Event};


fn main()
{
    
    let mut gui_context = gui::gui_init();
    let events = gui::Events::new();
    let mut running = true;

    while running
    {
        gui::draw_gui(&mut gui_context);

         match events.next().unwrap()
         {
            Event::Input(input) => match input 
            {
                Key::Char('p') => 
                {
                    break;
                }
                Key::Char('\n') => 
                {
                    if gui_context.txt_input == ":q" 
                    {
                        running = false;
                    }
                }
                Key::Char(c) => 
                {
                    gui_context.txt_input.push(c);
                }
                Key::Esc => {gui_context.txt_input.clear();}
                Key::Backspace => 
                {
                    gui_context.txt_input.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }
}