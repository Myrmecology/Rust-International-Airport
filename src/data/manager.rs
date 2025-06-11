use std::error::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration, Timelike};
use crate::modules::{
    flight::{Flight, FlightStatus, SeatClass},
    aircraft::{Aircraft, AircraftStatus},
    booking::{Booking, Passenger, PassengerType, BookingStatus},
    airport::Airport,
    admin::{AdminPanel, AdminUser, AdminLevel, PricingRule, SystemMetrics},
};
use crate::data::persistence::{DataPersistence, AirportDatabase};

pub struct DataManager {
    pub database: AirportDatabase,
    pub persistence: DataPersistence,
    pub admin_panel: AdminPanel,
    last_simulation_update: DateTime<Utc>,
}

impl DataManager {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        println!("🔧 Initializing Rust International Airport Data Manager...");
        
        let persistence = DataPersistence::new();
        
        // Initialize data persistence and create sample data if needed
        persistence.initialize().await?;
        
        // Load all data from files
        let database = persistence.load_all_data().await?;
        
        // Validate data integrity
        let issues = persistence.validate_data_integrity().await?;
        if !issues.is_empty() {
            println!("⚠️ Data integrity issues found:");
            for issue in &issues {
                println!("  - {}", issue);
            }
        }
        
        let mut admin_panel = AdminPanel::new();
        
        // Initialize system metrics
        admin_panel.system_metrics.update_flight_metrics(&database.flights);
        admin_panel.system_metrics.update_aircraft_metrics(&database.aircraft);
        admin_panel.system_metrics.total_bookings = database.bookings.len() as u32;
        
        // Add some default pricing rules
        let default_rules = vec![
            PricingRule::new(
                "Peak Hours Premium".to_string(),
                None, // Apply to all routes
                Some((6, 9)), // 6 AM to 9 AM
                1.3, // 30% increase
                Uuid::new_v4(), // Default admin ID
            ),
            PricingRule::new(
                "Weekend Discount".to_string(),
                None,
                None, // All day
                0.9, // 10% discount
                Uuid::new_v4(),
            ),
            PricingRule::new(
                "Transatlantic Premium".to_string(),
                Some("*-LHR".to_string()), // Any route to London
                None,
                1.2, // 20% increase
                Uuid::new_v4(),
            ),
        ];
        
        for rule in default_rules {
            admin_panel.pricing_rules.push(rule);
        }

        println!("✅ Data Manager initialized successfully!");
        println!("📊 Loaded: {} flights, {} aircraft, {} bookings, {} airports", 
            database.flights.len(), 
            database.aircraft.len(), 
            database.bookings.len(), 
            database.airports.len()
        );

        Ok(Self {
            database,
            persistence,
            admin_panel,
            last_simulation_update: Utc::now(),
        })
    }

    // Flight Operations
    pub fn search_flights(
        &self, 
        origin: Option<&str>, 
        destination: Option<&str>, 
        date: Option<DateTime<Utc>>
    ) -> Vec<&Flight> {
        self.database.flights
            .iter()
            .filter(|flight| {
                if let Some(org) = origin {
                    if flight.origin != org {
                        return false;
                    }
                }
                if let Some(dest) = destination {
                    if flight.destination != dest {
                        return false;
                    }
                }
                if let Some(search_date) = date {
                    let flight_date = flight.departure_time.date_naive();
                    let search_date = search_date.date_naive();
                    if flight_date != search_date {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn get_flight_by_id(&self, flight_id: Uuid) -> Option<&Flight> {
        self.database.flights.iter().find(|f| f.id == flight_id)
    }

    pub fn get_flight_by_number(&self, flight_number: &str) -> Option<&Flight> {
        self.database.flights.iter().find(|f| f.flight_number == flight_number)
    }

    pub fn get_available_flights(&self) -> Vec<&Flight> {
        self.database.flights
            .iter()
            .filter(|f| f.is_available_for_booking())
            .collect()
    }

    // Booking Operations
    pub fn create_booking(
        &mut self,
        flight_id: Uuid,
        passenger: Passenger,
        seat_class: SeatClass,
    ) -> Result<Uuid, String> {
        // Find the flight
        let flight_idx = self.database.flights
            .iter()
            .position(|f| f.id == flight_id)
            .ok_or("Flight not found")?;

        // Check if flight is available for booking
        if !self.database.flights[flight_idx].is_available_for_booking() {
            return Err("Flight is not available for booking".to_string());
        }

        // Check seat availability
        if self.database.flights[flight_idx].get_available_seats(&seat_class) == 0 {
            return Err("No seats available in the selected class".to_string());
        }

        // Calculate price with dynamic multipliers
        let base_price = self.database.flights[flight_idx].get_price(&seat_class);
        let multiplier = self.admin_panel.get_applicable_multiplier(
            &self.database.flights[flight_idx].origin,
            &self.database.flights[flight_idx].destination,
            self.database.flights[flight_idx].departure_time.hour() as u8,
        );
        let final_price = base_price * multiplier;

        // Create booking
        let booking = Booking::new(
            flight_id,
            passenger,
            seat_class.clone(),
            final_price,
            "Credit Card".to_string(),
        );

        let booking_id = booking.id;

        // Reserve seat on flight
        self.database.flights[flight_idx].book_seat(&seat_class)?;

        // Add booking to database
        self.database.bookings.push(booking);

        // Update metrics
        self.admin_panel.system_metrics.total_bookings = self.database.bookings.len() as u32;
        self.admin_panel.system_metrics.revenue_today += final_price;
        self.admin_panel.system_metrics.revenue_month += final_price;

        println!("🎫 Booking created: {} for ${:.2}", booking_id, final_price);

        Ok(booking_id)
    }

    pub fn get_booking_by_ticket(&self, ticket_number: &str) -> Option<&Booking> {
        self.database.bookings.iter().find(|b| b.ticket_number == ticket_number)
    }

    pub fn get_booking_by_id(&self, booking_id: Uuid) -> Option<&Booking> {
        self.database.bookings.iter().find(|b| b.id == booking_id)
    }

    pub fn cancel_booking(&mut self, ticket_number: &str) -> Result<(), String> {
        let booking_idx = self.database.bookings
            .iter()
            .position(|b| b.ticket_number == ticket_number)
            .ok_or("Booking not found")?;

        // Cancel the booking
        self.database.bookings[booking_idx].cancel()?;

        // Find the associated flight and free up the seat
        let flight_id = self.database.bookings[booking_idx].flight_id;
        let seat_class = self.database.bookings[booking_idx].seat_class.clone();

        if let Some(flight) = self.database.flights.iter_mut().find(|f| f.id == flight_id) {
            // Add seat back to availability
            match seat_class {
                SeatClass::Economy => flight.seat_availability.economy += 1,
                SeatClass::Business => flight.seat_availability.business += 1,
                SeatClass::FirstClass => flight.seat_availability.first_class += 1,
            }
        }

        println!("❌ Booking cancelled: {}", ticket_number);
        Ok(())
    }

    // Aircraft Operations
    pub fn get_aircraft_by_id(&self, aircraft_id: Uuid) -> Option<&Aircraft> {
        self.database.aircraft.iter().find(|a| a.id == aircraft_id)
    }

    pub fn get_available_aircraft(&self) -> Vec<&Aircraft> {
        self.database.aircraft
            .iter()
            .filter(|a| a.is_available_for_flight())
            .collect()
    }

    pub fn get_aircraft_for_flight(&self, flight_id: Uuid) -> Option<&Aircraft> {
        if let Some(flight) = self.get_flight_by_id(flight_id) {
            self.get_aircraft_by_id(flight.aircraft_id)
        } else {
            None
        }
    }

    // Airport Operations
    pub fn get_airport_by_code(&self, code: &str) -> Option<&Airport> {
        self.database.airports.iter().find(|a| a.code == code)
    }

    pub fn get_all_airports(&self) -> &Vec<Airport> {
        &self.database.airports
    }

    pub fn get_departures_from_airport(&self, airport_code: &str) -> Vec<&Flight> {
        self.database.flights
            .iter()
            .filter(|f| f.origin == airport_code)
            .collect()
    }

    pub fn get_arrivals_to_airport(&self, airport_code: &str) -> Vec<&Flight> {
        self.database.flights
            .iter()
            .filter(|f| f.destination == airport_code)
            .collect()
    }

    // Admin Operations
    pub fn authenticate_admin(&mut self, username: &str, password: &str) -> Result<AdminUser, String> {
        self.admin_panel.authenticate(username, password)
    }

    pub fn is_admin_authenticated(&self) -> bool {
        self.admin_panel.is_authenticated()
    }

    pub fn logout_admin(&mut self) {
        self.admin_panel.logout();
    }

    pub fn set_flight_delay(&mut self, flight_number: &str, delay_minutes: i32) -> Result<(), String> {
        if !self.admin_panel.is_authenticated() {
            return Err("Admin authentication required".to_string());
        }

        let current_admin = self.admin_panel.current_admin.as_ref().unwrap();
        if !current_admin.can_manage_flights() {
            return Err("Insufficient permissions to manage flights".to_string());
        }

        let flight = self.database.flights
            .iter_mut()
            .find(|f| f.flight_number == flight_number)
            .ok_or("Flight not found")?;

        let old_status = flight.get_status_display();
        flight.set_delay(delay_minutes);
        let new_status = flight.get_status_display();

        // Log the action
        self.admin_panel.log_action(
            current_admin.id,
            "SET_DELAY".to_string(),
            format!("Set delay for flight {}", flight_number),
            Some(flight.id),
            Some(old_status),
            Some(new_status),
        );

        println!("⏰ Flight {} delay set to {} minutes", flight_number, delay_minutes);
        Ok(())
    }

    pub fn set_dynamic_pricing(&mut self, flight_number: &str, multiplier: f64) -> Result<(), String> {
        if !self.admin_panel.is_authenticated() {
            return Err("Admin authentication required".to_string());
        }

        let current_admin = self.admin_panel.current_admin.as_ref().unwrap();
        if !current_admin.can_manage_pricing() {
            return Err("Insufficient permissions to manage pricing".to_string());
        }

        let flight = self.database.flights
            .iter_mut()
            .find(|f| f.flight_number == flight_number)
            .ok_or("Flight not found")?;

        let old_multiplier = flight.pricing.dynamic_multiplier;
        flight.pricing.dynamic_multiplier = multiplier;

        // Log the action
        self.admin_panel.log_action(
            current_admin.id,
            "SET_PRICING".to_string(),
            format!("Set pricing multiplier for flight {}", flight_number),
            Some(flight.id),
            Some(old_multiplier.to_string()),
            Some(multiplier.to_string()),
        );

        println!("💰 Flight {} pricing multiplier set to {:.2}", flight_number, multiplier);
        Ok(())
    }

    // Real-time Simulation
    pub async fn update_simulation(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Utc::now();
        
        // Only update every minute
        if now.signed_duration_since(self.last_simulation_update).num_seconds() < 60 {
            return Ok(());
        }

        let mut updates_made = false;

        // Update flight statuses based on current time
        for flight in &mut self.database.flights {
            let time_to_departure = flight.departure_time.signed_duration_since(now);
            let time_since_departure = now.signed_duration_since(flight.departure_time);
            let time_to_arrival = flight.arrival_time.signed_duration_since(now);

            match flight.status {
                FlightStatus::OnTime | FlightStatus::Delayed(_) => {
                    if time_to_departure <= Duration::minutes(30) && time_to_departure > Duration::minutes(0) {
                        flight.status = FlightStatus::Boarding;
                        updates_made = true;
                    } else if time_since_departure >= Duration::minutes(0) && time_to_arrival > Duration::minutes(0) {
                        flight.status = FlightStatus::Departed;
                        updates_made = true;
                    } else if time_to_arrival <= Duration::minutes(0) {
                        flight.status = FlightStatus::Arrived;
                        updates_made = true;
                    }
                }
                FlightStatus::Boarding => {
                    if time_since_departure >= Duration::minutes(0) {
                        flight.status = FlightStatus::Departed;
                        updates_made = true;
                    }
                }
                FlightStatus::Departed => {
                    if time_to_arrival <= Duration::minutes(0) {
                        flight.status = FlightStatus::Arrived;
                        updates_made = true;
                    }
                }
                _ => {} // No updates needed for other statuses
            }
        }

        // Update aircraft statuses based on flight status
        for aircraft in &mut self.database.aircraft {
            let has_active_flight = self.database.flights
                .iter()
                .any(|f| f.aircraft_id == aircraft.id && 
                         matches!(f.status, FlightStatus::Boarding | FlightStatus::Departed));

            match aircraft.status {
                AircraftStatus::Active => {
                    if has_active_flight {
                        aircraft.status = AircraftStatus::InFlight;
                        updates_made = true;
                    }
                }
                AircraftStatus::InFlight => {
                    if !has_active_flight {
                        aircraft.status = AircraftStatus::Active;
                        updates_made = true;
                    }
                }
                _ => {} // No automatic updates for maintenance or retired aircraft
            }
        }

        if updates_made {
            // Update system metrics
            self.admin_panel.system_metrics.update_flight_metrics(&self.database.flights);
            self.admin_panel.system_metrics.update_aircraft_metrics(&self.database.aircraft);
            
            println!("🔄 Simulation updated - {} flights, {} aircraft statuses updated", 
                self.database.flights.len(), self.database.aircraft.len());
        }

        self.last_simulation_update = now;
        Ok(())
    }

    // Data Persistence Operations
    pub async fn save_all_data(&self) -> Result<(), Box<dyn Error>> {
        self.persistence.save_all_data(&self.database).await?;
        Ok(())
    }

    pub async fn create_backup(&self) -> Result<String, Box<dyn Error>> {
        let backup_path = self.persistence.create_backup().await?;
        Ok(backup_path)
    }

    // Statistics and Reporting
    pub fn get_system_metrics(&self) -> &SystemMetrics {
        &self.admin_panel.system_metrics
    }

    pub fn get_flight_statistics(&self) -> (u32, u32, u32, u32) {
        let total = self.database.flights.len() as u32;
        let on_time = self.database.flights.iter()
            .filter(|f| matches!(f.status, FlightStatus::OnTime))
            .count() as u32;
        let delayed = self.database.flights.iter()
            .filter(|f| matches!(f.status, FlightStatus::Delayed(_)))
            .count() as u32;
        let cancelled = self.database.flights.iter()
            .filter(|f| matches!(f.status, FlightStatus::Cancelled))
            .count() as u32;
        
        (total, on_time, delayed, cancelled)
    }

    pub fn get_booking_statistics(&self) -> (u32, u32, u32) {
        let total = self.database.bookings.len() as u32;
        let confirmed = self.database.bookings.iter()
            .filter(|b| matches!(b.status, BookingStatus::Confirmed | BookingStatus::CheckedIn))
            .count() as u32;
        let cancelled = self.database.bookings.iter()
            .filter(|b| matches!(b.status, BookingStatus::Cancelled))
            .count() as u32;
        
        (total, confirmed, cancelled)
    }
}