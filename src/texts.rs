//! User-visible texts of ROUTEPRO in the eight supported languages.
//! Order: English (default), Spanish, Italian, German, French, Portuguese, Chinese, Japanese.

use eh3d_drillpro::translations;

translations! { Text, tr, ALL_TEXTS;
    MenuLoad => ["~L~oad", "~C~argar", "~C~arica", "~L~aden", "~C~harger", "~C~arregar", "载入(~L~)", "読込(~L~)"],
    MenuPaths => ["~P~aths", "~T~razados", "~P~ercorsi", "P~f~ade", "~T~racés", "T~r~açados", "路径(~P~)", "パス(~P~)"],
    MenuTools => ["~T~ools", "~H~erramientas", "~U~tensili", "~W~erkzeuge", "~O~utils", "~F~erramentas", "刀具(~T~)", "工具(~T~)"],
    MenuConfig => ["~C~onfig.", "C~o~nfig.", "Co~n~fig.", "~K~onfig.", "Co~n~fig.", "Co~n~fig.", "配置(~C~)", "設定(~C~)"],
    MenuStart => ["~S~tart", "~I~niciar", "~A~vvia", "S~t~art", "~D~émarrer", "~I~niciar", "开始(~A~)", "開始(~A~)"],
    MenuLanguage => ["Lan~g~uage", "I~d~ioma", "~L~ingua", "S~p~rache", "~L~angue", "I~d~ioma", "语言(~G~)", "言語(~G~)"],
    MenuQuit => ["~Q~uit", "~S~alir", "~E~sci", "~B~eenden", "~Q~uitter", "~S~air", "退出(~Q~)", "終了(~Q~)"],

    TitleSelectFile => ["Select File", "Seleccionar archivo", "Seleziona file", "Datei wählen", "Choisir un fichier", "Selecionar ficheiro", "选择文件", "ファイル選択"],
    TitlePaths => ["Paths", "Trazados", "Percorsi", "Pfade", "Tracés", "Traçados", "路径", "パス"],
    TitleTools => ["Tool Data", "Datos de herramientas", "Dati utensili", "Werkzeugdaten", "Données d'outils", "Dados das ferramentas", "刀具数据", "工具データ"],
    TitleConfig => ["Configuration", "Configuración", "Configurazione", "Konfiguration", "Configuration", "Configuração", "配置", "設定"],
    TitleLanguage => ["Language", "Idioma", "Lingua", "Sprache", "Langue", "Idioma", "语言", "言語"],
    TitleRouting => ["Routing", "Fresado", "Fresatura", "Fräsen", "Fraisage", "Fresagem", "铣削", "ルーティング"],
    TitleQuit => ["Quit", "Salir", "Esci", "Beenden", "Quitter", "Sair", "退出", "終了"],

    LblFile => ["File", "Archivo", "File", "Datei", "Fichier", "Ficheiro", "文件", "ファイル"],
    LblPaths => ["Paths", "Trazados", "Percorsi", "Pfade", "Tracés", "Traçados", "路径数", "パス数"],
    LblPens => ["Tools", "Herramientas", "Utensili", "Werkzeuge", "Outils", "Ferramentas", "刀具", "工具"],
    LblSize => ["Size (mm)", "Tamaño (mm)", "Dimensioni (mm)", "Größe (mm)", "Taille (mm)", "Tamanho (mm)", "尺寸 (mm)", "サイズ (mm)"],
    LblCut => ["Cut length (mm)", "Longitud de corte (mm)", "Lunghezza di taglio (mm)", "Schnittlänge (mm)", "Longueur de coupe (mm)", "Comprimento de corte (mm)", "切削长度 (mm)", "切削長 (mm)"],
    LblProgress => ["Progress", "Progreso", "Avanzamento", "Fortschritt", "Progression", "Progresso", "进度", "進捗"],
    LblTime => ["Simulated time (s)", "Tiempo simulado (s)", "Tempo simulato (s)", "Simulierte Zeit (s)", "Temps simulé (s)", "Tempo simulado (s)", "模拟时间 (秒)", "シミュレーション時間 (秒)"],
    LblSerial => ["Serial interface", "Interfaz serie", "Interfaccia seriale", "Serielle Schnittstelle", "Interface série", "Interface série", "串口", "シリアル"],
    LblIgnored => ["Ignored commands", "Comandos ignorados", "Comandi ignorati", "Ignorierte Befehle", "Commandes ignorées", "Comandos ignorados", "忽略的命令", "無視したコマンド"],

    ColTool => ["Tool", "Herram.", "Utens.", "Werkz.", "Outil", "Ferram.", "刀具", "工具"],
    ColDepth => ["Depth (mm)", "Prof. (mm)", "Prof. (mm)", "Tiefe (mm)", "Prof. (mm)", "Prof. (mm)", "深度 (mm)", "深さ (mm)"],
    ColPasses => ["Passes", "Pasadas", "Passate", "Durchgänge", "Passes", "Passagens", "次数", "回数"],
    ColFeedXY => ["XY feed", "Avance XY", "Avanz. XY", "Vorschub XY", "Avance XY", "Avanço XY", "XY进给", "XY送り"],
    ColFeedZ => ["Z feed", "Avance Z", "Avanz. Z", "Vorschub Z", "Avance Z", "Avanço Z", "Z进给", "Z送り"],
    ColSpindle => ["Spindle", "Husillo", "Mandrino", "Spindel", "Broche", "Fuso", "主轴", "スピンドル"],

    NoFile => ["No file loaded", "Ningún archivo cargado", "Nessun file caricato", "Keine Datei geladen", "Aucun fichier chargé", "Nenhum ficheiro carregado", "未载入文件", "ファイル未読込"],
    NoToolFile => ["Tool data file not found", "No se encuentra el archivo de herramientas", "File utensili non trovato", "Werkzeugdatei nicht gefunden", "Fichier d'outils introuvable", "Ficheiro de ferramentas não encontrado", "未找到刀具数据文件", "工具データファイルが見つかりません"],
    NoConfigFile => ["Configuration file not found", "No se encuentra el archivo de configuración", "File di configurazione non trovato", "Konfigurationsdatei nicht gefunden", "Fichier de configuration introuvable", "Ficheiro de configuração não encontrado", "未找到配置文件", "設定ファイルが見つかりません"],
    ErrRead => ["Error reading plot file", "Error al leer el archivo de trazado", "Errore di lettura del file di tracciato", "Fehler beim Lesen der Plot-Datei", "Erreur de lecture du fichier de tracé", "Erro ao ler o ficheiro de traçado", "读取绘图文件出错", "プロットファイルの読込エラー"],
    SimOnly => ["SIMULATION - no machine connected", "SIMULACIÓN - sin máquina conectada", "SIMULAZIONE - nessuna macchina collegata", "SIMULATION - keine Maschine verbunden", "SIMULATION - aucune machine connectée", "SIMULAÇÃO - sem máquina ligada", "模拟 - 未连接机器", "シミュレーション - 機械未接続"],
    Uncalibrated => ["Machine not calibrated: motion is simulated only", "Máquina sin calibrar: el movimiento es solo simulado", "Macchina non calibrata: movimento solo simulato", "Maschine nicht kalibriert: Bewegung nur simuliert", "Machine non calibrée : mouvement simulé uniquement", "Máquina não calibrada: movimento apenas simulado", "机器未校准：仅模拟运动", "機械は未校正：動作はシミュレーションのみ"],
    Connecting => ["Connecting control unit ...", "Conectando con la controladora ...", "Connessione all'unità di controllo ...", "Verbinde mit der Steuereinheit ...", "Connexion à l'unité de commande ...", "A ligar à unidade de controlo ...", "正在连接控制器 ...", "コントローラーに接続中 ..."],
    Routing => ["Routing ...", "Fresando ...", "Fresatura ...", "Fräse ...", "Fraisage ...", "A fresar ...", "铣削中 ...", "ルーティング中 ..."],
    Done => ["Done", "Terminado", "Fatto", "Fertig", "Terminé", "Concluído", "完成", "完了"],
    QuitAsk => ["Do you really want to quit?", "¿Seguro que quieres salir?", "Vuoi davvero uscire?", "Wirklich beenden?", "Voulez-vous vraiment quitter ?", "Tem a certeza de que quer sair?", "确定要退出吗？", "本当に終了しますか？"],
    BtnYes => ["Yes", "Sí", "Sì", "Ja", "Oui", "Sim", "是", "はい"],
    BtnNo => ["No", "No", "No", "Nein", "Non", "Não", "否", "いいえ"],
    Directory => ["Directory", "Directorio", "Cartella", "Verzeichnis", "Répertoire", "Pasta", "目录", "フォルダー"],
    StatusMain => [
        "Alt+letter: menu   Esc: back   Enter: accept   Up/Down: move",
        "Alt+letra: menú   Esc: volver   Intro: aceptar   Arriba/Abajo: mover",
        "Alt+lettera: menu   Esc: indietro   Invio: conferma   Su/Giù: muovi",
        "Alt+Buchstabe: Menü   Esc: zurück   Enter: übernehmen   Hoch/Runter: bewegen",
        "Alt+lettre : menu   Échap : retour   Entrée : valider   Haut/Bas : déplacer",
        "Alt+letra: menu   Esc: voltar   Enter: aceitar   Cima/Baixo: mover",
        "Alt+字母：菜单  Esc：返回  回车：确定  上/下：移动",
        "Alt+文字：メニュー  Esc：戻る  Enter：決定  上/下：移動"
    ],

    WaitToolChange => ["Insert tool {} and press Enter", "Inserte la herramienta {} y pulse Intro", "Inserire l'utensile {} e premere Invio", "Werkzeug {} einsetzen und Enter drücken", "Insérez l'outil {} et appuyez sur Entrée", "Insira a ferramenta {} e prima Enter", "请装入刀具 {} 并按回车", "工具 {} を取り付けて Enter を押してください"],
    WaitSpindle => ["Press Enter to start the spindle", "Pulse Intro para arrancar el husillo", "Premere Invio per avviare il mandrino", "Enter drücken, um die Spindel zu starten", "Appuyez sur Entrée pour démarrer la broche", "Prima Enter para arrancar o fuso", "按回车启动主轴", "Enter を押してスピンドルを起動"],
    Stopped => ["Stopped (drivers off)", "Detenido (drivers apagados)", "Fermato (driver spenti)", "Gestoppt (Treiber aus)", "Arrêté (drivers coupés)", "Parado (drivers desligados)", "已停止（驱动器已关闭）", "停止しました（ドライバ off）"],
    LinkFailed => ["Connection failed", "Fallo de conexión", "Connessione fallita", "Verbindung fehlgeschlagen", "Échec de la connexion", "Falha na ligação", "连接失败", "接続に失敗しました"],
    LblLink => ["Link", "Enlace", "Collegamento", "Verbindung", "Liaison", "Ligação", "连接", "接続"],
    LblItems => ["Steps of the job", "Pasos del trabajo", "Passi del lavoro", "Arbeitsschritte", "Étapes du travail", "Passos do trabalho", "作业步骤", "作業ステップ"],
    KeyStop => ["Esc: stop", "Esc: detener", "Esc: ferma", "Esc: stoppen", "Échap : arrêter", "Esc: parar", "Esc：停止", "Esc：停止"],
    RealMachine => ["REAL MACHINE ({}) - Start moves it", "MÁQUINA REAL ({}) - Iniciar la mueve", "MACCHINA REALE ({}) - Avvia la muove", "ECHTE MASCHINE ({}) - Start bewegt sie", "MACHINE RÉELLE ({}) - Démarrer la déplace", "MÁQUINA REAL ({}) - Iniciar move-a", "真实机器 ({}) - 开始将使其运动", "実機 ({}) - 開始すると動きます"],
    RealStop => ["Esc stops it; keep a hand near the power switch", "Esc la detiene; ten una mano cerca del interruptor", "Esc la ferma; tieni una mano vicino all'interruttore", "Esc stoppt sie; Hand am Netzschalter halten", "Échap l'arrête ; gardez une main près de l'interrupteur", "Esc pára-a; tenha uma mão perto do interruptor", "Esc 停止；手放在电源开关旁", "Esc で停止；電源スイッチの近くに手を置いてください"],
}
