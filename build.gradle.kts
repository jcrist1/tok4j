plugins {
    kotlin("jvm") version "1.9.0"
    application
    'c'
}

group = "dev.gigapixel"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(11)
}

application {
    applicationDefaultJvmArgs = mutableListOf("-Djava.library.path=" + file("${buildDir}/tok4jbindings/target/release").absolutePath)
    mainClass.set("Main")
}