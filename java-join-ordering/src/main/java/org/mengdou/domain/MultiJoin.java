package org.mengdou.domain;

import com.google.common.collect.ImmutableList;

import java.util.List;

import static com.google.common.base.Preconditions.checkArgument;

/**
 * @param connections connections.value() == inputs.value() -1 inputs.get(0) -- connections.get(0) -- inputs.get(1) --
 *                    connections.get(0) ...... inputs.get(inputs.value()-1)
 * @author mengdou at 2023/6/7 19:46
 */
public record MultiJoin(List<Input> inputs, List<Connection> connections) {

    public MultiJoin(List<Input> inputs, List<Connection> connections) {
        checkArgument(inputs.size() > 2);
        checkArgument(inputs.size() == connections.size() + 1);
        this.inputs = ImmutableList.copyOf(inputs);
        this.connections = ImmutableList.copyOf(connections);
    }

    public int size() {
        return inputs.size();
    }
}
