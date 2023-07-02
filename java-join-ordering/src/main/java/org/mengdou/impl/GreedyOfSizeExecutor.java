package org.mengdou.impl;

import org.mengdou.Estimater;
import org.mengdou.OrderExecutor;
import org.mengdou.domain.*;

import java.util.Comparator;
import java.util.PriorityQueue;

/**
 * @author mengdou at 2023/6/7 10:24
 */
public class GreedyOfSizeExecutor extends OrderExecutor {

    public GreedyOfSizeExecutor(Estimater estimater) {
        super(estimater);
    }

    @Override
    public JoinTree order(MultiJoin multiJoin, int supportingFeatures) {
        PriorityQueue<Node> pq = new PriorityQueue<>(multiJoin.size(),
            Comparator.comparingDouble(o -> o.cardinality().value()));
        multiJoin
            .inputs()
            .stream()
            .map(i -> new LeafNode(idGenerator.incrementAndGet(), i, estimater.estimate(i)))
            .forEach(pq::add);
        while (pq.size() >= 2) {
            Node x1 = pq.poll();
            Node x2 = pq.poll();
            new BinaryJoinNode(idGenerator.incrementAndGet(), new Node[2]{x1,x2}, )
        }
    }
}
