package org.mengdou.domain;

import java.util.List;

/**
 * @author mengdou at 2023/6/7 10:15
 */
public record BinaryJoinNode(int id, Node[] nodes, Connection connection, Cardinality cardinality) implements Node {
}
