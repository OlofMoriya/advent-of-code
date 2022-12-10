#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
}

pub fn solve() -> String {
    let string = include_str!("../../input/22_08");
    let lines = string.lines();

    let mut matrix:Vec<Vec<Tree>> = vec!();
//    let input = input_helper::read_input("input/22_08_test");
    for line in lines {
        let new_line = line
                    .chars()
                    .map(|c| Tree{
                        height:c.to_digit(10).unwrap(),
                        visible: false,

                    })
                    .collect::<Vec<Tree>>();

        matrix.push(new_line);
    }

    let mut max_top = vec![0; matrix[0].len()];
    for (row_index, line) in matrix.iter_mut().enumerate() {
       let mut max_left = 0;
       for (column_index, tree) in line.iter_mut().enumerate() {
            //un
            if row_index == 0 || column_index == 0 {
                tree.visible = true;
            }
            if tree.height > max_top[column_index] || tree.height > max_left {
                tree.visible = true;
            }

            max_top[column_index] = tree.height.max(max_top[column_index]);
            max_left = tree.height.max(max_left);
       }
       line.reverse();
    }
    matrix.reverse();
    

    let mut max_top = vec![0; matrix[0].len()];
    for (row_index, line) in matrix.iter_mut().enumerate() {
        let mut max_left = 0;
       for (column_index, tree) in line.iter_mut().enumerate() {
            //un
            if row_index == 0 || column_index == 0 || tree.height > max_top[column_index] || tree.height > max_left {
                tree.visible = true;
            }

            max_top[column_index] = tree.height.max(max_top[column_index]);
            max_left = tree.height.max(max_left);
       }
    }
    
    let visible = matrix.iter().flatten().filter(|t| t.visible).count();
    return format!("{}", visible);
}

pub fn solve_two() -> String {
    let string = include_str!("../../input/22_08");
    let lines = string.lines();

    let mut matrix:Vec<Vec<Tree>> = vec!();
   
    for line in lines {
        let new_line = line
                    .chars()
                    .map(|c| Tree{
                        height:c.to_digit(10).unwrap(),
                        visible: false,

                    })
                    .collect::<Vec<Tree>>();

        matrix.push(new_line);
    }
    let mut max = 0;
    for row in 0..matrix.len(){
        let row_ = &matrix[row];
        for column in 0..row_.len(){
            let mut left = 0;
            let mut right = 0;
            let mut top = 0;
            let mut bottom = 0;
            for i in (0..column).rev() {
                left += 1; 
                if matrix[row][i].height >= matrix[row][column].height {
                    break;
                }
            } 
            for i in column+1..matrix[row].len() {
                right += 1; 
                if matrix[row][i].height >= matrix[row][column].height {
                    break;
                }
            } 
            for i in (0..row).rev() {
                top += 1; 
                if matrix[i][column].height >= matrix[row][column].height {
                    break;
                }
            } 
            for i in row+1..matrix.len() {
                bottom += 1; 
                //println!("{row},{column}; looking for bottom: current h: {}, comparing to {i},{column} with {}", matrix[row][column].height, matrix[i][column].height);
                if matrix[i][column].height >= matrix[row][column].height {
                    break;
                }
            } 
            let prod = left * right * top * bottom;
            //println!("x,y {row},{column} has top:{top}, left:{left}, right:{right}, bottom:{bottom}");
            max = max.max(prod);
        }
    }


    return format!("{}", max);
}
