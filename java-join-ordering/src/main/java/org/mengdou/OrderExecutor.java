package org.mengdou;

import org.mengdou.domain.JoinTree;
import org.mengdou.domain.MultiJoin;

import java.util.concurrent.atomic.AtomicInteger;

/**
 * @author mengdou at 2023/6/7 09:47
 */
public abstract class OrderExecutor {
    interface SupportingFeatures {
        int CrossProduct = 1;
        int LeftDeep = 2;
        int RightDeep = 4;
        int ZigZag = 8;
        int Bushy = 16;
        int Multiway = 32;
    }

    protected final Estimater estimater;
    protected final AtomicInteger idGenerator;

    public OrderExecutor(Estimater estimater) {
        this.estimater = estimater;
        this.idGenerator = new AtomicInteger();
    }

    public abstract JoinTree order(MultiJoin multiJoin, int supportingFeatures);
}
