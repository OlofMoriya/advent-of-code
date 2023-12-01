use itertools::Itertools;

#[derive(Debug,Clone)]
struct Tree {
    height: i32,
    visible: bool,
}

pub fn solve() -> String {
    let string = include_str!("../../input/22_08");
    let lines = string.lines();

    let mut matrix:Vec<Vec<Tree>> = vec!();


    let mut max_line = vec![-1;lines.clone().last().unwrap().len()];
    let mut max_line_bottom = max_line.clone();


    for line in lines {
        let line:Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let new_line: Vec<Tree> = line.iter().enumerate().map(|(i,d)| Tree{
            height:*d,
            visible: 
                line[0..i].iter().all(|f| f < d) || 
                line[i+1..].iter().all(|f| f < d) || 
                d > &max_line[i],
        }).collect();
        matrix.push(new_line.clone());
        max_line = max_line.iter().enumerate().map(|(i,d)| *(d.max(&new_line[i].height))).collect();
    }

    // Check from the bottom up which could not be done until all rows had been mapped out.
    for row in (0..matrix.len()).rev() {
        for i in 0..matrix[row].len() {
            matrix[row][i].visible = matrix[row][i].visible || matrix[row][i].height > max_line_bottom[i];
        }
        max_line_bottom = max_line_bottom.iter().enumerate().map(|(i,d)| *(d.max(&matrix[row][i].height))).collect();
    }

    return format!("{:#?}", matrix.iter().flatten().filter(|t| t.visible).count());
}

pub fn solve_two() -> String {
    let string = include_str!("../../input/22_08");
    let lines = string.lines();

    let mut matrix:Vec<Vec<Tree>> = vec!();
   
    for line in lines {
        let new_line = line
                    .chars()
                    .map(|c| Tree{
                        height:c.to_digit(10).unwrap() as i32,
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
                if matrix[i][column].height >= matrix[row][column].height {
                    break;
                }
            } 
            let prod = left * right * top * bottom;
            max = max.max(prod);
        }
    }


    return format!("{}", max);
}
