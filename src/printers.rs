use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style,Stylize,Color},
    symbols::border,
    text::Line,
    widgets::{Cell,Block,StatefulWidget,Row,Table,TableState},
};
use ratatui::prelude::*;
use ratatui::layout::Constraint;

use crate::cups::{
    printer::{
        Printer,
        remove_printer,
        get_all_printers,
    },
    device::get_all_devices,
    driver::get_all_drivers,
};

use crate::utils::{
    TUIMode,
    EditBlock,
    EditMode,
};

use crate::App;
use crossterm::event::{KeyCode,KeyEvent};

#[derive(Debug)]
pub struct Printers<'a> {
    pub state: TableState,
    pub printers:&'a Vec<Printer>,
    pub selected_printer: usize,    
}


impl<'a> Printers<'a> {
    pub fn new(printers: &'a Vec<Printer>,selected_printer:usize) -> Self {
        Printers{
            state: TableState::default()
                .with_selected(selected_printer),
            printers,
            selected_printer 
        }
    }
    pub fn handle_events(app:&mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k')=> Printers::previous_printer(app),
            KeyCode::Char('j')=> Printers::next_printer(app),
            KeyCode::Char('a')=> Printers::add_printer(app),
            KeyCode::Char('d')=> Printers::remove_printer(app),
            KeyCode::Char('q') => app.exit(),
            _ => {}
        }
    }

    fn next_printer(app:&mut App) {
        app.selected_printer= if app.selected_printer>= app.printers.len() - 1 {
            0
        } else {
            app.selected_printer + 1
        };
        app.selected_printer_name = app.printers.get(app.selected_printer)
            .map_or("No Printer".to_string(), |p| p.name.clone());
    }

    fn previous_printer(app:&mut App) {
        app.selected_printer= if app.selected_printer== 0 {
            app.printers.len() - 1
        } else {
            app.selected_printer- 1
        };
        app.selected_printer_name = app.printers.get(app.selected_printer)
            .map_or("No Printer".to_string(), |p| p.name.clone());
    }

    fn add_printer(app:&mut App) {
        app.printers.push(Printer::default()); 
        app.selected_printer = app.printers.len() - 1;
        app.selected_printer_name = app.printers.get(app.selected_printer)
            .map_or("No Printer".to_string(), |p| p.name.clone());
        app.selected_edit_block = EditBlock::Title;
        app.selected_edit_mode = EditMode::View;
        app.devices = get_all_devices();
        app.drivers = get_all_drivers();
        app.change_mode(TUIMode::Edit);
    }

    fn remove_printer(app:&mut App) {
        let printer_to_remove = app.printers.get(app.selected_printer)
            .map_or(Printer::default(), |p| p.clone());
        remove_printer(&printer_to_remove.name);
        app.printers = get_all_printers();
        app.selected_printer = 0;
    }

    fn printers_to_rows(&self) -> Vec<Row<'static>> {
        self.printers
            .iter()
            .map(|printer| {
                let name = printer.name.to_string();
                Row::new(vec![
                    Cell::from(name),
                ])
            })
            .collect()
    }
 
    fn options_to_rows(&self) -> Vec<Row<'static>> {
        self.printers[self.selected_printer].options
            .iter()
            .map(|option| {
                let name = option.to_string();
                Row::new(vec![
                    Cell::from(name),
                ])
            })
            .collect()
    }   

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
                 .direction(Direction::Horizontal)
                 .constraints(vec![
                     Constraint::Percentage(50),
                     Constraint::Percentage(50),
                 ])
                 .split(area);


        let title = Line::from(if self.printers.len()>1 {" Printers "}else{" Printer "}.white().bold());
        let mut instructions_items = vec![];
        instructions_items.push(" Up ".white());
        instructions_items.push("<k> ".blue().bold());
        instructions_items.push(" Down ".white());
        instructions_items.push("<j> ".blue().bold());
        instructions_items.push(" Add new printer ".white());
        instructions_items.push("<a> ".blue().bold());
        instructions_items.push(" Remove printer ".white());
        instructions_items.push("<d> ".blue().bold());
        instructions_items.push(" Quit ".white());
        instructions_items.push("<q> ".blue().bold());
        let instructions = Line::from(instructions_items);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(Color::White));

        let widths = [
            Constraint::Percentage(100),
        ];   

        let table = Table::new(self.printers_to_rows(),widths) 
            .column_spacing(1)
            .row_highlight_style(Style::new().yellow())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol("  ")
            .block(block);

        StatefulWidget::render(
            table,
            layout[0],
            buf,
            &mut self.state,
        );
        
        let title = Line::from(" Options ".white().bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(Color::White));

        let widths = [
            Constraint::Percentage(100),
        ];   

        let table = Table::new(self.options_to_rows(),widths) 
            .column_spacing(1)
            .block(block);

        StatefulWidget::render(
            table,
            layout[1],
            buf,
            &mut self.state,
        );
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ratatui::style::Style;
//     use crate::test_utils::tests::test_monitors;
//
//     #[test]
//     fn render_list() {
//         let mut list = MonitorList{
//             state: TableState::default(),
//             selected_row: Some(0),
//             mode: TUIMode::View,
//             monitors: &test_monitors(),
//         }; 
//         let mut buf = Buffer::empty(Rect::new(0, 0, 110, 7));
//
//         list.render(buf.area, &mut buf);
//
//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Displays ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
//             "┃     name              description                      resolution             position         scale      ┃",
//             "┃                                                                                                            ┃",
//             "┃     Monitor 1         Description 1                    1920x1080              (0,0)            1          ┃",
//             "┃     Monitor 2         Description 2                    1280x720               (1920,0)         1.25       ┃",
//             "┃                                                                                                            ┃",
//             "┗━━━━━━━━━━ Up <k>  Down <j>  Move <m>  Resolution <r>  Scale <s>  Disable <d>  Save <w>  Quit <q> ━━━━━━━━━━┛",
//         ]);
//
//         let border_style = Style::new().fg(Color::Yellow);
//         let title_style = Style::new().bold().fg(Color::White);
//         let header_style = Style::new().green().bold().reversed();
//         let empty_style = Style::new();
//         let instructions_label_style = Style::new().fg(Color::White);
//         let instructions_key_style = Style::new().blue().bold();
//         let connected_style = Style::new().fg(Color::Green);
//         let disconnected_style = Style::new().fg(Color::Red);
//         let row_style = Style::new();
//
//         // first line : title
//         expected.set_style(Rect::new(0, 0, 50, 1), border_style);
//         expected.set_style(Rect::new(50, 0, 10, 1), title_style);
//         expected.set_style(Rect::new(60, 0, 50, 1), border_style);       
//
//         // second line : header
//         expected.set_style(Rect::new(0, 1, 1, 1), border_style);
//         expected.set_style(Rect::new(1, 1, 108, 1), header_style);
//         expected.set_style(Rect::new(109, 1, 1, 1), border_style);
//
//         // third line : empty
//         expected.set_style(Rect::new(0, 2, 1, 1), border_style);
//         expected.set_style(Rect::new(1, 2, 108, 1), empty_style);
//         expected.set_style(Rect::new(109, 2, 1, 1), border_style);
//
//         // fourth line : first row 
//         expected.set_style(Rect::new(0, 3, 1, 1), border_style);
//         expected.set_style(Rect::new(1, 3, 5, 1), connected_style);
//         expected.set_style(Rect::new(6, 3, 103, 1), row_style);
//         expected.set_style(Rect::new(109, 3, 1, 1), border_style);      
//
//         // fifth line : second row 
//         expected.set_style(Rect::new(0, 4, 1, 1), border_style);
//         expected.set_style(Rect::new(1, 4, 5, 1), disconnected_style);
//         expected.set_style(Rect::new(6, 4, 103, 1), row_style);
//         expected.set_style(Rect::new(109, 4, 1, 1), border_style);   
//
//         // fifth line : empty
//         expected.set_style(Rect::new(0, 5, 1, 1), border_style);
//         expected.set_style(Rect::new(1, 5, 108, 1), empty_style);
//         expected.set_style(Rect::new(109, 5, 1, 1), border_style);
//
//         // last line : instructions 
//         expected.set_style(Rect::new(0,6,  11, 1), border_style);
//         expected.set_style(Rect::new(11, 6, 4, 1), instructions_label_style);
//         expected.set_style(Rect::new(15, 6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(19,6, 6, 1), instructions_label_style);
//         expected.set_style(Rect::new(25,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(29,6, 6, 1), instructions_label_style);
//         expected.set_style(Rect::new(35,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(39,6, 12, 1), instructions_label_style);
//         expected.set_style(Rect::new(51,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(55,6, 7, 1), instructions_label_style);
//         expected.set_style(Rect::new(62,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(66,6, 9, 1), instructions_label_style);
//         expected.set_style(Rect::new(75,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(79,6, 6, 1), instructions_label_style);
//         expected.set_style(Rect::new(85,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(89,6, 6, 1), instructions_label_style);
//         expected.set_style(Rect::new(95,6, 4, 1), instructions_key_style);
//
//         expected.set_style(Rect::new(99,6, 11, 1), border_style);
//
//         assert_eq!(buf, expected);
//     }
// }
