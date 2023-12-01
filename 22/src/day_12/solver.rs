#[derive(Debug)]
struct QueueItem {
    point: (usize, usize),
    cost: usize,
    elevation: char,
}


pub fn solve() -> String {
    fn get_neighbouring_points(queue_item: QueueItem, size: (usize, usize), map:&Vec<Vec<char>>) -> Vec<QueueItem> {
        let mut points = vec!();
        let point = queue_item.point;
        if point.0 > 0 {
            let new_point = (point.0-1, point.1);
            points.push(QueueItem{cost: queue_item.cost + 1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 
        if point.0 < size.0-1 {
            let new_point = (point.0+1, point.1);
            points.push(QueueItem{cost: queue_item.cost +1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 
        if point.1 > 0 {
            let new_point = (point.0, point.1-1);
            points.push(QueueItem{cost: queue_item.cost +1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 
        if point.1 < size.1-1 {
            let new_point = (point.0, point.1+1);
            points.push(QueueItem{cost: queue_item.cost +1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 

        return points.into_iter().filter(|p| {
            let mut elevation = p.elevation;
            if elevation == 'S' { elevation = 'a';}
            if elevation == 'E' { elevation = 'z';}
            return (elevation as usize) - 1 <= (queue_item.elevation as usize);
        }).collect::<Vec<QueueItem>>();
    }

    let input = include_str!("../../input/22_12_test");
    let map: Vec<Vec<char>> = input.lines().map(|r| r.chars().to_owned().collect()).collect();

    let mut start = (0,0);
    let mut end = (0,0);

    for row in 0..map.len(){
        for column in 0..map[row].len(){
            print!("{}",map[row][column]);
            if map[row][column] == 'S' {
                start = (row, column);
            }
            else if map[row][column] == 'E' {
                end = (row, column);
            }
        }
        println!();
    }

    let size = (map.len(), map[0].len());
    let mut best_total_cost = 100000;
    let mut cost_map: Vec<Vec<usize>> = vec![vec![100000; map[0].len()]; map.len()];
    
    let mut queue = get_neighbouring_points(QueueItem{cost: 0, elevation:'a', point:start}, size, &map);
    while queue.len() > 0 {
        let item = queue.pop().unwrap();
        if cost_map[item.point.0][item.point.1] > item.cost {
            cost_map[item.point.0][item.point.1] = item.cost;
            if item.point == end {
                best_total_cost = best_total_cost.min(item.cost);
            } else {
                for new_item in get_neighbouring_points(item, size, &map) {
                    queue.insert(0, new_item);
                }           
            }
        }
    }

    return format!("{}, end at {:?}",best_total_cost, end);
}

pub fn solve_two() -> String {
    
    fn get_neighbouring_points(queue_item: QueueItem, size: (usize, usize), map:&Vec<Vec<char>>) -> Vec<QueueItem> {
        let mut points = vec!();
        let point = queue_item.point;
        let mut ref_elevation = queue_item.elevation;

        if ref_elevation == 'S' { ref_elevation = 'a';}
        if ref_elevation == 'E' { ref_elevation = 'z';}

        if point.0 > 0 {
            let new_point = (point.0-1, point.1);
            points.push(QueueItem{cost: queue_item.cost + 1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 
        if point.0 < size.0-1 {
            let new_point = (point.0+1, point.1);
            points.push(QueueItem{cost: queue_item.cost +1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 
        if point.1 > 0 {
            let new_point = (point.0, point.1-1);
            points.push(QueueItem{cost: queue_item.cost +1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 
        if point.1 < size.1-1 {
            let new_point = (point.0, point.1+1);
            points.push(QueueItem{cost: queue_item.cost +1, point:new_point, elevation:map[new_point.0][new_point.1]});
        } 

        return points.into_iter().filter(|p| {
            let mut elevation = p.elevation;
            if elevation == 'S' { elevation = 'a';}
            if elevation == 'E' { elevation = 'z';}
            return (elevation as usize) >= (ref_elevation as usize) - 1 && p.point != queue_item.point;
        }).collect::<Vec<QueueItem>>();
    }

    let input = include_str!("../../input/22_12");
    let map: Vec<Vec<char>> = input.lines().map(|r| r.chars().to_owned().collect()).collect();

    let mut start = (0,0);
    let mut end = (0,0);

    for row in 0..map.len(){
        for column in 0..map[row].len(){
            print!("{}",map[row][column]);
            if map[row][column] == 'S' {
                start = (row, column);
            }
            else if map[row][column] == 'E' {
                end = (row, column);
            }
        }
        println!();
    }

    let size = (map.len(), map[0].len());
    let mut best_total_cost = 100000;
    let mut cost_map: Vec<Vec<usize>> = vec![vec![100000; map[0].len()]; map.len()];
    
    let mut queue = get_neighbouring_points(QueueItem{cost: 0, elevation:'z', point:end}, size, &map, );
    println!("queue after first {:?}", queue);
    while queue.len() > 0 {
        let item = queue.pop().unwrap();

        if cost_map[item.point.0][item.point.1] > item.cost {
            cost_map[item.point.0][item.point.1] = item.cost;
            if item.elevation == 'a' {
                best_total_cost = best_total_cost.min(item.cost);
            } else {
                for new_item in get_neighbouring_points(item, size, &map) {
                    queue.insert(0, new_item);
                }
            }
        }
    }

    return format!("{}, end at {:?}",best_total_cost, end);
}

