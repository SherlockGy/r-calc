use std::io;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

// 打印带颜色的文本并换行
pub fn println_colored(text: &str, color: Color) -> io::Result<()> {
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
        Print("\n")
    )
}

// 打印带颜色的文本
pub fn print_colored(text: &str, color: Color) -> io::Result<()> {
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor
    )
}