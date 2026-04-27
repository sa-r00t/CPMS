# EN 🇺🇸

## CPMS

**CPMS** stands for **"Communication Protocol Mohamed Seif"**.
It is a modular, privacy‑oriented communication protocol designed to give users full control over their data flow and network exchanges.

Unlike traditional protocols that impose rigid structures and trust the underlying transport layer, CPMS is built from the ground up to be **implementation‑agnostic** (compatible with Rust, C++, and any language capable of calling a C interface) and **transport‑agnostic** (works over TCP, UDP, serial links, LoRa, Bluetooth, or even physical custom radio).

The protocol is not a monolithic library. It is a **lightweight frame format** combined with a **shared DLL** that contains all core operations: frame construction, parsing, integrity checking, and optional obfuscation.

---

## Core Features

- **Fixed‑size header** (20 bytes) – MAGIC, VERSION, TYPE, LENGTH, TIMESTAMP.
- **Variable payload** – carry anything: text, binary, encrypted data, file chunks.
- **JCS32 checksum** – our own integrity algorithm, not a standard CRC.
- **DLL‑based architecture** – shared logic between different languages and platforms.
- **Modular by design** – add encryption, authentication, or custom flags without breaking the frame structure.
- **Ready for low‑bandwidth / long‑range** – LoRa, radio modems, satellite.

---

## JCS32 – Joint Checksum Standard 32

JCS32 is our **custom integrity algorithm**, designed specifically for CPMS.

It takes the input data, runs it through a chaotic propagation loop over 64 bits, compresses the result, applies a bit‑permutation step, and expands it to 8 bytes.

**No CRC. No standard. Just our own math.**

This ensures that:
- Two different inputs are extremely unlikely to produce the same checksum.
- No external dependency (zlib, etc.) is required.
- The algorithm remains fully under our control.

> ⚠️ The buffer returned by the JCS32 function is dynamically allocated.
> It must be freed using `FreeBuffer()` after use.

---

## Why CPMS ?

Most existing protocols make assumptions about the network, the transport, or the data format.  
CPMS does not.

You decide:
- What runs on top of it (chat, file transfer, command & control, telemetry).
- What runs below it (TCP, UDP, radio, serial).
- Who can read the payload (encryption is your choice, not forced).
- How integrity is verified (JCS32 is ours, but you can add more).

**Your data, your rules. Your network, your format.**

---

## Workflow / How It Works

CPMS is built around a **shared DLL** written in C++. This DLL contains all the core logic: frame encoding, decoding, JCS32 checksum calculation, and optional obfuscation.

On top of this DLL, we provide a **Rust wrapper** that loads the DLL at runtime using FFI (`libloading` on Windows / `dlopen` on Unix). The wrapper exposes simple, safe functions to the Rust application: `encode()`, `decode()`, `free_buffer()`.

### Step by step (encoding)

1. The Rust application calls `encode("hello")`.
2. The wrapper converts the string to a C‑compatible format (`CString`).
3. The wrapper calls the DLL's `Encode` function with the input data.
4. The DLL builds the frame:  
   `[MAGIC][VERSION][TYPE][LENGTH][TIMESTAMP][PAYLOAD]`
5. The DLL computes the **JCS32** checksum over the entire frame.
6. The DLL appends the checksum (8 bytes) and returns a pointer to the final frame.
7. The wrapper copies the frame data into a Rust `Vec<u8>` and calls `free_buffer()`.
8. The Rust application gets a clean `Vec<u8>` ready to send over the network.

### Step by step (decoding)

1. The Rust application calls `decode(&received_bytes)`.
2. The wrapper passes the raw bytes to the DLL's `Decode` function.
3. The DLL recomputes JCS32 over the frame (excluding the last 8 bytes).
4. If checksums match, the DLL extracts the PAYLOAD and returns it.
5. The wrapper copies the payload into a Rust `String` and frees the DLL buffer.
6. The Rust application gets the original message.

This architecture keeps the **core protocol logic** in one place (the DLL), while allowing **any language** to use it through a thin wrapper. The Rust wrapper is just an example – the same DLL can be called from C++, Python, or any language with FFI support.

---

## Implementation

CPMS has been implemented **from scratch** in **Rust** and **C++**.

- A **shared DLL** contains the core logic: frame encoding, decoding, JCS32 checksum, and optional data obfuscation.
- The **Rust wrapper** uses FFI (`libloading` or direct `LoadLibrary`) to call the DLL functions.
- The **C++ side** can use the same DLL natively.

This allows the same protocol to run in high‑level applications (Rust GUI, CLI tools) and low‑level systems (C++ servers, embedded devices, radio gateways).

---

## Philosophy

One sentence. One belief.

> **Your data, your rules. Your network, your format.**

No bloat. No unnecessary layers. No hidden telemetry.  
Just a clean, controlled, and auditable communication channel.

CPMS is not a product. It is a foundation.

---

# FR 🇫🇷

## CPMS

**CPMS** signifie **"Protocole de Communication Mohamed Seif"**.
Il s’agit d’un protocole de communication modulaire, orienté confidentialité, conçu pour donner à l’utilisateur un contrôle total sur ses échanges de données et son trafic réseau.

Contrairement aux protocoles traditionnels qui imposent des structures rigides et font confiance à la couche transport, CPMS est construit de A à Z pour être **indépendant de l’implémentation** (compatible Rust, C++, et tout langage pouvant appeler une interface C) et **indépendant du transport** (fonctionne sur TCP, UDP, liaison série, LoRa, Bluetooth, ou même radio personnalisée).

Le protocole n’est pas une bibliothèque monolithique. C’est un **format de trame léger** associé à une **DLL partagée** qui contient toutes les opérations essentielles : construction de trame, analyse, vérification d’intégrité et obfuscation optionnelle.

---

## Fonctionnalités principales

- **En‑tête de taille fixe** (20 octets) – MAGIC, VERSION, TYPE, LENGTH, TIMESTAMP.
- **Charge utile variable** – pour transporter n’importe quoi : texte, binaire, données chiffrées, fragments de fichiers.
- **Somme de contrôle JCS32** – notre propre algorithme d’intégrité, pas un CRC standard.
- **Architecture basée sur une DLL** – logique partagée entre différents langages et plateformes.
- **Conception modulaire** – ajout de chiffrement, d’authentification ou de drapeaux personnalisés sans casser la structure de la trame.
- **Prêt pour les liaisons bas débit / longue distance** – LoRa, modems radio, satellite.

---

## JCS32 – Joint Checksum Standard 32

JCS32 est notre **algorithme d’intégrité sur mesure**, spécifiquement conçu pour CPMS.

Il prend les données d’entrée, leur applique une boucle de propagation chaotique sur 64 bits, compresse le résultat, applique une permutation de bits, et l’étend à 8 octets.

**Pas de CRC. Pas de standard. Juste nos propres calculs.**

Cela garantit :
- Une probabilité extrêmement faible que deux entrées différentes produisent la même somme de contrôle.
- Aucune dépendance externe (zlib, etc.).
- L’algorithme reste entièrement sous notre contrôle.

> ⚠️ Le tampon retourné par la fonction JCS32 est alloué dynamiquement.
> Il doit être libéré avec `FreeBuffer()` après utilisation.

---

## Pourquoi CPMS ?

La plupart des protocoles existants font des hypothèses sur le réseau, le transport ou le format des données.  
CPMS, non.

Tu décides :
- Ce qui fonctionne au‑dessus (chat, transfert de fichiers, commande & contrôle, télémétrie).
- Ce qui fonctionne en dessous (TCP, UDP, radio, série).
- Qui peut lire la charge utile (le chiffrement est ton choix, pas imposé).
- Comment l’intégrité est vérifiée (JCS32 est le nôtre, mais tu peux en ajouter d’autres).

**Tes données, tes règles. Ton réseau, ton format.**

---

## Workflow / Comment ça marche

CPMS repose sur une **DLL partagée** écrite en C++. Cette DLL contient toute la logique centrale : encodage, décodage des trames, calcul de la somme JCS32, et obfuscation optionnelle.

Au‑dessus de cette DLL, un **wrapper Rust** charge la DLL au moment de l’exécution via FFI (`libloading` sous Windows, `dlopen` sous Unix). Le wrapper expose des fonctions simples et sécurisées à l’application Rust : `encode()`, `decode()`, `free_buffer()`.

### Étape par étape (encodage)

1. L’application Rust appelle `encode("hello")`.
2. Le wrapper convertit la chaîne en un format compatible C (`CString`).
3. Le wrapper appelle la fonction `Encode` de la DLL avec les données d’entrée.
4. La DLL construit la trame :  
   `[MAGIC][VERSION][TYPE][LENGTH][TIMESTAMP][PAYLOAD]`
5. La DLL calcule la somme **JCS32** sur l’ensemble de la trame.
6. La DLL ajoute la somme (8 octets) et retourne un pointeur vers la trame finale.
7. Le wrapper copie les données dans un `Vec<u8>` Rust et appelle `free_buffer()`.
8. L’application Rust reçoit un `Vec<u8>` prêt à être envoyé sur le réseau.

### Étape par étape (décodage)

1. L’application Rust appelle `decode(&octets_recus)`.
2. Le wrapper transmet les octets bruts à la fonction `Decode` de la DLL.
3. La DLL recalcule JCS32 sur la trame (sans les 8 derniers octets).
4. Si les sommes correspondent, la DLL extrait la CHARGE UTILE et la retourne.
5. Le wrapper copie la charge utile dans une `String` Rust et libère le tampon de la DLL.
6. L’application Rust obtient le message original.

Cette architecture maintient la **logique centrale du protocole** dans un seul endroit (la DLL), tout en permettant à **n’importe quel langage** de l’utiliser via une fine couche d’adaptation. Le wrapper Rust n’est qu’un exemple – la même DLL peut être appelée depuis C++, Python, ou tout langage supportant FFI.

---

## Implémentation

CPMS a été implémenté **de zéro** en **Rust** et **C++**.

- Une **DLL partagée** contient la logique centrale : encodage, décodage des trames, somme JCS32, et obfuscation optionnelle des données.
- Le **wrapper Rust** utilise l’interface FFI (`libloading` ou `LoadLibrary` direct) pour appeler les fonctions de la DLL.
- Le code **C++** peut utiliser nativement la même DLL.

Ainsi, le même protocole peut tourner aussi bien dans des applications haut niveau (interface Rust, outils en ligne de commande) que dans des systèmes bas niveau (serveurs C++, dispositifs embarqués, passerelles radio).

---

## Philosophie

Une phrase. Une conviction.

> **Tes données, tes règles. Ton réseau, ton format.**

Pas de fioritures. Pas de couches inutiles. Pas de télémétrie cachée.  
Juste un canal de communication propre, contrôlé et vérifiable.

CPMS n’est pas un produit. C’est une base.