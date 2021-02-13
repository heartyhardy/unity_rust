

/*
    First of all we specify that there should be no name mangling for this function

    Mangling happens when Complier changes the function name by altering it with additional information which is eventually
    used by the compilation process. We want to keep this name as it is so we specify #[no_mangle]
 
    We also need to specify that other languages(in our case C#) should be able to call the functions.
    using extern keyword we create an interface for other languages to do so.

    Read more about unsafe functions here.
     https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
*/


// Circular array generation

/* We plot the points of a cirlce, specified by the radius r,
    r sin(theta) + Y coordinate of the origin of the circle gets us the
    X coordinate of a point on the circle.

    r cos(theta) + X coordinate of the origin gives us the Y coordinate of a point on the circle

    therefore we get (X, Y) for given r and theta(0-360) = a complete circle!
*/


// Unity

/*
    Within Unity we can create all the necessary points via a for loop (there are many ways to do this)
    to generate an array of points on the circle.

    Then we can snap instantiated objects to these points, making a circular array of a given object type!

*/

/*
    Take note of the cargo.toml file to how to build this to a DLL
    once completed, build the project using:
        cargo build --release
    
    Go to your target -> release folder and copy the .dll file (in this case unityrust_interop.dll) to
    Unity's asset folder.

    Create a C# script and test out the DLL.
    I've included a sample unity project to test this in the repo
*/

#[no_mangle]
pub extern fn get_x(origin_y: f32, r: f32, theta: f32) -> f32{

    return r * theta.sin() + origin_y; 
}

#[no_mangle]
pub extern fn get_y(origin_x: f32, r: f32, theta: f32) -> f32{
    return r * theta.cos() + origin_x; 
}