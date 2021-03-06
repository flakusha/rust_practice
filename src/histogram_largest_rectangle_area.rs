/// Given n non-negative integers representing the histogram's bar height where
/// the width of each bar is 1, find the area of largest rectangle
/// in the histogram
 
pub fn histogram_largest_area (bars: Vec<i32>) -> i32 {

struct Solution;

#[derive(Debug)]
struct BarSerie {
    h: i32,
    l: i32,
}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.len() == 0 {return 0i32;}
        else if heights.len() == 1 {return heights[0];}

        let mut index: usize = 0;
        let mut bar_series: Vec<BarSerie> = vec![];
        let h_len = heights.len();
        // Do not check bars second time
        let mut index_ignore = vec![];

        loop {
            let bar: i32 = heights[index];
            let index_scan = index;
            let mut ser: i32 = 1;
            if index_ignore.contains(&index_scan) {}
            else {
                // Steps to the left and to the right from current index
                let mut index_scan_left = index_scan;
                let mut index_scan_right = index_scan;
                // Is scan finished?
                let mut scan_left = false;
                let mut scan_right = false;
                
                loop {
                    // Scan to the left from current index
                    loop {
                        if index_scan_left == 0 {scan_left = true; break;}
                        else if heights[index_scan_left - 1] == bar {
                            ser += 1;
                            index_scan_left -= 1;
                            index_ignore.push(index_scan_left);
                        }
                        else if heights[index_scan_left - 1] > bar {
                            ser += 1;
                            index_scan_left -= 1;
                        }
                        else {scan_left = true;}

                        if scan_left {break;}

                    }
                    // Scan to the right from the current index
                    loop {
                        if index_scan_right >= h_len - 1
                        {scan_right = true; break;}
                        else if heights[index_scan_right + 1] == bar {
                            ser += 1;
                            index_scan_right += 1;
                            index_ignore.push(index_scan_right);
                        }
                        else if heights[index_scan_right + 1] > bar {
                            ser += 1;
                            index_scan_right += 1;
                        }
                        else {scan_right = true;}

                        if scan_right {break;}
                    }

                    if scan_left && scan_right {
                        bar_series.push(BarSerie{h: bar, l: ser,});
                        break;
                    }
                    else {continue;}
                }
            }
            if index >= h_len - 1 {
                break;
            } 
            else {
                index += 1;
            }
        }
        let mut areas: Vec<i32> = vec![];
        for serie in bar_series.iter() {
            areas.push(serie.h * serie.l);
        }
        areas.sort();
        let result = areas[areas.len() - 1];
        result
    }
}
let result = Solution::largest_rectangle_area(bars);
result
}