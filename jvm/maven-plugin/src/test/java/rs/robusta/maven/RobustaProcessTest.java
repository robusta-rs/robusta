package rs.robusta.maven;

import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicReference;
import java.util.concurrent.locks.ReentrantLock;
import java.util.function.BooleanSupplier;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import rs.robusta.plugin.common.RobustaPlatform;

@Timeout(30)
class RobustaProcessTest {
    @TempDir Path temporaryDirectory;

    @ParameterizedTest
    @ValueSource(ints = {0, 1, 2})
    void releasesTheSlotAfterEveryExitCode(int exitCode) throws Exception {
        ReentrantLock lock = new ReentrantLock();
        assertEquals(exitCode, RobustaProcess.run(builder("--exit=" + exitCode), lock));
        assertFalse(lock.isLocked());
        assertEquals(0, RobustaProcess.run(builder(), lock));
    }

    @Test
    void releasesTheSlotWhenStartingFails() throws Exception {
        ReentrantLock lock = new ReentrantLock();
        assertThrows(IOException.class, () -> RobustaProcess.run(
                new ProcessBuilder(temporaryDirectory.resolve("missing-executable").toString()), lock));
        assertFalse(lock.isLocked());
        assertEquals(0, RobustaProcess.run(builder(), lock));
    }

    @Test
    void interruptionWhileWaitingDoesNotLaunchOrReleaseAnotherOwnersSlot() throws Exception {
        ReentrantLock lock = new ReentrantLock();
        AtomicReference<Throwable> failure = new AtomicReference<>();
        Path marker = temporaryDirectory.resolve("unexpected");
        Thread worker = worker(builder("--record=" + marker), lock, failure);
        lock.lock();
        try {
            worker.start();
            await(() -> lock.hasQueuedThread(worker));
            worker.interrupt();
            worker.join(5000);
            assertFalse(worker.isAlive());
            assertInstanceOf(InterruptedException.class, failure.get());
            assertTrue(lock.isHeldByCurrentThread());
            assertFalse(Files.exists(marker));
        } finally {
            lock.unlock();
            worker.interrupt();
            worker.join(5000);
        }
        assertEquals(0, RobustaProcess.run(builder(), lock));
    }

    @Test
    void interruptionTerminatesTheChildBeforeTheNextProcessStarts() throws Exception {
        Path activity = Files.createDirectory(temporaryDirectory.resolve("activity"));
        ReentrantLock lock = new ReentrantLock();
        AtomicReference<Throwable> firstFailure = new AtomicReference<>();
        AtomicReference<Throwable> secondFailure = new AtomicReference<>();
        Thread first = worker(builder("--activity=" + activity, "--await-release"), lock, firstFailure);
        Thread second = null;
        ProcessHandle child = null;
        try {
            first.start();
            await(() -> Files.exists(activity.resolve("waiting")));
            long pid = Long.parseLong(Files.readAllLines(activity.resolve("events")).get(0).split(" ")[1]);
            child = ProcessHandle.of(pid).orElseThrow();
            ProcessHandle runningChild = child;
            Path marker = temporaryDirectory.resolve("next-process");
            second = new Thread(() -> {
                try {
                    lock.lockInterruptibly();
                    try {
                        assertFalse(runningChild.isAlive(), "the cancelled child still owns native work");
                        assertEquals(0, RobustaProcess.run(builder("--record=" + marker), lock));
                    } finally {
                        lock.unlock();
                    }
                } catch (Throwable failure) {
                    secondFailure.set(failure);
                }
            });
            second.start();
            Thread queued = second;
            await(() -> lock.hasQueuedThread(queued));
            first.interrupt();
            first.join(5000);
            second.join(5000);
            assertFalse(first.isAlive());
            assertFalse(second.isAlive());
            assertInstanceOf(InterruptedException.class, firstFailure.get());
            assertNull(secondFailure.get());
            assertFalse(child.isAlive());
            assertTrue(Files.exists(marker));
            assertFalse(lock.isLocked());
        } finally {
            if (child != null && child.isAlive()) child.destroyForcibly();
            first.interrupt();
            first.join(5000);
            if (second != null) {
                second.interrupt();
                second.join(5000);
            }
        }
    }

    private Thread worker(ProcessBuilder builder, ReentrantLock lock, AtomicReference<Throwable> failure) {
        return new Thread(() -> {
            try {
                RobustaProcess.run(builder, lock);
            } catch (Throwable error) {
                failure.set(error);
            }
        });
    }

    private ProcessBuilder builder(String... arguments) throws Exception {
        RobustaPlatform platform =
                RobustaPlatform.detect(System.getProperty("os.name"), System.getProperty("os.arch"));
        Path fake = Path.of(System.getProperty("robusta.fake.bundle"))
                .resolve("META-INF/robusta/bin")
                .resolve(platform.bundledBinaryName());
        List<String> command = new ArrayList<>(List.of(fake.toString()));
        command.addAll(List.of(arguments));
        // The fake reads a behavior file from its working directory or any ancestor.
        return new ProcessBuilder(command).directory(temporaryDirectory.toFile()).inheritIO();
    }

    private static void await(BooleanSupplier condition) throws Exception {
        long deadline = System.nanoTime() + Duration.ofSeconds(10).toNanos();
        while (!condition.getAsBoolean()) {
            assertTrue(System.nanoTime() < deadline, "timed out waiting for process state");
            Thread.sleep(10);
        }
    }
}
