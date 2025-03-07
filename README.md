# Mathetool - Rechenschritte sichtbar machen

## Was ist dieses Projekt?

Dieses Tool ist ein Open-Source-Projekt, das Mathematik nicht nur ausrechnet, sondern die **Rechenschritte** zeigt.  
Es soll helfen, Mathematik besser zu verstehen, statt nur Ergebnisse zu liefern.

## Warum dieses Tool?

Mathematik wird oft als eine reine Rechenaufgabe gesehen – aber dahinter steckt **Logik, Struktur und klare Schritte**.  
Dieses Tool zeigt jeden einzelnen Rechenschritt und erklärt, was passiert.  
Zielgruppe sind alle, die Mathematik besser verstehen wollen – von Schülern bis zu Erwachsenen, die ihre Kenntnisse auffrischen.

## Technische Basis

- **Programmiersprache:** Rust (in Zukunft vielleicht auch ergänzend C)
- **Plattform:** Linux (getestet unter Manjaro)
- **Ziel:** Erst eine einfache Terminal-Anwendung, später mit grafischer Oberfläche

## Warum Rust und nicht C++?

C++ wurde ursprünglich in Betracht gezogen, aber ich habe mich bewusst für **Rust** entschieden, weil:

- Rust bietet **moderne Sicherheit** ohne Garbage Collection.
- Rust zwingt zu **klarem Denken** durch Ownership und Borrowing.
- Rust ist **systemnah**, aber vermeidet viele Fehler, die in C++ gerne übersehen werden.
- Rust passt perfekt zu meinem Ziel: **klare, sichere und nachvollziehbare Software**.

## Philosophie

Wer die Hintergründe meiner Arbeit verstehen möchte, kann in [PHILOSOPHY.md](./PHILOSOPHY.md) nachlesen, was meine Haltung zu Softwareentwicklung und Lernsoftware ist.

## Status

- Projektstart: Februar 2025
- Erste Version: Nur Terminal, Fokus auf einfache lineare Gleichungen mit Rechenschritten
- Nächste Schritte: Erweiterung auf quadratische Gleichungen, Bruchrechnung und Wurzelgleichungen
- Später: Grafische Oberfläche und evtl. Erweiterung auf andere Plattformen

## Lizenz

Dieses Projekt steht unter der **MIT-Lizenz** – freie Nutzung, aber mit Respekt vor dem Urheber.  
Details dazu in der Datei `LICENSE`.

## Mitmachen?

Konstruktive Vorschläge sind immer willkommen.  
Allerdings wird hier **Qualität vor Schnelligkeit** gesetzt – wer nur Copy-Paste-Hacks liefern will, ist hier falsch.

---

