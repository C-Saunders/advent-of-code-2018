use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), String>  {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let (map, creatures) = parse(&input);

    Ok(())
}

fn evaluate_round(cave: &Cave, creatures: &Creatures) {
    // determine turn order
    // 
}

enum SquareType {
    Wall,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

type Cave = HashMap<Point, SquareType>;
type Creatures = HashMap<Point, Creature>; 

fn parse(input: &str) -> (Cave, Creatures, Creatures) {
    let mut cave: Cave = HashMap::new();
    let mut elves: Creatures = HashMap::new();
    let mut goblins: Creatures = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            let this_point = Point::new(x, y);

            let square_type = match symbol {
                '#' => SquareType::Wall,
                '.' => SquareType::Empty,
                'E' => {
                    elves.insert(this_point.clone(), Creature::new(CreatureType::Elf));
                    SquareType::Empty
                },
                'G' => {
                    goblins.insert(this_point.clone(), Creature::new(CreatureType::Goblin));
                    SquareType::Empty
                },
                _ => panic!(""),
            };

            cave.insert(Point::new(x, y), square_type);
        }
    }

    (cave, elves, goblins)
}

enum CreatureType {
    Elf,
    Goblin,
}

struct Creature {
    creature_type: CreatureType,
    attack_power: usize,
    hit_points: usize,
}

impl Creature {
    fn new(creature_type: CreatureType) -> Self {
        Creature {
            creature_type,
            attack_power: 3,
            hit_points: 200,
        }
    }
}

