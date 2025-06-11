use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use crate::modules::{
    flight::{Flight, FlightStatus, SeatClass},
    aircraft::{Aircraft, AircraftStatus},
    booking::{Booking, Passenger, PassengerType},
    airport::Airport,
    admin::{AdminPanel, PricingRule, AdminUser, AdminLevel},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportDatabase {
    pub flights: Vec<Flight>,
    pub aircraft: Vec<Aircraft>,
    pub bookings: Vec<Booking>,
    pub airports: Vec<Airport>,
}

pub struct DataPersistence {
    data_dir: String,
}

impl DataPersistence {
    pub fn new() -> Self {
        Self {
            data_dir: "data".to_string(),
        }
    }

    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure data directories exist
        self.ensure_directories()?;
        
        // Create sample data files if they don't exist
        if !Path::new(&format!("{}/airports.json", self.data_dir)).exists() {
            self.create_sample_airports().await?;
        }
        
        if !Path::new(&format!("{}/aircraft.json", self.data_dir)).exists() {
            self.create_sample_aircraft().await?;
        }
        
        if !Path::new(&format!("{}/flights.json", self.data_dir)).exists() {
            self.create_sample_flights().await?;
        }

        Ok(())
    }

    fn ensure_directories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let directories = [
            &self.data_dir,
            &format!("{}/flights", self.data_dir),
            &format!("{}/bookings", self.data_dir),
            &format!("{}/aircraft", self.data_dir),
        ];

        for dir in &directories {
            if !Path::new(dir).exists() {
                fs::create_dir_all(dir)?;
                println!("📁 Created directory: {}", dir);
            }
        }

        Ok(())
    }

    // Airport Data Management
    pub async fn load_airports(&self) -> Result<Vec<Airport>, Box<dyn std::error::Error>> {
        let file_path = format!("{}/airports.json", self.data_dir);
        
        if !Path::new(&file_path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&file_path)?;
        let airports: Vec<Airport> = serde_json::from_str(&content)?;
        
        println!("✈️ Loaded {} airports", airports.len());
        Ok(airports)
    }

    pub async fn save_airports(&self, airports: &[Airport]) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/airports.json", self.data_dir);
        let content = serde_json::to_string_pretty(airports)?;
        fs::write(&file_path, content)?;
        
        println!("💾 Saved {} airports", airports.len());
        Ok(())
    }

    // Aircraft Data Management
    pub async fn load_aircraft(&self) -> Result<Vec<Aircraft>, Box<dyn std::error::Error>> {
        let file_path = format!("{}/aircraft.json", self.data_dir);
        
        if !Path::new(&file_path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&file_path)?;
        let aircraft: Vec<Aircraft> = serde_json::from_str(&content)?;
        
        println!("🛩️ Loaded {} aircraft", aircraft.len());
        Ok(aircraft)
    }

    pub async fn save_aircraft(&self, aircraft: &[Aircraft]) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/aircraft.json", self.data_dir);
        let content = serde_json::to_string_pretty(aircraft)?;
        fs::write(&file_path, content)?;
        
        println!("💾 Saved {} aircraft", aircraft.len());
        Ok(())
    }

    // Flight Data Management
    pub async fn load_flights(&self) -> Result<Vec<Flight>, Box<dyn std::error::Error>> {
        let file_path = format!("{}/flights.json", self.data_dir);
        
        if !Path::new(&file_path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&file_path)?;
        let flights: Vec<Flight> = serde_json::from_str(&content)?;
        
        println!("🛫 Loaded {} flights", flights.len());
        Ok(flights)
    }

    pub async fn save_flights(&self, flights: &[Flight]) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/flights.json", self.data_dir);
        let content = serde_json::to_string_pretty(flights)?;
        fs::write(&file_path, content)?;
        
        println!("💾 Saved {} flights", flights.len());
        Ok(())
    }

    // Booking Data Management
    pub async fn load_bookings(&self) -> Result<Vec<Booking>, Box<dyn std::error::Error>> {
        let file_path = format!("{}/bookings.json", self.data_dir);
        
        if !Path::new(&file_path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&file_path)?;
        let bookings: Vec<Booking> = serde_json::from_str(&content)?;
        
        println!("🎫 Loaded {} bookings", bookings.len());
        Ok(bookings)
    }

    pub async fn save_bookings(&self, bookings: &[Booking]) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/bookings.json", self.data_dir);
        let content = serde_json::to_string_pretty(bookings)?;
        fs::write(&file_path, content)?;
        
        println!("💾 Saved {} bookings", bookings.len());
        Ok(())
    }

    // Sample Data Creation
    async fn create_sample_airports(&self) -> Result<(), Box<dyn std::error::Error>> {
        let airports = vec![
            Airport::new(
                "LAX".to_string(),
                "KLAX".to_string(),
                "Los Angeles International Airport".to_string(),
                "Los Angeles".to_string(),
                "United States".to_string(),
                "America/Los_Angeles".to_string(),
                33.9425, -118.4081, 38,
            ),
            Airport::new(
                "JFK".to_string(),
                "KJFK".to_string(),
                "John F. Kennedy International Airport".to_string(),
                "New York".to_string(),
                "United States".to_string(),
                "America/New_York".to_string(),
                40.6413, -73.7781, 4,
            ),
            Airport::new(
                "LHR".to_string(),
                "EGLL".to_string(),
                "Heathrow Airport".to_string(),
                "London".to_string(),
                "United Kingdom".to_string(),
                "Europe/London".to_string(),
                51.4700, -0.4543, 25,
            ),
            Airport::new(
                "CDG".to_string(),
                "LFPG".to_string(),
                "Charles de Gaulle Airport".to_string(),
                "Paris".to_string(),
                "France".to_string(),
                "Europe/Paris".to_string(),
                49.0097, 2.5479, 119,
            ),
            Airport::new(
                "NRT".to_string(),
                "RJAA".to_string(),
                "Narita International Airport".to_string(),
                "Tokyo".to_string(),
                "Japan".to_string(),
                "Asia/Tokyo".to_string(),
                35.7653, 140.3856, 43,
            ),
            Airport::new(
                "DXB".to_string(),
                "OMDB".to_string(),
                "Dubai International Airport".to_string(),
                "Dubai".to_string(),
                "United Arab Emirates".to_string(),
                "Asia/Dubai".to_string(),
                25.2532, 55.3657, 20,
            ),
        ];

        self.save_airports(&airports).await?;
        println!("🌍 Created sample airports database");
        Ok(())
    }

    async fn create_sample_aircraft(&self) -> Result<(), Box<dyn std::error::Error>> {
        let aircraft = vec![
            Aircraft::new(
                "N123RIA".to_string(),
                "Boeing 737-800".to_string(),
                "Boeing".to_string(),
                2020,
            ),
            Aircraft::new(
                "N456RIA".to_string(),
                "Airbus A320".to_string(),
                "Airbus".to_string(),
                2019,
            ),
            Aircraft::new(
                "N789RIA".to_string(),
                "Boeing 777-300".to_string(),
                "Boeing".to_string(),
                2021,
            ),
            Aircraft::new(
                "N101RIA".to_string(),
                "Airbus A380".to_string(),
                "Airbus".to_string(),
                2018,
            ),
            Aircraft::new(
                "N202RIA".to_string(),
                "Boeing 737-800".to_string(),
                "Boeing".to_string(),
                2022,
            ),
            Aircraft::new(
                "N303RIA".to_string(),
                "Airbus A320".to_string(),
                "Airbus".to_string(),
                2023,
            ),
        ];

        self.save_aircraft(&aircraft).await?;
        println!("🛩️ Created sample aircraft database");
        Ok(())
    }

    async fn create_sample_flights(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Load aircraft to get their IDs for flight assignment
        let aircraft = self.load_aircraft().await?;
        
        if aircraft.is_empty() {
            return Err("No aircraft available for sample flights".into());
        }

        let now = Utc::now();
        let base_time = now + Duration::hours(2); // Start flights 2 hours from now

        let sample_routes = vec![
            ("LAX", "JFK", "RIA101", "Rust International Airways"),
            ("JFK", "LHR", "RIA201", "Rust International Airways"),
            ("LHR", "CDG", "RIA301", "Rust International Airways"),
            ("CDG", "NRT", "RIA401", "Rust International Airways"),
            ("NRT", "DXB", "RIA501", "Rust International Airways"),
            ("DXB", "LAX", "RIA601", "Rust International Airways"),
            ("LAX", "CDG", "RIA701", "Rust International Airways"),
            ("JFK", "NRT", "RIA801", "Rust International Airways"),
            ("LHR", "DXB", "RIA901", "Rust International Airways"),
            ("CDG", "LAX", "RIA001", "Rust International Airways"),
        ];

        let mut flights = Vec::new();

        for (i, (origin, destination, flight_num, airline)) in sample_routes.iter().enumerate() {
            let aircraft_id = aircraft[i % aircraft.len()].id;
            let departure_time = base_time + Duration::hours(i as i64 * 3);
            let flight_duration = Duration::hours(8 + (i as i64 % 4)); // 8-11 hour flights
            let arrival_time = departure_time + flight_duration;

            let mut flight = Flight::new(
                flight_num.to_string(),
                airline.to_string(),
                origin.to_string(),
                destination.to_string(),
                departure_time,
                arrival_time,
                aircraft_id,
                aircraft[i % aircraft.len()].total_capacity,
            );

            // Add some variety to flight statuses
            match i % 4 {
                0 => flight.status = FlightStatus::OnTime,
                1 => flight.set_delay(15),
                2 => flight.status = FlightStatus::Boarding,
                3 => flight.set_delay(30),
                _ => {}
            }

            // Assign gates
            let gates = ["A1", "A2", "B3", "B4", "C5", "C6", "D7", "D8", "E9", "E10"];
            flight.set_gate(gates[i % gates.len()].to_string());

            flights.push(flight);
        }

        self.save_flights(&flights).await?;
        println!("🛫 Created sample flights database");
        Ok(())
    }

    // Combined database operations
    pub async fn load_all_data(&self) -> Result<AirportDatabase, Box<dyn std::error::Error>> {
        let flights = self.load_flights().await?;
        let aircraft = self.load_aircraft().await?;
        let bookings = self.load_bookings().await?;
        let airports = self.load_airports().await?;

        Ok(AirportDatabase {
            flights,
            aircraft,
            bookings,
            airports,
        })
    }

    pub async fn save_all_data(&self, database: &AirportDatabase) -> Result<(), Box<dyn std::error::Error>> {
        self.save_flights(&database.flights).await?;
        self.save_aircraft(&database.aircraft).await?;
        self.save_bookings(&database.bookings).await?;
        self.save_airports(&database.airports).await?;
        
        println!("💾 Saved complete airport database");
        Ok(())
    }

    // Backup operations
    pub async fn create_backup(&self) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_dir = format!("{}/backups/{}", self.data_dir, timestamp);
        
        fs::create_dir_all(&backup_dir)?;
        
        // Copy all data files to backup directory
        let files = ["airports.json", "aircraft.json", "flights.json", "bookings.json"];
        
        for file in &files {
            let source = format!("{}/{}", self.data_dir, file);
            let destination = format!("{}/{}", backup_dir, file);
            
            if Path::new(&source).exists() {
                fs::copy(&source, &destination)?;
            }
        }
        
        println!("📋 Created backup: {}", backup_dir);
        Ok(backup_dir)
    }

    // Data validation
    pub async fn validate_data_integrity(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut issues = Vec::new();
        
        let database = self.load_all_data().await?;
        
        // Validate flight-aircraft relationships
        for flight in &database.flights {
            if !database.aircraft.iter().any(|a| a.id == flight.aircraft_id) {
                issues.push(format!("Flight {} references non-existent aircraft {}", 
                    flight.flight_number, flight.aircraft_id));
            }
        }
        
        // Validate booking-flight relationships
        for booking in &database.bookings {
            if !database.flights.iter().any(|f| f.id == booking.flight_id) {
                issues.push(format!("Booking {} references non-existent flight {}", 
                    booking.ticket_number, booking.flight_id));
            }
        }
        
        // Validate airport codes in flights
        let airport_codes: Vec<&String> = database.airports.iter().map(|a| &a.code).collect();
        for flight in &database.flights {
            if !airport_codes.contains(&&flight.origin) {
                issues.push(format!("Flight {} has invalid origin airport: {}", 
                    flight.flight_number, flight.origin));
            }
            if !airport_codes.contains(&&flight.destination) {
                issues.push(format!("Flight {} has invalid destination airport: {}", 
                    flight.flight_number, flight.destination));
            }
        }
        
        if issues.is_empty() {
            println!("✅ Data integrity validation passed");
        } else {
            println!("⚠️ Found {} data integrity issues", issues.len());
        }
        
        Ok(issues)
    }
}