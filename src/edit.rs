use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style,Stylize,Color},
    symbols::border,
    text::Line,
    widgets::{Cell,Block,StatefulWidget,Row,Table,TableState,Paragraph,Borders},
};
use ratatui::prelude::*;
use ratatui::layout::Constraint;
use crate::printer::{
    Printer,
    Device,
    Driver
};
use crate::utils::{
    TUIMode,
    EditBlock,
    EditMode,
};
use crossterm::event::{self,Event,KeyCode,KeyEvent,KeyEventKind};
use crate::App;
#[derive(Debug)]
pub struct Edit<'a> {
    pub device_state: TableState,
    pub driver_state: TableState,
    pub selected_block: EditBlock,
    pub printer_name: &'a String,
    pub devices:&'a Vec<Device>,
    pub drivers: &'a Vec<Driver>,
}

impl<'a> Edit<'a> {
    pub fn new( selected_block:EditBlock,printer_name:&'a String,devices: &'a Vec<Device>,drivers: &'a Vec<Driver>,selected_device:Option<usize>,selected_driver:Option<usize>) -> Self {
        Edit{
            device_state: TableState::default()
                .with_selected(selected_device),
            driver_state: TableState::default()
                .with_selected(selected_driver),
            selected_block,
            printer_name,
            devices,
            drivers,
        }
    }

    pub fn handle_events(app:&mut App,key_event: KeyEvent) {
        match app.selected_edit_mode {
            EditMode::View => Edit::handle_view_mode(app,key_event),
            EditMode::Edit => Edit::handle_edit_mode(app,key_event),
        }
    }
    fn handle_view_mode(app:&mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k') => Edit::previous_block(app),
            KeyCode::Char('j') => Edit::next_block(app),
            KeyCode::Char('e') => app.selected_edit_mode = EditMode::Edit,
            KeyCode::Char('w') => {
                Edit::write(app);
                app.printers = Printer::get_all();
                app.change_mode(TUIMode::View);
            }
            KeyCode::Esc => app.change_mode(TUIMode::View),
            _ => {}
        }
    }

    fn handle_edit_mode(app:&mut App, key_event: KeyEvent) {
        match app.selected_edit_block {
            EditBlock::Title => Edit::handle_edit_title_mode(app,key_event),
            EditBlock::Devices => Edit::handle_edit_devices_mode(app,key_event),
            EditBlock::Drivers => Edit::handle_edit_drivers_mode(app,key_event),
        }   
    }

    fn handle_edit_title_mode(app:&mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => app.selected_printer_name.push(c),
            KeyCode::Backspace => {
                app.selected_printer_name.pop();
            }
            KeyCode::Esc => app.selected_edit_mode = EditMode::View,
            _ => {}
        }
    }

    fn handle_edit_devices_mode(app:&mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k') => Edit::previous_device(app),
            KeyCode::Char('j') => Edit::next_device(app),
            KeyCode::Esc => app.selected_edit_mode = EditMode::View,
            _ => {}
        }
    }

    fn handle_edit_drivers_mode(app:&mut App, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k') => Edit::previous_driver(app),
            KeyCode::Char('j') => Edit::next_driver(app),
            KeyCode::Esc => app.selected_edit_mode = EditMode::View,
            _ => {}
        }
    }
 
    fn next_block(app:&mut App) {
        match app.selected_edit_block {
            EditBlock::Title => app.selected_edit_block = EditBlock::Devices,
            EditBlock::Devices => app.selected_edit_block = EditBlock::Drivers,
            EditBlock::Drivers => app.selected_edit_block = EditBlock::Title
        }
    }

    fn previous_block(app:&mut App) {
        match app.selected_edit_block {
            EditBlock::Title => app.selected_edit_block = EditBlock::Drivers,
            EditBlock::Devices => app.selected_edit_block = EditBlock::Title,
            EditBlock::Drivers => app.selected_edit_block = EditBlock::Devices
        }
    }
    fn next_device(app:&mut App) {
        app.selected_device= if app.selected_device >= app.devices.len() - 1 {
            0
        } else {
            app.selected_device + 1
        };
    }

    fn previous_device(app:&mut App) {
        app.selected_device = if app.selected_device == 0 {
            app.devices.len() - 1
        } else {
            app.selected_device - 1
        };
    }

    fn next_driver(app:&mut App) {
        app.selected_driver = if app.selected_driver >= app.drivers.len() - 1 {
            0
        } else {
            app.selected_driver + 1
        };
    }

    fn previous_driver(app:&mut App) {
        app.selected_driver = if app.selected_driver == 0 {
            app.drivers.len() - 1
        } else {
            app.selected_driver- 1
        };
    }

    fn printers_to_rows(&self) -> Vec<Row<'static>> {
        self.devices
            .iter()
            .map(|monitor| {
                let backend = monitor.backend.clone().unwrap_or_else(|| "No description".to_string());
                let uri = monitor.uri.clone().unwrap_or_else(|| "No description".to_string());
                
                Row::new(vec![
                    Cell::from(backend),
                    Cell::from(uri),
                ])
            }
            )
            .collect()
    }
  
    fn write(app:&mut App) {
        Printer::create(app.selected_printer_name.clone(), 
            Some(app.devices.get(app.selected_device)
                .map_or("No URI".to_string(), |d| {
                    let backend: String = d.backend.clone().unwrap_or("No URI".to_string());
                    let uri: String = d.uri.clone().unwrap_or("No URI".to_string());
                    format!("{}:{}", backend, uri)
                })),
            Some(app.drivers.get(app.selected_driver)
                .map_or("No Driver".to_string(), |d| d.backend.clone().unwrap_or("No Driver".to_string())))
        );
    }    
    fn drivers_to_rows(&self) -> Vec<Row<'static>> {
        self.drivers
            .iter()
            .map(|driver| {
                let backend = driver.backend.clone().unwrap_or_else(|| "No description".to_string());
                let uri = driver.uri.clone().unwrap_or_else(|| "No description".to_string());
                
                Row::new(vec![
                    Cell::from(backend),
                    Cell::from(uri),
                ])
            }
            )
            .collect()
    }   
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {

        let mut instructions_items = vec![];

        match self.selected_block {
            EditBlock::Title => {
                instructions_items.push(" Quit Edit Mode ".white());
                instructions_items.push("<Esc> ".blue().bold());
            },
            EditBlock::Devices => {
                instructions_items.push(" Up ".white());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Quit Edit Mode ".white());
                instructions_items.push("<Esc> ".blue().bold());
            },
            EditBlock::Drivers => {
                instructions_items.push(" Up ".white());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Quit Edit Mode ".white());
                instructions_items.push("<Esc> ".blue().bold());
            }
        }

        instructions_items.push(" Quit ".white());
        instructions_items.push("<q> ".blue().bold());

        let instructions = Line::from(instructions_items);

        let edit_title = Line::from(" Edit ".white().bold());
        let edit_block = Block::default().borders(Borders::ALL)
            .title(edit_title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let inner_area = edit_block.inner(area);
        edit_block.render(area, buf);

        let form_layout = Layout::default()
             .direction(Direction::Vertical)
             .constraints(vec![
                 Constraint::Length(3),
                 Constraint::Percentage(50),
                 Constraint::Percentage(50),
             ])
             .split(inner_area);

        
        let printer_name_title = Line::from(" Printer Name ".bold());
        let printer_name_block = Block::default().title(printer_name_title).borders(Borders::ALL)
            .border_set(border::THICK)
            .border_style(Style::default().fg(
                if self.selected_block == EditBlock::Title {Color::Yellow} else {Color::White}));

        let printer_name_paragraph = Paragraph::new(self.printer_name.as_str())
            .block(printer_name_block)
            .style(Style::default().fg(Color::White));
        printer_name_paragraph.render(form_layout[0], buf);

        let devices_title = Line::from(" Available Devices ".bold());
        let devices_block = Block::default()
            .title(devices_title)
            .title_style(Style::default().fg(
                if self.selected_block == EditBlock::Devices {Color::Yellow} else {Color::White}));

        let devices_table_widths = [
            Constraint::Percentage(15),
            Constraint::Percentage(40),
        ];   

        let devices_table = Table::new(self.printers_to_rows(),devices_table_widths) 
            .column_spacing(1)
            .header(
                Row::new(vec![
                    Cell::from("name"),
                    Cell::from("description")])
                    .bottom_margin(1)
                    .bold()
                    .green()
                    .reversed()
            )
            .row_highlight_style(Style::new().yellow())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol("  ")
            .block(devices_block);
        
        StatefulWidget::render(
            devices_table,
            form_layout[1],
            buf,
            &mut self.device_state,
        );

        let drivers_title = Line::from(" Available Drivers ".bold());
        let drivers_block = Block::default()
            .title(drivers_title)
            .title_style(Style::default().fg(
                if self.selected_block == EditBlock::Drivers {Color::Yellow} else {Color::White}));

        let drivers_widths = [
            Constraint::Percentage(15),
            Constraint::Percentage(40),
        ];   

        let drivers_table = Table::new(self.drivers_to_rows(),drivers_widths) 
            .column_spacing(1)
            .header(
                Row::new(vec![
                    Cell::from("name"),
                    Cell::from("description")])
                    .bottom_margin(1)
                    .bold()
                    .green()
                    .reversed()
            )
            .row_highlight_style(Style::new().yellow())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol("  ")
            .block(drivers_block);
        
        StatefulWidget::render(
            drivers_table,
            form_layout[2],
            buf,
            &mut self.driver_state,
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
