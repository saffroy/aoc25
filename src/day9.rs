#![allow(clippy::needless_range_loop)]

use itertools::{iproduct, Itertools};
use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

fn parse_text(text: &str) -> Vec<(i64, i64)> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(",")
             .map(|s| s.parse::<i64>().unwrap())
             .collect_tuple() // from Itertools
             .unwrap())
        .collect()
}

pub fn parse_1(text: &str) -> i64 {
    let points = parse_text(text);
    let n = points.len();
    let mut area = 0;

    for i in 0..n {
        for j in i+1..n {
            let a = ((points[i].0 - points[j].0).abs() + 1)
                * ((points[i].1 - points[j].1).abs() + 1);
            area = max(area, a);
        }
    }

    area
}

// "Coordinate compression":
// from a list of (possibly large) int values, create a compact
// mapping (HashMap) to smaller, consecutive ints, and also a reverse
// mapping (Vec) back to the original values
fn remap_numbers(coords: &[i64]) -> (HashMap<i64, usize>, Vec<i64>) {
    let set: HashSet<i64> = coords.iter().cloned().collect();
    let rev: Vec<i64> = set.iter().cloned().sorted().collect();
    let map: HashMap<i64, usize> =
        rev.iter().cloned().enumerate().map(|(i, n)| (n, i)).collect();

    (map, rev)
}

fn mark_edge(mark_grid: &mut [Vec<bool>], a: (usize, usize), b: (usize, usize)) {
    let (xa, ya) = a;
    let (xb, yb) = b;
    if ya == yb {
        // horizontal
        let xmin = min(xa, xb);
        let xmax = max(xa, xb);
        for x in xmin..=xmax {
            mark_grid[x][ya] = true;
        }
    } else {
        // vertical
        let ymin = min(ya, yb);
        let ymax = max(ya, yb);
        for y in ymin..=ymax {
            mark_grid[xa][y] = true;
        }
    }
}

fn neighbours(xmax: usize, ymax: usize, (x, y): (usize, usize))
              -> impl Iterator<Item = (usize, usize)> {
    let xs = vec![x.overflowing_sub(1).0, x, x+1];
    let ys = vec![y.overflowing_sub(1).0, y, y+1];
    iproduct!(xs.into_iter(), ys.into_iter())
        .filter(move |&(xn, yn)| xn <= xmax && yn <= ymax && (xn, yn) != (x, y))
}

fn write_grid_image(mark_grid: &[Vec<bool>],
                    col: image::Rgb<u8>,
                    name: &str) {
    let mut imgbuf = image::ImageBuffer::new(mark_grid.len() as u32,
                                             mark_grid[0].len() as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if mark_grid[x as usize][y as usize] {
            *pixel = col;
        }
    }
    imgbuf.save(name).unwrap();
}

fn render_grid(remapped_points: &[(usize, usize)], xmax: usize, ymax: usize)
               -> Vec<Vec<bool>> {
    // start with all points unmarked
    let mut mark_grid: Vec<Vec<bool>> = vec![vec![false; ymax+1]; xmax+1];
    // mark points on the edges between points
    let n = remapped_points.len();
    for i in 0..n {
        mark_edge(&mut mark_grid,
                  remapped_points[i],
                  remapped_points[(i+1) % n]);
    }

    // println!("{:?}", mark_grid);
    write_grid_image(&mark_grid, image::Rgb([0, 0, 255]), "9-edges.png");

    // iteratively grow the set of def_outside points:
    // - start from outer edge of the grid: initial fringe
    // - compute next fringe: set of new neighbours that are unmarked
    // - merge prev fridge into def_outside
    let mut def_outside: HashSet<(usize, usize)> = HashSet::new(); // empty
    let mut fringe: HashSet<(usize, usize)> =
        // definitely outside:
        // unmarked points that are on the outer edge of the grid
        iproduct!(0..=xmax, 0..=ymax)
        .filter(|&(x, y)| x == 0 || x == xmax || y == 0 || y == ymax)
        .filter(|&(x, y)| !mark_grid[x][y])
        .collect();
    loop {
        let next_fringe: HashSet<(usize, usize)> =
            // neighbours of fringe points that are unmarked and not
            // in def_outside yet
            fringe
            .iter()
            .flat_map(|&(x, y)| neighbours(xmax, ymax, (x, y)))
            .filter(|&(x, y)| !mark_grid[x][y] && !def_outside.contains(&(x, y)))
            .collect();
        if next_fringe.is_empty() {
            break;
        } else {
            fringe.iter().for_each(|&(x, y)| {
               def_outside.insert((x, y));
            });
            fringe = next_fringe;
        }
    }
    // mark points not in def_outside as being inside
    iproduct!(0..=xmax, 0..=ymax)
        .for_each(|(x, y)| {
            if !mark_grid[x][y] && !def_outside.contains(&(x, y)) {
                mark_grid[x][y] = true;
            }
        });

    // println!("{:?}", mark_grid);
    write_grid_image(&mark_grid, image::Rgb([0, 255, 0]), "9-filled.png");

    mark_grid
}

pub fn parse_2(text: &str) -> i64 {
    let points = parse_text(text);
    let n = points.len();

    // compute "compressed" coordinates, equivalent to original
    // coordinates, but within much smaller ranges
    let xcoords: Vec<i64> = points.iter().map(|&p| p.0).collect();
    let ycoords: Vec<i64> = points.iter().map(|&p| p.1).collect();
    let (xmap, xrev) = remap_numbers(&xcoords);
    let (ymap, yrev) = remap_numbers(&ycoords);
    // println!("{:?}", xrev);
    // println!("{:?}", yrev);

    let remapped_points: Vec<(usize, usize)> = points
        .iter()
        .map(|&(x, y)| (xmap[&x], ymap[&y]))
        .collect();
    // println!("{:?}", remapped_points);

    // draw grid of places inside/outside the points loop, in compressed
    // coordinates
    let xmax = xrev.len() - 1;
    let ymax = yrev.len() - 1;
    let mark_grid = render_grid(&remapped_points, xmax, ymax);

    // find valid rectangles in compressed coordinates,
    // then compute max area in original space
    let mut area = 0;

    for i in 0..n {
        for j in i+1..n {
            let (xa, ya) = remapped_points[i];
            let (xb, yb) = remapped_points[j];

            // rectangle is valid if all points inside are inside the loop
            if (min(xa, xb)..=max(xa, xb))
                .flat_map(|x| &mark_grid[x][min(ya, yb)..=max(ya, yb)])
                .all(|&b| b) {
                    let a = ((points[i].0 - points[j].0).abs() + 1)
                        * ((points[i].1 - points[j].1).abs() + 1);
                    area = max(area, a);
                }
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    #[test]
    fn test9_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 50);
    }
    #[test]
    fn test9_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 24);
    }
}
