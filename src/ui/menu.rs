use crate::data::manager::DataManager;
use crate::ui::{display::DisplayManager, input::InputManager};
use crate::modules::{
    flight::{Flight, SeatClass},
    booking::{Passenger, PassengerType},
};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
};
use colored::*;
use std::io::{self, Write};
use std::error::Error;
use uuid::Uuid;

pub struct MainMenu {
    data_manager: DataManager,
    display: DisplayManager,
    input: InputManager,
}

impl MainMenu {
    pub fn new(data_manager: DataManager) -> Self {
        Self {
            data_manager,
            display: DisplayManager::new(),
            input: InputManager::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            // Update real-time simulation
            self.data_manager.update_simulation().await?;
            
            self.display_main_menu()?;
            
            let choice = self.input.get_menu_choice("Enter your choice (1-7):", 1, 7)?;
            
            match choice {
                1 => self.search_flights().await?,
                2 => self.book_flight().await?,
                3 => self.manage_bookings().await?,
                4 => self.flight_info().await?,
                5 => self.aircraft_data().await?,
                6 => self.admin_panel().await?,
                7 => {
                    self.display.display_info_message("Saving data and exiting...")?;
                    self.data_manager.save_all_data().await?;
                    println!("\n{}", "Thank you for using Rust International Airport! Safe travels! ✈️".bright_green().bold());
                    break;
                }
                _ => {
                    self.display.display_error_message("Invalid option! Please try again.")?;
                    self.display.pause_for_user()?;
                }
            }
        }
        Ok(())
    }

    fn display_main_menu(&self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;

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
        
        // Show current system status
        let metrics = self.data_manager.get_system_metrics();
        println!("\n{} {}", "📊 System Status:".bright_blue().bold(), metrics.get_summary().bright_white());
        println!();

        Ok(())
    }

    // 1. Search Flights
    async fn search_flights(&mut self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;
        self.display.display_header("Flight Search")?;

        self.input.display_search_options()?;
        let search_type = self.input.get_menu_choice("Select search type:", 0, 6)?;

        if search_type == 0 {
            return Ok(());
        }

        let airports = self.data_manager.get_all_airports();
        let flights = match search_type {
            1 => {
                // Show all available flights
                self.data_manager.get_available_flights()
            }
            2 => {
                // Search by origin
                let origin = self.input.get_airport_code_input("Origin Airport:", airports)?;
                self.data_manager.search_flights(Some(&origin), None, None)
            }
            3 => {
                // Search by destination
                let destination = self.input.get_airport_code_input("Destination Airport:", airports)?;
                self.data_manager.search_flights(None, Some(&destination), None)
            }
            4 => {
                // Search by route
                let origin = self.input.get_airport_code_input("Origin Airport:", airports)?;
                let destination = self.input.get_airport_code_input("Destination Airport:", airports)?;
                self.data_manager.search_flights(Some(&origin), Some(&destination), None)
            }
            5 => {
                // Search by date
                let date = self.input.get_date_input("Travel Date:")?;
                self.data_manager.search_flights(None, None, Some(date))
            }
            6 => {
                // Custom search
                let (origin, destination, date) = self.input.get_flight_search_criteria(airports)?;
                self.data_manager.search_flights(
                    origin.as_deref(),
                    destination.as_deref(),
                    date
                )
            }
            _ => return Ok(()),
        };

        self.display.clear_screen()?;
        self.display.display_header("Search Results")?;
        self.display.display_flights_table(&flights)?;

        if !flights.is_empty() {
            if self.input.get_yes_no_input("Would you like to view details for a specific flight?")? {
                let flight_number = self.input.get_flight_number_input()?;
                if let Some(flight) = self.data_manager.get_flight_by_number(&flight_number) {
                    let aircraft = self.data_manager.get_aircraft_for_flight(flight.id);
                    self.display.clear_screen()?;
                    self.display.display_flight_details(flight, aircraft)?;
                } else {
                    self.display.display_error_message("Flight not found!")?;
                }
            }
        }

        self.display.pause_for_user()?;
        Ok(())
    }

    // 2. Book a Flight
    async fn book_flight(&mut self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;
        self.display.display_header("Flight Booking")?;

        // Show available flights
        let available_flights = self.data_manager.get_available_flights();
        if available_flights.is_empty() {
            self.display.display_warning_message("No flights available for booking at this time.")?;
            self.display.pause_for_user()?;
            return Ok(());
        }

        self.display.display_flights_table(&available_flights)?;

        // Get flight selection
        let flight_number = self.input.get_flight_number_input()?;
        let flight = match self.data_manager.get_flight_by_number(&flight_number) {
            Some(f) => f,
            None => {
                self.display.display_error_message("Flight not found!")?;
                self.display.pause_for_user()?;
                return Ok(());
            }
        };

        if !flight.is_available_for_booking() {
            self.display.display_error_message("This flight is not available for booking.")?;
            self.display.pause_for_user()?;
            return Ok(());
        }

        // Show flight details
        let aircraft = self.data_manager.get_aircraft_for_flight(flight.id);
        self.display.display_flight_details(flight, aircraft)?;

        // Get seat class
        let seat_class = self.input.get_seat_class_input()?;

        // Check seat availability
        if flight.get_available_seats(&seat_class) == 0 {
            self.display.display_error_message("No seats available in the selected class.")?;
            self.display.pause_for_user()?;
            return Ok(());
        }

        // Get passenger information
        let passenger = self.input.get_passenger_info_input()?;

        // Show booking summary
        self.display.clear_screen()?;
        self.display.display_header("Booking Summary")?;
        
        let price = flight.get_price(&seat_class);
        println!("{}", "═══ Booking Details ═══".bright_cyan().bold());
        println!("Flight: {} ({})", flight.flight_number.bright_white().bold(), flight.airline.bright_white());
        println!("Route: {} → {}", flight.origin.bright_green(), flight.destination.bright_green());
        println!("Date: {}", flight.departure_time.format("%Y-%m-%d").to_string().bright_white());
        println!("Time: {} → {}", 
            flight.departure_time.format("%H:%M").to_string().bright_blue(),
            flight.arrival_time.format("%H:%M").to_string().bright_blue());
        println!("Passenger: {}", passenger.full_name().bright_white().bold());
        println!("Class: {:?}", seat_class);
        println!("Price: ${:.2}", price.to_string().bright_green().bold());
        println!();

        // Confirm booking
        if self.input.confirm_action("complete this booking")? {
            match self.data_manager.create_booking(flight.id, passenger, seat_class) {
                Ok(booking_id) => {
                    if let Some(booking) = self.data_manager.get_booking_by_id(booking_id) {
                        self.display.display_success_message("Booking completed successfully!")?;
                        println!("\n{}", "═══ Your Ticket ═══".bright_green().bold());
                        println!("Ticket Number: {}", booking.ticket_number.bright_white().bold());
                        println!("Please save this ticket number for your records.");
                        
                        // Auto-assign seat
                        println!("\n{}", "ℹ️ Seat assignment will be completed at check-in.".bright_blue());
                    }
                }
                Err(e) => {
                    self.display.display_error_message(&format!("Booking failed: {}", e))?;
                }
            }
        } else {
            self.display.display_info_message("Booking cancelled.")?;
        }

        self.display.pause_for_user()?;
        Ok(())
    }

    // 3. Manage Bookings
    async fn manage_bookings(&mut self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;
        self.display.display_header("Booking Management")?;

        println!("{}", "Booking Management Options:".bright_cyan().bold());
        println!("  {} - View booking details", "1".bright_green());
        println!("  {} - Cancel booking", "2".bright_red());
        println!("  {} - View all bookings", "3".bright_blue());
        println!("  {} - Back to main menu", "0".bright_yellow());
        println!();

        let choice = self.input.get_menu_choice("Select option:", 0, 3)?;

        match choice {
            0 => return Ok(()),
            1 => {
                // View booking details
                let ticket_number = self.input.get_ticket_number_input()?;
                if let Some(booking) = self.data_manager.get_booking_by_ticket(&ticket_number) {
                    if let Some(flight) = self.data_manager.get_flight_by_id(booking.flight_id) {
                        self.display.clear_screen()?;
                        self.display.display_booking_details(booking)?;
                        self.display.display_flight_details(flight, 
                            self.data_manager.get_aircraft_for_flight(flight.id))?;
                    }
                } else {
                    self.display.display_error_message("Booking not found!")?;
                }
            }
            2 => {
                // Cancel booking
                let ticket_number = self.input.get_ticket_number_input()?;
                if let Some(booking) = self.data_manager.get_booking_by_ticket(&ticket_number) {
                    self.display.display_booking_details(booking)?;
                    
                    if booking.can_be_modified() {
                        if self.input.confirm_action("cancel this booking")? {
                            match self.data_manager.cancel_booking(&ticket_number) {
                                Ok(()) => {
                                    self.display.display_success_message("Booking cancelled successfully!")?;
                                }
                                Err(e) => {
                                    self.display.display_error_message(&format!("Cancellation failed: {}", e))?;
                                }
                            }
                        }
                    } else {
                        self.display.display_warning_message("This booking cannot be cancelled.")?;
                    }
                } else {
                    self.display.display_error_message("Booking not found!")?;
                }
            }
            3 => {
                // View all bookings
                let all_bookings: Vec<&_> = self.data_manager.database.bookings.iter().collect();
                self.display.clear_screen()?;
                self.display.display_header("All Bookings")?;
                self.display.display_bookings_table(&all_bookings)?;
            }
            _ => {}
        }

        self.display.pause_for_user()?;
        Ok(())
    }

    // 4. Flight Info
    async fn flight_info(&mut self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;
        self.display.display_header("Flight Information")?;

        println!("{}", "Flight Information Options:".bright_cyan().bold());
        println!("  {} - View specific flight details", "1".bright_green());
        println!("  {} - View all flights", "2".bright_blue());
        println!("  {} - View departures from airport", "3".bright_yellow());
        println!("  {} - View arrivals to airport", "4".bright_yellow());
        println!("  {} - Back to main menu", "0".bright_red());
        println!();

        let choice = self.input.get_menu_choice("Select option:", 0, 4)?;

        match choice {
            0 => return Ok(()),
            1 => {
                // Specific flight details
                let flight_number = self.input.get_flight_number_input()?;
                if let Some(flight) = self.data_manager.get_flight_by_number(&flight_number) {
                    let aircraft = self.data_manager.get_aircraft_for_flight(flight.id);
                    self.display.clear_screen()?;
                    self.display.display_flight_details(flight, aircraft)?;
                } else {
                    self.display.display_error_message("Flight not found!")?;
                }
            }
            2 => {
                // All flights
                let all_flights: Vec<&_> = self.data_manager.database.flights.iter().collect();
                self.display.clear_screen()?;
                self.display.display_header("All Flights")?;
                self.display.display_flights_table(&all_flights)?;
            }
            3 => {
                // Departures from airport
                let airport_code = self.input.get_airport_code_input("Airport Code:", self.data_manager.get_all_airports())?;
                let departures = self.data_manager.get_departures_from_airport(&airport_code);
                self.display.clear_screen()?;
                self.display.display_header(&format!("Departures from {}", airport_code))?;
                self.display.display_flights_table(&departures)?;
            }
            4 => {
                // Arrivals to airport
                let airport_code = self.input.get_airport_code_input("Airport Code:", self.data_manager.get_all_airports())?;
                let arrivals = self.data_manager.get_arrivals_to_airport(&airport_code);
                self.display.clear_screen()?;
                self.display.display_header(&format!("Arrivals to {}", airport_code))?;
                self.display.display_flights_table(&arrivals)?;
            }
            _ => {}
        }

        self.display.pause_for_user()?;
        Ok(())
    }

    // 5. Aircraft Data
    async fn aircraft_data(&mut self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;
        self.display.display_header("Aircraft Information")?;

        println!("{}", "Aircraft Information Options:".bright_cyan().bold());
        println!("  {} - View all aircraft", "1".bright_green());
        println!("  {} - View specific aircraft details", "2".bright_blue());
        println!("  {} - View available aircraft", "3".bright_yellow());
        println!("  {} - Back to main menu", "0".bright_red());
        println!();

        let choice = self.input.get_menu_choice("Select option:", 0, 3)?;

        match choice {
            0 => return Ok(()),
            1 => {
                // All aircraft
                let all_aircraft: Vec<&_> = self.data_manager.database.aircraft.iter().collect();
                self.display.clear_screen()?;
                self.display.display_header("Aircraft Registry")?;
                self.display.display_aircraft_table(&all_aircraft)?;
            }
            2 => {
                // Specific aircraft details
                let registration = self.input.get_string_input("Aircraft Registration (e.g., N123RIA):")?;
                if let Some(aircraft) = self.data_manager.database.aircraft.iter().find(|a| a.registration == registration) {
                    self.display.clear_screen()?;
                    self.display.display_aircraft_details(aircraft)?;
                } else {
                    self.display.display_error_message("Aircraft not found!")?;
                }
            }
            3 => {
                // Available aircraft
                let available_aircraft = self.data_manager.get_available_aircraft();
                self.display.clear_screen()?;
                self.display.display_header("Available Aircraft")?;
                self.display.display_aircraft_table(&available_aircraft)?;
            }
            _ => {}
        }

        self.display.pause_for_user()?;
        Ok(())
    }

    // 6. Admin Panel
    async fn admin_panel(&mut self) -> Result<(), Box<dyn Error>> {
        self.display.clear_screen()?;
        self.display.display_header("Admin Panel")?;

        // Check if already authenticated
        if !self.data_manager.is_admin_authenticated() {
            let (username, password) = self.input.get_admin_credentials()?;
            match self.data_manager.authenticate_admin(&username, &password) {
                Ok(admin) => {
                    self.display.display_success_message(&format!("Welcome, {}!", admin.full_name))?;
                    self.display.pause_for_user()?;
                }
                Err(e) => {
                    self.display.display_error_message(&format!("Authentication failed: {}", e))?;
                    self.display.pause_for_user()?;
                    return Ok(());
                }
            }
        }

        loop {
            self.display.clear_screen()?;
            self.display.display_header(&format!("Admin Panel - {}", self.data_manager.admin_panel.current_admin_name()))?;
            
            self.input.display_admin_menu()?;
            let choice = self.input.get_menu_choice("Select option:", 0, 6)?;

            match choice {
                0 => {
                    self.data_manager.logout_admin();
                    self.display.display_info_message("Logged out successfully.")?;
                    break;
                }
                1 => {
                    // View system metrics
                    self.display.clear_screen()?;
                    self.display.display_header("System Metrics")?;
                    let metrics = self.data_manager.get_system_metrics();
                    self.display.display_system_metrics(metrics)?;
                    
                    let (total_flights, on_time, delayed, cancelled) = self.data_manager.get_flight_statistics();
                    let (total_bookings, confirmed, cancelled_bookings) = self.data_manager.get_booking_statistics();
                    
                    println!("\n{}", "📈 Additional Statistics:".bright_cyan().bold());
                    println!("Flight Performance: {}/{} on time ({:.1}%)", 
                        on_time, total_flights, (on_time as f64 / total_flights as f64) * 100.0);
                    println!("Booking Success Rate: {}/{} confirmed ({:.1}%)", 
                        confirmed, total_bookings, (confirmed as f64 / total_bookings as f64) * 100.0);
                }
                2 => {
                    // Set flight delay
                    let flight_number = self.input.get_flight_number_input()?;
                    let delay_minutes = self.input.get_delay_minutes_input()?;
                    
                    match self.data_manager.set_flight_delay(&flight_number, delay_minutes) {
                        Ok(()) => {
                            self.display.display_success_message(&format!("Flight {} delay updated to {} minutes", flight_number, delay_minutes))?;
                        }
                        Err(e) => {
                            self.display.display_error_message(&format!("Failed to set delay: {}", e))?;
                        }
                    }
                }
                3 => {
                    // Set dynamic pricing
                    let flight_number = self.input.get_flight_number_input()?;
                    let multiplier = self.input.get_pricing_multiplier_input()?;
                    
                    match self.data_manager.set_dynamic_pricing(&flight_number, multiplier) {
                        Ok(()) => {
                            self.display.display_success_message(&format!("Flight {} pricing multiplier set to {:.2}", flight_number, multiplier))?;
                        }
                        Err(e) => {
                            self.display.display_error_message(&format!("Failed to set pricing: {}", e))?;
                        }
                    }
                }
                4 => {
                    // View admin log
                    self.display.clear_screen()?;
                    self.display.display_header("Admin Action Log")?;
                    let recent_actions = self.data_manager.admin_panel.get_recent_actions(20);
                    self.display.display_admin_log(&recent_actions, 20)?;
                }
                5 => {
                    // Aircraft management placeholder
                    self.display.display_info_message("Aircraft management features coming soon!")?;
                }
                6 => {
                    // Create backup
                    self.input.display_loading_message("Creating backup")?;
                    match self.data_manager.create_backup().await {
                        Ok(backup_path) => {
                            self.input.clear_loading_message()?;
                            self.display.display_success_message(&format!("Backup created: {}", backup_path))?;
                        }
                        Err(e) => {
                            self.input.clear_loading_message()?;
                            self.display.display_error_message(&format!("Backup failed: {}", e))?;
                        }
                    }
                }
                _ => {
                    self.display.display_error_message("Invalid option!")?;
                }
            }
            
            if choice != 0 {
                self.display.pause_for_user()?;
            }
        }

        Ok(())
    }
}