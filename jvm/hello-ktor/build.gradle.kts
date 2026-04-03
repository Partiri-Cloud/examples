plugins {
    kotlin("jvm") version "2.1.20"
    id("com.github.johnrengelman.shadow") version "8.3.6"
    application
}

group = "com.example"
version = "0.0.1"

val ktorVersion = "3.1.3"

application {
    mainClass.set("com.example.hello.ApplicationKt")
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("io.ktor:ktor-server-netty:$ktorVersion")
    implementation("io.ktor:ktor-server-core:$ktorVersion")
    implementation("ch.qos.logback:logback-classic:1.5.15")
}

tasks.shadowJar {
    archiveFileName.set("hello-ktor-0.0.1-all.jar")
    mergeServiceFiles()
}
