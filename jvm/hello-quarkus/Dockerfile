FROM eclipse-temurin:21-jdk AS builder

WORKDIR /app

COPY mvnw ./
COPY .mvn/ .mvn/
COPY pom.xml ./
RUN ./mvnw dependency:go-offline -B -q

COPY src/ src/
RUN ./mvnw package -DskipTests -B -q

FROM eclipse-temurin:21-jre

WORKDIR /app

COPY --from=builder /app/target/quarkus-app/ ./

RUN useradd -u 1001 -r -s /sbin/nologin appuser
USER 1001

EXPOSE 10000

ENTRYPOINT ["java", "-jar", "quarkus-run.jar"]
