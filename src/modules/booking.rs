use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::modules::flight::SeatClass;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BookingStatus {
    Confirmed,
    CheckedIn,
    Boarded,
    Completed,
    Cancelled,
    NoShow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassengerType {
    Adult,
    Child,
    Infant,
    Senior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passenger {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub passport_number: Option<String>,
    pub date_of_birth: String, // Format: "YYYY-MM-DD"
    pub passenger_type: PassengerType,
    pub special_requirements: Vec<String>, // e.g., "Wheelchair", "Vegetarian meal"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatAssignment {
    pub seat_number: String,   // e.g., "12A", "3F"
    pub seat_class: SeatClass,
    pub is_window: bool,
    pub is_aisle: bool,
    pub is_emergency_exit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingPayment {
    pub total_amount: f64,
    pub currency: String,
    pub payment_method: String, // e.g., "Credit Card", "PayPal"
    pub transaction_id: String,
    pub payment_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,                    // Ticket UUID
    pub ticket_number: String,       // Human-readable ticket number
    pub flight_id: Uuid,
    pub passenger: Passenger,
    pub seat_assignment: Option<SeatAssignment>,
    pub seat_class: SeatClass,
    pub booking_date: DateTime<Utc>,
    pub status: BookingStatus,
    pub payment: BookingPayment,
    pub baggage_count: u32,
    pub special_services: Vec<String>, // e.g., "Extra legroom", "Priority boarding"
    pub check_in_time: Option<DateTime<Utc>>,
    pub boarding_time: Option<DateTime<Utc>>,
}

impl Passenger {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone: String,
        date_of_birth: String,
        passenger_type: PassengerType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            phone,
            passport_number: None,
            date_of_birth,
            passenger_type,
            special_requirements: Vec::new(),
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn add_special_requirement(&mut self, requirement: String) {
        if !self.special_requirements.contains(&requirement) {
            self.special_requirements.push(requirement);
        }
    }

    pub fn set_passport(&mut self, passport_number: String) {
        self.passport_number = Some(passport_number);
    }
}

impl SeatAssignment {
    pub fn new(seat_number: String, seat_class: SeatClass) -> Self {
        // Simple logic to determine seat characteristics
        let is_window = seat_number.ends_with('A') || seat_number.ends_with('F');
        let is_aisle = seat_number.ends_with('C') || seat_number.ends_with('D');
        let row_number: u32 = seat_number.chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(1);
        let is_emergency_exit = row_number >= 12 && row_number <= 15; // Typical emergency exit rows

        Self {
            seat_number,
            seat_class,
            is_window,
            is_aisle,
            is_emergency_exit,
        }
    }

    pub fn get_seat_type(&self) -> String {
        let mut types = Vec::new();
        
        if self.is_window {
            types.push("Window");
        } else if self.is_aisle {
            types.push("Aisle");
        } else {
            types.push("Middle");
        }

        if self.is_emergency_exit {
            types.push("Emergency Exit");
        }

        types.join(" + ")
    }
}

impl Booking {
    pub fn new(
        flight_id: Uuid,
        passenger: Passenger,
        seat_class: SeatClass,
        total_amount: f64,
        payment_method: String,
    ) -> Self {
        let booking_id = Uuid::new_v4();
        let ticket_number = Self::generate_ticket_number();
        
        let payment = BookingPayment {
            total_amount,
            currency: "USD".to_string(),
            payment_method,
            transaction_id: Uuid::new_v4().to_string(),
            payment_date: Utc::now(),
        };

        Self {
            id: booking_id,
            ticket_number,
            flight_id,
            passenger,
            seat_assignment: None,
            seat_class,
            booking_date: Utc::now(),
            status: BookingStatus::Confirmed,
            payment,
            baggage_count: 1, // Default one bag
            special_services: Vec::new(),
            check_in_time: None,
            boarding_time: None,
        }
    }

    fn generate_ticket_number() -> String {
        // Generate a human-readable ticket number (airline code + 6 digits)
        let airline_code = "RIA"; // Rust International Airport
        let number = rand::random::<u32>() % 1000000;
        format!("{}{:06}", airline_code, number)
    }

    pub fn assign_seat(&mut self, seat_number: String) {
        self.seat_assignment = Some(SeatAssignment::new(seat_number, self.seat_class.clone()));
    }

    pub fn check_in(&mut self) -> Result<(), String> {
        match self.status {
            BookingStatus::Confirmed => {
                self.status = BookingStatus::CheckedIn;
                self.check_in_time = Some(Utc::now());
                Ok(())
            }
            _ => Err("Cannot check in - booking not in confirmed status".to_string()),
        }
    }

    pub fn board(&mut self) -> Result<(), String> {
        match self.status {
            BookingStatus::CheckedIn => {
                self.status = BookingStatus::Boarded;
                self.boarding_time = Some(Utc::now());
                Ok(())
            }
            _ => Err("Cannot board - must be checked in first".to_string()),
        }
    }

    pub fn cancel(&mut self) -> Result<(), String> {
        match self.status {
            BookingStatus::Confirmed | BookingStatus::CheckedIn => {
                self.status = BookingStatus::Cancelled;
                Ok(())
            }
            BookingStatus::Boarded | BookingStatus::Completed => {
                Err("Cannot cancel - flight already boarded or completed".to_string())
            }
            _ => Err("Booking already cancelled or invalid status".to_string()),
        }
    }

    pub fn add_baggage(&mut self, count: u32) {
        self.baggage_count += count;
    }

    pub fn add_special_service(&mut self, service: String) {
        if !self.special_services.contains(&service) {
            self.special_services.push(service);
        }
    }

    pub fn get_status_display(&self) -> String {
        match self.status {
            BookingStatus::Confirmed => "Confirmed ✅".to_string(),
            BookingStatus::CheckedIn => "Checked In 🎫".to_string(),
            BookingStatus::Boarded => "Boarded ✈️".to_string(),
            BookingStatus::Completed => "Completed 🛬".to_string(),
            BookingStatus::Cancelled => "Cancelled ❌".to_string(),
            BookingStatus::NoShow => "No Show ⚠️".to_string(),
        }
    }

    pub fn get_ticket_summary(&self) -> String {
        let seat_info = match &self.seat_assignment {
            Some(seat) => format!("Seat: {} ({})", seat.seat_number, seat.get_seat_type()),
            None => "Seat: Not assigned".to_string(),
        };

        format!(
            "Ticket: {} | Passenger: {} | Class: {:?} | {} | Status: {} | Amount: ${:.2}",
            self.ticket_number,
            self.passenger.full_name(),
            self.seat_class,
            seat_info,
            self.get_status_display(),
            self.payment.total_amount
        )
    }

    pub fn can_be_modified(&self) -> bool {
        matches!(self.status, BookingStatus::Confirmed | BookingStatus::CheckedIn)
    }
}

impl std::fmt::Display for Booking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.get_ticket_summary()
        )
    }
}

// Random number generation for ticket numbers
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random<T: Hash>() -> u64 {
        let mut hasher = DefaultHasher::new();
        
        // Use current time as seed
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        time.hash(&mut hasher);
        hasher.finish()
    }
}