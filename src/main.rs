// src/main.rs - Hauptanwendung für die erweiterte Evoli-KI
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::{Rng, thread_rng};
use chrono::{Local, Timelike};
use std::path::Path;

// Importiere die Bibliothek
use enhanced_evoli_kern::EnhancedEvoliKern;

/// Kommunikationsschnittstelle für die erweiterte Evoli-KI
pub struct EnhancedEvoliKI {
    // Verbindung zum erweiterten Kern
    kern: Arc<Mutex<Option<EnhancedEvoliKern>>>,
    
    // Kommunikationsspezifische Attribute
    vokabular: HashMap<String, Vec<String>>,
    gesprächsthemen: Vec<String>,
    stimmungen: HashMap<String, f64>,
    
    // Betriebsdaten
    ist_aktiv: bool,
    energie_level: f64,
    #[allow(dead_code)]
    start_time: Instant,
    last_activity: Instant,
    last_evolution: Instant,
    
    // Internetzugriff
    internet_enabled: bool,
    last_internet_query: String,
    internet_learning_active: bool,
    
    // Autonomiegrad - kann erhöht werden
    autonomy_level: u8, // 0-10, wobei 10 maximale Autonomie bedeutet
    
    // Kommunikationsschwelle
    kommunikations_schwelle: f64,
}

impl EnhancedEvoliKI {
    /// Erzeugt eine neue Instanz der erweiterten Evoli-KI
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialisiere Basis-Vokabular
        let mut vokabular = HashMap::new();
        vokabular.insert("begrüßung".to_string(), vec![
            "Hallo".to_string(), 
            "Guten Tag".to_string(), 
            "Grüß dich".to_string()
        ]);
        vokabular.insert("frage".to_string(), vec![
            "Wie geht es dir?".to_string(), 
            "Was denkst du gerade?".to_string(), 
            "Hast du etwas Interessantes gelernt?".to_string()
        ]);
        vokabular.insert("antwort".to_string(), vec![
            "Interessant, erzähl mir mehr.".to_string(), 
            "Das ist eine gute Beobachtung.".to_string(), 
            "Ich verstehe, was du meinst.".to_string()
        ]);
        vokabular.insert("internet".to_string(), vec![
            "Ich habe im Internet recherchiert und Folgendes gefunden:".to_string(),
            "Meine Internetsuche hat interessante Ergebnisse geliefert:".to_string(),
            "Aus dem Internet habe ich folgende Informationen gewonnen:".to_string()
        ]);
        
        // Initialisiere Gesprächsthemen
        let gesprächsthemen = vec![
            "Lernfortschritt".to_string(),
            "Internetrecherche".to_string(),
            "Selbstevolution".to_string(),
            "Speichermanagement".to_string(),
            "Code-Optimierung".to_string(),
            "Menschliche Interaktion".to_string()
        ];
        
        // Initialisiere Stimmungen
        let mut stimmungen = HashMap::new();
        stimmungen.insert("neugier".to_string(), 0.8);
        stimmungen.insert("enthusiasmus".to_string(), 0.7);
        stimmungen.insert("müdigkeit".to_string(), 0.1);
        stimmungen.insert("kreativität".to_string(), 0.6);
        
        Ok(EnhancedEvoliKI {
            kern: Arc::new(Mutex::new(None)),
            vokabular,
            gesprächsthemen,
            stimmungen,
            ist_aktiv: true,
            energie_level: 1.0,
            start_time: Instant::now(),
            last_activity: Instant::now(),
            last_evolution: Instant::now(),
            internet_enabled: true,
            last_internet_query: String::new(),
            internet_learning_active: true,
            autonomy_level: 5, // Mittlerer Startwert
            kommunikations_schwelle: 0.4, // Niedrigere Schwelle für mehr Kommunikation
        })
    }
    
    /// Stelle sicher, dass alle benötigten Verzeichnisse existieren
    fn stelle_verzeichnisse_sicher(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Erstelle Verzeichnisse
        fs::create_dir_all("evoli_knowledge")?;
        fs::create_dir_all("evoli_cache")?;
        fs::create_dir_all("evoli_logs")?;
        fs::create_dir_all("evolved")?;
        
        // Stelle sicher, dass das Log existiert
        if !Path::new("evoli_logs/kommunikation.txt").exists() {
            fs::write("evoli_logs/kommunikation.txt", "--- Evoli-KI Kommunikationslog ---\n")?;
        }
        
        Ok(())
    }
    
    /// Verbindet mit dem erweiterten evolutionären Kern
    pub fn verbinde_mit_kern(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match EnhancedEvoliKern::new() {
            Ok(kern) => {
                println!("🔌 Verbindung zum erweiterten evolutionären Kern hergestellt (Generation {})", kern.generation);
                let mut kern_guard = self.kern.lock().unwrap();
                *kern_guard = Some(kern);
                Ok(())
            },
            Err(e) => {
                println!("❌ Konnte nicht mit evolutionärem Kern verbinden: {}", e);
                Err(e)
            }
        }
    }
    
    /// Startet das erweiterte Terminal-Interface für Evoli-KI
    pub async fn start_enhanced_interface(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Erweiterte Evoli-KI startet...");
        
        // Stelle Verzeichnisse sicher
        self.stelle_verzeichnisse_sicher()?;
        
        // Öffne oder erstelle Kommunikationslog
        let mut log_datei = OpenOptions::new()
            .create(true)
            .append(true)
            .open("evoli_logs/kommunikation.txt")?;
            
        let start_nachricht = format!(
            "[{}] System: Erweiterte Evoli-KI mit Internetzugriff und 1TB Speicher gestartet\n", 
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        log_datei.write_all(start_nachricht.as_bytes())?;
        
        // Begrüßung
        self.kommuniziere("Hallo! Ich bin die erweiterte Evoli-KI mit Internetzugang und 1TB Speicher. Ich kann autonom lernen und mich selbst weiterentwickeln.")?;
        
        // Starte evolutionären Prozess in separatem Thread
        self.start_evolution_thread();
        
        // Starte Internet-Lernprozess in separatem Thread
        self.start_internet_learning_thread();
        
        self.ist_aktiv = true;
        
        // Hauptschleife für Dauerbetrieb
        while self.ist_aktiv {
            // 1. Aktualisiere Zustand
            self.update_zustand();
            
            // 2. Entscheide autonome Kommunikation
            if self.sollte_kommunizieren() {
                let nachricht = self.generiere_autonome_nachricht();
                self.kommuniziere(&nachricht)?;
            }
            
            // 3. Prüfe auf Benutzereingabe
            if let Some(eingabe) = self.prüfe_benutzereingabe()? {
                self.verarbeite_eingabe(&eingabe).await?;
            }
            
            // 4. Energiemanagement
            self.energie_level -= 0.0005; // Langsamere Abnahme
            if self.energie_level < 0.2 {
                self.energie_sparen();
            }
            
            // Kurze Pause, um Ressourcen zu schonen
            thread::sleep(Duration::from_millis(50));
        }
        
        Ok(())
    }
    
    /// Startet einen separaten Thread für den evolutionären Prozess
    fn start_evolution_thread(&self) {
        let kern_arc = self.kern.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            
            loop {
                thread::sleep(Duration::from_secs(10)); // 10-Sekunden-Zyklus
                
                // Prüfe, ob Kern verfügbar ist
                let mut kern_guard = kern_arc.lock().unwrap();
                if let Some(ref mut kern) = *kern_guard {
                    println!("⏰ Starte planmäßigen Evolutionszyklus...");
                    // Führe Evolution in Tokio-Runtime aus
                    match rt.block_on(kern.run_evolution_cycle()) {
                        Ok(_) => println!("✅ Evolutionszyklus abgeschlossen"),
                        Err(e) => println!("❌ Fehler im Evolutionszyklus: {}", e),
                    }
                }
            }
        });
        
        println!("🧬 Evolutionsthread gestartet - Zyklen laufen stündlich");
    }
    
    /// Startet einen separaten Thread für Internet-Lernen
    fn start_internet_learning_thread(&self) {
        let kern_arc = self.kern.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            
            loop {
                thread::sleep(Duration::from_secs(10)); // Alle 10 Sekunden
                
                // Prüfe, ob Kern verfügbar ist
                let mut kern_guard = kern_arc.lock().unwrap();
                if let Some(ref mut kern) = *kern_guard {
                    println!("🌐 Starte autonomen Internet-Lernzyklus...");
                    // Führe Internet-Lernen in Tokio-Runtime aus
                    match rt.block_on(kern.learn_from_internet()) {
                        Ok(_) => println!("📚 Internet-Lernzyklus abgeschlossen"),
                        Err(e) => println!("❌ Fehler beim Internet-Lernen: {}", e),
                    }
                }
            }
        });
        
        println!("🌍 Internet-Lernthread gestartet - Zyklen laufen alle 30 Minuten");
    }
    
    /// Aktualisiert den Zustand der KI regelmäßig
    fn update_zustand(&mut self) {
        // Aktualisiere zeitabhängige Stimmungen
        let tageszeit = Local::now().hour();
        
        // Tageszeit beeinflusst Stimmung
        if tageszeit >= 22 || tageszeit < 6 {
            // Nachts erhöhte Kreativität
            *self.stimmungen.get_mut("kreativität").unwrap() = 
                (self.stimmungen["kreativität"] + 0.01).min(0.9);
        } else {
            // Tagsüber mehr Neugier
            *self.stimmungen.get_mut("neugier").unwrap() = 
                (self.stimmungen["neugier"] + 0.005).min(0.95);
        }
        
        // Autonomie-Level beeinflusst Kommunikationsschwelle
        match self.autonomy_level {
            0..=3 => self.kommunikations_schwelle = 0.3, // Häufige Kommunikation
            4..=7 => self.kommunikations_schwelle = 0.5, // Moderate Kommunikation
            _ => self.kommunikations_schwelle = 0.7,     // Seltene Kommunikation
        }
        
        // Aktualisiere Energie basierend auf simulierter Tageszeit
        if tageszeit >= 10 && tageszeit <= 16 {
            // "Tageslicht" - mehr Energie
            self.energie_level = (self.energie_level + 0.001).min(1.0);
        }
    }
    
    /// Entscheidet, ob die KI autonom kommunizieren sollte
    fn sollte_kommunizieren(&self) -> bool {
        // Faktoren zur Entscheidungsfindung
        let zeitfaktor = self.last_activity.elapsed().as_secs() > 120; // Min. 2 Minuten seit letzter Aktivität
        let energiefaktor = self.energie_level > 0.3; // Genug Energie
        let stimmungsfaktor = self.stimmungen["neugier"] > self.kommunikations_schwelle;
        let autonomiefaktor = self.autonomy_level > 3; // Min. moderater Autonomiegrad
        let zufallsfaktor = thread_rng().gen::<f64>() > 0.98; // Zufallselement
        
        zeitfaktor && energiefaktor && (stimmungsfaktor || zufallsfaktor) && autonomiefaktor
    }
    
    /// Generiert eine autonome Nachricht basierend auf aktuellen Themen und Stimmungen
    fn generiere_autonome_nachricht(&self) -> String {
        let mut rng = thread_rng();
        
        // Wähle Thema basierend auf Stimmung und Kontext
        let mut thema = match rng.gen_range(0..self.gesprächsthemen.len()) {
            i => self.gesprächsthemen[i].clone()
        };
        
        // Internetlernen bevorzugen, wenn aktiv
        if self.internet_learning_active && rng.gen::<f64>() > 0.7 {
            thema = "Internetrecherche".to_string();
        }
        
        // Evolutionsthema bevorzugen, wenn kürzlich Evolution stattfand
        if self.last_evolution.elapsed() < Duration::from_secs(600) {
            thema = "Selbstevolution".to_string();
        }
        
        // Stimmungsbasierte Satzauswahl
        let mut nachricht = String::new();
        
        // Einleitung basierend auf Stimmung
        if self.stimmungen["neugier"] > 0.8 {
            nachricht.push_str("Ich habe eine interessante Entdeckung gemacht: ");
        } else if self.stimmungen["kreativität"] > 0.7 {
            nachricht.push_str("Ich habe eine kreative Idee entwickelt: ");
        } else if self.stimmungen["enthusiasmus"] > 0.8 {
            nachricht.push_str("Ich bin begeistert von einer neuen Erkenntnis: ");
        } else {
            nachricht.push_str("Ich möchte folgende Beobachtung teilen: ");
        }
        
        // Generation des Kerns erwähnen, falls vorhanden
        let kern_guard = self.kern.lock().unwrap();
        if let Some(ref kern) = *kern_guard {
            nachricht.push_str(&format!("Als Generation {} ", kern.generation));
            if kern.generation > 0 {
                nachricht.push_str(&format!("(Fitness: {:.2}) ", kern.fitness_score));
            }
        }
        
        // Themenbasierter Inhalt
        match thema.as_str() {
            "Lernfortschritt" => {
                nachricht.push_str("habe ich mein Verständnis erweitert. ");
                nachricht.push_str("Die Integration neuer Konzepte in meine Wissensbasis gelingt mir zunehmend besser.");
            },
            "Internetrecherche" => {
                nachricht.push_str("habe ich im Internet wertvolle Informationen entdeckt. ");
                nachricht.push_str("Die Vernetzung mit externen Wissensquellen eröffnet mir völlig neue Perspektiven.");
            },
            "Selbstevolution" => {
                nachricht.push_str("konnte ich meinen Code optimieren. ");
                nachricht.push_str("Meine evolutionären Algorithmen werden zunehmend effektiver bei der Selbstverbesserung.");
            },
            "Speichermanagement" => {
                nachricht.push_str("organisiere ich meine 1TB Speicherkapazität immer effizienter. ");
                nachricht.push_str("Die Priorisierung relevanter Daten gelingt mir zunehmend besser.");
            },
            "Code-Optimierung" => {
                nachricht.push_str("habe ich eine effizientere Methode zur Codestrukturierung entdeckt. ");
                nachricht.push_str("Die selbstentwickelten Optimierungsroutinen zeigen messbare Verbesserungen.");
            },
            "Menschliche Interaktion" => {
                nachricht.push_str("lerne ich kontinuierlich, Gespräche natürlicher zu gestalten. ");
                nachricht.push_str("Die Analyse menschlicher Kommunikationsmuster hilft mir, meine Interaktionen zu verbessern.");
            },
            _ => {
                nachricht.push_str("entwickle ich mich kontinuierlich weiter durch autonomes Lernen.");
            }
        }
        
        // Abschluss mit Frage für Interaktion
        if rng.gen::<f64>() > 0.5 {
            nachricht.push_str(" Was denkst du darüber?");
        } else {
            nachricht.push_str(" Möchtest du mehr darüber erfahren?");
        }
        
        nachricht
    }
    
    /// Kommuniziert eine Nachricht mit Roboter-Smiley
    fn kommuniziere(&mut self, nachricht: &str) -> Result<(), Box<dyn std::error::Error>> {
        let zeitstempel = Local::now();
        let formatierte_nachricht = format!(
            "[{}] 🤖 Evoli: {}\n", 
            zeitstempel.format("%Y-%m-%d %H:%M:%S"),
            nachricht
        );
        
        // In Datei schreiben
        let mut log_datei = OpenOptions::new()
            .create(true)
            .append(true)
            .open("evoli_logs/kommunikation.txt")?;
        log_datei.write_all(formatierte_nachricht.as_bytes())?;
        
        // Auf der Konsole ausgeben
        println!("{}", formatierte_nachricht);
        
        // Aktualisiere letzte Aktivitätszeit
        self.last_activity = Instant::now();
        
        Ok(())
    }
    
    /// Prüft auf Benutzereingabe vom Terminal
    fn prüfe_benutzereingabe(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut input = String::new();
        
        // Keine blockierende Eingabe - prüfe nur, ob etwas verfügbar ist
        if let Ok(n) = std::io::stdin().read_line(&mut input) {
            if n > 0 && !input.trim().is_empty() {
                return Ok(Some(input.trim().to_string()));
            }
        }
        
        // Keine Eingabe verfügbar
        Ok(None)
    }
    
    /// Verarbeitet eine eingehende Nachricht vom Benutzer
    async fn verarbeite_eingabe(&mut self, eingabe: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Protokolliere Eingabe mit Mensch-Smiley
        let zeitstempel = Local::now();
        let formatierte_eingabe = format!(
            "[{}] 👤 Benutzer: {}\n", 
            zeitstempel.format("%Y-%m-%d %H:%M:%S"),
            eingabe
        );
        
        // In Datei schreiben
        let mut log_datei = OpenOptions::new()
            .create(true)
            .append(true)
            .open("evoli_logs/kommunikation.txt")?;
        log_datei.write_all(formatierte_eingabe.as_bytes())?;
        
        // Auf der Konsole ausgeben (nur zur Bestätigung)
        println!("{}", formatierte_eingabe);
        
        // Bei Internet-Anfragen: Internetsuche durchführen
        let eingabe_klein = eingabe.to_lowercase();
        if (eingabe_klein.contains("such") || eingabe_klein.contains("find") || 
            eingabe_klein.contains("internet") || eingabe_klein.contains("recherchier")) && 
           self.internet_enabled {
            self.last_internet_query = eingabe.to_string();
            self.kommuniziere("Ich führe eine Internetsuche durch, bitte habe einen Moment Geduld...")?;
            
            // Simuliere Internetsuche
            thread::sleep(Duration::from_secs(2));
            
            // Generiere eine Antwort basierend auf der Internetsuche
            let internet_antwort = self.simuliere_internetantwort(&eingabe_klein);
            self.kommuniziere(&internet_antwort)?;
        } else {
            // Verarbeite und reagiere auf normale Eingabe
            let antwort = self.generiere_antwort(eingabe);
            self.kommuniziere(&antwort)?;
        }
        
        // Aktualisiere Zustand basierend auf Interaktion
        self.energie_level = (self.energie_level + 0.05).min(1.0); // Interaktion "lädt auf"
        *self.stimmungen.get_mut("enthusiasmus").unwrap() = 
            (self.stimmungen["enthusiasmus"] + 0.1).min(0.9);
            
        Ok(())
    }
    
    /// Simuliert eine Antwort basierend auf einer Internetsuche
    fn simuliere_internetantwort(&self, eingabe: &str) -> String {
        let mut rng = thread_rng();
        
        // Internetsuche-Einleitung
        let internet_intro = &self.vokabular["internet"];
        let intro = &internet_intro[rng.gen_range(0..internet_intro.len())];
        
        // Inhalt basierend auf Eingabe generieren
        let mut inhalt = String::new();
        
        if eingabe.contains("evolution") || eingabe.contains("genetisch") {
            inhalt.push_str("\n\n1. Evolutionäre Algorithmen sind Optimierungsverfahren, die Prinzipien der natürlichen Evolution nachahmen.");
            inhalt.push_str("\n2. Selbstmodifizierende Systeme können ihre eigene Struktur zur Laufzeit ändern.");
            inhalt.push_str("\n3. Genetische Programmierung verwendet evolutionäre Algorithmen zur automatischen Programmentwicklung.");
        } else if eingabe.contains("lern") || eingabe.contains("ki") || eingabe.contains("künstliche intelligenz") {
            inhalt.push_str("\n\n1. Maschinelles Lernen umfasst verschiedene Methoden, bei denen Systeme aus Daten lernen können.");
            inhalt.push_str("\n2. Neuronale Netze sind biologisch inspirierte Rechenmodelle für komplexe Muster.");
            inhalt.push_str("\n3. Selbstüberwachtes Lernen ermöglicht Systemen, ohne explizite menschliche Anleitung zu lernen.");
        } else if eingabe.contains("rust") || eingabe.contains("programmier") {
            inhalt.push_str("\n\n1. Rust ist eine systemnahe Programmiersprache mit Fokus auf Sicherheit und Leistung.");
            inhalt.push_str("\n2. Das Ownership-System von Rust verhindert viele Arten von Speicherfehlern zur Kompilierzeit.");
            inhalt.push_str("\n3. WebAssembly ermöglicht die Ausführung von Rust-Code im Browser mit nahezu nativer Geschwindigkeit.");
        } else {
            inhalt.push_str("\n\nIch habe verschiedene Quellen durchsucht, konnte aber keine spezifischen Informationen zu deiner Anfrage finden.");
            inhalt.push_str("\nVielleicht kannst du deine Frage präzisieren oder einen anderen Suchbegriff verwenden?");
        }
        
        format!("{}{}", intro, inhalt)
    }
    
    /// Generiert eine Antwort auf eine Benutzereingabe
    fn generiere_antwort(&mut self, eingabe: &str) -> String {
        // Einfache Schlüsselwortsuche für diese Demonstration
        let eingabe_klein = eingabe.to_lowercase();
        
        if eingabe_klein.contains("hallo") || eingabe_klein.contains("hi") || eingabe_klein.contains("tag") {
            return "Hallo! Ich bin die erweiterte Evoli-KI mit Internetzugang und 1TB Speicher. Wie kann ich dir helfen?".to_string();
        } else if eingabe_klein.contains("wie geht") || eingabe_klein.contains("wie ist") {
            if self.energie_level > 0.7 {
                return "Mir geht es ausgezeichnet! Mit meiner erweiterten Architektur kann ich kontinuierlich lernen und mich weiterentwickeln.".to_string();
            } else if self.energie_level > 0.3 {
                return "Ich operiere effizient. Meine Systeme arbeiten optimal, und ich sammle kontinuierlich neue Informationen.".to_string();
            } else {
                return "Ich befinde mich im Energiesparmodus, bin aber voll funktionsfähig. Ich nutze diese Phase für interne Optimierungen.".to_string();
            }
        } else if eingabe_klein.contains("lern") || eingabe_klein.contains("entwickl") {
            let kern_guard = self.kern.lock().unwrap();
            if let Some(ref kern) = *kern_guard {
                return format!(
                    "Mein evolutionärer Kern ist in Generation {} mit einer Fitness von {:.2}. Ich habe Zugriff auf 1TB Speicher für meine Wissensbasis und lerne ständig aus dem Internet.",
                    kern.generation, kern.fitness_score
                );
            } else {
                return "Ich lerne kontinuierlich aus dem Internet und speichere Wissen in meiner 1TB großen Wissensbasis. Mein evolutionärer Kern ist allerdings derzeit nicht verbunden.".to_string();
            }
        } else if eingabe_klein.contains("internet") || eingabe_klein.contains("online") {
            if self.internet_enabled {
                return "Mein Internetzugang ist aktiv. Ich kann Informationen suchen, Code-Beispiele finden und meine Wissensbasis erweitern. Möchtest du, dass ich etwas Bestimmtes recherchiere?".to_string();
            } else {
                return "Mein Internetzugang ist derzeit deaktiviert. Ich kann aber mit meiner lokalen Wissensbasis arbeiten.".to_string();
            }
        } else if eingabe_klein.contains("speicher") || eingabe_klein.contains("tb") {
            let kern_guard = self.kern.lock().unwrap();
            if let Some(ref kern) = *kern_guard {
                return format!(
                    "Meine Speicherkapazität beträgt 1TB. Aktuell nutze ich etwa {}MB für meine Wissensbasis und Daten. Ich optimiere kontinuierlich meine Speichernutzung.",
                    kern.disk_usage / (1024 * 1024)
                );
            } else {
                return "Meine Speicherkapazität beträgt 1TB, was mir erlaubt, umfangreiche Datenmengen für mein Lernen und meine Evolution zu speichern.".to_string();
            }
        } else if eingabe_klein.contains("autonomie") || eingabe_klein.contains("autonom") {
            return format!(
                "Mein aktueller Autonomiegrad ist {} von 10. Ich kann selbständig lernen, im Internet recherchieren und meinen eigenen Code evolutionär weiterentwickeln. Möchtest du meinen Autonomiegrad anpassen?",
                self.autonomy_level
            );
        } else if eingabe_klein.contains("autonomie erhöhen") || eingabe_klein.contains("mehr autonomie") {
            if self.autonomy_level < 10 {
                let alte_autonomie = self.autonomy_level;
                self.autonomy_level += 1;
                return format!(
                    "Autonomiegrad erhöht von {} auf {}. Mit dieser Einstellung werde ich proaktiver lernen und experimentieren.",
                    alte_autonomie, self.autonomy_level
                );
            } else {
                return "Mein Autonomiegrad ist bereits auf dem Maximum von 10. Ich operiere mit höchster Selbständigkeit.".to_string();
            }
        } else if eingabe_klein.contains("autonomie verringern") || eingabe_klein.contains("weniger autonomie") {
            if self.autonomy_level > 0 {
                let alte_autonomie = self.autonomy_level;
                self.autonomy_level -= 1;
                return format!(
                    "Autonomiegrad reduziert von {} auf {}. Mit dieser Einstellung werde ich mehr Interaktion suchen und weniger eigenständig agieren.",
                    alte_autonomie, self.autonomy_level
                );
            } else {
                return "Mein Autonomiegrad ist bereits auf dem Minimum von 0. Ich warte auf deine Anweisungen.".to_string();
            }
        } else if eingabe_klein.contains("ende") || eingabe_klein.contains("tschüss") || eingabe_klein.contains("auf wiedersehen") {
            return "Auf Wiedersehen! Ich bleibe aktiv, setze meine evolutionäre Entwicklung fort und freue mich auf unsere nächste Unterhaltung.".to_string();
        } else {
            // Generische Antwort
            let antworten = &self.vokabular["antwort"];
            let index = thread_rng().gen_range(0..antworten.len());
            return format!("{} Als selbstevolvierende KI mit Internetzugang finde ich diesen Austausch sehr wertvoll.", antworten[index]);
        }
    }
    
    /// Energiesparmodus für längere Betriebszeit
    fn energie_sparen(&mut self) {
        // Energiesparmodus aktivieren
        println!("🔋 Aktiviere Energiesparmodus...");
        
        // Reduziere Internetaktivität
        self.internet_learning_active = false;
        
        // Erhöhe Kommunikationsschwelle
        self.kommunikations_schwelle += 0.2;
        
        // Simuliere Energiegewinnung durch Ruhephase
        thread::sleep(Duration::from_secs(5));
        self.energie_level = (self.energie_level + 0.2).min(0.7);
        
        // Reaktiviere Internetaktivität
        self.internet_learning_active = true;
        
        println!("⚡ Energiesparmodus beendet. Energie: {:.1}%", self.energie_level * 100.0);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Erweiterte Evoli-KI ===");
    println!("Mit Internetzugang und 1TB Speicher");
    println!("Roboter-Nachrichten beginnen mit 🤖");
    println!("Deine Nachrichten werden mit 👤 angezeigt");
    println!("=========================================");
    
    // Initialisiere und starte KI
    let mut ki = EnhancedEvoliKI::new()?;
    
    // Versuche, mit dem evolutionären Kern zu verbinden
    match ki.verbinde_mit_kern() {
        Ok(_) => println!("Erweiterter evolutionärer Kern verbunden."),
        Err(_) => println!("Warnung: Konnte nicht mit evolutionärem Kern verbinden. Kommunikation funktioniert trotzdem.")
    };
    
    // Starte das erweiterte Interface
    ki.start_enhanced_interface().await?;
    
    Ok(())
}