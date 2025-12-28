#!/usr/bin/env python3
"""
🔥 NUCLEAR ADVANCED REPORT GENERATOR
Genera reportes avanzados de análisis multi-agente
"""

import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any
import argparse

class NuclearReportGenerator:
    def __init__(self):
        self.templates = {
            'header': """
# 🚀 NUCLEAR CRAWLER HYBRID - ANÁLISIS AVANZADO
**Generado:** {timestamp}
**Proyecto:** Nuclear Crawler Hybrid 2025
**Modo:** Multi-Agente Avanzado

---
""",
            'summary': """
## 📊 RESUMEN EJECUTIVO

| Métrica | Valor | Estado |
|---------|-------|--------|
| Puntaje de Calidad | {quality_score:.1f}/100 | {quality_status} |
| Issues Críticos | {critical_issues} | {critical_status} |
| Issues de Advertencia | {warning_issues} | {warning_status} |
| Cobertura de Análisis | {coverage:.1%} | {coverage_status} |
| Tiempo de Análisis | {analysis_time:.2f}s | ✅ |

### 🎯 Puntuación General: **{overall_score:.1f}/100**
""",
            'agent_analysis': """
## 🤖 ANÁLISIS POR AGENTE

### CodeAnalysisAgent
- **Estado:** {code_agent_status}
- **Issues Encontrados:** {code_issues}
- **Recomendaciones:** {code_recommendations}
- **Tiempo de Ejecución:** {code_time:.2f}s

### ResearchAgent
- **Estado:** {research_agent_status}
- **Fuentes Consultadas:** {research_sources}
- **Información Relevante:** {research_findings}
- **Tiempo de Ejecución:** {research_time:.2f}s

### AutomationAgent
- **Estado:** {automation_agent_status}
- **Workflows Optimizados:** {automation_workflows}
- **Mejoras Aplicadas:** {automation_improvements}
- **Tiempo de Ejecución:** {automation_time:.2f}s

### DevOpsAgent
- **Estado:** {devops_agent_status}
- **Infraestructura Analizada:** {devops_infrastructure}
- **Optimizaciones:** {devops_optimizations}
- **Tiempo de Ejecución:** {devops_time:.2f}s
""",
            'issues_critical': """
## 🚨 ISSUES CRÍTICOS

{issues_table}
""",
            'recommendations': """
## 💡 RECOMENDACIONES PRIORITARIAS

{recommendations_list}
""",
            'performance': """
## ⚡ ANÁLISIS DE PERFORMANCE

### Métricas de Rendimiento
- **CPU Usage:** {cpu_usage:.1f}%
- **Memory Usage:** {memory_usage:.1f}%
- **Response Time:** {response_time:.2f}ms
- **Throughput:** {throughput:.1f} req/s

### Optimizaciones Sugeridas
{performance_optimizations}
""",
            'security': """
## 🔒 ANÁLISIS DE SEGURIDAD

### Vulnerabilidades Encontradas
{security_issues}

### Recomendaciones de Seguridad
{security_recommendations}
""",
            'research_findings': """
## 🔬 HALLAZGOS DE INVESTIGACIÓN

### Tecnologías Emergentes
{emerging_technologies}

### Mejores Prácticas
{best_practices}

### Implementaciones Sugeridas
{suggested_implementations}
""",
            'footer': """
---

## 📈 MÉTRICAS HISTÓRICAS

```json
{metrics_json}
```

**Generado por:** Nuclear Multi-Agent System v2025
**Versión:** {version}
**Commit:** {commit_sha}
"""
        }

    def generate_report(self, analysis_results: Dict[str, Any],
                       metrics: Dict[str, Any] = None,
                       template_name: str = 'nuclear_template.md') -> str:

        # Calcular métricas
        quality_score = analysis_results.get('quality_score', 0)
        critical_issues = len([i for i in analysis_results.get('errors', [])
                              if i.get('severity') == 'critical'])
        warning_issues = len([i for i in analysis_results.get('warnings', [])
                             if i.get('severity') == 'warning'])

        # Determinar estados
        quality_status = "🟢 Excelente" if quality_score > 85 else "🟡 Bueno" if quality_score > 70 else "🔴 Requiere Atención"
        critical_status = "✅ Ninguno" if critical_issues == 0 else f"🚨 {critical_issues} críticos"
        warning_status = "✅ Ninguno" if warning_issues == 0 else f"⚠️ {warning_issues} advertencias"

        # Calcular cobertura y tiempo
        coverage = analysis_results.get('coverage', 0.85)
        coverage_status = "🟢 Completa" if coverage > 0.8 else "🟡 Parcial"
        analysis_time = analysis_results.get('analysis_time', 0)

        # Calcular puntuación general
        overall_score = (
            quality_score * 0.4 +
            (100 - critical_issues * 10) * 0.3 +
            (100 - warning_issues * 5) * 0.2 +
            coverage * 100 * 0.1
        )

        # Generar secciones
        report = self.templates['header'].format(
            timestamp=datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        )

        report += self.templates['summary'].format(
            quality_score=quality_score,
            quality_status=quality_status,
            critical_issues=critical_issues,
            critical_status=critical_status,
            warning_issues=warning_issues,
            warning_status=warning_status,
            coverage=coverage,
            coverage_status=coverage_status,
            analysis_time=analysis_time,
            overall_score=min(overall_score, 100)
        )

        # Análisis por agente (simulado basado en resultados)
        report += self.templates['agent_analysis'].format(
            code_agent_status="✅ Activo",
            code_issues=len(analysis_results.get('errors', [])),
            code_recommendations=len(analysis_results.get('recommendations', [])),
            code_time=analysis_results.get('code_analysis_time', 0),
            research_agent_status="✅ Activo",
            research_sources=len(analysis_results.get('research_sources', [])),
            research_findings=len(analysis_results.get('research_findings', [])),
            research_time=analysis_results.get('research_time', 0),
            automation_agent_status="✅ Activo",
            automation_workflows=len(analysis_results.get('automation_workflows', [])),
            automation_improvements=len(analysis_results.get('automation_improvements', [])),
            automation_time=analysis_results.get('automation_time', 0),
            devops_agent_status="✅ Activo",
            devops_infrastructure=len(analysis_results.get('infrastructure_components', [])),
            devops_optimizations=len(analysis_results.get('devops_optimizations', [])),
            devops_time=analysis_results.get('devops_time', 0)
        )

        # Issues críticos
        if critical_issues > 0:
            issues_table = self._generate_issues_table(analysis_results.get('errors', []))
            report += self.templates['issues_critical'].format(issues_table=issues_table)

        # Recomendaciones
        recommendations = self._generate_recommendations(analysis_results.get('recommendations', []))
        report += self.templates['recommendations'].format(recommendations_list=recommendations)

        # Performance
        report += self.templates['performance'].format(
            cpu_usage=metrics.get('cpu_usage', 0) if metrics else 0,
            memory_usage=metrics.get('memory_usage', 0) if metrics else 0,
            response_time=metrics.get('response_time', 0) if metrics else 0,
            throughput=metrics.get('throughput', 0) if metrics else 0,
            performance_optimizations=self._generate_performance_optimizations(analysis_results)
        )

        # Security
        report += self.templates['security'].format(
            security_issues=self._generate_security_issues(analysis_results.get('security_issues', [])),
            security_recommendations=self._generate_security_recommendations(analysis_results)
        )

        # Research findings
        report += self.templates['research_findings'].format(
            emerging_technologies=self._generate_emerging_technologies(analysis_results.get('research_findings', [])),
            best_practices=self._generate_best_practices(analysis_results),
            suggested_implementations=self._generate_suggested_implementations(analysis_results)
        )

        # Footer
        report += self.templates['footer'].format(
            metrics_json=json.dumps(metrics or {}, indent=2),
            version="2025.1.0",
            commit_sha="abc123def456"
        )

        return report

    def _generate_issues_table(self, errors: List[Dict]) -> str:
        if not errors:
            return "✅ No se encontraron issues críticos."

        table = "| Severidad | Archivo | Línea | Descripción | Solución |\n"
        table += "|-----------|---------|-------|-------------|----------|\n"

        for error in errors[:10]:  # Limitar a 10 issues
            severity = error.get('severity', 'unknown')
            file = error.get('file', 'unknown')
            line = error.get('line', 'N/A')
            desc = error.get('message', '')[:50] + '...' if len(error.get('message', '')) > 50 else error.get('message', '')
            solution = error.get('solution', 'Revisar manualmente')[:30] + '...' if len(error.get('solution', '')) > 30 else error.get('solution', '')

            emoji = "🔴" if severity == 'critical' else "🟡" if severity == 'warning' else "🔵"
            table += f"| {emoji} {severity} | {file} | {line} | {desc} | {solution} |\n"

        return table

    def _generate_recommendations(self, recommendations: List[Dict]) -> str:
        if not recommendations:
            return "✅ No hay recomendaciones adicionales."

        rec_list = ""
        for i, rec in enumerate(recommendations[:5], 1):
            priority = rec.get('priority', 'medium')
            emoji = "🔴" if priority == 'high' else "🟡" if priority == 'medium' else "🔵"
            rec_list += f"{i}. {emoji} **{rec.get('title', 'Recomendación')}**\n"
            rec_list += f"   {rec.get('description', '')}\n\n"

        return rec_list

    def _generate_performance_optimizations(self, analysis_results: Dict) -> str:
        optimizations = analysis_results.get('performance_optimizations', [])
        if not optimizations:
            return "- ✅ El código está optimizado"

        opt_list = ""
        for opt in optimizations:
            opt_list += f"- {opt.get('description', '')}\n"
        return opt_list

    def _generate_security_issues(self, security_issues: List[Dict]) -> str:
        if not security_issues:
            return "✅ No se encontraron vulnerabilidades de seguridad."

        issues = ""
        for issue in security_issues:
            issues += f"- **{issue.get('severity', 'unknown')}**: {issue.get('description', '')}\n"
        return issues

    def _generate_security_recommendations(self, analysis_results: Dict) -> str:
        return """- 🔐 Implementar autenticación robusta
- 🛡️ Actualizar dependencias vulnerables
- 🔒 Usar HTTPS en todas las comunicaciones
- 📝 Implementar logging seguro
- 🧪 Agregar pruebas de seguridad"""

    def _generate_emerging_technologies(self, research_findings: List[Dict]) -> str:
        if not research_findings:
            return "- 🔍 Investigando tecnologías emergentes..."

        tech_list = ""
        for finding in research_findings[:3]:
            tech_list += f"- **{finding.get('technology', '')}**: {finding.get('description', '')}\n"
        return tech_list

    def _generate_best_practices(self, analysis_results: Dict) -> str:
        return """- 🚀 Usar async/await para operaciones I/O
- 📦 Implementar manejo robusto de errores
- 🔧 Usar configuración centralizada
- 📊 Implementar métricas y monitoreo
- 🧪 Escribir pruebas automatizadas"""

    def _generate_suggested_implementations(self, analysis_results: Dict) -> str:
        return """- 🤖 Integrar modelos de IA para análisis avanzado
- 🔄 Implementar arquitectura de microservicios
- 📈 Agregar sistema de caché distribuido
- 🔍 Implementar búsqueda semántica
- 📊 Dashboard de métricas en tiempo real"""

def main():
    parser = argparse.ArgumentParser(description='Generar reporte avanzado de Nuclear Crawler')
    parser.add_argument('--input', '-i', required=True, help='Archivo JSON con resultados de análisis')
    parser.add_argument('--metrics', '-m', help='Archivo JSON con métricas adicionales')
    parser.add_argument('--template', '-t', default='nuclear_template.md', help='Plantilla a usar')
    parser.add_argument('--output', '-o', required=True, help='Archivo de salida')

    args = parser.parse_args()

    # Cargar resultados de análisis
    with open(args.input, 'r', encoding='utf-8') as f:
        analysis_results = json.load(f)

    # Cargar métricas adicionales si existen
    metrics = None
    if args.metrics and Path(args.metrics).exists():
        with open(args.metrics, 'r', encoding='utf-8') as f:
            metrics = json.load(f)

    # Generar reporte
    generator = NuclearReportGenerator()
    report = generator.generate_report(analysis_results, metrics, args.template)

    # Guardar reporte
    with open(args.output, 'w', encoding='utf-8') as f:
        f.write(report)

    print(f"✅ Reporte avanzado generado: {args.output}")

if __name__ == '__main__':
    main()
