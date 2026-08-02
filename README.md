# Gestor de Notas

🔗 **App en vivo:** https://malvarezdam.github.io/Gestor-de-Notas/

Aplicación web para llevar el control de notas universitarias: ramos (cursos), secciones (cátedra, laboratorio, etc.), tipos de evaluación (certámenes, controles, tareas...) y evaluaciones individuales, con cálculo de promedio ponderado ascendente. Corre enteramente en el navegador (no hay servidor ni base de datos), con guardado automático local y sincronización opcional con Google Drive.

## Qué hace

- Organiza tus ramos en una barra lateral: agregar, renombrar, borrar y reordenar.
- Por cada ramo, arma columnas tipo tablero para sus secciones (Cátedra, Laboratorio, etc.), cada una con su propia ponderación y un **factor η** (ajuste de esfuerzo, como se usa en algunas universidades chilenas).
- Dentro de cada sección se agregan tipos de evaluación, y dentro de cada tipo, las evaluaciones individuales con su nota y ponderación.
- El cálculo va de abajo hacia arriba: evaluación → tipo de evaluación → sección (multiplicada por η) → nota final del ramo.
- Un botón "Promediar" en cada nivel reparte el 100% en partes iguales entre sus elementos hijos y muestra la nota resultante.
- Puedes definir una **nota objetivo** por ramo: si solo te falta ingresar una nota, la app calcula automáticamente qué necesitas sacarte para llegar a tu objetivo (y te avisa si es alcanzable según la nota máxima/mínima de tu escala, configurable en Ajustes).
- Guardado automático en el navegador (localStorage) — no se pierde nada al recargar la página.
- Conexión opcional con Google Drive para respaldar y sincronizar tus datos entre dispositivos.

## Cómo usarla

1. Abre https://malvarezdam.github.io/Gestor-de-Notas/
2. En la barra lateral, presiona **"+ Agregar ramo"** y ponle nombre.
3. Dentro del ramo, presiona **"+ Agregar sección"** para crear columnas (ej. "Cátedra", "Laboratorio"), define su ponderación (%) y su factor η.
4. Dentro de cada sección, presiona **"+ Agregar tipo de evaluación"** (ej. "Certámenes") y su ponderación dentro de la sección.
5. Dentro de cada tipo, presiona **"+ Agregar evaluación"** para ingresar evaluaciones individuales (nombre, ponderación y nota).
6. Usa los botones **▲ / ▼** en cualquier nivel para reordenar, y el botón **"Promediar"** para repartir ponderaciones parejas y ver la nota resultante.
7. En **"⚙ Ajustes"** puedes definir la nota mínima y máxima de tu escala (1–7, 1–10, 0–100, la que uses).
8. En **"Nota objetivo"** (dentro de cada ramo) puedes definir a qué nota final quieres llegar.
9. Todo se guarda solo en tu navegador. Si quieres respaldarlo en la nube, usa el botón **"Conectar Google Drive"** en la barra lateral (ver configuración más abajo).

## Lenguajes y tecnologías

- **Rust**, compilado a **WebAssembly (WASM)**.
- [**Leptos**](https://leptos.dev/) como framework de UI reactiva (modo CSR, sin servidor).
- **HTML/CSS** para el tema oscuro y el layout.
- **JavaScript** mínimo, solo como puente para APIs del navegador que Rust/WASM no puede llamar directo (Google Identity Services para el login de Drive).
- [**Trunk**](https://trunkrs.dev/) como bundler/build tool.
- Desplegado como sitio estático en **GitHub Pages**, publicado automáticamente vía **GitHub Actions**.

## Requisitos para desarrollo local

- [Rust](https://rustup.rs/) con el target `wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Levantar el servidor de desarrollo:

```bash
trunk serve --open
```

Para probar la integración con Google Drive en local, exporta el Client ID antes de levantar el servidor (ver sección siguiente):

```powershell
$env:GOOGLE_CLIENT_ID="tu-client-id.apps.googleusercontent.com"
trunk serve --open
```

## Build de producción manual

```bash
trunk build --release
```

El resultado queda en `dist/`.
