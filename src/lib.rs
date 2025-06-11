//! # Rust International Airport Management System
//! 
//! A comprehensive airport management system built entirely in Rust.
//! This system provides complete functionality for managing flights, aircraft,
//! bookings, airports, and administrative operations.
//! 
//! ## Features
//! 
//! - **Flight Management**: Search, view, and manage flight operations
//! - **Booking System**: Complete passenger booking with seat management
//! - **Aircraft Registry**: Detailed aircraft specifications and status tracking
//! - **Airport Operations**: Multi-airport support with terminal management
//! - **Admin Panel**: Role-based access with audit logging and system metrics
//! - **Real-time Simulation**: Automatic flight status updates and system monitoring
//! - **Data Persistence**: JSON-based storage with backup and validation
//! 
//! ## Architecture
//! 
//! The system is organized into several main modules:
//! 
//! - `modules`: Core data structures and business logic
//! - `data`: Data management and persistence layer
//! - `ui`: User interface and interaction components
//! 
//! ## Usage
//! 
//! ```rust
//! use rust_international_airport::data::manager::DataManager;
//! use rust_international_airport::ui::menu::MainMenu;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let data_manager = DataManager::new().await?;
//!     let mut menu = MainMenu::new(data_manager);
//!     menu.run().await?;
//!     Ok(())
//! }
//! ```

pub mod modules {
    //! Core data structures and business logic for the airport system.
    //! 
    //! This module contains all the fundamental types that represent
    //! the entities in our airport management system.
    
    pub mod flight;
    pub mod aircraft;
    pub mod booking;
    pub mod airport;
    pub mod admin;
}

pub mod data {
    //! Data management and persistence layer.
    //! 
    //! Handles all data operations including loading, saving, validation,
    //! and providing a unified interface for data access.
    
    pub mod manager;
    pub mod persistence;
}

pub mod ui {
    //! User interface and interaction components.
    //! 
    //! Provides a professional terminal-based interface with color coding,
    //! formatted displays, and robust input validation.
    
    pub mod menu;
    pub mod display;
    pub mod input;
}

// Re-export commonly used types for convenience
pub use modules::{
    flight::{Flight, FlightStatus, SeatClass},
    aircraft::{Aircraft, AircraftStatus},
    booking::{Booking, Passenger, PassengerType, BookingStatus},
    airport::Airport,
    admin::{AdminPanel, AdminUser, AdminLevel, SystemMetrics},
};

pub use data::{
    manager::DataManager,
    persistence::{DataPersistence, AirportDatabase},
};

pub use ui::{
    menu::MainMenu,
    display::DisplayManager,
    input::InputManager,
};

/// Version information for the Rust International Airport system
pub const VERSION: &str = "1.0.0";

/// System name
pub const SYSTEM_NAME: &str = "Rust International Airport";

/// System description
pub const SYSTEM_DESCRIPTION: &str = "Professional Airport Management System";

/// Default airport code for the main hub
pub const DEFAULT_HUB_CODE: &str = "RIA";

/// Maximum number of passengers per flight
pub const MAX_PASSENGERS_PER_FLIGHT: u32 = 853; // Airbus A380 capacity

/// Maximum delay time in minutes before automatic cancellation
pub const MAX_DELAY_MINUTES: i32 = 480; // 8 hours

/// Default currency for pricing
pub const DEFAULT_CURRENCY: &str = "USD";

/// System configuration and constants
pub mod config {
    //! System configuration constants and default values.
    
    /// Default data directory for file storage
    pub const DATA_DIR: &str = "data";
    
    /// Default backup directory
    pub const BACKUP_DIR: &str = "data/backups";
    
    /// Simulation update interval in seconds
    pub const SIMULATION_UPDATE_INTERVAL: u64 = 60;
    
    /// Maximum number of recent admin actions to display
    pub const MAX_ADMIN_LOG_ENTRIES: usize = 100;
    
    /// Default seat distribution percentages
    pub mod seats {
        pub const ECONOMY_PERCENTAGE: f32 = 0.70;
        pub const BUSINESS_PERCENTAGE: f32 = 0.25;
        pub const FIRST_CLASS_PERCENTAGE: f32 = 0.05;
    }
    
    /// Default pricing configuration
    pub mod pricing {
        pub const BASE_ECONOMY_PRICE: f64 = 299.99;
        pub const BASE_BUSINESS_PRICE: f64 = 899.99;
        pub const BASE_FIRST_CLASS_PRICE: f64 = 1999.99;
        pub const DEFAULT_MULTIPLIER: f64 = 1.0;
    }
    
    /// Baggage allowances by seat class (in kg)
    pub mod baggage {
        pub const ECONOMY_ALLOWANCE: u32 = 23;
        pub const BUSINESS_ALLOWANCE: u32 = 32;
        pub const FIRST_CLASS_ALLOWANCE: u32 = 46;
    }
}

/// Utility functions for the airport system
pub mod utils {
    //! Utility functions and helpers for common operations.
    
    use chrono::{DateTime, Utc, Duration};
    
    /// Calculate the distance between two geographical points using the Haversine formula
    pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371.0; // Earth's radius in kilometers
        
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();
        let delta_lat = (lat2 - lat1).to_radians();
        let delta_lon = (lon2 - lon1).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) +
                lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c
    }
    
    /// Estimate flight duration based on distance
    pub fn estimate_flight_duration(distance_km: f64) -> Duration {
        // Average commercial aircraft speed: 850 km/h
        let hours = distance_km / 850.0;
        Duration::milliseconds((hours * 3600.0 * 1000.0) as i64)
    }
    
    /// Format duration for display
    pub fn format_duration(duration: Duration) -> String {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        format!("{}h {}m", hours, minutes)
    }
    
    /// Generate a random seat number for a given row and seat count
    pub fn generate_seat_number(row: u32, max_seats_per_row: u32) -> String {
        let seat_letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K'];
        let seat_index = (row * max_seats_per_row) % seat_letters.len() as u32;
        format!("{}{}", row, seat_letters[seat_index as usize])
    }
    
    /// Validate an airport code (should be 3 uppercase letters)
    pub fn validate_airport_code(code: &str) -> bool {
        code.len() == 3 && code.chars().all(|c| c.is_ascii_uppercase())
    }
    
    /// Validate an email address (basic validation)
    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
    
    /// Format currency amount
    pub fn format_currency(amount: f64, currency: &str) -> String {
        match currency {
            "USD" => format!("${:.2}", amount),
            "EUR" => format!("€{:.2}", amount),
            "GBP" => format!("£{:.2}", amount),
            _ => format!("{:.2} {}", amount, currency),
        }
    }
    
    /// Calculate load factor percentage
    pub fn calculate_load_factor(booked_seats: u32, total_capacity: u32) -> f64 {
        if total_capacity == 0 {
            0.0
        } else {
            (booked_seats as f64 / total_capacity as f64) * 100.0
        }
    }
}

/// Error types specific to the airport system
pub mod errors {
    //! Custom error types for the airport management system.
    
    use thiserror::Error;
    use uuid::Uuid;
    
    #[derive(Error, Debug)]
    pub enum AirportError {
        #[error("Flight not found: {flight_id}")]
        FlightNotFound { flight_id: Uuid },
        
        #[error("Aircraft not found: {aircraft_id}")]
        AircraftNotFound { aircraft_id: Uuid },
        
        #[error("Booking not found: {ticket_number}")]
        BookingNotFound { ticket_number: String },
        
        #[error("Airport not found: {code}")]
        AirportNotFound { code: String },
        
        #[error("No seats available in {class:?}")]
        NoSeatsAvailable { class: crate::SeatClass },
        
        #[error("Flight {flight_number} is not available for booking")]
        FlightNotAvailable { flight_number: String },
        
        #[error("Insufficient permissions for operation: {operation}")]
        InsufficientPermissions { operation: String },
        
        #[error("Authentication failed for user: {username}")]
        AuthenticationFailed { username: String },
        
        #[error("Data validation failed: {message}")]
        ValidationError { message: String },
        
        #[error("System error: {message}")]
        SystemError { message: String },
    }
    
    pub type Result<T> = std::result::Result<T, AirportError>;
}

#[cfg(test)]
mod tests {
    //! Unit tests for the airport system.
    
    use super::*;
    use crate::utils::*;
    
    #[test]
    fn test_distance_calculation() {
        // Distance between LAX and JFK (approximately 3944 km)
        let distance = calculate_distance(33.9425, -118.4081, 40.6413, -73.7781);
        assert!((distance - 3944.0).abs() < 50.0); // Within 50km tolerance
    }
    
    #[test]
    fn test_airport_code_validation() {
        assert!(validate_airport_code("LAX"));
        assert!(validate_airport_code("JFK"));
        assert!(!validate_airport_code("lax"));
        assert!(!validate_airport_code("LAXX"));
        assert!(!validate_airport_code("LA"));
    }
    
    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test.email+tag@domain.co.uk"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
    }
    
    #[test]
    fn test_load_factor_calculation() {
        assert_eq!(calculate_load_factor(150, 200), 75.0);
        assert_eq!(calculate_load_factor(0, 200), 0.0);
        assert_eq!(calculate_load_factor(200, 200), 100.0);
        assert_eq!(calculate_load_factor(100, 0), 0.0);
    }
    
    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(299.99, "USD"), "$299.99");
        assert_eq!(format_currency(199.50, "EUR"), "€199.50");
        assert_eq!(format_currency(149.99, "GBP"), "£149.99");
        assert_eq!(format_currency(99.99, "CAD"), "99.99 CAD");
    }
}