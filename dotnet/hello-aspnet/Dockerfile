FROM mcr.microsoft.com/dotnet/sdk:9.0 AS builder

WORKDIR /app

COPY hello-aspnet.csproj ./
RUN dotnet restore

COPY . ./
RUN dotnet publish -c Release -o out

FROM mcr.microsoft.com/dotnet/aspnet:9.0

WORKDIR /app

COPY --from=builder /app/out ./

RUN useradd -u 1001 -r -s /sbin/nologin appuser
USER 1001

EXPOSE 10000

ENTRYPOINT ["dotnet", "hello-aspnet.dll"]
