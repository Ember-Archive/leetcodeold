/* 
 * Author: Ember Hext
 * Problem: Median of Two Sorted Arrays
 * Link: https://leetcode.com/problems/median-of-two-sorted-arrays/
 * 
 * This solution uses a simple algorithm with two pointers
 * and a loop to find the median of two sorted arrays. It calculates
 * the middle of the two combined array lengths and iterates through
 * until it reaches that. Then it adjusts for if the total length is even
 * (and thus requires the average of two numbers to find the median).
 * 
 * This solution works in O(n) time.
 */

function findMedianSortedArrays(nums1: number[], nums2: number[]): number {
    let totalLength = nums1.length + nums2.length;
    let halfLength = Math.ceil(totalLength / 2);

    let firstIndex=0;
    let secondIndex=0;
    let currElement = 0;
    
    while (firstIndex + secondIndex < halfLength ) {
        if(nums2[secondIndex] == undefined || nums1[firstIndex] < nums2[secondIndex]) currElement = nums1[firstIndex++];
        else currElement = nums2[secondIndex++];
    }
    
    if(!(totalLength & 1)) {
        let nextElement = 0;
        if(nums2[secondIndex] == undefined || nums1[firstIndex] < nums2[secondIndex]) nextElement = nums1[firstIndex++];
        else nextElement = nums2[secondIndex++];
        return (nextElement + currElement) / 2;
    }

    return currElement;
};