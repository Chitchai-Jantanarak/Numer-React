// integration.rs
use crate::utils;

use std::f64::consts::PI;

use meval::Expr;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;



// Duplication struct

#[derive(Serialize)] // Serialize the struct
pub(crate) struct IntegralResult {
    pub(crate) true_result: f64,
    pub(crate) result: f64,
    pub(crate) error: f64
}

#[derive(Serialize)] // Serialize the struct
pub(crate) struct RombergResult { 
    pub(crate) true_result: f64,
    pub(crate) result: Vec<Vec<f64>>,
    pub(crate) error: Vec<Vec<f64>>
}

#[derive(Serialize)] // Serialize the struct
pub(crate) struct GuassIntegralResult {
    pub(crate) true_result: f64,
    pub(crate) result: f64,
    pub(crate) error: f64,
    pub(crate) abscissas: Vec<f64>,
    pub(crate) weight: Vec<f64>
}



// wasm conversion JsValue

#[wasm_bindgen]
pub fn trapezodial(equation: &str, bound_least: f64, bound_most: f64, trapezoid_count: usize, true_result: f64) -> JsValue {
    match trapezodial_core(equation, bound_least, bound_most, trapezoid_count, true_result) {
        Ok(result) => to_value(&result).unwrap_or_else(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => JsValue::from_str(&e),
    }
}

#[wasm_bindgen]
pub fn simpson_1in3(equation: &str, bound_least: f64, bound_most: f64, trapezoid_count: usize, true_result: f64) -> JsValue {
    match simpson_1in3_core(equation, bound_least, bound_most, trapezoid_count, true_result) {
        Ok(result) => to_value(&result).unwrap_or_else(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => JsValue::from_str(&e),
    }
}

#[wasm_bindgen]
pub fn simpson_3in8(equation: &str, bound_least: f64, bound_most: f64, trapezoid_count: usize, true_result: f64) -> JsValue {
    match simpson_3in8_core(equation, bound_least, bound_most, trapezoid_count, true_result) {
        Ok(result) => to_value(&result).unwrap_or_else(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => JsValue::from_str(&e),
    }
}

#[wasm_bindgen]
pub fn romberg(equation: &str, bound_least: f64, bound_most: f64, true_result: f64) -> JsValue {
    match romberg_core(equation, bound_least, bound_most, true_result) {
        Ok(result) => to_value(&result).unwrap_or_else(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => JsValue::from_str(&e),
    }
}

#[wasm_bindgen]
pub fn guass_integration(equation: &str, bound_least: f64, bound_most: f64, true_result: f64, points: usize) -> JsValue {
    match guass_integration_core(equation, bound_least, bound_most, true_result, points) {
        Ok(result) => to_value(&result).unwrap_or_else(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => JsValue::from_str(&e),
    }
}



// Add implement method

pub(crate) fn trapezodial_core
(
    equation: &str,
    bound_least: f64, 
    bound_most: f64, 
    trapezoid_count: usize, 
    true_result: f64
) -> Result<IntegralResult, String> {

    let expr: Expr = match equation.parse() {
        Ok(e)  => e,
        Err(_) => return Err("Invalid function".to_string()),
    };

    let each_bound : f64 = (bound_most - bound_least) / (trapezoid_count as f64);
    let mut result : f64 = 0.0;

    for i in 0..=trapezoid_count {
        let curr_bound: f64 = bound_least + i as f64 * each_bound;

        if i == 0 || i == trapezoid_count {
            result += utils::evaluate_expr(&expr, curr_bound);
        }
        else {
            result += 2.0 * utils::evaluate_expr(&expr, curr_bound);
        }
    }

    // trapezoidal calculation (h / 2 * sum)
    result = each_bound / 2.0 * result;

    let error = utils::error_calc(true_result, result);

    Ok(IntegralResult {
        true_result,
        result,
        error 
    })
}

pub(crate) fn simpson_1in3_core
(
    equation: &str,
    bound_least: f64, 
    bound_most: f64, 
    mut trapezoid_count: usize, 
    true_result: f64
) -> Result<IntegralResult, String> {

    let expr: Expr = match equation.parse() {
        Ok(e)  => e,
        Err(_) => return Err("Invalid function".to_string()),
    };

    // Single count not permitted
    if trapezoid_count == 1 {
        trapezoid_count += 1;
    }

    let each_bound : f64 = (bound_most - bound_least) / (trapezoid_count as f64);
    let mut result : f64 = 0.0;

    for i in 0..=trapezoid_count {
        let curr_bound: f64 = bound_least + i as f64 * each_bound;

        if i == 0 || i == trapezoid_count {
            result += utils::evaluate_expr(&expr, curr_bound);
        }
        else if i % 2 == 1 {
            result += 4.0 * utils::evaluate_expr(&expr, curr_bound);
        }
        else if i % 2 == 0 {
            result += 2.0 * utils::evaluate_expr(&expr, curr_bound);
        }
    }

    // trapezoidal calculation (h / 2 * sum)
    result = each_bound / 3.0 * result;

    let error = utils::error_calc(true_result, result);

    Ok(IntegralResult {
        true_result,
        result,
        error 
    })
}

pub(crate) fn simpson_3in8_core
(
    equation: &str,
    bound_least: f64, 
    bound_most: f64, 
    mut trapezoid_count: usize, 
    true_result: f64
) -> Result<IntegralResult, String> {

    let expr: Expr = match equation.parse() {
        Ok(e)  => e,
        Err(_) => return Err("Invalid function".to_string()),
    };

    // Single count not permitted
    if trapezoid_count == 1 {
        trapezoid_count += 1;
    }

    let each_bound : f64 = (bound_most - bound_least) / (trapezoid_count as f64);
    let mut result : f64 = 0.0;

    for i in 0..=trapezoid_count {
        let curr_bound: f64 = bound_least + i as f64 * each_bound;

        if i == 0 || i == trapezoid_count {
            result += utils::evaluate_expr(&expr, curr_bound);
            continue;
        }
        
        if i % 3 == 0 {
            result += 2.0 * utils::evaluate_expr(&expr, curr_bound);
        }
        else {
            result += 3.0 * utils::evaluate_expr(&expr, curr_bound);
        }
    }

    // trapezoidal calculation (h / 2 * sum)
    result = each_bound * 3.0 / 8.0 * result;

    let error = utils::error_calc(true_result, result);

    Ok(IntegralResult {
        true_result,
        result,
        error 
    })
}

pub(crate) fn romberg_core 
(
    equation: &str,
    bound_least: f64, 
    bound_most: f64, 
    true_result: f64
) -> Result<RombergResult, String>    
{

    let expr: Expr = match equation.parse() {
        Ok(e)  => e,
        Err(_) => return Err("Invalid function".to_string()),
    };

    let mut result : Vec<Vec<f64>> = Vec::new();
    let mut error  : Vec<Vec<f64>> = Vec::new();
    let mut height : f64           = bound_most - bound_least;
    
    result.push(vec![trapezodial_calc(expr.clone(), bound_least, bound_most, height)]);
    error.push(vec![utils::error_calc(true_result, result[0][0])]);
    let mut iteration: usize = 0;

    loop {
        iteration += 1;
        height /= 2.0;

        result.push(vec![0.0; iteration + 1]);
        error.push(vec![0.0; iteration + 1]);

        result[iteration][0] = trapezodial_calc(expr.clone(), bound_least, bound_most, height);

        for i in 1..=iteration {
            result[iteration][i] = {
                4.0_f64.powi(i as i32) * result[iteration][i - 1] - result[iteration - 1][i - 1] 
                / ( 4.0_f64.powi(i as i32) - 1.0 )
            };

            error[iteration][i] = utils::error_calc(result[iteration][i], result[iteration][i - 1]);
            if error[iteration][i] < 1e-12 {

                error.push(vec![utils::error_calc(true_result, result[iteration][i])]);

                return Ok(RombergResult {
                    true_result, 
                    result: result.clone(), 
                    error: error.clone()
                   
               });
            }
        }
    }
}

pub(crate) fn guass_integration_core 
(
    equation: &str,
    bound_least: f64,
    bound_most: f64,
    true_result: f64,
    points: usize
) -> Result<GuassIntegralResult, String>
{
    // Substitute function
    let expr: Expr = match equation.parse() {
        Ok(e)  => e,
        Err(_) => return Err("Invalid function".to_string()),
    };

    // Fetch the "x" as (a+b) / 2 + (b-a) / 2 * zeta
    let subs_bound : f64      = (bound_most - bound_least) / 2.0;
    let mean_bound : f64      = (bound_most + bound_least) / 2.0; 
    let abscissas  : Vec<f64> = legendre_abscissas(points);
    let weight     : Vec<f64> = legendre_weight(points, abscissas.clone());

    let mut result = 0.0;
    for i in 0..points {
        let expression = mean_bound + subs_bound * abscissas[i];
        result += weight[i] * utils::evaluate_expr(&expr, expression);
    }
    result *= subs_bound;

    let error = utils::error_calc(true_result, result);

    Ok(GuassIntegralResult {
        true_result,
        result,
        error,
        abscissas: abscissas.clone(),
        weight: weight.clone()
    })
}



// Calculations

// Split function from #trapezodial_core
fn trapezodial_calc(
    expr: Expr,
    bound_least: f64, 
    bound_most: f64, 
    height: f64, 
) -> f64 {

    let mut result : f64 = 0.0;

    result += utils::evaluate_expr(&expr, bound_least);
    result += utils::evaluate_expr(&expr, bound_most);

    let mut inner_bound = bound_least + height;
    while inner_bound < bound_most {
        result += 2.0 * utils::evaluate_expr(&expr, inner_bound);
        inner_bound += height;
    }

    // trapezoidal calculation (h / 2 * sum)
    height / 2.0 * result
}

fn legendre_polynomial(points: usize, x: f64) -> f64 {

    let mut result: Vec<f64> = vec![0.0; points + 1];
    result[0] = 1.0;
    result[1] = x;

    for iter in 2..=points {
        result[iter] = (((2 * iter - 1) as f64 * x * result[iter - 1])
         - ((iter - 1) as f64 * result[iter - 2] )) / (iter as f64);
    }
    
    result[points]
}

fn legendre_polynomial_deriv(points: usize, x: f64) -> f64 {

    let mut result: Vec<f64> = vec![0.0; points + 1];
    result[0] = 1.0;
    result[1] = x;

    for iter in 2..=points {
        result[iter] = (((2 * iter - 1) as f64 * x * result[iter - 1])
         - ((iter - 1) as f64 * result[iter - 2] )) / (iter as f64);
    }

    let diff = points as f64 * (result[points - 1] - x * result[points]) / (1.0 - x * x);
    
    diff
}

/// [`Using Newton's method`]
/// Initial guess by Cosine mapping in form of Chebyshev Nodes [[0,π]] to [[-1,1]]
fn legendre_abscissas(points: usize) -> Vec<f64> {
    let mut result = vec![0.0; points];

    // Initial guess
    for i in 0..points {
        result[i] = ( PI * (4.0 * i as f64 + 3.0) / (4.0 * points as f64 + 2.0) ).cos();
    }

    for i in 0..points {
        let mut x: f64 = result[i];

        loop {
            let legendre_poly_value       : f64 = legendre_polynomial(points, x);
            let legendre_poly_deriv_value : f64 = legendre_polynomial_deriv(points, x);
            let x_new = x - legendre_poly_value / legendre_poly_deriv_value;

            if utils::error_calc(x_new, x) < 1e-12 {
                break;
            }

            x = x_new;
        }

        result[i] = x;
    }

    result
}

fn legendre_weight(points: usize, abscissas: Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0; points];

    for i in 0..points {
        let legendre_poly_deriv_value : f64 = legendre_polynomial_deriv(points, abscissas[i]);
        result[i] = 2.0 / ((1.0 - abscissas[i].powi(2)) * legendre_poly_deriv_value.powi(2));
    }

    result
}