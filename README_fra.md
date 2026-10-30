<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-ROUTEPRO banner" width="100%">
</p>

# 🪚 EH3D-ROUTEPRO

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | 🇫🇷 <b>Français</b> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### Une réimplémentation multiplateforme en Rust du flux de fraisage du logiciel DOS classique des machines Bungard CCD, à partir de fichiers de traçage HP-GL, avec une interface en mode texte qui rappelle l'original, en huit langues

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.5-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**Vérification d'honnêteté - ce qui tourne aujourd'hui :** un programme Rust avec une interface texte de 80x25 qui lit les fichiers de tracé HP-GL `.PLT`, la table d'outils et la configuration de la machine, planifie des trajectoires de fraisage à plusieurs passes (déplacement rapide, plongée, coupe, remontée) et les exécute **contre un simulateur** (par défaut) ou, quand le fichier de la machine l'indique, **sur une machine réelle** par le port série de l'ordinateur, un adaptateur USB-série ou l'EH3D-NODE-BUNGARD. 50 tests s'exécutent sur un ordinateur. Le protocole série qu'il partage avec EH3D-DRILLPRO a été essayé sur une Bungard CCD MTC réelle (la référence des trois axes et les déplacements de chaque axe) ; **un travail de fraisage complet n'a pas encore tourné sur la machine**, et les pas par millimètre ne sont vérifiés que par rapport à la configuration de l'original. Par défaut, ce sont les écrans classiques du programme MS-DOS, vérifiés cellule par cellule avec l'original ; `--modern` lance l'interface à menus. Ils se manipulent au clavier et à la souris, la fenêtre peut être agrandie (l'écran s'adapte de 640x400 à 1080p et plus), les fichiers de configuration et d'outils se modifient et s'enregistrent depuis les dialogues, et le bouton Info indique les machines Bungard CCD et CCD/2 (MTC prises en charge, ATC pas encore). Une version console (`routepro-console`) affiche les mêmes écrans dans un terminal, au clavier et à la souris et sans système de fenêtres (SSH, console texte, vieux PC). Les paquets de chaque version contiennent la configuration de base, expliquée ligne par ligne, et un dossier `examples` d'un carré à cent mille points. Le changeur d'outils des modèles ATC n'est pas encore géré.

---

> **État de la sûreté et de la sécurité :** ce programme n'est pas un système de sécurité. Avec une connexion réelle (`connection` dans le fichier de la machine) **Démarrer déplace la machine** : retirez l'outil, gardez une main sur l'interrupteur de la machine et soyez prêt à appuyer sur `Échap`, qui arrête le travail aussitôt et coupe les drivers des moteurs. Les fins de course sont surveillées par l'unité de commande de la machine elle-même (elle arrête un axe à son contact toute seule) ; ce logiciel n'a pas d'arrêt d'urgence propre, et des octets de pas ou des vitesses erronés peuvent entraîner l'outil contre la pièce ou la table ou faire perdre des pas aux moteurs. La couche de protocole refuse de déplacer quoi que ce soit tant qu'elle n'a pas été armée. Le projet est indépendant : il n'est ni fabriqué, ni approuvé, ni soutenu par Bungard Elektronik, et les noms de produits n'apparaissent que pour dire avec quels formats de fichier et quelles machines il doit fonctionner.

## 📋 En un coup d'œil

| | |
|---|---|
| Version | `0.0.5` |
| Maturité | Échafaudage |
| Tests | 50 |
| Pile technique | Rust |
| Sur du matériel réel | Oui, sur une Bungard CCD MTC : la référence et les déplacements de chaque axe ; un travail complet n'a pas encore tourné. Sans fichier de machine, il s'exécute sur le simulateur |

**Aller à :** [Présentation](#-présentation) · [Architecture](#-architecture) · [Démarrage rapide](#-démarrage-rapide) · [Tests](#-tests) · [Structure](#-structure-du-dépôt) · [Liés](#-projets-liés)

## 🚦 État du projet

Chaque ligne est une partie de ce dépôt et le feu dit où elle en est aujourd'hui. Elle est écrite d'après ce que le dépôt peut montrer (tests, compilations, exécutions), pas d'après ce qui est prévu.

🟢 Fait et vérifié · 🟡 Fonctionne, avec des limites connues · 🟠 À moitié fait ou pas encore vérifié · 🔴 Pas fait ou ne fonctionne pas

| Domaine | État | Ce que cela veut dire ici |
|---|---|---|
| Logiciel | 🟡 | Un programme Rust à écran texte 80x25 lit des fichiers de tracé HP-GL, la table d'outils et le fichier de la machine, planifie des parcours de fraisage en plusieurs passes et les exécute sur un simulateur, ou sur une vraie machine si le fichier de la machine le dit ; le dossier d'EH3D-DRILLPRO doit se trouver à côté. |
| Tests | 🟢 | 50 tests réussissent (`cargo test --workspace`) : le lecteur HP-GL, la planification des parcours et les écrans dans les huit langues ; le protocole est couvert par les 212 tests d'EH3D-DRILLPRO. |
| Lecteur HP-GL | 🟡 | Seuls `IN`, `SP`, `PU`, `PD`, `PA` et `PR` sont compris ; toute autre commande est comptée et ignorée, jamais fatale. Essayé seulement avec de petits fichiers d'exemple écrits pour les tests. |
| Planification des trajets | 🟡 | Chaque parcours devient, à chaque passe, un déplacement rapide, une plongée, la coupe et un dégagement, avec la profondeur répartie également et l'outil finissant toujours en surface ; vérifié seulement en tests et sur le simulateur. |
| Matériel réel | 🟠 | Le protocole série qu'il partage avec EH3D-DRILLPRO a été essayé sur une Bungard CCD MTC (la prise de référence et les déplacements de chaque axe) ; aucun travail de fraisage n'a tourné sur une machine, et les sorties de broche et d'aspirateur n'ont pas été essayées. |
| Sécurité d'emploi | 🟡 | `Esc` arrête le travail en une fraction de seconde et coupe les pilotes de moteurs, et l'unité de commande de la machine surveille les fins de course ; le logiciel n'a pas d'arrêt d'urgence propre. |
| Connexion au serveur | 🟠 | Avec `connection = server`, le programme envoie le travail à EH3D-SERVER et suit sa progression ; ce chemin n'a été exercé que sur le simulateur. |
| Paquets prêts à l'emploi | 🟡 | La première publication contient des paquets pour Windows, Linux et macOS (Apple silicon et Intel), chacun avec son SHA-256, construits par le workflow de publication. |
| Documentation | 🟢 | README en huit langues et les notes du protocole avec ce qui reste hypothèse ; le guide du fichier de la machine se trouve dans EH3D-DRILLPRO. |

## 🎯 Présentation

* Les **lecteurs de fichiers** : fichiers de traçage HP-GL `.PLT` (`IN`, `SP`, `PU`, `PD`, `PA`, `PR` ; toute autre commande est comptée et ignorée, jamais fatale), la table d'outils `ROUTEPRO.RTD` (profondeur, passes, avances en XY et Z, vitesse de broche) et la configuration de la machine `ROUTEPRO.RCF`.
* Le **planificateur de trajectoires** : chaque trajectoire devient un déplacement rapide vers son début puis, pour chaque passe, une plongée, la coupe le long de la trajectoire et un retrait ; la profondeur est répartie également entre les passes et l'outil termine toujours à la surface.
* L'**interface** : un écran en mode texte de 80x25 avec les couleurs CGA, des menus, une boîte de dialogue de fichiers et une vue vectorielle des trous, affichés dans une fenêtre. Elle parle anglais, espagnol, italien, allemand, français, portugais, chinois et japonais : choisissez la langue dans le menu ou lancez avec `--lang`.
* Le **protocole de la machine** (`protocol/`, crate `eh3d-routepro-protocol`) : réexporte la bibliothèque de protocole d'EH3D-DRILLPRO (transport d'octets, planification du mouvement, barrière de sécurité) et ajoute le plan de fraisage.
* Le **simulateur** : le travail complet s'exécute jusqu'au bout sans matériel ; les tests l'utilisent.
* Les **machines** : celles dont l'unité de commande passe par le port série et utilisait le logiciel DOS 4.6 : Bungard CCD MTC et CCD/2 MTC (changement d'outil manuel ; la CCD/2 a la table plus petite de 325 x 270 mm). La taille de la table et les autres constantes viennent du fichier de configuration de la machine. Seule une CCD MTC a été essayée, et les sorties de la broche et de la commande de l'aspirateur n'ont pas encore été essayées. Les deux modèles ATC (CCD ATC et CCD/2 ATC, changement automatique) auront des applications séparées, construites avec ce que permet la rétro-ingénierie. Les modèles XL et les machines postérieures ont leur propre unité de commande et logiciel modernes et ne sont pas couverts.
* Le **fichier de la machine** (`machine.cfg`, partagé par DRILLPRO et ROUTEPRO) : comment le travail atteint la machine (`demo` simulateur, `rs232` le port série de l'ordinateur lui-même, `usb` un adaptateur USB-série ou `node` l'EH3D-NODE-BUNGARD par le réseau) et, une valeur par ligne, tout ce qui a été réglé sur une machine réelle : vitesses et rampes de recherche, le saut d'étalonnage, les limites des moteurs.
* La **référence** : Z monte jusqu'à sa fin de course, puis X et Y, puis un saut d'étalonnage doux. L'unité de commande signale « les trois axes en origine » sur la ligne RI, que le programme surveille ; quand la machine est garée, il saute les recherches, éloigne X et Y de 10 mm de leurs contacts et les cherche de nouveau, ainsi il les voit arriver en direct.
* **Mouvement régulier** : les déplacements partent en un seul flux continu dont les positions d'octet portent le temps (sinon un adaptateur USB livre les octets par rafales), Z se déplace octet par octet, et `Échap` annule un travail en une fraction de seconde et coupe les drivers des moteurs.
* La **broche** : par défaut celle de l'unité de commande elle-même (ses bits de verrou, jusqu'à 60000 tr/min) ; avec `spindle = node` dans le fichier de la machine, le travail la demande à l'EH3D-NODE-BUNGARD (un variateur de fréquence sur les sorties du nœud, toute vitesse jusqu'au maximum du nœud, arrêtée à tout arrêt). Avec un nœud, les programmes lisent aussi les voyants de la machine, si bien que chaque axe qui arrive en origine est vu en direct sans lire la ligne d'origine au milieu du flux.
* La **connexion au serveur** (`connection = server` dans le fichier de la machine, avec `server_address`, `server_token` et `server_machine`) : au lieu de piloter lui-même la machine, le programme envoie le travail à l'[EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER), qui le planifie avec les constantes de la machine, l'exécute par le nœud de la machine et dit au programme comment il avance ; le programme montre la progression et répond aux dialogues comme d'habitude, `Échap` arrête le travail sur le serveur, et fermer le programme ne l'arrête pas.
* Les **archives de travail, la fenêtre Offset et la simulation** : un travail peut être empaqueté dans une archive ZIP (la fenêtre Select File y entre, et `routepro JOB.ZIP` charge son premier fichier de fraisage) ; la flèche à côté du nom du fichier liste les motifs `*.PLT`, `*.ZIP` et `*.*` ; les boutons Load des fenêtres d'outils et de configuration choisissent le fichier à charger ; sur une vraie machine la fenêtre Offset fait la prise d'origine, suit chaque mouvement de la fenêtre et refait la prise d'origine ; et le travail simulé dure le temps que prendrait la machine.
* Les **sons** : `sound_finish` et `sound_tool_change` dans le fichier de la machine valent chacun off (par défaut), `default` (une tonalité du programme) ou le chemin d'un fichier WAV personnel ; le premier joue quand un travail se termine seul, le second quand la machine demande un outil.
* Les **dessins d'une carte** : outre le HP-GL, un dessin `.DXF` (lignes, polylignes avec arcs, arcs et cercles) ou la couche de contour d'une carte Gerber (`.GKO`, `.GM1`, `.GML`, ou un `.GBR` qui se déclare profil) se charge comme trajet de fraisage ; les arcs, cercles et mises à l'échelle du HP-GL (`AA`, `AR`, `CI`, `SC`, `IP`) sont découpés en cordes selon l'angle de corde de la configuration.

## 🧩 Rôle dans EH3D

EH3D-ROUTEPRO est le frère de fraisage d'EH3D-DRILLPRO et en dépend par chemin : le dossier `EH3D-DRILLPRO` doit se trouver à côté de celui-ci. Il réutilise le protocole série, l'écran et la prise en charge des langues. Il ne dépend encore d'aucun autre dépôt EH3D. Les fichiers de la machine (celui de perçage, le HP-GL, les tables d'outils et la configuration de la machine) viennent du crate `eh3d-bungard-formats` d'EH3D-COMMON, qui doit aussi se trouver à côté.

## 🏛️ Architecture

<p align="center">
  <img src="images/architecture.svg" alt="Architecture de EH3D-ROUTEPRO : qui l'utilise, ce qu'il contient et de quoi il dépend" width="100%">
</p>

Comment les parties de ce dépôt s'articulent : qui l'utilise, ce qu'il contient et de quoi il dépend. L'image est dessinée à partir de `docs/architecture.json` et vérifiée par rapport au code.

## 🖼️ Captures d'écran

<p align="center">
  <img src="images/screenshots/main.png" alt="main screen" width="80%">
</p>

<p align="center"><em>L'écran principal du programme classique de fraisage : l'échelle, le décalage et la coupe, les vecteurs du tracé et les boutons.</em></p>

<p align="center">
  <img src="images/screenshots/tools.png" alt="tool data" width="80%">
</p>

<p align="center"><em>Les données d'outils : la profondeur, les passes, les avances le long du tracé et de plongée, et la vitesse de la broche de chaque outil.</em></p>

## 🚀 Démarrage rapide

Il faut une chaîne d'outils Rust stable.

```bash
cargo build --release --features gui
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

Lancez-le avec `run.bat` (Windows) ou `./run.sh` (Linux et macOS) ; les deux compilent d'abord le programme si nécessaire. Les deux ont besoin de Rust et du dossier frère `EH3D-DRILLPRO` : `run.bat [FICHIER.PLT] [--lang en|es|it|de|fr|pt|zh|ja] [--data DIR] [--dump]`. `--data` est le dossier contenant `ROUTEPRO.RTD` et `ROUTEPRO.RCF` (par défaut : `testdata/`) et `--dump` affiche l'écran en texte brut. Touches : `Alt` plus la lettre en surbrillance ouvre un menu, les flèches déplacent, `Entrée` valide, `Échap` revient ; la barre de menus est aussi cliquable. Le chinois et le japonais nécessitent une police CJK parmi les polices du système. `--connection FILE` désigne un autre fichier de machine que celui par défaut (`%APPDATA%\EH3D\machine.cfg` sous Windows, `~/.config/eh3d/machine.cfg` ailleurs) ; sans fichier de machine, le programme s'exécute sur le simulateur. `--modern` lance l'interface à menus au lieu des écrans classiques.

## 🧪 Tests

La suite compte 50 tests (`cargo test --workspace`). Ils vérifient le lecteur HP-GL, la planification des trajets (passes, plongée, coupe et relevage), le plan de fraisage de la crate du protocole, les écrans classiques cellule par cellule et les écrans dans les huit langues. Le protocole réutilisé (octets, mouvement, barrière de sécurité) est testé dans EH3D-DRILLPRO. La CI exécute aussi `cargo fmt --check` et `cargo clippy` avec les avertissements comme erreurs, et `python -I tools/ci_validate.py` contrôle le manifeste, les versions, le journal des modifications et les huit README.

## 🏗️ Compilation et publication

La version se trouve dans `Cargo.toml` et dans `eh3d.project.json`, et chaque version a son entrée dans [CHANGELOG.md](CHANGELOG.md). Les versions sont publiées sur GitHub.

**Paquets prêts à l'emploi :** chaque version publiée contient un paquet pour Windows (zip), Linux (tar.gz) et macOS (Apple silicon et Intel, tar.gz) avec le programme, la licence et ce README, chacun avec son SHA-256 à côté. Ils sont construits par le workflow `Release binaries` de ce dépôt. Téléchargez-en un depuis la page Releases ; sous Linux, la fenêtre a besoin de X11 ou de Wayland.

## 📂 Structure du dépôt

<p align="center">
  <img src="images/repository-structure.svg" alt="Structure du dépôt EH3D-ROUTEPRO" width="100%">
</p>

<details>
<summary>Version texte de l'arborescence</summary>

```text
EH3D-ROUTEPRO/
├── src/
│   ├── job.rs                                   les trajets HP-GL transformés en travail de fraisage
│   ├── app.rs texts.rs                          l'application et ses textes en huit langues
│   └── bin/                                     les exécutables routepro et routepro-console
├── protocol/src/
│   ├── route.rs                                 passes, plongée, coupe et relevage pour chaque trajet
│   └── lib.rs                                   réexporte le protocole d'EH3D-DRILLPRO
├── tests/                                       lecture HP-GL, planification des trajets, écrans classiques et écrans dans toutes les langues
├── docs/                                        les notes de protocole, les écrans classiques
├── testdata/                                    petits fichiers d'exemple écrits pour les tests
├── config/                                      la configuration de base, expliquée ligne par ligne
├── examples/                                    exemples, d'un carré à 100000 points
├── tools/                                       contrôle de CI, du README et outils d'images
├── images/                                      bannière, les deux images et les captures d'écran
├── .github/                                     workflows de CI et de publication, modèles d'issues et de pull requests
├── build.sh build.bat                           scripts de compilation et de test
├── run.sh run.bat                               lancent le programme
├── eh3d.project.json                            manifeste : version, rôle, maturité et commande de compilation
├── Cargo.toml Cargo.lock                        le paquet Rust et ses dépendances figées
├── CHANGELOG.md                                 ce qui a changé dans chaque version
├── SECURITY.md SUPPORT.md                       signaler une vulnérabilité et poser des questions
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   contribuer, code de conduite et licence
```

</details>

## 🗺️ Feuille de route et limites connues

**Limites connues aujourd'hui**

* Aucun travail de fraisage n'a encore tourné sur une machine réelle : le protocole série qu'il partage avec EH3D-DRILLPRO a été essayé sur une Bungard CCD MTC (la prise de référence des trois axes et les mouvements de chaque axe), les pas par millimètre ne sont vérifiés que par rapport à la configuration de l'original, et les sorties de la broche et de l'aspirateur n'ont pas été essayées.
* Seules les commandes HP-GL `IN`, `SP`, `PU`, `PD`, `PA` et `PR` sont comprises ; toute autre commande est comptée et ignorée, jamais fatale.
* Le lecteur et le planificateur n'ont été essayés qu'avec de petits fichiers d'exemple écrits pour les tests.
* Le programme a besoin du dossier d'EH3D-DRILLPRO à côté de lui, car il en dépend par chemin.
* Seuls les modèles MTC sont pris en charge ; les modèles ATC pas encore et les modèles XL ne sont pas couverts. Le chinois et le japonais demandent une police CJK parmi les polices du système.

**Ensuite**

* Exécuter un travail de fraisage sur la machine et confirmer les pas par millimètre, la broche et les sorties de l'aspirateur.
* Essayer le lecteur HP-GL avec des fichiers de tracé de vraies cartes.
* Lire le HP-GL avec le lecteur partagé d'[EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS), où il est prévu.

## 🌍 Langues

Le README et les textes existent en anglais, espagnol, français, italien, allemand, chinois, japonais et portugais européen.

## 🔗 Projets liés

* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): le frère pour le perçage ; ce projet utilise son protocole, son écran et ses langues
* [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS): les lecteurs G-code et Excellon partagés de la suite ; le HP-GL y est prévu
* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): le conteneur de projet et les types de télémétrie de machine de la suite
* [EH3D-NODE-BUNGARD](https://github.com/JuanenRac/EH3D-NODE-BUNGARD) : un nœud ESP32-S3 (Ethernet ou Wi-Fi) qui se place dans le coffret de la machine, envoie les pas par son propre port série avec le cadencement qu'un adaptateur USB ne sait pas tenir, lit les voyants de la machine et pilote le variateur de la broche ; il a son propre panneau web
* [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER) : le centre de la famille EH3D : il reçoit les travaux de ce programme (`connection = server`) et des autres clients, et les exécute sur la machine par son nœud

## 📚 Documentation

* [Notes sur le protocole](docs/PROTOCOL.md): ce que l'on sait de la façon dont l'unité de commande est pilotée (partagé avec EH3D-DRILLPRO) et ce qui reste une hypothèse.
* [Connexion et réglage](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/CONNECTION.md) : le fichier de la machine, les trois connexions et chaque valeur
* [Les écrans classiques](docs/CLASSIC.md) : comment les écrans du programme MS-DOS sont reproduits et vérifiés
* [Journal des modifications](CHANGELOG.md)

## 🤝 Contribuer et communauté

* Avant une pull request, exécutez `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` et `python -I tools/ci_validate.py` : tout doit passer. Voir [CONTRIBUTING.md](CONTRIBUTING.md) et le [code de conduite](CODE_OF_CONDUCT.md).
* Le README existe en huit langues : une modification du texte se fait dans les huit fichiers, et les deux images sont dessinées à partir de `docs/architecture.json` et `docs/structure.json` par `tools/make_architecture.py` et `tools/make_structure.py`.
* Ne décrivez pas comme fonctionnant sur une machine ce qui n'a été essayé que sur le simulateur.
* Signalez une vulnérabilité en privé : [SECURITY.md](SECURITY.md). Questions, idées et rapports : electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTEUR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENCE

Logiciel : GPL-3.0-or-later. Voir [LICENSE](LICENSE). Les conceptions matérielles de la suite utilisent la CERN OHL-S.
