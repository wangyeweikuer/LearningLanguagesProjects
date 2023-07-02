package org.mengdou.impl;

import org.mengdou.Estimater;
import org.mengdou.OrderExecutor;
import org.mengdou.domain.JoinTree;
import org.mengdou.domain.MultiJoin;

/**
 * @author mengdou at 2023/6/7 10:24
 */
public class DPHypExecutor extends OrderExecutor {

    public DPHypExecutor(Estimater estimater) {
        super(estimater);
    }

    @Override
    public JoinTree order(MultiJoin multiJoin, int supportingFeatures) {
        return null;
    }
}
