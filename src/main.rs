//Define Position Data Structure
struct Position {
    x: f32,
    y: f32,
}

//Define Velocity Data Structure
struct Velocity {
    vx: f32,
    vy: f32,
}

// Define Object Data Structure
struct Object {
    mass: f32,
    radius: f32,
    position: Position,
    velocity: Velocity,
}

// Define some constants
const TIME_STEP: f32 = 0.1;
const COEF_OF_RESTIT: f32 = -0.9;

fn main() {
    // Define mutable variable named "balls" of the vector (of objects) type, and initialize it with the value returned by the 'new' function associated with the Vec type
    let mut balls: Vec<Object> = Vec::new();

    // Add an Object instance to the Object vector
    balls.push(Object {
        mass: 1.0,
        radius: 10.0,
        position: Position { x: -10.0, y: 0.0 },
        velocity: Velocity {
            vx: 10.0,
            vy: 0.0,
        },
    });
    balls.push(Object {
        mass: 1.0,
        radius: 1.0,
        position: Position { x: 10.0, y: 0.0 },
        velocity: Velocity {
            vx: -10.0,
            vy: 0.0,
        },
    });

    // Main Simulation loop
    for _ in 0..10 {
        // I like to move it move it
        for ball in &mut balls {
            ball.position.x += ball.velocity.vx * TIME_STEP;
            ball.position.y += ball.velocity.vy * TIME_STEP;
        }

        // Collision detection
        fn check_collisions(ball1: &Object, ball2: &Object) -> bool {
            // Distance Components
            let dx: f32 = ball2.position.x - ball1.position.x;
            let dy: f32 = ball2.position.y - ball1.position.y;

            // Oh great pythagorus
            let distance_squared = dx * dx + dy * dy;

            // Min Distance Calc (sort of, if I never sqroot the distance ^^^, to make this equivalent i just have to square the sum of the radii. this seems less computationally expensive)
            let radius_sum_squared = (ball1.radius + ball2.radius).powi(2);

            //Check for collision
            distance_squared < radius_sum_squared
        }

        // Start loop that iterates over all indicies inside of "balls" vector (len() method returns length of vector)
        for i in 0..balls.len() {
            // Starts a loop that runs once per iteration that acts on i+1, this means that it checks the current iteration, and every one that hasnt been checked with it. if that makes sense.
            for j in (i + 1)..balls.len() {
                if check_collisions(&balls[i], &balls[j]) {
                    println!("THEY ARE TOUCHING EWWWWWWWW");

                    // basic collisons for now: just reverse velocities of the 2 balls
                    balls[i].velocity.vx *= COEF_OF_RESTIT;
                    balls[i].velocity.vy *= COEF_OF_RESTIT;
                    balls[j].velocity.vx *= COEF_OF_RESTIT;
                    balls[j].velocity.vy *= COEF_OF_RESTIT;
                }
            }
        }
        for (index, ball) in balls.iter().enumerate() {
            println!(
                "Ball {}: Position ({}, {})",
                index, ball.position.x, ball.position.y
            );
        }
    }
}
