import java.util.List;
import java.util.Random;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        var x = Executors.newSingleThreadExecutor();
        System.out.println("Hello world!"+ System.currentTimeMillis());
        x.invokeAll(List.of(() -> {
            do {
                var y = 3.3;
                var co = new Random().doubles().findFirst().orElse(12.3);
                for (int i = 0; i < 100_000_00; i++) {
                    y = y * 3.8;
                    y /= 2.3;
                    y *= co;
                }
                System.out.println("---- " + y);
//            } while (true);
            } while (!Thread.interrupted());
            System.out.println("---- interrupted."+ System.currentTimeMillis());
            return null;
        }), 10, TimeUnit.SECONDS);

        System.out.println("first done."+ System.currentTimeMillis());
        Thread.sleep(1000);
        System.out.println("second task."+ System.currentTimeMillis());
        x.invokeAll(List.of(() -> {
                System.out.println("===== TEST second task."+ System.currentTimeMillis());
                return null;
            }), 5, TimeUnit.SECONDS)
            .forEach(g -> {
                try {
                    g.get();
                } catch (InterruptedException | ExecutionException e) {
                    throw new RuntimeException(e);
                }
            });
        System.out.println("done."+ System.currentTimeMillis());
    }
}