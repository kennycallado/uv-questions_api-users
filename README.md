# Questions api BASE

## Ramas git

Este procedimiento es importante para mantener la coherencia entre todo el proyecto.

### Iniciar nuevo repositorio

Existen una serie de pasos importantes a seguir:

1. Crear el repositorio local
   ``` bash
   git init
   ```

1. Crear una rama base
   ``` bash
   git checkout -b base
   ```

1. Añadir remoto a base
   ``` bash
   git remote add base git@github.com:kennycallado/uv-questions_api-base.git
   ```

1. Captura la rama main de base
   ``` bash
   git fetch base
   ```

1. Incorporar esta rama en local
   ``` bash
   git merge base
   ```

1. Apuntar el upstream de la rama base con main de base
   ``` bash
   git branch --set-upstream-to=base/main
   ```

1. Creamos rama main
   ``` bash
   git checkout -b main
   ```

1. Añadir remoto a main **ojo** nombre repositorio
   ```bash
   git remote add origin git@github.com:<repositorio>.git

1. Enviamos contenido al nuevo repositorio
   ``` bash
   git push origin main
   ```

1. Apuntar el upstream de la rama main con main de origin
   ``` bash
   git branch --set-upstream-to=origin/main
   ```
---

### Incorporar cambios desde base



---

## Ficheros para adaptar

### Raíz del proyecto

- [ ] .env
  - Dirección de la base de datos
- [ ] Cargo.toml
  - Nombre del paquete
  - Revisar dependencias
- [ ] compose.yaml
  - Variables de entorno
  - Servicios extra
- [ ] Dockerfile
  - Nombre del binario, viene desde Cargo.toml
- [ ] Rocket.toml
  - Dirección de la base de datos

### Directorio src

- [ ] Tests

### Migraciones

Cada api tiene sus propias migraciones localizadas en el directorio `src/database/migrations`

### Modules

Directorio principal de trabajo de cada api. Contendrá un módulo por cada entidad con la que trabaje la api y administrará sus rustas.

## Module

Cada módulo deberá contener, `model.rs` y `controller.rs`. En caso de ser necesario el controlador puede ayudarse de un directorio `handlers` y el modelo puede tener un repositorio dentro del directorio `services`.

El directorio de servicios del módulo también puede contener por ejemplo, `helpers` para el controlador o implementación de `claims` para entidad user.
