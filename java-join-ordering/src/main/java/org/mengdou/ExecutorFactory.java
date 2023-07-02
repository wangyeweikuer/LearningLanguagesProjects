package org.mengdou;

import org.mengdou.impl.DPSizeExecutor;
import org.mengdou.impl.GOOExecutor;
import org.mengdou.impl.GreedyOfSizeExecutor;

import javax.annotation.Nonnull;
import java.lang.reflect.InvocationTargetException;

/**
 * @author mengdou at 2023/6/7 09:55
 */
public class ExecutorFactory {
    public static OrderExecutor of(@Nonnull Algorithm algorithm, @Nonnull Estimater estimater) {
        var name = OrderExecutor.class.getPackage().getName() + ".impl." + algorithm + "Executor";
        try {
            var c = Class.forName(name);
            return (OrderExecutor) c.getDeclaredConstructor(Estimater.class).newInstance(estimater);
        } catch (ClassNotFoundException e) {
            throw new RuntimeException("Can't find an executor for algorithm:" + algorithm);
        } catch (InvocationTargetException | InstantiationException | IllegalAccessException |
                 NoSuchMethodException e) {
            throw new RuntimeException("Can't initialize the executor[" + name + "] for algorithm:" + algorithm);
        }
    }
}
