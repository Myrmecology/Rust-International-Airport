use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AirportSize {
    Small,      // < 1M passengers/year
    Medium,     // 1M - 10M passengers/year
    Large,      // 10M - 40M passengers/year
    Hub,        // > 40M passengers/year
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminal {
    pub id: String,
    pub name: String,
    pub gates: Vec<String>,
    pub amenities: Vec<String>,
    pub is_international: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runway {
    pub id: String,
    pub length_meters: u32,
    pub width_meters: u32,
    pub surface_type: String, // e.g., "Asphalt", "Concrete"
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Airport {
    pub id: Uuid,
    pub code: String,           // IATA code (e.g., "LAX")
    pub icao_code: String,      // ICAO code (e.g., "KLAX")
    pub name: String,           // Full name
    pub city: String,
    pub country: String,
    pub timezone: String,       // e.g., "America/Los_Angeles"
    pub coordinates: Coordinates,
    pub elevation_meters: i32,
    pub airport_size: AirportSize,
    pub terminals: Vec<Terminal>,
    pub runways: Vec<Runway>,
    pub annual_passengers: u64,
    pub cargo_capacity_tonnes: u32,
    pub operating_hours: (u8, u8), // (start_hour, end_hour) in 24h format
    pub services: Vec<String>,     // Available services
    pub is_international: bool,
    pub customs_available: bool,
}

impl Airport {
    pub fn new(
        code: String,
        icao_code: String,
        name: String,
        city: String,
        country: String,
        timezone: String,
        latitude: f64,
        longitude: f64,
        elevation_meters: i32,
    ) -> Self {
        // Determine airport size based on typical patterns
        let airport_size = Self::determine_size(&code);
        
        // Generate default terminals and runways based on size
        let (terminals, runways) = Self::generate_infrastructure(&airport_size, &code);
        
        // Estimate annual passengers based on size
        let annual_passengers = match airport_size {
            AirportSize::Small => 500_000,
            AirportSize::Medium => 5_000_000,
            AirportSize::Large => 25_000_000,
            AirportSize::Hub => 80_000_000,
        };

        // Standard services available at most airports
        let services = vec![
            "Car Rental".to_string(),
            "Taxi Service".to_string(),
            "Parking".to_string(),
            "Dining".to_string(),
            "Shopping".to_string(),
            "WiFi".to_string(),
            "ATM".to_string(),
            "Lost & Found".to_string(),
        ];

        Self {
            id: Uuid::new_v4(),
            code,
            icao_code,
            name,
            city,
            country,
            timezone,
            coordinates: Coordinates { latitude, longitude },
            elevation_meters,
            airport_size,
            terminals,
            runways,
            annual_passengers,
            cargo_capacity_tonnes: 100_000, // Default cargo capacity
            operating_hours: (5, 23), // Most airports operate 5 AM to 11 PM
            services,
            is_international: true, // Most airports in our system are international
            customs_available: true,
        }
    }

    fn determine_size(code: &str) -> AirportSize {
        // Classify based on well-known airport codes
        match code {
            "LAX" | "JFK" | "LHR" | "CDG" | "DXB" | "ATL" | "ORD" | "DFW" => AirportSize::Hub,
            "SFO" | "MIA" | "BOS" | "SEA" | "DEN" | "LAS" | "PHX" | "IAH" => AirportSize::Large,
            "AUS" | "SAN" | "MSP" | "DTW" | "PHL" | "CLT" | "BWI" | "MDW" => AirportSize::Medium,
            _ => AirportSize::Small,
        }
    }

    fn generate_infrastructure(size: &AirportSize, code: &str) -> (Vec<Terminal>, Vec<Runway>) {
        let terminals = match size {
            AirportSize::Hub => vec![
                Terminal {
                    id: "T1".to_string(),
                    name: "Terminal 1 - International".to_string(),
                    gates: (1..=30).map(|i| format!("A{}", i)).collect(),
                    amenities: vec!["Duty Free".to_string(), "Lounges".to_string(), "Restaurants".to_string()],
                    is_international: true,
                },
                Terminal {
                    id: "T2".to_string(),
                    name: "Terminal 2 - Domestic".to_string(),
                    gates: (1..=25).map(|i| format!("B{}", i)).collect(),
                    amenities: vec!["Fast Food".to_string(), "Shops".to_string(), "Business Center".to_string()],
                    is_international: false,
                },
                Terminal {
                    id: "T3".to_string(),
                    name: "Terminal 3 - Mixed".to_string(),
                    gates: (1..=20).map(|i| format!("C{}", i)).collect(),
                    amenities: vec!["Restaurants".to_string(), "Shopping".to_string()],
                    is_international: true,
                },
            ],
            AirportSize::Large => vec![
                Terminal {
                    id: "T1".to_string(),
                    name: "Terminal 1".to_string(),
                    gates: (1..=20).map(|i| format!("A{}", i)).collect(),
                    amenities: vec!["Restaurants".to_string(), "Shops".to_string(), "Lounges".to_string()],
                    is_international: true,
                },
                Terminal {
                    id: "T2".to_string(),
                    name: "Terminal 2".to_string(),
                    gates: (1..=15).map(|i| format!("B{}", i)).collect(),
                    amenities: vec!["Fast Food".to_string(), "Shopping".to_string()],
                    is_international: false,
                },
            ],
            AirportSize::Medium => vec![
                Terminal {
                    id: "T1".to_string(),
                    name: "Main Terminal".to_string(),
                    gates: (1..=12).map(|i| format!("A{}", i)).collect(),
                    amenities: vec!["Restaurants".to_string(), "Shops".to_string()],
                    is_international: true,
                },
            ],
            AirportSize::Small => vec![
                Terminal {
                    id: "T1".to_string(),
                    name: "Terminal".to_string(),
                    gates: (1..=6).map(|i| format!("A{}", i)).collect(),
                    amenities: vec!["Café".to_string(), "Gift Shop".to_string()],
                    is_international: false,
                },
            ],
        };

        let runways = match size {
            AirportSize::Hub => vec![
                Runway {
                    id: "07L/25R".to_string(),
                    length_meters: 4000,
                    width_meters: 60,
                    surface_type: "Concrete".to_string(),
                    is_active: true,
                },
                Runway {
                    id: "07R/25L".to_string(),
                    length_meters: 3800,
                    width_meters: 60,
                    surface_type: "Concrete".to_string(),
                    is_active: true,
                },
                Runway {
                    id: "06L/24R".to_string(),
                    length_meters: 3500,
                    width_meters: 45,
                    surface_type: "Asphalt".to_string(),
                    is_active: true,
                },
            ],
            AirportSize::Large => vec![
                Runway {
                    id: "09/27".to_string(),
                    length_meters: 3500,
                    width_meters: 45,
                    surface_type: "Concrete".to_string(),
                    is_active: true,
                },
                Runway {
                    id: "04/22".to_string(),
                    length_meters: 3200,
                    width_meters: 45,
                    surface_type: "Asphalt".to_string(),
                    is_active: true,
                },
            ],
            AirportSize::Medium => vec![
                Runway {
                    id: "12/30".to_string(),
                    length_meters: 2800,
                    width_meters: 45,
                    surface_type: "Asphalt".to_string(),
                    is_active: true,
                },
            ],
            AirportSize::Small => vec![
                Runway {
                    id: "18/36".to_string(),
                    length_meters: 2000,
                    width_meters: 30,
                    surface_type: "Asphalt".to_string(),
                    is_active: true,
                },
            ],
        };

        (terminals, runways)
    }

    pub fn get_all_gates(&self) -> Vec<String> {
        self.terminals
            .iter()
            .flat_map(|terminal| &terminal.gates)
            .cloned()
            .collect()
    }

    pub fn find_available_gate(&self) -> Option<String> {
        // Simple gate assignment - return first gate
        // In real system, this would check current usage
        self.terminals
            .first()
            .and_then(|terminal| terminal.gates.first())
            .cloned()
    }

    pub fn is_operating(&self, hour: u8) -> bool {
        hour >= self.operating_hours.0 && hour <= self.operating_hours.1
    }

    pub fn can_handle_aircraft(&self, aircraft_length: u32) -> bool {
        self.runways
            .iter()
            .any(|runway| runway.is_active && runway.length_meters >= aircraft_length)
    }

    pub fn get_distance_to(&self, other: &Airport) -> f64 {
        // Haversine formula for distance calculation
        let lat1 = self.coordinates.latitude.to_radians();
        let lat2 = other.coordinates.latitude.to_radians();
        let delta_lat = (other.coordinates.latitude - self.coordinates.latitude).to_radians();
        let delta_lon = (other.coordinates.longitude - self.coordinates.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        6371.0 * c // Earth's radius in kilometers
    }

    pub fn get_terminal_info(&self) -> String {
        let terminal_count = self.terminals.len();
        let gate_count: usize = self.terminals.iter().map(|t| t.gates.len()).sum();
        
        format!(
            "{} terminals, {} gates",
            terminal_count,
            gate_count
        )
    }

    pub fn get_size_display(&self) -> String {
        match self.airport_size {
            AirportSize::Small => "Regional Airport 🏛️".to_string(),
            AirportSize::Medium => "City Airport 🏢".to_string(),
            AirportSize::Large => "Major Airport 🏗️".to_string(),
            AirportSize::Hub => "International Hub ✈️".to_string(),
        }
    }
}

impl std::fmt::Display for Airport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) | {} | {} | {}",
            self.name,
            self.code,
            self.city,
            self.country,
            self.get_size_display()
        )
    }
}