package org.mengdou.domain;

/**
 * @author mengdou at 2023/6/7 09:48
 */
public record LeafNode(int id, Input input, Cardinality cardinality) implements Node {
}
