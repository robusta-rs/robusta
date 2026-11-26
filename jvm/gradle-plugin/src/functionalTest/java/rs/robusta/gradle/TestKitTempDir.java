package rs.robusta.gradle;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Stream;

import org.junit.jupiter.api.extension.BeforeEachCallback;
import org.junit.jupiter.api.extension.ExtendWith;
import org.junit.jupiter.api.extension.ExtensionContext;
import org.junit.platform.commons.support.AnnotationSupport;
import org.junit.platform.commons.support.ReflectionSupport;

/**
 * Like {@code @TempDir}, but retries deletion while a TestKit daemon still holds files in the
 * directory: a build returns before its daemon closes the project's caches, and Windows cannot
 * delete an open file.
 */
@Target(ElementType.FIELD)
@Retention(RetentionPolicy.RUNTIME)
@ExtendWith(TestKitTempDir.Extension.class)
@interface TestKitTempDir {
    final class Extension implements BeforeEachCallback {
        private static final ExtensionContext.Namespace NAMESPACE =
                ExtensionContext.Namespace.create(Extension.class);

        @Override
        public void beforeEach(ExtensionContext context) throws Exception {
            for (Object instance : context.getRequiredTestInstances().getAllInstances()) {
                List<Field> fields = AnnotationSupport.findAnnotatedFields(
                        instance.getClass(), TestKitTempDir.class);
                for (Field field : fields) {
                    Path directory = Files.createTempDirectory("junit-");
                    context.getStore(NAMESPACE).put(field, new DeletedDirectory(directory));
                    ReflectionSupport.makeAccessible(field).set(instance, directory);
                }
            }
        }
    }

    final class DeletedDirectory implements AutoCloseable {
        private static final Duration TIMEOUT = Duration.ofSeconds(10);
        private static final Duration RETRY_INTERVAL = Duration.ofMillis(100);

        private final Path directory;

        DeletedDirectory(Path directory) {
            this.directory = directory;
        }

        @Override
        public void close() throws IOException, InterruptedException {
            long deadline = System.nanoTime() + TIMEOUT.toNanos();
            while (true) {
                try {
                    delete();
                    return;
                } catch (IOException failure) {
                    if (System.nanoTime() - deadline >= 0) {
                        throw failure;
                    }
                    Thread.sleep(RETRY_INTERVAL.toMillis());
                }
            }
        }

        private void delete() throws IOException {
            try (Stream<Path> paths = Files.walk(directory)) {
                for (Path path : paths.sorted(Comparator.reverseOrder()).toList()) {
                    Files.deleteIfExists(path);
                }
            } catch (UncheckedIOException failure) {
                throw failure.getCause();
            }
        }
    }
}
