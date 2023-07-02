import java.util.Stack;

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 * int val;
 * TreeNode left;
 * TreeNode right;
 * TreeNode(int x) { val = x; }
 * }
 */
class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode(int x) {
        val = x;
    }
}

class Solution1 {

    /**
     * 两个栈模拟左右两个不同的指针（像#167一样的左右索引）
     * @param root
     * @param k
     * @return
     */
    public boolean findTarget(TreeNode root, int k) {
        if (root == null || root.left == null && root.right == null) {
            return false;
        }


        Stack<TreeNode> left = new Stack<>();
        pushMost(left, root, true);

        Stack<TreeNode> right = new Stack<>();
        pushMost(right, root, false);

        TreeNode i = left.pop();
        TreeNode j = right.pop();
        while (i != j) {
            int v = i.val + j.val;
            if (v == k) {
                return true;
            } else if (v < k) {
                if (i.right != null) {
                    pushMost(left, i.right, true);
                }
                i = left.pop();
            } else {
                if (j.left != null) {
                    pushMost(right, j.left, false);
                }
                j = right.pop();
            }
        }

        return false;
    }

    private void pushMost(Stack<TreeNode> stack, TreeNode one, boolean left) {
        TreeNode tmp = one;
        while (tmp != null) {
            stack.push(tmp);
            tmp = left ? tmp.left : tmp.right;
        }
    }

    public static void main(String[] args) {
        TreeNode t1 = new TreeNode(2);
        t1.left = new TreeNode(1);
        t1.right = new TreeNode(3);
        Solution1 solution1 = new Solution1();
        System.out.println(solution1.findTarget(t1, 1));
    }
}
