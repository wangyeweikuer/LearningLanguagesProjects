import java.util.Arrays;

class Solution167 {
    /**
     * n*log(n)算法，比暴力稍微快一点
     */
    public int[] twoSum2(int[] numbers, int target) {
        for (int i = 0; i < numbers.length; i++) {
            int minus = target - numbers[i];
            int h = Arrays.binarySearch(numbers, i + 1, numbers.length, minus);
            if (h >= 0) {
                return new int[]{i + 1, h + 1};
            }
        }
        return null;
    }

    /**
     * 做一个反值的数组
     *
     * @param numbers
     * @param target
     * @return
     */
    public int[] twoSum3(int[] numbers, int target) {
        int len = numbers.length;
        int[] num2 = new int[len];
        for (int i = 0; i < len; i++) {
            num2[i] = target - numbers[i];
        }

        int i = 0, j = len - 1;
        while (i < len && j >= 0) {
            int x = numbers[i];
            int y = num2[j];
            if (x == y) {
                return new int[]{Math.min(i + 1, j + 1), Math.max(i + 1, j + 1)};
            } else if (x > y) {
                j--;
            } else {
                i++;
            }
        }
        return null;
    }

    public int[] twoSum4(int[] numbers, int target) {
        int len = numbers.length;
        int i = 0, j = len - 1;
        while (i < len && j >= 0) {
            int x = numbers[i];
            int y = target - numbers[j];
            if (x == y) {
                return new int[]{Math.min(i + 1, j + 1), Math.max(i + 1, j + 1)};
            } else if (x > y) {
                j--;
            } else {
                i++;
            }
        }
        return null;
    }

    public int[] twoSum(int[] numbers, int target) {
        int len = numbers.length;
        int i = 0, j = len - 1;
        while (i < j) {
            int x = numbers[i];
            int y = target - numbers[j];
            if (x == y) {
                return new int[]{i + 1, j + 1};
            } else if (x > y) {
                j--;
            } else {
                i++;
            }
        }
        return null;
    }
}
