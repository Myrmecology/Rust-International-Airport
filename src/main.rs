use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Write};
use colored::*;

mod modules;
mod ui;
mod data;

use ui::menu::MainMenu;
use data::manager::DataManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal
    let mut stdout = io::stdout();
    
    // Clear screen and show welcome
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    // Display welcome banner
    display_welcome_banner()?;
    
    // Initialize data manager
    let mut data_manager = DataManager::new().await?;
    
    // Create and run main menu
    let mut main_menu = MainMenu::new(data_manager);
    main_menu.run().await?;

    // Clean exit
    execute!(stdout, ResetColor)?;
    println!("\n{}", "Thank you for using Rust International Airport! ✈️".bright_cyan());
    
    Ok(())
}

fn display_welcome_banner() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("╔══════════════════════════════════════════════════════════════╗\n"),
        Print("║                                                              ║\n"),
        Print("║            🛫  RUST INTERNATIONAL AIRPORT  🛬               ║\n"),
        Print("║                                                              ║\n"),
        Print("║              Professional Airport Management System          ║\n"),
        Print("║                        Version 1.0.0                        ║\n"),
        Print("║                                                              ║\n"),
        Print("╚══════════════════════════════════════════════════════════════╝\n"),
        ResetColor,
        Print("\n")
    )?;
    
    stdout.flush()?;
    std::thread::sleep(std::time::Duration::from_millis(1500));
    
    Ok(())
}