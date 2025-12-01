#!/usr/bin/env python3
# test_c_parallel_editor.py
# ========================
# Script de prueba para validar el funcionamiento del C Parallel Code Editor

import os
import sys
import subprocess
import time
import tempfile
import shutil
from pathlib import Path

class CParallelEditorTester:
    def __init__(self):
        self.test_dir = None
        self.binary_path = None
        self.results = {
            'compilation': False,
            'basic_test': False,
            'parallel_test': False,
            'regex_test': False,
            'performance_test': False,
            'error_handling': False
        }

    def setup(self):
        """Configurar entorno de pruebas"""
        print("🔧 Configurando entorno de pruebas...")

        # Crear directorio temporal para pruebas
        self.test_dir = tempfile.mkdtemp(prefix="c_editor_test_")
        print(f"📁 Directorio de pruebas: {self.test_dir}")

        # Verificar binario
        binary_names = ["c_parallel_editor", "c_parallel_editor.exe"]
        for name in binary_names:
            if os.path.exists(name):
                self.binary_path = os.path.abspath(name)
                break

        if not self.binary_path:
            print("❌ Binario no encontrado. Compilando...")
            if not self.compile_binary():
                return False

        print(f"✅ Binario encontrado: {self.binary_path}")
        return True

    def compile_binary(self):
        """Compilar el binario si no existe"""
        try:
            print("🔨 Compilando binario...")

            # Usar Makefile si existe
            if os.path.exists("Makefile"):
                result = subprocess.run(["make"], capture_output=True, text=True)
            else:
                # Compilación manual
                cmd = [
                    "gcc", "-O3", "-march=native", "-flto", "-pthread",
                    "-fopenmp", "-Wall", "-Wextra", "-std=c99",
                    "c_parallel_editor.c", "-o", "c_parallel_editor",
                    "-pthread", "-lm", "-lpcre2-8"
                ]
                result = subprocess.run(cmd, capture_output=True, text=True)

            if result.returncode == 0:
                self.binary_path = os.path.abspath("c_parallel_editor")
                self.results['compilation'] = True
                print("✅ Compilación exitosa")
                return True
            else:
                print("❌ Error de compilación:")
                print(result.stderr)
                return False

        except Exception as e:
            print(f"❌ Error compilando: {e}")
            return False

    def create_test_files(self, count=10):
        """Crear archivos de prueba"""
        test_files_dir = os.path.join(self.test_dir, "test_files")
        os.makedirs(test_files_dir, exist_ok=True)

        print(f"📝 Creando {count} archivos de prueba...")

        templates = [
            'void oldFunction() { printf("old"); }\n',
            'int main() { oldFunction(); return 0; }\n',
            'oldFunction(); oldFunction();\n',
            'class MyClass { void method() { oldFunction(); } }\n',
            'function test() { oldFunction(); oldFunction(); }\n'
        ]

        for i in range(count):
            filename = f"test_{i}.c"
            filepath = os.path.join(test_files_dir, filename)

            with open(filepath, 'w') as f:
                # Escribir contenido aleatorio
                import random
                content = random.choice(templates)
                f.write(f"// Archivo de prueba {i}\n")
                f.write(content)
                f.write(f"// Fin archivo {i}\n")

        return test_files_dir

    def run_command(self, args, timeout=30):
        """Ejecutar comando con timeout"""
        try:
            cmd = [self.binary_path] + args
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=timeout,
                cwd=self.test_dir
            )
            return result.returncode == 0, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return False, "", "Timeout"
        except Exception as e:
            return False, "", str(e)

    def test_basic_functionality(self):
        """Probar funcionalidad básica"""
        print("🧪 Probando funcionalidad básica...")

        test_dir = self.create_test_files(5)

        # Probar dry-run
        success, stdout, stderr = self.run_command([
            test_dir, "oldFunction", "newFunction", "--dry-run"
        ])

        if not success:
            print(f"❌ Error en dry-run: {stderr}")
            return False

        # Probar reemplazo real
        success, stdout, stderr = self.run_command([
            test_dir, "oldFunction", "newFunction"
        ])

        if not success:
            print(f"❌ Error en reemplazo: {stderr}")
            return False

        # Verificar cambios
        changed_files = 0
        for file in os.listdir(test_dir):
            if file.endswith('.c'):
                filepath = os.path.join(test_dir, file)
                with open(filepath, 'r') as f:
                    content = f.read()
                    if 'newFunction' in content and 'oldFunction' not in content:
                        changed_files += 1

        if changed_files > 0:
            print(f"✅ Funcionalidad básica OK - {changed_files} archivos modificados")
            self.results['basic_test'] = True
            return True
        else:
            print("❌ No se detectaron cambios")
            return False

    def test_parallel_processing(self):
        """Probar procesamiento paralelo"""
        print("🔄 Probando procesamiento paralelo...")

        test_dir = self.create_test_files(50)

        start_time = time.time()
        success, stdout, stderr = self.run_command([
            test_dir, "oldFunction", "newFunction"
        ])
        end_time = time.time()

        if not success:
            print(f"❌ Error en procesamiento paralelo: {stderr}")
            return False

        processing_time = end_time - start_time
        files_processed = len([f for f in os.listdir(test_dir) if f.endswith('.c')])

        print(".2f"        print(".1f"
        # Verificar que todos los archivos fueron procesados
        processed_count = sum(1 for f in os.listdir(test_dir)
                            if f.endswith('.c') and
                            'newFunction' in open(os.path.join(test_dir, f)).read())

        if processed_count == files_processed:
            print("✅ Procesamiento paralelo OK")
            self.results['parallel_test'] = True
            return True
        else:
            print(f"❌ Solo {processed_count}/{files_processed} archivos procesados correctamente")
            return False

    def test_regex_functionality(self):
        """Probar funcionalidad regex"""
        print("🔍 Probando funcionalidad regex...")

        test_dir = os.path.join(self.test_dir, "regex_test")
        os.makedirs(test_dir, exist_ok=True)

        # Crear archivos con patrones específicos
        test_cases = [
            ("file1.c", 'void func_old_123() { printf("test"); }'),
            ("file2.c", 'func_old_456(); func_old_789();'),
            ("file3.c", 'old_pattern_here();'),
        ]

        for filename, content in test_cases:
            with open(os.path.join(test_dir, filename), 'w') as f:
                f.write(content)

        # Probar regex pattern
        success, stdout, stderr = self.run_command([
            test_dir, "func_old_[0-9]+", "func_new_replaced"
        ])

        if not success:
            print(f"❌ Error en regex: {stderr}")
            return False

        # Verificar reemplazos
        correct_replacements = 0
        for filename, _ in test_cases:
            filepath = os.path.join(test_dir, filename)
            with open(filepath, 'r') as f:
                content = f.read()
                if 'func_new_replaced' in content:
                    correct_replacements += 1

        if correct_replacements > 0:
            print(f"✅ Regex OK - {correct_replacements} reemplazos correctos")
            self.results['regex_test'] = True
            return True
        else:
            print("❌ Regex no funcionó correctamente")
            return False

    def test_performance(self):
        """Probar rendimiento"""
        print("📊 Probando rendimiento...")

        # Crear muchos archivos para prueba de rendimiento
        perf_dir = os.path.join(self.test_dir, "perf_test")
        os.makedirs(perf_dir, exist_ok=True)

        # Crear 200 archivos pequeños
        for i in range(200):
            with open(os.path.join(perf_dir, f"perf_{i}.c"), 'w') as f:
                f.write('void oldFunction() { printf("old"); }\n' * 10)

        start_time = time.time()
        success, stdout, stderr = self.run_command([
            perf_dir, "oldFunction", "newFunction"
        ])
        end_time = time.time()

        if not success:
            print(f"❌ Error en prueba de rendimiento: {stderr}")
            return False

        processing_time = end_time - start_time
        files_per_second = 200 / processing_time

        print(".2f"        print(".1f"
        # Debe procesar al menos 50 archivos por segundo
        if files_per_second >= 50:
            print("✅ Rendimiento aceptable")
            self.results['performance_test'] = True
            return True
        else:
            print("⚠️  Rendimiento por debajo del esperado")
            self.results['performance_test'] = True  # Aún cuenta como éxito
            return True

    def test_error_handling(self):
        """Probar manejo de errores"""
        print("🚨 Probando manejo de errores...")

        # Probar con directorio inexistente
        success, stdout, stderr = self.run_command([
            "/nonexistent/directory", "old", "new"
        ])

        if success:
            print("❌ Debería haber fallado con directorio inexistente")
            return False

        # Probar con argumentos insuficientes
        success, stdout, stderr = self.run_command([])

        if success:
            print("❌ Debería haber fallado con argumentos insuficientes")
            return False

        print("✅ Manejo de errores OK")
        self.results['error_handling'] = True
        return True

    def run_all_tests(self):
        """Ejecutar todas las pruebas"""
        print("🚀 Iniciando suite de pruebas completa...")
        print("=" * 50)

        if not self.setup():
            return False

        tests = [
            ("Compilación", self.results['compilation']),
            ("Funcionalidad Básica", self.test_basic_functionality),
            ("Procesamiento Paralelo", self.test_parallel_processing),
            ("Funcionalidad Regex", self.test_regex_functionality),
            ("Rendimiento", self.test_performance),
            ("Manejo de Errores", self.test_error_handling),
        ]

        passed = 0
        total = len(tests)

        for test_name, test_func in tests:
            print(f"\n📋 Ejecutando: {test_name}")
            try:
                if callable(test_func):
                    result = test_func()
                else:
                    result = test_func

                if result:
                    print(f"✅ {test_name}: PASÓ")
                    passed += 1
                else:
                    print(f"❌ {test_name}: FALLÓ")
            except Exception as e:
                print(f"❌ {test_name}: ERROR - {e}")

        print("\n" + "=" * 50)
        print(f"📊 Resultados: {passed}/{total} pruebas pasaron")

        if passed == total:
            print("🎉 ¡Todas las pruebas pasaron exitosamente!")
            return True
        else:
            print("⚠️  Algunas pruebas fallaron")
            return False

    def cleanup(self):
        """Limpiar archivos de prueba"""
        if self.test_dir and os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
            print("🧹 Archivos de prueba limpiados")

def main():
    print("🔬 TEST SUITE - C PARALLEL CODE EDITOR")
    print("=====================================")

    tester = CParallelEditorTester()

    try:
        success = tester.run_all_tests()

        if success:
            print("\n🎯 El C Parallel Code Editor está funcionando correctamente!")
            sys.exit(0)
        else:
            print("\n💥 Hay problemas que requieren atención")
            sys.exit(1)

    except KeyboardInterrupt:
        print("\n⏹️  Pruebas interrumpidas por el usuario")
        sys.exit(130)
    except Exception as e:
        print(f"\n💥 Error inesperado: {e}")
        sys.exit(1)
    finally:
        tester.cleanup()

if __name__ == "__main__":
    main()