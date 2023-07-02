package org.mengdou.domain;

/**
 * @author mengdou at 2023/6/7 10:29
 */
public enum JoinType {
    /**
     * inner or natural join
     */
    INNER,
    CROSS_PRODUCT,
    LEFT,
    RIGHT,
    FULL,
    LEFT_SEMI,
    LEFT_ANTI
}
