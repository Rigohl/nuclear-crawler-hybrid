# Configuración de entorno para Nuclear Crawler Hybrid
# Este archivo establece las variables de entorno necesarias

# JARVIXSERVER - Backend de análisis e IA
export JARVIX_URL=http://localhost:5050

# Nuclear MCP Server
export NUCLEAR_MCP_PORT=8079

# TRAE CLI - Configuración por defecto
export TRAE_PROJECT_PATH="$(pwd)"

echo "🔧 Variables de entorno configuradas:"
echo "   JARVIXSERVER: $JARVIX_URL"
echo "   Nuclear MCP: http://localhost:$NUCLEAR_MCP_PORT"
echo "   TRAE CLI Project: $TRAE_PROJECT_PATH"
