# Nuclear Crawler Hybrid - Proyecto Spark Java

Proyecto Spark Java integrado con el sistema Nuclear Crawler Hybrid existente.

## Características

- Procesamiento distribuido con Apache Spark
- Integración con el sistema de crawling existente
- Análisis de datos en tiempo real
- Escalabilidad horizontal
- Arquitectura modular y mantenible

## Estructura del Proyecto

```
spark_project/
├── src/
│   ├── main/
│   │   ├── java/
│   │   │   └── com/nuclearcrawler/spark/
│   │   │       ├── Main.java
│   │   │       ├── CrawlerIntegration.java
│   │   │       ├── Analytics.java
│   │   │       └── KryoRegistrator.java
│   │   └── resources/
│   │       ├── log4j.properties
│   │       └── spark-defaults.conf
│   ├── test/
│   │   └── java/
│   │       └── com/nuclearcrawler/spark/
│   │           ├── MainTest.java
│   │           └── AnalyticsTest.java
│   └── scripts/
│       ├── build_and_run.sh
│       ├── setup_spark_env.sh
│       └── start_spark_cluster.sh
├── pom.xml
├── README.md
└── .env.example
```

## Requisitos Previos

1. **Java JDK 11+** instalado
2. **Apache Maven 3.6+** instalado  
3. **Apache Spark 3.5.1** instalado
4. **Apache Hadoop 3.x** (opcional, para HDFS)

## Instalación

1. Clonar el repositorio
2. Configurar variables de entorno:
   ```bash
   export JAVA_HOME="/path/to/jdk-11"
   export MAVEN_HOME="/path/to/maven-3.9.5"
   export PATH="$JAVA_HOME/bin:$MAVEN_HOME/bin:$PATH"
   ```
3. Navegar al directorio del proyecto:
   ```bash
   cd D:\PROJECTS\nuclear-crawler-hybrid\spark_project
   ```
4. Compilar y ejecutar:
   ```bash
   ./scripts/build_and_run.sh
   ```

## Uso

### Ejecución Principal
```bash
# Compilar y ejecutar
./scripts/build_and_run.sh

# O ejecutar manualmente
mvn clean compile
mvn package
java -cp target/nuclear-crawler-spark-1.0.0.jar com.nuclearcrawler.spark.Main
```

### Integración con Crawler
```java
// Ejemplo de integración
CrawlerIntegration integration = new CrawlerIntegration();
Dataset<Row> processedData = integration.integrateWithCrawler("../data/crawler_output.json");
integration.saveToStorage(processedData, "../data/processed_data");
integration.close();
```

### Análisis de Datos
```java
// Ejemplo de análisis
Analytics analytics = new Analytics();
Dataset<Row> data = analytics.loadData("../data/processed_data");
Dataset<Row> features = analytics.featureEngineering(data);
Analytics.ClusteringResult result = analytics.clusteringAnalysis(features, 5);
Dataset<Row> anomalies = analytics.anomalyDetection(data);
Dataset<Row> timeSeries = analytics.timeSeriesAnalysis(data);

// Guardar resultados
analytics.saveResults(result.getPredictions(), "../data/analysis_results");
analytics.close();
```

## Configuración

### Variables de Entorno
Crea un archivo `.env` basado en `.env.example`:

```bash
# Spark Configuration
SPARK_MASTER=local[*]
SPARK_DRIVER_MEMORY=4g
SPARK_EXECUTOR_MEMORY=4g
SPARK_SERIALIZER=org.apache.spark.serializer.KryoSerializer

# Data Paths
CRAWLER_DATA_PATH=../data/crawler_output.json
PROCESSED_DATA_PATH=../data/processed_data
ANALYSIS_RESULTS_PATH=../data/analysis_results
```

### Configuración de Spark
Edita `src/main/resources/spark-defaults.conf`:

```properties
# Spark Master
spark.master local[*]

# Memory
spark.driver.memory 4g
spark.executor.memory 4g

# Serialization
spark.serializer org.apache.spark.serializer.KryoSerializer
spark.kryo.registrator com.nuclearcrawler.spark.KryoRegistrator

# Performance
spark.sql.shuffle.partitions 200
spark.task.maxFailures 4
spark.stage.maxConsecutiveAttempts 3
```

## Integración con Sistema Existente

El proyecto está diseñado para integrarse con tu sistema Nuclear Crawler Hybrid existente:

1. **Lectura de datos**: Lee archivos JSON generados por el crawler
2. **Procesamiento**: Aplica transformaciones y enriquecimiento de datos
3. **Análisis**: Realiza clustering, detección de anomalías y análisis de series de tiempo
4. **Almacenamiento**: Guarda resultados en formatos optimizados (Parquet, JSON, CSV)

## Scripts Disponibles

- `build_and_run.sh`: Compila, prueba y ejecuta el proyecto
- `setup_spark_env.sh`: Configura el entorno Spark
- `start_spark_cluster.sh`: Inicia un cluster Spark local

## Testing

```bash
# Ejecutar tests
mvn test

# Ejecutar tests específicos
mvn test -Dtest=MainTest
mvn test -Dtest=AnalyticsTest
```

## Monitoreo y Logging

El proyecto utiliza SLF4J con Log4j para logging:
- Nivel de log configurable en `src/main/resources/log4j.properties`
- Logs detallados para depuración y monitoreo
- Integración con herramientas de monitoreo Spark

## Despliegue

### Despliegue Local
```bash
# Compilar JAR
mvn package

# Ejecutar
java -jar target/nuclear-crawler-spark-1.0.0.jar
```

### Despliegue en Cluster
```bash
# Enviar JAR al cluster
scp target/nuclear-crawler-spark-1.0.0.jar user@cluster:/path/to/app/

# Ejecutar en cluster
spark-submit \
  --class com.nuclearcrawler.spark.Main \
  --master spark://master:7077 \
  --executor-memory 4g \
  --driver-memory 4g \
  /path/to/app/nuclear-crawler-spark-1.0.0.jar
```

## Troubleshooting

### Problemas Comunes

1. **Error de memoria**: Aumenta los parámetros de memoria en `pom.xml`
2. **Error de serialización**: Verifica la configuración de KryoSerializer
3. **Error de permisos**: Asegúrate de tener permisos de escritura en los directorios de datos
4. **Error de dependencias**: Ejecuta `mvn clean install` para resolver dependencias

### Logs de Error

Los logs se guardan en `logs/` con rotación automática:
- `logs/app.log`: Logs de la aplicación
- `logs/spark.log`: Logs de Spark
- `logs/error.log`: Errores críticos

## Contribución

1. Fork del repositorio
2. Crear feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit de cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la branch (`git push origin feature/AmazingFeature`)
5. Abrir Pull Request

## Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo LICENSE para detalles.

## Contacto

Para soporte y consultas:
- Email: support@nuclearcrawler.com
- Issues: GitHub Issues
- Documentation: docs.nuclearcrawler.com