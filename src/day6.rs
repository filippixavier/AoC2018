extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use self::regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    upper_right: Option<(i32, i32)>,
    upper_left: Option<(i32, i32)>,
    lower_right: Option<(i32, i32)>,
    lower_left: Option<(i32, i32)>,
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("./data/day6.txt"))?;
    let reg = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let mut points = HashMap::<(i32, i32), Point>::new();
    let mut areas = HashMap::<(i32, i32), i32>::new();
    let mut map = HashMap::<(i32, i32), String>::new();
    
    // Our map and a list of discovered location
    for capture in reg.captures_iter(&input) {
        let (col, line) = (capture.get(1).unwrap().as_str().parse::<i32>().unwrap(), capture.get(2).unwrap().as_str().parse::<i32>().unwrap());

        let mut new_pt = Point{x: col, y: line, upper_right: None, upper_left: None, lower_right: None, lower_left: None};

        for pt in points.values_mut() {
            new_pt.upper_left = if pt.x <= new_pt.x && pt.y <= new_pt.y {
                match pt.lower_right {
                    Some((proxy_x, proxy_y)) => {
                        let man = (new_pt.x - pt.x).abs() + (new_pt.y - pt.y).abs();
                        let proxy_man = (pt.x - proxy_x).abs() + (pt.y - proxy_y).abs();

                        if man < proxy_man {
                            pt.lower_right = Some((new_pt.x, new_pt.y));
                        }

                        match new_pt.upper_left {
                            Some((prev_x, prev_y)) => {
                                let prev_man = (new_pt.x - prev_x).abs() + (new_pt.y - prev_y).abs();
                                if prev_man > man {
                                    Some((pt.x, pt.y))
                                } else {
                                    new_pt.upper_left
                                }
                            }
                            None => Some((pt.x, pt.y)),
                        }
                    },
                    None => {
                        pt.lower_right = Some((new_pt.x, new_pt.y));
                        Some((pt.x, pt.y))
                    }
                }
            } else {
                new_pt.upper_left
            };
            new_pt.upper_right = if pt.x >= new_pt.x && pt.y <= new_pt.y {
                match pt.lower_left {
                    Some((proxy_x, proxy_y)) => {
                        let man = (new_pt.x - pt.x).abs() + (new_pt.y - pt.y).abs();
                        let proxy_man = (pt.x - proxy_x).abs() + (pt.y - proxy_y).abs();

                        if man < proxy_man {
                            pt.lower_left = Some((new_pt.x, new_pt.y));
                        }

                        match new_pt.upper_right {
                            Some((prev_x, prev_y)) => {
                                let prev_man = (new_pt.x - prev_x).abs() + (new_pt.y - prev_y).abs();
                                if prev_man > man {
                                    Some((pt.x, pt.y))
                                } else {
                                    new_pt.upper_right
                                }
                            }
                            None => Some((pt.x, pt.y)),
                        }
                    },
                    None => {
                        pt.lower_left = Some((new_pt.x, new_pt.y));
                        Some((pt.x, pt.y))
                    }
                }
            } else {
                new_pt.upper_right
            };
            new_pt.lower_left = if pt.x <= new_pt.x && pt.y >= new_pt.y {
                match pt.upper_right {
                    Some((proxy_x, proxy_y)) => {
                        let man = (new_pt.x - pt.x).abs() + (new_pt.y - pt.y).abs();
                        let proxy_man = (pt.x - proxy_x).abs() + (pt.y - proxy_y).abs();
                        if man < proxy_man {
                            pt.upper_right = Some((new_pt.x, new_pt.y));
                        }
                        match new_pt.lower_left {
                            Some((prev_x, prev_y)) => {
                                let prev_man = (new_pt.x - prev_x).abs() + (new_pt.y - prev_y).abs();
                                if prev_man > man {
                                    Some((pt.x, pt.y))
                                } else {
                                    new_pt.lower_left
                                }
                            }
                            None => Some((pt.x, pt.y)),
                        }
                    },
                    None => {
                        pt.upper_right = Some((new_pt.x, new_pt.y));
                        Some((pt.x, pt.y))
                    }
                }
            } else {
                new_pt.lower_left
            };
            new_pt.lower_right = if pt.x >= new_pt.x && pt.y >= new_pt.y {
                match pt.upper_left {
                    Some((proxy_x, proxy_y)) => {
                        let man = (new_pt.x - pt.x).abs() + (new_pt.y - pt.y).abs();
                        let proxy_man = (pt.x - proxy_x).abs() + (pt.y - proxy_y).abs();

                        if man < proxy_man {
                            pt.upper_left = Some((new_pt.x, new_pt.y));
                        }
                        match new_pt.lower_right {
                            Some((prev_x, prev_y)) => {
                                let prev_man = (new_pt.x - prev_x).abs() + (new_pt.y - prev_y).abs();
                                if prev_man > man {
                                    Some((pt.x, pt.y))
                                } else {
                                    new_pt.lower_right
                                }
                            }
                            None => Some((pt.x, pt.y)),
                        }
                    },
                    None => {
                        pt.upper_left = Some((new_pt.x, new_pt.y));
                        Some((pt.x, pt.y))
                    }
                }
            } else {
                new_pt.lower_right
            };
        }

        points.insert((new_pt.x, new_pt.y), new_pt);
   }


    println!("Biggest safe area: {}", 1);

    Ok(())
}

fn name(dir: &mut Option<(i32, i32)>, opposite: &mut Option<(i32, i32)>, pt_x: i32, pt_y: i32, new_x: i32, new_y: i32) -> Option<(i32, i32)> {
    let (x, y) = match dir {
        Some((x, y)) => (*x, *y),
        None => (0, 0),
    };

    if dir.is_none() {
        *opposite = Some((new_x, new_y));
        Some((pt_x, pt_y))
    } else {
        
    }

    /*match dir {
        Some((proxy_x, proxy_y)) => {
            
        },
        None => {
            *opposite = Some((new_x, new_y));
            Some((pt_x, pt_y))
        }
    }*/
}


pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}