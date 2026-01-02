FROM maven:3.9-eclipse-temurin-17 AS builder

WORKDIR /app

COPY pom.xml .
COPY src ./src

RUN mvn clean package -DskipTests

FROM eclipse-temurin:17-jre-alpine

WORKDIR /app

RUN mkdir /logs && chown -R nobody:nobody /logs

COPY --from=builder /app/target/*.jar app.jar

USER nobody

EXPOSE 8080

CMD ["java", "-jar", "app.jar"]
