use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::modules::flight::{Flight, FlightStatus};
use crate::modules::aircraft::{Aircraft, AircraftStatus};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminLevel {
    SuperAdmin,    // Full system access
    FlightManager, // Flight operations only
    AircraftManager, // Aircraft management only
    FinanceManager, // Pricing and revenue only
    Viewer,        // Read-only access
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUser {
    pub id: Uuid,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub level: AdminLevel,
    pub created_date: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAction {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub action_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub affected_entity_id: Option<Uuid>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_flights: u32,
    pub active_flights: u32,
    pub delayed_flights: u32,
    pub cancelled_flights: u32,
    pub total_aircraft: u32,
    pub active_aircraft: u32,
    pub aircraft_in_maintenance: u32,
    pub total_bookings: u32,
    pub revenue_today: f64,
    pub revenue_month: f64,
    pub average_load_factor: f64, // Percentage of seats filled
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: Uuid,
    pub rule_name: String,
    pub route_pattern: Option<String>, // e.g., "LAX-*", "*-JFK", "LAX-JFK"
    pub time_period: Option<(u8, u8)>, // Hour range (start, end)
    pub multiplier: f64,
    pub is_active: bool,
    pub created_by: Uuid,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct AdminPanel {
    pub current_admin: Option<AdminUser>,
    pub audit_log: Vec<AdminAction>,
    pub pricing_rules: Vec<PricingRule>,
    pub system_metrics: SystemMetrics,
}

impl AdminUser {
    pub fn new(
        username: String,
        full_name: String,
        email: String,
        level: AdminLevel,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            full_name,
            email,
            level,
            created_date: Utc::now(),
            last_login: None,
            is_active: true,
        }
    }

    pub fn can_manage_flights(&self) -> bool {
        matches!(
            self.level,
            AdminLevel::SuperAdmin | AdminLevel::FlightManager
        )
    }

    pub fn can_manage_aircraft(&self) -> bool {
        matches!(
            self.level,
            AdminLevel::SuperAdmin | AdminLevel::AircraftManager
        )
    }

    pub fn can_manage_pricing(&self) -> bool {
        matches!(
            self.level,
            AdminLevel::SuperAdmin | AdminLevel::FinanceManager
        )
    }

    pub fn can_view_reports(&self) -> bool {
        // All admin levels can view reports
        true
    }

    pub fn login(&mut self) {
        self.last_login = Some(Utc::now());
    }

    pub fn get_level_display(&self) -> String {
        match self.level {
            AdminLevel::SuperAdmin => "Super Admin 👑".to_string(),
            AdminLevel::FlightManager => "Flight Manager ✈️".to_string(),
            AdminLevel::AircraftManager => "Aircraft Manager 🔧".to_string(),
            AdminLevel::FinanceManager => "Finance Manager 💰".to_string(),
            AdminLevel::Viewer => "Viewer 👁️".to_string(),
        }
    }
}

impl AdminAction {
    pub fn new(
        admin_id: Uuid,
        action_type: String,
        description: String,
        affected_entity_id: Option<Uuid>,
        old_value: Option<String>,
        new_value: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            admin_id,
            action_type,
            description,
            timestamp: Utc::now(),
            affected_entity_id,
            old_value,
            new_value,
        }
    }

    pub fn format_for_log(&self) -> String {
        let change_info = match (&self.old_value, &self.new_value) {
            (Some(old), Some(new)) => format!(" (Changed from '{}' to '{}')", old, new),
            (None, Some(new)) => format!(" (Set to '{}')", new),
            (Some(old), None) => format!(" (Removed '{}')", old),
            (None, None) => String::new(),
        };

        format!(
            "[{}] {} - {}{}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.action_type,
            self.description,
            change_info
        )
    }
}

impl PricingRule {
    pub fn new(
        rule_name: String,
        route_pattern: Option<String>,
        time_period: Option<(u8, u8)>,
        multiplier: f64,
        created_by: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_name,
            route_pattern,
            time_period,
            multiplier,
            is_active: true,
            created_by,
            created_date: Utc::now(),
        }
    }

    pub fn applies_to_route(&self, origin: &str, destination: &str) -> bool {
        match &self.route_pattern {
            Some(pattern) => {
                let route = format!("{}-{}", origin, destination);
                if pattern.contains('*') {
                    // Wildcard matching
                    if pattern.starts_with('*') && pattern.ends_with('*') {
                        // *-pattern-* (contains)
                        let middle = pattern.trim_start_matches('*').trim_end_matches('*');
                        route.contains(middle)
                    } else if pattern.starts_with('*') {
                        // *-destination
                        route.ends_with(&pattern[1..])
                    } else if pattern.ends_with('*') {
                        // origin-*
                        route.starts_with(&pattern[..pattern.len()-1])
                    } else {
                        false
                    }
                } else {
                    // Exact match
                    route == *pattern
                }
            }
            None => true, // Apply to all routes if no pattern specified
        }
    }

    pub fn applies_to_time(&self, hour: u8) -> bool {
        match self.time_period {
            Some((start, end)) => hour >= start && hour <= end,
            None => true, // Apply to all times if no period specified
        }
    }
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            total_flights: 0,
            active_flights: 0,
            delayed_flights: 0,
            cancelled_flights: 0,
            total_aircraft: 0,
            active_aircraft: 0,
            aircraft_in_maintenance: 0,
            total_bookings: 0,
            revenue_today: 0.0,
            revenue_month: 0.0,
            average_load_factor: 0.0,
            last_updated: Utc::now(),
        }
    }

    pub fn update_flight_metrics(&mut self, flights: &[Flight]) {
        self.total_flights = flights.len() as u32;
        self.active_flights = flights
            .iter()
            .filter(|f| matches!(f.status, FlightStatus::OnTime | FlightStatus::Delayed(_)))
            .count() as u32;
        self.delayed_flights = flights
            .iter()
            .filter(|f| matches!(f.status, FlightStatus::Delayed(_)))
            .count() as u32;
        self.cancelled_flights = flights
            .iter()
            .filter(|f| matches!(f.status, FlightStatus::Cancelled))
            .count() as u32;
        
        self.last_updated = Utc::now();
    }

    pub fn update_aircraft_metrics(&mut self, aircraft: &[Aircraft]) {
        self.total_aircraft = aircraft.len() as u32;
        self.active_aircraft = aircraft
            .iter()
            .filter(|a| matches!(a.status, AircraftStatus::Active | AircraftStatus::InFlight))
            .count() as u32;
        self.aircraft_in_maintenance = aircraft
            .iter()
            .filter(|a| matches!(a.status, AircraftStatus::Maintenance))
            .count() as u32;
        
        self.last_updated = Utc::now();
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Flights: {} active, {} delayed | Aircraft: {} active, {} maintenance | Revenue: ${:.2} today",
            self.active_flights,
            self.delayed_flights,
            self.active_aircraft,
            self.aircraft_in_maintenance,
            self.revenue_today
        )
    }
}

impl AdminPanel {
    pub fn new() -> Self {
        Self {
            current_admin: None,
            audit_log: Vec::new(),
            pricing_rules: Vec::new(),
            system_metrics: SystemMetrics::new(),
        }
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<AdminUser, String> {
        // In a real system, this would check against a database
        // For demo purposes, we'll create default admin users
        let default_admin = match username {
            "admin" if password == "admin123" => AdminUser::new(
                "admin".to_string(),
                "System Administrator".to_string(),
                "admin@rust-airport.com".to_string(),
                AdminLevel::SuperAdmin,
            ),
            "flight_mgr" if password == "flight123" => AdminUser::new(
                "flight_mgr".to_string(),
                "Flight Manager".to_string(),
                "flights@rust-airport.com".to_string(),
                AdminLevel::FlightManager,
            ),
            "aircraft_mgr" if password == "aircraft123" => AdminUser::new(
                "aircraft_mgr".to_string(),
                "Aircraft Manager".to_string(),
                "aircraft@rust-airport.com".to_string(),
                AdminLevel::AircraftManager,
            ),
            _ => return Err("Invalid username or password".to_string()),
        };

        let mut admin = default_admin;
        admin.login();
        self.current_admin = Some(admin.clone());
        
        self.log_action(
            admin.id,
            "LOGIN".to_string(),
            format!("User {} logged into admin panel", username),
            None,
            None,
            None,
        );

        Ok(admin)
    }

    pub fn logout(&mut self) {
        if let Some(admin) = &self.current_admin {
            self.log_action(
                admin.id,
                "LOGOUT".to_string(),
                format!("User {} logged out of admin panel", admin.username),
                None,
                None,
                None,
            );
        }
        self.current_admin = None;
    }

    pub fn log_action(
        &mut self,
        admin_id: Uuid,
        action_type: String,
        description: String,
        affected_entity_id: Option<Uuid>,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        let action = AdminAction::new(
            admin_id,
            action_type,
            description,
            affected_entity_id,
            old_value,
            new_value,
        );
        self.audit_log.push(action);
    }

    pub fn add_pricing_rule(&mut self, rule: PricingRule) -> Result<(), String> {
        if let Some(admin) = &self.current_admin {
            if !admin.can_manage_pricing() {
                return Err("Insufficient permissions to manage pricing".to_string());
            }
            
            self.log_action(
                admin.id,
                "ADD_PRICING_RULE".to_string(),
                format!("Added pricing rule: {}", rule.rule_name),
                Some(rule.id),
                None,
                Some(format!("Multiplier: {}", rule.multiplier)),
            );
            
            self.pricing_rules.push(rule);
            Ok(())
        } else {
            Err("No admin user logged in".to_string())
        }
    }

    pub fn get_applicable_multiplier(&self, origin: &str, destination: &str, hour: u8) -> f64 {
        self.pricing_rules
            .iter()
            .filter(|rule| rule.is_active)
            .filter(|rule| rule.applies_to_route(origin, destination))
            .filter(|rule| rule.applies_to_time(hour))
            .map(|rule| rule.multiplier)
            .fold(1.0, |acc, multiplier| acc * multiplier)
    }

    pub fn get_recent_actions(&self, limit: usize) -> Vec<&AdminAction> {
        self.audit_log
            .iter()
            .rev()
            .take(limit)
            .collect()
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_admin.is_some()
    }

    pub fn current_admin_name(&self) -> String {
        self.current_admin
            .as_ref()
            .map(|admin| admin.full_name.clone())
            .unwrap_or_else(|| "Not logged in".to_string())
    }
}