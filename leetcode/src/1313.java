import java.util.Arrays;

class Solution {
    public int[] decompressRLElist(int[] nums) {
        //不用arraylist，长度可以预先计算，避免arraylist的动态分配
//        List<Integer> integers = new ArrayList<>();
        int len = 0;
        for (int i = 0; i < nums.length; i += 2) {
            len += nums[i];
        }
        int[] results = new int[len];

        int k = 0;
        for (int i = 0; i < nums.length; i += 2) {
            int a = nums[i];
            int b = nums[i + 1];


            //不知道为啥，用了这个静态函数，就会快一些（社区也有类似的），不知道是不是跟CPU指令某些地方相关
            Arrays.fill(results, k, k + a, b);
            k += a;

//            for (int j = 0; j < a; j++) {
//                results[k++] = b;
//            }

        }
        return results;
    }
}