use rand::prelude::*;
use std::io;


struct Pit {
    x: i8,
    y: i8,
}
fn main() {
    println!("Enter your move: (WASD), to fire arrows: (IJKL)");
    let mut arrows: u8 = 2;
    let mut i_quit: bool = false;
    let mut points = 0;
    let mut px: i8 = 0;// player's initial position
    let mut py: i8 = 0;
    let mut wy = rand::thread_rng().gen_range(-10..=10); // the wumpus.
    let mut wx = rand::thread_rng().gen_range(-10..=10);
    if i_quit == true {
        println!("something to add later.");
    }
    //println!("{} {}", wx, wy);

    let mut pit1 = Pit {
        x: rand::thread_rng().gen_range(-10..=10),
        y: rand::thread_rng().gen_range(-10..=10),
    };
    let mut pit2 = Pit {
        x: rand::thread_rng().gen_range(-10..=10),
        y: rand::thread_rng().gen_range(-10..=10),
    };
    let mut pit3 = Pit {
        x: rand::thread_rng().gen_range(-10..=10),
        y: rand::thread_rng().gen_range(-10..=10),
    };
    let mut pit4 = Pit {
        x: rand::thread_rng().gen_range(-10..=10),
        y: rand::thread_rng().gen_range(-10..=10),
    };
    for pit in [&mut pit1, &mut pit2, &mut pit3, &mut pit4].iter_mut() {
        while pit.x == px && pit.y == py {
            pit.x = rand::thread_rng().gen_range(-10..=10);
            pit.y = rand::thread_rng().gen_range(-10..=10);
        }
    }

    loop {
        if wy == py && wx == px {
            wy = rand::thread_rng().gen_range(-10..=10);
            wx = rand::thread_rng().gen_range(-10..=10);
        } else {
            break;
        }
    }   

    loop {
        i_quit = dangers(&mut wx, &mut wy, py, px, [&pit1, &pit2, &pit3, &pit4]);
        if i_quit == true {
            break; 
        }
        i_quit = move_player(&mut px, &mut py, &mut points, &mut arrows, &mut wx, &mut wy); // this also detects and handles collisions with the walls, checks for invalid input, and checks if the user would like to quit.
        if i_quit == true {
            break; 
        }
        points += 1;
        print!("({}) ", points);
    }    
}
fn move_player(px: &mut i8, py: &mut i8, points: &mut i32, arrows: &mut u8, wx: &mut i8, wy: &mut i8) -> bool  { 
    println!("x: {}, y: {}", *px, *py);
    let pxs: i8 = *px;
    let pys: i8 = *py;
    let mut i_quit: bool = false;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input == "w" {
        *py += 1;
    } else if input == "s" {
        *py -= 1;
    } else if input == "a" {
        *px -= 1;
    } else if input == "d" {
        *px += 1;
    } else {
        if input == "q" {
            println!("goodbye!");
            i_quit = true;
        } else {
            if input == "i" || input == "j" || input == "k" || input == "l" {
                i_quit = shoot(arrows, *py, *px, *wy, *wx, input);
                println!("you have {} arrows left", *arrows);
            } else {
                    println!("you should only write notes when you are truly sure you don't want to go somewhere.");
                    move_player(px, py, points, arrows, wx, wy);   
            }

        }    
    }
    if px.abs() > 10 {
        println!("you bumped into the wall");
        *px = pxs;
        *points -= 1;
    }
    if py.abs() > 10 {
        println!("you bumped into the wall");
        *py = pys;
        *points -= 1;
    } 
    i_quit
}
fn dangers(wx: &mut i8, wy: &mut i8, py: i8, px: i8, pits: [&Pit; 4],) -> bool {
    let mut quit: bool = false;
    // warnings:
    if //wumpus is nearby:
    (px == *wx + 1 && py == *wy) ||
    (px == *wx - 1 && py == *wy) ||
    (py == *wy + 1 && px == *wx) ||
    (py == *wy - 1 && px == *wx)   
    {
        println!("you smell something unpleasant...");
    }
    // bottomless pit:
    for pit in pits {
        if     
        (px == pit.x + 1 && py == pit.y) ||
        (px == pit.x - 1 && py == pit.y) ||
        (py == pit.y + 1 && px == pit.x) ||
        (py == pit.y - 1 && px == pit.x) {
            println!("you feel a draft...");
        }
    }
    // die/treasure:
    for pit in pits {
        if pit.x == px && pit.y == py {
            println!("you fell into a bottomless pit!");
            quit = true;
        }
        if pit.x == *wx && pit.y == *wy {
            if rand::random() {
                *wx += 1;
            } else {
                *wy += 1;
            }
        }
    }
    if *wx == px && *wy == py { // wumpus
        if rand::random() {
            println!("you hear footsteps shuffling away...");
            if rand::random() {
                *wx += rand::thread_rng().gen_range(-3..=2) + 1;
            } else {
                *wy += rand::thread_rng().gen_range(-3..=2) + 1;
            } 
            if  
            (px == *wx + 1 && py == *wy) ||
            (px == *wx - 1 && py == *wy) ||
            (py == *wy + 1 && px == *wx) ||
            (py == *wy - 1 && px == *wx)   
            {
                println!("you smell something unpleasant...");
            }  
        } else {
            println!("the wumpus has eaten you! it thinks you're very tasty.");
            quit = true;
        }
    }
    quit
}
fn shoot(arrows: &mut u8, py: i8, px: i8, wy: i8, wx: i8, input: &str) -> bool {
    let mut done = false;
    if input == "k" {
        if py == wy + 1 && px == wx {
            println!("congratulations! you shot the wumpus!");
            done = true;
        } else {
            println!("you missed the wumpus.");
            *arrows -= 1;
            if *arrows == 0 {
                println!("you have no arrows left. game over.");
                done = true;
            }
        }    
    }
    if input == "j" {
        if px == wx + 1 && py == wy {
            println!("congratulations! you shot the wumpus!");
            done = true;
        } else {
            println!("you missed the wumpus.");
            *arrows -= 1;
            if *arrows == 0 {
                println!("you have no arrows left. game over.");
                done = true;
            }
        }
    }
    if input == "l" {
        if px == wx - 1 && py == wy {
            println!("congratulations! you shot the wumpus!");
            done = true;
        } else {
            println!("you missed the wumpus.");
            *arrows -= 1;
            if *arrows == 0 {
                println!("you have no arrows left. game over.");
                done = true;
            }
        }
    }
    if input == "i" {
        if py == wy - 1 && px == wx {
            println!("congratulations! you shot the wumpus!");
            done = true;
        } else {
            println!("you missed the wumpus.");
            *arrows -= 1;
            if *arrows == 0 {
                println!("you have no arrows left. game over.");
                done = true;
            }
        }    
       
    }
    done
}
