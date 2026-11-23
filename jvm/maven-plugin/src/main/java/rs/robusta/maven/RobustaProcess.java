package rs.robusta.maven;

import java.io.IOException;
import java.util.concurrent.locks.Lock;

/** Holds the invocation's execution slot until the native process exits. */
final class RobustaProcess {
    private RobustaProcess() {}

    static int run(ProcessBuilder builder, Lock lock) throws IOException, InterruptedException {
        lock.lockInterruptibly();
        try {
            Process process = builder.start();
            try {
                return process.waitFor();
            } catch (InterruptedException interrupted) {
                process.destroyForcibly();
                // Keep the slot until termination, even if cancellation interrupts cleanup again.
                while (true) {
                    try {
                        process.waitFor();
                        break;
                    } catch (InterruptedException ignored) {
                        // The caller restores the interrupt after cleanup.
                    }
                }
                throw interrupted;
            }
        } finally {
            lock.unlock();
        }
    }
}
