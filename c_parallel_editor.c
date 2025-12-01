#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <pthread.h>
#include <regex.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <sys/wait.h>

// Configuración HPC
#define MAX_THREADS 16
#define CHUNK_SIZE 8192
#define MAX_PATH 4096
#define CHAPEL_BINARY "./chpl_parallel_editor"
#define DOCKER_CHAPEL "docker run --rm -v $(pwd):/workspace chapel-hpc chpl_parallel_editor"

// Estructura para resultados de procesamiento
typedef struct {
    char filepath[MAX_PATH];
    int changes;
    char status[32];
} ProcessResult;

// Estructura para argumentos de thread
typedef struct {
    char **files;
    int start_idx;
    int end_idx;
    const char *search_pattern;
    const char *replace_pattern;
    int dry_run;
    int use_chapel;
    ProcessResult *results;
} ThreadArgs;

// Función para leer archivo completo
char* read_file(const char *filepath, size_t *size) {
    FILE *file = fopen(filepath, "rb");
    if (!file) return NULL;

    fseek(file, 0, SEEK_END);
    *size = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *content = malloc(*size + 1);
    if (!content) {
        fclose(file);
        return NULL;
    }

    fread(content, 1, *size, file);
    content[*size] = '\0';
    fclose(file);
    return content;
}

// Función para escribir archivo
int write_file(const char *filepath, const char *content) {
    FILE *file = fopen(filepath, "wb");
    if (!file) return 0;

    size_t len = strlen(content);
    size_t written = fwrite(content, 1, len, file);
    fclose(file);
    return written == len;
}

// Función de búsqueda y reemplazo con regex
int regex_replace(char **content, const char *pattern, const char *replacement) {
    regex_t regex;
    regmatch_t match;
    int changes = 0;

    if (regcomp(&regex, pattern, REG_EXTENDED) != 0) {
        return -1; // Error en regex
    }

    char *result = NULL;
    size_t result_size = 0;
    const char *current = *content;

    while (regexec(&regex, current, 1, &match, 0) == 0) {
        // Calcular tamaños
        size_t prefix_len = match.rm_so;
        size_t match_len = match.rm_eo - match.rm_so;
        size_t suffix_len = strlen(current + match.rm_eo);
        size_t replacement_len = strlen(replacement);

        // Reallocar resultado
        size_t new_size = result_size + prefix_len + replacement_len + suffix_len + 1;
        result = realloc(result, new_size);

        if (!result) break;

        // Copiar prefijo
        memcpy(result + result_size, current, prefix_len);
        result_size += prefix_len;

        // Copiar reemplazo
        memcpy(result + result_size, replacement, replacement_len);
        result_size += replacement_len;

        // Avanzar
        current += match.rm_eo;
        changes++;
    }

    // Copiar resto
    if (result) {
        strcpy(result + result_size, current);
        free(*content);
        *content = result;
    }

    regfree(&regex);
    return changes;
}

// Función para procesar un archivo individual
void* process_file(void *arg) {
    ThreadArgs *args = (ThreadArgs*)arg;

    for (int i = args->start_idx; i <= args->end_idx; i++) {
        const char *filepath = args->files[i];
        ProcessResult *result = &args->results[i];

        strcpy(result->filepath, filepath);
        result->changes = 0;
        strcpy(result->status, "ERROR");

        // Leer archivo
        size_t size;
        char *content = read_file(filepath, &size);
        if (!content) {
            strcpy(result->status, "READ_ERROR");
            continue;
        }

        char *original_content = strdup(content);

        // Aplicar reemplazo
        int changes = regex_replace(&content, args->search_pattern, args->replace_pattern);
        if (changes < 0) {
            strcpy(result->status, "REGEX_ERROR");
            free(content);
            free(original_content);
            continue;
        }

        result->changes = changes;

        // Solo escribir si cambió y no es dry-run
        if (strcmp(content, original_content) != 0) {
            if (!args->dry_run && write_file(filepath, content)) {
                strcpy(result->status, "MODIFIED");
            } else if (args->dry_run) {
                strcpy(result->status, "WOULD_MODIFY");
            } else {
                strcpy(result->status, "WRITE_ERROR");
            }
        } else {
            strcpy(result->status, "UNCHANGED");
        }

        free(content);
        free(original_content);
    }

    return NULL;
}

// Función para ejecutar Chapel como subprocess
int execute_chapel(const char *root_dir, const char *search_pattern, const char *replace_pattern, int dry_run) {
    printf("🌟 LLAMANDO A CHAPEL - INTEGRACIÓN HPC ULTRA-AVANZADA\n");
    printf("==================================================\n");

    // Verificar si Chapel está disponible
    if (access(CHAPEL_BINARY, F_OK) != 0) {
        printf("⚠️  Binario Chapel no encontrado. Intentando con Docker...\n");

        // Intentar con Docker
        char docker_cmd[1024];
        snprintf(docker_cmd, sizeof(docker_cmd),
                 "docker run --rm -v \"%s:/workspace\" -w /workspace chapel/chapel:latest "
                 "./chpl_parallel_editor \"%s\" \"%s\" \"%s\" %s",
                 getcwd(NULL, 0), root_dir, search_pattern, replace_pattern,
                 dry_run ? "--dry-run" : "");

        printf("🐳 Ejecutando Chapel via Docker: %s\n", docker_cmd);
        int result = system(docker_cmd);

        if (result == 0) {
            printf("✅ Chapel ejecutado exitosamente via Docker\n");
            return 1;
        } else {
            printf("❌ Error ejecutando Chapel via Docker\n");
            return 0;
        }
    }

    // Ejecutar Chapel directamente
    char chapel_cmd[512];
    snprintf(chapel_cmd, sizeof(chapel_cmd), "%s \"%s\" \"%s\" \"%s\" %s",
             CHAPEL_BINARY, root_dir, search_pattern, replace_pattern,
             dry_run ? "--dry-run" : "");

    printf("🚀 Ejecutando Chapel directamente: %s\n", chapel_cmd);
    int result = system(chapel_cmd);

    if (result == 0) {
        printf("✅ Chapel ejecutado exitosamente\n");
        return 1;
    } else {
        printf("❌ Error ejecutando Chapel\n");
        return 0;
    }
}

// Función para compilar Chapel si es necesario
int compile_chapel_if_needed() {
    if (access(CHAPEL_BINARY, F_OK) == 0) {
        printf("✅ Binario Chapel ya existe\n");
        return 1;
    }

    printf("🔨 Compilando Chapel con optimizaciones HPC...\n");

    // Verificar que chpl existe
    if (system("chpl --version > /dev/null 2>&1") != 0) {
        printf("❌ Chapel compiler no encontrado. Intentando con Docker...\n");

        // Intentar compilar con Docker
        const char *docker_compile = "docker run --rm -v \"$(pwd):/workspace\" -w /workspace "
                                   "chapel/chapel:latest "
                                   "chpl --fast --specialize --optimize-forall-unordered-ops "
                                   "--optimize-loop-iterators --optimize-on-clauses --optimize-on "
                                   "--inline --vectorize --cache-remote "
                                   "--report-optimized-loop-iterators "
                                   "chpl_parallel_editor.chpl -o chpl_parallel_editor";

        printf("🐳 Compilando Chapel via Docker...\n");
        int result = system(docker_compile);

        if (result == 0) {
            printf("✅ Chapel compilado exitosamente via Docker\n");
            return 1;
        } else {
            printf("❌ Error compilando Chapel via Docker\n");
            return 0;
        }
    }

    // Compilar Chapel directamente
    const char *compile_cmd = "chpl --fast --specialize --optimize-forall-unordered-ops "
                             "--optimize-loop-iterators --optimize-on-clauses --optimize-on "
                             "--inline --vectorize --cache-remote "
                             "--report-optimized-loop-iterators "
                             "chpl_parallel_editor.chpl -o chpl_parallel_editor";

    printf("🔨 Compilando Chapel directamente...\n");
    int result = system(compile_cmd);

    if (result == 0) {
        printf("✅ Chapel compilado exitosamente\n");
        return 1;
    } else {
        printf("❌ Error compilando Chapel\n");
        return 0;
    }
}

// Función para determinar si usar Chapel
int should_use_chapel(int num_files, const char *pattern) {
    // Usar Chapel para:
    // - Más de 100 archivos
    // - Patrones complejos de regex
    // - Procesamiento masivo

    if (num_files > 100) return 1;
    if (strlen(pattern) > 20) return 1; // Patrón complejo
    if (strchr(pattern, '[') || strchr(pattern, '+') || strchr(pattern, '*')) return 1;

    return 0; // Usar C puro para casos simples
}

// Función principal con integración Chapel
int main(int argc, char *argv[]) {
    if (argc < 4) {
        printf("🔬 C PARALLEL CODE EDITOR - HPC POWER + CHAPEL INTEGRATION\n");
        printf("=======================================================\n\n");
        printf("Uso: %s <directorio> <patrón_búsqueda> <patrón_reemplazo> [--dry-run] [--chapel]\n\n", argv[0]);
        printf("Ejemplos:\n");
        printf("  %s . \"old_function\" \"new_function\"\n", argv[0]);
        printf("  %s /src \"println!\" \"writeln!\" --dry-run\n", argv[0]);
        printf("  %s . \"complex.*pattern\" \"replacement\" --chapel\n\n", argv[0]);
        printf("⚡ Potencia: C + pthreads + Chapel HPC integration\n");
        printf("🌟 Chapel se activa automáticamente para procesamiento masivo\n");
        return 1;
    }

    const char *root_dir = argv[1];
    const char *search_pattern = argv[2];
    const char *replace_pattern = argv[3];
    int dry_run = 0;
    int force_chapel = 0;

    // Parsear argumentos adicionales
    for (int i = 4; i < argc; i++) {
        if (strcmp(argv[i], "--dry-run") == 0) dry_run = 1;
        if (strcmp(argv[i], "--chapel") == 0) force_chapel = 1;
    }

    printf("🔬 C PARALLEL CODE EDITOR - HPC POWER + CHAPEL INTEGRATION\n");
    printf("=======================================================\n");
    printf("🎯 Procesamiento paralelo C + Chapel HPC ultra-avanzado\n\n");

    printf("📁 Directorio: %s\n", root_dir);
    printf("🔍 Patrón: %s\n", search_pattern);
    printf("✏️  Reemplazo: %s\n", replace_pattern);
    printf("🔍 Modo: %s\n", dry_run ? "dry-run" : "aplicar cambios");

    // Encontrar archivos primero
    printf("\n📁 Fase 1: Escaneando archivos...\n");
    char **files;
    int num_files = find_files(root_dir, "*", &files);
    printf("📊 Encontrados %d archivos para procesar\n", num_files);

    if (num_files == 0) {
        printf("ℹ️  No se encontraron archivos para procesar\n");
        return 0;
    }

    // Decidir si usar Chapel
    int use_chapel = force_chapel || should_use_chapel(num_files, search_pattern);

    if (use_chapel) {
        printf("🌟 Usando CHAPEL para procesamiento HPC ultra-avanzado\n");
        printf("   📊 %d archivos - Procesamiento masivo justifica Chapel\n", num_files);

        // Compilar Chapel si es necesario
        if (!compile_chapel_if_needed()) {
            printf("❌ No se pudo preparar Chapel, cayendo a C puro\n");
            use_chapel = 0;
        } else {
            // Ejecutar Chapel
            if (execute_chapel(root_dir, search_pattern, replace_pattern, dry_run)) {
                printf("\n🎉 ¡Procesamiento Chapel completado exitosamente!\n");
                printf("💡 Chapel ha manejado el procesamiento masivo con paralelismo HPC\n");

                // Limpiar memoria
                for (int i = 0; i < num_files; i++) {
                    free(files[i]);
                }
                free(files);

                return 0;
            } else {
                printf("❌ Error en Chapel, cayendo a C puro\n");
                use_chapel = 0;
            }
        }
    }

    if (!use_chapel) {
        printf("⚡ Usando C PURO para procesamiento paralelo nativo\n");
        printf("   📊 %d archivos - Optimizado para C + pthreads\n", num_files);
    }

    printf("\n🚀 Fase 2: Procesamiento paralelo HPC...\n");
    clock_t start_time = clock();

    // Crear threads
    int num_threads = (num_files < MAX_THREADS) ? num_files : MAX_THREADS;
    int files_per_thread = num_files / num_threads;

    pthread_t threads[MAX_THREADS];
    ThreadArgs thread_args[MAX_THREADS];
    ProcessResult results[MAX_THREADS * 100]; // Ajustar según necesidad

    // Distribuir trabajo
    int current_idx = 0;
    for (int t = 0; t < num_threads; t++) {
        thread_args[t].files = files;
        thread_args[t].start_idx = current_idx;
        thread_args[t].end_idx = (t == num_threads - 1) ? num_files - 1 : current_idx + files_per_thread - 1;
        thread_args[t].search_pattern = search_pattern;
        thread_args[t].replace_pattern = replace_pattern;
        thread_args[t].dry_run = dry_run;
        thread_args[t].results = &results[current_idx];

        pthread_create(&threads[t], NULL, process_file, &thread_args[t]);
        current_idx = thread_args[t].end_idx + 1;
    }

    // Esperar threads
    for (int t = 0; t < num_threads; t++) {
        pthread_join(threads[t], NULL);
    }

    clock_t end_time = clock();
    double total_time = (double)(end_time - start_time) / CLOCKS_PER_SEC;

    // Agregar resultados
    int total_changes = 0;
    int modified_files = 0;
    int error_files = 0;

    for (int i = 0; i < num_files; i++) {
        total_changes += results[i].changes;
        if (strcmp(results[i].status, "MODIFIED") == 0 ||
            strcmp(results[i].status, "WOULD_MODIFY") == 0) {
            modified_files++;
        }
        if (strstr(results[i].status, "ERROR") != NULL) {
            error_files++;
        }
    }

    // Reporte final
    printf("\n🎯 RESULTADOS DEL PROCESAMIENTO PARALELO\n");
    printf("=======================================\n");
    printf("⏱️  Tiempo total: %.2f segundos\n", total_time);
    printf("📁 Archivos procesados: %d\n", num_files);
    printf("✏️  Archivos modificados: %d\n", modified_files);
    printf("🔄 Cambios totales realizados: %d\n", total_changes);
    printf("❌ Archivos con errores: %d\n", error_files);

    double files_per_sec = num_files / total_time;
    printf("⚡ Rendimiento: %.0f archivos/segundo\n", files_per_sec);
    printf("🚀 Eficiencia paralela: %.1f archivos/segundo por thread\n", files_per_sec / num_threads);

    printf("\n");
    if (total_changes > 0 && !dry_run) {
        printf("🎉 ¡Procesamiento completado exitosamente!\n");
        if (use_chapel) {
            printf("💡 Los cambios han sido aplicados usando Chapel HPC + C integration\n");
            printf("🌟 Chapel se mantiene vivo y potente gracias a la integración C-Chapel\n");
        } else {
            printf("💡 Los cambios han sido aplicados usando paralelismo C nativo\n");
        }
    } else if (dry_run) {
        printf("🔍 Dry-run completado - Usa sin --dry-run para aplicar cambios\n");
    } else {
        printf("ℹ️  No se encontraron coincidencias para el patrón especificado\n");
    }

    // Limpiar memoria
    for (int i = 0; i < num_files; i++) {
        free(files[i]);
    }
    free(files);

    return 0;
}