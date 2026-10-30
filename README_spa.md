<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-ROUTEPRO banner" width="100%">
</p>

# 🪚 EH3D-ROUTEPRO

<p align="center"><a href="README.md">🇺🇸 English</a> | 🇪🇸 <b>Español</b> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### Una reimplementación multiplataforma en Rust del flujo de fresado del clásico software DOS para máquinas Bungard CCD, a partir de ficheros de trazado HP-GL, con una interfaz de modo texto parecida a la original en ocho idiomas

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.5-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**Comprobación de honestidad - lo que se ejecuta hoy:** un programa en Rust con interfaz de texto de 80x25 que lee ficheros de trazado HP-GL `.PLT`, la tabla de herramientas y la configuración de la máquina, planifica trayectorias de fresado de varias pasadas (avance rápido, penetración, corte, elevación) y las ejecuta **contra un simulador** (por defecto) o, cuando el fichero de la máquina lo indica, **en una máquina real** por el puerto serie del ordenador, un adaptador USB-serie o el EH3D-NODE-BUNGARD. 50 pruebas se ejecutan en un ordenador. El protocolo serie que comparte con EH3D-DRILLPRO se ha probado en una Bungard CCD MTC real (la referencia de los tres ejes y los movimientos de cada eje); **un trabajo de fresado completo todavía no se ha ejecutado en la máquina**, y los pasos por milímetro solo se han contrastado con la configuración del original. Por defecto se muestran las pantallas clásicas del programa MS-DOS, comprobadas celda a celda contra el original; `--modern` inicia la interfaz de menús. Se manejan con teclado y ratón, la ventana se puede maximizar (la pantalla escala de 640x400 a 1080p y más), los ficheros de configuración y de herramientas se editan y se guardan desde los diálogos, y el botón Info lista las máquinas Bungard CCD y CCD/2 (MTC soportadas, ATC todavía no). Una versión de consola (`routepro-console`) dibuja las mismas pantallas en un terminal, con teclado y ratón y sin sistema de ventanas (SSH, consola de texto, un PC antiguo). Los paquetes de cada release incluyen la configuración básica, explicada línea a línea, y una carpeta `examples` desde un cuadrado hasta cien mil puntos. El cambiador de herramienta de los modelos ATC todavía no está admitido.

---

> **Estado de seguridad (safety) y de seguridad informática:** este programa no es un sistema de seguridad. Con una conexión real (`connection` en el fichero de la máquina) **Iniciar mueve la máquina**: saca la herramienta, mantén una mano en el interruptor de la máquina y ten lista la tecla `Esc`, que detiene el trabajo al instante y apaga los drivers de los motores. Los finales de carrera los vigila la propia unidad de control de la máquina (para el eje en su tope por sí sola); este software no tiene parada de emergencia propia, y unos bytes de paso o velocidades erróneos pueden llevar la herramienta contra la pieza o la mesa o hacer que los motores pierdan pasos. La capa de protocolo se niega a mover nada hasta que se arma. El proyecto es independiente: no lo fabrica, respalda ni da soporte Bungard Elektronik, y los nombres de producto aparecen solo para indicar con qué formatos de fichero y máquinas debe funcionar.

## 📋 De un vistazo

| | |
|---|---|
| Versión | `0.0.5` |
| Madurez | Andamiaje |
| Pruebas | 50 |
| Pila tecnológica | Rust |
| En hardware real | Sí, en una Bungard CCD MTC: la referencia y los movimientos de cada eje; un trabajo completo todavía no se ha ejecutado. Sin fichero de máquina se ejecuta en el simulador |

**Ir a:** [Resumen](#-resumen) · [Arquitectura](#-arquitectura) · [Inicio rápido](#-inicio-rápido) · [Pruebas](#-pruebas) · [Estructura](#-estructura-del-repositorio) · [Relacionados](#-proyectos-relacionados)

## 🚦 Estado del proyecto

Cada línea es una parte de este repositorio y la luz dice hasta dónde llega hoy. Se escribe con lo que el repositorio puede demostrar (pruebas, compilaciones, ejecuciones), no con lo previsto.

🟢 Hecho y verificado · 🟡 Funciona, con límites conocidos · 🟠 Hecho a medias o sin verificar · 🔴 Sin hacer o sin funcionar

| Área | Estado | Qué significa aquí |
|---|---|---|
| Software | 🟡 | Un programa en Rust con pantalla de texto de 80x25 lee ficheros de trazado HP-GL, la tabla de herramientas y el fichero de la máquina, planifica trayectorias de fresado de varias pasadas y las ejecuta contra un simulador, o en una máquina real si el fichero de la máquina lo indica; necesita la carpeta de EH3D-DRILLPRO a su lado. |
| Pruebas | 🟢 | Pasan 50 pruebas (`cargo test --workspace`): el lector de HP-GL, la planificación de trayectorias y las pantallas en los ocho idiomas; el protocolo lo cubren las 212 pruebas de EH3D-DRILLPRO. |
| Lector de HP-GL | 🟡 | Solo se entienden `IN`, `SP`, `PU`, `PD`, `PA` y `PR`; cualquier otra orden se cuenta y se ignora, nunca es fatal. Probado solo con pequeños ficheros de muestra escritos para las pruebas. |
| Planificación de trayectorias | 🟡 | Cada trayectoria se convierte en un desplazamiento rápido, una penetración, el corte y una elevación en cada pasada, con la profundidad repartida por igual y la herramienta terminando siempre en la superficie; comprobado solo en pruebas y en el simulador. |
| Hardware real | 🟠 | El protocolo serie que comparte con EH3D-DRILLPRO se ha probado en una Bungard CCD MTC (la marcha de referencia y los movimientos de cada eje); ningún trabajo de fresado se ha ejecutado en una máquina, y las salidas del husillo y del aspirador no se han probado. |
| Seguridad de uso | 🟡 | `Esc` detiene el trabajo en una fracción de segundo y apaga los drivers de los motores, y la propia unidad de control de la máquina vigila los finales de carrera; el software no tiene parada de emergencia propia. |
| Conexión con el servidor | 🟠 | Con `connection = server` el programa envía el trabajo a EH3D-SERVER y sigue su avance; este camino solo se ha ejercitado contra el simulador. |
| Paquetes listos para usar | 🟡 | La primera versión publicada lleva paquetes para Windows, Linux y macOS (Apple silicon e Intel), cada uno con su SHA-256, generados por el flujo de publicación. |
| Documentación | 🟢 | README en ocho idiomas y las notas del protocolo con lo que aún es hipótesis; la guía del fichero de la máquina está en EH3D-DRILLPRO. |

## 🎯 Resumen

* Los **lectores de ficheros**: ficheros de trazado HP-GL `.PLT` (`IN`, `SP`, `PU`, `PD`, `PA`, `PR`; cualquier otra orden se cuenta y se ignora, nunca es fatal), la tabla de herramientas `ROUTEPRO.RTD` (profundidad, pasadas, avances en XY y Z, velocidad del husillo) y la configuración de la máquina `ROUTEPRO.RCF`.
* El **planificador de trayectorias**: cada trayectoria se convierte en un movimiento rápido a su inicio y, en cada pasada, una penetración, el corte a lo largo de la trayectoria y una elevación; la profundidad se reparte por igual entre las pasadas y la herramienta siempre termina en la superficie.
* La **interfaz**: una pantalla de modo texto de 80x25 con colores CGA, menús, un diálogo de ficheros y una vista vectorial de los agujeros, mostrada en una ventana. Habla inglés, español, italiano, alemán, francés, portugués, chino y japonés: elige el idioma en el menú o arranca con `--lang`.
* El **protocolo de la máquina** (`protocol/`, crate `eh3d-routepro-protocol`): reexporta la biblioteca de protocolo de EH3D-DRILLPRO (transporte de bytes, planificación de movimiento, compuerta de seguridad) y añade el plan de fresado.
* El **simulador**: el trabajo completo se ejecuta hasta el final sin hardware; las pruebas lo usan.
* Las **máquinas**: las que llevan una central por puerto serie que usaba el software de MS-DOS 4.6: Bungard CCD MTC y CCD/2 MTC (cambio de herramienta manual; la CCD/2 tiene la mesa más pequeña de 325 x 270 mm). El tamaño de la mesa y demás constantes salen del fichero de configuración de la máquina. Solo se ha probado en una CCD MTC, y las salidas del taladro (husillo) y de la activación de la aspiradora todavía no se han probado. Los dos modelos ATC (CCD ATC y CCD/2 ATC, cambio automático) tendrán aplicaciones aparte, hechas con lo que permita la ingeniería inversa. Los modelos XL y las máquinas posteriores llevan unidad de control y software modernos propios y no están cubiertos.
* El **fichero de la máquina** (`machine.cfg`, compartido por DRILLPRO y ROUTEPRO): cómo llega el trabajo a la máquina (`demo` simulador, `rs232` el puerto serie del propio ordenador, `usb` un adaptador USB-serie o `node` el EH3D-NODE-BUNGARD por la red) y, un valor por línea, todo lo afinado en una máquina real: velocidades y rampas de búsqueda, el salto de calibración, los límites de los motores.
* La **referencia**: Z sube hasta su final de carrera, luego X e Y, y después un salto de calibración suave. La unidad de control indica «los tres ejes en home» por la línea RI, que el programa vigila; con la máquina aparcada se salta las búsquedas, separa X e Y 10 mm de sus finales de carrera y los busca de nuevo, así los ve llegar en directo.
* **Movimiento estable**: los movimientos salen como un único flujo continuo cuyas posiciones de byte llevan el tiempo (un adaptador USB entrega los bytes a ráfagas si no), Z se mueve byte a byte, y `Esc` cancela un trabajo en una fracción de segundo y apaga los drivers de los motores.
* El **husillo**: por defecto el de la propia central (sus bits de latch, hasta 60000 rpm); con `spindle = node` en el fichero de la máquina el trabajo se lo pide al EH3D-NODE-BUNGARD (un variador de frecuencia en las salidas del nodo, cualquier velocidad hasta el máximo del nodo, parado en cualquier parada). Con un nodo los programas leen además los LED de la máquina, así cada eje que llega a home se ve en directo sin leer la línea de home en mitad del flujo.
* La **conexión al servidor** (`connection = server` en el fichero de la máquina, con `server_address`, `server_token` y `server_machine`): en vez de manejar la máquina él mismo, el programa envía el trabajo al [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER), que lo planifica con las constantes de la máquina, lo ejecuta a través del nodo de la máquina y le cuenta al programa cómo va; el programa muestra el avance y responde a los diálogos como siempre, `Esc` detiene el trabajo en el servidor, y cerrar el programa no lo detiene.
* Los **archivos de trabajo, la ventana Offset y la simulación**: un trabajo puede ir empaquetado en un archivo ZIP (la ventana Select File entra en él y `routepro JOB.ZIP` carga su primer fichero de fresado); la flecha junto al nombre del fichero lista los patrones `*.PLT`, `*.ZIP` y `*.*`; los botones Load de las ventanas de herramientas y de configuración eligen el fichero que se carga; en una máquina real la ventana Offset hace el home, sigue cada movimiento de la ventana y vuelve a hacer el home; y el trabajo simulado tarda lo que tardaría la máquina.
* Los **sonidos**: `sound_finish` y `sound_tool_change` en el fichero de la máquina son cada uno off (lo normal), `default` (un tono que hace el programa) o la ruta de un archivo WAV propio; el primero suena cuando un trabajo termina solo, el segundo cuando la máquina pide una herramienta.
* Los **dibujos de una placa**: además del HP-GL, un dibujo `.DXF` (líneas, polilíneas con arcos, arcos y círculos) o la capa de contorno de una placa Gerber (`.GKO`, `.GM1`, `.GML`, o un `.GBR` que se declara perfil) se carga como trayectoria de fresado; los arcos, círculos y escalado del HP-GL (`AA`, `AR`, `CI`, `SC`, `IP`) se cortan en cuerdas según el ángulo de cuerda de la configuración.
* **Varias máquinas y estaciones**: con `connection = server`, `routepro --machines` lista todas las máquinas del servidor, agrupadas por estación, con lo que hace cada una, su trabajo y los trabajos listos (el servidor responde por todas a la vez), y `routepro --machine ID` envía los trabajos de este programa a esa máquina en lugar de la del fichero de máquina; las máquinas trabajan a la vez, un trabajo en cada una.
* **La conexión elegida en la ventana Config:** un desplegable con las cinco formas en que el trabajo llega a la máquina (el simulador, el puerto serie, un adaptador USB, el nodo EH3D y el servidor EH3D) y, debajo, lo que necesita cada una: un puerto, una dirección y un token, una máquina del servidor; lo elegido se comprueba, se aplica con OK y se escribe en el fichero de máquina sin tocar el resto de sus líneas.
* **Un cursor que parpadea:** el cursor de un campo numérico o de texto que se edita parpadea (medio segundo encendido, medio segundo apagado), como en las pantallas que sigue.

### 🆚 El original y esta versión

El original es el programa cerrado para MS-DOS de Bungard que fresa con las máquinas CCD. Esta versión está escrita de nuevo a partir de grabaciones de lo que hace el original (sus pantallas, sus ficheros y lo que envía a la unidad de control); no lleva código del original. Lo que es igual y lo que es distinto:

| Aspecto | Original (MS-DOS) | Esta versión |
|---|---|---|
| Equipo | Solo un PC con MS-DOS (o un emulador) | Windows, Linux y macOS, 32 y 64 bits; una versión de consola para SSH y PCs antiguos |
| Pantallas | Modo texto 80x25 | Las mismas pantallas clásicas por defecto, comprobadas celda a celda con el original, con ratón y una ventana que escala de 640x400 a 1080p y más; `--modern` ofrece una interfaz con menús y EH3D-BUNGARD-GUI una gráfica |
| Idiomas | El del programa | Ocho: inglés, español, italiano, alemán, francés, portugués, chino y japonés, que se cambian desde el menú o con `--lang` |
| Paneles | Copias de la placa en la mesa | Copias de la placa, y el exterior del panel se puede rutear con muescas (cuántas por lado, de qué ancho y dónde) que sujetan las placas hasta romperlas; un trabajo interrumpido a medias puede seguir |
| Enlace con la máquina | El puerto serie del equipo hacia la unidad de control | El simulador por defecto; en una máquina real el puerto serie, un adaptador USB serie, el EH3D-NODE-BUNGARD por la red o el EH3D-SERVER, elegido en un desplegable de la ventana Config |
| Añadidos | Ninguno de estos | Un simulador que ejecuta un trabajo entero, sonidos para el fin del trabajo y para la petición de herramienta, varias máquinas a la vez con el servidor, archivos ZIP de trabajo, contornos DXF y Gerber, un cursor parpadeante y un conjunto de pruebas que corre en cada cambio |
| Código y licencia | Cerrado, propietario | Código abierto (GPL-3.0-or-later), escrito en Rust |
| Madurez | Años de uso en máquinas reales | Mucho más joven: el recorrido de referencia y los movimientos de cada eje se han probado en una Bungard CCD MTC real; un trabajo entero aún no se ha ejecutado en una máquina, y el cambiador de herramientas de los modelos ATC no ha funcionado en una real |

## 🧩 Papel en EH3D

EH3D-ROUTEPRO es el hermano de fresado de EH3D-DRILLPRO y depende de él por ruta: la carpeta `EH3D-DRILLPRO` debe estar junto a esta. Reutiliza el protocolo serie, la pantalla y el soporte de idiomas. Todavía no depende de ningún otro repositorio de EH3D. Los ficheros de la máquina (el de taladrado, el HP-GL, las tablas de herramientas y la configuración de la máquina) vienen del crate `eh3d-bungard-formats` de EH3D-COMMON, que también tiene que estar a su lado.

## 🏛️ Arquitectura

<p align="center">
  <img src="images/architecture.svg" alt="Arquitectura de EH3D-ROUTEPRO: quién lo usa, qué contiene y de qué depende" width="100%">
</p>

Cómo encajan las partes de este repositorio: quién lo usa, qué contiene y de qué depende. La imagen se dibuja desde `docs/architecture.json` y se comprueba contra el código.

## 🖼️ Capturas de pantalla

<p align="center">
  <img src="images/screenshots/main.png" alt="main screen" width="80%">
</p>

<p align="center"><em>La pantalla principal del programa clásico de fresado: la escala, el desplazamiento y el recorte, los vectores del trazado y los botones.</em></p>

<p align="center">
  <img src="images/screenshots/tools.png" alt="tool data" width="80%">
</p>

<p align="center"><em>Los datos de herramientas: la profundidad, las pasadas, los avances por la trayectoria y de penetración, y la velocidad del husillo de cada herramienta.</em></p>

<p align="center">
  <img src="images/screenshots/connection.png" alt="connection" width="80%">
</p>

<p align="center"><em>La conexión con la máquina: la forma elegida en el desplegable y lo que necesita (aquí el servidor EH3D).</em></p>

## 🚀 Inicio rápido

Necesitas una cadena de herramientas de Rust estable.

```bash
cargo build --release --features gui
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

Ejecútalo con `run.bat` (Windows) o `./run.sh` (Linux y macOS); ambos compilan antes el programa si hace falta. Los dos necesitan Rust y la carpeta hermana `EH3D-DRILLPRO`: `run.bat [FICHERO.PLT] [--lang en|es|it|de|fr|pt|zh|ja] [--data DIR] [--dump]`. `--data` es la carpeta con `ROUTEPRO.RTD` y `ROUTEPRO.RCF` (por defecto: `testdata/`) y `--dump` imprime la pantalla como texto plano. Teclas: `Alt` más la letra resaltada abre un menú, las flechas mueven, `Enter` acepta, `Esc` vuelve; también se puede pulsar la barra de menús. El chino y el japonés necesitan una fuente CJK entre las del sistema. `--connection FILE` indica otro fichero de máquina distinto del predeterminado (`%APPDATA%\EH3D\machine.cfg` en Windows, `~/.config/eh3d/machine.cfg` en el resto); sin fichero de máquina el programa se ejecuta en el simulador. `--modern` inicia la interfaz de menús en lugar de las pantallas clásicas.

## 🧪 Pruebas

La suite tiene 50 pruebas (`cargo test --workspace`). Comprueban el lector HP-GL, la planificación de trayectorias (pasadas, penetración, corte y elevación), el plan de fresado del crate del protocolo, las pantallas clásicas celda a celda y las pantallas en los ocho idiomas. El protocolo que reutiliza (bytes, movimiento, compuerta de seguridad) se prueba en EH3D-DRILLPRO. La CI también ejecuta `cargo fmt --check` y `cargo clippy` con las advertencias como errores, y `python -I tools/ci_validate.py` comprueba el manifiesto, las versiones, el registro de cambios y los ocho README.

## 🏗️ Compilación y publicación

La versión vive en `Cargo.toml` y en `eh3d.project.json`, y cada versión tiene su entrada en [CHANGELOG.md](CHANGELOG.md). Las versiones se publican en GitHub.

**Paquetes ya compilados:** cada versión publicada lleva un paquete para Windows (zip), Linux (tar.gz) y macOS (Apple silicon e Intel, tar.gz) con el programa, la licencia y este README, cada uno con su SHA-256 al lado. Los construye el workflow `Release binaries` de este repositorio. Descarga uno desde la página de Releases; en Linux la ventana necesita X11 o Wayland.

## 📂 Estructura del repositorio

<p align="center">
  <img src="images/repository-structure.svg" alt="Estructura del repositorio EH3D-ROUTEPRO" width="100%">
</p>

<details>
<summary>Versión en texto del árbol</summary>

```text
EH3D-ROUTEPRO/
├── src/
│   ├── job.rs                                   las rutas HP-GL convertidas en el trabajo de fresado
│   ├── app.rs texts.rs                          la aplicación y sus textos en ocho idiomas
│   └── bin/                                     los ejecutables routepro y routepro-console
├── protocol/src/
│   ├── route.rs                                 pasadas, penetración, corte y elevación de cada trayecto
│   └── lib.rs                                   reexporta el protocolo de EH3D-DRILLPRO
├── tests/                                       lectura de HP-GL, planificación de trayectorias, pantallas clásicas y pantallas en todos los idiomas
├── docs/                                        las notas del protocolo, las pantallas clásicas
├── testdata/                                    pequeños archivos de muestra escritos para las pruebas
├── config/                                      la configuración básica, explicada línea a línea
├── examples/                                    ejemplos, de un cuadrado a 100000 puntos
├── tools/                                       comprobación de CI, del README y las herramientas de imágenes
├── images/                                      banner, las dos imágenes y las capturas de pantalla
├── .github/                                     flujos de CI y de publicación, plantillas de issues y pull requests
├── build.sh build.bat                           scripts de compilación y pruebas
├── run.sh run.bat                               arrancan el programa
├── eh3d.project.json                            manifiesto: versión, rol, madurez y comando de compilación
├── Cargo.toml Cargo.lock                        el paquete de Rust y sus dependencias fijadas
├── CHANGELOG.md                                 qué cambió en cada versión
├── SECURITY.md SUPPORT.md                       cómo informar de una vulnerabilidad y dónde preguntar
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   cómo contribuir, el código de conducta y la licencia
```

</details>

## 🗺️ Hoja de ruta y límites conocidos

**Límites conocidos hoy**

* Todavía no ha corrido un trabajo de fresado en una máquina real: el protocolo serie que comparte con EH3D-DRILLPRO se ha probado en una Bungard CCD MTC (la marcha de referencia de los tres ejes y los movimientos de cada eje), los pasos por milímetro solo están comprobados contra la configuración del original y las salidas del husillo y del aspirador no se han probado.
* Solo se entienden los comandos HP-GL `IN`, `SP`, `PU`, `PD`, `PA` y `PR`; cualquier otro comando se cuenta y se ignora, nunca es fatal.
* El lector y el planificador solo se han probado con pequeños ficheros de muestra escritos para las pruebas.
* El programa necesita la carpeta de EH3D-DRILLPRO junto a él, porque depende de ella por ruta.
* Solo se admiten los modelos MTC; los modelos ATC todavía no y los modelos XL no están cubiertos. El chino y el japonés necesitan una fuente CJK entre las del sistema.

**Siguiente**

* Ejecutar un trabajo de fresado en la máquina y confirmar los pasos por milímetro, el husillo y las salidas del aspirador.
* Probar el lector HP-GL con ficheros de trazado de placas reales.
* Leer HP-GL con el lector compartido de [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS), donde está previsto.

## 🌍 Idiomas

El README y los textos existen en inglés, español, francés, italiano, alemán, chino, japonés y portugués de Portugal.

## 🔗 Proyectos relacionados

* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): el hermano de taladrado; este proyecto usa su protocolo, su pantalla y sus idiomas
* [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS): los lectores de G-code y Excellon compartidos de la suite; el HP-GL está previsto allí
* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): el contenedor de proyecto y los tipos de telemetría de máquina de la suite
* [EH3D-NODE-BUNGARD](https://github.com/JuanenRac/EH3D-NODE-BUNGARD): un nodo ESP32-S3 (Ethernet o Wi-Fi) que va en la caja de la máquina, saca los pasos por su propio puerto serie con la temporización que un adaptador USB no puede mantener, lee los LED de la máquina y maneja el variador del husillo; tiene su propio panel web
* [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER): el centro de la familia EH3D: recibe los trabajos de este programa (`connection = server`) y de los demás clientes, y los ejecuta en la máquina a través de su nodo

## 📚 Documentación

* [Notas del protocolo](docs/PROTOCOL.md): lo que se sabe de cómo se maneja la unidad de control (compartido con EH3D-DRILLPRO) y lo que sigue siendo una hipótesis.
* [Conexión y ajuste](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/CONNECTION.md): el fichero de la máquina, las tres conexiones y cada valor
* [Las pantallas clásicas](docs/CLASSIC.md): cómo se reproducen y se comprueban las pantallas del programa de MS-DOS
* [Registro de cambios](CHANGELOG.md)

## 🤝 Contribuir y comunidad

* Antes de una pull request ejecuta `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` y `python -I tools/ci_validate.py`: todo debe pasar. Consulta [CONTRIBUTING.md](CONTRIBUTING.md) y el [código de conducta](CODE_OF_CONDUCT.md).
* El README existe en ocho idiomas: un cambio en el texto se hace en los ocho archivos, y las dos imágenes se dibujan a partir de `docs/architecture.json` y `docs/structure.json` con `tools/make_architecture.py` y `tools/make_structure.py`.
* No describas como funcionando en una máquina lo que solo se ha probado en el simulador.
* Informa de una vulnerabilidad en privado: [SECURITY.md](SECURITY.md). Preguntas, ideas e informes: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTOR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENCIA

Software: GPL-3.0-or-later. Véase [LICENSE](LICENSE). Los diseños de hardware de la suite usan CERN OHL-S.
