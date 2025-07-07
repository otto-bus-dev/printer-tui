use std::io;
use crossterm::event::{self,Event,KeyEvent,KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    DefaultTerminal,Frame,
};
use ratatui::prelude::*;
mod new_printer;
mod printers;
mod cups;
mod utils;
mod test_utils;
use new_printer::NewPrinter;
use printers::Printers;

use cups::device::Device;
use cups::printer::{
    Printer,
    get_all_printers,
};
use cups::driver::Driver;

use utils::{
    TUIMode,
    EditBlock,
    EditMode,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    exit:bool,
    printers: Vec<Printer>,
    devices: Vec<Device>,
    drivers: Vec<Driver>,
    selected_printer: usize,
    selected_device: usize,
    selected_driver: usize,
    selected_edit_block: EditBlock,
    selected_edit_mode: EditMode,
    selected_printer_name: String,
    mode: TUIMode,
}

impl App{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.printers = get_all_printers();
        self.selected_printer= 0;
        self.selected_printer_name = self.printers.get(self.selected_printer)
            .map_or("No Printer".to_string(), |p| p.name.clone());
        self.mode = TUIMode::View;
        self.selected_edit_block = EditBlock::Title;
        self.selected_edit_mode = EditMode::View;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self,frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
               self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            TUIMode::View => Printers::handle_events(self,key_event),
            TUIMode::Edit => NewPrinter::handle_events(self,key_event),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
  
    fn change_mode(&mut self, mode: TUIMode) {
       self.mode = mode;
    }
}

impl Widget for &App {

    fn render(self,area: Rect, buf: &mut Buffer) {
        let mut new_printer= NewPrinter::new(
            self.selected_edit_block,
            self.selected_edit_mode,
            &self.devices,
            &self.drivers,
            Some(self.selected_device), 
            Some(self.selected_driver),
            &self.selected_printer_name,
        );

        let mut printers = Printers::new(
            &self.printers,
            self.selected_printer, 
        );
        
        let outer_layout = Layout::default()
             .direction(Direction::Vertical)
             .constraints(vec![
                 Constraint::Percentage(100),
             ])
             .split(area);
        

        match self.mode {
            TUIMode::View => {
                printers.render(outer_layout[0], buf);
            }
            TUIMode::Edit => {
                new_printer.render(outer_layout[0], buf);
            }
        }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::tests::test_monitors;
//
//     #[test]
//     fn handle_mode_view_key_event() -> io::Result<()> {
//         let mut app = App{
//             monitors: test_monitors(),
//             selected_monitor: 0,
//             ..Default::default()
//         };
//
//         app.handle_key_event(KeyCode::Char('k').into());
//         assert_eq!(app.selected_monitor, 1);
//
//         app.handle_key_event(KeyCode::Char('j').into());
//         assert_eq!(app.selected_monitor, 0);
//
//         app.handle_key_event(KeyCode::Char('j').into());
//         assert_eq!(app.selected_monitor, app.monitors.len() - 1);
//
//         app.handle_key_event(KeyCode::Char('k').into());
//         assert_eq!(app.selected_monitor, 0);
//
//         app.handle_key_event(KeyCode::Char('m').into());
//         assert_eq!(app.mode, TUIMode::Move);
//
//         app.handle_key_event(KeyCode::Esc.into());
//         assert_eq!(app.mode, TUIMode::View);
//
//         app.handle_key_event(KeyCode::Char('r').into());
//         assert_eq!(app.mode, TUIMode::Resolution);
//
//         app.handle_key_event(KeyCode::Esc.into());
//         assert_eq!(app.mode, TUIMode::View);
//
//         app.handle_key_event(KeyCode::Char('s').into());
//         assert_eq!(app.mode, TUIMode::Scale);
//
//         app.handle_key_event(KeyCode::Esc.into());
//         assert_eq!(app.mode, TUIMode::View);
//
//         app.handle_key_event(KeyCode::Char('q').into());
//         assert!(app.exit);
//
//         Ok(())
//     }
//
//
//     #[test]
//     fn handle_mode_move_key_event() -> io::Result<()> {
//         let mut app = App{
//             monitors: test_monitors(),
//             selected_monitor: 0,
//             ..Default::default()
//         };
//
//         app.handle_key_event(KeyCode::Char('m').into());
//         assert_eq!(app.mode, TUIMode::Move);
//
//         app.handle_key_event(KeyCode::Char('k').into());
//         let monitor = app.monitors[app.selected_monitor].clone();
//         assert_eq!(monitor.position.unwrap().y, -10);
//
//         app.handle_key_event(KeyCode::Char('j').into());
//         let monitor = app.monitors[app.selected_monitor].clone();
//         assert_eq!(monitor.position.unwrap().y, 0);
//
//         app.handle_key_event(KeyCode::Char('h').into());
//         let monitor = app.monitors[app.selected_monitor].clone();
//         assert_eq!(monitor.position.unwrap().x, -10);
//
//         app.handle_key_event(KeyCode::Char('l').into());
//         let monitor = app.monitors[app.selected_monitor].clone();
//         assert_eq!(monitor.position.unwrap().x, 0);
//
//         app.handle_key_event(KeyCode::Char('q').into());
//         assert!(app.exit);
//
//         Ok(())
//     }       
//     #[test]
//     fn handle_mode_resolution_key_event() -> io::Result<()> {
//         let mut app = App{
//             monitors: test_monitors(),
//             selected_monitor: 0,
//             ..Default::default()
//         };
//
//         app.handle_key_event(KeyCode::Char('r').into());
//         assert_eq!(app.mode, TUIMode::Resolution);
//
//         app.selected_resolution = 0;
//         app.handle_key_event(KeyCode::Char('j').into());
//         assert_eq!(app.selected_resolution, 1);
//
//         app.handle_key_event(KeyCode::Char('k').into());
//         assert_eq!(app.selected_resolution, 0);
//
//         app.handle_key_event(KeyCode::Char(' ').into());
//         let monitor = app.monitors[0].clone();
//         assert_eq!(monitor.modes[0].current, true);
//
//         app.handle_key_event(KeyCode::Char('q').into());
//         assert!(app.exit);
//
//         Ok(())
//     }    
//
//     #[test]
//     fn handle_mode_scale_key_event() -> io::Result<()> {
//         let mut app = App{
//             monitors: test_monitors(),
//             selected_monitor: 0,
//             ..Default::default()
//         };
//
//         app.handle_key_event(KeyCode::Char('s').into());
//         assert_eq!(app.mode, TUIMode::Scale);
//
//         app.selected_scale = 0;
//         app.handle_key_event(KeyCode::Char('j').into());
//         assert_eq!(app.selected_scale, 1);
//
//         app.handle_key_event(KeyCode::Char('k').into());
//         assert_eq!(app.selected_scale, 0);
//
//         app.handle_key_event(KeyCode::Char(' ').into());
//         let monitor = app.monitors[0].clone();
//         assert_eq!(monitor.scale, Some(0.5));
//
//         app.handle_key_event(KeyCode::Char('q').into());
//         assert!(app.exit);
//
//         Ok(())
//     }       
// }
