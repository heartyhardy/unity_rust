mod lib;
use crate::lib::*;

fn main(){
    println!("Testing Circular Array generation");

    // We select the origin as (0,0)

    // Calculates R Sin(theta) = 5 Sin(0) = a X coordinate of a point in the circle
    println!("X coordinate for given R and Origin Y: {}", get_x(0., 5., 0.));

    // Calculates R Cos(theta) = 5 Cos(0) = a Y coordinate of a point in the circle
    println!("Y coordinate for given R and Origin X: {}", get_y(0., 5., 0.));
}