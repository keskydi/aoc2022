use std::{fs, collections::{HashSet, VecDeque}, hash::Hash};

fn level_1(input: &str) -> u32 {
    let rows:HashSet<(usize,usize,usize)> = input.lines().map(|s| {
        let mut coords = s.split(",");
        let (Some(x),Some(y),Some(z)) = (coords.next(),coords.next(),coords.next()) else{
            panic!("format invalid");
        };
        (x.parse().unwrap(),y.parse().unwrap(),z.parse().unwrap())
    }).collect();

    let total_sides = 6 * rows.len();
    let mut connections = 0;


    for row in &rows{
        for (dx,dy,dz,fwd) in [(1,0,0,true),(1,0,0,false),(0,1,0,true),(0,1,0,false),(0,0,1,true),(0,0,1,false)]{
            let coord = if fwd{
                (row.0+dx,row.1+dy,row.2+dz)
            }else{
                (row.0-dx,row.1-dy,row.2-dz)
            };
            if rows.contains(&coord){
                connections+=1;
            }
        }
    }

    total_sides as u32 -connections
}

fn level_2(input: &str) -> u32 {
    // 0 is empty
    // 1 is lava
    // 2 is outside air
    let mut map = vec![vec![vec![0_u8;22];22];22];

    input.lines().for_each(|s| {
        let mut coords = s.split(",");
        let (Some(x),Some(y),Some(z)) = (coords.next(),coords.next(),coords.next()) else{
            panic!("format invalid");
        };
        map[x.parse::<usize>().unwrap()+1][y.parse::<usize>().unwrap()+1][z.parse::<usize>().unwrap()+1] = 1
    });

    let rows:HashSet<(usize,usize,usize)> = input.lines().map(|s| {
        let mut coords = s.split(",");
        let (Some(x),Some(y),Some(z)) = (coords.next(),coords.next(),coords.next()) else{
            panic!("format invalid");
        };
        (x.parse::<usize>().unwrap()+1,y.parse::<usize>().unwrap()+1,z.parse::<usize>().unwrap()+1)
    }).collect();

    for (x,y,z) in &rows{
        map[*x][*y][*z] = 1
    }

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0,0,0));

    while let Some(value) = queue.pop_front(){
        if seen.insert(value) && map[value.0][value.1][value.2] == 0 {
            map[value.0][value.1][value.2] = 2;
            for (dx,dy,dz,fwd) in [(1_i32,0_i32,0_i32,true),(1_i32,0_i32,0_i32,false),(0_i32,1_i32,0_i32,true),(0_i32,1_i32,0_i32,false),(0_i32,0_i32,1_i32,true),(0_i32,0_i32,1_i32,false)]{
                let coord = if fwd{
                    (value.0 as i32 +dx,value.1 as i32+dy,value.2 as i32+dz)
                }else{
                    (value.0 as i32-dx,value.1 as i32-dy,value.2 as i32-dz)
                };

                if coord.0 >= 0 && coord.1 >= 0 && coord.2 >= 0 && coord.0 < 22 && coord.1 < 22 && coord.2 < 22 {
                    queue.push_back((coord.0 as usize,coord.1 as usize,coord.2 as usize))
                }
            }
        }
    }

    let mut connections = 0;


    for row in &rows{
        for (dx,dy,dz,fwd) in [(1,0,0,true),(1,0,0,false),(0,1,0,true),(0,1,0,false),(0,0,1,true),(0,0,1,false)]{
            let coord = if fwd{
                (row.0+dx,row.1+dy,row.2+dz)
            }else{
                (row.0-dx,row.1-dy,row.2-dz)
            };
            if map[coord.0][coord.1][coord.2] == 2 {
                connections+=1;
            }
        }
    }

    connections
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/bin/day18/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
