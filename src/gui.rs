
use std::io::{self};
use std::error::Error;

use tui::{Frame, Terminal};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{ Block, Borders, Paragraph, Text, Widget};
use crate::inventory::{Inventory,load_inventory};

pub enum InviLayout
{
    Terminal,
    Search
}

pub struct AppContext
{
    pub txt_input : String,
    pub cursor_pos : u16,
    pub layout : InviLayout,
    txt_terminal : String,
    pub inventory : Inventory,
    size_term : Rect,
    gui_dirty    : bool,
}

impl AppContext
{
    pub fn new(file_name : String) -> Result<(AppContext), Box<dyn Error>> 
    {
        return Ok(AppContext
        {
            txt_input    : String::new(),
            cursor_pos   : 0,
            layout       : InviLayout::Terminal,
            txt_terminal : String::new(),
            inventory    : load_inventory(file_name)?,
            size_term    : Rect::new(0,0,0,0),
            gui_dirty    : true, 
        });
    }

    pub fn clear_terminal(&mut self)
    {
        self.txt_terminal.clear();
        self.gui_dirty = true;
    }
    pub fn write_to_terminal(&mut self, msg : &str)
    {
        self.txt_terminal.push_str(msg);
        self.gui_dirty = true;
    }

    pub fn check_if_changed<B: Backend>(&mut self, terminal: &Terminal<B>)
    {
        if self.size_term != terminal.size().unwrap()
        {
            self.gui_dirty = true
        }

        self.size_term = terminal.size().unwrap();
    }

    pub fn need_redraw(&mut self) -> bool
    {
        let dirty = self.gui_dirty;
        self.gui_dirty = false;
        return dirty;
    }
}


pub fn draw<B: Backend>(terminal: &mut Terminal<B>, context: &mut AppContext) 
{
    if context.need_redraw() != true {return;}

    terminal.draw(|mut f| 
    {
        let chunks = Layout::default()
            .constraints([Constraint::Min(0),Constraint::Length(3)].as_ref())
            .direction(Direction::Vertical)
            .split(f.size());

        match context.layout
        {
            InviLayout::Terminal => {draw_terminal (&mut f, chunks[0], &context.txt_terminal[..]);}
            InviLayout::Search => {draw_first_tab (&mut f, chunks[0]);}
        }
       
       Paragraph::new([Text::raw(&context.txt_input[..])].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .render(&mut f, chunks[1]);
    }).unwrap();
}

fn draw_terminal<B>(f: &mut Frame<B>, area: Rect, msg : &str) where B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50)].as_ref())
        .split(area);
 
    //TODO: perform line wrap
    Paragraph::new([Text::raw(msg)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Main Terminal"))
        .render(f, chunks[0]);
}

fn draw_first_tab<B>(f: &mut Frame<B>, area: Rect) where B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    Paragraph::new([Text::raw("This is just some text, This is just some text,This is just some text, This is just some text")].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .render(f, chunks[1]);

    Paragraph::new([Text::raw("This is just some other")].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .render(f, chunks[0]);
}

/* ****************************+ textfield handling ****************************+ */

pub fn get_input_str_and_clear(gui_context : &mut AppContext) -> String
{
    let tmp = gui_context.txt_input.clone();
    gui_context.txt_input.clear();
    gui_context.cursor_pos = 0;
    gui_context.gui_dirty = true;
    return tmp;
}

pub fn handle_input_key (key : Key, gui_context : &mut AppContext)
{
    match key 
    {
        Key::Char(c) =>
        {
            gui_context.gui_dirty = true;
            gui_context.txt_input.insert(gui_context.cursor_pos as usize, c);
            gui_context.cursor_pos += 1;
        }

        Key::Left =>
        {
            gui_context.cursor_pos = if gui_context.cursor_pos > 0 {gui_context.cursor_pos - 1} else {0}
        }

        Key::Right =>
        {
            gui_context.cursor_pos = if gui_context.cursor_pos < gui_context.txt_input.len() as u16
            {gui_context.cursor_pos + 1} else {gui_context.txt_input.len() as u16}
        }

        Key::Home => 
        {
            gui_context.cursor_pos = 0;
        }
        
        Key::End => 
        {
            gui_context.cursor_pos = gui_context.txt_input.len() as u16;
        }

        Key::Esc => 
        {
            gui_context.gui_dirty = true;
            gui_context.txt_input.clear();gui_context.cursor_pos = 0;
        }

        Key::Backspace => 
        {
            gui_context.gui_dirty = true;
            if gui_context.cursor_pos > 0 {gui_context.txt_input.remove(gui_context.cursor_pos as usize -1);}
            gui_context.cursor_pos = if gui_context.cursor_pos < gui_context.txt_input.len() as u16 
            {gui_context.cursor_pos - 1} else {gui_context.txt_input.len() as u16};
        }
        _ => {}
    }
}


/* ****************************+ input stuff ****************************+ */

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> 
{
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events 
{
    rx: mpsc::Receiver<Event<Key>>,
    //input_handle: thread::JoinHandle<()>,
    //tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config 
{
    pub tick_rate: Duration,
}

impl Default for Config 
{
    fn default() -> Config 
    {
        Config 
        {
            tick_rate: Duration::from_millis(100),
        }
    }
}

impl Events 
{
    pub fn new() -> Events 
    {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events 
    {
        let (tx, rx) = mpsc::channel();
        let _input_handle = 
        {
            let tx = tx.clone();
            thread::spawn(move || 
            {
                let stdin = io::stdin();
                for evt in stdin.keys() 
                {
                    match evt 
                    {
                        Ok(key) => 
                        {
                            if let Err(_) = tx.send(Event::Input(key)) 
                            {
                                return;
                            }
                        }
                        Err(_) => {}
                    }
                }
            })
        };

        let _tick_handle = 
        {
            let tx = tx.clone();
            thread::spawn(move || 
            {
                let tx = tx.clone();
                loop 
                {
                    tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        
        Events 
        {
            rx,
            //input_handle,
            //tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> 
    {
        self.rx.recv()
    }
}