use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::modules::flight::SeatClass;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AircraftStatus {
    Active,
    Maintenance,
    Retired,
    InFlight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatConfiguration {
    pub economy_rows: u32,
    pub economy_seats_per_row: u32,
    pub business_rows: u32,
    pub business_seats_per_row: u32,
    pub first_class_rows: u32,
    pub first_class_seats_per_row: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSpecs {
    pub max_speed_kmh: u32,
    pub cruise_speed_kmh: u32,
    pub max_altitude_m: u32,
    pub range_km: u32,
    pub fuel_efficiency_l_per_100km: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aircraft {
    pub id: Uuid,
    pub registration: String,     // e.g., "N123AA"
    pub model: String,           // e.g., "Boeing 737-800"
    pub manufacturer: String,    // e.g., "Boeing"
    pub year_manufactured: u32,
    pub status: AircraftStatus,
    pub seat_configuration: SeatConfiguration,
    pub total_capacity: u32,
    pub baggage_capacity_kg: u32,
    pub max_cargo_weight_kg: u32,
    pub performance: PerformanceSpecs,
    pub maintenance_hours: f64,
    pub flight_hours: f64,
}

impl Aircraft {
    pub fn new(
        registration: String,
        model: String,
        manufacturer: String,
        year_manufactured: u32,
    ) -> Self {
        // Default configuration for a medium-haul aircraft
        let seat_config = match model.as_str() {
            "Boeing 737-800" => SeatConfiguration {
                economy_rows: 28,
                economy_seats_per_row: 6,
                business_rows: 4,
                business_seats_per_row: 4,
                first_class_rows: 2,
                first_class_seats_per_row: 4,
            },
            "Airbus A320" => SeatConfiguration {
                economy_rows: 25,
                economy_seats_per_row: 6,
                business_rows: 3,
                business_seats_per_row: 4,
                first_class_rows: 2,
                first_class_seats_per_row: 4,
            },
            "Boeing 777-300" => SeatConfiguration {
                economy_rows: 42,
                economy_seats_per_row: 9,
                business_rows: 8,
                business_seats_per_row: 6,
                first_class_rows: 4,
                first_class_seats_per_row: 4,
            },
            "Airbus A380" => SeatConfiguration {
                economy_rows: 50,
                economy_seats_per_row: 10,
                business_rows: 12,
                business_seats_per_row: 6,
                first_class_rows: 6,
                first_class_seats_per_row: 4,
            },
            _ => SeatConfiguration {
                economy_rows: 20,
                economy_seats_per_row: 6,
                business_rows: 3,
                business_seats_per_row: 4,
                first_class_rows: 2,
                first_class_seats_per_row: 4,
            },
        };

        let total_capacity = Self::calculate_total_capacity(&seat_config);

        let performance = match model.as_str() {
            "Boeing 737-800" => PerformanceSpecs {
                max_speed_kmh: 876,
                cruise_speed_kmh: 828,
                max_altitude_m: 12500,
                range_km: 5665,
                fuel_efficiency_l_per_100km: 3.2,
            },
            "Airbus A320" => PerformanceSpecs {
                max_speed_kmh: 871,
                cruise_speed_kmh: 828,
                max_altitude_m: 12000,
                range_km: 6150,
                fuel_efficiency_l_per_100km: 2.9,
            },
            "Boeing 777-300" => PerformanceSpecs {
                max_speed_kmh: 905,
                cruise_speed_kmh: 892,
                max_altitude_m: 13100,
                range_km: 11135,
                fuel_efficiency_l_per_100km: 4.8,
            },
            "Airbus A380" => PerformanceSpecs {
                max_speed_kmh: 945,
                cruise_speed_kmh: 903,
                max_altitude_m: 13100,
                range_km: 15200,
                fuel_efficiency_l_per_100km: 6.2,
            },
            _ => PerformanceSpecs {
                max_speed_kmh: 800,
                cruise_speed_kmh: 750,
                max_altitude_m: 11000,
                range_km: 4000,
                fuel_efficiency_l_per_100km: 3.5,
            },
        };

        Self {
            id: Uuid::new_v4(),
            registration,
            model,
            manufacturer,
            year_manufactured,
            status: AircraftStatus::Active,
            seat_configuration: seat_config,
            total_capacity,
            baggage_capacity_kg: total_capacity * 25, // Approximate 25kg per passenger
            max_cargo_weight_kg: total_capacity * 35,  // Approximate cargo capacity
            performance,
            maintenance_hours: 0.0,
            flight_hours: 0.0,
        }
    }

    fn calculate_total_capacity(config: &SeatConfiguration) -> u32 {
        (config.economy_rows * config.economy_seats_per_row) +
        (config.business_rows * config.business_seats_per_row) +
        (config.first_class_rows * config.first_class_seats_per_row)
    }

    pub fn get_seats_by_class(&self, class: &SeatClass) -> u32 {
        match class {
            SeatClass::Economy => {
                self.seat_configuration.economy_rows * self.seat_configuration.economy_seats_per_row
            }
            SeatClass::Business => {
                self.seat_configuration.business_rows * self.seat_configuration.business_seats_per_row
            }
            SeatClass::FirstClass => {
                self.seat_configuration.first_class_rows * self.seat_configuration.first_class_seats_per_row
            }
        }
    }

    pub fn is_available_for_flight(&self) -> bool {
        matches!(self.status, AircraftStatus::Active)
    }

    pub fn set_status(&mut self, status: AircraftStatus) {
        self.status = status;
    }

    pub fn add_flight_hours(&mut self, hours: f64) {
        self.flight_hours += hours;
        // Every 100 flight hours requires 10 hours of maintenance
        if self.flight_hours - self.maintenance_hours >= 100.0 {
            self.status = AircraftStatus::Maintenance;
        }
    }

    pub fn perform_maintenance(&mut self, hours: f64) {
        self.maintenance_hours += hours;
        if self.maintenance_hours >= self.flight_hours {
            self.status = AircraftStatus::Active;
        }
    }

    pub fn get_age(&self) -> u32 {
        2025 - self.year_manufactured
    }

    pub fn get_status_display(&self) -> String {
        match self.status {
            AircraftStatus::Active => "Active ✅".to_string(),
            AircraftStatus::Maintenance => "Maintenance 🔧".to_string(),
            AircraftStatus::Retired => "Retired 🚫".to_string(),
            AircraftStatus::InFlight => "In Flight ✈️".to_string(),
        }
    }

    pub fn get_baggage_allowance(&self) -> HashMap<SeatClass, u32> {
        let mut allowance = HashMap::new();
        
        // Standard baggage allowance by class (in kg)
        allowance.insert(SeatClass::Economy, 23);
        allowance.insert(SeatClass::Business, 32);
        allowance.insert(SeatClass::FirstClass, 46);
        
        allowance
    }

    pub fn get_detailed_specs(&self) -> String {
        format!(
            "Model: {} | Capacity: {} seats | Range: {} km | Fuel Efficiency: {:.1}L/100km",
            self.model,
            self.total_capacity,
            self.performance.range_km,
            self.performance.fuel_efficiency_l_per_100km
        )
    }
}

impl std::fmt::Display for Aircraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {} | {} seats | {}",
            self.registration,
            self.model,
            self.get_age(),
            self.total_capacity,
            self.get_status_display()
        )
    }
}