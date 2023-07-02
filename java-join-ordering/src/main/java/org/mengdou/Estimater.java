package org.mengdou;

import org.mengdou.domain.*;

/**
 * @author mengdou at 2023/6/7 09:56
 */
public interface Estimater {
    Cardinality estimate(Node node);

    Cardinality estimate(Input input);

    Selectivity estimate(Expression expression);
}
