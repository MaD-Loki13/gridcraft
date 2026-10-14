//! DoeringCraft: German for messages (alerts, toasts, the notice bar, formula errors). Messages
//! are built in English all over the engine, often with values filled in, so they are translated
//! where they are shown: an exact label first, then the templates below. A `{}` in a template
//! matches any text; that text is translated again, so a formula error keeps its parser detail
//! in German too. A message without a template stays English.

use std::sync::OnceLock;

/// English template → German. `{}` placeholders fill in order; `{1}`, `{2}` … in the German text
/// pick a placeholder by position when the order changes.
const TEMPLATES: &[(&str, &str)] = &[
    // Formula input
    ("There's a problem with this formula: {}", "Mit dieser Formel stimmt etwas nicht: {}"),
    ("There's a problem with this formula.", "Mit dieser Formel stimmt etwas nicht."),
    ("expected `,` or `)`, found {}", "erwartet wurde `,` oder `)`, gefunden: {}"),
    ("expected a reference after the sheet name, found {}", "nach dem Blattnamen wurde ein Bezug erwartet, gefunden: {}"),
    ("expected a number in array, found {}", "in der Matrix wurde eine Zahl erwartet, gefunden: {}"),
    ("array constants may only contain constants, found {}", "Matrixkonstanten dürfen nur Konstanten enthalten, gefunden: {}"),
    ("expected `,`, `;` or `}`, found {}", "erwartet wurde `,`, `;` oder `}`, gefunden: {}"),
    ("expected {}, found {}", "erwartet wurde {}, gefunden: {}"),
    ("unexpected `{}`", "unerwartet: `{}`"),
    ("unexpected {}", "unerwartet: {}"),
    ("formula is nested too deeply", "die Formel ist zu tief verschachtelt"),
    ("too many arguments", "zu viele Argumente"),
    ("invalid 3-D reference", "ungültiger 3D-Bezug"),
    ("array rows must have the same length", "alle Zeilen einer Matrix müssen gleich lang sein"),
    ("a structured reference names at most two columns", "ein strukturierter Verweis nennt höchstens zwei Spalten"),
    ("unclosed `[` in structured reference", "nicht geschlossene `[` im strukturierten Verweis"),
    ("end of formula", "Formelende"),
    ("The formula contains an error.", "Die Formel enthält einen Fehler."),
    ("The formula has an unterminated field name.", "In der Formel ist ein Feldname nicht abgeschlossen."),
    ("The formula has an unterminated text value.", "In der Formel ist ein Text nicht abgeschlossen (fehlendes Anführungszeichen)."),
    ("The formula is empty.", "Die Formel ist leer."),
    ("The formula is too long.", "Die Formel ist zu lang."),
    ("Cell must contain a formula.", "Die Zelle muss eine Formel enthalten."),
    ("Cell must contain a value.", "Die Zelle muss einen Wert enthalten."),
    ("The cell has no formula.", "Die Zelle enthält keine Formel."),
    ("You can't change part of an array.", "Teile einer Matrix können nicht geändert werden."),
    ("Multi-cell array formulas aren't allowed in tables.", "Matrixformeln über mehrere Zellen sind in Tabellen nicht erlaubt."),
    ("Reference isn't valid.", "Der Bezug ist ungültig."),
    ("The reference isn't valid.", "Der Bezug ist ungültig."),
    ("Input cell reference is not valid.", "Der Bezug der Eingabezelle ist ungültig."),
    // Files
    ("We can't open '{}': the encrypted file is damaged (it holds another encrypted file).", "„{}“ lässt sich nicht öffnen: Die verschlüsselte Datei ist beschädigt (sie enthält eine weitere verschlüsselte Datei)."),
    ("We can't open '{}': {}", "„{}“ lässt sich nicht öffnen: {}"),
    ("We can't import '{}': {}", "„{}“ lässt sich nicht importieren: {}"),
    ("'{}' is a legacy binary file but not an Excel workbook (no workbook stream). GridCraft can't open it.", "„{}“ ist eine ältere Binärdatei, aber keine Excel-Arbeitsmappe. DoeringCraft kann sie nicht öffnen."),
    ("'{}' is protected.", "„{}“ ist geschützt."),
    ("{}: unknown file format. Save as .xlsx, .csv, .tsv, .json or .html.", "{}: unbekanntes Dateiformat. Speichern Sie als .xlsx, .csv, .tsv, .json oder .html."),
    ("Excel 97-2003 (.xls) is supported for import only. Save as .xlsx or another supported export format.", "Excel 97-2003 (.xls) kann nur importiert werden. Speichern Sie als .xlsx oder in einem anderen unterstützten Format."),
    ("XLSB is supported for data import only. Save as .xlsx or another supported export format.", "XLSB kann nur für den Datenimport geöffnet werden. Speichern Sie als .xlsx oder in einem anderen unterstützten Format."),
    ("ODS is supported for data import only. Save as .xlsx or another supported export format.", "ODS kann nur für den Datenimport geöffnet werden. Speichern Sie als .xlsx oder in einem anderen unterstützten Format."),
    ("The file is empty.", "Die Datei ist leer."),
    ("Nothing to import.", "Es gibt nichts zu importieren."),
    ("files larger than 2 GB aren't supported", "Dateien über 2 GB werden nicht unterstützt"),
    ("the workbook stream is larger than 1 GiB", "der Arbeitsmappen-Datenstrom ist größer als 1 GiB"),
    ("stream too large", "Datenstrom zu groß"),
    ("damaged compound file: {}", "beschädigte Verbunddatei: {}"),
    ("passwords hashed with {} aren't supported", "Kennwörter mit dem Verfahren {} werden nicht unterstützt"),
    ("the password uses a spin count of {}, more than the {} allowed", "das Kennwort nutzt {} Durchläufe, erlaubt sind höchstens {}"),
    ("the stored password hash is damaged", "der gespeicherte Kennwort-Hash ist beschädigt"),
    ("The password you supplied is not correct.", "Das eingegebene Kennwort ist falsch."),
    ("the sheet is too tall for a web page ({} rows, at most {}). Save as .xlsx or CSV instead.", "das Blatt ist für eine Webseite zu lang ({} Zeilen, höchstens {}). Speichern Sie stattdessen als .xlsx oder CSV."),
    ("{}: no file system in the browser", "{}: im Browser gibt es kein Dateisystem"),
    ("not a GridCraft JSON workbook: {}", "keine DoeringCraft-JSON-Mappe: {}"),
    ("Couldn't decode the generated PDF.", "Das erzeugte PDF konnte nicht gelesen werden."),
    ("HTML clipboard unavailable; copying plain text: {}", "HTML-Zwischenablage nicht verfügbar; es wird reiner Text kopiert: {}"),
    // Printing
    ("A print job is already being submitted.", "Ein Druckauftrag wird gerade gesendet."),
    ("Select an available printer in File > Print before submitting.", "Wählen Sie vor dem Senden unter Datei › Drucken einen verfügbaren Drucker."),
    ("The print command did not produce a PDF.", "Der Druckbefehl hat kein PDF erzeugt."),
    ("Printing is unavailable here; export a PDF instead.", "Drucken ist hier nicht möglich; exportieren Sie stattdessen ein PDF."),
    ("Print job submitted: {}", "Druckauftrag gesendet: {}"),
    ("Print failed: {}", "Drucken fehlgeschlagen: {}"),
    ("native printer discovery is unavailable here", "die Druckersuche ist hier nicht verfügbar"),
    ("printer discovery or submission is already active", "Druckersuche oder Druckauftrag läuft bereits"),
    // Sheets and workbooks
    ("A sheet name can't be blank.", "Ein Blattname darf nicht leer sein."),
    ("A sheet name can't be longer than 31 characters.", "Ein Blattname darf höchstens 31 Zeichen lang sein."),
    ("A sheet name can't begin or end with an apostrophe.", "Ein Blattname darf nicht mit einem Apostroph beginnen oder enden."),
    ("A sheet name can't contain any of these characters: : \\ / ? * [ ]", "Ein Blattname darf keines dieser Zeichen enthalten: : \\ / ? * [ ]"),
    ("\"History\" is a reserved name.", "„Verlauf“ (History) ist ein reservierter Name."),
    ("That name is already taken. Try a different one.", "Dieser Name ist schon vergeben. Wählen Sie einen anderen."),
    ("A workbook must contain at least one visible worksheet.", "Eine Mappe muss mindestens ein sichtbares Blatt enthalten."),
    ("No sheets are hidden.", "Es sind keine Blätter ausgeblendet."),
    ("There's no such sheet.", "Dieses Blatt gibt es nicht."),
    ("no such sheet", "dieses Blatt gibt es nicht"),
    ("no workbook open", "keine Mappe geöffnet"),
    ("The copied sheet no longer exists.", "Das kopierte Blatt gibt es nicht mehr."),
    ("The workbook is protected and cannot be changed.", "Die Mappe ist geschützt und kann nicht geändert werden."),
    ("The sheet is protected with a password. Unprotect it first (Review › Unprotect Sheet).", "Das Blatt ist mit einem Kennwort geschützt. Heben Sie den Schutz zuerst auf (Überprüfen › Blattschutz aufheben)."),
    ("The cell or chart you're trying to change is on a protected sheet.", "Die Zelle oder das Diagramm, das Sie ändern möchten, befindet sich auf einem geschützten Blatt."),
    ("The cell you're trying to change is on a protected sheet.", "Die Zelle, die Sie ändern möchten, befindet sich auf einem geschützten Blatt."),
    ("Unprotect the sheet to change its edit ranges.", "Heben Sie den Blattschutz auf, um die bearbeitbaren Bereiche zu ändern."),
    ("this workbook has never been saved: pass `path` (e.g. \"/tmp/book.xlsx\")", "diese Mappe wurde noch nie gespeichert"),
    // Editing
    ("Nothing to paste.", "Es gibt nichts zum Einfügen."),
    ("the clipboard is empty", "die Zwischenablage ist leer"),
    ("nothing to undo", "nichts rückgängig zu machen"),
    ("nothing to redo", "nichts zu wiederholen"),
    ("Pick up a format first.", "Übernehmen Sie zuerst ein Format."),
    ("The copied cells don't fit on the sheet at this position.", "Die kopierten Zellen passen an dieser Stelle nicht auf das Blatt."),
    ("The copy area is the whole sheet; there is no room to insert it.", "Der Kopierbereich ist das ganze Blatt; zum Einfügen ist kein Platz."),
    ("The cut rows or columns can't be inserted inside themselves.", "Ausgeschnittene Zeilen oder Spalten können nicht in sich selbst eingefügt werden."),
    ("Insert Cut Cells works on whole rows or columns; cut whole rows or columns, or use Paste.", "Ausgeschnittene Zellen einfügen geht nur mit ganzen Zeilen oder Spalten; schneiden Sie ganze Zeilen oder Spalten aus oder verwenden Sie Einfügen."),
    ("This action won't work on multiple selections.", "Diese Aktion funktioniert nicht bei Mehrfachauswahl."),
    ("This operation isn't allowed: it would shift cells in a table on your worksheet.", "Dieser Vorgang ist nicht erlaubt: Er würde Zellen einer Tabelle auf dem Blatt verschieben."),
    ("To do this, all the merged cells need to be the same size.", "Dazu müssen alle verbundenen Zellen gleich groß sein."),
    ("To prevent possible loss of data, cells can't be pushed off the end of the worksheet.", "Um Datenverlust zu vermeiden, können Zellen nicht über das Ende des Blatts hinaus verschoben werden."),
    ("To prevent possible loss of data, columns cannot be shifted off of the worksheet.", "Um Datenverlust zu vermeiden, können Spalten nicht aus dem Blatt hinaus verschoben werden."),
    ("To prevent possible loss of data, Excel cannot shift nonblank cells off of the worksheet.", "Um Datenverlust zu vermeiden, können nicht leere Zellen nicht aus dem Blatt hinaus verschoben werden."),
    ("Column width must be between 0 and 255 characters.", "Die Spaltenbreite muss zwischen 0 und 255 Zeichen liegen."),
    ("Row height must be between 0 and 409 points.", "Die Zeilenhöhe muss zwischen 0 und 409 Punkt liegen."),
    ("The number must be between 1 and 409.", "Die Zahl muss zwischen 1 und 409 liegen."),
    ("The weekday series runs past the supported date range.", "Die Wochentagsreihe reicht über den unterstützten Datumsbereich hinaus."),
    ("We looked at all the data next to your selection and didn't see a pattern for filling in values for you.", "Neben Ihrer Auswahl wurde kein Muster gefunden, nach dem sich Werte ausfüllen lassen."),
    ("All done. We made {} replacements.", "Fertig. Es wurden {} Ersetzungen vorgenommen."),
    ("We couldn't find what you were looking for: {}", "Der Suchbegriff wurde nicht gefunden: {}"),
    ("No cells were found.", "Es wurden keine Zellen gefunden."),
    ("not found", "nicht gefunden"),
    ("No dictionary is available on this system (GridCraft uses the system word list).", "Auf diesem System ist kein Wörterbuch verfügbar."),
    ("The name entered already exists. Enter a unique name.", "Der Name ist schon vorhanden. Geben Sie einen eindeutigen Namen ein."),
    ("The name that you entered is not valid.", "Der eingegebene Name ist ungültig."),
    ("There's no field named '{}'.", "Es gibt kein Feld mit dem Namen „{}“."),
    // Comments, links
    ("There's no comment in this cell.", "Diese Zelle enthält keinen Kommentar."),
    ("There are no comments in this workbook.", "Diese Mappe enthält keine Kommentare."),
    ("there are no comments in this workbook", "diese Mappe enthält keine Kommentare"),
    ("There's no hyperlink in this cell.", "Diese Zelle enthält keinen Hyperlink."),
    ("This workbook asks to open the address below. Links can lead to harmful sites or start other programs: open it only if you trust the workbook.\n\n{}", "Diese Mappe möchte die folgende Adresse öffnen. Links können zu schädlichen Seiten führen oder andere Programme starten: Öffnen Sie sie nur, wenn Sie der Mappe vertrauen.\n\n{}"),
    // Tables, charts, PivotTables
    ("A table can't overlap another table.", "Eine Tabelle darf keine andere Tabelle überlappen."),
    ("Select a cell inside a table first, or pass `table`.", "Wählen Sie zuerst eine Zelle in einer Tabelle aus."),
    ("There's no room below the table for the total row.", "Unter der Tabelle ist kein Platz für die Ergebniszeile."),
    ("There's no room below the selection for the totals.", "Unter der Auswahl ist kein Platz für die Ergebnisse."),
    ("The headers must remain in the same row, and the resulting table range must overlap the original table range.", "Die Überschriften müssen in derselben Zeile bleiben, und der neue Tabellenbereich muss den ursprünglichen überlappen."),
    ("Select data for the chart first.", "Wählen Sie zuerst die Daten für das Diagramm aus."),
    ("A PivotTable can have at most {} row and column fields.", "Eine PivotTable kann höchstens {} Zeilen- und Spaltenfelder haben."),
    ("A PivotTable can have at most {} value fields.", "Eine PivotTable kann höchstens {} Wertfelder haben."),
    ("A PivotTable report cannot overlap a table.", "Ein PivotTable-Bericht darf keine Tabelle überlappen."),
    ("A PivotTable report cannot overlap another PivotTable report.", "Ein PivotTable-Bericht darf keinen anderen PivotTable-Bericht überlappen."),
    ("The PivotTable report can't overlap its source data. Choose a different location.", "Der PivotTable-Bericht darf seine Quelldaten nicht überlappen. Wählen Sie eine andere Stelle."),
    ("The PivotTable report is too large.", "Der PivotTable-Bericht ist zu groß."),
    ("The PivotTable report will not fit on the sheet. Choose a different location.", "Der PivotTable-Bericht passt nicht auf das Blatt. Wählen Sie eine andere Stelle."),
    ("The PivotTable report will not fit on the sheet.", "Der PivotTable-Bericht passt nicht auf das Blatt."),
    ("The source range is too large for a PivotTable.", "Der Quellbereich ist für eine PivotTable zu groß."),
    ("The table needs a header row to be used as a PivotTable source.", "Die Tabelle braucht eine Kopfzeile, um als PivotTable-Quelle zu dienen."),
    ("There's no such PivotTable.", "Diese PivotTable gibt es nicht."),
    ("no filter", "kein Filter"),
    // Analysis, forecast, solver
    ("Anova needs at least two groups, each with at least one number.", "Die Varianzanalyse braucht mindestens zwei Gruppen mit je mindestens einer Zahl."),
    ("Each input range needs at least two numbers.", "Jeder Eingabebereich braucht mindestens zwei Zahlen."),
    ("Input range contains non-numeric data.", "Der Eingabebereich enthält nicht numerische Daten."),
    ("Input range must be a single row or column.", "Der Eingabebereich muss eine einzelne Zeile oder Spalte sein."),
    ("Input ranges must have the same number of values.", "Die Eingabebereiche müssen gleich viele Werte haben."),
    ("Input X range must have {} columns or fewer.", "Der X-Eingabebereich darf höchstens {} Spalten haben."),
    ("Input Y range and input X range must have the same number of rows.", "Y- und X-Eingabebereich müssen gleich viele Zeilen haben."),
    ("Input Y range must be a single column.", "Der Y-Eingabebereich muss eine einzelne Spalte sein."),
    ("Too few observations for this many X variables.", "Zu wenige Beobachtungen für so viele X-Variablen."),
    ("The X variables are collinear; the regression can't be computed.", "Die X-Variablen sind kollinear; die Regression lässt sich nicht berechnen."),
    ("The bin range holds no numbers.", "Der Klassenbereich enthält keine Zahlen."),
    ("The input range holds no numbers.", "Der Eingabebereich enthält keine Zahlen."),
    ("The damping factor must be between 0 and 1.", "Der Dämpfungsfaktor muss zwischen 0 und 1 liegen."),
    ("The interval must be a whole number from 1 to the number of data points.", "Das Intervall muss eine ganze Zahl zwischen 1 und der Anzahl der Datenpunkte sein."),
    ("The output range {} {}; pass overwrite: true to replace it.", "Der Ausgabebereich {} {}."),
    ("There's already data in {}!{}. Do you want to replace it? (Run again with replace: true to overwrite.)", "In {}!{} stehen schon Daten. Möchten Sie sie ersetzen?"),
    ("The report doesn't fit on the sheet below and right of the output cell.", "Der Bericht passt rechts unterhalb der Ausgabezelle nicht auf das Blatt."),
    ("The consolidated data doesn't fit below and right of the destination.", "Die konsolidierten Daten passen rechts unterhalb des Ziels nicht auf das Blatt."),
    ("The criteria label `{}` doesn't match a column of the list.", "Die Kriterienbeschriftung `{}` passt zu keiner Spalte der Liste."),
    ("The extract range can't overlap the list range.", "Der Zielbereich darf den Listenbereich nicht überlappen."),
    ("Select a timeline column and a values column for the forecast.", "Wählen Sie für die Prognose eine Zeitachsenspalte und eine Wertespalte."),
    ("The forecast end must be after the last point of the timeline.", "Das Prognoseende muss nach dem letzten Punkt der Zeitachse liegen."),
    ("The forecast start needs at least two values before it to fit the forecast.", "Vor dem Prognosebeginn müssen mindestens zwei Werte liegen."),
    ("The timeline must hold numbers or dates ({} isn't one).", "Die Zeitachse muss Zahlen oder Datumswerte enthalten ({} ist keiner)."),
    ("The timeline needs at least two points a constant step apart.", "Die Zeitachse braucht mindestens zwei Punkte mit gleichem Abstand."),
    ("There are no values to forecast.", "Es gibt keine Werte für eine Prognose."),
    ("Goal Seeking found a solution: {}", "Die Zielwertsuche hat eine Lösung gefunden: {}"),
    ("A scenario named '{}' already exists on this sheet.", "Auf diesem Blatt gibt es schon ein Szenario namens „{}“."),
    ("There are no scenarios on this sheet.", "Auf diesem Blatt gibt es keine Szenarien."),
    ("This sheet has no saved Solver model.", "Dieses Blatt hat kein gespeichertes Solver-Modell."),
    ("The objective cell must contain a formula.", "Die Zielzelle muss eine Formel enthalten."),
    ("The objective must be a single cell.", "Das Ziel muss eine einzelne Zelle sein."),
    ("The variable cells must be a reference like B2:B4.", "Die Variablenzellen müssen ein Bezug wie B2:B4 sein."),
    ("The variable cells must be on the objective cell's sheet.", "Die Variablenzellen müssen auf dem Blatt der Zielzelle liegen."),
    ("Constraint cells must be on the objective cell's sheet.", "Die Zellen der Nebenbedingungen müssen auf dem Blatt der Zielzelle liegen."),
    ("Variable cells must hold constants, not formulas.", "Variablenzellen müssen Konstanten enthalten, keine Formeln."),
    ("Integer and binary constraints must be on variable cells.", "Ganzzahlige und binäre Nebenbedingungen müssen sich auf Variablenzellen beziehen."),
    ("The constraint's two sides must be the same size (or the right side one cell).", "Beide Seiten der Nebenbedingung müssen gleich groß sein (oder die rechte Seite eine Zelle)."),
    ("Too many constraints (at most {} cells).", "Zu viele Nebenbedingungen (höchstens {} Zellen)."),
    ("Too many constraints (at most {}).", "Zu viele Nebenbedingungen (höchstens {})."),
    ("Too many variable cells (at most {}).", "Zu viele Variablenzellen (höchstens {})."),
    ("Too many report sheets.", "Zu viele Berichtsblätter."),
    ("The Limits report doesn't apply to models with integer or binary constraints.", "Der Grenzwertbericht gilt nicht für Modelle mit ganzzahligen oder binären Nebenbedingungen."),
    ("The Limits report is available for Simplex LP and GRG Nonlinear.", "Der Grenzwertbericht ist für Simplex-LP und GRG-Nichtlinear verfügbar."),
    ("The Evolutionary method needs a lower and an upper bound on every variable cell (constraints such as \"A1:A3 <= 10\"), or requireBounds: false.", "Die EA-Methode braucht für jede Variablenzelle eine untere und obere Grenze (Nebenbedingungen wie \"A1:A3 <= 10\")."),
    // Status bar, panes, dialogs (labels with values)
    ("Sum: {}", "Summe: {}"),
    ("Count: {}", "Anzahl: {}"),
    ("Average: {}", "Mittelwert: {}"),
    ("Width: {} ({} pixels)", "Breite: {} ({} Pixel)"),
    ("Height: {} ({} pixels)", "Höhe: {} ({} Pixel)"),
    ("Circular References: {}", "Zirkelbezüge: {}"),
    ("Page {}", "Seite {}"),
    ("Series {}", "Reihe {}"),
    ("Layout {}", "Layout {}"),
    ("Picture: {}", "Bild: {}"),
    ("Sheets: {}", "Blätter: {}"),
    ("Hidden: {}", "Ausgeblendet: {}"),
    ("Undo {}", "Rückgängig: {}"),
    ("Version {}", "Version {}"),
    ("Not in Dictionary ({}):", "Nicht im Wörterbuch ({}):"),
    ("Do you want to save the changes you made to {}?", "Möchten Sie die Änderungen an „{}“ speichern?"),
    ("Save a copy as .{}", "Kopie als .{} speichern"),
    ("Ranges that stay editable when the sheet is protected:\n{}", "Bereiche, die bei geschütztem Blatt bearbeitbar bleiben:\n{}"),
    (" is not available right now", " ist gerade nicht verfügbar"),
    ("{} is not available right now", "{} ist gerade nicht verfügbar"),
];

/// Parser tokens as `ParseError` spells them (`Debug` of the lexer's `Tok`), in German.
const TOKENS: &[(&str, &str)] = &[
    ("Semicolon", "`;`"),
    ("Colon", "`:`"),
    ("Comma", "`,`"),
    ("LParen", "`(`"),
    ("RParen", "`)`"),
    ("LBrace", "`{`"),
    ("RBrace", "`}`"),
    ("Space", "Leerzeichen"),
    ("Eof", "Formelende"),
];

/// Templates split at their placeholders, longest literal text first so the most specific wins.
fn compiled() -> &'static [(Vec<&'static str>, &'static str)] {
    static C: OnceLock<Vec<(Vec<&'static str>, &'static str)>> = OnceLock::new();
    C.get_or_init(|| {
        let mut v: Vec<(Vec<&'static str>, &'static str)> = TEMPLATES.iter().map(|(en, de)| (en.split("{}").collect(), *de)).collect();
        v.sort_by_key(|(parts, _)| std::cmp::Reverse(parts.iter().map(|p| p.len()).sum::<usize>()));
        v
    })
}

/// The values `parts` leave open in `text`, or `None` if `text` doesn't fit the template.
fn captures<'a>(parts: &[&str], text: &'a str) -> Option<Vec<&'a str>> {
    let (first, rest) = parts.split_first()?;
    let mut tail = text.strip_prefix(first)?;
    let mut out = Vec::new();
    for (i, part) in rest.iter().enumerate() {
        let last = i + 1 == rest.len();
        if last {
            let value = tail.strip_suffix(part)?;
            out.push(value);
            tail = "";
        } else if part.is_empty() {
            return None;
        } else {
            let at = tail.find(part)?;
            out.push(&tail[..at]);
            tail = &tail[at + part.len()..];
        }
    }
    tail.is_empty().then_some(out)
}

/// Fills `{}` (in order) or `{1}`, `{2}` … (by position) in a German template.
fn fill(template: &str, values: &[String]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut next = 0;
    let mut rest = template;
    while let Some(at) = rest.find('{') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[at..]);
            return out;
        };
        let inner = &after[..end];
        let value = if inner.is_empty() {
            next += 1;
            values.get(next - 1)
        } else {
            inner.parse::<usize>().ok().and_then(|n| values.get(n.saturating_sub(1)))
        };
        match value {
            Some(v) => out.push_str(v),
            None => {
                out.push('{');
                out.push_str(inner);
                out.push('}');
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

/// A parser token or a value from a message, in German where it is one of ours.
fn value(text: &str, depth: u8) -> String {
    if let Some((_, de)) = TOKENS.iter().find(|(en, _)| *en == text) {
        return (*de).to_string();
    }
    if let Some(inner) = text.strip_prefix("Number(").and_then(|r| r.strip_suffix(')')) {
        return format!("Zahl {inner}");
    }
    if let Some(inner) = text.strip_prefix("Word(").and_then(|r| r.strip_suffix(')')) {
        return inner.to_string();
    }
    if let Some(inner) = text.strip_prefix("Func(").and_then(|r| r.strip_suffix(')')) {
        return format!("Funktion {}", inner.trim_matches('"'));
    }
    if let Some(inner) = text.strip_prefix("Text(").and_then(|r| r.strip_suffix(')')) {
        return format!("Text {inner}");
    }
    if let Some(inner) = text.strip_prefix("Op(").and_then(|r| r.strip_suffix(')')) {
        return format!("Operator {}", inner.trim_matches('"'));
    }
    // Only a nested message is translated again; a single word is a name (file, sheet, field) and stays.
    if depth < 3 && text.contains(' ') { translate_at(text, depth + 1) } else { text.to_string() }
}

fn translate_at(text: &str, depth: u8) -> String {
    if let Some(de) = super::de::translate(text) {
        return de.to_string();
    }
    for (parts, de) in compiled() {
        if parts.len() > 1
            && let Some(caps) = captures(parts, text)
        {
            let values: Vec<String> = caps.iter().map(|c| value(c, depth)).collect();
            return fill(de, &values);
        }
        if parts.len() == 1 && parts[0] == text {
            return (*de).to_string();
        }
    }
    text.to_string()
}

/// A message in German: an exact label, else the first template that fits, else unchanged.
pub(super) fn translate(text: &str) -> String {
    translate_at(text, 0)
}

#[cfg(test)]
mod tests {
    use super::translate;

    #[test]
    fn formula_errors_read_german() {
        assert_eq!(
            translate("There's a problem with this formula: expected `,` or `)`, found Semicolon"),
            "Mit dieser Formel stimmt etwas nicht: erwartet wurde `,` oder `)`, gefunden: `;`"
        );
        assert_eq!(translate("Nothing to paste."), "Es gibt nichts zum Einfügen.");
        assert_eq!(translate("Sum: 42"), "Summe: 42");
        assert_eq!(translate("We can't open 'a.xlsx': bad zip"), "„a.xlsx“ lässt sich nicht öffnen: bad zip");
        assert_eq!(translate("Paste"), "Einfügen");
        assert_eq!(translate("Something new"), "Something new");
    }
}
