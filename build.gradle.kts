plugins {
    kotlin("jvm") version "1.9.0"
    application
    `maven-publish`
}



group = "dev.gigapixel"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
    maven {
        url = uri("https://bio.informatik.uni-jena.de/repository/libs-release-oss/")
    }
}

// declare a "configuration" named "someConfiguration"
val someConfiguration by configurations.creating

dependencies {
    implementation("cz.adamh:native-utils:1.0")
    implementation("commons-io:commons-io:2.14.0")
    testImplementation(kotlin("test"))
}
publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "dev.gigapixel"
            artifactId = "tok4j"
            version = "1.0-SNAPSHOT"

            from(components["java"])
        }
    }
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