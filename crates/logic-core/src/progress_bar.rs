

use std::io::{self, Write};
use std::time::{Duration, Instant};
use crossterm::cursor::position;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

/// 进度条结构体
pub struct ProgressBar {
    total: u64,            // 总任务数
    current: u64,          // 当前完成数
    start_time: Instant,   // 开始时间
    width: Option<u16>,    // 进度条宽度
    message: String,       // 自定义消息
    bar_style: BarStyle,   // 进度条样式
    show_percentage: bool, // 是否显示百分比
    show_count: bool,      // 是否显示计数
    show_time: bool,       // 是否显示时间
    row: Option<u16>,
}

/// 进度条样式配置
pub struct BarStyle {
    pub complete_char: char,    // 完成部分字符
    pub incomplete_char: char,  // 未完成部分字符
    pub start_char: char,       // 起始字符
    pub end_char: char,         // 结束字符
    pub complete_color: Color,  // 完成部分颜色
    pub incomplete_color: Color, // 未完成部分颜色
}

impl Default for BarStyle {
    fn default() -> Self {
        BarStyle {
            complete_char: '█',
            incomplete_char: '░',
            start_char: '[',
            end_char: ']',
            complete_color: Color::Green,
            incomplete_color: Color::DarkGrey,
        }
    }
}

impl ProgressBar {
    /// 创建新进度条
    pub fn new(total: u64) -> Self {
        ProgressBar {
            total,
            current: 0,
            start_time: Instant::now(),
            width: None,
            message: String::new(),
            bar_style: BarStyle::default(),
            show_percentage: true,
            show_count: true,
            show_time: true,
            row: None,
        }
    }

    /// 设置自定义消息
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// 设置进度条宽度
    pub fn with_width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置进度条样式
    pub fn with_style(mut self, style: BarStyle) -> Self {
        self.bar_style = style;
        self
    }

    /// 更新进度
    pub fn update(&mut self, current: u64) -> io::Result<()> {
        if self.row.is_none() {
            let (_, row) = position()?;
            self.row = Some(row);
        }
        self.current = current.min(self.total);
        self.render()
    }

    /// 增加进度
    pub fn inc(&mut self, delta: u64) -> io::Result<()> {
        self.update(self.current + delta)
    }

    /// 完成进度条
    pub fn finish(&mut self) -> io::Result<()> {
        self.current = self.total;
        self.render()?;
        
        // 完成后换行
        println!();
        Ok(())
    }

    /// 渲染进度条
    fn render(&self) -> io::Result<()> {

        let width = self.width.unwrap_or_else(|| {
            crossterm::terminal::size()
                .map(|(w, _)| w - 20) // 为其他信息留空间
                .unwrap_or(50)
        }) as usize;

        // 计算进度百分比
        let percent = if self.total > 0 {
            self.current as f64 / self.total as f64
        } else {
            0.0
        };

        // 构建进度条
        let bar_width = width - 2; // 减去两边的括号
        let completed_len = (percent * bar_width as f64) as usize;
        let remaining_len = bar_width - completed_len;

        // 隐藏光标避免闪烁
        execute!(io::stdout(), cursor::Hide)?;

        // 移到行首并清除行
        execute!(
            io::stdout(),
            cursor::MoveToRow(self.row.unwrap()-1),
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine)
        )?;

        // 打印进度条
        execute!(
            io::stdout(),
            Print(self.bar_style.start_char),
            SetForegroundColor(self.bar_style.complete_color),
            Print(self.bar_style.complete_char.to_string().repeat(completed_len)),
            SetForegroundColor(self.bar_style.incomplete_color),
            Print(self.bar_style.incomplete_char.to_string().repeat(remaining_len)),
            ResetColor,
            Print(self.bar_style.end_char),
            Print(" ")
        )?;

        // 打印附加信息
        if self.show_percentage {
            execute!(
                io::stdout(),
                Print(format!("{:.1}%", percent * 100.0)),
                Print(" ")
            )?;
        }

        if self.show_count {
            execute!(
                io::stdout(),
                Print(format!("({}/{}", self.current, self.total)),
                Print(") ")
            )?;
        }

        if self.show_time {
            let elapsed = self.start_time.elapsed();
            let remaining = if percent > 0.0 {
                Duration::from_secs_f64(elapsed.as_secs_f64() / percent * (1.0 - percent))
            } else {
                Duration::from_secs(0)
            };
            let (col, row) = position().unwrap();
            execute!(
                io::stdout(),
                Print(format!(
                    "已用: {} 剩余: {}",
                    format_duration(elapsed),
                    format_duration(remaining)
                )),
                Print(format!("column: {}, row: {}", col, row)),
            )?;
        }

        // 打印自定义消息
        if !self.message.is_empty() {
            execute!(io::stdout(), Print(" | "), Print(&self.message))?;
        }

        // 立即刷新输出
        io::stdout().flush()?;
        
        // 恢复光标显示
        execute!(io::stdout(), cursor::Show)?;

        Ok(())
    }
}

/// 格式化时间为 mm:ss
fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", mins, secs)
}