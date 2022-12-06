use std::fs;

fn level_1(input:&str){

    println!("level_1: {}", input.lines().filter_map(|x|{
        let (e,u) = x.split_once(" ").unwrap();
        Some(calc_score(e,u))
    }).sum::<usize>());

}

fn level_2(input:&str){

    println!("level_2: {}", input.lines().filter_map(|x|{
        let (e,u) = x.split_once(" ").unwrap();
        Some(cheat_score(e,u))
    }).sum::<usize>());

}

fn calc_score(him:&str,you:&str)->usize{
    // A for Rock, B for Paper, and C for Scissors
    let ennemy_hand = ["A","B","C"];
    // X for Rock, Y for Paper, and Z for Scissors
    let your_hand = ["X","Y","Z"];

    let position = your_hand.iter().position(|&x| x == you).unwrap();

    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    if &him == ennemy_hand.get(position).unwrap(){
        position + 4 
    }else if &him == ennemy_hand.get((position+1)%3).unwrap(){
        position + 1
    }else{
        position + 7
    }
}

fn cheat_score(him:&str,you:&str)->usize{
    // A for Rock, B for Paper, and C for Scissors
    let ennemy_hand = ["A","B","C"];
    // X for Lose, Y for Draw, and Z for Win

    let position = ennemy_hand.iter().position(|&x| x == him).unwrap();

    match you{
        "X"=> {
            (position+2)%3+1
        },
        "Y"=> {
            position + 4
        },
        _=> {// "Z"
            (position+1)%3 +7
        },
    }

}

#[test]
fn test_calc(){
 assert_eq!(calc_score("A","Y"),8);
 assert_eq!(calc_score("B","X"),1);
 assert_eq!(calc_score("C","Z"),6);
}

#[test]
fn test_cheat(){
 assert_eq!(cheat_score("A","Y"),4);
 assert_eq!(cheat_score("B","X"),1);
 assert_eq!(cheat_score("C","Z"),7);
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day02/input.in").expect("file not found");

    level_1(&input);
    level_2(&input);
    println!("time: {:?}", now.elapsed());
}