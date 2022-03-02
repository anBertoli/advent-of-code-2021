use std::cmp::{max, min};

// Input: target area: x=85..145, y=-163..-108
const TARGET_AREA_X: (i32, i32) = (85, 145);
const TARGET_AREA_Y: (i32, i32) = (-163, -108);

fn main() {
    println!("Day 17!");
    println!("Part 1: {:?}", find_highest_trajectory(TARGET_AREA_Y));
    println!(
        "Part 2: {:?}",
        find_possible_velocities(TARGET_AREA_X, TARGET_AREA_Y).len()
    );
}

fn find_highest_trajectory((yt1, _): (i32, i32)) -> i32 {
    // Note that the y velocity when we return at the
    // baseline (y=0) is -(yv + 1).
    //
    // Bigger the starting yv velocity, higher the peak.
    // But we have a limit: we need to make sure the next
    // step will not surpass the target area.
    let max_yv = yt1.abs() - 1;
    let y_peak = max_yv * (max_yv + 1) / 2;
    y_peak
}

fn find_possible_velocities((xt1, xt2): (i32, i32), (yt1, yt2): (i32, i32)) -> Vec<(i32, i32)> {
    let mut valid_velocities = vec![];

    // The highest possible velocity is equal to xt2. In
    // this case in one step we will reach the border of
    // the target area.
    for x in 0..=xt2 {
        // The highest possible velocity is equal to yt1 - 1, while
        // the lower value is -yt1.abs() itself. In both cases we
        // will reach the border of the target area in one step.
        for y in -yt1.abs()..=yt1.abs() - 1 {
            if will_fall_in_target((xt1, xt2), (yt1, yt2), x, y) {
                valid_velocities.push((x, y));
            }
        }
    }

    valid_velocities
}

fn will_fall_in_target((xt1, xt2): (i32, i32), (yt1, yt2): (i32, i32), x: i32, y: i32) -> bool {
    for t in 0.. {
        let (x_pos, y_pos) = calc_pos((x, y), t);
        if y_pos >= yt1 && y_pos <= yt2 && x_pos >= xt1 && x_pos <= xt2 {
            return true;
        }
        if y_pos < yt1 {
            // Target surpassed.
            break;
        }
    }
    false
}

fn calc_pos((xv, yv): (i32, i32), t: i32) -> (i32, i32) {
    let x_pos = min(t, xv) * (xv + (max(xv - t, 0) + 1)) / 2;
    let y_pos = t * (yv + (yv - t + 1)) / 2;
    (x_pos, y_pos)
}
