#![allow(unused)]
use std::{io::{stdout, Write}, time::Duration};

use crossterm::{cursor::{position, DisableBlinking, EnableBlinking, MoveDown, MoveTo, MoveUp}, event::{poll, read, Event, KeyCode, KeyModifiers}, execute, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, size, window_size, Clear, ClearType, WindowSize}};

use crate::Result;

pub enum Direction {
  Up,
  Down,
  Left,
  Right,
  Unknown,
}

impl From<KeyCode> for Direction {
  fn from(value: KeyCode) -> Self {
      match value {
        KeyCode::Up => Direction::Up,
        KeyCode::Down => Direction::Down,
        KeyCode::Left => Direction::Left,
        KeyCode::Right => Direction::Right,
        _ => Direction::Unknown,
      }
  }
}

struct Char {
  content: char,
}

impl Char {
  fn new(content: char) -> Self {
    Self {
      content
    }
  }
}

impl Default for Char {
  fn default() -> Char {
      Char::new(' ')
  }
}

struct Line {
  content: Vec<Char>,
  is_active: bool, // 是否光标所在
}

impl Line {
  fn new() -> Self {
    Self { content: Vec::new(), is_active: false }
  }
  /// 计算所占可视区行数
  pub fn calculate_lines(&self) -> Result<u16> {
    let WindowSize {
      width,
      height,
      ..
    } = window_size()?;
    let length = self.content.len() as u16;
    let mut lines = length / width;
    if length % height != 0 {
      lines += 1;
    }
    Ok(lines)
  }
}

pub struct Editor {
  buffer: Vec<Line>,

  /// 当前指针所在的定位
  cursor_row: u16,
  cursor_col: u16,

  /// 最初开始的定位
  start_row: u16,
  start_col: u16,

  terminal_scroll_distance: i16,
  terminal_width: u16,
  terminal_height: u16,

  /// 开始光标距离终端顶部边界的距离（单位：行）
  /// 可以为负数，如果为负数则表示开始坐标已经不在可视区了
  start_cursor_margin_top: i16,
}

impl Editor {
  pub fn new() -> Result<Self> {
    let (terminal_width, terminal_height) = size()?;
    let (start_col, start_row) = position()?;
    Ok(Self {
      buffer: Vec::new(),
      cursor_row: start_row,
      cursor_col: start_col,

      start_row,
      start_col,
      start_cursor_margin_top: start_row as i16,
      terminal_width,
      terminal_height,
      terminal_scroll_distance: 0,
    })
  }

  pub fn start_position(&mut self, position: (u16, u16)) {
    self.start_col = position.0;
    self.start_row = position.1;
  }

  pub fn current_position(&mut self, position: (u16, u16)) {
    self.cursor_col = position.0;
    self.cursor_row = position.1;
  }

  pub fn get_cursor(&self) -> (u16, u16) {
    (self.cursor_col, self.cursor_row)
  }

  /// 计算当前开始定位 -> 计算内容动态变化之后的开始定位
  fn calculate_start_position(&self) -> Result<()> {

    Ok(())
  }

  /// 手动触发鼠标移动
  pub fn manual_move_cursor(&mut self, direction: Direction) -> Result<()> {
    /// 思路
    /// 1、判断光标是否在内容区范围内
    /// 2、如果在，可正常移动光标，如果在临界点，则不移动光标
    /// 3、判断光标是否
    match direction {
      Direction::Up => {
        
      }
      Direction::Down => {

      }
      Direction::Left => {
        self.cursor_col -= 1;
      }
      Direction::Right => {
        let (buf_cursor_row, curr_start_line_in_screen) = self.get_line_by_cursor()?;
        let index = (self.cursor_row - curr_start_line_in_screen) * self.terminal_width + self.cursor_col;
        if index >= self.buffer[buf_cursor_row as usize].content.len() as u16 {
          // 超出行范围，不允许继续向右移动，应该向下换行（如果存在下一行的话）
          if self.buffer.len() - 1 > buf_cursor_row as usize {
            // 光标移动到下一行
            self.cursor_col = 0;
            if self.cursor_row == self.terminal_height - 1 {
              self.terminal_scroll_distance = 1;
              // 需要维护start位置
              self.start_row -= 1;
            } else {
              self.cursor_row += 1;
            }
          } else {
            // 不允许移动光标
          }
        } else {
          self.cursor_col += 1;
        }
      }
      _ => {

      }
    }
    Ok(())
  }

  /// 处理移动鼠标（用于输入字符，光标自动右移）
  fn auto_move_cursor(&mut self) -> Result<()> {
    if self.cursor_col == self.terminal_width - 1 {
      self.cursor_col = 0;
      if self.cursor_row == self.terminal_height - 1 {
        self.terminal_scroll_distance = 1;
      } else {
        self.cursor_row += 1;
      }
    } else {
      self.cursor_col += 1;
    }
    Ok(())
  }

  fn insert_line(&mut self) -> Result<()> {
    let line_struct = Line::new();
    self.buffer.push(line_struct);
    Ok(())
  }

  /// 根据坐标得出所在行数
  /// 0:所在buffer的行数，1:所在终端的起始行
  fn get_line_by_cursor(&self) -> Result<(u16, u16)> {
    let mut total_lines = self.start_cursor_margin_top;
    let mut position_index = 0u16;
    let mut start_line_in_buffer = 0;

    for (index, val) in self.buffer.iter().enumerate() {
      total_lines = total_lines + (val.calculate_lines()? as i16);
      if (self.cursor_row as i16) <= total_lines {
        start_line_in_buffer = total_lines - (val.calculate_lines()? as i16);
        position_index = index as u16;
        break;
      }
    }

    Ok((position_index, start_line_in_buffer as u16))
  }

  /// 插入字符
  pub fn insert_char(&mut self, c: char) -> Result<()> {
    if self.buffer.len() == 0 {
      self.insert_line()?;
    }

    let c_struct = Char::new(c);
    let (index, start_line) = self.get_line_by_cursor()?;

    let i = if start_line != self.cursor_row {
      (self.cursor_row-start_line) * self.terminal_width + self.cursor_col
    } else { self.cursor_col };

    let res = &mut self.buffer[index as usize];

    res.content.insert(i as usize, c_struct);
    self.auto_move_cursor()?;

    Ok(())
  }

  fn delete_char(&mut self) -> Result<()> {

    Ok(())
  }

  /// 渲染数据上屏
  fn render(&mut self) -> Result<()> {
    // 1、先移动到开始坐标
    // 2、清空开始坐标之后的内容
    // 3、重新绘制整个内容

    let mut stdout = stdout();

    queue!(
      stdout,
      MoveTo(self.start_col, self.start_row),
      Clear(ClearType::FromCursorDown),
    )?;

    self.buffer.iter().for_each(|item| {
      
      let line_count = item.calculate_lines().unwrap();

      if line_count + self.cursor_row >= self.terminal_height {
        // 表示内容已经超出了范围
        let distance = line_count+self.cursor_row-self.terminal_height;
        // 接下来需要滚动预留空间，并维护terminal_scroll_distance字段
        // 1、滚动到distance+1
        queue!(stdout, MoveUp(distance+1)).unwrap();
        // 2、self.terminal_scroll_distance-(distance+1)
        self.terminal_scroll_distance = self.terminal_scroll_distance - (distance as i16) - 1;
      }

      let strs: String = item.content.iter().map(|i| i.content.to_string()).collect();
      queue!(stdout, Print(strs)).unwrap();
    });

    Ok(())
  }


  pub fn run(&mut self) -> Result<()> {

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnableBlinking)?;

    loop {
        if poll(Duration::from_millis(51))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Char(c)
                        if c == 'c' && key_event.modifiers == KeyModifiers::CONTROL =>
                    {
                        break;
                    }
                    KeyCode::Char(c) => {
                      self.insert_char(c)?;
                    }
                    direction @ (KeyCode::Down|KeyCode::Up|KeyCode::Left|KeyCode::Right) => {
                      self.manual_move_cursor(Direction::from(direction))?;
                    }
                    _ => {}
                }

                if self.terminal_scroll_distance > 0 {
                  queue!(stdout, MoveUp(self.terminal_scroll_distance as u16))?;
                } else if self.terminal_scroll_distance < 0 {
                  queue!(stdout, MoveDown(self.terminal_scroll_distance.abs() as u16))?;
                }


                self.render()?;

                
                self.terminal_scroll_distance = 0;
                
                queue!(
                  stdout, 
                  MoveTo(self.cursor_col, self.cursor_row)
                )?;
                stdout.flush()?;
            }
            
        }
    }

    execute!(stdout, DisableBlinking)?;
    disable_raw_mode()?;

    Ok(())
  }
}

