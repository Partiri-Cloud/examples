FROM eclipse-temurin:21-jdk AS builder

WORKDIR /app

COPY gradlew settings.gradle build.gradle ./
COPY gradle/ gradle/
RUN ./gradlew dependencies --no-daemon

COPY src/ src/
RUN ./gradlew build -x test --no-daemon

FROM eclipse-temurin:21-jre

WORKDIR /app

COPY --from=builder /app/build/libs/hello-spring-0.0.1.jar app.jar

RUN useradd -u 1001 -r -s /sbin/nologin appuser
USER 1001

EXPOSE 3000

ENTRYPOINT ["java", "-jar", "app.jar"]
