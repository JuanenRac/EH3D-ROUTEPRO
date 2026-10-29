<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-ROUTEPRO banner" width="100%">
</p>

# 🪚 EH3D-ROUTEPRO

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | 🇵🇹 <b>Português</b></p>

### Uma reimplementação multiplataforma em Rust do fluxo de fresagem do clássico software DOS das máquinas Bungard CCD, a partir de ficheiros de traçado HP-GL, com uma interface em modo de texto semelhante à original, em oito idiomas

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.2-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**Verificação de honestidade - o que funciona hoje:** um programa em Rust com interface de texto de 80x25 que lê ficheiros de traçado HP-GL `.PLT`, a tabela de ferramentas e a configuração da máquina, planeia trajetórias de fresagem de várias passagens (avanço rápido, penetração, corte, elevação) e executa-as **contra um simulador** (por omissão) ou, quando o ficheiro da máquina o indica, **numa máquina real** pela porta série do computador, um adaptador USB-série ou o EH3D-NODE-BUNGARD. 21 testes correm num computador. O protocolo série que partilha com o EH3D-DRILLPRO foi experimentado numa Bungard CCD MTC real (a referência dos três eixos e os movimentos de cada eixo); **um trabalho de fresagem completo ainda não correu na máquina**, e os passos por milímetro só foram confrontados com a configuração do original. Por predefinição, aparecem os ecrãs clássicos do programa MS-DOS, verificados célula a célula com o original; `--modern` inicia a interface de menus. Funcionam com teclado e rato, a janela pode ser maximizada (o ecrã ajusta-se de 640x400 a 1080p e mais), os ficheiros de configuração e de ferramentas editam-se e gravam-se nos diálogos, e o botão Sobre indica as máquinas Bungard CCD e CCD/2 (MTC suportadas, ATC ainda não). Uma versão de consola (`routepro-console`) desenha os mesmos ecrãs num terminal, com teclado e rato e sem sistema de janelas (SSH, consola de texto, um PC antigo).

---

> **Estado de segurança (safety) e de segurança informática:** este programa não é um sistema de segurança. Com uma ligação real (`connection` no ficheiro da máquina) **Iniciar move a máquina**: retire a ferramenta, mantenha uma mão no interruptor da máquina e tenha pronta a tecla `Esc`, que pára o trabalho de imediato e desliga os drivers dos motores. Os fins de curso são vigiados pela própria unidade de controlo da máquina (pára o eixo no seu fim de curso por si só); este software não tem paragem de emergência própria, e bytes de passo ou velocidades errados podem levar a ferramenta contra a peça ou a mesa ou fazer os motores perderem passos. A camada de protocolo recusa-se a mover o que quer que seja até ser armada. O projeto é independente: não é fabricado, apoiado nem suportado pela Bungard Elektronik, e os nomes de produtos aparecem apenas para dizer com que formatos de ficheiro e máquinas deve funcionar.

## 📋 Num relance

| | |
|---|---|
| Versão | `0.0.2` |
| Maturidade | Estrutura inicial |
| Testes | 21 |
| Tecnologias | Rust |
| Em hardware real | Sim, numa Bungard CCD MTC: a referência e os movimentos de cada eixo; um trabalho completo ainda não correu. Sem ficheiro da máquina corre no simulador |

## 🚦 Estado do projeto

Cada linha é uma parte deste repositório e a luz diz até onde chega hoje. É escrita com o que o repositório consegue demonstrar (testes, compilações, execuções), não com o que está previsto.

🟢 Feito e verificado · 🟡 Funciona, com limites conhecidos · 🟠 Feito a meio ou ainda por verificar · 🔴 Por fazer ou a não funcionar

| Área | Estado | O que significa aqui |
|---|---|---|
| Software | 🟡 | Um programa Rust com ecrã de texto de 80x25 lê ficheiros de traçado HP-GL, a tabela de ferramentas e o ficheiro da máquina, planeia trajetos de fresagem de várias passagens e executa-os num simulador, ou numa máquina real se o ficheiro da máquina o indicar; precisa da pasta do EH3D-DRILLPRO ao lado. |
| Testes | 🟢 | Passam 21 testes (`cargo test --workspace`): o leitor de HP-GL, o planeamento de trajetos e os ecrãs nos oito idiomas; o protocolo é coberto pelos 123 testes do EH3D-DRILLPRO. |
| Leitor de HP-GL | 🟡 | Só se entendem `IN`, `SP`, `PU`, `PD`, `PA` e `PR`; qualquer outro comando é contado e ignorado, nunca é fatal. Experimentado só com pequenos ficheiros de exemplo escritos para os testes. |
| Planeamento de trajetos | 🟡 | Cada trajeto torna-se, em cada passagem, um movimento rápido, uma penetração, o corte e uma elevação, com a profundidade repartida por igual e a ferramenta a terminar sempre à superfície; verificado só em testes e no simulador. |
| Hardware real | 🟠 | O protocolo série que partilha com o EH3D-DRILLPRO foi experimentado numa Bungard CCD MTC (o movimento de referência e os movimentos de cada eixo); nenhum trabalho de fresagem correu numa máquina, e as saídas do fuso e do aspirador não foram experimentadas. |
| Segurança de utilização | 🟡 | `Esc` pára o trabalho numa fração de segundo e desliga os drivers dos motores, e a própria unidade de controlo da máquina vigia os fins de curso; o software não tem paragem de emergência própria. |
| Ligação ao servidor | 🟠 | Com `connection = server` o programa envia o trabalho ao EH3D-SERVER e acompanha o progresso; este caminho só foi exercitado contra o simulador. |
| Pacotes prontos a usar | 🟡 | A primeira versão publicada traz pacotes para Windows, Linux e macOS (Apple silicon e Intel), cada um com o seu SHA-256, construídos pelo fluxo de publicação. |
| Documentação | 🟢 | README em oito idiomas e as notas do protocolo com o que ainda é hipótese; o guia do ficheiro da máquina está no EH3D-DRILLPRO. |

## 🎯 Descrição geral

* Os **leitores de ficheiros**: ficheiros de traçado HP-GL `.PLT` (`IN`, `SP`, `PU`, `PD`, `PA`, `PR`; qualquer outro comando é contado e ignorado, nunca é fatal), a tabela de ferramentas `ROUTEPRO.RTD` (profundidade, passagens, avanços em XY e Z, velocidade do fuso) e a configuração da máquina `ROUTEPRO.RCF`.
* O **planeador de trajetos**: cada trajeto torna-se um movimento rápido até ao início e, em cada passagem, uma penetração, o corte ao longo do trajeto e uma elevação; a profundidade reparte-se por igual pelas passagens e a ferramenta termina sempre à superfície.
* A **interface**: um ecrã em modo de texto de 80x25 com cores CGA, menus, uma caixa de diálogo de ficheiros e uma vista vetorial dos furos, mostrada numa janela. Fala inglês, espanhol, italiano, alemão, francês, português, chinês e japonês: escolha o idioma no menu ou arranque com `--lang`.
* O **protocolo da máquina** (`protocol/`, crate `eh3d-routepro-protocol`): reexporta a biblioteca de protocolo do EH3D-DRILLPRO (transporte de bytes, planeamento de movimento, barreira de segurança) e acrescenta o plano de fresagem.
* O **simulador**: o trabalho completo corre até ao fim sem hardware; os testes usam-no.
* As **máquinas**: as que têm uma unidade de controlo por porta série que usava o software MS-DOS 4.6: Bungard CCD MTC e CCD/2 MTC (troca de ferramenta manual; a CCD/2 tem a mesa mais pequena de 325 x 270 mm). O tamanho da mesa e as restantes constantes vêm do ficheiro de configuração da máquina. Só foi experimentado numa CCD MTC, e as saídas do eixo-árvore (broca) e da ativação do aspirador ainda não foram experimentadas. Os dois modelos ATC (CCD ATC e CCD/2 ATC, troca automática) terão aplicações separadas, feitas com o que a engenharia inversa permitir. Os modelos XL e as máquinas posteriores têm unidade de controlo e software modernos próprios e não estão abrangidos.
* O **ficheiro da máquina** (`machine.cfg`, partilhado pelo DRILLPRO e pelo ROUTEPRO): como o trabalho chega à máquina (`demo` simulador, `rs232` a porta série do próprio computador, `usb` um adaptador USB-série ou `node` o EH3D-NODE-BUNGARD pela rede) e, um valor por linha, tudo o que foi afinado numa máquina real: velocidades e rampas de procura, o salto de calibração, os limites dos motores.
* A **referência**: Z sobe até ao fim de curso, depois X e Y, e por fim um salto de calibração suave. A unidade de controlo indica «os três eixos em origem» pela linha RI, que o programa vigia; com a máquina estacionada, salta as procuras, afasta X e Y 10 mm dos fins de curso e procura-os de novo, assim vê-los chegar em direto.
* **Movimento estável**: os movimentos saem como um único fluxo contínuo cujas posições de byte levam o tempo (senão um adaptador USB entrega os bytes em rajadas), Z move-se byte a byte, e `Esc` cancela um trabalho numa fração de segundo e desliga os drivers dos motores.
* O **eixo-árvore**: por omissão o da própria unidade de controlo (os seus bits de latch, até 60000 rpm); com `spindle = node` no ficheiro da máquina o trabalho pede-o ao EH3D-NODE-BUNGARD (um variador de frequência nas saídas do nó, qualquer velocidade até ao máximo do nó, parado em qualquer paragem). Com um nó, os programas leem também os LED da máquina, de modo que cada eixo que chega a casa é visto em direto sem ler a linha de home a meio do fluxo.
* A **ligação ao servidor** (`connection = server` no ficheiro da máquina, com `server_address`, `server_token` e `server_machine`): em vez de conduzir ele próprio a máquina, o programa envia o trabalho ao [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER), que o planeia com as constantes da máquina, o executa através do nó da máquina e diz ao programa como vai; o programa mostra o progresso e responde aos diálogos como sempre, `Esc` pára o trabalho no servidor, e fechar o programa não o pára.

## 🧩 Papel no EH3D

O EH3D-ROUTEPRO é o irmão de fresagem do EH3D-DRILLPRO e depende dele por caminho: a pasta `EH3D-DRILLPRO` tem de estar ao lado desta. Reutiliza o protocolo série, o ecrã e o suporte de idiomas. Ainda não depende de nenhum outro repositório EH3D.

## 📂 Estrutura do repositório

```text
EH3D-ROUTEPRO/
├── src/          library: HP-GL reader, tool table, path planning, application, texts
├── src/bin/      the routepro executable (feature gui)
├── protocol/     eh3d-routepro-protocol: the routing plan on top of the DRILLPRO protocol
├── tests/        HP-GL parsing, path planning, screens in all languages
├── testdata/     small sample files written for the tests
├── docs/         PROTOCOL.md e CONNECTION.md: o protocolo, o ficheiro da máquina e cada valor de afinação; machine.cfg.example: o ficheiro com todos os seus valores
├── tools/        ci_validate.py: manifest, versions, changelog and README checks
├── images/       banner
└── eh3d.project.json   the manifest of the project
```

## 🛠️ COMPILAR E EXECUTAR

É preciso uma cadeia de ferramentas Rust estável.

```bash
cargo build --release --features gui
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

Execute-o com `run.bat` (Windows) ou `./run.sh` (Linux e macOS); ambos compilam primeiro o programa, se for preciso. Os dois precisam de Rust e da pasta irmã `EH3D-DRILLPRO`: `run.bat [FICHEIRO.PLT] [--lang en|es|it|de|fr|pt|zh|ja] [--data DIR] [--dump]`. `--data` é a pasta com `ROUTEPRO.RTD` e `ROUTEPRO.RCF` (por omissão: `testdata/`) e `--dump` imprime o ecrã como texto simples. Teclas: `Alt` mais a letra realçada abre um menu, as setas movem, `Enter` aceita, `Esc` volta atrás; a barra de menus também se pode clicar. O chinês e o japonês precisam de um tipo de letra CJK entre os do sistema. `--connection FILE` indica outro ficheiro de máquina que não o predefinido (`%APPDATA%\EH3D\machine.cfg` no Windows, `~/.config/eh3d/machine.cfg` nos restantes); sem ficheiro de máquina o programa corre no simulador. `--modern` inicia a interface de menus em vez dos ecrãs clássicos.

**Pacotes já compilados:** cada versão publicada inclui um pacote para Windows (zip), Linux (tar.gz) e macOS (Apple silicon e Intel, tar.gz) com o programa, a licença e este README, cada um com o seu SHA-256 ao lado. São construídos pelo workflow `Release binaries` deste repositório. Descarregue um a partir da página de Releases; no Linux, a janela precisa de X11 ou Wayland.

## 🔗 Projetos relacionados

* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): o irmão de furação; este projeto usa o seu protocolo, o seu ecrã e os seus idiomas
* [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS): os leitores de G-code e Excellon partilhados da suite; o HP-GL está previsto lá
* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): o contentor de projeto e os tipos de telemetria de máquina da suite
* [EH3D-NODE-BUNGARD](https://github.com/JuanenRac/EH3D-NODE-BUNGARD): um nó ESP32-S3 (Ethernet ou Wi-Fi) que fica na caixa da máquina, envia os passos pela sua própria porta série com a temporização que um adaptador USB não consegue manter, lê os LED da máquina e comanda o variador do eixo-árvore; tem o seu próprio painel web
* [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER): o centro da família EH3D: recebe os trabalhos deste programa (`connection = server`) e dos outros clientes, e executa-os na máquina através do seu nó

## 📚 Documentação e comunidade

* [Notas do protocolo](docs/PROTOCOL.md): o que se sabe sobre como a unidade de controlo é comandada (partilhado com o EH3D-DRILLPRO) e o que continua a ser uma hipótese.
* [Ligação e afinação](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/CONNECTION.md): o ficheiro da máquina, as três ligações e cada valor
* [Registo de alterações](CHANGELOG.md)
* [Como contribuir](CONTRIBUTING.md)
* [Política de segurança](SECURITY.md)
* [Suporte](SUPPORT.md)

## 👤 AUTOR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENÇA

Software: GPL-3.0-or-later. Ver [LICENSE](LICENSE). Os projetos de hardware da suite usam a CERN OHL-S.