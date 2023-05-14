use std::io::{BufReader, BufRead};
use std::fs::File;

mod genmatrix;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: u16,
    y: u16
}

struct PointRange{
    bump: bool,
    to: Point,
    pos: Point
}

impl PointRange {
    fn new(from: Point, to: Point) -> Self {
        PointRange { to: to, pos: from, bump: false }
    }
}

impl Iterator for PointRange {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {

        if self.bump {
            return None;
        } else {

            let res = Some(self.pos.clone());

            if self.pos == self.to {
                self.bump = true;
            }
            
            self.pos.x = self.pos.x + ((self.pos.x < self.to.x) as u16) - ((self.pos.x > self.to.x) as u16);
            self.pos.y = self.pos.y + ((self.pos.y < self.to.y) as u16) - ((self.pos.y > self.to.y) as u16);
            
            return res;
        }
    }
}

fn main() {

    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut pairs: Vec<(Point, Point)> = vec![];

    let mut max_x = 0;
    let mut max_y = 0;
    
    for line in lines.map(|l| l.unwrap()) {

        let mut points: Vec<Point> = line.split(" -> ").map(|combo| {
            let c: Vec<u16> = combo.split(',').map(|coord| coord.parse::<u16>().unwrap()).collect();
            return Point{x: c[0], y: c[1]};
            }).collect();
        
        max_x = std::cmp::max(max_x, points[0].x);
        max_x = std::cmp::max(max_x, points[1].x);

        max_y = std::cmp::max(max_y, points[0].y);
        max_y = std::cmp::max(max_y, points[1].y);

        pairs.push((points.pop().unwrap(), points.pop().unwrap()));
    }

    let mut matrix: genmatrix::GenMatrix<u16>= genmatrix::GenMatrix::from_default((max_y as usize + 1, max_x as usize + 1), 0);
    let mut count = 0;

    for (from, to) in pairs {
        for p in PointRange::new(from, to) {

            let e = matrix[p.y as usize][p.x as usize];
    
            if e == 1 {
                count += 1;
            }
    
            matrix.set_element((p.y as usize, p.x as usize), e + 1);
        }
    }   

    println!("Number of double elements {}", count);
}