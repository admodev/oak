#[cfg(test)]
#[allow(warnings)]
#[test]
fn test_binary_operation() {
    use crate::{
        interpreter::Interpreter,
        parser::{Assign, BinOp, Node, Number, Value, Var},
    };

    let expr = BinOp::parse(
        Box::new(Number::parse("3")),
        "+".to_string(),
        Box::new(Number::parse("4")),
    );

    let assignment = Assign::parse("x".to_string(), Box::new(expr));
    let mut interpreter = Interpreter::new();

    assignment.accept(&mut interpreter);

    let var = Var::parse("x".to_string());
    let result = var.accept(&mut interpreter);

    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_math_functions() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sin function
    let sin_call = FunctionCall::parse(
        "sin".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = sin_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(0.0));

    // Test cos function
    let cos_call = FunctionCall::parse(
        "cos".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = cos_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(1.0));

    // Test sqrt function
    let sqrt_call = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("4"))],
    );
    let result = sqrt_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(2.0));

    // Test abs function
    let abs_call = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("-5"))],
    );
    let result = abs_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_math_functions_error_handling() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sqrt with negative input - should return NaN
    let sqrt_negative = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = sqrt_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("sqrt(-1) should return NaN"),
    }

    // Test log with zero - should return NaN
    let log_zero = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = log_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("log(0) should return NaN"),
    }

    // Test log with negative input - should return NaN
    let log_negative = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = log_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("log(-1) should return NaN"),
    }

    // Test tan(PI/2) - should return NaN (undefined)
    let tan_pi_over_2 = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("1.5707963267948966"))], // PI/2
    );
    let result = tan_pi_over_2.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "tan(PI/2) should return NaN, got {}", val),
        _ => panic!("tan(PI/2) should return NaN"),
    }

    // Test tan(3*PI/2) - should return NaN (undefined)
    let tan_3pi_over_2 = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("4.71238898038469"))], // 3*PI/2
    );
    let result = tan_3pi_over_2.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "tan(3*PI/2) should return NaN, got {}", val),
        _ => panic!("tan(3*PI/2) should return NaN"),
    }

    // Test tan(0) - should return 0 (defined)
    let tan_zero = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = tan_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "tan(0) should return 0, got {}", val),
        _ => panic!("tan(0) should return 0"),
    }

    // Test tan(PI) - should return 0 (defined)
    let tan_pi = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("3.141592653589793"))], // PI
    );
    let result = tan_pi.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "tan(PI) should return 0, got {}", val),
        _ => panic!("tan(PI) should return 0"),
    }

    // Test tan(PI/4) - should return 1 (defined)
    let tan_pi_over_4 = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("0.7853981633974483"))], // PI/4
    );
    let result = tan_pi_over_4.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 1.0).abs() < 1e-10, "tan(PI/4) should return 1, got {}", val),
        _ => panic!("tan(PI/4) should return 1"),
    }
}

#[test]
fn test_angle_conversion_functions() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test to_radians function
    let to_radians_call = FunctionCall::parse(
        "to_radians".to_string(),
        vec![Box::new(Number::parse("180"))],
    );
    let result = to_radians_call.accept(&mut interpreter);
    match result {
        Value::Number(val) => {
            assert!((val - std::f64::consts::PI).abs() < 1e-10);
        }
        _ => panic!("to_radians(180) should return PI"),
    }

    // Test to_degrees function
    let to_degrees_call = FunctionCall::parse(
        "to_degrees".to_string(),
        vec![Box::new(Number::parse(&std::f64::consts::PI.to_string()))],
    );
    let result = to_degrees_call.accept(&mut interpreter);
    match result {
        Value::Number(val) => {
            assert!((val - 180.0).abs() < 1e-10);
        }
        _ => panic!("to_degrees(PI) should return 180"),
    }
}

#[test]
fn test_math_constants() {
    use crate::{
        interpreter::Interpreter,
        parser::{Node, Value, Var},
    };

    let mut interpreter = Interpreter::new();

    // Test PI constant
    let pi_var = Var::parse("PI".to_string());
    let result = pi_var.accept(&mut interpreter);
    match result {
        Value::Number(pi_value) => {
            assert!((pi_value - std::f64::consts::PI).abs() < 1e-10);
        }
        _ => panic!("PI should be a number"),
    }

    // Test E constant
    let e_var = Var::parse("E".to_string());
    let result = e_var.accept(&mut interpreter);
    match result {
        Value::Number(e_value) => {
            assert!((e_value - std::f64::consts::E).abs() < 1e-10);
        }
        _ => panic!("E should be a number"),
    }
}

#[test]
fn test_math_function_with_variable() {
    use crate::{
        interpreter::Interpreter,
        parser::{Assign, FunctionCall, Node, Number, Value, Var},
    };

    let mut interpreter = Interpreter::new();

    // Assign a value to a variable
    let assignment = Assign::parse(
        "x".to_string(),
        Box::new(Number::parse("16")),
    );
    assignment.accept(&mut interpreter);

    // Use the variable in a math function
    let sqrt_call = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Var::parse("x".to_string()))],
    );
    let result = sqrt_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_runtime_script_parsing() {
    use crate::parser::parse_script;

    let script_source: String = "./test.oak".to_string();

    if let Err(_) = parse_script(script_source) {
        println!("Failed to assert the result of file parsing was ok!");
        std::process::exit(1);
    } else {
        println!("File parsing result was ok!");
        std::process::exit(0);
    }
}

#[test]
fn test_math_functions_edge_cases() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sqrt(0) - should return 0
    let sqrt_zero = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = sqrt_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "sqrt(0) should return 0, got {}", val),
        _ => panic!("sqrt(0) should return 0"),
    }

    // Test log(1) - should return 0
    let log_one = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("1"))],
    );
    let result = log_one.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "log(1) should return 0, got {}", val),
        _ => panic!("log(1) should return 0"),
    }

    // Test exp(0) - should return 1
    let exp_zero = FunctionCall::parse(
        "exp".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = exp_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 1.0).abs() < 1e-10, "exp(0) should return 1, got {}", val),
        _ => panic!("exp(0) should return 1"),
    }

    // Test abs(0) - should return 0
    let abs_zero = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = abs_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "abs(0) should return 0, got {}", val),
        _ => panic!("abs(0) should return 0"),
    }

    // Test abs(-0) - should return 0
    let abs_negative_zero = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("-0"))],
    );
    let result = abs_negative_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "abs(-0) should return 0, got {}", val),
        _ => panic!("abs(-0) should return 0"),
    }
}

#[test]
fn test_building_stability_verification() {
    use crate::math::MathModule;

    // Test a stable building
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );

    assert!(result.is_ok());
    let stability = result.unwrap();
    
    // Verify the building is stable (Me/Mv > 3)
    assert!(stability.is_stable);
    assert!(stability.stability_ratio > 3.0);
    assert!(stability.safety_margin > 0.0);
    
    // Verify all values are positive and finite
    assert!(stability.resisting_moment > 0.0);
    assert!(stability.overturning_moment > 0.0);
    assert!(stability.stability_ratio > 0.0);
    assert!(stability.safety_margin.is_finite());
}

#[test]
fn test_building_stability_unstable() {
    use crate::math::MathModule;

    // Test an unstable building (high wind load, low dead load)
    let result = MathModule::verify_building_stability(
        1.0,    // dead_load_per_sqm (kN/m²) - low
        5.0,    // wind_load_per_sqm (kN/m²) - high
        10.0,   // building_length_a (m)
        10.0,   // building_width_b (m)
        20.0,   // building_height (m)
        5,      // num_floors
        10.0,   // wind_force_height (m)
    );

    assert!(result.is_ok());
    let stability = result.unwrap();
    
    // Verify the building is unstable (Me/Mv < 3)
    assert!(!stability.is_stable);
    assert!(stability.stability_ratio < 3.0);
    assert!(stability.safety_margin < 0.0);
}

#[test]
fn test_building_stability_edge_cases() {
    use crate::math::MathModule;

    // Test a tall narrow building (likely unstable)
    let result = MathModule::verify_building_stability(
        8.0,    // dead_load_per_sqm (kN/m²)
        2.0,    // wind_load_per_sqm (kN/m²)
        5.0,    // building_length_a (m) - narrow
        5.0,    // building_width_b (m) - narrow
        100.0,  // building_height (m) - tall
        20,     // num_floors
        50.0,   // wind_force_height (m)
    );

    assert!(result.is_ok());
    let stability = result.unwrap();
    
    // This building should be unstable due to high height-to-width ratio
    assert!(!stability.is_stable);
    assert!(stability.stability_ratio < 3.0);
}

#[test]
fn test_building_stability_validation_errors() {
    use crate::math::MathModule;

    // Test negative dead load
    let result = MathModule::verify_building_stability(
        -1.0,   // dead_load_per_sqm (kN/m²) - negative
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Dead load per square meter must be positive"));

    // Test zero number of floors
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        0,      // num_floors - zero
        15.0,   // wind_force_height (m)
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Number of floors must be at least 1"));

    // Test wind force height exceeding building height
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        35.0,   // wind_force_height (m) - exceeds building height
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Wind force height must be positive and not exceed building height"));
}

#[test]
fn test_calculate_minimum_dead_load() {
    use crate::math::MathModule;

    // Test minimum dead load calculation
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        8,      // num_floors
        15.0,   // wind_force_height (m)
        3.0,    // safety_factor
    );

    assert!(result.is_ok());
    let min_dead_load = result.unwrap();
    
    // Verify the result is positive and finite
    assert!(min_dead_load > 0.0);
    assert!(min_dead_load.is_finite());
    
    // Verify that using this dead load results in exactly Me/Mv = 3.0
    let stability_result = MathModule::verify_building_stability(
        min_dead_load,
        2.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        8,      // num_floors
        15.0,   // wind_force_height (m)
    ).unwrap();
    
    // Debug output
    println!("Minimum dead load: {}", min_dead_load);
    println!("Stability ratio: {}", stability_result.stability_ratio);
    println!("Is stable: {}", stability_result.is_stable);
    println!("Safety margin: {}", stability_result.safety_margin);
    
    // The stability ratio should be very close to 3.0
    assert!((stability_result.stability_ratio - 3.0).abs() < 1e-10);
    assert!(stability_result.is_stable);
}

#[test]
fn test_calculate_minimum_dead_load_validation() {
    use crate::math::MathModule;

    // Test negative wind load
    let result = MathModule::calculate_minimum_dead_load(
        -1.0,   // wind_load_per_sqm (kN/m²) - negative
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        8,      // num_floors
        15.0,   // wind_force_height (m)
        3.0,    // safety_factor
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Wind load per square meter must be positive"));

    // Test zero number of floors
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        0,      // num_floors - zero
        15.0,   // wind_force_height (m)
        3.0,    // safety_factor
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Number of floors must be at least 1"));
}

#[test]
fn test_stability_result_structure() {
    use crate::math::{MathModule, StabilityResult};

    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    ).unwrap();

    // Test that the result can be cloned
    let cloned_result = result.clone();
    assert_eq!(result.resisting_moment, cloned_result.resisting_moment);
    assert_eq!(result.overturning_moment, cloned_result.overturning_moment);
    assert_eq!(result.stability_ratio, cloned_result.stability_ratio);
    assert_eq!(result.is_stable, cloned_result.is_stable);
    assert_eq!(result.safety_margin, cloned_result.safety_margin);

    // Test debug formatting
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("StabilityResult"));
}

#[test]
fn test_building_stability_extreme_values() {
    use crate::math::MathModule;

    // Test very small building dimensions
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        0.05,   // building_length_a (m) - too small
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building dimensions must be at least 0.1 meters"));

    // Test very large building dimensions
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20000.0, // building_length_a (m) - too large
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building dimensions exceed maximum allowed values"));

    // Test valid extreme values
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        0.1,    // building_length_a (m) - minimum valid
        0.1,    // building_width_b (m) - minimum valid
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );
    assert!(result.is_ok());
}

#[test]
fn test_calculate_minimum_dead_load_extreme_values() {
    use crate::math::MathModule;

    // Test very small building dimensions
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm (kN/m²)
        0.05,   // building_length_a (m) - too small
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        8,      // num_floors
        15.0,   // wind_force_height (m)
        3.0,    // safety_factor
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building dimensions must be at least 0.1 meters"));

    // Test very large building dimensions
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm (kN/m²)
        20000.0, // building_length_a (m) - too large
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        8,      // num_floors
        15.0,   // wind_force_height (m)
        3.0,    // safety_factor
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building dimensions exceed maximum allowed values"));
}

#[test]
fn test_building_stability_overflow_protection() {
    use crate::math::MathModule;

    // Test with very large values that could cause overflow
    let result = MathModule::verify_building_stability(
        1e6,    // dead_load_per_sqm (kN/m²) - very large
        1e6,    // wind_load_per_sqm (kN/m²) - very large
        10000.0, // building_length_a (m) - maximum allowed
        10000.0, // building_width_b (m) - maximum allowed
        10000.0, // building_height (m) - maximum allowed
        100,    // num_floors - large number
        5000.0, // wind_force_height (m)
    );

    // This should either succeed with valid results or fail with a clear error
    match result {
        Ok(stability) => {
            // If it succeeds, verify all values are finite
            assert!(stability.resisting_moment.is_finite());
            assert!(stability.overturning_moment.is_finite());
            assert!(stability.stability_ratio.is_finite());
            assert!(stability.safety_margin.is_finite());
        }
        Err(error) => {
            // If it fails, it should be due to overflow protection
            assert!(error.contains("overflow") || error.contains("invalid value"));
        }
    }
}

#[test]
fn test_calculate_minimum_dead_load_overflow_protection() {
    use crate::math::MathModule;

    // Test with very large values that could cause overflow
    let result = MathModule::calculate_minimum_dead_load(
        1e6,    // wind_load_per_sqm (kN/m²) - very large
        10000.0, // building_length_a (m) - maximum allowed
        10000.0, // building_width_b (m) - maximum allowed
        10000.0, // building_height (m) - maximum allowed
        100,    // num_floors - large number
        5000.0, // wind_force_height (m)
        3.0,    // safety_factor
    );

    // This should either succeed with valid results or fail with a clear error
    match result {
        Ok(min_dead_load) => {
            // If it succeeds, verify the result is finite and positive
            assert!(min_dead_load.is_finite());
            assert!(min_dead_load > 0.0);
        }
        Err(error) => {
            // If it fails, it should be due to overflow protection
            assert!(error.contains("overflow") || error.contains("invalid value"));
        }
    }
}

#[test]
fn test_building_stability_zero_overturning_moment() {
    use crate::math::MathModule;

    // Test with zero wind load (should result in perfect stability: ratio = 1e6)
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1e-20,  // wind_load_per_sqm (kN/m²) - extremely small to ensure overturning moment < f64::EPSILON
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );

    assert!(result.is_ok());
    let stability = result.unwrap();
    assert_eq!(stability.stability_ratio, 1e6);
    assert!(stability.is_stable);
    assert!(stability.safety_margin > 0.0);
}

#[test]
fn test_building_stability_negative_overturning_moment() {
    use crate::math::MathModule;

    // Negative wind force height (physically impossible, should error)
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        -15.0,  // wind_force_height (m) - negative
    );
    assert!(result.is_err());
    // The error should be about wind force height, not negative overturning moment, due to earlier validation
    // But let's also test a case where overturning moment is negative due to negative wind load
    let result2 = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        -1.0,   // wind_load_per_sqm (kN/m²) - negative
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10,     // num_floors
        15.0,   // wind_force_height (m)
    );
    assert!(result2.is_err());
}

#[test]
fn test_wind_stiffness_compliance() {
    use crate::math::MathModule;
    // Compliant case: 20x15 (b/a = 0.75 > 0.2)
    let result = MathModule::check_wind_stiffness_compliance(20.0, 15.0).unwrap();
    assert!(result.is_compliant);
    assert!((result.slenderness_ratio - 0.75).abs() < 1e-10);
    assert!(result.warning_message.is_none());

    // Non-compliant case: 20x3 (b/a = 0.15 < 0.2)
    let result = MathModule::check_wind_stiffness_compliance(20.0, 3.0).unwrap();
    assert!(!result.is_compliant);
    assert!((result.slenderness_ratio - 0.15).abs() < 1e-10);
    assert!(result.warning_message.is_some());
}

#[test]
fn test_calc_architecture_command() {
    use crate::math::calc_architecture_command;
    // Wind stiffness compliant
    let out = calc_architecture_command("wind_stiffness", vec![20.0, 15.0]);
    assert!(out.contains("compliant"));
    assert!(out.contains("0.750"));
    // Wind stiffness non-compliant
    let out = calc_architecture_command("wind_stiffness", vec![20.0, 3.0]);
    assert!(out.contains("non-compliant"));
    assert!(out.contains("0.150"));
    // Slenderness ratio
    let out = calc_architecture_command("slenderness_ratio", vec![20.0, 15.0]);
    assert!(out.contains("Slenderness ratio: 0.750"));
    // Stability (should be stable)
    let out = calc_architecture_command("stability", vec![5.0, 1.0, 20.0, 15.0, 30.0, 10.0, 15.0]);
    assert!(out.contains("stable"));
    // Minimum dead load
    let out = calc_architecture_command("min_dead_load", vec![2.0, 20.0, 15.0, 30.0, 8.0, 15.0, 3.0]);
    assert!(out.contains("Minimum required dead load"));
    // Error: wrong params
    let out = calc_architecture_command("wind_stiffness", vec![20.0]);
    assert!(out.contains("Error"));
    // Error: unknown type
    let out = calc_architecture_command("unknown_type", vec![1.0, 2.0]);
    assert!(out.contains("Unknown calculation type"));
}

#[test]
fn test_calc_architecture_division_by_zero_protection() {
    use crate::math::calc_architecture_command;
    
    // Test calc_architecture slenderness_ratio with zero length
    let result = calc_architecture_command("slenderness_ratio", vec![0.0, 15.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Building length must be positive"));
    
    // Test calc_architecture slenderness_ratio with zero width
    let result = calc_architecture_command("slenderness_ratio", vec![20.0, 0.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Building width must be positive"));
}

#[test]
fn test_division_by_zero_protection() {
    use crate::math::MathModule;
    
    // Test wind stiffness compliance with zero length
    let result = MathModule::check_wind_stiffness_compliance(0.0, 15.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building length must be positive"));
    
    // Test wind stiffness compliance with zero width
    let result = MathModule::check_wind_stiffness_compliance(20.0, 0.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building width must be positive"));
    
    // Test slenderness ratio with zero length
    let result = MathModule::calculate_slenderness_ratio(0.0, 15.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building length must be positive"));
    
    // Test slenderness ratio with zero width
    let result = MathModule::calculate_slenderness_ratio(20.0, 0.0);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Building width must be positive"));
}

#[test]
fn test_safe_f64_to_u32_conversion() {
    use crate::math::calc_architecture_command;
    
    // Test stability calculation with NaN floors
    let result = calc_architecture_command("stability", vec![5.0, 1.0, 20.0, 15.0, 30.0, f64::NAN, 15.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors cannot be NaN or infinite"));
    
    // Test stability calculation with infinite floors
    let result = calc_architecture_command("stability", vec![5.0, 1.0, 20.0, 15.0, 30.0, f64::INFINITY, 15.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors cannot be NaN or infinite"));
    
    // Test stability calculation with negative floors
    let result = calc_architecture_command("stability", vec![5.0, 1.0, 20.0, 15.0, 30.0, -5.0, 15.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors cannot be negative"));
    
    // Test stability calculation with floors exceeding u32::MAX
    let result = calc_architecture_command("stability", vec![5.0, 1.0, 20.0, 15.0, 30.0, (u32::MAX as f64) + 1.0, 15.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors exceeds maximum allowed value"));
    
    // Test min_dead_load calculation with NaN floors
    let result = calc_architecture_command("min_dead_load", vec![2.0, 20.0, 15.0, 30.0, f64::NAN, 15.0, 3.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors cannot be NaN or infinite"));
    
    // Test min_dead_load calculation with infinite floors
    let result = calc_architecture_command("min_dead_load", vec![2.0, 20.0, 15.0, 30.0, f64::INFINITY, 15.0, 3.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors cannot be NaN or infinite"));
    
    // Test min_dead_load calculation with negative floors
    let result = calc_architecture_command("min_dead_load", vec![2.0, 20.0, 15.0, 30.0, -5.0, 15.0, 3.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors cannot be negative"));
    
    // Test min_dead_load calculation with floors exceeding u32::MAX
    let result = calc_architecture_command("min_dead_load", vec![2.0, 20.0, 15.0, 30.0, (u32::MAX as f64) + 1.0, 15.0, 3.0]);
    assert!(result.contains("Error"));
    assert!(result.contains("Number of floors exceeds maximum allowed value"));
    
    // Test valid conversions (should succeed)
    let result = calc_architecture_command("stability", vec![5.0, 1.0, 20.0, 15.0, 30.0, 10.0, 15.0]);
    assert!(!result.contains("Error"));
    assert!(result.contains("stable") || result.contains("unstable"));
    
    let result = calc_architecture_command("min_dead_load", vec![2.0, 20.0, 15.0, 30.0, 8.0, 15.0, 3.0]);
    assert!(!result.contains("Error"));
    assert!(result.contains("Minimum required dead load"));
}
