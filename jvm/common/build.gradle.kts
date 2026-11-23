plugins {
    `java-test-fixtures`
}

sourceSets.main {
    java.srcDir("src/main/generated")
}

// Framework-neutral mechanics shared by the Gradle and Maven adapters.
//
// This module is never published on its own: both plugin jars compile its
// classes in, so Robusta's public artifact set stays at two coordinates. It
// must therefore depend on nothing but the JDK — a dependency here would become
// an unpublished transitive dependency of both plugins.
dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter:5.13.4")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}
