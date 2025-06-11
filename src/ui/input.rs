use colored::*;
use std::io::{self, Write};
use chrono::{DateTime, Utc, NaiveDate, TimeZone};
use uuid::Uuid;
use crate::modules::{
    flight::SeatClass,
    booking::{Passenger, PassengerType},
    airport::Airport,
};

pub struct InputManager;

impl InputManager {
    pub fn new() -> Self {
        Self
    }

    // Basic input functions
    pub fn get_string_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        print!("{} ", prompt.bright_yellow());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn get_string_input_with_validation(
        &self, 
        prompt: &str,
        validator: fn(&str) -> bool,
        error_message: &str
    ) -> Result<String, Box<dyn std::error::Error>> {
        loop {
            let input = self.get_string_input(prompt)?;
            if validator(&input) {
                return Ok(input);
            }
            println!("{} {}", "❌".bright_red(), error_message.bright_red());
        }
    }

    pub fn get_number_input<T>(&self, prompt: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        loop {
            let input = self.get_string_input(prompt)?;
            match input.parse::<T>() {
                Ok(number) => return Ok(number),
                Err(e) => {
                    println!("{} Invalid number format: {}", "❌".bright_red(), e.to_string().bright_red());
                }
            }
        }
    }

    pub fn get_number_input_with_range<T>(
        &self, 
        prompt: &str, 
        min: T, 
        max: T
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: std::str::FromStr + std::cmp::PartialOrd + std::fmt::Display + Copy,
        T::Err: std::fmt::Display,
    {
        loop {
            let number = self.get_number_input::<T>(prompt)?;
            if number >= min && number <= max {
                return Ok(number);
            }
            println!("{} Number must be between {} and {}", 
                "❌".bright_red(), 
                min.to_string().bright_yellow(), 
                max.to_string().bright_yellow());
        }
    }

    pub fn get_yes_no_input(&self, prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
        loop {
            let input = self.get_string_input(&format!("{} (y/n)", prompt))?;
            match input.to_lowercase().as_str() {
                "y" | "yes" | "1" | "true" => return Ok(true),
                "n" | "no" | "0" | "false" => return Ok(false),
                _ => {
                    println!("{} Please enter 'y' for yes or 'n' for no", "❌".bright_red());
                }
            }
        }
    }

    // Specialized input functions for airport system
    pub fn get_airport_code_input(&self, prompt: &str, airports: &[Airport]) -> Result<String, Box<dyn std::error::Error>> {
        println!("\n{}", "Available Airports:".bright_cyan().bold());
        for airport in airports {
            println!("  {} - {} ({})", 
                airport.code.bright_green().bold(), 
                airport.name.bright_white(),
                airport.city.bright_cyan());
        }
        println!();

        loop {
            let input = self.get_string_input(prompt)?;
            let code = input.to_uppercase();
            
            if airports.iter().any(|a| a.code == code) {
                return Ok(code);
            }
            
            println!("{} Invalid airport code. Please choose from the list above.", "❌".bright_red());
        }
    }

    pub fn get_seat_class_input(&self) -> Result<SeatClass, Box<dyn std::error::Error>> {
        println!("\n{}", "Available Seat Classes:".bright_cyan().bold());
        println!("  {} - Economy Class", "1".bright_green().bold());
        println!("  {} - Business Class", "2".bright_yellow().bold());
        println!("  {} - First Class", "3".bright_magenta().bold());
        println!();

        loop {
            let input = self.get_string_input("Select seat class (1-3):")?;
            match input.as_str() {
                "1" => return Ok(SeatClass::Economy),
                "2" => return Ok(SeatClass::Business),
                "3" => return Ok(SeatClass::FirstClass),
                _ => {
                    println!("{} Please enter 1, 2, or 3", "❌".bright_red());
                }
            }
        }
    }

    pub fn get_passenger_type_input(&self) -> Result<PassengerType, Box<dyn std::error::Error>> {
        println!("\n{}", "Passenger Types:".bright_cyan().bold());
        println!("  {} - Adult (18+ years)", "1".bright_green().bold());
        println!("  {} - Child (2-17 years)", "2".bright_yellow().bold());
        println!("  {} - Infant (under 2 years)", "3".bright_blue().bold());
        println!("  {} - Senior (65+ years)", "4".bright_magenta().bold());
        println!();

        loop {
            let input = self.get_string_input("Select passenger type (1-4):")?;
            match input.as_str() {
                "1" => return Ok(PassengerType::Adult),
                "2" => return Ok(PassengerType::Child),
                "3" => return Ok(PassengerType::Infant),
                "4" => return Ok(PassengerType::Senior),
                _ => {
                    println!("{} Please enter 1, 2, 3, or 4", "❌".bright_red());
                }
            }
        }
    }

    pub fn get_date_input(&self, prompt: &str) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        println!("\n{}", "Date format: YYYY-MM-DD (e.g., 2025-06-15)".bright_blue().dimmed());
        
        loop {
            let input = self.get_string_input(prompt)?;
            
            // Try to parse the date
            match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
                Ok(date) => {
                    // Convert to UTC datetime at start of day
                    let datetime = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap());
                    return Ok(datetime);
                }
                Err(_) => {
                    println!("{} Invalid date format. Please use YYYY-MM-DD", "❌".bright_red());
                }
            }
        }
    }

    pub fn get_email_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.get_string_input_with_validation(
            prompt,
            |email| email.contains('@') && email.contains('.') && email.len() > 5,
            "Please enter a valid email address (e.g., user@example.com)"
        )
    }

    pub fn get_phone_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.get_string_input_with_validation(
            prompt,
            |phone| phone.chars().filter(|c| c.is_ascii_digit()).count() >= 10,
            "Please enter a valid phone number (at least 10 digits)"
        )
    }

    pub fn get_name_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.get_string_input_with_validation(
            prompt,
            |name| !name.trim().is_empty() && name.trim().len() >= 2,
            "Name must be at least 2 characters long"
        )
    }

    pub fn get_passenger_info_input(&self) -> Result<Passenger, Box<dyn std::error::Error>> {
        println!("\n{}", "═══ Passenger Information ═══".bright_cyan().bold());
        
        let first_name = self.get_name_input("First Name:")?;
        let last_name = self.get_name_input("Last Name:")?;
        let email = self.get_email_input("Email Address:")?;
        let phone = self.get_phone_input("Phone Number:")?;
        
        println!("\n{}", "Date of Birth (YYYY-MM-DD):".bright_cyan());
        let date_of_birth = self.get_string_input_with_validation(
            "Date of Birth:",
            |date| NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok(),
            "Please enter date in YYYY-MM-DD format"
        )?;
        
        let passenger_type = self.get_passenger_type_input()?;
        
        let mut passenger = Passenger::new(
            first_name,
            last_name,
            email,
            phone,
            date_of_birth,
            passenger_type,
        );

        // Optional passport number for international flights
        if self.get_yes_no_input("\nDo you have a passport number to add?")? {
            let passport = self.get_string_input_with_validation(
                "Passport Number:",
                |passport| !passport.trim().is_empty() && passport.trim().len() >= 6,
                "Passport number must be at least 6 characters"
            )?;
            passenger.set_passport(passport);
        }

        // Optional special requirements
        if self.get_yes_no_input("\nDo you have any special requirements?")? {
            println!("\n{}", "Common Special Requirements:".bright_cyan());
            println!("  - Wheelchair assistance");
            println!("  - Vegetarian meal");
            println!("  - Kosher meal");
            println!("  - Extra legroom");
            println!("  - Pet travel");
            println!("  - Medical equipment");
            println!();
            
            loop {
                let requirement = self.get_string_input("Special requirement (or 'done' to finish):")?;
                if requirement.to_lowercase() == "done" {
                    break;
                }
                if !requirement.trim().is_empty() {
                    passenger.add_special_requirement(requirement);
                    println!("{} Added: {}", "✅".bright_green(), requirement.bright_white());
                }
            }
        }

        Ok(passenger)
    }

    pub fn get_menu_choice(&self, prompt: &str, min: u32, max: u32) -> Result<u32, Box<dyn std::error::Error>> {
        self.get_number_input_with_range(prompt, min, max)
    }

    pub fn get_flight_search_criteria(&self, airports: &[Airport]) -> Result<(Option<String>, Option<String>, Option<DateTime<Utc>>), Box<dyn std::error::Error>> {
        println!("\n{}", "═══ Flight Search ═══".bright_cyan().bold());
        
        let origin = if self.get_yes_no_input("Do you want to search by origin airport?")? {
            Some(self.get_airport_code_input("Origin Airport Code:", airports)?)
        } else {
            None
        };

        let destination = if self.get_yes_no_input("Do you want to search by destination airport?")? {
            Some(self.get_airport_code_input("Destination Airport Code:", airports)?)
        } else {
            None
        };

        let date = if self.get_yes_no_input("Do you want to search by specific date?")? {
            Some(self.get_date_input("Travel Date:")?)
        } else {
            None
        };

        Ok((origin, destination, date))
    }

    pub fn get_admin_credentials(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        println!("\n{}", "═══ Admin Authentication ═══".bright_cyan().bold());
        println!("{}", "Demo Credentials:".bright_blue().dimmed());
        println!("{}", "  admin / admin123 (Super Admin)".bright_blue().dimmed());
        println!("{}", "  flight_mgr / flight123 (Flight Manager)".bright_blue().dimmed());
        println!("{}", "  aircraft_mgr / aircraft123 (Aircraft Manager)".bright_blue().dimmed());
        println!();
        
        let username = self.get_string_input("Username:")?;
        let password = self.get_password_input("Password:")?;
        
        Ok((username, password))
    }

    pub fn get_password_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // In a real application, you'd use a library like `rpassword` to hide password input
        // For demo purposes, we'll just use regular input
        print!("{} ", prompt.bright_yellow());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn get_flight_number_input(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.get_string_input_with_validation(
            "Flight Number (e.g., RIA101):",
            |flight_num| !flight_num.trim().is_empty() && flight_num.trim().len() >= 3,
            "Flight number must be at least 3 characters"
        )
    }

    pub fn get_ticket_number_input(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.get_string_input_with_validation(
            "Ticket Number (e.g., RIA123456):",
            |ticket| !ticket.trim().is_empty() && ticket.trim().len() >= 6,
            "Ticket number must be at least 6 characters"
        )
    }

    pub fn get_delay_minutes_input(&self) -> Result<i32, Box<dyn std::error::Error>> {
        println!("\n{}", "Enter delay in minutes (0 = on time, negative = early):".bright_cyan());
        self.get_number_input_with_range("Delay minutes:", -60, 480) // -1 hour to +8 hours
    }

    pub fn get_pricing_multiplier_input(&self) -> Result<f64, Box<dyn std::error::Error>> {
        println!("\n{}", "Pricing multiplier (1.0 = normal, 1.5 = 50% increase, 0.8 = 20% discount):".bright_cyan());
        self.get_number_input_with_range("Multiplier:", 0.1, 5.0)
    }

    pub fn display_search_options(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "Search Options:".bright_cyan().bold());
        println!("  {} - Search all flights", "1".bright_green());
        println!("  {} - Search by origin", "2".bright_green());
        println!("  {} - Search by destination", "3".bright_green());
        println!("  {} - Search by route (origin + destination)", "4".bright_green());
        println!("  {} - Search by date", "5".bright_green());
        println!("  {} - Custom search (multiple criteria)", "6".bright_green());
        println!("  {} - Back to main menu", "0".bright_red());
        Ok(())
    }

    pub fn display_admin_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "═══ Admin Panel ═══".bright_cyan().bold());
        println!("  {} - View System Metrics", "1".bright_green());
        println!("  {} - Set Flight Delay", "2".bright_yellow());
        println!("  {} - Set Dynamic Pricing", "3".bright_yellow());
        println!("  {} - View Admin Log", "4".bright_blue());
        println!("  {} - Aircraft Management", "5".bright_blue());
        println!("  {} - Create Backup", "6".bright_magenta());
        println!("  {} - Logout", "0".bright_red());
        Ok(())
    }

    pub fn confirm_action(&self, action: &str) -> Result<bool, Box<dyn std::error::Error>> {
        self.get_yes_no_input(&format!("Are you sure you want to {}?", action))
    }

    pub fn display_loading_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        print!("{} {}...", "🔄".bright_blue(), message.bright_blue());
        io::stdout().flush()?;
        Ok(())
    }

    pub fn clear_loading_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        print!("\r{}\r", " ".repeat(50)); // Clear the line
        io::stdout().flush()?;
        Ok(())
    }
}