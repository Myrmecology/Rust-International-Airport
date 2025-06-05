use crate::data::manager::DataManager;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use colored::*;
use std::io::{self, Write};
use std::error::Error;

pub struct MainMenu {
    data_manager: DataManager,
}

impl MainMenu {
    pub fn new(data_manager: DataManager) -> Self {
        Self { data_manager }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.display_main_menu()?;
            
            let choice = self.get_user_input("Enter your choice (1-7): ")?;
            
            match choice.trim() {
                "1" => self.search_flights().await?,
                "2" => self.book_flight().await?,
                "3" => self.manage_bookings().await?,
                "4" => self.flight_info().await?,
                "5" => self.aircraft_data().await?,
                "6" => self.admin_panel().await?,
                "7" => {
                    println!("\n{}", "Exiting system... Safe travels! ✈️".bright_green());
                    break;
                }
                _ => {
                    println!("\n{}", "❌ Invalid option! Please try again.".bright_red());
                    self.pause_for_user()?;
                }
            }
        }
        Ok(())
    }

    fn display_main_menu(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();
        
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║                      🛫 MAIN MENU 🛬                        ║".bright_cyan());
        println!("{}", "╠══════════════════════════════════════════════════════════════╣".bright_cyan());
        println!("{}", "║                                                              ║".bright_cyan());
        println!("{}", "║  1. 🔍 Search Flights                                       ║".bright_cyan());
        println!("{}", "║  2. 🎫 Book a Flight                                        ║".bright_cyan());
        println!("{}", "║  3. 📋 Manage Bookings                                      ║".bright_cyan());
        println!("{}", "║  4. ℹ️  Flight Info                                          ║".bright_cyan());
        println!("{}", "║  5. ✈️  Aircraft Data                                        ║".bright_cyan());
        println!("{}", "║  6. 🔧 Admin Panel                                          ║".bright_cyan());
        println!("{}", "║  7. 🚪 Exit                                                  ║".bright_cyan());
        println!("{}", "║                                                              ║".bright_cyan());
        println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_cyan());
        println!();

        Ok(())
    }

    fn get_user_input(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        print!("{}", prompt.bright_yellow());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn pause_for_user(&self) -> Result<(), Box<dyn Error>> {
        self.get_user_input("\nPress Enter to continue...")?;
        Ok(())
    }

    // Placeholder methods for menu options
    async fn search_flights(&self) -> Result<(), Box<dyn Error>> {
        println!("\n{}", "🔍 Search Flights - Coming Soon!".bright_blue());
        self.pause_for_user()?;
        Ok(())
    }

    async fn book_flight(&self) -> Result<(), Box<dyn Error>> {
        println!("\n{}", "🎫 Book Flight - Coming Soon!".bright_blue());
        self.pause_for_user()?;
        Ok(())
    }

    async fn manage_bookings(&self) -> Result<(), Box<dyn Error>> {
        println!("\n{}", "📋 Manage Bookings - Coming Soon!".bright_blue());
        self.pause_for_user()?;
        Ok(())
    }

    async fn flight_info(&self) -> Result<(), Box<dyn Error>> {
        println!("\n{}", "ℹ️ Flight Info - Coming Soon!".bright_blue());
        self.pause_for_user()?;
        Ok(())
    }

    async fn aircraft_data(&self) -> Result<(), Box<dyn Error>> {
        println!("\n{}", "✈️ Aircraft Data - Coming Soon!".bright_blue());
        self.pause_for_user()?;
        Ok(())
    }

    async fn admin_panel(&self) -> Result<(), Box<dyn Error>> {
        println!("\n{}", "🔧 Admin Panel - Coming Soon!".bright_blue());
        self.pause_for_user()?;
        Ok(())
    }
}