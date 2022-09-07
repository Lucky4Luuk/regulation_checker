use anyhow::Result;

use serde::Deserialize;

fn parse_float(s: &str) -> Result<f32> {
    Ok(s.trim().parse::<f32>()?)
}

fn parse_int(s: &str) -> Result<usize> {
    Ok(parse_float(s)? as usize)
}

#[derive(Debug)]
pub struct Car {
// Basic information
    pub car_name: String,
    pub model_name: String,
    pub model_year: usize,
    pub wheelbase: usize,

// Stats
    pub drivability_rating: f32,
    pub sportiness_rating: f32,
    pub reliability_rating: f32,
    pub safety_rating: f32,
    pub practicality_rating: f32,
    pub comfort_rating: f32,

// Engine
    pub engine_year: usize,
    pub cylinder_count: usize,
    pub aspiration: String,

// Part information
    pub chassis_type: String,
    pub chassis_material: String,
    pub panel_material: String,

    pub front_suspension: String,
    pub rear_suspension: String,

    pub engine_placement: String,
    pub engine_orientation: String,

    pub model_body_quality: f32,
    pub model_chassis_quality: f32,
    pub model_body_techpool: f32,
}

impl Car {
    pub fn from_directory(path: &str) -> Result<Self> {
        let raw = RawCar::from_directory(path)?;
        Ok(Self {
            car_name: raw.car_name,
            model_name: raw.model_name,
            model_year: parse_int(&raw.model_year)?,
            wheelbase: parse_int(&raw.wheelbase)?,

            drivability_rating: parse_float(&raw.drivability_rating)?,
            sportiness_rating: parse_float(&raw.sportiness_rating)?,
            reliability_rating: parse_float(&raw.trim_reliability)?,
            safety_rating: parse_float(&raw.safety_rating)?,
            practicality_rating: parse_float(&raw.practicality_rating)?,
            comfort_rating: parse_float(&raw.comfort_rating)?,

            engine_year: parse_int(&raw.variant_year)?,
            cylinder_count: parse_int(&raw.cylinder_count)?,
            aspiration: raw.aspiration,

            chassis_type: raw.chassis_type,
            chassis_material: raw.chassis_material,
            panel_material: raw.panel_material,

            front_suspension: raw.front_suspension,
            rear_suspension: raw.rear_suspension,

            engine_placement: raw.engine_placement,
            engine_orientation: raw.engine_orientation,

            model_body_quality: parse_float(&raw.model_body_quality)?,
            model_chassis_quality: parse_float(&raw.model_chassis_quality)?,
            model_body_techpool: parse_float(&raw.model_body_techpool)?,
        })
    }

    pub fn has_turbo(&self) -> bool {
        &self.aspiration.trim() != &"Naturally Aspirated"
    }
}

#[derive(Debug)]
pub enum ImportError {
    Unknown,
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", format!("{:?}", self))
    }
}

impl std::error::Error for ImportError {}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct RawCar {
    #[serde(rename(deserialize = "Exporter Version"))]
    pub exporter_version: String,
    #[serde(rename(deserialize = "Game Version"))]
    pub game_version: String,
    #[serde(rename(deserialize = "Car Name"))]
    pub car_name: String,
    #[serde(rename(deserialize = "Model Name"))]
    pub model_name: String,
    #[serde(rename(deserialize = "Model Year"))]
    pub model_year: String,
    #[serde(rename(deserialize = "Wheelbase"))]
    pub wheelbase: String,
    #[serde(rename(deserialize = "Chassis Type"))]
    pub chassis_type: String,
    #[serde(rename(deserialize = "Chassis Material"))]
    pub chassis_material: String,
    #[serde(rename(deserialize = "Panel Material"))]
    pub panel_material: String,
    #[serde(rename(deserialize = "Front Suspension"))]
    pub front_suspension: String,
    #[serde(rename(deserialize = "Rear Suspension"))]
    pub rear_suspension: String,
    #[serde(rename(deserialize = "Engine Placement"))]
    pub engine_placement: String,
    #[serde(rename(deserialize = "Engine Orientation"))]
    pub engine_orientation: String,
    #[serde(rename(deserialize = "Model Body Quality"))]
    pub model_body_quality: String,
    #[serde(rename(deserialize = "Model Chassis Quality"))]
    pub model_chassis_quality: String,
    #[serde(rename(deserialize = "Model Body Techpool"))]
    pub model_body_techpool: String,
    #[serde(rename(deserialize = "Model Chassis Techpool"))]
    pub model_chassis_techpool: String,
    #[serde(rename(deserialize = "Trim Name"))]
    pub trim_name: String,
    #[serde(rename(deserialize = "Trim Year"))]
    pub trim_year: String,
    #[serde(rename(deserialize = "Trim Body Quality"))]
    pub trim_body_quality: String,
    #[serde(rename(deserialize = "Trim Aerodynamics Quality"))]
    pub trim_aerodynamics_quality: String,
    #[serde(rename(deserialize = "Trim Interior Quality"))]
    pub trim_interior_quality: String,
    #[serde(rename(deserialize = "Trim Fixture Quality"))]
    pub trim_fixture_quality: String,
    #[serde(rename(deserialize = "Trim Tyre Quality"))]
    pub trim_tyre_quality: String,
    #[serde(rename(deserialize = "Trim Suspension Quality"))]
    pub trim_suspension_quality: String,
    #[serde(rename(deserialize = "Trim Safety Quality"))]
    pub trim_safety_quality: String,
    #[serde(rename(deserialize = "Trim Brake Quality"))]
    pub trim_brake_quality: String,
    #[serde(rename(deserialize = "Trim Assist Quality"))]
    pub trim_assist_quality: String,
    #[serde(rename(deserialize = "Trim Drivetrain Quality"))]
    pub trim_drivetrain_quality: String,
    #[serde(rename(deserialize = "Trim Chassis Techpool"))]
    pub trim_chassis_techpool: String,
    #[serde(rename(deserialize = "Trim Body Techpool"))]
    pub trim_body_techpool: String,
    #[serde(rename(deserialize = "Trim Aerodynamics Techpool"))]
    pub trim_aerodynamics_techpool: String,
    #[serde(rename(deserialize = "Trim Interior Techpool"))]
    pub trim_interior_techpool: String,
    #[serde(rename(deserialize = "Trim Fixture Techpool"))]
    pub trim_fixture_techpool: String,
    #[serde(rename(deserialize = "Trim Tyre Techpool"))]
    pub trim_tyre_techpool: String,
    #[serde(rename(deserialize = "Trim Suspension Techpool"))]
    pub trim_suspension_techpool: String,
    #[serde(rename(deserialize = "Trim Safety Techpool"))]
    pub trim_safety_techpool: String,
    #[serde(rename(deserialize = "Trim Brake Techpool"))]
    pub trim_brake_techpool: String,
    #[serde(rename(deserialize = "Trim Assist Techpool"))]
    pub trim_assist_techpool: String,
    #[serde(rename(deserialize = "Trim Drivetrain Techpool"))]
    pub trim_drivetrain_techpool: String,
    #[serde(rename(deserialize = "Body Type"))]
    pub body_type: String,
    #[serde(rename(deserialize = "Doors"))]
    pub doors: String,
    #[serde(rename(deserialize = "Body Name"))]
    pub body_name: String,
    #[serde(rename(deserialize = "Convertible Type"))]
    pub convertible_type: String,
    #[serde(rename(deserialize = "Full 1st Row Seats"))]
    pub full_1st_row_seats: String,
    #[serde(rename(deserialize = "Small 1st Row Seats"))]
    pub small_1st_row_seats: String,
    #[serde(rename(deserialize = "Full 2nd Row Seats"))]
    pub full_2nd_row_seats: String,
    #[serde(rename(deserialize = "Small 2nd Row Seats"))]
    pub small_2nd_row_seats: String,
    #[serde(rename(deserialize = "Full 3rd Row Seats"))]
    pub full_3rd_row_seats: String,
    #[serde(rename(deserialize = "Small 3rd Row Seats"))]
    pub small_3rd_row_seats: String,
    #[serde(rename(deserialize = "Drive Type"))]
    pub drive_type: String,
    #[serde(rename(deserialize = "Gearbox Type"))]
    pub gearbox_type: String,
    #[serde(rename(deserialize = "Gear Count"))]
    pub gear_count: String,
    #[serde(rename(deserialize = "Gear Spacing"))]
    pub gear_spacing: String,
    #[serde(rename(deserialize = "Power To Front"))]
    pub power_to_front: String,
    #[serde(rename(deserialize = "Differential Type"))]
    pub differential_type: String,
    #[serde(rename(deserialize = "Final Drive"))]
    pub final_drive: String,
    #[serde(rename(deserialize = "Speed Limiter"))]
    pub speed_limiter: String,
    #[serde(rename(deserialize = "Active Cooling"))]
    pub active_cooling: String,
    #[serde(rename(deserialize = "Cooling Amount"))]
    pub cooling_amount: String,
    #[serde(rename(deserialize = "Undertray"))]
    pub undertray: String,
    #[serde(rename(deserialize = "Active Aero"))]
    pub active_aero: String,
    #[serde(rename(deserialize = "Rear Wing Angle"))]
    pub rear_wing_angle: String,
    #[serde(rename(deserialize = "Front Wing Angle"))]
    pub front_wing_angle: String,
    #[serde(rename(deserialize = "Interior"))]
    pub interior: String,
    #[serde(rename(deserialize = "Entertainment"))]
    pub entertainment: String,
    #[serde(rename(deserialize = "Power Steering"))]
    pub power_steering: String,
    #[serde(rename(deserialize = "Assists"))]
    pub assists: String,
    #[serde(rename(deserialize = "Safety"))]
    pub safety: String,
    #[serde(rename(deserialize = "Springs"))]
    pub springs: String,
    #[serde(rename(deserialize = "Front Spring Stiffness"))]
    pub front_spring_stiffness: String,
    #[serde(rename(deserialize = "Rear Spring Stiffness"))]
    pub rear_spring_stiffness: String,
    #[serde(rename(deserialize = "Dampers"))]
    pub dampers: String,
    #[serde(rename(deserialize = "Front Damper Stiffness"))]
    pub front_damper_stiffness: String,
    #[serde(rename(deserialize = "Rear Damper Stiffness"))]
    pub rear_damper_stiffness: String,
    #[serde(rename(deserialize = "Sway Bars"))]
    pub sway_bars: String,
    #[serde(rename(deserialize = "Front Sway Bar Stiffness"))]
    pub front_sway_bar_stiffness: String,
    #[serde(rename(deserialize = "Rear Sway Bar Stiffness"))]
    pub rear_sway_bar_stiffness: String,
    #[serde(rename(deserialize = "Ride Height"))]
    pub ride_height: String,
    #[serde(rename(deserialize = "Front Camber"))]
    pub front_camber: String,
    #[serde(rename(deserialize = "Rear Camber"))]
    pub rear_camber: String,
    #[serde(rename(deserialize = "Rim Material"))]
    pub rim_material: String,
    #[serde(rename(deserialize = "Tyre Type"))]
    pub tyre_type: String,
    #[serde(rename(deserialize = "Tyre Compound"))]
    pub tyre_compound: String,
    #[serde(rename(deserialize = "Front Rim Size"))]
    pub front_rim_size: String,
    #[serde(rename(deserialize = "Rear Rim Size"))]
    pub rear_rim_size: String,
    #[serde(rename(deserialize = "Front Wheel Offset"))]
    pub front_wheel_offset: String,
    #[serde(rename(deserialize = "Rear Wheel Offset"))]
    pub rear_wheel_offset: String,
    #[serde(rename(deserialize = "Front Tyre Profile"))]
    pub front_tyre_profile: String,
    #[serde(rename(deserialize = "Rear Tyre Profile"))]
    pub rear_tyre_profile: String,
    #[serde(rename(deserialize = "Front Tyre Width"))]
    pub front_tyre_width: String,
    #[serde(rename(deserialize = "Rear Tyre Width"))]
    pub rear_tyre_width: String,
    #[serde(rename(deserialize = "Front Wheel Diameter"))]
    pub front_wheel_diameter: String,
    #[serde(rename(deserialize = "Rear Wheel Diameter"))]
    pub rear_wheel_diameter: String,
    #[serde(rename(deserialize = "Front Wheel Diameter 2"))]
    pub front_wheel_diameter_2: String,
    #[serde(rename(deserialize = "Rear Wheel Diameter 2"))]
    pub rear_wheel_diameter_2: String,
    #[serde(rename(deserialize = "Front Brake Type"))]
    pub front_brake_type: String,
    #[serde(rename(deserialize = "Rear Brake Type"))]
    pub rear_brake_type: String,
    #[serde(rename(deserialize = "Front Caliper Pistons"))]
    pub front_caliper_pistons: String,
    #[serde(rename(deserialize = "Rear Caliper Pistons"))]
    pub rear_caliper_pistons: String,
    #[serde(rename(deserialize = "Front Brake Diameter"))]
    pub front_brake_diameter: String,
    #[serde(rename(deserialize = "Rear Brake Diameter"))]
    pub rear_brake_diameter: String,
    #[serde(rename(deserialize = "Front Brake Pad Type"))]
    pub front_brake_pad_type: String,
    #[serde(rename(deserialize = "Rear Brake Pad Type"))]
    pub rear_brake_pad_type: String,
    #[serde(rename(deserialize = "Front Brake Force"))]
    pub front_brake_force: String,
    #[serde(rename(deserialize = "Rear Brake Force"))]
    pub rear_brake_force: String,
    #[serde(rename(deserialize = "Weight Optimization Tune"))]
    pub weight_optimization_tune: String,
    #[serde(rename(deserialize = "Weight Distribution Tune"))]
    pub weight_distribution_tune: String,
    #[serde(rename(deserialize = "Trim Weight"))]
    pub trim_weight: String,
    #[serde(rename(deserialize = "Tow Weight"))]
    pub tow_weight: String,
    #[serde(rename(deserialize = "Body Stiffness"))]
    pub body_stiffness: String,
    #[serde(rename(deserialize = "Sportiness Rating"))]
    pub sportiness_rating: String,
    #[serde(rename(deserialize = "Drivability Rating"))]
    pub drivability_rating: String,
    #[serde(rename(deserialize = "Utility Rating"))]
    pub utility_rating: String,
    #[serde(rename(deserialize = "Environmental Resistance"))]
    pub environmental_resistance: String,
    #[serde(rename(deserialize = "Trim Emissions"))]
    pub trim_emissions: String,
    #[serde(rename(deserialize = "Offroad Rating"))]
    pub offroad_rating: String,
    #[serde(rename(deserialize = "Prestige Rating"))]
    pub prestige_rating: String,
    #[serde(rename(deserialize = "Trim Reliability"))]
    pub trim_reliability: String,
    #[serde(rename(deserialize = "Trim Economy"))]
    pub trim_economy: String,
    #[serde(rename(deserialize = "Practicality Rating"))]
    pub practicality_rating: String,
    #[serde(rename(deserialize = "Safety Rating"))]
    pub safety_rating: String,
    #[serde(rename(deserialize = "Comfort Rating"))]
    pub comfort_rating: String,
    #[serde(rename(deserialize = "Passenger Volume"))]
    pub passenger_volume: String,
    #[serde(rename(deserialize = "Cargo Volume"))]
    pub cargo_volume: String,
    #[serde(rename(deserialize = "Trim Engineering Time"))]
    pub trim_engineering_time: String,
    #[serde(rename(deserialize = "Trim Production Units"))]
    pub trim_production_units: String,
    #[serde(rename(deserialize = "Trim Price"))]
    pub trim_price: String,
    #[serde(rename(deserialize = "Trim Tooling Costs"))]
    pub trim_tooling_costs: String,
    #[serde(rename(deserialize = "Trim Service Costs"))]
    pub trim_service_costs: String,
    #[serde(rename(deserialize = "Trim Total Costs"))]
    pub trim_total_costs: String,
    #[serde(rename(deserialize = "Trim Cost"))]
    pub trim_cost: String,
    #[serde(rename(deserialize = "Trim Material Cost"))]
    pub trim_material_cost: String,
    #[serde(rename(deserialize = "Trim Engineering Costs"))]
    pub trim_engineering_costs: String,
    #[serde(rename(deserialize = "Braking Distance"))]
    pub braking_distance: String,
    #[serde(rename(deserialize = "Max Body Roll"))]
    pub max_body_roll: String,
    #[serde(rename(deserialize = "Cornering"))]
    pub cornering: String,
    #[serde(rename(deserialize = "Front Downforce"))]
    pub front_downforce: String,
    #[serde(rename(deserialize = "Rear Downforce"))]
    pub rear_downforce: String,
    #[serde(rename(deserialize = "Tyre Speed Index"))]
    pub tyre_speed_index: String,
    #[serde(rename(deserialize = "Tyre Speed Rating (km/h)"))]
    pub tyre_speed_rating_km_h: String,
    #[serde(rename(deserialize = "Top Speed (km/h)"))]
    pub top_speed_km_h: String,
    #[serde(rename(deserialize = "Top Speed Gear"))]
    pub top_speed_gear: String,
    #[serde(rename(deserialize = "Top Speed Engine Power"))]
    pub top_speed_engine_power: String,
    #[serde(rename(deserialize = "Top Speed Engine RPM"))]
    pub top_speed_engine_rpm: String,
    #[serde(rename(deserialize = "60 Time"))]
    pub time_60: String,
    #[serde(rename(deserialize = "80 Time"))]
    pub time_80: String,
    #[serde(rename(deserialize = "100 Time"))]
    pub time_100: String,
    #[serde(rename(deserialize = "200 Time"))]
    pub time_200: String,
    #[serde(rename(deserialize = "120 Time"))]
    pub time_120: String,
    #[serde(rename(deserialize = "80 To 120 Time"))]
    pub time_80_to_120: String,
    #[serde(rename(deserialize = "Kilometer Time"))]
    pub kilometer_time: String,
    #[serde(rename(deserialize = "Kilometer Speed (km/h)"))]
    pub kilometer_speed_km_h: String,
    #[serde(rename(deserialize = "1/4 Mile Time"))]
    pub quarter_mile_time: String,
    #[serde(rename(deserialize = "1/4 Mile Speed (km/h)"))]
    pub quarter_mile_speed_km_h: String,
    #[serde(rename(deserialize = "Engine Family Name"))]
    pub engine_family_name: String,
    #[serde(rename(deserialize = "Engine Family Year"))]
    pub engine_family_year: String,
    #[serde(rename(deserialize = "Family Quality"))]
    pub family_quality: String,
    #[serde(rename(deserialize = "Family Techpool"))]
    pub family_techpool: String,
    #[serde(rename(deserialize = "Family Top End Techpool"))]
    pub family_top_end_techpool: String,
    #[serde(rename(deserialize = "Family Bore"))]
    pub family_bore: String,
    #[serde(rename(deserialize = "Family Stroke"))]
    pub family_stroke: String,
    #[serde(rename(deserialize = "Family Displacement"))]
    pub family_displacement: String,
    #[serde(rename(deserialize = "Block Type"))]
    pub block_type: String,
    #[serde(rename(deserialize = "Cylinder Count"))]
    pub cylinder_count: String,
    #[serde(rename(deserialize = "Block Material"))]
    pub block_material: String,
    #[serde(rename(deserialize = "Head Material"))]
    pub head_material: String,
    #[serde(rename(deserialize = "Head Type"))]
    pub head_type: String,
    #[serde(rename(deserialize = "Intake Valves"))]
    pub intake_valves: String,
    #[serde(rename(deserialize = "Exhaust Valves"))]
    pub exhaust_valves: String,
    #[serde(rename(deserialize = "Variant Name"))]
    pub variant_name: String,
    #[serde(rename(deserialize = "Variant Year"))]
    pub variant_year: String,
    #[serde(rename(deserialize = "Variant Top End Quality"))]
    pub variant_top_end_quality: String,
    #[serde(rename(deserialize = "Variant Bottom End Quality"))]
    pub variant_bottom_end_quality: String,
    #[serde(rename(deserialize = "Variant Aspiration Quality"))]
    pub variant_aspiration_quality: String,
    #[serde(rename(deserialize = "Variant Fuel System Quality"))]
    pub variant_fuel_system_quality: String,
    #[serde(rename(deserialize = "Variant Exhaust Quality"))]
    pub variant_exhaust_quality: String,
    #[serde(rename(deserialize = "Variant Top End Techpool"))]
    pub variant_top_end_techpool: String,
    #[serde(rename(deserialize = "Variant Family Techpool"))]
    pub variant_family_techpool: String,
    #[serde(rename(deserialize = "Variant Bottom End Techpool"))]
    pub variant_bottom_end_techpool: String,
    #[serde(rename(deserialize = "Variant Aspiration Techpool"))]
    pub variant_aspiration_techpool: String,
    #[serde(rename(deserialize = "Variant Fuel System Techpool"))]
    pub variant_fuel_system_techpool: String,
    #[serde(rename(deserialize = "Variant Exhaust Techpool"))]
    pub variant_exhaust_techpool: String,
    #[serde(rename(deserialize = "Variant Bore"))]
    pub variant_bore: String,
    #[serde(rename(deserialize = "Variant Stroke"))]
    pub variant_stroke: String,
    #[serde(rename(deserialize = "Variant Displacement"))]
    pub variant_displacement: String,
    #[serde(rename(deserialize = "Balance Shaft"))]
    pub balance_shaft: String,
    #[serde(rename(deserialize = "Crankshaft"))]
    pub crankshaft: String,
    #[serde(rename(deserialize = "Conrods"))]
    pub conrods: String,
    #[serde(rename(deserialize = "Pistons"))]
    pub pistons: String,
    #[serde(rename(deserialize = "Compression"))]
    pub compression: String,
    #[serde(rename(deserialize = "Cam Profile"))]
    pub cam_profile: String,
    #[serde(rename(deserialize = "Valve Spring Stiffness"))]
    pub valve_spring_stiffness: String,
    #[serde(rename(deserialize = "VVT"))]
    pub vvt: String,
    #[serde(rename(deserialize = "VVL"))]
    pub vvl: String,
    #[serde(rename(deserialize = "Aspiration"))]
    pub aspiration: String,
    #[serde(rename(deserialize = "Intercooler Size"))]
    pub intercooler_size: String,
    #[serde(rename(deserialize = "Aspiration Item 1"))]
    pub aspiration_item_1: String,
    #[serde(rename(deserialize = "Aspiration Sub Item 1"))]
    pub aspiration_sub_item_1: String,
    #[serde(rename(deserialize = "Aspiration Item 2"))]
    pub aspiration_item_2: String,
    #[serde(rename(deserialize = "Aspiration Sub Item 2"))]
    pub aspiration_sub_item_2: String,
    #[serde(rename(deserialize = "Boost Control"))]
    pub boost_control: String,
    #[serde(rename(deserialize = "Compressor Size 1"))]
    pub compressor_size_1: String,
    #[serde(rename(deserialize = "Compressor Size 2"))]
    pub compressor_size_2: String,
    #[serde(rename(deserialize = "Turbine Size 1"))]
    pub turbine_size_1: String,
    #[serde(rename(deserialize = "Turbine Size 2"))]
    pub turbine_size_2: String,
    #[serde(rename(deserialize = "Aspiration Tune 1"))]
    pub aspiration_tune_1: String,
    #[serde(rename(deserialize = "Aspiration Tune 2"))]
    pub aspiration_tune_2: String,
    #[serde(rename(deserialize = "Fuel System Type"))]
    pub fuel_system_type: String,
    #[serde(rename(deserialize = "Fuel System"))]
    pub fuel_system: String,
    #[serde(rename(deserialize = "Throttle Configuration"))]
    pub throttle_configuration: String,
    #[serde(rename(deserialize = "Intake"))]
    pub intake: String,
    #[serde(rename(deserialize = "Leaded Fuel"))]
    pub leaded_fuel: String,
    #[serde(rename(deserialize = "Fuel Octane"))]
    pub fuel_octane: String,
    #[serde(rename(deserialize = "Octane Offset"))]
    pub octane_offset: String,
    #[serde(rename(deserialize = "AFR"))]
    pub afr: String,
    #[serde(rename(deserialize = "Ignition Timing"))]
    pub ignition_timing: String,
    #[serde(rename(deserialize = "RPM Limit"))]
    pub rpm_limit: String,
    #[serde(rename(deserialize = "Headers"))]
    pub headers: String,
    #[serde(rename(deserialize = "Exhaust Count"))]
    pub exhaust_count: String,
    #[serde(rename(deserialize = "Exhaust Diameter"))]
    pub exhaust_diameter: String,
    #[serde(rename(deserialize = "Exhaust Bypass Valves"))]
    pub exhaust_bypass_valves: String,
    #[serde(rename(deserialize = "Catalytic Converter"))]
    pub catalytic_converter: String,
    #[serde(rename(deserialize = "Muffler 1"))]
    pub muffler_1: String,
    #[serde(rename(deserialize = "Muffler 2"))]
    pub muffler_2: String,
    #[serde(rename(deserialize = "Peak Power"))]
    pub peak_power: String,
    #[serde(rename(deserialize = "Peak Power RPM"))]
    pub peak_power_rpm: String,
    #[serde(rename(deserialize = "Peak Torque"))]
    pub peak_torque: String,
    #[serde(rename(deserialize = "Peak Torque RPM"))]
    pub peak_torque_rpm: String,
    #[serde(rename(deserialize = "Peak Boost"))]
    pub peak_boost: String,
    #[serde(rename(deserialize = "Peak Boost RPM"))]
    pub peak_boost_rpm: String,
    #[serde(rename(deserialize = "Idle RPM"))]
    pub idle_rpm: String,
    #[serde(rename(deserialize = "Max RPM"))]
    pub max_rpm: String,
    #[serde(rename(deserialize = "Min Economy RPM"))]
    pub min_economy_rpm: String,
    #[serde(rename(deserialize = "Engine Weight"))]
    pub engine_weight: String,
    #[serde(rename(deserialize = "Flywheel Weight"))]
    pub flywheel_weight: String,
    #[serde(rename(deserialize = "Required Cooling"))]
    pub required_cooling: String,
    #[serde(rename(deserialize = "Engine Performance Index"))]
    pub engine_performance_index: String,
    #[serde(rename(deserialize = "Engine Responsiveness"))]
    pub engine_responsiveness: String,
    #[serde(rename(deserialize = "Engine Smoothness"))]
    pub engine_smoothness: String,
    #[serde(rename(deserialize = "Engine Emissions"))]
    pub engine_emissions: String,
    #[serde(rename(deserialize = "Engine Efficiency"))]
    pub engine_efficiency: String,
    #[serde(rename(deserialize = "Engine Noise"))]
    pub engine_noise: String,
    #[serde(rename(deserialize = "Intake Noise"))]
    pub intake_noise: String,
    #[serde(rename(deserialize = "Exhaust Noise"))]
    pub exhaust_noise: String,
    #[serde(rename(deserialize = "Engine RON Rating"))]
    pub engine_ron_rating: String,
    #[serde(rename(deserialize = "Engine Engineering Time"))]
    pub engine_engineering_time: String,
    #[serde(rename(deserialize = "Engine Production Units"))]
    pub engine_production_units: String,
    #[serde(rename(deserialize = "Engine Engineering Costs"))]
    pub engine_engineering_costs: String,
    #[serde(rename(deserialize = "Engine Tooling Costs"))]
    pub engine_tooling_costs: String,
    #[serde(rename(deserialize = "Engine Material Cost"))]
    pub engine_material_cost: String,
    #[serde(rename(deserialize = "Engine Total Cost"))]
    pub engine_total_cost: String,
    #[serde(rename(deserialize = "Engine Service Cost"))]
    pub engine_service_cost: String,
    #[serde(rename(deserialize = "Crankshaft Max RPM"))]
    pub crankshaft_max_rpm: String,
    #[serde(rename(deserialize = "Crankshaft Max Torque"))]
    pub crankshaft_max_torque: String,
    #[serde(rename(deserialize = "Conrods Max RPM"))]
    pub conrods_max_rpm: String,
    #[serde(rename(deserialize = "Conrods Max Torque"))]
    pub conrods_max_torque: String,
    #[serde(rename(deserialize = "Pistons Max RPM"))]
    pub pistons_max_rpm: String,
    #[serde(rename(deserialize = "Pistons Max Torque"))]
    pub pistons_max_torque: String,
}

impl RawCar {
    pub fn from_directory(path: &str) -> Result<Self> {
        let name = path.replace("cars/", "").replace(r"cars\", "");
        let p = format!("{}/{}_utf8.csv", path, name);
        let mut rdr = csv::ReaderBuilder::default().has_headers(false).from_path(p)?;
        Ok(rdr.deserialize().nth(1).ok_or(ImportError::Unknown)??)
    }
}
