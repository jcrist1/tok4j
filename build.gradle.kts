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

// declare a "configuration" named "someConfiguration"
val someConfiguration by configurations.creating

dependencies {
    implementation("commons-io:commons-io:2.14.0")
    testImplementation(kotlin("test"))
}





tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(11)
}

application {
    applicationDefaultJvmArgs = mutableListOf("-Djava.library.path=" + file("src/tok4jbindings/target/release").absolutePath)
    mainClass.set("dev.gigapixel.tok4j.Main")
}