FROM eclipse-temurin:21-jdk AS builder

WORKDIR /app

COPY gradlew settings.gradle.kts build.gradle.kts ./
COPY gradle/ gradle/
RUN ./gradlew dependencies --no-daemon

COPY src/ src/
RUN ./gradlew shadowJar --no-daemon

FROM eclipse-temurin:21-jre

WORKDIR /app

COPY --from=builder /app/build/libs/hello-ktor-0.0.1-all.jar app.jar

RUN useradd -u 1001 -r -s /sbin/nologin appuser
USER 1001

EXPOSE 3000

ENTRYPOINT ["java", "-jar", "app.jar"]
