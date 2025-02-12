/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end],
    merge all overlapping intervals and return a list of non-overlapping intervals.

    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.

    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::cmp::max;
use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals

    // 克隆输入的区间列表，避免直接修改原始数据
    let mut tmp = intervals.clone();

    // 对区间按照起始点进行排序
    // 使用 `sort_by` 方法，传入一个闭包来比较两个区间的起始点
    tmp.sort_by(|a, b| a[0].cmp(&b[0]));

    // 初始化索引，用于遍历区间列表
    let mut index = 0;

    // 遍历区间列表，合并重叠的区间
    while index < tmp.len() - 1 {
        // 检查当前区间和下一个区间是否重叠
        // 如果下一个区间的起始点 <= 当前区间的结束点，说明有重叠
        if tmp[index + 1][0] <= tmp[index][1] {
            // 合并两个区间：更新当前区间的结束点为两个区间结束点的最大值
            tmp[index][1] = max(tmp[index][1], tmp[index + 1][1]);

            // 移除下一个区间，因为已经合并到当前区间
            tmp.remove(index + 1);
        } else {
            // 如果没有重叠，移动到下一个区间
            index += 1;
        }
    }

    // 返回合并后的区间列表
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![0, 4]]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![vec![1, 10], vec![2, 6], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 10]]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![4, 7], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![3, 7], vec![8, 10]]);
    }
}
