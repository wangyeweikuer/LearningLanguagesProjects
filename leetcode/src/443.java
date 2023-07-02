class Solution443 {
    public int compress(char[] chars) {
        int writeIndex = 0;
        for (int i = 0; i < chars.length; ) {
            char c = chars[i];
            int j = i + 1;
            for (; j < chars.length; j++) {
                if (c != chars[j]) {
                    break;
                }
            }
            int localCount = j - i;
            i = j;

            chars[writeIndex++] = c;
            if (localCount > 1) {
                int base = 1000;
                boolean notFirst = false;
                while (base > 0) {
                    int x = localCount / base;
                    localCount %= base;
                    base /= 10;
                    if (x > 0 || notFirst) {
                        chars[writeIndex++] = (char) (x + '0');
                        notFirst = true;//输出过之后，后续都要输出：["o","o","o","o","o","o","o","o","o","o"]
                    }
                }
            }
        }
        return writeIndex;
    }

    public static void main(String[] args) {
//        char[] chars = new char[]{'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o', 'o'};
        char[] chars = new char[]{'a','a','b','b','c','c','c'};
        int len = new Solution443().compress(chars);
        for (int i = 0; i < len; i++) {
            System.out.println(chars[i]);
        }
    }
}