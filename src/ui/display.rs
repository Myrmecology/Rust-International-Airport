use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use colored::*;
use std::io::{self, Write};
use chrono::{DateTime, Utc};
use crate::modules::{
    flight::{Flight, SeatClass},
    aircraft::Aircraft,
    booking::Booking,
    airport::Airport,
    admin::{SystemMetrics, AdminAction},
};

pub struct DisplayManager;

impl DisplayManager {
    pub fn new() -> Self {
        Self
    }

    pub fn clear_screen(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        Ok(())
    }

    pub fn display_header(&self, title: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        let border_length = title.len() + 4;
        let border = "═".repeat(border_length);
        
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("╔{}╗\n", border)),
            Print(format!("║ {} ║\n", title)),
            Print(format!("╚{}╝\n", border)),
            ResetColor,
            Print("\n")
        )?;
        Ok(())
    }

    pub fn display_section_header(&self, title: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", format!("▓▓▓ {} ▓▓▓", title).bright_cyan().bold());
        println!("{}", "─".repeat(50).bright_blue());
        Ok(())
    }

    pub fn display_flights_table(&self, flights: &[&Flight]) -> Result<(), Box<dyn std::error::Error>> {
        if flights.is_empty() {
            println!("{}", "No flights found.".bright_yellow());
            return Ok(());
        }

        self.display_section_header("Flight Information")?;
        
        // Table header
        println!(
            "{:<10} {:<4} {:<6} {:<6} {:<8} {:<8} {:<15} {:<6} {:<12}",
            "Flight".bright_white().bold(),
            "Gate".bright_white().bold(),
            "Origin".bright_white().bold(),
            "Dest".bright_white().bold(),
            "Departure".bright_white().bold(),
            "Arrival".bright_white().bold(),
            "Status".bright_white().bold(),
            "Eco".bright_white().bold(),
            "Bus/First".bright_white().bold()
        );
        println!("{}", "─".repeat(95).bright_blue());

        // Table rows
        for flight in flights {
            let gate = flight.gate.as_deref().unwrap_or("--");
            let status = flight.get_status_display();
            let departure_time = flight.departure_time.format("%H:%M");
            let arrival_time = flight.arrival_time.format("%H:%M");
            
            // Color code status
            let status_colored = match flight.status {
                crate::modules::flight::FlightStatus::OnTime => status.bright_green(),
                crate::modules::flight::FlightStatus::Delayed(_) => status.bright_red(),
                crate::modules::flight::FlightStatus::Boarding => status.bright_yellow(),
                crate::modules::flight::FlightStatus::Departed => status.bright_blue(),
                crate::modules::flight::FlightStatus::Arrived => status.bright_magenta(),
                crate::modules::flight::FlightStatus::Cancelled => status.bright_red().bold(),
            };

            println!(
                "{:<10} {:<4} {:<6} {:<6} {:<8} {:<8} {:<15} {:<6} {:<5}/{:<6}",
                flight.flight_number.bright_white(),
                gate.bright_cyan(),
                flight.origin.bright_green(),
                flight.destination.bright_green(),
                departure_time.to_string().bright_blue(),
                arrival_time.to_string().bright_blue(),
                status_colored,
                flight.seat_availability.economy.to_string().bright_white(),
                flight.seat_availability.business.to_string().bright_white(),
                flight.seat_availability.first_class.to_string().bright_white()
            );
        }
        
        println!();
        Ok(())
    }

    pub fn display_flight_details(&self, flight: &Flight, aircraft: Option<&Aircraft>) -> Result<(), Box<dyn std::error::Error>> {
        self.display_section_header(&format!("Flight {} Details", flight.flight_number))?;
        
        println!("{}  {}", "✈️ Flight:".bright_cyan().bold(), flight.flight_number.bright_white().bold());
        println!("{}  {}", "🏢 Airline:".bright_cyan(), flight.airline.bright_white());
        println!("{}  {} → {}", "🛫 Route:".bright_cyan(), 
            flight.origin.bright_green().bold(), 
            flight.destination.bright_green().bold());
        
        println!("{}  {}", "🕐 Departure:".bright_cyan(), 
            flight.departure_time.format("%Y-%m-%d %H:%M UTC").to_string().bright_white());
        println!("{}  {}", "🕑 Arrival:".bright_cyan(), 
            flight.arrival_time.format("%Y-%m-%d %H:%M UTC").to_string().bright_white());
        println!("{}  {}", "⏱️ Duration:".bright_cyan(), 
            format!("{} hours {} minutes", 
                flight.duration().num_hours(), 
                flight.duration().num_minutes() % 60).bright_white());
        
        println!("{}  {}", "📍 Status:".bright_cyan(), flight.get_status_display());
        
        if let Some(gate) = &flight.gate {
            println!("{}  {}", "🚪 Gate:".bright_cyan(), gate.bright_white().bold());
        }

        // Seat availability
        println!("\n{}", "💺 Seat Availability:".bright_cyan().bold());
        println!("   Economy: {} seats (${:.2})", 
            flight.seat_availability.economy.to_string().bright_green(),
            flight.get_price(&SeatClass::Economy));
        println!("   Business: {} seats (${:.2})", 
            flight.seat_availability.business.to_string().bright_yellow(),
            flight.get_price(&SeatClass::Business));
        println!("   First Class: {} seats (${:.2})", 
            flight.seat_availability.first_class.to_string().bright_magenta(),
            flight.get_price(&SeatClass::FirstClass));

        // Aircraft information
        if let Some(aircraft) = aircraft {
            println!("\n{}", "🛩️ Aircraft Information:".bright_cyan().bold());
            println!("   Model: {}", aircraft.model.bright_white());
            println!("   Registration: {}", aircraft.registration.bright_white());
            println!("   Capacity: {} passengers", aircraft.total_capacity.to_string().bright_white());
            println!("   Status: {}", aircraft.get_status_display());
        }

        println!();
        Ok(())
    }

    pub fn display_aircraft_table(&self, aircraft: &[&Aircraft]) -> Result<(), Box<dyn std::error::Error>> {
        if aircraft.is_empty() {
            println!("{}", "No aircraft found.".bright_yellow());
            return Ok(());
        }

        self.display_section_header("Aircraft Registry")?;
        
        // Table header
        println!(
            "{:<12} {:<20} {:<6} {:<8} {:<12} {:<15}",
            "Registration".bright_white().bold(),
            "Model".bright_white().bold(),
            "Year".bright_white().bold(),
            "Capacity".bright_white().bold(),
            "Status".bright_white().bold(),
            "Flight Hours".bright_white().bold()
        );
        println!("{}", "─".repeat(85).bright_blue());

        // Table rows
        for craft in aircraft {
            let status_colored = match craft.status {
                crate::modules::aircraft::AircraftStatus::Active => craft.get_status_display().bright_green(),
                crate::modules::aircraft::AircraftStatus::Maintenance => craft.get_status_display().bright_red(),
                crate::modules::aircraft::AircraftStatus::InFlight => craft.get_status_display().bright_blue(),
                crate::modules::aircraft::AircraftStatus::Retired => craft.get_status_display().bright_red().dimmed(),
            };

            println!(
                "{:<12} {:<20} {:<6} {:<8} {:<12} {:<15}",
                craft.registration.bright_white(),
                craft.model.bright_cyan(),
                craft.year_manufactured.to_string().bright_white(),
                craft.total_capacity.to_string().bright_white(),
                status_colored,
                format!("{:.1}h", craft.flight_hours).bright_white()
            );
        }
        
        println!();
        Ok(())
    }

    pub fn display_aircraft_details(&self, aircraft: &Aircraft) -> Result<(), Box<dyn std::error::Error>> {
        self.display_section_header(&format!("Aircraft {} Details", aircraft.registration))?;
        
        println!("{}  {}", "✈️ Registration:".bright_cyan().bold(), aircraft.registration.bright_white().bold());
        println!("{}  {}", "🏭 Manufacturer:".bright_cyan(), aircraft.manufacturer.bright_white());
        println!("{}  {}", "🛩️ Model:".bright_cyan(), aircraft.model.bright_white());
        println!("{}  {} ({} years old)", "📅 Year:".bright_cyan(), 
            aircraft.year_manufactured.to_string().bright_white(),
            aircraft.get_age().to_string().bright_yellow());
        println!("{}  {}", "📊 Status:".bright_cyan(), aircraft.get_status_display());
        
        // Capacity breakdown
        println!("\n{}", "💺 Seating Configuration:".bright_cyan().bold());
        println!("   Economy: {} seats ({} rows × {} seats)", 
            aircraft.get_seats_by_class(&SeatClass::Economy).to_string().bright_green(),
            aircraft.seat_configuration.economy_rows,
            aircraft.seat_configuration.economy_seats_per_row);
        println!("   Business: {} seats ({} rows × {} seats)", 
            aircraft.get_seats_by_class(&SeatClass::Business).to_string().bright_yellow(),
            aircraft.seat_configuration.business_rows,
            aircraft.seat_configuration.business_seats_per_row);
        println!("   First Class: {} seats ({} rows × {} seats)", 
            aircraft.get_seats_by_class(&SeatClass::FirstClass).to_string().bright_magenta(),
            aircraft.seat_configuration.first_class_rows,
            aircraft.seat_configuration.first_class_seats_per_row);
        println!("   Total Capacity: {} passengers", aircraft.total_capacity.to_string().bright_white().bold());

        // Performance specs
        println!("\n{}", "⚡ Performance Specifications:".bright_cyan().bold());
        println!("   Max Speed: {} km/h", aircraft.performance.max_speed_kmh.to_string().bright_white());
        println!("   Cruise Speed: {} km/h", aircraft.performance.cruise_speed_kmh.to_string().bright_white());
        println!("   Max Altitude: {} meters", aircraft.performance.max_altitude_m.to_string().bright_white());
        println!("   Range: {} km", aircraft.performance.range_km.to_string().bright_white());
        println!("   Fuel Efficiency: {:.1} L/100km", aircraft.performance.fuel_efficiency_l_per_100km.to_string().bright_white());

        // Operational data
        println!("\n{}", "🔧 Operational Data:".bright_cyan().bold());
        println!("   Flight Hours: {:.1} hours", aircraft.flight_hours.to_string().bright_white());
        println!("   Maintenance Hours: {:.1} hours", aircraft.maintenance_hours.to_string().bright_white());
        println!("   Baggage Capacity: {} kg", aircraft.baggage_capacity_kg.to_string().bright_white());
        println!("   Max Cargo Weight: {} kg", aircraft.max_cargo_weight_kg.to_string().bright_white());

        println!();
        Ok(())
    }

    pub fn display_bookings_table(&self, bookings: &[&Booking]) -> Result<(), Box<dyn std::error::Error>> {
        if bookings.is_empty() {
            println!("{}", "No bookings found.".bright_yellow());
            return Ok(());
        }

        self.display_section_header("Booking Information")?;
        
        // Table header
        println!(
            "{:<12} {:<25} {:<8} {:<10} {:<15} {:<10}",
            "Ticket #".bright_white().bold(),
            "Passenger".bright_white().bold(),
            "Class".bright_white().bold(),
            "Seat".bright_white().bold(),
            "Status".bright_white().bold(),
            "Amount".bright_white().bold()
        );
        println!("{}", "─".repeat(85).bright_blue());

        // Table rows
        for booking in bookings {
            let seat_info = match &booking.seat_assignment {
                Some(seat) => seat.seat_number.clone(),
                None => "Not assigned".to_string(),
            };

            let status_colored = match booking.status {
                crate::modules::booking::BookingStatus::Confirmed => booking.get_status_display().bright_green(),
                crate::modules::booking::BookingStatus::CheckedIn => booking.get_status_display().bright_blue(),
                crate::modules::booking::BookingStatus::Boarded => booking.get_status_display().bright_cyan(),
                crate::modules::booking::BookingStatus::Completed => booking.get_status_display().bright_magenta(),
                crate::modules::booking::BookingStatus::Cancelled => booking.get_status_display().bright_red(),
                crate::modules::booking::BookingStatus::NoShow => booking.get_status_display().bright_red().bold(),
            };

            println!(
                "{:<12} {:<25} {:<8} {:<10} {:<15} ${:<9.2}",
                booking.ticket_number.bright_white(),
                booking.passenger.full_name().bright_cyan(),
                format!("{:?}", booking.seat_class).bright_yellow(),
                seat_info.bright_white(),
                status_colored,
                booking.payment.total_amount
            );
        }
        
        println!();
        Ok(())
    }

    pub fn display_booking_details(&self, booking: &Booking) -> Result<(), Box<dyn std::error::Error>> {
        self.display_section_header(&format!("Booking {} Details", booking.ticket_number))?;
        
        println!("{}  {}", "🎫 Ticket Number:".bright_cyan().bold(), booking.ticket_number.bright_white().bold());
        println!("{}  {}", "📅 Booking Date:".bright_cyan(), 
            booking.booking_date.format("%Y-%m-%d %H:%M UTC").to_string().bright_white());
        println!("{}  {}", "📊 Status:".bright_cyan(), booking.get_status_display());

        // Passenger information
        println!("\n{}", "👤 Passenger Information:".bright_cyan().bold());
        println!("   Name: {}", booking.passenger.full_name().bright_white());
        println!("   Email: {}", booking.passenger.email.bright_white());
        println!("   Phone: {}", booking.passenger.phone.bright_white());
        println!("   Type: {:?}", booking.passenger.passenger_type);

        if let Some(passport) = &booking.passenger.passport_number {
            println!("   Passport: {}", passport.bright_white());
        }

        // Seat information
        println!("\n{}", "💺 Seat Information:".bright_cyan().bold());
        println!("   Class: {:?}", booking.seat_class);
        
        if let Some(seat) = &booking.seat_assignment {
            println!("   Seat Number: {}", seat.seat_number.bright_white().bold());
            println!("   Seat Type: {}", seat.get_seat_type().bright_white());
        } else {
            println!("   Seat: Not assigned yet");
        }

        // Payment information
        println!("\n{}", "💳 Payment Information:".bright_cyan().bold());
        println!("   Total Amount: ${:.2}", booking.payment.total_amount.to_string().bright_green().bold());
        println!("   Currency: {}", booking.payment.currency.bright_white());
        println!("   Payment Method: {}", booking.payment.payment_method.bright_white());
        println!("   Transaction ID: {}", booking.payment.transaction_id.bright_white());
        println!("   Payment Date: {}", 
            booking.payment.payment_date.format("%Y-%m-%d %H:%M UTC").to_string().bright_white());

        // Baggage and services
        println!("\n{}", "🧳 Additional Information:".bright_cyan().bold());
        println!("   Baggage Count: {} pieces", booking.baggage_count.to_string().bright_white());
        
        if !booking.special_services.is_empty() {
            println!("   Special Services: {}", booking.special_services.join(", ").bright_white());
        }

        if !booking.passenger.special_requirements.is_empty() {
            println!("   Special Requirements: {}", booking.passenger.special_requirements.join(", ").bright_yellow());
        }

        // Check-in and boarding times
        if let Some(checkin_time) = booking.check_in_time {
            println!("\n{}", "⏰ Timeline:".bright_cyan().bold());
            println!("   Check-in: {}", checkin_time.format("%Y-%m-%d %H:%M UTC").to_string().bright_white());
        }

        if let Some(boarding_time) = booking.boarding_time {
            println!("   Boarding: {}", boarding_time.format("%Y-%m-%d %H:%M UTC").to_string().bright_white());
        }

        println!();
        Ok(())
    }

    pub fn display_airports_table(&self, airports: &[&Airport]) -> Result<(), Box<dyn std::error::Error>> {
        if airports.is_empty() {
            println!("{}", "No airports found.".bright_yellow());
            return Ok(());
        }

        self.display_section_header("Airport Directory")?;
        
        // Table header
        println!(
            "{:<6} {:<35} {:<15} {:<15} {:<12}",
            "Code".bright_white().bold(),
            "Name".bright_white().bold(),
            "City".bright_white().bold(),
            "Country".bright_white().bold(),
            "Type".bright_white().bold()
        );
        println!("{}", "─".repeat(85).bright_blue());

        // Table rows
        for airport in airports {
            println!(
                "{:<6} {:<35} {:<15} {:<15} {:<12}",
                airport.code.bright_green().bold(),
                airport.name.bright_white(),
                airport.city.bright_cyan(),
                airport.country.bright_yellow(),
                airport.get_size_display()
            );
        }
        
        println!();
        Ok(())
    }

    pub fn display_system_metrics(&self, metrics: &SystemMetrics) -> Result<(), Box<dyn std::error::Error>> {
        self.display_section_header("System Status Dashboard")?;
        
        println!("{}", "📊 Flight Operations:".bright_cyan().bold());
        println!("   Total Flights: {}", metrics.total_flights.to_string().bright_white().bold());
        println!("   Active Flights: {}", metrics.active_flights.to_string().bright_green());
        println!("   Delayed Flights: {}", metrics.delayed_flights.to_string().bright_red());
        println!("   Cancelled Flights: {}", metrics.cancelled_flights.to_string().bright_red().bold());
        
        println!("\n{}", "✈️ Aircraft Status:".bright_cyan().bold());
        println!("   Total Aircraft: {}", metrics.total_aircraft.to_string().bright_white().bold());
        println!("   Active Aircraft: {}", metrics.active_aircraft.to_string().bright_green());
        println!("   In Maintenance: {}", metrics.aircraft_in_maintenance.to_string().bright_yellow());
        
        println!("\n{}", "🎫 Booking Statistics:".bright_cyan().bold());
        println!("   Total Bookings: {}", metrics.total_bookings.to_string().bright_white().bold());
        
        println!("\n{}", "💰 Revenue:".bright_cyan().bold());
        println!("   Today: ${:.2}", metrics.revenue_today.to_string().bright_green().bold());
        println!("   This Month: ${:.2}", metrics.revenue_month.to_string().bright_green().bold());
        
        if metrics.average_load_factor > 0.0 {
            println!("\n{}", "📈 Performance:".bright_cyan().bold());
            println!("   Average Load Factor: {:.1}%", (metrics.average_load_factor * 100.0).to_string().bright_white());
        }
        
        println!("\n{}", "🕐 Last Updated:".bright_cyan().bold());
        println!("   {}", metrics.last_updated.format("%Y-%m-%d %H:%M:%S UTC").to_string().bright_white());
        
        println!();
        Ok(())
    }

    pub fn display_admin_log(&self, actions: &[&AdminAction], limit: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.display_section_header(&format!("Recent Admin Actions (Last {})", limit))?;
        
        if actions.is_empty() {
            println!("{}", "No admin actions recorded.".bright_yellow());
            return Ok(());
        }

        for action in actions.iter().take(limit) {
            println!("{}", action.format_for_log().bright_white());
        }
        
        println!();
        Ok(())
    }

    pub fn display_success_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{} {}", "✅".bright_green(), message.bright_green().bold());
        Ok(())
    }

    pub fn display_error_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{} {}", "❌".bright_red(), message.bright_red().bold());
        Ok(())
    }

    pub fn display_warning_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{} {}", "⚠️".bright_yellow(), message.bright_yellow().bold());
        Ok(())
    }

    pub fn display_info_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{} {}", "ℹ️".bright_blue(), message.bright_blue());
        Ok(())
    }

    pub fn pause_for_user(&self) -> Result<(), Box<dyn std::error::Error>> {
        print!("\n{}", "Press Enter to continue...".bright_yellow().dimmed());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(())
    }
}