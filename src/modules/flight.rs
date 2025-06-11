use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlightStatus {
    OnTime,
    Delayed(i32), // minutes delayed
    Boarding,
    Departed,
    Arrived,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SeatClass {
    Economy,
    Business,
    FirstClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatAvailability {
    pub economy: u32,
    pub business: u32,
    pub first_class: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightPricing {
    pub economy: f64,
    pub business: f64,
    pub first_class: f64,
    pub dynamic_multiplier: f64, // For admin dynamic pricing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight {
    pub id: Uuid,
    pub flight_number: String,
    pub airline: String,
    pub origin: String,          // Airport code (e.g., "LAX")
    pub destination: String,     // Airport code (e.g., "JFK")
    pub departure_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
    pub status: FlightStatus,
    pub aircraft_id: Uuid,
    pub gate: Option<String>,
    pub seat_availability: SeatAvailability,
    pub pricing: FlightPricing,
    pub total_capacity: u32,
    pub baggage_allowance: HashMap<SeatClass, u32>, // kg per class
}

impl Flight {
    pub fn new(
        flight_number: String,
        airline: String,
        origin: String,
        destination: String,
        departure_time: DateTime<Utc>,
        arrival_time: DateTime<Utc>,
        aircraft_id: Uuid,
        total_capacity: u32,
    ) -> Self {
        let economy_seats = (total_capacity as f32 * 0.7) as u32;
        let business_seats = (total_capacity as f32 * 0.25) as u32;
        let first_class_seats = total_capacity - economy_seats - business_seats;

        let mut baggage_allowance = HashMap::new();
        baggage_allowance.insert(SeatClass::Economy, 23);
        baggage_allowance.insert(SeatClass::Business, 32);
        baggage_allowance.insert(SeatClass::FirstClass, 46);

        Self {
            id: Uuid::new_v4(),
            flight_number,
            airline,
            origin,
            destination,
            departure_time,
            arrival_time,
            status: FlightStatus::OnTime,
            aircraft_id,
            gate: None,
            seat_availability: SeatAvailability {
                economy: economy_seats,
                business: business_seats,
                first_class: first_class_seats,
            },
            pricing: FlightPricing {
                economy: 299.99,
                business: 899.99,
                first_class: 1999.99,
                dynamic_multiplier: 1.0,
            },
            total_capacity,
            baggage_allowance,
        }
    }

    pub fn duration(&self) -> Duration {
        self.arrival_time - self.departure_time
    }

    pub fn is_available_for_booking(&self) -> bool {
        matches!(self.status, FlightStatus::OnTime | FlightStatus::Delayed(_))
            && self.departure_time > Utc::now()
    }

    pub fn get_available_seats(&self, class: &SeatClass) -> u32 {
        match class {
            SeatClass::Economy => self.seat_availability.economy,
            SeatClass::Business => self.seat_availability.business,
            SeatClass::FirstClass => self.seat_availability.first_class,
        }
    }

    pub fn get_price(&self, class: &SeatClass) -> f64 {
        let base_price = match class {
            SeatClass::Economy => self.pricing.economy,
            SeatClass::Business => self.pricing.business,
            SeatClass::FirstClass => self.pricing.first_class,
        };
        base_price * self.pricing.dynamic_multiplier
    }

    pub fn book_seat(&mut self, class: &SeatClass) -> Result<(), String> {
        if !self.is_available_for_booking() {
            return Err("Flight is not available for booking".to_string());
        }

        match class {
            SeatClass::Economy => {
                if self.seat_availability.economy > 0 {
                    self.seat_availability.economy -= 1;
                    Ok(())
                } else {
                    Err("No economy seats available".to_string())
                }
            }
            SeatClass::Business => {
                if self.seat_availability.business > 0 {
                    self.seat_availability.business -= 1;
                    Ok(())
                } else {
                    Err("No business seats available".to_string())
                }
            }
            SeatClass::FirstClass => {
                if self.seat_availability.first_class > 0 {
                    self.seat_availability.first_class -= 1;
                    Ok(())
                } else {
                    Err("No first class seats available".to_string())
                }
            }
        }
    }

    pub fn set_delay(&mut self, minutes: i32) {
        if minutes > 0 {
            self.status = FlightStatus::Delayed(minutes);
            // Update arrival time accordingly
            self.arrival_time = self.arrival_time + Duration::minutes(minutes as i64);
        } else {
            self.status = FlightStatus::OnTime;
        }
    }

    pub fn set_gate(&mut self, gate: String) {
        self.gate = Some(gate);
    }

    pub fn get_status_display(&self) -> String {
        match &self.status {
            FlightStatus::OnTime => "On Time ✅".to_string(),
            FlightStatus::Delayed(mins) => format!("Delayed {} min ⏰", mins),
            FlightStatus::Boarding => "Boarding 🚪".to_string(),
            FlightStatus::Departed => "Departed ✈️".to_string(),
            FlightStatus::Arrived => "Arrived 🛬".to_string(),
            FlightStatus::Cancelled => "Cancelled ❌".to_string(),
        }
    }
}

impl std::fmt::Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} → {} | {} | {}",
            self.flight_number,
            self.origin,
            self.destination,
            self.departure_time.format("%H:%M"),
            self.get_status_display()
        )
    }
}