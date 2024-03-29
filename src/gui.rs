
use std::io::{self};
use std::error::Error;
use std::cmp::{max};

use tui::{Frame, Terminal};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect, Corner};
use tui::style::{Color, Style};
use tui::widgets::{ Block, Borders, Paragraph, Text, Widget,List};
use crate::inventory::{Inventory,load_inventory_from_home};


pub enum InviLayout
{
    Terminal,
    Search,
    Overview
}

//eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
pub struct AppContext
{
    pub txt_input  : String,
    pub cursor_pos : u16,
    pub layout     : InviLayout,
    size_term      : Rect,
    pub gui_dirty      : bool,
    pub inventory  : Inventory,
    pub invi_dirty : bool,
    pub scroll_back: usize,
    pub scroll_items : Vec<String>,
    
    term_txt   : String,
}

impl AppContext
{
    pub fn new() -> Result<(AppContext), Box<dyn Error>> 
    {

        let sc_items = vec![":acont ".to_string(),":acomp ".to_string(), ":aitem ".to_string(), ":atag ".to_string()];
        let sc_items_len = sc_items.len();
        let context = AppContext
        {
            txt_input    : String::new(),
            cursor_pos   : 0,
            layout       : InviLayout::Overview,
            inventory    : load_inventory_from_home()?,
            invi_dirty   : false,
            size_term    : Rect::new(0,0,0,0),
            gui_dirty    : true, 
            scroll_items : sc_items,
            scroll_back  : sc_items_len,

            term_txt : String::new(),
        };

        return Ok(context);
    }

    pub fn clear_terminal(&mut self)
    {
        self.term_txt.clear();
        self.gui_dirty = true;
    }
    
    pub fn write_to_terminal(&mut self, msg : &str)
    {
        self.term_txt.push_str(msg);
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

    pub fn get_terminal_ref(&mut self) -> &mut String
    {
        return &mut self.term_txt;
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
            InviLayout::Terminal => {draw_terminal (&mut f, chunks[0], &context.term_txt);}
            InviLayout::Search => {draw_first_tab (&mut f, chunks[0]);}
            InviLayout::Overview => {draw_overview (&mut f, chunks[0],context);}
        }
       
       Paragraph::new([Text::raw(&context.txt_input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .render(&mut f, chunks[1]);
    }).unwrap();
}

fn draw_overview<B>(f: &mut Frame<B>, area: Rect, context : &AppContext) where B: Backend,
{
    let style = Style::default().fg(Color::White).bg(Color::Reset);

    let main_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
    .split(area);
    
    let comp = context.inventory.compartments.iter().map(|value| 
    {
        let txt = format!("{:04X} : {}", value.id, value.name);
        Text::styled
        (
            txt,
            style
        )
    });

    let conts = context.inventory.containers.iter().map(|value| 
    {
        let txt = format!("{:04X} {:04X} : {}",value.id_comp, value.id, value.name); 
        Text::styled
        (
            txt,
            style
        )
    });

    let items = context.inventory.items.iter().map(|value| 
    {
        let txt = format!("{:04X} {:04X} : {}",value.id_cont, value.id, value.name); 
        Text::styled
        (
            txt,
            style
        )
    });

    let tags = context.inventory.tags.iter().map(|value| 
    {
        let txt = format!("{:4X} : {}", value.id, value.name);
        Text::styled
        (
            txt,    
            style
        )
    });



    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(25), Constraint::Percentage(25),Constraint::Percentage(25),Constraint::Percentage(25)].as_ref())
    .split(main_chunks[0]);

    Paragraph::new([Text::raw(&context.term_txt)].iter())
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(" Main Terminal "))
        .render(f, main_chunks[1]);

    // SelectableList::default()
    // .block(Block::default().borders(Borders::ALL).title("List"))
    // .items(&some)
    // .select(Option::None)
    // .style(style)
    // .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
    // .highlight_symbol(">")
    // .render(f, chunks[0]);



    List::new(comp)
        .block(Block::default().borders(Borders::ALL).title(" Compartmets "))
        .start_corner(Corner::TopRight)
        .render(f, chunks[0]);

    List::new(conts)
        .block(Block::default().borders(Borders::ALL).title(" Containers "))
        .start_corner(Corner::TopRight)
        .render(f, chunks[1]);

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Items "))
        .start_corner(Corner::TopRight)
        .render(f, chunks[2]);

    List::new(tags)
        .block(Block::default().borders(Borders::ALL).title(" Tags "))
        .start_corner(Corner::TopRight)
        .render(f, chunks[3]);
}

fn draw_terminal<B>(f: &mut Frame<B>, area: Rect, msg : &str) where B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50)].as_ref())
        .split(area);
 
    //TODO: perform line wrap
    Paragraph::new([Text::raw(msg)].iter())
        .style(Style::default().fg(Color::Cyan))
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
    gui_context.scroll_back = 0;
    return tmp;
}

fn get_target_index(in_str : &mut String, index : usize) -> usize
{
    let mut target_index : usize = 0;
    let mut position = index;

    for c in in_str.chars()
    {
        if position == 0 {break;}
        target_index += c.len_utf8();
        position -= 1;
    }

    return target_index;
}

fn get_len_any(in_str : & String) -> usize
{
    let mut cnt : usize = 0; 

    for _c in in_str.chars()
        {cnt += 1;}

    return cnt;
}

fn remote_any(in_str : &mut String, index : usize)
{
    let target = get_target_index(in_str, index);
    in_str.remove(target);
}

fn insert_any(in_str : &mut String, index : usize, ch : char)
{
    let target = get_target_index(in_str, index);
    in_str.insert(target, ch);
}

pub fn set_txt_input(context : &mut AppContext, msg : String)
{
    context.txt_input = msg;
    context.cursor_pos = get_len_any(&context.txt_input) as u16;
    context.gui_dirty = true;
}

pub fn handle_input_key (key : Key, context : &mut AppContext)
{
    match key 
    {
        Key::Char(c) =>
        {
            context.gui_dirty = true;
            insert_any(&mut context.txt_input, context.cursor_pos as usize, c);
            context.cursor_pos += 1;
        }

        Key::Left =>
        {
            context.cursor_pos = if context.cursor_pos > 0 {context.cursor_pos - 1} else {0}
        }

        Key::Right =>
        {
            context.cursor_pos = if context.cursor_pos < get_len_any(&context.txt_input) as u16
            {context.cursor_pos + 1} else {get_len_any(&context.txt_input) as u16}
        }

        Key::Home => 
        {
            context.cursor_pos = 0;
        }
        
        Key::End => 
        {
            context.cursor_pos = get_len_any(&context.txt_input) as u16;
        }

        Key::Esc => 
        {
            context.gui_dirty = true;
            context.txt_input.clear();
            context.cursor_pos = 0;
            context.scroll_back = 0;
        }

        Key::Backspace => 
        {
            context.gui_dirty = true;
            if context.cursor_pos > 0 {remote_any(&mut context.txt_input,context.cursor_pos as usize -1);}
            context.cursor_pos = if context.cursor_pos < get_len_any(&context.txt_input) as u16 
            {context.cursor_pos - 1} else {get_len_any(&context.txt_input) as u16};
        }

        Key::Up => 
        {
            context.scroll_back = if context.scroll_back < 1 {context.scroll_items.len() - 1} else {context.scroll_back - 1};
            let scroll_item = context.scroll_items[ context.scroll_back].clone();
            set_txt_input(context, scroll_item);
        }

        Key::Down => 
        {
            context.scroll_back = if context.scroll_back + 1  < context.scroll_items.len() {context.scroll_back + 1} else {0};
            let scroll_item = context.scroll_items[ context.scroll_back].clone();
            set_txt_input(context, scroll_item);
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
