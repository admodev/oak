// Math module providing mathematical functions
use std::f64::consts::PI;

/// Mathematical functions for the Oak programming language
pub struct MathModule;

/// Building stability verification result
///
/// - If `overturning_moment` is near zero, `stability_ratio` will be 1e6 ("perfect stability").
/// - If `overturning_moment` is negative, the function returns an error.
#[derive(Debug, Clone)]
pub struct StabilityResult {
    pub resisting_moment: f64,
    pub overturning_moment: f64,
    pub stability_ratio: f64,
    pub is_stable: bool,
    pub safety_margin: f64,
}

/// Wind stiffness compliance result
#[derive(Debug, Clone)]
pub struct WindStiffnessResult {
    pub length_a: f64,
    pub width_b: f64,
    pub slenderness_ratio: f64,
    pub is_compliant: bool,
    pub warning_message: Option<String>,
}

/// Architectural calculation result
#[derive(Debug, Clone)]
pub struct ArchitecturalResult {
    pub calculation_type: String,
    pub result_value: f64,
    pub is_success: bool,
    pub message: String,
    pub details: Option<String>,
}

impl MathModule {
    /// Calculate the sine of an angle in radians
    /// Always defined for all real numbers
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    /// Calculate the cosine of an angle in radians
    /// Always defined for all real numbers
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    /// Calculate the tangent of an angle in radians
    /// Returns NaN for undefined values (e.g., tan(PI/2), tan(3*PI/2))
    pub fn tan(x: f64) -> f64 {
        // Check for undefined tangent values (where cos(x) = 0)
        // These occur at PI/2 + n*PI for any integer n
        let cos_val = x.cos();
        if cos_val.abs() < f64::EPSILON {
            f64::NAN
        } else {
            x.tan()
        }
    }

    /// Calculate the square root of a number
    /// Returns NaN for negative numbers
    pub fn sqrt(x: f64) -> f64 {
        if x < 0.0 {
            f64::NAN
        } else {
            x.sqrt()
        }
    }

    /// Calculate the natural logarithm of a number
    /// Returns NaN for non-positive numbers
    pub fn log(x: f64) -> f64 {
        if x <= 0.0 {
            f64::NAN
        } else {
            x.ln()
        }
    }

    /// Calculate e raised to the power of x
    /// Always defined for all real numbers
    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    /// Calculate the absolute value of a number
    /// Always defined for all real numbers
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    /// Convert degrees to radians
    /// Always defined for all real numbers
    pub fn to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    /// Convert radians to degrees
    /// Always defined for all real numbers
    pub fn to_degrees(radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    /// Get the value of PI
    pub fn pi() -> f64 {
        PI
    }

    /// Get the value of e
    pub fn e() -> f64 {
        std::f64::consts::E
    }

    /// Check if a value is NaN (Not a Number)
    pub fn is_nan(x: f64) -> bool {
        x.is_nan()
    }

    /// Check if a value is infinite
    pub fn is_infinite(x: f64) -> bool {
        x.is_infinite()
    }

    /// Check if a value is finite (not NaN and not infinite)
    pub fn is_finite(x: f64) -> bool {
        x.is_finite()
    }

    // Helper functions for building stability calculations

    /// Safely convert f64 to u32 with comprehensive validation
    /// 
    /// # Arguments
    /// * `value` - The f64 value to convert
    /// * `parameter_name` - Name of the parameter for error messages
    /// 
    /// # Returns
    /// * `Ok(u32)` if conversion is safe
    /// * `Err(String)` with descriptive error message if conversion is unsafe
    fn safe_f64_to_u32(value: f64, parameter_name: &str) -> Result<u32, String> {
        if value.is_nan() || value.is_infinite() || value < 0.0 || value > u32::MAX as f64 {
            return Err(format!("{} cannot be NaN, infinite, negative or exceeds maximum value", parameter_name));
        }
        Ok(value as u32)
    }

    /// Validate building dimension parameters
    /// 
    /// # Arguments
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// * `building_height` - Total height of building (m)
    /// * `num_floors` - Number of floors (integer)
    /// 
    /// # Returns
    /// * `Ok(())` if all parameters are valid
    /// * `Err(String)` with error message if validation fails
    fn validate_building_parameters(
        building_length_a: f64,
        building_width_b: f64,
        building_height: f64,
        num_floors: u32,
    ) -> Result<(), String> {
        if building_length_a <= 0.0 {
            return Err("Building length must be positive".to_string());
        }
        if building_width_b <= 0.0 {
            return Err("Building width must be positive".to_string());
        }
        if building_height <= 0.0 {
            return Err("Building height must be positive".to_string());
        }
        if num_floors == 0 {
            return Err("Number of floors must be at least 1".to_string());
        }

        // Check for extremely small buildings that might cause numerical issues
        if building_length_a < 0.1 || building_width_b < 0.1 {
            return Err("Building dimensions must be at least 0.1 meters".to_string());
        }

        // Check for extremely large values that might cause overflow
        if building_length_a > 10000.0 || building_width_b > 10000.0 || building_height > 10000.0 {
            return Err("Building dimensions exceed maximum allowed values (10,000 m)".to_string());
        }

        Ok(())
    }

    /// Validate wind-related parameters
    /// 
    /// # Arguments
    /// * `wind_load_per_sqm` - Wind load per square meter (kN/m²)
    /// * `wind_force_height` - Height where wind force acts (m)
    /// * `building_height` - Total height of building (m)
    /// 
    /// # Returns
    /// * `Ok(())` if all parameters are valid
    /// * `Err(String)` with error message if validation fails
    fn validate_wind_parameters(
        wind_load_per_sqm: f64,
        wind_force_height: f64,
        building_height: f64,
    ) -> Result<(), String> {
        if wind_load_per_sqm <= 0.0 {
            return Err("Wind load per square meter must be positive".to_string());
        }
        if wind_force_height <= 0.0 || wind_force_height > building_height {
            return Err("Wind force height must be positive and not exceed building height".to_string());
        }
        Ok(())
    }

    /// Validate calculation result for overflow or NaN
    /// 
    /// # Arguments
    /// * `value` - The calculated value to validate
    /// * `calculation_name` - Name of the calculation for error messages
    /// 
    /// # Returns
    /// * `Ok(())` if the value is valid
    /// * `Err(String)` with error message if validation fails
    fn validate_calculation_result(value: f64, calculation_name: &str) -> Result<(), String> {
        if value.is_infinite() || value.is_nan() {
            return Err(format!("{} resulted in invalid value (overflow or NaN)", calculation_name));
        }
        Ok(())
    }

    /// Calculate center to corner distance (diagonal distance from center to corner)
    /// 
    /// # Arguments
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// 
    /// # Returns
    /// * `Ok(f64)` - The center to corner distance
    /// * `Err(String)` with error message if calculation fails
    fn calculate_center_to_corner_distance(
        building_length_a: f64,
        building_width_b: f64,
    ) -> Result<f64, String> {
        let center_to_corner_distance = MathModule::sqrt(
            (building_length_a / 2.0).powi(2) + (building_width_b / 2.0).powi(2)
        );

        // Check for invalid center to corner distance
        MathModule::validate_calculation_result(center_to_corner_distance, "Center to corner distance calculation")?;

        // Check for division by zero
        if center_to_corner_distance == 0.0 {
            return Err("Center to corner distance cannot be zero".to_string());
        }

        Ok(center_to_corner_distance)
    }

    /// Verify building stability against overturning due to wind loads
    /// 
    /// # Arguments
    /// * `dead_load_per_sqm` - Dead load per square meter (kN/m²)
    /// * `wind_load_per_sqm` - Wind load per square meter (kN/m²)
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// * `building_height` - Total height of building (m)
    /// * `num_floors` - Number of floors (integer)
    /// * `wind_force_height` - Height where wind force acts (m), typically h/2
    /// 
    /// # Returns
    /// * `StabilityResult` with detailed calculation results
    ///
    /// # Special Cases
    /// * If overturning moment is near zero (abs < f64::EPSILON), returns a stability ratio of 1e6 ("perfect stability").
    /// * If overturning moment is negative, returns an error (physically impossible).
    ///
    /// # Safety Criterion
    /// The building is considered stable if Me/Mv >= 3
    /// where Me is the resisting moment and Mv is the overturning moment
    ///
    /// # Example
    /// ```rust
    /// use oak::MathModule;
    /// let result = MathModule::verify_building_stability(
    ///     5.0, 1.0, 20.0, 15.0, 30.0, 10, 15.0
    /// );
    /// assert!(result.is_ok());
    /// let stability = result.unwrap();
    /// assert!(stability.is_stable);
    /// ```
    pub fn verify_building_stability(
        dead_load_per_sqm: f64,
        wind_load_per_sqm: f64,
        building_length_a: f64,
        building_width_b: f64,
        building_height: f64,
        num_floors: u32,
        wind_force_height: f64,
    ) -> Result<StabilityResult, String> {
        // Validate input parameters
        if dead_load_per_sqm <= 0.0 {
            return Err("Dead load per square meter must be positive".to_string());
        }
        MathModule::validate_building_parameters(building_length_a, building_width_b, building_height, num_floors)?;
        MathModule::validate_wind_parameters(wind_load_per_sqm, wind_force_height, building_height)?;

        // Calculate total dead load G
        let total_dead_load = dead_load_per_sqm * building_length_a * building_width_b * num_floors as f64;
        MathModule::validate_calculation_result(total_dead_load, "Dead load calculation")?;

        // Calculate distance from center of gravity to furthest corner (da)
        let center_to_corner_distance = MathModule::calculate_center_to_corner_distance(building_length_a, building_width_b)?;

        // Calculate resisting moment Me = G * da
        let resisting_moment = total_dead_load * center_to_corner_distance;
        MathModule::validate_calculation_result(resisting_moment, "Resisting moment calculation")?;

        // Calculate wind force W = qw * h * a
        let wind_force = wind_load_per_sqm * building_height * building_length_a;
        MathModule::validate_calculation_result(wind_force, "Wind force calculation")?;

        // Calculate overturning moment Mv = W * d
        let overturning_moment = wind_force * wind_force_height;
        MathModule::validate_calculation_result(overturning_moment, "Overturning moment calculation")?;

        // Calculate stability ratio with division by zero and negative protection
        let stability_ratio = if overturning_moment > f64::EPSILON {
            let ratio = resisting_moment / overturning_moment;
            if ratio.is_infinite() || ratio.is_nan() {
                return Err("Stability ratio calculation resulted in invalid value".to_string());
            }
            ratio
        } else if overturning_moment.abs() < f64::EPSILON {
            // Special case: no overturning moment means perfect stability
            // Use a large finite value to indicate this
            1e6
        } else {
            return Err("Negative overturning moment is physically impossible".to_string());
        };

        // Check stability criterion (Me/Mv >= 3)
        let is_stable = stability_ratio >= 3.0;
        let safety_margin = stability_ratio - 3.0;

        // Final validation of result values
        MathModule::validate_calculation_result(safety_margin, "Safety margin calculation")?;

        Ok(StabilityResult {
            resisting_moment,
            overturning_moment,
            stability_ratio,
            is_stable,
            safety_margin,
        })
    }

    /// Calculate the minimum required dead load for stability
    /// 
    /// # Arguments
    /// * `wind_load_per_sqm` - Wind load per square meter (kN/m²)
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// * `building_height` - Total height of building (m)
    /// * `num_floors` - Number of floors (integer)
    /// * `wind_force_height` - Height where wind force acts (m)
    /// * `safety_factor` - Required safety factor (default 3.0)
    /// 
    /// # Returns
    /// * Minimum dead load per square meter required for stability
    pub fn calculate_minimum_dead_load(
        wind_load_per_sqm: f64,
        building_length_a: f64,
        building_width_b: f64,
        building_height: f64,
        num_floors: u32,
        wind_force_height: f64,
        safety_factor: f64,
    ) -> Result<f64, String> {
        // Validate input parameters
        MathModule::validate_building_parameters(building_length_a, building_width_b, building_height, num_floors)?;
        MathModule::validate_wind_parameters(wind_load_per_sqm, wind_force_height, building_height)?;
        if safety_factor <= 0.0 {
            return Err("Safety factor must be positive".to_string());
        }

        // Calculate wind force
        let wind_force = wind_load_per_sqm * building_height * building_length_a;
        MathModule::validate_calculation_result(wind_force, "Wind force calculation")?;
        
        // Calculate overturning moment
        let overturning_moment = wind_force * wind_force_height;
        MathModule::validate_calculation_result(overturning_moment, "Overturning moment calculation")?;
        
        // Calculate center to corner distance
        let center_to_corner_distance = MathModule::calculate_center_to_corner_distance(building_length_a, building_width_b)?;
        
        // Calculate required resisting moment
        let required_resisting_moment = overturning_moment * safety_factor;
        MathModule::validate_calculation_result(required_resisting_moment, "Required resisting moment calculation")?;
        
        // Calculate required total dead load
        let required_total_dead_load = required_resisting_moment / center_to_corner_distance;
        MathModule::validate_calculation_result(required_total_dead_load, "Required total dead load calculation")?;
        
        // Calculate building area
        let building_area = building_length_a * building_width_b * num_floors as f64;
        
        // Check for division by zero
        if building_area == 0.0 {
            return Err("Building area cannot be zero".to_string());
        }
        
        // Calculate required dead load per square meter
        let required_dead_load_per_sqm = required_total_dead_load / building_area;
        
        // Final validation of result
        MathModule::validate_calculation_result(required_dead_load_per_sqm, "Required dead load per square meter calculation")?;
        
        Ok(required_dead_load_per_sqm)
    }

    /// Check wind stiffness compliance based on slenderness ratio
    /// 
    /// # Arguments
    /// * `length_a` - Longer side (length) of the building (m)
    /// * `width_b` - Shorter side (width) of the building (m)
    /// 
    /// # Returns
    /// * `WindStiffnessResult` with compliance check results
    /// 
    /// # Compliance Criterion
    /// The building is considered compliant if b/a > 1/5
    /// where b is the shorter side and a is the longer side
    /// 
    /// # Example
    /// ```rust
    /// use oak::MathModule;
    /// let result = MathModule::check_wind_stiffness_compliance(20.0, 15.0);
    /// assert!(result.unwrap().is_compliant); // 15/20 = 0.75 > 0.2
    /// ```
    pub fn check_wind_stiffness_compliance(length_a: f64, width_b: f64) -> Result<WindStiffnessResult, String> {
        // Use existing calculate_slenderness_ratio function for validation and calculation
        let slenderness_ratio = MathModule::calculate_slenderness_ratio(length_a, width_b)?;
        
        // Identify longer and shorter sides for the result
        let (a, b) = if length_a >= width_b {
            (length_a, width_b)
        } else {
            (width_b, length_a)
        };
        
        // Check compliance criterion (b/a > 1/5)
        let is_compliant = slenderness_ratio > 0.2; // 1/5 = 0.2

        // Generate warning message if not compliant
        let warning_message = if !is_compliant {
            Some(format!(
                "Building is too slender. Slenderness ratio {:.3} is below the minimum requirement of 0.2. Consider increasing the shorter dimension or adding structural reinforcements.",
                slenderness_ratio
            ))
        } else {
            None
        };

        Ok(WindStiffnessResult {
            length_a: a,
            width_b: b,
            slenderness_ratio,
            is_compliant,
            warning_message,
        })
    }

    /// Perform comprehensive architectural calculations
    /// 
    /// # Arguments
    /// * `calculation_type` - Type of calculation to perform
    /// * `params` - Vector of parameters for the calculation
    /// 
    /// # Supported Calculation Types
    /// * "wind_stiffness" - Check wind stiffness compliance (params: [length, width])
    /// * "stability" - Verify building stability (params: [dead_load, wind_load, length, width, height, floors, wind_height])
    /// * "min_dead_load" - Calculate minimum dead load (params: [wind_load, length, width, height, floors, wind_height, safety_factor])
    /// * "slenderness_ratio" - Calculate slenderness ratio (params: [length, width])
    /// 
    /// # Returns
    /// * `ArchitecturalResult` with calculation results
    pub fn calc_architecture(calculation_type: &str, params: Vec<f64>) -> Result<ArchitecturalResult, String> {
        match calculation_type.to_lowercase().as_str() {
            "wind_stiffness" => {
                if params.len() != 2 {
                    return Err("Wind stiffness calculation requires exactly 2 parameters: [length, width]".to_string());
                }
                
                let result = MathModule::check_wind_stiffness_compliance(params[0], params[1])?;
                let message = if result.is_compliant {
                    format!("Wind stiffness compliant. Slenderness ratio: {:.3}", result.slenderness_ratio)
                } else {
                    format!("Wind stiffness non-compliant. Slenderness ratio: {:.3}", result.slenderness_ratio)
                };
                
                Ok(ArchitecturalResult {
                    calculation_type: "wind_stiffness".to_string(),
                    result_value: result.slenderness_ratio,
                    is_success: result.is_compliant,
                    message,
                    details: result.warning_message,
                })
            }
            
            "stability" => {
                if params.len() != 7 {
                    return Err("Stability calculation requires exactly 7 parameters: [dead_load, wind_load, length, width, height, floors, wind_height]".to_string());
                }
                
                // Validate floors parameter for safe f64 to u32 conversion
                let num_floors = MathModule::safe_f64_to_u32(params[5], "Number of floors")?;
                
                let result = MathModule::verify_building_stability(
                    params[0], params[1], params[2], params[3], params[4], 
                    num_floors, params[6]
                )?;
                
                let message = if result.is_stable {
                    format!("Building is stable. Stability ratio: {:.3}", result.stability_ratio)
                } else {
                    format!("Building is unstable. Stability ratio: {:.3}", result.stability_ratio)
                };
                
                let details = format!(
                    "Resisting moment: {:.2} kN⋅m, Overturning moment: {:.2} kN⋅m, Safety margin: {:.3}",
                    result.resisting_moment, result.overturning_moment, result.safety_margin
                );
                
                Ok(ArchitecturalResult {
                    calculation_type: "stability".to_string(),
                    result_value: result.stability_ratio,
                    is_success: result.is_stable,
                    message,
                    details: Some(details),
                })
            }
            
            "min_dead_load" => {
                if params.len() != 7 {
                    return Err("Minimum dead load calculation requires exactly 7 parameters: [wind_load, length, width, height, floors, wind_height, safety_factor]".to_string());
                }
                
                // Validate floors parameter for safe f64 to u32 conversion
                let num_floors = MathModule::safe_f64_to_u32(params[4], "Number of floors")?;
                
                let result = MathModule::calculate_minimum_dead_load(
                    params[0], params[1], params[2], params[3], 
                    num_floors, params[5], params[6]
                )?;
                
                let message = format!("Minimum required dead load: {:.3} kN/m²", result);
                
                Ok(ArchitecturalResult {
                    calculation_type: "min_dead_load".to_string(),
                    result_value: result,
                    is_success: true,
                    message,
                    details: None,
                })
            }
            
            "slenderness_ratio" => {
                if params.len() != 2 {
                    return Err("Slenderness ratio calculation requires exactly 2 parameters: [length, width]".to_string());
                }
                
                // Use existing calculate_slenderness_ratio function for validation and calculation
                let ratio = MathModule::calculate_slenderness_ratio(params[0], params[1])?;
                
                // Identify longer and shorter sides for the details
                let (a, b) = if params[0] >= params[1] {
                    (params[0], params[1])
                } else {
                    (params[1], params[0])
                };
                
                let message = format!("Slenderness ratio: {:.3} (b/a)", ratio);
                
                Ok(ArchitecturalResult {
                    calculation_type: "slenderness_ratio".to_string(),
                    result_value: ratio,
                    is_success: true,
                    message,
                    details: Some(format!("Longer side (a): {:.2} m, Shorter side (b): {:.2} m", a, b)),
                })
            }
            
            _ => {
                Err(format!("Unknown calculation type: {}. Supported types: wind_stiffness, stability, min_dead_load, slenderness_ratio", calculation_type))
            }
        }
    }

    /// Calculate slenderness ratio (b/a) for building footprint
    /// 
    /// # Arguments
    /// * `length_a` - Longer side (length) of the building (m)
    /// * `width_b` - Shorter side (width) of the building (m)
    /// 
    /// # Returns
    /// * Slenderness ratio (b/a)
    /// 
    /// # Example
    /// ```rust
    /// use oak::MathModule;
    /// let ratio = MathModule::calculate_slenderness_ratio(20.0, 15.0);
    /// assert_eq!(ratio, Ok(0.75)); // 15/20 = 0.75
    /// ```
    pub fn calculate_slenderness_ratio(length_a: f64, width_b: f64) -> Result<f64, String> {
        // Validate input parameters
        if length_a <= 0.0 {
            return Err("Building length must be positive".to_string());
        }
        if width_b <= 0.0 {
            return Err("Building width must be positive".to_string());
        }

        // Identify longer and shorter sides
        let (a, b) = if length_a >= width_b {
            (length_a, width_b)
        } else {
            (width_b, length_a)
        };

        // Check for division by zero before calculation
        if a == 0.0 {
            return Err("Building length cannot be zero".to_string());
        }

        // Calculate slenderness ratio
        let slenderness_ratio = b / a;
        
        // Validate result
        MathModule::validate_calculation_result(slenderness_ratio, "Slenderness ratio calculation")?;

        Ok(slenderness_ratio)
    }
}

/// Function registry for math functions
pub fn get_math_functions() -> std::collections::HashMap<String, fn(f64) -> f64> {
    let mut functions = std::collections::HashMap::new();
    
    functions.insert("sin".to_string(), MathModule::sin as fn(f64) -> f64);
    functions.insert("cos".to_string(), MathModule::cos as fn(f64) -> f64);
    functions.insert("tan".to_string(), MathModule::tan as fn(f64) -> f64);
    functions.insert("sqrt".to_string(), MathModule::sqrt as fn(f64) -> f64);
    functions.insert("log".to_string(), MathModule::log as fn(f64) -> f64);
    functions.insert("exp".to_string(), MathModule::exp as fn(f64) -> f64);
    functions.insert("abs".to_string(), MathModule::abs as fn(f64) -> f64);
    functions.insert("to_radians".to_string(), MathModule::to_radians as fn(f64) -> f64);
    functions.insert("to_degrees".to_string(), MathModule::to_degrees as fn(f64) -> f64);
    
    functions
}

/// Function registry for math constants
pub fn get_math_constants() -> std::collections::HashMap<String, f64> {
    let mut constants = std::collections::HashMap::new();
    
    constants.insert("PI".to_string(), MathModule::pi());
    constants.insert("E".to_string(), MathModule::e());
    
    constants
}

/// Expose architectural calculation as a command for the interpreter/CLI
pub fn calc_architecture_command(calculation_type: &str, params: Vec<f64>) -> String {
    match MathModule::calc_architecture(calculation_type, params) {
        Ok(result) => {
            let mut output = format!("{}: {}\n", result.calculation_type, result.message);
            if let Some(details) = result.details {
                output.push_str(&format!("{}\n", details));
            }
            output
        }
        Err(e) => format!("Error: {}", e),
    }
} 