pluginManagement {
    repositories {
        // Current Vanniktech releases publish their plugin marker here.
        mavenCentral()
        gradlePluginPortal()
    }
}

rootProject.name = "robusta-jvm"

include("common")
include("gradle-plugin")
include("maven-plugin")
