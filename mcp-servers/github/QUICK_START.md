# Quick Start Guide

Guía rápida para comenzar con github-mcp-server.

## Instalación Rápida

### Opción 1: Docker (Recomendado)

```bash
# 1. Obtener token de GitHub
# Visita: https://github.com/settings/personal-access-tokens/new

# 2. Ejecutar con Docker
docker run -i --rm \
  -e GITHUB_PERSONAL_ACCESS_TOKEN=tu_token_aqui \
  ghcr.io/github/github-mcp-server
```

### Opción 2: Desarrollo Local

```bash
# 1. Clonar repositorio
git clone https://github.com/github/github-mcp-server.git
cd github-mcp-server

# 2. Configurar entorno
./script/setup-dev

# 3. Editar .env y agregar tu token
# GITHUB_PERSONAL_ACCESS_TOKEN=tu_token_aqui

# 4. Ejecutar
make run
```

## Configuración Básica

### Variables de Entorno Mínimas

```bash
# Requerido
GITHUB_PERSONAL_ACCESS_TOKEN=ghp_xxxxxxxxxxxxx

# Opcionales
GITHUB_TOOLSETS=repos,issues,pull_requests
GITHUB_READ_ONLY=0
```

### Configuración en VS Code

Agrega a tu configuración de MCP:

```json
{
  "mcp": {
    "servers": {
      "github": {
        "command": "docker",
        "args": [
          "run", "-i", "--rm",
          "-e", "GITHUB_PERSONAL_ACCESS_TOKEN",
          "ghcr.io/github/github-mcp-server"
        ],
        "env": {
          "GITHUB_PERSONAL_ACCESS_TOKEN": "${input:github_token}"
        }
      }
    }
  }
}
```

## Comandos Útiles

```bash
# Ver ayuda
make help

# Validar configuración
./script/validate-config

# Ejecutar tests
make test

# Construir binario
make build

# Ejecutar linters
make lint
```

## Solución de Problemas

### Token no válido
```bash
# Verificar formato del token
./script/validate-config
```

### Docker no funciona
```bash
# Verificar que Docker esté corriendo
docker info
```

### Errores de permisos
```bash
# Verificar scopes del token
# El token necesita al menos: repo, read:packages, read:org
```

## Próximos Pasos

- Lee [README.md](README.md) para documentación completa
- Revisa [CONTRIBUTING.md](CONTRIBUTING.md) para contribuir
- Consulta [docs/](docs/) para guías detalladas

