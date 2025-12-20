#!/usr/bin/env python3
"""
⚡ NUCLEAR PERFORMANCE BENCHMARK
Sistema de benchmarking avanzado para medir rendimiento del proyecto
"""

import time
import psutil
import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional
import argparse
import statistics
import threading
import requests
from concurrent.futures import ThreadPoolExecutor, as_completed

class NuclearBenchmark:
    def __init__(self):
        self.metrics = {}
        self.baseline_file = Path('performance_baseline.json')

    def run_full_benchmark(self) -> Dict[str, Any]:
        """Ejecutar benchmark completo del sistema"""
        print("🚀 Iniciando benchmark completo de Nuclear Crawler...")

        results = {
            'timestamp': time.time(),
            'system_info': self._get_system_info(),
            'benchmarks': {}
        }

        # Benchmark de compilación
        print("🔨 Benchmarking compilación...")
        results['benchmarks']['compilation'] = self._benchmark_compilation()

        # Benchmark de análisis de código
        print("🔍 Benchmarking análisis de código...")
        results['benchmarks']['code_analysis'] = self._benchmark_code_analysis()

        # Benchmark de búsqueda web
        print("🌐 Benchmarking búsqueda web...")
        results['benchmarks']['web_search'] = self._benchmark_web_search()

        # Benchmark de procesamiento MCP
        print("🤖 Benchmarking procesamiento MCP...")
        results['benchmarks']['mcp_processing'] = self._benchmark_mcp_processing()

        # Benchmark de rendimiento de memoria
        print("💾 Benchmarking uso de memoria...")
        results['benchmarks']['memory_usage'] = self._benchmark_memory_usage()

        # Calcular métricas agregadas
        results['summary'] = self._calculate_summary_metrics(results['benchmarks'])

        # Comparar con baseline
        results['comparison'] = self._compare_with_baseline(results)

        return results

    def _get_system_info(self) -> Dict[str, Any]:
        """Obtener información del sistema"""
        return {
            'cpu_count': psutil.cpu_count(),
            'cpu_freq': psutil.cpu_freq().current if psutil.cpu_freq() else None,
            'memory_total': psutil.virtual_memory().total,
            'python_version': sys.version,
            'platform': sys.platform
        }

    def _benchmark_compilation(self) -> Dict[str, Any]:
        """Benchmark de tiempo de compilación"""
        times = []

        for i in range(3):  # 3 ejecuciones para promedio
            start_time = time.time()

            try:
                result = subprocess.run(
                    ['cargo', 'build', '--release'],
                    capture_output=True,
                    text=True,
                    timeout=300  # 5 minutos timeout
                )

                end_time = time.time()
                if result.returncode == 0:
                    times.append(end_time - start_time)
                else:
                    print(f"❌ Error en compilación {i+1}: {result.stderr}")
            except subprocess.TimeoutExpired:
                print(f"⏰ Timeout en compilación {i+1}")
                times.append(300)  # Máximo tiempo

        return {
            'times': times,
            'average_time': statistics.mean(times) if times else 0,
            'min_time': min(times) if times else 0,
            'max_time': max(times) if times else 0,
            'success_rate': len(times) / 3
        }

    def _benchmark_code_analysis(self) -> Dict[str, Any]:
        """Benchmark de análisis de código"""
        # Simular análisis de archivos del proyecto
        project_files = list(Path('.').rglob('*.rs'))
        sample_files = project_files[:10]  # Analizar máximo 10 archivos

        times = []
        issues_found = []

        for file_path in sample_files:
            start_time = time.time()

            try:
                # Ejecutar cargo check en el archivo
                result = subprocess.run(
                    ['cargo', 'check', '--message-format=json'],
                    capture_output=True,
                    text=True,
                    cwd=file_path.parent if file_path.parent != Path('.') else None,
                    timeout=60
                )

                end_time = time.time()
                times.append(end_time - start_time)

                # Contar warnings/errors
                issues = len([line for line in result.stdout.split('\n')
                             if '"level":"error"' in line or '"level":"warning"' in line])
                issues_found.append(issues)

            except subprocess.TimeoutExpired:
                times.append(60)
                issues_found.append(0)

        return {
            'files_analyzed': len(sample_files),
            'average_time_per_file': statistics.mean(times) if times else 0,
            'total_time': sum(times),
            'average_issues_per_file': statistics.mean(issues_found) if issues_found else 0,
            'throughput': len(sample_files) / sum(times) if times else 0
        }

    def _benchmark_web_search(self) -> Dict[str, Any]:
        """Benchmark de búsqueda web"""
        # Este benchmark requiere que el servidor MCP esté ejecutándose
        # Simularemos llamadas HTTP al endpoint MCP

        test_queries = [
            "Rust async programming best practices",
            "Web scraping techniques 2025",
            "MCP protocol implementation",
            "Performance optimization Rust",
            "AI agents for code analysis"
        ]

        times = []
        results_count = []

        for query in test_queries:
            start_time = time.time()

            try:
                # Simular llamada al MCP server (ajusta URL según tu configuración)
                response = requests.post(
                    'http://localhost:5050/mcp/tools/call',
                    json={
                        "name": "websearch",
                        "arguments": {
                            "query": query,
                            "max_results": 5
                        }
                    },
                    timeout=30
                )

                end_time = time.time()

                if response.status_code == 200:
                    times.append(end_time - start_time)
                    data = response.json()
                    count = len(data.get('content', []))
                    results_count.append(count)
                else:
                    times.append(30)  # Timeout
                    results_count.append(0)

            except (requests.RequestException, requests.Timeout):
                times.append(30)
                results_count.append(0)

        return {
            'queries_tested': len(test_queries),
            'average_response_time': statistics.mean(times) if times else 0,
            'average_results_per_query': statistics.mean(results_count) if results_count else 0,
            'success_rate': len([t for t in times if t < 30]) / len(times),
            'throughput': len(test_queries) / sum(times) if times else 0
        }

    def _benchmark_mcp_processing(self) -> Dict[str, Any]:
        """Benchmark de procesamiento MCP"""
        # Ejecutar múltiples herramientas MCP en paralelo

        tasks = [
            {"name": "stats", "arguments": {}},
            {"name": "ultimas_busquedas", "arguments": {"limit": 10}},
            {"name": "urls_visitadas", "arguments": {"limit": 20}},
            {"name": "scan_project", "arguments": {"project_path": ".", "search_solutions": False}}
        ]

        times = []

        def execute_task(task):
            start_time = time.time()
            try:
                response = requests.post(
                    'http://localhost:5050/mcp/tools/call',
                    json=task,
                    timeout=60
                )
                end_time = time.time()
                return end_time - start_time if response.status_code == 200 else 60
            except:
                return 60

        # Ejecutar en paralelo
        with ThreadPoolExecutor(max_workers=4) as executor:
            futures = [executor.submit(execute_task, task) for task in tasks]
            for future in as_completed(futures):
                times.append(future.result())

        return {
            'tasks_executed': len(tasks),
            'average_response_time': statistics.mean(times) if times else 0,
            'parallel_throughput': len(tasks) / max(times) if times else 0,
            'success_rate': len([t for t in times if t < 60]) / len(times)
        }

    def _benchmark_memory_usage(self) -> Dict[str, Any]:
        """Benchmark de uso de memoria"""
        process = None

        try:
            # Iniciar el proceso
            process = subprocess.Popen(
                ['cargo', 'run', '--release', '--', '--stdio'],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE
            )

            # Esperar a que inicie
            time.sleep(2)

            memory_usage = []
            cpu_usage = []

            # Monitorear durante 30 segundos
            for _ in range(30):
                if process.poll() is None:  # Si el proceso aún está vivo
                    try:
                        proc = psutil.Process(process.pid)
                        memory_usage.append(proc.memory_info().rss / 1024 / 1024)  # MB
                        cpu_usage.append(proc.cpu_percent(interval=0.1))
                    except psutil.NoSuchProcess:
                        break
                time.sleep(1)

            # Terminar proceso
            process.terminate()
            process.wait(timeout=5)

        except Exception as e:
            print(f"Error en benchmark de memoria: {e}")
            memory_usage = [0]
            cpu_usage = [0]

        finally:
            if process and process.poll() is None:
                process.kill()

        return {
            'average_memory_mb': statistics.mean(memory_usage) if memory_usage else 0,
            'peak_memory_mb': max(memory_usage) if memory_usage else 0,
            'average_cpu_percent': statistics.mean(cpu_usage) if cpu_usage else 0,
            'peak_cpu_percent': max(cpu_usage) if cpu_usage else 0
        }

    def _calculate_summary_metrics(self, benchmarks: Dict[str, Any]) -> Dict[str, Any]:
        """Calcular métricas de resumen"""
        return {
            'overall_performance_score': self._calculate_performance_score(benchmarks),
            'bottlenecks': self._identify_bottlenecks(benchmarks),
            'recommendations': self._generate_recommendations(benchmarks)
        }

    def _calculate_performance_score(self, benchmarks: Dict[str, Any]) -> float:
        """Calcular puntuación general de rendimiento"""
        scores = []

        # Compilación (peso 20%)
        comp_time = benchmarks.get('compilation', {}).get('average_time', 60)
        comp_score = max(0, 100 - (comp_time / 60) * 100)  # Menos de 60s = mejor
        scores.append(comp_score * 0.2)

        # Análisis de código (peso 25%)
        analysis_time = benchmarks.get('code_analysis', {}).get('average_time_per_file', 10)
        analysis_score = max(0, 100 - (analysis_time / 10) * 100)  # Menos de 10s por archivo = mejor
        scores.append(analysis_score * 0.25)

        # Búsqueda web (peso 20%)
        web_time = benchmarks.get('web_search', {}).get('average_response_time', 30)
        web_score = max(0, 100 - (web_time / 30) * 100)  # Menos de 30s = mejor
        scores.append(web_score * 0.2)

        # Procesamiento MCP (peso 20%)
        mcp_time = benchmarks.get('mcp_processing', {}).get('average_response_time', 30)
        mcp_score = max(0, 100 - (mcp_time / 30) * 100)  # Menos de 30s = mejor
        scores.append(mcp_score * 0.2)

        # Memoria (peso 15%)
        memory_mb = benchmarks.get('memory_usage', {}).get('average_memory_mb', 500)
        memory_score = max(0, 100 - (memory_mb / 500) * 100)  # Menos de 500MB = mejor
        scores.append(memory_score * 0.15)

        return sum(scores)

    def _identify_bottlenecks(self, benchmarks: Dict[str, Any]) -> List[str]:
        """Identificar cuellos de botella"""
        bottlenecks = []

        # Verificar tiempos de compilación
        if benchmarks.get('compilation', {}).get('average_time', 0) > 120:
            bottlenecks.append("Tiempo de compilación excesivo (>2min)")

        # Verificar análisis de código lento
        if benchmarks.get('code_analysis', {}).get('average_time_per_file', 0) > 15:
            bottlenecks.append("Análisis de código lento (>15s por archivo)")

        # Verificar uso de memoria alto
        if benchmarks.get('memory_usage', {}).get('average_memory_mb', 0) > 1000:
            bottlenecks.append("Uso de memoria excesivo (>1GB)")

        # Verificar bajo throughput en búsquedas
        if benchmarks.get('web_search', {}).get('throughput', 0) < 0.1:
            bottlenecks.append("Bajo throughput en búsquedas web (<0.1 q/s)")

        return bottlenecks

    def _generate_recommendations(self, benchmarks: Dict[str, Any]) -> List[str]:
        """Generar recomendaciones de optimización"""
        recommendations = []

        # Recomendaciones basadas en métricas
        if benchmarks.get('compilation', {}).get('average_time', 0) > 60:
            recommendations.append("Optimizar dependencias y reducir tiempo de compilación")

        if benchmarks.get('memory_usage', {}).get('average_memory_mb', 0) > 500:
            recommendations.append("Implementar gestión de memoria más eficiente")

        if benchmarks.get('web_search', {}).get('success_rate', 1) < 0.8:
            recommendations.append("Mejorar estabilidad de búsquedas web")

        if benchmarks.get('mcp_processing', {}).get('parallel_throughput', 0) < 1:
            recommendations.append("Optimizar procesamiento paralelo en MCP")

        return recommendations

    def _compare_with_baseline(self, current_results: Dict[str, Any]) -> Dict[str, Any]:
        """Comparar con resultados baseline"""
        if not self.baseline_file.exists():
            return {"status": "No baseline available"}

        try:
            with open(self.baseline_file, 'r') as f:
                baseline = json.load(f)

            comparison = {
                "status": "Compared with baseline",
                "performance_change": current_results['summary']['overall_performance_score'] -
                                    baseline.get('summary', {}).get('overall_performance_score', 0),
                "improvements": [],
                "regressions": []
            }

            # Comparar métricas individuales
            for benchmark_name, current_benchmark in current_results['benchmarks'].items():
                baseline_benchmark = baseline.get('benchmarks', {}).get(benchmark_name, {})

                if 'average_time' in current_benchmark and 'average_time' in baseline_benchmark:
                    change = current_benchmark['average_time'] - baseline_benchmark['average_time']
                    if change < -1:  # Mejora significativa
                        comparison['improvements'].append(f"{benchmark_name}: {abs(change):.2f}s faster")
                    elif change > 1:  # Regresión
                        comparison['regressions'].append(f"{benchmark_name}: {change:.2f}s slower")

            return comparison

        except Exception as e:
            return {"status": f"Error comparing with baseline: {e}"}

    def save_baseline(self, results: Dict[str, Any]) -> None:
        """Guardar resultados como baseline"""
        with open(self.baseline_file, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"✅ Baseline guardado en {self.baseline_file}")

def main():
    parser = argparse.ArgumentParser(description='Benchmark de rendimiento para Nuclear Crawler')
    parser.add_argument('--output', '-o', help='Archivo de salida para resultados')
    parser.add_argument('--save-baseline', '-b', action='store_true',
                       help='Guardar resultados como baseline')
    parser.add_argument('--compare-baseline', '-c', action='store_true',
                       help='Comparar con baseline existente')

    args = parser.parse_args()

    benchmark = NuclearBenchmark()
    results = benchmark.run_full_benchmark()

    # Mostrar resumen
    print("\n📊 RESULTADOS DEL BENCHMARK"    print(f"Overall Performance Score: {results['summary']['overall_performance_score']:.1f}/100")

    if results['summary']['bottlenecks']:
        print("\n🚧 Cuellos de botella identificados:")
        for bottleneck in results['summary']['bottlenecks']:
            print(f"  - {bottleneck}")

    if results['summary']['recommendations']:
        print("\n💡 Recomendaciones:")
        for rec in results['summary']['recommendations']:
            print(f"  - {rec}")

    # Comparación con baseline
    if args.compare_baseline:
        comparison = results['comparison']
        if comparison['status'] == 'Compared with baseline':
            change = comparison['performance_change']
            if change > 0:
                print(".1f"            elif change < 0:
                print(".1f"            else:
                print("📊 Rendimiento estable (sin cambios significativos)")

    # Guardar resultados
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(results, f, indent=2)
        print(f"\n📄 Resultados completos guardados en: {args.output}")

    # Guardar como baseline
    if args.save_baseline:
        benchmark.save_baseline(results)

if __name__ == '__main__':
    main()
