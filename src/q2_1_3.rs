mod q2_1_3 {
    use std::collections::VecDeque;

    pub fn solve(n: i32, m: i32, maze_plot: Vec<Vec<&str>>) -> i32 {
        let mut maze = Maze::new(n, m, maze_plot);

        for i in 0..n {
            for j in 0..m {
                if "S" == maze.maze[i as usize][j as usize] {
                    return maze.bfs(i, j);
                }
            }
        }
        0
    }

    struct Maze<'a> {
        nsize: i32,
        msize: i32,
        maze: Vec<Vec<&'a str>>,
    }

    impl Maze<'_> {
        pub fn new(n: i32, m: i32, maze: Vec<Vec<&str>>) -> Maze {
            Maze {
                nsize: n,
                msize: m,
                maze,
            }
        }

        pub fn bfs(&mut self, x: i32, y: i32) -> i32 {
            let directions = [[-1, 0], [0, -1], [0, 1], [1, 0]];
            let mut queue = VecDeque::new();
            queue.push_front([x, y, 1]);

            while queue.len() > 0 {
                let current = queue.pop_back().unwrap();
                for direction in directions.iter() {
                    let nx = current[0] + direction[0];
                    let ny = current[1] + direction[1];
                    if 0 <= nx && nx < self.nsize && 0 <= ny && ny < self.msize {
                        if self.maze[nx as usize][ny as usize] == "." {
                            self.maze[nx as usize][ny as usize] = "+";
                            queue.push_front([nx, ny, current[2] + 1]);
                        }
                        if self.maze[nx as usize][ny as usize] == "G" {
                            return current[2];
                        }
                    }
                }
            }
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::q2_1_3::*;

    #[test]
    fn test() {
        let n = 10;
        let m = 10;
        let maze = vec![
            vec!["#", "S", "#", "#", "#", "#", "#", "#", ".", "#"],
            vec![".", ".", ".", ".", ".", ".", "#", ".", ".", "#"],
            vec![".", "#", ".", "#", "#", ".", "#", "#", ".", "#"],
            vec![".", "#", ".", ".", ".", ".", ".", ".", ".", "."],
            vec!["#", "#", ".", "#", "#", ".", "#", "#", "#", "#"],
            vec![".", ".", ".", ".", "#", ".", ".", ".", ".", "#"],
            vec![".", "#", "#", "#", "#", "#", "#", "#", ".", "#"],
            vec![".", ".", ".", ".", "#", ".", ".", ".", ".", "."],
            vec![".", "#", "#", "#", "#", ".", "#", "#", "#", "."],
            vec![".", ".", ".", ".", "#", ".", ".", ".", "G", "#"],
        ];
        assert_eq!(solve(n, m, maze), 22);
    }
}
