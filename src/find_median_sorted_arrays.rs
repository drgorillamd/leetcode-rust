/*
    find the median of two sorted arrays, eg [1, 3] and [2] should return 2.0

    time complexity should be O(log(m+n)
    -> divide & conquer (alternative is brute-force, either merge the arrays O(m+n) or merge up until the median, O(k))

    walkthrough: https://www.youtube.com/watch?v=LPFhl65R7ww
*/
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // nums1 is the smaller array, for simplicity
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1);
    }


    if nums1.is_empty() && nums2.is_empty() {
        return 0.0;
    }

    if nums1.is_empty() {
        match nums2.len() % 2 {
            0 => return (nums2[nums2.len() / 2] + nums2[nums2.len() / 2 - 1]) as f64 / 2.0,
            _ => return nums2[nums2.len() / 2] as f64,
        }
    }

    // We will divide both array into 2 parts, the nb of left elements == nb right elements (or +1 if odd)
    let mut start = 0;
    let mut end = nums1.len();

    let total_length = nums1.len() + nums2.len();

    while end >= start {

        let nums1_pivot = (start + end) / 2;
        let nums2_pivot = (total_length + 1) / 2 - nums1_pivot;

        let max_left_nums1 = if nums1_pivot == 0 { std::i32::MIN } else { nums1[nums1_pivot - 1] };
        let min_right_nums1 = if nums1_pivot >= nums1.len() { std::i32::MAX } else { nums1[nums1_pivot] };

        let max_left_nums2 = if nums2_pivot == 0 { std::i32::MIN } else { nums2[nums2_pivot - 1] };
        let min_right_nums2 = if nums2_pivot >= nums2.len() { std::i32::MAX } else { nums2[nums2_pivot] };

        // We are at the median if:
        // 1. max left nums1 <= min right nums2
        // 2. max left nums2 <= min right nums1
        // (as the left_ right_ indexes are the last or first element of the partition, and the partitions are sorted, they
        // are the max and min of the partition respectively)
        
        // The median is then:
        // 1. if total_length is odd: max(max left nums1, max left nums2)
        // 2. if total_length is even: (max(max left nums1, max left nums2) + min(min right nums1, min right nums2)) / 2

        // if not, we need to move the partition to the left or right -> if max left nums1 > min right nums2, we need to move the partition to the left in nums1 (and vice versa)
        match (max_left_nums1 <= min_right_nums2, max_left_nums2 <= min_right_nums1) {
            (true, true) => {
                // We are at the median
                match total_length % 2 {
                    0 => {
                        let max_left = std::cmp::max(max_left_nums1, max_left_nums2);
                        let min_right = std::cmp::min(min_right_nums1, min_right_nums2);
                        return (max_left + min_right) as f64 / 2.0;
                    },
                    _ => return std::cmp::max(max_left_nums1, max_left_nums2) as f64,
                }
            },
            (false, _) => {
                // max left num1 > min right nums2: we need to move the partition to the left
                end -= 1;
            },
            (_, _) => {
                // max left nums1 > min right nums2: we need to move the partition to the right
                start += 1;
            },
        }
    }

    panic!("Unsorted arrays");
}


#[test]
fn test_one() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
}

#[test]
fn test_two() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
}

#[test]
fn test_three() {
    let nums1 = vec![0, 0];
    let nums2 = vec![0, 0];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.0);
}

#[test]
fn test_four() {
    let nums1 = vec![];
    let nums2 = vec![1];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.0);
}

#[test]
fn test_five() {
    let nums1 = vec![2];
    let nums2 = vec![];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
}

#[test]
fn test_six() {
    let nums1 = vec![3, 4];
    let nums2 = vec![1, 2];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
}