extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use self::regex::Regex;

#[derive(Debug)]
struct Source {
    area: i32,
    infinite: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    coord: (i32, i32),
    dist: i32,
    owner: Option<(i32, i32)>,
}

/**
 * Algo first_star
 * Init
 *     Liste: liste de point Source
 *     map: tableau avec point Source placé
 *     file: file avec point Source
 * Début
 *     Tant que file non vide
 *         Soit point:  premier élem de file
 *         Soit pN: voisins de point tel que
 *             pN.coord = point.coord + (0|1, 0|1)
 *             pN.dist = point.dist + 1
 *             pN.owner = point.owner
 *         pour chaque voisin: claim_cell(liste, map, file, point, pN)
 *      Fin Tant Que
 *      soit max_area = 0
 *      Pour chaque source de liste
 *         Si source.area > max_area
 *             max_area = source_area
 *         fin Si
 *      fin Pour
 *      Retourn max_area
 * Fin
 */
pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("../data/day6.txt"))?;
    let reg = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let mut sources_list = HashMap::<(i32, i32), Source>::new();
    let mut map = HashMap::<(i32, i32), Point>::new();
    let mut heap = Vec::<Point>::new();

    for capture in reg.captures_iter(&input) {
        let (x, y) = (
            capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        );
        let source_point = Point {
            coord: (x, y),
            dist: 0,
            owner: Some((x, y)),
        };
        sources_list.insert(
            (x, y),
            Source {
                area: 1,
                infinite: false,
            },
        );
        map.insert((x, y), source_point);
        heap.insert(0, source_point);
    }

    while !heap.is_empty() {
        let p_start = heap.pop().unwrap();
        claim_cell(
            &mut sources_list,
            &mut map,
            &mut heap,
            p_start,
            Point {
                coord: (p_start.coord.0 + 1, p_start.coord.1),
                dist: p_start.dist + 1,
                owner: p_start.owner,
            },
        );
        claim_cell(
            &mut sources_list,
            &mut map,
            &mut heap,
            p_start,
            Point {
                coord: (p_start.coord.0 - 1, p_start.coord.1),
                dist: p_start.dist + 1,
                owner: p_start.owner,
            },
        );

        claim_cell(
            &mut sources_list,
            &mut map,
            &mut heap,
            p_start,
            Point {
                coord: (p_start.coord.0, p_start.coord.1 + 1),
                dist: p_start.dist + 1,
                owner: p_start.owner,
            },
        );
        claim_cell(
            &mut sources_list,
            &mut map,
            &mut heap,
            p_start,
            Point {
                coord: (p_start.coord.0, p_start.coord.1 - 1),
                dist: p_start.dist + 1,
                owner: p_start.owner,
            },
        );
    }

    let mut max_area = 0;

    for source in sources_list.values() {
        if max_area < source.area && !source.infinite {
            max_area = source.area;
        }
    }

    println!("Minimal area: {}", max_area);

    Ok(())
}

/**
 * Algo claim_cell
 * Init
 *     liste: liste de point Source
 *     map: tableau de point
 *     file: file de point a placer
 *     point: point précédent
 *     pN: point à vérifier
 *
 * Debut
 *     Si !is_infinite(point, p[1..4], liste)
 *         Si map[pN.coord] existe ET map[pN.coord].owner != pN.owner  
 *             Si pN.dist < map[pN.coord].dist
 *                 list[map[pN.coord].owner].area --
 *                 list[pN].area ++
 *                 file.push(pN)
 *                 map[pN.coord] = pN
 *             Sinon Si pN.dist == map[pN].dist ET pN.owner != map[pN].owner
 *                 Si map[pN.coord].owner
 *                     list[map[pN.coord].owner].area --
 *                     map[pN.coord].owner = NULL
 *                 fin Si
 *             fin Si/Sinon
 *         Sinon
 *             map[pN.coord] = pN
 *             liste[pN.owner].area ++
 *             file.push(pN)
 *         fin Si/Sinon
 *     fin Si
 * Fin
 */

fn claim_cell(
    sources_list: &mut HashMap<(i32, i32), Source>,
    map: &mut HashMap<(i32, i32), Point>,
    heap: &mut Vec<Point>,
    p_start: Point,
    p_end: Point,
) {
    if !is_infinite(p_start.coord, p_end.coord, sources_list) {
        let (coord, point) = match map.get_mut(&p_end.coord) {
            Some(p_claim) => {
                if p_end.dist < p_claim.dist {
                    if let Some(owner) = p_claim.owner {
                        if let Some(source) = sources_list.get_mut(&owner) {
                            source.area -= 1;
                        }
                    }
                    if let Some(source) = sources_list.get_mut(&p_end.owner.unwrap()) {
                        source.area += 1;
                    }
                    heap.insert(0, p_end);
                    (p_end.coord, p_end)
                } else if p_end.dist == p_claim.dist
                    && p_claim.owner.is_some()
                    && p_claim.owner.unwrap() != p_end.owner.unwrap()
                {
                    if let Some(source) = sources_list.get_mut(&p_claim.owner.unwrap()) {
                        source.area -= 1;
                    }
                    (
                        p_end.coord,
                        Point {
                            coord: p_end.coord,
                            dist: p_end.dist,
                            owner: None,
                        },
                    )
                } else {
                    (p_claim.coord, *p_claim)
                }
            }
            None => {
                heap.insert(0, p_end);
                if let Some(source) = sources_list.get_mut(&p_end.owner.unwrap()) {
                    source.area += 1;
                }
                (p_end.coord, p_end)
            }
        };
        map.insert(coord, point);
    } else if let Some(source) = sources_list.get_mut(&p_start.owner.unwrap()) {
        source.infinite = true;
    }
}

/**
 * Algo is_infinite
 * Init
 *     pS: point de départ
 *     pE: point d'arrivée
 *     liste: liste des points Sources
 * Debut
 *     soit infinity: vrai
 *     Pour chaque point p de liste
 *         Si distance entre pS et p est supérieur à distance entre pE et p
 *             infinity = false
 *             sortie de boucle
 *         fin Si
 *     fin Pour
 *     retourne not(infinity)
 * Fin
 */
fn is_infinite(
    p_start: (i32, i32),
    p_end: (i32, i32),
    sources: &HashMap<(i32, i32), Source>,
) -> bool {
    sources.keys().all(|&p_source| {
        let old_dist = (p_start.0 - p_source.0).abs() + (p_start.1 - p_source.1).abs();
        let new_dist = (p_end.0 - p_source.0).abs() + (p_end.1 - p_source.1).abs();
        new_dist > old_dist
    })
}

const MAX_MANHATTAN_DIST: i32 = 10000;

/**
 * Algo second_star
 * Init
 *     min_x: plus petite abscisse parmis les Sources
 *     min_y: plus petite ordonnée parmis les Sources
 *     max_x: plus grande abscisse parmis les Sources
 *     max_y: plus grande ordonnée parmis les Sources
 *     sources: liste de sources
 * Debut
 *     Soit area := 0 surface initial
 *     Pour tout x de min_x a max_x
 *         Pour tout y de min_y a max_y
 *             si valid_cell
 *                 area++;
 *     fin Pour
 *     Retourner area
 * Fin
 */
pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let input = fs::read_to_string(Path::new("../data/day6.txt"))?;
    let reg = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let mut sources = Vec::<(i32, i32)>::new();

    let mut area = 0;
    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (0, 0, MAX_MANHATTAN_DIST, MAX_MANHATTAN_DIST);

    for capture in reg.captures_iter(&input) {
        let (x, y) = (
            capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        );
        sources.push((x, y));
        min_y = if min_y > y { y } else { min_y };

        min_x = if min_x > x { x } else { min_x };

        max_y = if max_y < y { y } else { max_y };

        max_x = if max_x < x { y } else { max_x };
    }

    for x in min_x..max_x {
        for y in min_y..max_y {
            if valid_cell((x, y), &sources) {
                area += 1;
            }
        }
    }

    println!("Valid area: {}", area);

    Ok(())
}

/**
 * Algo valid_cell
 * Init
 *     pt: point à comparer
 *     sources: liste de points
 * Debut
 *     soit total := 0
 *     Pour chaques point de sources
 *         total += (abs(pt.x - point.x) + abs(pt.y - point.y))
 *         Si total > MAX
 *             retourne faux
 *         fin Si
 *     fin Pour
 *     retourne vrai
 * Fin
 */

fn valid_cell(pt: (i32, i32), sources: &[(i32, i32)]) -> bool {
    let mut total = 0;
    for source in sources {
        total += (pt.0 - source.0).abs() + (pt.1 - source.1).abs();
        if total > MAX_MANHATTAN_DIST {
            return false;
        }
    }
    true
}
