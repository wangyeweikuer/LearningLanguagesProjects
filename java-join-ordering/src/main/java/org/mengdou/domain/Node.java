package org.mengdou.domain;

import javax.annotation.Nonnegative;

/**
 * @author mengdou at 2023/6/7 10:16
 */
public interface Node {
    int id();

    Cardinality cardinality();

}
