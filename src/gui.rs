
use std::io::{self, Write};

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::cursor::Goto;

use tui::backend::TermionBackend;
use tui::{Frame, Terminal};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
use tui::widgets::{
    Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, Marker, Paragraph, Row,
    SelectableList, Sparkline, Table, Tabs, Text, Widget,
};

pub struct GuiContext
{
    terminal : tui::Terminal<TermionBackend<termion::screen::AlternateScreen<termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>>>>,
    pub txt_input : String,
    pub cursor_pos : u16,
    pub layout : u16,
    pub txt_terminal : String
}



pub fn gui_init() -> GuiContext
{
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let context = GuiContext
    {
        terminal : Terminal::new(backend).unwrap(),
        txt_input : String::new(),
        cursor_pos : 0,
        layout : 0,
        txt_terminal : String::new(),
    };

    //context.terminal.hide_cursor().unwrap();

    return context;
}

pub fn draw_gui(context : &mut GuiContext)
{
    let txt : String = context.txt_input.clone();
    let layout = context.layout;
    let term_msg = context.txt_terminal.clone();

    context.terminal.draw(|mut f| 
    {
        let chunks = Layout::default()
            .constraints([Constraint::Min(0),Constraint::Length(3)].as_ref())
            .direction(Direction::Vertical)
            .split(f.size());

        match layout
        {
            0 => {draw_terminal (&mut f, chunks[0], &term_msg[..]);}
            1 => {draw_first_tab (&mut f, chunks[0]);}
            _ => {} 
        }
       
       Paragraph::new([Text::raw(txt)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .render(&mut f, chunks[1]);
    }).unwrap();

    let text_field_pos = context.terminal.size().unwrap().height - 1;

    write!(context.terminal.backend_mut(),"{}", Goto(2 + context.cursor_pos as u16, text_field_pos)).unwrap();

    io::stdout().flush().ok();
}

fn draw_terminal<B>(f: &mut Frame<B>, area: Rect, msg : &str) where B: Backend,
{
    let chunks = Layout::default()
            .direction(Direction::Horizontal)
            //.margin(2)
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(area);
 
        Paragraph::new([Text::raw(msg)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Main Terminal"))
                .render(f, chunks[0]);

}

fn draw_first_tab<B>(f: &mut Frame<B>, area: Rect) where B: Backend,
{
    let chunks = Layout::default()
            .direction(Direction::Horizontal)
            //.margin(2)
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

pub fn get_input_str_and_clear(gui_context : &mut GuiContext) -> String
{
    let tmp = gui_context.txt_input.clone();
    gui_context.txt_input.clear();
    gui_context.cursor_pos = 0;
    return tmp;
}

pub fn handle_input_key (key : Key, gui_context : &mut GuiContext)
{
    match key 
    {
        Key::Char(c) =>   
        {
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

        Key::Home => {gui_context.cursor_pos = 0;}
        Key::End => {gui_context.cursor_pos = gui_context.txt_input.len() as u16;}

        Key::Esc =>       {gui_context.txt_input.clear();gui_context.cursor_pos = 0;}
        Key::Backspace => 
        {
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
        let input_handle = 
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

        let tick_handle = 
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