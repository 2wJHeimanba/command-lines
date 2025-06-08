use std::{
    io::{stdout, Read, Write},
    os::fd::{AsFd, AsRawFd},
    time::Duration,
};

use crossterm::{
    cursor::{self, position, MoveTo, MoveToNextLine},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap},
};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn commands() -> Res<Vec<String>> {
    let mut current_dir = std::env::current_dir()?;

    current_dir.push("his.li");

    let mut file = std::fs::OpenOptions::new().read(true).open(&current_dir)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lines: Vec<String> = content.split('\n').map(|item| item.to_string()).collect();

    Ok(lines)
}

fn handle_backspace() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();

    execute!(
        stdout,
        cursor::MoveLeft(1),
        Print(""),
        // cursor::MoveLeft(1),
    )?;

    Ok(())
}

fn print_repeat(cont: char, repeat_count: usize) {
    let mut stdout = std::io::stdout();
    let content = cont.to_string().repeat(repeat_count);
    execute!(stdout, Print(content)).unwrap();
}

// 字符的基本信息，可能涉及到多字节字符
struct Char {
    content: char,
    length: u8, // 字符的字节长度
}

impl Char {
    fn new(content: char, length: u8) -> Self {
        Self {
            content,
            length,
        }
    }
}

macro_rules! new_char {
    ($content: expr) => {
        Char::new($content, 1u8)
    };
    ($content: expr, $length: expr) => {
        Char::new($content, $length)
    }
}




struct Editor {
    buffer: Vec<Vec<char>>,
    cursor_row: usize,
    cursor_col: usize,
    start_row: u16,
    start_col: u16,
    terminal_width: u16,
    terminal_height: u16,
}

impl Editor {
    fn new(start_row: u16, start_col: u16) -> Res<Self> {
        let (terminal_width, terminal_height) = size()?;
        let (cursor_col, cursor_row) = position()?;
        let buffer = vec![vec!['h','e'], vec!['h','e'], vec!['h','e']];
        Ok(Self {
            buffer,
            cursor_row: cursor_row as usize,
            cursor_col: cursor_col as usize,
            start_row,
            start_col,
            terminal_width,
            terminal_height,
        })
    }

    fn initial(&self) -> Res<()> {

        Ok(())
    }

    fn refresh(&mut self) -> Res<()> {
        let mut stdout = stdout();
        for (index, line) in self.buffer.iter().enumerate() {
            if index > 0 {
                if self.cursor_row == (self.terminal_height - 1) as usize {
                    queue!(stdout, terminal::ScrollUp(1))?;
                }
                queue!(stdout, MoveToNextLine(1))?;
            }
            queue!(stdout, Print(line.iter().collect::<String>()))?;
        }
        self.move_cursor()?;
        stdout.flush()?;

        Ok(())
    }

    fn move_cursor(&self) -> Res<()> {
        // let mut stdout = stdout();
        
        // // 计算相对起始位置的光标位置
        // let mut row_offset = 0;
        // let mut col_offset = 0;
        
        // // 计算当前行之前的所有行占用的行数
        // for i in 0..self.cursor_row {
        //     let line_len = self.buffer[i].len();
        //     row_offset += (line_len as u16 + col_offset) / self.terminal_width;
        //     col_offset = (line_len as u16 + col_offset) % self.terminal_width;
        // }
        
        // // 计算当前行的位置
        // let current_line_len = self.buffer[self.cursor_row][..self.cursor_col].len();
        // let total_offset = col_offset + current_line_len as u16;
        // row_offset += total_offset / self.terminal_width;
        // col_offset = total_offset % self.terminal_width;
        
        // // 移动光标
        // execute!(
        //     stdout,
        //     MoveTo(
        //         self.start_col + col_offset,
        //         self.start_row + row_offset
        //     )
        // )?;

        Ok(())
    }

    // 处理按键输入
    fn process_key(&mut self, key: KeyEvent) -> Res<bool> {
        match key.code {
            KeyCode::Char(c) if c == 'c' && key.modifiers == KeyModifiers::CONTROL => {
                // Ctrl+C 退出
                return Ok(true);
            }
            KeyCode::Char(c) => {
                // 普通字符输入
                self.insert_char(c);
            }
            KeyCode::Backspace => {
                // 退格键删除
                self.delete_char();
            }
            KeyCode::Enter => {
                // 回车键换行
                self.insert_newline();
            }
            KeyCode::Left => {
                // 左移光标
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.buffer[self.cursor_row].len();
                }
            }
            KeyCode::Right => {
                // 右移光标
                if self.cursor_col < self.buffer[self.cursor_row].len() {
                    self.cursor_col += 1;
                } else if self.cursor_row < self.buffer.len() - 1 {
                    self.cursor_row += 1;
                    self.cursor_col = 0;
                }
            }
            KeyCode::Up => {
                // 上移光标
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.cursor_col.min(self.buffer[self.cursor_row].len());
                }
            }
            KeyCode::Down => {
                // 下移光标
                if self.cursor_row < self.buffer.len() - 1 {
                    self.cursor_row += 1;
                    self.cursor_col = self.cursor_col.min(self.buffer[self.cursor_row].len());
                }
            }
            _ => {}
        }

        // 刷新显示
        self.refresh()?;
        Ok(false)
    }

    // 插入字符
    fn insert_char(&mut self, c: char) {
        if self.cursor_row < self.buffer.len() {
            self.buffer[self.cursor_row].insert(self.cursor_col, c);
            self.cursor_col += 1;
        }
    }

    // 插入换行符
    fn insert_newline(&mut self) {
        // 分割当前行
        let current_line = self.buffer[self.cursor_row].split_off(self.cursor_col);

        // 插入新行
        self.buffer.insert(self.cursor_row + 1, current_line);

        // 移动光标到新行开头
        self.cursor_row += 1;
        self.cursor_col = 0;
    }

    // 删除字符
    fn delete_char(&mut self) {
        if self.cursor_col > 0 {
            // 删除当前行中的字符
            self.buffer[self.cursor_row].remove(self.cursor_col - 1);
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            // 删除换行符（合并行）
            let current_row = self.cursor_row;
            let prev_row_len = self.buffer[current_row - 1].len();

            // 将当前行内容添加到前一行
            let mut current_line = self.buffer.remove(current_row);
            self.buffer[current_row - 1].append(&mut current_line);

            // 更新光标位置
            self.cursor_row -= 1;
            self.cursor_col = prev_row_len;
        }
    }

    // 获取完整文本
    fn get_text(&self) -> String {
        self.buffer
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置终端原始模式
    enable_raw_mode()?;

    let mut stdout = stdout();
    // execute!(stdout, DisableLineWrap)?;

    // 获取起始位置
    let (_, start_row) = crossterm::cursor::position()?;
    let start_col = 0;
    // 创建编辑器
    let mut editor = Editor::new(start_row, start_col)?;
    editor.refresh()?;

    // 事件循环
    loop {
        let event = read()?;

        if let Event::Key(key_event) = event {
            if editor.process_key(key_event)? {
                break;
            }
        }
    }

    execute!(stdout, EnableLineWrap)?;

    // 退出前恢复终端设置
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
