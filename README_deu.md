<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-ROUTEPRO banner" width="100%">
</p>

# 🪚 EH3D-ROUTEPRO

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | 🇩🇪 <b>Deutsch</b> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### Eine plattformübergreifende Neuimplementierung des Fräs-Ablaufs der klassischen DOS-Software für Bungard-CCD-Maschinen in Rust, aus HP-GL-Plotdateien, mit einer an das Original angelehnten Textmodus-Oberfläche in acht Sprachen

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.2-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
</p>

---

**Ehrlichkeitscheck - was heute tatsächlich läuft:** ein Rust-Programm mit einer 80x25-Textoberfläche, das HP-GL-Plotdateien `.PLT`, die Werkzeugtabelle und die Maschinenkonfiguration liest, mehrfach durchlaufende Fräsbahnen plant (Eilgang, Eintauchen, Schnitt, Abheben) und entweder **gegen einen Simulator** (Standard) oder, wenn die Maschinendatei es vorgibt, **an einer echten Maschine** über die serielle Schnittstelle des Rechners, einen USB-Seriell-Adapter oder den EH3D-NODE-BUNGARD ausführt. 42 Tests laufen auf einem Rechner. Das serielle Protokoll, das es mit EH3D-DRILLPRO teilt, wurde an einer echten Bungard CCD MTC erprobt (die Referenzfahrt der drei Achsen und die Bewegungen jeder Achse); **ein vollständiger Fräsauftrag lief noch nicht an der Maschine**, und die Schritte pro Millimeter sind nur gegen die Konfiguration des Originals geprüft. Standardmäßig erscheinen die klassischen Bildschirme des MS-DOS-Programms, Zelle für Zelle mit dem Original verglichen; `--modern` startet stattdessen die menügesteuerte Oberfläche. Sie lassen sich mit Tastatur und Maus bedienen, das Fenster kann maximiert werden (der Bildschirm skaliert von 640x400 bis 1080p und mehr), die Konfigurations- und Werkzeugdateien werden in den Dialogen bearbeitet und gespeichert, und die Schaltfläche Info nennt die Bungard-Maschinen CCD und CCD/2 (MTC unterstützt, ATC noch nicht). Eine Konsolenversion (`routepro-console`) zeigt dieselben Bildschirme im Terminal, mit Tastatur und Maus und ohne Fenstersystem (SSH, Textkonsole, alter PC).

---

> **Stand von Sicherheit und IT-Sicherheit:** dieses Programm ist kein Sicherheitssystem. Bei einer echten Verbindung (`connection` in der Maschinendatei) **bewegt Start die Maschine**: Werkzeug herausnehmen, eine Hand am Netzschalter der Maschine halten und `Esc` bereithalten, das den Auftrag sofort stoppt und die Schrittmotortreiber abschaltet. Die Endschalter überwacht die Steuereinheit der Maschine selbst (sie stoppt eine Achse am Schalter von allein); diese Software hat keinen eigenen Not-Aus, und falsche Schrittbytes oder Geschwindigkeiten können das Werkzeug in das Werkstück oder den Tisch fahren oder die Motoren Schritte verlieren lassen. Die Protokollschicht weigert sich, etwas zu bewegen, bevor sie scharf geschaltet wurde. Das Projekt ist unabhängig: Es wird nicht von Bungard Elektronik hergestellt, empfohlen oder unterstützt, und Produktnamen erscheinen nur, um zu sagen, mit welchen Dateiformaten und Maschinen es arbeiten soll.

## 📋 Auf einen Blick

| | |
|---|---|
| Version | `0.0.2` |
| Reifegrad | Gerüst |
| Tests | 42 |
| Technologie-Stack | Rust |
| Auf echter Hardware | Ja, an einer Bungard CCD MTC: die Referenzfahrt und die Bewegungen jeder Achse; ein ganzer Auftrag lief noch nicht. Ohne Maschinendatei läuft es im Simulator |

**Springe zu:** [Überblick](#-überblick) · [Architektur](#-architektur) · [Schnellstart](#-schnellstart) · [Tests](#-tests) · [Struktur](#-struktur-des-repositorys) · [Verwandte](#-verwandte-projekte)

## 🚦 Projektstatus

Jede Zeile ist ein Teil dieses Repositorys, und die Ampel sagt, wie weit er heute ist. Sie beruht auf dem, was das Repository zeigen kann (Tests, Builds, Läufe), nicht auf dem, was geplant ist.

🟢 Fertig und überprüft · 🟡 Funktioniert, mit bekannten Grenzen · 🟠 Teilweise fertig oder noch nicht überprüft · 🔴 Nicht gemacht oder funktioniert nicht

| Bereich | Stand | Was das hier bedeutet |
|---|---|---|
| Software | 🟡 | Ein Rust-Programm mit 80x25-Textbildschirm liest HP-GL-Plotdateien, die Werkzeugtabelle und die Maschinendatei, plant mehrstufige Fräsbahnen und führt sie gegen einen Simulator aus, oder auf einer echten Maschine, wenn die Maschinendatei es sagt; der Ordner von EH3D-DRILLPRO muss daneben liegen. |
| Tests | 🟢 | 42 Tests bestehen (`cargo test --workspace`): der HP-GL-Leser, die Bahnplanung und die Bildschirme in allen acht Sprachen; das Protokoll decken die 182 Tests von EH3D-DRILLPRO ab. |
| HP-GL-Leser | 🟡 | Nur `IN`, `SP`, `PU`, `PD`, `PA` und `PR` werden verstanden; jeder andere Befehl wird gezählt und ignoriert, nie fatal. Nur mit kleinen, für die Tests geschriebenen Beispieldateien erprobt. |
| Bahnplanung | 🟡 | Jede Bahn wird für jeden Durchgang zu Eilgang, Eintauchen, Schnitt und Abheben, mit gleichmäßig verteilter Tiefe, und das Werkzeug endet immer an der Oberfläche; nur in Tests und am Simulator geprüft. |
| Echte Hardware | 🟠 | Das serielle Protokoll, das es mit EH3D-DRILLPRO teilt, wurde an einer Bungard CCD MTC erprobt (Referenzlauf und Bewegungen jeder Achse); kein Fräsauftrag lief auf einer Maschine, und die Ausgänge für Spindel und Sauger sind unerprobt. |
| Betriebssicherheit | 🟡 | `Esc` stoppt den Auftrag in einem Bruchteil einer Sekunde und schaltet die Schrittmotortreiber ab, und die Steuereinheit der Maschine überwacht die Endschalter; die Software hat keinen eigenen Not-Halt. |
| Serververbindung | 🟠 | Mit `connection = server` schickt das Programm den Auftrag an EH3D-SERVER und verfolgt dessen Fortschritt; dieser Weg wurde nur gegen den Simulator geübt. |
| Fertige Pakete | 🟡 | Die erste Veröffentlichung enthält Pakete für Windows, Linux und macOS (Apple Silicon und Intel), jeweils mit SHA-256, gebaut vom Release-Workflow. |
| Dokumentation | 🟢 | README in acht Sprachen und die Protokollnotizen mit dem, was noch Hypothese ist; die Anleitung zur Maschinendatei liegt in EH3D-DRILLPRO. |

## 🎯 Überblick

* Die **Dateileser**: HP-GL-Plotdateien `.PLT` (`IN`, `SP`, `PU`, `PD`, `PA`, `PR`; jeder andere Befehl wird gezählt und ignoriert, nie ein Fehler), die Werkzeugtabelle `ROUTEPRO.RTD` (Tiefe, Durchgänge, Vorschübe in XY und Z, Spindeldrehzahl) und die Maschinenkonfiguration `ROUTEPRO.RCF`.
* Der **Bahnplaner**: Jede Bahn wird zu einer Eilbewegung zu ihrem Anfang und dann für jeden Durchgang zu einem Eintauchen, dem Schnitt entlang der Bahn und einem Abheben; die Tiefe wird gleichmäßig auf die Durchgänge verteilt und das Werkzeug endet immer an der Oberfläche.
* Die **Oberfläche**: ein 80x25-Textbildschirm mit CGA-Farben, Menüs, einem Dateidialog und einer Vektoransicht der Bohrungen in einem Fenster. Sie spricht Englisch, Spanisch, Italienisch, Deutsch, Französisch, Portugiesisch, Chinesisch und Japanisch: Die Sprache wählen Sie im Menü oder mit `--lang` beim Start.
* Das **Maschinenprotokoll** (`protocol/`, Crate `eh3d-routepro-protocol`): exportiert die Protokollbibliothek von EH3D-DRILLPRO erneut (Byte-Transport, Bewegungsplanung, Sicherheitssperre) und ergänzt den Fräsplan.
* Der **Simulator**: Der ganze Auftrag läuft ohne Hardware bis zum Ende; die Tests verwenden ihn.
* Die **Maschinen**: die mit einer Steuereinheit an der seriellen Schnittstelle, die die DOS-Software 4.6 nutzte: Bungard CCD MTC und CCD/2 MTC (manueller Werkzeugwechsel; die CCD/2 hat den kleineren Tisch von 325 x 270 mm). Tischgröße und die übrigen Maschinenkonstanten stammen aus der Konfigurationsdatei der Maschine. Erprobt wurde nur eine CCD MTC, und die Ausgänge für Bohrspindel und Staubsaugerschaltung sind noch nicht erprobt. Die beiden ATC-Modelle (CCD ATC und CCD/2 ATC, automatischer Werkzeugwechsel) bekommen eigene Anwendungen, gebaut mit dem, was die Rückwärtsanalyse hergibt. Die XL-Modelle und die späteren Maschinen haben eine eigene moderne Steuereinheit und Software und sind nicht abgedeckt.
* Die **Maschinendatei** (`machine.cfg`, von DRILLPRO und ROUTEPRO gemeinsam genutzt): wie der Auftrag zur Maschine gelangt (`demo` Simulator, `rs232` die serielle Schnittstelle des Rechners selbst, `usb` ein USB-Seriell-Adapter oder `node` der EH3D-NODE-BUNGARD über das Netzwerk) und, ein Wert pro Zeile, alles, was an einer echten Maschine abgestimmt wurde: Suchgeschwindigkeiten und Rampen, der Kalibriersprung, die Grenzen der Motoren.
* Die **Referenzfahrt**: Z fährt bis zu seinem Endschalter, dann X und Y, danach ein sanfter Kalibriersprung. Die Steuereinheit meldet „alle drei Achsen in Referenz“ auf der RI-Leitung, die das Programm überwacht; steht die Maschine geparkt, überspringt es die Suchläufe, fährt X und Y 10 mm von ihren Schaltern weg und sucht sie erneut, sodass es ihr Ankommen live sieht.
* **Ruhige Bewegung**: Bewegungen gehen als ein durchgehender Strom hinaus, dessen Bytepositionen die Zeit tragen (ein USB-Adapter liefert die Bytes sonst in Schüben), Z fährt Byte für Byte, und `Esc` bricht einen Auftrag in Sekundenbruchteilen ab und schaltet die Schrittmotortreiber ab.
* Die **Spindel**: standardmäßig die der Steuereinheit selbst (ihre Latch-Bits, bis 60000 U/min); mit `spindle = node` in der Maschinendatei fragt der Auftrag stattdessen beim EH3D-NODE-BUNGARD an (ein Frequenzumrichter an den Ausgängen des Knotens, jede Drehzahl bis zum Maximum des Knotens, bei jedem Stopp angehalten). Mit einem Knoten lesen die Programme außerdem die Lampen der Maschine, sodass jede Achse, die in Referenz kommt, live gesehen wird, ohne die Home-Leitung mitten im Datenstrom zu lesen.
* Die **Serververbindung** (`connection = server` in der Maschinendatei, mit `server_address`, `server_token` und `server_machine`): Statt die Maschine selbst zu steuern, schickt das Programm den Auftrag an den [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER), der ihn mit den Konstanten der Maschine plant, über den Knoten der Maschine ausführt und dem Programm meldet, wie es läuft; das Programm zeigt den Fortschritt und beantwortet die Dialoge wie gewohnt, `Esc` stoppt den Auftrag auf dem Server, und das Schließen des Programms stoppt ihn nicht.

## 🧩 Rolle in EH3D

EH3D-ROUTEPRO ist das Fräs-Geschwister von EH3D-DRILLPRO und hängt per Pfad davon ab: Der Ordner `EH3D-DRILLPRO` muss neben diesem liegen. Es verwendet das serielle Protokoll, den Bildschirm und die Sprachunterstützung wieder. Es hängt noch von keinem anderen EH3D-Repository ab.

## 🏛️ Architektur

<p align="center">
  <img src="images/architecture.svg" alt="Architektur von EH3D-ROUTEPRO: wer es nutzt, was es enthält und wovon es abhängt" width="100%">
</p>

Wie die Teile dieses Repositorys zusammenpassen: wer es nutzt, was es enthält und wovon es abhängt. Das Bild wird aus `docs/architecture.json` gezeichnet und gegen den Code geprüft.

## 🚀 Schnellstart

Du brauchst eine stabile Rust-Toolchain.

```bash
cargo build --release --features gui
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

Starten Sie es mit `run.bat` (Windows) oder `./run.sh` (Linux und macOS); beide bauen das Programm bei Bedarf zuerst. Beide brauchen Rust und den Nachbarordner `EH3D-DRILLPRO`: `run.bat [DATEI.PLT] [--lang en|es|it|de|fr|pt|zh|ja] [--data DIR] [--dump]`. `--data` ist der Ordner mit `ROUTEPRO.RTD` und `ROUTEPRO.RCF` (Standard: `testdata/`), `--dump` gibt den Bildschirm als einfachen Text aus. Tasten: `Alt` plus der hervorgehobene Buchstabe öffnet ein Menü, Pfeile bewegen, `Enter` bestätigt, `Esc` geht zurück; die Menüleiste ist auch anklickbar. Chinesisch und Japanisch brauchen eine CJK-Schrift unter den Systemschriften. `--connection FILE` verweist auf eine andere Maschinendatei als die Standarddatei (`%APPDATA%\EH3D\machine.cfg` unter Windows, sonst `~/.config/eh3d/machine.cfg`); ohne Maschinendatei läuft das Programm im Simulator. `--modern` startet statt der klassischen Bildschirme die menügesteuerte Oberfläche.

## 🧪 Tests

Die Suite hat 42 Tests (`cargo test --workspace`). Sie prüfen den HP-GL-Leser, die Bahnplanung (Durchgänge, Eintauchen, Schnitt und Abheben), den Frässplan des Protokoll-Crates, die klassischen Bildschirme Zelle für Zelle und die Bildschirme in allen acht Sprachen. Das wiederverwendete Protokoll (Bytes, Bewegung, Sicherheitstor) wird in EH3D-DRILLPRO getestet. Die CI führt außerdem `cargo fmt --check` und `cargo clippy` mit Warnungen als Fehlern aus, und `python -I tools/ci_validate.py` prüft Manifest, Versionen, Changelog und die acht READMEs.

## 🏗️ Build und Veröffentlichung

Die Version steht in `Cargo.toml` und in `eh3d.project.json`, und jede Version hat ihren Eintrag in [CHANGELOG.md](CHANGELOG.md). Releases werden auf GitHub veröffentlicht.

**Fertige Pakete:** Jede Version enthält ein Paket für Windows (zip), Linux (tar.gz) und macOS (Apple Silicon und Intel, tar.gz) mit dem Programm, der Lizenz und dieser README, jeweils mit der SHA-256-Prüfsumme daneben. Sie werden vom Workflow `Release binaries` dieses Repositorys gebaut. Laden Sie eines von der Releases-Seite herunter; unter Linux braucht das Fenster X11 oder Wayland.

## 📂 Struktur des Repositorys

<p align="center">
  <img src="images/repository-structure.svg" alt="Struktur des Repositorys EH3D-ROUTEPRO" width="100%">
</p>

<details>
<summary>Textversion des Baums</summary>

```text
EH3D-ROUTEPRO/
├── src/
│   ├── hpgl.rs                                  der HP-GL-Leser (IN, SP, PU, PD, PA, PR)
│   ├── tools.rs                                 die Werkzeugtabelle ROUTEPRO.RTD
│   ├── job.rs                                   HP-GL-Bahnen als Fräsauftrag
│   ├── app.rs texts.rs                          die Anwendung und ihre Texte in acht Sprachen
│   └── bin/                                     die Programme routepro und routepro-console
├── protocol/src/
│   ├── route.rs                                 Durchgänge, Eintauchen, Schnitt und Abheben für jede Bahn
│   └── lib.rs                                   exportiert das Protokoll von EH3D-DRILLPRO erneut
├── tests/                                       HP-GL-Lesen, Bahnplanung, die klassischen Bildschirme und die Bildschirme in allen Sprachen
├── docs/                                        die Protokollnotizen, die klassischen Bildschirme
├── testdata/                                    kleine Beispieldateien für die Tests
├── config/                                      die Basiskonfiguration, Zeile für Zeile erklärt
├── examples/                                    Beispiele, von einem Quadrat bis 100000 Punkten
├── tools/                                       CI-Prüfung, README-Prüfung und die Bild-Werkzeuge
├── images/                                      Banner und die Bilder von Architektur und Repository-Struktur
├── .github/                                     CI- und Release-Workflows, Vorlagen für Issues und Pull Requests
├── build.sh build.bat                           Build- und Testskripte
├── run.sh run.bat                               starten das Programm
├── eh3d.project.json                            Manifest: Version, Rolle, Reifegrad und Build-Befehl
├── Cargo.toml Cargo.lock                        das Rust-Paket und seine festgelegten Abhängigkeiten
├── CHANGELOG.md                                 was sich in jeder Version geändert hat
├── SECURITY.md SUPPORT.md                       Schwachstellen melden und Fragen stellen
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   Mitwirken, Verhaltenskodex und Lizenz
```

</details>

## 🗺️ Roadmap und bekannte Grenzen

**Bekannte Grenzen heute**

* Ein Fräsauftrag ist auf einer echten Maschine noch nicht gelaufen: Das serielle Protokoll, das es mit EH3D-DRILLPRO teilt, wurde auf einer Bungard CCD MTC erprobt (die Referenzfahrt der drei Achsen und die Bewegungen jeder Achse), die Schritte pro Millimeter sind nur gegen die Konfiguration des Originals geprüft, und die Ausgänge für Spindel und Staubsauger sind unerprobt.
* Verstanden werden nur die HP-GL-Befehle `IN`, `SP`, `PU`, `PD`, `PA` und `PR`; jeder andere Befehl wird gezählt und ignoriert, nie fatal.
* Der Leser und die Planung wurden nur mit kleinen, für die Tests geschriebenen Beispieldateien erprobt.
* Das Programm braucht den Ordner von EH3D-DRILLPRO neben sich, weil es per Pfad davon abhängt.
* Unterstützt werden nur die MTC-Modelle; die ATC-Modelle noch nicht, die XL-Modelle sind nicht abgedeckt. Chinesisch und Japanisch brauchen eine CJK-Schrift unter den Systemschriften.

**Als Nächstes**

* Einen Fräsauftrag auf der Maschine ausführen und die Schritte pro Millimeter, die Spindel und die Staubsauger-Ausgänge bestätigen.
* Den HP-GL-Leser mit Plot-Dateien von echten Platinen erproben.
* HP-GL mit dem gemeinsamen Leser von [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS) lesen, wo er geplant ist.

## 🌍 Sprachen

README und Texte gibt es auf Englisch, Spanisch, Französisch, Italienisch, Deutsch, Chinesisch, Japanisch und europäischem Portugiesisch.

## 🔗 Verwandte Projekte

* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): das Geschwister für das Bohren; dieses Projekt verwendet sein Protokoll, seinen Bildschirm und seine Sprachen
* [EH3D-CORE-LIBS](https://github.com/JuanenRac/EH3D-CORE-LIBS): die gemeinsamen G-Code- und Excellon-Leser der Suite; HP-GL ist dort geplant
* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): der Projektcontainer und die Maschinen-Telemetrietypen der Suite
* [EH3D-NODE-BUNGARD](https://github.com/JuanenRac/EH3D-NODE-BUNGARD): ein ESP32-S3-Knoten (Ethernet oder WLAN), der im Gehäuse der Maschine sitzt, die Schritte über seine eigene serielle Schnittstelle mit dem Timing ausgibt, das ein USB-Adapter nicht halten kann, die Lampen der Maschine liest und den Spindelantrieb steuert; er hat ein eigenes Web-Panel
* [EH3D-SERVER](https://github.com/JuanenRac/EH3D-SERVER): das Zentrum der EH3D-Familie: Es nimmt die Aufträge dieses Programms (`connection = server`) und der anderen Clients an und führt sie über den Knoten auf der Maschine aus

## 📚 Dokumentation

* [Protokollnotizen](docs/PROTOCOL.md): was über die Ansteuerung der Steuereinheit bekannt ist (gemeinsam mit EH3D-DRILLPRO) und was noch eine Hypothese ist.
* [Verbindung und Abstimmung](https://github.com/JuanenRac/EH3D-DRILLPRO/blob/main/docs/CONNECTION.md): die Maschinendatei, die drei Verbindungen und jeder Wert
* [Die klassischen Bildschirme](docs/CLASSIC.md): wie die Bildschirme des MS-DOS-Programms nachgebaut und geprüft werden
* [Änderungsprotokoll](CHANGELOG.md)

## 🤝 Mitwirken und Community

* Führe vor einem Pull Request `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` und `python -I tools/ci_validate.py` aus: alles muss bestehen. Siehe [CONTRIBUTING.md](CONTRIBUTING.md) und den [Verhaltenskodex](CODE_OF_CONDUCT.md).
* Das README gibt es in acht Sprachen: Eine Textänderung wird in allen acht Dateien gemacht, und die beiden Bilder werden aus `docs/architecture.json` und `docs/structure.json` mit `tools/make_architecture.py` und `tools/make_structure.py` gezeichnet.
* Beschreibe nicht als auf einer Maschine funktionierend, was nur im Simulator erprobt wurde.
* Melde eine Schwachstelle privat: [SECURITY.md](SECURITY.md). Fragen, Ideen und Meldungen: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTOR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LIZENZ

Software: GPL-3.0-or-later. Siehe [LICENSE](LICENSE). Die Hardware-Entwürfe der Suite verwenden die CERN OHL-S.
