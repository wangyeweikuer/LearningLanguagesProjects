package k8s;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

import java.io.IOException;
import java.io.OutputStream;
import java.net.Inet4Address;
import java.net.InetSocketAddress;
import java.net.InterfaceAddress;
import java.net.NetworkInterface;
import java.net.SocketException;
import java.util.Date;
import java.util.Enumeration;
import java.util.TreeSet;

/**
 * @author mengdou/ye.wangy
 * @version 28/8/2019 09:44
 * @since 2.6.9
 */
public class Demo {

    private static final int PORT = 8000;
    private static final TreeSet<String> addresses = new TreeSet<>();
    static {
        try {
            Enumeration<NetworkInterface> nis = NetworkInterface.getNetworkInterfaces();
            while (nis.hasMoreElements()) {
                NetworkInterface e = nis.nextElement();
                e.getInterfaceAddresses().stream()
                        .map(InterfaceAddress::getAddress)
                        .filter(Inet4Address.class::isInstance)
                        .map(Inet4Address.class::cast)
                        .map(Inet4Address::toString)
                        .forEach(addresses::add);
            }
        } catch (SocketException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) throws Exception {
        HttpServer server = HttpServer.create(new InetSocketAddress(PORT), 0);
        server.createContext("/test", new MyHandler());
        server.createContext("/", new MyHandler());
        server.setExecutor(null); // creates a default executor
        server.start();
        System.out.println("HTTP Server is started, listening at " + PORT);
    }

    static class MyHandler implements HttpHandler {

        @Override
        public void handle(HttpExchange t) throws IOException {
            String response = "Times: " + (new Date()) + ",IPs -> " + addresses + "\r\n";
            t.sendResponseHeaders(200, response.length());
            OutputStream os = t.getResponseBody();
            os.write(response.getBytes());
            os.close();
        }
    }
}
