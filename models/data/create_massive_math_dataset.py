#!/usr/bin/env python3
"""
CREADOR DE DATASET MATEMÁTICO MASIVO
Procesa todo el contenido matemático y crea dataset comprehensivo
"""

import os
import json
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional
import hashlib

class MassiveMathDatasetCreator:
    """
    Creador de dataset matemático masivo que incluye:
    - Todo el código fuente matemático
    - Teorías y libros de referencia
    - Problemas para resolver
    - Tutoriales de Julia
    - Aplicaciones prácticas
    - Información Six Sigma
    """

    def __init__(self, math_root: str, output_dir: str):
        self.math_root = Path(math_root)
        self.output_dir = Path(output_dir)
        self.dataset = []
        self.processed_files = set()

        # Crear estructura de salida
        self._create_output_structure()

    def _create_output_structure(self):
        """Crear estructura de directorios para el dataset"""
        subdirs = [
            "raw_content",
            "theories_books",
            "problems_solutions",
            "julia_tutorials",
            "six_sigma",
            "research_centers",
            "applications"
        ]

        for subdir in subdirs:
            (self.output_dir / subdir).mkdir(parents=True, exist_ok=True)

    def process_all_mathematical_content(self):
        """Procesar todo el contenido matemático disponible"""
        print("INICIANDO CREACION DE DATASET MATEMATICO MASIVO EXPANDIDO")
        print("=" * 60)

        # 1. Procesar carpetas principales
        self._process_main_folders()

        # 2. Procesar contenido específico expandido
        self._process_quantum_content()
        self._process_six_sigma_content()
        self._process_information_theory_content()
        self._process_advanced_theory()
        self._process_decision_science()

        # 3. Agregar teorías y libros expandidos
        self._add_mathematical_theories()
        self._add_six_sigma_theories()
        self._add_quantum_theories_books()
        self._add_information_theory_books()
        self._add_debug_repair_training()
        self._add_mathematical_application_training()
        self._add_rust_julia_advanced_training()
        self._add_mojo_enrichment_training()
        self._add_julia_tutorials()

        # 4. Crear problemas y soluciones expandidos
        self._generate_problems_and_solutions()
        self._generate_quantum_problems()
        self._generate_six_sigma_problems()
        self._generate_information_theory_problems()
        self._generate_test_problems()

        # 5. Crear contenido de investigación
        self._create_research_center_content()

        # 6. Guardar dataset completo
        self._save_complete_dataset()

        print(f"\nDATASET COMPLETADO: {len(self.dataset)} registros")
        print(f"Guardado en: {self.output_dir}")

    def _process_main_folders(self):
        """Procesar las carpetas principales de matemáticas"""
        main_folders = [
            "01_BAYESIAN_DECISION",
            "01_CATEGORY_THEORY",
            "01_M_THEORY",
            "02_HOMOTOPY_TYPE_THEORY",
            "02_LINEAR_PROGRAMMING",
            "02_QUANTUM_FIELD_THEORY",
            "03_ADS_CFT",
            "03_GAME_THEORY",
            "03_NUMBER_THEORY",
            "04_DIFFERENTIAL_GEOMETRY",
            "04_OPTIMAL_CONTROL",
            "04_STATISTICAL_MECHANICS",
            "05_ALGEBRAIC_GEOMETRY",
            "05_LOOP_QUANTUM_GRAVITY",
            "05_MARKOV_DECISION",
            "06_INFORMATION_THEORY",
            "07_CONVEX_OPTIMIZATION",
            "08_QUEUEING_THEORY",
            "09_DESIGN_EXPERIMENTS",
            "10_BAYESIAN_OPTIMIZATION"
        ]

        for folder in main_folders:
            folder_path = self.math_root / folder
            if folder_path.exists():
                print(f"Procesando carpeta: {folder}")
                self._process_folder_content(folder_path, folder.replace("0", "").replace("_", " ").strip())

    def _process_quantum_content(self):
        """Procesar contenido de física cuántica"""
        quantum_path = self.math_root / "CUANTICA"
        if quantum_path.exists():
            print("Procesando contenido cuantico...")
            self._process_folder_content(quantum_path, "Quantum Physics")

            # Procesar archivos específicos
            quantum_files = [
                "PythonQuantumFullStack.py",
                "QuantumInformation.jl",
                "QuantumML.qrisp",
                "QuantumOptimization.qrisp",
                "QuantumStats.qrisp"
            ]

            for qfile in quantum_files:
                file_path = quantum_path / qfile
                if file_path.exists():
                    self._process_quantum_file(file_path)

    def _process_six_sigma_content(self):
        """Procesar contenido de Six Sigma expandido"""
        six_sigma_path = self.math_root / "SIXSIGMA"
        if six_sigma_path.exists():
            print("Procesando Six Sigma expandido...")
            self._process_folder_content(six_sigma_path, "Six Sigma")

            # Procesar archivos específicos
            sigma_files = [
                "src/Capability_Executive.py",
                "src/Capability_Pro.jl",
                "src/DOE_Taguchi.R",
                "src/SPC_RealTime.rs"
            ]

            for sfile in sigma_files:
                file_path = six_sigma_path / sfile
                if file_path.exists():
                    self._process_six_sigma_file(file_path)

            # Agregar contenido expandido de Six Sigma
            self._add_expanded_six_sigma_content()

    def _process_advanced_theory(self):
        """Procesar teoría avanzada"""
        theory_path = self.math_root / "ADVANCED_THEORY"
        if theory_path.exists():
            print("Procesando teoria avanzada...")
            self._process_folder_content(theory_path, "Advanced Mathematical Theory")

    def _process_decision_science(self):
        """Procesar ciencia de decisión"""
        decision_path = self.math_root / "DECISION_SCIENCE"
        if decision_path.exists():
            print("Procesando ciencia de decision...")
            self._process_folder_content(decision_path, "Decision Science")

    def _process_information_theory_content(self):
        """Procesar contenido de teoría de la información"""
        info_theory_path = self.math_root / "06_INFORMATION_THEORY"
        if info_theory_path.exists():
            print("Procesando teoria de la informacion...")
            self._process_folder_content(info_theory_path, "Information Theory")

            # Procesar archivos específicos de teoría de información
            info_files = [
                "shannon_theory.py",
                "entropy_calculations.jl",
                "channel_capacity.rs",
                "error_correcting_codes.cpp"
            ]

            for ifile in info_files:
                file_path = info_theory_path / ifile
                if file_path.exists():
                    self._process_information_theory_file(file_path)

            # Agregar contenido expandido de teoría de información
            self._add_expanded_information_theory_content()

    def _process_folder_content(self, folder_path: Path, category: str):
        """Procesar contenido de una carpeta"""
        for file_path in folder_path.rglob("*"):
            if file_path.is_file() and file_path.suffix in ['.py', '.jl', '.hs', '.lean', '.cpp', '.f90', '.r', '.qs', '.qrisp']:
                self._process_source_file(file_path, category)

    def _process_source_file(self, file_path: Path, category: str):
        """Procesar archivo fuente"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()

            if len(content.strip()) == 0:
                return

            # Crear entrada de dataset
            entry = {
                "id": self._generate_id(file_path),
                "source": "source_code",
                "file_path": str(file_path),
                "category": category,
                "language": file_path.suffix[1:],
                "content": content,
                "size": len(content),
                "topic": self._extract_topic_from_path(file_path),
                "difficulty": self._estimate_difficulty(content),
                "tags": self._extract_tags(content, category)
            }

            self.dataset.append(entry)

        except Exception as e:
            print(f"Error procesando {file_path}: {e}")

    def _process_quantum_file(self, file_path: Path):
        """Procesar archivo cuántico específico"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()

            entry = {
                "id": self._generate_id(file_path),
                "source": "quantum_computing",
                "file_path": str(file_path),
                "category": "Quantum Physics",
                "language": file_path.suffix[1:],
                "content": content,
                "quantum_concepts": self._extract_quantum_concepts(content),
                "complexity": "advanced"
            }

            self.dataset.append(entry)

        except Exception as e:
            print(f"Error procesando archivo cuántico {file_path}: {e}")

    def _process_six_sigma_file(self, file_path: Path):
        """Procesar archivo Six Sigma específico"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()

            entry = {
                "id": self._generate_id(file_path),
                "source": "six_sigma",
                "file_path": str(file_path),
                "category": "Quality Management",
                "language": file_path.suffix[1:],
                "content": content,
                "six_sigma_tools": self._extract_six_sigma_tools(content),
                "methodology": "DMAIC/DMADV"
            }

            self.dataset.append(entry)

        except Exception as e:
            print(f"Error procesando archivo Six Sigma {file_path}: {e}")

    def _process_information_theory_file(self, file_path: Path):
        """Procesar archivo de teoría de información específico"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()

            entry = {
                "id": self._generate_id(file_path),
                "source": "information_theory",
                "file_path": str(file_path),
                "category": "Information Theory",
                "language": file_path.suffix[1:],
                "content": content,
                "information_concepts": self._extract_information_concepts(content),
                "complexity": "advanced"
            }

            self.dataset.append(entry)

        except Exception as e:
            print(f"Error procesando archivo de teoría de información {file_path}: {e}")

    def _extract_quantum_concepts(self, content: str) -> List[str]:
        """Extraer conceptos cuánticos del contenido"""
        concepts = []
        quantum_keywords = [
            "qubit", "entanglement", "superposition", "quantum gate",
            "hadamard", "cnot", "bell state", "quantum circuit",
            "measurement", "quantum algorithm", "shor", "grover",
            "quantum fourier", "phase estimation", "teleportation"
        ]

        content_lower = content.lower()
        for keyword in quantum_keywords:
            if keyword in content_lower:
                concepts.append(keyword)

        return list(set(concepts))

    def _extract_six_sigma_tools(self, content: str) -> List[str]:
        """Extraer herramientas Six Sigma del contenido"""
        tools = []
        sigma_keywords = [
            "dmaic", "dmadv", "sipoc", "ctq", "kpi", "doe",
            "control chart", "histogram", "pareto", "fishbone",
            "fmea", "gage rr", "capability analysis", "anova",
            "hypothesis testing", "regression", "correlation"
        ]

        content_lower = content.lower()
        for keyword in sigma_keywords:
            if keyword.replace(" ", "") in content_lower.replace(" ", ""):
                tools.append(keyword)

        return list(set(tools))

    def _extract_information_concepts(self, content: str) -> List[str]:
        """Extraer conceptos de teoría de información del contenido"""
        concepts = []
        info_keywords = [
            "entropy", "shannon", "channel_capacity", "mutual_information",
            "kl_divergence", "huffman_coding", "source_coding", "channel_coding",
            "rate_distortion", "fano_inequality", "data_compression",
            "noiseless_coding", "lossy_compression", "error_correction",
            "turbo_codes", "ldpc", "polar_codes", "network_coding"
        ]

        content_lower = content.lower()
        for keyword in info_keywords:
            if keyword in content_lower:
                concepts.append(keyword)

        return list(set(concepts))

    def _generate_id(self, file_path: Path) -> str:
        """Generar ID único para archivo"""
        file_str = str(file_path)
        return hashlib.md5(file_str.encode()).hexdigest()[:12]

    def _extract_topic_from_path(self, file_path: Path) -> str:
        """Extraer tema del path del archivo"""
        parts = file_path.parts
        for part in reversed(parts):
            if part[0].isdigit() and "_" in part:
                return part.split("_", 1)[1].replace("_", " ")
        return file_path.stem

    def _estimate_difficulty(self, content: str) -> str:
        """Estimar dificultad del contenido"""
        complexity_indicators = [
            "lambda", "monad", "homotopy", "category theory",
            "differential geometry", "quantum field", "string theory",
            "advanced calculus", "complex analysis"
        ]

        score = 0
        content_lower = content.lower()

        for indicator in complexity_indicators:
            if indicator in content_lower:
                score += 1

        if score >= 3:
            return "expert"
        elif score >= 1:
            return "advanced"
        else:
            return "intermediate"

    def _extract_tags(self, content: str, category: str) -> List[str]:
        """Extraer tags del contenido"""
        tags = [category.lower().replace(" ", "_")]

        # Tags específicos por categoría
        if "quantum" in category.lower():
            tags.extend(["physics", "quantum_mechanics", "computation"])
        elif "geometry" in category.lower():
            tags.extend(["mathematics", "topology", "algebra"])
        elif "optimization" in category.lower():
            tags.extend(["mathematics", "algorithms", "decision_making"])
        elif "statistics" in category.lower():
            tags.extend(["mathematics", "probability", "data_analysis"])

        return list(set(tags))

    def _add_mathematical_theories(self):
        """Agregar teorías matemáticas y libros de referencia"""
        print("Agregando teorias matematicas...")

        theories = [
            {
                "title": "Algebraic Geometry - Theory and Applications",
                "author": "Various",
                "content": """
                La geometría algebraica estudia variedades algebraicas, que son conjuntos definidos
                por ecuaciones polinomiales. Conceptos fundamentales incluyen:

                1. Variedades algebraicas y esquemas
                2. Teoría de haces y cohomología
                3. Geometría proyectiva y biracional
                4. Curvas algebraicas y superficies
                5. Teoría de Galois e invariantes

                Aplicaciones en criptografía, teoría de números, física matemática.
                """,
                "category": "Algebraic Geometry",
                "difficulty": "advanced"
            },
            {
                "title": "Loop Quantum Gravity - Foundations",
                "author": "Carlo Rovelli",
                "content": """
                La gravedad cuántica de lazos es un enfoque para cuantizar la relatividad general
                usando técnicas de teoría de gauge y geometría no-conmutativa.

                Conceptos clave:
                1. Espinor networks y grafos de espín
                2. Área y volumen operadores cuánticos
                3. Estados coherentes y semiclassical
                4. Cosmología cuántica
                5. Unificación con materia cuántica
                """,
                "category": "Quantum Gravity",
                "difficulty": "expert"
            }
        ]

        for theory in theories:
            entry = {
                "id": self._generate_id(Path(theory["title"])),
                "source": "mathematical_theory",
                "title": theory["title"],
                "author": theory["author"],
                "content": theory["content"],
                "category": theory["category"],
                "difficulty": theory["difficulty"],
                "tags": ["theory", "reference", "mathematics"]
            }
            self.dataset.append(entry)

    def _add_six_sigma_theories(self):
        """Agregar teorías y libros de Six Sigma"""
        print("Agregando teorias Six Sigma...")

        six_sigma_content = [
            {
                "title": "Six Sigma - DMAIC Methodology",
                "content": """
                DMAIC (Define, Measure, Analyze, Improve, Control) es la metodología principal de Six Sigma:

                1. DEFINE: Identificar problema, stakeholders, requerimientos
                2. MEASURE: Recopilar datos, establecer métricas, capability analysis
                3. ANALYZE: Identificar causas raíz, análisis estadístico, hypothesis testing
                4. IMPROVE: Generar soluciones, DOE (Design of Experiments), optimization
                5. CONTROL: Implementar controles, monitoreo continuo, SPC (Statistical Process Control)

                Herramientas clave: Pareto charts, Fishbone diagrams, FMEA, ANOVA, Regression
                """,
                "category": "Six Sigma",
                "methodology": "DMAIC"
            },
            {
                "title": "Design of Experiments (DOE) - Taguchi Methods",
                "content": """
                Los métodos Taguchi para diseño de experimentos optimizan calidad minimizando variabilidad:

                Conceptos principales:
                1. Pérdida cuadrática de calidad
                2. Función ideal de calidad
                3. Arrays ortogonales para experimentos eficientes
                4. Análisis de varianza (ANOVA)
                5. Signal-to-Noise ratio
                6. Robust design principles

                Aplicaciones: Optimización de procesos, reducción de defectos, mejora de calidad.
                """,
                "category": "Six Sigma",
                "tools": ["DOE", "Taguchi", "ANOVA", "S/N ratio"]
            }
        ]

        for content in six_sigma_content:
            entry = {
                "id": self._generate_id(Path(content["title"])),
                "source": "six_sigma_theory",
                "title": content["title"],
                "content": content["content"],
                "category": content["category"],
                "difficulty": "advanced",
                "tags": ["quality", "statistics", "process_improvement"]
            }
            if "methodology" in content:
                entry["methodology"] = content["methodology"]
            if "tools" in content:
                entry["tools"] = content["tools"]

            self.dataset.append(entry)

    def _add_quantum_theories_books(self):
        """Agregar teorías y libros de física cuántica expandidos"""
        print("Agregando teorias y libros de fisica cuantica...")

        quantum_books = [
            {
                "title": "Introduction to Quantum Mechanics - David J. Griffiths",
                "content": """
                LIBRO FUNDAMENTAL DE MECÁNICA CUÁNTICA:

                CAPÍTULOS PRINCIPALES:
                1. Ondas y partículas - Dualidad onda-partícula
                2. Función de onda - Interpretación probabilística
                3. Ecuación de Schrödinger - Tiempo dependiente e independiente
                4. Teoría de cuantización - Oscilador armónico, átomo de hidrógeno
                5. Sistemas de múltiples partículas - Identical particles
                6. Simetría y leyes de conservación - Teoría de grupos
                7. Teoría de perturbaciones - Correcciones de primer y segundo orden
                8. Estructura atómica - Orbitales, configuración electrónica

                CONCEPTOS CLAVE:
                - Principio de incertidumbre de Heisenberg
                - Superposición y medida cuántica
                - Efecto túnel y barreras de potencial
                - Spin y momento angular
                - Teorema adiabático
                - Oscilaciones y transiciones

                APLICACIONES:
                - Espectroscopia atómica y molecular
                - Láseres y amplificación cuántica
                - Semiconductores y dispositivos electrónicos
                - Resonancia magnética nuclear (RMN)
                """,
                "category": "Quantum Physics",
                "type": "Textbook",
                "difficulty": "intermediate",
                "topics": ["wave_mechanics", "schrodinger_equation", "quantum_states"]
            },
            {
                "title": "Quantum Computation and Quantum Information - Nielsen & Chuang",
                "content": """
                BIBLIA DE LA COMPUTACIÓN CUÁNTICA:

                FUNDAMENTOS TEÓRICOS:
                1. Fundamentos de mecánica cuántica
                   - Estados, medidas, operaciones unitarias
                   - Composición de sistemas, producto tensorial

                2. Computación cuántica
                   - Modelo de circuitos cuánticos
                   - Algoritmos: Shor, Grover, Deutsch-Jozsa
                   - Teleportación cuántica y enredamiento

                3. Teoría de la información cuántica
                   - Estados mixtos y operadores densidad
                   - Entropía de von Neumann
                   - Canales cuánticos y capacidad

                4. Corrección de errores cuánticos
                   - Códigos estabilizadores
                   - Teorema del umbral
                   - Corrección de errores en computadoras cuánticas

                5. Criptografía cuántica
                   - Distribución de clave cuántica (BB84)
                   - Bit cuántico comprometido
                   - Autenticación cuántica

                CONCEPTOS AVANZADOS:
                - No-clonación y no-borrado
                - Computación reversible
                - Teoría de decoherencia
                - Modelos de ruido cuántico
                """,
                "category": "Quantum Computing",
                "type": "Textbook",
                "difficulty": "advanced",
                "topics": ["quantum_algorithms", "quantum_information", "error_correction"]
            },
            {
                "title": "Principles of Quantum Mechanics - R. Shankar",
                "content": """
                TRATADO COMPLETO DE MECÁNICA CUÁNTICA:

                ESTRUCTURA MATEMÁTICA:
                1. Espacios de Hilbert y operadores
                   - Espacios vectoriales complejos
                   - Operadores lineales y hermíticos
                   - Valores propios y funciones propias

                2. Mecánica cuántica exacta
                   - Partícula en una caja
                   - Potencial armónico
                   - Átomo de hidrógeno completo

                3. Teoría de scattering
                   - Sección eficaz
                   - Método de Born
                   - Resonancias y fases

                4. Sistemas idénticos y spin
                   - Estadística Bose-Einstein y Fermi-Dirac
                   - Momento angular total
                   - Estructura fina del átomo de hidrógeno

                5. Teoría de perturbaciones avanzada
                   - Perturbación dependiente del tiempo
                   - Transiciones y reglas de selección
                   - Teoría de variaciones

                6. Métodos aproximados
                   - Teoría de WKB
                   - Método de Wentzel-Kramers-Brillouin
                   - Teoría de la matriz S

                APLICACIONES MODERNAS:
                - Óptica cuántica
                - Física de materiales
                - Espectroscopia
                - Física nuclear
                """,
                "category": "Quantum Physics",
                "type": "Reference",
                "difficulty": "expert",
                "topics": ["mathematical_physics", "perturbation_theory", "scattering_theory"]
            }
        ]

        for book in quantum_books:
            entry = {
                "id": self._generate_id(Path(book["title"])),
                "source": "quantum_book_theory",
                "title": book["title"],
                "content": book["content"],
                "category": book["category"],
                "type": book["type"],
                "difficulty": book["difficulty"],
                "topics": book["topics"],
                "tags": ["quantum_physics", "textbook", "theory", "physics"]
            }
            self.dataset.append(entry)

    def _add_information_theory_books(self):
        """Agregar libros y teorías de teoría de la información"""
        print("Agregando libros y teorias de teoria de la informacion...")

        info_theory_books = [
            {
                "title": "A Mathematical Theory of Communication - Claude Shannon",
                "content": """
                EL ARTÍCULO ORIGINAL QUE FUNDÓ LA TEORÍA DE LA INFORMACIÓN (1948):

                CONTRIBUCIONES PRINCIPALES:

                1. DEFINICIÓN DE INFORMACIÓN
                   - Cantidad de información en un mensaje
                   - Unidades: bits, Hartley, natural units
                   - Medida logarítmica de incertidumbre

                2. ENTROPÍA Y INFORMACIÓN
                   - Entropía de una fuente discreta: H(X) = -Σ p(x) log p(x)
                   - Entropía conjunta y condicional
                   - Información mutua: I(X;Y) = H(X) + H(Y) - H(X,Y)

                3. CODIFICACIÓN FUENTE (SOURCE CODING)
                   - Teorema de codificación fuente
                   - Eficiencia de códigos sin ruido
                   - Compresión de datos óptima

                4. CODIFICACIÓN CANAL (CHANNEL CODING)
                   - Capacidad de canal: C = max I(X;Y)
                   - Teorema de codificación de canal
                   - Límites fundamentales de comunicación

                5. CANALES CONTINUOS
                   - Ruido gaussiano blanco
                   - Teorema de muestreo de Nyquist-Shannon
                   - Ancho de banda y capacidad

                IMPACTO HISTÓRICO:
                - Fundación de la teoría de la información
                - Base matemática de telecomunicaciones
                - Influencia en criptografía y compresión
                - Premio Kyoto en 1985
                """,
                "category": "Information Theory",
                "type": "Original Paper",
                "difficulty": "advanced",
                "topics": ["entropy", "channel_capacity", "source_coding"]
            },
            {
                "title": "Information Theory: Coding Theorems for Discrete Memoryless Systems - Imre Csiszár & János Körner",
                "content": """
                TRATADO AVANZADO DE TEORÍA DE LA INFORMACIÓN:

                CONTENIDO PRINCIPAL:

                1. TEORÍA DE LA INFORMACIÓN DISCRETA
                   - Entropía y cantidad de información
                   - Propiedades de la entropía
                   - Divergencia Kullback-Leibler
                   - Información mutua y canales

                2. CODIFICACIÓN FUENTE
                   - Códigos de Huffman óptimos
                   - Teorema de Shannon-McMillan-Breiman
                   - Codificación aritmética
                   - Compresión universal

                3. CODIFICACIÓN CANAL
                   - Límites de error en transmisión
                   - Códigos lineales y cíclicos
                   - Teorema de Hamming
                   - Códigos convolucionales

                4. TEORÍA DE LA COMPLEJIDAD
                   - Complejidad de Kolmogorov
                   - Teoría algorítmica de la información
                   - Aleatoriedad algorítmica

                5. TEMAS AVANZADOS
                   - Información en sistemas continuos
                   - Teoría de juegos y información
                   - Criptografía y teoría de la información

                APLICACIONES MODERNAS:
                - Compresión de datos (JPEG, MP3, MPEG)
                - Corrección de errores (CD, DVD, satélites)
                - Criptografía (AES, RSA)
                - Aprendizaje automático (entropía en ML)
                """,
                "category": "Information Theory",
                "type": "Textbook",
                "difficulty": "expert",
                "topics": ["coding_theory", "complexity_theory", "algorithms"]
            },
            {
                "title": "Elements of Information Theory - Cover & Thomas",
                "content": """
                TEXTO CLÁSICO DE TEORÍA DE LA INFORMACIÓN:

                ESTRUCTURA DEL LIBRO:

                1. ENTROPÍA, RELATIVA ENTROPÍA Y INEQUIDADES MUTUAS
                   - Medidas de entropía
                   - Desigualdades fundamentales
                   - Propiedades de la información mutua

                2. ASINTÓTICA EQUIV-DISTRIBUCIÓN
                   - Ley de los grandes números
                   - Teorema del método de tipos
                   - Sanov's theorem

                3. CODIFICACIÓN DE DATOS
                   - Kraft inequality
                   - Optimal codes
                   - Shannon codes
                   - Tunstall codes

                4. TECNICAS DE CODIFICACIÓN
                   - Compresión de texto
                   - Codificación predictiva
                   - Transform coding
                   - Wavelet compression

                5. CODIFICACIÓN CANAL
                   - Confusability and rate-distortion
                   - Gaussian channel
                   - Parallel channels
                   - Feedback channels

                6. ALGUNAS EXTENSIONES
                   - Universal data compression
                   - Kolmogorov complexity
                   - Network information theory

                APLICACIONES PRÁCTICAS:
                - Compresión de imágenes y video
                - Procesamiento de señales
                - Comunicaciones digitales
                - Teoría de control
                """,
                "category": "Information Theory",
                "type": "Textbook",
                "difficulty": "advanced",
                "topics": ["entropy", "data_compression", "channel_coding"]
            }
        ]

        for book in info_theory_books:
            entry = {
                "id": self._generate_id(Path(book["title"])),
                "source": "information_theory_book",
                "title": book["title"],
                "content": book["content"],
                "category": book["category"],
                "type": book["type"],
                "difficulty": book["difficulty"],
                "topics": book["topics"],
                "tags": ["information_theory", "mathematics", "communication"]
            }
            self.dataset.append(entry)

    def _add_expanded_six_sigma_content(self):
        """Agregar contenido expandido de Six Sigma"""
        print("Agregando contenido expandido de Six Sigma...")

        expanded_sigma = [
            {
                "title": "Six Sigma Black Belt - DMAIC Deep Dive",
                "content": """
                METODOLOGÍA DMAIC PARA BLACK BELTS:

                1. DEFINE PHASE - PROFUNDIDAD
                   - Project Charter detallado con métricas SMART
                   - Stakeholder Analysis y Communication Plan
                   - Problem Statement usando 5W2H
                   - Goal Statement con baseline y target
                   - Project Scope con boundaries claros
                   - SIPOC Diagram expandido
                   - VOC/VOB/VOE Analysis

                2. MEASURE PHASE - ANÁLISIS DETALLADO
                   - Data Collection Plan con MSA
                   - Gage R&R Studies para measurement systems
                   - Process Capability Analysis (Cp, Cpk, Pp, Ppk)
                   - Baseline Performance Metrics
                   - Process Sigma Calculation
                   - Value Stream Mapping
                   - Spaghetti Diagrams

                3. ANALYZE PHASE - HERRAMIENTAS AVANZADAS
                   - Hypothesis Testing (t-tests, ANOVA, Chi-square)
                   - Regression Analysis (simple, multiple, logistic)
                   - Correlation Analysis (Pearson, Spearman)
                   - Multi-vari Analysis
                   - DOE (Design of Experiments) - Full factorial, fractional
                   - Failure Mode and Effects Analysis (FMEA)
                   - Root Cause Analysis usando 5-Why y Fishbone

                4. IMPROVE PHASE - OPTIMIZACIÓN
                   - Brainstorming techniques (Affinity Diagrams, Nominal Group)
                   - Pugh Matrix para concept selection
                   - Pilot Testing y Implementation Plan
                   - Risk Assessment y Mitigation Plans
                   - Change Management Strategy
                   - Training Plans para operadores

                5. CONTROL PHASE - SOSTENIBILIDAD
                   - Control Charts (X-bar R, I-MR, P, NP, C, U)
                   - Process Control Plans
                   - Standard Operating Procedures (SOP)
                   - Visual Management Systems
                   - Reaction Plans para out-of-control conditions
                   - Audit Systems y Scorecards

                HERRAMIENTAS ESTADÍSTICAS AVANZADAS:
                - Statistical Process Control (SPC)
                - Measurement System Analysis (MSA)
                - Design of Experiments (DOE)
                - Analysis of Variance (ANOVA)
                - Regression Analysis
                - Reliability Engineering
                - Queuing Theory
                """,
                "category": "Six Sigma",
                "methodology": "DMAIC",
                "belt_level": "Black Belt",
                "tools": ["SPC", "DOE", "MSA", "ANOVA", "Regression", "FMEA"]
            },
            {
                "title": "DMADV Methodology for Design for Six Sigma (DFSS)",
                "content": """
                METODOLOGÍA DMADV PARA DESARROLLO DE NUEVOS PRODUCTOS/PROCESOS:

                1. DEFINE PHASE - DEFINIR REQUERIMIENTOS
                   - Quality Function Deployment (QFD) - House of Quality
                   - Critical to Quality (CTQ) characteristics
                   - Requirements cascade usando CTQ Flowdown
                   - Benchmarking competitivo
                   - VOC translation to CTQs
                   - Project prioritization usando Pugh Matrix

                2. MEASURE PHASE - MEDIR CAPABILIDADES
                   - Functional requirements measurement
                   - Technical requirements definition
                   - Capability analysis de componentes
                   - Tolerance analysis (Worst Case, RSS, Monte Carlo)
                   - Measurement uncertainty analysis
                   - Design verification methods

                3. ANALYZE PHASE - ANALIZAR ALTERNATIVAS DE DISEÑO
                   - Design alternatives evaluation
                   - Robust design principles (Taguchi Methods)
                   - Design for Manufacturability (DFM)
                   - Design for Assembly (DFA)
                   - Failure Mode and Effects Analysis (DFMEA)
                   - P-Diagram para identificar ruido
                   - Robust parameter design

                4. DESIGN PHASE - DESARROLLAR DETALLES
                   - Detailed design specifications
                   - Tolerance allocation usando statistical methods
                   - Design verification and validation plans
                   - Prototype development and testing
                   - Design reviews y checkpoints
                   - Risk management plans
                   - Supplier quality requirements

                5. VERIFY PHASE - VERIFICAR DISEÑO
                   - Design verification testing (DVT)
                   - Process validation (PV)
                   - Production validation (PPAP)
                   - Capability studies en producción
                   - Control plan development
                   - Training y documentation
                   - Transition to operations

                HERRAMIENTAS DFSS ESPECÍFICAS:
                - Quality Function Deployment (QFD)
                - TRIZ (Theory of Inventive Problem Solving)
                - Robust Design (Parameter Design, Tolerance Design)
                - Design of Experiments (DOE) para caracterización
                - Design for X (DFX) methodologies
                - Reliability prediction methods
                - Accelerated life testing
                """,
                "category": "Six Sigma",
                "methodology": "DMADV",
                "belt_level": "Master Black Belt",
                "tools": ["QFD", "DFMEA", "Robust_Design", "DOE", "TRIZ"]
            },
            {
                "title": "Lean Six Sigma - Integration and Tools",
                "content": """
                INTEGRACIÓN LEAN Y SIX SIGMA:

                PRINCIPIOS FUNDAMENTALES:
                1. Enfoque en el cliente (Customer Focus)
                2. Flujo de valor (Value Stream)
                3. Eliminación de desperdicios (Waste Elimination)
                4. Mejora continua (Kaizen)
                5. Respeto por las personas (Respect for People)
                6. Calidad por diseño (Quality by Design)

                HERRAMIENTAS LEAN INTEGRADAS:

                1. MAPEO DE FLUJO DE VALOR (Value Stream Mapping)
                   - Current State Map
                   - Future State Map
                   - Kaizen events identification
                   - Lead time reduction

                2. 5S WORKPLACE ORGANIZATION
                   - Sort (Seiri) - Eliminar innecesarios
                   - Set in order (Seiton) - Organizar eficientemente
                   - Shine (Seiso) - Limpiar y mantener
                   - Standardize (Seiketsu) - Establecer estándares
                   - Sustain (Shitsuke) - Disciplina y mejora continua

                3. JUST-IN-TIME (JIT) PRODUCTION
                   - Pull systems (Kanban)
                   - Takt time calculation
                   - Level loading (Heijunka)
                   - Setup time reduction (SMED)

                4. JIDOKA (AUTONOMATION)
                   - Poka-yoke (Mistake proofing)
                   - Andon systems
                   - Stop at first defect
                   - Root cause analysis

                5. TOTAL PRODUCTIVE MAINTENANCE (TPM)
                   - Overall Equipment Effectiveness (OEE)
                   - Mean Time Between Failures (MTBF)
                   - Mean Time To Repair (MTTR)
                   - Preventive maintenance

                INTEGRACIÓN LEAN-SIX SIGMA:
                - Lean para velocidad y eliminación de desperdicios
                - Six Sigma para reducción de variabilidad
                - DMAIC para proyectos de mejora
                - DMADV para desarrollo de nuevos productos
                - Lean tools en Define y Measure phases
                - Six Sigma tools en Analyze, Improve, Control

                MÉTRICAS CLAVE:
                - Cycle Time
                - Throughput
                - First Pass Yield
                - Defect per Million Opportunities (DPMO)
                - Process Sigma Level
                - Overall Equipment Effectiveness (OEE)
                - Customer Satisfaction Scores
                """,
                "category": "Six Sigma",
                "methodology": "Lean Six Sigma",
                "belt_level": "Green Belt/Black Belt",
                "tools": ["VSM", "5S", "Kanban", "Jidoka", "TPM", "PokaYoke"]
            }
        ]

        for content in expanded_sigma:
            entry = {
                "id": self._generate_id(Path(content["title"])),
                "source": "six_sigma_expanded",
                "title": content["title"],
                "content": content["content"],
                "category": content["category"],
                "methodology": content.get("methodology"),
                "belt_level": content.get("belt_level"),
                "tools": content.get("tools", []),
                "difficulty": "advanced",
                "tags": ["quality", "statistics", "process_improvement", "lean", "six_sigma"]
            }
            self.dataset.append(entry)

    def _add_expanded_information_theory_content(self):
        """Agregar contenido expandido de teoría de la información"""
        print("Agregando contenido expandido de teoria de la informacion...")

        expanded_info_theory = [
            {
                "title": "Shannon's Channel Coding Theorem - Mathematical Proof",
                "content": """
                DEMOSTRACIÓN DEL TEOREMA DE CODIFICACIÓN DE CANAL DE SHANNON:

                ENUNCIADO:
                Para cualquier canal discreto sin memoria con capacidad C,
                existe un código con tasa R < C tal que la probabilidad de error
                puede hacerse arbitrariamente pequeña.

                DEMOSTRACIÓN (ESQUEMA):

                1. CODIFICACIÓN ALEATORIA
                   - Generar códigos aleatorios de longitud n
                   - Usar distribución típica para entrada
                   - Análisis de probabilidad de error

                2. LEMMA DE CODIFICACIÓN
                   - Si R < C, existe código con P_e → 0
                   - Propiedad de distancia típica
                   - Unión bound para probabilidad de error

                3. LEMMA DE CONVERSIÓN
                   - Para cualquier δ > 0, si R > C, P_e ≥ δ
                   - Límite fundamental de capacidad
                   - Contradicción por reducción al absurdo

                4. CODIFICACIÓN EXPLÍCITA
                   - Construcción de códigos que alcanzan capacidad
                   - Códigos polares (Arıkan, 2009)
                   - LDPC codes (Gallager, MacKay)
                   - Turbo codes (Berrou et al.)

                CONSECUENCIAS:
                - Capacidad es el límite fundamental
                - Posibilidad de comunicación confiable
                - Base para sistemas de comunicación modernos
                - Influencia en teoría de la complejidad
                """,
                "category": "Information Theory",
                "topic": "Channel Coding Theorem",
                "type": "Theorem Proof",
                "difficulty": "expert",
                "concepts": ["channel_capacity", "error_correction", "random_coding"]
            },
            {
                "title": "Entropy and Information Measures",
                "content": """
                MEDIDAS DE INFORMACIÓN EN TEORÍA DE LA INFORMACIÓN:

                1. ENTROPÍA DE SHANNON
                   H(X) = -Σᵢ p(xᵢ) log₂ p(xᵢ)
                   - Medida de incertidumbre promedio
                   - Unidades: bits (log₂), nats (ln), hartleys (log₁₀)
                   - Propiedades: no-negativa, cóncava, simétrica

                2. ENTROPÍA CONDICIONAL
                   H(X|Y) = Σᵢⱼ p(xᵢ,yⱼ) log₂ p(xᵢ|yⱼ)
                   - Información adicional requerida
                   - H(X|Y) = H(X,Y) - H(Y)
                   - Cadena de Markov: H(X,Y,Z) = H(X) + H(Y|X) + H(Z|Y)

                3. INFORMACIÓN MUTUA
                   I(X;Y) = Σᵢⱼ p(xᵢ,yⱼ) log₂ [p(xᵢ,yⱼ)/(p(xᵢ)p(yⱼ))]
                   - I(X;Y) = H(X) - H(X|Y) = H(Y) - H(Y|X)
                   - Medida de dependencia estadística
                   - I(X;Y) ≥ 0, con igualdad solo si independiente

                4. DIVERGENCIA KULLBACK-LEIBLER
                   D_KL(p||q) = Σᵢ p(xᵢ) log₂ [p(xᵢ)/q(xᵢ)]
                   - Distancia entre distribuciones
                   - No simétrica: D_KL(p||q) ≠ D_KL(q||p)
                   - Siempre ≥ 0, =0 solo si p=q

                5. ENTROPÍA RELATIVA Y CRUZADA
                   H(p,q) = -Σᵢ p(xᵢ) log₂ q(xᵢ) = H(p) + D_KL(p||q)
                   - Entropía cruzada: medida de codificación ineficiente
                   - Usada en aprendizaje automático (loss functions)

                6. ENTROPÍA DE RENYI
                   H_α(X) = (1/(1-α)) log₂ Σᵢ p(xᵢ)^α    para α > 0, α ≠ 1
                   - Generalización de entropía Shannon (α→1)
                   - α=2: entropía de colisión
                   - α=∞: min-entropy

                APLICACIONES:
                - Compresión de datos (Huffman, LZW)
                - Criptografía (entropía de clave)
                - Aprendizaje automático (feature selection)
                - Física estadística (entropía termodinámica)
                - Ecología (diversidad de especies)
                """,
                "category": "Information Theory",
                "topic": "Entropy Measures",
                "type": "Mathematical Foundation",
                "difficulty": "advanced",
                "concepts": ["shannon_entropy", "mutual_information", "kl_divergence"]
            },
            {
                "title": "Error-Correcting Codes - Theory and Practice",
                "content": """
                TEORÍA DE CÓDIGOS CORRECTORES DE ERRORES:

                1. CONCEPTOS BÁSICOS
                   - Código: conjunto de palabras código
                   - Distancia Hamming: d(x,y) = número de posiciones diferentes
                   - Distancia mínima d del código
                   - Capacidad de corrección: t = floor((d-1)/2)
                   - Capacidad de detección: d-1

                2. CÓDIGOS LINEALES
                   - Definición: subespacio vectorial sobre F₂
                   - Matriz generadora G: k×n
                   - Matriz de paridad H: (n-k)×n
                   - Síndrome: H·y para decodificación

                3. CÓDIGOS DE BLOQUE
                   - Hamming codes: distancia mínima 3
                   - BCH codes: polinomiales
                   - Reed-Solomon: no binarios, óptimos
                   - Golay codes: [24,12,8] perfecto

                4. CÓDIGOS CONVULUCIONALES
                   - Definición: generados por polinomios
                   - Trellis representation
                   - Algoritmo de Viterbi para decodificación
                   - Soft-decision vs hard-decision

                5. CÓDIGOS MODERNOS
                   - Turbo codes: concatenación paralela
                   - LDPC (Low-Density Parity-Check)
                   - Polar codes: alcanzan capacidad
                   - Fountain codes: rateless

                6. ALGORITMOS DE DECODIFICACIÓN
                   - Exhaustive search (máxima verosimilitud)
                   - Syndrome decoding
                   - Algoritmo de Berlekamp-Massey
                   - Belief Propagation (sum-product algorithm)
                   - BCJR algorithm para turbo codes

                LÍMITES TEÓRICOS:
                - Singleton bound: d ≤ n-k+1
                - Hamming bound: códigos perfectos
                - Gilbert-Varshamov bound
                - Plotkin bound para códigos largos

                APLICACIONES:
                - Comunicaciones (satélites, móviles, WiFi)
                - Almacenamiento (discos duros, SSD, RAID)
                - Computación cuántica (códigos estabilizadores)
                - Criptografía (códigos de autenticación)
                """,
                "category": "Information Theory",
                "topic": "Error-Correcting Codes",
                "type": "Coding Theory",
                "difficulty": "advanced",
                "concepts": ["hamming_distance", "linear_codes", "decoding_algorithms"]
            },
            {
                "title": "Network Information Theory - Multiple Access Channels",
                "content": """
                TEORÍA DE LA INFORMACIÓN EN REDES:

                1. CANALES DE ACCESO MÚLTIPLE (MAC)
                   - Varios transmisores, un receptor
                   - Capacidad de región: conjunto de tasas alcanzables
                   - MAC determinístico: C = {(R₁,R₂) : R₁ ≤ C₁, R₂ ≤ C₂, R₁+R₂ ≤ C}
                   - MAC gaussiano: solución explícita usando waterfilling

                2. CANALES DE INTERFERENCIA
                   - Múltiples pares transmisor-receptor
                   - Interferencia tratada como ruido
                   - Interferencia como ruido más fuerte (INR)
                   - Achievability usando Han-Kobayashi scheme

                3. CANALES DE DIFUSIÓN (Broadcast)
                   - Un transmisor, múltiples receptores
                   - Capacidad: min de capacidades individuales
                   - Superposición coding para degradado BC
                   - Marton coding para general BC

                4. RELAY CHANNELS
                   - Fuente → Relay → Destino
                   - Cut-set bound
                   - Compress-and-forward
                   - Decode-and-forward
                   - Amplify-and-forward

                5. TWO-WAY CHANNELS
                   - Comunicación bidireccional simultánea
                   - Capacidad mayor que canales separados
                   - Compute-and-forward para redes
                   - Interference alignment

                6. TEORÍA DE LA INFORMACIÓN EN REDES
                   - Information flow en networks
                   - Max-flow min-cut theorem para redes
                   - Linear network coding
                   - Index coding problem

                CONCEPTOS AVANZADOS:
                - Multiple description coding
                - Distributed source coding (Slepian-Wolf)
                - CEO problem
                - Common information

                APLICACIONES MODERNAS:
                - Redes celulares (uplink multiple access)
                - Redes WiFi (CSMA, OFDMA)
                - Redes de sensores
                - Internet of Things (IoT)
                - 5G/6G communications
                - Satellite communications
                """,
                "category": "Information Theory",
                "topic": "Network Information Theory",
                "type": "Advanced Theory",
                "difficulty": "expert",
                "concepts": ["multiple_access", "broadcast_channels", "relay_channels"]
            }
        ]

        for content in expanded_info_theory:
            entry = {
                "id": self._generate_id(Path(content["title"])),
                "source": "information_theory_expanded",
                "title": content["title"],
                "content": content["content"],
                "category": content["category"],
                "topic": content.get("topic"),
                "type": content.get("type"),
                "difficulty": content["difficulty"],
                "concepts": content.get("concepts", []),
                "tags": ["information_theory", "communication", "coding", "networks"]
            }
            self.dataset.append(entry)

    def _add_julia_tutorials(self):
        """Agregar tutoriales de Julia para investigación"""
        print("Agregando tutoriales de Julia...")

        julia_tutorials = [
            {
                "title": "Creando Centros de Investigación con Julia",
                "content": """
                Guía para establecer centros de investigación matemática usando Julia:

                1. INSTALACIÓN Y CONFIGURACIÓN:
                   - Instalar Julia y Jupyter
                   - Configurar Pluto.jl para notebooks interactivos
                   - Instalar paquetes científicos principales

                2. ESTRUCTURA DEL CENTRO:
                   - Módulos especializados por área matemática
                   - Sistemas de documentación automática
                   - Integración con Git y CI/CD
                   - Bases de datos para resultados

                3. ÁREAS DE INVESTIGACIÓN:
                   - Simulaciones numéricas de alta performance
                   - Análisis de datos científicos
                   - Modelado matemático avanzado
                   - Computación simbólica

                4. COLABORACIÓN:
                   - Uso de GitHub para código abierto
                   - Documentación con Documenter.jl
                   - Testing automático con Travis CI
                   - Publicación de paquetes
                """,
                "category": "Julia Programming",
                "focus": "Research Centers"
            },
            {
                "title": "Julia para Matemáticas Avanzadas",
                "content": """
                Uso de Julia en investigación matemática avanzada:

                PAQUETES ESENCIALES:
                - Symbolics.jl: Computación simbólica
                - DifferentialEquations.jl: Ecuaciones diferenciales
                - Optimization.jl: Optimización numérica
                - Plots.jl: Visualización científica

                APLICACIONES:
                1. Geometría algebraica computacional
                2. Teoría de números algorítmica
                3. Simulaciones físicas
                4. Estadística bayesiana
                5. Optimización convexa

                VENTAJAS:
                - Performance comparable a C/Fortran
                - Sintaxis matemática natural
                - GPU computing con CUDA.jl
                - Paralelización automática
                """,
                "category": "Julia Programming",
                "focus": "Advanced Mathematics"
            }
        ]

        for tutorial in julia_tutorials:
            entry = {
                "id": self._generate_id(Path(tutorial["title"])),
                "source": "julia_tutorial",
                "title": tutorial["title"],
                "content": tutorial["content"],
                "category": tutorial["category"],
                "focus": tutorial["focus"],
                "difficulty": "intermediate",
                "tags": ["julia", "programming", "research", "mathematics"]
            }
            self.dataset.append(entry)

    def _generate_problems_and_solutions(self):
        """Generar problemas matemáticos y soluciones"""
        print("Generando problemas y soluciones...")

        problems = [
            {
                "title": "Problema de Geometría Algebraica",
                "category": "Algebraic Geometry",
                "difficulty": "advanced",
                "problem": """
                Considere la variedad algebraica definida por la ecuación:
                x³ + y³ + z³ - 3xyz = 0

                1. Demuestre que esta es una superficie cúbica en P³
                2. Encuentre los puntos singulares
                3. Determine si es racional o no
                """,
                "solution": """
                SOLUCIÓN:

                1. La ecuación x³ + y³ + z³ - 3xyz = 0 define una hipersuperficie
                   de grado 3 en el espacio proyectivo P³.

                2. Los puntos singulares se obtienen derivando parcialmente:
                   ∂f/∂x = 3x² - 3yz = 3(x² - yz)
                   ∂f/∂y = 3y² - 3xz = 3(y² - xz)
                   ∂f/∂z = 3z² - 3xy = 3(z² - xy)

                3. Esta superficie es biracionalmente equivalente a P²,
                   por lo tanto es racional.
                """,
                "tags": ["algebraic_geometry", "projective_variety", "rational_surface"]
            },
            {
                "title": "Problema de Mecánica Estadística",
                "category": "Statistical Mechanics",
                "difficulty": "expert",
                "problem": """
                En un gas ideal de N partículas, calcule la capacidad calorífica
                usando el teorema de equipartición de energía.

                Considere: Energía interna U = (3/2)NkT para monóatomico
                """,
                "solution": """
                SOLUCIÓN:

                Para un gas ideal monóatomico:
                U = (3/2)NkT

                Capacidad calorífica a volumen constante:
                Cv = (∂U/∂T)v = (3/2)Nk

                A presión constante:
                Cp = Cv + Nk = (3/2)Nk + Nk = (5/2)Nk

                Razón de capacidades específicas:
                γ = Cp/Cv = (5/2) / (3/2) = 5/3 ≈ 1.667

                Para diatómico: Cv = (5/2)Nk, γ = 7/5 = 1.4
                """,
                "tags": ["statistical_mechanics", "thermodynamics", "gas_ideal"]
            }
        ]

        for problem in problems:
            entry = {
                "id": self._generate_id(Path(problem["title"])),
                "source": "mathematical_problem",
                "title": problem["title"],
                "category": problem["category"],
                "difficulty": problem["difficulty"],
                "problem": problem["problem"],
                "solution": problem["solution"],
                "tags": problem["tags"]
            }
            self.dataset.append(entry)

    def _generate_quantum_problems(self):
        """Generar problemas de física cuántica"""
        print("Generando problemas de fisica cuantica...")

        quantum_problems = [
            {
                "title": "Problema de Mecánica Cuántica - Partícula en Caja",
                "category": "Quantum Physics",
                "difficulty": "intermediate",
                "problem": """
                Una partícula de masa m está confinada en una caja unidimensional de longitud L
                con paredes infinitas. Calcule:

                1. Las funciones de onda estacionarias ψ_n(x)
                2. Los valores propios de energía E_n
                3. La probabilidad de encontrar la partícula en el primer tercio de la caja
                   para el estado fundamental
                4. Demuestre que los estados son ortogonales
                """,
                "solution": """
                SOLUCIÓN:

                1. FUNCIONES DE ONDA:
                   ψ_n(x) = √(2/L) sin(nπx/L)    para 0 < x < L
                   ψ_n(x) = 0                     en las paredes

                2. ENERGÍAS:
                   E_n = n²h²/(8mL²) = n²π²ℏ²/(2mL²)

                3. PROBABILIDAD EN PRIMER TERCIO (n=1):
                   P = ∫₀ᴸ⁽¹/³⁾ |ψ₁(x)|² dx = (2/L) ∫₀ᴸ⁽¹/³⁾ sin²(πx/L) dx
                   = (2/L) ∫₀ᴸ⁽¹/³⁾ [1 - cos(2πx/L)]/2 dx
                   = (1/L) [x - (L/(2π)) sin(2πx/L)]₀ᴸ⁽¹/³⁾
                   = (1/L) [(L/3) - (L/(2π)) sin(2π/3)]
                   = 1/3 - (1/(2π)) sin(2π/3)
                   = 1/3 - (1/(2π)) (√3/2) ≈ 1/3 - 0.1376 ≈ 0.1957

                4. ORTOGONALIDAD:
                   ∫₀ᴸ ψ_m(x) ψ_n(x) dx = δ_mn
                   Para m ≠ n: ∫ sin(mπx/L) sin(nπx/L) dx = 0
                """,
                "tags": ["particle_in_box", "infinite_well", "quantum_states", "probability"]
            },
            {
                "title": "Problema de Computación Cuántica - Algoritmo de Deutsch-Jozsa",
                "category": "Quantum Computing",
                "difficulty": "advanced",
                "problem": """
                Considere el algoritmo de Deutsch-Jozsa para determinar si una función booleana
                f: {0,1}ⁿ → {0,1} es constante o equilibrada.

                1. Describa el circuito cuántico del algoritmo
                2. Demuestre que usa O(1) evaluaciones vs O(2ⁿ⁻¹) clásico
                3. Calcule la probabilidad de medir |0...0⟩ en el registro auxiliar
                4. Explique el principio de interferencia cuántica utilizado
                """,
                "solution": """
                SOLUCIÓN:

                1. CIRCUITO CUÁNTICO:
                   - Estado inicial: |0...0⟩|1⟩
                   - Hadamard en todos los qubits: |+...+⟩|−⟩
                   - Aplicar U_f: suma módulo 2 de f(x) al qubit auxiliar
                   - Hadamard en qubits de entrada
                   - Medir qubits de entrada

                2. COMPLEJIDAD:
                   - Clásico: necesita 2ⁿ⁻¹ + 1 evaluaciones en peor caso
                   - Cuántico: 1 evaluación determina la respuesta
                   - Ventaja exponencial: O(1) vs O(2ⁿ)

                3. PROBABILIDAD:
                   - Para función constante: P(|0...0⟩) = 1
                   - Para función equilibrada: P(|0...0⟩) = 0
                   - Estado final: (1/√2ⁿ) Σₓ (-1)^{f(x)} |x⟩
                   - Amplitud en |0...0⟩: (1/√2ⁿ) Σₓ (-1)^{f(x)}

                4. INTERFERENCIA CUÁNTICA:
                   - Transformada de Hadamard crea superposición
                   - Interferencia constructiva/destructiva según f(x)
                   - Para constante: todas las fases igual → amplificación
                   - Para equilibrada: fases alternan → cancelación
                """,
                "tags": ["quantum_algorithms", "deutsch_jozsa", "oracle", "interference"]
            },
            {
                "title": "Problema de Información Cuántica - Teleportación",
                "category": "Quantum Information",
                "difficulty": "expert",
                "problem": """
                En el protocolo de teleportación cuántica:

                1. Demuestre que el estado |ψ⟩ = α|0⟩ + β|1⟩ se puede teleportar
                2. Calcule las probabilidades de los resultados de medición
                3. Muestre cómo reconstruir el estado original
                4. ¿Por qué no viola el teorema de no-clonación?
                """,
                "solution": """
                SOLUCIÓN:

                1. ESTADO INICIAL ENREDADO:
                   |Φ⁺⟩ = (|00⟩ + |11⟩)/√2 compartido entre Alice y Bob

                2. ESTADO TOTAL:
                   |ψ⟩|Φ⁺⟩ = α|0⟩(|00⟩ + |11⟩)/√2 + β|1⟩(|00⟩ + |11⟩)/√2
                   = (1/√2) [α|000⟩ + α|011⟩ + β|100⟩ + β|111⟩]

                3. MEDICIÓN DE ALICE (BELL BASIS):
                   Base: {|Φ⁺⟩, |Φ⁻⟩, |Ψ⁺⟩, |Ψ⁻⟩}
                   Probabilidades todas 1/4

                4. RECONSTRUCCIÓN POR BOB:
                   - Si mide |Φ⁺⟩: Bob aplica I → obtiene α|0⟩ + β|1⟩
                   - Si mide |Φ⁻⟩: Bob aplica Z → α|0⟩ - β|1⟩
                   - Si mide |Ψ⁺⟩: Bob aplica X → β|0⟩ + α|1⟩
                   - Si mide |Ψ⁻⟩: Bob aplica XZ → β|0⟩ - α|1⟩

                5. NO VIOLA NO-CLONACIÓN:
                   - El estado original se destruye en Alice
                   - Solo se transfiere, no se copia
                   - Requiere canal clásico adicional
                """,
                "tags": ["quantum_teleportation", "bell_states", "entanglement", "no_cloning"]
            }
        ]

        for problem in quantum_problems:
            entry = {
                "id": self._generate_id(Path(problem["title"])),
                "source": "quantum_problem",
                "title": problem["title"],
                "category": problem["category"],
                "difficulty": problem["difficulty"],
                "problem": problem["problem"],
                "solution": problem["solution"],
                "tags": problem["tags"]
            }
            self.dataset.append(entry)

    def _generate_six_sigma_problems(self):
        """Generar problemas de Six Sigma"""
        print("Generando problemas de Six Sigma...")

        sigma_problems = [
            {
                "title": "Problema DMAIC - Reducción de Defectos en Producción",
                "category": "Six Sigma",
                "difficulty": "advanced",
                "methodology": "DMAIC",
                "problem": """
                Una línea de producción tiene actualmente 4500 defectos por millón de oportunidades
                (DPMO = 4500). El objetivo es reducir esto a 1500 DPMO.

                1. Calcule el nivel sigma actual y objetivo
                2. ¿Cuánta mejora representa esto?
                3. Diseñe un plan DMAIC básico para lograr este objetivo
                4. Identifique métricas clave a rastrear
                """,
                "solution": """
                SOLUCIÓN:

                1. NIVEL SIGMA:
                   - Sigma actual: Para DPMO=4500, ≈ 3.5σ (lookup table)
                   - Sigma objetivo: Para DPMO=1500, ≈ 4.2σ
                   - Mejora: 4.2σ - 3.5σ = 0.7σ

                2. MEJORA TOTAL:
                   - Reducción de defectos: (4500-1500)/4500 = 66.7%
                   - Rendimiento mejora de 99.55% a 99.85%

                3. PLAN DMAIC:
                   DEFINE: Charter, SIPOC, VOC
                   MEASURE: Baseline data, capability analysis
                   ANALYZE: Root cause usando fishbone, FMEA
                   IMPROVE: DOE para soluciones, pilot testing
                   CONTROL: Control charts, reaction plans

                4. MÉTRICAS CLAVE:
                   - DPMO por proceso
                   - First Pass Yield (FPY)
                   - Cycle Time
                   - Customer Satisfaction Score
                   - Cost of Poor Quality (COPQ)
                """,
                "tags": ["dmaic", "dpmo", "process_improvement", "sigma_level"]
            },
            {
                "title": "Problema DOE - Optimización de Proceso Químico",
                "category": "Six Sigma",
                "difficulty": "expert",
                "methodology": "DOE",
                "problem": """
                Un proceso químico tiene 3 factores controlables:
                - Temperatura (A): 80°C, 90°C, 100°C
                - Presión (B): 1 atm, 2 atm, 3 atm
                - Tiempo (C): 30 min, 45 min, 60 min

                Rendimiento actual: 75% ± 5%
                Objetivo: Maximizar rendimiento > 90%

                1. Diseñe un experimento factorial completo
                2. Calcule grados de libertad y poder estadístico
                3. Identifique posibles interacciones importantes
                4. Proponga análisis estadístico para los resultados
                """,
                "solution": """
                SOLUCIÓN:

                1. EXPERIMENTO FACTORIAL 3³:
                   - 27 corridas experimentales (3×3×3)
                   - Incluye todos los efectos principales e interacciones
                   - Replicación recomendada: 2-3 veces por punto

                2. GRADOS DE LIBERTAD:
                   - Total: 27×r - 1 (r=replicas)
                   - Efectos principales: 3×2 = 6 df cada uno
                   - Interacciones 2-factor: 3×2×2 = 12 df cada una
                   - Interacción 3-factor: 2×2×2 = 8 df
                   - Error: total - suma de df anteriores

                3. INTERACCIONES IMPORTANTES:
                   - A×B: Temperatura-Presión (efecto cinético)
                   - A×C: Temperatura-Tiempo (equilibrio)
                   - B×C: Presión-Tiempo (solubilidad)
                   - A×B×C: Efecto combinado completo

                4. ANÁLISIS ESTADÍSTICO:
                   - ANOVA para identificar efectos significativos
                   - Gráficos de efectos principales e interacciones
                   - Modelo de regresión: Y = β₀ + ΣβᵢXᵢ + ΣβᵢⱼXᵢXⱼ + ε
                   - Optimización usando response surface methodology
                   - Confirmación con corridas adicionales
                """,
                "tags": ["doe", "factorial_design", "optimization", "statistical_analysis"]
            }
        ]

        for problem in sigma_problems:
            entry = {
                "id": self._generate_id(Path(problem["title"])),
                "source": "six_sigma_problem",
                "title": problem["title"],
                "category": problem["category"],
                "difficulty": problem["difficulty"],
                "methodology": problem.get("methodology"),
                "problem": problem["problem"],
                "solution": problem["solution"],
                "tags": problem["tags"]
            }
            self.dataset.append(entry)

    def _generate_information_theory_problems(self):
        """Generar problemas de teoría de la información"""
        print("Generando problemas de teoria de la informacion...")

        info_problems = [
            {
                "title": "Problema de Entropía - Cadena de Markov",
                "category": "Information Theory",
                "difficulty": "advanced",
                "topic": "Entropy",
                "problem": """
                Considere una cadena de Markov con estados {A, B, C} y matriz de transición:

                P = | 0.5  0.3  0.2 |
                    | 0.1  0.8  0.1 |
                    | 0.4  0.2  0.4 |

                Con distribución estacionaria π = [0.3, 0.4, 0.3]

                1. Calcule H(X), H(X|Y), I(X;Y)
                2. Determine si la cadena es irreducible
                3. Calcule la entropía de la fuente
                """,
                "solution": """
                SOLUCIÓN:

                1. ENTROPÍAS:
                   H(X) = -Σᵢ πᵢ log₂ πᵢ = -(0.3×log₂0.3 + 0.4×log₂0.4 + 0.3×log₂0.3)
                        = -(0.3×(-1.737) + 0.4×(-1.322) + 0.3×(-1.737))
                        = -( -0.521 - 0.529 - 0.521) = 1.571 bits

                   H(X|Y) requiere distribución conjunta P(X,Y)
                   I(X;Y) = H(X) - H(X|Y)

                2. CADENA IRREDUCIBLE:
                   Desde cualquier estado se puede llegar a cualquier otro:
                   A→B (0.3), A→C (0.2), B→A (0.1), B→C (0.1), C→A (0.4), C→B (0.2)
                   Sí, irreducible.

                3. ENTROPÍA DE LA FUENTE:
                   Para cadena de Markov ergódica: H = -Σᵢⱼ πᵢ pᵢⱼ log₂ pᵢⱼ
                   H = 0.3×H(X|A) + 0.4×H(X|B) + 0.3×H(X|C)
                   Donde H(X|A) = entropía de fila A, etc.
                """,
                "tags": ["entropy", "markov_chain", "stationary_distribution", "mutual_information"]
            },
            {
                "title": "Problema de Codificación - Código de Hamming",
                "category": "Information Theory",
                "difficulty": "intermediate",
                "topic": "Error-Correcting Codes",
                "problem": """
                Diseñe un código de Hamming (7,4) para corrección de un error.

                1. Determine la matriz generadora G y de paridad H
                2. Codifique la palabra 1011
                3. Si se recibe 1011010, ¿cuál fue la palabra original?
                4. Calcule la tasa y eficiencia del código
                """,
                "solution": """
                SOLUCIÓN:

                1. MATRICES:
                   G = [I₄ | P] donde P es la matriz de paridad 4×3
                   H = [P^T | I₃] matriz de síndromes 3×7

                   G = | 1 0 0 0 | 1 1 0 |
                       | 0 1 0 0 | 1 0 1 |
                       | 0 0 1 0 | 0 1 1 |
                       | 0 0 0 1 | 1 1 1 |

                   H = | 1 1 0 1 1 0 0 |
                       | 1 0 1 1 0 1 0 |
                       | 0 1 1 1 0 0 1 |

                2. CODIFICACIÓN DE 1011:
                   [1 0 1 1] × G = [1 0 1 1 | 1⊕1⊕0 0⊕0⊕1 0⊕1⊕1 1⊕1⊕1]
                   = [1 0 1 1 | 0⊕0⊕1 0⊕1⊕1 1⊕1⊕1] = [1 0 1 1 0 1 1]

                3. DECODIFICACIÓN DE 1011010:
                   Síndrome s = H × y^T = [1 0 0]^T → error en posición 1
                   Palabra corregida: 0011010 → datos: 0110

                4. TASA Y EFICIENCIA:
                   Tasa R = k/n = 4/7 ≈ 0.571
                   Eficiencia = 1 - (redundancia) = 4/7 ≈ 57.1%
                """,
                "tags": ["hamming_code", "error_correction", "linear_codes", "parity_check"]
            },
            {
                "title": "Problema de Capacidad de Canal - Canal Binario Simétrico",
                "category": "Information Theory",
                "difficulty": "advanced",
                "topic": "Channel Capacity",
                "problem": """
                Un canal binario simétrico (BSC) tiene probabilidad de cruce p = 0.1.

                1. Calcule la capacidad del canal C
                2. ¿Cuál es la distribución de entrada óptima?
                3. Si p = 0, ¿cuál es C? Si p = 0.5, ¿cuál es C?
                4. Diseñe un código que alcance capacidad
                """,
                "solution": """
                SOLUCIÓN:

                1. CAPACIDAD DEL BSC:
                   C = 1 - h(p) donde h(p) = -p log₂p - (1-p) log₂(1-p)
                   h(0.1) = -0.1×log₂0.1 - 0.9×log₂0.9 = -0.1×(-3.32) - 0.9×(-0.15)
                   = 0.332 + 0.135 = 0.467 bits
                   C = 1 - 0.467 = 0.533 bits

                2. DISTRIBUCIÓN ÓPTIMA:
                   Para BSC, distribución uniforme: P(0)=P(1)=0.5
                   Cualquier distribución simétrica alcanza capacidad

                3. CASOS EXTREMOS:
                   p=0: Canal perfecto, C=1 bit
                   p=0.5: Canal useless, C=0 bits

                4. CÓDIGO QUE ALCANZA CAPACIDAD:
                   - Códigos polares (Arikan) alcanzan C para BSC
                   - LDPC codes con buena construcción
                   - Turbo codes con iteración suficiente
                """,
                "tags": ["channel_capacity", "bsc", "binary_symmetric_channel", "polar_codes"]
            }
        ]

        for problem in info_problems:
            entry = {
                "id": self._generate_id(Path(problem["title"])),
                "source": "information_theory_problem",
                "title": problem["title"],
                "category": problem["category"],
                "difficulty": problem["difficulty"],
                "topic": problem.get("topic"),
                "problem": problem["problem"],
                "solution": problem["solution"],
                "tags": problem["tags"]
            }
            self.dataset.append(entry)

    def _create_research_center_content(self):
        """Crear contenido sobre centros de investigación"""
        print("Creando contenido de centros de investigacion...")

        research_content = [
            {
                "title": "Centro de Investigación Matemática Computacional",
                "content": """
                ESTRUCTURA RECOMENDADA PARA UN CENTRO DE INVESTIGACIÓN MATEMÁTICA:

                1. INFRAESTRUCTURA TÉCNICA:
                   - Cluster de computación de alto rendimiento
                   - GPUs para deep learning matemático
                   - Almacenamiento masivo para datasets científicos
                   - Red de alta velocidad

                2. EQUIPOS DE INVESTIGACIÓN:
                   - Grupo de geometría algebraica computacional
                   - Equipo de optimización convexa
                   - Investigadores en física matemática
                   - Desarrolladores de software científico

                3. METODOLOGÍAS:
                   - Desarrollo colaborativo con Git
                   - Code review riguroso
                   - Testing automático comprehensivo
                   - Documentación técnica detallada

                4. OUTPUTS ESPERADOS:
                   - Algoritmos optimizados publicados
                   - Librerías de software matemático
                   - Papers en journals especializados
                   - Datasets de investigación públicos
                """,
                "category": "Research Center Design",
                "language": "Julia",
                "focus": "Computational Mathematics"
            }
        ]

        for content in research_content:
            entry = {
                "id": self._generate_id(Path(content["title"])),
                "source": "research_center",
                "title": content["title"],
                "content": content["content"],
                "category": content["category"],
                "language": content.get("language"),
                "focus": content.get("focus"),
                "difficulty": "advanced",
                "tags": ["research", "organization", "mathematics", "computing"]
            }
            self.dataset.append(entry)

    def _save_complete_dataset(self):
        """Guardar el dataset completo"""
        print(f"Guardando dataset completo ({len(self.dataset)} registros)...")

        # Guardar como JSONL (un registro por línea)
        output_file = self.output_dir / "massive_math_dataset.jsonl"

        with open(output_file, 'w', encoding='utf-8') as f:
            for entry in self.dataset:
                json.dump(entry, f, ensure_ascii=False)
                f.write('\n')

        # Crear índice del dataset
        index = {
            "total_records": len(self.dataset),
            "categories": {},
            "languages": {},
            "sources": {},
            "difficulties": {},
            "created_at": "2024-01-24",
            "version": "1.0"
        }

        # Contar por categorías
        for entry in self.dataset:
            cat = entry.get("category", "unknown")
            lang = entry.get("language", "unknown")
            src = entry.get("source", "unknown")
            diff = entry.get("difficulty", "unknown")

            index["categories"][cat] = index["categories"].get(cat, 0) + 1
            index["languages"][lang] = index["languages"].get(lang, 0) + 1
            index["sources"][src] = index["sources"].get(src, 0) + 1
            index["difficulties"][diff] = index["difficulties"].get(diff, 0) + 1

        # Guardar índice
        index_file = self.output_dir / "dataset_index.json"
        with open(index_file, 'w', encoding='utf-8') as f:
            json.dump(index, f, indent=2, ensure_ascii=False)

        print(f"Dataset guardado: {output_file}")
        print(f"Indice guardado: {index_file}")

def main():
    """Función principal"""
    math_root = r"D:\PROJECTS\NUCLEARCRAWLER\MATH"
    output_dir = r"D:\models\data\math_dataset"

    creator = MassiveMathDatasetCreator(math_root, output_dir)
    creator.process_all_mathematical_content()

if __name__ == "__main__":
    main()