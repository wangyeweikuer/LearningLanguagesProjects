package org.mengdou.domain;

/**
 * @author mengdou at 2023/6/7 10:42
 */
public class JoinTree {
    private BinaryJoinNode root;

    public String print() {
        StringBuilder sw = new StringBuilder();
        print(root, 0, sw);
        return sw.toString();
    }

    public static void print(Node node, int depth, StringBuilder sw) {
        sw.append("  ".repeat(Math.max(0, depth)));
        sw.append(node.id());
        if (node instanceof LeafNode n) {
            sw.append(",name=").append(n.name()).append(",cardinality=").append(n.cardinality()).append('\n');
        } else {
            sw.append('\n');
            var h = (BinaryJoinNode) node;
            print(h.left(), depth + 1, sw);
            print(h.right(), depth + 1, sw);
        }
    }
}
