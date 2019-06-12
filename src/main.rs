
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
                Key::Char('\n') => 
                {
                    let input = gui::get_input_str_and_clear(&mut gui_context);
                    
                    match input.as_ref()
                    {
                        ":q" => {running = false;}
                        ":0" => {gui_context.layout = 0;}
                        ":1" => {gui_context.layout = 1;}
                        other => {gui_context.txt_terminal += &format!("{}\n",other)[..];}
                    }
                }

                other => gui::handle_input_key(other, &mut gui_context)
            },
            _ => {}
        }
    }
}