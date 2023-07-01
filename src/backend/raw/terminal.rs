use crate::{backend::raw::cell::Cell, mask::Style};

use std::ops::{Index, RangeBounds};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::cell::Flag;

/// Terminal emulator just for tests.
#[derive(Default)]
pub(crate) struct Terminal {
    grid: Vec<Cell>,
    width: usize,
    height: usize,
    cursor: Cursor,
}

#[derive(Default, Debug)]
struct Cursor {
    column: usize,
    line: usize,
}

impl Terminal {
    pub fn new(mut width: usize, mut height: usize) -> Self {
        let grid = vec![Cell::default(); width * height];
        Self { grid, width, height, ..Self::default() }
    }

    pub fn move_cursor(&mut self, column: usize, line: usize) {
        assert!(column < self.width);
        assert!(line < self.height);

        self.cursor.column = column;
        self.cursor.line = line;
    }

    pub fn line_to_string(&self, line: usize) -> String {
        let mut result = String::with_capacity(self.width);
        let mut column = 0;
        while column < self.width {
            let cell = self.get(column, line);

            result.push_str(&cell.grapheme);

            if cell.flags.contains(Flag::WIDE_START) {
                assert!(self.get(column + 1, line).flags.contains(Flag::WIDE_END));
                column += 1;
            } else if cell.flags.contains(Flag::WIDE_LEADING) {
                assert!(column == self.width - 1);
                // Nothing yet
            }

            column += 1;
        }

        result
    }

    pub fn write_str(&mut self, s: &str, styles: Style) {
        if self.is_zero_terminal() {
            return;
        }
        for g in UnicodeSegmentation::graphemes(s, true) {
            let width = UnicodeWidthStr::width(g);

            if width == 0 {
                match g {
                    "\n" | "\r\n" => {
                        self.next_line();
                        continue;
                    }
                    _ => {}
                }
            } else if width == 1 {
                *self.current_cell() = Cell::new(g, styles);
            } else {
                if self.cursor.column + 1 >= self.width {
                    *self.current_cell() = Cell::wide_leading();
                    self.current_cell().style = styles;
                    self.next_line();
                }

                let cell = self.current_cell();
                *cell = Cell::new(g, styles);
                cell.flags = Flag::WIDE_START;

                self.next_column();
                *self.current_cell() = Cell::wide_end();
                self.current_cell().style = styles;
            }

            self.next_column();
        }
    }

    /// Fill from current cursor position to the end with **1** grapheme with width = 1.
    pub fn fill(&mut self, s: &str) {
        if self.is_zero_terminal() {
            return;
        }

        assert_eq!(UnicodeSegmentation::graphemes(s, true).count(), 1);
        assert_eq!(UnicodeWidthStr::width(s), 1);

        loop {
            *self.current_cell() = Cell::new_str(s);
            if self.cursor.line == self.height - 1 && self.cursor.column == self.width - 1 {
                break;
            }
            self.next_column();
        }
    }

    pub fn lines_to_vec(&self) -> Vec<String> {
        (0..self.height).map(|line| self.line_to_string(line)).collect()
    }

    pub fn get(&self, column: usize, line: usize) -> &Cell {
        &self.grid[self.coord((column, line))]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_zero_terminal(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn assert_styles<R>(&self, column: R, line: R, style: Style)
    where
        R: RangeBounds<usize>,
    {
        // let column = bound_to_range(column);
        // let line = bound_to_range(line);
        //
        // assert!(*column.end() < self.width);
        // assert!(*line.end() < self.height);
        //
        // for l in line {
        //     for c in column.clone() {
        //         let cell = self.get(c, l);
        //         assert_eq!(cell.style, style, " (column {}, line {})", c, l);
        //     }
        // }
    }

    fn current_cell(&mut self) -> &mut Cell {
        let index = self.coord((self.cursor.column, self.cursor.line));
        &mut self.grid[index]
    }

    fn next_column(&mut self) {
        if self.cursor.column + 1 >= self.width {
            self.next_line();
        } else {
            self.cursor.column += 1;
        }
    }

    fn next_line(&mut self) {
        if self.cursor.line + 1 < self.height {
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
    }

    fn coord(&self, (column, line): (usize, usize)) -> usize {
        self.width * line + column
    }
}

impl Index<(usize, usize)> for Terminal {
    type Output = Cell;

    fn index(&self, (column, line): (usize, usize)) -> &Self::Output {
        self.get(column, line)
    }
}

#[cfg(test)]
mod tests {
    use crate::mask::Color;

    use super::*;

    #[test]
    fn line_to_string() {
        let mut terminal = Terminal::new(5, 4);

        let mut widecell = Cell::new_str("老");
        widecell.flags |= Flag::WIDE_START;

        terminal.grid = vec![
            // 0
            Cell::new_str("h"),
            Cell::new_str("e"),
            Cell::new_str("l"),
            Cell::new_str("l"),
            Cell::new_str("o"),
            // 1
            widecell.clone(),
            Cell::wide_end(),
            Cell::new_str(" "),
            Cell::new_str("!"),
            Cell::new_str("@"),
            // 2
            Cell::new_str("1"),
            Cell::new_str("2"),
            Cell::new_str("3"),
            Cell::new_str("4"),
            Cell::wide_leading(),
            // 3
            widecell.clone(),
            Cell::wide_end(),
            Cell::new_str(" "),
            Cell::new_str("!"),
            Cell::new_str("y\u{0306}"),
        ];

        let line0 = "hello";
        let line1 = "老 !@";
        let line2 = "1234 ";
        let line3 = "老 !y\u{0306}";

        assert_eq!(terminal.line_to_string(0), line0);
        assert_eq!(terminal.line_to_string(1), line1);
        assert_eq!(terminal.line_to_string(2), line2);
        assert_eq!(terminal.line_to_string(3), line3);

        assert_eq!(terminal.lines_to_vec(), vec![line0, line1, line2, line3]);
    }

    #[test]
    fn write_str() {
        let mut terminal = Terminal::new(5, 5);

        terminal.write_str("hello", Style::default());
        terminal.write_str("老!34", Style::new().bg(Color::Yellow));

        terminal.write_str("1", Style::new().fg(Color::Red));
        terminal.write_str("2", Style::new().fg(Color::Blue));
        terminal.write_str("3", Style::new().fg(Color::Green));
        terminal.write_str("4", Style::new().fg(Color::Yellow));
        terminal.write_str("老", Style::new().fg(Color::Magenta));

        terminal.write_str("y\u{0306}et more tex@", Style::default());

        #[rustfmt::skip]
        let lines = vec![
            "hello", 
            "老!34", 
            "1234 ", 
            "老y\u{0306}et", 
            " mor@"
        ];

        assert_eq!(terminal.lines_to_vec(), lines);
        terminal.assert_styles(0..=4, 1..=1, Style::new().bg(Color::Yellow));
        assert_eq!(terminal[(0, 2)].style, Style::new().fg(Color::Red));
        assert_eq!(terminal[(1, 2)].style, Style::new().fg(Color::Blue));
        assert_eq!(terminal[(2, 2)].style, Style::new().fg(Color::Green));
        assert_eq!(terminal[(3, 2)].style, Style::new().fg(Color::Yellow));
        assert_eq!(terminal[(4, 2)].style, Style::new().fg(Color::Magenta));
    }

    #[test]
    fn write_str_with_new_line() {
        let mut terminal = Terminal::new(4, 3);

        terminal.write_str("tex\ntext\nt", Style::default());

        #[rustfmt::skip]
        let lines = vec![
            "tex ",
            "text",
            "t   "
        ];

        assert_eq!(terminal.lines_to_vec(), lines);
    }

    #[test]
    fn fill() {
        let mut terminal = Terminal::new(5, 5);
        terminal.fill("h");

        let lines = vec!["hhhhh"; 5];

        assert_eq!(terminal.lines_to_vec(), lines);
    }
}
