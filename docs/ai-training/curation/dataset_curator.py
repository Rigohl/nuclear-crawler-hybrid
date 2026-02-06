#!/usr/bin/env python3
"""
Dataset Curator
Cura datasets para AI training filtrando datos sintéticos y rankeando por calidad
"""

import json
import os
from typing import List, Dict
from datetime import datetime
from synthetic_detector import SyntheticDetector

class DatasetCurator:
    """Curador de datasets para AI training"""
    
    def __init__(self, min_quality_score: float = 60.0):
        self.detector = SyntheticDetector()
        self.min_quality_score = min_quality_score
        self.stats = {
            'total_processed': 0,
            'synthetic_filtered': 0,
            'low_quality_filtered': 0,
            'final_count': 0
        }
    
    def curate_dataset(self, conversations: List[Dict]) -> List[Dict]:
        """
        Cura dataset completo
        1. Detecta y filtra sintéticos
        2. Calcula quality scores
        3. Filtra por calidad mínima
        4. Rankea por calidad
        """
        
        print(f"🔍 Curando dataset de {len(conversations)} conversaciones...")
        self.stats['total_processed'] = len(conversations)
        
        # PASO 1: Filtrar sintéticos
        real_convs = self._filter_synthetic(conversations)
        print(f"   ✓ Paso 1: {len(real_convs)} conversaciones reales")
        
        # PASO 2: Calcular quality scores
        scored_convs = self._calculate_quality_scores(real_convs)
        print(f"   ✓ Paso 2: Scores calculados")
        
        # PASO 3: Filtrar por calidad mínima
        quality_convs = self._filter_by_quality(scored_convs)
        print(f"   ✓ Paso 3: {len(quality_convs)} conversaciones de calidad")
        
        # PASO 4: Rankear
        ranked_convs = self._rank_conversations(quality_convs)
        print(f"   ✓ Paso 4: Conversaciones rankeadas")
        
        self.stats['final_count'] = len(ranked_convs)
        
        return ranked_convs
    
    def _filter_synthetic(self, conversations: List[Dict]) -> List[Dict]:
        """Filtra conversaciones sintéticas"""
        real_convs = []
        
        for conv in conversations:
            # Extraer texto
            text = self._extract_text(conv)
            
            if not text:
                continue
            
            # Detectar si es real
            is_real, confidence, details = self.detector.detect(text)
            
            if is_real and confidence > 0.5:
                conv['is_real'] = True
                conv['authenticity_score'] = details['total_score']
                conv['detection_confidence'] = confidence
                real_convs.append(conv)
            else:
                self.stats['synthetic_filtered'] += 1
        
        return real_convs
    
    def _extract_text(self, conv: Dict) -> str:
        """Extrae texto de diferentes formatos de conversación"""
        # Reddit format
        if 'body' in conv:
            return conv['body']
        
        # Twitter format
        if 'text' in conv:
            return conv['text']
        
        # Chat format
        if 'message' in conv:
            return conv['message']
        
        # Generic
        if 'content' in conv:
            return conv['content']
        
        # Comentarios Reddit
        if 'comments' in conv and conv['comments']:
            # Usar primer comentario como proxy
            return conv['comments'][0].get('text', '')
        
        return ''
    
    def _calculate_quality_scores(self, conversations: List[Dict]) -> List[Dict]:
        """Calcula quality scores para cada conversación"""
        
        for conv in conversations:
            text = self._extract_text(conv)
            
            # Componentes del score
            length_score = self._calculate_length_score(text)
            engagement_score = self._calculate_engagement_score(conv)
            authenticity_score = conv.get('authenticity_score', 50.0)
            relevance_score = self._calculate_relevance_score(text, conv)
            
            # Score total (promedio ponderado)
            total_score = (
                length_score * 0.20 +
                engagement_score * 0.30 +
                authenticity_score * 0.30 +
                relevance_score * 0.20
            )
            
            conv['quality_score'] = total_score
            conv['quality_breakdown'] = {
                'length': length_score,
                'engagement': engagement_score,
                'authenticity': authenticity_score,
                'relevance': relevance_score
            }
        
        return conversations
    
    def _calculate_length_score(self, text: str) -> float:
        """Score basado en longitud (conversaciones más largas = mejor)"""
        if not text:
            return 0.0
        
        # Optimal length: 100-500 caracteres
        length = len(text)
        
        if length < 50:
            return (length / 50) * 40  # 0-40
        elif length < 500:
            return 40 + ((length - 50) / 450) * 60  # 40-100
        else:
            # Penalizar textos muy largos (posible spam)
            excess = length - 500
            penalty = min(excess / 1000, 0.3)  # Hasta 30% penalty
            return max(70, 100 * (1 - penalty))
    
    def _calculate_engagement_score(self, conv: Dict) -> float:
        """Score basado en engagement (upvotes, likes, etc.)"""
        score = 0.0
        
        # Reddit score
        if 'score' in conv:
            reddit_score = conv['score']
            score = min(reddit_score / 10, 100)  # 10 upvotes = 100 score
        
        # Twitter engagement
        if 'likes' in conv and 'retweets' in conv:
            twitter_score = conv['likes'] + (conv['retweets'] * 2)
            score = min(twitter_score / 20, 100)
        
        # Si no hay engagement data, asumir promedio
        if score == 0.0:
            score = 50.0
        
        return score
    
    def _calculate_relevance_score(self, text: str, conv: Dict) -> float:
        """Score basado en relevancia (keywords match)"""
        # Keywords importantes para marketing/sales
        keywords = [
            'marketing', 'sales', 'customer', 'client', 'business',
            'conversion', 'funnel', 'lead', 'prospect', 'pitch',
            'close', 'deal', 'objection', 'strategy', 'campaign'
        ]
        
        text_lower = text.lower()
        matches = sum(1 for kw in keywords if kw in text_lower)
        
        # 0-5 matches = 0-100 score
        score = min((matches / 5) * 100, 100)
        
        # Bonus si tiene source relevante
        source = conv.get('source', '') or conv.get('subreddit', '')
        relevant_sources = ['sales', 'marketing', 'entrepreneur', 'smallbusiness']
        
        if any(rs in source.lower() for rs in relevant_sources):
            score = min(score + 20, 100)
        
        return score
    
    def _filter_by_quality(self, conversations: List[Dict]) -> List[Dict]:
        """Filtra conversaciones por calidad mínima"""
        quality_convs = []
        
        for conv in conversations:
            if conv.get('quality_score', 0) >= self.min_quality_score:
                quality_convs.append(conv)
            else:
                self.stats['low_quality_filtered'] += 1
        
        return quality_convs
    
    def _rank_conversations(self, conversations: List[Dict]) -> List[Dict]:
        """Rankea conversaciones por quality score (descendente)"""
        return sorted(conversations, key=lambda x: x.get('quality_score', 0), reverse=True)
    
    def save_curated_dataset(self, conversations: List[Dict], output_dir: str = 'ai-training/curation/output'):
        """Guarda dataset curado"""
        os.makedirs(output_dir, exist_ok=True)
        
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        filename = f'curated_dataset_{timestamp}.json'
        filepath = os.path.join(output_dir, filename)
        
        # Dataset
        dataset = {
            'metadata': {
                'created_at': timestamp,
                'total_conversations': len(conversations),
                'curation_stats': self.stats,
                'min_quality_score': self.min_quality_score
            },
            'conversations': conversations
        }
        
        with open(filepath, 'w', encoding='utf-8') as f:
            json.dump(dataset, f, indent=2, ensure_ascii=False)
        
        print(f"\n✅ Dataset curado guardado: {filepath}")
        print(f"   Total conversaciones: {len(conversations)}")
        print(f"   Quality avg: {sum(c['quality_score'] for c in conversations) / len(conversations):.1f}")
    
    def print_stats(self):
        """Imprime estadísticas de curación"""
        print("\n" + "=" * 70)
        print("📊 ESTADÍSTICAS DE CURACIÓN")
        print("=" * 70)
        print(f"Total procesadas: {self.stats['total_processed']}")
        print(f"Sintéticas filtradas: {self.stats['synthetic_filtered']}")
        print(f"Baja calidad filtradas: {self.stats['low_quality_filtered']}")
        print(f"Final count: {self.stats['final_count']}")
        print(f"Ratio calidad: {(self.stats['final_count'] / self.stats['total_processed'] * 100):.1f}%")
        print("=" * 70)


def main():
    """Demo de uso"""
    print("=" * 70)
    print("  DATASET CURATOR")
    print("  Curación de datasets para AI training")
    print("=" * 70)
    print()
    
    # Cargar datos de ejemplo
    data_files = [
        'ai-training/scraping/output/reddit_marketing_real.json',
        'ai-training/scraping/output/twitter_marketers_real.json',
        'ai-training/osint/output/osint_results.json'
    ]
    
    all_conversations = []
    
    for file in data_files:
        if os.path.exists(file):
            with open(file, 'r', encoding='utf-8') as f:
                data = json.load(f)
                if isinstance(data, list):
                    all_conversations.extend(data)
                elif isinstance(data, dict) and 'conversations' in data:
                    all_conversations.extend(data['conversations'])
            print(f"✓ Cargado: {file} ({len(all_conversations)} total)")
    
    if not all_conversations:
        print("\n⚠️  No se encontraron datos. Ejecuta primero los scrapers:")
        print("   python ai-training/scraping/reddit_scraper.py")
        print("   python ai-training/scraping/twitter_scraper.py")
        return
    
    # Curar dataset
    curator = DatasetCurator(min_quality_score=65.0)
    curated = curator.curate_dataset(all_conversations)
    
    # Guardar
    curator.save_curated_dataset(curated)
    
    # Estadísticas
    curator.print_stats()
    
    # Mostrar top 5
    print("\n🏆 TOP 5 CONVERSACIONES:")
    for i, conv in enumerate(curated[:5]):
        print(f"\n#{i+1} - Score: {conv['quality_score']:.1f}")
        text = curator._extract_text(conv)[:100] + "..."
        print(f"   {text}")


if __name__ == '__main__':
    main()
