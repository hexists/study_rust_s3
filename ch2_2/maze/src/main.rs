use rand::Rng;

fn main() {
    // 전체 미로 크기
    const MAP_N:usize = 25;
    let mut maze = [[0; MAP_N]; MAP_N];
    let mut rng = rand::thread_rng();

    // 미로 초기화
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[n][MAP_N-1] = 1;
        maze[0][n] = 1;
        maze[MAP_N-1][n] = 1;
    }

    // 미로 벽 배치
    for y in 2..(MAP_N-2) {
        for x in 2..(MAP_N-2) {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1;
            let r:i32 = rng.gen_range(0..=3);
            if r == 0 {
                maze[y-1][x] = 1
            }
            else if r == 1 {
                maze[y+1][x] = 1
            }
            else if r == 2 {
                maze[y][x-1] = 1
            }
            else if r == 3 {
                maze[y][x+1] = 1
            }
        }
    }

    // 미로 출력
    let tiles = ["⬜","⬛"];

    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!();
    }
}
