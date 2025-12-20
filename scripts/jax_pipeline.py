#!/usr/bin/env python3
"""
JAX Pipeline - Pipeline completo de procesamiento acelerado para Nuclear Crawler
Procesa datos masivamente usando JAX (GPU/TPU) y ahorra código
"""

import sys
import json
import argparse
import numpy as np
from typing import List, Dict, Any, Optional

try:
    import jax
    import jax.numpy as jnp
    from jax import jit, vmap, pmap
    JAX_AVAILABLE = True
except ImportError:
    JAX_AVAILABLE = False
    print("Warning: JAX not available, using NumPy fallback", file=sys.stderr)

class JAXPipeline:
    """Pipeline completo de procesamiento JAX"""

    def __init__(self, device='cpu'):
        self.device = device
        if JAX_AVAILABLE:
            if device == 'gpu':
                jax.config.update('jax_platform_name', 'gpu')
            elif device == 'tpu':
                jax.config.update('jax_platform_name', 'tpu')
            else:
                jax.config.update('jax_platform_name', 'cpu')

    @staticmethod
    @jit
    def calculate_relevance_scores(query_embeddings, content_embeddings):
        """Calcula scores de relevancia vectorizado"""
        # Simulación de embeddings (en producción usar modelos reales)
        scores = jnp.dot(query_embeddings, content_embeddings.T)
        return scores

    @staticmethod
    @jit
    def calculate_quality_scores(content_features):
        """Calcula scores de calidad vectorizado"""
        # content_features: [length, links_count, images_count, headings_count, ...]
        weights = jnp.array([0.3, 0.2, 0.2, 0.15, 0.15])  # Pesos para cada feature
        scores = jnp.dot(content_features, weights)
        return scores

    @staticmethod
    @jit
    def extract_features_vectorized(html_lengths, links_counts, images_counts, headings_counts, tables_counts):
        """Extrae features de contenido vectorizado"""
        features = jnp.stack([
            html_lengths / 10000.0,  # Normalizar
            links_counts / 100.0,
            images_counts / 50.0,
            headings_counts / 20.0,
            tables_counts / 10.0,
        ], axis=1)
        return features

    def process_batch(self, batch: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Procesa un batch de resultados de scraping"""
        if not JAX_AVAILABLE:
            return self._process_batch_numpy(batch)

        # Extraer datos
        html_lengths = jnp.array([len(item.get('html', '')) for item in batch], dtype=jnp.float32)
        links_counts = jnp.array([len(item.get('links', [])) for item in batch], dtype=jnp.float32)
        images_counts = jnp.array([len(item.get('images', [])) for item in batch], dtype=jnp.float32)
        headings_counts = jnp.array([item.get('headings_count', 0) for item in batch], dtype=jnp.float32)
        tables_counts = jnp.array([item.get('tables_count', 0) for item in batch], dtype=jnp.float32)

        # Calcular features
        features = self.extract_features_vectorized(
            html_lengths, links_counts, images_counts, headings_counts, tables_counts
        )

        # Calcular quality scores
        quality_scores = self.calculate_quality_scores(features)

        # Actualizar batch con scores
        results = []
        for i, item in enumerate(batch):
            item['quality_score'] = float(quality_scores[i])
            item['relevance_score'] = item.get('relevance_score', 0.5)
            item['total_score'] = float(quality_scores[i] * 0.6 + item.get('relevance_score', 0.5) * 0.4)
            results.append(item)

        return results

    def _process_batch_numpy(self, batch: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Fallback a NumPy"""
        html_lengths = np.array([len(item.get('html', '')) for item in batch], dtype=np.float32)
        links_counts = np.array([len(item.get('links', [])) for item in batch], dtype=np.float32)
        images_counts = np.array([len(item.get('images', [])) for item in batch], dtype=np.float32)

        # Calcular scores simples
        quality_scores = (html_lengths / 10000.0 * 0.3 +
                         links_counts / 100.0 * 0.2 +
                         images_counts / 50.0 * 0.2)

        results = []
        for i, item in enumerate(batch):
            item['quality_score'] = float(quality_scores[i])
            item['relevance_score'] = item.get('relevance_score', 0.5)
            item['total_score'] = float(quality_scores[i] * 0.6 + item.get('relevance_score', 0.5) * 0.4)
            results.append(item)

        return results

    def vectorize_urls(self, urls: List[str], operation: str = 'normalize') -> List[str]:
        """Vectoriza operaciones sobre URLs"""
        if operation == 'normalize':
            # Normalizar URLs (remover fragmentos, etc.)
            return [url.split('#')[0].split('?')[0] for url in urls]
        elif operation == 'extract_domain':
            # Extraer dominios
            return [url.split('/')[2] if len(url.split('/')) > 2 else url for url in urls]
        return urls

    def batch_process_urls(self, urls: List[str], batch_size: int = 32) -> List[Dict[str, Any]]:
        """Procesa URLs en batches"""
        results = []
        for i in range(0, len(urls), batch_size):
            batch = urls[i:i+batch_size]
            # Procesar batch (normalizar, extraer features, etc.)
            processed = self.vectorize_urls(batch, 'normalize')
            results.extend([{'url': url, 'normalized': True} for url in processed])
        return results

    def pipeline_process(self, data: List[Dict[str, Any]], stages: List[str]) -> List[Dict[str, Any]]:
        """Pipeline completo de procesamiento"""
        result = data

        for stage in stages:
            if stage == 'normalize':
                # Normalizar URLs
                for item in result:
                    if 'url' in item:
                        item['url'] = item['url'].split('#')[0].split('?')[0]

            elif stage == 'extract_features':
                # Extraer features
                batch = self.process_batch(result)
                result = batch

            elif stage == 'calculate_scores':
                # Calcular scores
                batch = self.process_batch(result)
                result = batch

            elif stage == 'rank':
                # Ordenar por score
                result = sorted(result, key=lambda x: x.get('total_score', 0), reverse=True)

            elif stage == 'filter':
                # Filtrar por score mínimo
                result = [item for item in result if item.get('total_score', 0) > 0.5]

        return result

    def extract_pdf_text(self, pdf_data: bytes) -> str:
        """Extrae texto de PDF usando procesamiento básico (sin librerías externas)"""
        try:
            # Simulación básica: buscar patrones de texto en bytes
            # En producción, usar pdfplumber o similar, pero aquí usamos lo básico
            text = pdf_data.decode('latin-1', errors='ignore')
            # Extraer líneas que parecen texto (heurística simple)
            lines = [line.strip() for line in text.split('\n') if len(line.strip()) > 10 and not line.startswith('%')]
            return '\n'.join(lines[:100])  # Limitar a 100 líneas
        except Exception as e:
            return f"Error extracting PDF text: {str(e)}"


def main():
    parser = argparse.ArgumentParser(description='JAX Pipeline for Nuclear Crawler')
    parser.add_argument('--device', default='cpu', choices=['cpu', 'gpu', 'tpu'],
                       help='Device to use (cpu, gpu, tpu)')
    parser.add_argument('--operation', required=True,
                       choices=['process_batch', 'vectorize_urls', 'batch_process_urls', 'pipeline', 'extract_pdf'],
                       help='Operation to perform')
    parser.add_argument('--data', required=True, help='JSON array of data')
    parser.add_argument('--stages', help='Pipeline stages (comma-separated)')
    parser.add_argument('--batch_size', type=int, default=32, help='Batch size')

    args = parser.parse_args()

    # Parsear datos
    try:
        data = json.loads(args.data)
    except json.JSONDecodeError:
        print(json.dumps({"error": "Invalid JSON data"}), file=sys.stderr)
        sys.exit(1)

    # Crear pipeline
    pipeline = JAXPipeline(device=args.device)

    # Procesar
    try:
        if args.operation == 'process_batch':
            result = pipeline.process_batch(data)

        elif args.operation == 'vectorize_urls':
            result = pipeline.vectorize_urls(data)

        elif args.operation == 'batch_process_urls':
            result = pipeline.batch_process_urls(data, args.batch_size)

        elif args.operation == 'pipeline':
            stages = args.stages.split(',') if args.stages else ['normalize', 'extract_features', 'calculate_scores', 'rank']
            result = pipeline.pipeline_process(data, stages)

        elif args.operation == 'extract_pdf':
            # Leer PDF data de stdin
            pdf_data = sys.stdin.buffer.read()
            result = pipeline.extract_pdf_text(pdf_data)

        else:
            result = data

        # Output JSON
        print(json.dumps(result))

    except Exception as e:
        print(json.dumps({"error": str(e)}), file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()

