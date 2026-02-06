#!/usr/bin/env python3
"""
Synthetic Data Detector
Detecta si un texto es REAL (humano) o SINTÉTICO (generado por IA)
"""

import re
from typing import Dict, Tuple
import numpy as np

class SyntheticDetector:
    """Detector de contenido sintético vs real"""
    
    # Patrones de IA (frases que usan los LLMs)
    AI_PATTERNS = [
        r"as an ai",
        r"i'?m an ai",
        r"i'?m here to help",
        r"i'?d be happy to",
        r"i'?m sorry,? but",
        r"i don'?t have the ability",
        r"i can'?t provide",
        r"it'?s important to note",
        r"however,? it'?s worth mentioning",
        r"in conclusion",
        r"to summarize",
        r"as mentioned (earlier|previously)",
        r"let me explain",
        r"allow me to",
        r"please note that",
        r"keep in mind that",
        r"it'?s crucial to understand",
        r"consider the following",
        r"here are some key points",
        r"to put it simply",
    ]
    
    # Patrones humanos (conversaciones reales)
    HUMAN_PATTERNS = [
        r"\bjaja\b",          # Risa en español
        r"\bhaha\b",          # Risa en inglés
        r"\blol\b",           # Internet slang
        r"\blmao\b",
        r"\bromfl\b",
        r"\btbh\b",           # To be honest
        r"\bimo\b",           # In my opinion
        r"\bidk\b",           # I don't know
        r"\bwtf\b",
        r"\bomg\b",
        r"\bbruh\b",
        r"\bngl\b",           # Not gonna lie
        r"\bfr\b",            # For real
        r"\[deleted\]",       # Reddit
        r"\[removed\]",       # Reddit
        r"\.\.\.+",           # Pausas naturales
        r"!{2,}",             # Múltiples exclamaciones
        r"\?{2,}",            # Múltiples preguntas
    ]
    
    # Emojis (humanos los usan más)
    EMOJI_PATTERN = r"[\U0001F600-\U0001F64F\U0001F300-\U0001F5FF\U0001F680-\U0001F6FF\U0001F1E0-\U0001F1FF]"
    
    def __init__(self):
        self.ai_regex = [re.compile(p, re.IGNORECASE) for p in self.AI_PATTERNS]
        self.human_regex = [re.compile(p, re.IGNORECASE) for p in self.HUMAN_PATTERNS]
        self.emoji_regex = re.compile(self.EMOJI_PATTERN)
    
    def detect(self, text: str) -> Tuple[bool, float, Dict]:
        """
        Detecta si el texto es sintético o real
        
        Returns:
            (is_real: bool, confidence: float, details: dict)
        """
        
        if not text or len(text) < 10:
            return (False, 0.0, {"reason": "texto_muy_corto"})
        
        # Calcular features
        ai_score = self._calculate_ai_score(text)
        human_score = self._calculate_human_score(text)
        style_score = self._calculate_style_score(text)
        
        # Score total (0-100)
        total_score = (human_score * 0.5) + (style_score * 0.3) - (ai_score * 0.2)
        
        # Normalizar 0-100
        total_score = max(0, min(100, total_score))
        
        # Threshold: >60 = real, <40 = synthetic
        is_real = total_score > 60
        confidence = abs(total_score - 50) / 50  # 0-1
        
        details = {
            "total_score": total_score,
            "ai_score": ai_score,
            "human_score": human_score,
            "style_score": style_score,
            "is_real": is_real,
            "confidence": confidence
        }
        
        return (is_real, confidence, details)
    
    def _calculate_ai_score(self, text: str) -> float:
        """Score de patrones de IA (más alto = más sintético)"""
        matches = 0
        text_lower = text.lower()
        
        for regex in self.ai_regex:
            if regex.search(text_lower):
                matches += 1
        
        # Normalizar por longitud
        score = (matches / len(self.ai_regex)) * 100
        
        return min(100, score)
    
    def _calculate_human_score(self, text: str) -> float:
        """Score de patrones humanos (más alto = más real)"""
        matches = 0
        text_lower = text.lower()
        
        for regex in self.human_regex:
            if regex.search(text_lower):
                matches += 1
        
        # Contar emojis
        emoji_count = len(self.emoji_regex.findall(text))
        
        # Bonus por emojis (pero no demasiados)
        emoji_bonus = min(emoji_count * 5, 20)
        
        score = (matches / len(self.human_regex)) * 80 + emoji_bonus
        
        return min(100, score)
    
    def _calculate_style_score(self, text: str) -> float:
        """Score de estilo natural (typos, contractions, etc.)"""
        score = 0
        
        # Contractions (humans use them more)
        contractions = [
            r"\bcan't\b", r"\bwon't\b", r"\bdon't\b", r"\bdidn't\b",
            r"\bisn't\b", r"\baren't\b", r"\bwasn't\b", r"\bweren't\b",
            r"\bi'm\b", r"\byou're\b", r"\bhe's\b", r"\bshe's\b",
            r"\bit's\b", r"\bwe're\b", r"\bthey're\b"
        ]
        
        contraction_count = 0
        for pattern in contractions:
            contraction_count += len(re.findall(pattern, text, re.IGNORECASE))
        
        score += min(contraction_count * 5, 30)
        
        # Variación en longitud de oraciones (humanos varían más)
        sentences = re.split(r'[.!?]+', text)
        if len(sentences) > 2:
            lengths = [len(s.split()) for s in sentences if s.strip()]
            if lengths:
                variance = np.var(lengths)
                score += min(variance * 2, 30)
        
        # Capitalización inconsistente (humanos son menos perfectos)
        words = text.split()
        if len(words) > 10:
            caps_variations = sum(1 for w in words if w and w[0].isupper())
            caps_ratio = caps_variations / len(words)
            # Humanos tienen ratio entre 0.1-0.3
            if 0.1 <= caps_ratio <= 0.3:
                score += 20
        
        # Typos comunes (detectar algunos)
        common_typos = [
            r"\bteh\b", r"\bthe\b",  # the/teh
            r"\brecieve\b",          # receive (mal escrito)
            r"\boccured\b",          # occurred (mal escrito)
            r"\bwont\b",             # won't sin apóstrofe
            r"\bcant\b",             # can't sin apóstrofe
        ]
        
        typo_count = 0
        for pattern in common_typos:
            typo_count += len(re.findall(pattern, text, re.IGNORECASE))
        
        # Typos = human (pero no demasiados)
        score += min(typo_count * 5, 20)
        
        return min(100, score)
    
    def batch_detect(self, texts: list) -> list:
        """Detecta múltiples textos en batch"""
        return [self.detect(text) for text in texts]
    
    def get_real_conversations(self, conversations: list) -> list:
        """Filtra solo conversaciones reales"""
        real_convs = []
        
        for conv in conversations:
            text = conv.get('text') or conv.get('body') or conv.get('message') or ''
            is_real, confidence, details = self.detect(text)
            
            if is_real and confidence > 0.6:
                conv['is_real'] = True
                conv['authenticity_score'] = details['total_score']
                conv['confidence'] = confidence
                real_convs.append(conv)
        
        return real_convs


def main():
    """Demo de uso"""
    detector = SyntheticDetector()
    
    # Test 1: Texto de IA
    ai_text = """
    I'd be happy to help you with that. As an AI language model, 
    I can provide information on various topics. It's important to 
    note that you should always verify information from multiple sources.
    """
    
    is_real, confidence, details = detector.detect(ai_text)
    print("Test 1 (IA):")
    print(f"  Real: {is_real}")
    print(f"  Confidence: {confidence:.2f}")
    print(f"  Score: {details['total_score']:.2f}")
    print()
    
    # Test 2: Texto humano
    human_text = """
    jaja sí, me pasó lo mismo el otro día... no puedo creer que hiciste eso lol
    fue demasiado wtf. pero bueno, al final salió bien 😊 creo que si lo haces 
    así tmb te va a funcionar
    """
    
    is_real, confidence, details = detector.detect(human_text)
    print("Test 2 (Humano):")
    print(f"  Real: {is_real}")
    print(f"  Confidence: {confidence:.2f}")
    print(f"  Score: {details['total_score']:.2f}")
    print()
    
    # Test 3: Batch processing
    texts = [ai_text, human_text]
    results = detector.batch_detect(texts)
    
    print("Batch results:")
    for i, (is_real, conf, det) in enumerate(results):
        print(f"  Text {i+1}: Real={is_real}, Score={det['total_score']:.1f}")


if __name__ == '__main__':
    main()
