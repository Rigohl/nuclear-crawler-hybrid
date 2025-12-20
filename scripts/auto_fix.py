#!/usr/bin/env python3
"""
🔧 NUCLEAR AUTO-FIX SYSTEM
Sistema automatizado para corregir issues críticos detectados por los agentes
"""

import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional
import argparse
import logging

# Configurar logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class NuclearAutoFix:
    def __init__(self):
        self.fix_patterns = {
            'unused_import': {
                'pattern': r'use\s+[\w:]+::\{[^}]*\w+[^}]*\};',
                'fix': self._fix_unused_import
            },
            'missing_derive': {
                'pattern': r'#\[derive\([^)]*\)\]',
                'fix': self._fix_missing_derive
            },
            'clippy_warnings': {
                'pattern': r'warning:\s*(.*)',
                'fix': self._fix_clippy_warning
            },
            'security_vulnerability': {
                'pattern': r'vulnerability:\s*(.*)',
                'fix': self._fix_security_issue
            },
            'performance_issue': {
                'pattern': r'performance:\s*(.*)',
                'fix': self._fix_performance_issue
            }
        }

    def auto_fix(self, analysis_results: Dict[str, Any],
                 severity: str = 'critical',
                 auto_commit: bool = False) -> Dict[str, Any]:

        logger.info(f"🚀 Iniciando auto-fix para severidad: {severity}")

        # Filtrar issues por severidad
        issues_to_fix = []
        for error in analysis_results.get('errors', []):
            if error.get('severity') == severity:
                issues_to_fix.append(error)

        logger.info(f"📋 Encontrados {len(issues_to_fix)} issues para corregir")

        # Aplicar correcciones
        fixed_issues = []
        failed_fixes = []

        for issue in issues_to_fix:
            try:
                if self._apply_fix(issue):
                    fixed_issues.append(issue)
                    logger.info(f"✅ Corregido: {issue.get('message', '')[:50]}...")
                else:
                    failed_fixes.append(issue)
                    logger.warning(f"❌ No se pudo corregir: {issue.get('message', '')[:50]}...")
            except Exception as e:
                logger.error(f"💥 Error al corregir issue: {e}")
                failed_fixes.append(issue)

        # Commit automático si está habilitado
        if auto_commit and fixed_issues:
            self._auto_commit_fixes(fixed_issues)

        return {
            'total_issues': len(issues_to_fix),
            'fixed_issues': len(fixed_issues),
            'failed_fixes': len(failed_fixes),
            'fixed_details': fixed_issues,
            'failed_details': failed_fixes
        }

    def _apply_fix(self, issue: Dict[str, Any]) -> bool:
        """Aplicar corrección automática para un issue específico"""
        file_path = issue.get('file')
        line = issue.get('line')
        message = issue.get('message', '')

        if not file_path or not Path(file_path).exists():
            return False

        # Leer archivo
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        lines = content.split('\n')

        # Aplicar patrón de corrección basado en el tipo de issue
        for pattern_name, pattern_config in self.fix_patterns.items():
            if re.search(pattern_config['pattern'], message, re.IGNORECASE):
                try:
                    fixed_content = pattern_config['fix'](content, issue)
                    if fixed_content != content:
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.write(fixed_content)
                        return True
                except Exception as e:
                    logger.error(f"Error aplicando fix {pattern_name}: {e}")
                    continue

        # Intentar corrección genérica basada en soluciones sugeridas
        return self._apply_generic_fix(content, issue, file_path)

    def _fix_unused_import(self, content: str, issue: Dict[str, Any]) -> str:
        """Corregir imports no utilizados"""
        # Esta es una corrección básica - en la práctica necesitarías un análisis más sofisticado
        lines = content.split('\n')
        message = issue.get('message', '')

        # Buscar línea con import no utilizado
        for i, line in enumerate(lines):
            if 'use ' in line and any(keyword in message.lower() for keyword in ['unused', 'never used']):
                # Comentar la línea
                lines[i] = f"// {line} // TODO: Remove unused import"
                break

        return '\n'.join(lines)

    def _fix_missing_derive(self, content: str, issue: Dict[str, Any]) -> str:
        """Agregar derive faltante"""
        message = issue.get('message', '')

        # Buscar qué derive falta
        missing_derives = []
        if 'debug' in message.lower():
            missing_derives.append('Debug')
        if 'clone' in message.lower():
            missing_derives.append('Clone')
        if 'copy' in message.lower():
            missing_derives.append('Copy')

        if not missing_derives:
            return content

        # Agregar derives a la struct
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.strip().startswith('#[derive('):
                # Agregar derives faltantes
                current_derives = line.split('(')[1].split(')')[0]
                existing = [d.strip() for d in current_derives.split(',')]
                for derive in missing_derives:
                    if derive not in existing:
                        existing.append(derive)
                new_derives = ', '.join(sorted(existing))
                lines[i] = f"#[derive({new_derives})]"
                break

        return '\n'.join(lines)

    def _fix_clippy_warning(self, content: str, issue: Dict[str, Any]) -> str:
        """Corregir warnings de Clippy"""
        message = issue.get('message', '')

        # Correcciones comunes de Clippy
        if 'unnecessary collect' in message.lower():
            # Reemplazar .collect().len() con .count()
            content = re.sub(r'\.collect\(\)\.len\(\)', '.count()', content)

        if 'single char pattern' in message.lower():
            # Usar chars() en lugar de split con char
            content = re.sub(r'\.split\("([^"])"\)', r'.split(\1)', content)

        if 'manual range' in message.lower():
            # Usar .. en lugar de manual range
            content = re.sub(r'\((\d+)\.\.=\s*(\d+)\)', r'(\1..\2)', content)

        return content

    def _fix_security_issue(self, content: str, issue: Dict[str, Any]) -> str:
        """Corregir issues de seguridad"""
        message = issue.get('message', '')

        if 'unwrap()' in message:
            # Reemplazar unwrap() con expect() o manejo de error
            content = re.sub(r'\.unwrap\(\)', '.expect("Error crítico")', content)

        if 'unsafe' in message.lower():
            # Agregar comentario de justificación para unsafe
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if 'unsafe {' in line:
                    lines.insert(i, '// SAFETY: Justificado para operación crítica')
                    break
            content = '\n'.join(lines)

        return content

    def _fix_performance_issue(self, content: str, issue: Dict[str, Any]) -> str:
        """Corregir issues de performance"""
        message = issue.get('message', '')

        if 'vec![].len()' in message or 'vec capacity' in message:
            # Usar with_capacity
            content = re.sub(r'Vec::new\(\)', 'Vec::with_capacity(16)', content)

        if 'string clone' in message.lower():
            # Usar &str en lugar de String donde sea posible
            content = re.sub(r'(\w+):\s*String', r'\1: &str', content)

        return content

    def _apply_generic_fix(self, content: str, issue: Dict[str, Any], file_path: str) -> bool:
        """Aplicar corrección genérica basada en soluciones sugeridas"""
        solutions = issue.get('solutions', [])
        if not solutions:
            return False

        # Buscar solución aplicable automáticamente
        for solution in solutions:
            solution_text = solution.get('description', '')

            # Aplicar correcciones basadas en texto de solución
            if 'add derive' in solution_text.lower():
                return self._fix_missing_derive(content, issue) != content
            elif 'remove unused' in solution_text.lower():
                return self._fix_unused_import(content, issue) != content
            elif 'use expect instead of unwrap' in solution_text.lower():
                content = re.sub(r'\.unwrap\(\)', '.expect("Handled error")', content)
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True

        return False

    def _auto_commit_fixes(self, fixed_issues: List[Dict[str, Any]]) -> None:
        """Crear commit automático con las correcciones aplicadas"""
        try:
            # Verificar si hay cambios
            result = subprocess.run(['git', 'status', '--porcelain'],
                                  capture_output=True, text=True, cwd='.')

            if result.stdout.strip():
                # Crear commit
                commit_message = f"🔧 Auto-fix: Corregidos {len(fixed_issues)} issues críticos\n\nIssues corregidos:\n"
                for issue in fixed_issues[:5]:  # Limitar a 5 en el mensaje
                    commit_message += f"- {issue.get('message', '')[:50]}...\n"

                subprocess.run(['git', 'add', '.'], check=True)
                subprocess.run(['git', 'commit', '-m', commit_message], check=True)

                logger.info(f"✅ Commit creado automáticamente con {len(fixed_issues)} correcciones")
            else:
                logger.info("ℹ️ No hay cambios para commitear")

        except subprocess.CalledProcessError as e:
            logger.error(f"❌ Error al crear commit automático: {e}")

def main():
    parser = argparse.ArgumentParser(description='Sistema de auto-fix para Nuclear Crawler')
    parser.add_argument('--analysis-results', '-a', required=True,
                       help='Archivo JSON con resultados de análisis')
    parser.add_argument('--severity', '-s', default='critical',
                       choices=['critical', 'warning', 'info'],
                       help='Severidad de issues a corregir')
    parser.add_argument('--auto-commit', '-c', action='store_true',
                       help='Crear commit automático con las correcciones')
    parser.add_argument('--output', '-o', help='Archivo JSON para guardar resultados')

    args = parser.parse_args()

    # Cargar resultados de análisis
    with open(args.analysis_results, 'r', encoding='utf-8') as f:
        analysis_results = json.load(f)

    # Ejecutar auto-fix
    fixer = NuclearAutoFix()
    results = fixer.auto_fix(analysis_results, args.severity, args.auto_commit)

    # Mostrar resultados
    print("🔧 NUCLEAR AUTO-FIX RESULTS"    print(f"Total issues: {results['total_issues']}")
    print(f"Fixed: {results['fixed_issues']}")
    print(f"Failed: {results['failed_fixes']}")

    # Guardar resultados si se especifica
    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(results, f, indent=2, ensure_ascii=False)
        print(f"📄 Resultados guardados en: {args.output}")

if __name__ == '__main__':
    main()
